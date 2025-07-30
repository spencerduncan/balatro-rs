//! High-Performance In-Memory Storage Implementation
//!
//! Uses DashMap for lock-free concurrent access and RAII patterns for resource management.
//! Designed to maintain <20MB per session and O(1) operations.

use super::{StoreConfig, StorageError, GameSession, SessionMetadata, StorageStats};
use crate::infrastructure::SessionId;
use dashmap::DashMap;
use std::sync::{
    atomic::{AtomicU64, AtomicUsize, Ordering},
    Arc,
};
use std::time::Instant;
use serde_json::Value;

/// High-performance memory store with concurrent access
pub struct HighPerformanceMemoryStore {
    /// Session storage with lock-free concurrent access
    sessions: Arc<DashMap<SessionId, GameSession>>,
    /// Configuration
    config: StoreConfig,
    /// Runtime statistics
    stats: Arc<MemoryStoreStats>,
    /// Background cleanup task handle
    cleanup_handle: tokio::task::JoinHandle<()>,
    /// Store creation time for uptime calculation
    created_at: Instant,
}

impl HighPerformanceMemoryStore {
    /// Create new high-performance memory store
    pub fn new(config: StoreConfig) -> Result<Self, StorageError> {
        let sessions = Arc::new(DashMap::new());
        let stats = Arc::new(MemoryStoreStats::new());
        let created_at = Instant::now();

        // Start background cleanup task
        let cleanup_sessions = sessions.clone();
        let cleanup_stats = stats.clone();
        let cleanup_config = config.clone();

        let cleanup_handle = tokio::spawn(async move {
            Self::cleanup_task(cleanup_sessions, cleanup_stats, cleanup_config).await;
        });

        tracing::info!("High-performance memory store created with max_memory: {}MB, max_sessions: {}",
            config.max_memory_mb, config.max_sessions);

        Ok(Self {
            sessions,
            config,
            stats,
            cleanup_handle,
            created_at,
        })
    }

    /// Create new game session
    pub async fn create_session(&self, session_id: SessionId) -> Result<(), StorageError> {
        let start = Instant::now();

        // Check if session already exists
        if self.sessions.contains_key(&session_id) {
            return Err(StorageError::SessionAlreadyExists { session_id });
        }

        // Check session limit
        if self.sessions.len() >= self.config.max_sessions {
            return Err(StorageError::OperationFailed {
                operation: "create_session".to_string(),
                message: format!("Session limit exceeded: {}/{}", self.sessions.len(), self.config.max_sessions),
            });
        }

        // Check memory limit
        let current_memory_mb = self.memory_usage_mb();
        if current_memory_mb > self.config.max_memory_mb {
            return Err(StorageError::MemoryLimitExceeded {
                current_mb: current_memory_mb,
                limit_mb: self.config.max_memory_mb,
            });
        }

        // Create and insert session
        let session = GameSession::new(session_id);
        self.sessions.insert(session_id, session);

        // Update statistics
        self.stats.total_operations.fetch_add(1, Ordering::Relaxed);
        self.stats.sessions_created.fetch_add(1, Ordering::Relaxed);

        let duration = start.elapsed();
        if duration.as_millis() > 10 {
            tracing::warn!("Slow session creation: {}ms for {}", duration.as_millis(), session_id);
        }

        #[cfg(feature = "monitoring")]
        {
            metrics::counter!("storage_sessions_created", 1);
            metrics::histogram!("storage_create_session_duration_ms", duration.as_millis() as f64);
        }

        tracing::debug!("Session created: {}", session_id);
        Ok(())
    }

    /// Get session by ID
    pub async fn get_session(&self, session_id: &SessionId) -> Option<GameSession> {
        let start = Instant::now();

        let result = self.sessions.get(session_id).map(|entry| {
            let mut session = entry.value().clone();
            session.touch(); // Update last accessed time
            session
        });

        // Update the session in storage with new last_accessed time
        if let Some(ref session) = result {
            if let Some(mut entry) = self.sessions.get_mut(session_id) {
                entry.last_accessed = session.last_accessed;
            }
        }

        self.stats.total_operations.fetch_add(1, Ordering::Relaxed);

        let duration = start.elapsed();
        #[cfg(feature = "monitoring")]
        {
            metrics::histogram!("storage_get_session_duration_ms", duration.as_millis() as f64);
        }

        result
    }

    /// Handle session action (PERFORMANCE CRITICAL - part of <10ms action latency)
    pub async fn handle_session_action(&self, session_id: SessionId, action_data: Value) -> Result<Value, StorageError> {
        let start = Instant::now();

        // Get session
        let mut session = self.sessions.get_mut(&session_id)
            .ok_or(StorageError::SessionNotFound { session_id })?;

        // Update session metadata
        session.metadata.record_action(start.elapsed().as_nanos() as u64);
        session.touch();

        // Process action (this would integrate with the actual game engine)
        let result = self.process_game_action(&mut session, action_data).await?;

        let duration = start.elapsed();
        self.stats.total_operations.fetch_add(1, Ordering::Relaxed);

        #[cfg(feature = "monitoring")]
        {
            metrics::histogram!("storage_handle_action_duration_ms", duration.as_millis() as f64);
            if duration.as_millis() > 5 {
                metrics::counter!("storage_slow_actions", 1);
            }
        }

        if duration.as_millis() > 5 {
            tracing::warn!("Slow action processing: {}ms for session {}", duration.as_millis(), session_id);
        }

        Ok(result)
    }

    /// Get session game state
    pub async fn get_session_state(&self, session_id: SessionId) -> Result<Option<Value>, StorageError> {
        let start = Instant::now();

        let result = if let Some(session) = self.sessions.get(&session_id) {
            // For now, return basic session info
            // In real implementation, this would deserialize the actual game state
            Some(serde_json::json!({
                "session_id": session_id,
                "created_at": session.created_at.elapsed().as_secs(),
                "last_accessed": session.last_accessed.elapsed().as_secs(),
                "action_count": session.metadata.action_count,
                "memory_usage_bytes": session.memory_usage_bytes()
            }))
        } else {
            None
        };

        let duration = start.elapsed();
        self.stats.total_operations.fetch_add(1, Ordering::Relaxed);

        #[cfg(feature = "monitoring")]
        {
            metrics::histogram!("storage_get_state_duration_ms", duration.as_millis() as f64);
        }

        Ok(result)
    }

    /// Remove session
    pub async fn remove_session(&self, session_id: &SessionId) -> Option<GameSession> {
        let result = self.sessions.remove(session_id).map(|(_, session)| session);

        if result.is_some() {
            self.stats.sessions_removed.fetch_add(1, Ordering::Relaxed);

            #[cfg(feature = "monitoring")]
            {
                metrics::counter!("storage_sessions_removed", 1);
            }

            tracing::debug!("Session removed: {}", session_id);
        }

        self.stats.total_operations.fetch_add(1, Ordering::Relaxed);
        result
    }

    /// Get current memory usage in MB
    pub fn memory_usage_mb(&self) -> f64 {
        let total_bytes: usize = self.sessions.iter()
            .map(|entry| entry.value().memory_usage_bytes())
            .sum();

        // Add overhead for DashMap and other structures
        let overhead_bytes = self.sessions.len() * 64; // Estimate

        (total_bytes + overhead_bytes) as f64 / (1024.0 * 1024.0)
    }

    /// Get uptime in seconds
    pub fn uptime_seconds(&self) -> u64 {
        self.created_at.elapsed().as_secs()
    }

    /// Get storage statistics
    pub fn get_stats(&self) -> StorageStats {
        let session_count = self.sessions.len();
        let total_memory_bytes = (self.memory_usage_mb() * 1024.0 * 1024.0) as usize;

        StorageStats {
            session_count,
            total_memory_bytes,
            average_memory_per_session_bytes: if session_count > 0 {
                total_memory_bytes as f64 / session_count as f64
            } else {
                0.0
            },
            expired_sessions_cleaned: self.stats.expired_sessions_cleaned.load(Ordering::Relaxed),
            total_operations: self.stats.total_operations.load(Ordering::Relaxed),
            failed_operations: self.stats.failed_operations.load(Ordering::Relaxed),
        }
    }

    /// Process game action (stub implementation)
    async fn process_game_action(&self, session: &mut dashmap::mapref::one::RefMut<SessionId, GameSession>, action_data: Value) -> Result<Value, StorageError> {
        // This is where the actual game engine integration would happen
        // For now, just return a success response

        // Simulate some processing time
        tokio::time::sleep(tokio::time::Duration::from_micros(100)).await;

        Ok(serde_json::json!({
            "success": true,
            "action": action_data,
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "session_id": session.session_id
        }))
    }

    /// Background cleanup task for expired sessions
    async fn cleanup_task(
        sessions: Arc<DashMap<SessionId, GameSession>>,
        stats: Arc<MemoryStoreStats>,
        config: StoreConfig,
    ) {
        let mut interval = tokio::time::interval(
            tokio::time::Duration::from_secs(config.cleanup_interval_seconds)
        );

        loop {
            interval.tick().await;

            let mut cleaned_count = 0;

            // Remove expired sessions
            sessions.retain(|session_id, session| {
                if session.is_expired(config.session_timeout_seconds) {
                    tracing::info!("Cleaning up expired session: {}", session_id);
                    cleaned_count += 1;
                    false
                } else {
                    true
                }
            });

            if cleaned_count > 0 {
                stats.expired_sessions_cleaned.fetch_add(cleaned_count, Ordering::Relaxed);

                #[cfg(feature = "monitoring")]
                {
                    metrics::counter!("storage_expired_sessions_cleaned", cleaned_count);
                    metrics::gauge!("storage_active_sessions", sessions.len() as f64);
                }

                tracing::info!("Cleaned up {} expired sessions", cleaned_count);
            }
        }
    }
}

/// Internal statistics for the memory store
struct MemoryStoreStats {
    total_operations: AtomicU64,
    failed_operations: AtomicU64,
    sessions_created: AtomicU64,
    sessions_removed: AtomicU64,
    expired_sessions_cleaned: AtomicU64,
}

impl MemoryStoreStats {
    fn new() -> Self {
        Self {
            total_operations: AtomicU64::new(0),
            failed_operations: AtomicU64::new(0),
            sessions_created: AtomicU64::new(0),
            sessions_removed: AtomicU64::new(0),
            expired_sessions_cleaned: AtomicU64::new(0),
        }
    }
}

// RAII cleanup for the memory store
impl Drop for HighPerformanceMemoryStore {
    fn drop(&mut self) {
        // Abort cleanup task
        self.cleanup_handle.abort();

        let session_count = self.sessions.len();
        let memory_usage = self.memory_usage_mb();

        tracing::info!(
            "High-performance memory store shutting down: {} sessions, {:.2}MB memory usage",
            session_count, memory_usage
        );

        #[cfg(feature = "monitoring")]
        {
            metrics::gauge!("storage_final_session_count", session_count as f64);
            metrics::gauge!("storage_final_memory_usage_mb", memory_usage);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_memory_store_creation() {
        let config = StoreConfig::default();
        let store = HighPerformanceMemoryStore::new(config);
        assert!(store.is_ok(), "Memory store should be created successfully");
    }

    #[tokio::test]
    async fn test_session_crud_operations() {
        let config = StoreConfig::default();
        let store = HighPerformanceMemoryStore::new(config).unwrap();
        let session_id = uuid::Uuid::new_v4();

        // Create session
        let result = store.create_session(session_id).await;
        assert!(result.is_ok(), "Session creation should succeed");

        // Get session
        let retrieved = store.get_session(&session_id).await;
        assert!(retrieved.is_some(), "Session should be retrievable");
        assert_eq!(retrieved.unwrap().session_id, session_id);

        // Remove session
        let removed = store.remove_session(&session_id).await;
        assert!(removed.is_some(), "Session should be removable");

        // Verify removal
        let not_found = store.get_session(&session_id).await;
        assert!(not_found.is_none(), "Removed session should not be found");
    }

    #[tokio::test]
    async fn test_session_action_handling() {
        let config = StoreConfig::default();
        let store = HighPerformanceMemoryStore::new(config).unwrap();
        let session_id = uuid::Uuid::new_v4();

        // Create session
        store.create_session(session_id).await.unwrap();

        // Handle action
        let action_data = serde_json::json!({"action": "test_action", "params": {}});
        let result = store.handle_session_action(session_id, action_data).await;

        assert!(result.is_ok(), "Action handling should succeed");
        let response = result.unwrap();
        assert_eq!(response["success"], true);
    }

    #[tokio::test]
    async fn test_memory_usage_tracking() {
        let config = StoreConfig::default();
        let store = HighPerformanceMemoryStore::new(config).unwrap();

        let initial_memory = store.memory_usage_mb();
        assert!(initial_memory >= 0.0, "Initial memory usage should be non-negative");

        // Create some sessions
        for i in 0..10 {
            let session_id = uuid::Uuid::new_v4();
            store.create_session(session_id).await.unwrap();
        }

        let after_memory = store.memory_usage_mb();
        assert!(after_memory > initial_memory, "Memory usage should increase with sessions");
    }

    #[tokio::test]
    async fn test_session_limit_enforcement() {
        let config = StoreConfig {
            max_sessions: 2,
            ..Default::default()
        };
        let store = HighPerformanceMemoryStore::new(config).unwrap();

        // Create sessions up to limit
        let session1 = uuid::Uuid::new_v4();
        let session2 = uuid::Uuid::new_v4();

        assert!(store.create_session(session1).await.is_ok());
        assert!(store.create_session(session2).await.is_ok());

        // Third session should fail
        let session3 = uuid::Uuid::new_v4();
        let result = store.create_session(session3).await;
        assert!(result.is_err(), "Should reject session beyond limit");
    }

    #[tokio::test]
    async fn test_duplicate_session_rejection() {
        let config = StoreConfig::default();
        let store = HighPerformanceMemoryStore::new(config).unwrap();
        let session_id = uuid::Uuid::new_v4();

        // Create session
        assert!(store.create_session(session_id).await.is_ok());

        // Try to create duplicate
        let result = store.create_session(session_id).await;
        assert!(result.is_err(), "Should reject duplicate session");

        match result.unwrap_err() {
            StorageError::SessionAlreadyExists { .. } => {},
            _ => panic!("Expected SessionAlreadyExists error"),
        }
    }

    #[test]
    fn test_store_config_defaults() {
        let config = StoreConfig::default();

        assert_eq!(config.max_memory_mb, 1000.0);
        assert_eq!(config.max_sessions, 1000);
        assert_eq!(config.session_timeout_seconds, 3600);
        assert!(config.memory_monitoring_enabled);
        assert!(config.compression_enabled);
    }
}
