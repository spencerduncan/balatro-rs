//! Minimal working test for MockRng

mod common;

use common::mocks::MockRng;

#[test]
fn test_mock_rng_works() {
    // Create a mock RNG with predetermined values
    let mut rng = MockRng::with_sequence(vec![0.1, 0.5, 0.9]);

    // Verify it returns the expected values
    assert_eq!(rng.next_f64(), 0.1);
    assert_eq!(rng.next_f64(), 0.5);
    assert_eq!(rng.next_f64(), 0.9);

    println!("MockRng is working correctly!");
}

#[test]
fn test_mock_rng_with_seed() {
    // Create a mock RNG with a specific seed
    let mut rng = MockRng::with_seed(42);

    // It should generate consistent values
    let value1 = rng.next_f64();
    let value2 = rng.next_f64();

    // Create another RNG with the same seed
    let mut rng2 = MockRng::with_seed(42);

    // Should get the same values
    assert_eq!(rng2.next_f64(), value1);
    assert_eq!(rng2.next_f64(), value2);

    println!("MockRng with seed is deterministic!");
}

#[test]
fn test_mock_rng_constant() {
    // Create a mock RNG that always returns the same value
    let mut rng = MockRng::constant(0.7);

    // Should always return 0.7
    for _ in 0..10 {
        assert_eq!(rng.next_f64(), 0.7);
    }

    println!("MockRng constant mode works!");
}
