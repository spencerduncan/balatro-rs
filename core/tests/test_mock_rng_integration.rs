//! Integration test demonstrating MockRng usage with game components
//!
//! This test shows how MockRng can be used for deterministic testing
//! of game mechanics that depend on random number generation.

use balatro_rs::card::{Card, Suit, Value};

// Import our mock RNG from the test utilities
mod common;
use common::mocks::MockRng;

/// Test that we can use MockRng to control card drawing deterministically
#[test]
fn test_deterministic_card_drawing() {
    // Create a MockRng with a specific sequence
    let mut rng = MockRng::with_sequence(vec![
        0.0,  // First draw position
        0.5,  // Second draw position
        0.99, // Third draw position
    ]);

    // Create a small deck for testing
    let cards = [
        Card::new(Value::Ace, Suit::Spade),
        Card::new(Value::King, Suit::Heart),
        Card::new(Value::Queen, Suit::Diamond),
        Card::new(Value::Jack, Suit::Club),
    ];

    // Simulate drawing cards with deterministic positions
    // In real game code, this would be integrated with Deck's shuffle/draw
    let draw_index1 = rng.gen_range(0, cards.len() as i32 - 1) as usize;
    let draw_index2 = rng.gen_range(0, cards.len() as i32 - 1) as usize;
    let draw_index3 = rng.gen_range(0, cards.len() as i32 - 1) as usize;

    // With our sequence, we should get consistent indices
    assert_eq!(draw_index1, 0); // 0.0 maps to index 0
    assert_eq!(draw_index2, 2); // 0.5 maps to index 2
    assert_eq!(draw_index3, 3); // 0.99 maps to index 3 (last valid index)

    // Verify the cards drawn are deterministic
    assert_eq!(cards[draw_index1].value, Value::Ace);
    assert_eq!(cards[draw_index2].value, Value::Queen);
    assert_eq!(cards[draw_index3].value, Value::Jack);
}

/// Test MockRng replay functionality for debugging failed tests
#[test]
fn test_replay_for_debugging() {
    let mut rng = MockRng::with_sequence(vec![0.1, 0.2, 0.3, 0.4, 0.5]);

    // Simulate some game operations
    let mut results = Vec::new();
    for _ in 0..3 {
        let value = rng.gen_range(1, 10);
        results.push(value);
    }

    // Start replay mode to reproduce the same sequence
    rng.start_replay();

    // Verify we get the same results
    for expected in &results {
        let value = rng.gen_range(1, 10);
        assert_eq!(value, *expected, "Replay should produce identical results");
    }
}

/// Test MockRng with strict mode for catching sequence exhaustion
#[test]
#[should_panic(expected = "Sequence exhausted")]
fn test_strict_mode_catches_exhaustion() {
    let mut rng = MockRng::with_sequence(vec![0.5]);
    rng.set_strict(true);

    // First call succeeds
    let _val1 = rng.gen_range(1, 100);

    // Second call should panic in strict mode
    let _val2 = rng.gen_range(1, 100);
}

/// Test MockRng configuration integration
#[test]
fn test_mock_config_integration() {
    // The default config has max_recorded_actions = 1000
    // MockRng::with_seed uses the default config
    let mut rng = MockRng::with_seed(12345);

    // First verify that history grows normally
    for i in 1..=10 {
        let _val = rng.next_f64();
        assert_eq!(
            rng.get_history().len(),
            i,
            "History should grow with each value generated"
        );
    }

    // Test that MockRng with sequence also respects history limits
    // The default max_recorded_actions is 1000, which is enough for this test
    let mut rng2 = MockRng::with_sequence(vec![0.1; 100]);
    for _ in 0..100 {
        let _val = rng2.next_f64();
    }
    assert_eq!(
        rng2.get_history().len(),
        100,
        "History should track all 100 generated values"
    );
}

/// Test that MockRng produces consistent results for game simulations
#[test]
fn test_game_simulation_consistency() {
    // Two RNGs with the same seed should produce identical results
    let mut rng1 = MockRng::with_seed(42);
    let mut rng2 = MockRng::with_seed(42);

    // Simulate multiple game operations
    for _ in 0..20 {
        let val1 = rng1.gen_range(1, 52); // Card positions
        let val2 = rng2.gen_range(1, 52);
        assert_eq!(val1, val2, "Same seed should produce same sequence");

        let bool1 = rng1.gen_bool(0.5); // 50% chance events
        let bool2 = rng2.gen_bool(0.5);
        assert_eq!(bool1, bool2, "Boolean generation should be consistent");
    }
}

/// Test MockRng with constant values for simplified testing
#[test]
fn test_constant_rng_for_edge_cases() {
    // Always return minimum value
    let mut min_rng = MockRng::constant(0.0);
    for _ in 0..5 {
        assert_eq!(min_rng.gen_range(10, 20), 10);
    }

    // Always return maximum value
    let mut max_rng = MockRng::constant(0.999);
    for _ in 0..5 {
        assert_eq!(max_rng.gen_range(10, 20), 20);
    }

    // Always return middle value
    let mut mid_rng = MockRng::constant(0.5);
    for _ in 0..5 {
        assert_eq!(mid_rng.gen_range(10, 20), 15);
    }
}
