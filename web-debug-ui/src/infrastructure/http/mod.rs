#![allow(missing_docs)]
//! High-Performance Axum HTTP Server
//!
//! Designed for <10ms action latency and 100+ concurrent connections.

#![cfg(feature = "http-server")]

pub mod server;

pub use server::HttpServer;

use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde_json::{json, Value};
use std::sync::Arc;

/// HTTP server error types
#[derive(thiserror::Error, Debug)]
pub enum HttpServerError {
    #[error("Server startup failed: {message}")]
    Startup { message: String },

    #[error("Request processing failed: {message}")]
    RequestProcessing { message: String },

    #[error("WebSocket upgrade failed: {message}")]
    WebSocketUpgrade { message: String },

    #[error("Performance threshold exceeded: {operation} took {duration_ms}ms")]
    PerformanceThreshold { operation: String, duration_ms: u64 },
}

/// HTTP server configuration with performance optimizations
#[derive(Debug, Clone)]
pub struct ServerConfig {
    /// Maximum concurrent connections
    pub max_connections: usize,
    /// Request timeout in milliseconds
    pub request_timeout_ms: u64,
    /// Keep-alive timeout in seconds
    pub keep_alive_seconds: u64,
    /// Enable HTTP/2
    pub http2_enabled: bool,
    /// TCP nodelay for low latency
    pub tcp_nodelay: bool,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            max_connections: 1000,
            request_timeout_ms: 5000, // 5 second timeout
            keep_alive_seconds: 60,
            http2_enabled: true,
            tcp_nodelay: true, // Disable Nagle's algorithm for low latency
        }
    }
}

/// Create the core HTTP router with all endpoints
pub fn create_router(
    websocket_pool: Arc<crate::infrastructure::websocket::WebSocketConnectionPool>,
    storage: Arc<crate::infrastructure::storage::HighPerformanceMemoryStore>,
) -> Router {
    Router::new()
        .route("/", get(health_check))
        .route("/health", get(detailed_health))
        .route("/metrics", get(metrics_endpoint))
        .route("/ws", get(websocket_upgrade))
        .route("/api/session", post(create_session))
        .route("/api/session/:id/action", post(handle_action))
        .route("/api/session/:id/state", get(get_session_state))
        .with_state(AppState {
            websocket_pool,
            storage,
        })
}

/// Application state shared across handlers
#[derive(Clone)]
pub struct AppState {
    pub websocket_pool: Arc<crate::infrastructure::websocket::WebSocketConnectionPool>,
    pub storage: Arc<crate::infrastructure::storage::HighPerformanceMemoryStore>,
}

/// Health check endpoint
async fn health_check() -> &'static str {
    "OK"
}

/// Detailed health endpoint with metrics
async fn detailed_health(State(state): State<AppState>) -> Json<Value> {
    let health = crate::infrastructure::InfrastructureHealth {
        websocket_connections: state.websocket_pool.connection_count(),
        memory_usage_mb: state.storage.memory_usage_mb(),
        uptime_seconds: state.storage.uptime_seconds(),
    };

    Json(json!({
        "status": "healthy",
        "websocket_connections": health.websocket_connections,
        "memory_usage_mb": health.memory_usage_mb,
        "uptime_seconds": health.uptime_seconds,
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

/// Prometheus metrics endpoint
async fn metrics_endpoint() -> Result<String, StatusCode> {
    #[cfg(feature = "monitoring")]
    {
        // Export Prometheus metrics
        // This would need to be properly implemented with a global metrics handle
        Ok("# Metrics not fully implemented yet\n".to_string())
    }

    #[cfg(not(feature = "monitoring"))]
    {
        Err(StatusCode::NOT_FOUND)
    }
}

/// WebSocket upgrade endpoint
async fn websocket_upgrade(
    ws: axum::extract::WebSocketUpgrade,
    State(state): State<AppState>,
) -> axum::response::Response {
    ws.on_upgrade(move |socket| async move {
        // Generate new session ID
        let session_id = uuid::Uuid::new_v4();

        // Add connection to pool
        if let Err(e) = state
            .websocket_pool
            .add_connection(session_id, socket)
            .await
        {
            tracing::error!("Failed to add WebSocket connection: {}", e);
        }
    })
}

/// Create new game session
async fn create_session(State(state): State<AppState>) -> Result<Json<Value>, StatusCode> {
    let session_id = uuid::Uuid::new_v4();

    // Create new game session in storage
    // This would integrate with the actual game engine
    match state.storage.create_session(session_id).await {
        Ok(_) => Ok(Json(json!({
            "session_id": session_id,
            "status": "created"
        }))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// Handle game action (PERFORMANCE CRITICAL - <10ms target)
async fn handle_action(
    axum::extract::Path(session_id): axum::extract::Path<uuid::Uuid>,
    State(state): State<AppState>,
    Json(action_data): Json<Value>,
) -> Result<Json<Value>, StatusCode> {
    use std::time::Instant;
    let start = Instant::now();

    // This is the critical path - must be <10ms end-to-end
    match state
        .storage
        .handle_session_action(session_id, action_data)
        .await
    {
        Ok(result) => {
            let duration = start.elapsed();

            #[cfg(feature = "monitoring")]
            {
                metrics::histogram!("action_execution_duration_ms")
                    .record(duration.as_millis() as f64);
                if duration.as_millis() > 10 {
                    metrics::counter!("slow_actions_total").increment(1);
                    tracing::warn!(
                        "Slow action execution: {}ms for session {}",
                        duration.as_millis(),
                        session_id
                    );
                }
            }

            Ok(Json(result))
        }
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}

/// Get session state
async fn get_session_state(
    axum::extract::Path(session_id): axum::extract::Path<uuid::Uuid>,
    State(state): State<AppState>,
) -> Result<Json<Value>, StatusCode> {
    match state.storage.get_session_state(session_id).await {
        Ok(Some(state_data)) => Ok(Json(state_data)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum_test::TestServer;

    #[tokio::test]
    async fn test_health_endpoint() {
        // This would need proper setup with mock storage and websocket pool
        // For now, just test that the router can be created
        let storage = Arc::new(
            crate::infrastructure::storage::HighPerformanceMemoryStore::new(
                crate::infrastructure::storage::StoreConfig::default(),
            )
            .unwrap(),
        );
        let websocket_pool = Arc::new(
            crate::infrastructure::websocket::WebSocketConnectionPool::new(
                crate::infrastructure::websocket::ConnectionPoolConfig::default(),
            )
            .unwrap(),
        );

        let router = create_router(websocket_pool, storage);
        let server = TestServer::new(router).unwrap();

        let response = server.get("/health").await;
        assert!(response.status_code().is_success());
    }
}
