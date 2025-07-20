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
use crate::joker::{Joker, JokerId};
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

/// Error types for slot operations
#[derive(Debug, Error)]
pub enum SlotError {
    #[error("Slot {index} is out of bounds (capacity: {capacity})")]
    IndexOutOfBounds { index: usize, capacity: usize },
    #[error("No empty slots available (capacity: {capacity})")]
    NoEmptySlots { capacity: usize },
    #[error("Slot {index} is already empty")]
    SlotEmpty { index: usize },
}

/// Error types for target validation
#[derive(Debug, Error, Clone)]
pub enum TargetValidationError {
    #[error("Card index {index} out of bounds (hand size: {hand_size})")]
    CardIndexOutOfBounds { index: usize, hand_size: usize },
    #[error("Joker slot {slot} is empty or invalid (joker count: {joker_count})")]
    JokerSlotInvalid { slot: usize, joker_count: usize },
    #[error("Hand type {hand_type:?} is not available")]
    HandTypeNotAvailable { hand_type: HandRank },
    #[error("No cards available for targeting")]
    NoCardsAvailable,
    #[error("Shop slot {slot} is invalid or empty")]
    ShopSlotInvalid { slot: usize },
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

    /// Check if this target is valid for the current game state (simple boolean check)
    pub fn is_valid(&self, game: &Game) -> bool {
        self.validate(game).is_ok()
    }

    /// Validate this target against the current game state with detailed error reporting
    pub fn validate(&self, game: &Game) -> Result<(), TargetValidationError> {
        match self {
            Target::None => Ok(()),
            Target::Cards(indices) => {
                // Check if any cards are available
                let hand_size = game.available.cards().len();
                if hand_size == 0 {
                    return Err(TargetValidationError::NoCardsAvailable);
                }

                // Check if indices list is empty
                if indices.is_empty() {
                    return Err(TargetValidationError::NoCardsAvailable);
                }

                // Validate each card index
                for &index in indices {
                    if index >= hand_size {
                        return Err(TargetValidationError::CardIndexOutOfBounds {
                            index,
                            hand_size,
                        });
                    }
                }
                Ok(())
            }
            Target::HandType(_hand_type) => {
                // For now, all hand types are considered available
                // In future implementations, we might check if the hand type
                // has been discovered/unlocked by the player
                Ok(())
            }
            Target::Joker(slot) => {
                let joker_count = game.jokers.len();
                if *slot >= joker_count {
                    Err(TargetValidationError::JokerSlotInvalid {
                        slot: *slot,
                        joker_count,
                    })
                } else {
                    Ok(())
                }
            }
            Target::Deck => Ok(()), // Deck is always a valid target
            Target::Shop(_slot) => {
                // Shop validation would require shop state implementation
                // For now, we'll accept any shop slot as valid
                // In future: check against actual shop inventory
                Ok(())
            }
        }
    }

    /// Convert a joker target to a JokerTarget struct for enhanced validation
    pub fn as_joker_target(&self) -> Option<JokerTarget> {
        match self {
            Target::Joker(slot) => Some(JokerTarget::new(*slot)),
            _ => None,
        }
    }

    /// Create a target for a joker at the specified slot
    pub fn joker_at_slot(slot: usize) -> Self {
        Target::Joker(slot)
    }

    /// Create a target for an active joker at the specified slot
    /// Note: For full active joker validation, use JokerTarget::active_joker directly
    pub fn active_joker_at_slot(slot: usize) -> Self {
        Target::Joker(slot)
    }

    /// Get all available targets of a specific type for the current game state
    pub fn get_available_targets(target_type: TargetType, game: &Game) -> Vec<Target> {
        match target_type {
            TargetType::None => vec![Target::None],
            TargetType::Cards(count) => {
                let hand_size = game.available.cards().len();
                if hand_size == 0 || count > hand_size {
                    vec![] // No valid card combinations available
                } else if count == 1 {
                    // For single card selection, return each card as a separate target
                    (0..hand_size)
                        .map(|i| Target::Cards(vec![i]))
                        .collect()
                } else {
                    // For multiple card selection, generate all valid combinations
                    // Limited to reasonable number of combinations for performance
                    if count > 5 || count > hand_size {
                        vec![] // Too many combinations or impossible selection
                    } else {
                        generate_card_combinations(hand_size, count)
                    }
                }
            }
            TargetType::HandType => {
                // Return all possible hand types as targets
                use HandRank::*;
                vec![
                    Target::HandType(HighCard),
                    Target::HandType(OnePair),
                    Target::HandType(TwoPair),
                    Target::HandType(ThreeOfAKind),
                    Target::HandType(Straight),
                    Target::HandType(Flush),
                    Target::HandType(FullHouse),
                    Target::HandType(FourOfAKind),
                    Target::HandType(StraightFlush),
                    Target::HandType(RoyalFlush),
                    Target::HandType(FiveOfAKind),
                    Target::HandType(FlushHouse),
                    Target::HandType(FlushFive),
                ]
            }
            TargetType::Joker => {
                // Return targets for each joker slot that has a joker
                (0..game.jokers.len())
                    .map(|i| Target::Joker(i))
                    .collect()
            }
            TargetType::Deck => vec![Target::Deck],
            TargetType::Shop => {
                // Without shop implementation, return empty
                // In future: return available shop slots
                vec![]
            }
        }
    }
}

/// Target information for joker selection with enhanced validation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct JokerTarget {
    /// The slot index of the joker
    pub slot: usize,
    /// Whether the joker must be active (not disabled/sealed)
    pub require_active: bool,
    /// Optional requirement for specific joker type
    pub joker_type: Option<JokerId>,
}

impl JokerTarget {
    /// Create a new JokerTarget for any joker at the given slot
    pub fn new(slot: usize) -> Self {
        Self {
            slot,
            require_active: false,
            joker_type: None,
        }
    }

    /// Create a target that requires an active joker at the given slot
    pub fn active_joker(slot: usize) -> Self {
        Self {
            slot,
            require_active: true,
            joker_type: None,
        }
    }

    /// Create a target for a specific joker type at the given slot
    pub fn joker_of_type(slot: usize, joker_type: JokerId) -> Self {
        Self {
            slot,
            require_active: false,
            joker_type: Some(joker_type),
        }
    }

    /// Validate that this target is valid for the current game state
    pub fn validate(&self, game: &Game) -> Result<(), JokerTargetError> {
        // Check if slot is within bounds
        if self.slot >= game.jokers.len() {
            return Err(JokerTargetError::EmptySlot { slot: self.slot });
        }

        // Get the joker at this slot
        let joker = &game.jokers[self.slot];

        // Check if joker is active if required
        // TODO: This will need to be updated when joker state management is fully implemented
        // For now, we assume all jokers are active
        if self.require_active {
            // In the future, check joker.is_active() or similar
            // For now, this check passes
        }

        // Check joker type if specified
        if let Some(expected_type) = self.joker_type {
            let actual_type = joker.id();
            if actual_type != expected_type {
                return Err(JokerTargetError::WrongJokerType {
                    expected: expected_type,
                    actual: actual_type,
                });
            }
        }

        Ok(())
    }

    /// Get the joker at this target's slot
    pub fn get_joker<'a>(&self, game: &'a Game) -> Result<&'a dyn Joker, JokerTargetError> {
        // Validate first
        self.validate(game)?;
        
        // Return the joker (we know it's valid from validation)
        Ok(&*game.jokers[self.slot])
    }

    /// Check if the slot is occupied (without full validation)
    pub fn is_slot_occupied(&self, game: &Game) -> bool {
        self.slot < game.jokers.len()
    }
}

/// Error types specific to joker targeting
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum JokerTargetError {
    #[error("Joker slot {slot} is empty")]
    EmptySlot { slot: usize },
    #[error("Joker at slot {slot} is not active")]
    InactiveJoker { slot: usize },
    #[error("Expected joker type {expected:?} but found {actual:?}")]
    WrongJokerType {
        expected: JokerId,
        actual: JokerId,
    },
}

/// Generate all possible combinations of selecting `count` cards from `hand_size` total cards
fn generate_card_combinations(hand_size: usize, count: usize) -> Vec<Target> {
    if count == 0 || count > hand_size {
        return vec![];
    }
    
    let mut combinations = Vec::new();
    let mut current_combination = Vec::new();
    
    generate_combinations_recursive(0, hand_size, count, &mut current_combination, &mut combinations);
    
    combinations.into_iter()
        .map(|indices| Target::Cards(indices))
        .collect()
}

/// Recursive helper function to generate combinations
fn generate_combinations_recursive(
    start: usize,
    total: usize,
    remaining: usize,
    current: &mut Vec<usize>,
    all_combinations: &mut Vec<Vec<usize>>,
) {
    if remaining == 0 {
        all_combinations.push(current.clone());
        return;
    }
    
    for i in start..=(total - remaining) {
        current.push(i);
        generate_combinations_recursive(i + 1, total, remaining - 1, current, all_combinations);
        current.pop();
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
#[derive(Debug)]
pub struct ConsumableSlots {
    /// Current maximum capacity of slots
    capacity: usize,
    /// Vector of optional consumable slots
    slots: Vec<Option<Box<dyn Consumable>>>,
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

    /// Adds a consumable to the first available slot
    ///
    /// Returns the index where the consumable was placed, or an error if no slots are available.
    ///
    /// # Arguments
    ///
    /// * `consumable` - The consumable to add
    ///
    /// # Errors
    ///
    /// * `SlotError::NoEmptySlots` - If all slots are currently occupied
    ///
    /// # Examples
    ///
    /// ```rust
    /// use balatro_rs::consumables::{ConsumableSlots, SlotError};
    ///
    /// let mut slots = ConsumableSlots::new();
    /// let consumable = create_consumable(); // Some consumable
    /// 
    /// match slots.add_consumable(consumable) {
    ///     Ok(index) => println!("Added to slot {}", index),
    ///     Err(SlotError::NoEmptySlots { capacity }) => {
    ///         println!("No empty slots (capacity: {})", capacity);
    ///     }
    ///     _ => unreachable!(),
    /// }
    /// ```
    pub fn add_consumable(&mut self, consumable: Box<dyn Consumable>) -> Result<usize, SlotError> {
        if let Some(index) = self.find_empty_slot() {
            self.slots[index] = Some(consumable);
            Ok(index)
        } else {
            Err(SlotError::NoEmptySlots {
                capacity: self.capacity,
            })
        }
    }

    /// Removes a consumable from the specified slot
    ///
    /// Returns the removed consumable, or an error if the index is invalid or the slot is empty.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the slot to remove from
    ///
    /// # Errors
    ///
    /// * `SlotError::IndexOutOfBounds` - If the index is >= capacity
    /// * `SlotError::SlotEmpty` - If the slot at index is already empty
    ///
    /// # Examples
    ///
    /// ```rust
    /// use balatro_rs::consumables::{ConsumableSlots, SlotError};
    ///
    /// let mut slots = ConsumableSlots::new();
    /// // Add a consumable first
    /// let index = slots.add_consumable(create_consumable()).unwrap();
    /// 
    /// // Remove it
    /// match slots.remove_consumable(index) {
    ///     Ok(consumable) => println!("Removed consumable"),
    ///     Err(SlotError::SlotEmpty { index }) => {
    ///         println!("Slot {} is empty", index);
    ///     }
    ///     Err(SlotError::IndexOutOfBounds { index, capacity }) => {
    ///         println!("Index {} out of bounds (capacity: {})", index, capacity);
    ///     }
    ///     _ => unreachable!(),
    /// }
    /// ```
    pub fn remove_consumable(&mut self, index: usize) -> Result<Box<dyn Consumable>, SlotError> {
        if index >= self.capacity {
            return Err(SlotError::IndexOutOfBounds {
                index,
                capacity: self.capacity,
            });
        }

        self.slots[index].take().ok_or(SlotError::SlotEmpty { index })
    }

    /// Gets a reference to the consumable at the specified index
    ///
    /// Returns None if the index is out of bounds or the slot is empty.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the slot to access
    ///
    /// # Examples
    ///
    /// ```rust
    /// use balatro_rs::consumables::ConsumableSlots;
    ///
    /// let mut slots = ConsumableSlots::new();
    /// let index = slots.add_consumable(create_consumable()).unwrap();
    /// 
    /// if let Some(consumable) = slots.get_consumable(index) {
    ///     println!("Found consumable: {:?}", consumable);
    /// }
    /// ```
    pub fn get_consumable(&self, index: usize) -> Option<&dyn Consumable> {
        if index >= self.capacity {
            return None;
        }
        self.slots[index].as_ref().map(|boxed| boxed.as_ref())
    }

    /// Gets a mutable reference to the consumable at the specified index
    ///
    /// Returns None if the index is out of bounds or the slot is empty.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the slot to access
    ///
    /// # Examples
    ///
    /// ```rust
    /// use balatro_rs::consumables::ConsumableSlots;
    ///
    /// let mut slots = ConsumableSlots::new();
    /// let index = slots.add_consumable(create_consumable()).unwrap();
    /// 
    /// if let Some(consumable) = slots.get_consumable_mut(index) {
    ///     // Modify consumable if needed
    /// }
    /// ```
    pub fn get_consumable_mut(&mut self, index: usize) -> Option<&mut dyn Consumable> {
        if index >= self.capacity {
            return None;
        }
        self.slots[index].as_mut().map(|boxed| boxed.as_mut())
    }

    /// Finds the first empty slot
    ///
    /// Returns Some(index) if an empty slot is found, None otherwise.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use balatro_rs::consumables::ConsumableSlots;
    ///
    /// let mut slots = ConsumableSlots::new();
    /// assert_eq!(slots.find_empty_slot(), Some(0)); // First slot is empty
    /// 
    /// // Fill first slot
    /// slots.add_consumable(create_consumable()).unwrap();
    /// assert_eq!(slots.find_empty_slot(), Some(1)); // Second slot is empty
    /// ```
    pub fn find_empty_slot(&self) -> Option<usize> {
        self.slots.iter().position(|slot| slot.is_none())
    }

    /// Clears a specific slot, removing any consumable in it
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the slot to clear
    ///
    /// # Errors
    ///
    /// * `SlotError::IndexOutOfBounds` - If the index is >= capacity
    ///
    /// # Examples
    ///
    /// ```rust
    /// use balatro_rs::consumables::{ConsumableSlots, SlotError};
    ///
    /// let mut slots = ConsumableSlots::new();
    /// slots.add_consumable(create_consumable()).unwrap();
    /// 
    /// // Clear the first slot
    /// slots.clear_slot(0).unwrap();
    /// assert_eq!(slots.len(), 0);
    /// ```
    pub fn clear_slot(&mut self, index: usize) -> Result<(), SlotError> {
        if index >= self.capacity {
            return Err(SlotError::IndexOutOfBounds {
                index,
                capacity: self.capacity,
            });
        }
        self.slots[index] = None;
        Ok(())
    }

    /// Returns an iterator over all consumables in the slots
    ///
    /// This iterates only over occupied slots, skipping empty ones.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use balatro_rs::consumables::ConsumableSlots;
    ///
    /// let mut slots = ConsumableSlots::new();
    /// slots.add_consumable(create_consumable()).unwrap();
    /// 
    /// for consumable in slots.iter() {
    ///     println!("Consumable: {:?}", consumable);
    /// }
    /// ```
    pub fn iter(&self) -> impl Iterator<Item = &dyn Consumable> {
        self.slots
            .iter()
            .filter_map(|slot| slot.as_ref())
            .map(|boxed| boxed.as_ref())
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
pub use SlotError;
