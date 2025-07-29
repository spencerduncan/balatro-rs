//! Skip Tag System Implementation
//!
//! The Skip Tag System provides strategic rewards when players skip blinds instead of playing them.
//! This module implements all skip tag functionality following clean code principles.

pub mod shop_tags;
pub mod tag_effects;
pub mod tag_registry;
pub mod utility_tags;

#[cfg(test)]
mod integration_tests;

// Re-export public API
pub use shop_tags::*;
pub use tag_effects::*;
pub use tag_registry::*;

use crate::game::Game;
use crate::stage::Blind;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Categories of tag effects - unified from both implementations
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TagEffectType {
    /// Immediate rewards (money, packs, etc.)
    ImmediateReward,
    /// Modifies next shop
    NextShopModifier,
    /// Modifies game state temporarily
    GameStateModifier,
    /// Special mechanics (duplication, etc.)
    SpecialMechanic,
    /// Boss blind interactions
    BossBlindModifier,
}

/// All available skip tag IDs - comprehensive set from both implementations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "python", pyo3::pyclass(eq, eq_int))]
pub enum SkipTagId {
    // Economic Tags (Phase 2 - Issue #693)
    Economy,
    Investment,
    Garbage,
    Speed,
    Handy,

    // Shop Enhancement Tags (Phase 2 - Issue #694)
    Voucher,
    Coupon,
    D6,
    Foil,
    Holographic,
    Polychrome,

    // Reward Tags (Phase 2 - Issue #692)
    Charm,
    Ethereal,
    Buffoon,
    Standard,
    Meteor,
    Rare,
    Uncommon,
    TopUp,

    // Utility Tags (Phase 3 - from main branch)
    Double,
    Boss,
    Orbital,
    Juggle,
}

impl SkipTagId {
    /// Get the display name for this tag
    pub fn name(&self) -> &'static str {
        match self {
            SkipTagId::Economy => "Economy",
            SkipTagId::Investment => "Investment",
            SkipTagId::Garbage => "Garbage",
            SkipTagId::Speed => "Speed",
            SkipTagId::Handy => "Handy",

            SkipTagId::Voucher => "Voucher",
            SkipTagId::Coupon => "Coupon",
            SkipTagId::D6 => "D6",
            SkipTagId::Foil => "Foil",
            SkipTagId::Holographic => "Holographic",
            SkipTagId::Polychrome => "Polychrome",

            SkipTagId::Charm => "Charm",
            SkipTagId::Ethereal => "Ethereal",
            SkipTagId::Buffoon => "Buffoon",
            SkipTagId::Standard => "Standard",
            SkipTagId::Meteor => "Meteor",
            SkipTagId::Rare => "Rare",
            SkipTagId::Uncommon => "Uncommon",
            SkipTagId::TopUp => "TopUp",

            SkipTagId::Double => "Double",
            SkipTagId::Boss => "Boss",
            SkipTagId::Orbital => "Orbital",
            SkipTagId::Juggle => "Juggle",
        }
    }
}

impl fmt::Display for SkipTagId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// Context for skip tag activation - unified approach
#[derive(Debug)]
pub struct SkipTagContext {
    /// The game state when tag is activated
    pub game: Game,
    /// The blind that was skipped (if applicable)
    pub skipped_blind: Option<Blind>,
    /// Additional tags available for duplication (for Double tag)
    pub available_tags: Vec<SkipTagId>,
}

/// Result of applying a skip tag effect
#[derive(Debug, Clone, PartialEq)]
pub struct TagEffectResult {
    /// Money to award immediately
    pub money_reward: i32,
    /// Messages to display to the player
    pub messages: Vec<String>,
    /// Whether this tag should persist for future events
    pub persist_tag: bool,
}

impl TagEffectResult {
    /// Create a new empty result
    pub fn new() -> Self {
        Self {
            money_reward: 0,
            messages: Vec::new(),
            persist_tag: false,
        }
    }

    /// Create a result with money reward
    pub fn with_money(money: i32) -> Self {
        Self {
            money_reward: money,
            messages: Vec::new(),
            persist_tag: false,
        }
    }

    /// Create a result with money and message
    pub fn with_money_and_message(money: i32, message: String) -> Self {
        Self {
            money_reward: money,
            messages: vec![message],
            persist_tag: false,
        }
    }

    /// Create a result that persists for future events
    pub fn with_persistence(money: i32, message: String) -> Self {
        Self {
            money_reward: money,
            messages: vec![message],
            persist_tag: true,
        }
    }
}

impl Default for TagEffectResult {
    fn default() -> Self {
        Self::new()
    }
}

/// Core trait that all skip tags must implement - unified interface
///
/// This trait defines the contract for skip tag behavior following the Single Responsibility Principle.
/// Each tag has one clear purpose and implements the minimal interface needed.
pub trait SkipTag: fmt::Debug + Send + Sync {
    /// Unique identifier for this tag
    fn tag_id(&self) -> SkipTagId;

    /// Display name for this tag
    fn name(&self) -> &'static str;

    /// What type of effect this tag has
    fn effect_type(&self) -> TagEffectType;

    /// Human-readable description of what this tag does
    fn description(&self) -> &'static str;

    /// Get the rarity (affects skip chance)
    fn rarity(&self) -> TagRarity;

    /// Can this tag be stacked?
    fn stackable(&self) -> bool;

    /// Can this tag be selected (some are automatic)
    fn selectable(&self) -> bool {
        true
    }

    /// Check if this tag can be applied in the current game state
    fn can_apply(&self, _game_state: &Game) -> bool {
        true // Default: most tags can always be applied
    }

    /// Apply the tag's effect to the game state
    ///
    /// Returns the result of applying the effect, including any rewards or messages.
    /// The game state may be modified as a side effect.
    fn apply_effect(&self, game_state: &Game) -> TagEffectResult;

    /// Handle boss blind defeat (for Investment tag)
    ///
    /// Default implementation does nothing. Only tags that need to respond to
    /// boss blind defeats should override this.
    fn on_boss_blind_defeated(&self, _game_state: &Game) -> TagEffectResult {
        TagEffectResult::new()
    }

    /// Check if this tag can be activated in the given context
    fn can_activate(&self, _context: &SkipTagContext) -> bool {
        true
    }
}

/// Tag rarity affects generation probability
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TagRarity {
    Common,
    Uncommon,
    Rare,
    Legendary,
}

impl TagRarity {
    /// Get the base probability weight for this rarity
    pub fn weight(&self) -> f64 {
        match self {
            Self::Common => 1.0,
            Self::Uncommon => 0.6,
            Self::Rare => 0.3,
            Self::Legendary => 0.1,
        }
    }
}

/// Error types for skip tag operations
#[derive(Debug, Clone, PartialEq)]
pub enum TagError {
    /// Tag cannot be applied in current game state
    CannotApply(String),
    /// Invalid tag ID
    InvalidTagId(SkipTagId),
    /// Game state is in invalid condition for tag application
    InvalidGameState(String),
}

impl fmt::Display for TagError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TagError::CannotApply(reason) => write!(f, "Cannot apply tag: {reason}"),
            TagError::InvalidTagId(id) => write!(f, "Invalid tag ID: {id}"),
            TagError::InvalidGameState(reason) => write!(f, "Invalid game state: {reason}"),
        }
    }
}

impl std::error::Error for TagError {}

/// A skip tag instance with possible stacking
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SkipTagInstance {
    pub id: SkipTagId,
    pub stack_count: usize,
}

impl SkipTagInstance {
    pub fn new(id: SkipTagId) -> Self {
        Self { id, stack_count: 1 }
    }

    pub fn with_stack(id: SkipTagId, count: usize) -> Self {
        Self {
            id,
            stack_count: count,
        }
    }

    /// Add to stack if stackable
    pub fn add_stack(&mut self, registry: &tag_registry::SkipTagRegistry) -> bool {
        if let Some(tag) = registry.get_tag(self.id) {
            if tag.stackable() {
                self.stack_count += 1;
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skip_tag_id_display() {
        assert_eq!(SkipTagId::Double.to_string(), "Double");
        assert_eq!(SkipTagId::Boss.to_string(), "Boss");
        assert_eq!(SkipTagId::Orbital.to_string(), "Orbital");
        assert_eq!(SkipTagId::Juggle.to_string(), "Juggle");
        
        // Test shop enhancement tags
        assert_eq!(SkipTagId::Voucher.to_string(), "Voucher");
        assert_eq!(SkipTagId::Coupon.to_string(), "Coupon");
        assert_eq!(SkipTagId::D6.to_string(), "D6");
    }

    #[test]
    fn test_tag_rarity_weights() {
        assert!(TagRarity::Common.weight() > TagRarity::Uncommon.weight());
        assert!(TagRarity::Uncommon.weight() > TagRarity::Rare.weight());
        assert!(TagRarity::Rare.weight() > TagRarity::Legendary.weight());
    }

    #[test]
    fn test_skip_tag_instance_creation() {
        let instance = SkipTagInstance::new(SkipTagId::Double);
        assert_eq!(instance.id, SkipTagId::Double);
        assert_eq!(instance.stack_count, 1);

        let stacked = SkipTagInstance::with_stack(SkipTagId::Juggle, 3);
        assert_eq!(stacked.stack_count, 3);
    }

    #[test]
    fn test_tag_effect_result() {
        let result = TagEffectResult::new();
        assert_eq!(result.money_reward, 0);
        assert!(result.messages.is_empty());
        assert!(!result.persist_tag);

        let result_with_money = TagEffectResult::with_money(100);
        assert_eq!(result_with_money.money_reward, 100);
    }
}
