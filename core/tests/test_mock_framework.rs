//! Mock Framework Integration Tests
//!
//! Demonstrates the mock framework capabilities through simple test scenarios.

mod common;

use common::{
    get_mock_config, reset_mock_config, set_mock_config, MockConfig, MockRng, RngReplay,
    RngSequence,
};

#[test]
fn test_deterministic_rng() {
    // Create a deterministic RNG
    let mut rng = MockRng::with_sequence(vec![0.5, 0.2, 0.8, 0.1, 0.9]);

    // Verify deterministic values
    assert_eq!(rng.next_f64(), 0.5);
    assert_eq!(rng.gen_range(0, 10), 2); // 0.2 * 11 = 2.2 -> 2
    assert!(!rng.gen_bool(0.5)); // 0.8 >= 0.5 = false
}

#[test]
fn test_rng_replay_for_debugging() {
    let mut rng = MockRng::with_sequence(vec![0.1, 0.2, 0.3, 0.4, 0.5]);
    let mut replay = RngReplay::new();

    // Simulate a game sequence with snapshots
    let v1 = rng.next_f64();
    replay.snapshot(&rng, "after_first_draw");

    let v2 = rng.next_f64();
    let v3 = rng.next_f64();
    replay.snapshot(&rng, "after_combat");

    // Verify original sequence
    assert_eq!(v1, 0.1);
    assert_eq!(v2, 0.2);
    assert_eq!(v3, 0.3);

    // Restore from snapshot and replay
    if let Some(mut restored) = replay.restore(0) {
        assert_eq!(restored.next_f64(), 0.2); // Should continue from snapshot
    }

    // Export for debugging
    let export = replay.export();
    assert!(export.contains("after_first_draw"));
    assert!(export.contains("after_combat"));
}

#[test]
fn test_mock_config_thread_safety() {
    // Set custom configuration
    let config = MockConfig {
        strict_validation: false,
        record_transitions: true,
        seed: 12345,
        max_recorded_actions: 500,
    };
    set_mock_config(config);

    // Spawn a thread with different config
    let handle = std::thread::spawn(|| {
        let thread_config = MockConfig {
            strict_validation: true,
            record_transitions: false,
            seed: 99999,
            max_recorded_actions: 100,
        };
        set_mock_config(thread_config);

        let retrieved = get_mock_config();
        retrieved.seed
    });

    // Main thread should keep its config
    let main_config = get_mock_config();
    assert_eq!(main_config.seed, 12345);

    // Thread should have different config
    let thread_seed = handle.join().unwrap();
    assert_eq!(thread_seed, 99999);
}

#[test]
fn test_complex_rng_sequence_builder() {
    let mut rng = RngSequence::new()
        .then(0.1)
        .then_repeat(0.5, 3)
        .then_range(0.0, 1.0, 5)
        .then_pseudo_random(3, 42)
        .build();

    // Verify the sequence
    assert_eq!(rng.next_f64(), 0.1);
    assert_eq!(rng.next_f64(), 0.5);
    assert_eq!(rng.next_f64(), 0.5);
    assert_eq!(rng.next_f64(), 0.5);
    assert_eq!(rng.next_f64(), 0.0);
    assert_eq!(rng.next_f64(), 0.25);
    assert_eq!(rng.next_f64(), 0.5);
    assert_eq!(rng.next_f64(), 0.75);
    assert_eq!(rng.next_f64(), 1.0);

    // The next values are pseudo-random but deterministic
    let v1 = rng.next_f64();
    let v2 = rng.next_f64();
    let v3 = rng.next_f64();

    // Create another RNG with same seed to verify determinism
    let mut rng2 = RngSequence::new()
        .then_repeat(0.0, 9) // Skip first 9 values
        .then_pseudo_random(3, 42)
        .build();

    for _ in 0..9 {
        rng2.next_f64();
    }

    assert_eq!(rng2.next_f64(), v1);
    assert_eq!(rng2.next_f64(), v2);
    assert_eq!(rng2.next_f64(), v3);
}

#[test]
fn test_mock_rng_constant_mode() {
    let mut rng = MockRng::constant(0.7);

    // Should always return the same value
    for _ in 0..10 {
        assert_eq!(rng.next_f64(), 0.7);
    }
}

#[test]
#[should_panic(expected = "Sequence exhausted")]
fn test_mock_rng_strict_mode() {
    let mut rng = MockRng::with_sequence(vec![0.5]);
    rng.set_strict(true);

    rng.next_f64(); // OK
    rng.next_f64(); // Should panic
}

/// Integration test showing full mock framework usage
#[test]
fn test_full_mock_integration() {
    // Configure mock framework
    set_mock_config(MockConfig {
        strict_validation: true,
        record_transitions: true,
        seed: 42,
        max_recorded_actions: 100,
    });

    // Create deterministic RNG
    let mut rng = RngSequence::new()
        .then_repeat(0.5, 5)
        .then_range(0.0, 1.0, 5)
        .build();

    // Use the RNG for testing
    let values: Vec<f64> = (0..10).map(|_| rng.next_f64()).collect();

    // Verify deterministic behavior
    assert_eq!(values[0], 0.5);
    assert_eq!(values[4], 0.5);
    assert_eq!(values[5], 0.0);
    assert_eq!(values[9], 1.0);

    // Reset config
    reset_mock_config();
    let config = get_mock_config();
    assert_eq!(config.seed, 42); // Back to default
}
