//! SessionId Value Object
//!
//! SessionId represents a unique identifier for a game session.
//! Following Domain-Driven Design principles, it encapsulates validation
//! and provides type safety for session identification.

use std::fmt;

/// Unique identifier for a game session
///
/// SessionId is a value object that wraps a UUID and provides
/// type safety and validation for session identification.
///
/// # Examples
///
/// ```
/// use balatro_rs::domain::SessionId;
///
/// // Generate a new session ID
/// let session_id = SessionId::new();
///
/// // Parse from string
/// let parsed = SessionId::try_from("550e8400-e29b-41d4-a716-446655440000".to_string()).unwrap();
///
/// // Convert to string
/// let id_string = session_id.to_string();
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionId(uuid::Uuid);

impl SessionId {
    /// Generate a new random SessionId
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4())
    }

    /// Get the underlying UUID
    pub fn as_uuid(&self) -> &uuid::Uuid {
        &self.0
    }

    /// Convert to string representation
    pub fn as_str(&self) -> String {
        self.0.to_string()
    }
}

impl Default for SessionId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for SessionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<String> for SessionId {
    type Error = uuid::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        uuid::Uuid::parse_str(&value).map(Self)
    }
}

impl From<uuid::Uuid> for SessionId {
    fn from(uuid: uuid::Uuid) -> Self {
        Self(uuid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn new_session_id_is_unique() {
        let id1 = SessionId::new();
        let id2 = SessionId::new();

        assert_ne!(id1, id2, "Each new SessionId should be unique");
    }

    #[test]
    fn session_id_can_be_displayed() {
        let id = SessionId::new();
        let display_string = format!("{id}");

        // Should be a valid UUID string format
        assert_eq!(display_string.len(), 36); // UUID format: 8-4-4-4-12
        assert!(display_string.contains('-'));
    }

    #[test]
    fn session_id_can_be_parsed_from_valid_string() {
        let uuid_str = "550e8400-e29b-41d4-a716-446655440000";
        let session_id = SessionId::try_from(uuid_str.to_string()).unwrap();

        assert_eq!(session_id.to_string(), uuid_str);
    }

    #[test]
    fn session_id_parsing_fails_for_invalid_string() {
        let invalid_str = "not-a-uuid";
        let result = SessionId::try_from(invalid_str.to_string());

        assert!(result.is_err(), "Should fail to parse invalid UUID string");
    }

    #[test]
    fn session_id_implements_required_traits() {
        let id1 = SessionId::new();
        let id2 = id1.clone();

        // Test Clone
        assert_eq!(id1, id2);

        // Test Debug
        let debug_string = format!("{id1:?}");
        assert!(debug_string.contains("SessionId"));

        // Test Hash (can be used in HashSet)
        let mut set = HashSet::new();
        set.insert(id1);
        assert_eq!(set.len(), 1);
    }
}
