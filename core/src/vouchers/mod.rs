//! Vouchers module for Balatro game engine
//!
//! This module provides the infrastructure for voucher cards in Balatro.
//! Vouchers are permanent upgrades that can be purchased in the shop.
//!
//! # Module Organization
//!
//! - `mod.rs` - Core types and traits for vouchers
//! - `implementations.rs` - Specific voucher implementations (future)
//!
//! # Design Principles
//!
//! - Vouchers provide permanent effects that persist across rounds
//! - Each voucher can only be purchased once per run
//! - Vouchers may have prerequisites (other vouchers that must be owned first)
//! - Effects are applied passively to game state

use serde::{Deserialize, Serialize};
use std::fmt;
use strum::{EnumIter, IntoEnumIterator};

/// Categorization of voucher effects for game system integration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum VoucherEffect {
    /// Increases hand size by the specified amount
    HandSizeIncrease(usize),
    /// Increases joker slots by the specified amount
    JokerSlotIncrease(usize),
    /// Provides money gain (immediate or per-round)
    MoneyGain(usize),
    /// Modifies ante scaling (multiplier)
    AnteScaling(f64),
    /// Adds extra pack options in shop
    ExtraPackOptions(usize),
    /// Reduces blind score requirements (multiplier)
    BlindScoreReduction(f64),
    /// Adds starting cards to deck
    StartingCards(Vec<crate::card::Card>),
    /// Increases shop slots
    ShopSlotIncrease(usize),
    /// Increases discards per round
    DiscardIncrease(usize),
    /// Increases plays per round
    PlayIncrease(usize),
}

impl VoucherEffect {
    /// Check if this effect is permanent (applies for entire run)
    pub fn is_permanent(&self) -> bool {
        match self {
            VoucherEffect::MoneyGain(_) => false, // One-time effect
            _ => true, // Most effects are permanent
        }
    }

    /// Check if this effect affects shop mechanics
    pub fn affects_shop(&self) -> bool {
        matches!(self, 
            VoucherEffect::ExtraPackOptions(_) | 
            VoucherEffect::ShopSlotIncrease(_) |
            VoucherEffect::JokerSlotIncrease(_)
        )
    }

    /// Check if this effect affects money systems
    pub fn affects_money(&self) -> bool {
        matches!(self, VoucherEffect::MoneyGain(_))
    }

    /// Check if this effect affects hand mechanics
    pub fn affects_hand(&self) -> bool {
        matches!(self, 
            VoucherEffect::HandSizeIncrease(_) |
            VoucherEffect::DiscardIncrease(_) |
            VoucherEffect::PlayIncrease(_)
        )
    }

    /// Check if this effect has a numeric value
    pub fn has_numeric_value(&self) -> bool {
        match self {
            VoucherEffect::StartingCards(_) => false,
            _ => true,
        }
    }

    /// Get hand size bonus if applicable
    pub fn hand_size_bonus(&self) -> Option<usize> {
        match self {
            VoucherEffect::HandSizeIncrease(amount) => Some(*amount),
            _ => None,
        }
    }

    /// Get joker slot bonus if applicable
    pub fn joker_slot_bonus(&self) -> Option<usize> {
        match self {
            VoucherEffect::JokerSlotIncrease(amount) => Some(*amount),
            _ => None,
        }
    }

    /// Get money bonus if applicable
    pub fn money_bonus(&self) -> Option<usize> {
        match self {
            VoucherEffect::MoneyGain(amount) => Some(*amount),
            _ => None,
        }
    }
}

/// Tier classification for voucher upgrade paths
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum VoucherTier {
    /// Base tier voucher
    Base,
    /// Upgraded tier voucher (enhanced version)
    Upgraded,
}

impl VoucherTier {
    /// Check if this is a base tier voucher
    pub fn is_base(&self) -> bool {
        matches!(self, VoucherTier::Base)
    }

    /// Check if this is an upgraded tier voucher
    pub fn is_upgraded(&self) -> bool {
        matches!(self, VoucherTier::Upgraded)
    }

    /// Get the upgraded version of this tier, if available
    pub fn upgrade(&self) -> Option<VoucherTier> {
        match self {
            VoucherTier::Base => Some(VoucherTier::Upgraded),
            VoucherTier::Upgraded => None,
        }
    }
}

/// Rules for how voucher effects can stack
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StackingRule {
    /// Cannot stack - only one instance allowed
    NoStacking,
    /// Can stack without limit
    UnlimitedStacking,
    /// Can stack up to specified limit
    LimitedStacking(usize),
}

impl StackingRule {
    /// Check if this rule allows stacking
    pub fn allows_stacking(&self) -> bool {
        !matches!(self, StackingRule::NoStacking)
    }

    /// Get maximum stack size if limited
    pub fn max_stack_size(&self) -> Option<usize> {
        match self {
            StackingRule::NoStacking => Some(1),
            StackingRule::UnlimitedStacking => None,
            StackingRule::LimitedStacking(limit) => Some(*limit),
        }
    }

    /// Check if this stacking rule is compatible with another
    pub fn is_compatible_with(&self, other: &StackingRule) -> bool {
        self == other
    }
}

/// Simplified game state interface for voucher operations
/// This provides the minimal interface vouchers need without full Game dependency
#[derive(Debug, Clone)]
pub struct GameState {
    pub money: usize,
    pub ante: usize,
    pub hand_size: usize,
    pub joker_slots: usize,
    pub vouchers_owned: std::collections::HashSet<VoucherId>,
}

impl GameState {
    /// Create a minimal game state for testing
    pub fn new() -> Self {
        Self {
            money: 100,
            ante: 1,
            hand_size: 8,
            joker_slots: 5,
            vouchers_owned: std::collections::HashSet::new(),
        }
    }
}

impl From<&crate::game::Game> for GameState {
    fn from(game: &crate::game::Game) -> Self {
        // Convert Ante enum to usize
        let ante_value = match game.ante_current {
            crate::ante::Ante::Zero => 0,
            crate::ante::Ante::One => 1,
            crate::ante::Ante::Two => 2,
            crate::ante::Ante::Three => 3,
            crate::ante::Ante::Four => 4,
            crate::ante::Ante::Five => 5,
            crate::ante::Ante::Six => 6,
            crate::ante::Ante::Seven => 7,
            crate::ante::Ante::Eight => 8,
        };
        
        Self {
            money: game.money,
            ante: ante_value,
            hand_size: 8, // Base hand size, vouchers would modify this
            joker_slots: game.config.joker_slots,
            vouchers_owned: game.vouchers.owned_vouchers().into_iter().collect(),
        }
    }
}

/// Core trait that all voucher types must implement
/// Updated to support Issue #16 requirements
pub trait Voucher: Send + Sync + std::fmt::Debug {
    /// Get the unique identifier for this voucher
    fn id(&self) -> VoucherId;

    /// Get the tier (base or upgraded) of this voucher
    fn tier(&self) -> VoucherTier;

    /// Get the single prerequisite voucher (if any)
    /// None if no prerequisite required
    fn prerequisite(&self) -> Option<VoucherId>;

    /// Check if this voucher can be purchased given the current game state
    fn can_purchase(&self, game_state: &GameState) -> bool;

    /// Apply the effect of this voucher to the game state
    fn apply_effect(&self, game_state: &mut GameState);

    /// Get all effects this voucher provides
    fn get_effects(&self) -> Vec<VoucherEffect>;

    /// Get the stacking rules for this voucher
    fn stacking_rule(&self) -> StackingRule {
        StackingRule::NoStacking // Default: vouchers don't stack
    }

    /// Get the name of this voucher (optional, for display)
    fn name(&self) -> &'static str {
        "Unnamed Voucher"
    }

    /// Get the description of this voucher (optional, for display)
    fn description(&self) -> &'static str {
        "No description available"
    }

    /// Get the cost of this voucher (optional, can use VoucherId default)
    fn cost(&self) -> usize {
        self.id().base_cost()
    }
}

/// Identifier for all voucher cards in the game
/// This will be extended as voucher implementations are added
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumIter)]
pub enum VoucherId {
    /// Grab Bag voucher - +1 pack option for all booster packs
    GrabBag,
    /// Placeholder for future voucher implementations
    VoucherPlaceholder,
}

impl fmt::Display for VoucherId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VoucherId::GrabBag => write!(f, "Grab Bag"),
            VoucherId::VoucherPlaceholder => write!(f, "Voucher Placeholder"),
        }
    }
}

impl VoucherId {
    /// Get all available voucher IDs
    pub fn all() -> Vec<VoucherId> {
        Self::iter().collect()
    }

    /// Check if this voucher has any prerequisites
    pub fn has_prerequisites(&self) -> bool {
        !self.prerequisites().is_empty()
    }

    /// Get the prerequisite vouchers for this voucher
    pub fn prerequisites(&self) -> Vec<VoucherId> {
        match self {
            VoucherId::GrabBag => vec![], // No prerequisites
            VoucherId::VoucherPlaceholder => vec![],
        }
    }

    /// Get the base cost of this voucher
    pub fn base_cost(&self) -> usize {
        match self {
            VoucherId::GrabBag => 10, // Reasonable cost for +1 pack option
            VoucherId::VoucherPlaceholder => 10,
        }
    }
}

/// Set of vouchers owned by the player
/// Provides efficient lookup and management of owned vouchers
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VoucherCollection {
    owned: std::collections::HashSet<VoucherId>,
}

impl VoucherCollection {
    /// Create a new empty voucher collection
    pub fn new() -> Self {
        Self {
            owned: std::collections::HashSet::new(),
        }
    }

    /// Add a voucher to the collection
    pub fn add(&mut self, voucher: VoucherId) {
        self.owned.insert(voucher);
    }

    /// Check if a voucher is owned
    pub fn owns(&self, voucher: VoucherId) -> bool {
        self.owned.contains(&voucher)
    }

    /// Get all owned vouchers
    pub fn owned_vouchers(&self) -> Vec<VoucherId> {
        self.owned.iter().copied().collect()
    }

    /// Check if all prerequisites for a voucher are met
    pub fn can_purchase(&self, voucher: VoucherId) -> bool {
        if self.owns(voucher) {
            return false; // Already owned
        }

        voucher
            .prerequisites()
            .iter()
            .all(|&prereq| self.owns(prereq))
    }

    /// Get the number of vouchers owned
    pub fn count(&self) -> usize {
        self.owned.len()
    }
}

// Re-export commonly used types
pub use VoucherId::*;
