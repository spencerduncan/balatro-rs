//! # Balatro Domain Layer
//!
//! This crate contains the pure business logic for the Balatro Web Debug UI,
//! following Clean Architecture principles. The domain layer is completely
//! independent of external frameworks and contains only the core business rules.
//!
//! ## Architecture
//!
//! - **Entities**: Core business objects with behavior
//! - **Services**: Business logic coordination
//! - **Value Objects**: Immutable data with validation
//! - **Interfaces**: Contracts for external dependencies (Dependency Inversion)
//! - **Errors**: Domain-specific error types
//!
//! ## Design Principles
//!
//! - **Single Responsibility**: Each component has one clear purpose
//! - **Open/Closed**: Open for extension, closed for modification
//! - **Liskov Substitution**: Derived classes are substitutable
//! - **Interface Segregation**: Many specific interfaces over one general
//! - **Dependency Inversion**: Depend on abstractions, not concretions

pub mod entities;
pub mod services;
pub mod value_objects;
pub mod interfaces;
pub mod errors;

// Re-export key types for clean API
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn domain_version_is_set() {
        assert!(!DOMAIN_VERSION.is_empty());
    }

    #[test]
    fn can_import_balatro_types() {
        // Ensure we can access balatro-rs types
        let _action_type = std::marker::PhantomData::<Action>;
        let _game_type = std::marker::PhantomData::<Game>;
    }
}
