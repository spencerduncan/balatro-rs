//! Domain Layer Module
//!
//! This module exports all domain layer components including entities, services,
//! value objects, interfaces, and errors. The domain layer contains pure business
//! logic and is independent of external frameworks.

pub mod entities;
pub mod services;
pub mod value_objects;
pub mod interfaces;
pub mod errors;

// Re-export key types for clean API - matching the original lib.rs structure
pub use entities::{GameSession};
pub use entities::game_session::{GameConfig, GameStats};
pub use services::{ActionValidator, BalatroActionValidator};
pub use value_objects::{SessionId, ValidationResult, ValidationError};
pub use interfaces::{GameRepository, StateNotifier, ActionResult};
pub use errors::{DomainError, DomainResult};

// Re-export balatro-rs types for convenience
// Temporarily disabled due to balatro-rs compilation issues
// #[cfg(not(test))]
// pub use balatro_rs::{Action, Game};

// Use stubs when balatro-rs is not available (for testing and development)
pub mod stubs;
pub use stubs::{Action, Game};

/// Domain layer version for compatibility tracking
pub const DOMAIN_VERSION: &str = "0.1.0";