//! Domain Errors
//!
//! This module contains all domain-specific error types that represent
//! business rule violations and exceptional conditions in the domain layer.
//!
//! Following Clean Architecture principles, these errors are completely
//! independent of external frameworks and focus purely on domain concerns.

use crate::value_objects::SessionId;
use thiserror::Error;

/// Domain-specific errors that can occur during business operations
///
/// DomainError represents all possible failures that can occur within
/// the domain layer, focusing on business rule violations rather than
/// technical failures.
///
/// # Design Principles
///
/// - **Expressive**: Each error clearly indicates what business rule was violated
/// - **Actionable**: Errors provide enough context for proper handling
/// - **Hierarchical**: Related errors are grouped logically
/// - **Serializable**: Errors can be persisted or transmitted if needed
///
/// # Examples
///
/// ```
/// use balatro_domain::{DomainError, SessionId};
///
/// // Invalid action error
/// let error = DomainError::InvalidAction {
///     reason: "Cannot play cards during Shop stage".to_string()
/// };
///
/// // Session expiry error
/// let session_id = SessionId::new();
/// let expired = DomainError::SessionExpired {
///     session_id: session_id.to_string()
/// };
/// ```
#[derive(Debug, Clone, Error, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DomainError {
    /// Action is not valid for the current game state
    #[error("Invalid action for current game state: {reason}")]
    InvalidAction {
        /// Human-readable explanation of why the action is invalid
        reason: String
    },

    /// Game session has expired and cannot be used
    #[error("Session expired: {session_id}")]
    SessionExpired {
        /// ID of the expired session
        session_id: String
    },

    /// Game session was not found
    #[error("Session not found: {session_id}")]
    SessionNotFound {
        /// ID of the missing session
        session_id: String
    },

    /// Game state is in an inconsistent condition
    #[error("Game state inconsistency: {details}")]
    StateInconsistency {
        /// Detailed description of the inconsistency
        details: String
    },

    /// Action validation failed due to business rules
    #[error("Action validation failed: {reason}")]
    ValidationFailed {
        /// Specific validation failure reason
        reason: String
    },

    /// Session creation failed
    #[error("Failed to create session: {reason}")]
    SessionCreationFailed {
        /// Why session creation failed
        reason: String
    },

    /// Concurrent modification detected
    #[error("Concurrent modification detected for session: {session_id}")]
    ConcurrentModification {
        /// ID of the session with concurrent modification
        session_id: String
    },

    /// Repository operation failed
    #[error("Repository operation failed: {operation} - {reason}")]
    RepositoryError {
        /// The operation that failed
        operation: String,
        /// Why it failed
        reason: String
    },

    /// Business rule violation
    #[error("Business rule violation: {rule} - {context}")]
    BusinessRuleViolation {
        /// The business rule that was violated
        rule: String,
        /// Context about the violation
        context: String
    },
}

impl DomainError {
    /// Create an invalid action error
    pub fn invalid_action<S: Into<String>>(reason: S) -> Self {
        Self::InvalidAction {
            reason: reason.into(),
        }
    }

    /// Create a session expired error
    pub fn session_expired(session_id: &SessionId) -> Self {
        Self::SessionExpired {
            session_id: session_id.to_string(),
        }
    }

    /// Create a session not found error
    pub fn session_not_found(session_id: &SessionId) -> Self {
        Self::SessionNotFound {
            session_id: session_id.to_string(),
        }
    }

    /// Create a state inconsistency error
    pub fn state_inconsistency<S: Into<String>>(details: S) -> Self {
        Self::StateInconsistency {
            details: details.into(),
        }
    }

    /// Create a validation failed error
    pub fn validation_failed<S: Into<String>>(reason: S) -> Self {
        Self::ValidationFailed {
            reason: reason.into(),
        }
    }

    /// Create a session creation failed error
    pub fn session_creation_failed<S: Into<String>>(reason: S) -> Self {
        Self::SessionCreationFailed {
            reason: reason.into(),
        }
    }

    /// Create a concurrent modification error
    pub fn concurrent_modification(session_id: &SessionId) -> Self {
        Self::ConcurrentModification {
            session_id: session_id.to_string(),
        }
    }

    /// Create a repository error
    pub fn repository_error<S: Into<String>>(operation: S, reason: S) -> Self {
        Self::RepositoryError {
            operation: operation.into(),
            reason: reason.into(),
        }
    }

    /// Create a business rule violation error
    pub fn business_rule_violation<S: Into<String>>(rule: S, context: S) -> Self {
        Self::BusinessRuleViolation {
            rule: rule.into(),
            context: context.into(),
        }
    }

    /// Check if this error indicates a retryable condition
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            Self::ConcurrentModification { .. } | Self::RepositoryError { .. }
        )
    }

    /// Check if this error indicates a client error (4xx equivalent)
    pub fn is_client_error(&self) -> bool {
        matches!(
            self,
            Self::InvalidAction { .. }
                | Self::SessionExpired { .. }
                | Self::SessionNotFound { .. }
                | Self::ValidationFailed { .. }
                | Self::BusinessRuleViolation { .. }
        )
    }

    /// Check if this error indicates a server error (5xx equivalent)
    pub fn is_server_error(&self) -> bool {
        matches!(
            self,
            Self::StateInconsistency { .. }
                | Self::SessionCreationFailed { .. }
                | Self::ConcurrentModification { .. }
                | Self::RepositoryError { .. }
        )
    }
}

/// Type alias for domain results
pub type DomainResult<T> = Result<T, DomainError>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SessionId;

    #[test]
    fn invalid_action_error_creation() {
        let error = DomainError::invalid_action("Cannot play cards in shop");

        match &error {
            DomainError::InvalidAction { reason } => {
                assert_eq!(reason, "Cannot play cards in shop");
            }
            _ => panic!("Expected InvalidAction error"),
        }

        assert!(error.is_client_error());
        assert!(!error.is_server_error());
        assert!(!error.is_retryable());
    }

    #[test]
    fn session_expired_error_creation() {
        let session_id = SessionId::new();
        let error = DomainError::session_expired(&session_id);

        match &error {
            DomainError::SessionExpired { session_id: id } => {
                assert_eq!(id, &session_id.to_string());
            }
            _ => panic!("Expected SessionExpired error"),
        }

        assert!(error.is_client_error());
        assert!(!error.is_server_error());
        assert!(!error.is_retryable());
    }

    #[test]
    fn session_not_found_error_creation() {
        let session_id = SessionId::new();
        let error = DomainError::session_not_found(&session_id);

        match &error {
            DomainError::SessionNotFound { session_id: id } => {
                assert_eq!(id, &session_id.to_string());
            }
            _ => panic!("Expected SessionNotFound error"),
        }

        assert!(error.is_client_error());
    }

    #[test]
    fn state_inconsistency_error_creation() {
        let error = DomainError::state_inconsistency("Game state corrupted");

        match &error {
            DomainError::StateInconsistency { details } => {
                assert_eq!(details, "Game state corrupted");
            }
            _ => panic!("Expected StateInconsistency error"),
        }

        assert!(error.is_server_error());
        assert!(!error.is_client_error());
        assert!(!error.is_retryable());
    }

    #[test]
    fn validation_failed_error_creation() {
        let error = DomainError::validation_failed("Invalid hand composition");

        match &error {
            DomainError::ValidationFailed { reason } => {
                assert_eq!(reason, "Invalid hand composition");
            }
            _ => panic!("Expected ValidationFailed error"),
        }

        assert!(error.is_client_error());
    }

    #[test]
    fn session_creation_failed_error_creation() {
        let error = DomainError::session_creation_failed("Database unavailable");

        match &error {
            DomainError::SessionCreationFailed { reason } => {
                assert_eq!(reason, "Database unavailable");
            }
            _ => panic!("Expected SessionCreationFailed error"),
        }

        assert!(error.is_server_error());
    }

    #[test]
    fn concurrent_modification_error_creation() {
        let session_id = SessionId::new();
        let error = DomainError::concurrent_modification(&session_id);

        match &error {
            DomainError::ConcurrentModification { session_id: id } => {
                assert_eq!(id, &session_id.to_string());
            }
            _ => panic!("Expected ConcurrentModification error"),
        }

        assert!(error.is_server_error());
        assert!(error.is_retryable());
    }

    #[test]
    fn repository_error_creation() {
        let error = DomainError::repository_error("save_session", "Connection timeout");

        match &error {
            DomainError::RepositoryError { operation, reason } => {
                assert_eq!(operation, "save_session");
                assert_eq!(reason, "Connection timeout");
            }
            _ => panic!("Expected RepositoryError error"),
        }

        assert!(error.is_server_error());
        assert!(error.is_retryable());
    }

    #[test]
    fn business_rule_violation_error_creation() {
        let error = DomainError::business_rule_violation(
            "MaxJokersPerGame",
            "Attempted to add 6th joker when limit is 5"
        );

        match &error {
            DomainError::BusinessRuleViolation { rule, context } => {
                assert_eq!(rule, "MaxJokersPerGame");
                assert_eq!(context, "Attempted to add 6th joker when limit is 5");
            }
            _ => panic!("Expected BusinessRuleViolation error"),
        }

        assert!(error.is_client_error());
    }

    #[test]
    fn error_classification_methods() {
        // Client errors
        let client_errors = vec![
            DomainError::invalid_action("test"),
            DomainError::session_expired(&SessionId::new()),
            DomainError::session_not_found(&SessionId::new()),
            DomainError::validation_failed("test"),
            DomainError::business_rule_violation("rule", "context"),
        ];

        for error in client_errors {
            assert!(error.is_client_error(), "Should be client error: {:?}", error);
            assert!(!error.is_server_error(), "Should not be server error: {:?}", error);
        }

        // Server errors
        let server_errors = vec![
            DomainError::state_inconsistency("test"),
            DomainError::session_creation_failed("test"),
            DomainError::concurrent_modification(&SessionId::new()),
            DomainError::repository_error("op", "reason"),
        ];

        for error in server_errors {
            assert!(error.is_server_error(), "Should be server error: {:?}", error);
            assert!(!error.is_client_error(), "Should not be client error: {:?}", error);
        }

        // Retryable errors
        let retryable_errors = vec![
            DomainError::concurrent_modification(&SessionId::new()),
            DomainError::repository_error("op", "reason"),
        ];

        for error in retryable_errors {
            assert!(error.is_retryable(), "Should be retryable: {:?}", error);
        }
    }

    #[test]
    fn error_display_formatting() {
        let error = DomainError::invalid_action("Cannot play cards in shop");
        let formatted = format!("{}", error);
        assert!(formatted.contains("Invalid action"));
        assert!(formatted.contains("Cannot play cards in shop"));
    }

    #[test]
    fn error_equality_and_cloning() {
        let error1 = DomainError::invalid_action("Same reason");
        let error2 = DomainError::invalid_action("Same reason");
        let error3 = DomainError::invalid_action("Different reason");

        assert_eq!(error1, error2);
        assert_ne!(error1, error3);

        let cloned = error1.clone();
        assert_eq!(error1, cloned);
    }

    #[test]
    fn domain_result_type_alias_works() {
        let success: DomainResult<i32> = Ok(42);
        let failure: DomainResult<i32> = Err(DomainError::invalid_action("test"));

        assert!(success.is_ok());
        assert!(failure.is_err());
    }

    #[cfg(feature = "serde")]
    #[test]
    fn errors_can_be_serialized() {
        let error = DomainError::invalid_action("Test serialization");
        let serialized = serde_json::to_string(&error).unwrap();
        let deserialized: DomainError = serde_json::from_str(&serialized).unwrap();

        assert_eq!(error, deserialized);
    }
}
