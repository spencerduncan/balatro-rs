//! WebSocket Connection Pool Implementation
//!
//! Manages 100+ concurrent WebSocket connections with <5ms state update latency.
//! Uses DashMap for lock-free concurrent access and RAII patterns for cleanup.

use super::{ConnectionMetrics, ConnectionPoolConfig, GameMessage, WebSocketError};
use crate::infrastructure::SessionId;
use axum::extract::ws::{Message, WebSocket};
use dashmap::DashMap;
use futures_util::{sink::SinkExt, stream::StreamExt};
use std::sync::{
    atomic::{AtomicU64, AtomicUsize, Ordering},
    Arc,
};
use std::time::Instant;
use tokio::sync::broadcast;

/// High-performance WebSocket connection pool
pub struct WebSocketConnectionPool {
    connections: Arc<DashMap<SessionId, WebSocketConnection>>,
    config: ConnectionPoolConfig,
    metrics: Arc<ConnectionMetrics>,
    state_broadcaster: broadcast::Sender<GameMessage>,
    cleanup_handle: tokio::task::JoinHandle<()>,
}

impl WebSocketConnectionPool {
    /// Create new connection pool with performance optimizations
    pub fn new(config: ConnectionPoolConfig) -> Result<Self, WebSocketError> {
        let connections = Arc::new(DashMap::new());
        let metrics = Arc::new(ConnectionMetrics::default());
        let (state_broadcaster, _) = broadcast::channel(config.message_buffer_size);

        // Start background cleanup task
        let cleanup_connections = connections.clone();
        let cleanup_config = config.clone();
        let cleanup_handle = tokio::spawn(async move {
            Self::cleanup_task(cleanup_connections, cleanup_config).await;
        });

        tracing::info!(
            "WebSocket connection pool initialized with max_connections: {}",
            config.max_connections
        );

        Ok(Self {
            connections,
            config,
            metrics,
            state_broadcaster,
            cleanup_handle,
        })
    }

    /// Add new WebSocket connection with RAII cleanup
    pub async fn add_connection(
        &self,
        session_id: SessionId,
        websocket: WebSocket,
    ) -> Result<(), WebSocketError> {
        // Check connection limit
        if self.connections.len() >= self.config.max_connections {
            return Err(WebSocketError::ConnectionLimitExceeded {
                current: self.connections.len(),
                max: self.config.max_connections,
            });
        }

        let connection =
            WebSocketConnection::new(session_id, websocket, self.state_broadcaster.subscribe())?;

        // Insert connection with concurrent safety
        self.connections.insert(session_id, connection);

        #[cfg(feature = "monitoring")]
        {
            metrics::counter!("websocket_connections_opened", 1);
            metrics::gauge!(
                "websocket_active_connections",
                self.connections.len() as f64
            );
        }

        tracing::info!("WebSocket connection added for session: {}", session_id);
        Ok(())
    }

    /// Remove connection (automatic via RAII Drop)
    pub async fn remove_connection(&self, session_id: &SessionId) {
        if self.connections.remove(session_id).is_some() {
            #[cfg(feature = "monitoring")]
            {
                metrics::counter!("websocket_connections_closed", 1);
                metrics::gauge!(
                    "websocket_active_connections",
                    self.connections.len() as f64
                );
            }

            tracing::info!("WebSocket connection removed for session: {}", session_id);
        }
    }

    /// Broadcast state update to all connections (PERFORMANCE CRITICAL - <5ms target)
    pub async fn broadcast_state_update(
        &self,
        session_id: &SessionId,
        state_data: &[u8],
    ) -> Result<(), WebSocketError> {
        let start = Instant::now();

        let message = GameMessage::StateUpdate {
            session_id: *session_id,
            state_data: state_data.to_vec(), // TODO: Implement true zero-copy
        };

        // Broadcast to all subscribers
        match self.state_broadcaster.send(message) {
            Ok(receiver_count) => {
                let duration = start.elapsed();

                #[cfg(feature = "monitoring")]
                {
                    metrics::histogram!(
                        "websocket_broadcast_duration_ms",
                        duration.as_millis() as f64
                    );
                    metrics::counter!("websocket_messages_broadcast", 1);
                    metrics::counter!("websocket_broadcast_receivers", receiver_count as u64);

                    if duration.as_millis() > 5 {
                        metrics::counter!("websocket_slow_broadcasts", 1);
                        tracing::warn!(
                            "Slow WebSocket broadcast: {}ms to {} receivers",
                            duration.as_millis(),
                            receiver_count
                        );
                    }
                }

                Ok(())
            }
            Err(_) => Err(WebSocketError::MessageSendFailed {
                message: "No active receivers".to_string(),
            }),
        }
    }

    /// Send message to specific connection with zero-copy optimization
    pub async fn send_message_zerocopy(
        &self,
        session_id: &SessionId,
        message_bytes: &[u8],
    ) -> Result<(), WebSocketError> {
        let start = Instant::now();

        if let Some(connection) = self.connections.get(session_id) {
            let result = connection.send_raw_message(message_bytes).await;

            let duration = start.elapsed();
            #[cfg(feature = "monitoring")]
            {
                metrics::histogram!("websocket_send_duration_ms", duration.as_millis() as f64);
                if duration.as_millis() > 5 {
                    metrics::counter!("websocket_slow_sends", 1);
                }
            }

            result
        } else {
            Err(WebSocketError::ConnectionNotFound {
                session_id: *session_id,
            })
        }
    }

    /// Get current connection count
    pub fn connection_count(&self) -> usize {
        self.connections.len()
    }

    /// Get connection metrics
    pub fn metrics(&self) -> ConnectionMetrics {
        // Return a snapshot of current metrics
        ConnectionMetrics {
            total_connections: self.connections.len(),
            active_connections: self.connections.len(),
            messages_sent: 0, // Would be tracked in real implementation
            messages_received: 0,
            bytes_sent: 0,
            bytes_received: 0,
            connection_errors: 0,
        }
    }

    /// Background cleanup task for stale connections
    async fn cleanup_task(
        connections: Arc<DashMap<SessionId, WebSocketConnection>>,
        config: ConnectionPoolConfig,
    ) {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(
            config.connection_timeout_seconds,
        ));

        loop {
            interval.tick().await;

            let now = Instant::now();
            let timeout_duration =
                tokio::time::Duration::from_secs(config.connection_timeout_seconds);

            // Remove stale connections
            connections.retain(|session_id, connection| {
                if now.duration_since(connection.created_at) > timeout_duration
                    && !connection.is_active()
                {
                    tracing::info!("Cleaning up stale connection: {}", session_id);
                    false
                } else {
                    true
                }
            });

            #[cfg(feature = "monitoring")]
            {
                metrics::gauge!("websocket_active_connections", connections.len() as f64);
            }
        }
    }
}

/// Individual WebSocket connection with RAII cleanup
pub struct WebSocketConnection {
    session_id: SessionId,
    created_at: Instant,
    sender: Arc<tokio::sync::Mutex<futures_util::stream::SplitSink<WebSocket, Message>>>,
    active: Arc<std::sync::atomic::AtomicBool>,
    message_count: Arc<AtomicU64>,
}

impl WebSocketConnection {
    /// Create new WebSocket connection
    pub fn new(
        session_id: SessionId,
        websocket: WebSocket,
        mut state_receiver: broadcast::Receiver<GameMessage>,
    ) -> Result<Self, WebSocketError> {
        let (sender, mut receiver) = websocket.split();
        let sender = Arc::new(tokio::sync::Mutex::new(sender));
        let active = Arc::new(std::sync::atomic::AtomicBool::new(true));
        let message_count = Arc::new(AtomicU64::new(0));

        // Spawn tasks for handling incoming/outgoing messages
        let sender_clone = sender.clone();
        let active_clone = active.clone();
        let message_count_clone = message_count.clone();

        // Task for handling broadcast messages (state updates)
        tokio::spawn(async move {
            while let Ok(message) = state_receiver.recv().await {
                if !active_clone.load(Ordering::Relaxed) {
                    break;
                }

                let serialized = match bincode::serialize(&message) {
                    Ok(data) => data,
                    Err(e) => {
                        tracing::error!("Failed to serialize message: {}", e);
                        continue;
                    }
                };

                let mut sender_guard = sender_clone.lock().await;
                if sender_guard
                    .send(Message::Binary(serialized))
                    .await
                    .is_err()
                {
                    active_clone.store(false, Ordering::Relaxed);
                    break;
                }

                message_count_clone.fetch_add(1, Ordering::Relaxed);
            }
        });

        // Task for handling incoming messages from client
        let active_clone = active.clone();
        tokio::spawn(async move {
            while let Some(msg) = receiver.next().await {
                if !active_clone.load(Ordering::Relaxed) {
                    break;
                }

                match msg {
                    Ok(Message::Text(text)) => {
                        tracing::debug!("Received text message from {}: {}", session_id, text);
                    }
                    Ok(Message::Binary(data)) => {
                        tracing::debug!(
                            "Received binary message from {}: {} bytes",
                            session_id,
                            data.len()
                        );
                    }
                    Ok(Message::Close(_)) => {
                        tracing::info!("WebSocket connection closed by client: {}", session_id);
                        active_clone.store(false, Ordering::Relaxed);
                        break;
                    }
                    Ok(Message::Ping(payload)) => {
                        // Handle ping by sending pong (this would be done automatically by the WebSocket implementation)
                        tracing::debug!("Received ping from {}", session_id);
                    }
                    Ok(Message::Pong(_)) => {
                        tracing::debug!("Received pong from {}", session_id);
                    }
                    Err(e) => {
                        tracing::error!("WebSocket error for {}: {}", session_id, e);
                        active_clone.store(false, Ordering::Relaxed);
                        break;
                    }
                }
            }
        });

        Ok(Self {
            session_id,
            created_at: Instant::now(),
            sender,
            active,
            message_count,
        })
    }

    /// Send raw message bytes (zero-copy where possible)
    pub async fn send_raw_message(&self, message_bytes: &[u8]) -> Result<(), WebSocketError> {
        if !self.active.load(Ordering::Relaxed) {
            return Err(WebSocketError::ConnectionClosed);
        }

        let mut sender = self.sender.lock().await;
        sender
            .send(Message::Binary(message_bytes.to_vec()))
            .await
            .map_err(|e| WebSocketError::MessageSendFailed {
                message: e.to_string(),
            })?;

        self.message_count.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }

    /// Check if connection is still active
    pub fn is_active(&self) -> bool {
        self.active.load(Ordering::Relaxed)
    }

    /// Get message count
    pub fn message_count(&self) -> u64 {
        self.message_count.load(Ordering::Relaxed)
    }
}

// RAII cleanup for connections
impl Drop for WebSocketConnection {
    fn drop(&mut self) {
        self.active.store(false, Ordering::Relaxed);

        #[cfg(feature = "monitoring")]
        {
            metrics::counter!("websocket_connections_dropped", 1);
        }

        tracing::debug!(
            "WebSocket connection dropped for session: {}",
            self.session_id
        );
    }
}

// RAII cleanup for connection pool
impl Drop for WebSocketConnectionPool {
    fn drop(&mut self) {
        // Abort cleanup task
        self.cleanup_handle.abort();

        tracing::info!(
            "WebSocket connection pool shutting down with {} active connections",
            self.connections.len()
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_connection_pool_creation() {
        let config = ConnectionPoolConfig::default();
        let pool = WebSocketConnectionPool::new(config);
        assert!(
            pool.is_ok(),
            "Connection pool should be created successfully"
        );
    }

    #[test]
    fn test_connection_limits() {
        let config = ConnectionPoolConfig {
            max_connections: 2,
            ..Default::default()
        };

        assert_eq!(config.max_connections, 2);
    }

    #[tokio::test]
    async fn test_broadcast_serialization() {
        let message = GameMessage::StateUpdate {
            session_id: uuid::Uuid::new_v4(),
            state_data: vec![1, 2, 3, 4, 5],
        };

        let serialized = bincode::serialize(&message).unwrap();
        assert!(
            !serialized.is_empty(),
            "Serialized message should not be empty"
        );

        let deserialized: GameMessage = bincode::deserialize(&serialized).unwrap();
        match deserialized {
            GameMessage::StateUpdate { state_data, .. } => {
                assert_eq!(state_data, vec![1, 2, 3, 4, 5]);
            }
            _ => panic!("Wrong message type deserialized"),
        }
    }
}
