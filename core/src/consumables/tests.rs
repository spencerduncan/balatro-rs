//! Tests for consumable card targeting functionality

#[cfg(test)]
mod tests {
    use crate::card::{Card, Suit, Value};
    use crate::config::Config;
    use crate::consumables::{
        CardCollection, CardTarget, ConsumableSlots, Target, TargetType, TargetValidationError,
    };
    use crate::game::Game;
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
            TargetValidationError::NoCardsAvailable => {
                // Expected - this functionality isn't fully implemented yet
            }
            _ => panic!("Expected NoCardsAvailable error"),
        }
    }

    #[test]
    fn test_consumable_slots_basic_operations() {
        let mut slots = ConsumableSlots::new();

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
}
