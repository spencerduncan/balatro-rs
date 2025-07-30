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

#[cfg(feature = "python")]
use pyo3;
use serde::{Deserialize, Serialize};
use std::fmt;
use strum::{EnumIter, IntoEnumIterator};
use thiserror::Error;

// Module for individual voucher implementations
pub mod implementations;

/// Errors that can occur during voucher operations
#[derive(Error, Debug, Clone)]
pub enum VoucherError {
    #[error("Hand size increase too large: {amount} (max: 50)")]
    ExcessiveHandSize { amount: usize },
    #[error("Money gain too large: {amount} (max: 10000)")]
    ExcessiveMoneyGain { amount: usize },
    #[error("Invalid ante scaling: {multiplier} (must be finite, positive, and ≤ 10.0)")]
    InvalidScaling { multiplier: f64 },
    #[error("Too many pack options: {amount} (max: 10)")]
    ExcessivePackOptions { amount: usize },
    #[error("Invalid blind score reduction: {multiplier} (must be finite, positive, and ≤ 1.0)")]
    InvalidBlindReduction { multiplier: f64 },
    #[error("Too many starting cards: {count} (max: 52)")]
    ExcessiveStartingCards { count: usize },
    #[error("Shop slot increase too large: {amount} (max: 20)")]
    ExcessiveShopSlots { amount: usize },
    #[error("Discard increase too large: {amount} (max: 50)")]
    ExcessiveDiscards { amount: usize },
    #[error("Play increase too large: {amount} (max: 50)")]
    ExcessivePlays { amount: usize },
    #[error("Joker slot increase too large: {amount} (max: 20)")]
    ExcessiveJokerSlots { amount: usize },
}

/// Errors that can occur during game state operations
#[derive(Error, Debug, Clone)]
pub enum GameStateError {
    #[error("Voucher validation failed")]
    VoucherValidation(#[from] VoucherError),
    #[error("Invalid game state: {reason}")]
    InvalidState { reason: String },
}

/// Categorization of voucher effects for game system integration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum VoucherEffect {
    /// Increases hand size by the specified amount
    HandSizeIncrease(usize),
    /// Decreases hand size by the specified amount (for negative effects)
    HandSizeDecrease(usize),
    /// Increases joker slots by the specified amount
    JokerSlotIncrease(usize),
    /// Decreases joker slots by the specified amount (for negative effects)
    JokerSlotDecrease(usize),
    /// Provides money gain (immediate or per-round)
    MoneyGain(usize),
    /// Increases the interest cap by the specified amount
    InterestCapIncrease(usize),
    /// Modifies ante scaling (multiplier)
    AnteScaling(f64),
    /// Increases ante required to win by the specified amount
    AnteWinRequirementIncrease(usize),
    /// Decreases ante required to win by the specified amount
    AnteWinRequirementDecrease(usize),
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
    /// Decreases discards per round (for negative effects)
    DiscardDecrease(usize),
    /// Increases plays per round
    PlayIncrease(usize),
    /// Enables playing cards to be purchased from shop
    ShopPlayingCardsEnabled,
    /// Enables playing cards in shop to have enhancements
    ShopEnhancementsEnabled,
    /// Multiplies Tarot card appearance frequency
    TarotFrequencyMultiplier(f64),
    /// Multiplies Planet card appearance frequency
    PlanetFrequencyMultiplier(f64),
    /// Multiplies enhanced card (foil/holo/polychrome) appearance frequency
    PolychromeFrequencyMultiplier(f64),
    /// Provides percentage discount on all shop items
    ShopDiscountPercent(f64),
    /// Applies discount multiplier to shop items (0.5 = 50% off)
    ShopDiscountMultiplier(f64),
    /// Reduces reroll cost by specified amount
    RerollCostReduction(usize),
    /// Increases consumable slots
    ConsumableSlotIncrease(usize),
    /// Enables boss blind reroll functionality (limited or unlimited)
    BossBlindRerollEnabled {
        unlimited: bool,
        cost_per_roll: usize,
    },
    /// No effect (flavor voucher)
    NoEffect,
}

impl VoucherEffect {
    /// Check if this effect is permanent (applies for entire run)
    pub fn is_permanent(&self) -> bool {
        match self {
            VoucherEffect::MoneyGain(_) => false, // One-time effect
            _ => true,                            // Most effects are permanent
        }
    }

    /// Check if this effect affects shop mechanics
    /// Check if this effect affects shop mechanics
    pub fn affects_shop(&self) -> bool {
        matches!(
            self,
            VoucherEffect::ExtraPackOptions(_)
                | VoucherEffect::ShopSlotIncrease(_)
                | VoucherEffect::JokerSlotIncrease(_)
                | VoucherEffect::JokerSlotDecrease(_)
                | VoucherEffect::ShopPlayingCardsEnabled
                | VoucherEffect::ShopEnhancementsEnabled
                | VoucherEffect::TarotFrequencyMultiplier(_)
                | VoucherEffect::PlanetFrequencyMultiplier(_)
                | VoucherEffect::PolychromeFrequencyMultiplier(_)
                | VoucherEffect::ShopDiscountPercent(_)
                | VoucherEffect::ShopDiscountMultiplier(_)
                | VoucherEffect::RerollCostReduction(_)
                | VoucherEffect::ConsumableSlotIncrease(_)
                | VoucherEffect::BossBlindRerollEnabled { .. }
        )
    }

    /// Check if this effect affects money systems
    pub fn affects_money(&self) -> bool {
        matches!(self, VoucherEffect::MoneyGain(_))
    }

    /// Check if this effect affects hand mechanics
    pub fn affects_hand(&self) -> bool {
        matches!(
            self,
            VoucherEffect::HandSizeIncrease(_)
                | VoucherEffect::DiscardIncrease(_)
                | VoucherEffect::PlayIncrease(_)
        )
    }

    /// Check if this effect has a numeric value
    pub fn has_numeric_value(&self) -> bool {
        !matches!(self, VoucherEffect::StartingCards(_))
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

    /// Validate that the effect has reasonable bounds
    pub fn validate(&self) -> Result<(), VoucherError> {
        match self {
            VoucherEffect::HandSizeIncrease(amount) => {
                if *amount > 50 {
                    return Err(VoucherError::ExcessiveHandSize { amount: *amount });
                }
            }
            VoucherEffect::JokerSlotIncrease(amount) => {
                if *amount > 20 {
                    return Err(VoucherError::ExcessiveJokerSlots { amount: *amount });
                }
            }
            VoucherEffect::MoneyGain(amount) => {
                if *amount > 10000 {
                    return Err(VoucherError::ExcessiveMoneyGain { amount: *amount });
                }
            }
            VoucherEffect::AnteScaling(multiplier) => {
                if !multiplier.is_finite() || *multiplier <= 0.0 || *multiplier > 10.0 {
                    return Err(VoucherError::InvalidScaling {
                        multiplier: *multiplier,
                    });
                }
            }
            VoucherEffect::ExtraPackOptions(amount) => {
                if *amount > 10 {
                    return Err(VoucherError::ExcessivePackOptions { amount: *amount });
                }
            }
            VoucherEffect::BlindScoreReduction(multiplier) => {
                if !multiplier.is_finite() || *multiplier <= 0.0 || *multiplier > 1.0 {
                    return Err(VoucherError::InvalidBlindReduction {
                        multiplier: *multiplier,
                    });
                }
            }
            VoucherEffect::StartingCards(cards) => {
                if cards.len() > 52 {
                    return Err(VoucherError::ExcessiveStartingCards { count: cards.len() });
                }
            }
            VoucherEffect::ShopSlotIncrease(amount) => {
                if *amount > 20 {
                    return Err(VoucherError::ExcessiveShopSlots { amount: *amount });
                }
            }
            VoucherEffect::DiscardIncrease(amount) => {
                if *amount > 50 {
                    return Err(VoucherError::ExcessiveDiscards { amount: *amount });
                }
            }
            VoucherEffect::PlayIncrease(amount) => {
                if *amount > 50 {
                    return Err(VoucherError::ExcessivePlays { amount: *amount });
                }
            }
            VoucherEffect::HandSizeDecrease(amount) => {
                if *amount > 50 {
                    return Err(VoucherError::ExcessiveHandSize { amount: *amount });
                }
            }
            VoucherEffect::JokerSlotDecrease(amount) => {
                if *amount > 20 {
                    return Err(VoucherError::ExcessiveJokerSlots { amount: *amount });
                }
            }
            VoucherEffect::InterestCapIncrease(amount) => {
                if *amount > 10 {
                    return Err(VoucherError::ExcessiveMoneyGain { amount: *amount });
                }
            }
            VoucherEffect::AnteWinRequirementIncrease(amount) => {
                if *amount > 8 {
                    return Err(VoucherError::ExcessiveHandSize { amount: *amount });
                    // Reuse error type
                }
            }
            VoucherEffect::AnteWinRequirementDecrease(amount) => {
                if *amount > 8 {
                    return Err(VoucherError::ExcessiveHandSize { amount: *amount });
                    // Reuse error type - same bounds as increase
                }
            }
            VoucherEffect::DiscardDecrease(amount) => {
                if *amount > 50 {
                    return Err(VoucherError::ExcessiveDiscards { amount: *amount });
                }
            }
            VoucherEffect::TarotFrequencyMultiplier(multiplier) => {
                if !multiplier.is_finite() || *multiplier <= 0.0 || *multiplier > 10.0 {
                    return Err(VoucherError::InvalidScaling {
                        multiplier: *multiplier,
                    });
                }
            }
            VoucherEffect::PlanetFrequencyMultiplier(multiplier) => {
                if !multiplier.is_finite() || *multiplier <= 0.0 || *multiplier > 10.0 {
                    return Err(VoucherError::InvalidScaling {
                        multiplier: *multiplier,
                    });
                }
            }
            VoucherEffect::PolychromeFrequencyMultiplier(multiplier) => {
                if !multiplier.is_finite() || *multiplier <= 0.0 || *multiplier > 10.0 {
                    return Err(VoucherError::InvalidScaling {
                        multiplier: *multiplier,
                    });
                }
            }
            VoucherEffect::ShopDiscountPercent(discount) => {
                if !discount.is_finite() || *discount <= 0.0 || *discount > 100.0 {
                    return Err(VoucherError::InvalidScaling {
                        multiplier: *discount,
                    });
                }
            }
            VoucherEffect::ShopDiscountMultiplier(multiplier) => {
                if !multiplier.is_finite() || *multiplier <= 0.0 || *multiplier > 1.0 {
                    return Err(VoucherError::InvalidBlindReduction {
                        multiplier: *multiplier,
                    });
                }
            }
            VoucherEffect::RerollCostReduction(amount) => {
                if *amount > 10 {
                    return Err(VoucherError::ExcessiveMoneyGain { amount: *amount });
                }
            }
            VoucherEffect::ConsumableSlotIncrease(amount) => {
                if *amount > 10 {
                    return Err(VoucherError::ExcessiveJokerSlots { amount: *amount });
                }
            }
            VoucherEffect::BossBlindRerollEnabled { cost_per_roll, .. } => {
                if *cost_per_roll > 100 {
                    return Err(VoucherError::ExcessiveMoneyGain {
                        amount: *cost_per_roll,
                    });
                }
            }
            VoucherEffect::ShopPlayingCardsEnabled => {}
            VoucherEffect::ShopEnhancementsEnabled => {}
            VoucherEffect::NoEffect => {}
        }
        Ok(())
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
    money: usize,
    ante: usize,
    hand_size: usize,
    joker_slots: usize,
    vouchers_owned: std::collections::HashSet<VoucherId>,
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
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

    /// Get current money amount
    pub fn money(&self) -> usize {
        self.money
    }

    /// Get current ante level
    pub fn ante(&self) -> usize {
        self.ante
    }

    /// Get current hand size
    pub fn hand_size(&self) -> usize {
        self.hand_size
    }

    /// Get current joker slots
    pub fn joker_slots(&self) -> usize {
        self.joker_slots
    }

    /// Get reference to owned vouchers
    pub fn vouchers_owned(&self) -> &std::collections::HashSet<VoucherId> {
        &self.vouchers_owned
    }

    /// Check if player can afford a cost
    pub fn can_afford(&self, cost: usize) -> bool {
        self.money >= cost
    }

    /// Check if a voucher is owned
    pub fn owns_voucher(&self, voucher_id: VoucherId) -> bool {
        self.vouchers_owned.contains(&voucher_id)
    }

    /// Apply a voucher effect to the game state with validation and safety checks
    pub fn apply_voucher_effect(&mut self, effect: &VoucherEffect) -> Result<(), GameStateError> {
        // First validate the effect
        effect.validate()?;

        // Apply the effect with bounds checking
        match effect {
            VoucherEffect::HandSizeIncrease(amount) => {
                self.hand_size = (self.hand_size + amount).min(50);
            }
            VoucherEffect::JokerSlotIncrease(amount) => {
                self.joker_slots = (self.joker_slots + amount).min(20);
            }
            VoucherEffect::MoneyGain(amount) => {
                self.money = self.money.saturating_add(*amount);
            }
            VoucherEffect::AnteScaling(_multiplier) => {
                // Ante scaling affects ante progression, not current ante
                // This would be handled by the game engine during ante advancement
            }
            VoucherEffect::ExtraPackOptions(_amount) => {
                // Pack options affect shop generation, not game state directly
                // This would be handled by the shop system
            }
            VoucherEffect::BlindScoreReduction(_multiplier) => {
                // Blind score reduction affects scoring calculations, not game state directly
                // This would be handled by the scoring system
            }
            VoucherEffect::StartingCards(_cards) => {
                // Starting cards affect deck initialization, not current game state
                // This would be handled during game setup
            }
            VoucherEffect::ShopSlotIncrease(_amount) => {
                // Shop slots affect shop generation, not game state directly
                // This would be handled by the shop system
            }
            VoucherEffect::DiscardIncrease(_amount) => {
                // Discard increases affect round mechanics, not persistent game state
                // This would be handled by the round system
            }
            VoucherEffect::PlayIncrease(_amount) => {
                // Play increases affect round mechanics, not persistent game state
                // This would be handled by the round system
            }
            VoucherEffect::HandSizeDecrease(amount) => {
                self.hand_size = self.hand_size.saturating_sub(*amount).max(1);
            }
            VoucherEffect::JokerSlotDecrease(amount) => {
                self.joker_slots = self.joker_slots.saturating_sub(*amount).max(1);
            }
            VoucherEffect::InterestCapIncrease(_amount) => {
                // Interest cap increases affect interest calculation, not game state directly
                // This would be handled by the interest system
            }
            VoucherEffect::AnteWinRequirementIncrease(_amount) => {
                // Ante win requirement affects victory condition, not current game state
                // This would be handled by the victory system
            }
            VoucherEffect::AnteWinRequirementDecrease(_amount) => {
                // Ante win requirement affects victory condition, not current game state
                // This would be handled by the victory system
            }
            VoucherEffect::DiscardDecrease(_amount) => {
                // Discard decreases affect round mechanics, not persistent game state
                // This would be handled by the round system
            }
            VoucherEffect::TarotFrequencyMultiplier(_multiplier) => {
                // Tarot frequency affects shop/pack generation, not game state directly
                // This would be handled by the shop system
            }
            VoucherEffect::PlanetFrequencyMultiplier(_multiplier) => {
                // Planet frequency affects shop/pack generation, not game state directly
                // This would be handled by the shop system
            }
            VoucherEffect::PolychromeFrequencyMultiplier(_multiplier) => {
                // Enhanced card frequency affects shop/pack generation, not game state directly
                // This would be handled by the shop system
            }
            VoucherEffect::ShopDiscountPercent(_discount) => {
                // Shop discount affects item pricing, not game state directly
                // This would be handled by the shop system
            }
            VoucherEffect::ShopDiscountMultiplier(_multiplier) => {
                // Shop discount affects shop pricing, not game state directly
                // This would be handled by the shop system
            }
            VoucherEffect::RerollCostReduction(_amount) => {
                // Reroll cost reduction affects shop reroll pricing, not game state directly
                // This would be handled by the shop system
            }
            VoucherEffect::ConsumableSlotIncrease(_amount) => {
                // Consumable slots affect inventory capacity, not current game state
                // This would be handled by the inventory system
            }
            VoucherEffect::BossBlindRerollEnabled { .. } => {
                // Boss blind reroll affects blind mechanics, not game state directly
                // This would be handled by the blind system
            }
            VoucherEffect::ShopPlayingCardsEnabled => {
                // Shop playing cards enable affects shop generation, not game state directly
                // This would be handled by the shop system
            }
            VoucherEffect::ShopEnhancementsEnabled => {
                // Shop enhancements enable affects shop generation, not game state directly
                // This would be handled by the shop system
            }
            VoucherEffect::NoEffect => {
                // Blank voucher does nothing
            }
        }
        // Validate final state consistency
        self.validate_state()
    }

    /// Add a voucher to the owned collection
    pub fn add_voucher(&mut self, voucher_id: VoucherId) {
        self.vouchers_owned.insert(voucher_id);
    }

    /// Spend money if sufficient funds available
    pub fn spend_money(&mut self, amount: usize) -> Result<(), GameStateError> {
        if self.money < amount {
            return Err(GameStateError::InvalidState {
                reason: format!("Insufficient funds: have {}, need {}", self.money, amount),
            });
        }
        self.money -= amount;
        Ok(())
    }

    /// Validate that the game state is consistent and within reasonable bounds
    pub fn validate_state(&self) -> Result<(), GameStateError> {
        if self.hand_size > 50 {
            return Err(GameStateError::InvalidState {
                reason: format!("Hand size too large: {}", self.hand_size),
            });
        }
        if self.joker_slots > 20 {
            return Err(GameStateError::InvalidState {
                reason: format!("Too many joker slots: {}", self.joker_slots),
            });
        }
        if self.ante > 8 {
            return Err(GameStateError::InvalidState {
                reason: format!("Ante too high: {}", self.ante),
            });
        }
        if self.vouchers_owned.len() > 100 {
            return Err(GameStateError::InvalidState {
                reason: format!("Too many vouchers owned: {}", self.vouchers_owned.len()),
            });
        }
        Ok(())
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
            money: game.money as usize,
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
/// Extended with all shop voucher implementations for Issue #17
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumIter)]
#[cfg_attr(feature = "python", pyo3::pyclass(eq))]
pub enum VoucherId {
    // Shop vouchers from Issue #17

    // Shop vouchers from Issue #17
    /// Overstock voucher - +1 card slot in shop
    Overstock,
    /// Overstock+ voucher - +2 card slots in shop (upgraded version)
    OverstockPlus,
    /// Clearance Sale voucher - All items in shop 50% off
    ClearanceSale,
    /// Hone voucher - Foil/Holo/Polychrome cards appear 2X more
    Hone,
    /// Reroll Surplus voucher - Rerolls cost $1 less
    RerollSurplus,
    /// Crystal Ball voucher - +1 consumable slot
    CrystalBall,
    /// Liquidation voucher - All cards and packs in shop are 50% off
    Liquidation,
    /// Reroll Glut voucher - Rerolls cost $2 less
    RerollGlut,

    // Gameplay vouchers from Issue #18
    /// Grabber voucher - +1 hand size permanently
    Grabber,
    /// Nacho Tong voucher - +1 hand size permanently
    NachoTong,
    /// Wasteful voucher - +1 hand size, +1 discard each round
    Wasteful,
    /// Seed Money voucher - +$1 interest cap
    SeedMoney,
    /// Money Tree voucher - +$2 interest cap
    MoneyTree,
    /// Hieroglyph voucher - -1 Ante, -1 hand each round
    Hieroglyph,
    /// Petroglyph voucher - -1 Ante, -1 discard each round
    Petroglyph,
    /// Antimatter voucher - +1 Joker slot
    Antimatter,
    /// Magic Trick voucher - Playing cards can be purchased from shop
    MagicTrick,
    /// Illusion voucher - Playing cards in shop may have enhancements
    Illusion,
    /// Blank voucher - Does nothing (flavor text)
    Blank,
    /// Paint Brush voucher - +1 hand size, -1 joker slot
    PaintBrush,
    /// Tarot Merchant voucher - Tarot cards appear 2X more
    TarotMerchant,
    /// Tarot Tycoon voucher - Tarot cards appear 4X more
    TarotTycoon,

    // Missing upgrade vouchers from Issue #727
    /// Glow Up voucher - Foil, Holographic, and Polychrome cards appear 4X more often (upgrade of Hone)
    GlowUp,
    /// Recyclomancy voucher - Permanently gain +1 discard each round (upgrade of Wasteful)
    Recyclomancy,
    /// Planet Merchant voucher - Planet cards appear 2X more frequently in shop
    PlanetMerchant,
    /// Planet Tycoon voucher - Planet cards appear 4X more frequently in shop (upgrade of Planet Merchant)
    PlanetTycoon,
    /// Director's Cut voucher - Reroll Boss Blind 1 time per Ante, $10 per roll
    DirectorsCut,
    /// Retcon voucher - Reroll Boss Blinds unlimited times, $10 per roll (upgrade of Director's Cut)
    Retcon,
    /// Palette voucher - +1 hand size (upgrade of Paint Brush)
    Palette,
}

impl fmt::Display for VoucherId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VoucherId::Overstock => write!(f, "Overstock"),
            VoucherId::OverstockPlus => write!(f, "Overstock Plus"),
            VoucherId::ClearanceSale => write!(f, "Clearance Sale"),
            VoucherId::Hone => write!(f, "Hone"),
            VoucherId::RerollSurplus => write!(f, "Reroll Surplus"),
            VoucherId::CrystalBall => write!(f, "Crystal Ball"),
            VoucherId::Liquidation => write!(f, "Liquidation"),
            VoucherId::RerollGlut => write!(f, "Reroll Glut"),
            VoucherId::Grabber => write!(f, "Grabber"),
            VoucherId::NachoTong => write!(f, "Nacho Tong"),
            VoucherId::Wasteful => write!(f, "Wasteful"),
            VoucherId::SeedMoney => write!(f, "Seed Money"),
            VoucherId::MoneyTree => write!(f, "Money Tree"),
            VoucherId::Hieroglyph => write!(f, "Hieroglyph"),
            VoucherId::Petroglyph => write!(f, "Petroglyph"),
            VoucherId::Antimatter => write!(f, "Antimatter"),
            VoucherId::MagicTrick => write!(f, "Magic Trick"),
            VoucherId::Illusion => write!(f, "Illusion"),
            VoucherId::Blank => write!(f, "Blank"),
            VoucherId::PaintBrush => write!(f, "Paint Brush"),
            VoucherId::TarotMerchant => write!(f, "Tarot Merchant"),
            VoucherId::TarotTycoon => write!(f, "Tarot Tycoon"),
            VoucherId::GlowUp => write!(f, "Glow Up"),
            VoucherId::Recyclomancy => write!(f, "Recyclomancy"),
            VoucherId::PlanetMerchant => write!(f, "Planet Merchant"),
            VoucherId::PlanetTycoon => write!(f, "Planet Tycoon"),
            VoucherId::DirectorsCut => write!(f, "Director's Cut"),
            VoucherId::Retcon => write!(f, "Retcon"),
            VoucherId::Palette => write!(f, "Palette"),
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
            // Base vouchers have no prerequisites
            VoucherId::Overstock => vec![],
            VoucherId::ClearanceSale => vec![],
            VoucherId::Hone => vec![],
            VoucherId::RerollSurplus => vec![],
            VoucherId::CrystalBall => vec![],
            VoucherId::Liquidation => vec![VoucherId::ClearanceSale],
            VoucherId::RerollGlut => vec![VoucherId::RerollSurplus],

            // Upgraded versions require base versions
            VoucherId::OverstockPlus => vec![VoucherId::Overstock],

            // Gameplay vouchers from Issue #18 - most are base vouchers
            VoucherId::Grabber => vec![],
            VoucherId::NachoTong => vec![VoucherId::Grabber],
            VoucherId::Wasteful => vec![],
            VoucherId::SeedMoney => vec![],
            VoucherId::Hieroglyph => vec![],
            VoucherId::Petroglyph => vec![VoucherId::Hieroglyph],
            VoucherId::Antimatter => vec![VoucherId::Blank],
            VoucherId::MagicTrick => vec![],
            VoucherId::Illusion => vec![VoucherId::MagicTrick],
            VoucherId::Blank => vec![],
            VoucherId::PaintBrush => vec![],
            VoucherId::TarotMerchant => vec![],

            // Upgraded versions require base versions
            VoucherId::MoneyTree => vec![VoucherId::SeedMoney],
            VoucherId::TarotTycoon => vec![VoucherId::TarotMerchant],

            // Missing upgrade vouchers from Issue #727
            VoucherId::GlowUp => vec![VoucherId::Hone],
            VoucherId::Recyclomancy => vec![VoucherId::Wasteful],
            VoucherId::PlanetMerchant => vec![],
            VoucherId::PlanetTycoon => vec![VoucherId::PlanetMerchant],
            VoucherId::DirectorsCut => vec![],
            VoucherId::Retcon => vec![VoucherId::DirectorsCut],
            VoucherId::Palette => vec![VoucherId::PaintBrush],
        }
    }

    /// Get the base cost of this voucher
    pub fn base_cost(&self) -> usize {
        match self {
            VoucherId::Overstock => 10,
            VoucherId::OverstockPlus => 10,
            VoucherId::ClearanceSale => 10,
            VoucherId::Hone => 10,
            VoucherId::RerollSurplus => 10,
            VoucherId::CrystalBall => 10,
            VoucherId::Liquidation => 10,
            VoucherId::RerollGlut => 10,

            // Gameplay vouchers from Issue #18
            VoucherId::Grabber => 10,
            VoucherId::NachoTong => 10,
            VoucherId::Wasteful => 10,
            VoucherId::SeedMoney => 10,
            VoucherId::MoneyTree => 10,
            VoucherId::Hieroglyph => 10,
            VoucherId::Petroglyph => 10,
            VoucherId::Antimatter => 10,
            VoucherId::MagicTrick => 10,
            VoucherId::Illusion => 10,
            VoucherId::Blank => 10,
            VoucherId::PaintBrush => 10, // Mixed effect
            VoucherId::TarotMerchant => 10,
            VoucherId::TarotTycoon => 10,
            VoucherId::GlowUp => 10,
            VoucherId::Recyclomancy => 10,
            VoucherId::PlanetMerchant => 10,
            VoucherId::PlanetTycoon => 10,
            VoucherId::DirectorsCut => 10,
            VoucherId::Retcon => 10,
            VoucherId::Palette => 10,
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

// Re-export individual voucher implementations
pub use implementations::*;
