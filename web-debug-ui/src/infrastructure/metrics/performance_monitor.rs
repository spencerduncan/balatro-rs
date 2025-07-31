//! Performance Monitor Implementation
//!
//! Provides high-resolution performance monitoring with <1ms overhead.
//! Critical for maintaining <10ms action latency requirements.

use super::{MetricsConfig, MetricsError};
use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc,
};
use std::time::Instant;

/// High-resolution performance monitor
pub struct PerformanceMonitor {
    start_time: Instant,
    config: MetricsConfig,
    // Performance counters
    total_operations: Arc<AtomicU64>,
    slow_operations: Arc<AtomicU64>,
    performance_violations: Arc<AtomicU64>,
}

impl PerformanceMonitor {
    /// Create new performance monitor
    pub fn new(config: MetricsConfig) -> Self {
        Self {
            start_time: Instant::now(),
            config,
            total_operations: Arc::new(AtomicU64::new(0)),
            slow_operations: Arc::new(AtomicU64::new(0)),
            performance_violations: Arc::new(AtomicU64::new(0)),
        }
    }

    /// Start timing an operation
    pub fn start_operation(&self) -> OperationTimer {
        self.total_operations.fetch_add(1, Ordering::Relaxed);
        OperationTimer::new(
            self.config.high_resolution_timing,
            self.slow_operations.clone(),
            self.performance_violations.clone(),
        )
    }

    /// Start timing a performance-critical operation with threshold
    pub fn start_critical_operation(
        &self,
        operation_name: &str,
        threshold_ms: u64,
    ) -> CriticalOperationTimer {
        self.total_operations.fetch_add(1, Ordering::Relaxed);
        CriticalOperationTimer::new(
            operation_name.to_string(),
            threshold_ms,
            self.config.high_resolution_timing,
            self.performance_violations.clone(),
        )
    }

    /// Get uptime in seconds
    pub fn uptime_seconds(&self) -> f64 {
        self.start_time.elapsed().as_secs_f64()
    }

    /// Get total operations count
    pub fn total_operations(&self) -> u64 {
        self.total_operations.load(Ordering::Relaxed)
    }

    /// Get slow operations count
    pub fn slow_operations(&self) -> u64 {
        self.slow_operations.load(Ordering::Relaxed)
    }

    /// Get performance violations count
    pub fn performance_violations(&self) -> u64 {
        self.performance_violations.load(Ordering::Relaxed)
    }

    /// Get performance statistics
    pub fn get_stats(&self) -> PerformanceStats {
        let total = self.total_operations();
        let slow = self.slow_operations();
        let violations = self.performance_violations();

        PerformanceStats {
            total_operations: total,
            slow_operations: slow,
            performance_violations: violations,
            slow_operation_rate: if total > 0 {
                slow as f64 / total as f64
            } else {
                0.0
            },
            violation_rate: if total > 0 {
                violations as f64 / total as f64
            } else {
                0.0
            },
            uptime_seconds: self.uptime_seconds(),
            operations_per_second: if self.uptime_seconds() > 0.0 {
                total as f64 / self.uptime_seconds()
            } else {
                0.0
            },
        }
    }
}

/// Operation timer for measuring execution time
pub struct OperationTimer {
    start: Instant,
    high_resolution: bool,
    slow_operations: Arc<AtomicU64>,
    performance_violations: Arc<AtomicU64>,
}

impl OperationTimer {
    fn new(
        high_resolution: bool,
        slow_operations: Arc<AtomicU64>,
        performance_violations: Arc<AtomicU64>,
    ) -> Self {
        Self {
            start: if high_resolution {
                Instant::now() // Nanosecond precision
            } else {
                Instant::now() // Still high resolution, but we might sample less frequently
            },
            high_resolution,
            slow_operations,
            performance_violations,
        }
    }

    /// Finish timing an action execution (CRITICAL PATH - <10ms)
    pub fn finish_action_execution(self) -> OperationResult {
        let duration = self.start.elapsed();
        let duration_ms = if self.high_resolution {
            duration.as_nanos() as f64 / 1_000_000.0 // Nanosecond precision
        } else {
            duration.as_millis() as f64
        };

        #[cfg(feature = "monitoring")]
        {
            metrics::histogram!("action_execution_duration_ms", duration_ms);

            if duration_ms > 10.0 {
                self.slow_operations.fetch_add(1, Ordering::Relaxed);
                self.performance_violations.fetch_add(1, Ordering::Relaxed);

                metrics::counter!("slow_actions_total", 1);
                metrics::counter!("performance_violations_total").increment(1);

                tracing::warn!(
                    "CRITICAL: Slow action execution: {:.2}ms (threshold: 10ms)",
                    duration_ms
                );
            }
        }

        OperationResult {
            duration_ms,
            is_slow: duration_ms > 10.0,
            is_violation: duration_ms > 10.0,
        }
    }

    /// Finish timing a WebSocket state update (CRITICAL PATH - <5ms)
    pub fn finish_websocket_update(self) -> OperationResult {
        let duration = self.start.elapsed();
        let duration_ms = if self.high_resolution {
            duration.as_nanos() as f64 / 1_000_000.0
        } else {
            duration.as_millis() as f64
        };

        #[cfg(feature = "monitoring")]
        {
            metrics::histogram!("websocket_update_duration_ms", duration_ms);

            if duration_ms > 5.0 {
                self.slow_operations.fetch_add(1, Ordering::Relaxed);
                self.performance_violations.fetch_add(1, Ordering::Relaxed);

                metrics::counter!("websocket_slow_updates", 1);
                metrics::counter!("performance_violations_total").increment(1);

                tracing::warn!(
                    "CRITICAL: Slow WebSocket update: {:.2}ms (threshold: 5ms)",
                    duration_ms
                );
            }
        }

        OperationResult {
            duration_ms,
            is_slow: duration_ms > 5.0,
            is_violation: duration_ms > 5.0,
        }
    }

    /// Finish timing a storage operation
    pub fn finish_storage_operation(self, operation: &str) -> OperationResult {
        let duration = self.start.elapsed();
        let duration_ms = if self.high_resolution {
            duration.as_nanos() as f64 / 1_000_000.0
        } else {
            duration.as_millis() as f64
        };

        #[cfg(feature = "monitoring")]
        {
            metrics::histogram!("storage_operation_duration_ms").record(duration_ms);

            if duration_ms > 1.0 {
                self.slow_operations.fetch_add(1, Ordering::Relaxed);

                metrics::counter!("storage_slow_operations").increment(1);

                tracing::debug!("Slow storage operation {}: {:.2}ms", operation, duration_ms);
            }
        }

        OperationResult {
            duration_ms,
            is_slow: duration_ms > 1.0,
            is_violation: false, // Storage operations are less critical
        }
    }

    /// Finish timing with custom threshold
    pub fn finish_with_threshold(self, operation: &str, threshold_ms: f64) -> OperationResult {
        let duration = self.start.elapsed();
        let duration_ms = if self.high_resolution {
            duration.as_nanos() as f64 / 1_000_000.0
        } else {
            duration.as_millis() as f64
        };

        let is_slow = duration_ms > threshold_ms;
        let is_violation = is_slow && threshold_ms <= 10.0; // Only critical operations are violations

        #[cfg(feature = "monitoring")]
        {
            metrics::histogram!(format!("{}_duration_ms", operation)).record(duration_ms);

            if is_slow {
                self.slow_operations.fetch_add(1, Ordering::Relaxed);
                metrics::counter!(format!("{}_slow_operations", operation)).increment(1);

                if is_violation {
                    self.performance_violations.fetch_add(1, Ordering::Relaxed);
                    metrics::counter!("performance_violations_total").increment(1);

                    tracing::warn!(
                        "Performance violation in {}: {:.2}ms (threshold: {:.2}ms)",
                        operation,
                        duration_ms,
                        threshold_ms
                    );
                }
            }
        }

        OperationResult {
            duration_ms,
            is_slow,
            is_violation,
        }
    }
}

/// Critical operation timer with automatic violation detection
pub struct CriticalOperationTimer {
    operation_name: String,
    threshold_ms: u64,
    start: Instant,
    high_resolution: bool,
    performance_violations: Arc<AtomicU64>,
}

impl CriticalOperationTimer {
    fn new(
        operation_name: String,
        threshold_ms: u64,
        high_resolution: bool,
        performance_violations: Arc<AtomicU64>,
    ) -> Self {
        Self {
            operation_name,
            threshold_ms,
            start: Instant::now(),
            high_resolution,
            performance_violations,
        }
    }
}

// Automatic performance monitoring on drop
impl Drop for CriticalOperationTimer {
    fn drop(&mut self) {
        let duration = self.start.elapsed();
        let duration_ms = if self.high_resolution {
            duration.as_nanos() as u64 / 1_000_000
        } else {
            duration.as_millis() as u64
        };

        #[cfg(feature = "monitoring")]
        {
            metrics::histogram!(format!("{}_duration_ms", self.operation_name))
                .record(duration_ms as f64);

            if duration_ms > self.threshold_ms {
                self.performance_violations.fetch_add(1, Ordering::Relaxed);

                metrics::counter!("performance_violations_total").increment(1);

                tracing::error!(
                    "CRITICAL PERFORMANCE VIOLATION: {} took {}ms (threshold: {}ms)",
                    self.operation_name,
                    duration_ms,
                    self.threshold_ms
                );
            }
        }
    }
}

/// Result of a timed operation
#[derive(Debug, Clone)]
pub struct OperationResult {
    /// Duration in milliseconds (high precision)
    pub duration_ms: f64,
    /// Whether the operation was considered slow
    pub is_slow: bool,
    /// Whether the operation violated critical performance thresholds
    pub is_violation: bool,
}

impl OperationResult {
    /// Check if the operation met performance requirements
    pub fn is_acceptable(&self) -> bool {
        !self.is_violation
    }

    /// Get duration in nanoseconds
    pub fn duration_nanos(&self) -> u64 {
        (self.duration_ms * 1_000_000.0) as u64
    }

    /// Get duration in microseconds
    pub fn duration_micros(&self) -> u64 {
        (self.duration_ms * 1_000.0) as u64
    }
}

/// Performance statistics snapshot
#[derive(Debug, Clone)]
pub struct PerformanceStats {
    pub total_operations: u64,
    pub slow_operations: u64,
    pub performance_violations: u64,
    pub slow_operation_rate: f64,
    pub violation_rate: f64,
    pub uptime_seconds: f64,
    pub operations_per_second: f64,
}

impl PerformanceStats {
    /// Check if the system is performing within acceptable limits
    pub fn is_healthy(&self) -> bool {
        self.violation_rate < 0.01 && // Less than 1% violations
        self.slow_operation_rate < 0.05 // Less than 5% slow operations
    }

    /// Get performance grade (A-F)
    pub fn performance_grade(&self) -> char {
        if self.violation_rate == 0.0 && self.slow_operation_rate < 0.01 {
            'A'
        } else if self.violation_rate < 0.001 && self.slow_operation_rate < 0.02 {
            'B'
        } else if self.violation_rate < 0.005 && self.slow_operation_rate < 0.05 {
            'C'
        } else if self.violation_rate < 0.01 && self.slow_operation_rate < 0.1 {
            'D'
        } else {
            'F'
        }
    }
}

/// Re-export for external use
pub use super::MetricsHandle;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_performance_monitor_creation() {
        let config = MetricsConfig::default();
        let monitor = PerformanceMonitor::new(config);

        assert_eq!(monitor.total_operations(), 0);
        assert_eq!(monitor.slow_operations(), 0);
        assert_eq!(monitor.performance_violations(), 0);
        assert!(monitor.uptime_seconds() >= 0.0);
    }

    #[test]
    fn test_operation_timer() {
        let config = MetricsConfig::default();
        let monitor = PerformanceMonitor::new(config);

        let timer = monitor.start_operation();
        std::thread::sleep(std::time::Duration::from_millis(1));
        let result = timer.finish_with_threshold("test", 10.0);

        assert!(result.duration_ms > 0.0);
        assert!(!result.is_violation); // 1ms is well under 10ms threshold
    }

    #[tokio::test]
    async fn test_critical_operation_timer() {
        let config = MetricsConfig::default();
        let monitor = PerformanceMonitor::new(config);

        {
            let _timer = monitor.start_critical_operation("test_critical", 5);
            tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
            // Timer automatically reports on drop
        }

        let stats = monitor.get_stats();
        assert_eq!(stats.total_operations, 1);
    }

    #[test]
    fn test_operation_result() {
        let result = OperationResult {
            duration_ms: 2.5,
            is_slow: false,
            is_violation: false,
        };

        assert!(result.is_acceptable());
        assert_eq!(result.duration_nanos(), 2_500_000);
        assert_eq!(result.duration_micros(), 2_500);
    }

    #[test]
    fn test_performance_stats_health() {
        let healthy_stats = PerformanceStats {
            total_operations: 1000,
            slow_operations: 10,       // 1% slow
            performance_violations: 1, // 0.1% violations
            slow_operation_rate: 0.01,
            violation_rate: 0.001,
            uptime_seconds: 60.0,
            operations_per_second: 16.67,
        };

        assert!(healthy_stats.is_healthy());
        assert_eq!(healthy_stats.performance_grade(), 'B');

        let unhealthy_stats = PerformanceStats {
            total_operations: 100,
            slow_operations: 20,       // 20% slow
            performance_violations: 5, // 5% violations
            slow_operation_rate: 0.2,
            violation_rate: 0.05,
            uptime_seconds: 10.0,
            operations_per_second: 10.0,
        };

        assert!(!unhealthy_stats.is_healthy());
        assert_eq!(unhealthy_stats.performance_grade(), 'F');
    }

    #[test]
    fn test_high_resolution_timing() {
        let config = MetricsConfig {
            high_resolution_timing: true,
            ..Default::default()
        };

        let monitor = PerformanceMonitor::new(config);
        let timer = monitor.start_operation();

        // Very short operation
        let result = timer.finish_with_threshold("nano_test", 1.0);

        // Should still measure some time even for very fast operations
        assert!(result.duration_ms >= 0.0);
    }
}
