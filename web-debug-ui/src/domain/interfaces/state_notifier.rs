//! StateNotifier Interface
//!
//! Defines the contract for notifying external systems about game state changes.
//! This interface enables the domain layer to publish events without coupling
//! to specific notification mechanisms (WebSocket, SSE, message queues, etc.).

use crate::domain::stubs::Game;
use crate::domain::{value_objects::SessionId, Action};

/// Action result information for notifications
///
/// ActionResult contains information about the outcome of applying an action
/// to a game session, enabling external systems to react appropriately.
#[derive(Debug, Clone, PartialEq)]
pub struct ActionResult {
    /// The action that was applied
    pub action: Action,
    /// Whether the action was successfully applied
    pub success: bool,
    /// Human-readable description of the result
    pub message: Option<String>,
    /// Score delta from this action (if applicable)
    pub score_delta: Option<f64>,
    /// Money delta from this action (if applicable)
    pub money_delta: Option<f64>,
}

impl ActionResult {
    /// Create a successful action result
    pub fn success(action: Action) -> Self {
        Self {
            action,
            success: true,
            message: None,
            score_delta: None,
            money_delta: None,
        }
    }

    /// Create a successful action result with message
    pub fn success_with_message<S: Into<String>>(action: Action, message: S) -> Self {
        Self {
            action,
            success: true,
            message: Some(message.into()),
            score_delta: None,
            money_delta: None,
        }
    }

    /// Create a successful action result with score and money deltas
    pub fn success_with_deltas(
        action: Action,
        score_delta: Option<f64>,
        money_delta: Option<f64>,
    ) -> Self {
        Self {
            action,
            success: true,
            message: None,
            score_delta,
            money_delta,
        }
    }

    /// Create a failed action result
    pub fn failure<S: Into<String>>(action: Action, message: S) -> Self {
        Self {
            action,
            success: false,
            message: Some(message.into()),
            score_delta: None,
            money_delta: None,
        }
    }
}

/// Notification interface for game state changes
///
/// StateNotifier defines the contract that notification implementations must fulfill
/// to receive updates about game state changes. This enables the domain layer to
/// publish events without coupling to specific delivery mechanisms.
///
/// # Design Principles
///
/// - **Single Responsibility**: Only handles event notification
/// - **Interface Segregation**: Focused interface for state notifications
/// - **Dependency Inversion**: Domain depends on this abstraction
/// - **Event-Driven**: Supports reactive programming patterns
///
/// # Examples
///
/// ```ignore
/// use balatro_domain::{StateNotifier, SessionId, ActionResult};
/// use balatro_rs::Game;
///
/// struct WebSocketNotifier;
///
/// impl StateNotifier for WebSocketNotifier {
///     fn notify_state_change(&self, session_id: &SessionId, state: &Game) {
///         // Send state update via WebSocket
///     }
///
///     fn notify_action_result(&self, session_id: &SessionId, result: &ActionResult) {
///         // Send action result via WebSocket
///     }
/// }
/// ```
pub trait StateNotifier: Send + Sync {
    /// Notify external systems of a game state change
    ///
    /// Called whenever the game state changes in a way that external systems
    /// (like web clients) should be aware of.
    ///
    /// # Arguments
    ///
    /// * `session_id` - The unique identifier of the session that changed
    /// * `state` - The current game state after the change
    ///
    /// # Implementation Notes
    ///
    /// - Implementations should be non-blocking to avoid impacting game performance
    /// - Failed notifications should not cause domain operations to fail
    /// - Consider implementing retry logic for critical notifications
    fn notify_state_change(&self, session_id: &SessionId, state: &Game);

    /// Notify external systems of an action result
    ///
    /// Called after an action is applied (successfully or unsuccessfully)
    /// to provide feedback about the operation.
    ///
    /// # Arguments
    ///
    /// * `session_id` - The unique identifier of the session
    /// * `result` - Information about the action that was applied
    ///
    /// # Implementation Notes
    ///
    /// - Should provide enough information for clients to update their UI
    /// - Failed actions should still be notified so clients can show errors
    /// - Consider rate limiting for high-frequency actions
    fn notify_action_result(&self, session_id: &SessionId, result: &ActionResult);

    /// Notify external systems that a session started
    ///
    /// Called when a new game session is created and becomes available.
    ///
    /// # Arguments
    ///
    /// * `session_id` - The unique identifier of the new session
    /// * `initial_state` - The initial game state
    fn notify_session_started(&self, session_id: &SessionId, initial_state: &Game) {
        // Default implementation delegates to state change
        self.notify_state_change(session_id, initial_state);
    }

    /// Notify external systems that a session ended
    ///
    /// Called when a game session is completed or terminated.
    ///
    /// # Arguments
    ///
    /// * `session_id` - The unique identifier of the ended session
    /// * `final_state` - The final game state (if available)
    fn notify_session_ended(&self, session_id: &SessionId, final_state: Option<&Game>);

    /// Notify external systems of an error condition
    ///
    /// Called when domain operations encounter errors that external systems
    /// should be aware of.
    ///
    /// # Arguments
    ///
    /// * `session_id` - The unique identifier of the affected session (if applicable)
    /// * `error_message` - Human-readable error description
    /// * `error_code` - Optional machine-readable error code
    fn notify_error(
        &self,
        session_id: Option<&SessionId>,
        error_message: &str,
        error_code: Option<&str>,
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::stubs::{Action, Config, Game};
    use crate::domain::SessionId;
    use std::sync::{Arc, Mutex};

    /// Event log entry for testing
    #[derive(Debug, Clone, PartialEq)]
    enum Event {
        StateChange {
            session_id: SessionId,
            stage: String, // We'll use stage name for simplified testing
        },
        ActionResult {
            session_id: SessionId,
            action: String, // Simplified action representation
            success: bool,
        },
        SessionStarted {
            session_id: SessionId,
        },
        SessionEnded {
            session_id: SessionId,
            had_final_state: bool,
        },
        Error {
            session_id: Option<SessionId>,
            message: String,
            code: Option<String>,
        },
    }

    /// Mock implementation of StateNotifier for testing
    #[derive(Debug, Default)]
    struct MockStateNotifier {
        events: Arc<Mutex<Vec<Event>>>,
    }

    impl MockStateNotifier {
        fn new() -> Self {
            Self::default()
        }

        fn get_events(&self) -> Vec<Event> {
            self.events.lock().unwrap().clone()
        }

        fn clear_events(&self) {
            self.events.lock().unwrap().clear();
        }

        fn event_count(&self) -> usize {
            self.events.lock().unwrap().len()
        }
    }

    impl StateNotifier for MockStateNotifier {
        fn notify_state_change(&self, session_id: &SessionId, state: &Game) {
            let mut events = self.events.lock().unwrap();
            events.push(Event::StateChange {
                session_id: session_id.clone(),
                stage: format!("{:?}", state.stage), // Simplified representation
            });
        }

        fn notify_action_result(&self, session_id: &SessionId, result: &ActionResult) {
            let mut events = self.events.lock().unwrap();
            events.push(Event::ActionResult {
                session_id: session_id.clone(),
                action: format!("{:?}", result.action), // Simplified representation
                success: result.success,
            });
        }

        fn notify_session_started(&self, session_id: &SessionId, _initial_state: &Game) {
            let mut events = self.events.lock().unwrap();
            events.push(Event::SessionStarted {
                session_id: session_id.clone(),
            });
        }

        fn notify_session_ended(&self, session_id: &SessionId, final_state: Option<&Game>) {
            let mut events = self.events.lock().unwrap();
            events.push(Event::SessionEnded {
                session_id: session_id.clone(),
                had_final_state: final_state.is_some(),
            });
        }

        fn notify_error(
            &self,
            session_id: Option<&SessionId>,
            error_message: &str,
            error_code: Option<&str>,
        ) {
            let mut events = self.events.lock().unwrap();
            events.push(Event::Error {
                session_id: session_id.cloned(),
                message: error_message.to_string(),
                code: error_code.map(|s| s.to_string()),
            });
        }
    }

    #[test]
    fn action_result_success_creation() {
        let action = Action::Play();
        let result = ActionResult::success(action.clone());

        assert_eq!(result.action, action);
        assert!(result.success);
        assert!(result.message.is_none());
        assert!(result.score_delta.is_none());
        assert!(result.money_delta.is_none());
    }

    #[test]
    fn action_result_success_with_message() {
        let action = Action::Play();
        let result = ActionResult::success_with_message(action.clone(), "Hand played successfully");

        assert_eq!(result.action, action);
        assert!(result.success);
        assert_eq!(result.message.as_deref(), Some("Hand played successfully"));
    }

    #[test]
    fn action_result_success_with_deltas() {
        let action = Action::Play();
        let result = ActionResult::success_with_deltas(action.clone(), Some(1500.0), Some(50.0));

        assert_eq!(result.action, action);
        assert!(result.success);
        assert_eq!(result.score_delta, Some(1500.0));
        assert_eq!(result.money_delta, Some(50.0));
    }

    #[test]
    fn action_result_failure_creation() {
        let action = Action::Play();
        let result = ActionResult::failure(action.clone(), "Invalid hand");

        assert_eq!(result.action, action);
        assert!(!result.success);
        assert_eq!(result.message.as_deref(), Some("Invalid hand"));
    }

    #[test]
    fn mock_notifier_tracks_state_changes() {
        let notifier = MockStateNotifier::new();
        let session_id = SessionId::new();
        let game = Game::new(Config::default());

        notifier.notify_state_change(&session_id, &game);

        let events = notifier.get_events();
        assert_eq!(events.len(), 1);

        match &events[0] {
            Event::StateChange { session_id: id, .. } => {
                assert_eq!(id, &session_id);
            }
            _ => panic!("Expected StateChange event"),
        }
    }

    #[test]
    fn mock_notifier_tracks_action_results() {
        let notifier = MockStateNotifier::new();
        let session_id = SessionId::new();
        let result = ActionResult::success(Action::Play());

        notifier.notify_action_result(&session_id, &result);

        let events = notifier.get_events();
        assert_eq!(events.len(), 1);

        match &events[0] {
            Event::ActionResult {
                session_id: id,
                success,
                ..
            } => {
                assert_eq!(id, &session_id);
                assert!(success);
            }
            _ => panic!("Expected ActionResult event"),
        }
    }

    #[test]
    fn mock_notifier_tracks_session_lifecycle() {
        let notifier = MockStateNotifier::new();
        let session_id = SessionId::new();
        let game = Game::new(Config::default());

        // Session started
        notifier.notify_session_started(&session_id, &game);
        assert_eq!(notifier.event_count(), 1);

        // Session ended
        notifier.notify_session_ended(&session_id, Some(&game));
        assert_eq!(notifier.event_count(), 2);

        let events = notifier.get_events();
        match &events[0] {
            Event::SessionStarted { session_id: id } => {
                assert_eq!(id, &session_id);
            }
            _ => panic!("Expected SessionStarted event"),
        }

        match &events[1] {
            Event::SessionEnded {
                session_id: id,
                had_final_state,
            } => {
                assert_eq!(id, &session_id);
                assert!(had_final_state);
            }
            _ => panic!("Expected SessionEnded event"),
        }
    }

    #[test]
    fn mock_notifier_tracks_errors() {
        let notifier = MockStateNotifier::new();
        let session_id = SessionId::new();

        notifier.notify_error(Some(&session_id), "Test error occurred", Some("ERR_001"));

        let events = notifier.get_events();
        assert_eq!(events.len(), 1);

        match &events[0] {
            Event::Error {
                session_id: id,
                message,
                code,
            } => {
                assert_eq!(id, &Some(session_id));
                assert_eq!(message, "Test error occurred");
                assert_eq!(code, &Some("ERR_001".to_string()));
            }
            _ => panic!("Expected Error event"),
        }
    }

    #[test]
    fn mock_notifier_can_clear_events() {
        let notifier = MockStateNotifier::new();
        let session_id = SessionId::new();
        let game = Game::new(Config::default());

        notifier.notify_state_change(&session_id, &game);
        assert_eq!(notifier.event_count(), 1);

        notifier.clear_events();
        assert_eq!(notifier.event_count(), 0);
    }

    #[test]
    fn trait_can_be_implemented() {
        let _notifier = MockStateNotifier::new();
        // This test ensures the trait can be implemented
    }

    #[test]
    fn notifier_trait_is_object_safe() {
        let notifier = MockStateNotifier::new();
        let _trait_object: &dyn StateNotifier = &notifier;
        // This test ensures the trait is object-safe (can be used as trait object)
    }

    #[test]
    fn notifier_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<MockStateNotifier>();
        // This test ensures implementations can be used across threads
    }

    #[test]
    fn action_result_equality_and_cloning() {
        let result1 = ActionResult::success(Action::Play());
        let result2 = ActionResult::success(Action::Play());
        let result3 = ActionResult::failure(Action::Play(), "Error");

        assert_eq!(result1, result2);
        assert_ne!(result1, result3);

        let cloned = result1.clone();
        assert_eq!(result1, cloned);
    }
}
