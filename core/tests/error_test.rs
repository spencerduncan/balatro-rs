use balatro_rs::error::{
    ActionSpaceError,
    DeveloperActionSpaceError,
    DeveloperGameError,
    DeveloperPlayHandError,
    ErrorDetailLevel,
    ErrorSanitizer,
    GameError,
    PlayHandError, // Backward compatibility aliases
    UserError,
};
use std::error::Error;

#[cfg(test)]
mod play_hand_error_tests {
    use super::*;

    #[test]
    fn test_play_hand_error_display() {
        assert_eq!(
            format!("{}", PlayHandError::TooManyCards),
            "Played hand contains more than 5 cards"
        );
        assert_eq!(
            format!("{}", PlayHandError::NoCards),
            "Played hand contains no cards"
        );
        assert_eq!(
            format!("{}", PlayHandError::UnknownHand),
            "Played hand could not determine best hand"
        );
    }

    #[test]
    fn test_play_hand_error_debug() {
        assert_eq!(format!("{:?}", PlayHandError::TooManyCards), "TooManyCards");
        assert_eq!(format!("{:?}", PlayHandError::NoCards), "NoCards");
        assert_eq!(format!("{:?}", PlayHandError::UnknownHand), "UnknownHand");
    }

    #[test]
    fn test_play_hand_error_clone() {
        let error = PlayHandError::TooManyCards;
        let cloned_error = error.clone();
        assert_eq!(format!("{}", error), format!("{}", cloned_error));
    }

    #[test]
    fn test_play_hand_error_string_equality() {
        // Test that the same error types produce the same string representation
        let error1 = PlayHandError::TooManyCards;
        let error2 = PlayHandError::TooManyCards;
        assert_eq!(format!("{}", error1), format!("{}", error2));
    }

    #[test]
    fn test_play_hand_error_as_error() {
        let error = PlayHandError::TooManyCards;
        let error_trait: &dyn Error = &error;
        assert_eq!(
            error_trait.to_string(),
            "Played hand contains more than 5 cards"
        );
    }

    #[test]
    fn test_play_hand_error_source() {
        let error = PlayHandError::UnknownHand;
        assert!(error.source().is_none());
    }
}

#[cfg(test)]
mod action_space_error_tests {
    use super::*;

    #[test]
    fn test_action_space_error_display() {
        assert_eq!(
            format!("{}", ActionSpaceError::InvalidIndex),
            "Invalid index"
        );
        assert_eq!(
            format!("{}", ActionSpaceError::InvalidActionConversion),
            "Invalid conversion to action"
        );
        assert_eq!(
            format!("{}", ActionSpaceError::MaskedAction),
            "Masked action"
        );
    }

    #[test]
    fn test_action_space_error_debug() {
        assert_eq!(
            format!("{:?}", ActionSpaceError::InvalidIndex),
            "InvalidIndex"
        );
        assert_eq!(
            format!("{:?}", ActionSpaceError::InvalidActionConversion),
            "InvalidActionConversion"
        );
        assert_eq!(
            format!("{:?}", ActionSpaceError::MaskedAction),
            "MaskedAction"
        );
    }

    #[test]
    fn test_action_space_error_clone() {
        let error = ActionSpaceError::InvalidIndex;
        let cloned_error = error.clone();
        assert_eq!(format!("{}", error), format!("{}", cloned_error));
    }

    #[test]
    fn test_action_space_error_string_equality() {
        // Test that the same error types produce the same string representation
        let error1 = ActionSpaceError::InvalidIndex;
        let error2 = ActionSpaceError::InvalidIndex;
        assert_eq!(format!("{}", error1), format!("{}", error2));
    }

    #[test]
    fn test_action_space_error_as_error() {
        let error = ActionSpaceError::InvalidActionConversion;
        let error_trait: &dyn Error = &error;
        assert_eq!(error_trait.to_string(), "Invalid conversion to action");
    }
}

#[cfg(test)]
mod game_error_tests {
    use super::*;

    #[test]
    fn test_game_error_simple_variants() {
        assert_eq!(
            format!("{}", GameError::NoRemainingDiscards),
            "No remaining discards"
        );
        assert_eq!(
            format!("{}", GameError::NoRemainingPlays),
            "No remaining plays"
        );
        assert_eq!(format!("{}", GameError::InvalidStage), "Invalid stage");
        assert_eq!(format!("{}", GameError::InvalidAction), "Invalid action");
        assert_eq!(format!("{}", GameError::InvalidBlind), "No blind match");
        assert_eq!(format!("{}", GameError::NoCardMatch), "No card match");
        assert_eq!(format!("{}", GameError::NoJokerMatch), "No joker match");
        assert_eq!(
            format!("{}", GameError::InvalidMoveDirection),
            "Invalid move direction"
        );
        assert_eq!(
            format!("{}", GameError::NoAvailableSlot),
            "No available slot"
        );
        assert_eq!(format!("{}", GameError::InvalidBalance), "Invalid balance");
        assert_eq!(
            format!("{}", GameError::InvalidMoveCard),
            "Invalid move card"
        );
        assert_eq!(
            format!("{}", GameError::InvalidSelectCard),
            "Invalid select card"
        );
        assert_eq!(
            format!("{}", GameError::InvalidActionSpace),
            "Invalid action space"
        );
        assert_eq!(format!("{}", GameError::InvalidSlot), "Invalid slot index");
        assert_eq!(
            format!("{}", GameError::JokerNotInShop),
            "Joker not available in shop"
        );
        assert_eq!(format!("{}", GameError::MutexPoisoned), "Mutex poisoned");
    }

    #[test]
    fn test_game_error_with_data() {
        let joker_not_found = GameError::JokerNotFound("TestJoker".to_string());
        assert_eq!(format!("{}", joker_not_found), "Joker not found: TestJoker");

        let invalid_operation = GameError::InvalidOperation("test operation".to_string());
        assert_eq!(
            format!("{}", invalid_operation),
            "Invalid operation: test operation"
        );
    }

    #[test]
    fn test_game_error_from_play_hand_error() {
        let play_hand_error = PlayHandError::TooManyCards;
        let game_error = GameError::from(play_hand_error);

        match game_error {
            GameError::InvalidHand(inner) => {
                // Test that the inner error produces the expected string
                assert_eq!(
                    format!("{}", inner),
                    "Played hand contains more than 5 cards"
                );
            }
            _ => panic!("Expected InvalidHand variant"),
        }
    }

    #[test]
    fn test_game_error_from_action_space_error() {
        let action_space_error = ActionSpaceError::InvalidIndex;
        let game_error = GameError::from(action_space_error);

        // Test that the conversion produces the expected error message
        assert_eq!(format!("{}", game_error), "Invalid action space");
    }

    #[test]
    fn test_game_error_chain_from_play_hand_error() {
        let play_hand_error = PlayHandError::NoCards;
        let game_error: GameError = play_hand_error.into();

        if let GameError::InvalidHand(inner) = game_error {
            assert_eq!(format!("{}", inner), "Played hand contains no cards");
        } else {
            panic!("Expected InvalidHand variant");
        }
    }

    #[test]
    fn test_game_error_debug() {
        assert_eq!(format!("{:?}", GameError::InvalidAction), "InvalidAction");

        let joker_not_found = GameError::JokerNotFound("test".to_string());
        assert_eq!(format!("{:?}", joker_not_found), "JokerNotFound(\"test\")");
    }

    #[test]
    fn test_game_error_clone() {
        let error = GameError::InvalidBalance;
        let cloned_error = error.clone();
        assert_eq!(format!("{}", error), format!("{}", cloned_error));
    }

    #[test]
    fn test_game_error_string_representation() {
        // Test string representations are consistent
        let error1 = GameError::InvalidAction;
        let error2 = GameError::InvalidAction;
        assert_eq!(format!("{}", error1), format!("{}", error2));

        let joker1 = GameError::JokerNotFound("joker1".to_string());
        let joker2 = GameError::JokerNotFound("joker1".to_string());
        let joker3 = GameError::JokerNotFound("joker2".to_string());

        assert_eq!(format!("{}", joker1), format!("{}", joker2));
        assert_ne!(format!("{}", joker1), format!("{}", joker3));
    }

    #[test]
    fn test_game_error_as_error() {
        let error = GameError::NoRemainingPlays;
        let error_trait: &dyn Error = &error;
        assert_eq!(error_trait.to_string(), "No remaining plays");
    }

    #[test]
    fn test_game_error_source() {
        // Simple errors should have no source
        let simple_error = GameError::InvalidAction;
        assert!(simple_error.source().is_none());

        // Errors with inner errors should have a source
        let inner_error = PlayHandError::TooManyCards;
        let wrapped_error = GameError::InvalidHand(inner_error);
        assert!(wrapped_error.source().is_some());
    }

    #[test]
    fn test_all_game_error_variants() {
        // Test that all variants can be constructed and displayed
        let errors = vec![
            GameError::NoRemainingDiscards,
            GameError::NoRemainingPlays,
            GameError::InvalidHand(PlayHandError::TooManyCards),
            GameError::InvalidStage,
            GameError::InvalidAction,
            GameError::InvalidBlind,
            GameError::NoCardMatch,
            GameError::NoJokerMatch,
            GameError::InvalidMoveDirection,
            GameError::NoAvailableSlot,
            GameError::InvalidBalance,
            GameError::InvalidMoveCard,
            GameError::InvalidSelectCard,
            GameError::InvalidActionSpace,
            GameError::InvalidSlot,
            GameError::JokerNotInShop,
            GameError::JokerNotFound("test".to_string()),
            GameError::InvalidOperation("test".to_string()),
            GameError::MutexPoisoned,
        ];

        for error in errors {
            let _ = format!("{}", error);
            let _ = format!("{:?}", error);
        }
    }
}

#[cfg(test)]
mod error_conversion_tests {
    use super::*;

    #[test]
    fn test_play_hand_error_to_game_error() {
        let play_hand_errors = vec![
            PlayHandError::TooManyCards,
            PlayHandError::NoCards,
            PlayHandError::UnknownHand,
        ];

        for play_error in play_hand_errors {
            let expected_message = format!("{}", play_error);
            let game_error: GameError = play_error.into();
            match game_error {
                GameError::InvalidHand(inner) => {
                    assert_eq!(format!("{}", inner), expected_message);
                }
                _ => panic!("Expected InvalidHand variant"),
            }
        }
    }

    #[test]
    fn test_action_space_error_to_game_error() {
        let action_space_errors = vec![
            ActionSpaceError::InvalidIndex,
            ActionSpaceError::InvalidActionConversion,
            ActionSpaceError::MaskedAction,
        ];

        for action_error in action_space_errors {
            let game_error: GameError = action_error.into();
            assert_eq!(format!("{}", game_error), "Invalid action space");
        }
    }
}

#[cfg(all(test, feature = "python"))]
mod python_error_tests {
    use super::*;
    use pyo3::PyErr;

    #[test]
    fn test_game_error_to_py_err() {
        let game_error = GameError::InvalidAction;
        let py_err: PyErr = game_error.into();

        // Verify it's a PyException with sanitized message (security improvement)
        let error_string = py_err.to_string();
        assert!(error_string.contains("Invalid game state"));
    }

    #[test]
    fn test_complex_game_error_to_py_err() {
        let game_error = GameError::JokerNotFound("TestJoker".to_string());
        let py_err: PyErr = game_error.into();

        // The error should NOT contain the joker name for security (information disclosure prevention)
        let error_string = py_err.to_string();
        assert!(!error_string.contains("TestJoker"));
        assert!(error_string.contains("Resource not found"));
    }

    #[test]
    fn test_nested_error_to_py_err() {
        let play_hand_error = PlayHandError::TooManyCards;
        let game_error = GameError::InvalidHand(play_hand_error);
        let py_err: PyErr = game_error.into();

        // Should use sanitized message, not detailed error info
        let error_string = py_err.to_string();
        assert!(error_string.contains("Invalid game state"));
        assert!(!error_string.contains("Invalid hand played"));
        assert!(!error_string.contains("more than 5 cards"));
    }
}

#[cfg(test)]
mod comprehensive_error_tests {
    use super::*;

    #[test]
    fn test_error_chain() {
        // Test a complete error chain: PlayHandError -> GameError -> String
        let original_error = PlayHandError::UnknownHand;
        let game_error: GameError = original_error.into();
        let error_string = game_error.to_string();

        assert!(error_string.contains("Invalid hand played"));
    }

    #[test]
    fn test_error_downcasting() {
        let game_error = GameError::InvalidHand(PlayHandError::NoCards);

        if let GameError::InvalidHand(inner) = game_error {
            assert_eq!(format!("{}", inner), "Played hand contains no cards");
        } else {
            panic!("Error downcasting failed");
        }
    }

    #[test]
    fn test_error_memory_safety() {
        // Test that errors with owned data (String) work correctly
        let long_string = "a".repeat(1000);
        let error = GameError::JokerNotFound(long_string.clone());

        assert!(format!("{}", error).contains(&long_string));

        // Test cloning with owned data
        let cloned_error = error.clone();
        assert_eq!(format!("{}", error), format!("{}", cloned_error));
    }

    #[test]
    fn test_error_display_consistency() {
        // Verify that Display and Debug are consistent for simple variants
        let simple_errors = vec![
            GameError::InvalidAction,
            GameError::InvalidBalance,
            GameError::MutexPoisoned,
        ];

        for error in simple_errors {
            let display_str = format!("{}", error);
            let debug_str = format!("{:?}", error);

            // Both should be non-empty
            assert!(!display_str.is_empty());
            assert!(!debug_str.is_empty());
        }
    }
}

#[cfg(test)]
mod security_error_tests {
    use super::*;

    #[test]
    fn test_user_error_messages_are_generic() {
        // Verify that UserError messages are generic and don't expose system details
        let user_errors = vec![
            UserError::InvalidInput,
            UserError::InvalidOperation,
            UserError::NotFound,
            UserError::OperationFailed,
            UserError::InvalidState,
            UserError::SystemError,
        ];

        for error in user_errors {
            let message = format!("{}", error);

            // Ensure messages are generic and don't contain sensitive keywords
            assert!(!message.to_lowercase().contains("internal"));
            assert!(!message.to_lowercase().contains("debug"));
            assert!(!message.to_lowercase().contains("stack"));
            assert!(!message.to_lowercase().contains("path"));
            assert!(!message.to_lowercase().contains("file"));
            assert!(!message.to_lowercase().contains("line"));

            // Special check for SystemError - it can contain "system" but should be generic
            if let UserError::SystemError = error {
                assert_eq!(message, "System error occurred");
            } else {
                assert!(!message.to_lowercase().contains("system"));
            }

            // Messages should be short and generic
            assert!(message.len() < 50, "Error message too long: {}", message);
        }
    }

    #[test]
    fn test_error_detail_level_default() {
        // Verify that default detail level is appropriate for build type
        let default_level = ErrorDetailLevel::default();

        #[cfg(debug_assertions)]
        assert_eq!(default_level, ErrorDetailLevel::Development);

        #[cfg(not(debug_assertions))]
        assert_eq!(default_level, ErrorDetailLevel::Production);
    }

    #[test]
    fn test_error_sanitizer_production_mode() {
        let sanitizer = ErrorSanitizer::new(ErrorDetailLevel::Production);

        // Test various developer errors get sanitized to generic user errors
        let test_cases = vec![
            (
                DeveloperGameError::JokerNotFound("SecretJoker".to_string()),
                UserError::NotFound,
            ),
            (
                DeveloperGameError::InvalidInput("sensitive_data".to_string()),
                UserError::InvalidInput,
            ),
            (
                DeveloperGameError::InvalidOperation("internal_state".to_string()),
                UserError::InvalidOperation,
            ),
            (DeveloperGameError::NoCardMatch, UserError::NotFound),
            (DeveloperGameError::InvalidStage, UserError::InvalidState),
            (DeveloperGameError::MutexPoisoned, UserError::SystemError),
        ];

        for (dev_error, expected_user_error) in test_cases {
            let sanitized = sanitizer.sanitize_game_error(&dev_error);
            assert_eq!(format!("{}", sanitized), format!("{}", expected_user_error));
        }
    }

    #[test]
    fn test_error_sanitizer_development_mode() {
        let sanitizer = ErrorSanitizer::new(ErrorDetailLevel::Development);

        // Development mode should still sanitize but can be slightly more specific
        let test_cases = vec![
            (
                DeveloperGameError::NoRemainingDiscards,
                UserError::OperationFailed,
            ),
            (
                DeveloperGameError::NoRemainingPlays,
                UserError::OperationFailed,
            ),
            (
                DeveloperGameError::JokerNotFound("TestJoker".to_string()),
                UserError::NotFound,
            ),
            (DeveloperGameError::InvalidStage, UserError::InvalidState),
        ];

        for (dev_error, expected_user_error) in test_cases {
            let sanitized = sanitizer.sanitize_game_error(&dev_error);
            assert_eq!(format!("{}", sanitized), format!("{}", expected_user_error));
        }
    }

    #[test]
    fn test_error_sanitizer_no_information_disclosure() {
        let sanitizer = ErrorSanitizer::new(ErrorDetailLevel::Production);

        // Test that sensitive information is not disclosed in sanitized errors
        let sensitive_errors = vec![
            DeveloperGameError::JokerNotFound("/path/to/secret/file".to_string()),
            DeveloperGameError::InvalidInput("password=secret123".to_string()),
            DeveloperGameError::InvalidOperation("user_id=12345".to_string()),
            DeveloperGameError::RngFailed("seed=0x12345678".to_string()),
            DeveloperGameError::HandAnalysisFailed("stack_trace_info".to_string()),
        ];

        for sensitive_error in sensitive_errors {
            let sanitized = sanitizer.sanitize_game_error(&sensitive_error);
            let sanitized_msg = format!("{}", sanitized);

            // Ensure no sensitive data leaks through
            assert!(!sanitized_msg.contains("secret"));
            assert!(!sanitized_msg.contains("password"));
            assert!(!sanitized_msg.contains("user_id"));
            assert!(!sanitized_msg.contains("seed"));
            assert!(!sanitized_msg.contains("stack_trace"));
            assert!(!sanitized_msg.contains("/path/"));
            assert!(!sanitized_msg.contains("0x"));
        }
    }

    #[test]
    fn test_developer_error_preserve_detail() {
        // Verify that developer errors preserve all necessary detail for debugging
        let detailed_errors = vec![
            DeveloperGameError::JokerNotFound("SpecificJokerName".to_string()),
            DeveloperGameError::InvalidInput("specific validation failure".to_string()),
            DeveloperGameError::RngFailed("random seed exhausted".to_string()),
        ];

        for error in detailed_errors {
            let error_msg = format!("{}", error);

            // Developer errors should contain specific details
            match error {
                DeveloperGameError::JokerNotFound(ref name) => {
                    assert!(error_msg.contains(name));
                }
                DeveloperGameError::InvalidInput(ref details) => {
                    assert!(error_msg.contains(details));
                }
                DeveloperGameError::RngFailed(ref reason) => {
                    assert!(error_msg.contains(reason));
                }
                _ => {}
            }
        }
    }

    #[test]
    fn test_backward_compatibility_aliases() {
        // Ensure backward compatibility aliases work correctly
        let play_error: PlayHandError = DeveloperPlayHandError::TooManyCards;
        let game_error: GameError = DeveloperGameError::InvalidAction;
        let action_error: ActionSpaceError = DeveloperActionSpaceError::InvalidIndex;

        // Should be able to format them
        let _ = format!("{}", play_error);
        let _ = format!("{}", game_error);
        let _ = format!("{}", action_error);

        // Should be able to convert between them
        let converted_game_error: GameError = play_error.into();
        let _ = format!("{}", converted_game_error);
    }
}

#[cfg(all(test, feature = "python"))]
mod python_security_tests {
    use super::*;
    use pyo3::PyErr;

    #[test]
    fn test_python_bindings_use_sanitized_errors() {
        // Test that Python bindings automatically sanitize errors for security
        let sensitive_error = DeveloperGameError::JokerNotFound("internal_system_path".to_string());
        let py_err: PyErr = sensitive_error.into();

        let error_string = py_err.to_string();

        // Should not contain the sensitive path information
        assert!(!error_string.contains("internal_system_path"));

        // Should contain a generic message instead
        assert!(
            error_string.contains("Resource not found")
                || error_string.contains("not found")
                || error_string.contains("NotFound")
        );
    }

    #[test]
    fn test_user_error_to_python() {
        let user_error = UserError::InvalidInput;
        let py_err: PyErr = user_error.into();

        let error_string = py_err.to_string();
        assert!(error_string.contains("Invalid input provided"));
    }

    #[test]
    fn test_python_error_no_stack_traces() {
        // Ensure that Python errors don't expose Rust stack traces
        let errors = vec![
            DeveloperGameError::MutexPoisoned,
            DeveloperGameError::EmptyIterator,
            DeveloperGameError::RngFailed("internal failure".to_string()),
        ];

        for error in errors {
            let py_err: PyErr = error.into();
            let error_string = py_err.to_string();

            // Should not contain stack trace keywords
            assert!(!error_string.contains("backtrace"));
            assert!(!error_string.contains("frame"));
            assert!(!error_string.contains("panic"));
            assert!(!error_string.contains("unwrap"));
            assert!(!error_string.contains("expect"));
        }
    }
}

#[cfg(test)]
mod error_security_compliance_tests {
    use super::*;

    #[test]
    fn test_no_sensitive_data_in_user_errors() {
        // Comprehensive test to ensure no sensitive data patterns in user errors
        let all_user_errors = vec![
            UserError::InvalidInput,
            UserError::InvalidOperation,
            UserError::NotFound,
            UserError::OperationFailed,
            UserError::InvalidState,
            UserError::SystemError,
        ];

        let forbidden_patterns = vec![
            "password",
            "token",
            "key",
            "secret",
            "private",
            "internal",
            "debug",
            "trace",
            "stack",
            "frame",
            "file",
            "path",
            "directory",
            "home",
            "user",
            "database",
            "sql",
            "query",
            "connection",
            "0x",
            ":::",
            "src/",
            "lib.rs",
            ".rs",
        ];

        for error in all_user_errors {
            let error_msg = format!("{}", error).to_lowercase();

            for pattern in &forbidden_patterns {
                assert!(
                    !error_msg.contains(pattern),
                    "User error '{}' contains forbidden pattern '{}'",
                    error_msg,
                    pattern
                );
            }
        }
    }

    #[test]
    fn test_error_message_length_limits() {
        // Ensure user error messages are appropriately concise
        let all_user_errors = vec![
            UserError::InvalidInput,
            UserError::InvalidOperation,
            UserError::NotFound,
            UserError::OperationFailed,
            UserError::InvalidState,
            UserError::SystemError,
        ];

        for error in all_user_errors {
            let error_msg = format!("{}", error);

            // User error messages should be short and generic
            assert!(
                error_msg.len() <= 30,
                "User error message too long: '{}' (length: {})",
                error_msg,
                error_msg.len()
            );

            // Should not be empty
            assert!(!error_msg.is_empty(), "User error message is empty");
        }
    }

    #[test]
    fn test_error_sanitizer_comprehensive_coverage() {
        let sanitizer = ErrorSanitizer::new(ErrorDetailLevel::Production);

        // Test all possible DeveloperGameError variants get mapped
        let all_developer_errors = vec![
            DeveloperGameError::NoRemainingDiscards,
            DeveloperGameError::NoRemainingPlays,
            DeveloperGameError::InvalidHand(DeveloperPlayHandError::TooManyCards),
            DeveloperGameError::InvalidStage,
            DeveloperGameError::InvalidAction,
            DeveloperGameError::InvalidBlind,
            DeveloperGameError::NoCardMatch,
            DeveloperGameError::NoJokerMatch,
            DeveloperGameError::InvalidMoveDirection,
            DeveloperGameError::NoAvailableSlot,
            DeveloperGameError::InvalidBalance,
            DeveloperGameError::InvalidMoveCard,
            DeveloperGameError::InvalidSelectCard,
            DeveloperGameError::InvalidActionSpace,
            DeveloperGameError::InvalidSlot,
            DeveloperGameError::JokerNotInShop,
            DeveloperGameError::JokerNotFound("test".to_string()),
            DeveloperGameError::InvalidOperation("test".to_string()),
            DeveloperGameError::InvalidInput("test".to_string()),
            DeveloperGameError::MutexPoisoned,
            DeveloperGameError::EmptyCollection,
            DeveloperGameError::MissingBlindState,
            DeveloperGameError::EmptyIterator,
            DeveloperGameError::HandAnalysisFailed("test".to_string()),
            DeveloperGameError::RngFailed("test".to_string()),
        ];

        // All errors should be sanitizable without panicking
        for error in all_developer_errors {
            let sanitized = sanitizer.sanitize_game_error(&error);
            let _ = format!("{}", sanitized); // Should not panic
        }
    }
}
