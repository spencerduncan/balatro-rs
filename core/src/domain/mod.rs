//! # Domain Layer
//!
//! Simple domain types for the Balatro game engine.
//! No enterprise patterns, just useful value objects and entities.

pub mod entities;
pub mod errors;
pub mod value_objects;

// Re-export common types for convenience
pub use entities::GameSession;
pub use errors::{DomainError, DomainResult};
pub use value_objects::{Money, Score, SessionId};
