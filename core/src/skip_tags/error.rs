//! Comprehensive error handling framework for the skip tag system
//!
//! This module provides a security-focused error handling system following
//! the established patterns from the main error module. It prevents information
//! disclosure while maintaining developer productivity.
//!
//! # Security Architecture
//!
//! The error system uses the same dual-error approach as the main system:
//! - **TagError**: Detailed errors for internal debugging (never exposed to users)
//! - **UserError**: Sanitized, generic errors safe for user consumption (via main error system)
//!
//! # Error Categories
//!
//! - **Tag Lookup Errors**: Registry and factory failures
//! - **Application Errors**: Tag effect application failures
//! - **Validation Errors**: Game state and condition validation failures
//! - **System Errors**: Internal system and threading errors
//!
//! # Usage Guidelines
//!
//! ```rust,ignore
//! use crate::skip_tags::error::{TagError, TagErrorKind};
//!
//! // Detailed error for internal use
//! fn apply_tag_effect() -> Result<(), TagError> {
//!     Err(TagError::new(
//!         TagErrorKind::InsufficientResources,
//!         "Player has insufficient money for Economy tag effect"
//!     ))
//! }
//!
//! // Convert to user-safe error
//! match apply_tag_effect() {
//!     Ok(_) => {},
//!     Err(tag_error) => {
//!         let user_error = tag_error.into(); // Converts to DeveloperGameError
//!         return Err(user_error);
//!     }
//! }
//! ```

use crate::error::DeveloperGameError;
use crate::skip_tags::TagId;
use thiserror::Error;

/// Comprehensive error type for skip tag system operations.
///
/// This error type provides detailed information for debugging while being
/// designed to convert safely to user-facing errors through the main error system.
///
/// # Design Principles
///
/// - **Detailed Context**: Includes specific error kinds, messages, and context
/// - **Safe Conversion**: Converts to DeveloperGameError for user-facing APIs
/// - **Performance**: Efficient creation and handling for hot paths
/// - **Debuggable**: Rich information for developer debugging
#[derive(Error, Debug, Clone)]
#[error("{kind}: {message}")]
pub struct TagError {
    /// The specific type of error that occurred
    pub kind: TagErrorKind,
    /// Detailed error message for debugging
    pub message: String,
    /// Optional context information (tag ID, game state, etc.)
    pub context: Option<TagErrorContext>,
}

impl TagError {
    /// Creates a new TagError with the specified kind and message.
    ///
    /// # Arguments
    /// - `kind`: The specific error type
    /// - `message`: Detailed error description
    ///
    /// # Example
    /// ```rust,ignore
    /// let error = TagError::new(
    ///     TagErrorKind::TagNotFound,
    ///     "Tag with ID 'InvalidTag' not found in registry"
    /// );
    /// ```
    pub fn new(kind: TagErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
            context: None,
        }
    }

    /// Creates a new TagError with additional context information.
    ///
    /// # Arguments
    /// - `kind`: The specific error type
    /// - `message`: Detailed error description
    /// - `context`: Additional context information
    ///
    /// # Example
    /// ```rust,ignore
    /// let error = TagError::with_context(
    ///     TagErrorKind::ApplicationFailed,
    ///     "Economy tag failed to apply effect",
    ///     TagErrorContext::TagId(TagId::Economy)
    /// );
    /// ```
    pub fn with_context(
        kind: TagErrorKind,
        message: impl Into<String>,
        context: TagErrorContext,
    ) -> Self {
        Self {
            kind,
            message: message.into(),
            context: Some(context),
        }
    }

    /// Convenience constructor for tag not found errors.
    pub fn tag_not_found(tag_id: TagId) -> Self {
        Self::with_context(
            TagErrorKind::TagNotFound,
            format!("Tag with ID '{tag_id:?}' not found in registry"),
            TagErrorContext::TagId(tag_id),
        )
    }

    /// Convenience constructor for invalid game state errors.
    pub fn invalid_game_state(message: impl Into<String>) -> Self {
        Self::new(TagErrorKind::InvalidGameState, message)
    }

    /// Convenience constructor for application failure errors.
    pub fn application_failed(tag_id: TagId, reason: impl Into<String>) -> Self {
        Self::with_context(
            TagErrorKind::ApplicationFailed,
            format!("Tag '{:?}' application failed: {}", tag_id, reason.into()),
            TagErrorContext::TagId(tag_id),
        )
    }

    /// Convenience constructor for insufficient resources errors.
    pub fn insufficient_resources(resource: impl Into<String>) -> Self {
        Self::new(
            TagErrorKind::InsufficientResources,
            format!("Insufficient resources: {}", resource.into()),
        )
    }

    /// Returns true if this error indicates a recoverable failure.
    ///
    /// Recoverable errors are those that might succeed if retried later
    /// with different game state or conditions.
    pub fn is_recoverable(&self) -> bool {
        matches!(
            self.kind,
            TagErrorKind::InvalidGameState
                | TagErrorKind::InsufficientResources
                | TagErrorKind::ConditionNotMet
        )
    }

    /// Returns true if this error indicates a permanent failure.
    ///
    /// Permanent errors are those that indicate bugs, system failures,
    /// or configuration issues that won't resolve without intervention.
    pub fn is_permanent(&self) -> bool {
        !self.is_recoverable()
    }
}

/// Specific error types for different failure modes in the skip tag system.
///
/// Each variant represents a distinct category of error with specific
/// handling requirements and user-facing behavior.
#[derive(Error, Debug, Clone, Copy, PartialEq, Eq)]
pub enum TagErrorKind {
    // Registry and Lookup Errors
    /// Requested tag ID was not found in the registry
    #[error("Tag not found")]
    TagNotFound,

    /// Tag factory failed to create instance
    #[error("Tag creation failed")]
    TagCreationFailed,

    /// Registry is not properly initialized
    #[error("Registry not initialized")]
    RegistryNotInitialized,

    /// Registry is locked and cannot be accessed
    #[error("Registry access denied")]
    RegistryLocked,

    // Application and Effect Errors
    /// Tag effect application failed
    #[error("Tag application failed")]
    ApplicationFailed,

    /// Game state is invalid for tag application
    #[error("Invalid game state")]
    InvalidGameState,

    /// Required resources are not available
    #[error("Insufficient resources")]
    InsufficientResources,

    /// Tag-specific conditions are not met
    #[error("Condition not met")]
    ConditionNotMet,

    /// Tag effect would exceed system limits
    #[error("Effect exceeds limits")]
    EffectLimitsExceeded,

    // Validation and Input Errors
    /// Invalid input parameters provided
    #[error("Invalid input")]
    InvalidInput,

    /// Tag configuration is invalid
    #[error("Invalid configuration")]
    InvalidConfiguration,

    /// Operation is not supported by this tag type
    #[error("Operation not supported")]
    UnsupportedOperation,

    // System and Threading Errors
    /// Internal system error occurred
    #[error("System error")]
    SystemError,

    /// Thread synchronization error
    #[error("Thread synchronization failed")]
    ThreadSyncError,

    /// Performance limit exceeded (e.g., timeout)
    #[error("Performance limit exceeded")]
    PerformanceLimitExceeded,

    // Integration Errors
    /// Error integrating with shop system
    #[error("Shop integration failed")]
    ShopIntegrationFailed,

    /// Error integrating with pack system
    #[error("Pack integration failed")]
    PackIntegrationFailed,

    /// Error integrating with joker system
    #[error("Joker integration failed")]
    JokerIntegrationFailed,

    /// Error integrating with blind system
    #[error("Blind integration failed")]
    BlindIntegrationFailed,
}

/// Additional context information for tag errors.
///
/// Provides structured context that can be used for debugging,
/// logging, and error analysis without exposing sensitive information.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TagErrorContext {
    /// Error is related to a specific tag
    TagId(TagId),
    /// Error occurred during a specific operation
    Operation(String),
    /// Error is related to game state
    GameState(String),
    /// Error is related to system resources
    Resource(String),
    /// Multiple context items
    Multiple(Vec<TagErrorContext>),
}

impl TagErrorContext {
    /// Creates a new operation context.
    pub fn operation(op: impl Into<String>) -> Self {
        Self::Operation(op.into())
    }

    /// Creates a new game state context.
    pub fn game_state(state: impl Into<String>) -> Self {
        Self::GameState(state.into())
    }

    /// Creates a new resource context.
    pub fn resource(resource: impl Into<String>) -> Self {
        Self::Resource(resource.into())
    }

    /// Combines multiple context items.
    pub fn multiple(contexts: Vec<TagErrorContext>) -> Self {
        Self::Multiple(contexts)
    }
}

/// Convert TagError to the main error system for user-facing APIs.
///
/// This conversion ensures that detailed tag errors are properly sanitized
/// through the existing error handling system before being exposed to users.
impl From<TagError> for DeveloperGameError {
    fn from(error: TagError) -> Self {
        match error.kind {
            // Map registry errors to existing error types
            TagErrorKind::TagNotFound => DeveloperGameError::NoJokerMatch, // Closest equivalent
            TagErrorKind::TagCreationFailed => {
                DeveloperGameError::InvalidOperation("Tag creation failed".to_string())
            }
            TagErrorKind::RegistryNotInitialized | TagErrorKind::RegistryLocked => {
                DeveloperGameError::InvalidActionSpace
            }

            // Map application errors
            TagErrorKind::ApplicationFailed => DeveloperGameError::InvalidOperation(error.message),
            TagErrorKind::InvalidGameState => DeveloperGameError::InvalidStage,
            TagErrorKind::InsufficientResources => DeveloperGameError::InvalidBalance,
            TagErrorKind::ConditionNotMet => DeveloperGameError::InvalidAction,
            TagErrorKind::EffectLimitsExceeded => {
                DeveloperGameError::InvalidOperation("Effect limits exceeded".to_string())
            }

            // Map validation errors
            TagErrorKind::InvalidInput => DeveloperGameError::InvalidInput(error.message),
            TagErrorKind::InvalidConfiguration => {
                DeveloperGameError::InvalidOperation("Invalid configuration".to_string())
            }
            TagErrorKind::UnsupportedOperation => DeveloperGameError::InvalidAction,

            // Map system errors
            TagErrorKind::SystemError => {
                DeveloperGameError::InvalidOperation("System error".to_string())
            }
            TagErrorKind::ThreadSyncError => DeveloperGameError::MutexPoisoned,
            TagErrorKind::PerformanceLimitExceeded => {
                DeveloperGameError::InvalidOperation("Performance limit exceeded".to_string())
            }

            // Map integration errors
            TagErrorKind::ShopIntegrationFailed
            | TagErrorKind::PackIntegrationFailed
            | TagErrorKind::JokerIntegrationFailed
            | TagErrorKind::BlindIntegrationFailed => {
                DeveloperGameError::InvalidOperation(format!("Integration error: {}", error.kind))
            }
        }
    }
}

/// Result type alias for skip tag operations.
///
/// Provides a convenient way to work with tag operations that may fail.
/// All tag system functions should use this result type for consistency.
pub type TagResult<T> = Result<T, TagError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tag_error_creation() {
        let error = TagError::new(TagErrorKind::TagNotFound, "Test error message");
        assert_eq!(error.kind, TagErrorKind::TagNotFound);
        assert_eq!(error.message, "Test error message");
        assert!(error.context.is_none());
    }

    #[test]
    fn test_tag_error_with_context() {
        let error = TagError::with_context(
            TagErrorKind::ApplicationFailed,
            "Test error with context",
            TagErrorContext::TagId(TagId::Charm),
        );
        assert_eq!(error.kind, TagErrorKind::ApplicationFailed);
        assert_eq!(error.message, "Test error with context");
        assert_eq!(error.context, Some(TagErrorContext::TagId(TagId::Charm)));
    }

    #[test]
    fn test_convenience_constructors() {
        // Test tag_not_found
        let error = TagError::tag_not_found(TagId::Economy);
        assert_eq!(error.kind, TagErrorKind::TagNotFound);
        assert!(error.message.contains("Economy"));
        assert_eq!(error.context, Some(TagErrorContext::TagId(TagId::Economy)));

        // Test invalid_game_state
        let error = TagError::invalid_game_state("Invalid state");
        assert_eq!(error.kind, TagErrorKind::InvalidGameState);
        assert_eq!(error.message, "Invalid state");

        // Test application_failed
        let error = TagError::application_failed(TagId::Charm, "Failed to apply");
        assert_eq!(error.kind, TagErrorKind::ApplicationFailed);
        assert!(error.message.contains("Charm"));
        assert!(error.message.contains("Failed to apply"));

        // Test insufficient_resources
        let error = TagError::insufficient_resources("money");
        assert_eq!(error.kind, TagErrorKind::InsufficientResources);
        assert!(error.message.contains("money"));
    }

    #[test]
    fn test_error_recoverability() {
        // Recoverable errors
        let recoverable_kinds = [
            TagErrorKind::InvalidGameState,
            TagErrorKind::InsufficientResources,
            TagErrorKind::ConditionNotMet,
        ];

        for kind in recoverable_kinds.iter() {
            let error = TagError::new(*kind, "test");
            assert!(error.is_recoverable());
            assert!(!error.is_permanent());
        }

        // Permanent errors
        let permanent_kinds = [
            TagErrorKind::TagNotFound,
            TagErrorKind::TagCreationFailed,
            TagErrorKind::SystemError,
        ];

        for kind in permanent_kinds.iter() {
            let error = TagError::new(*kind, "test");
            assert!(!error.is_recoverable());
            assert!(error.is_permanent());
        }
    }

    #[test]
    fn test_error_context_creation() {
        let ctx = TagErrorContext::operation("test_operation");
        assert_eq!(
            ctx,
            TagErrorContext::Operation("test_operation".to_string())
        );

        let ctx = TagErrorContext::game_state("invalid_stage");
        assert_eq!(ctx, TagErrorContext::GameState("invalid_stage".to_string()));

        let ctx = TagErrorContext::resource("money");
        assert_eq!(ctx, TagErrorContext::Resource("money".to_string()));

        let multiple_ctx = TagErrorContext::multiple(vec![
            TagErrorContext::TagId(TagId::Charm),
            TagErrorContext::Operation("apply".to_string()),
        ]);
        match multiple_ctx {
            TagErrorContext::Multiple(contexts) => {
                assert_eq!(contexts.len(), 2);
                assert_eq!(contexts[0], TagErrorContext::TagId(TagId::Charm));
                assert_eq!(contexts[1], TagErrorContext::Operation("apply".to_string()));
            }
            _ => panic!("Expected Multiple context"),
        }
    }

    #[test]
    fn test_conversion_to_developer_game_error() {
        // Test various error kind conversions
        let test_cases = [
            (TagErrorKind::TagNotFound, "NoJokerMatch equivalent"),
            (TagErrorKind::InvalidGameState, "InvalidStage equivalent"),
            (
                TagErrorKind::InsufficientResources,
                "InvalidBalance equivalent",
            ),
            (TagErrorKind::InvalidInput, "InvalidInput equivalent"),
            (TagErrorKind::SystemError, "System error conversion"),
        ];

        for (kind, description) in test_cases.iter() {
            let tag_error = TagError::new(*kind, format!("Test error: {description}"));
            let game_error: DeveloperGameError = tag_error.into();

            // Verify conversion doesn't panic and produces valid error
            assert!(!game_error.to_string().is_empty());
        }
    }

    #[test]
    fn test_error_display() {
        let error = TagError::new(TagErrorKind::TagNotFound, "Test message");
        let display = format!("{error}");
        assert!(display.contains("Tag not found"));
        assert!(display.contains("Test message"));
    }

    #[test]
    fn test_error_debug() {
        let error = TagError::with_context(
            TagErrorKind::ApplicationFailed,
            "Debug test",
            TagErrorContext::TagId(TagId::Economy),
        );
        let debug = format!("{error:?}");
        assert!(debug.contains("ApplicationFailed"));
        assert!(debug.contains("Debug test"));
        assert!(debug.contains("Economy"));
    }

    #[test]
    fn test_all_error_kinds_have_display() {
        // Ensure all error kinds can be displayed
        let kinds = [
            TagErrorKind::TagNotFound,
            TagErrorKind::TagCreationFailed,
            TagErrorKind::RegistryNotInitialized,
            TagErrorKind::RegistryLocked,
            TagErrorKind::ApplicationFailed,
            TagErrorKind::InvalidGameState,
            TagErrorKind::InsufficientResources,
            TagErrorKind::ConditionNotMet,
            TagErrorKind::EffectLimitsExceeded,
            TagErrorKind::InvalidInput,
            TagErrorKind::InvalidConfiguration,
            TagErrorKind::UnsupportedOperation,
            TagErrorKind::SystemError,
            TagErrorKind::ThreadSyncError,
            TagErrorKind::PerformanceLimitExceeded,
            TagErrorKind::ShopIntegrationFailed,
            TagErrorKind::PackIntegrationFailed,
            TagErrorKind::JokerIntegrationFailed,
            TagErrorKind::BlindIntegrationFailed,
        ];

        for kind in kinds.iter() {
            let display = format!("{kind}");
            assert!(!display.is_empty(), "Error kind {kind:?} has empty display");
        }
    }

    #[test]
    fn test_tag_result_alias() {
        // Test that TagResult alias works correctly
        let success: TagResult<i32> = Ok(42);
        match success {
            Ok(val) => assert_eq!(val, 42),
            Err(_) => panic!("Expected Ok, got Err"),
        }

        let failure: TagResult<i32> = Err(TagError::new(TagErrorKind::SystemError, "test"));
        assert!(failure.is_err());
    }
}
