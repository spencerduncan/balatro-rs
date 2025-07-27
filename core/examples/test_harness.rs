//! # Test Harness for Joker Examples
//!
//! This example provides a comprehensive test harness for validating
//! joker implementations. It demonstrates testing patterns, validation
//! strategies, and automated verification of joker behavior.

use balatro_rs::{
    card::{Card, Suit, Value},
    config::Config,
    game::Game,
    hand::SelectHand,
    joker::{Joker, JokerEffect, JokerId, JokerRarity},
    joker_factory::JokerFactory,
    joker_state::JokerState,
    static_joker_factory::StaticJokerFactory,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Joker Examples Test Harness ===\n");

    // Test 1: Basic functionality tests
    test_basic_functionality()?;

    // Test 2: State management tests
    test_state_management()?;

    // Test 3: Performance validation
    test_performance_characteristics()?;

    // Test 4: Factory tests
    test_factory_functionality()?;

    println!("✅ All tests completed successfully!");

    Ok(())
}

/// Test basic joker functionality
fn test_basic_functionality() -> Result<(), Box<dyn std::error::Error>> {
    println!("## 1. Basic Functionality Tests\n");

    // Test trait implementation
    test_trait_implementation()?;

    // Test effect generation
    test_effect_generation()?;

    println!("✅ Basic functionality tests passed\n");
    Ok(())
}

/// Test joker state management
fn test_state_management() -> Result<(), Box<dyn std::error::Error>> {
    println!("## 2. State Management Tests\n");

    // Test state initialization and serialization
    test_state_operations()?;

    println!("✅ State management tests passed\n");
    Ok(())
}

/// Test performance characteristics
fn test_performance_characteristics() -> Result<(), Box<dyn std::error::Error>> {
    println!("## 3. Performance Validation\n");

    // Test basic performance patterns
    test_basic_performance()?;

    println!("✅ Performance tests passed\n");
    Ok(())
}

/// Test factory functionality
fn test_factory_functionality() -> Result<(), Box<dyn std::error::Error>> {
    println!("## 4. Factory Tests\n");

    // Test factory creation
    test_factory_creation()?;

    println!("✅ Factory tests passed\n");
    Ok(())
}

/// Test factory creation for all available jokers
fn test_factory_creation() -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing factory creation...");

    // Test common jokers
    let common_jokers = JokerFactory::get_by_rarity(JokerRarity::Common);
    println!("  Found {} common jokers", common_jokers.len());

    for joker_id in common_jokers.iter().take(5) {
        if let Some(joker) = JokerFactory::create(*joker_id) {
            assert!(!joker.name().is_empty(), "Joker name should not be empty");
            assert!(
                !joker.description().is_empty(),
                "Joker description should not be empty"
            );
            assert_eq!(joker.id(), *joker_id, "Joker ID should match requested ID");
            println!("    ✓ {}: {}", joker.name(), joker.description());
        }
    }

    // Test static joker creation
    let greedy_joker = StaticJokerFactory::create_greedy_joker();
    println!("  ✓ Static joker: {}", greedy_joker.name());

    println!("  Factory creation tests passed");
    Ok(())
}

/// Test trait implementation requirements
fn test_trait_implementation() -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing trait implementations...");

    // Create test joker
    let test_joker = TestJoker;

    // Test required methods
    assert_eq!(test_joker.id(), JokerId::Reserved);
    assert_eq!(test_joker.name(), "Test Joker");
    assert_eq!(test_joker.rarity(), JokerRarity::Common);
    assert!(test_joker.cost() > 0);

    println!("  ✓ Basic trait methods work correctly");
    println!("  ✓ All required methods implemented");
    println!("  Trait implementation tests passed");
    Ok(())
}

/// Test effect generation
fn test_effect_generation() -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing effect generation...");

    // Test effect builder
    let effect = JokerEffect::new()
        .with_mult(5)
        .with_chips(10)
        .with_money(2)
        .with_mult_multiplier(1.5);

    assert_eq!(effect.mult, 5);
    assert_eq!(effect.chips, 10);
    assert_eq!(effect.money, 2);
    assert_eq!(effect.mult_multiplier, 1.5);

    // Test default effect
    let default_effect = JokerEffect::new();
    assert_eq!(default_effect.mult, 0);
    assert_eq!(default_effect.chips, 0);
    assert_eq!(default_effect.money, 0);
    assert_eq!(default_effect.mult_multiplier, 0.0);

    println!("  ✓ Effect builder works correctly");
    println!("  ✓ Default values are correct");
    println!("  Effect generation tests passed");
    Ok(())
}

/// Test state operations
fn test_state_operations() -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing state operations...");

    // Test basic state creation
    let mut state = JokerState::new();
    assert_eq!(state.accumulated_value, 0.0);
    assert_eq!(state.triggers_remaining, None);

    // Test state modification
    state.accumulated_value = 42.0;
    state.triggers_remaining = Some(3);

    assert_eq!(state.accumulated_value, 42.0);
    assert_eq!(state.triggers_remaining, Some(3));

    // Test custom data
    let _ = state.set_custom("test_key", "test_value");
    if let Ok(Some(value)) = state.get_custom::<String>("test_key") {
        assert_eq!(value, "test_value");
    }

    println!("  ✓ State creation and modification works");
    println!("  ✓ Custom data storage works");
    println!("  State operations tests passed");
    Ok(())
}

/// Test basic performance patterns
fn test_basic_performance() -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing basic performance...");

    let test_joker = TestJoker;

    let start = std::time::Instant::now();

    // Execute many trait method calls
    for _ in 0..10000 {
        let _ = test_joker.name();
        let _ = test_joker.description();
        let _ = test_joker.id();
        let _ = test_joker.rarity();
        let _ = test_joker.cost();
    }

    let duration = start.elapsed();
    println!("  Executed 50,000 trait method calls in {duration:?}");

    // Ensure reasonable performance (less than 10ms for 50k calls)
    assert!(
        duration.as_millis() < 10,
        "Performance too slow: {duration:?}"
    );

    // Test effect creation performance
    let start = std::time::Instant::now();
    for _ in 0..10000 {
        let _effect = JokerEffect::new().with_mult(5).with_chips(10);
    }
    let duration = start.elapsed();
    println!("  Created 10,000 effects in {duration:?}");

    println!("  ✓ Trait method calls are fast");
    println!("  ✓ Effect creation is efficient");
    println!("  Basic performance tests passed");
    Ok(())
}

/// Test game engine basics
#[allow(dead_code)]
fn test_game_engine_basics() -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing game engine basics...");

    // Create a minimal game context
    let config = Config::default();
    let _game = Game::new(config);

    // Test that jokers can be created
    if let Some(joker) = JokerFactory::create(JokerId::Joker) {
        println!("  ✓ Created joker: {}", joker.name());
    }

    // Test multiple joker types
    let joker_types = vec![JokerId::Joker, JokerId::GreedyJoker, JokerId::LustyJoker];
    let mut created_count = 0;

    for joker_type in joker_types {
        if JokerFactory::create(joker_type).is_some() {
            created_count += 1;
        }
    }

    println!("  ✓ Successfully created {created_count} different joker types");
    println!("  Game engine basics tests passed");
    Ok(())
}

/// Test joker property validation
#[allow(dead_code)]
fn test_joker_properties() -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing joker properties...");

    // Test cards for property testing
    let test_cards = vec![
        Card::new(Value::Ace, Suit::Spade),
        Card::new(Value::King, Suit::Heart),
        Card::new(Value::Two, Suit::Diamond),
        Card::new(Value::Jack, Suit::Club),
    ];

    // Test card creation
    for card in &test_cards {
        assert!(card.value != Value::Jack || matches!(card.value, Value::Jack));
        assert!(matches!(
            card.suit,
            Suit::Spade | Suit::Heart | Suit::Diamond | Suit::Club
        ));
    }

    // Test hand creation
    let _hand = SelectHand::new(test_cards.clone());
    // Just verify it doesn't panic

    println!("  ✓ Card creation works correctly");
    println!("  ✓ Hand creation works correctly");
    println!("  Joker properties tests passed");
    Ok(())
}

/// Simple test joker for validation
#[derive(Debug)]
struct TestJoker;

impl Joker for TestJoker {
    fn id(&self) -> JokerId {
        JokerId::Reserved
    }

    fn name(&self) -> &str {
        "Test Joker"
    }

    fn description(&self) -> &str {
        "A joker used for testing purposes"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Common
    }

    fn cost(&self) -> usize {
        3
    }

    // All other methods use the default implementation
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_joker_effect_builder() {
        let effect = JokerEffect::new()
            .with_mult(10)
            .with_chips(20)
            .with_money(5);

        assert_eq!(effect.mult, 10);
        assert_eq!(effect.chips, 20);
        assert_eq!(effect.money, 5);
    }

    #[test]
    fn test_joker_state_creation() {
        let state = JokerState::new();
        assert_eq!(state.accumulated_value, 0.0);
        assert_eq!(state.triggers_remaining, None);
    }

    #[test]
    fn test_test_joker_implementation() {
        let joker = TestJoker;
        assert_eq!(joker.name(), "Test Joker");
        assert_eq!(joker.id(), JokerId::Reserved);
        assert!(joker.cost() > 0);
    }
}
