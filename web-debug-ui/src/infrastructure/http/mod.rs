//! High-Performance Axum HTTP Server
//!
//! Designed for <10ms action latency and 100+ concurrent connections.

#![cfg(feature = "http-server")]

pub mod server;

pub use server::{HttpServer, HttpServerError, ServerConfig};

use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, Json},
    routing::{get, post},
    Router,
};
use serde_json::{json, Value};
use std::sync::Arc;

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
        use metrics_exporter_prometheus::PrometheusHandle;
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
                metrics::histogram!("action_execution_duration_ms", duration.as_millis() as f64);
                if duration.as_millis() > 10 {
                    metrics::counter!("slow_actions_total", 1);
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
    // use axum_test::TestServer;  // Disabled due to UUID version conflict

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
