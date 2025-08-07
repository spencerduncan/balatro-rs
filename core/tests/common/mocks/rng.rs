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

    /// Add values to the sequence
    pub fn push_sequence(&mut self, values: Vec<f64>) {
        self.sequence.extend(values);
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

    /// Get the next integer in range
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

/// Builder for creating complex RNG sequences
#[derive(Clone)]
pub struct RngSequence {
    values: Vec<f64>,
}

impl RngSequence {
    /// Create a new sequence builder
    pub fn new() -> Self {
        Self { values: Vec::new() }
    }

    /// Add a single value
    pub fn then(&mut self, value: f64) -> &mut Self {
        self.values.push(value);
        self
    }

    /// Add multiple identical values
    pub fn then_repeat(&mut self, value: f64, count: usize) -> &mut Self {
        self.values.extend(vec![value; count]);
        self
    }

    /// Add a range of values
    pub fn then_range(&mut self, start: f64, end: f64, steps: usize) -> &mut Self {
        if steps > 1 {
            let step = (end - start) / (steps - 1) as f64;
            for i in 0..steps {
                self.values.push(start + step * i as f64);
            }
        }
        self
    }

    /// Add random-looking but deterministic values
    pub fn then_pseudo_random(&mut self, count: usize, seed: u64) -> &mut Self {
        let mut rng = StdRng::seed_from_u64(seed);
        for _ in 0..count {
            self.values.push(rng.gen_range(0.0..1.0));
        }
        self
    }

    /// Build the MockRng
    pub fn build(self) -> MockRng {
        MockRng::with_sequence(self.values)
    }
}

/// Replay recorder for debugging test failures
#[derive(Debug, Clone)]
pub struct RngReplay {
    snapshots: Vec<RngSnapshot>,
}

#[derive(Debug, Clone)]
struct RngSnapshot {
    label: String,
    history: Vec<f64>,
    sequence_remaining: Vec<f64>,
}

impl RngReplay {
    /// Create a new replay recorder
    pub fn new() -> Self {
        Self {
            snapshots: Vec::new(),
        }
    }

    /// Take a snapshot of the RNG state
    pub fn snapshot(&mut self, rng: &MockRng, label: impl Into<String>) {
        self.snapshots.push(RngSnapshot {
            label: label.into(),
            history: rng.history.clone(),
            sequence_remaining: rng.sequence.iter().copied().collect(),
        });
    }

    /// Export replay data for debugging
    pub fn export(&self) -> String {
        let mut output = String::new();
        output.push_str("=== RNG Replay Data ===\n");

        for (i, snapshot) in self.snapshots.iter().enumerate() {
            output.push_str(&format!("\n[{}] {}\n", i, snapshot.label));
            output.push_str(&format!("  History: {:?}\n", snapshot.history));
            output.push_str(&format!("  Remaining: {:?}\n", snapshot.sequence_remaining));
        }

        output
    }

    /// Create a MockRng from a snapshot
    pub fn restore(&self, index: usize) -> Option<MockRng> {
        self.snapshots.get(index).map(|snapshot| {
            let mut rng = MockRng::with_sequence(snapshot.sequence_remaining.clone());
            rng.history = snapshot.history.clone();
            rng
        })
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
        let mut rng = MockRng::with_sequence(vec![0.0, 0.5, 1.0]);

        assert_eq!(rng.gen_range(0, 10), 0);
        assert_eq!(rng.gen_range(0, 10), 5);
        assert_eq!(rng.gen_range(0, 10), 10);
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
    fn test_rng_sequence_builder() {
        let mut rng = RngSequence::new()
            .then(0.1)
            .then_repeat(0.5, 3)
            .then_range(0.0, 1.0, 5)
            .build();

        assert_eq!(rng.next_f64(), 0.1);
        assert_eq!(rng.next_f64(), 0.5);
        assert_eq!(rng.next_f64(), 0.5);
        assert_eq!(rng.next_f64(), 0.5);
        assert_eq!(rng.next_f64(), 0.0);
        assert_eq!(rng.next_f64(), 0.25);
        assert_eq!(rng.next_f64(), 0.5);
        assert_eq!(rng.next_f64(), 0.75);
        assert_eq!(rng.next_f64(), 1.0);
    }

    #[test]
    fn test_rng_replay_snapshots() {
        let mut rng = MockRng::with_sequence(vec![0.1, 0.2, 0.3, 0.4]);
        let mut replay = RngReplay::new();

        rng.next_f64();
        replay.snapshot(&rng, "after first");

        rng.next_f64();
        rng.next_f64();
        replay.snapshot(&rng, "after third");

        // Restore from first snapshot
        if let Some(mut restored) = replay.restore(0) {
            assert_eq!(restored.next_f64(), 0.2);
        }

        // Restore from second snapshot
        if let Some(mut restored) = replay.restore(1) {
            assert_eq!(restored.next_f64(), 0.4);
        }
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
