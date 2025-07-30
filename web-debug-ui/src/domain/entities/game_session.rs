//! GameSession Entity
//!
//! GameSession is the core business entity that represents a game session with all
//! its associated business rules and behavior. It encapsulates the game state,
//! action history, and lifecycle management following Domain-Driven Design principles.

use crate::domain::{
    errors::DomainError,
    interfaces::ActionResult,
    value_objects::{SessionId, ValidationResult},
    Action,
};
use crate::domain::stubs::{Config, Game, Stage};
use std::time::{Duration, Instant};

/// Configuration for creating a new game session
///
/// GameConfig provides the parameters needed to initialize a new game session
/// with appropriate business rules and constraints.
#[derive(Debug, Clone, PartialEq)]
pub struct GameConfig {
    /// Configuration for the underlying balatro-rs game
    pub game_config: Config,
    /// Maximum number of actions to keep in history
    pub max_action_history: usize,
    /// Session timeout duration
    pub session_timeout: Duration,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            game_config: Config::default(),
            max_action_history: 1000,
            session_timeout: Duration::from_secs(3600), // 1 hour
        }
    }
}

/// Core business entity representing a game session
///
/// GameSession encapsulates all the business logic and rules for managing
/// a single game session. It maintains the game state, tracks action history,
/// and enforces business constraints.
///
/// # Business Rules
///
/// - Sessions must have unique identifiers
/// - Actions must be validated before application
/// - Session expiry is based on last activity time
/// - Action history is bounded to prevent memory issues
/// - Game state consistency must be maintained
///
/// # Examples
///
/// ```ignore
/// use balatro_domain::{GameSession, GameConfig, Action};
///
/// let config = GameConfig::default();
/// let mut session = GameSession::new(config).unwrap();
///
/// // Validate an action
/// let action = Action::Play();
/// let validation = session.validate_action(&action);
/// if validation.is_valid() {
///     let result = session.apply_action(action).unwrap();
///     println!("Action applied: {:?}", result);
/// }
/// ```
#[derive(Debug, Clone)]
pub struct GameSession {
    /// Unique identifier for this session
    id: SessionId,
    /// The underlying game state
    game: Game,
    /// History of applied actions (bounded)
    action_history: Vec<Action>,
    /// Maximum number of actions to keep in history
    max_action_history: usize,
    /// When this session was created
    created_at: Instant,
    /// When the last action was applied
    last_action_at: Instant,
    /// Session timeout duration
    session_timeout: Duration,
}

impl GameSession {
    /// Create a new game session with the given configuration
    ///
    /// # Arguments
    ///
    /// * `config` - Configuration parameters for the session
    ///
    /// # Returns
    ///
    /// * `Ok(GameSession)` - Successfully created session
    /// * `Err(DomainError)` - Session creation failed
    ///
    /// # Business Rules
    ///
    /// - Session must have a valid unique identifier
    /// - Game must be initialized successfully
    /// - Configuration parameters must be valid
    ///
    /// # Errors
    ///
    /// * `DomainError::SessionCreationFailed` - Invalid configuration or initialization failure
    pub fn new(config: GameConfig) -> Result<Self, DomainError> {
        // Validate configuration
        if config.max_action_history == 0 {
            return Err(DomainError::session_creation_failed(
                "max_action_history must be greater than 0",
            ));
        }

        if config.session_timeout.is_zero() {
            return Err(DomainError::session_creation_failed(
                "session_timeout must be greater than 0",
            ));
        }

        // Create game instance
        let game = Game::new(config.game_config);
        let now = Instant::now();

        Ok(Self {
            id: SessionId::new(),
            game,
            action_history: Vec::new(),
            max_action_history: config.max_action_history,
            created_at: now,
            last_action_at: now,
            session_timeout: config.session_timeout,
        })
    }

    /// Get the session ID
    pub fn id(&self) -> &SessionId {
        &self.id
    }

    /// Get a reference to the current game state
    pub fn game(&self) -> &Game {
        &self.game
    }

    /// Get the action history
    pub fn action_history(&self) -> &[Action] {
        &self.action_history
    }

    /// Get when this session was created
    pub fn created_at(&self) -> Instant {
        self.created_at
    }

    /// Get when the last action was applied
    pub fn last_action_at(&self) -> Instant {
        self.last_action_at
    }

    /// Validate an action against the current game state
    ///
    /// # Arguments
    ///
    /// * `action` - The action to validate
    ///
    /// # Returns
    ///
    /// * `ValidationResult::Valid` - Action is valid for current state
    /// * `ValidationResult::Invalid` - Action cannot be applied
    ///
    /// # Business Rules
    ///
    /// - Action must be valid for current game stage
    /// - Game state must be consistent
    /// - Session expiry is checked separately in apply_action
    pub fn validate_action(&self, action: &Action) -> ValidationResult {
        // Check if game is in a valid state
        if self.game.stage == Stage::End {
            return ValidationResult::invalid("Game has ended");
        }

        // Use balatro-rs game validation if available
        // For now, we'll implement basic validation
        match action {
            Action::Play() => {
                if self.game.available.selected.is_empty() {
                    ValidationResult::invalid("No cards selected for play")
                } else {
                    ValidationResult::valid()
                }
            }
            Action::Discard() => {
                if self.game.available.selected.is_empty() {
                    ValidationResult::invalid("No cards selected for discard")
                } else {
                    ValidationResult::valid()
                }
            }
            Action::SelectCard(_) => ValidationResult::valid(),
            Action::NextRound() => {
                match self.game.stage {
                    Stage::PostBlind => ValidationResult::valid(),
                    _ => ValidationResult::invalid("Can only advance round from PostBlind stage"),
                }
            }
            _ => ValidationResult::valid(), // For now, allow other actions
        }
    }

    /// Apply an action to the game session
    ///
    /// # Arguments
    ///
    /// * `action` - The action to apply
    ///
    /// # Returns
    ///
    /// * `Ok(ActionResult)` - Action was successfully applied
    /// * `Err(DomainError)` - Action could not be applied
    ///
    /// # Business Rules
    ///
    /// - Action must be valid for current state
    /// - Game state must remain consistent after application
    /// - Action history must be updated
    /// - Last action time must be updated
    ///
    /// # Errors
    ///
    /// * `DomainError::InvalidAction` - Action is not valid for current state
    /// * `DomainError::SessionExpired` - Session has expired
    /// * `DomainError::StateInconsistency` - Game state became inconsistent
    pub fn apply_action(&mut self, action: Action) -> Result<ActionResult, DomainError> {
        // Validate action first
        let validation = self.validate_action(&action);
        if let ValidationResult::Invalid(error) = validation {
            return Err(DomainError::invalid_action(error.reason()));
        }

        // Check session expiry
        if self.is_expired() {
            return Err(DomainError::session_expired(&self.id));
        }

        // Store previous state for comparison
        let previous_score = self.game.score;
        let previous_money = self.game.money;

        // Apply action to game
        let game_result = self.game.handle_action(action.clone());
        
        // Handle game engine result
        match game_result {
            Ok(_) => {
                // Update session state
                self.last_action_at = Instant::now();
                self.add_to_history(action.clone());

                // Calculate deltas
                let score_delta = if self.game.score != previous_score {
                    Some(self.game.score - previous_score)
                } else {
                    None
                };

                let money_delta = if self.game.money != previous_money {
                    Some(self.game.money - previous_money)
                } else {
                    None
                };

                Ok(ActionResult::success_with_deltas(action, score_delta, money_delta))
            }
            Err(game_error) => {
                // Convert game error to domain error
                Err(DomainError::invalid_action(format!("Game engine error: {}", game_error)))
            }
        }
    }

    /// Check if the session has expired
    ///
    /// # Arguments
    ///
    /// * `ttl` - Optional custom time-to-live duration. If None, uses session's configured timeout
    ///
    /// # Returns
    ///
    /// * `true` - Session has expired
    /// * `false` - Session is still active
    ///
    /// # Business Rules
    ///
    /// - Expiry is based on last activity time
    /// - Sessions with no activity beyond TTL are expired
    pub fn is_expired_with_ttl(&self, ttl: Duration) -> bool {
        Instant::now().duration_since(self.last_action_at) > ttl
    }

    /// Check if the session has expired (using configured timeout)
    pub fn is_expired(&self) -> bool {
        self.is_expired_with_ttl(self.session_timeout)
    }

    /// Get session activity duration
    pub fn activity_duration(&self) -> Duration {
        self.last_action_at.duration_since(self.created_at)
    }

    /// Get time since last action
    pub fn time_since_last_action(&self) -> Duration {
        Instant::now().duration_since(self.last_action_at)
    }

    /// Get the number of actions applied to this session
    pub fn action_count(&self) -> usize {
        self.action_history.len()
    }

    /// Check if the game is finished
    pub fn is_game_finished(&self) -> bool {
        matches!(self.game.stage, Stage::End)
    }

    /// Get current game statistics
    pub fn game_stats(&self) -> GameStats {
        GameStats {
            score: self.game.score,
            money: self.game.money,
            ante: self.game.ante_current.0,
            round: self.game.round,
            stage: format!("{:?}", self.game.stage),
        }
    }

    // Private helper methods

    /// Add an action to the history, maintaining the maximum size constraint
    fn add_to_history(&mut self, action: Action) {
        self.action_history.push(action);
        
        // Maintain bounded history
        while self.action_history.len() > self.max_action_history {
            self.action_history.remove(0);
        }
    }
}

/// Game statistics snapshot
#[derive(Debug, Clone, PartialEq)]
pub struct GameStats {
    pub score: f64,
    pub money: f64,
    pub ante: u32,
    pub round: f64,
    pub stage: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::stubs::Action;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn game_config_default_values() {
        let config = GameConfig::default();
        
        assert_eq!(config.max_action_history, 1000);
        assert_eq!(config.session_timeout, Duration::from_secs(3600));
    }

    #[test]
    fn game_session_creation_success() {
        let config = GameConfig::default();
        let session = GameSession::new(config).unwrap();
        
        assert!(!session.id().to_string().is_empty());
        assert!(session.action_history().is_empty());
        assert_eq!(session.action_count(), 0);
        assert!(!session.is_expired());
        assert!(!session.is_game_finished());
    }

    #[test]
    fn game_session_creation_fails_with_zero_max_history() {
        let mut config = GameConfig::default();
        config.max_action_history = 0;
        
        let result = GameSession::new(config);
        assert!(result.is_err());
        
        match result.unwrap_err() {
            DomainError::SessionCreationFailed { reason } => {
                assert!(reason.contains("max_action_history"));
            }
            _ => panic!("Expected SessionCreationFailed error"),
        }
    }

    #[test]
    fn game_session_creation_fails_with_zero_timeout() {
        let mut config = GameConfig::default();
        config.session_timeout = Duration::from_secs(0);
        
        let result = GameSession::new(config);
        assert!(result.is_err());
        
        match result.unwrap_err() {
            DomainError::SessionCreationFailed { reason } => {
                assert!(reason.contains("session_timeout"));
            }
            _ => panic!("Expected SessionCreationFailed error"),
        }
    }

    #[test]
    fn session_has_unique_ids() {
        let config = GameConfig::default();
        let session1 = GameSession::new(config.clone()).unwrap();
        let session2 = GameSession::new(config).unwrap();
        
        assert_ne!(session1.id(), session2.id());
    }

    #[test]
    fn session_tracks_creation_time() {
        let before = Instant::now();
        let session = GameSession::new(GameConfig::default()).unwrap();
        let after = Instant::now();
        
        assert!(session.created_at() >= before);
        assert!(session.created_at() <= after);
        assert_eq!(session.created_at(), session.last_action_at());
    }

    #[test]
    fn validate_action_rejects_play_with_no_cards() {
        let session = GameSession::new(GameConfig::default()).unwrap();
        let action = Action::Play();
        
        let validation = session.validate_action(&action);
        assert!(validation.is_invalid());
        
        if let ValidationResult::Invalid(error) = validation {
            assert!(error.reason().contains("No cards selected"));
        }
    }

    #[test]
    fn validate_action_rejects_discard_with_no_cards() {
        let session = GameSession::new(GameConfig::default()).unwrap();
        let action = Action::Discard();
        
        let validation = session.validate_action(&action);
        assert!(validation.is_invalid());
        
        if let ValidationResult::Invalid(error) = validation {
            assert!(error.reason().contains("No cards selected"));
        }
    }

    #[test]
    fn validate_action_allows_card_selection() {
        let session = GameSession::new(GameConfig::default()).unwrap();
        
        // Get a card from the available cards to select
        if let Some(card) = session.game().available.cards.first() {
            let action = Action::SelectCard(*card);
            let validation = session.validate_action(&action);
            assert!(validation.is_valid());
        }
    }

    #[test] 
    fn session_expiry_based_on_custom_ttl() {
        let mut config = GameConfig::default();
        config.session_timeout = Duration::from_secs(10);
        let session = GameSession::new(config).unwrap();
        
        // Should not be expired with a long TTL
        assert!(!session.is_expired_with_ttl(Duration::from_secs(3600)));
        
        // Should be expired with a very short TTL
        assert!(session.is_expired_with_ttl(Duration::from_nanos(1)));
    }

    #[test]
    fn session_activity_duration_tracking() {
        let session = GameSession::new(GameConfig::default()).unwrap();
        
        // Initially, activity duration should be zero (or very small)
        let duration = session.activity_duration();
        assert!(duration < Duration::from_millis(100));
        
        // Time since last action should be small
        let time_since = session.time_since_last_action();
        assert!(time_since < Duration::from_millis(100));
    }

    #[test]
    fn action_history_is_bounded() {
        let mut config = GameConfig::default();
        config.max_action_history = 2; // Very small for testing
        
        let mut session = GameSession::new(config).unwrap();
        
        // Add more actions than the limit
        for i in 0..5 {
            if let Some(card) = session.game().available.cards.get(i % session.game().available.cards.len()) {
                let action = Action::SelectCard(*card);
                // We expect this might fail due to game rules, but we're testing history bounds
                let _ = session.apply_action(action);
            }
        }
        
        // History should be bounded
        assert!(session.action_count() <= 2);
    }

    #[test]
    fn game_stats_reflect_current_state() {
        let session = GameSession::new(GameConfig::default()).unwrap();
        let stats = session.game_stats();
        
        assert_eq!(stats.score, session.game().score);
        assert_eq!(stats.money, session.game().money);
        assert_eq!(stats.ante, session.game().ante_current.0);
        assert_eq!(stats.round, session.game().round);
    }

    #[test]
    fn session_clone_creates_independent_copy() {
        let session1 = GameSession::new(GameConfig::default()).unwrap();
        let mut session2 = session1.clone();
        
        // Sessions should have same ID (they're clones)
        assert_eq!(session1.id(), session2.id());
        
        // But they should be independent - modifying one doesn't affect the other
        if let Some(card) = session2.game().available.cards.first() {
            let action = Action::SelectCard(*card);
            let _ = session2.apply_action(action);
        }
        
        // Original session should be unchanged
        assert_eq!(session1.action_count(), 0);
    }

    #[test]
    fn apply_action_updates_last_action_time() {
        let mut session = GameSession::new(GameConfig::default()).unwrap();
        let initial_time = session.last_action_at();
        
        // Wait a bit to ensure time difference
        thread::sleep(Duration::from_millis(10));
        
        // Apply a valid action
        if let Some(card) = session.game().available.cards.first() {
            let action = Action::SelectCard(*card);
            let _ = session.apply_action(action);
            
            assert!(session.last_action_at() > initial_time);
        }
    }

    #[test]
    fn apply_action_fails_for_invalid_action() {
        let mut session = GameSession::new(GameConfig::default()).unwrap();
        let action = Action::Play(); // Invalid because no cards selected
        
        let result = session.apply_action(action);
        assert!(result.is_err());
        
        match result.unwrap_err() {
            DomainError::InvalidAction { reason } => {
                assert!(reason.contains("No cards selected"));
            }
            _ => panic!("Expected InvalidAction error"),
        }
    }

    #[test]
    fn expired_session_rejects_actions() {
        let mut config = GameConfig::default();
        config.session_timeout = Duration::from_nanos(1); // Immediate expiry
        
        let mut session = GameSession::new(config).unwrap();
        
        // Wait for expiry
        thread::sleep(Duration::from_millis(1));
        
        // Any action should be rejected
        if let Some(card) = session.game().available.cards.first() {
            let action = Action::SelectCard(*card);
            let result = session.apply_action(action);
            
            assert!(result.is_err());
            match result.unwrap_err() {
                DomainError::SessionExpired { .. } => {
                    // Expected
                }
                _ => panic!("Expected SessionExpired error"),
            }
        }
    }

    #[test]
    fn successful_action_returns_appropriate_result() {
        let mut session = GameSession::new(GameConfig::default()).unwrap();
        
        if let Some(card) = session.game().available.cards.first() {
            let action = Action::SelectCard(*card);
            let result = session.apply_action(action.clone()).unwrap();
            
            assert_eq!(result.action, action);
            assert!(result.success);
            // Score/money deltas might be None for card selection
        }
    }
}