//! # ActionValidator Service
//!
//! This module provides a clean validation abstraction for game actions.
//! It separates validation logic from game logic, following Single Responsibility Principle.

use crate::action::Action;
use crate::domain::value_objects::ValidationResult;
use crate::game::Game;
use std::sync::Arc;

/// Trait for action validation rules
///
/// Each rule implementation focuses on a specific validation concern,
/// following the Single Responsibility Principle.
pub trait ValidationRule: Send + Sync {
    /// Get the name of this validation rule
    fn name(&self) -> &str;

    /// Validate an action against this rule
    fn validate(
        &self,
        action: &Action,
        game: &Game,
        context: &ValidationContext,
    ) -> ValidationResult;

    /// Check if this rule applies to the given action
    fn applies_to(&self, _action: &Action) -> bool {
        // By default, apply to all actions
        true
    }

    /// Get the priority of this rule (higher runs first)
    fn priority(&self) -> u32 {
        50
    }
}

/// Context for validation operations
///
/// Provides additional information and state access during validation.
#[derive(Debug, Clone)]
pub struct ValidationContext {
    /// Session ID for the current game session
    pub session_id: Option<String>,
    /// Whether to perform strict validation
    pub strict_mode: bool,
    /// Maximum allowed action history
    pub max_action_history: usize,
    /// Whether async validation is enabled
    pub async_enabled: bool,
    /// Custom metadata for validation
    pub metadata: std::collections::HashMap<String, String>,
}

impl Default for ValidationContext {
    fn default() -> Self {
        Self {
            session_id: None,
            strict_mode: false,
            max_action_history: 1000,
            async_enabled: false,
            metadata: std::collections::HashMap::new(),
        }
    }
}

impl ValidationContext {
    /// Create a new validation context
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the session ID
    pub fn with_session(mut self, session_id: String) -> Self {
        self.session_id = Some(session_id);
        self
    }

    /// Enable strict validation mode
    pub fn with_strict_mode(mut self, enabled: bool) -> Self {
        self.strict_mode = enabled;
        self
    }

    /// Add custom metadata
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }
}

/// ActionValidator service for validating game actions
///
/// This service provides a clean abstraction for validating actions
/// with composable validation rules.
pub struct ActionValidator {
    rules: Vec<Arc<dyn ValidationRule>>,
    context: ValidationContext,
}

impl ActionValidator {
    /// Create a new ActionValidator
    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
            context: ValidationContext::default(),
        }
    }

    /// Create with a specific context
    pub fn with_context(context: ValidationContext) -> Self {
        Self {
            rules: Vec::new(),
            context,
        }
    }

    /// Add a validation rule
    pub fn add_rule(&mut self, rule: Arc<dyn ValidationRule>) {
        self.rules.push(rule);
        // Sort by priority (higher first)
        self.rules.sort_by_key(|r| std::cmp::Reverse(r.priority()));
    }

    /// Validate an action
    pub fn validate(&self, action: &Action, game: &Game) -> ValidationResult {
        // Run through all applicable rules
        for rule in &self.rules {
            if rule.applies_to(action) {
                let result = rule.validate(action, game, &self.context);
                if result.is_invalid() {
                    return result;
                }
            }
        }

        ValidationResult::valid()
    }

    /// Validate with custom context
    pub fn validate_with_context(
        &self,
        action: &Action,
        game: &Game,
        context: &ValidationContext,
    ) -> ValidationResult {
        for rule in &self.rules {
            if rule.applies_to(action) {
                let result = rule.validate(action, game, context);
                if result.is_invalid() {
                    return result;
                }
            }
        }

        ValidationResult::valid()
    }

    /// Get the number of registered rules
    pub fn rule_count(&self) -> usize {
        self.rules.len()
    }

    /// Clear all validation rules
    pub fn clear_rules(&mut self) {
        self.rules.clear();
    }
}

impl Default for ActionValidator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Mock validation rule for testing
    struct MockRule {
        name: String,
        should_pass: bool,
        priority: u32,
    }

    impl MockRule {
        fn new(name: &str, should_pass: bool) -> Self {
            Self {
                name: name.to_string(),
                should_pass,
                priority: 50,
            }
        }

        fn with_priority(mut self, priority: u32) -> Self {
            self.priority = priority;
            self
        }
    }

    impl ValidationRule for MockRule {
        fn name(&self) -> &str {
            &self.name
        }

        fn validate(
            &self,
            _action: &Action,
            _game: &Game,
            _context: &ValidationContext,
        ) -> ValidationResult {
            if self.should_pass {
                ValidationResult::valid()
            } else {
                ValidationResult::invalid(format!("{} validation failed", self.name))
            }
        }

        fn priority(&self) -> u32 {
            self.priority
        }
    }

    #[test]
    fn test_validator_with_passing_rules() {
        let mut validator = ActionValidator::new();
        validator.add_rule(Arc::new(MockRule::new("Rule1", true)));
        validator.add_rule(Arc::new(MockRule::new("Rule2", true)));

        let game = Game::default();
        let action = Action::NextRound();

        let result = validator.validate(&action, &game);
        assert!(result.is_valid());
    }

    #[test]
    fn test_validator_with_failing_rule() {
        let mut validator = ActionValidator::new();
        validator.add_rule(Arc::new(MockRule::new("PassingRule", true)));
        validator.add_rule(Arc::new(MockRule::new("FailingRule", false)));

        let game = Game::default();
        let action = Action::NextRound();

        let result = validator.validate(&action, &game);
        assert!(result.is_invalid());
        assert!(result.error().unwrap().reason().contains("FailingRule"));
    }

    #[test]
    fn test_validator_priority_ordering() {
        let mut validator = ActionValidator::new();

        // Add rules with different priorities
        validator.add_rule(Arc::new(
            MockRule::new("LowPriority", true).with_priority(10),
        ));
        validator.add_rule(Arc::new(
            MockRule::new("HighPriority", false).with_priority(100),
        ));
        validator.add_rule(Arc::new(
            MockRule::new("MediumPriority", true).with_priority(50),
        ));

        let game = Game::default();
        let action = Action::NextRound();

        let result = validator.validate(&action, &game);
        assert!(result.is_invalid());
        // High priority rule should fail first
        assert!(result.error().unwrap().reason().contains("HighPriority"));
    }

    #[test]
    fn test_validation_context() {
        let context = ValidationContext::new()
            .with_session("test-session".to_string())
            .with_strict_mode(true)
            .with_metadata("key".to_string(), "value".to_string());

        assert_eq!(context.session_id, Some("test-session".to_string()));
        assert!(context.strict_mode);
        assert_eq!(context.metadata.get("key"), Some(&"value".to_string()));
    }

    #[test]
    fn test_validator_clear_rules() {
        let mut validator = ActionValidator::new();
        validator.add_rule(Arc::new(MockRule::new("Rule1", true)));
        validator.add_rule(Arc::new(MockRule::new("Rule2", true)));

        assert_eq!(validator.rule_count(), 2);

        validator.clear_rules();
        assert_eq!(validator.rule_count(), 0);
    }
}
