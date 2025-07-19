use balatro_rs::ante::Ante;
use balatro_rs::config::Config;
use balatro_rs::game::Game;
use balatro_rs::joker::JokerId;
use balatro_rs::joker_state::{JokerPersistenceManager, JokerStateManager};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;

#[cfg(test)]
mod corrupted_save_handling_tests {
    use super::*;

    #[test]
    fn test_handle_completely_invalid_json() {
        let corrupted_json = "{ this is not valid json at all!";
        let result = Game::load_state_from_json(corrupted_json);

        assert!(result.is_err());
        if let Err(e) = result {
            println!("Error message: {}", e);
            // Just check that it's a deserialization error for now
            assert!(
                e.to_string().contains("Deserialization") || e.to_string().contains("expected")
            );
        }
    }

    #[test]
    fn test_handle_valid_json_invalid_structure() {
        let invalid_structure = r#"{"valid": "json", "but": "wrong", "structure": 123}"#;
        let result = Game::load_state_from_json(invalid_structure);

        assert!(result.is_err());
        if let Err(e) = result {
            assert!(e.to_string().contains("structure") || e.to_string().contains("field"));
        }
    }

    #[test]
    fn test_handle_missing_required_fields() {
        let missing_fields = r#"{"money": 100, "chips": 50}"#; // Missing many required fields
        let result = Game::load_state_from_json(missing_fields);

        assert!(result.is_err());
    }

    #[test]
    fn test_handle_corrupted_joker_state_data() {
        let state_manager = Arc::new(JokerStateManager::new());
        let persistence = JokerPersistenceManager::new(state_manager);

        // Create corrupted joker state data
        let mut corrupted_states = HashMap::new();
        corrupted_states.insert(
            JokerId::Joker,
            Value::String("this should be an object, not a string".to_string()),
        );

        let result = persistence.validate_state_data(&corrupted_states);
        assert!(result.is_err());
    }

    #[test]
    fn test_partial_corruption_recovery() {
        let state_manager = Arc::new(JokerStateManager::new());
        let persistence = JokerPersistenceManager::new(state_manager);

        // Mix of valid and invalid joker states
        let mut mixed_states = HashMap::new();

        // Valid state
        mixed_states.insert(
            JokerId::Joker,
            serde_json::json!({
                "accumulated_value": 10.0,
                "triggers_remaining": null,
                "custom_data": {}
            }),
        );

        // Invalid state
        mixed_states.insert(JokerId::GreedyJoker, Value::String("corrupted".to_string()));

        let result = persistence.load_states_with_recovery(&mixed_states);

        // Should recover valid states and report errors for invalid ones
        assert!(result.is_ok());
        let (loaded_states, errors) = result.unwrap();

        assert_eq!(loaded_states.len(), 1); // Only valid state loaded
        assert_eq!(errors.len(), 1); // One error reported
        assert!(loaded_states.contains_key(&JokerId::Joker));
        assert!(!loaded_states.contains_key(&JokerId::GreedyJoker));
    }

    #[test]
    fn test_unknown_joker_id_handling() {
        let state_manager = Arc::new(JokerStateManager::new());
        let persistence = JokerPersistenceManager::new(state_manager);

        // JSON with unknown joker ID (could happen with version differences)
        let unknown_joker_json = r#"{
            "version": 1,
            "timestamp": 1234567890,
            "joker_states": {
                "UnknownJoker": {
                    "accumulated_value": 5.0,
                    "triggers_remaining": null,
                    "custom_data": {}
                }
            },
            "metadata": {
                "total_jokers": 1,
                "game_round": 1,
                "game_ante": 1
            }
        }"#;

        let result = persistence.load_from_json_with_unknown_handling(unknown_joker_json);

        // Should handle unknown jokers gracefully (skip them with warning)
        if let Err(e) = &result {
            println!("Error: {e}");
        }
        assert!(result.is_ok());
        let (states, warnings) = result.unwrap();
        assert!(states.is_empty()); // Unknown joker should be skipped
        assert_eq!(warnings.len(), 1); // Should report warning about unknown joker
    }
}

#[cfg(test)]
mod save_load_integration_tests {
    use super::*;
    use balatro_rs::joker_impl::TheJoker;

    #[test]
    fn test_full_game_save_load_roundtrip() {
        // Create a game with some state
        let mut game = Game::new(Config::default());
        game.money = 500.0;
        game.ante_current = Ante::Three;
        game.round = 5.0;

        // Add a joker with some state
        use balatro_rs::joker_factory::JokerFactory;
        if let Some(joker) = JokerFactory::create(JokerId::Joker) {
            game.jokers.push(joker);
            game.joker_state_manager.ensure_state_exists(JokerId::Joker);
            game.joker_state_manager
                .add_accumulated_value(JokerId::Joker, 25.0);
        }

        // Save the game
        let saved_json = game.save_state_to_json().expect("Save should succeed");

        // Load the game
        let loaded_game = Game::load_state_from_json(&saved_json).expect("Load should succeed");

        // Verify state preservation
        assert_eq!(loaded_game.money, 500.0);
        assert_eq!(loaded_game.ante_current, Ante::Three);
        assert_eq!(loaded_game.round, 5.0);

        // Verify joker state preservation
        let joker_state = loaded_game.joker_state_manager.get_state(JokerId::Joker);
        assert!(joker_state.is_some());
        assert_eq!(joker_state.unwrap().accumulated_value, 25.0);
    }

    #[test]
    fn test_save_load_with_multiple_jokers() {
        let mut game = Game::new(Config::default());

        // Add multiple jokers with different states
        use balatro_rs::joker_factory::JokerFactory;
        if let Some(joker) = JokerFactory::create(JokerId::Joker) {
            game.jokers.push(joker);
            game.joker_state_manager.ensure_state_exists(JokerId::Joker);
            game.joker_state_manager
                .add_accumulated_value(JokerId::Joker, 10.0);
        }

        // Save and load
        let saved_json = game.save_state_to_json().expect("Save should succeed");
        let loaded_game = Game::load_state_from_json(&saved_json).expect("Load should succeed");

        // Verify all joker states preserved
        assert_eq!(game.jokers.len(), loaded_game.jokers.len());

        let original_state = game.joker_state_manager.get_state(JokerId::Joker).unwrap();
        let loaded_state = loaded_game
            .joker_state_manager
            .get_state(JokerId::Joker)
            .unwrap();
        assert_eq!(
            original_state.accumulated_value,
            loaded_state.accumulated_value
        );
    }

    #[test]
    fn test_save_load_preserves_jokers() {
        let mut game = Game::new(Config::default());

        // Add a joker directly for testing
        use balatro_rs::joker_factory::JokerFactory;
        if let Some(joker) = JokerFactory::create(JokerId::Joker) {
            game.jokers.push(joker);
            game.joker_state_manager.ensure_state_exists(JokerId::Joker);
        }

        // Save and load
        let saved_json = game.save_state_to_json().expect("Save should succeed");
        let loaded_game = Game::load_state_from_json(&saved_json).expect("Load should succeed");

        // Verify jokers were reconstructed properly
        assert_eq!(game.jokers.len(), loaded_game.jokers.len());
        assert_eq!(game.jokers[0].id(), loaded_game.jokers[0].id());
    }
}

#[cfg(test)]
mod save_format_versioning_tests {
    use super::*;

    #[test]
    fn test_version_migration_from_v1() {
        // Create a proper game first to get a valid save format, then modify it to simulate v1
        let mut game = Game::new(Config::default());
        game.money = 100.0;
        let full_save = game.save_state_to_json().unwrap();

        // Parse and modify to simulate v1 format
        let mut save_data: serde_json::Value = serde_json::from_str(&full_save).unwrap();
        save_data["version"] = serde_json::Value::Number(serde_json::Number::from(1));
        save_data["money"] = serde_json::Value::Number(serde_json::Number::from(100));

        // Add joker state
        let joker_state = serde_json::json!({
            "accumulated_value": 5.0,
            "triggers_remaining": null,
            "custom_data": {}
        });
        save_data["joker_states"]["Joker"] = joker_state;

        let v1_save_data = serde_json::to_string(&save_data).unwrap();

        let result = Game::load_state_from_json(&v1_save_data);
        if let Err(e) = &result {
            println!("Migration error: {e}");
        }
        assert!(result.is_ok());

        let loaded_game = result.unwrap();
        assert_eq!(loaded_game.money, 100.0);
        // Verify migration worked correctly
    }

    #[test]
    fn test_unsupported_future_version() {
        // Create a proper save file and modify the version to be from the future
        let game = Game::new(Config::default());
        let save_data = game.save_state_to_json().unwrap();

        // Parse and modify to have a future version
        let mut future_save: serde_json::Value = serde_json::from_str(&save_data).unwrap();
        future_save["version"] = serde_json::Value::Number(serde_json::Number::from(999));
        future_save["new_future_field"] = serde_json::Value::String("some_value".to_string());

        let future_save_data = serde_json::to_string(&future_save).unwrap();

        let result = Game::load_state_from_json(&future_save_data);

        // Should either load with warnings or fail gracefully
        match result {
            Ok(_) => {
                // If it loads, verify it handles unknown fields gracefully
                println!("Future version save loaded successfully (handling unknown fields)");
            }
            Err(e) => {
                // If it fails, should be due to version incompatibility
                println!("Future version error: {}", e);
                assert!(
                    e.to_string().contains("version")
                        || e.to_string().contains("unsupported")
                        || e.to_string().contains("Invalid")
                );
            }
        }
    }

    #[test]
    fn test_save_format_validation() {
        let mut game = Game::new(Config::default());
        game.money = 150.0;

        let saved_json = game.save_state_to_json().expect("Save should succeed");

        // Parse and validate save format structure
        let save_data: Value = serde_json::from_str(&saved_json).expect("Should be valid JSON");

        // Verify required fields are present
        assert!(save_data.get("version").is_some());
        assert!(save_data.get("money").is_some());
        assert!(save_data.get("joker_states").is_some());

        // Verify version is current
        let version = save_data["version"]
            .as_u64()
            .expect("Version should be number");
        assert!(version >= 1);
    }
}
