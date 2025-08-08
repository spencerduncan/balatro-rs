use serde::{Deserialize, Serialize};
use std::fmt;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

// Performance-optimized SessionId using atomic operations for thread-safe ID generation
// Uses 64-bit IDs for efficient comparison and storage (8 bytes vs UUID's 16 bytes)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SessionId {
    value: u64,
}

// Global atomic counter for session IDs - ensures uniqueness across threads
static SESSION_COUNTER: AtomicU64 = AtomicU64::new(0);

impl SessionId {
    /// Create a new unique SessionId
    /// Uses timestamp + atomic counter for guaranteed uniqueness even in high-throughput scenarios
    pub fn new() -> Self {
        // Get current timestamp in microseconds for time-based uniqueness
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_micros() as u64;

        // Atomic increment for thread-safe counter
        let counter = SESSION_COUNTER.fetch_add(1, Ordering::SeqCst);

        // Combine timestamp (high 44 bits) with counter (low 20 bits)
        // This allows ~1M unique IDs per microsecond, sufficient for any game server
        let value = (timestamp << 20) | (counter & 0xFFFFF);

        Self { value }
    }

    /// Create a SessionId from a raw value (for deserialization/testing)
    pub fn from_value(value: u64) -> Self {
        Self { value }
    }

    /// Get the raw value of the SessionId
    pub fn value(&self) -> u64 {
        self.value
    }

    /// Extract timestamp component (for TTL calculations)
    pub fn timestamp_micros(&self) -> u64 {
        self.value >> 20
    }
}

impl Default for SessionId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for SessionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "session_{:016x}", self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use std::thread;

    #[test]
    fn test_session_id_uniqueness() {
        let mut ids = HashSet::new();
        for _ in 0..10000 {
            let id = SessionId::new();
            assert!(ids.insert(id), "Duplicate SessionId generated");
        }
    }

    #[test]
    fn test_session_id_thread_safety() {
        let handles: Vec<_> = (0..10)
            .map(|_| {
                thread::spawn(|| {
                    let mut local_ids = Vec::new();
                    for _ in 0..1000 {
                        local_ids.push(SessionId::new());
                    }
                    local_ids
                })
            })
            .collect();

        let mut all_ids = HashSet::new();
        for handle in handles {
            let ids = handle.join().unwrap();
            for id in ids {
                assert!(all_ids.insert(id), "Thread-unsafe duplicate detected");
            }
        }
    }

    #[test]
    fn test_session_id_serialization() {
        let id = SessionId::from_value(0x123456789ABCDEF0);
        let serialized = serde_json::to_string(&id).unwrap();
        let deserialized: SessionId = serde_json::from_str(&serialized).unwrap();
        assert_eq!(id, deserialized);
    }
}
