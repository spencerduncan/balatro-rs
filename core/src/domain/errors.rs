//! Simple domain error handling

use crate::error::{DeveloperGameError, UserError};
use std::fmt;

/// Domain error type
#[derive(Debug, Clone)]
pub enum DomainError {
    NotFound(String),
    ValidationFailed(String),
    InvalidState(String),
}

impl fmt::Display for DomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound(msg) => write!(f, "Not found: {msg}"),
            Self::ValidationFailed(msg) => write!(f, "Validation failed: {msg}"),
            Self::InvalidState(msg) => write!(f, "Invalid state: {msg}"),
        }
    }
}

impl std::error::Error for DomainError {}

pub type DomainResult<T> = Result<T, DomainError>;

impl From<DomainError> for DeveloperGameError {
    fn from(err: DomainError) -> Self {
        match err {
            DomainError::NotFound(msg) => DeveloperGameError::InvalidOperation(msg),
            DomainError::ValidationFailed(msg) => DeveloperGameError::InvalidInput(msg),
            DomainError::InvalidState(_) => DeveloperGameError::InvalidStage,
        }
    }
}

impl From<DomainError> for UserError {
    fn from(err: DomainError) -> Self {
        match err {
            DomainError::NotFound(_) => UserError::NotFound,
            DomainError::ValidationFailed(_) => UserError::InvalidInput,
            DomainError::InvalidState(_) => UserError::InvalidState,
        }
    }
}
