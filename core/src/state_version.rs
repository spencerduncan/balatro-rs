//! State version management for game serialization compatibility
//!
//! This module provides version tracking for game state serialization,
//! enabling migration of save files from older versions of the game.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Version identifier for game state serialization
///
/// This enum tracks the version of the game state format to enable
/// migration of save files when the state structure changes.
///
/// # Migration Strategy
///
/// When adding new fields or changing existing ones:
/// 1. Add a new version variant
/// 2. Update `StateVersion::current()` to return the new version
/// 3. Implement migration logic in the Game struct's deserialization
/// 4. Test migration from all previous versions
///
/// # Examples
///
/// ```rust
/// use balatro_rs::state_version::StateVersion;
///
/// // Get current version for new saves
/// let current = StateVersion::current();
/// assert_eq!(current, StateVersion::V2);
///
/// // Check if migration is needed
/// let loaded_version = StateVersion::V1;
/// if loaded_version != StateVersion::current() {
///     // Perform migration
/// }
/// ```
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StateVersion {
    /// Initial version with basic game state
    ///
    /// Includes: config, shop, deck, available, discarded, blind, stage,
    /// ante progression, jokers, playing state, scoring state
    V1,

    /// Extended state version with consumables, vouchers, and boss blinds
    ///
    /// Adds: consumables_in_hand, vouchers, boss_blind_state, state_version
    V2,
}

impl StateVersion {
    /// Get the current version for new save files
    ///
    /// This should be updated whenever the state format changes.
    ///
    /// # Returns
    /// The current `StateVersion` variant
    pub fn current() -> Self {
        Self::V2
    }

    /// Check if this version can be migrated to the current version
    ///
    /// # Returns
    /// `true` if migration is supported, `false` otherwise
    pub fn can_migrate_to_current(&self) -> bool {
        *self <= Self::current()
    }

    /// Get a human-readable description of this version
    ///
    /// # Returns
    /// A string describing what this version includes
    pub fn description(&self) -> &'static str {
        match self {
            Self::V1 => "Basic game state with jokers and core mechanics",
            Self::V2 => "Extended state with consumables, vouchers, and boss blinds",
        }
    }

    /// Get the version number for display
    ///
    /// # Returns
    /// The version number as a string
    pub fn version_number(&self) -> &'static str {
        match self {
            Self::V1 => "1.0",
            Self::V2 => "2.0",
        }
    }
}

impl Default for StateVersion {
    fn default() -> Self {
        Self::current()
    }
}

impl std::fmt::Display for StateVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "v{}", self.version_number())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_current_version() {
        assert_eq!(StateVersion::current(), StateVersion::V2);
    }

    #[test]
    fn test_version_ordering() {
        assert!(StateVersion::V1 < StateVersion::V2);
        assert!(StateVersion::V2 >= StateVersion::V1);
    }

    #[test]
    fn test_migration_support() {
        assert!(StateVersion::V1.can_migrate_to_current());
        assert!(StateVersion::V2.can_migrate_to_current());
    }

    #[test]
    fn test_version_display() {
        assert_eq!(StateVersion::V1.to_string(), "v1.0");
        assert_eq!(StateVersion::V2.to_string(), "v2.0");
    }

    #[test]
    fn test_version_descriptions() {
        assert!(!StateVersion::V1.description().is_empty());
        assert!(!StateVersion::V2.description().is_empty());
    }

    #[test]
    fn test_default_version() {
        assert_eq!(StateVersion::default(), StateVersion::current());
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_version_serialization() {
        let version = StateVersion::V2;
        let serialized = serde_json::to_string(&version).unwrap();
        let deserialized: StateVersion = serde_json::from_str(&serialized).unwrap();
        assert_eq!(version, deserialized);
    }
}
