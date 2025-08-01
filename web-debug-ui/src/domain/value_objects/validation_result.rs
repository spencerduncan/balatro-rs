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
/// use balatro_domain::{ValidationResult, ValidationError};
///
/// // Valid operation
/// let valid = ValidationResult::Valid;
/// assert!(valid.is_valid());
///
/// // Invalid operation
/// let invalid = ValidationResult::Invalid(ValidationError::new(
///     "Cannot play cards when not in Play stage".to_string()
/// ));
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
    error_code: Option<String>,
    details: Option<String>,
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

    /// Create an invalid result with detailed error
    pub fn invalid_with_details<S: Into<String>>(
        reason: S,
        error_code: Option<String>,
        details: Option<String>,
    ) -> Self {
        Self::Invalid(ValidationError::with_details(
            reason.into(),
            error_code,
            details,
        ))
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
        Self {
            reason,
            error_code: None,
            details: None,
        }
    }

    /// Create a validation error with detailed information
    pub fn with_details(
        reason: String,
        error_code: Option<String>,
        details: Option<String>,
    ) -> Self {
        Self {
            reason,
            error_code,
            details,
        }
    }

    /// Get the error reason
    pub fn reason(&self) -> &str {
        &self.reason
    }

    /// Get the error code
    pub fn error_code(&self) -> Option<&str> {
        self.error_code.as_deref()
    }

    /// Get the error details
    pub fn details(&self) -> Option<&str> {
        self.details.as_deref()
    }
}

impl fmt::Display for ValidationResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Valid => write!(f, "Valid"),
            Self::Invalid(error) => write!(f, "{error}"),
        }
    }
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.reason)?;

        if let Some(code) = &self.error_code {
            write!(f, " (Code: {code})")?;
        }

        if let Some(details) = &self.details {
            write!(f, " - {details}")?;
        }

        Ok(())
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
    fn validation_result_invalid_with_details() {
        let result = ValidationResult::invalid_with_details(
            "Invalid action",
            Some("ACTION_001".to_string()),
            Some("Cannot play cards in Shop stage".to_string()),
        );

        assert!(result.is_invalid());
        let error = result.error().unwrap();
        assert_eq!(error.reason(), "Invalid action");
        assert_eq!(error.error_code(), Some("ACTION_001"));
        assert_eq!(error.details(), Some("Cannot play cards in Shop stage"));
    }

    #[test]
    fn validation_error_new_creates_simple_error() {
        let error = ValidationError::new("Simple error".to_string());

        assert_eq!(error.reason(), "Simple error");
        assert!(error.error_code().is_none());
        assert!(error.details().is_none());
    }

    #[test]
    fn validation_error_with_details_creates_detailed_error() {
        let error = ValidationError::with_details(
            "Detailed error".to_string(),
            Some("ERR_001".to_string()),
            Some("Additional context".to_string()),
        );

        assert_eq!(error.reason(), "Detailed error");
        assert_eq!(error.error_code(), Some("ERR_001"));
        assert_eq!(error.details(), Some("Additional context"));
    }

    #[test]
    fn validation_result_can_be_displayed() {
        let valid = ValidationResult::valid();
        assert_eq!(format!("{}", valid), "Valid");

        let invalid = ValidationResult::invalid("Test error");
        assert_eq!(format!("{}", invalid), "Invalid: Test error");
    }

    #[test]
    fn validation_error_can_be_displayed() {
        let simple = ValidationError::new("Simple error".to_string());
        assert_eq!(format!("{}", simple), "Simple error");

        let detailed = ValidationError::with_details(
            "Detailed error".to_string(),
            Some("ERR_001".to_string()),
            Some("More info".to_string()),
        );
        assert_eq!(
            format!("{}", detailed),
            "Detailed error (Code: ERR_001) - More info"
        );
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

    #[test]
    fn validation_error_converts_to_validation_result() {
        let error = ValidationError::new("Test error".to_string());
        let result: ValidationResult = error.into();

        assert!(result.is_invalid());
        assert_eq!(result.error().unwrap().reason(), "Test error");
    }

    #[test]
    fn validation_result_equality_works() {
        let valid1 = ValidationResult::valid();
        let valid2 = ValidationResult::valid();
        assert_eq!(valid1, valid2);

        let invalid1 = ValidationResult::invalid("Same error");
        let invalid2 = ValidationResult::invalid("Same error");
        assert_eq!(invalid1, invalid2);

        let different = ValidationResult::invalid("Different error");
        assert_ne!(invalid1, different);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn validation_result_can_be_serialized() {
        let valid = ValidationResult::valid();
        let serialized = serde_json::to_string(&valid).unwrap();
        let deserialized: ValidationResult = serde_json::from_str(&serialized).unwrap();
        assert_eq!(valid, deserialized);

        let invalid = ValidationResult::invalid("Test error");
        let serialized = serde_json::to_string(&invalid).unwrap();
        let deserialized: ValidationResult = serde_json::from_str(&serialized).unwrap();
        assert_eq!(invalid, deserialized);
    }
}
