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

use crate::card::Card;
use crate::game::Game;
use crate::joker::{Joker, JokerId};
use crate::rank::HandRank;
use serde::{Deserialize, Serialize};
use std::fmt;
use strum::{EnumIter, IntoEnumIterator};
use thiserror::Error;

/// Represents different collections of cards that can be targeted by consumables
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CardCollection {
    /// Cards currently in the player's hand
    Hand,
    /// Cards in the deck
    Deck,
    /// Cards in the discard pile
    DiscardPile,
    /// Cards that were played this round
    PlayedCards,
}

/// Error types for consumable operations
#[derive(Error, Debug, Clone, PartialEq)]
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

impl fmt::Display for CardCollection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CardCollection::Hand => write!(f, "Hand"),
            CardCollection::Deck => write!(f, "Deck"),
            CardCollection::DiscardPile => write!(f, "Discard Pile"),
            CardCollection::PlayedCards => write!(f, "Played Cards"),
        }
    }
}

/// Error types for target validation
#[derive(Debug, Error, Clone, PartialEq)]
pub enum TargetValidationError {
    #[error("Card index {index} out of bounds (hand size: {hand_size})")]
    CardIndexOutOfBounds { index: usize, hand_size: usize },
    #[error("Card index {index} out of bounds (deck size: {deck_size})")]
    DeckIndexOutOfBounds { index: usize, deck_size: usize },
    #[error("Card index {index} out of bounds (discard pile size: {discard_size})")]
    DiscardIndexOutOfBounds { index: usize, discard_size: usize },
    #[error("Joker slot {slot} is empty or invalid (joker count: {joker_count})")]
    JokerSlotInvalid { slot: usize, joker_count: usize },
    #[error("Hand type {hand_type:?} is not available")]
    HandTypeNotAvailable { hand_type: HandRank },
    #[error("No cards available for targeting")]
    NoCardsAvailable,
    #[error("Shop slot {slot} is invalid or empty")]
    ShopSlotInvalid { slot: usize },
    #[error("Invalid number of cards selected: expected between {min} and {max}, got {actual}")]
    InvalidCardCount {
        min: usize,
        max: usize,
        actual: usize,
    },
    #[error("Card at index {index} is already targeted")]
    CardAlreadyTargeted { index: usize },
}

/// Represents a target for joker-related consumable effects
///
/// This struct provides robust validation for joker targeting with production-ready
/// error handling and clear failure modes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JokerTarget {
    /// The slot index of the targeted joker
    pub slot: usize,
    /// Whether the joker must be active to be targeted
    pub require_active: bool,
    /// Optional requirement for specific joker type
    pub joker_type: Option<JokerId>,
}

impl JokerTarget {
    /// Creates a new JokerTarget with basic slot targeting
    ///
    /// # Arguments
    /// * `slot` - The joker slot index to target
    ///
    /// # Examples
    /// ```
    /// use balatro_rs::consumables::JokerTarget;
    /// let target = JokerTarget::new(0);
    /// assert_eq!(target.slot, 0);
    /// assert!(!target.require_active);
    /// ```
    pub fn new(slot: usize) -> Self {
        Self {
            slot,
            require_active: false,
            joker_type: None,
        }
    }

    /// Creates a JokerTarget that requires the joker to be active
    ///
    /// # Arguments
    /// * `slot` - The joker slot index to target
    ///
    /// # Examples
    /// ```
    /// use balatro_rs::consumables::JokerTarget;
    /// let target = JokerTarget::active_joker(1);
    /// assert_eq!(target.slot, 1);
    /// assert!(target.require_active);
    /// ```
    pub fn active_joker(slot: usize) -> Self {
        Self {
            slot,
            require_active: true,
            joker_type: None,
        }
    }

    /// Creates a JokerTarget that requires a specific joker type
    ///
    /// # Arguments
    /// * `slot` - The joker slot index to target
    /// * `joker_type` - The specific JokerId required at this slot
    ///
    /// # Examples
    /// ```
    /// use balatro_rs::consumables::JokerTarget;
    /// use balatro_rs::joker::JokerId;
    /// let target = JokerTarget::joker_of_type(2, JokerId::Joker);
    /// assert_eq!(target.slot, 2);
    /// assert_eq!(target.joker_type, Some(JokerId::Joker));
    /// ```
    pub fn joker_of_type(slot: usize, joker_type: JokerId) -> Self {
        Self {
            slot,
            require_active: false,
            joker_type: Some(joker_type),
        }
    }

    /// Validates this target against the current game state
    ///
    /// Performs comprehensive validation including:
    /// - Slot bounds checking
    /// - Joker existence verification
    /// - Type matching if specified
    /// - Active state validation if required
    ///
    /// # Production Considerations
    /// - Returns actionable error messages for debugging
    /// - Validates all constraints atomically
    /// - Provides clear failure reasons for telemetry
    pub fn validate(&self, game: &Game) -> Result<(), JokerTargetError> {
        // Check if slot is within bounds
        if self.slot >= game.jokers.len() {
            return Err(JokerTargetError::EmptySlot { slot: self.slot });
        }

        // Get the joker at this slot
        let joker = &game.jokers[self.slot];

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

        // Check active state if required
        if self.require_active && !self.is_joker_active(game) {
            return Err(JokerTargetError::InactiveJoker { slot: self.slot });
        }

        Ok(())
    }

    /// Gets a reference to the joker at the target slot
    ///
    /// # Returns
    /// - `Ok(&dyn Joker)` if validation passes
    /// - `Err(JokerTargetError)` with specific failure reason
    pub fn get_joker<'a>(&self, game: &'a Game) -> Result<&'a dyn Joker, JokerTargetError> {
        self.validate(game)?;
        Ok(game.jokers[self.slot].as_ref())
    }

    /// Checks if the target slot is occupied
    ///
    /// This is a non-failing check that returns false for invalid slots
    pub fn is_slot_occupied(&self, game: &Game) -> bool {
        self.slot < game.jokers.len()
    }

    /// Checks if the joker at this slot is active
    ///
    /// Currently returns true as a placeholder - will be updated when
    /// joker active state tracking is implemented
    fn is_joker_active(&self, _game: &Game) -> bool {
        // TODO: Implement actual active state checking when joker system supports it
        true
    }
}

/// Error types for joker targeting operations
///
/// Designed for production debugging with actionable error messages
/// and structured data for telemetry and monitoring.
#[derive(Debug, Clone, PartialEq, Error)]
pub enum JokerTargetError {
    /// The specified slot is empty or out of bounds
    #[error("Joker slot {slot} is empty or does not exist")]
    EmptySlot { slot: usize },

    /// The joker at the slot has the wrong type
    #[error("Expected joker type {expected:?} at slot but found {actual:?}")]
    WrongJokerType { expected: JokerId, actual: JokerId },

    /// The joker at the slot is not active
    #[error("Joker at slot {slot} is not active")]
    InactiveJoker { slot: usize },
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
// Note: Python bindings disabled for now due to complex enum structure
// #[cfg_attr(feature = "python", pyo3::pyclass)]
pub enum Target {
    /// No target required
    None,
    /// Target specific cards with full validation
    Cards(CardTarget),
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
            Target::Cards(cards) => TargetType::Cards(cards.indices.len()),
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
                cards.indices.len() == expected_count
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
            Target::Cards(cards) => cards.indices.len(),
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
            Target::Cards(cards) => cards.validate(game),
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

    /// Extract the CardTarget if this is a Cards target
    pub fn as_card_target(&self) -> Option<&CardTarget> {
        match self {
            Target::Cards(card_target) => Some(card_target),
            _ => None,
        }
    }

    /// Create a target for cards in hand
    pub fn cards_in_hand(indices: Vec<usize>) -> Self {
        Target::Cards(CardTarget::new(CardCollection::Hand, indices))
    }

    /// Create a target for cards in deck
    pub fn cards_in_deck(indices: Vec<usize>) -> Self {
        Target::Cards(CardTarget::new(CardCollection::Deck, indices))
    }

    /// Create a target for cards in discard pile
    pub fn cards_in_discard(indices: Vec<usize>) -> Self {
        Target::Cards(CardTarget::new(CardCollection::DiscardPile, indices))
    }

    /// Create a target for played cards
    pub fn cards_in_played(indices: Vec<usize>) -> Self {
        Target::Cards(CardTarget::new(CardCollection::PlayedCards, indices))
    }

    /// Create a target for an active joker at a specific slot
    ///
    /// This is a convenience method that creates a Target::Joker.
    /// For actual active validation, use JokerTarget::active_joker directly.
    pub fn active_joker_at_slot(slot: usize) -> Self {
        Target::Joker(slot)
    }

    /// Convert this Target to a JokerTarget if it's a Joker variant
    ///
    /// Returns None for non-Joker targets.
    ///
    /// # Examples
    /// ```
    /// use balatro_rs::consumables::{Target, JokerTarget};
    ///
    /// let joker_target = Target::Joker(2);
    /// let joker = joker_target.as_joker_target().unwrap();
    /// assert_eq!(joker.slot, 2);
    ///
    /// let card_target = Target::None;
    /// assert!(card_target.as_joker_target().is_none());
    /// ```
    pub fn as_joker_target(&self) -> Option<JokerTarget> {
        match self {
            Target::Joker(slot) => Some(JokerTarget::new(*slot)),
            _ => None,
        }
    }

    /// Generate all available targets for a given target type
    pub fn get_available_targets(target_type: TargetType, game: &Game) -> Vec<Target> {
        match target_type {
            TargetType::None => vec![Target::None],
            TargetType::Cards(count) => {
                if count == 0 || count > 5 {
                    // Return empty for performance reasons (> 5 cards) or invalid input (0 cards)
                    return vec![];
                }

                let hand_size = game.available.cards().len();
                if count > hand_size {
                    return vec![];
                }

                // Generate all combinations of selecting `count` cards from hand
                generate_card_combinations(hand_size, count)
            }
            TargetType::HandType => {
                // Return all available hand types (for now, all are available)
                use crate::rank::HandRank;
                vec![
                    Target::HandType(HandRank::HighCard),
                    Target::HandType(HandRank::OnePair),
                    Target::HandType(HandRank::TwoPair),
                    Target::HandType(HandRank::ThreeOfAKind),
                    Target::HandType(HandRank::Straight),
                    Target::HandType(HandRank::Flush),
                    Target::HandType(HandRank::FullHouse),
                    Target::HandType(HandRank::FourOfAKind),
                    Target::HandType(HandRank::StraightFlush),
                    Target::HandType(HandRank::RoyalFlush),
                    Target::HandType(HandRank::FiveOfAKind),
                    Target::HandType(HandRank::FlushHouse),
                    Target::HandType(HandRank::FlushFive),
                ]
            }
            TargetType::Joker => {
                // Return targets for all available joker slots
                (0..game.jokers.len()).map(Target::Joker).collect()
            }
            TargetType::Deck => vec![Target::Deck],
            TargetType::Shop => {
                // For now, return targets for shop slots 0-4 (typical shop size)
                // In future: check actual shop inventory
                (0..5).map(Target::Shop).collect()
            }
        }
    }
}

/// Generate all possible combinations of selecting `count` cards from `hand_size` total cards
fn generate_card_combinations(hand_size: usize, count: usize) -> Vec<Target> {
    if count == 0 || count > hand_size {
        return vec![];
    }

    let mut combinations = Vec::new();
    let mut current_combination = Vec::new();

    generate_combinations_recursive(
        0,
        hand_size,
        count,
        &mut current_combination,
        &mut combinations,
    );

    combinations
        .into_iter()
        .map(Target::cards_in_hand)
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
/// Represents targeting specific cards with validation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CardTarget {
    /// Indices of targeted cards
    pub indices: Vec<usize>,
    /// Which collection the cards are from
    pub collection: CardCollection,
    /// Minimum number of cards required
    pub min_cards: usize,
    /// Maximum number of cards allowed
    pub max_cards: usize,
}

impl CardTarget {
    /// Create a new card target with specified indices and collection
    pub fn new(collection: CardCollection, indices: Vec<usize>) -> Self {
        let count = indices.len();
        Self {
            indices,
            collection,
            min_cards: count,
            max_cards: count,
        }
    }

    /// Create a target for a single card
    pub fn single_card(collection: CardCollection, index: usize) -> Self {
        Self {
            indices: vec![index],
            collection,
            min_cards: 1,
            max_cards: 1,
        }
    }

    /// Create a target with variable card count
    pub fn with_count_range(
        collection: CardCollection,
        indices: Vec<usize>,
        min_cards: usize,
        max_cards: usize,
    ) -> Self {
        Self {
            indices,
            collection,
            min_cards,
            max_cards,
        }
    }

    /// Validate this target against the current game state
    pub fn validate(&self, game: &Game) -> Result<(), TargetValidationError> {
        // Validate card count
        let count = self.indices.len();
        if count < self.min_cards || count > self.max_cards {
            return Err(TargetValidationError::InvalidCardCount {
                min: self.min_cards,
                max: self.max_cards,
                actual: count,
            });
        }

        // Validate indices based on collection
        match self.collection {
            CardCollection::Hand => {
                let hand_size = game.available.cards().len();
                for &index in &self.indices {
                    if index >= hand_size {
                        return Err(TargetValidationError::CardIndexOutOfBounds {
                            index,
                            hand_size,
                        });
                    }
                }
            }
            CardCollection::Deck => {
                let deck_size = game.deck.len();
                for &index in &self.indices {
                    if index >= deck_size {
                        return Err(TargetValidationError::DeckIndexOutOfBounds {
                            index,
                            deck_size,
                        });
                    }
                }
            }
            CardCollection::DiscardPile => {
                let discard_size = game.discarded.len();
                for &index in &self.indices {
                    if index >= discard_size {
                        return Err(TargetValidationError::DiscardIndexOutOfBounds {
                            index,
                            discard_size,
                        });
                    }
                }
            }
            CardCollection::PlayedCards => {
                // For played cards, we check against selected cards
                let selected_size = game.available.selected().len();
                for &index in &self.indices {
                    if index >= selected_size {
                        return Err(TargetValidationError::CardIndexOutOfBounds {
                            index,
                            hand_size: selected_size,
                        });
                    }
                }
            }
        }

        // Check for duplicate indices
        let mut seen = std::collections::HashSet::new();
        for &index in &self.indices {
            if !seen.insert(index) {
                return Err(TargetValidationError::CardAlreadyTargeted { index });
            }
        }

        Ok(())
    }

    /// Get immutable references to the targeted cards
    pub fn get_cards<'a>(&self, game: &'a Game) -> Result<Vec<&'a Card>, TargetValidationError> {
        self.validate(game)?;

        match self.collection {
            CardCollection::Hand => {
                // For now, we'll return an error as accessing individual cards from Available
                // may require modifications to the Available struct
                Err(TargetValidationError::NoCardsAvailable)
            }
            CardCollection::Deck => {
                // Note: This requires deck to expose cards, which may need modification
                Err(TargetValidationError::NoCardsAvailable)
            }
            CardCollection::DiscardPile => {
                let cards: Vec<&Card> = self.indices.iter().map(|&i| &game.discarded[i]).collect();
                Ok(cards)
            }
            CardCollection::PlayedCards => {
                // This also may require modifications to access individual selected cards
                Err(TargetValidationError::NoCardsAvailable)
            }
        }
    }

    /// Get mutable references to the targeted cards
    pub fn get_cards_mut<'a>(
        &self,
        game: &'a mut Game,
    ) -> Result<Vec<&'a mut Card>, TargetValidationError> {
        self.validate(game)?;

        match self.collection {
            CardCollection::Hand => {
                // This is tricky due to borrowing rules - would need to modify Available struct
                Err(TargetValidationError::NoCardsAvailable)
            }
            CardCollection::Deck => Err(TargetValidationError::NoCardsAvailable),
            CardCollection::DiscardPile => {
                // Can't easily get mutable references to multiple cards due to borrowing rules
                // Would need to modify implementation to handle this differently
                Err(TargetValidationError::NoCardsAvailable)
            }
            CardCollection::PlayedCards => Err(TargetValidationError::NoCardsAvailable),
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

    /// Get mock ID for testing purposes - only used in tests
    /// Production implementations should not override this
    fn get_mock_id(&self) -> u32 {
        panic!("get_mock_id() called on non-mock consumable")
    }

    /// Get real ConsumableId for testing purposes - only used in tests
    /// Production implementations should not override this
    fn get_real_id(&self) -> ConsumableId {
        panic!("get_real_id() called on non-wrapper consumable")
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
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumIter, PartialOrd, Ord,
)]
#[cfg_attr(feature = "python", pyo3::pyclass)]
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
    /// The Empress - Enhances 2 selected cards to Mult Cards
    TheEmpress,
    /// The Lovers - Enhances 1 selected card to Wild Card
    TheLovers,
    /// The Chariot - Enhances 1 selected card to Steel Card
    TheChariot,
    /// Strength - Increases rank of up to 2 selected cards by 1
    Strength,
    /// The Hermit - Gain $20 money
    TheHermit,
    /// Wheel of Fortune - 1 in 4 chance to add Foil, Holographic, or Polychrome edition
    WheelOfFortune,

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
    /// Saturn - Levels up Straight
    Saturn,
    /// Uranus - Levels up Two Pair
    Uranus,
    /// Neptune - Levels up Straight Flush
    Neptune,
    /// Pluto - Levels up High Card
    Pluto,
    /// Planet X - Levels up Five of a Kind
    PlanetX,
    /// Ceres - Levels up Flush House
    Ceres,
    /// Eris - Levels up Flush Five
    Eris,

    // Spectral Cards
    /// Familiar - Destroys 1 random card, add 3 random Enhanced face cards to deck
    Familiar,
    /// Grim - Destroys 1 random card, add 2 random Enhanced Aces to deck
    Grim,
    /// Incantation - Destroys 1 random card, add 4 random Enhanced numbered cards to deck
    Incantation,
    /// Immolate - Destroys 5 random cards in hand, gain $20
    Immolate,
    /// Ankh - Create copy of random Joker, destroy all other Jokers
    Ankh,
    /// Deja Vu - Add Red Seal to 1 selected card
    DejaVu,
    /// Hex - Add Polychrome to random Joker, destroy all other Jokers
    Hex,
    /// Trance - Add Blue Seal to 1 selected card
    Trance,
    /// Medium - Add Purple Seal to 1 selected card
    Medium,
    /// Cryptid - Create 2 copies of 1 selected card
    Cryptid,
    /// The Soul - Creates a Legendary Joker (must be room)
    TheSoul,
    /// Black Hole - Upgrade every hand type by 1 level
    BlackHole,

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
            ConsumableId::TheEmpress => write!(f, "The Empress"),
            ConsumableId::TheLovers => write!(f, "The Lovers"),
            ConsumableId::TheChariot => write!(f, "The Chariot"),
            ConsumableId::Strength => write!(f, "Strength"),
            ConsumableId::TheHermit => write!(f, "The Hermit"),
            ConsumableId::WheelOfFortune => write!(f, "Wheel of Fortune"),

            // Planet Cards
            ConsumableId::Mercury => write!(f, "Mercury"),
            ConsumableId::Venus => write!(f, "Venus"),
            ConsumableId::Earth => write!(f, "Earth"),
            ConsumableId::Mars => write!(f, "Mars"),
            ConsumableId::Jupiter => write!(f, "Jupiter"),
            ConsumableId::Saturn => write!(f, "Saturn"),
            ConsumableId::Uranus => write!(f, "Uranus"),
            ConsumableId::Neptune => write!(f, "Neptune"),
            ConsumableId::Pluto => write!(f, "Pluto"),
            ConsumableId::PlanetX => write!(f, "Planet X"),
            ConsumableId::Ceres => write!(f, "Ceres"),
            ConsumableId::Eris => write!(f, "Eris"),

            // Spectral Cards
            ConsumableId::Familiar => write!(f, "Familiar"),
            ConsumableId::Grim => write!(f, "Grim"),
            ConsumableId::Incantation => write!(f, "Incantation"),
            ConsumableId::Immolate => write!(f, "Immolate"),
            ConsumableId::Ankh => write!(f, "Ankh"),
            ConsumableId::DejaVu => write!(f, "Deja Vu"),
            ConsumableId::Hex => write!(f, "Hex"),
            ConsumableId::Trance => write!(f, "Trance"),
            ConsumableId::Medium => write!(f, "Medium"),
            ConsumableId::Cryptid => write!(f, "Cryptid"),
            ConsumableId::TheSoul => write!(f, "The Soul"),
            ConsumableId::BlackHole => write!(f, "Black Hole"),

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
            | ConsumableId::TheEmpress
            | ConsumableId::TheLovers
            | ConsumableId::TheChariot
            | ConsumableId::Strength
            | ConsumableId::TheHermit
            | ConsumableId::WheelOfFortune
            | ConsumableId::TarotPlaceholder => ConsumableType::Tarot,

            // Planet Cards
            ConsumableId::Mercury
            | ConsumableId::Venus
            | ConsumableId::Earth
            | ConsumableId::Mars
            | ConsumableId::Jupiter
            | ConsumableId::Saturn
            | ConsumableId::Uranus
            | ConsumableId::Neptune
            | ConsumableId::Pluto
            | ConsumableId::PlanetX
            | ConsumableId::Ceres
            | ConsumableId::Eris
            | ConsumableId::PlanetPlaceholder => ConsumableType::Planet,

            // Spectral Cards
            ConsumableId::Familiar
            | ConsumableId::Grim
            | ConsumableId::Incantation
            | ConsumableId::Immolate
            | ConsumableId::Ankh
            | ConsumableId::DejaVu
            | ConsumableId::Hex
            | ConsumableId::Trance
            | ConsumableId::Medium
            | ConsumableId::Cryptid
            | ConsumableId::TheSoul
            | ConsumableId::BlackHole
            | ConsumableId::SpectralPlaceholder => ConsumableType::Spectral,
        }
    }

    /// Get all Tarot cards
    pub fn tarot_cards() -> Vec<ConsumableId> {
        vec![
            ConsumableId::TheFool,
            ConsumableId::TheMagician,
            ConsumableId::TheHighPriestess,
            ConsumableId::TheEmpress,
            ConsumableId::TheEmperor,
            ConsumableId::TheHierophant,
            ConsumableId::TheLovers,
            ConsumableId::TheChariot,
            ConsumableId::Strength,
            ConsumableId::TheHermit,
            ConsumableId::WheelOfFortune,
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
            ConsumableId::Saturn,
            ConsumableId::Uranus,
            ConsumableId::Neptune,
            ConsumableId::Pluto,
            ConsumableId::PlanetX,
            ConsumableId::Ceres,
            ConsumableId::Eris,
        ]
    }

    /// Get all Spectral cards
    pub fn spectral_cards() -> Vec<ConsumableId> {
        vec![
            ConsumableId::Familiar,
            ConsumableId::Grim,
            ConsumableId::Incantation,
            ConsumableId::Immolate,
            ConsumableId::Ankh,
            ConsumableId::DejaVu,
            ConsumableId::Hex,
            ConsumableId::Trance,
            ConsumableId::Medium,
            ConsumableId::Cryptid,
            ConsumableId::TheSoul,
            ConsumableId::BlackHole,
        ]
    }
}

/// Spectral card pool management for special restriction rules
///
/// Soul and Black Hole have special properties that distinguish them from
/// regular spectral cards, requiring different generation rules for various
/// sources like joker effects, deck restrictions, and pack distributions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SpectralPool {
    /// Regular spectral cards (16 cards) - excludes Soul and Black Hole
    /// Used by joker effects (Sixth Sense, Seance) and Ghost Deck purchases
    Regular,
    /// Special spectral cards (2 cards) - only Soul and Black Hole
    /// Used for identifying restricted cards
    Special,
    /// All spectral cards (18 cards) - complete set including special cards
    /// Used by most other spectral generation sources
    All,
}

impl SpectralPool {
    /// Get spectral cards for this pool type
    pub fn get_cards(&self) -> Vec<ConsumableId> {
        match self {
            SpectralPool::Regular => vec![
                ConsumableId::Familiar,
                ConsumableId::Grim,
                ConsumableId::Incantation,
                ConsumableId::Immolate,
                ConsumableId::Ankh,
                ConsumableId::DejaVu,
                ConsumableId::Hex,
                ConsumableId::Trance,
                ConsumableId::Medium,
                ConsumableId::Cryptid,
                // Additional regular spectral cards would go here
                // when more are implemented (total 16 in full game)
            ],
            SpectralPool::Special => vec![ConsumableId::TheSoul, ConsumableId::BlackHole],
            SpectralPool::All => {
                let mut all_cards = Self::Regular.get_cards();
                all_cards.extend(Self::Special.get_cards());
                all_cards
            }
        }
    }

    /// Check if a specific spectral card belongs to this pool
    pub fn contains(&self, card: ConsumableId) -> bool {
        self.get_cards().contains(&card)
    }

    /// Check if a spectral card is considered "special" (Soul or Black Hole)
    pub fn is_special_card(card: ConsumableId) -> bool {
        matches!(card, ConsumableId::TheSoul | ConsumableId::BlackHole)
    }

    /// Check if a spectral card is considered "regular" (not special)
    pub fn is_regular_card(card: ConsumableId) -> bool {
        card.consumable_type() == ConsumableType::Spectral && !Self::is_special_card(card)
    }

    /// Get the pool type that contains a specific card
    pub fn pool_containing(card: ConsumableId) -> Option<SpectralPool> {
        if Self::is_special_card(card) {
            Some(SpectralPool::Special)
        } else if Self::is_regular_card(card) {
            Some(SpectralPool::Regular)
        } else {
            None
        }
    }

    /// Get description of what this pool represents
    pub fn description(&self) -> &'static str {
        match self {
            SpectralPool::Regular => "Regular spectral cards (excludes Soul and Black Hole)",
            SpectralPool::Special => "Special spectral cards (Soul and Black Hole only)",
            SpectralPool::All => "All spectral cards (complete set)",
        }
    }
}

impl fmt::Display for SpectralPool {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SpectralPool::Regular => write!(f, "Regular"),
            SpectralPool::Special => write!(f, "Special"),
            SpectralPool::All => write!(f, "All"),
        }
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
    _default_capacity: usize,
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
        let mut slots = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            slots.push(None);
        }
        Self {
            capacity,
            slots,
            _default_capacity: 2,
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
    /// # use balatro_rs::consumables::{Consumable, ConsumableType, ConsumableEffect, TargetType, Target, ConsumableError};
    /// # use balatro_rs::game::Game;
    /// #
    /// # #[derive(Debug)]
    /// # struct MockConsumable;
    /// #
    /// # impl Consumable for MockConsumable {
    /// #     fn consumable_type(&self) -> ConsumableType { ConsumableType::Tarot }
    /// #     fn can_use(&self, _game_state: &Game, _target: &Target) -> bool { true }
    /// #     fn use_effect(&self, _game_state: &mut Game, _target: Target) -> Result<(), ConsumableError> { Ok(()) }
    /// #     fn get_description(&self) -> String { "Mock consumable".to_string() }
    /// #     fn get_target_type(&self) -> TargetType { TargetType::None }
    /// #     fn get_effect_category(&self) -> ConsumableEffect { ConsumableEffect::Utility }
    /// # }
    /// #
    /// # fn create_consumable() -> Box<dyn Consumable> {
    /// #     Box::new(MockConsumable)
    /// # }
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
    /// # use balatro_rs::consumables::{Consumable, ConsumableType, ConsumableEffect, TargetType, Target, ConsumableError};
    /// # use balatro_rs::game::Game;
    /// #
    /// # #[derive(Debug)]
    /// # struct MockConsumable;
    /// #
    /// # impl Consumable for MockConsumable {
    /// #     fn consumable_type(&self) -> ConsumableType { ConsumableType::Tarot }
    /// #     fn can_use(&self, _game_state: &Game, _target: &Target) -> bool { true }
    /// #     fn use_effect(&self, _game_state: &mut Game, _target: Target) -> Result<(), ConsumableError> { Ok(()) }
    /// #     fn get_description(&self) -> String { "Mock consumable".to_string() }
    /// #     fn get_target_type(&self) -> TargetType { TargetType::None }
    /// #     fn get_effect_category(&self) -> ConsumableEffect { ConsumableEffect::Utility }
    /// # }
    /// #
    /// # fn create_consumable() -> Box<dyn Consumable> {
    /// #     Box::new(MockConsumable)
    /// # }
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

        self.slots[index]
            .take()
            .ok_or(SlotError::SlotEmpty { index })
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
    /// # use balatro_rs::consumables::{Consumable, ConsumableType, ConsumableEffect, TargetType, Target, ConsumableError};
    /// # use balatro_rs::game::Game;
    /// #
    /// # #[derive(Debug)]
    /// # struct MockConsumable;
    /// #
    /// # impl Consumable for MockConsumable {
    /// #     fn consumable_type(&self) -> ConsumableType { ConsumableType::Tarot }
    /// #     fn can_use(&self, _game_state: &Game, _target: &Target) -> bool { true }
    /// #     fn use_effect(&self, _game_state: &mut Game, _target: Target) -> Result<(), ConsumableError> { Ok(()) }
    /// #     fn get_description(&self) -> String { "Mock consumable".to_string() }
    /// #     fn get_target_type(&self) -> TargetType { TargetType::None }
    /// #     fn get_effect_category(&self) -> ConsumableEffect { ConsumableEffect::Utility }
    /// # }
    /// #
    /// # fn create_consumable() -> Box<dyn Consumable> {
    /// #     Box::new(MockConsumable)
    /// # }
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
    /// # use balatro_rs::consumables::{Consumable, ConsumableType, ConsumableEffect, TargetType, Target, ConsumableError};
    /// # use balatro_rs::game::Game;
    /// #
    /// # #[derive(Debug)]
    /// # struct MockConsumable;
    /// #
    /// # impl Consumable for MockConsumable {
    /// #     fn consumable_type(&self) -> ConsumableType { ConsumableType::Tarot }
    /// #     fn can_use(&self, _game_state: &Game, _target: &Target) -> bool { true }
    /// #     fn use_effect(&self, _game_state: &mut Game, _target: Target) -> Result<(), ConsumableError> { Ok(()) }
    /// #     fn get_description(&self) -> String { "Mock consumable".to_string() }
    /// #     fn get_target_type(&self) -> TargetType { TargetType::None }
    /// #     fn get_effect_category(&self) -> ConsumableEffect { ConsumableEffect::Utility }
    /// # }
    /// #
    /// # fn create_consumable() -> Box<dyn Consumable> {
    /// #     Box::new(MockConsumable)
    /// # }
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
        match self.slots[index].as_mut() {
            Some(boxed) => Some(boxed.as_mut()),
            None => None,
        }
    }

    /// Finds the first empty slot
    ///
    /// Returns Some(index) if an empty slot is found, None otherwise.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use balatro_rs::consumables::ConsumableSlots;
    /// # use balatro_rs::consumables::{Consumable, ConsumableType, ConsumableEffect, TargetType, Target, ConsumableError};
    /// # use balatro_rs::game::Game;
    /// #
    /// # #[derive(Debug)]
    /// # struct MockConsumable;
    /// #
    /// # impl Consumable for MockConsumable {
    /// #     fn consumable_type(&self) -> ConsumableType { ConsumableType::Tarot }
    /// #     fn can_use(&self, _game_state: &Game, _target: &Target) -> bool { true }
    /// #     fn use_effect(&self, _game_state: &mut Game, _target: Target) -> Result<(), ConsumableError> { Ok(()) }
    /// #     fn get_description(&self) -> String { "Mock consumable".to_string() }
    /// #     fn get_target_type(&self) -> TargetType { TargetType::None }
    /// #     fn get_effect_category(&self) -> ConsumableEffect { ConsumableEffect::Utility }
    /// # }
    /// #
    /// # fn create_consumable() -> Box<dyn Consumable> {
    /// #     Box::new(MockConsumable)
    /// # }
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
    /// # use balatro_rs::consumables::{Consumable, ConsumableType, ConsumableEffect, TargetType, Target, ConsumableError};
    /// # use balatro_rs::game::Game;
    /// #
    /// # #[derive(Debug)]
    /// # struct MockConsumable;
    /// #
    /// # impl Consumable for MockConsumable {
    /// #     fn consumable_type(&self) -> ConsumableType { ConsumableType::Tarot }
    /// #     fn can_use(&self, _game_state: &Game, _target: &Target) -> bool { true }
    /// #     fn use_effect(&self, _game_state: &mut Game, _target: Target) -> Result<(), ConsumableError> { Ok(()) }
    /// #     fn get_description(&self) -> String { "Mock consumable".to_string() }
    /// #     fn get_target_type(&self) -> TargetType { TargetType::None }
    /// #     fn get_effect_category(&self) -> ConsumableEffect { ConsumableEffect::Utility }
    /// # }
    /// #
    /// # fn create_consumable() -> Box<dyn Consumable> {
    /// #     Box::new(MockConsumable)
    /// # }
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
    /// # use balatro_rs::consumables::{Consumable, ConsumableType, ConsumableEffect, TargetType, Target, ConsumableError};
    /// # use balatro_rs::game::Game;
    /// #
    /// # #[derive(Debug)]
    /// # struct MockConsumable;
    /// #
    /// # impl Consumable for MockConsumable {
    /// #     fn consumable_type(&self) -> ConsumableType { ConsumableType::Tarot }
    /// #     fn can_use(&self, _game_state: &Game, _target: &Target) -> bool { true }
    /// #     fn use_effect(&self, _game_state: &mut Game, _target: Target) -> Result<(), ConsumableError> { Ok(()) }
    /// #     fn get_description(&self) -> String { "Mock consumable".to_string() }
    /// #     fn get_target_type(&self) -> TargetType { TargetType::None }
    /// #     fn get_effect_category(&self) -> ConsumableEffect { ConsumableEffect::Utility }
    /// # }
    /// #
    /// # fn create_consumable() -> Box<dyn Consumable> {
    /// #     Box::new(MockConsumable)
    /// # }
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

impl Clone for ConsumableSlots {
    /// Clone implementation for ConsumableSlots
    /// Note: This creates a new ConsumableSlots with the same capacity but empty slots,
    /// since trait objects cannot be cloned. This is primarily for testing purposes.
    fn clone(&self) -> Self {
        Self::with_capacity(self.capacity)
    }
}

impl Default for ConsumableSlots {
    /// Creates ConsumableSlots with default capacity of 2
    fn default() -> Self {
        Self::new()
    }
}

/// Serializable representation of ConsumableSlots for serde support
/// Since trait objects cannot be serialized, we only store capacity and length
#[derive(Serialize, Deserialize)]
struct ConsumableSlotsData {
    capacity: usize,
    occupied_count: usize,
}

impl Serialize for ConsumableSlots {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let data = ConsumableSlotsData {
            capacity: self.capacity,
            occupied_count: self.len(),
        };
        data.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for ConsumableSlots {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let data = ConsumableSlotsData::deserialize(deserializer)?;
        // Create empty slots with the same capacity
        // Note: We can't restore the actual consumables since they weren't serialized
        Ok(Self::with_capacity(data.capacity))
    }
}

// Re-export submodules when they are implemented
// pub mod planet;
pub mod tarot;
// pub mod spectral;

// Re-export key tarot types for convenience
pub use tarot::{CardEnhancement, TarotCard, TarotEffect, TarotError, TarotFactory, TarotRarity};

// Test module
#[cfg(test)]
mod tests;

// Re-export commonly used types
pub use ConsumableId::*;
