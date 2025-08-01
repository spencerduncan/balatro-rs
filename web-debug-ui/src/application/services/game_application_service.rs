//! Game Application Service - Use Case Orchestration
//!
//! This service orchestrates game-related use cases, coordinating between
//! the domain layer (game logic) and infrastructure layer (persistence,
//! notifications, metrics). Designed for production scalability.

use crate::application::{
    config::{ApplicationConfig, GameConfig, SessionId},
    container::{
        ActionResult, ActionValidator, GameRepository, MetricsCollector, StateChangeEvent,
        StateNotifier,
    },
    errors::ApplicationError,
};
use crate::domain::{Action, Game};
use async_trait::async_trait;
use std::sync::Arc;
use std::time::Instant;

/// Game Application Service
///
/// Orchestrates game-related use cases with production-grade:
/// - Action validation and execution
/// - State change notifications
/// - Comprehensive metrics collection
/// - Error handling and recovery
pub struct GameApplicationService {
    validator: Arc<dyn ActionValidator>,
    repository: Arc<dyn GameRepository>,
    notifier: Arc<dyn StateNotifier>,
    metrics: Arc<dyn MetricsCollector>,
    config: ApplicationConfig,
}

impl GameApplicationService {
    /// Create a new game application service
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

    /// Execute an action in a game session
    ///
    /// # Arguments
    /// * `session_id` - Session to execute action in
    /// * `action` - Action to execute
    ///
    /// # Returns
    /// * `Ok(ActionResult)` - Action execution result
    /// * `Err(ApplicationError)` - Execution failure
    pub async fn execute_action(
        &self,
        session_id: &SessionId,
        action: Action,
    ) -> Result<ActionResult, ApplicationError> {
        let start_time = Instant::now();
        let _timer = self.metrics.start_timer(
            "action.execution",
            &[("action_type", &format!("{:?}", action))],
        );

        // Load current game state
        let mut game = self.repository.load_game(session_id).await?;

        // Validate action
        let validation_result = self.validator.validate(&action, &game);
        if !validation_result.is_valid() {
            return Err(ApplicationError::Domain {
                message: "Action validation failed".to_string(),
                source: Box::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Validation failed")),
            });
        }

        // Record pre-execution state
        let initial_score = game.score;
        let initial_money = game.money;

        // Execute action
        let execution_result = game.handle_action(action.clone());

        let success = execution_result.is_ok();
        let execution_time = start_time.elapsed();

        // Create action result
        let action_result = ActionResult {
            success,
            score_change: (game.score - initial_score) as i64,
            money_change: (game.money - initial_money) as i32,
            execution_time_us: execution_time.as_micros() as u64,
        };

        if success {
            // Save updated game state
            self.repository.save_game(session_id, &game).await?;

            // Notify state change
            let event = StateChangeEvent::ActionExecuted {
                action,
                result: action_result.clone(),
            };
            self.notifier.notify_state_change(session_id, event).await?;

            // Record success metrics
            self.metrics
                .increment_counter("action.executed", 1, &[("success", "true")])
                .await;
        } else {
            // Record failure metrics
            self.metrics
                .increment_counter("action.executed", 1, &[("success", "false")])
                .await;

            // Return the original execution error
            execution_result?;
        }

        // Record timing metrics
        self.metrics
            .record_timing("action.execution_time", execution_time, &[])
            .await;

        Ok(action_result)
    }

    /// Get current game state for a session
    ///
    /// # Arguments
    /// * `session_id` - Session to get state for
    ///
    /// # Returns
    /// * `Ok(Game)` - Current game state
    /// * `Err(ApplicationError)` - State retrieval failure
    pub async fn get_game_state(&self, session_id: &SessionId) -> Result<Game, ApplicationError> {
        let _timer = self.metrics.start_timer("state.retrieval", &[]);

        let game = self.repository.load_game(session_id).await?;

        self.metrics
            .increment_counter("state.retrieved", 1, &[])
            .await;
        Ok(game)
    }

    /// Get available actions for a session
    ///
    /// # Arguments
    /// * `session_id` - Session to get actions for
    ///
    /// # Returns
    /// * `Ok(Vec<Action>)` - Available actions
    /// * `Err(ApplicationError)` - Action generation failure
    pub async fn get_available_actions(
        &self,
        session_id: &SessionId,
    ) -> Result<Vec<Action>, ApplicationError> {
        let _timer = self.metrics.start_timer("actions.generation", &[]);

        let game = self.repository.load_game(session_id).await?;
        let actions: Vec<Action> = game.gen_actions().collect();

        self.metrics
            .record_gauge("actions.available_count", actions.len() as f64, &[])
            .await;
        self.metrics
            .increment_counter("actions.generated", 1, &[])
            .await;

        Ok(actions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::container::{MetricsHealth, StorageHealth, Timer};
    use std::collections::HashMap;
    use std::sync::Mutex;
    use std::time::Duration;

    // Mock implementations for testing
    struct MockActionValidator {
        should_fail: bool,
    }

    impl MockActionValidator {
        fn new() -> Self {
            Self { should_fail: false }
        }

        fn with_failure() -> Self {
            Self { should_fail: true }
        }
    }

    #[async_trait]
    impl ActionValidator for MockActionValidator {
        async fn validate_action(
            &self,
            _action: &Action,
            _game: &Game,
        ) -> Result<(), ApplicationError> {
            if self.should_fail {
                Err(ApplicationError::validation(
                    "action",
                    "Mock validation failure",
                    None,
                ))
            } else {
                Ok(())
            }
        }

        fn get_validation_rules(&self) -> Vec<String> {
            vec!["mock_rule".to_string()]
        }
    }

    struct MockGameRepository {
        sessions: Arc<Mutex<HashMap<SessionId, Game>>>,
        should_fail: bool,
    }

    impl MockGameRepository {
        fn new() -> Self {
            Self {
                sessions: Arc::new(Mutex::new(HashMap::new())),
                should_fail: false,
            }
        }
    }

    #[async_trait]
    impl GameRepository for MockGameRepository {
        async fn save_game(
            &self,
            session_id: &SessionId,
            game: &Game,
        ) -> Result<(), ApplicationError> {
            if self.should_fail {
                return Err(ApplicationError::infrastructure(
                    "storage",
                    true,
                    std::io::Error::new(std::io::ErrorKind::Other, "mock failure"),
                ));
            }

            let mut sessions = self.sessions.lock().unwrap();
            sessions.insert(session_id.clone(), game.clone());
            Ok(())
        }

        async fn load_game(&self, session_id: &SessionId) -> Result<Game, ApplicationError> {
            if self.should_fail {
                return Err(ApplicationError::infrastructure(
                    "storage",
                    true,
                    std::io::Error::new(std::io::ErrorKind::Other, "mock failure"),
                ));
            }

            let sessions = self.sessions.lock().unwrap();
            sessions
                .get(session_id)
                .cloned()
                .ok_or_else(|| ApplicationError::SessionNotFound {
                    session_id: session_id.as_str(),
                    ttl: Some(Duration::from_secs(3600)),
                })
        }

        async fn delete_game(&self, session_id: &SessionId) -> Result<(), ApplicationError> {
            let mut sessions = self.sessions.lock().unwrap();
            sessions.remove(session_id);
            Ok(())
        }

        async fn list_sessions(&self) -> Result<Vec<SessionId>, ApplicationError> {
            let sessions = self.sessions.lock().unwrap();
            Ok(sessions.keys().cloned().collect())
        }

        async fn health_check(&self) -> Result<StorageHealth, ApplicationError> {
            Ok(StorageHealth {
                is_healthy: !self.should_fail,
                latency_ms: 1,
                storage_size_mb: 100,
                active_connections: 1,
                error_rate: if self.should_fail { 1.0 } else { 0.0 },
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
            _callback: Arc<dyn crate::application::container::StateChangeCallback>,
        ) -> Result<(), ApplicationError> {
            Ok(())
        }

        async fn health_check(
            &self,
        ) -> Result<crate::application::container::NotificationHealth, ApplicationError> {
            Ok(crate::application::container::NotificationHealth {
                is_healthy: true,
                active_subscriptions: 0,
                message_queue_depth: 0,
                error_rate: 0.0,
            })
        }
    }

    struct MockMetricsCollector {
        metrics: Arc<Mutex<HashMap<String, f64>>>,
    }

    impl MockMetricsCollector {
        fn new() -> Self {
            Self {
                metrics: Arc::new(Mutex::new(HashMap::new())),
            }
        }

        fn get_metric(&self, name: &str) -> Option<f64> {
            let metrics = self.metrics.lock().unwrap();
            metrics.get(name).copied()
        }
    }

    #[async_trait]
    impl MetricsCollector for MockMetricsCollector {
        async fn increment_counter(&self, name: &str, value: u64, _tags: &[(&str, &str)]) {
            let mut metrics = self.metrics.lock().unwrap();
            *metrics.entry(name.to_string()).or_insert(0.0) += value as f64;
        }

        async fn record_gauge(&self, name: &str, value: f64, _tags: &[(&str, &str)]) {
            let mut metrics = self.metrics.lock().unwrap();
            metrics.insert(name.to_string(), value);
        }

        async fn record_histogram(&self, _name: &str, _value: f64, _tags: &[(&str, &str)]) {}

        async fn record_timing(&self, _name: &str, _duration: Duration, _tags: &[(&str, &str)]) {}

        fn start_timer(&self, _name: &str, _tags: &[(&str, &str)]) -> Box<dyn Timer> {
            Box::new(MockTimer)
        }

        async fn get_metrics_summary(
            &self,
        ) -> Result<crate::application::container::MetricsSummary, ApplicationError> {
            Ok(crate::application::container::MetricsSummary {
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
    async fn test_get_game_state_success() {
        let validator = Arc::new(MockActionValidator::new());
        let repository = Arc::new(MockGameRepository::new());
        let notifier = Arc::new(MockStateNotifier);
        let metrics = Arc::new(MockMetricsCollector::new());
        let config = ApplicationConfig::default();

        let service = GameApplicationService::new(
            validator,
            repository.clone(),
            notifier,
            metrics.clone(),
            config,
        );

        // Create a session first
        let session_id = SessionId::new();
        let game = Game::default();
        repository.save_game(&session_id, &game).await.unwrap();

        // Get game state
        let retrieved_game = service.get_game_state(&session_id).await.unwrap();

        // Verify metrics
        assert_eq!(metrics.get_metric("state.retrieved"), Some(1.0));
    }

    #[tokio::test]
    async fn test_get_game_state_not_found() {
        let validator = Arc::new(MockActionValidator::new());
        let repository = Arc::new(MockGameRepository::new());
        let notifier = Arc::new(MockStateNotifier);
        let metrics = Arc::new(MockMetricsCollector::new());
        let config = ApplicationConfig::default();

        let service = GameApplicationService::new(validator, repository, notifier, metrics, config);

        let session_id = SessionId::new();
        let result = service.get_game_state(&session_id).await;

        assert!(matches!(
            result,
            Err(ApplicationError::SessionNotFound { .. })
        ));
    }

    #[tokio::test]
    async fn test_get_available_actions() {
        let validator = Arc::new(MockActionValidator::new());
        let repository = Arc::new(MockGameRepository::new());
        let notifier = Arc::new(MockStateNotifier);
        let metrics = Arc::new(MockMetricsCollector::new());
        let config = ApplicationConfig::default();

        let service = GameApplicationService::new(
            validator,
            repository.clone(),
            notifier,
            metrics.clone(),
            config,
        );

        // Create a session with a started game
        let session_id = SessionId::new();
        let mut game = Game::default();
        game.start();
        repository.save_game(&session_id, &game).await.unwrap();

        // Get available actions
        let actions = service.get_available_actions(&session_id).await.unwrap();

        // Should have some actions available
        assert!(!actions.is_empty());

        // Verify metrics
        assert_eq!(metrics.get_metric("actions.generated"), Some(1.0));
        assert!(metrics.get_metric("actions.available_count").unwrap() > 0.0);
    }
}
