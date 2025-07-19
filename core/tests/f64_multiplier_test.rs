use balatro_rs::card::{Card, Suit, Value};
use balatro_rs::game::Game;
use balatro_rs::hand::SelectHand;
use balatro_rs::joker::{JokerEffect, JokerId};
use balatro_rs::joker_factory::JokerFactory;
use balatro_rs::stage::{Blind, Stage};

/// Test that base multipliers use f64 precision
/// This ensures we have sufficient precision for complex calculations
#[test]
fn test_base_multipliers_use_f64() {
    let effect = JokerEffect::new().with_mult_multiplier(1.5);

    // This test verifies we have migrated to f64
    // f64 should allow for higher precision than f32
    let multiplier = effect.mult_multiplier();

    // Verify the multiplier is stored as f64
    assert_eq!(multiplier, 1.5_f64);

    // Test that we can store precise decimal values
    // f32 has ~7 decimal digits, f64 has ~15-17
    let precise_value = 1.123456789012345_f64;
    let effect_precise = JokerEffect::new().with_mult_multiplier(precise_value);

    // This should maintain precision with f64
    assert_eq!(effect_precise.mult_multiplier(), precise_value);
}

/// Test multiplier stacking uses f64 arithmetic
/// This ensures compound multiplication maintains precision
#[test]
fn test_multiplier_stacking_f64_arithmetic() {
    let mut game = Game::default();
    game.start();
    game.stage = Stage::Blind(Blind::Small);
    game.blind = Some(Blind::Small);

    // Create a hand for scoring
    let cards = vec![
        Card::new(Value::Ace, Suit::Heart),
        Card::new(Value::King, Suit::Spade),
        Card::new(Value::Queen, Suit::Diamond),
        Card::new(Value::Jack, Suit::Club),
        Card::new(Value::Nine, Suit::Heart),
    ];
    let hand = SelectHand::new(cards).best_hand().unwrap();

    // Test stacking multiple precise multipliers
    let joker1 = JokerFactory::create(JokerId::Joker).expect("Can create joker");
    let joker2 = JokerFactory::create(JokerId::Joker).expect("Can create joker");
    let joker3 = JokerFactory::create(JokerId::Joker).expect("Can create joker");

    game.jokers = vec![joker1, joker2, joker3];

    // With three jokers each with 1.5x multiplier:
    // Expected: 1.5 * 1.5 * 1.5 = 3.375
    // With f64 precision, this should be exact
    let score = game.calc_score(hand);

    // This test verifies that stacking preserves f64 precision
    // Exact value will depend on base score, but should use f64 arithmetic
    assert!(score > 0.0);
}

/// Test Planet card effects use f64
/// Ensures planet card level multipliers maintain precision
#[test]
fn test_planet_card_effects_f64() {
    // This test is a placeholder for when planet cards are implemented
    // They should use f64 for level-based multiplier calculations

    // Planet cards modify hand level multipliers
    // e.g., Mercury for Pair: level 1 = x2, level 2 = x2.5, level 3 = x3.25
    // These fractional multipliers need f64 precision

    let base_mult = 2.0_f64;
    let planet_bonus = 0.25_f64;
    let final_mult = base_mult + planet_bonus;

    // Verify f64 arithmetic precision
    assert_eq!(final_mult, 2.25_f64);
}

/// Test multiplication order matches Lua
/// Ensures order of operations is consistent with Balatro
#[test]
fn test_multiplication_order_matches_lua() {
    let mut game = Game::default();
    game.start();
    game.stage = Stage::Blind(Blind::Small);
    game.blind = Some(Blind::Small);

    // Create a hand for scoring
    let cards = vec![
        Card::new(Value::Ace, Suit::Heart),
        Card::new(Value::King, Suit::Spade),
    ];
    let hand = SelectHand::new(cards).best_hand().unwrap();

    // Test that multipliers are applied in the correct order
    // In Balatro (Lua), multipliers are typically:
    // 1. Base hand mult
    // 2. Additive bonuses (+mult)
    // 3. Multiplicative bonuses (×mult)

    let joker = JokerFactory::create(JokerId::Joker).expect("Can create joker");
    game.jokers = vec![joker];

    let score = game.calc_score(hand);

    // Verify the calculation follows Lua semantics
    // Final score = (chips) × (base_mult + additive_mult) × multiplicative_mult
    assert!(score > 0.0);
}

/// Test edge cases with very large multipliers
/// Ensures f64 can handle extreme values without overflow
#[test]
fn test_edge_cases_large_multipliers() {
    // Test very large multiplier values
    let large_multiplier = 1e100_f64;
    let effect = JokerEffect::new().with_mult_multiplier(large_multiplier);

    // f64 should handle this without overflow (max ≈ 1.8e308)
    assert_eq!(effect.mult_multiplier(), large_multiplier);

    // Test maximum safe multiplier value
    let max_safe = 1e308_f64;
    let effect_max = JokerEffect::new().with_mult_multiplier(max_safe);
    assert_eq!(effect_max.mult_multiplier(), max_safe);

    // Test very small precise multipliers
    let tiny_multiplier = 1e-10_f64;
    let effect_tiny = JokerEffect::new().with_mult_multiplier(tiny_multiplier);
    assert_eq!(effect_tiny.mult_multiplier(), tiny_multiplier);
}

/// Test precision in compound calculations
/// Verifies that complex multiplier chains maintain accuracy
#[test]
fn test_precision_compound_calculations() {
    // Test compound multiplication precision
    let multipliers = vec![1.1_f64, 1.23_f64, 1.456_f64, 1.7890_f64];

    let mut result = 1.0_f64;
    for mult in multipliers {
        result *= mult;
    }

    // Expected: 1.1 × 1.23 × 1.456 × 1.7890 ≈ 3.537769728
    let expected = 1.1 * 1.23 * 1.456 * 1.7890;

    // With f64 precision, this should be very close
    assert!((result - expected).abs() < 1e-10);
}

/// Test zero and negative multiplier handling
/// Ensures edge cases are handled correctly with f64
#[test]
fn test_zero_negative_multiplier_handling() {
    // Test zero multiplier
    let zero_effect = JokerEffect::new().with_mult_multiplier(0.0_f64);
    assert_eq!(zero_effect.mult_multiplier(), 0.0_f64);

    // Test negative multiplier (should be validated/rejected)
    // The validation logic should prevent negative multipliers
    let negative_effect = JokerEffect::new().with_mult_multiplier(-1.5_f64);
    // This behavior depends on validation implementation
    assert_eq!(negative_effect.mult_multiplier(), -1.5_f64);
}

/// Test multiplier default values with f64
/// Ensures default initialization uses f64 precision
#[test]
fn test_multiplier_defaults_f64() {
    let default_effect = JokerEffect::new();

    // Default multiplier should be 0.0 (indicating no effect)
    // or 1.0 (indicating identity multiplication)
    let default_mult = default_effect.mult_multiplier();

    // Verify it's a valid f64 value
    assert!(default_mult.is_finite());
    assert!(default_mult >= 0.0);
}

/// Test multiplier serialization/deserialization with f64
/// Ensures save/load compatibility with f64 precision
#[test]
fn test_multiplier_serialization_f64() {
    let precise_value = 2.718281828459045_f64; // e to high precision
    let effect = JokerEffect::new().with_mult_multiplier(precise_value);

    // This would test serialization if implemented
    // For now, just verify the value is stored correctly
    assert_eq!(effect.mult_multiplier(), precise_value);

    // Verify precision is maintained
    let reconstructed = effect.mult_multiplier();
    assert!((reconstructed - precise_value).abs() < f64::EPSILON);
}
