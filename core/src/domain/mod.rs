//! # Domain Layer
//!
//! This module implements the domain layer following Domain-Driven Design (DDD) principles
//! and Clean Architecture patterns. It provides a clear separation between business logic
//! and infrastructure concerns.
//!
//! ## Architecture
//!
//! The domain layer is organized into the following components:
//!
//! - **Entities**: Core business entities with identity and lifecycle
//! - **Value Objects**: Immutable domain concepts without identity
//! - **Errors**: Domain-specific error types with proper error handling
//! - **Repositories**: Trait definitions for data access abstractions
//! - **Services**: Business logic services following SOLID principles
//!
//! ## Design Principles
//!
//! This module follows key Clean Code and SOLID principles:
//!
//! - **Single Responsibility**: Each component has one clear purpose
//! - **Open/Closed**: Open for extension, closed for modification
//! - **Liskov Substitution**: Interfaces are properly segregated
//! - **Interface Segregation**: Small, focused interfaces
//! - **Dependency Inversion**: Depend on abstractions, not concretions
//!
//! ## Usage
//!
//! ```rust
//! use balatro_rs::domain::{DomainResult, repositories::GameRepository};
//!
//! // Use repository traits for dependency injection
//! fn process_game<R: GameRepository>(repo: &R) -> DomainResult<()> {
//!     let game = repo.find_by_id("game-123")?;
//!     // Business logic here
//!     Ok(())
//! }
//! ```

pub mod entities;
pub mod errors;
pub mod repositories;
pub mod services;
pub mod value_objects;

// Re-export common types for convenience
pub use entities::GameSession;
pub use errors::{DomainError, DomainResult};
pub use value_objects::{Money, Score, SessionId};

/// Domain configuration and initialization
pub struct DomainConfig {
    /// Enable detailed error messages for development
    pub detailed_errors: bool,
    /// Maximum number of retries for transient failures
    pub max_retries: u32,
    /// Timeout for domain operations in milliseconds
    pub operation_timeout_ms: u64,
}

impl Default for DomainConfig {
    fn default() -> Self {
        Self {
            detailed_errors: cfg!(debug_assertions),
            max_retries: 3,
            operation_timeout_ms: 5000,
        }
    }
}

/// Initialize the domain layer
///
/// This function sets up any required domain-level infrastructure
/// and should be called during application startup.
pub fn initialize(config: DomainConfig) -> DomainResult<()> {
    // Future: Initialize domain event bus
    // Future: Set up domain metrics collection
    // Future: Configure domain-level logging

    // For now, just validate configuration
    if config.operation_timeout_ms == 0 {
        return Err(DomainError::Configuration(
            "Operation timeout must be greater than 0".into(),
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_domain_initialization() {
        let config = DomainConfig::default();
        assert!(initialize(config).is_ok());
    }

    #[test]
    fn test_invalid_configuration() {
        let config = DomainConfig {
            operation_timeout_ms: 0,
            ..Default::default()
        };
        assert!(initialize(config).is_err());
    }
}
