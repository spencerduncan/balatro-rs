//! Domain Layer
//!
//! This module contains domain-specific types and abstractions that enforce
//! business rules and invariants at the type level.

pub mod entities;
pub mod errors;
pub mod repositories;
pub mod services;
pub mod value_objects;

// Re-export commonly used value objects and types
pub use entities::GameSession;
pub use errors::{DomainError, DomainResult};
pub use value_objects::{
    Money, Score, SessionId, SessionIdError, ValidationError, ValidationResult,
};
