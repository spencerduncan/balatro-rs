//! Minimal Mock Framework for Deterministic Testing
//!
//! This module provides mock RNG capabilities for testing the Balatro game engine.
//! Day 1 implementation focuses on deterministic RNG for reproducible tests.
//! Advanced mocking features will be added in Day 2-3 of the salvage plan.

pub mod rng;

// Re-export MockRng for easy access
pub use rng::MockRng;

/// Configuration for mock framework behavior
#[derive(Debug, Clone)]
pub struct MockConfig {
    /// Enable strict validation of action sequences
    pub strict_validation: bool,

    /// Record all state transitions for debugging
    pub record_transitions: bool,

    /// Deterministic seed for reproducible tests
    pub seed: u64,

    /// Maximum number of actions to record
    pub max_recorded_actions: usize,
}

impl Default for MockConfig {
    fn default() -> Self {
        Self {
            strict_validation: true,
            record_transitions: false,
            seed: 42,
            max_recorded_actions: 1000,
        }
    }
}

// Global mock configuration (thread-local for test isolation)
thread_local! {
    static MOCK_CONFIG: std::cell::RefCell<MockConfig> = std::cell::RefCell::new(MockConfig::default());
}

/// Set the global mock configuration for the current thread
pub fn set_mock_config(config: MockConfig) {
    MOCK_CONFIG.with(|c| *c.borrow_mut() = config);
}

/// Get the current mock configuration
pub fn get_mock_config() -> MockConfig {
    MOCK_CONFIG.with(|c| c.borrow().clone())
}

/// Reset mock configuration to defaults
pub fn reset_mock_config() {
    MOCK_CONFIG.with(|c| *c.borrow_mut() = MockConfig::default());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_config() {
        // Test default configuration
        reset_mock_config();
        let config = get_mock_config();
        assert_eq!(config.seed, 42);
        assert!(config.strict_validation);

        // Test custom configuration
        let custom = MockConfig {
            strict_validation: false,
            record_transitions: true,
            seed: 12345,
            max_recorded_actions: 500,
        };
        set_mock_config(custom.clone());

        let retrieved = get_mock_config();
        assert_eq!(retrieved.seed, 12345);
        assert!(!retrieved.strict_validation);
        assert!(retrieved.record_transitions);
        assert_eq!(retrieved.max_recorded_actions, 500);

        // Reset and verify
        reset_mock_config();
        let reset = get_mock_config();
        assert_eq!(reset.seed, 42);
    }
}
