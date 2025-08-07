// Debug functionality for game state inspection and testing

use crate::memory_monitor::MemoryMonitor;

/// Maximum debug messages to keep in memory (for practical memory management)
#[cfg(any(debug_assertions, test))]
const MAX_DEBUG_MESSAGES: usize = 10000;

/// Debug manager for handling debug logging and memory monitoring functionality
#[derive(Debug)]
pub struct DebugManager {
    /// Debug logging enabled flag
    pub debug_logging_enabled: bool,

    /// Debug messages buffer
    pub debug_messages: Vec<String>,

    /// Memory monitor for tracking and controlling memory usage
    pub memory_monitor: MemoryMonitor,
}

impl Default for DebugManager {
    fn default() -> Self {
        Self::new()
    }
}

impl DebugManager {
    /// Create a new DebugManager with default settings
    pub fn new() -> Self {
        Self {
            debug_logging_enabled: false,
            debug_messages: Vec::new(),
            memory_monitor: MemoryMonitor::default(),
        }
    }

    /// Enable debug logging for joker scoring
    pub fn enable_debug_logging(&mut self) {
        self.debug_logging_enabled = true;
        self.debug_messages.clear();
    }

    /// Get current debug messages
    pub fn get_debug_messages(&self) -> &[String] {
        &self.debug_messages
    }

    /// Add a debug message with automatic memory management
    /// Only compiles in debug builds and tests to eliminate overhead in release
    #[cfg(any(debug_assertions, test))]
    pub fn add_debug_message(&mut self, message: String) {
        if self.debug_logging_enabled {
            self.debug_messages.push(message);

            // Keep memory usage reasonable - remove oldest messages if we exceed limit
            if self.debug_messages.len() > MAX_DEBUG_MESSAGES {
                self.debug_messages
                    .drain(0..self.debug_messages.len() - MAX_DEBUG_MESSAGES);
            }
        }
    }

    /// No-op version for release builds (but not tests)
    #[cfg(not(any(debug_assertions, test)))]
    #[inline]
    pub fn add_debug_message(&mut self, _message: String) {
        // No-op in release builds
    }

    /// Configure memory monitoring for RL training scenarios
    pub fn enable_rl_memory_monitoring(&mut self, action_history_limit: &mut usize) {
        let config = crate::memory_monitor::MemoryConfig::for_rl_training();
        self.memory_monitor.update_config(config.clone());

        // Update action history limit to match memory config
        *action_history_limit = config.max_action_history;
    }

    /// Configure memory monitoring for simulation scenarios
    pub fn enable_simulation_memory_monitoring(&mut self, action_history_limit: &mut usize) {
        let config = crate::memory_monitor::MemoryConfig::for_simulation();
        self.memory_monitor.update_config(config.clone());

        // Update action history limit to match memory config
        *action_history_limit = config.max_action_history;
    }

    /// Get current memory usage statistics (optimized to avoid double-checking)
    pub fn get_memory_stats(
        &mut self,
        estimated_bytes: usize,
        total_actions: usize,
    ) -> Option<crate::memory_monitor::MemoryStats> {
        // should_check() was already called by caller, so proceed with check_memory()
        let stats = self.memory_monitor.check_memory(
            estimated_bytes,
            1, // Number of active snapshots (hard to track, estimate as 1)
            total_actions,
        );
        Some(stats)
    }

    /// Generate a memory usage report
    pub fn generate_memory_report(&self) -> String {
        self.memory_monitor.generate_report()
    }

    /// Check if memory usage exceeds safe limits
    pub fn check_memory_safety(&mut self, estimated_bytes: usize, total_actions: usize) -> bool {
        if let Some(stats) = self.get_memory_stats(estimated_bytes, total_actions) {
            !stats.exceeds_critical(self.memory_monitor.config())
        } else {
            true // Assume safe if no stats available
        }
    }

    /// Estimate memory usage of debug messages
    pub fn estimate_debug_memory_usage(&self) -> usize {
        self.debug_messages
            .iter()
            .map(|msg| msg.len())
            .sum::<usize>()
    }
}
