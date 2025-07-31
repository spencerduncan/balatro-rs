//! GameRepository Interface
//!
//! Defines the contract for persisting and retrieving game sessions.
//! This interface follows the Repository pattern and enables the domain layer
//! to work with different storage implementations without coupling.

use crate::domain::{entities::GameSession, errors::DomainError, value_objects::SessionId};
use std::time::Duration;

/// Repository interface for managing game session persistence
///
/// GameRepository defines the contract that storage implementations must fulfill
/// to integrate with the domain layer. This enables dependency inversion by
/// allowing the domain to depend on abstractions rather than concrete storage.
///
/// # Design Principles
///
/// - **Single Responsibility**: Only handles session persistence
/// - **Interface Segregation**: Focused interface for session operations
/// - **Dependency Inversion**: Domain depends on this abstraction
///
/// # Examples
///
/// ```ignore
/// use balatro_domain::{GameRepository, GameSession, SessionId, DomainError};
///
/// struct InMemoryRepository;
///
/// impl GameRepository for InMemoryRepository {
///     fn save_session(&self, session: &GameSession) -> Result<(), DomainError> {
///         // Implementation here
///         Ok(())
///     }
///
///     fn load_session(&self, id: &SessionId) -> Result<GameSession, DomainError> {
///         // Implementation here
///         todo!()
///     }
///
///     // ... other methods
/// }
/// ```
pub trait GameRepository: Send + Sync {
    /// Save a game session to persistent storage
    ///
    /// # Arguments
    ///
    /// * `session` - The game session to save
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the session was successfully saved
    /// * `Err(DomainError)` - If saving failed
    ///
    /// # Errors
    ///
    /// * `DomainError::RepositoryError` - Storage operation failed
    /// * `DomainError::ConcurrentModification` - Session was modified concurrently
    fn save_session(&self, session: &GameSession) -> Result<(), DomainError>;

    /// Load a game session from persistent storage
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the session to load
    ///
    /// # Returns
    ///
    /// * `Ok(GameSession)` - The loaded session
    /// * `Err(DomainError)` - If loading failed
    ///
    /// # Errors
    ///
    /// * `DomainError::SessionNotFound` - Session does not exist
    /// * `DomainError::RepositoryError` - Storage operation failed
    /// * `DomainError::StateInconsistency` - Loaded data is corrupted
    fn load_session(&self, id: &SessionId) -> Result<GameSession, DomainError>;

    /// Delete a game session from persistent storage
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the session to delete
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the session was successfully deleted or didn't exist
    /// * `Err(DomainError)` - If deletion failed
    ///
    /// # Errors
    ///
    /// * `DomainError::RepositoryError` - Storage operation failed
    fn delete_session(&self, id: &SessionId) -> Result<(), DomainError>;

    /// Check if a session exists in storage
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the session to check
    ///
    /// # Returns
    ///
    /// * `Ok(true)` - If the session exists
    /// * `Ok(false)` - If the session does not exist
    /// * `Err(DomainError)` - If the check failed
    ///
    /// # Errors
    ///
    /// * `DomainError::RepositoryError` - Storage operation failed
    fn session_exists(&self, id: &SessionId) -> Result<bool, DomainError>;

    /// List all active session IDs
    ///
    /// Returns all session IDs that are currently stored and not expired.
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<SessionId>)` - List of active session IDs
    /// * `Err(DomainError)` - If listing failed
    ///
    /// # Errors
    ///
    /// * `DomainError::RepositoryError` - Storage operation failed
    fn list_active_sessions(&self) -> Result<Vec<SessionId>, DomainError>;

    /// Clean up expired sessions
    ///
    /// Removes sessions that have been inactive for longer than the specified duration.
    ///
    /// # Arguments
    ///
    /// * `max_age` - Maximum age for sessions before they are considered expired
    ///
    /// # Returns
    ///
    /// * `Ok(usize)` - Number of sessions that were cleaned up
    /// * `Err(DomainError)` - If cleanup failed
    ///
    /// # Errors
    ///
    /// * `DomainError::RepositoryError` - Storage operation failed
    fn cleanup_expired_sessions(&self, max_age: Duration) -> Result<usize, DomainError>;

    /// Get repository health status
    ///
    /// Checks if the repository is healthy and can perform operations.
    ///
    /// # Returns
    ///
    /// * `Ok(true)` - Repository is healthy
    /// * `Ok(false)` - Repository has issues but is still functional
    /// * `Err(DomainError)` - Repository is completely unavailable
    ///
    /// # Errors
    ///
    /// * `DomainError::RepositoryError` - Repository is unavailable
    fn health_check(&self) -> Result<bool, DomainError>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{entities::GameSession, value_objects::SessionId};
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};
    use std::time::{Duration, Instant};

    /// Mock implementation of GameRepository for testing
    #[derive(Debug, Default)]
    struct MockGameRepository {
        sessions: Arc<Mutex<HashMap<SessionId, (GameSession, Instant)>>>,
        fail_save: bool,
        fail_load: bool,
        fail_delete: bool,
        fail_exists: bool,
        fail_list: bool,
        fail_cleanup: bool,
        fail_health: bool,
    }

    impl MockGameRepository {
        fn new() -> Self {
            Self::default()
        }

        fn with_failure(mut self, operation: &str) -> Self {
            match operation {
                "save" => self.fail_save = true,
                "load" => self.fail_load = true,
                "delete" => self.fail_delete = true,
                "exists" => self.fail_exists = true,
                "list" => self.fail_list = true,
                "cleanup" => self.fail_cleanup = true,
                "health" => self.fail_health = true,
                _ => panic!("Unknown operation: {}", operation),
            }
            self
        }
    }

    impl GameRepository for MockGameRepository {
        fn save_session(&self, session: &GameSession) -> Result<(), DomainError> {
            if self.fail_save {
                return Err(DomainError::repository_error(
                    "save_session",
                    "Mock failure",
                ));
            }

            let mut sessions = self.sessions.lock().unwrap();
            sessions.insert(session.id().clone(), (session.clone(), Instant::now()));
            Ok(())
        }

        fn load_session(&self, id: &SessionId) -> Result<GameSession, DomainError> {
            if self.fail_load {
                return Err(DomainError::repository_error(
                    "load_session",
                    "Mock failure",
                ));
            }

            let sessions = self.sessions.lock().unwrap();
            sessions
                .get(id)
                .map(|(session, _)| session.clone())
                .ok_or_else(|| DomainError::session_not_found(id))
        }

        fn delete_session(&self, id: &SessionId) -> Result<(), DomainError> {
            if self.fail_delete {
                return Err(DomainError::repository_error(
                    "delete_session",
                    "Mock failure",
                ));
            }

            let mut sessions = self.sessions.lock().unwrap();
            sessions.remove(id);
            Ok(())
        }

        fn session_exists(&self, id: &SessionId) -> Result<bool, DomainError> {
            if self.fail_exists {
                return Err(DomainError::repository_error(
                    "session_exists",
                    "Mock failure",
                ));
            }

            let sessions = self.sessions.lock().unwrap();
            Ok(sessions.contains_key(id))
        }

        fn list_active_sessions(&self) -> Result<Vec<SessionId>, DomainError> {
            if self.fail_list {
                return Err(DomainError::repository_error(
                    "list_active_sessions",
                    "Mock failure",
                ));
            }

            let sessions = self.sessions.lock().unwrap();
            Ok(sessions.keys().cloned().collect())
        }

        fn cleanup_expired_sessions(&self, max_age: Duration) -> Result<usize, DomainError> {
            if self.fail_cleanup {
                return Err(DomainError::repository_error(
                    "cleanup_expired_sessions",
                    "Mock failure",
                ));
            }

            let mut sessions = self.sessions.lock().unwrap();
            let now = Instant::now();
            let initial_count = sessions.len();

            sessions.retain(|_, (_, created_at)| now.duration_since(*created_at) < max_age);

            Ok(initial_count - sessions.len())
        }

        fn health_check(&self) -> Result<bool, DomainError> {
            if self.fail_health {
                return Err(DomainError::repository_error(
                    "health_check",
                    "Mock failure",
                ));
            }

            Ok(true)
        }
    }

    // Note: These tests require GameSession to be implemented
    // They will be uncommented once GameSession is available

    #[test]
    fn trait_can_be_implemented() {
        let _repo = MockGameRepository::new();
        // This test ensures the trait can be implemented
        // Actual functionality tests will be added after GameSession is implemented
    }

    #[test]
    fn repository_trait_is_object_safe() {
        let repo = MockGameRepository::new();
        let _trait_object: &dyn GameRepository = &repo;
        // This test ensures the trait is object-safe (can be used as trait object)
    }

    #[test]
    fn repository_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<MockGameRepository>();
        // This test ensures implementations can be used across threads
    }

    // TODO: Add comprehensive integration tests once GameSession is implemented
    // These tests will cover:
    // - save_session success and failure cases
    // - load_session success and failure cases
    // - delete_session success and failure cases
    // - session_exists behavior
    // - list_active_sessions functionality
    // - cleanup_expired_sessions behavior
    // - health_check responses
    // - Error type propagation
    // - Concurrent access scenarios
}
