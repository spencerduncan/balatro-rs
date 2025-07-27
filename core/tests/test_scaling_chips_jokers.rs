use balatro_rs::card::{Card, Suit, Value};
use balatro_rs::hand::{Hand, SelectHand};
use balatro_rs::joker::{GameContext, Joker};
use balatro_rs::joker_state::JokerStateManager;
use balatro_rs::rank::HandRank;
use balatro_rs::scaling_chips_jokers::{
    ArrowheadJoker, CastleJoker, HikerJoker, OddToddJoker, ScholarJoker, StuntmanJoker, WeeJoker,
};
use balatro_rs::stage::{Blind, Stage};
use std::collections::HashMap;
use std::sync::{Arc, OnceLock};

/// Test helper to create a basic GameContext for testing
fn create_test_context() -> GameContext<'static> {
    create_test_context_with_round(1)
}

/// Test helper to create a GameContext with specific round number  
fn create_test_context_with_round(round: u32) -> GameContext<'static> {
    static STAGE: Stage = Stage::Blind(Blind::Small);
    static HAND: OnceLock<Hand> = OnceLock::new();
    let hand = HAND.get_or_init(|| Hand::new(Vec::new()));

    static HAND_TYPE_COUNTS: OnceLock<HashMap<HandRank, u32>> = OnceLock::new();
    let hand_type_counts = HAND_TYPE_COUNTS.get_or_init(HashMap::new);

    static JOKER_STATE_MANAGER: OnceLock<Arc<JokerStateManager>> = OnceLock::new();
    let joker_state_manager =
        JOKER_STATE_MANAGER.get_or_init(|| Arc::new(JokerStateManager::new()));

    // Create static RNG for testing
    static TEST_RNG: OnceLock<balatro_rs::rng::GameRng> = OnceLock::new();
    let rng = TEST_RNG.get_or_init(|| balatro_rs::rng::GameRng::for_testing(42));

    GameContext {
        chips: 0,
        mult: 1,
        money: 10,
        ante: 1,
        round,
        stage: &STAGE,
        hands_played: 0,
        discards_used: 0,
        jokers: &[],
        hand,
        discarded: &[],
        hand_type_counts,
        joker_state_manager,
        cards_in_deck: 52,
        stone_cards_in_deck: 0,
        steel_cards_in_deck: 0,
        rng,
    }
}

/// Test helper to create a test card
fn create_test_card(suit: Suit, value: Value) -> Card {
    Card::new(value, suit)
}

#[cfg(test)]
mod castle_joker_tests {
    use super::*;

    #[test]
    fn test_castle_joker_identity() {
        let joker = CastleJoker::new();

        assert_eq!(joker.name(), "Castle");
        assert_eq!(joker.cost(), 8);
        assert_eq!(joker.rarity(), balatro_rs::joker::JokerRarity::Rare);
    }

    #[test]
    fn test_castle_joker_suit_cycling() {
        // Test that target suit changes based on round number
        let joker = CastleJoker::new();
        let mut context_round_0 = create_test_context_with_round(0);
        let mut context_round_1 = create_test_context_with_round(1);
        let mut context_round_2 = create_test_context_with_round(2);
        let mut context_round_3 = create_test_context_with_round(3);
        let mut context_round_4 = create_test_context_with_round(4);

        let hand = SelectHand::new(vec![]);

        // Round 0 % 4 = 0 -> Heart
        let effect_0 = joker.on_hand_played(&mut context_round_0, &hand);
        assert!(effect_0.message.as_ref().unwrap().contains("Heart"));

        // Round 1 % 4 = 1 -> Diamond
        let effect_1 = joker.on_hand_played(&mut context_round_1, &hand);
        assert!(effect_1.message.as_ref().unwrap().contains("Diamond"));

        // Round 2 % 4 = 2 -> Club
        let effect_2 = joker.on_hand_played(&mut context_round_2, &hand);
        assert!(effect_2.message.as_ref().unwrap().contains("Club"));

        // Round 3 % 4 = 3 -> Spade
        let effect_3 = joker.on_hand_played(&mut context_round_3, &hand);
        assert!(effect_3.message.as_ref().unwrap().contains("Spade"));

        // Round 4 % 4 = 0 -> Heart (cycles back)
        let effect_4 = joker.on_hand_played(&mut context_round_4, &hand);
        assert!(effect_4.message.as_ref().unwrap().contains("Heart"));
    }

    #[test]
    fn test_castle_joker_discard_scaling() {
        let joker = CastleJoker::new();
        let mut context = create_test_context_with_round(0); // Heart round

        // Discard some hearts
        let heart_cards = vec![
            create_test_card(Suit::Heart, Value::Ace),
            create_test_card(Suit::Heart, Value::King),
        ];

        let effect = joker.on_discard(&mut context, &heart_cards);
        assert!(effect.message.is_some());
        assert!(effect.message.as_ref().unwrap().contains("+6 Chips gained"));
    }
}

#[cfg(test)]
mod wee_joker_tests {
    use super::*;

    #[test]
    fn test_wee_joker_identity() {
        let joker = WeeJoker::new();

        assert_eq!(joker.name(), "Wee Joker");
        assert_eq!(joker.cost(), 8);
        assert_eq!(joker.rarity(), balatro_rs::joker::JokerRarity::Rare);
    }

    #[test]
    fn test_wee_joker_two_scoring() {
        let joker = WeeJoker::new();
        let mut context = create_test_context();

        // Score a Two
        let two_card = create_test_card(Suit::Heart, Value::Two);
        let effect = joker.on_card_scored(&mut context, &two_card);

        assert!(effect.message.is_some());
        assert!(effect.message.as_ref().unwrap().contains("+8 Chips gained"));
    }

    #[test]
    fn test_wee_joker_non_two_scoring() {
        let joker = WeeJoker::new();
        let mut context = create_test_context();

        // Score a non-Two
        let ace_card = create_test_card(Suit::Heart, Value::Ace);
        let effect = joker.on_card_scored(&mut context, &ace_card);

        assert!(effect.message.is_none());
    }
}

#[cfg(test)]
mod stuntman_joker_tests {
    use super::*;

    #[test]
    fn test_stuntman_joker_identity() {
        let joker = StuntmanJoker::new();

        assert_eq!(joker.name(), "Stuntman");
        assert_eq!(joker.cost(), 8);
        assert_eq!(joker.rarity(), balatro_rs::joker::JokerRarity::Rare);
    }

    #[test]
    fn test_stuntman_joker_flat_bonus() {
        let joker = StuntmanJoker::new();
        let mut context = create_test_context();
        let hand = SelectHand::new(vec![]);

        let effect = joker.on_hand_played(&mut context, &hand);

        assert_eq!(effect.chips, 300);
        assert!(effect.message.as_ref().unwrap().contains("+300 Chips"));
    }

    #[test]
    fn test_stuntman_joker_hand_size_reduction() {
        let joker = StuntmanJoker::new();
        let context = create_test_context();

        let base_hand_size = 8;
        let modified_size = joker.modify_hand_size(&context, base_hand_size);

        assert_eq!(modified_size, 6); // 8 - 2 = 6
    }

    #[test]
    fn test_stuntman_joker_hand_size_reduction_edge_case() {
        let joker = StuntmanJoker::new();
        let context = create_test_context();

        // Test edge case where hand size is very small
        let base_hand_size = 1;
        let modified_size = joker.modify_hand_size(&context, base_hand_size);

        assert_eq!(modified_size, 0); // saturating_sub prevents underflow
    }
}

#[cfg(test)]
mod hiker_joker_tests {
    use super::*;

    #[test]
    fn test_hiker_joker_identity() {
        let joker = HikerJoker::new();

        assert_eq!(joker.name(), "Hiker");
        assert_eq!(joker.cost(), 6);
        assert_eq!(joker.rarity(), balatro_rs::joker::JokerRarity::Uncommon);
    }

    #[test]
    fn test_hiker_joker_card_enhancement() {
        let joker = HikerJoker::new();
        let mut context = create_test_context();

        let card = create_test_card(Suit::Heart, Value::Ace);
        let effect = joker.on_card_scored(&mut context, &card);

        assert_eq!(effect.chips, 5);
        assert!(effect.message.as_ref().unwrap().contains("+5 Chips"));
    }
}

#[cfg(test)]
mod odd_todd_joker_tests {
    use super::*;

    #[test]
    fn test_odd_todd_joker_identity() {
        let joker = OddToddJoker::new();

        assert_eq!(joker.name(), "Odd Todd");
        assert_eq!(joker.cost(), 3);
        assert_eq!(joker.rarity(), balatro_rs::joker::JokerRarity::Common);
    }

    #[test]
    fn test_odd_todd_joker_odd_ranks() {
        let joker = OddToddJoker::new();
        let mut context = create_test_context();

        // Test odd ranks
        let odd_cards = vec![
            (Value::Ace, true),
            (Value::Three, true),
            (Value::Five, true),
            (Value::Seven, true),
            (Value::Nine, true),
        ];

        for (value, should_trigger) in odd_cards {
            let card = create_test_card(Suit::Heart, value);
            let effect = joker.on_card_scored(&mut context, &card);

            if should_trigger {
                assert_eq!(effect.chips, 30);
                assert!(effect.message.as_ref().unwrap().contains("+30 Chips"));
            } else {
                assert_eq!(effect.chips, 0);
                assert!(effect.message.is_none());
            }
        }
    }

    #[test]
    fn test_odd_todd_joker_even_ranks() {
        let joker = OddToddJoker::new();
        let mut context = create_test_context();

        // Test even ranks (should not trigger)
        let even_cards = vec![
            Value::Two,
            Value::Four,
            Value::Six,
            Value::Eight,
            Value::Ten,
            Value::Jack,
            Value::Queen,
            Value::King,
        ];

        for value in even_cards {
            let card = create_test_card(Suit::Heart, value);
            let effect = joker.on_card_scored(&mut context, &card);

            assert_eq!(effect.chips, 0);
            assert!(effect.message.is_none());
        }
    }
}

#[cfg(test)]
mod arrowhead_joker_tests {
    use super::*;

    #[test]
    fn test_arrowhead_joker_identity() {
        let joker = ArrowheadJoker::new();

        assert_eq!(joker.name(), "Arrowhead");
        assert_eq!(joker.cost(), 3);
        assert_eq!(joker.rarity(), balatro_rs::joker::JokerRarity::Common);
    }

    #[test]
    fn test_arrowhead_joker_spade_cards() {
        let joker = ArrowheadJoker::new();
        let mut context = create_test_context();

        // Test Spade card
        let spade_card = create_test_card(Suit::Spade, Value::Ace);
        let effect = joker.on_card_scored(&mut context, &spade_card);

        assert_eq!(effect.chips, 50);
        assert!(effect.message.as_ref().unwrap().contains("+50 Chips"));
    }

    #[test]
    fn test_arrowhead_joker_non_spade_cards() {
        let joker = ArrowheadJoker::new();
        let mut context = create_test_context();

        // Test non-Spade cards
        let non_spade_suits = vec![Suit::Heart, Suit::Diamond, Suit::Club];

        for suit in non_spade_suits {
            let card = create_test_card(suit, Value::Ace);
            let effect = joker.on_card_scored(&mut context, &card);

            assert_eq!(effect.chips, 0);
            assert!(effect.message.is_none());
        }
    }
}

#[cfg(test)]
mod scholar_joker_tests {
    use super::*;

    #[test]
    fn test_scholar_joker_identity() {
        let joker = ScholarJoker::new();

        assert_eq!(joker.name(), "Scholar");
        assert_eq!(joker.cost(), 3);
        assert_eq!(joker.rarity(), balatro_rs::joker::JokerRarity::Common);
    }

    #[test]
    fn test_scholar_joker_ace_cards() {
        let joker = ScholarJoker::new();
        let mut context = create_test_context();

        // Test Ace card
        let ace_card = create_test_card(Suit::Heart, Value::Ace);
        let effect = joker.on_card_scored(&mut context, &ace_card);

        assert_eq!(effect.chips, 20);
        assert_eq!(effect.mult, 4);
        assert!(effect
            .message
            .as_ref()
            .unwrap()
            .contains("+20 Chips, +4 Mult"));
    }

    #[test]
    fn test_scholar_joker_non_ace_cards() {
        let joker = ScholarJoker::new();
        let mut context = create_test_context();

        // Test non-Ace cards
        let non_ace_values = vec![
            Value::Two,
            Value::Three,
            Value::Four,
            Value::Five,
            Value::Six,
            Value::Seven,
            Value::Eight,
            Value::Nine,
            Value::Ten,
            Value::Jack,
            Value::Queen,
            Value::King,
        ];

        for value in non_ace_values {
            let card = create_test_card(Suit::Heart, value);
            let effect = joker.on_card_scored(&mut context, &card);

            assert_eq!(effect.chips, 0);
            assert_eq!(effect.mult, 0);
            assert!(effect.message.is_none());
        }
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_all_scaling_chips_jokers_create_successfully() {
        // Test that all jokers can be created without panicking
        let _castle = CastleJoker::new();
        let _wee = WeeJoker::new();
        let _stuntman = StuntmanJoker::new();
        let _hiker = HikerJoker::new();
        let _odd_todd = OddToddJoker::new();
        let _arrowhead = ArrowheadJoker::new();
        let _scholar = ScholarJoker::new();
    }

    #[test]
    fn test_rarity_distribution() {
        // Test that jokers have expected rarities
        assert_eq!(
            CastleJoker::new().rarity(),
            balatro_rs::joker::JokerRarity::Rare
        );
        assert_eq!(
            WeeJoker::new().rarity(),
            balatro_rs::joker::JokerRarity::Rare
        );
        assert_eq!(
            StuntmanJoker::new().rarity(),
            balatro_rs::joker::JokerRarity::Rare
        );
        assert_eq!(
            HikerJoker::new().rarity(),
            balatro_rs::joker::JokerRarity::Uncommon
        );
        assert_eq!(
            OddToddJoker::new().rarity(),
            balatro_rs::joker::JokerRarity::Common
        );
        assert_eq!(
            ArrowheadJoker::new().rarity(),
            balatro_rs::joker::JokerRarity::Common
        );
        assert_eq!(
            ScholarJoker::new().rarity(),
            balatro_rs::joker::JokerRarity::Common
        );
    }
}
