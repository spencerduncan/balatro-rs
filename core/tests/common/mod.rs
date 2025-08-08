//! Common test utilities and mocks
//!
//! This module provides shared testing infrastructure including
//! mock implementations, test helpers, and deterministic testing tools.

#![allow(unused_imports)]

pub mod mocks;

// Re-export only the available mock types
pub use mocks::MockRng;
