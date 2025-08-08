//! # Repository Traits
//!
//! This module defines trait boundaries for data access following the Repository pattern.
//! These traits provide clean abstractions over data persistence, allowing the domain
//! layer to remain independent of infrastructure concerns.
//!
//! ## Design Principles
//!
//! - **Interface Segregation**: Small, focused interfaces for specific needs
//! - **Dependency Inversion**: Domain depends on abstractions, not concrete implementations
//! - **Single Responsibility**: Each repository handles one aggregate root

use crate::action::Action;
#[allow(unused_imports)] // Used in test module
use crate::domain::DomainError;
use crate::domain::DomainResult;
use crate::game::Game;
use std::future::Future;
use std::pin::Pin;

/// Base repository trait with common operations
///
/// This trait defines the minimum contract that all repositories must fulfill.
/// Following ISP, specific repositories can extend this with their own methods.
pub trait Repository: Send + Sync {
    /// Check if the repository is healthy and accessible
    fn health_check(&self) -> DomainResult<()>;

    /// Get repository metrics for monitoring
    fn metrics(&self) -> RepositoryMetrics {
        RepositoryMetrics::default()
    }
}

/// Repository for game state persistence
///
/// This trait defines how game state is loaded and saved.
/// Implementations might use files, databases, or in-memory storage.
pub trait GameRepository: Repository {
    /// Find a game by its unique identifier
    fn find_by_id(&self, id: &str) -> DomainResult<Game>;

    /// Save or update a game
    fn save(&mut self, id: &str, game: &Game) -> DomainResult<()>;

    /// Delete a game by its identifier
    fn delete(&mut self, id: &str) -> DomainResult<()>;

    /// Check if a game exists
    fn exists(&self, id: &str) -> DomainResult<bool>;

    /// List all game identifiers with pagination
    fn list_ids(&self, offset: usize, limit: usize) -> DomainResult<Vec<String>>;
}

/// Repository for game session management
///
/// This trait handles session-specific data that may be separate from game state.
pub trait SessionRepository: Repository {
    /// Create a new session
    fn create_session(&mut self, session_id: &str, game_id: &str) -> DomainResult<()>;

    /// Get session metadata
    fn get_session(&self, session_id: &str) -> DomainResult<SessionInfo>;

    /// Update session timestamp
    fn touch_session(&mut self, session_id: &str) -> DomainResult<()>;

    /// End a session
    fn end_session(&mut self, session_id: &str) -> DomainResult<()>;

    /// Find active sessions for a game
    fn find_active_sessions(&self, game_id: &str) -> DomainResult<Vec<SessionInfo>>;
}

/// Repository for action history and replay
///
/// This trait manages the history of actions for replay and analysis.
pub trait ActionHistoryRepository: Repository {
    /// Record an action
    fn record_action(&mut self, game_id: &str, action: &Action) -> DomainResult<()>;

    /// Get action history for a game
    fn get_history(&self, game_id: &str) -> DomainResult<Vec<Action>>;

    /// Clear action history for a game
    fn clear_history(&mut self, game_id: &str) -> DomainResult<()>;

    /// Get action count for statistics
    fn get_action_count(&self, game_id: &str) -> DomainResult<usize>;
}

/// Async repository trait for non-blocking operations
///
/// This trait provides async versions of repository methods for high-performance scenarios.
pub trait AsyncRepository: Send + Sync {
    /// Async health check
    fn health_check_async(&self) -> Pin<Box<dyn Future<Output = DomainResult<()>> + Send + '_>>;
}

/// Session information structure
#[derive(Debug, Clone)]
pub struct SessionInfo {
    pub session_id: String,
    pub game_id: String,
    pub created_at: u64,
    pub last_activity: u64,
    pub is_active: bool,
}

/// Repository metrics for monitoring
#[derive(Debug, Default, Clone)]
pub struct RepositoryMetrics {
    pub total_operations: u64,
    pub failed_operations: u64,
    pub average_latency_ms: f64,
    pub last_error: Option<String>,
}

/// Unit of Work pattern for transactional operations
///
/// This trait ensures multiple repository operations can be grouped into atomic transactions.
pub trait UnitOfWork: Send + Sync {
    /// Begin a new transaction
    fn begin(&mut self) -> DomainResult<()>;

    /// Commit the current transaction
    fn commit(&mut self) -> DomainResult<()>;

    /// Rollback the current transaction
    fn rollback(&mut self) -> DomainResult<()>;

    /// Check if a transaction is active
    fn in_transaction(&self) -> bool;
}

/// Factory trait for creating repositories
///
/// This trait follows the Abstract Factory pattern for repository creation.
pub trait RepositoryFactory: Send + Sync {
    /// Create a game repository
    fn create_game_repository(&self) -> DomainResult<Box<dyn GameRepository>>;

    /// Create a session repository
    fn create_session_repository(&self) -> DomainResult<Box<dyn SessionRepository>>;

    /// Create an action history repository
    fn create_action_history_repository(&self) -> DomainResult<Box<dyn ActionHistoryRepository>>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::Cow;

    /// Mock implementation for testing - using simple state tracking
    struct MockGameRepository {
        game_exists: std::collections::HashSet<String>,
    }

    impl Repository for MockGameRepository {
        fn health_check(&self) -> DomainResult<()> {
            Ok(())
        }
    }

    impl GameRepository for MockGameRepository {
        fn find_by_id(&self, id: &str) -> DomainResult<Game> {
            if self.game_exists.contains(id) {
                Ok(Game::default())
            } else {
                Err(DomainError::NotFound(Cow::Owned(format!(
                    "Game {id} not found"
                ))))
            }
        }

        fn save(&mut self, id: &str, _game: &Game) -> DomainResult<()> {
            self.game_exists.insert(id.to_string());
            Ok(())
        }

        fn delete(&mut self, id: &str) -> DomainResult<()> {
            if self.game_exists.remove(id) {
                Ok(())
            } else {
                Err(DomainError::NotFound(Cow::Owned(format!(
                    "Game {id} not found"
                ))))
            }
        }

        fn exists(&self, id: &str) -> DomainResult<bool> {
            Ok(self.game_exists.contains(id))
        }

        fn list_ids(&self, offset: usize, limit: usize) -> DomainResult<Vec<String>> {
            Ok(self
                .game_exists
                .iter()
                .skip(offset)
                .take(limit)
                .cloned()
                .collect())
        }
    }

    #[test]
    fn test_mock_repository() {
        let mut repo = MockGameRepository {
            game_exists: std::collections::HashSet::new(),
        };

        assert!(repo.health_check().is_ok());
        assert!(!repo.exists("test").unwrap());

        let game = Game::default();
        repo.save("test", &game).unwrap();
        assert!(repo.exists("test").unwrap());

        // Test that we can load a game (returns default game in mock)
        let loaded = repo.find_by_id("test");
        assert!(loaded.is_ok());

        // Test deletion
        assert!(repo.delete("test").is_ok());
        assert!(!repo.exists("test").unwrap());
    }
}
