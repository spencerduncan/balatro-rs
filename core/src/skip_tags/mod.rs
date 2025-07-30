//! Skip Tags System
//!
//! Skip tags are special rewards that can be obtained when skipping blinds.
//! They provide various effects to enhance gameplay.

pub mod tag_effects;
pub mod tag_registry;
pub mod utility_tags;

#[cfg(test)]
mod integration_tests;

use crate::game::Game;
use crate::stage::Blind;
use std::fmt;

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
    // Utility Tags (Phase 3)
    Double,
    Boss,
    Orbital,
    Juggle,
}

impl fmt::Display for SkipTagId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Double => write!(f, "Double"),
            Self::Boss => write!(f, "Boss"),
            Self::Orbital => write!(f, "Orbital"),
            Self::Juggle => write!(f, "Juggle"),
        }
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
