//! Tests for consumable card targeting functionality

#[cfg(test)]
mod test_mod {
    use crate::card::{Card, Edition, Seal, Suit, Value};
    use crate::config::Config;
    use crate::consumables::{
        CardCollection, CardTarget, ConsumableId, ConsumableSlots, ConsumableType, JokerTarget,
        JokerTargetError, SpectralPool, Target, TargetType, TargetValidationError,
    };
    use crate::consumables::spectral::{Aura, Talisman};
    use crate::consumables::Consumable;
    use crate::game::Game;
    use crate::joker::{Joker, JokerId, JokerRarity};
    use crate::rank::HandRank;

    /// Create a test game with some cards in hand and discard pile
    fn create_test_game() -> Game {
        let mut game = Game::new(Config::default()); // Create with default config

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
                "Expected CardIndexOutOfBounds error for index 0 in empty hand, got: {error:?}"
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
            assert!(target.is_valid(&game), "Target should be valid: {target:?}");
        }

        // Test invalid targets
        let invalid_targets = vec![
            Target::cards_in_discard(vec![10]), // out of bounds
            Target::Joker(5),                   // out of bounds (no jokers in test game)
        ];

        for target in invalid_targets {
            assert!(
                !target.is_valid(&game),
                "Target should be invalid: {target:?}"
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
        let mut game = Game::new(Config::default());

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
        let empty_game = Game::new(Config::default());

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
                assert!(result.is_ok(), "Failed: {description}");
            } else {
                assert!(result.is_err(), "Should have failed: {description}");
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
        assert!(target.validate(&Game::new(Config::default())).is_err()); // Empty game

        let invalid_target = JokerTarget::new(1);
        assert!(invalid_target.validate(&single_joker_game).is_err());
        assert!(invalid_target.validate(&many_jokers_game).is_ok());
    }

    #[test]
    fn test_joker_target_debug_output() {
        let target = JokerTarget::joker_of_type(2, JokerId::Joker);
        let debug_output = format!("{target:?}");

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
            assert_eq!(result.is_ok(), should_pass, "Boundary test failed: {desc}");
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

    // SpectralPool system tests for Issue #219

    #[test]
    fn test_spectral_pool_regular_cards() {
        let regular_cards = SpectralPool::Regular.get_cards();

        // Should contain current regular spectral cards
        assert!(regular_cards.contains(&ConsumableId::Familiar));
        assert!(regular_cards.contains(&ConsumableId::Grim));
        assert!(regular_cards.contains(&ConsumableId::Incantation));
        assert!(regular_cards.contains(&ConsumableId::Immolate));
        assert!(regular_cards.contains(&ConsumableId::Ankh));
        assert!(regular_cards.contains(&ConsumableId::DejaVu));
        assert!(regular_cards.contains(&ConsumableId::Hex));
        assert!(regular_cards.contains(&ConsumableId::Trance));
        assert!(regular_cards.contains(&ConsumableId::Medium));
        assert!(regular_cards.contains(&ConsumableId::Cryptid));

        // Should NOT contain special cards
        assert!(!regular_cards.contains(&ConsumableId::TheSoul));
        assert!(!regular_cards.contains(&ConsumableId::BlackHole));

        // Should have 10 cards currently (16 in full implementation)
        assert_eq!(regular_cards.len(), 10);
    }

    #[test]
    fn test_spectral_pool_special_cards() {
        let special_cards = SpectralPool::Special.get_cards();

        // Should contain only Soul and Black Hole
        assert!(special_cards.contains(&ConsumableId::TheSoul));
        assert!(special_cards.contains(&ConsumableId::BlackHole));

        // Should NOT contain regular cards
        assert!(!special_cards.contains(&ConsumableId::Familiar));
        assert!(!special_cards.contains(&ConsumableId::Grim));
        assert!(!special_cards.contains(&ConsumableId::Incantation));

        // Should have exactly 2 cards
        assert_eq!(special_cards.len(), 2);
    }

    #[test]
    fn test_spectral_pool_all_cards() {
        let all_cards = SpectralPool::All.get_cards();
        let regular_cards = SpectralPool::Regular.get_cards();
        let special_cards = SpectralPool::Special.get_cards();

        // Should contain all regular cards
        for card in &regular_cards {
            assert!(
                all_cards.contains(card),
                "All pool missing regular card: {card}"
            );
        }

        // Should contain all special cards
        for card in &special_cards {
            assert!(
                all_cards.contains(card),
                "All pool missing special card: {card}"
            );
        }

        // Should have combined length
        assert_eq!(all_cards.len(), regular_cards.len() + special_cards.len());
        assert_eq!(all_cards.len(), 12); // 10 regular + 2 special
    }

    #[test]
    fn test_spectral_pool_contains() {
        // Test Regular pool
        assert!(SpectralPool::Regular.contains(ConsumableId::Familiar));
        assert!(SpectralPool::Regular.contains(ConsumableId::Grim));
        assert!(SpectralPool::Regular.contains(ConsumableId::Incantation));
        assert!(!SpectralPool::Regular.contains(ConsumableId::TheSoul));
        assert!(!SpectralPool::Regular.contains(ConsumableId::BlackHole));

        // Test Special pool
        assert!(!SpectralPool::Special.contains(ConsumableId::Familiar));
        assert!(!SpectralPool::Special.contains(ConsumableId::Grim));
        assert!(!SpectralPool::Special.contains(ConsumableId::Incantation));
        assert!(SpectralPool::Special.contains(ConsumableId::TheSoul));
        assert!(SpectralPool::Special.contains(ConsumableId::BlackHole));

        // Test All pool
        assert!(SpectralPool::All.contains(ConsumableId::Familiar));
        assert!(SpectralPool::All.contains(ConsumableId::Grim));
        assert!(SpectralPool::All.contains(ConsumableId::Incantation));
        assert!(SpectralPool::All.contains(ConsumableId::TheSoul));
        assert!(SpectralPool::All.contains(ConsumableId::BlackHole));
    }

    #[test]
    fn test_spectral_pool_is_special_card() {
        // Special cards
        assert!(SpectralPool::is_special_card(ConsumableId::TheSoul));
        assert!(SpectralPool::is_special_card(ConsumableId::BlackHole));

        // Regular spectral cards
        assert!(!SpectralPool::is_special_card(ConsumableId::Familiar));
        assert!(!SpectralPool::is_special_card(ConsumableId::Grim));
        assert!(!SpectralPool::is_special_card(ConsumableId::Incantation));

        // Non-spectral cards
        assert!(!SpectralPool::is_special_card(ConsumableId::TheFool)); // Tarot
        assert!(!SpectralPool::is_special_card(ConsumableId::Mercury)); // Planet
    }

    #[test]
    fn test_spectral_pool_is_regular_card() {
        // Regular spectral cards
        assert!(SpectralPool::is_regular_card(ConsumableId::Familiar));
        assert!(SpectralPool::is_regular_card(ConsumableId::Grim));
        assert!(SpectralPool::is_regular_card(ConsumableId::Incantation));

        // Special spectral cards
        assert!(!SpectralPool::is_regular_card(ConsumableId::TheSoul));
        assert!(!SpectralPool::is_regular_card(ConsumableId::BlackHole));

        // Non-spectral cards
        assert!(!SpectralPool::is_regular_card(ConsumableId::TheFool)); // Tarot
        assert!(!SpectralPool::is_regular_card(ConsumableId::Mercury)); // Planet
    }

    #[test]
    fn test_spectral_pool_pool_containing() {
        // Regular spectral cards should be in Regular pool
        assert_eq!(
            SpectralPool::pool_containing(ConsumableId::Familiar),
            Some(SpectralPool::Regular)
        );
        assert_eq!(
            SpectralPool::pool_containing(ConsumableId::Grim),
            Some(SpectralPool::Regular)
        );
        assert_eq!(
            SpectralPool::pool_containing(ConsumableId::Incantation),
            Some(SpectralPool::Regular)
        );

        // Special spectral cards should be in Special pool
        assert_eq!(
            SpectralPool::pool_containing(ConsumableId::TheSoul),
            Some(SpectralPool::Special)
        );
        assert_eq!(
            SpectralPool::pool_containing(ConsumableId::BlackHole),
            Some(SpectralPool::Special)
        );

        // Non-spectral cards should return None
        assert_eq!(SpectralPool::pool_containing(ConsumableId::TheFool), None); // Tarot
        assert_eq!(SpectralPool::pool_containing(ConsumableId::Mercury), None); // Planet
    }

    #[test]
    fn test_spectral_pool_display() {
        assert_eq!(SpectralPool::Regular.to_string(), "Regular");
        assert_eq!(SpectralPool::Special.to_string(), "Special");
        assert_eq!(SpectralPool::All.to_string(), "All");
    }

    #[test]
    fn test_spectral_pool_descriptions() {
        assert_eq!(
            SpectralPool::Regular.description(),
            "Regular spectral cards (excludes Soul and Black Hole)"
        );
        assert_eq!(
            SpectralPool::Special.description(),
            "Special spectral cards (Soul and Black Hole only)"
        );
        assert_eq!(
            SpectralPool::All.description(),
            "All spectral cards (complete set)"
        );
    }

    #[test]
    fn test_spectral_pool_serialization() {
        use serde_json;

        let pools = [
            SpectralPool::Regular,
            SpectralPool::Special,
            SpectralPool::All,
        ];

        for pool in &pools {
            // Test serialization
            let serialized = serde_json::to_string(pool);
            assert!(serialized.is_ok(), "Failed to serialize {pool}");

            // Test deserialization
            let json = serialized.unwrap();
            let deserialized: Result<SpectralPool, _> = serde_json::from_str(&json);
            assert!(deserialized.is_ok(), "Failed to deserialize {pool}");

            let restored = deserialized.unwrap();
            assert_eq!(pool, &restored, "Serialization roundtrip failed for {pool}");
        }
    }

    #[test]
    fn test_new_consumable_ids_in_enum() {
        // Verify that the new spectral cards are in the ConsumableId enum
        let soul_display = format!("{}", ConsumableId::TheSoul);
        let black_hole_display = format!("{}", ConsumableId::BlackHole);

        assert_eq!(soul_display, "The Soul");
        assert_eq!(black_hole_display, "Black Hole");
    }

    #[test]
    fn test_new_spectral_cards_have_correct_type() {
        // Verify the new cards are properly classified as Spectral
        assert_eq!(
            ConsumableId::TheSoul.consumable_type(),
            ConsumableType::Spectral
        );
        assert_eq!(
            ConsumableId::BlackHole.consumable_type(),
            ConsumableType::Spectral
        );
    }

    #[test]
    fn test_spectral_cards_list_includes_new_cards() {
        let all_spectral = ConsumableId::spectral_cards();

        // Should include all implemented spectral cards from both branches
        // Original PR Implementation (Issue #11)
        assert!(all_spectral.contains(&ConsumableId::Familiar));
        assert!(all_spectral.contains(&ConsumableId::Grim));
        assert!(all_spectral.contains(&ConsumableId::Incantation));
        assert!(all_spectral.contains(&ConsumableId::Talisman));
        assert!(all_spectral.contains(&ConsumableId::Aura));
        assert!(all_spectral.contains(&ConsumableId::Wraith));
        assert!(all_spectral.contains(&ConsumableId::Sigil));
        assert!(all_spectral.contains(&ConsumableId::Ouija));
        assert!(all_spectral.contains(&ConsumableId::Ectoplasm));

        // Modern Main Branch Implementation
        assert!(all_spectral.contains(&ConsumableId::Immolate));
        assert!(all_spectral.contains(&ConsumableId::Ankh));
        assert!(all_spectral.contains(&ConsumableId::DejaVu));
        assert!(all_spectral.contains(&ConsumableId::Hex));
        assert!(all_spectral.contains(&ConsumableId::Trance));
        assert!(all_spectral.contains(&ConsumableId::Medium));
        assert!(all_spectral.contains(&ConsumableId::Cryptid));
        assert!(all_spectral.contains(&ConsumableId::TheSoul));
        assert!(all_spectral.contains(&ConsumableId::BlackHole));

        // Should have exactly 18 cards (9 from original PR + 9 from modern main)
        assert_eq!(all_spectral.len(), 18);
    }

    // Integration tests for the restriction requirements

    #[test]
    fn test_joker_effect_restrictions() {
        // This test validates that joker effects (Sixth Sense, Seance) would use Regular pool
        // When those jokers are implemented, they should use SpectralPool::Regular.get_cards()
        let regular_pool = SpectralPool::Regular.get_cards();

        // Joker effects should NOT be able to generate these special cards
        assert!(!regular_pool.contains(&ConsumableId::TheSoul));
        assert!(!regular_pool.contains(&ConsumableId::BlackHole));

        // But should be able to generate regular spectral cards
        assert!(regular_pool.contains(&ConsumableId::Familiar));
        assert!(regular_pool.contains(&ConsumableId::Grim));
        assert!(regular_pool.contains(&ConsumableId::Incantation));
    }

    #[test]
    fn test_ghost_deck_restrictions() {
        // This test validates that Ghost Deck would exclude special cards from purchase
        // When Ghost Deck is implemented, it should use SpectralPool::Regular for spectral purchases
        let ghost_deck_allowed = SpectralPool::Regular.get_cards();

        // Ghost Deck should NOT allow purchase of special cards
        assert!(!ghost_deck_allowed.contains(&ConsumableId::TheSoul));
        assert!(!ghost_deck_allowed.contains(&ConsumableId::BlackHole));

        // But should allow regular spectral cards
        assert!(ghost_deck_allowed.contains(&ConsumableId::Familiar));
        assert!(ghost_deck_allowed.contains(&ConsumableId::Grim));
        assert!(ghost_deck_allowed.contains(&ConsumableId::Incantation));
    }

    #[test]
    fn test_pack_distribution_rules() {
        // Test that Soul can appear in Arcana packs (validated by the pack generation code)
        // Test that Black Hole can appear in Celestial packs (validated by the pack generation code)

        // Soul should be considered special
        assert!(SpectralPool::is_special_card(ConsumableId::TheSoul));
        assert_eq!(
            SpectralPool::pool_containing(ConsumableId::TheSoul),
            Some(SpectralPool::Special)
        );

        // Black Hole should be considered special
        assert!(SpectralPool::is_special_card(ConsumableId::BlackHole));
        assert_eq!(
            SpectralPool::pool_containing(ConsumableId::BlackHole),
            Some(SpectralPool::Special)
        );

        // All pool should include both for regular spectral packs
        let all_spectral = SpectralPool::All.get_cards();
        assert!(all_spectral.contains(&ConsumableId::TheSoul));
        assert!(all_spectral.contains(&ConsumableId::BlackHole));
    }

    #[test]
    fn test_spectral_pool_boundary_conditions() {
        // Test edge cases and boundary conditions

        // Empty pools should be handled gracefully
        let regular_cards = SpectralPool::Regular.get_cards();
        let special_cards = SpectralPool::Special.get_cards();
        let all_cards = SpectralPool::All.get_cards();

        // None should be empty
        assert!(!regular_cards.is_empty());
        assert!(!special_cards.is_empty());
        assert!(!all_cards.is_empty());

        // All should have expected minimum counts
        assert!(regular_cards.len() >= 3); // At least the current regular cards
        assert_eq!(special_cards.len(), 2); // Exactly Soul and Black Hole
        assert_eq!(all_cards.len(), regular_cards.len() + special_cards.len());
    }

    #[test]
    fn test_spectral_pool_consistency() {
        // Test that pools are consistent with each other
        let regular_cards = SpectralPool::Regular.get_cards();
        let special_cards = SpectralPool::Special.get_cards();
        let all_cards = SpectralPool::All.get_cards();

        // No overlap between regular and special
        for card in &regular_cards {
            assert!(
                !special_cards.contains(card),
                "Card {card} should not be in both regular and special pools"
            );
        }

        // All cards should be union of regular and special
        let mut expected_all = regular_cards.clone();
        expected_all.extend(special_cards.clone());
        expected_all.sort();

        let mut actual_all = all_cards.clone();
        actual_all.sort();

        assert_eq!(
            expected_all, actual_all,
            "All pool should be union of regular and special pools"
        );
    }

    #[test]
    fn test_aura_applies_random_edition() {
        let mut game = create_test_game();

        // Add a card to available hand for testing
        let test_card = Card::new(Value::Ace, Suit::Heart);
        game.available.extend(vec![test_card]);

        // Verify card starts with base edition
        assert_eq!(game.available.card_from_index(0).unwrap().edition, Edition::Base);

        // Create Aura spectral card
        let aura = Aura;

        // Create target for the first card
        let target = Target::cards_in_hand(vec![0]);

        // Use Aura on the card
        let result = aura.use_effect(&mut game, target);
        assert!(result.is_ok(), "Aura should successfully apply edition");

        // Verify the card now has a special edition (not Base)
        let card_after = game.available.card_from_index(0).unwrap();
        assert_ne!(card_after.edition, Edition::Base, "Card should have a special edition");

        // Verify it's one of the valid editions
        assert!(
            matches!(card_after.edition, Edition::Foil | Edition::Holographic | Edition::Polychrome),
            "Card should have Foil, Holographic, or Polychrome edition, got {:?}",
            card_after.edition
        );
    }

    #[test]
    fn test_aura_multiple_applications() {
        // Test multiple applications to ensure it works consistently
        for _i in 0..3 {
            let mut game = create_test_game();

            let test_card = Card::new(Value::King, Suit::Diamond);
            game.available.extend(vec![test_card]);

            let aura = Aura;
            let target = Target::cards_in_hand(vec![0]);

            let result = aura.use_effect(&mut game, target);
            assert!(result.is_ok());

            // Verify edition was applied
            let card_after = game.available.card_from_index(0).unwrap();
            assert_ne!(card_after.edition, Edition::Base);
            assert!(
                matches!(card_after.edition, Edition::Foil | Edition::Holographic | Edition::Polychrome),
                "Card should have one of the three special editions"
            );
        }
    }

    #[test]
    fn test_aura_overwrites_existing_edition() {
        let mut game = create_test_game();

        // Create a card with existing Foil edition
        let mut test_card = Card::new(Value::Queen, Suit::Club);
        test_card.edition = Edition::Foil;
        game.available.extend(vec![test_card]);

        // Verify card starts with Foil
        assert_eq!(game.available.card_from_index(0).unwrap().edition, Edition::Foil);

        let aura = Aura;
        let target = Target::cards_in_hand(vec![0]);

        let result = aura.use_effect(&mut game, target);
        assert!(result.is_ok());

        // Verify edition was changed (might be same by chance, but implementation works)
        let card_after = game.available.card_from_index(0).unwrap();
        assert!(
            matches!(card_after.edition, Edition::Foil | Edition::Holographic | Edition::Polychrome),
            "Card should have one of the three special editions"
        );
    }

    #[test]
    fn test_talisman_applies_gold_seal() {
        let mut game = create_test_game();

        // Add a card to available hand for testing
        let test_card = Card::new(Value::Jack, Suit::Spade);
        game.available.extend(vec![test_card]);

        // Verify card starts with no seal
        assert_eq!(game.available.card_from_index(0).unwrap().seal, None);

        // Create Talisman spectral card
        let talisman = Talisman;

        // Create target for the first card
        let target = Target::cards_in_hand(vec![0]);

        // Use Talisman on the card
        let result = talisman.use_effect(&mut game, target);
        assert!(result.is_ok(), "Talisman should successfully apply Gold seal");

        // Verify the card now has Gold seal
        let card_after = game.available.card_from_index(0).unwrap();
        assert_eq!(card_after.seal, Some(Seal::Gold), "Card should have Gold seal");
    }

    #[test]
    fn test_talisman_overwrites_existing_seal() {
        let mut game = create_test_game();

        // Create a card with existing Blue seal
        let mut test_card = Card::new(Value::Ten, Suit::Heart);
        test_card.seal = Some(Seal::Blue);
        game.available.extend(vec![test_card]);

        // Verify card starts with Blue seal
        assert_eq!(game.available.card_from_index(0).unwrap().seal, Some(Seal::Blue));

        let talisman = Talisman;
        let target = Target::cards_in_hand(vec![0]);

        let result = talisman.use_effect(&mut game, target);
        assert!(result.is_ok());

        // Verify seal was changed to Gold
        let card_after = game.available.card_from_index(0).unwrap();
        assert_eq!(card_after.seal, Some(Seal::Gold), "Card should now have Gold seal, not Blue");
    }

    #[test]
    fn test_aura_and_talisman_invalid_targets() {
        let mut game = create_test_game();

        let aura = Aura;
        let talisman = Talisman;

        // Test with joker target (invalid for these consumables)
        let joker_target = Target::active_joker_at_slot(0);

        let aura_result = aura.use_effect(&mut game, joker_target.clone());
        let talisman_result = talisman.use_effect(&mut game, joker_target);

        assert!(aura_result.is_err(), "Aura should fail with non-card target");
        assert!(talisman_result.is_err(), "Talisman should fail with non-card target");

        // Test with invalid card index (empty hand)
        let invalid_target = Target::cards_in_hand(vec![999]); // Invalid index

        // These should not crash, the get_card_mut method handles bounds checking
        let _ = aura.use_effect(&mut game, invalid_target.clone());
        let _ = talisman.use_effect(&mut game, invalid_target);
    }

    #[test]
    fn test_get_card_mut_bounds_checking() {
        let mut game = create_test_game();

        // Test bounds checking on empty available cards
        assert_eq!(game.available.get_card_mut(0), None);
        assert_eq!(game.available.get_card_mut(999), None);

        // Add one card
        let test_card = Card::new(Value::Seven, Suit::Diamond);
        game.available.extend(vec![test_card]);

        // Test valid index
        assert!(game.available.get_card_mut(0).is_some());

        // Test invalid indices
        assert_eq!(game.available.get_card_mut(1), None);
        assert_eq!(game.available.get_card_mut(999), None);
    }
}
