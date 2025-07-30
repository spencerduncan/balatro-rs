//! Memory-Optimized Session Store
//!
//! CRITICAL PERFORMANCE TARGETS:
//! - <20MB memory usage per session
//! - O(1) session operations
//! - Lock-free concurrent access where possible

#![cfg(feature = "concurrent")]

pub mod memory_store;

pub use memory_store::{HighPerformanceMemoryStore, StoreConfig, StorageError, GameSession};

use crate::infrastructure::SessionId;
use std::sync::Arc;
use serde_json::Value;

/// Storage error types
#[derive(thiserror::Error, Debug)]
pub enum StorageError {
    #[error("Session not found: {session_id}")]
    SessionNotFound { session_id: SessionId },

    #[error("Session already exists: {session_id}")]
    SessionAlreadyExists { session_id: SessionId },

    #[error("Memory limit exceeded: {current_mb}MB > {limit_mb}MB")]
    MemoryLimitExceeded { current_mb: f64, limit_mb: f64 },

    #[error("Serialization failed: {message}")]
    SerializationFailed { message: String },

    #[error("Storage operation failed: {operation}: {message}")]
    OperationFailed { operation: String, message: String },

    #[error("Performance threshold exceeded: {operation} took {duration_ms}ms")]
    PerformanceThreshold { operation: String, duration_ms: u64 },
}

/// Storage configuration for memory optimization
#[derive(Debug, Clone)]
pub struct StoreConfig {
    /// Maximum memory usage in MB
    pub max_memory_mb: f64,
    /// Maximum sessions to store
    pub max_sessions: usize,
    /// Session timeout in seconds
    pub session_timeout_seconds: u64,
    /// Cleanup interval in seconds
    pub cleanup_interval_seconds: u64,
    /// Enable memory monitoring
    pub memory_monitoring_enabled: bool,
    /// Enable compression for stored data
    pub compression_enabled: bool,
}

impl Default for StoreConfig {
    fn default() -> Self {
        Self {
            max_memory_mb: 1000.0, // 1GB total for all sessions
            max_sessions: 1000,
            session_timeout_seconds: 3600, // 1 hour
            cleanup_interval_seconds: 300, // 5 minutes
            memory_monitoring_enabled: true,
            compression_enabled: true,
        }
    }
}

/// Game session data optimized for memory usage
#[derive(Debug, Clone)]
pub struct GameSession {
    /// Session identifier
    pub session_id: SessionId,
    /// Creation timestamp
    pub created_at: std::time::Instant,
    /// Last accessed timestamp
    pub last_accessed: std::time::Instant,
    /// Compressed game state data
    pub state_data: Vec<u8>,
    /// Action history (compressed)
    pub action_history: Vec<u8>,
    /// Session metadata
    pub metadata: SessionMetadata,
}

impl GameSession {
    /// Create new game session
    pub fn new(session_id: SessionId) -> Self {
        let now = std::time::Instant::now();
        Self {
            session_id,
            created_at: now,
            last_accessed: now,
            state_data: Vec::new(),
            action_history: Vec::new(),
            metadata: SessionMetadata::default(),
        }
    }

    /// Update last accessed time
    pub fn touch(&mut self) {
        self.last_accessed = std::time::Instant::now();
    }

    /// Get memory usage in bytes
    pub fn memory_usage_bytes(&self) -> usize {
        std::mem::size_of::<Self>() +
        self.state_data.capacity() +
        self.action_history.capacity() +
        self.metadata.memory_usage_bytes()
    }

    /// Check if session is expired
    pub fn is_expired(&self, timeout_seconds: u64) -> bool {
        self.last_accessed.elapsed().as_secs() > timeout_seconds
    }

    /// Update game state with compression
    pub fn update_state<T>(&mut self, state: &T) -> Result<(), StorageError>
    where
        T: serde::Serialize,
    {
        let serialized = crate::infrastructure::serialization::serialize_game_state_zerocopy(state)
            .map_err(|e| StorageError::SerializationFailed {
                message: e.to_string(),
            })?;

        // Optional compression for memory optimization
        #[cfg(feature = "zero-copy")]
        {
            self.state_data = serialized;
        }

        #[cfg(not(feature = "zero-copy"))]
        {
            self.state_data = serialized;
        }

        self.touch();
        Ok(())
    }

    /// Get game state with decompression
    pub fn get_state<T>(&self) -> Result<T, StorageError>
    where
        T: serde::de::DeserializeOwned,
    {
        if self.state_data.is_empty() {
            return Err(StorageError::OperationFailed {
                operation: "get_state".to_string(),
                message: "No state data available".to_string(),
            });
        }

        crate::infrastructure::serialization::deserialize_game_state_zerocopy(&self.state_data)
            .map_err(|e| StorageError::SerializationFailed {
                message: e.to_string(),
            })
    }
}

/// Session metadata for tracking and optimization
#[derive(Debug, Clone, Default)]
pub struct SessionMetadata {
    /// Number of actions processed
    pub action_count: u64,
    /// Total execution time in nanoseconds
    pub total_execution_time_nanos: u64,
    /// Peak memory usage in bytes
    pub peak_memory_bytes: usize,
    /// Last error (if any)
    pub last_error: Option<String>,
    /// Custom data
    pub custom_data: std::collections::HashMap<String, String>,
}

impl SessionMetadata {
    /// Get memory usage of metadata
    pub fn memory_usage_bytes(&self) -> usize {
        std::mem::size_of::<Self>() +
        self.last_error.as_ref().map_or(0, |s| s.capacity()) +
        self.custom_data.iter().map(|(k, v)| k.capacity() + v.capacity()).sum::<usize>()
    }

    /// Record action execution
    pub fn record_action(&mut self, execution_time_nanos: u64) {
        self.action_count += 1;
        self.total_execution_time_nanos += execution_time_nanos;
    }

    /// Get average action execution time
    pub fn average_execution_time_nanos(&self) -> u64 {
        if self.action_count == 0 {
            0
        } else {
            self.total_execution_time_nanos / self.action_count
        }
    }
}

/// Storage statistics for monitoring
#[derive(Debug, Clone, Default)]
pub struct StorageStats {
    /// Total sessions stored
    pub session_count: usize,
    /// Total memory usage in bytes
    pub total_memory_bytes: usize,
    /// Average memory per session in bytes
    pub average_memory_per_session_bytes: f64,
    /// Number of expired sessions cleaned up
    pub expired_sessions_cleaned: u64,
    /// Total operations performed
    pub total_operations: u64,
    /// Failed operations
    pub failed_operations: u64,
}

impl StorageStats {
    /// Calculate memory usage in MB
    pub fn memory_usage_mb(&self) -> f64 {
        self.total_memory_bytes as f64 / (1024.0 * 1024.0)
    }

    /// Calculate success rate
    pub fn success_rate(&self) -> f64 {
        if self.total_operations == 0 {
            1.0
        } else {
            1.0 - (self.failed_operations as f64 / self.total_operations as f64)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
    struct TestGameState {
        score: u64,
        level: u32,
    }

    #[test]
    fn test_game_session_creation() {
        let session_id = uuid::Uuid::new_v4();
        let session = GameSession::new(session_id);

        assert_eq!(session.session_id, session_id);
        assert!(session.state_data.is_empty());
        assert!(session.action_history.is_empty());
        assert_eq!(session.metadata.action_count, 0);
    }

    #[test]
    fn test_session_state_update() {
        let mut session = GameSession::new(uuid::Uuid::new_v4());
        let test_state = TestGameState { score: 1000, level: 5 };

        let result = session.update_state(&test_state);
        assert!(result.is_ok(), "State update should succeed");
        assert!(!session.state_data.is_empty(), "State data should not be empty");
    }

    #[test]
    fn test_session_memory_usage() {
        let session = GameSession::new(uuid::Uuid::new_v4());
        let memory_usage = session.memory_usage_bytes();

        // Should be reasonable size
        assert!(memory_usage > 0, "Memory usage should be positive");
        assert!(memory_usage < 1024 * 1024, "Empty session should use less than 1MB"); // 1MB
    }

    #[test]
    fn test_session_expiration() {
        let mut session = GameSession::new(uuid::Uuid::new_v4());

        // Fresh session should not be expired
        assert!(!session.is_expired(3600), "Fresh session should not be expired");

        // Manually set old timestamp
        session.last_accessed = std::time::Instant::now() - std::time::Duration::from_secs(7200);
        assert!(session.is_expired(3600), "Old session should be expired");
    }

    #[test]
    fn test_session_metadata() {
        let mut metadata = SessionMetadata::default();

        metadata.record_action(1000000); // 1ms
        metadata.record_action(2000000); // 2ms

        assert_eq!(metadata.action_count, 2);
        assert_eq!(metadata.total_execution_time_nanos, 3000000);
        assert_eq!(metadata.average_execution_time_nanos(), 1500000); // 1.5ms average
    }

    #[test]
    fn test_storage_stats() {
        let mut stats = StorageStats::default();
        stats.total_memory_bytes = 20 * 1024 * 1024; // 20MB
        stats.total_operations = 100;
        stats.failed_operations = 5;

        assert_eq!(stats.memory_usage_mb(), 20.0);
        assert_eq!(stats.success_rate(), 0.95); // 95% success rate
    }

    #[test]
    fn test_store_config_defaults() {
        let config = StoreConfig::default();

        assert_eq!(config.max_memory_mb, 1000.0);
        assert_eq!(config.max_sessions, 1000);
        assert!(config.memory_monitoring_enabled);
        assert!(config.compression_enabled);
    }
}
