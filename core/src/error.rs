//! # Error Handling Security Framework
//!
//! This module provides a security-focused error handling system that prevents
//! information disclosure while maintaining developer productivity.
//!
//! ## Security Architecture
//!
//! The error system uses a dual-error approach:
//! - **DeveloperError**: Detailed errors for internal debugging (never exposed to users)
//! - **UserError**: Sanitized, generic errors safe for user consumption
//!
//! ## Error Detail Levels
//!
//! Three configuration levels control information disclosure:
//! - `Production`: Only generic error messages (secure)
//! - `Development`: Slightly more specific errors for debugging
//! - `Testing`: Full error details for comprehensive testing
//!
//! ## Usage Guidelines
//!
//! ### For Library Developers
//! ```rust
//! use balatro_rs::error::{DeveloperGameError, UserError, ErrorSanitizer, ErrorDetailLevel};
//!
//! // Use DeveloperError types for internal error handling
//! fn risky_operation() -> Result<(), DeveloperGameError> {
//!     // Detailed error with sensitive information is OK here
//!     Err(DeveloperGameError::JokerNotFound("internal_path_42".to_string()))
//! }
//!
//! // Sanitize errors before exposing to users
//! fn public_api() -> Result<(), UserError> {
//!     let sanitizer = ErrorSanitizer::new(ErrorDetailLevel::Production);
//!     match risky_operation() {
//!         Ok(val) => Ok(val),
//!         Err(dev_error) => Err(sanitizer.sanitize_game_error(&dev_error))
//!     }
//! }
//! ```
//!
//! ### For Application Developers
//! ```rust
//! use balatro_rs::error::{UserError, ErrorDetailLevel, ErrorSanitizer};
//!
//! // Configure error detail level based on environment
//! let detail_level = if cfg!(debug_assertions) {
//!     ErrorDetailLevel::Development
//! } else {
//!     ErrorDetailLevel::Production
//! };
//!
//! let sanitizer = ErrorSanitizer::new(detail_level);
//! ```
//!
//! ## Security Best Practices
//!
//! 1. **Never expose DeveloperError directly to users**
//! 2. **Always use ErrorSanitizer for user-facing errors**
//! 3. **Log detailed errors internally for debugging**
//! 4. **Use Production mode in release builds**
//! 5. **Review error messages for information disclosure**
//!
//! ## Information Disclosure Prevention
//!
//! The system prevents disclosure of:
//! - File paths and system internals
//! - Stack traces and debug information
//! - Database queries and connection details
//! - User IDs and sensitive identifiers
//! - Configuration values and secrets
//!
//! ## Backward Compatibility
//!
//! Type aliases maintain compatibility with existing code:
//! ```rust
//! use balatro_rs::error::{DeveloperGameError, DeveloperPlayHandError, DeveloperActionSpaceError};
//!
//! // These still work but are deprecated
//! type GameError = DeveloperGameError;
//! type PlayHandError = DeveloperPlayHandError;
//! type ActionSpaceError = DeveloperActionSpaceError;
//! ```

#[cfg(feature = "python")]
use pyo3::exceptions::PyException;
#[cfg(feature = "python")]
use pyo3::prelude::*;
use thiserror::Error;

/// Configuration for error detail levels to control information disclosure
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ErrorDetailLevel {
    /// Production mode - only generic error messages (secure)
    Production,
    /// Development mode - detailed error messages for debugging
    Development,
    /// Testing mode - full error details for comprehensive testing
    Testing,
}

impl Default for ErrorDetailLevel {
    fn default() -> Self {
        #[cfg(debug_assertions)]
        return ErrorDetailLevel::Development;
        #[cfg(not(debug_assertions))]
        return ErrorDetailLevel::Production;
    }
}

/// User-facing errors with sanitized, generic messages to prevent information disclosure
#[derive(Error, Debug, Clone)]
pub enum UserError {
    #[error("Invalid input provided")]
    InvalidInput,
    #[error("Operation not permitted")]
    InvalidOperation,
    #[error("Resource not found")]
    NotFound,
    #[error("Operation failed")]
    OperationFailed,
    #[error("Invalid game state")]
    InvalidState,
    #[error("System error occurred")]
    SystemError,
}

/// Developer-facing errors with detailed information for debugging (preserved from original)
#[derive(Error, Debug, Clone)]
pub enum DeveloperPlayHandError {
    #[error("Played hand contains more than 5 cards")]
    TooManyCards,
    #[error("Played hand contains no cards")]
    NoCards,
    #[error("Played hand could not determine best hand")]
    UnknownHand,
}

/// Developer-facing errors with detailed information for debugging (preserved from original)
#[derive(Error, Debug, Clone)]
pub enum DeveloperGameError {
    #[error("No remaining discards")]
    NoRemainingDiscards,
    #[error("No remaining plays")]
    NoRemainingPlays,
    #[error("Invalid hand played")]
    InvalidHand(#[from] DeveloperPlayHandError),
    #[error("Invalid stage")]
    InvalidStage,
    #[error("Invalid action")]
    InvalidAction,
    #[error("No blind match")]
    InvalidBlind,
    #[error("No card match")]
    NoCardMatch,
    #[error("No joker match")]
    NoJokerMatch,
    #[error("Invalid move direction")]
    InvalidMoveDirection,
    #[error("No available slot")]
    NoAvailableSlot,
    #[error("Invalid balance")]
    InvalidBalance,
    #[error("Invalid move card")]
    InvalidMoveCard,
    #[error("Invalid select card")]
    InvalidSelectCard,
    #[error("Invalid action space")]
    InvalidActionSpace,
    #[error("Invalid slot index")]
    InvalidSlot,
    #[error("Joker not available in shop")]
    JokerNotInShop,
    #[error("Joker not found: {0}")]
    JokerNotFound(String),
    #[error("Invalid operation: {0}")]
    InvalidOperation(String),
    #[error("Input validation error: {0}")]
    InvalidInput(String),
    #[error("Mutex poisoned")]
    MutexPoisoned,
    #[error("Empty collection - cannot select random item")]
    EmptyCollection,
    #[error("Blind state missing when expected")]
    MissingBlindState,
    #[error("Empty iterator - cannot compute min/max")]
    EmptyIterator,
    #[error("Hand analysis failed: {0}")]
    HandAnalysisFailed(String),
    #[error("RNG operation failed: {0}")]
    RngFailed(String),
}

/// Developer-facing action space errors with detailed information
#[derive(Error, Debug, Clone)]
pub enum DeveloperActionSpaceError {
    #[error("Invalid index")]
    InvalidIndex,
    #[error("Invalid conversion to action")]
    InvalidActionConversion,
    #[error("Masked action")]
    MaskedAction,
}

impl std::convert::From<DeveloperActionSpaceError> for DeveloperGameError {
    fn from(_err: DeveloperActionSpaceError) -> DeveloperGameError {
        DeveloperGameError::InvalidActionSpace
    }
}

/// Error sanitizer that converts detailed developer errors to safe user errors
pub struct ErrorSanitizer {
    detail_level: ErrorDetailLevel,
}

impl ErrorSanitizer {
    pub fn new(detail_level: ErrorDetailLevel) -> Self {
        Self { detail_level }
    }

    /// Convert developer errors to user-safe errors based on configuration
    pub fn sanitize_game_error(&self, error: &DeveloperGameError) -> UserError {
        match self.detail_level {
            ErrorDetailLevel::Production => self.to_generic_user_error(error),
            ErrorDetailLevel::Development | ErrorDetailLevel::Testing => {
                // In development/testing, we can provide slightly more specific errors
                // but still avoid exposing internal system details
                self.to_specific_user_error(error)
            }
        }
    }

    fn to_generic_user_error(&self, error: &DeveloperGameError) -> UserError {
        match error {
            DeveloperGameError::InvalidInput(_) => UserError::InvalidInput,
            DeveloperGameError::InvalidOperation(_) => UserError::InvalidOperation,
            DeveloperGameError::NoCardMatch
            | DeveloperGameError::NoJokerMatch
            | DeveloperGameError::JokerNotFound(_)
            | DeveloperGameError::JokerNotInShop => UserError::NotFound,
            DeveloperGameError::InvalidHand(_)
            | DeveloperGameError::InvalidStage
            | DeveloperGameError::InvalidAction
            | DeveloperGameError::InvalidBlind
            | DeveloperGameError::InvalidBalance
            | DeveloperGameError::InvalidActionSpace
            | DeveloperGameError::InvalidSlot => UserError::InvalidState,
            _ => UserError::SystemError,
        }
    }

    fn to_specific_user_error(&self, error: &DeveloperGameError) -> UserError {
        match error {
            DeveloperGameError::InvalidInput(_) => UserError::InvalidInput,
            DeveloperGameError::InvalidOperation(_) => UserError::InvalidOperation,
            DeveloperGameError::NoCardMatch
            | DeveloperGameError::NoJokerMatch
            | DeveloperGameError::JokerNotFound(_)
            | DeveloperGameError::JokerNotInShop => UserError::NotFound,
            DeveloperGameError::InvalidHand(_)
            | DeveloperGameError::InvalidStage
            | DeveloperGameError::InvalidAction
            | DeveloperGameError::InvalidBlind
            | DeveloperGameError::InvalidBalance
            | DeveloperGameError::InvalidActionSpace
            | DeveloperGameError::InvalidSlot => UserError::InvalidState,
            DeveloperGameError::NoRemainingDiscards | DeveloperGameError::NoRemainingPlays => {
                UserError::OperationFailed
            }
            _ => UserError::SystemError,
        }
    }
}

impl Default for ErrorSanitizer {
    fn default() -> Self {
        Self::new(ErrorDetailLevel::default())
    }
}

/// Backward compatibility aliases (deprecated - use DeveloperGameError directly)
pub type PlayHandError = DeveloperPlayHandError;
pub type GameError = DeveloperGameError;
pub type ActionSpaceError = DeveloperActionSpaceError;

#[cfg(feature = "python")]
impl std::convert::From<DeveloperGameError> for PyErr {
    fn from(err: DeveloperGameError) -> PyErr {
        // In Python bindings, always use sanitized errors for security
        let sanitizer = ErrorSanitizer::default();
        let user_error = sanitizer.sanitize_game_error(&err);
        PyException::new_err(user_error.to_string())
    }
}

#[cfg(feature = "python")]
impl std::convert::From<UserError> for PyErr {
    fn from(err: UserError) -> PyErr {
        PyException::new_err(err.to_string())
    }
}
