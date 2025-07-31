//! Dependency Injection Container and Trait Definitions
//!
//! This module implements a comprehensive dependency injection framework
//! following production patterns from Google-scale systems. All external
//! dependencies are abstracted through traits, enabling testing, monitoring,
//! and runtime configuration.
//!
//! ## Production Architecture
//!
//! The DI system follows these principles:
//! - Interface Segregation: Small, focused trait interfaces
//! - Dependency Inversion: High-level modules don't depend on low-level modules
//! - Single Responsibility: Each trait has one clear purpose
//! - Liskov Substitution: Implementations are safely interchangeable

use crate::application::{
    config::{ApplicationConfig, GameConfig, SessionId},
    errors::ApplicationError,
};
use crate::domain::{Action, Game};
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

// Re-export the domain ActionValidator trait instead of redefining it
pub use crate::domain::services::ActionValidator;

/// Game repository trait for persistence operations
///
/// Abstracts game state persistence, enabling different storage backends
/// (memory, Redis, PostgreSQL, etc.) without changing business logic.
#[async_trait]
pub trait GameRepository: Send + Sync {
    /// Save game state to persistent storage
    ///
    /// # Arguments
    /// * `session_id` - Unique session identifier
    /// * `game` - Game state to persist
    ///
    /// # Returns
    /// * `Ok(())` on successful save
    /// * `Err(ApplicationError)` on persistence failure
    async fn save_game(&self, session_id: &SessionId, game: &Game) -> Result<(), ApplicationError>;

    /// Load game state from persistent storage
    ///
    /// # Arguments
    /// * `session_id` - Session to load
    ///
    /// # Returns
    /// * `Ok(Game)` with loaded game state
    /// * `Err(ApplicationError::SessionNotFound)` if session doesn't exist
    async fn load_game(&self, session_id: &SessionId) -> Result<Game, ApplicationError>;

    /// Delete game state from storage
    ///
    /// Used for cleanup and session termination.
    async fn delete_game(&self, session_id: &SessionId) -> Result<(), ApplicationError>;

    /// List all active sessions
    ///
    /// Returns session IDs for management and monitoring purposes.
    async fn list_sessions(&self) -> Result<Vec<SessionId>, ApplicationError>;

    /// Get storage health metrics
    ///
    /// Provides operational visibility into storage performance.
    async fn health_check(&self) -> Result<StorageHealth, ApplicationError>;
}

/// Storage health information for monitoring
#[derive(Debug, Clone)]
pub struct StorageHealth {
    pub is_healthy: bool,
    pub latency_ms: u64,
    pub storage_size_mb: u64,
    pub active_connections: usize,
    pub error_rate: f64,
}

/// State change notification trait for reactive systems
///
/// Enables real-time updates, webhooks, and event-driven architectures.
/// Critical for user interfaces and external system integration.
#[async_trait]
pub trait StateNotifier: Send + Sync {
    /// Notify interested parties of state changes
    ///
    /// # Arguments
    /// * `session_id` - Session that changed
    /// * `event` - Type of state change that occurred
    async fn notify_state_change(
        &self,
        session_id: &SessionId,
        event: StateChangeEvent,
    ) -> Result<(), ApplicationError>;

    /// Register a callback for state change events
    ///
    /// Enables pub/sub patterns for reactive architectures.
    async fn register_callback(
        &self,
        callback: Arc<dyn StateChangeCallback>,
    ) -> Result<(), ApplicationError>;

    /// Get notification system health
    async fn health_check(&self) -> Result<NotificationHealth, ApplicationError>;
}

/// Types of state change events for notification
#[derive(Debug, Clone)]
pub enum StateChangeEvent {
    SessionCreated {
        config: GameConfig,
    },
    ActionExecuted {
        action: Action,
        result: ActionResult,
    },
    SessionEnded {
        reason: String,
    },
    ErrorOccurred {
        error: String,
    },
}

/// Result of action execution for notifications
#[derive(Debug, Clone)]
pub struct ActionResult {
    pub success: bool,
    pub score_change: i64,
    pub money_change: i32,
    pub execution_time_us: u64,
}

/// Callback trait for state change notifications
#[async_trait]
pub trait StateChangeCallback: Send + Sync {
    async fn on_state_change(
        &self,
        session_id: &SessionId,
        event: StateChangeEvent,
    ) -> Result<(), ApplicationError>;
}

/// Notification system health information
#[derive(Debug, Clone)]
pub struct NotificationHealth {
    pub is_healthy: bool,
    pub active_subscriptions: usize,
    pub message_queue_depth: usize,
    pub error_rate: f64,
}

/// Metrics collection trait for observability
///
/// Provides comprehensive metrics for monitoring, alerting,
/// and performance optimization. Essential for production operations.
#[async_trait]
pub trait MetricsCollector: Send + Sync {
    /// Record a counter metric (monotonically increasing)
    async fn increment_counter(&self, name: &str, value: u64, tags: &[(&str, &str)]);

    /// Record a gauge metric (current value)
    async fn record_gauge(&self, name: &str, value: f64, tags: &[(&str, &str)]);

    /// Record a histogram metric (distribution of values)
    async fn record_histogram(&self, name: &str, value: f64, tags: &[(&str, &str)]);

    /// Record a timing metric (latency measurement)
    async fn record_timing(&self, name: &str, duration: Duration, tags: &[(&str, &str)]);

    /// Start a timer for measuring operation duration
    fn start_timer(&self, name: &str, tags: &[(&str, &str)]) -> Box<dyn Timer>;

    /// Get current metrics summary
    async fn get_metrics_summary(&self) -> Result<MetricsSummary, ApplicationError>;

    /// Health check for metrics system
    async fn health_check(&self) -> Result<MetricsHealth, ApplicationError>;
}

/// Timer trait for measuring operation duration
pub trait Timer: Send + Sync {
    /// Stop the timer and record the measurement
    fn stop(self: Box<Self>);
}

/// Summary of collected metrics
#[derive(Debug, Clone)]
pub struct MetricsSummary {
    pub counters: HashMap<String, u64>,
    pub gauges: HashMap<String, f64>,
    pub histograms: HashMap<String, HistogramStats>,
}

/// Histogram statistics
#[derive(Debug, Clone)]
pub struct HistogramStats {
    pub count: u64,
    pub sum: f64,
    pub min: f64,
    pub max: f64,
    pub mean: f64,
    pub p50: f64,
    pub p95: f64,
    pub p99: f64,
}

/// Metrics system health information
#[derive(Debug, Clone)]
pub struct MetricsHealth {
    pub is_healthy: bool,
    pub metrics_per_second: f64,
    pub buffer_usage_percent: u8,
    pub export_error_rate: f64,
}

/// Service container for dependency injection
///
/// Central registry for all application dependencies. Provides
/// type-safe dependency resolution and lifecycle management.
/// Designed for production with proper error handling and monitoring.
pub struct ServiceContainer {
    validator: Arc<dyn ActionValidator>,
    repository: Arc<dyn GameRepository>,
    notifier: Arc<dyn StateNotifier>,
    metrics: Arc<dyn MetricsCollector>,
    config: ApplicationConfig,
}

impl ServiceContainer {
    /// Create a new service container with all dependencies
    ///
    /// # Arguments
    /// * `validator` - Action validation implementation
    /// * `repository` - Game persistence implementation
    /// * `notifier` - State change notification implementation
    /// * `metrics` - Metrics collection implementation
    /// * `config` - Application configuration
    pub fn new(
        validator: Arc<dyn ActionValidator>,
        repository: Arc<dyn GameRepository>,
        notifier: Arc<dyn StateNotifier>,
        metrics: Arc<dyn MetricsCollector>,
        config: ApplicationConfig,
    ) -> Self {
        Self {
            validator,
            repository,
            notifier,
            metrics,
            config,
        }
    }

    /// Get action validator dependency
    pub fn validator(&self) -> Arc<dyn ActionValidator> {
        self.validator.clone()
    }

    /// Get game repository dependency
    pub fn repository(&self) -> Arc<dyn GameRepository> {
        self.repository.clone()
    }

    /// Get state notifier dependency
    pub fn notifier(&self) -> Arc<dyn StateNotifier> {
        self.notifier.clone()
    }

    /// Get metrics collector dependency
    pub fn metrics(&self) -> Arc<dyn MetricsCollector> {
        self.metrics.clone()
    }

    /// Get application configuration
    pub fn config(&self) -> &ApplicationConfig {
        &self.config
    }

    /// Perform health check on all dependencies
    ///
    /// Returns comprehensive health status for operational monitoring.
    pub async fn health_check(&self) -> ContainerHealth {
        let start_time = Instant::now();

        let storage_health =
            self.repository
                .health_check()
                .await
                .unwrap_or_else(|_| StorageHealth {
                    is_healthy: false,
                    latency_ms: 0,
                    storage_size_mb: 0,
                    active_connections: 0,
                    error_rate: 1.0,
                });

        let notification_health =
            self.notifier
                .health_check()
                .await
                .unwrap_or_else(|_| NotificationHealth {
                    is_healthy: false,
                    active_subscriptions: 0,
                    message_queue_depth: 0,
                    error_rate: 1.0,
                });

        let metrics_health = self
            .metrics
            .health_check()
            .await
            .unwrap_or_else(|_| MetricsHealth {
                is_healthy: false,
                metrics_per_second: 0.0,
                buffer_usage_percent: 100,
                export_error_rate: 1.0,
            });

        let health_check_duration = start_time.elapsed();

        ContainerHealth {
            is_healthy: storage_health.is_healthy
                && notification_health.is_healthy
                && metrics_health.is_healthy,
            health_check_duration_ms: health_check_duration.as_millis() as u64,
            storage: storage_health,
            notifications: notification_health,
            metrics: metrics_health,
        }
    }
}

/// Container health status for monitoring
#[derive(Debug, Clone)]
pub struct ContainerHealth {
    pub is_healthy: bool,
    pub health_check_duration_ms: u64,
    pub storage: StorageHealth,
    pub notifications: NotificationHealth,
    pub metrics: MetricsHealth,
}

/// Builder pattern for creating service container
///
/// Provides fluent API for dependency injection configuration
/// with validation and sensible defaults.
pub struct ServiceContainerBuilder {
    validator: Option<Arc<dyn ActionValidator>>,
    repository: Option<Arc<dyn GameRepository>>,
    notifier: Option<Arc<dyn StateNotifier>>,
    metrics: Option<Arc<dyn MetricsCollector>>,
    config: Option<ApplicationConfig>,
}

impl ServiceContainerBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self {
            validator: None,
            repository: None,
            notifier: None,
            metrics: None,
            config: None,
        }
    }

    /// Set action validator
    pub fn with_validator(mut self, validator: Arc<dyn ActionValidator>) -> Self {
        self.validator = Some(validator);
        self
    }

    /// Set game repository
    pub fn with_repository(mut self, repository: Arc<dyn GameRepository>) -> Self {
        self.repository = Some(repository);
        self
    }

    /// Set state notifier
    pub fn with_notifier(mut self, notifier: Arc<dyn StateNotifier>) -> Self {
        self.notifier = Some(notifier);
        self
    }

    /// Set metrics collector
    pub fn with_metrics(mut self, metrics: Arc<dyn MetricsCollector>) -> Self {
        self.metrics = Some(metrics);
        self
    }

    /// Set application configuration
    pub fn with_config(mut self, config: ApplicationConfig) -> Self {
        self.config = Some(config);
        self
    }

    /// Build the service container with validation
    ///
    /// # Returns
    /// * `Ok(ServiceContainer)` if all dependencies are provided
    /// * `Err(ApplicationError)` if any required dependency is missing
    pub fn build(self) -> Result<ServiceContainer, ApplicationError> {
        let validator = self
            .validator
            .ok_or_else(|| ApplicationError::Configuration {
                parameter: "validator".to_string(),
                message: "ActionValidator implementation required".to_string(),
            })?;

        let repository = self
            .repository
            .ok_or_else(|| ApplicationError::Configuration {
                parameter: "repository".to_string(),
                message: "GameRepository implementation required".to_string(),
            })?;

        let notifier = self
            .notifier
            .ok_or_else(|| ApplicationError::Configuration {
                parameter: "notifier".to_string(),
                message: "StateNotifier implementation required".to_string(),
            })?;

        let metrics = self
            .metrics
            .ok_or_else(|| ApplicationError::Configuration {
                parameter: "metrics".to_string(),
                message: "MetricsCollector implementation required".to_string(),
            })?;

        let config = self.config.unwrap_or_default();

        Ok(ServiceContainer::new(
            validator, repository, notifier, metrics, config,
        ))
    }
}

impl Default for ServiceContainerBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::SystemTime;

    // Mock implementations for testing

    struct MockActionValidator;

    #[async_trait]
    impl ActionValidator for MockActionValidator {
        async fn validate_action(
            &self,
            _action: &Action,
            _game: &Game,
        ) -> Result<(), ApplicationError> {
            Ok(())
        }

        fn get_validation_rules(&self) -> Vec<String> {
            vec!["mock_rule".to_string()]
        }
    }

    struct MockGameRepository;

    #[async_trait]
    impl GameRepository for MockGameRepository {
        async fn save_game(
            &self,
            _session_id: &SessionId,
            _game: &Game,
        ) -> Result<(), ApplicationError> {
            Ok(())
        }

        async fn load_game(&self, _session_id: &SessionId) -> Result<Game, ApplicationError> {
            Ok(Game::default())
        }

        async fn delete_game(&self, _session_id: &SessionId) -> Result<(), ApplicationError> {
            Ok(())
        }

        async fn list_sessions(&self) -> Result<Vec<SessionId>, ApplicationError> {
            Ok(vec![])
        }

        async fn health_check(&self) -> Result<StorageHealth, ApplicationError> {
            Ok(StorageHealth {
                is_healthy: true,
                latency_ms: 1,
                storage_size_mb: 100,
                active_connections: 5,
                error_rate: 0.0,
            })
        }
    }

    struct MockStateNotifier;

    #[async_trait]
    impl StateNotifier for MockStateNotifier {
        async fn notify_state_change(
            &self,
            _session_id: &SessionId,
            _event: StateChangeEvent,
        ) -> Result<(), ApplicationError> {
            Ok(())
        }

        async fn register_callback(
            &self,
            _callback: Arc<dyn StateChangeCallback>,
        ) -> Result<(), ApplicationError> {
            Ok(())
        }

        async fn health_check(&self) -> Result<NotificationHealth, ApplicationError> {
            Ok(NotificationHealth {
                is_healthy: true,
                active_subscriptions: 0,
                message_queue_depth: 0,
                error_rate: 0.0,
            })
        }
    }

    struct MockMetricsCollector;

    #[async_trait]
    impl MetricsCollector for MockMetricsCollector {
        async fn increment_counter(&self, _name: &str, _value: u64, _tags: &[(&str, &str)]) {}

        async fn record_gauge(&self, _name: &str, _value: f64, _tags: &[(&str, &str)]) {}

        async fn record_histogram(&self, _name: &str, _value: f64, _tags: &[(&str, &str)]) {}

        async fn record_timing(&self, _name: &str, _duration: Duration, _tags: &[(&str, &str)]) {}

        fn start_timer(&self, _name: &str, _tags: &[(&str, &str)]) -> Box<dyn Timer> {
            Box::new(MockTimer)
        }

        async fn get_metrics_summary(&self) -> Result<MetricsSummary, ApplicationError> {
            Ok(MetricsSummary {
                counters: HashMap::new(),
                gauges: HashMap::new(),
                histograms: HashMap::new(),
            })
        }

        async fn health_check(&self) -> Result<MetricsHealth, ApplicationError> {
            Ok(MetricsHealth {
                is_healthy: true,
                metrics_per_second: 100.0,
                buffer_usage_percent: 10,
                export_error_rate: 0.0,
            })
        }
    }

    struct MockTimer;

    impl Timer for MockTimer {
        fn stop(self: Box<Self>) {}
    }

    #[tokio::test]
    async fn test_service_container_builder() {
        let container = ServiceContainerBuilder::new()
            .with_validator(Arc::new(MockActionValidator))
            .with_repository(Arc::new(MockGameRepository))
            .with_notifier(Arc::new(MockStateNotifier))
            .with_metrics(Arc::new(MockMetricsCollector))
            .build()
            .unwrap();

        // Test dependency resolution
        let _validator = container.validator();
        let _repository = container.repository();
        let _notifier = container.notifier();
        let _metrics = container.metrics();
        let _config = container.config();
    }

    #[tokio::test]
    async fn test_service_container_health_check() {
        let container = ServiceContainerBuilder::new()
            .with_validator(Arc::new(MockActionValidator))
            .with_repository(Arc::new(MockGameRepository))
            .with_notifier(Arc::new(MockStateNotifier))
            .with_metrics(Arc::new(MockMetricsCollector))
            .build()
            .unwrap();

        let health = container.health_check().await;
        assert!(health.is_healthy);
        assert!(health.storage.is_healthy);
        assert!(health.notifications.is_healthy);
        assert!(health.metrics.is_healthy);
    }

    #[test]
    fn test_builder_validation() {
        let result = ServiceContainerBuilder::new().build();
        assert!(result.is_err());

        if let Err(ApplicationError::Configuration { parameter, .. }) = result {
            assert_eq!(parameter, "validator");
        } else {
            panic!("Expected configuration error");
        }
    }
}
