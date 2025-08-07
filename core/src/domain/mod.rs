//! Domain Layer
//!
//! This module contains domain-specific types and abstractions that enforce
//! business rules and invariants at the type level.

pub mod value_objects;

// Re-export commonly used value objects
pub use value_objects::{Money, Score, SessionId, ValidationError, ValidationResult};
