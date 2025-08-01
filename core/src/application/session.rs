//! Simple Session Management for Game Engine
//!
//! Provides basic session management functionality without enterprise complexity.
//! Designed for game engine contexts where simplicity and performance matter.

use crate::action::Action;
use crate::config::Config;
use crate::game::Game;
use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

/// Session identifier type
pub type SessionId = String;

/// Simple error types for session operations
#[derive(Debug, Clone)]
pub enum SessionError {
    /// Session not found
    SessionNotFound { session_id: SessionId },
    /// Invalid action for current game state
    InvalidAction { action: Action, reason: String },
    /// Game configuration error
    ConfigurationError { message: String },
    /// Session already exists
    SessionExists { session_id: SessionId },
}

impl fmt::Display for SessionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SessionError::SessionNotFound { session_id } => {
                write!(f, "Session not found: {session_id}")
            }
            SessionError::InvalidAction { action, reason } => {
                write!(f, "Invalid action {action:?}: {reason}")
            }
            SessionError::ConfigurationError { message } => {
                write!(f, "Configuration error: {message}")
            }
            SessionError::SessionExists { session_id } => {
                write!(f, "Session already exists: {session_id}")
            }
        }
    }
}

impl std::error::Error for SessionError {}

/// A game session with metadata
#[derive(Debug)]
pub struct GameSession {
    /// The core game instance
    pub game: Game,
    /// Session creation time
    pub created_at: Instant,
    /// Last activity time
    pub last_activity: Instant,
}

impl GameSession {
    /// Create a new game session
    pub fn new(config: Config) -> Self {
        let now = Instant::now();
        Self {
            game: Game::new(config),
            created_at: now,
            last_activity: now,
        }
    }

    /// Update last activity timestamp
    pub fn touch(&mut self) {
        self.last_activity = Instant::now();
    }

    /// Get session age
    pub fn age(&self) -> Duration {
        self.created_at.elapsed()
    }

    /// Get time since last activity
    pub fn idle_time(&self) -> Duration {
        self.last_activity.elapsed()
    }

    /// Check if session is considered stale
    pub fn is_stale(&self, max_idle: Duration) -> bool {
        self.idle_time() > max_idle
    }
}

/// Simple session manager for game instances
pub struct SessionManager {
    sessions: Arc<RwLock<HashMap<SessionId, GameSession>>>,
    default_config: Config,
    max_idle_duration: Duration,
}

impl SessionManager {
    /// Create a new session manager
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
            default_config: Config::default(),
            max_idle_duration: Duration::from_secs(3600), // 1 hour
        }
    }

    /// Create a new session manager with custom configuration
    pub fn with_config(config: Config, max_idle: Duration) -> Self {
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
            default_config: config,
            max_idle_duration: max_idle,
        }
    }

    /// Create a new game session
    pub fn create_session(&self, session_id: SessionId) -> Result<(), SessionError> {
        let mut sessions = self.sessions.write().unwrap();

        if sessions.contains_key(&session_id) {
            return Err(SessionError::SessionExists { session_id });
        }

        let mut session = GameSession::new(self.default_config.clone());
        session.game.start();
        sessions.insert(session_id, session);

        Ok(())
    }

    /// Create a new game session with custom configuration
    pub fn create_session_with_config(
        &self,
        session_id: SessionId,
        config: Config,
    ) -> Result<(), SessionError> {
        let mut sessions = self.sessions.write().unwrap();

        if sessions.contains_key(&session_id) {
            return Err(SessionError::SessionExists { session_id });
        }

        let mut session = GameSession::new(config);
        session.game.start();
        sessions.insert(session_id, session);

        Ok(())
    }

    /// Get available actions for a session
    pub fn get_actions(&self, session_id: &SessionId) -> Result<Vec<Action>, SessionError> {
        let mut sessions = self.sessions.write().unwrap();

        let session =
            sessions
                .get_mut(session_id)
                .ok_or_else(|| SessionError::SessionNotFound {
                    session_id: session_id.clone(),
                })?;

        session.touch();
        let actions: Vec<Action> = session.game.gen_actions().collect();
        Ok(actions)
    }

    /// Execute an action in a session
    pub fn execute_action(
        &self,
        session_id: &SessionId,
        action: Action,
    ) -> Result<(), SessionError> {
        let mut sessions = self.sessions.write().unwrap();

        let session =
            sessions
                .get_mut(session_id)
                .ok_or_else(|| SessionError::SessionNotFound {
                    session_id: session_id.clone(),
                })?;

        session.touch();

        session
            .game
            .handle_action(action.clone())
            .map_err(|e| SessionError::InvalidAction {
                action,
                reason: e.to_string(),
            })?;

        Ok(())
    }

    /// Get game state reference for a session
    pub fn with_game<T, F>(&self, session_id: &SessionId, f: F) -> Result<T, SessionError>
    where
        F: FnOnce(&Game) -> T,
    {
        let mut sessions = self.sessions.write().unwrap();

        let session =
            sessions
                .get_mut(session_id)
                .ok_or_else(|| SessionError::SessionNotFound {
                    session_id: session_id.clone(),
                })?;

        session.touch();
        Ok(f(&session.game))
    }

    /// Check if game is over for a session
    pub fn is_game_over(&self, session_id: &SessionId) -> Result<bool, SessionError> {
        let sessions = self.sessions.read().unwrap();

        let session = sessions
            .get(session_id)
            .ok_or_else(|| SessionError::SessionNotFound {
                session_id: session_id.clone(),
            })?;

        Ok(session.game.is_over())
    }

    /// Delete a session
    pub fn delete_session(&self, session_id: &SessionId) -> Result<(), SessionError> {
        let mut sessions = self.sessions.write().unwrap();

        sessions
            .remove(session_id)
            .ok_or_else(|| SessionError::SessionNotFound {
                session_id: session_id.clone(),
            })?;

        Ok(())
    }

    /// List all active sessions
    pub fn list_sessions(&self) -> Vec<SessionId> {
        let sessions = self.sessions.read().unwrap();
        sessions.keys().cloned().collect()
    }

    /// Get session count
    pub fn session_count(&self) -> usize {
        let sessions = self.sessions.read().unwrap();
        sessions.len()
    }

    /// Clean up stale sessions
    pub fn cleanup_stale_sessions(&self) -> usize {
        let mut sessions = self.sessions.write().unwrap();

        let stale_sessions: Vec<SessionId> = sessions
            .iter()
            .filter(|(_, session)| session.is_stale(self.max_idle_duration))
            .map(|(id, _)| id.clone())
            .collect();

        let count = stale_sessions.len();
        for session_id in stale_sessions {
            sessions.remove(&session_id);
        }

        count
    }

    /// Get session metadata without touching activity time
    pub fn get_session_info(
        &self,
        session_id: &SessionId,
    ) -> Result<(Duration, Duration, bool), SessionError> {
        let sessions = self.sessions.read().unwrap();

        let session = sessions
            .get(session_id)
            .ok_or_else(|| SessionError::SessionNotFound {
                session_id: session_id.clone(),
            })?;

        Ok((session.age(), session.idle_time(), session.game.is_over()))
    }
}

impl Default for SessionManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::action::Action;

    #[test]
    fn test_session_creation() {
        let manager = SessionManager::new();
        let session_id = "test_session".to_string();

        // Create session
        assert!(manager.create_session(session_id.clone()).is_ok());

        // Should fail to create duplicate
        assert!(matches!(
            manager.create_session(session_id.clone()),
            Err(SessionError::SessionExists { .. })
        ));

        // Should be able to get actions
        let actions = manager.get_actions(&session_id).unwrap();
        assert!(
            !actions.is_empty(),
            "New game should have available actions"
        );
    }

    #[test]
    fn test_session_not_found() {
        let manager = SessionManager::new();
        let session_id = "nonexistent".to_string();

        assert!(matches!(
            manager.get_actions(&session_id),
            Err(SessionError::SessionNotFound { .. })
        ));

        assert!(matches!(
            manager.execute_action(&session_id, Action::NextRound()),
            Err(SessionError::SessionNotFound { .. })
        ));
    }

    #[test]
    fn test_action_execution() {
        let manager = SessionManager::new();
        let session_id = "test_session".to_string();

        manager.create_session(session_id.clone()).unwrap();

        // Get available actions
        let actions = manager.get_actions(&session_id).unwrap();
        assert!(!actions.is_empty());

        // Execute first available action
        let action = actions[0].clone();
        assert!(manager.execute_action(&session_id, action).is_ok());
    }

    #[test]
    fn test_session_cleanup() {
        let manager = SessionManager::with_config(
            Config::default(),
            Duration::from_millis(1), // Very short timeout for testing
        );

        let session_id = "test_session".to_string();
        manager.create_session(session_id.clone()).unwrap();

        // Wait for session to become stale
        std::thread::sleep(Duration::from_millis(2));

        // Cleanup should remove the stale session
        let cleaned = manager.cleanup_stale_sessions();
        assert_eq!(cleaned, 1);

        // Session should no longer exist
        assert!(matches!(
            manager.get_actions(&session_id),
            Err(SessionError::SessionNotFound { .. })
        ));
    }

    #[test]
    fn test_session_metadata() {
        let manager = SessionManager::new();
        let session_id = "test_session".to_string();

        manager.create_session(session_id.clone()).unwrap();

        let (age, idle_time, is_over) = manager.get_session_info(&session_id).unwrap();

        assert!(age > Duration::ZERO);
        assert!(idle_time >= Duration::ZERO);
        assert!(!is_over); // New game shouldn't be over
    }

    #[test]
    fn test_session_list() {
        let manager = SessionManager::new();

        assert_eq!(manager.session_count(), 0);
        assert_eq!(manager.list_sessions().len(), 0);

        manager.create_session("session1".to_string()).unwrap();
        manager.create_session("session2".to_string()).unwrap();

        assert_eq!(manager.session_count(), 2);
        let sessions = manager.list_sessions();
        assert_eq!(sessions.len(), 2);
        assert!(sessions.contains(&"session1".to_string()));
        assert!(sessions.contains(&"session2".to_string()));
    }
}
