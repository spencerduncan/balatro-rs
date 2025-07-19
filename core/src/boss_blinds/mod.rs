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

/// Bounded data types for boss blind custom state
/// Replaces arbitrary JSON deserialization with type-safe alternatives
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BossBlindData {
    /// Integer value with reasonable bounds
    Integer(i64),
    /// Float value for multipliers and percentages
    Float(f64),
    /// Boolean flags for state tracking
    Boolean(bool),
    /// String value with length limits
    String(String), // TODO: Consider adding max length validation
    /// List of integer values for counters and collections
    IntegerList(Vec<i64>),
}

/// Trait for boss blind metadata and information
/// Follows Interface Segregation Principle by focusing only on metadata concerns
pub trait BossBlindInfo {
    /// Get the name of this boss blind
    fn name(&self) -> &'static str;

    /// Get the description of what this boss blind does
    fn description(&self) -> &'static str;

    /// Get the base score requirement for this boss blind
    fn base_score_requirement(&self) -> usize;

    /// Get the reward multiplier for defeating this boss blind
    fn reward_multiplier(&self) -> f64;
}

/// Trait for boss blind lifecycle management
/// Handles activation and deactivation events
pub trait BossBlindLifecycle {
    /// Apply the boss blind's effect when the blind starts
    /// Called when the player selects this boss blind
    fn on_blind_start(&self, game: &mut crate::game::Game);

    /// Apply the boss blind's effect when the blind ends
    /// Called when the blind is completed or failed
    fn on_blind_end(&self, game: &mut crate::game::Game);
}

/// Trait for boss blind scoring modifications
/// Handles scoring effects and card interactions
pub trait BossBlindScoring {
    /// Apply the boss blind's effect during hand evaluation
    /// Called for each hand played during the blind
    fn on_hand_played(
        &self,
        game: &mut crate::game::Game,
        hand: &crate::hand::Hand,
    ) -> HandModification;

    /// Check if this boss blind's effect should modify a specific card
    fn affects_card(&self, card: &crate::card::Card) -> bool {
        // Default implementation - most boss blinds don't affect specific cards
        let _ = card; // Suppress unused parameter warning
        false
    }
}

/// Convenience trait that combines all boss blind traits
/// Implementations can implement this for full boss blind functionality
/// Note: This is the legacy modular approach, kept for backward compatibility
pub trait BossBlindModular: BossBlindInfo + BossBlindLifecycle + BossBlindScoring {}

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
    /// Custom state data for boss blind effects (type-safe)
    pub custom_state: std::collections::HashMap<String, BossBlindData>,
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
    pub fn set_custom_state(&mut self, key: String, value: BossBlindData) {
        self.custom_state.insert(key, value);
    }

    /// Get custom state data for boss blind effects
    pub fn get_custom_state(&self, key: &str) -> Option<&BossBlindData> {
        self.custom_state.get(key)
    }
}

/// Counter types for tracking joker and voucher counters
///
/// Used by boss blinds to determine what counters to check for special effects.
/// Many boss blinds in Balatro have effects that trigger based on specific
/// player actions or game state changes during the blind.
///
/// # Examples
///
/// ```rust
/// use balatro_rs::boss_blinds::CounterType;
///
/// let counters = vec![CounterType::HandsPlayed, CounterType::CardsScored];
/// // Boss blind might disable jokers after 3 hands played
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CounterType {
    /// Count of hands played during the blind
    /// Often used for boss blinds that have escalating effects
    HandsPlayed,
    /// Count of cards scored during the blind
    /// Used for boss blinds that react to scoring mechanics  
    CardsScored,
    /// Count of money spent during the blind
    /// Used for boss blinds that punish spending
    MoneySpent,
    /// Count of cards discarded during the blind
    /// Used for boss blinds that limit or react to discards
    CardsDiscarded,
    /// Count of jokers purchased during the blind
    /// Used for boss blinds that react to shop interactions
    JokersPurchased,
    /// Count of vouchers purchased during the blind
    /// Used for boss blinds that react to voucher acquisition
    VouchersPurchased,
}

/// Effects that boss blinds can apply to modify gameplay
///
/// Provides categorization of different types of boss blind effects.
/// Each effect type represents a different way that boss blinds can
/// interact with and modify the core game mechanics.
///
/// # Design Philosophy
///
/// The effect system is designed to be:
/// - **Extensible**: New effect types can be added easily
/// - **Serializable**: Effects can be saved/loaded with game state
/// - **Descriptive**: Each effect carries information about what it does
///
/// # Examples
///
/// ```rust
/// use balatro_rs::boss_blinds::BlindEffect;
///
/// let effects = vec![
///     BlindEffect::DebuffCards("face_cards".to_string()),
///     BlindEffect::ModifyScoring("score * 0.5".to_string()),
/// ];
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BlindEffect {
    /// Debuff certain cards based on condition
    ///
    /// Examples: "face_cards", "hearts", "red_cards", "clubs_and_spades"
    /// The string describes which cards are affected by the debuff
    DebuffCards(String),

    /// Restrict certain player actions during the blind
    ///
    /// Examples: "no_discards", "max_1_discard", "no_rerolls", "first_hand_only"
    /// The string describes what actions are restricted or modified
    RestrictActions(String),

    /// Modify scoring calculations
    ///
    /// Examples: "score * 0.5", "mult * 2", "chips / 2", "no_face_card_bonus"
    /// The string describes how scoring is modified during the blind
    ModifyScoring(String),

    /// Special rules that don't fit other categories
    ///
    /// Examples: "All cards are red", "Hand size +2", "Jokers disabled after 3 hands"
    /// The string describes the special rule in human-readable format
    SpecialRule(String),
}

/// Unified BossBlind trait as specified in issue #28
///
/// Provides a consistent interface for all boss blind implementations.
/// This trait defines the core contract that all boss blinds must fulfill,
/// enabling consistent behavior across different boss blind types.
///
/// # Design Principles
///
/// - **Thread Safety**: Requires `Send + Sync` for multi-threaded usage
/// - **Debuggable**: Requires `Debug` for development and troubleshooting
/// - **Stateless Interface**: Methods take game state as parameters rather than storing it
/// - **Effect-Driven**: Boss blinds describe their effects rather than implementing them directly
///
/// # Implementation Guidelines
///
/// When implementing this trait:
/// 1. Keep implementations stateless where possible
/// 2. Use `get_effects()` to describe what the boss blind does
/// 3. Use `check_counters()` to specify what game events matter
/// 4. Use `apply_effects()` to modify game state when the blind activates
///
/// # Examples
///
/// ```rust
/// use balatro_rs::boss_blinds::{BossBlind, BlindEffect, CounterType};
/// use balatro_rs::game::Game;
///
/// #[derive(Debug)]
/// struct ExampleBoss;
///
/// impl BossBlind for ExampleBoss {
///     fn name(&self) -> &str { "The Example" }
///     fn min_ante(&self) -> u32 { 1 }
///     fn get_effects(&self) -> Vec<BlindEffect> {
///         vec![BlindEffect::ModifyScoring("score * 0.5".to_string())]
///     }
///     fn check_counters(&self, _game: &Game) -> Vec<CounterType> {
///         vec![CounterType::HandsPlayed]
///     }
///     fn apply_effects(&self, _game: &mut Game) {
///         // Implementation would modify game state here
///     }
/// }
/// ```
pub trait BossBlind: Send + Sync + std::fmt::Debug {
    /// Get the display name of this boss blind
    ///
    /// # Returns
    ///
    /// A string slice containing the human-readable name of the boss blind.
    /// This name is used in UI and logging.
    fn name(&self) -> &str;

    /// Apply the boss blind's effects to the game state
    ///
    /// Called when the boss blind becomes active. This method should modify
    /// the game state to implement the boss blind's effects.
    ///
    /// # Parameters
    ///
    /// * `game_state` - Mutable reference to the current game state
    ///
    /// # Implementation Notes
    ///
    /// - This method should be idempotent when possible
    /// - Effects should be applied immediately and completely
    /// - Use the existing game state fields rather than adding new ones
    fn apply_effects(&self, game_state: &mut crate::game::Game);

    /// Check what counters this boss blind needs to monitor
    ///
    /// Returns a list of counter types that affect this boss blind's behavior.
    /// The game engine can use this information to track relevant statistics
    /// and trigger additional effects when thresholds are reached.
    ///
    /// # Parameters
    ///
    /// * `game_state` - Immutable reference to the current game state
    ///
    /// # Returns
    ///
    /// A vector of `CounterType` values indicating which game events
    /// this boss blind cares about.
    fn check_counters(&self, game_state: &crate::game::Game) -> Vec<CounterType>;

    /// Get the list of effects this boss blind applies
    ///
    /// Returns a description of all effects this boss blind has on gameplay.
    /// This is primarily used for UI display and documentation purposes.
    ///
    /// # Returns
    ///
    /// A vector of `BlindEffect` values describing what this boss blind does.
    fn get_effects(&self) -> Vec<BlindEffect>;

    /// Get the minimum ante required for this boss blind to appear
    ///
    /// # Returns
    ///
    /// The minimum ante level (1-based) at which this boss blind can appear.
    /// Boss blinds with higher minimum antes are more challenging.
    fn min_ante(&self) -> u32;
}

// Re-export commonly used types
pub use BossBlindId::*;
