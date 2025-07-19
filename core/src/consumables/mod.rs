//! Consumables module for Balatro game engine
//!
//! This module provides the infrastructure for consumable cards in Balatro,
//! including Tarot cards, Planet cards, and Spectral cards.
//!
//! # Module Organization
//!
//! - `mod.rs` - Core types and traits for consumables
//! - `tarot.rs` - Tarot card implementations  
//! - `planet.rs` - Planet card implementations
//! - `spectral.rs` - Spectral card implementations
//!
//! # Design Principles
//!
//! - Follows similar patterns to the joker module for consistency
//! - Maintains clear separation between consumable types
//! - Provides extensible trait-based architecture
//! - Ensures compatibility with existing game flow

use crate::game::Game;
use crate::rank::HandRank;
use serde::{Deserialize, Serialize};
use std::fmt;
use strum::{EnumIter, IntoEnumIterator};
use thiserror::Error;

/// Error types for consumable operations
#[derive(Error, Debug, Clone)]
pub enum ConsumableError {
    #[error("Invalid target: {0}")]
    InvalidTarget(String),
    #[error("Insufficient resources to use consumable")]
    InsufficientResources,
    #[error("Invalid game state: {0}")]
    InvalidGameState(String),
    #[error("Effect failed to apply: {0}")]
    EffectFailed(String),
}

/// Categories of effects that consumables can have
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumIter)]
pub enum ConsumableEffect {
    /// Enhances cards or jokers
    Enhancement,
    /// Destroys cards or elements
    Destruction,
    /// Generates new cards or jokers
    Generation,
    /// Modifies game state or properties
    Modification,
    /// Utility effects like information or minor benefits
    Utility,
}

impl fmt::Display for ConsumableEffect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConsumableEffect::Enhancement => write!(f, "Enhancement"),
            ConsumableEffect::Destruction => write!(f, "Destruction"),
            ConsumableEffect::Generation => write!(f, "Generation"),
            ConsumableEffect::Modification => write!(f, "Modification"),
            ConsumableEffect::Utility => write!(f, "Utility"),
        }
    }
}

/// Types of targets that consumables can affect
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TargetType {
    /// No target required
    None,
    /// Targets a specific number of cards
    Cards(usize),
    /// Targets a hand type
    HandType,
    /// Targets a joker
    Joker,
    /// Targets the deck
    Deck,
}

/// Specific target for consumable application
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Target {
    /// No target
    None,
    /// Target specific cards by index
    Cards(Vec<usize>),
    /// Target a hand type
    HandType(HandRank),
    /// Target a joker by index
    Joker(usize),
    /// Target the deck
    Deck,
}

impl Target {
    /// Validate if this target is valid for the current game state
    pub fn is_valid(&self, game_state: &Game) -> bool {
        match self {
            Target::None => true,
            Target::Cards(cards) => {
                !cards.is_empty()
                    && cards
                        .iter()
                        .all(|&i| i < game_state.available.cards().len())
            }
            Target::HandType(_) => true,
            Target::Joker(index) => *index < game_state.jokers.len(),
            Target::Deck => true,
        }
    }
}

/// Core trait that all consumable types must implement
/// Enhanced version with target validation and effect categorization
pub trait Consumable: Send + Sync + fmt::Debug {
    /// Get the consumable type category
    fn consumable_type(&self) -> ConsumableType;

    /// Check if this consumable can be used with the given target in the current game state
    fn can_use(&self, game_state: &Game, target: &Target) -> bool;

    /// Apply the effect of this consumable to the game state
    /// Future versions will support async for animations
    fn use_effect(&self, game_state: &mut Game, target: Target) -> Result<(), ConsumableError>;

    /// Get the description of what this consumable does
    fn get_description(&self) -> String;

    /// Get the type of target this consumable requires
    fn get_target_type(&self) -> TargetType;

    /// Get the effect category for this consumable
    fn get_effect_category(&self) -> ConsumableEffect;

    // Legacy methods for backward compatibility
    /// Get the name of this consumable
    fn name(&self) -> &'static str {
        "Unknown Consumable"
    }

    /// Get the description as static str (legacy)
    fn description(&self) -> &'static str {
        "No description available"
    }

    /// Get the cost of this consumable in the shop
    fn cost(&self) -> usize {
        3
    }

    /// Legacy apply effect method for backward compatibility
    fn apply_effect(&self, game: &mut Game) -> bool {
        self.use_effect(game, Target::None).is_ok()
    }
}

/// Categories of consumable cards
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumIter)]
pub enum ConsumableType {
    /// Tarot cards that modify deck composition or provide benefits
    Tarot,
    /// Planet cards that upgrade poker hands
    Planet,
    /// Spectral cards with powerful, often risky effects
    Spectral,
}

impl fmt::Display for ConsumableType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConsumableType::Tarot => write!(f, "Tarot"),
            ConsumableType::Planet => write!(f, "Planet"),
            ConsumableType::Spectral => write!(f, "Spectral"),
        }
    }
}

/// Identifier for all consumable cards in the game
/// This will be extended as consumable implementations are added
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumIter)]
pub enum ConsumableId {
    // Tarot Cards
    /// The Fool - Creates last Joker used this round if possible
    TheFool,
    /// The Magician - Enhances 2 selected cards to Lucky Cards
    TheMagician,
    /// The High Priestess - Creates up to 2 Planet Cards
    TheHighPriestess,
    /// The Emperor - Creates up to 2 Tarot Cards
    TheEmperor,
    /// The Hierophant - Enhances 2 selected cards to Bonus Cards
    TheHierophant,

    // Planet Cards
    /// Mercury - Levels up Pair
    Mercury,
    /// Venus - Levels up Two Pair
    Venus,
    /// Earth - Levels up Full House
    Earth,
    /// Mars - Levels up Three of a Kind
    Mars,
    /// Jupiter - Levels up Straight
    Jupiter,

    // Spectral Cards
    /// Familiar - Destroys 1 random card, add 3 random Enhanced face cards to deck
    Familiar,
    /// Grim - Destroys 1 random card, add 2 random Enhanced Aces to deck
    Grim,
    /// Incantation - Destroys 1 random card, add 4 random Enhanced numbered cards to deck
    Incantation,

    // Placeholder variants - will be expanded in future implementations
    /// Placeholder for future Tarot card implementations
    TarotPlaceholder,
    /// Placeholder for future Planet card implementations  
    PlanetPlaceholder,
    /// Placeholder for future Spectral card implementations
    SpectralPlaceholder,
}

impl fmt::Display for ConsumableId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            // Tarot Cards
            ConsumableId::TheFool => write!(f, "The Fool"),
            ConsumableId::TheMagician => write!(f, "The Magician"),
            ConsumableId::TheHighPriestess => write!(f, "The High Priestess"),
            ConsumableId::TheEmperor => write!(f, "The Emperor"),
            ConsumableId::TheHierophant => write!(f, "The Hierophant"),

            // Planet Cards
            ConsumableId::Mercury => write!(f, "Mercury"),
            ConsumableId::Venus => write!(f, "Venus"),
            ConsumableId::Earth => write!(f, "Earth"),
            ConsumableId::Mars => write!(f, "Mars"),
            ConsumableId::Jupiter => write!(f, "Jupiter"),

            // Spectral Cards
            ConsumableId::Familiar => write!(f, "Familiar"),
            ConsumableId::Grim => write!(f, "Grim"),
            ConsumableId::Incantation => write!(f, "Incantation"),

            // Placeholders
            ConsumableId::TarotPlaceholder => write!(f, "Tarot Placeholder"),
            ConsumableId::PlanetPlaceholder => write!(f, "Planet Placeholder"),
            ConsumableId::SpectralPlaceholder => write!(f, "Spectral Placeholder"),
        }
    }
}

impl ConsumableId {
    /// Get all available consumable IDs
    pub fn all() -> Vec<ConsumableId> {
        Self::iter().collect()
    }

    /// Get the consumable type for this ID
    pub fn consumable_type(&self) -> ConsumableType {
        match self {
            // Tarot Cards
            ConsumableId::TheFool
            | ConsumableId::TheMagician
            | ConsumableId::TheHighPriestess
            | ConsumableId::TheEmperor
            | ConsumableId::TheHierophant
            | ConsumableId::TarotPlaceholder => ConsumableType::Tarot,

            // Planet Cards
            ConsumableId::Mercury
            | ConsumableId::Venus
            | ConsumableId::Earth
            | ConsumableId::Mars
            | ConsumableId::Jupiter
            | ConsumableId::PlanetPlaceholder => ConsumableType::Planet,

            // Spectral Cards
            ConsumableId::Familiar
            | ConsumableId::Grim
            | ConsumableId::Incantation
            | ConsumableId::SpectralPlaceholder => ConsumableType::Spectral,
        }
    }

    /// Get all Tarot cards
    pub fn tarot_cards() -> Vec<ConsumableId> {
        vec![
            ConsumableId::TheFool,
            ConsumableId::TheMagician,
            ConsumableId::TheHighPriestess,
            ConsumableId::TheEmperor,
            ConsumableId::TheHierophant,
        ]
    }

    /// Get all Planet cards
    pub fn planet_cards() -> Vec<ConsumableId> {
        vec![
            ConsumableId::Mercury,
            ConsumableId::Venus,
            ConsumableId::Earth,
            ConsumableId::Mars,
            ConsumableId::Jupiter,
        ]
    }

    /// Get all Spectral cards
    pub fn spectral_cards() -> Vec<ConsumableId> {
        vec![
            ConsumableId::Familiar,
            ConsumableId::Grim,
            ConsumableId::Incantation,
        ]
    }
}

// Re-export submodules when they are implemented
// pub mod tarot;
// pub mod planet;
// pub mod spectral;

// Re-export commonly used types
pub use ConsumableId::*;
