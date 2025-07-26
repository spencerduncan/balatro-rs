//! Tests for consumable card targeting functionality

#[cfg(test)]
mod tests {
    use crate::card::{Card, Suit, Value};
    use crate::config::Config;
    use crate::consumables::{
        CardCollection, CardTarget, ConsumableSlots, JokerTarget, JokerTargetError, Target,
        TargetType, TargetValidationError,
    };
    use crate::game::Game;
    use crate::joker::{Joker, JokerId, JokerRarity};
    use crate::rank::HandRank;

    /// Create a test game with some cards in hand and discard pile
    fn create_test_game() -> Game {
        let mut game = Game::new(Config::new()); // Create with default config

        // Add some cards to discard pile for testing
        for i in 0..5 {
            let card = Card::new(
                match i % 4 {
                    0 => Value::Ace,
                    1 => Value::Two,
                    2 => Value::King,
                    _ => Value::Queen,
                },
                match i % 4 {
                    0 => Suit::Heart,
                    1 => Suit::Diamond,
                    2 => Suit::Club,
                    _ => Suit::Spade,
                },
            );
            game.discarded.push(card);
        }

        game
    }

    #[test]
    fn test_card_collection_display() {
        assert_eq!(CardCollection::Hand.to_string(), "Hand");
        assert_eq!(CardCollection::Deck.to_string(), "Deck");
        assert_eq!(CardCollection::DiscardPile.to_string(), "Discard Pile");
        assert_eq!(CardCollection::PlayedCards.to_string(), "Played Cards");
    }

    #[test]
    fn test_card_target_creation() {
        // Test creating a single card target
        let target = CardTarget::single_card(CardCollection::Hand, 0);
        assert_eq!(target.indices, vec![0]);
        assert_eq!(target.collection, CardCollection::Hand);
        assert_eq!(target.min_cards, 1);
        assert_eq!(target.max_cards, 1);

        // Test creating a multi-card target
        let target = CardTarget::new(CardCollection::DiscardPile, vec![0, 2, 4]);
        assert_eq!(target.indices, vec![0, 2, 4]);
        assert_eq!(target.collection, CardCollection::DiscardPile);
        assert_eq!(target.min_cards, 3);
        assert_eq!(target.max_cards, 3);
    }

    #[test]
    fn test_card_target_validation_success() {
        let game = create_test_game();

        // Test valid discard pile targeting
        let target = CardTarget::new(CardCollection::DiscardPile, vec![0, 1, 2]);
        assert!(target.validate(&game).is_ok());

        // Test single card targeting
        let target = CardTarget::single_card(CardCollection::DiscardPile, 3);
        assert!(target.validate(&game).is_ok());
    }

    #[test]
    fn test_card_target_validation_out_of_bounds() {
        let game = create_test_game();

        // Test out of bounds for discard pile (we have 5 cards, so index 5 is invalid)
        let target = CardTarget::single_card(CardCollection::DiscardPile, 5);
        let result = target.validate(&game);
        assert!(result.is_err());

        match result.unwrap_err() {
            TargetValidationError::DiscardIndexOutOfBounds {
                index,
                discard_size,
            } => {
                assert_eq!(index, 5);
                assert_eq!(discard_size, 5);
            }
            _ => panic!("Expected DiscardIndexOutOfBounds error"),
        }
    }

    #[test]
    fn test_card_target_validation_duplicate_indices() {
        let game = create_test_game();

        // Test duplicate indices
        let target = CardTarget::new(CardCollection::DiscardPile, vec![0, 1, 0]);
        let result = target.validate(&game);
        assert!(result.is_err());

        match result.unwrap_err() {
            TargetValidationError::CardAlreadyTargeted { index } => {
                assert_eq!(index, 0);
            }
            _ => panic!("Expected CardAlreadyTargeted error"),
        }
    }

    #[test]
    fn test_target_helper_methods() {
        // Test Target helper methods
        let target = Target::cards_in_hand(vec![0, 1]);
        match target {
            Target::Cards(card_target) => {
                assert_eq!(card_target.collection, CardCollection::Hand);
                assert_eq!(card_target.indices, vec![0, 1]);
            }
            _ => panic!("Expected Cards target"),
        }

        let target = Target::cards_in_discard(vec![2, 3]);
        match target {
            Target::Cards(card_target) => {
                assert_eq!(card_target.collection, CardCollection::DiscardPile);
                assert_eq!(card_target.indices, vec![2, 3]);
            }
            _ => panic!("Expected Cards target"),
        }
    }

    #[test]
    fn test_target_type_checking() {
        let card_target = Target::cards_in_hand(vec![0, 1]);
        assert_eq!(card_target.target_type(), TargetType::Cards(2));
        assert!(card_target.is_valid_type(TargetType::Cards(2)));
        assert!(!card_target.is_valid_type(TargetType::Cards(3)));

        let hand_target = Target::HandType(HandRank::OnePair);
        assert_eq!(hand_target.target_type(), TargetType::HandType);
        assert!(hand_target.is_valid_type(TargetType::HandType));

        let joker_target = Target::Joker(0);
        assert_eq!(joker_target.target_type(), TargetType::Joker);
        assert!(joker_target.is_valid_type(TargetType::Joker));
    }

    #[test]
    fn test_card_target_get_cards_discard_pile() {
        let game = create_test_game();

        // Test getting cards from discard pile (this should work)
        let target = CardTarget::new(CardCollection::DiscardPile, vec![0, 2]);
        let result = target.get_cards(&game);
        assert!(result.is_ok());

        let cards = result.unwrap();
        assert_eq!(cards.len(), 2);
    }

    #[test]
    fn test_card_target_get_cards_hand_not_implemented() {
        let game = create_test_game();

        // Test that hand card access returns NoCardsAvailable (not yet implemented)
        let target = CardTarget::new(CardCollection::Hand, vec![0]);
        let result = target.get_cards(&game);
        assert!(result.is_err());

        match result.unwrap_err() {
            TargetValidationError::CardIndexOutOfBounds {
                index: 0,
                hand_size: 0,
            } => {
                // Expected - index 0 is out of bounds for empty hand
            }
            error => panic!(
                "Expected CardIndexOutOfBounds error for index 0 in empty hand, got: {:?}",
                error
            ),
        }
    }

    #[test]
    fn test_consumable_slots_basic_operations() {
        let slots = ConsumableSlots::new();

        // Test initial state
        assert_eq!(slots.capacity(), 2);
        assert_eq!(slots.len(), 0);
        assert!(slots.is_empty());
        assert!(!slots.is_full());
        assert_eq!(slots.available_slots(), 2);

        // Test find empty slot
        assert_eq!(slots.find_empty_slot(), Some(0));
    }

    #[test]
    fn test_consumable_slots_custom_capacity() {
        let slots = ConsumableSlots::with_capacity(5);
        assert_eq!(slots.capacity(), 5);
        assert_eq!(slots.available_slots(), 5);
    }

    #[test]
    fn test_target_validation_with_game_state() {
        let game = create_test_game();

        // Test valid targets
        let valid_targets = vec![
            Target::None,
            Target::cards_in_discard(vec![0, 1]),
            Target::HandType(HandRank::OnePair),
            Target::Deck,
        ];

        for target in valid_targets {
            assert!(
                target.is_valid(&game),
                "Target should be valid: {:?}",
                target
            );
        }

        // Test invalid targets
        let invalid_targets = vec![
            Target::cards_in_discard(vec![10]), // out of bounds
            Target::Joker(5),                   // out of bounds (no jokers in test game)
        ];

        for target in invalid_targets {
            assert!(
                !target.is_valid(&game),
                "Target should be invalid: {:?}",
                target
            );
        }
    }

    // JokerTarget and JokerTargetError tests

    /// Mock joker for testing
    #[derive(Debug, Clone)]
    struct MockJoker {
        id: JokerId,
        name: &'static str,
    }

    impl MockJoker {
        fn new(id: JokerId, name: &'static str) -> Self {
            Self { id, name }
        }
    }

    impl Joker for MockJoker {
        fn id(&self) -> JokerId {
            self.id
        }

        fn name(&self) -> &str {
            self.name
        }

        fn description(&self) -> &str {
            "Mock joker for testing"
        }

        fn rarity(&self) -> JokerRarity {
            JokerRarity::Common
        }
    }

    /// Helper function to create a game with mock jokers
    fn create_game_with_jokers(joker_count: usize) -> Game {
        let mut game = Game::new(Config::new());

        // Add mock jokers to the game
        for i in 0..joker_count {
            let joker_id = match i % 3 {
                0 => JokerId::Joker,
                1 => JokerId::GreedyJoker,
                _ => JokerId::LustyJoker,
            };
            let joker = Box::new(MockJoker::new(joker_id, "Mock Joker"));
            game.jokers.push(joker);
        }

        game
    }

    #[test]
    fn test_joker_target_new() {
        let target = JokerTarget::new(2);

        assert_eq!(target.slot, 2);
        assert!(!target.require_active);
        assert_eq!(target.joker_type, None);
    }

    #[test]
    fn test_joker_target_active_joker() {
        let target = JokerTarget::active_joker(1);

        assert_eq!(target.slot, 1);
        assert!(target.require_active);
        assert_eq!(target.joker_type, None);
    }

    #[test]
    fn test_joker_target_joker_of_type() {
        let target = JokerTarget::joker_of_type(3, JokerId::Joker);

        assert_eq!(target.slot, 3);
        assert!(!target.require_active);
        assert_eq!(target.joker_type, Some(JokerId::Joker));
    }

    #[test]
    fn test_joker_target_validate_valid_slot() {
        let game = create_game_with_jokers(3);
        let target = JokerTarget::new(1);

        let result = target.validate(&game);
        assert!(result.is_ok());
    }

    #[test]
    fn test_joker_target_validate_empty_slot() {
        let game = create_game_with_jokers(2);
        let target = JokerTarget::new(3); // Slot 3 doesn't exist (only 0, 1)

        let result = target.validate(&game);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            JokerTargetError::EmptySlot { slot: 3 }
        ));
    }

    #[test]
    fn test_joker_target_validate_wrong_joker_type() {
        let game = create_game_with_jokers(3);
        // Slot 0 has Joker, but we're expecting GreedyJoker
        let target = JokerTarget::joker_of_type(0, JokerId::GreedyJoker);

        let result = target.validate(&game);
        assert!(result.is_err());

        match result.unwrap_err() {
            JokerTargetError::WrongJokerType { expected, actual } => {
                assert_eq!(expected, JokerId::GreedyJoker);
                assert_eq!(actual, JokerId::Joker);
            }
            _ => panic!("Expected WrongJokerType error"),
        }
    }

    #[test]
    fn test_joker_target_validate_correct_joker_type() {
        let game = create_game_with_jokers(3);
        // Slot 0 has Joker, and we're expecting Joker
        let target = JokerTarget::joker_of_type(0, JokerId::Joker);

        let result = target.validate(&game);
        assert!(result.is_ok());
    }

    #[test]
    fn test_joker_target_get_joker_valid() {
        let game = create_game_with_jokers(3);
        let target = JokerTarget::new(1);

        let result = target.get_joker(&game);
        assert!(result.is_ok());
        let joker = result.unwrap();
        assert_eq!(joker.id(), JokerId::GreedyJoker); // Second joker has GreedyJoker
    }

    #[test]
    fn test_joker_target_get_joker_invalid() {
        let game = create_game_with_jokers(2);
        let target = JokerTarget::new(5); // Invalid slot

        let result = target.get_joker(&game);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            JokerTargetError::EmptySlot { slot: 5 }
        ));
    }

    #[test]
    fn test_joker_target_is_slot_occupied() {
        let game = create_game_with_jokers(3);

        let target_valid = JokerTarget::new(1);
        let target_invalid = JokerTarget::new(5);

        assert!(target_valid.is_slot_occupied(&game));
        assert!(!target_invalid.is_slot_occupied(&game));
    }

    #[test]
    fn test_joker_target_error_display() {
        let empty_slot_error = JokerTargetError::EmptySlot { slot: 2 };
        let inactive_joker_error = JokerTargetError::InactiveJoker { slot: 1 };
        let wrong_type_error = JokerTargetError::WrongJokerType {
            expected: JokerId::Joker,
            actual: JokerId::GreedyJoker,
        };

        assert!(empty_slot_error
            .to_string()
            .contains("Joker slot 2 is empty"));
        assert!(inactive_joker_error
            .to_string()
            .contains("Joker at slot 1 is not active"));
        assert!(wrong_type_error.to_string().contains("Expected joker type"));
        assert!(wrong_type_error.to_string().contains("but found"));
    }

    #[test]
    fn test_joker_target_equality_and_clone() {
        let target1 = JokerTarget::new(2);
        let target2 = JokerTarget::new(2);
        let target3 = JokerTarget::active_joker(2);

        assert_eq!(target1, target2);
        assert_ne!(target1, target3);

        let cloned = target1.clone();
        assert_eq!(target1, cloned);
    }

    #[test]
    fn test_joker_target_serialization() {
        use serde_json;

        let target = JokerTarget::joker_of_type(3, JokerId::Joker);

        // Test serialization
        let serialized = serde_json::to_string(&target);
        assert!(serialized.is_ok());

        // Test deserialization
        let json = serialized.unwrap();
        let deserialized: Result<JokerTarget, _> = serde_json::from_str(&json);
        assert!(deserialized.is_ok());

        let restored = deserialized.unwrap();
        assert_eq!(target, restored);
    }

    // Tests for Target enum integration methods

    #[test]
    fn test_target_as_joker_target() {
        let joker_target = Target::Joker(2);
        let card_target = Target::cards_in_hand(vec![0, 1]);
        let none_target = Target::None;

        // Test conversion from Target::Joker
        let result = joker_target.as_joker_target();
        assert!(result.is_some());
        let joker_target_struct = result.unwrap();
        assert_eq!(joker_target_struct.slot, 2);
        assert!(!joker_target_struct.require_active);
        assert_eq!(joker_target_struct.joker_type, None);

        // Test conversion from non-joker targets
        assert!(card_target.as_joker_target().is_none());
        assert!(none_target.as_joker_target().is_none());
    }

    #[test]
    fn test_target_active_joker_at_slot() {
        let target = Target::active_joker_at_slot(3);

        // Note: This returns Target::Joker(3) since Target enum doesn't store active requirement
        assert!(matches!(target, Target::Joker(3)));
        assert_eq!(target.target_type(), TargetType::Joker);
    }

    #[test]
    fn test_target_joker_methods_integration() {
        let game = create_game_with_jokers(4);

        // Test the workflow: Target -> JokerTarget -> validation
        let target = Target::Joker(2);
        let joker_target = target.as_joker_target().unwrap();
        let validation_result = joker_target.validate(&game);

        assert!(validation_result.is_ok());

        // Test getting the joker
        let joker = joker_target.get_joker(&game).unwrap();
        assert_eq!(joker.id(), JokerId::LustyJoker); // Third joker (index 2) has LustyJoker
    }

    #[test]
    fn test_target_joker_invalid_slot() {
        let game = create_game_with_jokers(2);

        let target = Target::Joker(5);
        let joker_target = target.as_joker_target().unwrap();
        let validation_result = joker_target.validate(&game);

        assert!(validation_result.is_err());
        assert!(matches!(
            validation_result.unwrap_err(),
            JokerTargetError::EmptySlot { slot: 5 }
        ));
    }

    #[test]
    fn test_joker_target_edge_cases() {
        let empty_game = Game::new(Config::new());

        // Test with empty game (no jokers)
        let target = JokerTarget::new(0);
        let result = target.validate(&empty_game);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            JokerTargetError::EmptySlot { slot: 0 }
        ));

        // Test is_slot_occupied with empty game
        assert!(!target.is_slot_occupied(&empty_game));
    }

    #[test]
    fn test_joker_target_comprehensive_validation() {
        let game = create_game_with_jokers(5);

        // Test various targeting scenarios
        let test_cases = vec![
            (JokerTarget::new(0), true, "Basic targeting slot 0"),
            (JokerTarget::new(4), true, "Basic targeting last slot"),
            (JokerTarget::new(5), false, "Targeting beyond bounds"),
            (JokerTarget::active_joker(0), true, "Active joker slot 0"),
            (
                JokerTarget::active_joker(10),
                false,
                "Active joker beyond bounds",
            ),
            (
                JokerTarget::joker_of_type(0, JokerId::Joker),
                true,
                "Correct type slot 0",
            ),
            (
                JokerTarget::joker_of_type(1, JokerId::Joker),
                false,
                "Wrong type slot 1",
            ),
            (
                JokerTarget::joker_of_type(2, JokerId::LustyJoker),
                true,
                "Correct type slot 2",
            ),
        ];

        for (target, should_pass, description) in test_cases {
            let result = target.validate(&game);
            if should_pass {
                assert!(result.is_ok(), "Failed: {}", description);
            } else {
                assert!(result.is_err(), "Should have failed: {}", description);
            }
        }
    }

    #[test]
    fn test_joker_target_with_different_game_states() {
        // Test with various joker configurations
        let single_joker_game = create_game_with_jokers(1);
        let many_jokers_game = create_game_with_jokers(10);

        // Test single joker game
        let target = JokerTarget::new(0);
        assert!(target.validate(&single_joker_game).is_ok());
        assert!(!target.validate(&Game::new(Config::new())).is_ok()); // Empty game

        let invalid_target = JokerTarget::new(1);
        assert!(!invalid_target.validate(&single_joker_game).is_ok());
        assert!(invalid_target.validate(&many_jokers_game).is_ok());
    }

    #[test]
    fn test_joker_target_debug_output() {
        let target = JokerTarget::joker_of_type(2, JokerId::Joker);
        let debug_output = format!("{:?}", target);

        assert!(debug_output.contains("JokerTarget"));
        assert!(debug_output.contains("slot: 2"));
        assert!(debug_output.contains("require_active"));
        assert!(debug_output.contains("joker_type"));
    }

    // Production-focused failure mode tests

    #[test]
    fn test_joker_target_failure_modes() {
        // Test various failure scenarios that could occur in production
        let game = create_game_with_jokers(3);

        // Scenario 1: Race condition - slot becomes empty after initial check
        let target = JokerTarget::new(2);
        assert!(target.is_slot_occupied(&game));
        // In production, another thread might remove the joker here
        // Our validation should still catch this
        assert!(target.validate(&game).is_ok());

        // Scenario 2: Type confusion - wrong joker type
        let typed_target = JokerTarget::joker_of_type(1, JokerId::Joker);
        let result = typed_target.validate(&game);
        assert!(result.is_err());
        // Error should be actionable for debugging
        if let Err(JokerTargetError::WrongJokerType { expected, actual }) = result {
            assert_eq!(expected, JokerId::Joker);
            assert_eq!(actual, JokerId::GreedyJoker);
        }

        // Scenario 3: Boundary testing
        let boundary_cases = vec![
            (usize::MAX, false, "MAX boundary"),
            (1000, false, "Large index"),
            (0, true, "Zero index"),
        ];

        for (slot, should_pass, desc) in boundary_cases {
            let target = JokerTarget::new(slot);
            let result = target.validate(&game);
            assert_eq!(
                result.is_ok(),
                should_pass,
                "Boundary test failed: {}",
                desc
            );
        }
    }

    #[test]
    fn test_joker_target_error_telemetry() {
        // Ensure errors contain enough context for production debugging
        let errors = vec![
            JokerTargetError::EmptySlot { slot: 42 },
            JokerTargetError::InactiveJoker { slot: 7 },
            JokerTargetError::WrongJokerType {
                expected: JokerId::Joker,
                actual: JokerId::GreedyJoker,
            },
        ];

        for error in errors {
            let error_string = error.to_string();
            // All errors should mention the slot or joker type for debugging
            assert!(!error_string.is_empty());
            assert!(error_string.contains("slot") || error_string.contains("type"));
        }
    }
}
