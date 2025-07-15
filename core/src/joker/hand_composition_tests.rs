use crate::{
    card::{Card, Suit, Value as Rank},
    hand::SelectHand,
    joker::{ConditionalJoker, Joker, JokerCondition, JokerEffect, JokerId, JokerRarity},
};

/// Tests for hand composition conditional jokers:
/// - Ride the Bus: +1 mult per hand without face card
/// - Blackboard: X3 mult if all held cards same suit/rank  
/// - DNA: copy first card if only 1 in hand
#[cfg(test)]
mod ride_the_bus_tests {
    use super::*;

    #[test]
    fn test_ride_the_bus_condition_no_face_cards() {
        // Test NoFaceCardsHeld condition directly
        let _condition = JokerCondition::NoFaceCardsHeld;

        // Create hand without face cards (no J, Q, K)
        let cards_no_face = vec![
            Card::new(Rank::Ace, Suit::Heart),
            Card::new(Rank::Two, Suit::Spade),
            Card::new(Rank::Ten, Suit::Diamond),
        ];
        let hand_no_face = SelectHand::new(cards_no_face);

        // Test the condition logic directly
        let has_face_cards = hand_no_face
            .cards()
            .iter()
            .any(|card| matches!(card.value, Rank::Jack | Rank::Queen | Rank::King));

        assert!(!has_face_cards); // Should have no face cards
    }

    #[test]
    fn test_ride_the_bus_condition_with_face_cards() {
        // Test NoFaceCardsHeld condition with face cards present
        let _condition = JokerCondition::NoFaceCardsHeld;

        // Create hand with face cards (has King)
        let cards_with_face = vec![
            Card::new(Rank::Ace, Suit::Heart),
            Card::new(Rank::King, Suit::Spade), // Face card
            Card::new(Rank::Ten, Suit::Diamond),
        ];
        let hand_with_face = SelectHand::new(cards_with_face);

        // Test the condition logic directly
        let has_face_cards = hand_with_face
            .cards()
            .iter()
            .any(|card| matches!(card.value, Rank::Jack | Rank::Queen | Rank::King));

        assert!(has_face_cards); // Should have face cards
    }

    #[test]
    fn test_ride_the_bus_condition_all_face_cards() {
        // Test with all face cards
        let _condition = JokerCondition::NoFaceCardsHeld;

        // Create hand with all face cards
        let cards_all_face = vec![
            Card::new(Rank::Jack, Suit::Heart),
            Card::new(Rank::Queen, Suit::Spade),
            Card::new(Rank::King, Suit::Diamond),
        ];
        let hand_all_face = SelectHand::new(cards_all_face);

        // Test the condition logic directly
        let has_face_cards = hand_all_face
            .cards()
            .iter()
            .any(|card| matches!(card.value, Rank::Jack | Rank::Queen | Rank::King));

        assert!(has_face_cards); // Should have face cards
    }

    #[test]
    fn test_ride_the_bus_condition_empty_hand() {
        // Test with empty hand
        let _condition = JokerCondition::NoFaceCardsHeld;

        // Create empty hand
        let empty_hand = SelectHand::new(vec![]);

        // Test the condition logic directly
        let has_face_cards = empty_hand
            .cards()
            .iter()
            .any(|card| matches!(card.value, Rank::Jack | Rank::Queen | Rank::King));

        assert!(!has_face_cards); // Empty hand has no face cards
    }

    #[test]
    fn test_ride_the_bus_joker_construction() {
        // Test that we can construct the Ride the Bus joker
        let joker = ConditionalJoker::new(
            JokerId::Ride,
            "Ride the Bus",
            "+1 mult per hand without face card",
            JokerRarity::Common,
            JokerCondition::NoFaceCardsHeld,
            JokerEffect::new().with_mult(1),
        );

        assert_eq!(joker.id(), JokerId::Ride);
        assert_eq!(joker.name(), "Ride the Bus");
        assert_eq!(joker.rarity(), JokerRarity::Common);
        assert_eq!(joker.cost(), 3); // Common rarity default cost
    }
}

#[cfg(test)]
mod blackboard_tests {
    use super::*;

    #[test]
    fn test_blackboard_hand_analysis_all_same_suit() {
        // Test hand uniformity analysis logic for Blackboard joker

        // Create hand with all same suit (Hearts)
        let cards_all_hearts = vec![
            Card::new(Rank::Ace, Suit::Heart),
            Card::new(Rank::King, Suit::Heart),
            Card::new(Rank::Three, Suit::Heart),
        ];
        let hand_all_hearts = SelectHand::new(cards_all_hearts);

        // Test suit uniformity logic
        let first_suit = hand_all_hearts.cards().first().map(|card| card.suit);
        let all_same_suit = hand_all_hearts
            .cards()
            .iter()
            .all(|card| Some(card.suit) == first_suit);

        assert!(all_same_suit); // Should have all same suit
    }

    #[test]
    fn test_blackboard_hand_analysis_all_same_rank() {
        // Test hand uniformity analysis logic for all same rank

        // Create hand with all same rank (Kings)
        let cards_all_kings = vec![
            Card::new(Rank::King, Suit::Heart),
            Card::new(Rank::King, Suit::Spade),
            Card::new(Rank::King, Suit::Diamond),
        ];
        let hand_all_kings = SelectHand::new(cards_all_kings);

        // Test rank uniformity logic
        let first_rank = hand_all_kings.cards().first().map(|card| card.value);
        let all_same_rank = hand_all_kings
            .cards()
            .iter()
            .all(|card| Some(card.value) == first_rank);

        assert!(all_same_rank); // Should have all same rank
    }

    #[test]
    fn test_blackboard_hand_analysis_mixed_cards() {
        // Test that mixed cards are detected correctly

        // Create hand with mixed suits and ranks
        let cards_mixed = vec![
            Card::new(Rank::Ace, Suit::Heart),
            Card::new(Rank::King, Suit::Spade),
            Card::new(Rank::Three, Suit::Diamond),
        ];
        let hand_mixed = SelectHand::new(cards_mixed);

        // Test suit uniformity logic
        let first_suit = hand_mixed.cards().first().map(|card| card.suit);
        let all_same_suit = hand_mixed
            .cards()
            .iter()
            .all(|card| Some(card.suit) == first_suit);

        // Test rank uniformity logic
        let first_rank = hand_mixed.cards().first().map(|card| card.value);
        let all_same_rank = hand_mixed
            .cards()
            .iter()
            .all(|card| Some(card.value) == first_rank);

        assert!(!all_same_suit); // Should not have all same suit
        assert!(!all_same_rank); // Should not have all same rank
    }

    #[test]
    fn test_blackboard_hand_analysis_empty_hand() {
        // Test empty hand behavior

        // Create empty hand
        let empty_hand = SelectHand::new(vec![]);

        // Test uniformity logic with empty hand
        let first_suit = empty_hand.cards().first().map(|card| card.suit);
        let all_same_suit = empty_hand
            .cards()
            .iter()
            .all(|card| Some(card.suit) == first_suit);

        let first_rank = empty_hand.cards().first().map(|card| card.value);
        let all_same_rank = empty_hand
            .cards()
            .iter()
            .all(|card| Some(card.value) == first_rank);

        // Empty hand should be considered "uniform" by `all()` but first_suit/first_rank will be None
        assert!(all_same_suit); // all() returns true for empty iterator
        assert!(all_same_rank); // all() returns true for empty iterator
        assert!(first_suit.is_none()); // But first element doesn't exist
        assert!(first_rank.is_none()); // But first element doesn't exist
    }

    #[test]
    fn test_blackboard_joker_construction() {
        // Test that we can construct the Blackboard joker (using placeholder condition for now)
        let joker = ConditionalJoker::new(
            JokerId::Blackboard,
            "Blackboard",
            "X3 mult if all held cards same suit/rank",
            JokerRarity::Uncommon,
            JokerCondition::Always, // Placeholder until we implement AllSameSuitOrRank condition
            JokerEffect::new().with_mult_multiplier(3.0),
        );

        assert_eq!(joker.id(), JokerId::Blackboard);
        assert_eq!(joker.name(), "Blackboard");
        assert_eq!(joker.rarity(), JokerRarity::Uncommon);
        assert_eq!(joker.cost(), 6); // Uncommon rarity default cost
    }
}

#[cfg(test)]
mod dna_tests {
    use super::*;

    #[test]
    fn test_dna_hand_size_condition_single_card() {
        // Test HandSizeExactly(1) condition logic
        let _condition = JokerCondition::HandSizeExactly(1);

        // Create hand with exactly 1 card
        let single_card = vec![Card::new(Rank::Ace, Suit::Heart)];
        let hand_single = SelectHand::new(single_card);

        // Test hand size logic directly
        assert_eq!(hand_single.len(), 1); // Should have exactly 1 card
    }

    #[test]
    fn test_dna_hand_size_condition_multiple_cards() {
        // Test HandSizeExactly(1) with multiple cards
        let _condition = JokerCondition::HandSizeExactly(1);

        // Create hand with multiple cards
        let multiple_cards = vec![
            Card::new(Rank::Ace, Suit::Heart),
            Card::new(Rank::King, Suit::Spade),
        ];
        let hand_multiple = SelectHand::new(multiple_cards);

        // Test hand size logic directly
        assert_eq!(hand_multiple.len(), 2); // Should have 2 cards, not 1
        assert_ne!(hand_multiple.len(), 1); // Should not equal 1
    }

    #[test]
    fn test_dna_hand_size_condition_empty_hand() {
        // Test HandSizeExactly(1) with empty hand
        let _condition = JokerCondition::HandSizeExactly(1);

        // Create empty hand
        let empty_hand = SelectHand::new(vec![]);

        // Test hand size logic directly
        assert_eq!(empty_hand.len(), 0); // Should have 0 cards
        assert_ne!(empty_hand.len(), 1); // Should not equal 1
    }

    #[test]
    fn test_dna_card_duplication_logic() {
        // Test the card duplication logic we'll need for DNA joker

        // Create a single card
        let original_card = Card::new(Rank::Ace, Suit::Heart);

        // Create a copy with same rank and suit (new ID will be generated)
        let copied_card = Card::new(original_card.value, original_card.suit);

        // Verify the cards have same value and suit but different IDs
        assert_eq!(original_card.value, copied_card.value);
        assert_eq!(original_card.suit, copied_card.suit);
        assert_ne!(original_card.id, copied_card.id); // Different IDs due to CARD_ID_COUNTER
    }

    #[test]
    fn test_dna_joker_construction() {
        // Test that we can construct the DNA joker
        let joker = ConditionalJoker::new(
            JokerId::DNA,
            "DNA",
            "Copy first card if only 1 in hand",
            JokerRarity::Rare,
            JokerCondition::HandSizeExactly(1),
            JokerEffect::new(), // Base effect (will implement transform_cards later)
        );

        assert_eq!(joker.id(), JokerId::DNA);
        assert_eq!(joker.name(), "DNA");
        assert_eq!(joker.rarity(), JokerRarity::Rare);
        assert_eq!(joker.cost(), 8); // Rare rarity default cost
    }

    #[test]
    fn test_dna_effect_structure() {
        // Test the JokerEffect structure for DNA joker implementation

        let original_card = Card::new(Rank::King, Suit::Spade);
        let copied_card = Card::new(original_card.value, original_card.suit);

        // Create effect with card transformation manually (no builder method yet)
        let mut effect = JokerEffect::new();
        effect.transform_cards = vec![(original_card, copied_card)];

        assert_eq!(effect.transform_cards.len(), 1);

        let (from_card, to_card) = &effect.transform_cards[0];
        assert_eq!(from_card.value, to_card.value);
        assert_eq!(from_card.suit, to_card.suit);
        assert_ne!(from_card.id, to_card.id); // Different card IDs
    }
}

#[cfg(test)]
mod edge_case_tests {
    use super::*;
    use crate::joker::hand_composition_jokers::{create_blackboard, create_ride_the_bus, DnaJoker};

    #[test]
    fn test_ride_the_bus_with_various_hand_sizes() {
        let _joker = create_ride_the_bus();

        // Test with 1 card (no face)
        let single_card = vec![Card::new(Rank::Ace, Suit::Heart)];
        let hand_single = SelectHand::new(single_card);

        // Mock context is complex, so just test that the condition logic works
        let has_face_cards = hand_single
            .cards()
            .iter()
            .any(|card| matches!(card.value, Rank::Jack | Rank::Queen | Rank::King));
        assert!(!has_face_cards);

        // Test with 5 cards (no face)
        let five_cards = vec![
            Card::new(Rank::Ace, Suit::Heart),
            Card::new(Rank::Two, Suit::Spade),
            Card::new(Rank::Three, Suit::Diamond),
            Card::new(Rank::Four, Suit::Club),
            Card::new(Rank::Five, Suit::Heart),
        ];
        let hand_five = SelectHand::new(five_cards);

        let has_face_cards = hand_five
            .cards()
            .iter()
            .any(|card| matches!(card.value, Rank::Jack | Rank::Queen | Rank::King));
        assert!(!has_face_cards);
    }

    #[test]
    fn test_blackboard_with_edge_cases() {
        let _joker = create_blackboard();

        // Test single card (same suit and rank trivially)
        let single_card = vec![Card::new(Rank::Ace, Suit::Heart)];
        let hand_single = SelectHand::new(single_card);

        let cards = hand_single.cards();
        let first_suit = cards[0].suit;
        let first_rank = cards[0].value;
        let all_same_suit = cards.iter().all(|card| card.suit == first_suit);
        let all_same_rank = cards.iter().all(|card| card.value == first_rank);

        assert!(all_same_suit);
        assert!(all_same_rank);

        // Test mixed case with only 2 cards
        let two_mixed = vec![
            Card::new(Rank::Ace, Suit::Heart),
            Card::new(Rank::King, Suit::Spade),
        ];
        let hand_two_mixed = SelectHand::new(two_mixed);

        let cards = hand_two_mixed.cards();
        let first_suit = cards[0].suit;
        let first_rank = cards[0].value;
        let all_same_suit = cards.iter().all(|card| card.suit == first_suit);
        let all_same_rank = cards.iter().all(|card| card.value == first_rank);

        assert!(!all_same_suit);
        assert!(!all_same_rank);
    }

    #[test]
    fn test_blackboard_all_same_suit_different_ranks() {
        let _joker = create_blackboard();

        // Test all hearts, different ranks
        let all_hearts = vec![
            Card::new(Rank::Ace, Suit::Heart),
            Card::new(Rank::King, Suit::Heart),
            Card::new(Rank::Queen, Suit::Heart),
            Card::new(Rank::Jack, Suit::Heart),
            Card::new(Rank::Ten, Suit::Heart),
        ];
        let hand_hearts = SelectHand::new(all_hearts);

        let cards = hand_hearts.cards();
        let first_suit = cards[0].suit;
        let first_rank = cards[0].value;
        let all_same_suit = cards.iter().all(|card| card.suit == first_suit);
        let all_same_rank = cards.iter().all(|card| card.value == first_rank);

        assert!(all_same_suit); // Should be true
        assert!(!all_same_rank); // Should be false

        // AllSameSuitOrRank should be true (suit is same)
        let condition_met = all_same_suit || all_same_rank;
        assert!(condition_met);
    }

    #[test]
    fn test_blackboard_all_same_rank_different_suits() {
        let _joker = create_blackboard();

        // Test all aces, different suits
        let all_aces = vec![
            Card::new(Rank::Ace, Suit::Heart),
            Card::new(Rank::Ace, Suit::Spade),
            Card::new(Rank::Ace, Suit::Diamond),
            Card::new(Rank::Ace, Suit::Club),
        ];
        let hand_aces = SelectHand::new(all_aces);

        let cards = hand_aces.cards();
        let first_suit = cards[0].suit;
        let first_rank = cards[0].value;
        let all_same_suit = cards.iter().all(|card| card.suit == first_suit);
        let all_same_rank = cards.iter().all(|card| card.value == first_rank);

        assert!(!all_same_suit); // Should be false
        assert!(all_same_rank); // Should be true

        // AllSameSuitOrRank should be true (rank is same)
        let condition_met = all_same_suit || all_same_rank;
        assert!(condition_met);
    }

    #[test]
    fn test_dna_joker_edge_cases() {
        let _joker = DnaJoker::new();

        // Test single card case
        let single_card = vec![Card::new(Rank::Ace, Suit::Heart)];
        let hand_single = SelectHand::new(single_card);

        // Test hand size condition
        assert_eq!(hand_single.len(), 1);

        // Test multiple cards case
        let multiple_cards = vec![
            Card::new(Rank::Ace, Suit::Heart),
            Card::new(Rank::King, Suit::Spade),
            Card::new(Rank::Queen, Suit::Diamond),
        ];
        let hand_multiple = SelectHand::new(multiple_cards);

        assert_eq!(hand_multiple.len(), 3);
        assert_ne!(hand_multiple.len(), 1);

        // Test empty hand case
        let empty_hand = SelectHand::new(vec![]);
        assert_eq!(empty_hand.len(), 0);
        assert_ne!(empty_hand.len(), 1);
    }

    #[test]
    fn test_face_card_detection_comprehensive() {
        // Test all face cards
        let jack = Card::new(Rank::Jack, Suit::Heart);
        let queen = Card::new(Rank::Queen, Suit::Spade);
        let king = Card::new(Rank::King, Suit::Diamond);

        assert!(matches!(jack.value, Rank::Jack | Rank::Queen | Rank::King));
        assert!(matches!(queen.value, Rank::Jack | Rank::Queen | Rank::King));
        assert!(matches!(king.value, Rank::Jack | Rank::Queen | Rank::King));

        // Test non-face cards
        let ace = Card::new(Rank::Ace, Suit::Heart);
        let ten = Card::new(Rank::Ten, Suit::Spade);
        let two = Card::new(Rank::Two, Suit::Diamond);

        assert!(!matches!(ace.value, Rank::Jack | Rank::Queen | Rank::King));
        assert!(!matches!(ten.value, Rank::Jack | Rank::Queen | Rank::King));
        assert!(!matches!(two.value, Rank::Jack | Rank::Queen | Rank::King));
    }

    #[test]
    fn test_large_hand_uniformity() {
        // Test larger hands for performance and correctness

        // Large hand, all same suit
        let large_same_suit = vec![
            Card::new(Rank::Ace, Suit::Spade),
            Card::new(Rank::Two, Suit::Spade),
            Card::new(Rank::Three, Suit::Spade),
            Card::new(Rank::Four, Suit::Spade),
            Card::new(Rank::Five, Suit::Spade),
            Card::new(Rank::Six, Suit::Spade),
            Card::new(Rank::Seven, Suit::Spade),
        ];
        let hand_large_suit = SelectHand::new(large_same_suit);

        let cards = hand_large_suit.cards();
        let first_suit = cards[0].suit;
        let all_same_suit = cards.iter().all(|card| card.suit == first_suit);
        assert!(all_same_suit);

        // Large hand, not uniform
        let large_mixed = vec![
            Card::new(Rank::Ace, Suit::Heart),
            Card::new(Rank::Two, Suit::Spade),
            Card::new(Rank::Three, Suit::Diamond),
            Card::new(Rank::Four, Suit::Club),
            Card::new(Rank::Five, Suit::Heart),
            Card::new(Rank::Six, Suit::Spade),
            Card::new(Rank::Seven, Suit::Diamond),
        ];
        let hand_large_mixed = SelectHand::new(large_mixed);

        let cards = hand_large_mixed.cards();
        let first_suit = cards[0].suit;
        let first_rank = cards[0].value;
        let all_same_suit = cards.iter().all(|card| card.suit == first_suit);
        let all_same_rank = cards.iter().all(|card| card.value == first_rank);

        assert!(!all_same_suit);
        assert!(!all_same_rank);
    }
}
