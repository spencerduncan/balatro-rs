use balatro_rs::card::{Card, Suit, Value};
use balatro_rs::game::Game;
use balatro_rs::hand::SelectHand;
use balatro_rs::joker::{GameContext, Joker, JokerEffect, JokerId, JokerRarity};
use balatro_rs::stage::{Blind, Stage};

/// Test that JokerEffect supports f64 precision for chips
#[test]
fn test_joker_effect_f64_chips_precision() {
    // Test with fractional chips that require f64 precision
    let effect = JokerEffect::new().with_chips(123.456);
    assert_eq!(effect.chips, 123.456);

    // Test with very large chip values
    let large_effect = JokerEffect::new().with_chips(999999999.999);
    assert_eq!(large_effect.chips, 999999999.999);
}

/// Test that JokerEffect supports f64 precision for mult
#[test]
fn test_joker_effect_f64_mult_precision() {
    // Test with fractional mult that require f64 precision
    let effect = JokerEffect::new().with_mult(45.789);
    assert_eq!(effect.mult, 45.789);

    // Test with very large mult values
    let large_effect = JokerEffect::new().with_mult(888888888.888);
    assert_eq!(large_effect.mult, 888888888.888);
}

/// Test that JokerEffect supports f64 precision for money
#[test]
fn test_joker_effect_f64_money_precision() {
    // Test with fractional money that require f64 precision
    let effect = JokerEffect::new().with_money(67.123);
    assert_eq!(effect.money, 67.123);

    // Test with very large money values
    let large_effect = JokerEffect::new().with_money(777777777.777);
    assert_eq!(large_effect.money, 777777777.777);
}

/// Test that JokerEffect supports f64 precision for mult_multiplier
#[test]
fn test_joker_effect_f64_mult_multiplier_precision() {
    // Test with high precision multiplier values
    let effect = JokerEffect::new().with_mult_multiplier(2.123456789);
    assert_eq!(effect.mult_multiplier, 2.123456789);

    // Test with very large multiplier values
    let large_effect = JokerEffect::new().with_mult_multiplier(12345.6789);
    assert_eq!(large_effect.mult_multiplier, 12345.6789);
}

/// Test that compound f64 effects work correctly
#[test]
fn test_joker_effect_f64_compound_effects() {
    let effect = JokerEffect::new()
        .with_chips(100.5)
        .with_mult(25.25)
        .with_money(10.75)
        .with_mult_multiplier(1.5);

    assert_eq!(effect.chips, 100.5);
    assert_eq!(effect.mult, 25.25);
    assert_eq!(effect.money, 10.75);
    assert_eq!(effect.mult_multiplier, 1.5);
}

/// Test joker that provides f64 precision effects in game scoring
#[test]
fn test_f64_precision_joker_in_game() {
    let mut game = Game::default();
    game.start();
    game.stage = Stage::Blind(Blind::Small);
    game.blind = Some(Blind::Small);

    // Add a test joker that provides fractional bonuses
    let joker = TestF64PrecisionJoker::new();
    game.jokers = vec![Box::new(joker)];

    let cards = vec![
        Card::new(Value::Ace, Suit::Heart),
        Card::new(Value::King, Suit::Spade),
    ];
    let hand = SelectHand::new(cards).best_hand().unwrap();

    let score = game.calc_score(hand.clone());

    // With f64 precision, we should be able to handle fractional scoring
    // Base high card: chips=5, mult=1
    // Cards: A(11) + K(10) = 21 chips
    // TestF64PrecisionJoker: +0.5 chips, +0.25 mult, *1.5 multiplier
    // Total: (5 + 21 + 0.5) * ((1 + 0.25) * 1.5) = 26.5 * 1.875 = 49.6875
    // But since final score is rounded to integer, we expect 50
    assert!(
        score >= 49 && score <= 51,
        "Score should be approximately 50, got {}",
        score
    );
}

/// Test that extreme values don't cause overflow with f64
#[test]
fn test_joker_effect_extreme_f64_values() {
    // Test with very large values that would overflow i32
    let large_chips = 2_147_483_648.0; // Larger than i32::MAX
    let large_mult = 4_294_967_296.0; // Larger than u32::MAX
    let large_money = 9_223_372_036.0; // Large but within f64 range

    let effect = JokerEffect::new()
        .with_chips(large_chips)
        .with_mult(large_mult)
        .with_money(large_money)
        .with_mult_multiplier(1000.0);

    assert_eq!(effect.chips, large_chips);
    assert_eq!(effect.mult, large_mult);
    assert_eq!(effect.money, large_money);
    assert_eq!(effect.mult_multiplier, 1000.0);
}

/// Test that fractional arithmetic works correctly in joker effects
#[test]
fn test_joker_effect_fractional_arithmetic() {
    // Create effects that use fractional values
    let effect1 = JokerEffect::new().with_chips(10.5).with_mult(2.25);

    let effect2 = JokerEffect::new().with_chips(5.25).with_mult(1.75);

    // Simulate combining effects (this would happen in the game engine)
    let combined_chips = effect1.chips + effect2.chips;
    let combined_mult = effect1.mult + effect2.mult;

    assert_eq!(combined_chips, 15.75);
    assert_eq!(combined_mult, 4.0);
}

// Test helper joker for f64 precision testing

#[derive(Debug)]
struct TestF64PrecisionJoker;

impl TestF64PrecisionJoker {
    fn new() -> Self {
        Self
    }
}

impl Joker for TestF64PrecisionJoker {
    fn id(&self) -> JokerId {
        JokerId::Joker
    }

    fn name(&self) -> &str {
        "Test F64 Precision Joker"
    }

    fn description(&self) -> &str {
        "Test joker for f64 precision effects"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Common
    }

    fn on_hand_played(&self, _context: &mut GameContext, _hand: &SelectHand) -> JokerEffect {
        JokerEffect::new()
            .with_chips(0.5)
            .with_mult(0.25)
            .with_mult_multiplier(1.5)
    }
}
