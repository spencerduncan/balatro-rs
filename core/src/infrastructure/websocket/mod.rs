//! High-Performance WebSocket Connection Management
//!
//! CRITICAL PERFORMANCE TARGETS:
//! - <5ms state updates via WebSocket
//! - 100+ simultaneous connections
//! - Zero-copy message sending where possible

#![cfg(feature = "websockets")]

pub mod connection_manager;

pub use connection_manager::{ConnectionPoolConfig, WebSocketConnectionPool, WebSocketError};

use axum::extract::ws::{Message, WebSocket};
use dashmap::DashMap;
use std::sync::Arc;
use tokio::sync::broadcast;

/// WebSocket error types
#[derive(thiserror::Error, Debug)]
pub enum WebSocketError {
    #[error("Connection limit exceeded: {current}/{max}")]
    ConnectionLimitExceeded { current: usize, max: usize },

    #[error("Connection not found: {session_id}")]
    ConnectionNotFound {
        session_id: crate::infrastructure::SessionId,
    },

    #[error("Message send failed: {message}")]
    MessageSendFailed { message: String },

    #[error("WebSocket closed unexpectedly")]
    ConnectionClosed,

    #[error("Performance threshold exceeded: {operation} took {duration_ms}ms")]
    PerformanceThreshold { operation: String, duration_ms: u64 },
}

/// WebSocket connection pool configuration
#[derive(Debug, Clone)]
pub struct ConnectionPoolConfig {
    /// Maximum concurrent WebSocket connections
    pub max_connections: usize,
    /// Ping interval in seconds
    pub ping_interval_seconds: u64,
    /// Connection timeout in seconds
    pub connection_timeout_seconds: u64,
    /// Message buffer size per connection
    pub message_buffer_size: usize,
    /// Enable compression
    pub compression_enabled: bool,
}

impl Default for ConnectionPoolConfig {
    fn default() -> Self {
        Self {
            max_connections: 100,
            ping_interval_seconds: 30,
            connection_timeout_seconds: 60,
            message_buffer_size: 1024,
            compression_enabled: true,
        }
    }
}

/// WebSocket message types for the game protocol
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type")]
pub enum GameMessage {
    /// Game state update
    StateUpdate {
        session_id: crate::infrastructure::SessionId,
        #[serde(with = "serde_bytes")]
        state_data: Vec<u8>, // Zero-copy serialized state
    },

    /// Action result
    ActionResult {
        action_id: String,
        success: bool,
        #[serde(with = "serde_bytes")]
        result_data: Vec<u8>,
    },

    /// Error message
    Error {
        code: u32,
        message: String,
    },

    /// Ping/Pong for connection health
    Ping {
        timestamp: u64,
    },
    Pong {
        timestamp: u64,
    },
}

/// Connection metrics for monitoring
#[derive(Debug, Clone)]
pub struct ConnectionMetrics {
    pub total_connections: usize,
    pub active_connections: usize,
    pub messages_sent: u64,
    pub messages_received: u64,
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub connection_errors: u64,
}

impl Default for ConnectionMetrics {
    fn default() -> Self {
        Self {
            total_connections: 0,
            active_connections: 0,
            messages_sent: 0,
            messages_received: 0,
            bytes_sent: 0,
            bytes_received: 0,
            connection_errors: 0,
        }
    }
}

/// Broadcast channel for efficient state updates
pub type StateBroadcaster = broadcast::Sender<GameMessage>;
pub type StateReceiver = broadcast::Receiver<GameMessage>;

/// Create a new state broadcaster with specified capacity
pub fn create_state_broadcaster(capacity: usize) -> (StateBroadcaster, StateReceiver) {
    broadcast::channel(capacity)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connection_pool_config_defaults() {
        let config = ConnectionPoolConfig::default();

        // Verify performance-oriented defaults
        assert_eq!(
            config.max_connections, 100,
            "Should support 100+ connections"
        );
        assert!(
            config.compression_enabled,
            "Compression should be enabled for efficiency"
        );
        assert_eq!(config.ping_interval_seconds, 30, "Reasonable ping interval");
        assert_eq!(config.message_buffer_size, 1024, "Adequate message buffer");
    }

    #[test]
    fn test_game_message_serialization() {
        let message = GameMessage::StateUpdate {
            session_id: uuid::Uuid::new_v4(),
            state_data: vec![1, 2, 3, 4],
        };

        let serialized = bincode::serialize(&message).unwrap();
        let deserialized: GameMessage = bincode::deserialize(&serialized).unwrap();

        match deserialized {
            GameMessage::StateUpdate { state_data, .. } => {
                assert_eq!(state_data, vec![1, 2, 3, 4]);
            }
            _ => panic!("Wrong message type"),
        }
    }

    #[test]
    fn test_connection_metrics_default() {
        let metrics = ConnectionMetrics::default();
        assert_eq!(metrics.total_connections, 0);
        assert_eq!(metrics.active_connections, 0);
        assert_eq!(metrics.messages_sent, 0);
    }
}
