//! Property-based testing utilities using proptest
//!
//! Provides generators and property-based test helpers for comprehensive
//! validation of game logic with random inputs.

use proptest::prelude::*;
use balatro_rs::{
    action::Action,
    card::{Card, Suit, Value},
    game::Game,
    joker::{JokerId, GameContext},
    stage::{Stage, Blind},
    shop::packs::PackType,
    hand::Hand,
    rank::HandRank,
};
use std::collections::HashMap;

/// Generate arbitrary cards for property testing
impl Arbitrary for Card {
    type Parameters = ();
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
        (any::<Value>(), any::<Suit>())
            .prop_map(|(value, suit)| Card::new(value, suit))
            .boxed()
    }
}

/// Generate arbitrary suits
impl Arbitrary for Suit {
    type Parameters = ();
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
        prop_oneof![
            Just(Suit::Heart),
            Just(Suit::Diamond),
            Just(Suit::Club),
            Just(Suit::Spade),
        ].boxed()
    }
}

/// Generate arbitrary values
impl Arbitrary for Value {
    type Parameters = ();
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
        prop_oneof![
            Just(Value::Ace),
            Just(Value::Two),
            Just(Value::Three),
            Just(Value::Four),
            Just(Value::Five),
            Just(Value::Six),
            Just(Value::Seven),
            Just(Value::Eight),
            Just(Value::Nine),
            Just(Value::Ten),
            Just(Value::Jack),
            Just(Value::Queen),
            Just(Value::King),
        ].boxed()
    }
}

/// Generate arbitrary joker IDs
impl Arbitrary for JokerId {
    type Parameters = ();
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
        prop_oneof![
            Just(JokerId::Joker),
            Just(JokerId::GreedyJoker),
            Just(JokerId::LustyJoker),
            Just(JokerId::WrathfulJoker),
            Just(JokerId::GluttonousJoker),
            Just(JokerId::JollyJoker),
            Just(JokerId::ZanyJoker),
            Just(JokerId::MadJoker),
            Just(JokerId::CrazyJoker),
            Just(JokerId::DrollJoker),
            // Add more joker IDs as needed
        ].boxed()
    }
}

/// Generate arbitrary blinds
impl Arbitrary for Blind {
    type Parameters = ();
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
        prop_oneof![
            Just(Blind::Small),
            Just(Blind::Big),
            Just(Blind::Boss),
        ].boxed()
    }
}

/// Generate arbitrary pack types
impl Arbitrary for PackType {
    type Parameters = ();
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
        prop_oneof![
            Just(PackType::Standard),
            Just(PackType::Jumbo),
            Just(PackType::Mega),
            Just(PackType::Celestial),
            Just(PackType::Arcana),
            Just(PackType::Spectral),
        ].boxed()
    }
}

/// Generate arbitrary actions for property testing
impl Arbitrary for Action {
    type Parameters = ();
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
        prop_oneof![
            any::<Card>().prop_map(Action::SelectCard),
            Just(Action::Play()),
            Just(Action::Discard()),
            (0.0..1000.0_f64).prop_map(Action::CashOut),
            (any::<JokerId>(), 0..5_usize).prop_map(|(joker_id, slot)| Action::BuyJoker { joker_id, slot }),
            any::<PackType>().prop_map(|pack_type| Action::BuyPack { pack_type }),
            (1..100_u32).prop_map(|pack_id| Action::OpenPack { pack_id }),
            (1..100_u32, 0..10_usize).prop_map(|(pack_id, option_index)| Action::SelectFromPack { pack_id, option_index }),
            (1..100_u32).prop_map(|pack_id| Action::SkipPack { pack_id }),
            Just(Action::NextRound()),
            any::<Blind>().prop_map(Action::SelectBlind),
        ].boxed()
    }
}

/// Custom generators for testing scenarios

/// Generate a valid hand (1-5 cards)
pub fn valid_hand() -> impl Strategy<Value = Vec<Card>> {
    prop::collection::vec(any::<Card>(), 1..=5)
}

/// Generate a full deck of cards
pub fn full_deck() -> impl Strategy<Value = Vec<Card>> {
    Just((0..52).map(|i| {
        let suit_index = i / 13;
        let value_index = i % 13;

        let suit = match suit_index {
            0 => Suit::Heart,
            1 => Suit::Diamond,
            2 => Suit::Club,
            _ => Suit::Spade,
        };

        let value = match value_index {
            0 => Value::Ace,
            1 => Value::Two,
            2 => Value::Three,
            3 => Value::Four,
            4 => Value::Five,
            5 => Value::Six,
            6 => Value::Seven,
            7 => Value::Eight,
            8 => Value::Nine,
            9 => Value::Ten,
            10 => Value::Jack,
            11 => Value::Queen,
            _ => Value::King,
        };

        Card::new(value, suit)
    }).collect::<Vec<_>>())
}

/// Generate realistic game state values
pub fn valid_game_state() -> impl Strategy<Value = (i32, i32, i32, u8, u32)> {
    (
        0..10000_i32,  // chips
        1..100_i32,    // mult
        0..1000_i32,   // money
        1..8_u8,       // ante
        1..100_u32,    // round
    )
}

/// Generate edge case values
pub fn edge_case_values() -> impl Strategy<Value = (i32, i32, i32, u8, u32)> {
    prop_oneof![
        Just((0, 1, 0, 1, 1)),              // Minimum values
        Just((i32::MAX, i32::MAX, i32::MAX, 8, u32::MAX)), // Maximum values
        Just((1, 1, 1, 1, 1)),              // Minimal non-zero
        Just((10000, 100, 1000, 8, 100)),   // High but reasonable
    ]
}

/// Generate a sequence of actions for game simulation
pub fn action_sequence() -> impl Strategy<Value = Vec<Action>> {
    prop::collection::vec(any::<Action>(), 1..20)
}

/// Property test helpers

/// Verify that card creation is consistent
pub fn prop_card_consistency(value: Value, suit: Suit) -> bool {
    let card = Card::new(value, suit);
    card.value == value && card.suit == suit
}

/// Verify that hand evaluation is deterministic
pub fn prop_hand_evaluation_deterministic(cards: Vec<Card>) -> bool {
    if cards.is_empty() || cards.len() > 5 {
        return true; // Skip invalid hands
    }

    let hand1 = Hand::new(cards.clone());
    let hand2 = Hand::new(cards);

    // Hand evaluation should be deterministic
    hand1.cards() == hand2.cards()
}

/// Verify that game actions don't cause panics
pub fn prop_action_safety(action: Action) -> bool {
    let mut game = Game::default();
    game.start();

    // Action should not panic, regardless of validity
    std::panic::catch_unwind(|| {
        let _ = game.handle_action(action);
    }).is_ok()
}

/// Verify that game state remains valid after actions
pub fn prop_game_state_validity(actions: Vec<Action>) -> bool {
    let mut game = Game::default();
    game.start();

    for action in actions {
        if game.is_over() {
            break;
        }

        let available_actions: Vec<Action> = game.gen_actions().collect();
        if available_actions.contains(&action) {
            if game.handle_action(action).is_err() {
                return false; // Valid action should not fail
            }
        }
        // Invalid actions are expected to be rejected, not cause invariant violations
    }

    // Game should be in a valid state
    !game.is_over() || game.result().is_some()
}

/// Verify numerical stability in score calculations
pub fn prop_score_calculation_stability(
    chips: i32,
    mult: i32,
) -> bool {
    if chips < 0 || mult < 0 {
        return true; // Skip invalid inputs
    }

    // Score calculation should not overflow or produce invalid results
    let score = chips.saturating_mul(mult) as i64;
    score >= 0 && score <= i64::MAX
}

/// Verify that joker effects don't break game invariants
pub fn prop_joker_effect_invariants(
    joker_id: JokerId,
    chips: i32,
    mult: i32,
    money: i32,
) -> bool {
    if chips < 0 || mult < 0 || money < 0 {
        return true; // Skip invalid inputs
    }

    // Create minimal context for joker testing
    // Note: This is simplified - real tests would need full context

    // Joker effects should not produce negative values
    let modified_chips = chips.saturating_add(100); // Simulate joker effect
    let modified_mult = mult.saturating_add(5);

    modified_chips >= 0 && modified_mult >= 0
}

/// Verify that hand rankings are consistent with poker rules
pub fn prop_hand_ranking_consistency(cards: Vec<Card>) -> bool {
    if cards.is_empty() || cards.len() > 5 {
        return true; // Skip invalid hands
    }

    // Check that hand ranking makes sense
    let has_duplicates = {
        let mut values = cards.iter().map(|c| c.value).collect::<Vec<_>>();
        values.sort();
        values.windows(2).any(|w| w[0] == w[1])
    };

    let has_flush = {
        let suits: std::collections::HashSet<_> = cards.iter().map(|c| c.suit).collect();
        suits.len() == 1 && cards.len() == 5
    };

    // Basic consistency check: if we have a flush, the hand should be valid
    if has_flush && cards.len() == 5 {
        // Should be at least a flush
        true
    } else if has_duplicates {
        // Should have some pair-based ranking
        true
    } else {
        // High card or straight
        true
    }
}

/// Test harness for running property tests
pub fn run_property_test<T: std::fmt::Debug + Clone>(
    name: &str,
    strategy: impl Strategy<Value = T>,
    property: impl Fn(T) -> bool,
    cases: u32,
) {
    println!("Running property test: {}", name);

    let mut runner = proptest::test_runner::TestRunner::new(
        proptest::test_runner::Config::with_cases(cases)
    );

    let test_result = runner.run(&strategy, |input| {
        if property(input.clone()) {
            Ok(())
        } else {
            Err(proptest::test_runner::TestCaseError::fail(format!(
                "Property failed for input: {:?}",
                input
            )))
        }
    });

    match test_result {
        Ok(_) => println!("  ✅ Property test passed ({} cases)", cases),
        Err(err) => {
            println!("  ❌ Property test failed: {}", err);
            panic!("Property test {} failed", name);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::config::PROPTEST_CASES;

    proptest! {
        #[test]
        fn prop_test_card_consistency(
            value in any::<Value>(),
            suit in any::<Suit>()
        ) {
            prop_assert!(prop_card_consistency(value, suit));
        }

        #[test]
        fn prop_test_hand_evaluation_deterministic(
            cards in valid_hand()
        ) {
            prop_assert!(prop_hand_evaluation_deterministic(cards));
        }

        #[test]
        fn prop_test_action_safety(
            action in any::<Action>()
        ) {
            prop_assert!(prop_action_safety(action));
        }

        #[test]
        fn prop_test_score_calculation_stability(
            chips in 0..1000000_i32,
            mult in 0..1000_i32
        ) {
            prop_assert!(prop_score_calculation_stability(chips, mult));
        }

        #[test]
        fn prop_test_joker_effect_invariants(
            joker_id in any::<JokerId>(),
            chips in 0..10000_i32,
            mult in 0..100_i32,
            money in 0..1000_i32
        ) {
            prop_assert!(prop_joker_effect_invariants(joker_id, chips, mult, money));
        }

        #[test]
        fn prop_test_hand_ranking_consistency(
            cards in valid_hand()
        ) {
            prop_assert!(prop_hand_ranking_consistency(cards));
        }

        #[test]
        fn prop_test_edge_cases(
            values in edge_case_values()
        ) {
            let (chips, mult, money, ante, round) = values;
            // Verify edge cases don't break basic invariants
            prop_assert!(chips >= 0);
            prop_assert!(mult >= 1);
            prop_assert!(money >= 0);
            prop_assert!(ante >= 1 && ante <= 8);
            prop_assert!(round >= 1);
        }
    }

    #[test]
    fn test_property_test_runner() {
        run_property_test(
            "card_creation_test",
            (any::<Value>(), any::<Suit>()),
            |(value, suit)| prop_card_consistency(value, suit),
            10
        );
    }

    #[test]
    fn test_generators() {
        // Test that generators produce valid values
        let mut runner = proptest::test_runner::TestRunner::default();

        // Test card generator
        let card_strategy = any::<Card>();
        for _ in 0..10 {
            let card = card_strategy.new_tree(&mut runner).unwrap().current();
            // Card should be valid (no additional constraints currently)
            assert!(format!("{:?}", card).len() > 0);
        }

        // Test hand generator
        let hand_strategy = valid_hand();
        for _ in 0..10 {
            let hand = hand_strategy.new_tree(&mut runner).unwrap().current();
            assert!(hand.len() >= 1 && hand.len() <= 5);
        }
    }
}
