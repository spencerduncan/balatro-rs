//! # Domain Error Handling
//!
//! This module provides domain-specific error types that integrate with the
//! core error handling system while maintaining clean separation of concerns.
//!
//! ## Error Philosophy
//!
//! Following Clean Code principles, errors should:
//! - Be specific enough to be actionable
//! - Not leak implementation details
//! - Support proper error chaining
//! - Be testable and observable

use crate::error::{DeveloperGameError, UserError};
use std::borrow::Cow;
use std::fmt;

/// Domain-specific error type
///
/// This enum represents all possible errors that can occur in the domain layer.
/// Each variant follows the Single Responsibility Principle.
#[derive(Debug, Clone)]
pub enum DomainError {
    /// Entity not found in repository
    NotFound(Cow<'static, str>),

    /// Business rule validation failed
    ValidationFailed(Cow<'static, str>),

    /// Operation not allowed in current state
    InvalidState(Cow<'static, str>),

    /// Concurrency conflict detected
    ConcurrencyConflict(Cow<'static, str>),

    /// Repository operation failed
    RepositoryError(Cow<'static, str>),

    /// Service operation failed
    ServiceError(Cow<'static, str>),

    /// Configuration error
    Configuration(Cow<'static, str>),

    /// Authorization failure
    Unauthorized(Cow<'static, str>),

    /// External dependency failure
    ExternalServiceError(Cow<'static, str>),

    /// Timeout occurred
    Timeout(Cow<'static, str>),
}

impl fmt::Display for DomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound(msg) => write!(f, "Entity not found: {msg}"),
            Self::ValidationFailed(msg) => write!(f, "Validation failed: {msg}"),
            Self::InvalidState(msg) => write!(f, "Invalid state: {msg}"),
            Self::ConcurrencyConflict(msg) => write!(f, "Concurrency conflict: {msg}"),
            Self::RepositoryError(msg) => write!(f, "Repository error: {msg}"),
            Self::ServiceError(msg) => write!(f, "Service error: {msg}"),
            Self::Configuration(msg) => write!(f, "Configuration error: {msg}"),
            Self::Unauthorized(msg) => write!(f, "Unauthorized: {msg}"),
            Self::ExternalServiceError(msg) => write!(f, "External service error: {msg}"),
            Self::Timeout(msg) => write!(f, "Operation timeout: {msg}"),
        }
    }
}

impl std::error::Error for DomainError {}

/// Result type for domain operations
pub type DomainResult<T> = Result<T, DomainError>;

/// Convert domain errors to developer errors for internal use
impl From<DomainError> for DeveloperGameError {
    fn from(err: DomainError) -> Self {
        match err {
            DomainError::NotFound(msg) => {
                DeveloperGameError::InvalidOperation(format!("Domain: {msg}"))
            }
            DomainError::ValidationFailed(msg) => {
                DeveloperGameError::InvalidInput(format!("Domain validation: {msg}"))
            }
            DomainError::InvalidState(_) => DeveloperGameError::InvalidStage,
            _ => DeveloperGameError::InvalidOperation(format!("Domain error: {err}")),
        }
    }
}

/// Convert domain errors to user-safe errors
impl From<DomainError> for UserError {
    fn from(err: DomainError) -> Self {
        match err {
            DomainError::NotFound(_) => UserError::NotFound,
            DomainError::ValidationFailed(_) => UserError::InvalidInput,
            DomainError::InvalidState(_) => UserError::InvalidState,
            DomainError::Unauthorized(_) => UserError::InvalidOperation,
            DomainError::Timeout(_) => UserError::OperationFailed,
            _ => UserError::SystemError,
        }
    }
}

/// Error context builder for better error messages
///
/// Following the Builder pattern for constructing detailed error contexts
pub struct ErrorContext {
    operation: Cow<'static, str>,
    entity: Option<Cow<'static, str>>,
    details: Vec<Cow<'static, str>>,
}

impl ErrorContext {
    /// Create a new error context
    pub fn new(operation: impl Into<Cow<'static, str>>) -> Self {
        Self {
            operation: operation.into(),
            entity: None,
            details: Vec::new(),
        }
    }

    /// Add entity information
    pub fn with_entity(mut self, entity: impl Into<Cow<'static, str>>) -> Self {
        self.entity = Some(entity.into());
        self
    }

    /// Add detail information
    pub fn with_detail(mut self, detail: impl Into<Cow<'static, str>>) -> Self {
        self.details.push(detail.into());
        self
    }

    /// Build the error message
    pub fn build(self) -> Cow<'static, str> {
        use std::fmt::Write;

        // Pre-allocate buffer with reasonable capacity
        let mut buffer = String::with_capacity(128);

        write!(&mut buffer, "Operation: {}", self.operation).unwrap();

        if let Some(entity) = self.entity {
            write!(&mut buffer, "; Entity: {entity}").unwrap();
        }

        if !self.details.is_empty() {
            write!(&mut buffer, "; Details: ").unwrap();
            for (i, detail) in self.details.iter().enumerate() {
                if i > 0 {
                    write!(&mut buffer, ", ").unwrap();
                }
                write!(&mut buffer, "{detail}").unwrap();
            }
        }

        Cow::Owned(buffer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = DomainError::NotFound(Cow::Borrowed("Game session 123"));
        assert_eq!(err.to_string(), "Entity not found: Game session 123");
    }

    #[test]
    fn test_error_context_builder() {
        let context = ErrorContext::new("UpdateGame")
            .with_entity("GameSession")
            .with_detail("Invalid state transition")
            .build();

        assert!(context.contains("Operation: UpdateGame"));
        assert!(context.contains("Entity: GameSession"));
        assert!(context.contains("Invalid state transition"));
    }

    #[test]
    fn test_domain_to_developer_error_conversion() {
        let domain_err = DomainError::ValidationFailed(Cow::Borrowed("Invalid move"));
        let dev_err: DeveloperGameError = domain_err.into();

        match dev_err {
            DeveloperGameError::InvalidInput(msg) => {
                assert!(msg.contains("Domain validation"));
            }
            _ => panic!("Unexpected error type"),
        }
    }

    #[test]
    fn test_domain_to_user_error_conversion() {
        let domain_err = DomainError::Unauthorized(Cow::Borrowed("Access denied"));
        let user_err: UserError = domain_err.into();

        assert!(matches!(user_err, UserError::InvalidOperation));
    }
}
