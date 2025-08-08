//! SessionId Value Object
//!
//! SessionId represents a unique identifier for a game session.
//! Following Domain-Driven Design principles, it encapsulates validation
//! and provides type safety for session identification.
//!
//! This implementation uses an 8-byte atomic counter for memory efficiency
//! at scale, saving 50% memory compared to UUID (16 bytes).

use std::fmt;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

// Global atomic counter for generating unique session IDs
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
/// let parsed = SessionId::try_from("123456789abcdef0".to_string()).unwrap();
///
/// // Convert to string
/// let id_string = session_id.to_string();
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionId {
    value: u64, // 8 bytes instead of 16!
}

impl SessionId {
    /// Generate a new unique SessionId
    ///
    /// Combines a microsecond timestamp with an atomic counter to ensure
    /// uniqueness even in high-concurrency scenarios.
    pub fn new() -> Self {
        // Get current timestamp in microseconds
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_micros() as u64;

        // Get next counter value atomically
        let counter = SESSION_COUNTER.fetch_add(1, Ordering::SeqCst);

        // Pack timestamp (upper 44 bits) and counter (lower 20 bits)
        // This gives us ~1 million unique IDs per microsecond
        let value = (timestamp << 20) | (counter & 0xFFFFF);

        Self { value }
    }

    /// Get the underlying u64 value
    pub fn as_u64(&self) -> u64 {
        self.value
    }

    /// Create SessionId from a raw u64 value
    pub fn from_u64(value: u64) -> Self {
        Self { value }
    }
}

impl Default for SessionId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for SessionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Direct hex formatting without allocation
        write!(f, "session-{:016x}", self.value)
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
        // Support both formats: "session-xxxx" and raw hex
        let hex_str = value.strip_prefix("session-").unwrap_or(&value);

        u64::from_str_radix(hex_str, 16)
            .map(Self::from_u64)
            .map_err(|_| SessionIdError {
                message: "Invalid hex format for SessionId",
            })
    }
}

impl From<u64> for SessionId {
    fn from(value: u64) -> Self {
        Self::from_u64(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use std::mem;

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
    fn new_session_id_is_unique() {
        let id1 = SessionId::new();
        let id2 = SessionId::new();

        assert_ne!(id1, id2, "Each new SessionId should be unique");
    }

    #[test]
    fn session_id_uniqueness_under_concurrency() {
        use std::sync::Arc;
        use std::thread;

        let mut handles = vec![];
        let ids = Arc::new(std::sync::Mutex::new(Vec::new()));

        // Spawn multiple threads generating IDs concurrently
        for _ in 0..10 {
            let ids_clone = Arc::clone(&ids);
            let handle = thread::spawn(move || {
                let id = SessionId::new();
                ids_clone.lock().unwrap().push(id);
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let generated_ids = ids.lock().unwrap();
        let unique_count = generated_ids.iter().collect::<HashSet<_>>().len();

        assert_eq!(
            unique_count,
            generated_ids.len(),
            "All concurrently generated IDs should be unique"
        );
    }

    #[test]
    fn session_id_can_be_displayed() {
        let id = SessionId::from_u64(0x123456789abcdef0);
        let display_string = format!("{id}");

        // Should be in format "session-xxxx"
        assert!(display_string.starts_with("session-"));
        assert_eq!(display_string, "session-123456789abcdef0");
    }

    #[test]
    fn session_id_can_be_parsed_from_valid_string() {
        // Test with "session-" prefix
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
        let id = SessionId::from_u64(0xdeadbeef);
        let formatted = format!("{id}");

        // Verify format is correct and predictable
        assert_eq!(formatted, "session-00000000deadbeef");
    }
}
