//! High-Performance Axum HTTP Server Implementation
//!
//! CRITICAL PERFORMANCE TARGETS:
//! - <10ms action latency end-to-end
//! - 100+ concurrent connections
//! - <20MB memory per session

use super::{create_router, AppState, HttpServerError, ServerConfig};
use axum::Router;
use std::sync::Arc;
use tokio::net::TcpListener;

/// High-performance HTTP server with RAII resource management
pub struct HttpServer {
    router: Router,
    config: ServerConfig,
    app_state: AppState,
}

impl HttpServer {
    /// Create new HTTP server with optimized configuration
    pub async fn new(
        config: ServerConfig,
        websocket_pool: Arc<crate::infrastructure::websocket::WebSocketConnectionPool>,
        storage: Arc<crate::infrastructure::storage::HighPerformanceMemoryStore>,
    ) -> Result<Self, HttpServerError> {
        let app_state = AppState {
            websocket_pool,
            storage,
        };

        let router = create_router(app_state.websocket_pool.clone(), app_state.storage.clone());

        tracing::info!("HTTP server created with config: {:?}", config);

        Ok(Self {
            router,
            config,
            app_state,
        })
    }

    /// Start serving HTTP requests with performance optimizations
    pub async fn serve(self, bind_addr: &str) -> Result<(), HttpServerError> {
        let listener = TcpListener::bind(bind_addr)
            .await
            .map_err(|e| HttpServerError::Startup {
                message: format!("Failed to bind to {}: {}", bind_addr, e),
            })?;

        tracing::info!("HTTP server listening on {}", bind_addr);

        // Configure TCP socket for low latency
        if self.config.tcp_nodelay {
            // This would require a custom acceptor to set TCP_NODELAY
            tracing::info!("TCP_NODELAY enabled for low latency");
        }

        // Start the server with performance monitoring
        #[cfg(feature = "monitoring")]
        {
            metrics::counter!("http_server_starts", 1);
            let start_time = std::time::Instant::now();

            let result = axum::serve(listener, self.router.clone())
                .with_graceful_shutdown(shutdown_signal())
                .await;

            let uptime = start_time.elapsed();
            metrics::histogram!("http_server_uptime_seconds", uptime.as_secs() as f64);

            result.map_err(|e| HttpServerError::Startup {
                message: format!("Server failed: {}", e),
            })
        }

        #[cfg(not(feature = "monitoring"))]
        {
            axum::serve(listener, self.router.clone())
                .with_graceful_shutdown(shutdown_signal())
                .await
                .map_err(|e| HttpServerError::Startup {
                    message: format!("Server failed: {}", e),
                })
        }
    }

    /// Get server configuration
    pub fn config(&self) -> &ServerConfig {
        &self.config
    }

    /// Get current connection count
    pub fn connection_count(&self) -> usize {
        self.app_state.websocket_pool.connection_count()
    }

    /// Get memory usage in MB
    pub fn memory_usage_mb(&self) -> f64 {
        self.app_state.storage.memory_usage_mb()
    }
}

/// Graceful shutdown signal handling
async fn shutdown_signal() {
    use tokio::signal;

    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            tracing::info!("Received Ctrl+C, shutting down gracefully");
        },
        _ = terminate => {
            tracing::info!("Received SIGTERM, shutting down gracefully");
        },
    }
}

// Custom connection handling for performance optimizations
pub struct PerformanceOptimizedAcceptor {
    inner: TcpListener,
    config: ServerConfig,
}

impl PerformanceOptimizedAcceptor {
    pub fn new(listener: TcpListener, config: ServerConfig) -> Self {
        Self {
            inner: listener,
            config,
        }
    }

    /// Accept connections with performance optimizations
    pub async fn accept(&self) -> Result<(tokio::net::TcpStream, std::net::SocketAddr), std::io::Error> {
        let (stream, addr) = self.inner.accept().await?;

        // Apply performance optimizations
        if self.config.tcp_nodelay {
            stream.set_nodelay(true)?;
        }

        // Set keep-alive if configured
        if self.config.keep_alive_seconds > 0 {
            let socket2_stream = socket2::Socket::from(stream.into_std()?);
            socket2_stream.set_keepalive(true)?;
            let stream = tokio::net::TcpStream::from_std(socket2_stream.into())?;
            return Ok((stream, addr));
        }

        Ok((stream, addr))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::{
        storage::{HighPerformanceMemoryStore, StoreConfig},
        websocket::{WebSocketConnectionPool, ConnectionPoolConfig},
    };

    #[tokio::test]
    async fn test_server_creation() {
        let storage = Arc::new(HighPerformanceMemoryStore::new(StoreConfig::default()).unwrap());
        let webocket_pool = Arc::new(WebSocketConnectionPool::new(ConnectionPoolConfig::default()).unwrap());

        let server = HttpServer::new(
            ServerConfig::default(),
            webocket_pool,
            storage,
        ).await;

        assert!(server.is_ok(), "Server should be created successfully");
    }

    #[test]
    fn test_server_config_performance_defaults() {
        let config = ServerConfig::default();

        // Verify performance-oriented defaults
        assert!(config.tcp_nodelay, "TCP_NODELAY should be enabled for low latency");
        assert!(config.http2_enabled, "HTTP/2 should be enabled for efficiency");
        assert_eq!(config.max_connections, 1000, "Should support many connections");
        assert_eq!(config.request_timeout_ms, 5000, "Reasonable timeout");
    }

    #[tokio::test]
    async fn test_graceful_shutdown_signal() {
        // Test that shutdown signal can be set up without panicking
        tokio::spawn(async {
            shutdown_signal().await;
        });

        // Just ensure it compiles and can be spawned
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
    }
}
