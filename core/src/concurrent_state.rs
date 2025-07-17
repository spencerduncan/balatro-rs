//! Concurrent state access patterns for efficient game state operations
//!
//! This module implements efficient state access patterns to improve game engine performance
//! for AI/RL applications. It provides:
//! - Concurrent-safe data structures for game state access
//! - Batch update mechanisms for state modifications
//! - Optimized read-heavy access patterns
//! - Minimal lock contention implementations
//! - Data consistency across concurrent operations

use crate::action::Action;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::RwLock;
use std::time::{Duration, Instant};

/// State update operations for batch processing
#[derive(Debug, Clone)]
pub enum StateUpdate {
    Money(usize),
    Chips(usize),
    Mult(usize),
    Plays(usize),
    Discards(usize),
    Score(usize),
}

/// Lock-free snapshot of frequently accessed game state
#[derive(Debug, Clone)]
pub struct LockFreeStateSnapshot {
    pub money: usize,
    pub chips: usize,
    pub mult: usize,
    pub score: usize,
    pub stage: String,
    pub round: usize,
    pub plays_remaining: usize,
    pub discards_remaining: usize,
}

/// Performance metrics for state operations
#[derive(Debug)]
pub struct PerformanceMetrics {
    pub average_action_generation_time: Duration,
    pub average_state_read_time: Duration,
    pub average_batch_update_time: Duration,
    pub memory_usage_mb: f64,
    pub cache_hit_rate: f64,
}

/// Efficient cache key to avoid string allocations
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CacheKey {
    pub stage_hash: u64, // Hash of stage enum to avoid string allocations
    pub money: usize,
    pub chips: usize,
    pub mult: usize,
}

/// Cache for action generation to optimize repeated operations
#[derive(Debug, Clone)]
pub struct ActionCache {
    cached_actions: Vec<Action>,
    cache_key: CacheKey,
    cache_time: Instant,
    cache_ttl: Duration,
}

impl Default for ActionCache {
    fn default() -> Self {
        Self::new()
    }
}

impl ActionCache {
    pub fn new() -> Self {
        Self {
            cached_actions: Vec::new(),
            cache_key: CacheKey {
                stage_hash: 0,
                money: 0,
                chips: 0,
                mult: 0,
            },
            cache_time: Instant::now(),
            cache_ttl: Duration::from_millis(100), // 100ms TTL
        }
    }

    /// Get cached actions if they're still valid, None otherwise
    /// This combines validation and retrieval to avoid race conditions
    pub fn get_if_valid(
        &self,
        stage_hash: u64,
        money: usize,
        chips: usize,
        mult: usize,
    ) -> Option<&[Action]> {
        let current_key = CacheKey {
            stage_hash,
            money,
            chips,
            mult,
        };

        if self.cache_key == current_key && self.cache_time.elapsed() < self.cache_ttl {
            Some(&self.cached_actions)
        } else {
            None
        }
    }

    pub fn update(
        &mut self,
        stage_hash: u64,
        money: usize,
        chips: usize,
        mult: usize,
        actions: Vec<Action>,
    ) {
        self.cache_key = CacheKey {
            stage_hash,
            money,
            chips,
            mult,
        };
        self.cached_actions = actions;
        self.cache_time = Instant::now();
    }

    pub fn get(&self) -> &[Action] {
        &self.cached_actions
    }
}

/// Concurrent state manager for thread-safe operations
#[derive(Debug)]
pub struct ConcurrentStateManager {
    state_cache: RwLock<HashMap<String, LockFreeStateSnapshot>>,
    action_cache: RwLock<ActionCache>,
    metrics: RwLock<PerformanceMetrics>,
}

impl ConcurrentStateManager {
    pub fn new() -> Self {
        Self {
            state_cache: RwLock::new(HashMap::new()),
            action_cache: RwLock::new(ActionCache::new()),
            metrics: RwLock::new(PerformanceMetrics {
                average_action_generation_time: Duration::from_micros(0),
                average_state_read_time: Duration::from_micros(0),
                average_batch_update_time: Duration::from_micros(0),
                memory_usage_mb: 0.0,
                cache_hit_rate: 0.0,
            }),
        }
    }

    pub fn get_state_snapshot(&self, key: &str) -> Option<LockFreeStateSnapshot> {
        self.state_cache.read().ok()?.get(key).cloned()
    }

    pub fn update_state_snapshot(&self, key: String, snapshot: LockFreeStateSnapshot) {
        if let Ok(mut cache) = self.state_cache.write() {
            cache.insert(key, snapshot);
        }
    }

    pub fn get_cached_actions(
        &self,
        stage_hash: u64,
        money: usize,
        chips: usize,
        mult: usize,
    ) -> Option<Vec<Action>> {
        let cache = self.action_cache.read().ok()?;
        cache
            .get_if_valid(stage_hash, money, chips, mult)
            .map(|actions| actions.to_vec())
    }

    pub fn cache_actions(
        &self,
        stage_hash: u64,
        money: usize,
        chips: usize,
        mult: usize,
        actions: Vec<Action>,
    ) {
        if let Ok(mut cache) = self.action_cache.write() {
            cache.update(stage_hash, money, chips, mult, actions);
        }
    }

    pub fn update_metrics(&self, metrics: PerformanceMetrics) {
        if let Ok(mut m) = self.metrics.write() {
            *m = metrics;
        }
    }

    pub fn get_metrics(&self) -> Option<PerformanceMetrics> {
        self.metrics.read().ok().map(|m| PerformanceMetrics {
            average_action_generation_time: m.average_action_generation_time,
            average_state_read_time: m.average_state_read_time,
            average_batch_update_time: m.average_batch_update_time,
            memory_usage_mb: m.memory_usage_mb,
            cache_hit_rate: m.cache_hit_rate,
        })
    }
}

impl Default for ConcurrentStateManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Error types for concurrent state operations
#[derive(Debug, thiserror::Error)]
pub enum ConcurrentStateError {
    #[error("Batch update failed: {message}")]
    BatchUpdateFailed { message: String },

    #[error("Lock contention detected")]
    LockContention,

    #[error("Cache invalidation error: {reason}")]
    CacheError { reason: String },
}

pub type Result<T> = std::result::Result<T, ConcurrentStateError>;

/// Utility function to compute stage hash avoiding string allocations
/// This properly hashes all enum variant data to prevent cache key collisions
pub fn compute_stage_hash(stage: &crate::stage::Stage) -> u64 {
    let mut hasher = DefaultHasher::new();
    
    // Hash the full stage data, not just the discriminant
    // This prevents different Stage::Blind(X) variants from having the same hash
    match stage {
        crate::stage::Stage::PreBlind() => {
            0u8.hash(&mut hasher); // Unique ID for PreBlind
        }
        crate::stage::Stage::Blind(blind) => {
            1u8.hash(&mut hasher); // Unique ID for Blind variant
            blind.hash(&mut hasher); // Hash the actual blind value
        }
        crate::stage::Stage::PostBlind() => {
            2u8.hash(&mut hasher); // Unique ID for PostBlind
        }
        crate::stage::Stage::Shop() => {
            3u8.hash(&mut hasher); // Unique ID for Shop
        }
        crate::stage::Stage::End(end) => {
            4u8.hash(&mut hasher); // Unique ID for End variant
            end.hash(&mut hasher); // Hash the actual end value
        }
    }
    
    hasher.finish()
}
