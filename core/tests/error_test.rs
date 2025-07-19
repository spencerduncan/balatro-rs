use balatro_rs::error::{ActionSpaceError, GameError, PlayHandError};
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
        let joker_not_found = GameError::JokerNotFound;
        assert_eq!(format!("{}", joker_not_found), "Joker not found");

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

        let joker_not_found = GameError::JokerNotFound;
        assert_eq!(format!("{:?}", joker_not_found), "JokerNotFound");
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

        let joker1 = GameError::JokerNotFound;
        let joker2 = GameError::JokerNotFound;
        let joker3 = GameError::InvalidAction; // Use different error for inequality test

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
            GameError::JokerNotFound,
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

        // Verify it's a PyException by checking the string representation
        let error_string = py_err.to_string();
        assert!(error_string.contains("Invalid action"));
    }

    #[test]
    fn test_complex_game_error_to_py_err() {
        let game_error = GameError::JokerNotFound;
        let py_err: PyErr = game_error.into();

        // The error should contain the error message
        let error_string = py_err.to_string();
        assert!(error_string.contains("Joker not found"));
    }

    #[test]
    fn test_nested_error_to_py_err() {
        let play_hand_error = PlayHandError::TooManyCards;
        let game_error = GameError::InvalidHand(play_hand_error);
        let py_err: PyErr = game_error.into();

        let error_string = py_err.to_string();
        assert!(error_string.contains("Invalid hand played"));
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
        let error = GameError::InvalidOperation(long_string.clone());

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
