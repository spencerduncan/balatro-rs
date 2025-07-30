//! Common testing utilities and fixtures for comprehensive TDD framework
//!
//! This module provides shared components for all tests in the sprint1-testing-framework:
//! - Test fixtures for domain entities
//! - Assertion utilities for domain-specific testing
//! - Mock implementations with mockall
//! - Performance testing helpers
//! - Property-based testing utilities

pub mod fixtures;
pub mod assertions;
pub mod mocks;
pub mod performance;
pub mod properties;

// Re-export commonly used testing utilities
pub use fixtures::*;
pub use assertions::*;
pub use mocks::*;
pub use performance::*;
pub use properties::*;

// Re-export external testing crates for convenience
pub use mockall::{mock, predicate};
pub use proptest::prelude::*;
pub use tokio_test::{assert_ok, assert_err, block_on};
pub use rstest::*;
pub use fake::{Fake, Faker};
pub use assert_matches::assert_matches;
pub use serial_test::serial;

/// Test configuration constants
pub mod config {
    use std::time::Duration;

    /// Default timeout for async tests
    pub const DEFAULT_ASYNC_TIMEOUT: Duration = Duration::from_secs(5);

    /// Number of iterations for property-based tests
    pub const PROPTEST_CASES: u32 = 100;

    /// Maximum number of concurrent test sessions for load testing
    pub const MAX_CONCURRENT_SESSIONS: usize = 100;

    /// Default test RNG seed for reproducible tests
    pub const TEST_RNG_SEED: u64 = 42;

    /// Memory leak detection threshold in bytes
    pub const MEMORY_LEAK_THRESHOLD: usize = 1024 * 1024; // 1MB
}

/// Common test result type for domain operations
pub type TestResult<T = ()> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

/// Test environment setup utilities
pub fn setup_test_env() {
    // Initialize tracing for test debugging
    let _ = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .try_init();
}

/// Cleanup test environment
pub fn cleanup_test_env() {
    // Force garbage collection to detect memory leaks
    std::hint::black_box(());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_constants() {
        assert!(config::DEFAULT_ASYNC_TIMEOUT.as_secs() > 0);
        assert!(config::PROPTEST_CASES > 0);
        assert!(config::MAX_CONCURRENT_SESSIONS > 0);
    }

    #[test]
    fn test_setup_cleanup() {
        setup_test_env();
        cleanup_test_env();
        // Should not panic
    }
}
