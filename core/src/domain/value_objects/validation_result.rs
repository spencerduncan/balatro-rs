//! ValidationResult Value Object
//!
//! ValidationResult represents the outcome of validating a domain operation.
//! It encapsulates both success and failure states with detailed error information.

use std::fmt;

/// Result of validating a domain operation
///
/// ValidationResult provides a clear way to express whether a domain
/// operation (like applying an action) is valid or not, with detailed
/// error information when invalid.
///
/// # Examples
///
/// ```
/// use balatro_rs::domain::{ValidationResult, ValidationError};
///
/// // Valid operation
/// let valid = ValidationResult::valid();
/// assert!(valid.is_valid());
///
/// // Invalid operation
/// let invalid = ValidationResult::invalid("Cannot play cards when not in Play stage");
/// assert!(!invalid.is_valid());
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ValidationResult {
    Valid,
    Invalid(ValidationError),
}

/// Detailed information about a validation failure
///
/// ValidationError contains human-readable information about why
/// a domain operation failed validation.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ValidationError {
    reason: String,
}

impl ValidationResult {
    /// Create a valid result
    pub fn valid() -> Self {
        Self::Valid
    }

    /// Create an invalid result with a reason
    pub fn invalid<S: Into<String>>(reason: S) -> Self {
        Self::Invalid(ValidationError::new(reason.into()))
    }

    /// Check if the result is valid
    pub fn is_valid(&self) -> bool {
        matches!(self, Self::Valid)
    }

    /// Check if the result is invalid
    pub fn is_invalid(&self) -> bool {
        !self.is_valid()
    }

    /// Get the validation error if invalid
    pub fn error(&self) -> Option<&ValidationError> {
        match self {
            Self::Valid => None,
            Self::Invalid(error) => Some(error),
        }
    }

    /// Convert to Result<(), ValidationError>
    pub fn into_result(self) -> Result<(), ValidationError> {
        match self {
            Self::Valid => Ok(()),
            Self::Invalid(error) => Err(error),
        }
    }
}

impl ValidationError {
    /// Create a new validation error with a reason
    pub fn new(reason: String) -> Self {
        Self { reason }
    }

    /// Get the error reason
    pub fn reason(&self) -> &str {
        &self.reason
    }
}

impl fmt::Display for ValidationResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Valid => write!(f, "Valid"),
            Self::Invalid(error) => write!(f, "Invalid: {error}"),
        }
    }
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.reason)
    }
}

impl std::error::Error for ValidationError {}

impl From<ValidationError> for ValidationResult {
    fn from(error: ValidationError) -> Self {
        Self::Invalid(error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validation_result_valid_is_valid() {
        let result = ValidationResult::valid();

        assert!(result.is_valid());
        assert!(!result.is_invalid());
        assert!(result.error().is_none());
    }

    #[test]
    fn validation_result_invalid_is_invalid() {
        let result = ValidationResult::invalid("Test error");

        assert!(!result.is_valid());
        assert!(result.is_invalid());
        assert!(result.error().is_some());
        assert_eq!(result.error().unwrap().reason(), "Test error");
    }

    #[test]
    fn validation_error_new_creates_simple_error() {
        let error = ValidationError::new("Simple error".to_string());

        assert_eq!(error.reason(), "Simple error");
    }

    #[test]
    fn validation_result_can_be_displayed() {
        let valid = ValidationResult::valid();
        assert_eq!(format!("{valid}"), "Valid");

        let invalid = ValidationResult::invalid("Test error");
        assert_eq!(format!("{invalid}"), "Invalid: Test error");
    }

    #[test]
    fn validation_result_into_result_works() {
        let valid = ValidationResult::valid();
        assert!(valid.into_result().is_ok());

        let invalid = ValidationResult::invalid("Test error");
        let result = invalid.into_result();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().reason(), "Test error");
    }

    #[test]
    fn validation_error_implements_error_trait() {
        let error = ValidationError::new("Test error".to_string());
        let _: &dyn std::error::Error = &error;
    }
}
