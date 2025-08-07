/// Memory monitoring and management for training scenarios
///
/// This module provides tools to monitor and control memory usage during
/// long-running RL training sessions to prevent memory exhaustion.
use serde::{Deserialize, Serialize};
use std::time::Instant;

/// Configuration for memory monitoring and limits
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryConfig {
    /// Maximum number of actions to keep in history (default: 10,000)
    pub max_action_history: usize,

    /// Enable memory usage monitoring (default: false)
    pub enable_monitoring: bool,

    /// Interval for memory checks in milliseconds (default: 5000ms)
    pub monitoring_interval_ms: u64,

    /// Memory usage threshold for warnings (in MB, default: 1024MB)
    pub warning_threshold_mb: usize,

    /// Memory usage threshold for critical alerts (in MB, default: 2048MB)
    pub critical_threshold_mb: usize,

    /// Enable automatic cleanup when memory thresholds are exceeded
    pub auto_cleanup: bool,

    /// Maximum memory usage before forcing cleanup (in MB, default: 4096MB)
    pub max_memory_mb: usize,
}

impl Default for MemoryConfig {
    fn default() -> Self {
        Self {
            max_action_history: crate::bounded_action_history::DEFAULT_MAX_ACTIONS,
            enable_monitoring: false,
            monitoring_interval_ms: 5000,
            warning_threshold_mb: 1024,
            critical_threshold_mb: 2048,
            auto_cleanup: true,
            max_memory_mb: 4096,
        }
    }
}

impl MemoryConfig {
    /// Create a configuration optimized for RL training
    pub fn for_rl_training() -> Self {
        Self {
            max_action_history: 5000, // Smaller history for training
            enable_monitoring: true,
            monitoring_interval_ms: 30000, // Check every 30 seconds (reduced frequency for performance)
            warning_threshold_mb: 512,
            critical_threshold_mb: 1024,
            auto_cleanup: true,
            max_memory_mb: 2048,
        }
    }

    /// Create a configuration for long-running simulations
    pub fn for_simulation() -> Self {
        Self {
            max_action_history: 1000, // Very small history
            enable_monitoring: true,
            monitoring_interval_ms: 30000, // Check every 30 seconds
            warning_threshold_mb: 256,
            critical_threshold_mb: 512,
            auto_cleanup: true,
            max_memory_mb: 1024,
        }
    }
}

/// Memory usage statistics
#[derive(Debug, Clone)]
pub struct MemoryStats {
    /// Timestamp when stats were collected
    pub timestamp: Instant,

    /// Estimated memory usage in bytes
    pub estimated_usage_bytes: usize,

    /// Estimated memory usage in MB
    pub estimated_usage_mb: usize,

    /// Number of active snapshots
    pub active_snapshots: usize,

    /// Total actions in history
    pub total_actions: usize,

    /// Action history memory usage in bytes
    pub action_history_bytes: usize,

    /// Game state memory usage in bytes
    pub game_state_bytes: usize,
}

impl MemoryStats {
    /// Check if memory usage exceeds warning threshold
    pub fn exceeds_warning(&self, config: &MemoryConfig) -> bool {
        self.estimated_usage_mb > config.warning_threshold_mb
    }

    /// Check if memory usage exceeds critical threshold
    pub fn exceeds_critical(&self, config: &MemoryConfig) -> bool {
        self.estimated_usage_mb > config.critical_threshold_mb
    }

    /// Check if memory usage exceeds maximum allowed
    pub fn exceeds_maximum(&self, config: &MemoryConfig) -> bool {
        self.estimated_usage_mb > config.max_memory_mb
    }
}

/// Memory monitoring and alerting system
#[derive(Debug)]
pub struct MemoryMonitor {
    config: MemoryConfig,
    last_check: Option<Instant>,
    warning_count: usize,
    critical_count: usize,
    last_stats: Option<MemoryStats>,
    /// Cached memory estimation to avoid expensive recalculation
    cached_base_memory: Option<(Instant, usize)>,
    /// Cache invalidation threshold - recalculate every N checks
    cache_invalidation_counter: u32,
}

impl Default for MemoryMonitor {
    fn default() -> Self {
        Self::new(MemoryConfig::default())
    }
}

impl MemoryMonitor {
    /// Create a new memory monitor with the given configuration
    pub fn new(config: MemoryConfig) -> Self {
        Self {
            config,
            last_check: None,
            warning_count: 0,
            critical_count: 0,
            last_stats: None,
            cached_base_memory: None,
            cache_invalidation_counter: 0,
        }
    }

    /// Check if it's time to perform a memory check (optimized for performance)
    pub fn should_check(&self) -> bool {
        if !self.config.enable_monitoring {
            return false;
        }

        match self.last_check {
            None => true,
            Some(last) => {
                // Use faster elapsed check - respect configured interval for tests
                let elapsed_ms = last.elapsed().as_millis() as u64;
                elapsed_ms >= self.config.monitoring_interval_ms
            }
        }
    }

    /// Perform a memory check and return statistics (simplified for minimal overhead)
    pub fn check_memory(
        &mut self,
        estimated_bytes: usize,
        snapshots: usize,
        bounded_actions: usize,
    ) -> MemoryStats {
        let now = Instant::now();

        // Simple calculation - no complex caching or optimizations that add overhead
        let action_history_bytes = bounded_actions * std::mem::size_of::<crate::action::Action>();

        let stats = MemoryStats {
            timestamp: now,
            estimated_usage_bytes: estimated_bytes,
            estimated_usage_mb: estimated_bytes / (1024 * 1024),
            active_snapshots: snapshots,
            total_actions: bounded_actions,
            action_history_bytes,
            game_state_bytes: estimated_bytes.saturating_sub(action_history_bytes),
        };

        // Update counts based on thresholds
        if stats.exceeds_warning(&self.config) {
            self.warning_count += 1;
        }
        if stats.exceeds_critical(&self.config) {
            self.critical_count += 1;
        }

        self.last_check = Some(now);
        self.last_stats = Some(stats.clone());

        stats
    }

    /// Get the current configuration
    pub fn config(&self) -> &MemoryConfig {
        &self.config
    }

    /// Update the configuration
    pub fn update_config(&mut self, config: MemoryConfig) {
        self.config = config;
    }

    /// Get warning count since monitor creation
    pub fn warning_count(&self) -> usize {
        self.warning_count
    }

    /// Get critical alert count since monitor creation
    pub fn critical_count(&self) -> usize {
        self.critical_count
    }

    /// Get the last collected statistics
    pub fn last_stats(&self) -> Option<&MemoryStats> {
        self.last_stats.as_ref()
    }

    /// Reset warning and critical counts
    pub fn reset_counts(&mut self) {
        self.warning_count = 0;
        self.critical_count = 0;
    }

    /// Invalidate the memory cache to force recalculation
    pub fn invalidate_cache(&mut self) {
        self.cached_base_memory = None;
        self.cache_invalidation_counter = 0;
    }

    /// Generate a memory usage report
    pub fn generate_report(&self) -> String {
        match &self.last_stats {
            Some(stats) => {
                format!(
                    "Memory Usage Report:\n\
                     - Estimated Usage: {:.2} MB ({} bytes)\n\
                     - Active Snapshots: {}\n\
                     - Total Actions: {}\n\
                     - Action History: {:.2} MB\n\
                     - Game State: {:.2} MB\n\
                     - Warnings: {}\n\
                     - Critical Alerts: {}\n\
                     - Status: {}",
                    stats.estimated_usage_mb,
                    stats.estimated_usage_bytes,
                    stats.active_snapshots,
                    stats.total_actions,
                    stats.action_history_bytes as f64 / (1024.0 * 1024.0),
                    stats.game_state_bytes as f64 / (1024.0 * 1024.0),
                    self.warning_count,
                    self.critical_count,
                    if stats.exceeds_maximum(&self.config) {
                        "CRITICAL - Maximum exceeded"
                    } else if stats.exceeds_critical(&self.config) {
                        "CRITICAL"
                    } else if stats.exceeds_warning(&self.config) {
                        "WARNING"
                    } else {
                        "OK"
                    }
                )
            }
            None => "No memory statistics available".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_memory_config_default() {
        let config = MemoryConfig::default();
        assert_eq!(
            config.max_action_history,
            crate::bounded_action_history::DEFAULT_MAX_ACTIONS
        );
        assert!(!config.enable_monitoring);
        assert!(config.auto_cleanup);
    }

    #[test]
    fn test_memory_config_rl_training() {
        let config = MemoryConfig::for_rl_training();
        assert!(config.enable_monitoring);
        assert!(config.max_action_history < MemoryConfig::default().max_action_history);
        assert!(config.warning_threshold_mb < MemoryConfig::default().warning_threshold_mb);
    }

    #[test]
    fn test_memory_monitor_thresholds() {
        let config = MemoryConfig {
            warning_threshold_mb: 100,
            critical_threshold_mb: 200,
            max_memory_mb: 300,
            ..Default::default()
        };

        let stats = MemoryStats {
            timestamp: Instant::now(),
            estimated_usage_bytes: 150 * 1024 * 1024, // 150 MB
            estimated_usage_mb: 150,
            active_snapshots: 5,
            total_actions: 1000,
            action_history_bytes: 1000 * 32,
            game_state_bytes: 150 * 1024 * 1024 - 1000 * 32,
        };

        assert!(stats.exceeds_warning(&config));
        assert!(!stats.exceeds_critical(&config));
        assert!(!stats.exceeds_maximum(&config));
    }

    #[test]
    fn test_memory_monitor_check_timing() {
        let config = MemoryConfig {
            enable_monitoring: true,
            monitoring_interval_ms: 100,
            ..Default::default()
        };

        let mut monitor = MemoryMonitor::new(config);

        // Should check initially
        assert!(monitor.should_check());

        // After checking, should not check immediately
        monitor.check_memory(1024, 1, 100);
        assert!(!monitor.should_check());

        // After interval, should check again
        std::thread::sleep(Duration::from_millis(150));
        assert!(monitor.should_check());
    }
}
