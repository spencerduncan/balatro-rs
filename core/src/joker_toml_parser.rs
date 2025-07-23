use crate::joker_toml_schema::{JokerConfig, TomlJokerDefinition};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use thiserror::Error;

/// Errors that can occur during TOML parsing and validation
#[derive(Error, Debug)]
pub enum TomlParserError {
    #[error("Failed to read TOML file: {0}")]
    FileRead(#[from] std::io::Error),

    #[error("Failed to parse TOML: {0}")]
    TomlParse(#[from] toml::de::Error),

    #[error("Schema validation failed: {0}")]
    ValidationFailed(String),

    #[error("Unsupported schema version: {0}")]
    UnsupportedSchemaVersion(String),

    #[error("Duplicate joker ID: {0}")]
    DuplicateJokerId(String),

    #[error("Invalid joker ID: {0}")]
    InvalidJokerId(String),

    #[error("Missing required field: {0}")]
    MissingRequiredField(String),

    #[error("Invalid field value: {field} = {value}")]
    InvalidFieldValue { field: String, value: String },
}

/// Parser for TOML joker definitions
pub struct TomlJokerParser {
    /// Supported schema versions
    _supported_versions: Vec<String>,

    /// Validation rules
    validators: Vec<Box<dyn JokerValidator>>,
}

impl Default for TomlJokerParser {
    fn default() -> Self {
        Self::new()
    }
}

impl TomlJokerParser {
    /// Create a new parser with default configuration
    pub fn new() -> Self {
        let mut parser = Self {
            _supported_versions: vec!["1.0.0".to_string()],
            validators: Vec::new(),
        };

        // Add default validators
        parser.add_validator(Box::new(SchemaVersionValidator));
        parser.add_validator(Box::new(UniqueIdValidator));
        parser.add_validator(Box::new(RequiredFieldValidator));
        parser.add_validator(Box::new(RangeValidator));
        parser.add_validator(Box::new(LogicValidator));

        parser
    }

    /// Add a custom validator
    pub fn add_validator(&mut self, validator: Box<dyn JokerValidator>) {
        self.validators.push(validator);
    }

    /// Parse joker definitions from a TOML file
    pub fn parse_file<P: AsRef<Path>>(&self, path: P) -> Result<JokerConfig, TomlParserError> {
        let content = fs::read_to_string(path)?;
        self.parse_string(&content)
    }

    /// Parse joker definitions from a TOML string
    pub fn parse_string(&self, content: &str) -> Result<JokerConfig, TomlParserError> {
        // Parse TOML
        let config: JokerConfig = toml::from_str(content)?;

        // Validate
        self.validate_config(&config)?;

        Ok(config)
    }

    /// Validate the parsed configuration
    fn validate_config(&self, config: &JokerConfig) -> Result<(), TomlParserError> {
        for validator in &self.validators {
            validator.validate(config)?;
        }
        Ok(())
    }

    /// Parse multiple TOML files and merge them
    pub fn parse_directory<P: AsRef<Path>>(
        &self,
        dir_path: P,
    ) -> Result<JokerConfig, TomlParserError> {
        let mut merged_config = JokerConfig {
            schema_version: "1.0.0".to_string(),
            jokers: Vec::new(),
        };

        let dir = fs::read_dir(dir_path)?;
        for entry in dir {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("toml") {
                let config = self.parse_file(&path)?;

                // Merge jokers
                merged_config.jokers.extend(config.jokers);

                // Use the latest schema version found
                if config.schema_version > merged_config.schema_version {
                    merged_config.schema_version = config.schema_version;
                }
            }
        }

        // Validate merged configuration
        self.validate_config(&merged_config)?;

        Ok(merged_config)
    }

    /// Hot-reload support: parse and return only changed jokers
    pub fn parse_with_hot_reload(
        &self,
        content: &str,
        previous_config: Option<&JokerConfig>,
    ) -> Result<(JokerConfig, Vec<String>), TomlParserError> {
        let new_config = self.parse_string(content)?;

        let changed_joker_ids = if let Some(prev) = previous_config {
            self.find_changed_jokers(prev, &new_config)
        } else {
            // All jokers are "new" if no previous config
            new_config.jokers.iter().map(|j| j.id.clone()).collect()
        };

        Ok((new_config, changed_joker_ids))
    }

    /// Find jokers that have changed between configurations
    fn find_changed_jokers(&self, old: &JokerConfig, new: &JokerConfig) -> Vec<String> {
        let mut changed = Vec::new();

        // Create lookup maps
        let old_jokers: HashMap<&str, &TomlJokerDefinition> =
            old.jokers.iter().map(|j| (j.id.as_str(), j)).collect();
        let new_jokers: HashMap<&str, &TomlJokerDefinition> =
            new.jokers.iter().map(|j| (j.id.as_str(), j)).collect();

        // Check for new or modified jokers
        for (id, new_joker) in &new_jokers {
            if let Some(old_joker) = old_jokers.get(id) {
                // Compare serialized forms to detect changes
                if self.joker_definition_changed(old_joker, new_joker) {
                    changed.push(id.to_string());
                }
            } else {
                // New joker
                changed.push(id.to_string());
            }
        }

        // Check for removed jokers
        for id in old_jokers.keys() {
            if !new_jokers.contains_key(id) {
                changed.push(id.to_string());
            }
        }

        changed
    }

    /// Check if a joker definition has changed
    fn joker_definition_changed(
        &self,
        old: &TomlJokerDefinition,
        new: &TomlJokerDefinition,
    ) -> bool {
        // Simple approach: serialize both and compare
        // In a production system, you might want more sophisticated diffing
        match (serde_json::to_string(old), serde_json::to_string(new)) {
            (Ok(old_json), Ok(new_json)) => old_json != new_json,
            _ => true, // Assume changed if serialization fails
        }
    }
}

/// Trait for validating TOML joker configurations
pub trait JokerValidator: Send + Sync {
    fn validate(&self, config: &JokerConfig) -> Result<(), TomlParserError>;
}

/// Validates schema version compatibility
pub struct SchemaVersionValidator;

impl JokerValidator for SchemaVersionValidator {
    fn validate(&self, config: &JokerConfig) -> Result<(), TomlParserError> {
        let supported_versions = ["1.0.0"];

        if !supported_versions.contains(&config.schema_version.as_str()) {
            return Err(TomlParserError::UnsupportedSchemaVersion(
                config.schema_version.clone(),
            ));
        }

        Ok(())
    }
}

/// Validates that all joker IDs are unique
pub struct UniqueIdValidator;

impl JokerValidator for UniqueIdValidator {
    fn validate(&self, config: &JokerConfig) -> Result<(), TomlParserError> {
        let mut seen_ids = std::collections::HashSet::new();

        for joker in &config.jokers {
            if !seen_ids.insert(&joker.id) {
                return Err(TomlParserError::DuplicateJokerId(joker.id.clone()));
            }
        }

        Ok(())
    }
}

/// Validates that all required fields are present
pub struct RequiredFieldValidator;

impl JokerValidator for RequiredFieldValidator {
    fn validate(&self, config: &JokerConfig) -> Result<(), TomlParserError> {
        for joker in &config.jokers {
            // Check required fields
            if joker.id.is_empty() {
                return Err(TomlParserError::MissingRequiredField("id".to_string()));
            }

            if joker.name.is_empty() {
                return Err(TomlParserError::MissingRequiredField("name".to_string()));
            }

            if joker.description.is_empty() {
                return Err(TomlParserError::MissingRequiredField(
                    "description".to_string(),
                ));
            }

            // Validate ID format (should be valid enum variant name)
            if !joker.id.chars().all(|c| c.is_alphanumeric() || c == '_') {
                return Err(TomlParserError::InvalidJokerId(joker.id.clone()));
            }
        }

        Ok(())
    }
}

/// Validates value ranges and constraints
pub struct RangeValidator;

impl JokerValidator for RangeValidator {
    fn validate(&self, config: &JokerConfig) -> Result<(), TomlParserError> {
        for joker in &config.jokers {
            // Validate cost range
            if let Some(cost) = joker.cost {
                if cost > 1000 {
                    return Err(TomlParserError::InvalidFieldValue {
                        field: "cost".to_string(),
                        value: cost.to_string(),
                    });
                }
            }

            // Validate effect values based on type
            match &joker.effect {
                crate::joker_toml_schema::TomlJokerEffect::Scoring {
                    chips,
                    mult,
                    money,
                    mult_multiplier,
                    ..
                } => {
                    if *chips < -1000 || *chips > 1000 {
                        return Err(TomlParserError::InvalidFieldValue {
                            field: "chips".to_string(),
                            value: chips.to_string(),
                        });
                    }

                    if *mult < -1000 || *mult > 1000 {
                        return Err(TomlParserError::InvalidFieldValue {
                            field: "mult".to_string(),
                            value: mult.to_string(),
                        });
                    }

                    if *money < -100 || *money > 100 {
                        return Err(TomlParserError::InvalidFieldValue {
                            field: "money".to_string(),
                            value: money.to_string(),
                        });
                    }

                    if *mult_multiplier < 0.0 || *mult_multiplier > 10.0 {
                        return Err(TomlParserError::InvalidFieldValue {
                            field: "mult_multiplier".to_string(),
                            value: mult_multiplier.to_string(),
                        });
                    }
                }
                _ => {
                    // Additional validation for other effect types can be added here
                }
            }
        }

        Ok(())
    }
}

/// Validates logical consistency of joker definitions
pub struct LogicValidator;

impl JokerValidator for LogicValidator {
    fn validate(&self, config: &JokerConfig) -> Result<(), TomlParserError> {
        for joker in &config.jokers {
            // Validate that conditional jokers have valid conditions
            if let crate::joker_toml_schema::TomlJokerEffect::Conditional { condition, .. } =
                &joker.effect
            {
                self.validate_condition(condition)?;
            }

            // Validate state consistency
            if let Some(state) = &joker.state {
                if let Some(behavior) = &joker.behavior {
                    self.validate_state_behavior_consistency(state, behavior)?;
                }
            }
        }

        Ok(())
    }
}

impl LogicValidator {
    fn validate_condition(
        &self,
        _condition: &crate::joker_toml_schema::TomlJokerCondition,
    ) -> Result<(), TomlParserError> {
        // Validate condition logic
        // This could include checking for impossible conditions,
        // circular references, etc.
        Ok(())
    }

    fn validate_state_behavior_consistency(
        &self,
        _state: &crate::joker_toml_schema::TomlJokerState,
        _behavior: &crate::joker_toml_schema::TomlJokerBehavior,
    ) -> Result<(), TomlParserError> {
        // Validate that behavior references valid state fields
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_basic_joker() {
        let toml_str = r#"
            schema_version = "1.0.0"
            
            [[jokers]]
            id = "joker"
            name = "Joker"
            description = "+4 Mult"
            rarity = "common"
            cost = 2
            
            [jokers.effect]
            type = "scoring"
            mult = 4
        "#;

        let parser = TomlJokerParser::new();
        let result = parser.parse_string(toml_str);

        assert!(result.is_ok());
        let config = result.unwrap();
        assert_eq!(config.jokers.len(), 1);
        assert_eq!(config.jokers[0].name, "Joker");
    }

    #[test]
    fn test_validation_duplicate_ids() {
        let toml_str = r#"
            schema_version = "1.0.0"
            
            [[jokers]]
            id = "joker"
            name = "Joker 1"
            description = "+4 Mult"
            rarity = "common"
            
            [jokers.effect]
            type = "scoring"
            mult = 4
            
            [[jokers]]
            id = "joker"
            name = "Joker 2"
            description = "+2 Mult"
            rarity = "common"
            
            [jokers.effect]
            type = "scoring"
            mult = 2
        "#;

        let parser = TomlJokerParser::new();
        let result = parser.parse_string(toml_str);

        assert!(result.is_err());
        match result.unwrap_err() {
            TomlParserError::DuplicateJokerId(id) => assert_eq!(id, "joker"),
            _ => panic!("Expected DuplicateJokerId error"),
        }
    }

    #[test]
    fn test_hot_reload_detection() {
        let original = r#"
            schema_version = "1.0.0"
            
            [[jokers]]
            id = "joker"
            name = "Joker"
            description = "+4 Mult"
            rarity = "common"
            
            [jokers.effect]
            type = "scoring"
            mult = 4
        "#;

        let modified = r#"
            schema_version = "1.0.0"
            
            [[jokers]]
            id = "joker"
            name = "Joker"
            description = "+5 Mult"
            rarity = "common"
            
            [jokers.effect]
            type = "scoring"
            mult = 5
        "#;

        let parser = TomlJokerParser::new();
        let original_config = parser.parse_string(original).unwrap();
        let (_, changed) = parser
            .parse_with_hot_reload(modified, Some(&original_config))
            .unwrap();

        assert_eq!(changed.len(), 1);
        assert_eq!(changed[0], "joker");
    }
}
