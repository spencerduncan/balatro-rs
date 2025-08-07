//! Common test utilities and mocks
//!
//! This module provides shared testing infrastructure including
//! mock implementations, test helpers, and deterministic testing tools.

pub mod mocks;

// Re-export commonly used testing utilities
pub use mocks::{
    get_mock_config, reset_mock_config, set_mock_config, ActionRecorder, ActionScript,
    ActionSequence, ActionValidator, GameScenario, MockConfig, MockGameBuilder, MockRng, RngReplay,
    RngSequence, StateSnapshot, StateTransitionTracker,
};
