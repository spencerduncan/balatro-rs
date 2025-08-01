//! Application Layer Error Handling and Recovery
//!
//! This module provides comprehensive error handling for the application layer,
//! including domain error propagation, infrastructure error wrapping, and
//! automated recovery strategies for production resilience.
//!
//! ## Production Error Design
//!
//! Following Google SRE principles:
//! - Every error must be actionable
//! - Error messages must aid debugging at 3 AM
//! - Recovery strategies must be automated where possible
//! - Error correlation must be preserved across service boundaries

use std::time::Duration;
use thiserror::Error;

/// Comprehensive application layer error types
///
/// Each error type includes context needed for:
/// - Automated recovery decisions
/// - Observability and debugging
/// - User-friendly error messages
/// - Operational alerting
#[derive(Debug, Error)]
pub enum ApplicationError {
    /// Domain layer errors - business rule violations
    #[error("Domain error: {message}")]
    Domain {
        message: String,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    /// Session not found - common in distributed systems
    #[error("Session not found: {session_id} (may have expired after {ttl:?})")]
    SessionNotFound {
        session_id: String,
        ttl: Option<Duration>,
    },

    /// Concurrent session limit exceeded - backpressure protection
    #[error("Concurrent session limit exceeded: {current}/{limit} (consider horizontal scaling)")]
    SessionLimitExceeded { current: usize, limit: usize },

    /// Session already exists - prevents duplicate creation
    #[error("Session already exists: {session_id} (use existing or delete first)")]
    SessionAlreadyExists { session_id: String },

    /// Infrastructure layer errors - external system failures
    #[error("Infrastructure error: {component} - {message}")]
    Infrastructure {
        component: String,
        message: String,
        retryable: bool,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// Validation errors - input sanitization failures
    #[error("Validation error: {field} - {message}")]
    Validation {
        field: String,
        message: String,
        value: Option<String>,
    },

    /// Configuration errors - deployment/startup issues
    #[error("Configuration error: {parameter} - {message}")]
    Configuration { parameter: String, message: String },

    /// Timeout errors - SLA violations
    #[error("Operation timeout: {operation} exceeded {timeout:?} (consider increasing limits)")]
    Timeout {
        operation: String,
        timeout: Duration,
    },

    /// Resource exhaustion - capacity planning indicators
    #[error("Resource exhausted: {resource} at {utilization}% (scale up required)")]
    ResourceExhausted { resource: String, utilization: u8 },

    /// Concurrent access conflicts - consistency violations
    #[error("Concurrent access conflict: {resource} - {message}")]
    ConcurrentAccess { resource: String, message: String },

    /// Service unavailable - dependency failures
    #[error("Service unavailable: {service} - {reason}")]
    ServiceUnavailable {
        service: String,
        reason: String,
        retry_after: Option<Duration>,
    },
}

impl ApplicationError {
    /// Create a domain error from any domain layer error
    pub fn domain<E>(error: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        Self::Domain {
            message: error.to_string(),
            source: Box::new(error),
        }
    }

    /// Create an infrastructure error with retry information
    pub fn infrastructure<E>(component: &str, retryable: bool, error: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        Self::Infrastructure {
            component: component.to_string(),
            message: error.to_string(),
            retryable,
            source: Some(Box::new(error)),
        }
    }

    /// Create a validation error for user input
    pub fn validation(field: &str, message: &str, value: Option<&str>) -> Self {
        Self::Validation {
            field: field.to_string(),
            message: message.to_string(),
            value: value.map(String::from),
        }
    }

    /// Check if this error indicates a transient failure
    pub fn is_retryable(&self) -> bool {
        match self {
            Self::Infrastructure { retryable, .. } => *retryable,
            Self::Timeout { .. } => true,
            Self::ServiceUnavailable { .. } => true,
            Self::ConcurrentAccess { .. } => true,
            _ => false,
        }
    }

    /// Check if this error should trigger an alert
    pub fn is_alertable(&self) -> bool {
        matches!(self, Self::SessionLimitExceeded { .. } | Self::ResourceExhausted { .. } | Self::ServiceUnavailable { .. } | Self::Configuration { .. })
    }

    /// Get the recommended retry delay for retryable errors
    pub fn retry_delay(&self) -> Option<Duration> {
        match self {
            Self::ServiceUnavailable { retry_after, .. } => *retry_after,
            Self::Timeout { .. } => Some(Duration::from_millis(100)),
            Self::ConcurrentAccess { .. } => Some(Duration::from_millis(50)),
            _ => None,
        }
    }

    /// Get error category for metrics and alerting
    pub fn category(&self) -> &'static str {
        match self {
            Self::Domain { .. } => "domain",
            Self::SessionNotFound { .. } => "session",
            Self::SessionLimitExceeded { .. } => "capacity",
            Self::SessionAlreadyExists { .. } => "session",
            Self::Infrastructure { .. } => "infrastructure",
            Self::Validation { .. } => "validation",
            Self::Configuration { .. } => "configuration",
            Self::Timeout { .. } => "performance",
            Self::ResourceExhausted { .. } => "capacity",
            Self::ConcurrentAccess { .. } => "concurrency",
            Self::ServiceUnavailable { .. } => "dependency",
        }
    }
}

/// Error recovery strategy trait
///
/// Implements automated recovery patterns for production resilience.
/// Each strategy encodes Google SRE best practices for handling
/// specific failure modes at scale.
pub trait ErrorRecoveryStrategy: Send + Sync + std::fmt::Debug {
    /// Determine if this strategy can recover from the given error
    fn can_recover(&self, error: &ApplicationError) -> bool;

    /// Attempt to recover from the error
    fn recover(&self, error: &ApplicationError) -> Result<(), ApplicationError>;

    /// Get the maximum number of recovery attempts
    fn max_attempts(&self) -> usize {
        3
    }

    /// Get the delay between recovery attempts
    fn recovery_delay(&self) -> Duration {
        Duration::from_millis(100)
    }
}

/// Exponential backoff recovery strategy
///
/// Implements exponential backoff with jitter for retryable errors.
/// This prevents thundering herd problems in distributed systems.
#[derive(Debug)]
#[allow(dead_code)]
pub struct ExponentialBackoffRecovery {
    max_attempts: usize,
    initial_delay: Duration,
    max_delay: Duration,
    jitter_factor: f64,
}

impl ExponentialBackoffRecovery {
    /// Create a new exponential backoff recovery strategy
    ///
    /// # Arguments
    /// * `max_attempts` - Maximum number of retry attempts
    /// * `initial_delay` - Initial delay before first retry
    /// * `max_delay` - Maximum delay between retries
    /// * `jitter_factor` - Jitter factor to prevent thundering herd (0.0-1.0)
    pub fn new(
        max_attempts: usize,
        initial_delay: Duration,
        max_delay: Duration,
        jitter_factor: f64,
    ) -> Self {
        Self {
            max_attempts,
            initial_delay,
            max_delay,
            jitter_factor: jitter_factor.clamp(0.0, 1.0),
        }
    }

    /// Create a default production-ready exponential backoff strategy
    pub fn production_default() -> Self {
        Self::new(
            3,                          // 3 attempts max
            Duration::from_millis(100), // 100ms initial delay
            Duration::from_secs(30),    // 30s max delay
            0.1,                        // 10% jitter
        )
    }
}

impl ErrorRecoveryStrategy for ExponentialBackoffRecovery {
    fn can_recover(&self, error: &ApplicationError) -> bool {
        error.is_retryable()
    }

    fn recover(&self, _error: &ApplicationError) -> Result<(), ApplicationError> {
        // In a real implementation, this would contain the retry logic
        // For now, we just indicate that recovery should be attempted
        Ok(())
    }

    fn max_attempts(&self) -> usize {
        self.max_attempts
    }

    fn recovery_delay(&self) -> Duration {
        self.initial_delay
    }
}

/// Circuit breaker recovery strategy
///
/// Implements circuit breaker pattern for dependency failures.
/// Prevents cascading failures by failing fast when dependencies are unhealthy.
#[allow(dead_code)]
#[derive(Debug)]
pub struct CircuitBreakerRecovery {
    failure_threshold: f64,
    recovery_timeout: Duration,
    min_requests: usize,
}

impl CircuitBreakerRecovery {
    /// Create a new circuit breaker recovery strategy
    pub fn new(failure_threshold: f64, recovery_timeout: Duration, min_requests: usize) -> Self {
        Self {
            failure_threshold: failure_threshold.clamp(0.0, 1.0),
            recovery_timeout,
            min_requests,
        }
    }

    /// Create a default production-ready circuit breaker strategy
    pub fn production_default() -> Self {
        Self::new(
            0.5,                     // 50% failure threshold
            Duration::from_secs(30), // 30s recovery timeout
            10,                      // 10 min requests before opening
        )
    }
}

impl ErrorRecoveryStrategy for CircuitBreakerRecovery {
    fn can_recover(&self, error: &ApplicationError) -> bool {
        matches!(error, ApplicationError::ServiceUnavailable { .. })
    }

    fn recover(&self, _error: &ApplicationError) -> Result<(), ApplicationError> {
        // In a real implementation, this would check circuit breaker state
        // and decide whether to allow the request through
        Ok(())
    }

    fn max_attempts(&self) -> usize {
        1 // Circuit breaker fails fast
    }

    fn recovery_delay(&self) -> Duration {
        Duration::from_millis(0) // No delay for circuit breaker
    }
}

/// Composite recovery strategy
///
/// Combines multiple recovery strategies, trying each in order
/// until one succeeds or all fail.
#[derive(Debug)]
pub struct CompositeRecoveryStrategy {
    strategies: Vec<Box<dyn ErrorRecoveryStrategy>>,
}

impl CompositeRecoveryStrategy {
    /// Create a new composite recovery strategy
    pub fn new() -> Self {
        Self {
            strategies: Vec::new(),
        }
    }

    /// Add a recovery strategy to the composite
    pub fn add_strategy(mut self, strategy: Box<dyn ErrorRecoveryStrategy>) -> Self {
        self.strategies.push(strategy);
        self
    }

    /// Create a production-ready composite strategy with common patterns
    pub fn production_default() -> Self {
        Self::new()
            .add_strategy(Box::new(CircuitBreakerRecovery::production_default()))
            .add_strategy(Box::new(ExponentialBackoffRecovery::production_default()))
    }
}

impl ErrorRecoveryStrategy for CompositeRecoveryStrategy {
    fn can_recover(&self, error: &ApplicationError) -> bool {
        self.strategies
            .iter()
            .any(|strategy| strategy.can_recover(error))
    }

    fn recover(&self, error: &ApplicationError) -> Result<(), ApplicationError> {
        for strategy in &self.strategies {
            if strategy.can_recover(error) {
                match strategy.recover(error) {
                    Ok(()) => return Ok(()),
                    Err(_) => continue, // Try next strategy
                }
            }
        }
        Err(ApplicationError::infrastructure(
            "recovery",
            false,
            std::io::Error::other("All recovery strategies failed"),
        ))
    }

    fn max_attempts(&self) -> usize {
        self.strategies
            .iter()
            .map(|s| s.max_attempts())
            .max()
            .unwrap_or(1)
    }

    fn recovery_delay(&self) -> Duration {
        self.strategies
            .iter()
            .map(|s| s.recovery_delay())
            .min()
            .unwrap_or(Duration::from_millis(100))
    }
}

impl Default for CompositeRecoveryStrategy {
    fn default() -> Self {
        Self::production_default()
    }
}

/// Conversions from domain layer errors
impl From<balatro_rs::error::DeveloperGameError> for ApplicationError {
    fn from(error: balatro_rs::error::DeveloperGameError) -> Self {
        Self::Domain {
            message: error.to_string(),
            source: Box::new(error),
        }
    }
}

/// Conversion from domain GameError (stub implementation)
impl From<crate::domain::stubs::GameError> for ApplicationError {
    fn from(error: crate::domain::stubs::GameError) -> Self {
        Self::Domain {
            message: error.to_string(),
            source: Box::new(error),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    #[test]
    fn test_application_error_creation() {
        let domain_error = ApplicationError::domain(io::Error::new(io::ErrorKind::Other, "test"));
        assert!(matches!(domain_error, ApplicationError::Domain { .. }));
        assert_eq!(domain_error.category(), "domain");

        let infra_error = ApplicationError::infrastructure(
            "test-service",
            true,
            io::Error::new(io::ErrorKind::Other, "test"),
        );
        assert!(matches!(
            infra_error,
            ApplicationError::Infrastructure { .. }
        ));
        assert!(infra_error.is_retryable());
        assert_eq!(infra_error.category(), "infrastructure");
    }

    #[test]
    fn test_error_retryability() {
        let timeout_error = ApplicationError::Timeout {
            operation: "test".to_string(),
            timeout: Duration::from_secs(1),
        };
        assert!(timeout_error.is_retryable());

        let validation_error = ApplicationError::validation("field", "message", None);
        assert!(!validation_error.is_retryable());
    }

    #[test]
    fn test_error_alertability() {
        let limit_error = ApplicationError::SessionLimitExceeded {
            current: 150,
            limit: 100,
        };
        assert!(limit_error.is_alertable());

        let session_error = ApplicationError::SessionNotFound {
            session_id: "test".to_string(),
            ttl: None,
        };
        assert!(!session_error.is_alertable());
    }

    #[test]
    fn test_exponential_backoff_recovery() {
        let recovery = ExponentialBackoffRecovery::production_default();

        let retryable_error = ApplicationError::Timeout {
            operation: "test".to_string(),
            timeout: Duration::from_secs(1),
        };
        assert!(recovery.can_recover(&retryable_error));

        let non_retryable_error = ApplicationError::validation("field", "message", None);
        assert!(!recovery.can_recover(&non_retryable_error));
    }

    #[test]
    fn test_circuit_breaker_recovery() {
        let recovery = CircuitBreakerRecovery::production_default();

        let service_error = ApplicationError::ServiceUnavailable {
            service: "test".to_string(),
            reason: "timeout".to_string(),
            retry_after: None,
        };
        assert!(recovery.can_recover(&service_error));

        let domain_error = ApplicationError::domain(io::Error::new(io::ErrorKind::Other, "test"));
        assert!(!recovery.can_recover(&domain_error));
    }

    #[test]
    fn test_composite_recovery_strategy() {
        let composite = CompositeRecoveryStrategy::production_default();

        let service_error = ApplicationError::ServiceUnavailable {
            service: "test".to_string(),
            reason: "timeout".to_string(),
            retry_after: None,
        };
        assert!(composite.can_recover(&service_error));

        let timeout_error = ApplicationError::Timeout {
            operation: "test".to_string(),
            timeout: Duration::from_secs(1),
        };
        assert!(composite.can_recover(&timeout_error));
    }
}
