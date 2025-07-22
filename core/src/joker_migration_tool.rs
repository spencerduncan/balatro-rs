use crate::joker::Joker;
use crate::joker::{JokerId, JokerRarity};
use crate::joker_toml_schema::{
    JokerConfig, TomlHandRank, TomlJokerAction, TomlJokerCondition, TomlJokerDefinition,
    TomlJokerEffect, TomlJokerRarity, TomlSuit,
};
use std::collections::HashMap;

/// Tool for migrating hardcoded jokers to TOML format
pub struct JokerMigrationTool {
    /// Custom migration rules for complex jokers
    custom_migrators: HashMap<JokerId, Box<dyn CustomJokerMigrator>>,
}

impl Default for JokerMigrationTool {
    fn default() -> Self {
        Self::new()
    }
}

impl JokerMigrationTool {
    /// Create a new migration tool
    pub fn new() -> Self {
        Self {
            custom_migrators: HashMap::new(),
        }
    }

    /// Add a custom migrator for a specific joker type
    pub fn add_custom_migrator(
        &mut self,
        joker_id: JokerId,
        migrator: Box<dyn CustomJokerMigrator>,
    ) {
        self.custom_migrators.insert(joker_id, migrator);
    }

    /// Get the number of custom migrators registered
    pub fn custom_migrator_count(&self) -> usize {
        self.custom_migrators.len()
    }

    /// Migrate a collection of jokers to TOML configuration
    pub fn migrate_jokers(&self, jokers: Vec<Box<dyn Joker>>) -> JokerConfig {
        let mut config = JokerConfig {
            schema_version: "1.0.0".to_string(),
            jokers: Vec::new(),
        };

        for joker in jokers {
            if let Some(definition) = self.migrate_single_joker(&*joker) {
                config.jokers.push(definition);
            }
        }

        config
    }

    /// Migrate a single joker to TOML definition
    pub fn migrate_single_joker(&self, joker: &dyn Joker) -> Option<TomlJokerDefinition> {
        let joker_id = joker.id();

        // Check for custom migrator first
        if let Some(migrator) = self.custom_migrators.get(&joker_id) {
            return migrator.migrate(joker);
        }

        // Use default migration strategy
        self.default_migrate(joker)
    }

    /// Default migration strategy for common joker patterns
    fn default_migrate(&self, joker: &dyn Joker) -> Option<TomlJokerDefinition> {
        let joker_id = joker.id();
        let name = joker.name();
        let description = joker.description();
        let rarity = self.convert_rarity(joker.rarity());
        let cost = Some(joker.cost());

        // Attempt to infer effect type from joker patterns
        let effect = self.infer_effect_type(joker)?;

        Some(TomlJokerDefinition {
            id: self.joker_id_to_string(joker_id),
            name: name.to_string(),
            description: description.to_string(),
            rarity,
            cost,
            effect,
            state: None,
            behavior: None,
        })
    }

    /// Convert JokerRarity to TomlJokerRarity
    fn convert_rarity(&self, rarity: JokerRarity) -> TomlJokerRarity {
        match rarity {
            JokerRarity::Common => TomlJokerRarity::Common,
            JokerRarity::Uncommon => TomlJokerRarity::Uncommon,
            JokerRarity::Rare => TomlJokerRarity::Rare,
            JokerRarity::Legendary => TomlJokerRarity::Legendary,
        }
    }

    /// Convert JokerId to string representation
    fn joker_id_to_string(&self, joker_id: JokerId) -> String {
        // Convert PascalCase enum variant to snake_case
        format!("{:?}", joker_id)
            .chars()
            .enumerate()
            .flat_map(|(i, c)| {
                if i > 0 && c.is_uppercase() {
                    vec!['_', c.to_lowercase().next().unwrap_or(c)]
                } else {
                    vec![c.to_lowercase().next().unwrap_or(c)]
                }
            })
            .collect()
    }

    /// Attempt to infer effect type based on joker patterns
    fn infer_effect_type(&self, joker: &dyn Joker) -> Option<TomlJokerEffect> {
        let joker_id = joker.id();
        let description = joker.description();

        // Pattern matching based on common joker types
        match joker_id {
            // Basic scoring jokers
            JokerId::Joker => Some(TomlJokerEffect::Scoring {
                chips: 0,
                mult: 4,
                money: 0,
                mult_multiplier: 1.0,
                per_card: false,
            }),

            // Suit-based conditional jokers
            JokerId::GreedyJoker => Some(TomlJokerEffect::Conditional {
                condition: TomlJokerCondition::SuitScored {
                    suit: TomlSuit::Diamonds,
                },
                action: TomlJokerAction::AddScore {
                    chips: 0,
                    mult: 3,
                    money: 0,
                    mult_multiplier: 1.0,
                },
                per_card: true,
            }),

            JokerId::LustyJoker => Some(TomlJokerEffect::Conditional {
                condition: TomlJokerCondition::SuitScored {
                    suit: TomlSuit::Hearts,
                },
                action: TomlJokerAction::AddScore {
                    chips: 0,
                    mult: 3,
                    money: 0,
                    mult_multiplier: 1.0,
                },
                per_card: true,
            }),

            JokerId::WrathfulJoker => Some(TomlJokerEffect::Conditional {
                condition: TomlJokerCondition::SuitScored {
                    suit: TomlSuit::Spades,
                },
                action: TomlJokerAction::AddScore {
                    chips: 0,
                    mult: 3,
                    money: 0,
                    mult_multiplier: 1.0,
                },
                per_card: true,
            }),

            JokerId::GluttonousJoker => Some(TomlJokerEffect::Conditional {
                condition: TomlJokerCondition::SuitScored {
                    suit: TomlSuit::Clubs,
                },
                action: TomlJokerAction::AddScore {
                    chips: 0,
                    mult: 3,
                    money: 0,
                    mult_multiplier: 1.0,
                },
                per_card: true,
            }),

            // Hand type conditional jokers
            JokerId::JollyJoker => Some(TomlJokerEffect::Conditional {
                condition: TomlJokerCondition::HandType {
                    hand_type: TomlHandRank::Pair,
                },
                action: TomlJokerAction::AddScore {
                    chips: 0,
                    mult: 8,
                    money: 0,
                    mult_multiplier: 1.0,
                },
                per_card: false,
            }),

            // For unknown jokers, try to infer from description
            _ => self.infer_from_description(description),
        }
    }

    /// Attempt to infer effect from description text
    fn infer_from_description(&self, description: &str) -> Option<TomlJokerEffect> {
        let desc_lower = description.to_lowercase();

        // Look for simple scoring patterns
        if let Some(mult) = self.extract_simple_mult(&desc_lower) {
            return Some(TomlJokerEffect::Scoring {
                chips: 0,
                mult,
                money: 0,
                mult_multiplier: 1.0,
                per_card: false,
            });
        }

        if let Some(chips) = self.extract_simple_chips(&desc_lower) {
            return Some(TomlJokerEffect::Scoring {
                chips,
                mult: 0,
                money: 0,
                mult_multiplier: 1.0,
                per_card: false,
            });
        }

        // Look for suit-based patterns
        if let Some(suit) = self.extract_suit_condition(&desc_lower) {
            if let Some(mult) = self.extract_simple_mult(&desc_lower) {
                return Some(TomlJokerEffect::Conditional {
                    condition: TomlJokerCondition::SuitScored { suit },
                    action: TomlJokerAction::AddScore {
                        chips: 0,
                        mult,
                        money: 0,
                        mult_multiplier: 1.0,
                    },
                    per_card: true,
                });
            }
        }

        // Look for hand type patterns
        if let Some(hand_type) = self.extract_hand_type_condition(&desc_lower) {
            if let Some(mult) = self.extract_simple_mult(&desc_lower) {
                return Some(TomlJokerEffect::Conditional {
                    condition: TomlJokerCondition::HandType { hand_type },
                    action: TomlJokerAction::AddScore {
                        chips: 0,
                        mult,
                        money: 0,
                        mult_multiplier: 1.0,
                    },
                    per_card: false,
                });
            }
        }

        // Default to special effect for complex jokers
        Some(TomlJokerEffect::Special {
            special_type: "custom".to_string(),
            parameters: HashMap::new(),
        })
    }

    /// Extract simple mult bonus from description
    fn extract_simple_mult(&self, description: &str) -> Option<i32> {
        // Look for patterns like "+4 Mult", "+8 mult", etc.
        if let Some(caps) = regex::Regex::new(r"\+(\d+)\s*mult")
            .ok()?
            .captures(description)
        {
            caps[1].parse().ok()
        } else {
            None
        }
    }

    /// Extract simple chips bonus from description
    fn extract_simple_chips(&self, description: &str) -> Option<i32> {
        // Look for patterns like "+100 Chips", "+30 chips", etc.
        if let Some(caps) = regex::Regex::new(r"\+(\d+)\s*chips")
            .ok()?
            .captures(description)
        {
            caps[1].parse().ok()
        } else {
            None
        }
    }

    /// Extract suit condition from description
    fn extract_suit_condition(&self, description: &str) -> Option<TomlSuit> {
        if description.contains("diamond") {
            Some(TomlSuit::Diamonds)
        } else if description.contains("heart") {
            Some(TomlSuit::Hearts)
        } else if description.contains("spade") {
            Some(TomlSuit::Spades)
        } else if description.contains("club") {
            Some(TomlSuit::Clubs)
        } else {
            None
        }
    }

    /// Extract hand type condition from description
    fn extract_hand_type_condition(&self, description: &str) -> Option<TomlHandRank> {
        if description.contains("pair") {
            Some(TomlHandRank::Pair)
        } else if description.contains("two pair") {
            Some(TomlHandRank::TwoPair)
        } else if description.contains("three of a kind") {
            Some(TomlHandRank::ThreeOfAKind)
        } else if description.contains("straight flush") {
            Some(TomlHandRank::StraightFlush)
        } else if description.contains("royal flush") {
            Some(TomlHandRank::RoyalFlush)
        } else if description.contains("flush") {
            Some(TomlHandRank::Flush)
        } else if description.contains("straight") {
            Some(TomlHandRank::Straight)
        } else if description.contains("full house") {
            Some(TomlHandRank::FullHouse)
        } else if description.contains("four of a kind") {
            Some(TomlHandRank::FourOfAKind)
        } else {
            None
        }
    }

    /// Generate TOML string from configuration
    pub fn generate_toml(&self, config: &JokerConfig) -> Result<String, toml::ser::Error> {
        toml::to_string_pretty(config)
    }

    /// Save configuration to file
    pub fn save_to_file<P: AsRef<std::path::Path>>(
        &self,
        config: &JokerConfig,
        path: P,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let toml_content = self.generate_toml(config)?;
        std::fs::write(path, toml_content)?;
        Ok(())
    }
}

/// Trait for custom joker migration strategies
pub trait CustomJokerMigrator: Send + Sync {
    fn migrate(&self, joker: &dyn Joker) -> Option<TomlJokerDefinition>;
}

/// Example custom migrator for Ice Cream joker
pub struct IceCreamMigrator;

impl CustomJokerMigrator for IceCreamMigrator {
    fn migrate(&self, joker: &dyn Joker) -> Option<TomlJokerDefinition> {
        if joker.id() != JokerId::IceCream {
            return None;
        }

        Some(TomlJokerDefinition {
            id: "ice_cream".to_string(),
            name: joker.name().to_string(),
            description: joker.description().to_string(),
            rarity: TomlJokerRarity::Common,
            cost: Some(joker.cost()),
            effect: TomlJokerEffect::Dynamic {
                base_effect: TomlJokerAction::AddScore {
                    chips: 100,
                    mult: 0,
                    money: 0,
                    mult_multiplier: 1.0,
                },
                state_modifiers: vec![crate::joker_toml_schema::TomlStateModifier {
                    state_field: "hands_played".to_string(),
                    multiplier: -5.0,
                }],
            },
            state: Some(crate::joker_toml_schema::TomlJokerState {
                fields: {
                    let mut fields = HashMap::new();
                    fields.insert(
                        "hands_played".to_string(),
                        crate::joker_toml_schema::TomlValue::Integer(0),
                    );
                    fields
                },
                persistent: true,
            }),
            behavior: Some(crate::joker_toml_schema::TomlJokerBehavior {
                on_hand_played: Some(TomlJokerAction::ModifyState {
                    field: "hands_played".to_string(),
                    operation: crate::joker_toml_schema::TomlStateOperation::Increment,
                    value: crate::joker_toml_schema::TomlValue::Integer(1),
                }),
                on_card_scored: None,
                on_blind_start: None,
                on_shop_open: None,
                on_discard: None,
                on_round_end: None,
                on_created: None,
                on_activated: None,
                on_deactivated: None,
                on_cleanup: None,
            }),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::joker_impl::{GreedyJoker, TheJoker};

    #[test]
    fn test_migrate_basic_joker() {
        let migration_tool = JokerMigrationTool::new();
        let joker = TheJoker;

        let definition = migration_tool.migrate_single_joker(&joker).unwrap();

        assert_eq!(definition.id, "joker");
        assert_eq!(definition.name, "Joker");

        if let TomlJokerEffect::Scoring { mult, .. } = definition.effect {
            assert_eq!(mult, 4);
        } else {
            panic!("Expected scoring effect");
        }
    }

    #[test]
    fn test_migrate_conditional_joker() {
        let migration_tool = JokerMigrationTool::new();
        let joker = GreedyJoker;

        let definition = migration_tool.migrate_single_joker(&joker).unwrap();

        assert_eq!(definition.id, "greedy_joker");
        assert_eq!(definition.name, "Greedy Joker");

        if let TomlJokerEffect::Conditional {
            condition,
            action,
            per_card,
        } = definition.effect
        {
            assert!(per_card);

            if let TomlJokerCondition::SuitScored { suit } = condition {
                assert!(matches!(suit, TomlSuit::Diamonds));
            } else {
                panic!("Expected suit condition");
            }

            if let TomlJokerAction::AddScore { mult, .. } = action {
                assert_eq!(mult, 3);
            } else {
                panic!("Expected add score action");
            }
        } else {
            panic!("Expected conditional effect");
        }
    }

    #[test]
    fn test_generate_toml_output() {
        let migration_tool = JokerMigrationTool::new();
        let jokers: Vec<Box<dyn Joker>> = vec![Box::new(TheJoker), Box::new(GreedyJoker)];

        let config = migration_tool.migrate_jokers(jokers);
        let toml_output = migration_tool.generate_toml(&config).unwrap();

        assert!(toml_output.contains("schema_version = \"1.0.0\""));
        assert!(toml_output.contains("id = \"joker\""));
        assert!(toml_output.contains("id = \"greedy_joker\""));
    }
}
