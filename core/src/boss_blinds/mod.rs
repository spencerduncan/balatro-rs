//! Boss Blinds module for Balatro game engine
//!
//! This module provides the infrastructure for boss blind cards in Balatro.
//! Boss blinds are special blinds with unique effects that modify gameplay.
//!
//! # Module Organization
//!
//! - `mod.rs` - Core types and traits for boss blinds
//! - `implementations.rs` - Specific boss blind implementations (future)
//!
//! # Design Principles
//!
//! - Boss blinds have special effects that activate during the blind
//! - Effects can modify scoring, card behavior, or game rules
//! - Boss blinds typically have higher score requirements than normal blinds
//! - Each boss blind has unique mechanics that challenge the player

use serde::{Deserialize, Serialize};
use std::fmt;
use strum::{EnumIter, IntoEnumIterator};

/// Core trait that all boss blind types must implement
pub trait BossBlind {
    /// Get the name of this boss blind
    fn name(&self) -> &'static str;

    /// Get the description of what this boss blind does
    fn description(&self) -> &'static str;

    /// Get the base score requirement for this boss blind
    fn base_score_requirement(&self) -> usize;

    /// Get the reward multiplier for defeating this boss blind
    fn reward_multiplier(&self) -> f64;

    /// Apply the boss blind's effect when the blind starts
    /// Called when the player selects this boss blind
    fn on_blind_start(&self, game: &mut crate::game::Game);

    /// Apply the boss blind's effect during hand evaluation
    /// Called for each hand played during the blind
    fn on_hand_played(
        &self,
        game: &mut crate::game::Game,
        hand: &crate::hand::Hand,
    ) -> HandModification;

    /// Apply the boss blind's effect when the blind ends
    /// Called when the blind is completed or failed
    fn on_blind_end(&self, game: &mut crate::game::Game);

    /// Check if this boss blind's effect should modify a specific card
    fn affects_card(&self, card: &crate::card::Card) -> bool {
        // Default implementation - most boss blinds don't affect specific cards
        let _ = card; // Suppress unused parameter warning
        false
    }
}

/// Modifications that can be applied to hand scoring by boss blinds
#[derive(Debug, Clone, Default)]
pub struct HandModification {
    /// Multiplier applied to the hand's score
    pub score_multiplier: f64,
    /// Additive bonus to the hand's score
    pub score_bonus: isize,
    /// Whether the hand should be discarded (some boss blinds can force discards)
    pub force_discard: bool,
    /// Whether certain cards should be disabled during scoring
    pub disabled_cards: Vec<crate::card::Card>,
}

impl HandModification {
    /// Create a new hand modification with default values
    pub fn new() -> Self {
        Self {
            score_multiplier: 1.0,
            score_bonus: 0,
            force_discard: false,
            disabled_cards: vec![],
        }
    }

    /// Create a hand modification that multiplies the score
    pub fn multiply_score(multiplier: f64) -> Self {
        Self {
            score_multiplier: multiplier,
            ..Default::default()
        }
    }

    /// Create a hand modification that adds to the score
    pub fn add_score(bonus: isize) -> Self {
        Self {
            score_bonus: bonus,
            ..Default::default()
        }
    }

    /// Create a hand modification that forces discard
    pub fn force_discard() -> Self {
        Self {
            force_discard: true,
            ..Default::default()
        }
    }
}

/// Identifier for all boss blind types in the game
/// This will be extended as boss blind implementations are added
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumIter)]
pub enum BossBlindId {
    // Placeholder variants - will be expanded in future implementations
    /// Placeholder for future boss blind implementations
    BossBlindPlaceholder,
}

impl fmt::Display for BossBlindId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BossBlindId::BossBlindPlaceholder => write!(f, "Boss Blind Placeholder"),
        }
    }
}

impl BossBlindId {
    /// Get all available boss blind IDs
    pub fn all() -> Vec<BossBlindId> {
        Self::iter().collect()
    }

    /// Get the base score requirement for this boss blind
    pub fn base_score_requirement(&self) -> usize {
        match self {
            BossBlindId::BossBlindPlaceholder => 300, // Placeholder value
        }
    }

    /// Get the reward multiplier for this boss blind
    pub fn reward_multiplier(&self) -> f64 {
        match self {
            BossBlindId::BossBlindPlaceholder => 1.5, // Placeholder value
        }
    }
}

/// Boss blind state tracking
/// Tracks which boss blind is currently active and its state
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BossBlindState {
    /// Currently active boss blind, if any
    pub active_boss: Option<BossBlindId>,
    /// Whether the boss blind effect is currently active
    pub effect_active: bool,
    /// Custom state data for boss blind effects
    pub custom_state: std::collections::HashMap<String, serde_json::Value>,
}

impl BossBlindState {
    /// Create a new boss blind state
    pub fn new() -> Self {
        Self::default()
    }

    /// Activate a boss blind
    pub fn activate(&mut self, boss: BossBlindId) {
        self.active_boss = Some(boss);
        self.effect_active = true;
        self.custom_state.clear();
    }

    /// Deactivate the current boss blind
    pub fn deactivate(&mut self) {
        self.active_boss = None;
        self.effect_active = false;
        self.custom_state.clear();
    }

    /// Check if a boss blind is currently active
    pub fn is_active(&self) -> bool {
        self.active_boss.is_some() && self.effect_active
    }

    /// Get the currently active boss blind
    pub fn active_boss(&self) -> Option<BossBlindId> {
        self.active_boss
    }

    /// Set custom state data for boss blind effects
    pub fn set_custom_state(&mut self, key: String, value: serde_json::Value) {
        self.custom_state.insert(key, value);
    }

    /// Get custom state data for boss blind effects
    pub fn get_custom_state(&self, key: &str) -> Option<&serde_json::Value> {
        self.custom_state.get(key)
    }
}

// Re-export commonly used types
pub use BossBlindId::*;
