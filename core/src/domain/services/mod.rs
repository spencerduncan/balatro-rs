//! # Service Layer
//!
//! This module implements domain services that orchestrate business operations.
//! Services encapsulate business logic that doesn't naturally fit within entities
//! or value objects, following Domain-Driven Design principles.
//!
//! ## Service Categories
//!
//! - **Application Services**: Orchestrate use cases and workflow
//! - **Domain Services**: Encapsulate core business logic
//! - **Infrastructure Services**: Handle technical concerns

use crate::action::Action;
use crate::domain::errors::ErrorContext;
use crate::domain::repositories::{ActionHistoryRepository, GameRepository, SessionRepository};
use crate::domain::{DomainError, DomainResult};
use crate::game::Game;
use std::borrow::Cow;
use std::sync::{Arc, RwLock};

/// Base trait for all domain services
///
/// Following the Dependency Inversion Principle, services depend on abstractions.
pub trait DomainService: Send + Sync {
    /// Get service name for logging and monitoring
    fn name(&self) -> &str;

    /// Check if the service is healthy
    fn health_check(&self) -> DomainResult<()> {
        Ok(())
    }
}

/// Game orchestration service
///
/// This service coordinates game operations across multiple repositories
/// and ensures business rules are enforced consistently.
pub struct GameService<G: GameRepository, S: SessionRepository, A: ActionHistoryRepository> {
    game_repo: Arc<RwLock<G>>,
    session_repo: Arc<RwLock<S>>,
    history_repo: Arc<RwLock<A>>,
}

impl<G, S, A> GameService<G, S, A>
where
    G: GameRepository,
    S: SessionRepository,
    A: ActionHistoryRepository,
{
    /// Create a new game service with dependency injection
    pub fn new(
        game_repo: Arc<RwLock<G>>,
        session_repo: Arc<RwLock<S>>,
        history_repo: Arc<RwLock<A>>,
    ) -> Self {
        Self {
            game_repo,
            session_repo,
            history_repo,
        }
    }

    /// Create a new game with session
    pub fn create_game(&self, game_id: &str, session_id: &str) -> DomainResult<Game> {
        // Check if game already exists
        {
            let game_repo = self.game_repo.read().map_err(|_| {
                DomainError::ServiceError(Cow::Borrowed(
                    "Failed to acquire read lock on game repository",
                ))
            })?;
            if game_repo.exists(game_id)? {
                return Err(DomainError::ValidationFailed(
                    ErrorContext::new("CreateGame")
                        .with_entity("Game")
                        .with_detail(format!("Game {game_id} already exists"))
                        .build(),
                ));
            }
        }

        // Create new game
        let mut game = Game::default();
        game.start();

        // Save game state
        {
            let mut game_repo = self.game_repo.write().map_err(|_| {
                DomainError::ServiceError(Cow::Borrowed(
                    "Failed to acquire write lock on game repository",
                ))
            })?;
            game_repo.save(game_id, &game)?;
        }

        // Create session
        {
            let mut session_repo = self.session_repo.write().map_err(|_| {
                DomainError::ServiceError(Cow::Borrowed(
                    "Failed to acquire write lock on session repository",
                ))
            })?;
            session_repo.create_session(session_id, game_id)?;
        }

        Ok(game)
    }

    /// Execute an action in a game
    pub fn execute_action(
        &self,
        game_id: &str,
        session_id: &str,
        action: Action,
    ) -> DomainResult<()> {
        // Validate session
        let session = {
            let session_repo = self.session_repo.read().map_err(|_| {
                DomainError::ServiceError(Cow::Borrowed(
                    "Failed to acquire read lock on session repository",
                ))
            })?;
            session_repo.get_session(session_id)?
        };

        if session.game_id != game_id {
            return Err(DomainError::ValidationFailed(
                ErrorContext::new("ExecuteAction")
                    .with_detail("Session does not match game")
                    .build(),
            ));
        }

        // Load game
        let mut game = {
            let game_repo = self.game_repo.read().map_err(|_| {
                DomainError::ServiceError(Cow::Borrowed(
                    "Failed to acquire read lock on game repository",
                ))
            })?;
            game_repo.find_by_id(game_id)?
        };

        // Validate action
        if !game.gen_actions().any(|a| a == action) {
            return Err(DomainError::InvalidState(
                ErrorContext::new("ExecuteAction")
                    .with_entity("Game")
                    .with_detail("Action not valid in current state")
                    .build(),
            ));
        }

        // Execute action
        game.handle_action(action.clone()).map_err(|e| {
            DomainError::ServiceError(Cow::Owned(format!("Failed to execute action: {e:?}")))
        })?;

        // Save updated game state
        {
            let mut game_repo = self.game_repo.write().map_err(|_| {
                DomainError::ServiceError(Cow::Borrowed(
                    "Failed to acquire write lock on game repository",
                ))
            })?;
            game_repo.save(game_id, &game)?;
        }

        // Record action in history
        {
            let mut history_repo = self.history_repo.write().map_err(|_| {
                DomainError::ServiceError(Cow::Borrowed(
                    "Failed to acquire write lock on history repository",
                ))
            })?;
            history_repo.record_action(game_id, &action)?;
        }

        // Update session activity
        {
            let mut session_repo = self.session_repo.write().map_err(|_| {
                DomainError::ServiceError(Cow::Borrowed(
                    "Failed to acquire write lock on session repository",
                ))
            })?;
            session_repo.touch_session(session_id)?;
        }

        Ok(())
    }

    /// Get game state with validation
    pub fn get_game(&self, game_id: &str, session_id: &str) -> DomainResult<Game> {
        // Validate session
        let session = {
            let session_repo = self.session_repo.read().map_err(|_| {
                DomainError::ServiceError(Cow::Borrowed(
                    "Failed to acquire read lock on session repository",
                ))
            })?;
            session_repo.get_session(session_id)?
        };

        if session.game_id != game_id {
            return Err(DomainError::Unauthorized(
                ErrorContext::new("GetGame")
                    .with_detail("Session does not have access to game")
                    .build(),
            ));
        }

        // Load and return game
        let game_repo = self.game_repo.read().map_err(|_| {
            DomainError::ServiceError(Cow::Borrowed(
                "Failed to acquire read lock on game repository",
            ))
        })?;
        game_repo.find_by_id(game_id)
    }
}

impl<G, S, A> DomainService for GameService<G, S, A>
where
    G: GameRepository,
    S: SessionRepository,
    A: ActionHistoryRepository,
{
    fn name(&self) -> &str {
        "GameService"
    }

    fn health_check(&self) -> DomainResult<()> {
        {
            let game_repo = self.game_repo.read().map_err(|_| {
                DomainError::ServiceError(Cow::Borrowed(
                    "Failed to acquire read lock on game repository",
                ))
            })?;
            game_repo.health_check()?;
        }
        {
            let session_repo = self.session_repo.read().map_err(|_| {
                DomainError::ServiceError(Cow::Borrowed(
                    "Failed to acquire read lock on session repository",
                ))
            })?;
            session_repo.health_check()?;
        }
        {
            let history_repo = self.history_repo.read().map_err(|_| {
                DomainError::ServiceError(Cow::Borrowed(
                    "Failed to acquire read lock on history repository",
                ))
            })?;
            history_repo.health_check()?;
        }
        Ok(())
    }
}

/// Validation service for business rules
///
/// This service encapsulates complex validation logic that spans multiple aggregates.
pub struct ValidationService {
    max_session_duration_ms: u64,
    max_actions_per_game: usize,
}

impl ValidationService {
    /// Create a new validation service
    pub fn new(max_session_duration_ms: u64, max_actions_per_game: usize) -> Self {
        Self {
            max_session_duration_ms,
            max_actions_per_game,
        }
    }

    /// Validate session duration
    pub fn validate_session_duration(
        &self,
        start_time: u64,
        current_time: u64,
    ) -> DomainResult<()> {
        let duration = current_time.saturating_sub(start_time);
        if duration > self.max_session_duration_ms {
            return Err(DomainError::ValidationFailed(
                ErrorContext::new("ValidateSession")
                    .with_detail("Session exceeded maximum duration")
                    .build(),
            ));
        }
        Ok(())
    }

    /// Validate action count
    pub fn validate_action_count(&self, count: usize) -> DomainResult<()> {
        if count > self.max_actions_per_game {
            return Err(DomainError::ValidationFailed(
                ErrorContext::new("ValidateActions")
                    .with_detail(format!(
                        "Exceeded maximum of {} actions",
                        self.max_actions_per_game
                    ))
                    .build(),
            ));
        }
        Ok(())
    }
}

impl DomainService for ValidationService {
    fn name(&self) -> &str {
        "ValidationService"
    }
}

/// Service locator for dependency injection
///
/// This trait provides a way to locate and create services at runtime.
pub trait ServiceLocator: Send + Sync {
    /// Get a domain service by type
    fn get_service<T: DomainService>(&self) -> DomainResult<Arc<T>>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_service() {
        let service = ValidationService::new(3600000, 1000);

        // Test valid session duration
        assert!(service.validate_session_duration(0, 3600000).is_ok());

        // Test invalid session duration
        assert!(service.validate_session_duration(0, 3600001).is_err());

        // Test valid action count
        assert!(service.validate_action_count(1000).is_ok());

        // Test invalid action count
        assert!(service.validate_action_count(1001).is_err());
    }

    #[test]
    fn test_service_name() {
        let service = ValidationService::new(3600000, 1000);
        assert_eq!(service.name(), "ValidationService");
    }
}
