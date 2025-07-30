//! Skip Tags System
//!
//! Skip tags are special rewards that can be obtained when skipping blinds.
//! They provide various effects to enhance gameplay.

pub mod shop_tags;
pub mod tag_effects;
pub mod tag_registry;

// Temporarily disable utility tags due to interface mismatch
// TODO: Re-enable and fix utility tags in a follow-up
// pub mod utility_tags;

#[cfg(test)]
mod integration_tests;

// Re-export public API
pub use shop_tags::*;
pub use tag_effects::*;
pub use tag_registry::*;

use crate::game::Game;
use crate::stage::Blind;
use std::fmt;

/// Error types for skip tag operations
#[derive(Debug, Clone)]
pub enum TagError {
    /// Tag ID is not registered
    InvalidTagId(SkipTagId),
    /// Tag cannot be applied in current context
    CannotApply(String),
    /// Registry error
    RegistryError(String),
}

impl fmt::Display for TagError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TagError::InvalidTagId(id) => write!(f, "Invalid tag ID: {id}"),
            TagError::CannotApply(msg) => write!(f, "Cannot apply tag: {msg}"),
            TagError::RegistryError(msg) => write!(f, "Registry error: {msg}"),
        }
    }
}

impl std::error::Error for TagError {}

/// Result of tag effect application
#[derive(Debug)]
pub struct TagEffectResult {
    /// Whether the effect was successful
    pub success: bool,
    /// Optional message about the effect
    pub message: Option<String>,
    /// Any additional data from the effect
    pub data: TagEffectData,
    /// Amount of money to award
    pub money_reward: i64,
    /// Whether to persist this tag for future effects
    pub persist_tag: bool,
}

/// Additional data from tag effects
#[derive(Debug)]
pub enum TagEffectData {
    /// No additional data
    None,
    /// Money gained
    Money(f64),
    /// Shop modifier applied
    ShopModifier,
    /// Other generic effect
    Other(String),
}

/// Modifiers for the next shop
#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NextShopModifiers {
    /// Number of additional vouchers
    pub additional_vouchers: u32,
    /// Whether initial items should be free
    pub coupon_active: bool,
    /// Whether rerolls should be free
    pub free_rerolls: bool,
    /// Foil tag is active
    pub foil_tag_active: bool,
    /// Holographic tag is active
    pub holographic_tag_active: bool,
    /// Polychrome tag is active
    pub polychrome_tag_active: bool,
}

/// Active skip tags management system
#[derive(Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ActiveSkipTags {
    /// Number of blinds skipped (for economic tags)
    pub blinds_skipped: u32,
    /// Number of investment tags collected (for Investment tag payout)
    pub investment_count: u32,
    /// Next shop modifiers pending
    pub next_shop_modifiers: NextShopModifiers,
    /// Active tag instances
    pub active_instances: Vec<SkipTagInstance>,
}

impl ActiveSkipTags {
    /// Create a new active skip tags manager
    pub fn new() -> Self {
        Self::default()
    }

    /// Consume and return next shop modifiers
    pub fn consume_next_shop_modifiers(&mut self) -> NextShopModifiers {
        std::mem::take(&mut self.next_shop_modifiers)
    }

    /// Add a shop modifier
    pub fn add_shop_modifier(&mut self, modifier: NextShopModifiers) {
        // Combine modifiers
        self.next_shop_modifiers.additional_vouchers += modifier.additional_vouchers;
        self.next_shop_modifiers.coupon_active |= modifier.coupon_active;
        self.next_shop_modifiers.free_rerolls |= modifier.free_rerolls;
        self.next_shop_modifiers.foil_tag_active |= modifier.foil_tag_active;
        self.next_shop_modifiers.holographic_tag_active |= modifier.holographic_tag_active;
        self.next_shop_modifiers.polychrome_tag_active |= modifier.polychrome_tag_active;
    }

    /// Add a tag instance
    pub fn push(&mut self, instance: SkipTagInstance) {
        self.active_instances.push(instance);
    }

    /// Get mutable iterator over tag instances
    pub fn iter_mut(&mut self) -> std::slice::IterMut<SkipTagInstance> {
        self.active_instances.iter_mut()
    }

    /// Get iterator over tag instances
    pub fn iter(&self) -> std::slice::Iter<SkipTagInstance> {
        self.active_instances.iter()
    }

    /// Check if there are no active tags
    pub fn is_empty(&self) -> bool {
        self.active_instances.is_empty()
    }

    /// Apply shop enhancement effect for the given tag
    pub fn apply_shop_enhancement_effect(&mut self, tag_id: SkipTagId) {
        let mut modifier = NextShopModifiers::default();

        match tag_id {
            SkipTagId::Voucher => {
                modifier.additional_vouchers += 1;
            }
            SkipTagId::Coupon => {
                modifier.coupon_active = true;
            }
            SkipTagId::D6 => {
                modifier.free_rerolls = true;
            }
            SkipTagId::Foil => {
                modifier.foil_tag_active = true;
            }
            SkipTagId::Holographic => {
                modifier.holographic_tag_active = true;
            }
            SkipTagId::Polychrome => {
                modifier.polychrome_tag_active = true;
            }
            _ => {
                // Other tags don't have shop enhancement effects
                return;
            }
        }

        self.add_shop_modifier(modifier);
    }
}

/// Get the global registry (convenience function)
pub fn get_registry() -> &'static tag_registry::SkipTagRegistry {
    tag_registry::global_registry()
}

/// Categories of tag effects
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

/// All available skip tag IDs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "python", pyo3::pyclass(eq, eq_int))]
pub enum SkipTagId {
    // Economic Tags (Phase 2 - Issue #693)
    Economy,
    Investment,
    Garbage,
    Speed,
    Handy,

    // Shop Enhancement Tags (Phase 2 - Issue #694) - Currently implemented
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

    // Utility Tags (Phase 3) - Temporarily disabled
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

/// Context for skip tag activation
#[derive(Debug)]
pub struct SkipTagContext {
    /// The game state when tag is activated
    pub game: Game,
    /// The blind that was skipped (if applicable)
    pub skipped_blind: Option<Blind>,
    /// Additional tags available for duplication (for Double tag)
    pub available_tags: Vec<SkipTagId>,
}

/// Result of skip tag activation
#[derive(Debug)]
pub struct SkipTagResult {
    /// Updated game state
    pub game: Game,
    /// Additional tags created (for Double tag)
    pub additional_tags: Vec<SkipTagId>,
    /// Success status
    pub success: bool,
    /// Optional message for UI
    pub message: Option<String>,
}

/// Core trait for all skip tags
pub trait SkipTag: fmt::Debug + Send + Sync {
    /// Get the tag ID
    fn id(&self) -> SkipTagId;

    /// Get the display name
    fn name(&self) -> &'static str;

    /// Get the description
    fn description(&self) -> &'static str;

    /// Get the effect type
    fn effect_type(&self) -> TagEffectType;

    /// Get the rarity (affects skip chance)
    fn rarity(&self) -> TagRarity;

    /// Can this tag be stacked?
    fn stackable(&self) -> bool;

    /// Can this tag be selected (some are automatic)
    fn selectable(&self) -> bool {
        true
    }

    /// Activate the skip tag effect
    fn activate(&self, context: SkipTagContext) -> SkipTagResult;

    /// Check if this tag can be activated in the given context
    fn can_activate(&self, _context: &SkipTagContext) -> bool {
        true
    }

    /// Check if this tag can be applied to the given game state
    fn can_apply(&self, _game: &Game) -> bool {
        true
    }

    /// Apply the tag effect to the game state
    fn apply_effect(&self, _game: &mut Game) -> TagEffectResult {
        // Default implementation for shop enhancement tags
        // Since shop enhancement tags don't have immediate money effects,
        // we just return a success result and let the shop modifier be applied
        TagEffectResult {
            success: true,
            message: Some(format!("{} effect applied", self.name())),
            data: TagEffectData::None,
            money_reward: 0,
            persist_tag: matches!(self.effect_type(), TagEffectType::NextShopModifier),
        }
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
}
