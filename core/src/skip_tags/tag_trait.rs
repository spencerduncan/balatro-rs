use crate::game::Game;
use crate::skip_tags::tag_error::TagError;
use std::fmt;

#[cfg(feature = "python")]
use pyo3::pyclass;

/// Core trait for all skip tags
/// 
/// Skip tags provide strategic rewards when players skip blinds instead of playing them.
/// Each tag has a unique ID, effect type, and can conditionally apply effects to the game state.
pub trait SkipTag: Send + Sync + std::fmt::Debug {
    /// Unique identifier for this tag
    fn id(&self) -> TagId;
    
    /// Human-readable name for this tag
    fn name(&self) -> &'static str;
    
    /// Classification of this tag's effect type
    fn effect_type(&self) -> TagEffectType;
    
    /// Check if this tag can be applied to the current game state
    /// 
    /// This method performs validation to ensure the tag effect makes sense
    /// in the current context. For example, pack generation tags require
    /// the player to be able to receive packs.
    fn can_apply(&self, game_state: &Game) -> bool;
    
    /// Apply this tag's effect to the game state
    /// 
    /// This method modifies the game state according to the tag's effect.
    /// It should be idempotent when possible and handle edge cases gracefully.
    /// 
    /// # Performance Requirements
    /// - Must complete in <100ms for any tag effect
    /// - Should not allocate more than 1KB of memory
    fn apply_effect(&self, game_state: &mut Game) -> Result<(), TagError>;
    
    /// Description of what this tag does
    fn description(&self) -> &'static str;
    
    /// Priority for tag selection when multiple tags are available
    /// Higher values are more likely to be selected
    /// Default implementation returns 1.0 (neutral priority)
    fn selection_priority(&self) -> f32 {
        1.0
    }
    
    /// Check if this tag should be available based on game statistics
    /// Default implementation allows all tags
    fn availability_condition(&self, game_state: &Game) -> bool {
        let _ = game_state; // Prevent unused parameter warning
        true
    }
}

/// Unique identifiers for all skip tags
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "python", pyclass(eq))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TagId {
    // Reward Tags (8) - Generate packs or immediate rewards
    Charm,      // Creates 2 playing cards
    Ethereal,   // Gives a spectral pack
    Buffoon,    // Gives a joker pack
    Standard,   // Gives a standard pack
    Meteor,     // Gives a planet pack
    Rare,       // Gives a rare joker pack
    Uncommon,   // Gives an uncommon joker pack
    TopUp,      // Gives the most common consumable pack
    
    // Economic Tags (5) - Provide money or interest bonuses
    Economy,    // Earns interest on current money
    Investment, // Earns $8
    Garbage,    // Earns $1 for each unused hand
    Speed,      // Earns $1 for each unused discard
    Handy,      // Earns $1 for each unused hand or discard
    
    // Shop Enhancement Tags (6) - Modify next shop visit
    Voucher,    // Adds a voucher to the shop
    Coupon,     // Offers a free reroll
    Foil,       // Gives a random foil joker
    Holographic, // Gives a random holographic joker
    Polychrome, // Gives a random polychrome joker
    Negative,   // Gives a random negative joker
    
    // Utility Tags (4) - Special mechanics and game state modifiers
    Double,     // Creates a copy of the tag (stacking effect)
    Boss,       // Rerolls current boss blind
    Orbital,    // Upgrades two random poker hands by 1 level
    Juggle,     // Ability to remove up to 5 selected cards from deck
    
    // Compatibility Tags (3) - For future expansion
    D6,         // Creates multiple random consumables (dice roll)
}

impl TagId {
    /// Get all tag IDs in a deterministic order for iteration
    pub fn all() -> &'static [TagId] {
        &[
            // Reward Tags
            TagId::Charm, TagId::Ethereal, TagId::Buffoon, TagId::Standard,
            TagId::Meteor, TagId::Rare, TagId::Uncommon, TagId::TopUp,
            
            // Economic Tags  
            TagId::Economy, TagId::Investment, TagId::Garbage, TagId::Speed, TagId::Handy,
            
            // Shop Enhancement Tags
            TagId::Voucher, TagId::Coupon, TagId::Foil, TagId::Holographic, 
            TagId::Polychrome, TagId::Negative,
            
            // Utility Tags
            TagId::Double, TagId::Boss, TagId::Orbital, TagId::Juggle,
            
            // Compatibility Tags
            TagId::D6,
        ]
    }
    
    /// Get the category this tag belongs to
    pub fn category(&self) -> TagCategory {
        match self {
            TagId::Charm | TagId::Ethereal | TagId::Buffoon | TagId::Standard
            | TagId::Meteor | TagId::Rare | TagId::Uncommon | TagId::TopUp => TagCategory::Reward,
            
            TagId::Economy | TagId::Investment | TagId::Garbage 
            | TagId::Speed | TagId::Handy => TagCategory::Economic,
            
            TagId::Voucher | TagId::Coupon | TagId::Foil | TagId::Holographic 
            | TagId::Polychrome | TagId::Negative => TagCategory::ShopEnhancement,
            
            TagId::Double | TagId::Boss | TagId::Orbital | TagId::Juggle => TagCategory::Utility,
            
            TagId::D6 => TagCategory::Compatibility,
        }
    }
}

impl fmt::Display for TagId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TagId::Charm => write!(f, "Charm"),
            TagId::Ethereal => write!(f, "Ethereal"),
            TagId::Buffoon => write!(f, "Buffoon"),
            TagId::Standard => write!(f, "Standard"),
            TagId::Meteor => write!(f, "Meteor"),
            TagId::Rare => write!(f, "Rare"),
            TagId::Uncommon => write!(f, "Uncommon"),
            TagId::TopUp => write!(f, "Top Up"),
            TagId::Economy => write!(f, "Economy"),
            TagId::Investment => write!(f, "Investment"),
            TagId::Garbage => write!(f, "Garbage"),
            TagId::Speed => write!(f, "Speed"),
            TagId::Handy => write!(f, "Handy"),
            TagId::Voucher => write!(f, "Voucher"),
            TagId::Coupon => write!(f, "Coupon"),
            TagId::Foil => write!(f, "Foil"),
            TagId::Holographic => write!(f, "Holographic"),
            TagId::Polychrome => write!(f, "Polychrome"),
            TagId::Negative => write!(f, "Negative"),
            TagId::Double => write!(f, "Double"),
            TagId::Boss => write!(f, "Boss"),
            TagId::Orbital => write!(f, "Orbital"),
            TagId::Juggle => write!(f, "Juggle"),
            TagId::D6 => write!(f, "D6"),
        }
    }
}

/// Categories for organizing skip tags
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TagCategory {
    Reward,           // Generate packs or immediate rewards
    Economic,         // Provide money or interest bonuses  
    ShopEnhancement,  // Modify next shop visit
    Utility,          // Special mechanics and game state modifiers
    Compatibility,    // For future expansion and compatibility
}

/// Classification of tag effect types for processing optimization
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TagEffectType {
    /// Immediate reward effects (money, packs, cards)
    /// Applied instantly when tag is selected
    ImmediateReward,
    
    /// Shop modifier effects (vouchers, editions, rerolls)  
    /// Applied during next shop generation/interaction
    NextShopModifier,
    
    /// Game state modifier effects (hand upgrades, deck changes)
    /// Applied to persistent game state
    GameStateModifier,
    
    /// Special mechanic effects (tag duplication, boss rerolls)
    /// Complex effects with custom application timing
    SpecialMechanic,
}

impl TagEffectType {
    /// Check if this effect type requires immediate application
    pub fn is_immediate(&self) -> bool {
        matches!(self, TagEffectType::ImmediateReward | TagEffectType::GameStateModifier)
    }
    
    /// Check if this effect type modifies future game interactions
    pub fn is_deferred(&self) -> bool {
        matches!(self, TagEffectType::NextShopModifier | TagEffectType::SpecialMechanic)
    }
}

/// Stub implementation for testing and development
/// This allows the tag system to compile and be tested before all tags are implemented
#[derive(Debug)]
pub struct StubTag {
    id: TagId,
    name: &'static str,
    effect_type: TagEffectType,
    description: &'static str,
}

impl StubTag {
    pub fn new(id: TagId, name: &'static str, effect_type: TagEffectType, description: &'static str) -> Self {
        Self { id, name, effect_type, description }
    }
}

impl SkipTag for StubTag {
    fn id(&self) -> TagId {
        self.id
    }
    
    fn name(&self) -> &'static str {
        self.name
    }
    
    fn effect_type(&self) -> TagEffectType {
        self.effect_type
    }
    
    fn can_apply(&self, _game_state: &Game) -> bool {
        true // Stub always allows application
    }
    
    fn apply_effect(&self, _game_state: &mut Game) -> Result<(), TagError> {
        // Stub does nothing but succeeds
        Ok(())
    }
    
    fn description(&self) -> &'static str {
        self.description
    }
}