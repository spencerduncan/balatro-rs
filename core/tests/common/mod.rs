//! Common test utilities and mocks
//!
//! This module provides shared testing infrastructure including
//! mock implementations, test helpers, and deterministic testing tools.

#![allow(unused_imports)]

pub mod mocks;

// Re-export the available mock types
pub use mocks::{get_mock_config, reset_mock_config, set_mock_config, MockConfig, MockRng, RngReplay, RngSequence};
