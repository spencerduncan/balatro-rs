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

use serde::{Deserialize, Serialize};
use std::fmt;
use strum::{EnumIter, IntoEnumIterator};

/// Core trait that all consumable types must implement
pub trait Consumable {
    /// Get the name of this consumable
    fn name(&self) -> &'static str;

    /// Get the description of what this consumable does
    fn description(&self) -> &'static str;

    /// Get the cost of this consumable in the shop
    fn cost(&self) -> usize;

    /// Get the consumable type category
    fn consumable_type(&self) -> ConsumableType;

    /// Apply the effect of this consumable to the game state
    /// Returns true if the consumable was successfully applied
    fn apply_effect(&self, game: &mut crate::game::Game) -> bool;
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
            ConsumableId::TarotPlaceholder => ConsumableType::Tarot,
            ConsumableId::PlanetPlaceholder => ConsumableType::Planet,
            ConsumableId::SpectralPlaceholder => ConsumableType::Spectral,
        }
    }
}

// Re-export submodules when they are implemented
// pub mod tarot;
// pub mod planet;
// pub mod spectral;

// Re-export commonly used types
pub use ConsumableId::*;
