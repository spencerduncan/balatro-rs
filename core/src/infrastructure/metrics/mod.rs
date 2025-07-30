//! Performance Monitoring & Metrics
//!
//! CRITICAL PERFORMANCE TARGETS:
//! - Real-time performance monitoring
//! - <1ms metrics collection overhead
//! - Prometheus export for observability

#![cfg(feature = "monitoring")]

pub mod performance_monitor;

pub use performance_monitor::{PerformanceMonitor, OperationTimer, MetricsHandle};

use std::time::Instant;

/// Metrics error types
#[derive(thiserror::Error, Debug)]
pub enum MetricsError {
    #[error("Metrics initialization failed: {message}")]
    InitializationFailed { message: String },

    #[error("Metrics export failed: {message}")]
    ExportFailed { message: String },

    #[error("Invalid metric name: {name}")]
    InvalidMetricName { name: String },

    #[error("Performance monitoring disabled")]
    MonitoringDisabled,
}

/// Metrics configuration
#[derive(Debug, Clone)]
pub struct MetricsConfig {
    /// Enable Prometheus metrics export
    pub prometheus_enabled: bool,
    /// Prometheus export port
    pub prometheus_port: u16,
    /// Metrics collection interval in seconds
    pub collection_interval_seconds: u64,
    /// Enable high-resolution timing (nanosecond precision)
    pub high_resolution_timing: bool,
    /// Maximum number of metrics to keep in memory
    pub max_metrics_count: usize,
}

impl Default for MetricsConfig {
    fn default() -> Self {
        Self {
            prometheus_enabled: true,
            prometheus_port: 9090,
            collection_interval_seconds: 10,
            high_resolution_timing: true,
            max_metrics_count: 10000,
        }
    }
}

/// Initialize metrics system with configuration
pub fn initialize_metrics(config: &MetricsConfig) -> Result<MetricsHandle, MetricsError> {
    #[cfg(feature = "monitoring")]
    {
        // Initialize metrics recorder
        let recorder = metrics_exporter_prometheus::PrometheusBuilder::new()
            .with_http_listener(([0, 0, 0, 0], config.prometheus_port))
            .build()
            .map_err(|e| MetricsError::InitializationFailed {
                message: e.to_string(),
            })?;

        metrics::set_boxed_recorder(Box::new(recorder))
            .map_err(|e| MetricsError::InitializationFailed {
                message: e.to_string(),
            })?;

        // Register core infrastructure metrics
        register_infrastructure_metrics();

        tracing::info!("Metrics system initialized with Prometheus export on port {}", config.prometheus_port);

        Ok(MetricsHandle::new(config.clone()))
    }

    #[cfg(not(feature = "monitoring"))]
    {
        Err(MetricsError::MonitoringDisabled)
    }
}

/// Register core infrastructure metrics
#[cfg(feature = "monitoring")]
fn register_infrastructure_metrics() {
    // HTTP server metrics
    metrics::describe_counter!("http_requests_total", "Total HTTP requests");
    metrics::describe_histogram!("http_request_duration_ms", "HTTP request duration in milliseconds");
    metrics::describe_gauge!("http_active_connections", "Active HTTP connections");

    // WebSocket metrics
    metrics::describe_counter!("websocket_connections_opened", "WebSocket connections opened");
    metrics::describe_counter!("websocket_connections_closed", "WebSocket connections closed");
    metrics::describe_gauge!("websocket_active_connections", "Active WebSocket connections");
    metrics::describe_histogram!("websocket_message_duration_ms", "WebSocket message processing duration");
    metrics::describe_counter!("websocket_messages_sent", "WebSocket messages sent");
    metrics::describe_counter!("websocket_messages_received", "WebSocket messages received");

    // Action processing metrics (CRITICAL PATH)
    metrics::describe_histogram!("action_execution_duration_ms", "Action execution duration in milliseconds");
    metrics::describe_counter!("actions_processed_total", "Total actions processed");
    metrics::describe_counter!("slow_actions_total", "Actions that exceeded performance threshold");

    // Storage metrics
    metrics::describe_gauge!("storage_active_sessions", "Active storage sessions");
    metrics::describe_histogram!("storage_operation_duration_ms", "Storage operation duration");
    metrics::describe_gauge!("storage_memory_usage_mb", "Storage memory usage in MB");
    metrics::describe_counter!("storage_sessions_created", "Sessions created");
    metrics::describe_counter!("storage_sessions_removed", "Sessions removed");

    // Memory metrics
    metrics::describe_gauge!("memory_usage_bytes", "Memory usage in bytes");
    metrics::describe_gauge!("memory_peak_usage_bytes", "Peak memory usage in bytes");

    // Performance threshold violations
    metrics::describe_counter!("performance_violations_total", "Performance threshold violations");

    tracing::debug!("Infrastructure metrics registered");
}

/// Metrics handle for managing the metrics system
pub struct MetricsHandle {
    config: MetricsConfig,
    start_time: Instant,
}

impl MetricsHandle {
    fn new(config: MetricsConfig) -> Self {
        Self {
            config,
            start_time: Instant::now(),
        }
    }

    /// Get uptime in seconds
    pub fn uptime_seconds(&self) -> f64 {
        self.start_time.elapsed().as_secs_f64()
    }

    /// Get metrics configuration
    pub fn config(&self) -> &MetricsConfig {
        &self.config
    }

    /// Record performance violation
    pub fn record_performance_violation(&self, operation: &str, duration_ms: u64, threshold_ms: u64) {
        #[cfg(feature = "monitoring")]
        {
            metrics::counter!("performance_violations_total", 1, "operation" => operation.to_string());
            tracing::warn!("Performance violation: {} took {}ms (threshold: {}ms)", operation, duration_ms, threshold_ms);
        }
    }

    /// Export current metrics as Prometheus format
    pub fn export_prometheus(&self) -> Result<String, MetricsError> {
        #[cfg(feature = "monitoring")]
        {
            // This would return the current Prometheus metrics
            // For now, return a placeholder
            Ok("# Infrastructure metrics placeholder\n".to_string())
        }

        #[cfg(not(feature = "monitoring"))]
        {
            Err(MetricsError::MonitoringDisabled)
        }
    }
}

// Ensure proper cleanup
impl Drop for MetricsHandle {
    fn drop(&mut self) {
        let uptime = self.uptime_seconds();
        tracing::info!("Metrics system shutting down after {:.2} seconds", uptime);

        #[cfg(feature = "monitoring")]
        {
            metrics::gauge!("infrastructure_uptime_seconds", uptime);
        }
    }
}

/// Convenience macros for common performance monitoring patterns
#[macro_export]
macro_rules! time_operation {
    ($operation:expr, $code:block) => {{
        let start = std::time::Instant::now();
        let result = $code;
        let duration = start.elapsed();

        #[cfg(feature = "monitoring")]
        {
            metrics::histogram!(
                concat!($operation, "_duration_ms"),
                duration.as_millis() as f64
            );

            if duration.as_millis() > 10 {
                metrics::counter!(
                    concat!($operation, "_slow_operations"), 1
                );
            }
        }

        result
    }};
}

#[macro_export]
macro_rules! record_performance_critical {
    ($operation:expr, $threshold_ms:expr, $code:block) => {{
        let start = std::time::Instant::now();
        let result = $code;
        let duration = start.elapsed();
        let duration_ms = duration.as_millis() as u64;

        #[cfg(feature = "monitoring")]
        {
            metrics::histogram!(
                concat!($operation, "_duration_ms"),
                duration_ms as f64
            );

            if duration_ms > $threshold_ms {
                metrics::counter!(
                    "performance_violations_total", 1,
                    "operation" => $operation,
                    "threshold_ms" => $threshold_ms.to_string(),
                    "actual_ms" => duration_ms.to_string()
                );

                tracing::warn!(
                    "PERFORMANCE VIOLATION: {} took {}ms (threshold: {}ms)",
                    $operation, duration_ms, $threshold_ms
                );
            }
        }

        result
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metrics_config_defaults() {
        let config = MetricsConfig::default();

        assert!(config.prometheus_enabled);
        assert_eq!(config.prometheus_port, 9090);
        assert_eq!(config.collection_interval_seconds, 10);
        assert!(config.high_resolution_timing);
        assert_eq!(config.max_metrics_count, 10000);
    }

    #[test]
    fn test_metrics_config_customization() {
        let config = MetricsConfig {
            prometheus_port: 8080,
            high_resolution_timing: false,
            ..Default::default()
        };

        assert_eq!(config.prometheus_port, 8080);
        assert!(!config.high_resolution_timing);
        assert!(config.prometheus_enabled); // Should still be default
    }

    #[tokio::test]
    async fn test_time_operation_macro() {
        let result = time_operation!("test_operation", {
            tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
            42
        });

        assert_eq!(result, 42);
    }

    #[tokio::test]
    async fn test_performance_critical_macro() {
        let result = record_performance_critical!("critical_test", 5, {
            tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
            "success"
        });

        assert_eq!(result, "success");
    }

    #[test]
    fn test_metrics_handle_creation() {
        let config = MetricsConfig::default();
        let handle = MetricsHandle::new(config);

        assert!(handle.uptime_seconds() >= 0.0);
        assert_eq!(handle.config().prometheus_port, 9090);
    }
}
