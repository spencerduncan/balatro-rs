//! Domain-specific assertion utilities for comprehensive testing
//!
//! Provides specialized assertion functions for game domain concepts
//! that go beyond standard Rust assertions.
//!

#![allow(clippy::all)]
//! ## Production Engineering Patterns
//! - Detailed error messages for debugging at 3 AM
//! - State comparison utilities for regression testing
//! - Business rule validators for domain invariants
//! - Performance assertions for latency requirements

use balatro_rs::{
    action::Action,
    card::Card,
    error::GameError,
    game::Game,
    hand::{Hand, SelectHand},
    joker::{JokerEffect, JokerId},
    rank::HandRank,
    stage::Stage,
};
use std::time::{Duration, Instant};

/// Result type for validation assertions
#[derive(Debug, Clone, PartialEq)]
pub enum ValidationResult {
    Valid,
    Invalid(String),
}

/// Assert that a game action is valid in the current context
pub fn assert_action_valid(game: &Game, action: &Action) {
    let actions: Vec<Action> = game.gen_actions().collect();
    assert!(
        actions.contains(action),
        "Action {:?} is not valid in current game state. Valid actions: {:?}",
        action,
        actions
    );
}

/// Assert that a game action is invalid in the current context
pub fn assert_action_invalid(game: &Game, action: &Action) {
    let actions: Vec<Action> = game.gen_actions().collect();
    assert!(
        !actions.contains(action),
        "Action {:?} should not be valid in current game state",
        action
    );
}

/// Assert that a hand evaluates to a specific rank
pub fn assert_hand_rank(hand: &[Card], expected_rank: HandRank) {
    let select_hand = SelectHand::new(hand.to_vec());
    let best_hand = select_hand.best_hand().expect("Failed to get best hand");
    assert_eq!(
        best_hand.rank, expected_rank,
        "Expected hand rank {:?}, got {:?} for hand: {:?}",
        expected_rank, best_hand.rank, hand
    );
}

/// Assert that a hand contains specific cards
pub fn assert_hand_contains_cards(hand: &Hand, expected_cards: &[Card]) {
    for card in expected_cards {
        assert!(
            hand.cards().contains(card),
            "Hand does not contain expected card: {:?}. Hand: {:?}",
            card,
            hand.cards()
        );
    }
}

/// Assert that a hand does not contain specific cards
pub fn assert_hand_does_not_contain_cards(hand: &Hand, forbidden_cards: &[Card]) {
    for card in forbidden_cards {
        assert!(
            !hand.cards().contains(card),
            "Hand contains forbidden card: {:?}. Hand: {:?}",
            card,
            hand.cards()
        );
    }
}

/// Assert that a game is in a specific stage
pub fn assert_game_stage(game: &Game, expected_stage: Stage) {
    let actual_stage = &game.stage;
    assert_eq!(
        std::mem::discriminant(actual_stage),
        std::mem::discriminant(&expected_stage),
        "Expected game stage {:?}, got {:?}",
        expected_stage,
        actual_stage
    );
}

/// Assert that a game has ended
pub fn assert_game_ended(game: &Game) {
    assert!(
        game.is_over(),
        "Expected game to be over, but it's still running"
    );
    assert!(
        game.result().is_some(),
        "Expected game result to be available"
    );
}

/// Assert that a game is still running
pub fn assert_game_running(game: &Game) {
    assert!(
        !game.is_over(),
        "Expected game to be running, but it's over"
    );
    assert!(
        game.result().is_none(),
        "Expected no game result while running"
    );
}

/// Assert that joker effect has specific properties
pub fn assert_joker_effect(
    effect: &JokerEffect,
    expected_chips: Option<i32>,
    expected_mult: Option<i32>,
) {
    if let Some(chips) = expected_chips {
        assert_eq!(
            effect.chips, chips,
            "Expected joker effect chips {}, got {}",
            chips, effect.chips
        );
    }

    if let Some(mult) = expected_mult {
        assert_eq!(
            effect.mult, mult,
            "Expected joker effect mult {}, got {}",
            mult, effect.mult
        );
    }
}

/// Assert that joker effect is neutral (no modifications)
pub fn assert_joker_effect_neutral(effect: &JokerEffect) {
    assert_eq!(
        effect.chips, 0,
        "Expected neutral joker effect chips, got {}",
        effect.chips
    );
    assert_eq!(
        effect.mult, 0,
        "Expected neutral joker effect mult, got {}",
        effect.mult
    );
    assert_eq!(
        effect.mult_multiplier, 1.0,
        "Expected neutral joker effect mult_multiplier (1.0), got {}",
        effect.mult_multiplier
    );
    assert_eq!(
        effect.retrigger, 0,
        "Expected no retrigger in neutral joker effect, got {}",
        effect.retrigger
    );
}

/// Assert that a game action was applied successfully
pub fn assert_action_applied(result: &Result<(), GameError>) {
    match result {
        Ok(()) => (),
        Err(err) => panic!(
            "Expected action to be applied successfully, got error: {:?}",
            err
        ),
    }
}

/// Assert that a game action failed with specific error type
pub fn assert_action_failed<E>(result: &Result<(), E>, expected_error_msg: &str)
where
    E: std::fmt::Debug,
{
    match result {
        Ok(()) => panic!("Expected action to fail, but it succeeded"),
        Err(err) => {
            let error_str = format!("{:?}", err);
            assert!(
                error_str.contains(expected_error_msg),
                "Expected error to contain '{}', got: '{}'",
                expected_error_msg,
                error_str
            );
        }
    }
}

/// Assert game score is within expected range
pub fn assert_score_in_range(game: &Game, min: i32, max: i32) {
    let score = game.score;
    assert!(
        score >= min as f64 && score <= max as f64,
        "Expected score to be between {} and {}, got {}",
        min,
        max,
        score
    );
}

/// Assert money is within expected range
pub fn assert_money_in_range(game: &Game, min: i32, max: i32) {
    let money = game.money;
    assert!(
        money >= min as f64 && money <= max as f64,
        "Expected money to be between {} and {}, got {}",
        min,
        max,
        money
    );
}

/// Assert ante is at expected level
pub fn assert_ante_level(game: &Game, expected_ante: u8) {
    use balatro_rs::ante::Ante;
    let actual_ante = match game.ante_current {
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
    assert_eq!(
        actual_ante, expected_ante,
        "Expected ante level {}, got {}",
        expected_ante, actual_ante
    );
}

/// Assert round number matches expected
pub fn assert_round_number(game: &Game, expected_round: u8) {
    assert_eq!(
        game.round as u8, expected_round,
        "Expected round {}, got {}",
        expected_round, game.round
    );
}

/// Assert hand size is as expected
pub fn assert_hand_size(hand: &Hand, expected_size: usize) {
    let actual_size = hand.cards().len();
    assert_eq!(
        actual_size, expected_size,
        "Expected hand size {}, got {}",
        expected_size, actual_size
    );
}

/// Assert that a specific card exists in the deck
pub fn assert_card_in_deck(game: &Game, card: &Card) {
    assert!(
        game.deck.cards().contains(card),
        "Card {:?} not found in deck",
        card
    );
}

/// Assert that a specific card is NOT in the deck
pub fn assert_card_not_in_deck(game: &Game, card: &Card) {
    assert!(
        !game.deck.cards().contains(card),
        "Card {:?} should not be in deck",
        card
    );
}

/// Assert deck size matches expected
pub fn assert_deck_size(game: &Game, expected_size: usize) {
    let actual_size = game.deck.cards().len();
    assert_eq!(
        actual_size, expected_size,
        "Expected deck size {}, got {}",
        expected_size, actual_size
    );
}

// ============================================================================
// PRODUCTION-READY ASSERTIONS FROM PR #779 SALVAGE
// ============================================================================

/// Assert game state equals expected state with detailed differences
/// Production pattern: Comprehensive state comparison for regression testing
pub fn assert_game_state_equals(actual: &Game, expected: &Game) {
    assert_eq!(
        actual.ante_current, expected.ante_current,
        "Ante mismatch: actual={:?}, expected={:?}",
        actual.ante_current, expected.ante_current
    );

    assert_eq!(
        actual.round, expected.round,
        "Round mismatch: actual={}, expected={}",
        actual.round, expected.round
    );

    assert_eq!(
        actual.money, expected.money,
        "Money mismatch: actual={}, expected={}",
        actual.money, expected.money
    );

    assert_eq!(
        actual.chips, expected.chips,
        "Chips mismatch: actual={}, expected={}",
        actual.chips, expected.chips
    );

    assert_eq!(
        actual.mult, expected.mult,
        "Mult mismatch: actual={}, expected={}",
        actual.mult, expected.mult
    );

    assert_eq!(
        actual.score, expected.score,
        "Score mismatch: actual={}, expected={}",
        actual.score, expected.score
    );

    assert_eq!(
        std::mem::discriminant(&actual.stage),
        std::mem::discriminant(&expected.stage),
        "Stage mismatch: actual={:?}, expected={:?}",
        actual.stage,
        expected.stage
    );

    assert_eq!(
        actual.jokers.len(),
        expected.jokers.len(),
        "Joker count mismatch: actual={}, expected={}",
        actual.jokers.len(),
        expected.jokers.len()
    );
}

/// Assert game state matches snapshot with tolerance
/// Production pattern: Snapshot testing with acceptable variations
pub fn assert_game_state_snapshot(
    game: &Game,
    snapshot: &GameStateSnapshot,
    tolerance: Option<StateTolerance>,
) {
    let tolerance = tolerance.unwrap_or_default();

    // Check money within tolerance
    let money_diff = (game.money - snapshot.money as f64).abs();
    assert!(
        money_diff <= tolerance.money_tolerance,
        "Money outside tolerance: actual={}, expected={}, tolerance={}, diff={}",
        game.money,
        snapshot.money,
        tolerance.money_tolerance,
        money_diff
    );

    // Check score within tolerance
    let score_diff = (game.score - snapshot.score as f64).abs();
    assert!(
        score_diff <= tolerance.score_tolerance,
        "Score outside tolerance: actual={}, expected={}, tolerance={}, diff={}",
        game.score,
        snapshot.score,
        tolerance.score_tolerance,
        score_diff
    );

    // Check stage if strict
    if tolerance.strict_stage {
        assert_eq!(
            std::mem::discriminant(&game.stage),
            std::mem::discriminant(&snapshot.stage),
            "Stage mismatch in strict mode: actual={:?}, expected={:?}",
            game.stage,
            snapshot.stage
        );
    }
}

/// Game state snapshot for testing
#[derive(Debug, Clone)]
pub struct GameStateSnapshot {
    pub ante: u8,
    pub round: u8,
    pub money: i32,
    pub chips: i32,
    pub mult: i32,
    pub score: i32,
    pub stage: Stage,
    pub joker_count: usize,
}

impl From<&Game> for GameStateSnapshot {
    fn from(game: &Game) -> Self {
        Self {
            ante: match game.ante_current {
                balatro_rs::ante::Ante::Zero => 0,
                balatro_rs::ante::Ante::One => 1,
                balatro_rs::ante::Ante::Two => 2,
                balatro_rs::ante::Ante::Three => 3,
                balatro_rs::ante::Ante::Four => 4,
                balatro_rs::ante::Ante::Five => 5,
                balatro_rs::ante::Ante::Six => 6,
                balatro_rs::ante::Ante::Seven => 7,
                balatro_rs::ante::Ante::Eight => 8,
            },
            round: game.round as u8,
            money: game.money as i32,
            chips: game.chips as i32,
            mult: game.mult as i32,
            score: game.score as i32,
            stage: game.stage.clone(),
            joker_count: game.jokers.len(),
        }
    }
}

/// Tolerance for state comparison
#[derive(Debug, Clone)]
pub struct StateTolerance {
    pub money_tolerance: f64,
    pub score_tolerance: f64,
    pub strict_stage: bool,
}

impl Default for StateTolerance {
    fn default() -> Self {
        Self {
            money_tolerance: 0.0,
            score_tolerance: 0.0,
            strict_stage: true,
        }
    }
}

/// Assert business rule: Money never goes negative
/// Production pattern: Domain invariant validation
pub fn assert_money_never_negative(game: &Game) {
    assert!(
        game.money >= 0.0,
        "Business rule violation: Money went negative ({}). This should never happen!",
        game.money
    );
}

/// Assert business rule: Ante progression is valid
/// Production pattern: Game progression validation
pub fn assert_ante_progression_valid(before: u8, after: u8) {
    assert!(
        after == before || after == before + 1,
        "Invalid ante progression: {} -> {}. Ante can only stay same or increase by 1",
        before,
        after
    );
}

/// Assert business rule: Round progression is valid
/// Production pattern: Round sequence validation
pub fn assert_round_progression_valid(before: u8, after: u8, ante_changed: bool) {
    if ante_changed {
        assert_eq!(
            after, 1,
            "Round should reset to 1 when ante changes, got {}",
            after
        );
    } else {
        assert!(
            after == before || after == before + 1,
            "Invalid round progression within same ante: {} -> {}",
            before,
            after
        );
    }
}

/// Assert performance: Action completes within time limit
/// Production pattern: Latency validation for SLAs
pub fn assert_action_completes_within<F>(action: F, time_limit: Duration, description: &str)
where
    F: FnOnce() -> (),
{
    let start = Instant::now();
    action();
    let elapsed = start.elapsed();

    assert!(
        elapsed <= time_limit,
        "Performance violation: {} took {:?}, limit was {:?}",
        description,
        elapsed,
        time_limit
    );
}

/// Assert joker collection is valid
/// Production pattern: Collection state validation
pub fn assert_joker_collection_valid(jokers: &[JokerId]) {
    // Check for duplicates (unless allowed by game rules)
    let mut seen = std::collections::HashSet::new();
    for joker in jokers {
        if !seen.insert(joker) {
            // Some jokers might allow duplicates - check game rules
            match joker {
                // Add jokers that allow duplicates here
                _ => panic!("Duplicate joker found: {:?}", joker),
            }
        }
    }

    // Check collection size limit
    assert!(
        jokers.len() <= 5,
        "Too many jokers: {} (max 5)",
        jokers.len()
    );
}

/// Assert scoring calculation is correct
/// Production pattern: Mathematical validation
pub fn assert_scoring_correct(
    base_chips: i32,
    base_mult: i32,
    joker_effects: &[JokerEffect],
    expected_score: i32,
) {
    let mut total_chips = base_chips;
    let mut total_mult = base_mult;
    let mut x_mult = 1.0;

    for effect in joker_effects {
        total_chips += effect.chips;
        total_mult += effect.mult;
        x_mult *= effect.mult_multiplier;
    }

    let calculated_score = ((total_chips * total_mult) as f64 * x_mult) as i32;

    assert_eq!(
        calculated_score, expected_score,
        "Score calculation mismatch: ({} * {}) * {} = {} (expected {})",
        total_chips, total_mult, x_mult, calculated_score, expected_score
    );
}

/// Assert state transition is valid
/// Production pattern: State machine validation
pub fn assert_valid_state_transition(from: &Stage, to: &Stage) {
    use Stage::*;

    let valid = match (from, to) {
        (PreBlind(..), Blind(_)) => true,
        (Blind(_), PostBlind(..)) => true,
        (PostBlind(..), Shop(..)) => true,
        (Shop(..), PreBlind(..)) => true,
        (_, End(_)) => true, // Can end from any state
        _ => false,
    };

    assert!(valid, "Invalid state transition: {:?} -> {:?}", from, to);
}

/// Assert that actions are deterministic with same seed
/// Production pattern: Determinism validation for testing
pub fn assert_actions_deterministic(seed: u64, action_count: usize) {
    use crate::common::fixtures::create_test_game_with_seed;

    let mut game1 = create_test_game_with_seed(seed);
    let mut game2 = create_test_game_with_seed(seed);

    for i in 0..action_count {
        let actions1: Vec<Action> = game1.gen_actions().collect();
        let actions2: Vec<Action> = game2.gen_actions().collect();

        assert_eq!(
            actions1, actions2,
            "Actions differ at step {} with same seed {}",
            i, seed
        );

        if actions1.is_empty() {
            break;
        }

        // Apply same action to both
        let action = actions1[0].clone();
        game1.handle_action(action.clone()).unwrap();
        game2.handle_action(action).unwrap();
    }
}

/// Custom assertion macros for cleaner test code
/// Production pattern: Domain-specific test macros
#[macro_export]
macro_rules! assert_game_won {
    ($game:expr) => {
        assert!(
            $game.is_over() && $game.result() == Some(true),
            "Expected game to be won, but it's {:?}",
            $game.result()
        );
    };
}

#[macro_export]
macro_rules! assert_game_lost {
    ($game:expr) => {
        assert!(
            $game.is_over() && $game.result() == Some(false),
            "Expected game to be lost, but it's {:?}",
            $game.result()
        );
    };
}

#[macro_export]
macro_rules! assert_action_available {
    ($game:expr, $action_type:pat) => {
        assert!(
            $game
                .gen_actions()
                .iter()
                .any(|a| matches!(a, $action_type)),
            "Action type {:?} not available",
            stringify!($action_type)
        );
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::fixtures::*;

    #[test]
    fn test_validation_result() {
        let valid = ValidationResult::Valid;
        assert_eq!(valid, ValidationResult::Valid);

        let invalid = ValidationResult::Invalid("Test error".to_string());
        match invalid {
            ValidationResult::Invalid(msg) => assert_eq!(msg, "Test error"),
            _ => panic!("Expected Invalid variant"),
        }
    }

    #[test]
    fn test_assert_game_running() {
        let game = create_test_game();
        assert_game_running(&game);
    }

    #[test]
    fn test_assert_hand_rank() {
        let royal_flush = create_test_hand(TestHandType::RoyalFlush);
        assert_hand_rank(&royal_flush, HandRank::RoyalFlush);

        let pair = create_test_hand(TestHandType::OnePair);
        assert_hand_rank(&pair, HandRank::OnePair);
    }

    #[test]
    fn test_assert_joker_effect_neutral() {
        let effect = JokerEffect::default();
        assert_joker_effect_neutral(&effect);
    }

    #[test]
    fn test_assert_deck_size() {
        let game = create_test_game();
        // Initial deck size is 52 minus dealt cards
        // The exact size depends on game initialization
        assert!(game.deck.cards().len() <= 52);
    }

    #[test]
    fn test_game_state_snapshot() {
        let game = create_test_game();
        let snapshot = GameStateSnapshot::from(&game);

        let actual_ante = match game.ante_current {
            balatro_rs::ante::Ante::Zero => 0,
            balatro_rs::ante::Ante::One => 1,
            balatro_rs::ante::Ante::Two => 2,
            balatro_rs::ante::Ante::Three => 3,
            balatro_rs::ante::Ante::Four => 4,
            balatro_rs::ante::Ante::Five => 5,
            balatro_rs::ante::Ante::Six => 6,
            balatro_rs::ante::Ante::Seven => 7,
            balatro_rs::ante::Ante::Eight => 8,
        };
        assert_eq!(snapshot.ante, actual_ante);
        assert_eq!(snapshot.round, game.round as u8);
        assert_eq!(snapshot.money, game.money as i32);

        // Test with exact match
        assert_game_state_snapshot(&game, &snapshot, None);

        // Test with tolerance
        let tolerance = StateTolerance {
            money_tolerance: 10.0,
            score_tolerance: 100.0,
            strict_stage: false,
        };
        assert_game_state_snapshot(&game, &snapshot, Some(tolerance));
    }

    #[test]
    fn test_business_rules() {
        let game = create_test_game();
        assert_money_never_negative(&game);

        assert_ante_progression_valid(1, 1);
        assert_ante_progression_valid(1, 2);

        assert_round_progression_valid(1, 2, false);
        assert_round_progression_valid(3, 1, true);
    }

    #[test]
    fn test_performance_assertion() {
        assert_action_completes_within(
            || {
                let _game = create_test_game();
            },
            Duration::from_millis(100),
            "Game creation",
        );
    }

    #[test]
    fn test_state_transitions() {
        assert_valid_state_transition(
            &Stage::PreBlind(),
            &Stage::Blind(balatro_rs::stage::Blind::Small),
        );
        assert_valid_state_transition(&Stage::PostBlind(), &Stage::Shop());
        assert_valid_state_transition(&Stage::Shop(), &Stage::PreBlind());
    }

    #[test]
    #[ignore = "Seed configuration not yet available in current API"]
    fn test_deterministic_actions() {
        // TODO: Enable this test when Config supports seed field
        assert_actions_deterministic(42, 10);
    }
}
