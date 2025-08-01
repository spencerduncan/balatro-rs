#![allow(missing_docs)]
//! Application Configuration and Session Types
//!
//! This module defines all configuration types, session management structures,
//! and runtime parameters for the application layer. Designed for production
//! scalability with proper resource limits and monitoring.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, SystemTime};
use uuid::Uuid;

/// Unique identifier for game sessions
///
/// Uses UUID v4 for global uniqueness and collision resistance
/// in distributed environments. Serializable for persistence
/// and network transmission.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SessionId(Uuid);

impl SessionId {
    /// Generate a new unique session ID
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// Create a session ID from a string (for testing/deserialization)
    pub fn from_string(s: &str) -> Result<Self, uuid::Error> {
        Ok(Self(Uuid::parse_str(s)?))
    }

    /// Get the underlying UUID
    pub fn as_uuid(&self) -> Uuid {
        self.0
    }

    /// Get string representation for logging/debugging
    pub fn as_str(&self) -> String {
        self.0.to_string()
    }
}

impl std::fmt::Display for SessionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Default for SessionId {
    fn default() -> Self {
        Self::new()
    }
}

/// Game configuration parameters for session creation
///
/// Encapsulates all configurable aspects of game creation,
/// allowing for different game modes, difficulty levels,
/// and experimental features.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameConfig {
    /// Random number generator seed for reproducible games
    pub seed: Option<u64>,

    /// Starting ante level (difficulty)
    pub starting_ante: u32,

    /// Maximum ante level before game ends
    pub max_ante: u32,

    /// Starting money amount
    pub starting_money: i32,

    /// Maximum hand size
    pub hand_size: usize,

    /// Number of discards per round
    pub discards: u32,

    /// Number of hands per round
    pub hands: u32,

    /// Enable/disable specific game features
    pub features: GameFeatures,

    /// Performance and resource limits
    pub limits: GameLimits,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            seed: None,
            starting_ante: 1,
            max_ante: 8,
            starting_money: 4,
            hand_size: 8,
            discards: 3,
            hands: 4,
            features: GameFeatures::default(),
            limits: GameLimits::default(),
        }
    }
}

/// Feature flags for game functionality
///
/// Allows enabling/disabling specific game features for
/// A/B testing, gradual rollouts, and experimental features.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameFeatures {
    /// Enable joker cards
    pub jokers_enabled: bool,

    /// Enable consumable cards (tarot, planet, spectral)
    pub consumables_enabled: bool,

    /// Enable voucher system
    pub vouchers_enabled: bool,

    /// Enable boss blinds
    pub boss_blinds_enabled: bool,

    /// Enable card packs in shop
    pub packs_enabled: bool,

    /// Enable advanced scoring mechanics
    pub advanced_scoring: bool,
}

impl Default for GameFeatures {
    fn default() -> Self {
        Self {
            jokers_enabled: true,
            consumables_enabled: true,
            vouchers_enabled: true,
            boss_blinds_enabled: true,
            packs_enabled: true,
            advanced_scoring: true,
        }
    }
}

/// Resource and performance limits for games
///
/// Prevents resource exhaustion and ensures fair resource
/// usage in multi-tenant environments.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameLimits {
    /// Maximum number of actions per session
    pub max_actions: usize,

    /// Maximum session duration
    pub max_duration: Duration,

    /// Maximum memory usage estimate (MB)
    pub max_memory_mb: usize,

    /// Maximum number of jokers in play
    pub max_jokers: usize,

    /// Maximum deck size
    pub max_deck_size: usize,
}

impl Default for GameLimits {
    fn default() -> Self {
        Self {
            max_actions: 10_000,                   // ~1000 rounds max
            max_duration: <std::time::Duration as crate::application::config::DurationExt>::from_hours(2), // 2 hour max session
            max_memory_mb: 50,                     // 50MB memory limit
            max_jokers: 25,                        // Balatro's natural limit
            max_deck_size: 200,                    // Reasonable deck limit
        }
    }
}

/// Session information and metadata
///
/// Tracks session lifecycle, performance metrics, and
/// operational data for monitoring and debugging.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionInfo {
    /// Unique session identifier
    pub id: SessionId,

    /// Session creation timestamp
    pub created_at: SystemTime,

    /// Last activity timestamp
    pub last_activity: SystemTime,

    /// Session configuration
    pub config: GameConfig,

    /// Current session status
    pub status: SessionStatus,

    /// Performance metrics
    pub metrics: SessionMetrics,

    /// Optional session metadata for debugging
    pub metadata: HashMap<String, String>,
}

impl SessionInfo {
    /// Create new session info
    pub fn new(id: SessionId, config: GameConfig) -> Self {
        let now = SystemTime::now();
        Self {
            id,
            created_at: now,
            last_activity: now,
            config,
            status: SessionStatus::Active,
            metrics: SessionMetrics::default(),
            metadata: HashMap::new(),
        }
    }

    /// Check if session has expired based on TTL
    pub fn is_expired(&self, ttl: Duration) -> bool {
        if let Ok(elapsed) = self.last_activity.elapsed() {
            elapsed > ttl
        } else {
            true // If we can't determine elapsed time, assume expired
        }
    }

    /// Update last activity timestamp
    pub fn touch(&mut self) {
        self.last_activity = SystemTime::now();
    }

    /// Add metadata entry
    pub fn add_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }
}

/// Session lifecycle status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SessionStatus {
    /// Session is active and accepting requests
    Active,

    /// Session is paused (no new requests accepted)
    Paused,

    /// Session is marked for cleanup
    Terminating,

    /// Session has completed successfully
    Completed,

    /// Session ended due to error
    Failed,
}

/// Session performance and usage metrics
///
/// Tracks operational metrics for monitoring, alerting,
/// and capacity planning.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionMetrics {
    /// Total number of actions executed
    pub total_actions: usize,

    /// Average action execution time (microseconds)
    pub avg_action_time_us: u64,

    /// Peak memory usage (MB)
    pub peak_memory_mb: usize,

    /// Current game score
    pub current_score: i64,

    /// Current ante level
    pub current_ante: u32,

    /// Number of errors encountered
    pub error_count: usize,
}

impl Default for SessionMetrics {
    fn default() -> Self {
        Self {
            total_actions: 0,
            avg_action_time_us: 0,
            peak_memory_mb: 0,
            current_score: 0,
            current_ante: 1,
            error_count: 0,
        }
    }
}

/// Application-wide configuration
///
/// Controls application layer behavior, resource limits,
/// and operational parameters. Designed for production
/// deployment with proper scaling parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct ApplicationConfig {
    /// Session management configuration
    pub session: SessionConfig,

    /// Performance and resource limits
    pub limits: ApplicationLimits,

    /// Monitoring and observability settings
    pub monitoring: MonitoringConfig,

    /// Feature flags and experiments
    pub features: ApplicationFeatures,
}

/// Session management configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionConfig {
    /// Session time-to-live before cleanup
    pub ttl: Duration,

    /// Cleanup interval for expired sessions
    pub cleanup_interval: Duration,

    /// Maximum number of concurrent sessions
    pub max_concurrent_sessions: usize,

    /// Session cleanup strategy
    pub cleanup_strategy: CleanupStrategy,
}

impl Default for SessionConfig {
    fn default() -> Self {
        Self {
            ttl: <std::time::Duration as crate::application::config::DurationExt>::from_hours(1),             // 1 hour TTL
            cleanup_interval: <std::time::Duration as crate::application::config::DurationExt>::from_mins(5), // 5 minute cleanup
            max_concurrent_sessions: 1000,            // 1000 concurrent sessions
            cleanup_strategy: CleanupStrategy::LeastRecentlyUsed,
        }
    }
}

/// Session cleanup strategies for resource management
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum CleanupStrategy {
    /// Remove least recently used sessions first
    LeastRecentlyUsed,

    /// Remove oldest sessions first
    FirstInFirstOut,

    /// Remove sessions with lowest activity
    LowestActivity,
}

/// Application-wide resource limits
///
/// Prevents resource exhaustion and ensures system stability
/// under high load conditions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationLimits {
    /// Maximum memory usage for entire application (MB)
    pub max_total_memory_mb: usize,

    /// Maximum CPU usage threshold (percentage)
    pub max_cpu_usage_percent: u8,

    /// Request rate limiting (requests per second)
    pub max_requests_per_second: usize,

    /// Maximum concurrent operations
    pub max_concurrent_operations: usize,
}

impl Default for ApplicationLimits {
    fn default() -> Self {
        Self {
            max_total_memory_mb: 2048,      // 2GB total memory limit
            max_cpu_usage_percent: 80,      // 80% CPU threshold
            max_requests_per_second: 1000,  // 1000 RPS limit
            max_concurrent_operations: 500, // 500 concurrent ops
        }
    }
}

/// Monitoring and observability configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    /// Enable metrics collection
    pub metrics_enabled: bool,

    /// Enable distributed tracing
    pub tracing_enabled: bool,

    /// Metrics export interval
    pub metrics_interval: Duration,

    /// Log level for application layer
    pub log_level: LogLevel,

    /// Enable performance profiling
    pub profiling_enabled: bool,
}

impl Default for MonitoringConfig {
    fn default() -> Self {
        Self {
            metrics_enabled: true,
            tracing_enabled: true,
            metrics_interval: Duration::from_secs(30),
            log_level: LogLevel::Info,
            profiling_enabled: false,
        }
    }
}

/// Application logging levels
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

/// Application-wide feature flags
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationFeatures {
    /// Enable experimental features
    pub experimental_features: bool,

    /// Enable A/B testing framework
    pub ab_testing: bool,

    /// Enable advanced monitoring
    pub advanced_monitoring: bool,

    /// Enable automatic recovery
    pub auto_recovery: bool,
}

impl Default for ApplicationFeatures {
    fn default() -> Self {
        Self {
            experimental_features: false,
            ab_testing: false,
            advanced_monitoring: true,
            auto_recovery: true,
        }
    }
}

// Add Duration helper trait for easier configuration
trait DurationExt {
    fn from_mins(mins: u64) -> Duration;
    fn from_hours(hours: u64) -> Duration;
}

impl DurationExt for Duration {
    fn from_mins(mins: u64) -> Duration {
        Duration::from_secs(mins * 60)
    }

    fn from_hours(hours: u64) -> Duration {
        Duration::from_secs(hours * 3600)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_id_creation() {
        let id1 = SessionId::new();
        let id2 = SessionId::new();
        assert_ne!(id1, id2);

        let uuid_str = id1.as_str();
        let id3 = SessionId::from_string(&uuid_str).unwrap();
        assert_eq!(id1, id3);
    }

    #[test]
    fn test_session_info_expiration() {
        let config = GameConfig::default();
        let id = SessionId::new();
        let mut session = SessionInfo::new(id, config);

        // Should not be expired immediately
        assert!(!session.is_expired(Duration::from_secs(10)));

        // Simulate old last_activity
        session.last_activity = SystemTime::now() - Duration::from_secs(20);
        assert!(session.is_expired(Duration::from_secs(10)));
    }

    #[test]
    fn test_session_touch() {
        let config = GameConfig::default();
        let id = SessionId::new();
        let mut session = SessionInfo::new(id, config);

        let original_time = session.last_activity;
        std::thread::sleep(Duration::from_millis(1));
        session.touch();

        assert!(session.last_activity > original_time);
    }

    #[test]
    fn test_game_config_defaults() {
        let config = GameConfig::default();
        assert_eq!(config.starting_ante, 1);
        assert_eq!(config.max_ante, 8);
        assert_eq!(config.hand_size, 8);
        assert!(config.features.jokers_enabled);
    }

    #[test]
    fn test_application_config_serialization() {
        let config = ApplicationConfig::default();
        let serialized = serde_json::to_string(&config).unwrap();
        let deserialized: ApplicationConfig = serde_json::from_str(&serialized).unwrap();

        // Compare some key fields
        assert_eq!(
            config.session.max_concurrent_sessions,
            deserialized.session.max_concurrent_sessions
        );
        assert_eq!(
            config.limits.max_requests_per_second,
            deserialized.limits.max_requests_per_second
        );
    }
}
