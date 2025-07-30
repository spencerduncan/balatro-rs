//! Core trait definitions for the skip tag system
//!
//! This module defines the foundational traits and types that all skip tags must implement.
//! The design follows the established patterns from the joker system while being
//! optimized for the specific needs of skip tag mechanics.

use crate::game::Game;
use crate::skip_tags::error::TagError;
#[cfg(feature = "python")]
use pyo3::pyclass;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Core trait that all skip tags must implement.
///
/// This trait defines the complete interface for skip tag behavior, including
/// identity, conditions, effects, and metadata. All methods are required to
/// ensure consistent behavior across all tag implementations.
///
/// # Design Principles
///
/// - **Identity**: Each tag has a unique ID, name, and description
/// - **Conditional**: Tags can check if they're applicable in current game state
/// - **Effectful**: Tags can modify game state when applied
/// - **Typed**: Tags are classified by their effect type for system integration
///
/// # Performance Requirements
///
/// - `can_apply()`: Must complete in <10μs for responsive UI
/// - `apply_effect()`: Must complete in <100ms for smooth gameplay
/// - All trait methods must be thread-safe (Send + Sync)
///
/// # Example Implementation
///
/// ```rust,ignore
/// struct CharmTag;
///
/// impl SkipTag for CharmTag {
///     fn id(&self) -> TagId { TagId::Charm }
///     fn name(&self) -> &'static str { "Charm" }
///     fn effect_type(&self) -> TagEffectType { TagEffectType::ImmediateReward }
///
///     fn can_apply(&self, game: &Game) -> bool {
///         // Always applicable - charm gives immediate reward
///         true
///     }
///
///     fn apply_effect(&self, game: &mut Game) -> Result<(), TagError> {
///         // Award $4 immediately
///         game.money += 4.0;
///         Ok(())
///     }
///
///     fn description(&self) -> &'static str {
///         "Gives $4 when selected"
///     }
/// }
/// ```
pub trait SkipTag: Send + Sync {
    /// Returns the unique identifier for this skip tag.
    ///
    /// This ID is used for registry lookups, serialization, and cross-system
    /// integration. Must be consistent across all instances of the same tag type.
    fn id(&self) -> TagId;

    /// Returns the human-readable name of this skip tag.
    ///
    /// Used for UI display and logging. Should be concise and descriptive.
    /// Example: "Charm", "Double Tag", "Voucher Pack"
    fn name(&self) -> &'static str;

    /// Returns the classification type of this skip tag's effect.
    ///
    /// Used by the game engine to determine when and how to process the tag's
    /// effects. Critical for correct integration with shop, blind, and reward systems.
    fn effect_type(&self) -> TagEffectType;

    /// Checks if this tag can be applied given the current game state.
    ///
    /// This method is called during tag selection to determine if a tag should
    /// be offered to the player. Should be fast (<10μs) and side-effect free.
    ///
    /// # Arguments
    /// - `game`: Current game state for condition checking
    ///
    /// # Returns
    /// - `true` if the tag can be applied
    /// - `false` if conditions are not met
    ///
    /// # Examples
    /// - Economy tags check if player has money to give
    /// - Shop modifier tags check if shop is accessible
    /// - Blind-specific tags check current blind type
    fn can_apply(&self, game: &Game) -> bool;

    /// Applies this tag's effect to the game state.
    ///
    /// This is where the tag's core functionality is implemented. Must be
    /// idempotent and handle all error conditions gracefully.
    ///
    /// # Arguments
    /// - `game`: Mutable game state to modify
    ///
    /// # Returns
    /// - `Ok(())` if effect was applied successfully
    /// - `Err(TagError)` if application failed
    ///
    /// # Error Handling
    /// Should return appropriate TagError variants for different failure modes:
    /// - `TagError::InvalidGameState` if game state prevents application
    /// - `TagError::InsufficientResources` if required resources unavailable
    /// - `TagError::SystemError` for unexpected internal errors
    fn apply_effect(&self, game: &mut Game) -> Result<(), TagError>;

    /// Returns a detailed description of what this tag does.
    ///
    /// Used for tooltips, help text, and documentation. Should be clear and
    /// specific about the tag's effects and any conditions.
    ///
    /// # Examples
    /// - "Gives $4 when selected"
    /// - "Next shop has all Jokers at half price"
    /// - "Reroll the Boss Blind"
    fn description(&self) -> &'static str;
}

/// Unique identifier for each skip tag in the system.
///
/// This enum contains all 26 skip tags organized by their categories.
/// The organization follows the Balatro game structure for consistency
/// and maintainability.
///
/// # Categories
/// - **Reward Tags (8)**: Immediate rewards (packs, money, etc.)
/// - **Economic Tags (5)**: Money and economy-based effects
/// - **Shop Enhancement Tags (7)**: Modify next shop experience
/// - **Utility Tags (4)**: Game state and mechanic modifiers
/// - **Special Tags (2)**: Unique mechanics (Double, Boss)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub enum TagId {
    // Reward Tags (8) - Immediate pack and item rewards
    /// Gives a free Mega Arcana Pack
    Charm,
    /// Gives a free Spectral Pack
    Ethereal,
    /// Gives a free Buffoon Pack
    Buffoon,
    /// Gives a free Standard Pack
    Standard,
    /// Gives a free Celestial Pack
    Meteor,
    /// Creates a rare Joker
    Rare,
    /// Creates an uncommon Joker
    Uncommon,
    /// Fills all consumable slots if there is room
    TopUp,

    // Economic Tags (5) - Money and resource management
    /// Gives $1 for every $5 you have (max of $25)
    Economy,
    /// Earn $25 when this is destroyed
    Investment,
    /// Gives $1 for each discard played (max of $6)
    Garbage,
    /// Gives $1 for each hand played this run (max of $48)
    Speed,
    /// Gives $1 for every 2 cards in hand (max of $4)
    Handy,

    // Shop Enhancement Tags (7) - Modify next shop experience
    /// Next shop has a free Mega Voucher Pack
    Voucher,
    /// Next shop has all items at half price
    Coupon,
    /// Reroll the shop up to 3 times for free
    D6,
    /// Next shop has Foil, Holographic, or Polychrome Jokers
    Foil,
    /// Next shop has Holographic Jokers
    Holographic,
    /// Next shop has Polychrome Jokers
    Polychrome,
    /// Creates a Negative Joker
    Negative,

    // Utility Tags (4) - Game state and mechanic modifiers
    /// Gives a copy of the next selected tag
    Double,
    /// Reroll the Boss Blind
    Boss,
    /// Upgrade the level of the most played poker hand
    Orbital,
    /// Gives +1 hand size for the next blind
    Juggle,
}

impl TagId {
    /// Returns all tag IDs in a static array for efficient iteration.
    ///
    /// Used by registry initialization and testing to ensure all tags
    /// are properly registered and covered.
    pub const fn all() -> [TagId; 24] {
        [
            // Reward Tags
            TagId::Charm,
            TagId::Ethereal,
            TagId::Buffoon,
            TagId::Standard,
            TagId::Meteor,
            TagId::Rare,
            TagId::Uncommon,
            TagId::TopUp,
            // Economic Tags
            TagId::Economy,
            TagId::Investment,
            TagId::Garbage,
            TagId::Speed,
            TagId::Handy,
            // Shop Enhancement Tags
            TagId::Voucher,
            TagId::Coupon,
            TagId::D6,
            TagId::Foil,
            TagId::Holographic,
            TagId::Polychrome,
            TagId::Negative,
            // Utility Tags
            TagId::Double,
            TagId::Boss,
            TagId::Orbital,
            TagId::Juggle,
        ]
    }

    /// Returns the category this tag belongs to.
    ///
    /// Used for organization, filtering, and probability calculations
    /// during tag selection.
    pub const fn category(&self) -> TagCategory {
        match self {
            TagId::Charm
            | TagId::Ethereal
            | TagId::Buffoon
            | TagId::Standard
            | TagId::Meteor
            | TagId::Rare
            | TagId::Uncommon
            | TagId::TopUp => TagCategory::Reward,

            TagId::Economy | TagId::Investment | TagId::Garbage | TagId::Speed | TagId::Handy => {
                TagCategory::Economic
            }

            TagId::Voucher
            | TagId::Coupon
            | TagId::D6
            | TagId::Foil
            | TagId::Holographic
            | TagId::Polychrome
            | TagId::Negative => TagCategory::ShopEnhancement,

            TagId::Double | TagId::Boss | TagId::Orbital | TagId::Juggle => TagCategory::Utility,
        }
    }
}

impl fmt::Display for TagId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            TagId::Charm => "Charm",
            TagId::Ethereal => "Ethereal",
            TagId::Buffoon => "Buffoon",
            TagId::Standard => "Standard",
            TagId::Meteor => "Meteor",
            TagId::Rare => "Rare",
            TagId::Uncommon => "Uncommon",
            TagId::TopUp => "Top Up",
            TagId::Economy => "Economy",
            TagId::Investment => "Investment",
            TagId::Garbage => "Garbage",
            TagId::Speed => "Speed",
            TagId::Handy => "Handy",
            TagId::Voucher => "Voucher",
            TagId::Coupon => "Coupon",
            TagId::D6 => "D6",
            TagId::Foil => "Foil",
            TagId::Holographic => "Holographic",
            TagId::Polychrome => "Polychrome",
            TagId::Negative => "Negative",
            TagId::Double => "Double",
            TagId::Boss => "Boss",
            TagId::Orbital => "Orbital",
            TagId::Juggle => "Juggle",
        };
        write!(f, "{name}")
    }
}

/// Classification system for skip tag effects.
///
/// This enum categorizes how and when tag effects are applied, enabling
/// the game engine to process them at the correct time and integrate
/// them properly with other systems.
///
/// # Integration Points
/// - **ImmediateReward**: Applied instantly when tag is selected
/// - **NextShopModifier**: Applied when entering the next shop
/// - **GameStateModifier**: Applied to ongoing game mechanics
/// - **SpecialMechanic**: Requires custom handling logic
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub enum TagEffectType {
    /// Effects that provide immediate rewards when selected.
    ///
    /// Examples: Money awards, pack generation, item creation
    /// Timing: Applied immediately during tag selection
    /// Integration: Reward system, inventory management
    ImmediateReward,

    /// Effects that modify the player's next shop experience.
    ///
    /// Examples: Price discounts, free rerolls, item upgrades
    /// Timing: Applied when entering the next shop
    /// Integration: Shop generation system, pricing calculations
    NextShopModifier,

    /// Effects that modify ongoing game state or mechanics.
    ///
    /// Examples: Hand size changes, blind modifications
    /// Timing: Applied for duration specified by effect
    /// Integration: Game flow, blind system, hand evaluation
    GameStateModifier,

    /// Effects that require special handling or unique mechanics.
    ///
    /// Examples: Tag duplication, boss blind rerolls
    /// Timing: Custom logic per tag type
    /// Integration: Multiple systems, requires specific implementation
    SpecialMechanic,
}

impl fmt::Display for TagEffectType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            TagEffectType::ImmediateReward => "Immediate Reward",
            TagEffectType::NextShopModifier => "Next Shop Modifier",
            TagEffectType::GameStateModifier => "Game State Modifier",
            TagEffectType::SpecialMechanic => "Special Mechanic",
        };
        write!(f, "{name}")
    }
}

/// Categories for organizing skip tags.
///
/// Used for UI organization, probability calculations, and system integration.
/// Each category has different selection weights and integration points.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub enum TagCategory {
    /// Tags that provide immediate rewards (packs, items, etc.)
    Reward,
    /// Tags focused on money and resource management
    Economic,
    /// Tags that enhance the next shop experience
    ShopEnhancement,
    /// Tags that modify game mechanics or state
    Utility,
}

impl fmt::Display for TagCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            TagCategory::Reward => "Reward",
            TagCategory::Economic => "Economic",
            TagCategory::ShopEnhancement => "Shop Enhancement",
            TagCategory::Utility => "Utility",
        };
        write!(f, "{name}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tag_id_all_count() {
        // Verify we have exactly 24 tags as implemented
        assert_eq!(TagId::all().len(), 24);
    }

    #[test]
    fn test_tag_id_all_unique() {
        // Verify all tag IDs are unique
        let tags = TagId::all();
        let mut unique_tags = std::collections::HashSet::new();

        for tag in tags.iter() {
            assert!(unique_tags.insert(*tag), "Duplicate tag found: {tag:?}");
        }

        assert_eq!(unique_tags.len(), 24);
    }

    #[test]
    fn test_tag_categories() {
        // Verify reward tags (8)
        let reward_tags = [
            TagId::Charm,
            TagId::Ethereal,
            TagId::Buffoon,
            TagId::Standard,
            TagId::Meteor,
            TagId::Rare,
            TagId::Uncommon,
            TagId::TopUp,
        ];
        for tag in reward_tags.iter() {
            assert_eq!(tag.category(), TagCategory::Reward);
        }

        // Verify economic tags (5)
        let economic_tags = [
            TagId::Economy,
            TagId::Investment,
            TagId::Garbage,
            TagId::Speed,
            TagId::Handy,
        ];
        for tag in economic_tags.iter() {
            assert_eq!(tag.category(), TagCategory::Economic);
        }

        // Verify shop enhancement tags (7)
        let shop_tags = [
            TagId::Voucher,
            TagId::Coupon,
            TagId::D6,
            TagId::Foil,
            TagId::Holographic,
            TagId::Polychrome,
            TagId::Negative,
        ];
        for tag in shop_tags.iter() {
            assert_eq!(tag.category(), TagCategory::ShopEnhancement);
        }

        // Verify utility tags (4)
        let utility_tags = [TagId::Double, TagId::Boss, TagId::Orbital, TagId::Juggle];
        for tag in utility_tags.iter() {
            assert_eq!(tag.category(), TagCategory::Utility);
        }
    }

    #[test]
    fn test_tag_id_display() {
        // Verify display names are correct
        assert_eq!(TagId::Charm.to_string(), "Charm");
        assert_eq!(TagId::TopUp.to_string(), "Top Up");
        assert_eq!(TagId::D6.to_string(), "D6");
        assert_eq!(TagId::Holographic.to_string(), "Holographic");
    }

    #[test]
    fn test_tag_effect_type_display() {
        assert_eq!(
            TagEffectType::ImmediateReward.to_string(),
            "Immediate Reward"
        );
        assert_eq!(
            TagEffectType::NextShopModifier.to_string(),
            "Next Shop Modifier"
        );
        assert_eq!(
            TagEffectType::GameStateModifier.to_string(),
            "Game State Modifier"
        );
        assert_eq!(
            TagEffectType::SpecialMechanic.to_string(),
            "Special Mechanic"
        );
    }

    #[test]
    fn test_tag_category_display() {
        assert_eq!(TagCategory::Reward.to_string(), "Reward");
        assert_eq!(TagCategory::Economic.to_string(), "Economic");
        assert_eq!(TagCategory::ShopEnhancement.to_string(), "Shop Enhancement");
        assert_eq!(TagCategory::Utility.to_string(), "Utility");
    }

    #[test]
    fn test_category_counts() {
        let tags = TagId::all();
        let mut category_counts = std::collections::HashMap::new();

        for tag in tags.iter() {
            *category_counts.entry(tag.category()).or_insert(0) += 1;
        }

        // Verify expected counts per architecture spec
        assert_eq!(category_counts[&TagCategory::Reward], 8);
        assert_eq!(category_counts[&TagCategory::Economic], 5);
        assert_eq!(category_counts[&TagCategory::ShopEnhancement], 7);
        assert_eq!(category_counts[&TagCategory::Utility], 4);
    }

    #[test]
    fn test_serialization() {
        // Test that tag IDs can be serialized/deserialized
        let tag = TagId::Charm;
        let serialized = serde_json::to_string(&tag).unwrap();
        let deserialized: TagId = serde_json::from_str(&serialized).unwrap();
        assert_eq!(tag, deserialized);
    }

    #[test]
    fn test_hash_consistency() {
        // Verify that TagId implements Hash consistently
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let tag1 = TagId::Charm;
        let tag2 = TagId::Charm;

        let mut hasher1 = DefaultHasher::new();
        tag1.hash(&mut hasher1);
        let hash1 = hasher1.finish();

        let mut hasher2 = DefaultHasher::new();
        tag2.hash(&mut hasher2);
        let hash2 = hasher2.finish();

        assert_eq!(hash1, hash2);
    }
}
