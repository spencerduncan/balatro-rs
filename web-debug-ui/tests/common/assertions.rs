//! Domain-specific assertion utilities for comprehensive testing
//!
//! Provides specialized assertion functions for game domain concepts
//! that go beyond standard Rust assertions.

use balatro_rs::{
    action::Action,
    card::{Card, Suit, Value},
    error::GameError,
    game::Game,
    hand::{Hand, SelectHand},
    joker::{GameContext, JokerEffect},
    rank::HandRank,
    stage::Stage,
};
use std::collections::HashMap;

/// Result type for validation assertions
#[derive(Debug, Clone, PartialEq)]
pub enum ValidationResult {
    Valid,
    Invalid(String),
}

/// Assert that a game action is valid in the current context
pub fn assert_action_valid(result: &ValidationResult) {
    match result {
        ValidationResult::Valid => (),
        ValidationResult::Invalid(err) => panic!("Expected valid action, got: {}", err),
    }
}

/// Assert that a game action is invalid with a specific error
pub fn assert_action_invalid(result: &ValidationResult, expected_error: &str) {
    match result {
        ValidationResult::Invalid(err) => {
            assert!(
                err.contains(expected_error),
                "Expected error '{}' but got '{}'",
                expected_error,
                err
            );
        }
        ValidationResult::Valid => panic!("Expected invalid action, but action was valid"),
    }
}

/// Assert that a hand evaluates to a specific rank
pub fn assert_hand_rank(hand: &[Card], expected_rank: HandRank) {
    let select_hand = SelectHand::new(hand.to_vec());
    let actual_rank = select_hand.best_hand().hand_rank;
    assert_eq!(
        actual_rank, expected_rank,
        "Expected hand rank {:?}, got {:?} for hand: {:?}",
        expected_rank, actual_rank, hand
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
pub fn assert_game_stage(game: &Game, expected_stage: &Stage) {
    assert_eq!(
        std::mem::discriminant(&game.stage),
        std::mem::discriminant(expected_stage),
        "Expected game stage {:?}, got {:?}",
        expected_stage,
        game.stage
    );
}

/// Assert that a game has ended with a specific result
pub fn assert_game_ended(game: &Game) {
    assert!(game.is_over(), "Expected game to be over, but it's still running");
    assert!(game.result().is_some(), "Expected game result to be available");
}

/// Assert that a game is still running
pub fn assert_game_running(game: &Game) {
    assert!(!game.is_over(), "Expected game to be running, but it's over");
    assert!(game.result().is_none(), "Expected no game result while running");
}

/// Assert that joker effect has specific properties
pub fn assert_joker_effect(effect: &JokerEffect, expected_chips: Option<i32>, expected_mult: Option<i32>) {
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
    assert_eq!(effect.chips, 0, "Expected neutral joker effect chips, got {}", effect.chips);
    assert_eq!(effect.mult, 0, "Expected neutral joker effect mult, got {}", effect.mult);
    assert_eq!(effect.x_mult, 1.0, "Expected neutral joker effect x_mult (1.0), got {}", effect.x_mult);
    assert!(!effect.retrigger, "Expected no retrigger in neutral joker effect");
}

/// Assert that a game action was applied successfully
pub fn assert_action_applied(result: &Result<(), GameError>) {
    match result {
        Ok(()) => (),
        Err(err) => panic!("Expected action to be applied successfully, got error: {:?}", err),
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
                "Expected error containing '{}', got: {:?}",
                expected_error_msg,
                err
            );
        }
    }
}

/// Assert that game context has specific values
pub fn assert_game_context(
    context: &GameContext,
    expected_chips: Option<i32>,
    expected_mult: Option<i32>,
    expected_money: Option<i32>,
    expected_ante: Option<u8>,
) {
    if let Some(chips) = expected_chips {
        assert_eq!(
            context.chips, chips,
            "Expected context chips {}, got {}",
            chips, context.chips
        );
    }

    if let Some(mult) = expected_mult {
        assert_eq!(
            context.mult, mult,
            "Expected context mult {}, got {}",
            mult, context.mult
        );
    }

    if let Some(money) = expected_money {
        assert_eq!(
            context.money, money,
            "Expected context money {}, got {}",
            money, context.money
        );
    }

    if let Some(ante) = expected_ante {
        assert_eq!(
            context.ante, ante,
            "Expected context ante {}, got {}",
            ante, context.ante
        );
    }
}

/// Assert that context values are within valid ranges
pub fn assert_game_context_valid_ranges(context: &GameContext) {
    assert!(context.chips >= 0, "Chips should be non-negative, got {}", context.chips);
    assert!(context.mult >= 0, "Mult should be non-negative, got {}", context.mult);
    assert!(context.money >= 0, "Money should be non-negative, got {}", context.money);
    assert!(context.ante > 0, "Ante should be positive, got {}", context.ante);
    assert!(context.ante <= 8, "Ante should not exceed 8, got {}", context.ante);
    assert!(context.round > 0, "Round should be positive, got {}", context.round);
    assert!(context.hands_remaining >= 0.0, "Hands remaining should be non-negative, got {}", context.hands_remaining);
    assert!(context.cards_in_deck <= 52, "Cards in deck should not exceed 52, got {}", context.cards_in_deck);
}

/// Assert that a collection is sorted by a specific key
pub fn assert_sorted_by<T, K>(items: &[T], key_fn: impl Fn(&T) -> K)
where
    K: Ord + std::fmt::Debug,
{
    for window in items.windows(2) {
        let key1 = key_fn(&window[0]);
        let key2 = key_fn(&window[1]);
        assert!(
            key1 <= key2,
            "Collection is not sorted: {:?} > {:?}",
            key1, key2
        );
    }
}

/// Assert that all items in a collection satisfy a predicate
pub fn assert_all<T>(items: &[T], predicate: impl Fn(&T) -> bool, message: &str) {
    for (i, item) in items.iter().enumerate() {
        assert!(
            predicate(item),
            "{} - Failed at index {}: {:?}",
            message, i, item
        );
    }
}

/// Assert that at least one item in a collection satisfies a predicate
pub fn assert_any<T>(items: &[T], predicate: impl Fn(&T) -> bool, message: &str) {
    assert!(
        items.iter().any(predicate),
        "{} - No item satisfied the predicate",
        message
    );
}

/// Assert that exactly N items in a collection satisfy a predicate
pub fn assert_count<T>(items: &[T], predicate: impl Fn(&T) -> bool, expected_count: usize, message: &str) {
    let actual_count = items.iter().filter(|item| predicate(item)).count();
    assert_eq!(
        actual_count, expected_count,
        "{} - Expected {} items to satisfy predicate, got {}",
        message, expected_count, actual_count
    );
}

/// Assert that two floating point values are approximately equal
pub fn assert_f64_approx_eq(a: f64, b: f64, epsilon: f64, message: &str) {
    let diff = (a - b).abs();
    assert!(
        diff <= epsilon,
        "{} - Values not approximately equal: {} vs {} (diff: {}, epsilon: {})",
        message, a, b, diff, epsilon
    );
}

/// Assert that a performance metric is within acceptable bounds
pub fn assert_performance_metric(
    actual: f64,
    expected_min: f64,
    expected_max: f64,
    metric_name: &str,
) {
    assert!(
        actual >= expected_min,
        "Performance metric '{}' too low: {} < {}",
        metric_name, actual, expected_min
    );
    assert!(
        actual <= expected_max,
        "Performance metric '{}' too high: {} > {}",
        metric_name, actual, expected_max
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::fixtures::{create_test_hand, TestHandType};

    #[test]
    fn test_assert_hand_rank() {
        let royal_flush = create_test_hand(TestHandType::RoyalFlush);
        assert_hand_rank(&royal_flush, HandRank::RoyalFlush);

        let high_card = create_test_hand(TestHandType::HighCard);
        assert_hand_rank(&high_card, HandRank::HighCard);
    }

    #[test]
    fn test_assert_joker_effect_neutral() {
        let neutral_effect = JokerEffect {
            chips: 0,
            mult: 0,
            x_mult: 1.0,
            retrigger: false,
        };
        assert_joker_effect_neutral(&neutral_effect);
    }

    #[test]
    fn test_assert_sorted_by() {
        let numbers = vec![1, 2, 3, 4, 5];
        assert_sorted_by(&numbers, |x| *x);
    }

    #[test]
    fn test_assert_all() {
        let positive_numbers = vec![1, 2, 3, 4, 5];
        assert_all(&positive_numbers, |x| *x > 0, "All numbers should be positive");
    }

    #[test]
    fn test_assert_count() {
        let numbers = vec![1, 2, 3, 4, 5, 6];
        assert_count(&numbers, |x| *x % 2 == 0, 3, "Should have 3 even numbers");
    }

    #[test]
    fn test_assert_f64_approx_eq() {
        assert_f64_approx_eq(1.0, 1.001, 0.01, "Values should be approximately equal");
    }

    #[test]
    #[should_panic(expected = "Performance metric")]
    fn test_assert_performance_metric_fails() {
        assert_performance_metric(5.0, 1.0, 4.0, "test_metric");
    }
}
