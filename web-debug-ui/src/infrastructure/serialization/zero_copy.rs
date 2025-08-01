//! Zero-Copy Serialization Implementation
//!
//! Stack-allocated responses and zero-copy operations for maximum performance.
//! Critical path operations must complete in <1ms.

use super::SerializationError;
use std::mem;
use std::time::Instant;

/// Stack-allocated response structure for maximum performance
///
/// This struct is designed to fit in CPU cache and avoid heap allocations
/// in the critical path of action processing.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct StackAllocatedActionResponse {
    /// Operation success status
    pub success: bool,
    /// Hash of the resulting game state for validation
    pub state_hash: u64,
    /// Execution time in nanoseconds
    pub execution_time_nanos: u64,
    /// Memory used in bytes
    pub memory_used_bytes: u32,
    /// Error code (0 = no error)
    pub error_code: u32,
    /// Reserved for future use (padding to cache line boundary)
    pub reserved: [u8; 16],
}

impl Default for StackAllocatedActionResponse {
    fn default() -> Self {
        Self {
            success: false,
            state_hash: 0,
            execution_time_nanos: 0,
            memory_used_bytes: 0,
            error_code: 0,
            reserved: [0; 16],
        }
    }
}

impl StackAllocatedActionResponse {
    /// Create successful response
    pub fn success(state_hash: u64, execution_time_nanos: u64, memory_used_bytes: u32) -> Self {
        Self {
            success: true,
            state_hash,
            execution_time_nanos,
            memory_used_bytes,
            error_code: 0,
            reserved: [0; 16],
        }
    }

    /// Create error response
    pub fn error(error_code: u32, execution_time_nanos: u64) -> Self {
        Self {
            success: false,
            state_hash: 0,
            execution_time_nanos,
            memory_used_bytes: 0,
            error_code,
            reserved: [0; 16],
        }
    }

    /// Zero-copy serialization to bytes
    pub fn as_bytes(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(self as *const Self as *const u8, mem::size_of::<Self>())
        }
    }

    /// Zero-copy deserialization from bytes
    pub fn from_bytes(bytes: &[u8]) -> Result<&Self, SerializationError> {
        if bytes.len() < mem::size_of::<Self>() {
            return Err(SerializationError::BufferTooSmall {
                required: mem::size_of::<Self>(),
                available: bytes.len(),
            });
        }

        unsafe { Ok(&*(bytes.as_ptr() as *const Self)) }
    }

    /// Get size in bytes (compile-time constant)
    pub const fn size() -> usize {
        mem::size_of::<Self>()
    }
}

/// High-performance zero-copy serializer
pub struct ZeroCopySerializer {
    buffer: Vec<u8>,
    performance_threshold_ms: u64,
}

impl ZeroCopySerializer {
    /// Create new serializer with pre-allocated buffer
    pub fn with_buffer_size(size: usize) -> Self {
        Self {
            buffer: Vec::with_capacity(size),
            performance_threshold_ms: 1, // 1ms threshold for serialization operations
        }
    }

    /// Serialize game state with zero-copy optimizations where possible
    pub fn serialize_state<T>(&mut self, state: &T) -> Result<&[u8], SerializationError>
    where
        T: serde::Serialize,
    {
        let start = Instant::now();

        // Clear buffer but keep capacity
        self.buffer.clear();

        // Use bincode for high-performance binary serialization
        bincode::serialize_into(&mut self.buffer, state).map_err(|e| {
            SerializationError::BincodeError {
                message: e.to_string(),
            }
        })?;

        let duration = start.elapsed();
        if duration.as_millis() > self.performance_threshold_ms as u128 {
            return Err(SerializationError::PerformanceThreshold {
                operation: "serialize_state".to_string(),
                duration_ms: duration.as_millis() as u64,
            });
        }

        Ok(&self.buffer)
    }

    /// Deserialize with performance monitoring
    pub fn deserialize_state<T>(&self, data: &[u8]) -> Result<T, SerializationError>
    where
        T: serde::de::DeserializeOwned,
    {
        let start = Instant::now();

        let result = bincode::deserialize(data).map_err(|e| SerializationError::BincodeError {
            message: e.to_string(),
        })?;

        let duration = start.elapsed();
        if duration.as_millis() > self.performance_threshold_ms as u128 {
            return Err(SerializationError::PerformanceThreshold {
                operation: "deserialize_state".to_string(),
                duration_ms: duration.as_millis() as u64,
            });
        }

        Ok(result)
    }

    /// Get current buffer capacity
    pub fn buffer_capacity(&self) -> usize {
        self.buffer.capacity()
    }

    /// Set performance threshold
    pub fn set_performance_threshold_ms(&mut self, threshold_ms: u64) {
        self.performance_threshold_ms = threshold_ms;
    }
}

/// High-performance game state serialization
pub fn serialize_game_state_zerocopy<T>(state: &T) -> Result<Vec<u8>, SerializationError>
where
    T: serde::Serialize,
{
    let start = Instant::now();

    // Use bincode for fastest binary serialization
    let result = bincode::serialize(state).map_err(|e| SerializationError::BincodeError {
        message: e.to_string(),
    })?;

    let duration = start.elapsed();
    if duration.as_millis() > 1 {
        return Err(SerializationError::PerformanceThreshold {
            operation: "serialize_game_state_zerocopy".to_string(),
            duration_ms: duration.as_millis() as u64,
        });
    }

    Ok(result)
}

/// High-performance game state deserialization
pub fn deserialize_game_state_zerocopy<T>(data: &[u8]) -> Result<T, SerializationError>
where
    T: serde::de::DeserializeOwned,
{
    let start = Instant::now();

    let result = bincode::deserialize(data).map_err(|e| SerializationError::BincodeError {
        message: e.to_string(),
    })?;

    let duration = start.elapsed();
    if duration.as_millis() > 1 {
        return Err(SerializationError::PerformanceThreshold {
            operation: "deserialize_game_state_zerocopy".to_string(),
            duration_ms: duration.as_millis() as u64,
        });
    }

    Ok(result)
}

/// Memory-aligned buffer for zero-copy operations
#[repr(align(64))] // Align to cache line boundary
pub struct AlignedBuffer {
    data: Vec<u8>,
}

impl AlignedBuffer {
    /// Create new aligned buffer with specified capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
        }
    }

    /// Get mutable slice for zero-copy writing
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        &mut self.data
    }

    /// Get immutable slice for zero-copy reading
    pub fn as_slice(&self) -> &[u8] {
        &self.data
    }

    /// Set length (unsafe - caller must ensure data is initialized)
    pub unsafe fn set_len(&mut self, len: usize) {
        self.data.set_len(len);
    }

    /// Get current capacity
    pub fn capacity(&self) -> usize {
        self.data.capacity()
    }

    /// Clear buffer while keeping capacity
    pub fn clear(&mut self) {
        self.data.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct TestGameState {
        score: u64,
        level: u32,
        player_name: String,
    }

    #[test]
    fn test_stack_allocated_response_zero_copy() {
        let response = StackAllocatedActionResponse::success(12345, 1000000, 1024);

        // Test zero-copy serialization
        let bytes = response.as_bytes();
        assert_eq!(bytes.len(), StackAllocatedActionResponse::size());

        // Test zero-copy deserialization
        let restored = StackAllocatedActionResponse::from_bytes(bytes).unwrap();
        assert_eq!(restored.success, true);
        assert_eq!(restored.state_hash, 12345);
        assert_eq!(restored.execution_time_nanos, 1000000);
        assert_eq!(restored.memory_used_bytes, 1024);
    }

    #[test]
    fn test_zero_copy_serializer() {
        let mut serializer = ZeroCopySerializer::with_buffer_size(1024);

        let test_state = TestGameState {
            score: 999,
            level: 5,
            player_name: "TestPlayer".to_string(),
        };

        // Test serialization
        let serialized_data = serializer.serialize_state(&test_state).unwrap();
        assert!(!serialized_data.is_empty());

        // Test deserialization
        let deserialized: TestGameState = serializer.deserialize_state(serialized_data).unwrap();
        assert_eq!(deserialized, test_state);
    }

    #[test]
    fn test_game_state_zero_copy_functions() {
        let test_state = TestGameState {
            score: 12345,
            level: 10,
            player_name: "Player1".to_string(),
        };

        // Test serialization function
        let serialized = serialize_game_state_zerocopy(&test_state).unwrap();
        assert!(!serialized.is_empty());

        // Test deserialization function
        let deserialized: TestGameState = deserialize_game_state_zerocopy(&serialized).unwrap();
        assert_eq!(deserialized, test_state);
    }

    #[test]
    fn test_aligned_buffer() {
        let mut buffer = AlignedBuffer::with_capacity(1024);
        assert_eq!(buffer.capacity(), 1024);

        // Test that we can write and read
        unsafe {
            let slice = buffer.as_mut_slice();
            slice.as_mut_ptr().write_bytes(0x42, 10);
            buffer.set_len(10);
        }

        let data = buffer.as_slice();
        assert_eq!(data.len(), 10);
        assert_eq!(data[0], 0x42);
    }

    #[test]
    fn test_error_response() {
        let error_response = StackAllocatedActionResponse::error(404, 500000);

        assert_eq!(error_response.success, false);
        assert_eq!(error_response.error_code, 404);
        assert_eq!(error_response.execution_time_nanos, 500000);
        assert_eq!(error_response.state_hash, 0);
    }

    #[test]
    fn test_buffer_too_small_error() {
        let small_buffer = [0u8; 4];
        let result = StackAllocatedActionResponse::from_bytes(&small_buffer);

        match result {
            Err(SerializationError::BufferTooSmall {
                required,
                available,
            }) => {
                assert_eq!(required, StackAllocatedActionResponse::size());
                assert_eq!(available, 4);
            }
            _ => panic!("Expected BufferTooSmall error"),
        }
    }
}
