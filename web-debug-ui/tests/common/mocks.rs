//! Mock implementations for comprehensive testing using mockall
//!
//! Provides mock implementations of key interfaces and external dependencies
//! to enable isolated unit testing and TDD practices.

use mockall::{mock, predicate::*};
use balatro_rs::{
    action::Action,
    card::Card,
    error::GameError,
    game::Game,
    joker::{GameContext, Joker, JokerEffect, JokerId},
    joker_state::{JokerState, JokerStateManager},
    hand::SelectHand,
    rng::GameRng,
};
use std::collections::HashMap;
use std::sync::Arc;

// Mock for game repository interface (hypothetical)
mock! {
    pub GameRepository {}

    impl GameRepository for GameRepository {
        fn save_session(&self, session: &Game) -> Result<(), GameError>;
        fn load_session(&self, id: &str) -> Result<Game, GameError>;
        fn delete_session(&self, id: &str) -> Result<(), GameError>;
        fn list_sessions(&self) -> Result<Vec<String>, GameError>;
    }
}

// Mock for state notification service (hypothetical)
mock! {
    pub StateNotifier {}

    impl StateNotifier for StateNotifier {
        fn notify_state_change(&self, session_id: &str, state: &str);
        fn notify_action_result(&self, session_id: &str, result: &str);
        fn notify_game_ended(&self, session_id: &str, final_score: i32);
    }
}

// Mock for metrics collection (hypothetical)
mock! {
    pub MetricsCollector {}

    impl MetricsCollector for MetricsCollector {
        fn record_action_latency(&self, duration_ms: f64);
        fn record_memory_usage(&self, bytes: usize);
        fn increment_counter(&self, name: &str);
        fn record_game_duration(&self, duration_ms: f64);
        fn record_score(&self, score: i32);
    }
}

// Mock for joker factory (for testing joker creation)
mock! {
    pub JokerFactory {}

    impl JokerFactory for JokerFactory {
        fn create_joker(&self, id: JokerId) -> Result<Box<dyn Joker>, GameError>;
        fn get_joker_metadata(&self, id: JokerId) -> Option<String>;
        fn list_available_jokers(&self) -> Vec<JokerId>;
    }
}

// Mock for external RNG service (for deterministic testing)
mock! {
    pub MockRng {}

    impl MockRng for MockRng {
        fn gen_range(&self, min: u32, max: u32) -> u32;
        fn gen_bool(&self, probability: f64) -> bool;
        fn shuffle<T>(&self, slice: &mut [T]);
        fn seed(&mut self, seed: u64);
    }
}

// Mock for joker state persistence
mock! {
    pub MockJokerStateManager {}

    impl MockJokerStateManager for MockJokerStateManager {
        fn get_state(&self, joker_id: JokerId) -> Option<JokerState>;
        fn set_state(&self, joker_id: JokerId, state: JokerState);
        fn clear_state(&self, joker_id: JokerId);
        fn clear_all_states(&self);
    }
}

// Mock for performance profiler
mock! {
    pub PerformanceProfiler {}

    impl PerformanceProfiler for PerformanceProfiler {
        fn start_timing(&self, operation: &str) -> u64; // Returns timing ID
        fn end_timing(&self, timing_id: u64) -> f64; // Returns duration in ms
        fn record_memory_snapshot(&self, label: &str) -> usize; // Returns memory usage
        fn get_performance_summary(&self) -> String;
    }
}

// Mock for concurrent session manager
mock! {
    pub SessionManager {}

    impl SessionManager for SessionManager {
        fn create_session(&self, config: &str) -> Result<String, GameError>; // Returns session ID
        fn get_session(&self, id: &str) -> Result<Arc<Game>, GameError>;
        fn remove_session(&self, id: &str) -> Result<(), GameError>;
        fn cleanup_expired_sessions(&self) -> usize; // Returns number cleaned up
        fn get_active_session_count(&self) -> usize;
    }
}

/// Helper functions for creating pre-configured mocks

/// Creates a mock repository that always succeeds
pub fn create_successful_repository() -> MockGameRepository {
    let mut mock = MockGameRepository::new();
    mock.expect_save_session()
        .returning(|_| Ok(()));
    mock.expect_load_session()
        .returning(|_| Ok(Game::default()));
    mock.expect_delete_session()
        .returning(|_| Ok(()));
    mock.expect_list_sessions()
        .returning(|| Ok(vec!["session1".to_string(), "session2".to_string()]));
    mock
}

/// Creates a mock repository that always fails
pub fn create_failing_repository() -> MockGameRepository {
    let mut mock = MockGameRepository::new();
    mock.expect_save_session()
        .returning(|_| Err(GameError::InvalidAction("Mock save failure".to_string())));
    mock.expect_load_session()
        .returning(|_| Err(GameError::InvalidAction("Mock load failure".to_string())));
    mock.expect_delete_session()
        .returning(|_| Err(GameError::InvalidAction("Mock delete failure".to_string())));
    mock.expect_list_sessions()
        .returning(|| Err(GameError::InvalidAction("Mock list failure".to_string())));
    mock
}

/// Creates a mock notifier that tracks all calls
pub fn create_tracking_notifier() -> MockStateNotifier {
    let mut mock = MockStateNotifier::new();
    mock.expect_notify_state_change()
        .returning(|_, _| ());
    mock.expect_notify_action_result()
        .returning(|_, _| ());
    mock.expect_notify_game_ended()
        .returning(|_, _| ());
    mock
}

/// Creates a mock metrics collector that accepts all metrics
pub fn create_mock_metrics() -> MockMetricsCollector {
    let mut mock = MockMetricsCollector::new();
    mock.expect_record_action_latency()
        .returning(|_| ());
    mock.expect_record_memory_usage()
        .returning(|_| ());
    mock.expect_increment_counter()
        .returning(|_| ());
    mock.expect_record_game_duration()
        .returning(|_| ());
    mock.expect_record_score()
        .returning(|_| ());
    mock
}

/// Creates a deterministic mock RNG for reproducible tests
pub fn create_deterministic_rng() -> MockRng {
    let mut mock = MockRng::new();

    // Always return predictable values for testing
    mock.expect_gen_range()
        .returning(|min, max| min + (max - min) / 2); // Always return middle value

    mock.expect_gen_bool()
        .returning(|p| p > 0.5); // Deterministic based on probability

    mock.expect_shuffle()
        .returning(|_| ()); // No-op shuffle for deterministic results

    mock.expect_seed()
        .returning(|_| ());

    mock
}

/// Creates a mock performance profiler for benchmarking tests
pub fn create_mock_profiler() -> MockPerformanceProfiler {
    let mut mock = MockPerformanceProfiler::new();

    // Track timing IDs
    let mut next_timing_id = 1u64;
    let timing_id = next_timing_id;
    next_timing_id += 1;

    mock.expect_start_timing()
        .returning(move |_| timing_id);

    mock.expect_end_timing()
        .returning(|_| 10.5); // Always return 10.5ms for predictable tests

    mock.expect_record_memory_snapshot()
        .returning(|_| 1024); // Always return 1KB for predictable tests

    mock.expect_get_performance_summary()
        .returning(|| "Mock performance summary".to_string());

    mock
}

/// Creates a mock session manager for concurrent testing
pub fn create_mock_session_manager() -> MockSessionManager {
    let mut mock = MockSessionManager::new();

    mock.expect_create_session()
        .returning(|_| Ok("test-session-123".to_string()));

    mock.expect_get_session()
        .returning(|_| Ok(Arc::new(Game::default())));

    mock.expect_remove_session()
        .returning(|_| Ok(()));

    mock.expect_cleanup_expired_sessions()
        .returning(|| 0);

    mock.expect_get_active_session_count()
        .returning(|| 1);

    mock
}

/// Mock implementations for testing scenarios

/// Simulates a slow repository for performance testing
pub fn create_slow_repository(delay_ms: u64) -> MockGameRepository {
    let mut mock = MockGameRepository::new();

    mock.expect_save_session()
        .returning(move |_| {
            std::thread::sleep(std::time::Duration::from_millis(delay_ms));
            Ok(())
        });

    mock.expect_load_session()
        .returning(move |_| {
            std::thread::sleep(std::time::Duration::from_millis(delay_ms));
            Ok(Game::default())
        });

    mock
}

/// Simulates memory pressure for stress testing
pub fn create_memory_pressure_metrics() -> MockMetricsCollector {
    let mut mock = MockMetricsCollector::new();

    mock.expect_record_memory_usage()
        .returning(|bytes| {
            // Simulate memory pressure by allocating and deallocating
            let _pressure: Vec<u8> = vec![0; bytes];
            // Memory automatically freed when _pressure goes out of scope
        });

    mock.expect_record_action_latency()
        .returning(|_| ());
    mock.expect_increment_counter()
        .returning(|_| ());
    mock.expect_record_game_duration()
        .returning(|_| ());
    mock.expect_record_score()
        .returning(|_| ());

    mock
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_successful_repository_mock() {
        let repo = create_successful_repository();
        let game = Game::default();

        // Test that mock returns expected results
        assert!(repo.save_session(&game).is_ok());
        assert!(repo.load_session("test").is_ok());
        assert!(repo.delete_session("test").is_ok());
        assert!(repo.list_sessions().is_ok());
    }

    #[test]
    fn test_failing_repository_mock() {
        let repo = create_failing_repository();
        let game = Game::default();

        // Test that mock returns expected failures
        assert!(repo.save_session(&game).is_err());
        assert!(repo.load_session("test").is_err());
        assert!(repo.delete_session("test").is_err());
        assert!(repo.list_sessions().is_err());
    }

    #[test]
    fn test_deterministic_rng_mock() {
        let rng = create_deterministic_rng();

        // Test that mock returns predictable values
        assert_eq!(rng.gen_range(0, 10), 5); // Should return middle value
        assert_eq!(rng.gen_bool(0.6), true); // Should return true for p > 0.5
        assert_eq!(rng.gen_bool(0.4), false); // Should return false for p <= 0.5
    }

    #[test]
    fn test_mock_profiler() {
        let profiler = create_mock_profiler();

        let timing_id = profiler.start_timing("test_operation");
        let duration = profiler.end_timing(timing_id);
        let memory = profiler.record_memory_snapshot("test_snapshot");
        let summary = profiler.get_performance_summary();

        assert_eq!(duration, 10.5);
        assert_eq!(memory, 1024);
        assert!(!summary.is_empty());
    }

    #[test]
    fn test_mock_session_manager() {
        let manager = create_mock_session_manager();

        let session_id = manager.create_session("test_config").unwrap();
        assert_eq!(session_id, "test-session-123");

        let session = manager.get_session(&session_id).unwrap();
        assert!(!session.is_over());

        assert!(manager.remove_session(&session_id).is_ok());
        assert_eq!(manager.get_active_session_count(), 1);
    }
}
