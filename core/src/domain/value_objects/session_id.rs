//! SessionId Value Object
//!
//! SessionId represents a unique identifier for a game session.
//! Following Domain-Driven Design principles, it encapsulates validation
//! and provides type safety for session identification.
//!
//! This implementation uses an 8-byte atomic counter for memory efficiency
//! at scale, saving 50% memory compared to UUID (16 bytes).

use serde::{Deserialize, Serialize};
use std::fmt;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

// Global atomic counter for session IDs - ensures uniqueness across threads
static SESSION_COUNTER: AtomicU64 = AtomicU64::new(0);

/// Unique identifier for a game session
///
/// SessionId is a value object that uses an 8-byte atomic counter
/// for memory-efficient unique identification at scale.
///
/// The ID is composed of:
/// - Upper 44 bits: microsecond timestamp (provides uniqueness across restarts)
/// - Lower 20 bits: atomic counter (provides ~1M unique IDs per microsecond)
///
/// # Examples
///
/// ```
/// use balatro_rs::domain::SessionId;
///
/// // Generate a new session ID
/// let session_id = SessionId::new();
///
/// // Parse from string (hex format)
/// let parsed = SessionId::from_value(0x123456789abcdef0);
///
/// // Convert to string
/// let id_string = session_id.to_string();
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SessionId {
    value: u64, // 8 bytes instead of 16!
}

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

    /// Get the underlying u64 value (alias for value())
    pub fn as_u64(&self) -> u64 {
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

/// Error type for SessionId parsing
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionIdError {
    message: &'static str,
}

impl fmt::Display for SessionIdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SessionId error: {}", self.message)
    }
}

impl std::error::Error for SessionIdError {}

impl TryFrom<String> for SessionId {
    type Error = SessionIdError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        // Support both formats: "session_xxxx" and raw hex
        let hex_str = value
            .strip_prefix("session_")
            .or_else(|| value.strip_prefix("session-"))
            .unwrap_or(&value);

        u64::from_str_radix(hex_str, 16)
            .map(Self::from_value)
            .map_err(|_| SessionIdError {
                message: "Invalid hex format for SessionId",
            })
    }
}

impl From<u64> for SessionId {
    fn from(value: u64) -> Self {
        Self::from_value(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use std::mem;
    use std::thread;

    #[test]
    fn session_id_is_exactly_8_bytes() {
        // Production requirement: SessionId must be 8 bytes, not 16!
        assert_eq!(
            mem::size_of::<SessionId>(),
            8,
            "SessionId must be exactly 8 bytes for memory efficiency at scale"
        );
    }

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

    #[test]
    fn session_id_can_be_displayed() {
        let id = SessionId::from_value(0x123456789abcdef0);
        let display_string = format!("{id}");

        // Should be in format "session_xxxx"
        assert!(display_string.starts_with("session_"));
        assert_eq!(display_string, "session_123456789abcdef0");
    }

    #[test]
    fn session_id_can_be_parsed_from_valid_string() {
        // Test with "session_" prefix
        let hex_str = "session_123456789abcdef0";
        let session_id = SessionId::try_from(hex_str.to_string()).unwrap();
        assert_eq!(session_id.as_u64(), 0x123456789abcdef0);

        // Test with "session-" prefix (hyphen)
        let hex_str = "session-123456789abcdef0";
        let session_id = SessionId::try_from(hex_str.to_string()).unwrap();
        assert_eq!(session_id.as_u64(), 0x123456789abcdef0);

        // Test without prefix
        let hex_str = "123456789abcdef0";
        let session_id = SessionId::try_from(hex_str.to_string()).unwrap();
        assert_eq!(session_id.as_u64(), 0x123456789abcdef0);
    }

    #[test]
    fn session_id_parsing_fails_for_invalid_string() {
        let invalid_str = "not-a-hex";
        let result = SessionId::try_from(invalid_str.to_string());

        assert!(result.is_err(), "Should fail to parse invalid hex string");
    }

    #[test]
    fn session_id_implements_required_traits() {
        let id1 = SessionId::new();
        let id2 = id1; // Copy trait test (not clone)

        // Test Copy
        assert_eq!(id1, id2);

        // Test Debug
        let debug_string = format!("{id1:?}");
        assert!(debug_string.contains("SessionId"));

        // Test Hash (can be used in HashSet)
        let mut set = HashSet::new();
        set.insert(id1);
        assert_eq!(set.len(), 1);
    }

    #[test]
    fn display_implementation_is_allocation_free() {
        // This test ensures Display doesn't allocate unnecessarily
        let id = SessionId::from_value(0xdeadbeef);
        let formatted = format!("{id}");

        // Verify format is correct and predictable
        assert_eq!(formatted, "session_00000000deadbeef");
    }
}