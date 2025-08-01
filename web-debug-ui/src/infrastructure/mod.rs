#![allow(missing_docs)]
//! Infrastructure Foundation - Sprint 1
//!
//! High-performance Rust infrastructure with Axum HTTP server and WebSocket support.
//! Designed to meet brutal performance requirements that eliminate the Python bridge.
//!
//! ## Performance Requirements (NON-NEGOTIABLE)
//! - **Action Latency**: <10ms end-to-end (this is the critical path!)
//! - **State Updates**: <5ms via WebSocket
//! - **Memory Usage**: <20MB per session (no Python garbage collection!)
//! - **Concurrent Connections**: 100+ simultaneous WebSocket connections
//! - **Zero-Copy**: Minimize allocations, stack allocation preferred

#![cfg(feature = "infrastructure")]

pub mod http;
pub mod metrics;
pub mod serialization;
pub mod storage;
pub mod websocket;

use anyhow::Result;
use std::sync::Arc;

/// Central configuration for the infrastructure foundation
#[derive(Debug, Clone)]
#[derive(Default)]
pub struct InfrastructureConfig {
    /// HTTP server configuration
    pub http_config: self::http::ServerConfig,
    /// WebSocket connection pool configuration
    pub websocket_config: self::websocket::ConnectionPoolConfig,
    /// Memory store configuration
    pub storage_config: self::storage::StoreConfig,
    /// Performance monitoring configuration
    pub metrics_config: self::metrics::MetricsConfig,
}

/// High-performance infrastructure error types
#[derive(thiserror::Error, Debug)]
pub enum InfrastructureError {
    #[error("HTTP server error: {0}")]
    HttpServer(#[from] self::http::HttpServerError),

    #[error("WebSocket connection error: {0}")]
    WebSocket(#[from] self::websocket::WebSocketError),

    #[error("Serialization error: {0}")]
    Serialization(#[from] self::serialization::SerializationError),

    #[error("Storage error: {0}")]
    Storage(#[from] self::storage::StorageError),

    #[error("Metrics error: {0}")]
    Metrics(#[from] self::metrics::MetricsError),

    #[error("Configuration error: {message}")]
    Configuration { message: String },

    #[error(
        "Performance threshold exceeded: {operation} took {duration_ms}ms (limit: {limit_ms}ms)"
    )]
    PerformanceThreshold {
        operation: String,
        duration_ms: u64,
        limit_ms: u64,
    },
}

/// Session identifier for tracking connections and game states
pub type SessionId = ::uuid::Uuid;

/// Initialize the infrastructure foundation with RAII patterns
pub async fn initialize(config: InfrastructureConfig) -> Result<InfrastructureFoundation> {
    #[cfg(feature = "monitoring")]
    {
        // Initialize metrics system first for monitoring initialization
        let metrics_handle = self::metrics::initialize_metrics(&config.metrics_config)?;
        tracing::info!("Infrastructure metrics initialized");

        // Initialize other components with monitoring
        let storage = Arc::new(self::storage::HighPerformanceMemoryStore::new(
            config.storage_config,
        )?);
        tracing::info!("High-performance memory store initialized");

        let websocket_pool = Arc::new(self::websocket::WebSocketConnectionPool::new(
            config.websocket_config,
        )?);
        tracing::info!("WebSocket connection pool initialized");

        let http_server = self::http::HttpServer::new(
            config.http_config,
            websocket_pool.clone(),
            storage.clone(),
        )
        .await?;
        tracing::info!("Axum HTTP server initialized");

        Ok(InfrastructureFoundation {
            http_server,
            websocket_pool,
            storage,
            metrics_handle: Some(metrics_handle),
        })
    }

    #[cfg(not(feature = "monitoring"))]
    {
        // Initialize without metrics for minimal overhead
        let storage = Arc::new(self::storage::HighPerformanceMemoryStore::new(
            config.storage_config,
        )?);
        let websocket_pool = Arc::new(self::websocket::WebSocketConnectionPool::new(
            config.websocket_config,
        )?);
        let http_server = self::http::HttpServer::new(
            config.http_config,
            websocket_pool.clone(),
            storage.clone(),
        )
        .await?;

        Ok(InfrastructureFoundation {
            http_server,
            websocket_pool,
            storage,
            metrics_handle: None,
        })
    }
}

/// Infrastructure foundation with RAII cleanup
pub struct InfrastructureFoundation {
    pub http_server: self::http::HttpServer,
    pub websocket_pool: Arc<self::websocket::WebSocketConnectionPool>,
    pub storage: Arc<self::storage::HighPerformanceMemoryStore>,
    #[cfg(feature = "monitoring")]
    pub metrics_handle: Option<self::metrics::MetricsHandle>,
    #[cfg(not(feature = "monitoring"))]
    pub metrics_handle: Option<()>,
}

impl InfrastructureFoundation {
    /// Start the infrastructure foundation
    pub async fn start(&self, bind_addr: &str) -> Result<()> {
        // Note: In a real implementation, we'd need to handle server lifecycle properly
        // For now, this is a stub that would start the server
        tracing::info!(
            "Infrastructure foundation would start HTTP server on {}",
            bind_addr
        );
        Ok(())
    }

    /// Get infrastructure health status for monitoring
    pub fn health_status(&self) -> InfrastructureHealth {
        InfrastructureHealth {
            websocket_connections: self.websocket_pool.connection_count(),
            memory_usage_mb: self.storage.memory_usage_mb(),
            uptime_seconds: self.storage.uptime_seconds(),
        }
    }
}

/// Infrastructure health status for monitoring
#[derive(Debug, Clone, serde::Serialize)]
pub struct InfrastructureHealth {
    pub websocket_connections: usize,
    pub memory_usage_mb: f64,
    pub uptime_seconds: u64,
}

// Ensure proper cleanup with Drop
impl Drop for InfrastructureFoundation {
    fn drop(&mut self) {
        #[cfg(feature = "monitoring")]
        {
            if let Some(_handle) = &self.metrics_handle {
                tracing::info!("Infrastructure foundation shutting down gracefully");
                // Metrics handle will auto-cleanup
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_infrastructure_initialization() {
        let config = InfrastructureConfig::default();
        let result = initialize(config).await;
        assert!(
            result.is_ok(),
            "Infrastructure should initialize successfully"
        );
    }

    #[test]
    fn test_infrastructure_config_default() {
        let config = InfrastructureConfig::default();
        // Verify all configs have reasonable defaults
        assert_eq!(config.http_config.max_connections, 1000);
        assert_eq!(config.websocket_config.max_connections, 100);
        assert!(config.storage_config.max_memory_mb > 0);
    }
}
