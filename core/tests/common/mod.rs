//! Common test utilities and infrastructure for balatro-rs
//!
//! This module provides comprehensive testing utilities including:
//! - Test fixtures for game states, cards, and actions
//! - Domain-specific assertions
//! - Helper functions for test scenarios
//! - Mock implementations (when mockall feature is enabled)
//!
//! ## Production Engineering Patterns from PR #779
//! This test infrastructure implements production-ready patterns salvaged from PR #779:
//! - Builder patterns for complex test data creation
//! - Deterministic test data generation for reproducible tests
//! - Performance monitoring and benchmarking utilities
//! - Resource lifecycle management with setup/teardown
//! - State snapshot testing for regression detection
//!
//! ## Architecture
//! The test infrastructure is organized into three main modules:
//! - `fixtures`: Test data factories and builders
//! - `assertions`: Domain-specific validation functions
//! - `helpers`: Test execution utilities and environment management

pub mod assertions;
pub mod fixtures;
pub mod helpers;

// ============================================================================
// CORE EXPORTS - Always available
// ============================================================================

// Re-export commonly used items for convenience
pub use assertions::*;
pub use fixtures::*;
pub use helpers::*;

// ============================================================================
// BUILDER PATTERN EXPORTS
// ============================================================================

/// Test data builders for fluent test creation
pub mod builders {
    pub use super::fixtures::{
        GameStateBuilder,
        JokerTestBuilder,
        DeckBuilder,
        TestDataGenerator,
    };
}

// ============================================================================
// ASSERTION EXPORTS
// ============================================================================

/// Domain-specific assertions for game testing
pub mod asserts {
    pub use super::assertions::{
        assert_action_valid,
        assert_action_invalid,
        assert_hand_rank,
        assert_game_stage,
        assert_game_ended,
        assert_game_running,
        assert_joker_effect,
        assert_joker_effect_neutral,
        assert_score_in_range,
        assert_money_in_range,
        assert_ante_level,
        assert_round_number,
        assert_deck_size,
        // Production assertions
        assert_game_state_equals,
        assert_game_state_snapshot,
        assert_money_never_negative,
        assert_ante_progression_valid,
        assert_round_progression_valid,
        assert_action_completes_within,
        assert_joker_collection_valid,
        assert_scoring_correct,
        assert_valid_state_transition,
        assert_actions_deterministic,
    };
}

// ============================================================================
// TEST ENVIRONMENT EXPORTS
// ============================================================================

/// Test environment and configuration utilities
pub mod environment {
    pub use super::helpers::{
        TestEnvironment,
        SeedManager,
        TestFixture,
        PerformanceMonitor,
        GameStateRecorder,
        TestValidator,
    };
}

// ============================================================================
// PERFORMANCE TESTING EXPORTS
// ============================================================================

/// Performance testing utilities
pub mod performance {
    pub use super::fixtures::{
        create_performance_test_data,
        PerformanceTestData,
        create_concurrent_test_fixtures,
        create_memory_test_fixtures,
        MemoryTestFixtures,
    };

    pub use super::helpers::{
        measure_execution_time,
        PerformanceMonitor,
        PerformanceMeasurement,
    };

    pub use super::assertions::{
        assert_action_completes_within,
    };
}

// ============================================================================
// SNAPSHOT TESTING EXPORTS
// ============================================================================

/// Snapshot testing utilities for regression detection
pub mod snapshot {
    pub use super::assertions::{
        GameStateSnapshot,
        StateTolerance,
        assert_game_state_snapshot,
        assert_game_state_equals,
    };

    pub use super::helpers::{
        GameStateRecorder,
        GameStateRecord,
    };
}

// ============================================================================
// COMMON TEST PATTERNS
// ============================================================================

/// Common test patterns and utilities
pub mod patterns {
    pub use super::helpers::{
        run_parameterized_test,
        retry_test,
        execute_action_sequence,
        play_until_game_over,
        game_with_action,
    };

    pub use super::fixtures::{
        create_edge_case_scenarios,
        TestScenario,
    };
}

// ============================================================================
// PRODUCTION TEST PRELUDE
// ============================================================================

/// A prelude for production-ready tests
///
/// Import this to get all commonly used test utilities:
/// ```rust
/// use common::prelude::*;
/// ```
pub mod prelude {
    pub use super::fixtures::{
        create_test_game,
        create_test_game_with_seed,
        create_test_deck,
        create_test_hand,
        create_test_actions,
        TestHandType,
        GameStateBuilder,
    };

    pub use super::assertions::{
        assert_action_valid,
        assert_game_running,
        assert_game_ended,
        assert_money_never_negative,
    };

    pub use super::helpers::{
        TestEnvironment,
        measure_execution_time,
        execute_action_sequence,
    };
}

// ============================================================================
// FEATURE-GATED EXPORTS
// ============================================================================

#[cfg(feature = "mock")]
/// Mock implementations for testing (requires `mock` feature)
pub mod mocks {
    // Mock implementations would go here when added
    // This is a placeholder for future mock framework integration
}

#[cfg(feature = "proptest")]
/// Property-based testing utilities (requires `proptest` feature)
pub mod property {
    // Property testing strategies would go here
    // This is a placeholder for proptest integration
}

// ============================================================================
// TEST INFRASTRUCTURE DOCUMENTATION
// ============================================================================

/// # Test Infrastructure Usage Guide
///
/// ## Quick Start
///
/// ```rust
/// use common::prelude::*;
///
/// #[test]
/// fn test_game_scenario() {
///     // Use builder pattern for complex setup
///     let game = GameStateBuilder::new()
///         .with_ante(3)
///         .with_money(50)
///         .with_seed(42)  // Deterministic testing
///         .build();
///
///     // Use domain assertions
///     assert_game_running(&game);
///     assert_money_never_negative(&game);
/// }
/// ```
///
/// ## Performance Testing
///
/// ```rust
/// use common::performance::*;
///
/// #[test]
/// fn test_performance() {
///     let mut monitor = PerformanceMonitor::new("my_test");
///
///     monitor.measure("operation", || {
///         // Code to benchmark
///     });
///
///     let avg = monitor.get_average_duration("operation");
///     assert!(avg.unwrap() < Duration::from_millis(10));
/// }
/// ```
///
/// ## Snapshot Testing
///
/// ```rust
/// use common::snapshot::*;
///
/// #[test]
/// fn test_state_snapshot() {
///     let game = create_test_game();
///     let snapshot = GameStateSnapshot::from(&game);
///
///     // Later, verify state matches snapshot
///     assert_game_state_snapshot(&game, &snapshot, None);
/// }
/// ```
pub struct _Documentation;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_exports() {
        // Test that key exports are available
        let _game = create_test_game();
        let _env = TestEnvironment::default();
        let _snapshot = GameStateSnapshot {
            ante: 1,
            round: 1,
            money: 4,
            chips: 0,
            mult: 0,
            score: 0,
            stage: balatro_rs::stage::Stage::PreBlind,
            joker_count: 0,
        };
    }

    #[test]
    fn test_builder_pattern() {
        let game = GameStateBuilder::new()
            .with_ante(2)
            .with_money(10)
            .build();

        assert_eq!(game.ante, 2);
        assert_eq!(game.money, 10);
    }

    #[test]
    fn test_prelude_imports() {
        use prelude::*;

        let game = create_test_game();
        assert_game_running(&game);

        let (_, duration) = measure_execution_time(|| {
            create_test_deck()
        });
        assert!(duration.as_nanos() > 0);
    }
}
