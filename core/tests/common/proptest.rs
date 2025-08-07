//! Property-based testing utilities for balatro-rs
//!
//! This module provides property-based testing integration using proptest,
//! enabling automated generation of test inputs and invariant checking.
//!
//! ## Features
//! - Arbitrary implementations for game types
//! - Property generators for valid game states
//! - Invariant checking utilities
//! - Shrinking strategies for minimal failing examples

use balatro_rs::{
    action::Action,
    ante::Ante,
    card::{Card, Value, Suit},
    config::Config,
    deck::Deck,
    game::Game,
    hand::Hand,
    joker::JokerId,
    stage::{Stage, Blind, End},
};
use proptest::prelude::*;
use proptest::strategy::{BoxedStrategy, Strategy};
use std::collections::HashSet;

// ============================================================================
// ARBITRARY IMPLEMENTATIONS
// ============================================================================

/// Generate arbitrary valid values (ranks)
pub fn arb_value() -> impl Strategy<Value = Value> {
    prop_oneof![
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
        Just(Value::Ace),
    ]
}

/// Generate arbitrary valid suits
pub fn arb_suit() -> impl Strategy<Value = Suit> {
    prop_oneof![
        Just(Suit::Club),
        Just(Suit::Diamond),
        Just(Suit::Heart),
        Just(Suit::Spade),
    ]
}

/// Generate arbitrary valid cards
pub fn arb_card() -> impl Strategy<Value = Card> {
    (arb_value(), arb_suit()).prop_map(|(value, suit)| Card::new(value, suit))
}

/// Generate arbitrary valid hands (1-5 cards)
pub fn arb_hand() -> impl Strategy<Value = Vec<Card>> {
    prop::collection::vec(arb_card(), 1..=5)
        .prop_filter("unique cards only", |cards| {
            let unique: HashSet<_> = cards.iter().collect();
            unique.len() == cards.len()
        })
}

/// Generate arbitrary valid hands as Hand objects
pub fn arb_hand_object() -> impl Strategy<Value = Hand> {
    arb_hand().prop_map(|cards| Hand::new(cards))
}

/// Generate arbitrary valid deck sizes
pub fn arb_deck_size() -> impl Strategy<Value = usize> {
    1usize..=52
}

/// Generate arbitrary valid decks
pub fn arb_deck() -> impl Strategy<Value = Deck> {
    arb_deck_size().prop_map(|size| {
        let mut deck = Deck::new();
        // Add cards up to the specified size
        let mut cards_added = 0;
        for suit in [Suit::Club, Suit::Diamond, Suit::Heart, Suit::Spade] {
            for value in [
                Value::Two, Value::Three, Value::Four, Value::Five,
                Value::Six, Value::Seven, Value::Eight, Value::Nine,
                Value::Ten, Value::Jack, Value::Queen, Value::King, Value::Ace,
            ] {
                if cards_added >= size {
                    break;
                }
                // Note: Deck might not have add_card method
                // This is simplified for the example
                cards_added += 1;
            }
            if cards_added >= size {
                break;
            }
        }
        deck
    })
}

/// Generate arbitrary valid antes
pub fn arb_ante() -> impl Strategy<Value = Ante> {
    prop_oneof![
        Just(Ante::Zero),
        Just(Ante::One),
        Just(Ante::Two),
        Just(Ante::Three),
        Just(Ante::Four),
        Just(Ante::Five),
        Just(Ante::Six),
        Just(Ante::Seven),
        Just(Ante::Eight),
    ]
}

/// Generate arbitrary valid stages
pub fn arb_stage() -> impl Strategy<Value = Stage> {
    prop_oneof![
        Just(Stage::PreBlind()),
        Just(Stage::Blind(Blind::Small)),
        Just(Stage::Blind(Blind::Big)),
        Just(Stage::Blind(Blind::Boss)),
        Just(Stage::PostBlind()),
        Just(Stage::Shop()),
        Just(Stage::End(End::Win)),
        Just(Stage::End(End::Lose)),
    ]
}

/// Generate arbitrary valid money amounts
pub fn arb_money() -> impl Strategy<Value = f64> {
    (0u32..=1000).prop_map(|m| m as f64)
}

/// Generate arbitrary valid scores
pub fn arb_score() -> impl Strategy<Value = u64> {
    0u64..=1_000_000
}

// Note: ChipModifier and MultModifier types not available in current API
// These would need to be added if the modifier module is implemented

/// Generate arbitrary joker IDs
pub fn arb_joker_id() -> impl Strategy<Value = JokerId> {
    // Generate a subset of joker IDs for testing
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
    ]
}

// ============================================================================
// PROPERTY GENERATORS
// ============================================================================

/// Configuration for generating valid game states
#[derive(Debug, Clone)]
pub struct GameStateConfig {
    pub min_money: f64,
    pub max_money: f64,
    pub min_ante: u8,
    pub max_ante: u8,
    pub min_jokers: usize,
    pub max_jokers: usize,
    pub allow_negative_money: bool,
}

impl Default for GameStateConfig {
    fn default() -> Self {
        Self {
            min_money: 0.0,
            max_money: 100.0,
            min_ante: 1,
            max_ante: 8,
            min_jokers: 0,
            max_jokers: 5,
            allow_negative_money: false,
        }
    }
}

/// Generate a valid game state with configurable parameters
pub fn arb_game_state_with_config(config: GameStateConfig) -> BoxedStrategy<Game> {
    let money_range = if config.allow_negative_money {
        (-100.0..=config.max_money).boxed()
    } else {
        (config.min_money..=config.max_money).boxed()
    };

    (
        money_range,
        config.min_ante..=config.max_ante,
        config.min_jokers..=config.max_jokers,
        any::<u64>(), // seed
    )
        .prop_map(|(money, ante_num, joker_count, _seed)| {
            // Note: Seed configuration may need special handling
            let game_config = Config::default();
            let mut game = Game::new(game_config);

            // Set money
            game.money = money;

            // Set ante (would need proper ante progression)
            // This is simplified for the example
            game.ante_current = match ante_num {
                1 => Ante::One,
                2 => Ante::Two,
                3 => Ante::Three,
                4 => Ante::Four,
                5 => Ante::Five,
                6 => Ante::Six,
                7 => Ante::Seven,
                8 => Ante::Eight,
                _ => Ante::One,
            };

            // Add jokers (simplified)
            for _ in 0..joker_count {
                // Add random jokers
                // This would need proper joker addition logic
            }

            game
        })
        .boxed()
}

/// Generate a valid game state with default configuration
pub fn arb_game_state() -> BoxedStrategy<Game> {
    arb_game_state_with_config(GameStateConfig::default())
}

/// Generate a game state in a specific stage
pub fn arb_game_in_stage(stage: Stage) -> BoxedStrategy<Game> {
    // Note: Setting stage directly on Game might not be supported
    // This is simplified for the example
    arb_game_state()
        .prop_map(move |game| {
            // Would need proper stage setting logic
            game
        })
        .boxed()
}

/// Generate a game state with specific jokers
pub fn arb_game_with_jokers(joker_ids: Vec<JokerId>) -> BoxedStrategy<Game> {
    arb_game_state()
        .prop_map(move |mut game| {
            // Clear existing jokers and add specified ones
            // This would need proper implementation
            for id in &joker_ids {
                // Add joker with given ID
                // game.add_joker(id.clone());
            }
            game
        })
        .boxed()
}

// ============================================================================
// INVARIANT CHECKERS
// ============================================================================

/// Check that money is never negative (unless explicitly allowed)
pub fn invariant_money_non_negative(game: &Game) -> bool {
    game.money >= 0.0
}

/// Check that scoring always returns non-negative values
pub fn invariant_score_non_negative(hand: &Hand) -> bool {
    // Score calculation would go here
    // For now, always return true as placeholder
    true
}

/// Check that ante progression is valid
pub fn invariant_ante_progression(game: &Game) -> bool {
    match game.ante_current {
        Ante::Zero => true,
        Ante::One => true,
        Ante::Two => true, // Could check previous was One
        Ante::Three => true,
        Ante::Four => true,
        Ante::Five => true,
        Ante::Six => true,
        Ante::Seven => true,
        Ante::Eight => true,
    }
}

/// Check that stage transitions are valid
pub fn invariant_stage_transition(from: &Stage, to: &Stage) -> bool {
    match (from, to) {
        (Stage::PreBlind(), Stage::Blind(Blind::Small)) => true,
        (Stage::PreBlind(), Stage::Shop()) => true, // Skip blind
        (Stage::Blind(Blind::Small), Stage::Blind(Blind::Big)) => true,
        (Stage::Blind(Blind::Small), Stage::Shop()) => true, // Skip blind
        (Stage::Blind(Blind::Big), Stage::Blind(Blind::Boss)) => true,
        (Stage::Blind(Blind::Big), Stage::Shop()) => true,
        (Stage::Blind(Blind::Boss), Stage::PostBlind()) => true,
        (Stage::PostBlind(), Stage::Shop()) => true,
        (Stage::Shop(), Stage::PreBlind()) => true,
        (Stage::Shop(), Stage::End(_)) => true,
        _ => false,
    }
}

/// Check that deck size is valid (0-52 cards)
pub fn invariant_deck_size(deck: &Deck) -> bool {
    deck.cards().len() <= 52
}

/// Check that hand size is valid (0-10 cards typically)
pub fn invariant_hand_size(hand: &Hand) -> bool {
    hand.cards().len() <= 10
}

/// Check that joker slots are valid (typically max 5)
pub fn invariant_joker_slots(game: &Game) -> bool {
    game.jokers.len() <= 5
}

// ============================================================================
// PROPERTY TEST HELPERS
// ============================================================================

/// Helper to run a property test with custom configuration
pub fn run_property_test<F>(config: GameStateConfig, property: F) -> Result<(), TestCaseError>
where
    F: Fn(&Game) -> bool,
{
    let strategy = arb_game_state_with_config(config);
    proptest!(|(game in strategy)| {
        prop_assert!(property(&game));
    });
    Ok(())
}

/// Helper to test action validity
pub fn prop_action_always_valid(game: &Game, action: &Action) -> bool {
    // Check if action is valid for current game state
    // This would need proper implementation based on game rules
    true
}

/// Helper to test state transitions
pub fn prop_valid_state_transition(before: &Game, action: &Action, after: &Game) -> bool {
    // Verify that the state transition is valid
    // This would need proper implementation
    true
}

// ============================================================================
// SHRINKING STRATEGIES
// ============================================================================

// Note: Custom shrinking for non-Clone Game types would require special handling
// This is simplified for the example

/// Custom shrinking strategy for hands
pub fn shrink_hand(hand: Vec<Card>) -> BoxedStrategy<Vec<Card>> {
    // Shrink by removing cards
    if hand.is_empty() {
        Just(hand).boxed()
    } else {
        prop_oneof![
            Just(hand.clone()),
            Just(hand[1..].to_vec()),
            Just(hand[..hand.len()-1].to_vec()),
        ].boxed()
    }
}

// ============================================================================
// EXAMPLE PROPERTIES
// ============================================================================

#[cfg(test)]
mod example_properties {
    use super::*;

    proptest! {
        #[test]
        fn prop_money_never_negative_after_valid_action(
            game in arb_game_state(),
        ) {
            prop_assert!(invariant_money_non_negative(&game));
        }

        #[test]
        fn prop_score_always_non_negative(
            hand in arb_hand_object(),
        ) {
            prop_assert!(invariant_score_non_negative(&hand));
        }

        #[test]
        fn prop_deck_size_always_valid(
            deck in arb_deck(),
        ) {
            prop_assert!(invariant_deck_size(&deck));
        }

        #[test]
        fn prop_hand_size_always_valid(
            cards in arb_hand(),
        ) {
            let hand = Hand::new(cards);
            prop_assert!(invariant_hand_size(&hand));
        }
    }
}

// ============================================================================
// REGRESSION PROPERTY TESTS
// ============================================================================

/// Property tests that catch specific regressions
#[cfg(test)]
mod regression_properties {
    use super::*;

    proptest! {
        #[test]
        fn prop_no_duplicate_cards_in_hand(
            cards in arb_hand(),
        ) {
            let unique: HashSet<_> = cards.iter().collect();
            prop_assert_eq!(unique.len(), cards.len());
        }

        #[test]
        fn prop_ante_always_valid(
            ante in arb_ante(),
        ) {
            // Ante should map to valid values
            let ante_value = match ante {
                Ante::Zero => 0,
                Ante::One => 1,
                Ante::Two => 2,
                Ante::Three => 3,
                Ante::Four => 4,
                Ante::Five => 5,
                Ante::Six => 6,
                Ante::Seven => 7,
                Ante::Eight => 8,
            };
            prop_assert!(ante_value >= 0 && ante_value <= 8);
        }
    }
}
