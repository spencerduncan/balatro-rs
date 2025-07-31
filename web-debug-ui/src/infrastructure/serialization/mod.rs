//! Zero-Copy Serialization for Maximum Performance
//!
//! CRITICAL PERFORMANCE TARGETS:
//! - Stack allocation for responses where possible
//! - Minimize heap allocations in critical paths
//! - <1ms serialization for common operations

#![cfg(feature = "zero-copy")]

pub mod zero_copy;

pub use zero_copy::{
    deserialize_game_state_zerocopy, serialize_game_state_zerocopy,
    StackAllocatedActionResponse, ZeroCopySerializer,
};

use std::mem;

/// Serialization error types
#[derive(thiserror::Error, Debug)]
pub enum SerializationError {
    #[error("Bincode serialization failed: {message}")]
    BincodeError { message: String },

    #[error("Buffer too small: need {required} bytes, have {available}")]
    BufferTooSmall { required: usize, available: usize },

    #[error("Invalid data format: {message}")]
    InvalidFormat { message: String },

    #[error("Serialization performance threshold exceeded: {operation} took {duration_ms}ms")]
    PerformanceThreshold { operation: String, duration_ms: u64 },
}

/// Common trait for zero-copy serializable types
pub trait ZeroCopySerializable {
    /// Serialize to stack-allocated buffer if possible
    fn serialize_to_stack(&self) -> Result<Vec<u8>, SerializationError>;

    /// Serialize directly to provided buffer (zero-copy)
    fn serialize_to_buffer(&self, buffer: &mut [u8]) -> Result<usize, SerializationError>;

    /// Get the expected serialized size for buffer allocation
    fn serialized_size(&self) -> usize;
}

/// High-performance message protocol for WebSocket communication
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MessageHeader {
    /// Message type identifier
    pub message_type: u32,
    /// Payload length in bytes
    pub payload_length: u32,
    /// Timestamp in microseconds
    pub timestamp_micros: u64,
    /// CRC32 checksum for integrity
    pub checksum: u32,
}

impl MessageHeader {
    pub const SIZE: usize = mem::size_of::<Self>();

    /// Create new message header
    pub fn new(message_type: u32, payload_length: u32) -> Self {
        Self {
            message_type,
            payload_length,
            timestamp_micros: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_micros() as u64,
            checksum: 0, // Would be calculated in real implementation
        }
    }

    /// Convert to bytes (zero-copy)
    pub fn as_bytes(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self as *const Self as *const u8, Self::SIZE) }
    }

    /// Create from bytes (zero-copy)
    pub fn from_bytes(bytes: &[u8]) -> Result<&Self, SerializationError> {
        if bytes.len() < Self::SIZE {
            return Err(SerializationError::BufferTooSmall {
                required: Self::SIZE,
                available: bytes.len(),
            });
        }

        unsafe { Ok(&*(bytes.as_ptr() as *const Self)) }
    }
}

/// Message types for the protocol
pub mod message_types {
    pub const STATE_UPDATE: u32 = 1;
    pub const ACTION_REQUEST: u32 = 2;
    pub const ACTION_RESPONSE: u32 = 3;
    pub const ERROR: u32 = 4;
    pub const PING: u32 = 5;
    pub const PONG: u32 = 6;
}

/// High-performance binary protocol builder
pub struct BinaryProtocolBuilder {
    buffer: Vec<u8>,
}

impl BinaryProtocolBuilder {
    /// Create new protocol builder with pre-allocated capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            buffer: Vec::with_capacity(capacity),
        }
    }

    /// Add message header
    pub fn add_header(&mut self, header: MessageHeader) -> &mut Self {
        self.buffer.extend_from_slice(header.as_bytes());
        self
    }

    /// Add payload data
    pub fn add_payload(&mut self, data: &[u8]) -> &mut Self {
        self.buffer.extend_from_slice(data);
        self
    }

    /// Build final message (consumes builder)
    pub fn build(self) -> Vec<u8> {
        self.buffer
    }

    /// Get current buffer size
    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    /// Check if buffer is empty
    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_header_zero_copy() {
        let header = MessageHeader::new(message_types::STATE_UPDATE, 1024);

        // Test zero-copy conversion to bytes
        let bytes = header.as_bytes();
        assert_eq!(bytes.len(), MessageHeader::SIZE);

        // Test zero-copy conversion from bytes
        let restored = MessageHeader::from_bytes(bytes).unwrap();
        assert_eq!(restored.message_type, message_types::STATE_UPDATE);
        assert_eq!(restored.payload_length, 1024);
    }

    #[test]
    fn test_binary_protocol_builder() {
        let mut builder = BinaryProtocolBuilder::with_capacity(1024);

        let header = MessageHeader::new(message_types::ACTION_RESPONSE, 8);
        let payload = [1u8, 2, 3, 4, 5, 6, 7, 8];

        let message = builder.add_header(header).add_payload(&payload).build();

        assert_eq!(message.len(), MessageHeader::SIZE + 8);

        // Verify we can parse it back
        let parsed_header = MessageHeader::from_bytes(&message[..MessageHeader::SIZE]).unwrap();
        assert_eq!(parsed_header.message_type, message_types::ACTION_RESPONSE);
        assert_eq!(parsed_header.payload_length, 8);
    }

    #[test]
    fn test_message_header_size() {
        // Verify the struct is packed as expected
        assert_eq!(MessageHeader::SIZE, 20); // 4 + 4 + 8 + 4 bytes
    }

    #[test]
    fn test_buffer_too_small_error() {
        let small_buffer = [0u8; 4];
        let result = MessageHeader::from_bytes(&small_buffer);

        match result {
            Err(SerializationError::BufferTooSmall {
                required,
                available,
            }) => {
                assert_eq!(required, MessageHeader::SIZE);
                assert_eq!(available, 4);
            }
            _ => panic!("Expected BufferTooSmall error"),
        }
    }
}
