use crate::action::Action;
use crate::domain::value_objects::SessionId;
use crate::game::Game;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

// ============================================================================
// GameSession Entity - Core (~150 lines)
// ============================================================================

/// Production-ready GameSession with TTL, history tracking, and persistence
/// Optimized for concurrent access and horizontal scaling
#[derive(Debug, Serialize, Deserialize)]
pub struct GameSession {
    // Core identity and metadata
    id: SessionId,
    game: Game,
    created_at: u64, // Unix timestamp in seconds
    updated_at: u64,
    ttl_seconds: u64,

    // Session state tracking
    state: SessionState,

    // History tracking for replay and undo/redo
    history: SessionHistory,

    // Lifecycle metadata
    lifecycle: SessionLifecycle,

    // Version for migration support
    version: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SessionState {
    Active,
    Suspended,
    Expired,
    Completed,
}

// ============================================================================
// Session History Tracking (~100 lines)
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionHistory {
    actions: Vec<TimestampedAction>,
    snapshots: Vec<StateSnapshot>,
    max_actions: usize,
    max_snapshots: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimestampedAction {
    pub action: Action,
    pub timestamp: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_index: Option<usize>, // Reference to snapshot taken after this action
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct StateSnapshot {
    game_state_data: Vec<u8>, // Serialized game state for efficient storage
    timestamp: u64,
    action_index: usize, // Index of action that led to this state
}

impl SessionHistory {
    fn new() -> Self {
        Self {
            actions: Vec::with_capacity(1024), // Pre-allocate for performance
            snapshots: Vec::with_capacity(32),
            max_actions: 10000, // Configurable limit for memory management
            max_snapshots: 100,
        }
    }

    fn record_action(&mut self, action: Action) {
        let timestamp = current_timestamp();

        // Circular buffer behavior - remove oldest when at capacity
        if self.actions.len() >= self.max_actions {
            self.actions.remove(0);
            // Adjust snapshot indices
            for snapshot in &mut self.snapshots {
                if snapshot.action_index > 0 {
                    snapshot.action_index -= 1;
                }
            }
        }

        self.actions.push(TimestampedAction {
            action,
            timestamp,
            snapshot_index: None,
        });
    }

    fn create_snapshot(&mut self, game: &Game) -> Result<(), String> {
        if self.actions.is_empty() {
            return Ok(());
        }

        let action_index = self.actions.len() - 1;
        let snapshot_index = self.snapshots.len();

        // Serialize game state for efficient storage
        let game_state_data =
            serde_json::to_vec(game).map_err(|e| format!("Failed to serialize game state: {e}"))?;

        // Update action to reference this snapshot
        if let Some(action) = self.actions.last_mut() {
            action.snapshot_index = Some(snapshot_index);
        }

        // Circular buffer for snapshots
        if self.snapshots.len() >= self.max_snapshots {
            self.snapshots.remove(0);
            // Update references in actions
            for action in &mut self.actions {
                if let Some(idx) = action.snapshot_index.as_mut() {
                    if *idx > 0 {
                        *idx -= 1;
                    }
                }
            }
        }

        self.snapshots.push(StateSnapshot {
            game_state_data,
            timestamp: current_timestamp(),
            action_index,
        });

        Ok(())
    }
}

// ============================================================================
// Session Lifecycle Management (~100 lines)
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionLifecycle {
    suspension_count: u32,
    total_active_time: u64, // Total seconds spent in Active state
    last_active_start: Option<u64>,
    expiration_reason: Option<ExpirationReason>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ExpirationReason {
    TTLExpired,
    ManualExpiration,
    ServerShutdown,
    InactivityTimeout,
}

impl SessionLifecycle {
    fn new() -> Self {
        Self {
            suspension_count: 0,
            total_active_time: 0,
            last_active_start: Some(current_timestamp()),
            expiration_reason: None,
        }
    }

    fn suspend(&mut self) {
        if let Some(start) = self.last_active_start {
            self.total_active_time += current_timestamp() - start;
            self.last_active_start = None;
            self.suspension_count += 1;
        }
    }

    fn resume(&mut self) {
        self.last_active_start = Some(current_timestamp());
    }

    fn expire(&mut self, reason: ExpirationReason) {
        if let Some(start) = self.last_active_start {
            self.total_active_time += current_timestamp() - start;
            self.last_active_start = None;
        }
        self.expiration_reason = Some(reason);
    }
}

// ============================================================================
// Session Implementation and Persistence (~100 lines)
// ============================================================================

impl GameSession {
    /// Create a new game session with default TTL of 1 hour
    pub fn new(game: Game) -> Self {
        Self::with_ttl(game, Duration::from_secs(3600))
    }

    /// Create a new game session with custom TTL
    pub fn with_ttl(game: Game, ttl: Duration) -> Self {
        let now = current_timestamp();
        Self {
            id: SessionId::new(),
            game,
            created_at: now,
            updated_at: now,
            ttl_seconds: ttl.as_secs(),
            state: SessionState::Active,
            history: SessionHistory::new(),
            lifecycle: SessionLifecycle::new(),
            version: 1,
        }
    }

    /// Get the session ID
    pub fn id(&self) -> SessionId {
        self.id
    }

    /// Get immutable reference to the game
    pub fn game(&self) -> &Game {
        &self.game
    }

    /// Get mutable reference to the game
    pub fn game_mut(&mut self) -> &mut Game {
        self.updated_at = current_timestamp();
        &mut self.game
    }

    /// Handle an action and update history
    pub fn handle_action(&mut self, action: Action) -> Result<(), String> {
        if self.state != SessionState::Active {
            return Err(format!("Session is not active: {:?}", self.state));
        }

        // Record action in history
        self.history.record_action(action.clone());

        // Apply action to game
        self.game
            .handle_action(action)
            .map_err(|e| format!("Game error: {e:?}"))?;

        // Update timestamp
        self.updated_at = current_timestamp();

        // Create snapshot every 10 actions for efficient replay
        if self.history.actions.len() % 10 == 0 {
            self.history.create_snapshot(&self.game)?;
        }

        Ok(())
    }

    /// Check if session has expired based on TTL
    pub fn is_expired(&self) -> bool {
        if self.state == SessionState::Expired {
            return true;
        }

        let now = current_timestamp();
        let elapsed = now - self.updated_at;
        elapsed > self.ttl_seconds
    }

    /// Suspend the session (e.g., player disconnected)
    pub fn suspend(&mut self) {
        if self.state == SessionState::Active {
            self.state = SessionState::Suspended;
            self.lifecycle.suspend();
            self.updated_at = current_timestamp();
        }
    }

    /// Resume a suspended session
    pub fn resume(&mut self) -> Result<(), String> {
        if self.state != SessionState::Suspended {
            return Err(format!("Cannot resume from state: {:?}", self.state));
        }

        if self.is_expired() {
            self.state = SessionState::Expired;
            self.lifecycle.expire(ExpirationReason::TTLExpired);
            return Err("Session has expired".to_string());
        }

        self.state = SessionState::Active;
        self.lifecycle.resume();
        self.updated_at = current_timestamp();
        Ok(())
    }

    /// Mark session as completed
    pub fn complete(&mut self) {
        self.state = SessionState::Completed;
        self.lifecycle.suspend(); // Stop tracking active time
        self.updated_at = current_timestamp();
    }

    /// Force expire the session
    pub fn expire(&mut self, reason: ExpirationReason) {
        self.state = SessionState::Expired;
        self.lifecycle.expire(reason);
        self.updated_at = current_timestamp();
    }

    /// Get session state
    pub fn state(&self) -> SessionState {
        self.state
    }

    /// Get action history
    pub fn history(&self) -> &[TimestampedAction] {
        &self.history.actions
    }

    /// Replay actions from a snapshot point
    pub fn replay_from_snapshot(&self, snapshot_index: usize) -> Result<Game, String> {
        if snapshot_index >= self.history.snapshots.len() {
            return Err("Invalid snapshot index".to_string());
        }

        let snapshot = &self.history.snapshots[snapshot_index];

        // Deserialize game state from snapshot
        let mut game: Game = serde_json::from_slice(&snapshot.game_state_data)
            .map_err(|e| format!("Failed to deserialize snapshot: {e}"))?;

        // Replay actions after the snapshot
        for i in (snapshot.action_index + 1)..self.history.actions.len() {
            game.handle_action(self.history.actions[i].action.clone())
                .map_err(|e| format!("Failed to replay action: {e:?}"))?;
        }

        Ok(game)
    }

    /// Serialize session for persistence
    pub fn serialize(&self) -> Result<Vec<u8>, String> {
        serde_json::to_vec(self).map_err(|e| e.to_string())
    }

    /// Deserialize session from persistence
    pub fn deserialize(data: &[u8]) -> Result<Self, String> {
        serde_json::from_slice(data).map_err(|e| e.to_string())
    }

    /// Migrate session to latest version
    pub fn migrate(mut self) -> Self {
        // Migration logic for future versions
        match self.version {
            1 => {
                // Current version, no migration needed
            }
            _ => {
                // Future migration paths
            }
        }
        self.version = 1;
        self
    }
}

// Helper function for consistent timestamp generation
fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

// ============================================================================
// Thread-Safe Session Repository (~50 lines)
// ============================================================================

/// Thread-safe repository for managing multiple sessions
pub struct SessionRepository {
    sessions: Arc<RwLock<std::collections::HashMap<SessionId, GameSession>>>,
}

impl SessionRepository {
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(RwLock::new(std::collections::HashMap::new())),
        }
    }

    pub fn save(&self, session: GameSession) -> Result<(), String> {
        let mut sessions = self.sessions.write().map_err(|_| "Lock poisoned")?;
        sessions.insert(session.id(), session);
        Ok(())
    }

    pub fn load(&self, id: SessionId) -> Result<Vec<u8>, String> {
        let sessions = self.sessions.read().map_err(|_| "Lock poisoned")?;
        sessions
            .get(&id)
            .ok_or_else(|| "Session not found".to_string())
            .and_then(|session| session.serialize())
    }

    pub fn delete(&self, id: SessionId) -> Result<(), String> {
        let mut sessions = self.sessions.write().map_err(|_| "Lock poisoned")?;
        sessions.remove(&id);
        Ok(())
    }

    pub fn cleanup_expired(&self) -> Result<usize, String> {
        let mut sessions = self.sessions.write().map_err(|_| "Lock poisoned")?;
        let expired: Vec<SessionId> = sessions
            .iter()
            .filter(|(_, s)| s.is_expired())
            .map(|(id, _)| *id)
            .collect();

        let count = expired.len();
        for id in expired {
            sessions.remove(&id);
        }
        Ok(count)
    }
}

impl Default for SessionRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_creation() {
        let game = Game::default();
        let session = GameSession::new(game);

        assert_eq!(session.state(), SessionState::Active);
        assert!(!session.is_expired());
    }

    #[test]
    fn test_session_lifecycle() {
        let mut session = GameSession::new(Game::default());

        // Test suspension
        session.suspend();
        assert_eq!(session.state(), SessionState::Suspended);

        // Test resumption
        assert!(session.resume().is_ok());
        assert_eq!(session.state(), SessionState::Active);

        // Test completion
        session.complete();
        assert_eq!(session.state(), SessionState::Completed);
    }

    #[test]
    fn test_session_serialization() {
        let session = GameSession::new(Game::default());

        let serialized = session.serialize().unwrap();
        let deserialized = GameSession::deserialize(&serialized).unwrap();

        assert_eq!(session.id(), deserialized.id());
        assert_eq!(session.state(), deserialized.state());
    }

    #[test]
    fn test_repository_operations() {
        let repo = SessionRepository::new();
        let session = GameSession::new(Game::default());
        let id = session.id();

        // Save
        assert!(repo.save(session).is_ok());

        // Load and verify it returns serialized data
        let loaded_data = repo.load(id).unwrap();
        assert!(!loaded_data.is_empty());

        // Verify we can deserialize it
        let deserialized = GameSession::deserialize(&loaded_data).unwrap();
        assert_eq!(deserialized.id(), id);

        // Delete
        assert!(repo.delete(id).is_ok());
        assert!(repo.load(id).is_err());
    }
}
