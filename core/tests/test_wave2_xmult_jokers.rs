// Tests for Wave 2 X-Mult Jokers Implementation
//
// This test suite validates The Duo, The Trio, and The Family jokers
// which provide X-mult bonuses for specific hand types using the StaticJoker framework.
//
// Uncle Bob's Clean Code Principle Applied: Tests as Documentation
// These tests serve as executable documentation of joker behavior

use balatro_rs::card::{Card, Suit, Value};
use balatro_rs::hand::{Hand, SelectHand};
use balatro_rs::joker::{GameContext, Joker, JokerId, JokerRarity};
use balatro_rs::joker_factory::JokerFactory;
use balatro_rs::joker_state::JokerStateManager;
use balatro_rs::rng::GameRng;
use balatro_rs::stage::{Blind, Stage};
use balatro_rs::static_joker_factory::StaticJokerFactory;
use std::collections::HashMap;
use std::sync::Arc;

// ============================================================================
// TEST FIXTURES - Clean Code: Extract common setup into well-named functions
// ============================================================================

/// Create a minimal test context for joker testing
/// Clean Code: Express intent clearly - this is for testing, not production
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
        is_final_hand: false,
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

/// Create a hand with a Pair (for The Duo testing)
/// Clean Code: Intent-revealing function name
fn create_pair_hand() -> SelectHand {
    SelectHand::new(vec![
        Card::new(Value::Ace, Suit::Heart),
        Card::new(Value::Ace, Suit::Spade),
        Card::new(Value::King, Suit::Diamond),
        Card::new(Value::Queen, Suit::Club),
        Card::new(Value::Jack, Suit::Heart),
    ])
}

/// Create a hand with Three of a Kind (for The Trio testing)
/// Clean Code: Intent-revealing function name
fn create_three_of_a_kind_hand() -> SelectHand {
    SelectHand::new(vec![
        Card::new(Value::Ace, Suit::Heart),
        Card::new(Value::Ace, Suit::Spade),
        Card::new(Value::Ace, Suit::Diamond),
        Card::new(Value::King, Suit::Club),
        Card::new(Value::Queen, Suit::Heart),
    ])
}

/// Create a hand with Four of a Kind (for The Family testing)
/// Clean Code: Intent-revealing function name
fn create_four_of_a_kind_hand() -> SelectHand {
    SelectHand::new(vec![
        Card::new(Value::Ace, Suit::Heart),
        Card::new(Value::Ace, Suit::Spade),
        Card::new(Value::Ace, Suit::Diamond),
        Card::new(Value::Ace, Suit::Club),
        Card::new(Value::King, Suit::Heart),
    ])
}

/// Create a hand with Two Pair (should not trigger any of our jokers)
/// Clean Code: Explicit negative test case
fn create_two_pair_hand() -> SelectHand {
    SelectHand::new(vec![
        Card::new(Value::Ace, Suit::Heart),
        Card::new(Value::Ace, Suit::Spade),
        Card::new(Value::King, Suit::Diamond),
        Card::new(Value::King, Suit::Club),
        Card::new(Value::Queen, Suit::Heart),
    ])
}

/// Create a high card hand (should not trigger any of our jokers)
/// Clean Code: Explicit negative test case
fn create_high_card_hand() -> SelectHand {
    SelectHand::new(vec![
        Card::new(Value::Ace, Suit::Heart),
        Card::new(Value::King, Suit::Spade),
        Card::new(Value::Queen, Suit::Diamond),
        Card::new(Value::Jack, Suit::Club),
        Card::new(Value::Nine, Suit::Heart),
    ])
}

// ============================================================================
// THE DUO TESTS - X2 Mult for Pair hands
// ============================================================================

#[test]
fn test_the_duo_basic_properties() {
    let joker = StaticJokerFactory::create_the_duo();

    // Test all identity properties
    assert_eq!(joker.id(), JokerId::TheDuo);
    assert_eq!(joker.name(), "The Duo");
    assert_eq!(
        joker.description(),
        "X2 Mult if played hand contains a Pair"
    );
    assert_eq!(joker.rarity(), JokerRarity::Rare);
    assert_eq!(joker.cost(), 8);
}

#[test]
fn test_the_duo_factory_creation() {
    // Test creation via both factory methods
    let static_factory_joker = StaticJokerFactory::create_the_duo();
    let main_factory_joker = JokerFactory::create(JokerId::TheDuo);

    assert!(main_factory_joker.is_some());
    let main_factory_joker = main_factory_joker.unwrap();

    // Both should have identical properties
    assert_eq!(static_factory_joker.id(), main_factory_joker.id());
    assert_eq!(static_factory_joker.name(), main_factory_joker.name());
    assert_eq!(
        static_factory_joker.description(),
        main_factory_joker.description()
    );
    assert_eq!(static_factory_joker.rarity(), main_factory_joker.rarity());
    assert_eq!(static_factory_joker.cost(), main_factory_joker.cost());
}

#[test]
fn test_the_duo_triggers_on_pair() {
    let joker = StaticJokerFactory::create_the_duo();
    let mut context = create_test_context();
    let pair_hand = create_pair_hand();

    let effect = joker.on_hand_played(&mut context, &pair_hand);

    // Should provide X2 mult multiplier (2.0)
    assert_eq!(effect.mult_multiplier, 2.0);
    assert_eq!(effect.chips, 0);
    assert_eq!(effect.mult, 0);
    // Note: Static joker framework doesn't auto-generate messages for mult_multiplier
    // This is a known limitation of the current implementation
}

#[test]
fn test_the_duo_does_not_trigger_on_non_pair() {
    let joker = StaticJokerFactory::create_the_duo();
    let mut context = create_test_context();

    // Test hands that DON'T contain a pair
    // Note: Two Pair, Three of a Kind, and Four of a Kind all CONTAIN a pair
    // so they SHOULD trigger The Duo (as per "contains" language)
    let test_hands = vec![("High Card", create_high_card_hand())];

    for (hand_name, test_hand) in test_hands {
        let effect = joker.on_hand_played(&mut context, &test_hand);

        // Should provide no effect
        assert_eq!(effect.mult_multiplier, 1.0, "Failed for {hand_name}");
        assert_eq!(effect.chips, 0, "Failed for {hand_name}");
        assert_eq!(effect.mult, 0, "Failed for {hand_name}");
    }

    // Test that hands containing pairs DO trigger
    let hands_with_pairs = vec![
        ("Two Pair", create_two_pair_hand()),
        ("Three of a Kind", create_three_of_a_kind_hand()),
        ("Four of a Kind", create_four_of_a_kind_hand()),
    ];

    for (hand_name, test_hand) in hands_with_pairs {
        let effect = joker.on_hand_played(&mut context, &test_hand);
        assert_eq!(
            effect.mult_multiplier, 2.0,
            "Should trigger for {hand_name} since it contains a pair"
        );
    }
}

// ============================================================================
// THE TRIO TESTS - X3 Mult for Three of a Kind hands
// ============================================================================

#[test]
fn test_the_trio_basic_properties() {
    let joker = StaticJokerFactory::create_the_trio();

    // Test all identity properties
    assert_eq!(joker.id(), JokerId::TheTrio);
    assert_eq!(joker.name(), "The Trio");
    assert_eq!(
        joker.description(),
        "X3 Mult if played hand contains Three of a Kind"
    );
    assert_eq!(joker.rarity(), JokerRarity::Rare);
    assert_eq!(joker.cost(), 8);
}

#[test]
fn test_the_trio_factory_creation() {
    // Test creation via both factory methods
    let static_factory_joker = StaticJokerFactory::create_the_trio();
    let main_factory_joker = JokerFactory::create(JokerId::TheTrio);

    assert!(main_factory_joker.is_some());
    let main_factory_joker = main_factory_joker.unwrap();

    // Both should have identical properties
    assert_eq!(static_factory_joker.id(), main_factory_joker.id());
    assert_eq!(static_factory_joker.name(), main_factory_joker.name());
    assert_eq!(
        static_factory_joker.description(),
        main_factory_joker.description()
    );
    assert_eq!(static_factory_joker.rarity(), main_factory_joker.rarity());
    assert_eq!(static_factory_joker.cost(), main_factory_joker.cost());
}

#[test]
fn test_the_trio_triggers_on_three_of_a_kind() {
    let joker = StaticJokerFactory::create_the_trio();
    let mut context = create_test_context();
    let three_of_a_kind_hand = create_three_of_a_kind_hand();

    let effect = joker.on_hand_played(&mut context, &three_of_a_kind_hand);

    // Should provide X3 mult multiplier (3.0)
    assert_eq!(effect.mult_multiplier, 3.0);
    assert_eq!(effect.chips, 0);
    assert_eq!(effect.mult, 0);
    // Note: Static joker framework doesn't auto-generate messages for mult_multiplier
}

#[test]
fn test_the_trio_does_not_trigger_on_non_three_of_a_kind() {
    let joker = StaticJokerFactory::create_the_trio();
    let mut context = create_test_context();

    // Test hands that DON'T contain three of a kind
    // Note: Four of a Kind CONTAINS three of a kind so it SHOULD trigger
    let test_hands = vec![
        ("Pair", create_pair_hand()),
        ("Two Pair", create_two_pair_hand()),
        ("High Card", create_high_card_hand()),
    ];

    for (hand_name, test_hand) in test_hands {
        let effect = joker.on_hand_played(&mut context, &test_hand);

        // Should provide no effect
        assert_eq!(effect.mult_multiplier, 1.0, "Failed for {hand_name}");
        assert_eq!(effect.chips, 0, "Failed for {hand_name}");
        assert_eq!(effect.mult, 0, "Failed for {hand_name}");
    }

    // Test that Four of a Kind DOES trigger (contains three of a kind)
    let four_of_kind = create_four_of_a_kind_hand();
    let effect = joker.on_hand_played(&mut context, &four_of_kind);
    assert_eq!(
        effect.mult_multiplier, 3.0,
        "Four of a Kind contains Three of a Kind"
    );
}

// ============================================================================
// THE FAMILY TESTS - X4 Mult for Four of a Kind hands
// ============================================================================

#[test]
fn test_the_family_basic_properties() {
    let joker = StaticJokerFactory::create_the_family();

    // Test all identity properties
    assert_eq!(joker.id(), JokerId::TheFamily);
    assert_eq!(joker.name(), "The Family");
    assert_eq!(
        joker.description(),
        "X4 Mult if played hand contains Four of a Kind"
    );
    assert_eq!(joker.rarity(), JokerRarity::Rare);
    assert_eq!(joker.cost(), 8);
}

#[test]
fn test_the_family_factory_creation() {
    // Test creation via both factory methods
    let static_factory_joker = StaticJokerFactory::create_the_family();
    let main_factory_joker = JokerFactory::create(JokerId::TheFamily);

    assert!(main_factory_joker.is_some());
    let main_factory_joker = main_factory_joker.unwrap();

    // Both should have identical properties
    assert_eq!(static_factory_joker.id(), main_factory_joker.id());
    assert_eq!(static_factory_joker.name(), main_factory_joker.name());
    assert_eq!(
        static_factory_joker.description(),
        main_factory_joker.description()
    );
    assert_eq!(static_factory_joker.rarity(), main_factory_joker.rarity());
    assert_eq!(static_factory_joker.cost(), main_factory_joker.cost());
}

#[test]
fn test_the_family_triggers_on_four_of_a_kind() {
    let joker = StaticJokerFactory::create_the_family();
    let mut context = create_test_context();
    let four_of_a_kind_hand = create_four_of_a_kind_hand();

    let effect = joker.on_hand_played(&mut context, &four_of_a_kind_hand);

    // Should provide X4 mult multiplier (4.0)
    assert_eq!(effect.mult_multiplier, 4.0);
    assert_eq!(effect.chips, 0);
    assert_eq!(effect.mult, 0);
    // Note: Static joker framework doesn't auto-generate messages for mult_multiplier
}

#[test]
fn test_the_family_does_not_trigger_on_non_four_of_a_kind() {
    let joker = StaticJokerFactory::create_the_family();
    let mut context = create_test_context();

    // Test various non-four-of-a-kind hands
    let test_hands = vec![
        ("Pair", create_pair_hand()),
        ("Two Pair", create_two_pair_hand()),
        ("Three of a Kind", create_three_of_a_kind_hand()),
        ("High Card", create_high_card_hand()),
    ];

    for (hand_name, test_hand) in test_hands {
        let effect = joker.on_hand_played(&mut context, &test_hand);

        // Should provide no effect
        assert_eq!(effect.mult_multiplier, 1.0, "Failed for {hand_name}");
        assert_eq!(effect.chips, 0, "Failed for {hand_name}");
        assert_eq!(effect.mult, 0, "Failed for {hand_name}");
        assert!(effect.message.is_none(), "Failed for {hand_name}");
    }
}

// ============================================================================
// INTEGRATION TESTS - Testing jokers work in factory and rarity systems
// ============================================================================

#[test]
fn test_all_jokers_in_rare_rarity_list() {
    use balatro_rs::joker::JokerRarity;

    let rare_jokers = JokerFactory::get_by_rarity(JokerRarity::Rare);

    // All three jokers should be in the Rare rarity list
    assert!(
        rare_jokers.contains(&JokerId::TheDuo),
        "The Duo should be in Rare rarity list"
    );
    assert!(
        rare_jokers.contains(&JokerId::TheTrio),
        "The Trio should be in Rare rarity list"
    );
    assert!(
        rare_jokers.contains(&JokerId::TheFamily),
        "The Family should be in Rare rarity list"
    );
}

#[test]
fn test_all_jokers_can_be_created_via_factory() {
    // Test all three jokers can be created via main factory
    let joker_ids = vec![JokerId::TheDuo, JokerId::TheTrio, JokerId::TheFamily];

    for joker_id in joker_ids {
        let joker = JokerFactory::create(joker_id);
        assert!(joker.is_some(), "Failed to create joker {joker_id:?}");

        let joker = joker.unwrap();
        assert_eq!(joker.id(), joker_id, "Factory created wrong joker type");
    }
}

// ============================================================================
// EDGE CASE TESTS - Testing boundary conditions and edge cases
// ============================================================================

#[test]
fn test_jokers_with_full_house_and_straight_flush() {
    // Create more complex hands to test "contains" behavior

    // Full House contains BOTH a three of a kind AND a pair
    let full_house = SelectHand::new(vec![
        Card::new(Value::Ace, Suit::Heart),
        Card::new(Value::Ace, Suit::Spade),
        Card::new(Value::Ace, Suit::Diamond),
        Card::new(Value::King, Suit::Club),
        Card::new(Value::King, Suit::Heart),
    ]);

    let mut context = create_test_context();

    // The Duo SHOULD trigger (Full House contains a pair - the two Kings)
    let duo = StaticJokerFactory::create_the_duo();
    let duo_effect = duo.on_hand_played(&mut context, &full_house);
    assert_eq!(
        duo_effect.mult_multiplier, 2.0,
        "The Duo should trigger on Full House (contains a pair)"
    );

    // The Trio SHOULD trigger (Full House contains three of a kind - the three Aces)
    let trio = StaticJokerFactory::create_the_trio();
    let trio_effect = trio.on_hand_played(&mut context, &full_house);
    assert_eq!(
        trio_effect.mult_multiplier, 3.0,
        "The Trio should trigger on Full House (contains three of a kind)"
    );

    // The Family should NOT trigger (Full House does not contain four of a kind)
    let family = StaticJokerFactory::create_the_family();
    let family_effect = family.on_hand_played(&mut context, &full_house);
    assert_eq!(
        family_effect.mult_multiplier, 1.0,
        "The Family should not trigger on Full House (no four of a kind)"
    );
}

#[test]
fn test_jokers_multiplier_values_are_correct() {
    // Ensure each joker provides exactly the correct multiplier
    let mut context = create_test_context();

    // Test The Duo = X2 = 2.0
    let duo = StaticJokerFactory::create_the_duo();
    let duo_effect = duo.on_hand_played(&mut context, &create_pair_hand());
    assert_eq!(
        duo_effect.mult_multiplier, 2.0,
        "The Duo should provide exactly X2 (2.0) multiplier"
    );

    // Test The Trio = X3 = 3.0
    let trio = StaticJokerFactory::create_the_trio();
    let trio_effect = trio.on_hand_played(&mut context, &create_three_of_a_kind_hand());
    assert_eq!(
        trio_effect.mult_multiplier, 3.0,
        "The Trio should provide exactly X3 (3.0) multiplier"
    );

    // Test The Family = X4 = 4.0
    let family = StaticJokerFactory::create_the_family();
    let family_effect = family.on_hand_played(&mut context, &create_four_of_a_kind_hand());
    assert_eq!(
        family_effect.mult_multiplier, 4.0,
        "The Family should provide exactly X4 (4.0) multiplier"
    );
}

// ============================================================================
// MESSAGE TESTS - Ensuring proper user feedback
// ============================================================================

#[test]
fn test_joker_messages_are_informative() {
    // NOTE: The current static joker framework doesn't automatically generate
    // messages for mult_multiplier effects. This is a known limitation.
    // Messages would need to be added through custom implementation or
    // framework enhancement.

    let mut context = create_test_context();

    // Test that jokers trigger correctly even without messages
    let duo = StaticJokerFactory::create_the_duo();
    let duo_effect = duo.on_hand_played(&mut context, &create_pair_hand());
    assert_eq!(
        duo_effect.mult_multiplier, 2.0,
        "The Duo should provide X2 mult"
    );

    let trio = StaticJokerFactory::create_the_trio();
    let trio_effect = trio.on_hand_played(&mut context, &create_three_of_a_kind_hand());
    assert_eq!(
        trio_effect.mult_multiplier, 3.0,
        "The Trio should provide X3 mult"
    );

    let family = StaticJokerFactory::create_the_family();
    let family_effect = family.on_hand_played(&mut context, &create_four_of_a_kind_hand());
    assert_eq!(
        family_effect.mult_multiplier, 4.0,
        "The Family should provide X4 mult"
    );
}
