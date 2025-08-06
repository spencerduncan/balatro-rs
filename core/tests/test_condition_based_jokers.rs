// Tests for condition-based jokers migrated to StaticJoker framework (Issue #676 Phase 3.2)
//
// This test suite validates that migrated condition-based jokers work exactly as before
// and comprehensively tests edge cases and boundary conditions

use balatro_rs::card::{Card, Suit, Value};
use balatro_rs::hand::{Hand, SelectHand};
use balatro_rs::joker::{GameContext, Joker, JokerId, JokerRarity};
use balatro_rs::joker_state::JokerStateManager;
use balatro_rs::rng::GameRng;
use balatro_rs::stage::{Blind, Stage};
use balatro_rs::static_joker_factory::StaticJokerFactory;
use std::collections::HashMap;
use std::sync::Arc;

// Helper function to create test context with all required fields
fn create_test_context() -> GameContext<'static> {
    let stage = Box::leak(Box::new(Stage::Blind(Blind::Small)));
    let hand = Box::leak(Box::new(Hand::new(vec![])));
    let jokers: &'static [Box<dyn Joker>] = Box::leak(Box::new([]));
    let discarded: &'static [Card] = Box::leak(Box::new([]));
    let joker_state_manager = Box::leak(Box::new(Arc::new(JokerStateManager::new())));
    let hand_type_counts = Box::leak(Box::new(HashMap::new()));
    let rng = Box::leak(Box::new(GameRng::for_testing(12345)));

    GameContext {
        chips: 0,
        mult: 1,
        money: 5,
        ante: 1,
        round: 1,
        stage,
        hands_played: 0,
        hands_remaining: 4.0,
        discards_used: 0,
        is_final_hand: false, // Test context
        jokers,
        hand,
        discarded,
        joker_state_manager,
        hand_type_counts,
        cards_in_deck: 52,
        stone_cards_in_deck: 0,
        steel_cards_in_deck: 0,
        enhanced_cards_in_deck: 0,
        rng,
    }
}

// Helper function to create test context with custom money
fn create_test_context_with_money(money: i32) -> GameContext<'static> {
    let mut context = create_test_context();
    context.money = money;
    context
}

// Helper function to create test context with custom deck size
fn create_test_context_with_deck_size(deck_size: usize) -> GameContext<'static> {
    let mut context = create_test_context();
    context.cards_in_deck = deck_size;
    context
}

// Helper function to create test context with custom stone cards count
fn create_test_context_with_stone_cards(stone_cards: usize) -> GameContext<'static> {
    let mut context = create_test_context();
    context.stone_cards_in_deck = stone_cards;
    context
}

// Helper function to create test context with custom discard usage
fn create_test_context_with_discards_used(discards_used: u32) -> GameContext<'static> {
    let mut context = create_test_context();
    context.discards_used = discards_used;
    context
}

// ============================================================================
// BULL JOKER TESTS - Money-based chips (+2 Chips per $1 owned)
// ============================================================================

#[test]
fn test_bull_joker_basic_properties() {
    let joker = StaticJokerFactory::create_bull_joker();

    assert_eq!(joker.id(), JokerId::BullMarket);
    assert_eq!(joker.name(), "Bull");
    assert_eq!(joker.description(), "+2 Chips per $1 owned");
    assert_eq!(joker.rarity(), JokerRarity::Common);
    assert_eq!(joker.cost(), 3);
}

#[test]
fn test_bull_joker_money_scaling() {
    let joker = StaticJokerFactory::create_bull_joker();
    let test_hand = SelectHand::new(vec![Card::new(Value::King, Suit::Heart)]);

    // Test with $0 - should give 0 chips (0 * 2)
    let mut context_0_money = create_test_context_with_money(0);
    let effect = joker.on_hand_played(&mut context_0_money, &test_hand);
    assert_eq!(effect.chips, 0, "Bull joker should give 0 chips with $0");
    assert_eq!(effect.mult, 0, "Bull joker should not give mult");

    // Test with $1 - should give 2 chips (1 * 2)
    let mut context_1_money = create_test_context_with_money(1);
    let effect = joker.on_hand_played(&mut context_1_money, &test_hand);
    assert_eq!(effect.chips, 2, "Bull joker should give 2 chips with $1");

    // Test with $5 - should give 10 chips (5 * 2)
    let mut context_5_money = create_test_context_with_money(5);
    let effect = joker.on_hand_played(&mut context_5_money, &test_hand);
    assert_eq!(effect.chips, 10, "Bull joker should give 10 chips with $5");

    // Test with $25 - should give 50 chips (25 * 2)
    let mut context_25_money = create_test_context_with_money(25);
    let effect = joker.on_hand_played(&mut context_25_money, &test_hand);
    assert_eq!(effect.chips, 50, "Bull joker should give 50 chips with $25");

    // Test with $100 - should give 200 chips (100 * 2)
    let mut context_100_money = create_test_context_with_money(100);
    let effect = joker.on_hand_played(&mut context_100_money, &test_hand);
    assert_eq!(
        effect.chips, 200,
        "Bull joker should give 200 chips with $100"
    );
}

#[test]
fn test_bull_joker_per_hand_not_per_card() {
    let joker = StaticJokerFactory::create_bull_joker();
    let mut context = create_test_context_with_money(10);

    // Test on_card_scored - should return no effect since it's per-hand
    let card = Card::new(Value::King, Suit::Heart);
    let card_effect = joker.on_card_scored(&mut context, &card);
    assert_eq!(
        card_effect.chips, 0,
        "Bull joker should not trigger on individual cards"
    );

    // Test on_hand_played - should return effect since it's per-hand
    let test_hand = SelectHand::new(vec![card]);
    let hand_effect = joker.on_hand_played(&mut context, &test_hand);
    assert_eq!(hand_effect.chips, 20, "Bull joker should trigger on hands");
}

#[test]
fn test_bull_joker_edge_case_negative_money() {
    let joker = StaticJokerFactory::create_bull_joker();
    let test_hand = SelectHand::new(vec![Card::new(Value::King, Suit::Heart)]);

    // Test with negative money - should give 0 chips (clamped)
    let mut context_negative_money = create_test_context_with_money(-5);
    let effect = joker.on_hand_played(&mut context_negative_money, &test_hand);
    assert_eq!(
        effect.chips, 0,
        "Bull joker should give 0 chips with negative money"
    );
}

// ============================================================================
// BLUE JOKER TESTS - Deck size-based chips (+2 Chips per remaining card)
// ============================================================================

#[test]
fn test_blue_joker_basic_properties() {
    let joker = StaticJokerFactory::create_blue_joker();

    assert_eq!(joker.id(), JokerId::BlueJoker);
    assert_eq!(joker.name(), "Blue Joker");
    assert_eq!(joker.description(), "+2 Chips per remaining card in deck");
    assert_eq!(joker.rarity(), JokerRarity::Common);
    assert_eq!(joker.cost(), 3);
}

#[test]
fn test_blue_joker_deck_size_scaling() {
    let joker = StaticJokerFactory::create_blue_joker();
    let test_hand = SelectHand::new(vec![Card::new(Value::King, Suit::Heart)]);

    // Test with 0 cards - should give 0 chips (0 * 2)
    let mut context_0_cards = create_test_context_with_deck_size(0);
    let effect = joker.on_hand_played(&mut context_0_cards, &test_hand);
    assert_eq!(
        effect.chips, 0,
        "Blue joker should give 0 chips with empty deck"
    );

    // Test with 1 card - should give 2 chips (1 * 2)
    let mut context_1_card = create_test_context_with_deck_size(1);
    let effect = joker.on_hand_played(&mut context_1_card, &test_hand);
    assert_eq!(
        effect.chips, 2,
        "Blue joker should give 2 chips with 1 card"
    );

    // Test with 26 cards - should give 52 chips (26 * 2)
    let mut context_26_cards = create_test_context_with_deck_size(26);
    let effect = joker.on_hand_played(&mut context_26_cards, &test_hand);
    assert_eq!(
        effect.chips, 52,
        "Blue joker should give 52 chips with 26 cards"
    );

    // Test with 52 cards (full deck) - should give 104 chips (52 * 2)
    let mut context_52_cards = create_test_context_with_deck_size(52);
    let effect = joker.on_hand_played(&mut context_52_cards, &test_hand);
    assert_eq!(
        effect.chips, 104,
        "Blue joker should give 104 chips with full deck"
    );
}

#[test]
fn test_blue_joker_per_hand_behavior() {
    let joker = StaticJokerFactory::create_blue_joker();
    let mut context = create_test_context_with_deck_size(20);

    // Test on_card_scored - should return no effect since it's per-hand
    let card = Card::new(Value::King, Suit::Heart);
    let card_effect = joker.on_card_scored(&mut context, &card);
    assert_eq!(
        card_effect.chips, 0,
        "Blue joker should not trigger on individual cards"
    );

    // Test on_hand_played - should return effect since it's per-hand
    let test_hand = SelectHand::new(vec![card]);
    let hand_effect = joker.on_hand_played(&mut context, &test_hand);
    assert_eq!(hand_effect.chips, 40, "Blue joker should trigger on hands");
}

// ============================================================================
// STONE JOKER TESTS - Stone cards count-based chips (+25 Chips per Stone card)
// ============================================================================

#[test]
fn test_stone_joker_basic_properties() {
    let joker = StaticJokerFactory::create_stone_joker();

    assert_eq!(joker.id(), JokerId::Stone);
    assert_eq!(joker.name(), "Stone Joker");
    assert_eq!(joker.description(), "+25 Chips per Stone card in deck");
    assert_eq!(joker.rarity(), JokerRarity::Uncommon);
    assert_eq!(joker.cost(), 4);
}

#[test]
fn test_stone_joker_stone_cards_scaling() {
    let joker = StaticJokerFactory::create_stone_joker();
    let test_hand = SelectHand::new(vec![Card::new(Value::King, Suit::Heart)]);

    // Test with 0 stone cards - should give 0 chips (0 * 25)
    let mut context_0_stone = create_test_context_with_stone_cards(0);
    let effect = joker.on_hand_played(&mut context_0_stone, &test_hand);
    assert_eq!(
        effect.chips, 0,
        "Stone joker should give 0 chips with no stone cards"
    );

    // Test with 1 stone card - should give 25 chips (1 * 25)
    let mut context_1_stone = create_test_context_with_stone_cards(1);
    let effect = joker.on_hand_played(&mut context_1_stone, &test_hand);
    assert_eq!(
        effect.chips, 25,
        "Stone joker should give 25 chips with 1 stone card"
    );

    // Test with 4 stone cards - should give 100 chips (4 * 25)
    let mut context_4_stone = create_test_context_with_stone_cards(4);
    let effect = joker.on_hand_played(&mut context_4_stone, &test_hand);
    assert_eq!(
        effect.chips, 100,
        "Stone joker should give 100 chips with 4 stone cards"
    );

    // Test with 10 stone cards - should give 250 chips (10 * 25)
    let mut context_10_stone = create_test_context_with_stone_cards(10);
    let effect = joker.on_hand_played(&mut context_10_stone, &test_hand);
    assert_eq!(
        effect.chips, 250,
        "Stone joker should give 250 chips with 10 stone cards"
    );
}

#[test]
fn test_stone_joker_per_hand_behavior() {
    let joker = StaticJokerFactory::create_stone_joker();
    let mut context = create_test_context_with_stone_cards(3);

    // Test on_card_scored - should return no effect since it's per-hand
    let card = Card::new(Value::King, Suit::Heart);
    let card_effect = joker.on_card_scored(&mut context, &card);
    assert_eq!(
        card_effect.chips, 0,
        "Stone joker should not trigger on individual cards"
    );

    // Test on_hand_played - should return effect since it's per-hand
    let test_hand = SelectHand::new(vec![card]);
    let hand_effect = joker.on_hand_played(&mut context, &test_hand);
    assert_eq!(hand_effect.chips, 75, "Stone joker should trigger on hands");
}

// ============================================================================
// BANNER JOKER TESTS - Discard count-based chips (+30 Chips per remaining discard)
// ============================================================================

#[test]
fn test_banner_joker_basic_properties() {
    let joker = StaticJokerFactory::create_banner();

    assert_eq!(joker.id(), JokerId::Banner);
    assert_eq!(joker.name(), "Banner");
    assert_eq!(joker.description(), "+30 Chips for each remaining discard");
    assert_eq!(joker.rarity(), JokerRarity::Common);
    assert_eq!(joker.cost(), 3);
}

#[test]
fn test_banner_joker_discard_scaling() {
    let joker = StaticJokerFactory::create_banner();
    let test_hand = SelectHand::new(vec![Card::new(Value::King, Suit::Heart)]);

    // Assuming 5 maximum discards per round (standard Balatro rule)

    // Test with 0 discards used (5 remaining) - should give 150 chips (5 * 30)
    let mut context_0_used = create_test_context_with_discards_used(0);
    let effect = joker.on_hand_played(&mut context_0_used, &test_hand);
    assert_eq!(
        effect.chips, 150,
        "Banner joker should give 150 chips with 5 discards remaining"
    );

    // Test with 1 discard used (4 remaining) - should give 120 chips (4 * 30)
    let mut context_1_used = create_test_context_with_discards_used(1);
    let effect = joker.on_hand_played(&mut context_1_used, &test_hand);
    assert_eq!(
        effect.chips, 120,
        "Banner joker should give 120 chips with 4 discards remaining"
    );

    // Test with 3 discards used (2 remaining) - should give 60 chips (2 * 30)
    let mut context_3_used = create_test_context_with_discards_used(3);
    let effect = joker.on_hand_played(&mut context_3_used, &test_hand);
    assert_eq!(
        effect.chips, 60,
        "Banner joker should give 60 chips with 2 discards remaining"
    );

    // Test with 5 discards used (0 remaining) - should give 0 chips (0 * 30)
    let mut context_5_used = create_test_context_with_discards_used(5);
    let effect = joker.on_hand_played(&mut context_5_used, &test_hand);
    assert_eq!(
        effect.chips, 0,
        "Banner joker should give 0 chips with no discards remaining"
    );
}

#[test]
fn test_banner_joker_per_hand_behavior() {
    let joker = StaticJokerFactory::create_banner();
    let mut context = create_test_context_with_discards_used(2);

    // Test on_card_scored - should return no effect since it's per-hand
    let card = Card::new(Value::King, Suit::Heart);
    let card_effect = joker.on_card_scored(&mut context, &card);
    assert_eq!(
        card_effect.chips, 0,
        "Banner joker should not trigger on individual cards"
    );

    // Test on_hand_played - should return effect since it's per-hand
    let test_hand = SelectHand::new(vec![card]);
    let hand_effect = joker.on_hand_played(&mut context, &test_hand);
    assert_eq!(
        hand_effect.chips, 90,
        "Banner joker should trigger on hands with 3 remaining discards"
    );
}

// ============================================================================
// INTEGRATION TESTS - Multiple conditions and edge cases
// ============================================================================

#[test]
fn test_condition_jokers_work_together() {
    // Test that multiple condition-based jokers can work in the same game state
    let bull_joker = StaticJokerFactory::create_bull_joker();
    let blue_joker = StaticJokerFactory::create_blue_joker();
    let stone_joker = StaticJokerFactory::create_stone_joker();
    let banner_joker = StaticJokerFactory::create_banner();

    let mut context = create_test_context();
    context.money = 10; // Bull: 10 * 2 = 20 chips
    context.cards_in_deck = 15; // Blue: 15 * 2 = 30 chips
    context.stone_cards_in_deck = 2; // Stone: 2 * 25 = 50 chips
    context.discards_used = 1; // Banner: (5-1) * 30 = 120 chips

    let test_hand = SelectHand::new(vec![Card::new(Value::King, Suit::Heart)]);

    let bull_effect = bull_joker.on_hand_played(&mut context, &test_hand);
    let blue_effect = blue_joker.on_hand_played(&mut context, &test_hand);
    let stone_effect = stone_joker.on_hand_played(&mut context, &test_hand);
    let banner_effect = banner_joker.on_hand_played(&mut context, &test_hand);

    assert_eq!(
        bull_effect.chips, 20,
        "Bull joker should contribute 20 chips"
    );
    assert_eq!(
        blue_effect.chips, 30,
        "Blue joker should contribute 30 chips"
    );
    assert_eq!(
        stone_effect.chips, 50,
        "Stone joker should contribute 50 chips"
    );
    assert_eq!(
        banner_effect.chips, 120,
        "Banner joker should contribute 120 chips"
    );

    let total_chips =
        bull_effect.chips + blue_effect.chips + stone_effect.chips + banner_effect.chips;
    assert_eq!(total_chips, 220, "Total chips should be 220");
}

#[test]
fn test_condition_jokers_boundary_conditions() {
    // Test various boundary conditions for all migrated jokers
    let test_hand = SelectHand::new(vec![Card::new(Value::King, Suit::Heart)]);

    // Test maximum integer values (within reasonable game bounds)
    let mut max_context = create_test_context();
    max_context.money = 999;
    max_context.cards_in_deck = 100;
    max_context.stone_cards_in_deck = 52;
    max_context.discards_used = 0; // Maximum remaining discards

    let bull_joker = StaticJokerFactory::create_bull_joker();
    let blue_joker = StaticJokerFactory::create_blue_joker();
    let stone_joker = StaticJokerFactory::create_stone_joker();
    let banner_joker = StaticJokerFactory::create_banner();

    let bull_effect = bull_joker.on_hand_played(&mut max_context, &test_hand);
    let blue_effect = blue_joker.on_hand_played(&mut max_context, &test_hand);
    let stone_effect = stone_joker.on_hand_played(&mut max_context, &test_hand);
    let banner_effect = banner_joker.on_hand_played(&mut max_context, &test_hand);

    assert_eq!(
        bull_effect.chips, 1998,
        "Bull joker should handle large money values"
    );
    assert_eq!(
        blue_effect.chips, 200,
        "Blue joker should handle large deck sizes"
    );
    assert_eq!(
        stone_effect.chips, 1300,
        "Stone joker should handle many stone cards"
    );
    assert_eq!(
        banner_effect.chips, 150,
        "Banner joker should handle maximum discards"
    );
}

// ============================================================================
// PERFORMANCE AND CONSISTENCY TESTS
// ============================================================================

#[test]
fn test_condition_jokers_consistency() {
    // Test that jokers give consistent results across multiple calls
    let joker = StaticJokerFactory::create_bull_joker();
    let mut context = create_test_context_with_money(15);
    let test_hand = SelectHand::new(vec![Card::new(Value::King, Suit::Heart)]);

    // Call multiple times with same context
    let effect1 = joker.on_hand_played(&mut context, &test_hand);
    let effect2 = joker.on_hand_played(&mut context, &test_hand);
    let effect3 = joker.on_hand_played(&mut context, &test_hand);

    assert_eq!(
        effect1.chips, effect2.chips,
        "Bull joker should give consistent results"
    );
    assert_eq!(
        effect2.chips, effect3.chips,
        "Bull joker should give consistent results"
    );
    assert_eq!(
        effect1.chips, 30,
        "Bull joker should give expected 30 chips"
    );
}

#[test]
fn test_migrated_jokers_match_original_behavior() {
    // This test validates that the migrated StaticJoker versions work exactly
    // as the original implementations would have worked

    // Test Bull joker behavior matches original money-based scaling
    let bull_joker = StaticJokerFactory::create_bull_joker();
    let mut context = create_test_context_with_money(7);
    let test_hand = SelectHand::new(vec![Card::new(Value::King, Suit::Heart)]);
    let effect = bull_joker.on_hand_played(&mut context, &test_hand);
    assert_eq!(
        effect.chips, 14,
        "Bull joker should match original +2 chips per $1"
    );

    // Test all jokers return appropriate effect types
    assert_eq!(effect.mult, 0, "Bull joker should not provide mult");
    assert_eq!(
        effect.mult_multiplier, 1.0,
        "Bull joker should not provide mult multiplier (1.0 = no effect)"
    );
    assert!(
        effect.message.is_none() || effect.message.as_ref().unwrap().contains("Bull"),
        "Bull joker should have appropriate message"
    );
}
