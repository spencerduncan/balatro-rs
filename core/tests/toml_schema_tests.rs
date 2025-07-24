use balatro_rs::joker_impl::{GreedyJoker, TheJoker};
use balatro_rs::joker_migration_tool::*;
use balatro_rs::joker_toml_parser::*;
use balatro_rs::joker_toml_schema::*;

/// Comprehensive tests for TOML schema system
#[cfg(test)]
mod toml_schema_integration_tests {
    use super::*;

    #[test]
    fn test_complete_basic_joker_workflow() {
        // Test the complete workflow: hardcoded -> TOML -> parsed -> validated

        // 1. Start with hardcoded joker
        let original_joker = TheJoker;

        // 2. Migrate to TOML
        let migration_tool = JokerMigrationTool::new();
        let definition = migration_tool
            .migrate_single_joker(&original_joker)
            .unwrap();

        // 3. Create config and serialize to TOML
        let config = JokerConfig {
            schema_version: "1.0.0".to_string(),
            jokers: vec![definition],
        };
        let toml_string = migration_tool.generate_toml(&config).unwrap();

        // 4. Parse back from TOML
        let parser = TomlJokerParser::new();
        let parsed_config = parser.parse_string(&toml_string).unwrap();

        // 5. Validate the round-trip
        assert_eq!(parsed_config.jokers.len(), 1);
        assert_eq!(parsed_config.jokers[0].name, "Joker");
        assert_eq!(parsed_config.jokers[0].id, "joker");

        // 6. Verify effect structure
        if let TomlJokerEffect::Scoring { mult, .. } = &parsed_config.jokers[0].effect {
            assert_eq!(*mult, 4);
        } else {
            panic!("Expected scoring effect");
        }
    }

    #[test]
    fn test_conditional_joker_workflow() {
        // Test conditional joker (Greedy Joker) workflow

        let original_joker = GreedyJoker;
        let migration_tool = JokerMigrationTool::new();
        let definition = migration_tool
            .migrate_single_joker(&original_joker)
            .unwrap();

        let config = JokerConfig {
            schema_version: "1.0.0".to_string(),
            jokers: vec![definition],
        };
        let toml_string = migration_tool.generate_toml(&config).unwrap();

        let parser = TomlJokerParser::new();
        let parsed_config = parser.parse_string(&toml_string).unwrap();

        assert_eq!(parsed_config.jokers[0].name, "Greedy Joker");

        if let TomlJokerEffect::Conditional {
            condition,
            action,
            per_card,
        } = &parsed_config.jokers[0].effect
        {
            assert!(*per_card);

            if let TomlJokerCondition::SuitScored { suit } = condition {
                assert!(matches!(suit, TomlSuit::Diamonds));
            } else {
                panic!("Expected suit condition");
            }

            if let TomlJokerAction::AddScore { mult, .. } = action {
                assert_eq!(*mult, 3);
            } else {
                panic!("Expected add score action");
            }
        } else {
            panic!("Expected conditional effect");
        }
    }

    #[test]
    fn test_validation_comprehensive() {
        let parser = TomlJokerParser::new();

        // Test all validation scenarios

        // 1. Schema version validation
        let invalid_version = r#"
            schema_version = "2.0.0"
            
            [[jokers]]
            id = "test"
            name = "Test"
            description = "Test joker"
            rarity = "common"
            
            [jokers.effect]
            type = "scoring"
            mult = 4
        "#;

        let result = parser.parse_string(invalid_version);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            TomlParserError::UnsupportedSchemaVersion(_)
        ));

        // 2. Duplicate ID validation
        let duplicate_ids = r#"
            schema_version = "1.0.0"
            
            [[jokers]]
            id = "test"
            name = "Test 1"
            description = "Test joker 1"
            rarity = "common"
            
            [jokers.effect]
            type = "scoring"
            mult = 4
            
            [[jokers]]
            id = "test"
            name = "Test 2"
            description = "Test joker 2"
            rarity = "common"
            
            [jokers.effect]
            type = "scoring"
            mult = 2
        "#;

        let result = parser.parse_string(duplicate_ids);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            TomlParserError::DuplicateJokerId(_)
        ));

        // 3. Range validation
        let invalid_range = r#"
            schema_version = "1.0.0"
            
            [[jokers]]
            id = "test"
            name = "Test"
            description = "Test joker"
            rarity = "common"
            cost = 2000
            
            [jokers.effect]
            type = "scoring"
            mult = 4
        "#;

        let result = parser.parse_string(invalid_range);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            TomlParserError::InvalidFieldValue { .. }
        ));
    }

    #[test]
    fn test_hot_reload_functionality() {
        let parser = TomlJokerParser::new();

        let original_toml = r#"
            schema_version = "1.0.0"
            
            [[jokers]]
            id = "test_joker"
            name = "Test Joker"
            description = "+4 Mult"
            rarity = "common"
            
            [jokers.effect]
            type = "scoring"
            mult = 4
        "#;

        let modified_toml = r#"
            schema_version = "1.0.0"
            
            [[jokers]]
            id = "test_joker"
            name = "Test Joker"
            description = "+5 Mult"
            rarity = "common"
            
            [jokers.effect]
            type = "scoring"
            mult = 5
            
            [[jokers]]
            id = "new_joker"
            name = "New Joker"
            description = "+2 Mult"
            rarity = "common"
            
            [jokers.effect]
            type = "scoring"
            mult = 2
        "#;

        let original_config = parser.parse_string(original_toml).unwrap();
        let (new_config, changed_ids) = parser
            .parse_with_hot_reload(modified_toml, Some(&original_config))
            .unwrap();

        // Should detect both the modified joker and the new joker
        assert_eq!(changed_ids.len(), 2);
        assert!(changed_ids.contains(&"test_joker".to_string()));
        assert!(changed_ids.contains(&"new_joker".to_string()));

        assert_eq!(new_config.jokers.len(), 2);
    }

    #[test]
    fn test_complex_joker_schema() {
        let parser = TomlJokerParser::new();

        let complex_toml = r#"
            schema_version = "1.0.0"
            
            [[jokers]]
            id = "ice_cream"
            name = "Ice Cream"
            description = "+100 Chips, -5 Chips per hand played"
            rarity = "common"
            cost = 3
            
            [jokers.effect]
            type = "dynamic"
            
            [jokers.effect.base_effect]
            type = "add_score"
            chips = 100
            
            [[jokers.effect.state_modifiers]]
            state_field = "hands_played"
            multiplier = -5.0
            
            [jokers.state]
            persistent = true
            
            [jokers.state.fields]
            hands_played = 0
            
            [jokers.behavior]
            
            [jokers.behavior.on_hand_played]
            type = "modify_state"
            field = "hands_played"
            operation = "increment"
            value = 1
        "#;

        let config = parser.parse_string(complex_toml).unwrap();
        assert_eq!(config.jokers.len(), 1);

        let joker = &config.jokers[0];
        assert_eq!(joker.name, "Ice Cream");

        // Verify dynamic effect structure
        if let TomlJokerEffect::Dynamic {
            base_effect,
            state_modifiers,
        } = &joker.effect
        {
            if let TomlJokerAction::AddScore { chips, .. } = base_effect {
                assert_eq!(*chips, 100);
            } else {
                panic!("Expected add score base effect");
            }

            assert_eq!(state_modifiers.len(), 1);
            assert_eq!(state_modifiers[0].state_field, "hands_played");
            assert_eq!(state_modifiers[0].multiplier, -5.0);
        } else {
            panic!("Expected dynamic effect");
        }

        // Verify state configuration
        assert!(joker.state.is_some());
        let state = joker.state.as_ref().unwrap();
        assert!(state.persistent);
        assert!(state.fields.contains_key("hands_played"));

        // Verify behavior configuration
        assert!(joker.behavior.is_some());
        let behavior = joker.behavior.as_ref().unwrap();
        assert!(behavior.on_hand_played.is_some());
    }

    #[test]
    fn test_composite_conditions() {
        let parser = TomlJokerParser::new();

        let composite_toml = r#"
            schema_version = "1.0.0"
            
            [[jokers]]
            id = "even_steven"
            name = "Even Steven"
            description = "Played cards with even rank give +4 Mult when scored"
            rarity = "common"
            
            [jokers.effect]
            type = "conditional"
            per_card = true
            
            [jokers.effect.condition]
            type = "any"
            conditions = [
                { type = "rank_scored", rank = "two" },
                { type = "rank_scored", rank = "four" },
                { type = "rank_scored", rank = "six" },
                { type = "rank_scored", rank = "eight" },
                { type = "rank_scored", rank = "ten" }
            ]
            
            [jokers.effect.action]
            type = "add_score"
            mult = 4
        "#;

        let config = parser.parse_string(composite_toml).unwrap();
        assert_eq!(config.jokers.len(), 1);

        let joker = &config.jokers[0];
        if let TomlJokerEffect::Conditional { condition, .. } = &joker.effect {
            if let TomlJokerCondition::Any { conditions } = condition {
                assert_eq!(conditions.len(), 5);

                // Verify all conditions are rank-based
                for cond in conditions {
                    assert!(matches!(cond, TomlJokerCondition::RankScored { .. }));
                }
            } else {
                panic!("Expected Any condition");
            }
        } else {
            panic!("Expected conditional effect");
        }
    }

    #[test]
    fn test_special_effect_jokers() {
        let parser = TomlJokerParser::new();

        let special_toml = r#"
            schema_version = "1.0.0"
            
            [[jokers]]
            id = "four_fingers"
            name = "Four Fingers"
            description = "All Flushes and Straights can be made with 4 cards"
            rarity = "uncommon"
            cost = 7
            
            [jokers.effect]
            type = "special"
            special_type = "hand_type_modifier"
            
            [jokers.effect.parameters]
            flush_requirement = 4
            straight_requirement = 4
        "#;

        let config = parser.parse_string(special_toml).unwrap();
        assert_eq!(config.jokers.len(), 1);

        let joker = &config.jokers[0];
        if let TomlJokerEffect::Special {
            special_type,
            parameters,
        } = &joker.effect
        {
            assert_eq!(special_type, "hand_type_modifier");
            assert!(parameters.contains_key("flush_requirement"));
            assert!(parameters.contains_key("straight_requirement"));
        } else {
            panic!("Expected special effect");
        }
    }

    #[test]
    fn test_migration_tool_custom_migrators() {
        let mut migration_tool = JokerMigrationTool::new();

        // Add custom migrator for Ice Cream
        migration_tool.add_custom_migrator(
            balatro_rs::joker::JokerId::IceCream,
            Box::new(IceCreamMigrator),
        );

        // Create a mock Ice Cream joker (this would need to be implemented)
        // For now, test that the migration tool accepts custom migrators
        assert_eq!(migration_tool.custom_migrator_count(), 1);
    }

    #[test]
    fn test_error_handling_comprehensive() {
        let parser = TomlJokerParser::new();

        // Test malformed TOML
        let malformed_toml = r#"
            schema_version = "1.0.0"
            
            [[jokers]
            id = "malformed"
            # Missing closing bracket
        "#;

        let result = parser.parse_string(malformed_toml);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), TomlParserError::TomlParse(_)));

        // Test missing required fields
        let missing_fields = r#"
            schema_version = "1.0.0"
            
            [[jokers]]
            id = "test"
            # Missing name, description, rarity, effect
        "#;

        let result = parser.parse_string(missing_fields);
        assert!(result.is_err());
        // This should fail during TOML deserialization due to missing required fields

        // Test empty ID
        let empty_id = r#"
            schema_version = "1.0.0"
            
            [[jokers]]
            id = ""
            name = "Test"
            description = "Test joker"
            rarity = "common"
            
            [jokers.effect]
            type = "scoring"
            mult = 4
        "#;

        let result = parser.parse_string(empty_id);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            TomlParserError::MissingRequiredField(_)
        ));
    }

    #[test]
    fn test_performance_large_config() {
        // Test parsing performance with a large number of jokers
        let mut toml_content = String::from(r#"schema_version = "1.0.0""#);
        toml_content.push('\n');

        // Generate 100 jokers
        for i in 0..100 {
            toml_content.push_str(&format!(
                r#"
                [[jokers]]
                id = "test_joker_{}"
                name = "Test Joker {}"
                description = "+{} Mult"
                rarity = "common"
                
                [jokers.effect]
                type = "scoring"
                mult = {}
            "#,
                i,
                i,
                i + 1,
                i + 1
            ));
        }

        let parser = TomlJokerParser::new();
        let start = std::time::Instant::now();
        let result = parser.parse_string(&toml_content);
        let duration = start.elapsed();

        assert!(result.is_ok());
        let config = result.unwrap();
        assert_eq!(config.jokers.len(), 100);

        // Should parse 100 jokers in under 100ms
        assert!(
            duration.as_millis() < 100,
            "Parsing took too long: {:?}",
            duration
        );
    }

    #[test]
    fn test_directory_parsing() {
        // This test would require setting up temporary files
        // For now, just test that the method exists and handles empty directories
        let parser = TomlJokerParser::new();

        // Create a temporary directory
        let temp_dir = std::env::temp_dir().join("toml_test");
        std::fs::create_dir_all(&temp_dir).unwrap();

        // Test parsing empty directory
        let result = parser.parse_directory(&temp_dir);
        assert!(result.is_ok());

        let config = result.unwrap();
        assert_eq!(config.jokers.len(), 0);

        // Cleanup
        std::fs::remove_dir(&temp_dir).unwrap();
    }
}
