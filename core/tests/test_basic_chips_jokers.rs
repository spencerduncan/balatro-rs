use balatro_rs::basic_chips_jokers::{
    BannerJoker, BlueJoker, BullJoker, ScaryFaceJoker, StoneJoker,
};
use balatro_rs::card::{Card, Suit, Value};
use balatro_rs::hand::{Hand, SelectHand};
use balatro_rs::joker::{GameContext, Joker};
use balatro_rs::joker_state::JokerStateManager;
use balatro_rs::rank::HandRank;
use balatro_rs::stage::{Blind, Stage};
use std::collections::HashMap;
use std::sync::{Arc, OnceLock};

/// Test helper to create a basic GameContext for testing
fn create_test_context(money: i32, discards_used: u32) -> GameContext<'static> {
    create_test_context_with_deck(money, discards_used, 52)
}

/// Test helper to create a GameContext with specific deck size
fn create_test_context_with_deck(money: i32, discards_used: u32, cards_in_deck: usize) -> GameContext<'static> {
    create_test_context_with_deck_and_stones(money, discards_used, cards_in_deck, 0)
}

/// Test helper to create a GameContext with specific deck size and stone cards
fn create_test_context_with_deck_and_stones(money: i32, discards_used: u32, cards_in_deck: usize, stone_cards_in_deck: usize) -> GameContext<'static> {
    static STAGE: Stage = Stage::Blind(Blind::Small);
    static HAND: OnceLock<Hand> = OnceLock::new();
    let hand = HAND.get_or_init(|| Hand::new(Vec::new()));

    static HAND_TYPE_COUNTS: OnceLock<HashMap<HandRank, u32>> = OnceLock::new();
    let hand_type_counts = HAND_TYPE_COUNTS.get_or_init(|| HashMap::new());

    static JOKER_STATE_MANAGER: OnceLock<Arc<JokerStateManager>> = OnceLock::new();
    let joker_state_manager =
        JOKER_STATE_MANAGER.get_or_init(|| Arc::new(JokerStateManager::new()));

    GameContext {
        chips: 0,
        mult: 1,
        money,
        ante: 1,
        round: 1,
        stage: &STAGE,
        hands_played: 0,
        discards_used,
        jokers: &[],
        hand,
        discarded: &[],
        hand_type_counts,
        joker_state_manager,
        cards_in_deck,
        stone_cards_in_deck,
    }
}

/// Test helper to create a test card
fn create_test_card(suit: Suit, value: Value) -> Card {
    Card::new(value, suit)
}

/// Test helper to create a face card (Jack, Queen, King)
fn create_face_card(suit: Suit, face_value: Value) -> Card {
    assert!(matches!(
        face_value,
        Value::Jack | Value::Queen | Value::King
    ));
    Card::new(face_value, suit)
}

#[cfg(test)]
mod banner_joker_tests {
    use super::*;

    #[test]
    fn test_banner_joker_zero_discards_remaining() {
        // ARRANGE: No discards remaining (5 used out of 5 total)
        let joker = create_banner_joker();
        let mut context = create_test_context(10, 5);
        let hand = SelectHand::new(vec![]);

        // ACT: Trigger joker effect
        let effect = joker.on_hand_played(&mut context, &hand);

        // ASSERT: Should give 0 chips (30 * 0 remaining discards)
        assert_eq!(effect.chips, 0);
        assert_eq!(effect.mult, 0);
        assert_eq!(effect.money, 0);
    }

    #[test]
    fn test_banner_joker_three_discards_remaining() {
        // ARRANGE: 3 discards remaining (2 used out of 5 total)
        let joker = create_banner_joker();
        let mut context = create_test_context(10, 2);
        let hand = SelectHand::new(vec![]);

        // ACT: Trigger joker effect
        let effect = joker.on_hand_played(&mut context, &hand);

        // ASSERT: Should give 90 chips (30 * 3 remaining discards)
        assert_eq!(effect.chips, 90);
        assert_eq!(effect.mult, 0);
        assert_eq!(effect.money, 0);
    }

    #[test]
    fn test_banner_joker_max_discards() {
        // ARRANGE: All 5 discards remaining (0 used out of 5 total)
        let joker = create_banner_joker();
        let mut context = create_test_context(10, 0);
        let hand = SelectHand::new(vec![]);

        // ACT: Trigger joker effect
        let effect = joker.on_hand_played(&mut context, &hand);

        // ASSERT: Should give 150 chips (30 * 5 remaining discards)
        assert_eq!(effect.chips, 150);
    }
}

#[cfg(test)]
mod bull_joker_tests {
    use super::*;

    #[test]
    fn test_bull_joker_zero_money() {
        // ARRANGE: No money
        let joker = create_bull_joker();
        let mut context = create_test_context(0, 2);
        let hand = SelectHand::new(vec![]);

        // ACT: Trigger joker effect
        let effect = joker.on_hand_played(&mut context, &hand);

        // ASSERT: Should give 0 chips (2 * $0)
        assert_eq!(effect.chips, 0);
    }

    #[test]
    fn test_bull_joker_ten_dollars() {
        // ARRANGE: $10 owned
        let joker = create_bull_joker();
        let mut context = create_test_context(10, 2);
        let hand = SelectHand::new(vec![]);

        // ACT: Trigger joker effect
        let effect = joker.on_hand_played(&mut context, &hand);

        // ASSERT: Should give 20 chips (2 * $10)
        assert_eq!(effect.chips, 20);
    }

    #[test]
    fn test_bull_joker_fifty_dollars() {
        // ARRANGE: $50 owned
        let joker = create_bull_joker();
        let mut context = create_test_context(50, 2);
        let hand = SelectHand::new(vec![]);

        // ACT: Trigger joker effect
        let effect = joker.on_hand_played(&mut context, &hand);

        // ASSERT: Should give 100 chips (2 * $50)
        assert_eq!(effect.chips, 100);
    }
}

#[cfg(test)]
mod stone_joker_tests {
    use super::*;

    #[test]
    fn test_stone_joker_no_stone_cards() {
        // ARRANGE: No Stone cards in deck
        let joker = create_stone_joker();
        let mut context = create_test_context(10, 2);
        let hand = SelectHand::new(vec![]);
        // TODO: Set up deck with no Stone cards

        // ACT: Trigger joker effect
        let effect = joker.on_hand_played(&mut context, &hand);

        // ASSERT: Should give 0 chips (25 * 0 Stone cards)
        assert_eq!(effect.chips, 0);
    }

    #[test]
    fn test_stone_joker_three_stone_cards() {
        // ARRANGE: 3 Stone cards in deck
        let joker = create_stone_joker();
        let mut context = create_test_context_with_deck_and_stones(10, 2, 52, 3);
        let hand = SelectHand::new(vec![]);

        // ACT: Trigger joker effect
        let effect = joker.on_hand_played(&mut context, &hand);

        // ASSERT: Should give 75 chips (25 * 3 Stone cards)
        assert_eq!(effect.chips, 75);
    }
}

#[cfg(test)]
mod scary_face_joker_tests {
    use super::*;

    #[test]
    fn test_scary_face_joker_no_face_cards() {
        // ARRANGE: No face cards scored
        let joker = create_scary_face_joker();
        let mut context = create_test_context(10, 2);
        let card = create_test_card(Suit::Heart, Value::Two);

        // ACT: Trigger joker effect
        let effect = joker.on_card_scored(&mut context, &card);

        // ASSERT: Should give 0 chips (not a face card)
        assert_eq!(effect.chips, 0);
    }

    #[test]
    fn test_scary_face_joker_jack_scored() {
        // ARRANGE: Jack scored
        let joker = create_scary_face_joker();
        let mut context = create_test_context(10, 2);
        let jack = create_face_card(Suit::Spade, Value::Jack);

        // ACT: Trigger joker effect
        let effect = joker.on_card_scored(&mut context, &jack);

        // ASSERT: Should give 30 chips (face card bonus)
        assert_eq!(effect.chips, 30);
    }

    #[test]
    fn test_scary_face_joker_queen_scored() {
        // ARRANGE: Queen scored
        let joker = create_scary_face_joker();
        let mut context = create_test_context(10, 2);
        let queen = create_face_card(Suit::Diamond, Value::Queen);

        // ACT: Trigger joker effect
        let effect = joker.on_card_scored(&mut context, &queen);

        // ASSERT: Should give 30 chips (face card bonus)
        assert_eq!(effect.chips, 30);
    }

    #[test]
    fn test_scary_face_joker_king_scored() {
        // ARRANGE: King scored
        let joker = create_scary_face_joker();
        let mut context = create_test_context(10, 2);
        let king = create_face_card(Suit::Club, Value::King);

        // ACT: Trigger joker effect
        let effect = joker.on_card_scored(&mut context, &king);

        // ASSERT: Should give 30 chips (face card bonus)
        assert_eq!(effect.chips, 30);
    }
}

#[cfg(test)]
mod blue_joker_tests {
    use super::*;

    #[test]
    fn test_blue_joker_empty_deck() {
        // ARRANGE: No cards remaining in deck
        let joker = create_blue_joker();
        let mut context = create_test_context_with_deck(10, 2, 0);
        let hand = SelectHand::new(vec![]);

        // ACT: Trigger joker effect
        let effect = joker.on_hand_played(&mut context, &hand);

        // ASSERT: Should give 0 chips (2 * 0 cards in deck)
        assert_eq!(effect.chips, 0);
    }

    #[test]
    fn test_blue_joker_twenty_cards_in_deck() {
        // ARRANGE: 20 cards remaining in deck
        let joker = create_blue_joker();
        let mut context = create_test_context_with_deck(10, 2, 20);
        let hand = SelectHand::new(vec![]);

        // ACT: Trigger joker effect
        let effect = joker.on_hand_played(&mut context, &hand);

        // ASSERT: Should give 40 chips (2 * 20 cards in deck)
        assert_eq!(effect.chips, 40);
    }

    #[test]
    fn test_blue_joker_full_deck() {
        // ARRANGE: Full 52-card deck
        let joker = create_blue_joker();
        let mut context = create_test_context_with_deck(10, 2, 52);
        let hand = SelectHand::new(vec![]);

        // ACT: Trigger joker effect
        let effect = joker.on_hand_played(&mut context, &hand);

        // ASSERT: Should give 104 chips (2 * 52 cards in deck)
        assert_eq!(effect.chips, 104);
    }
}

// Factory functions for creating joker instances
fn create_banner_joker() -> Box<dyn Joker> {
    Box::new(BannerJoker::new())
}

fn create_bull_joker() -> Box<dyn Joker> {
    Box::new(BullJoker::new())
}

fn create_stone_joker() -> Box<dyn Joker> {
    Box::new(StoneJoker::new())
}

fn create_scary_face_joker() -> Box<dyn Joker> {
    Box::new(ScaryFaceJoker::new())
}

fn create_blue_joker() -> Box<dyn Joker> {
    Box::new(BlueJoker::new())
}
