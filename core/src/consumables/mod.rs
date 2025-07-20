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
    /// Requires selecting specific number of cards
    Cards(usize),
    /// Requires selecting a hand type
    HandType,
    /// Requires selecting a joker
    Joker,
    /// Targets the deck
    Deck,
    /// Targets shop elements
    Shop,
}

/// Specific target for consumable application
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Target {
    /// No target required
    None,
    /// Target specific cards by index in hand/deck
    Cards(Vec<usize>),
    /// Target a specific hand type for planet cards
    HandType(HandRank),
    /// Target a joker by slot index
    Joker(usize),
    /// Target the entire deck
    Deck,
    /// Target shop slots for purchase effects
    Shop(usize),
}

impl Target {
    /// Get the target type for this target
    pub fn target_type(&self) -> TargetType {
        match self {
            Target::None => TargetType::None,
            Target::Cards(cards) => TargetType::Cards(cards.len()),
            Target::HandType(_) => TargetType::HandType,
            Target::Joker(_) => TargetType::Joker,
            Target::Deck => TargetType::Deck,
            Target::Shop(_) => TargetType::Shop,
        }
    }

    /// Check if this target is valid for the expected target type
    pub fn is_valid_type(&self, expected: TargetType) -> bool {
        match (self, expected) {
            (Target::None, TargetType::None) => true,
            (Target::Cards(cards), TargetType::Cards(expected_count)) => {
                cards.len() == expected_count
            }
            (Target::HandType(_), TargetType::HandType) => true,
            (Target::Joker(_), TargetType::Joker) => true,
            (Target::Deck, TargetType::Deck) => true,
            (Target::Shop(_), TargetType::Shop) => true,
            _ => false,
        }
    }

    /// Get the number of cards targeted by this target
    pub fn card_count(&self) -> usize {
        match self {
            Target::None => 0,
            Target::Cards(cards) => cards.len(),
            Target::HandType(_) => 0,
            Target::Joker(_) => 0,
            Target::Deck => 0,
            Target::Shop(_) => 0,
        }
    }

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
            Target::Shop(_) => true, // Shop validation would require shop state
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

/// Fixed capacity consumable card slots for managing player inventory
/// 
/// This struct provides the foundation for consumable inventory management
/// with proper capacity limits and basic slot operations. It maintains
/// a fixed capacity (default 2) and tracks which slots are occupied.
/// 
/// # Thread Safety
/// 
/// This struct is designed to be thread-safe through the use of standard
/// Rust collection types (Vec) and primitive types (usize), which have
/// proper Send + Sync implementations.
/// 
/// # Examples
/// 
/// ```rust
/// use balatro_rs::consumables::ConsumableSlots;
/// 
/// // Create slots with default capacity
/// let slots = ConsumableSlots::new();
/// assert_eq!(slots.capacity(), 2);
/// assert!(slots.is_empty());
/// 
/// // Create slots with custom capacity
/// let large_slots = ConsumableSlots::with_capacity(5);
/// assert_eq!(large_slots.capacity(), 5);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsumableSlots {
    /// Current maximum capacity of slots
    capacity: usize,
    /// Vector of optional consumable slots (storing IDs for now, will expand to full objects later)
    slots: Vec<Option<ConsumableId>>,
    /// Default capacity for new instances (always 2 as per Balatro base game)
    default_capacity: usize,
}

impl ConsumableSlots {
    /// Creates a new ConsumableSlots instance with default capacity of 2
    /// 
    /// This matches the base Balatro game behavior where players start
    /// with 2 consumable slots.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use balatro_rs::consumables::ConsumableSlots;
    /// 
    /// let slots = ConsumableSlots::new();
    /// assert_eq!(slots.capacity(), 2);
    /// assert_eq!(slots.len(), 0);
    /// assert!(slots.is_empty());
    /// ```
    pub fn new() -> Self {
        Self::with_capacity(2)
    }
    
    /// Creates a new ConsumableSlots instance with specified capacity
    /// 
    /// This allows for customization of slot capacity, which may be
    /// modified by vouchers or special game modes in the future.
    /// 
    /// # Arguments
    /// 
    /// * `capacity` - Maximum number of consumable slots
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use balatro_rs::consumables::ConsumableSlots;
    /// 
    /// let slots = ConsumableSlots::with_capacity(5);
    /// assert_eq!(slots.capacity(), 5);
    /// assert_eq!(slots.available_slots(), 5);
    /// ```
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            capacity,
            slots: vec![None; capacity],
            default_capacity: 2,
        }
    }
    
    /// Returns the current maximum capacity of the slots
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use balatro_rs::consumables::ConsumableSlots;
    /// 
    /// let slots = ConsumableSlots::new();
    /// assert_eq!(slots.capacity(), 2);
    /// 
    /// let large_slots = ConsumableSlots::with_capacity(10);
    /// assert_eq!(large_slots.capacity(), 10);
    /// ```
    pub fn capacity(&self) -> usize {
        self.capacity
    }
    
    /// Returns the number of currently occupied slots
    /// 
    /// This counts only the slots that contain consumables,
    /// not the total capacity.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use balatro_rs::consumables::ConsumableSlots;
    /// 
    /// let slots = ConsumableSlots::new();
    /// assert_eq!(slots.len(), 0); // No consumables yet
    /// ```
    pub fn len(&self) -> usize {
        self.slots.iter().filter(|slot| slot.is_some()).count()
    }
    
    /// Returns true if no slots are currently occupied
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use balatro_rs::consumables::ConsumableSlots;
    /// 
    /// let slots = ConsumableSlots::new();
    /// assert!(slots.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    
    /// Returns true if all slots are currently occupied
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use balatro_rs::consumables::ConsumableSlots;
    /// 
    /// let slots = ConsumableSlots::new();
    /// assert!(!slots.is_full()); // Empty slots are not full
    /// ```
    pub fn is_full(&self) -> bool {
        self.len() == self.capacity
    }
    
    /// Returns the number of available (empty) slots
    /// 
    /// This is equivalent to `capacity() - len()`.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use balatro_rs::consumables::ConsumableSlots;
    /// 
    /// let slots = ConsumableSlots::new();
    /// assert_eq!(slots.available_slots(), 2); // All slots available
    /// 
    /// let large_slots = ConsumableSlots::with_capacity(5);
    /// assert_eq!(large_slots.available_slots(), 5);
    /// ```
    pub fn available_slots(&self) -> usize {
        self.capacity - self.len()
    }
}

impl Default for ConsumableSlots {
    /// Creates ConsumableSlots with default capacity of 2
    fn default() -> Self {
        Self::new()
    }
}

// Re-export submodules when they are implemented
// pub mod tarot;
// pub mod planet;
// pub mod spectral;

// Re-export commonly used types
pub use ConsumableId::*;
