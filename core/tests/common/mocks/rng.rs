//! Mock RNG System for Deterministic Testing
//!
//! Provides controlled random number generation for reproducible test scenarios.
//! Supports sequence-based predictable outcomes and replay capabilities.

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::collections::VecDeque;

/// Mock RNG that returns predetermined values in sequence
#[derive(Debug, Clone)]
pub struct MockRng {
    /// Sequence of values to return (0.0 to 1.0)
    sequence: VecDeque<f64>,

    /// Fallback RNG for when sequence is exhausted
    fallback: StdRng,

    /// Record of all generated values for replay
    history: Vec<f64>,

    /// Current position in replay mode
    replay_position: usize,

    /// Whether we're in replay mode
    replay_mode: bool,

    /// Configuration
    strict_mode: bool,
}

impl MockRng {
    /// Create a new MockRng with a predetermined sequence
    pub fn with_sequence(values: Vec<f64>) -> Self {
        Self {
            sequence: values.into_iter().collect(),
            fallback: StdRng::seed_from_u64(42),
            history: Vec::new(),
            replay_position: 0,
            replay_mode: false,
            strict_mode: true,
        }
    }

    /// Create a MockRng with a specific seed
    pub fn with_seed(seed: u64) -> Self {
        Self {
            sequence: VecDeque::new(),
            fallback: StdRng::seed_from_u64(seed),
            history: Vec::new(),
            replay_position: 0,
            replay_mode: false,
            strict_mode: false,
        }
    }

    /// Create a MockRng that always returns the same value
    pub fn constant(value: f64) -> Self {
        Self::with_sequence(vec![value; 1000])
    }

    /// Get the next value (0.0 to 1.0)
    pub fn next_f64(&mut self) -> f64 {
        if self.replay_mode {
            if self.replay_position < self.history.len() {
                let value = self.history[self.replay_position];
                self.replay_position += 1;
                return value;
            }
            // Exit replay mode when exhausted
            self.replay_mode = false;
        }

        let value = if let Some(v) = self.sequence.pop_front() {
            v
        } else if self.strict_mode {
            panic!("MockRng: Sequence exhausted in strict mode");
        } else {
            self.fallback.gen_range(0.0..1.0)
        };

        self.history.push(value);
        value
    }

    /// Get the next integer in range [min, max)
    pub fn gen_range(&mut self, min: i32, max: i32) -> i32 {
        let f = self.next_f64();
        let range = (max - min) as f64;
        min + (f * range) as i32
    }

    /// Get the next boolean with probability
    pub fn gen_bool(&mut self, probability: f64) -> bool {
        self.next_f64() < probability
    }

    /// Enable replay mode to repeat the same sequence
    pub fn start_replay(&mut self) {
        self.replay_mode = true;
        self.replay_position = 0;
    }

    /// Get the history of generated values
    pub fn get_history(&self) -> &[f64] {
        &self.history
    }

    /// Clear history and sequence
    pub fn reset(&mut self) {
        self.sequence.clear();
        self.history.clear();
        self.replay_position = 0;
        self.replay_mode = false;
    }

    /// Set strict mode (panic when sequence exhausted)
    pub fn set_strict(&mut self, strict: bool) {
        self.strict_mode = strict;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_rng_sequence() {
        let mut rng = MockRng::with_sequence(vec![0.1, 0.5, 0.9]);

        assert_eq!(rng.next_f64(), 0.1);
        assert_eq!(rng.next_f64(), 0.5);
        assert_eq!(rng.next_f64(), 0.9);
    }

    #[test]
    fn test_mock_rng_range() {
        let mut rng = MockRng::with_sequence(vec![0.0, 0.5, 0.99]);

        assert_eq!(rng.gen_range(0, 10), 0);
        assert_eq!(rng.gen_range(0, 10), 5);
        assert_eq!(rng.gen_range(0, 10), 9);
    }

    #[test]
    fn test_mock_rng_bool() {
        let mut rng = MockRng::with_sequence(vec![0.3, 0.7]);

        assert!(rng.gen_bool(0.5)); // 0.3 < 0.5
        assert!(!rng.gen_bool(0.5)); // 0.7 >= 0.5
    }

    #[test]
    fn test_mock_rng_replay() {
        let mut rng = MockRng::with_sequence(vec![0.1, 0.2, 0.3]);

        // Generate some values
        let v1 = rng.next_f64();
        let v2 = rng.next_f64();
        let v3 = rng.next_f64();

        // Start replay
        rng.start_replay();

        // Should get same values
        assert_eq!(rng.next_f64(), v1);
        assert_eq!(rng.next_f64(), v2);
        assert_eq!(rng.next_f64(), v3);
    }

    #[test]
    #[should_panic(expected = "Sequence exhausted")]
    fn test_strict_mode_panic() {
        let mut rng = MockRng::with_sequence(vec![0.5]);
        rng.set_strict(true);

        rng.next_f64(); // OK
        rng.next_f64(); // Should panic
    }
}
