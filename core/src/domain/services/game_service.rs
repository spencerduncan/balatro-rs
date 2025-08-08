//! Game Service - Orchestrates game operations with proper thread-safe patterns
//!
//! This implementation uses RwLock for interior mutability instead of the broken
//! Arc::get_mut pattern which would always fail at runtime.

use crate::domain::value_objects::{SessionId, ValidationError};
use crate::error::GameError;
use std::borrow::Cow;
use std::sync::{Arc, RwLock};

/// Error type for domain services with zero-allocation string handling
#[derive(Debug, Clone)]
pub enum DomainError {
    /// Service-level errors with static strings to avoid allocations
    ServiceError(Cow<'static, str>),
    /// Repository errors
    RepositoryError(Cow<'static, str>),
    /// Validation errors
    ValidationError(ValidationError),
    /// Game logic errors
    GameError(GameError),
}

impl From<ValidationError> for DomainError {
    #[inline]
    fn from(err: ValidationError) -> Self {
        DomainError::ValidationError(err)
    }
}

impl From<GameError> for DomainError {
    #[inline]
    fn from(err: GameError) -> Self {
        DomainError::GameError(err)
    }
}

/// Trait for game repository operations
pub trait GameRepository: Send + Sync {
    /// Save game state
    fn save(&mut self, session_id: &SessionId, state: Vec<u8>) -> Result<(), DomainError>;

    /// Load game state
    fn load(&self, session_id: &SessionId) -> Result<Vec<u8>, DomainError>;

    /// Delete game state
    fn delete(&mut self, session_id: &SessionId) -> Result<(), DomainError>;
}

/// Trait for session repository operations
pub trait SessionRepository: Send + Sync {
    /// Create new session
    fn create(&mut self) -> Result<SessionId, DomainError>;

    /// Validate session exists
    fn exists(&self, session_id: &SessionId) -> Result<bool, DomainError>;

    /// Update session timestamp
    fn touch(&mut self, session_id: &SessionId) -> Result<(), DomainError>;
}

/// Trait for action history repository operations
pub trait ActionHistoryRepository: Send + Sync {
    /// Record an action
    fn record(&mut self, session_id: &SessionId, action: Vec<u8>) -> Result<(), DomainError>;

    /// Get action history
    fn get_history(&self, session_id: &SessionId) -> Result<Vec<Vec<u8>>, DomainError>;

    /// Clear history
    fn clear(&mut self, session_id: &SessionId) -> Result<(), DomainError>;
}

/// Game service that orchestrates game operations using repositories
///
/// Uses RwLock for proper interior mutability instead of Arc::get_mut
/// which would always fail with multiple references
pub struct GameService<G, S, A>
where
    G: GameRepository + 'static,
    S: SessionRepository + 'static,
    A: ActionHistoryRepository + 'static,
{
    /// Game repository with thread-safe interior mutability
    game_repo: Arc<RwLock<G>>,
    /// Session repository with thread-safe interior mutability
    session_repo: Arc<RwLock<S>>,
    /// Action history repository with thread-safe interior mutability
    history_repo: Arc<RwLock<A>>,
}

impl<G, S, A> GameService<G, S, A>
where
    G: GameRepository + 'static,
    S: SessionRepository + 'static,
    A: ActionHistoryRepository + 'static,
{
    /// Create new game service with proper thread-safe repositories
    #[inline]
    pub fn new(game_repo: G, session_repo: S, history_repo: A) -> Self {
        Self {
            game_repo: Arc::new(RwLock::new(game_repo)),
            session_repo: Arc::new(RwLock::new(session_repo)),
            history_repo: Arc::new(RwLock::new(history_repo)),
        }
    }

    /// Start a new game session
    pub fn start_new_game(&self) -> Result<SessionId, DomainError> {
        // Use write lock for mutable access
        let mut session_repo = self.session_repo.write().map_err(|_| {
            DomainError::ServiceError(Cow::Borrowed("Failed to acquire session lock"))
        })?;

        session_repo.create()
    }

    /// Save game state for a session
    pub fn save_game(&self, session_id: &SessionId, state: Vec<u8>) -> Result<(), DomainError> {
        // Validate session exists first (read lock)
        {
            let session_repo = self.session_repo.read().map_err(|_| {
                DomainError::ServiceError(Cow::Borrowed("Failed to acquire session lock"))
            })?;

            if !session_repo.exists(session_id)? {
                return Err(DomainError::ServiceError(Cow::Borrowed(
                    "Session not found",
                )));
            }
        }

        // Save game state (write lock)
        let mut game_repo = self
            .game_repo
            .write()
            .map_err(|_| DomainError::ServiceError(Cow::Borrowed("Failed to acquire game lock")))?;

        game_repo.save(session_id, state)?;

        // Update session timestamp (write lock)
        let mut session_repo = self.session_repo.write().map_err(|_| {
            DomainError::ServiceError(Cow::Borrowed("Failed to acquire session lock"))
        })?;

        session_repo.touch(session_id)
    }

    /// Load game state for a session
    pub fn load_game(&self, session_id: &SessionId) -> Result<Vec<u8>, DomainError> {
        // Validate session exists (read lock)
        {
            let session_repo = self.session_repo.read().map_err(|_| {
                DomainError::ServiceError(Cow::Borrowed("Failed to acquire session lock"))
            })?;

            if !session_repo.exists(session_id)? {
                return Err(DomainError::ServiceError(Cow::Borrowed(
                    "Session not found",
                )));
            }
        }

        // Load game state (read lock)
        let game_repo = self
            .game_repo
            .read()
            .map_err(|_| DomainError::ServiceError(Cow::Borrowed("Failed to acquire game lock")))?;

        game_repo.load(session_id)
    }

    /// Record an action for a session
    pub fn record_action(
        &self,
        session_id: &SessionId,
        action: Vec<u8>,
    ) -> Result<(), DomainError> {
        // Validate session exists (read lock)
        {
            let session_repo = self.session_repo.read().map_err(|_| {
                DomainError::ServiceError(Cow::Borrowed("Failed to acquire session lock"))
            })?;

            if !session_repo.exists(session_id)? {
                return Err(DomainError::ServiceError(Cow::Borrowed(
                    "Session not found",
                )));
            }
        }

        // Record action (write lock)
        let mut history_repo = self.history_repo.write().map_err(|_| {
            DomainError::ServiceError(Cow::Borrowed("Failed to acquire history lock"))
        })?;

        history_repo.record(session_id, action)
    }

    /// Get action history for a session
    pub fn get_action_history(&self, session_id: &SessionId) -> Result<Vec<Vec<u8>>, DomainError> {
        // Get history (read lock)
        let history_repo = self.history_repo.read().map_err(|_| {
            DomainError::ServiceError(Cow::Borrowed("Failed to acquire history lock"))
        })?;

        history_repo.get_history(session_id)
    }

    /// Delete all data for a session
    pub fn delete_session(&self, session_id: &SessionId) -> Result<(), DomainError> {
        // Delete game state (write lock)
        {
            let mut game_repo = self.game_repo.write().map_err(|_| {
                DomainError::ServiceError(Cow::Borrowed("Failed to acquire game lock"))
            })?;

            game_repo.delete(session_id)?;
        }

        // Clear history (write lock)
        {
            let mut history_repo = self.history_repo.write().map_err(|_| {
                DomainError::ServiceError(Cow::Borrowed("Failed to acquire history lock"))
            })?;

            history_repo.clear(session_id)?;
        }

        Ok(())
    }

    /// Get a cloned reference to the game repository
    #[inline]
    pub fn game_repository(&self) -> Arc<RwLock<G>> {
        Arc::clone(&self.game_repo)
    }

    /// Get a cloned reference to the session repository
    #[inline]
    pub fn session_repository(&self) -> Arc<RwLock<S>> {
        Arc::clone(&self.session_repo)
    }

    /// Get a cloned reference to the history repository
    #[inline]
    pub fn history_repository(&self) -> Arc<RwLock<A>> {
        Arc::clone(&self.history_repo)
    }
}

/// Thread-safe and cloneable
impl<G, S, A> Clone for GameService<G, S, A>
where
    G: GameRepository + 'static,
    S: SessionRepository + 'static,
    A: ActionHistoryRepository + 'static,
{
    fn clone(&self) -> Self {
        Self {
            game_repo: Arc::clone(&self.game_repo),
            session_repo: Arc::clone(&self.session_repo),
            history_repo: Arc::clone(&self.history_repo),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    /// Mock game repository for testing
    struct MockGameRepository {
        storage: HashMap<SessionId, Vec<u8>>,
    }

    impl MockGameRepository {
        fn new() -> Self {
            Self {
                storage: HashMap::new(),
            }
        }
    }

    impl GameRepository for MockGameRepository {
        fn save(&mut self, session_id: &SessionId, state: Vec<u8>) -> Result<(), DomainError> {
            self.storage.insert(session_id.clone(), state);
            Ok(())
        }

        fn load(&self, session_id: &SessionId) -> Result<Vec<u8>, DomainError> {
            self.storage
                .get(session_id)
                .cloned()
                .ok_or(DomainError::RepositoryError(Cow::Borrowed(
                    "Game state not found",
                )))
        }

        fn delete(&mut self, session_id: &SessionId) -> Result<(), DomainError> {
            self.storage.remove(session_id);
            Ok(())
        }
    }

    /// Mock session repository for testing
    struct MockSessionRepository {
        sessions: HashMap<SessionId, bool>,
    }

    impl MockSessionRepository {
        fn new() -> Self {
            Self {
                sessions: HashMap::new(),
            }
        }
    }

    impl SessionRepository for MockSessionRepository {
        fn create(&mut self) -> Result<SessionId, DomainError> {
            let session_id = SessionId::new();
            self.sessions.insert(session_id.clone(), true);
            Ok(session_id)
        }

        fn exists(&self, session_id: &SessionId) -> Result<bool, DomainError> {
            Ok(self.sessions.contains_key(session_id))
        }

        fn touch(&mut self, session_id: &SessionId) -> Result<(), DomainError> {
            if !self.sessions.contains_key(session_id) {
                return Err(DomainError::RepositoryError(Cow::Borrowed(
                    "Session not found",
                )));
            }
            Ok(())
        }
    }

    /// Mock action history repository for testing
    struct MockActionHistoryRepository {
        history: HashMap<SessionId, Vec<Vec<u8>>>,
    }

    impl MockActionHistoryRepository {
        fn new() -> Self {
            Self {
                history: HashMap::new(),
            }
        }
    }

    impl ActionHistoryRepository for MockActionHistoryRepository {
        fn record(&mut self, session_id: &SessionId, action: Vec<u8>) -> Result<(), DomainError> {
            self.history
                .entry(session_id.clone())
                .or_default()
                .push(action);
            Ok(())
        }

        fn get_history(&self, session_id: &SessionId) -> Result<Vec<Vec<u8>>, DomainError> {
            Ok(self.history.get(session_id).cloned().unwrap_or_default())
        }

        fn clear(&mut self, session_id: &SessionId) -> Result<(), DomainError> {
            self.history.remove(session_id);
            Ok(())
        }
    }

    #[test]
    fn test_game_service_thread_safety() {
        let game_repo = MockGameRepository::new();
        let session_repo = MockSessionRepository::new();
        let history_repo = MockActionHistoryRepository::new();

        let service = GameService::new(game_repo, session_repo, history_repo);

        // Test that we can clone the service (multiple Arc references)
        let service_clone = service.clone();

        // Start a new game
        let session_id = service.start_new_game().expect("Failed to start game");

        // Save game state - this would fail with Arc::get_mut but works with RwLock
        let state = vec![1, 2, 3, 4];
        service
            .save_game(&session_id, state.clone())
            .expect("Failed to save game");

        // Load game state from cloned service
        let loaded_state = service_clone
            .load_game(&session_id)
            .expect("Failed to load game");
        assert_eq!(loaded_state, state);

        // Record action
        let action = vec![5, 6, 7];
        service
            .record_action(&session_id, action.clone())
            .expect("Failed to record action");

        // Get history
        let history = service
            .get_action_history(&session_id)
            .expect("Failed to get history");
        assert_eq!(history.len(), 1);
        assert_eq!(history[0], action);
    }

    #[test]
    fn test_concurrent_access() {
        use std::sync::Arc;
        use std::thread;

        let game_repo = MockGameRepository::new();
        let session_repo = MockSessionRepository::new();
        let history_repo = MockActionHistoryRepository::new();

        let service = Arc::new(GameService::new(game_repo, session_repo, history_repo));

        // Start a session
        let session_id = service.start_new_game().expect("Failed to start game");

        // Spawn multiple threads accessing the service concurrently
        let mut handles = vec![];

        for i in 0..10 {
            let service_clone = Arc::clone(&service);
            let session_id_clone = session_id.clone();

            let handle = thread::spawn(move || {
                // Each thread saves and loads game state
                let state = vec![i as u8; 4];
                service_clone
                    .save_game(&session_id_clone, state.clone())
                    .expect("Failed to save game");

                // Record an action
                let action = vec![i as u8; 2];
                service_clone
                    .record_action(&session_id_clone, action)
                    .expect("Failed to record action");
            });

            handles.push(handle);
        }

        // Wait for all threads to complete
        for handle in handles {
            handle.join().expect("Thread panicked");
        }

        // Verify we have 10 actions recorded
        let history = service
            .get_action_history(&session_id)
            .expect("Failed to get history");
        assert_eq!(history.len(), 10);
    }
}
