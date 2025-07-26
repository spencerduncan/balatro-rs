//! New trait definitions for the Joker system
//!
//! This module defines focused, single-responsibility traits that will eventually
//! replace the monolithic Joker trait. Each trait handles a specific aspect of
//! joker behavior, making the system more modular and maintainable.

use crate::card::Card;
use crate::hand::SelectHand;
use crate::joker_state::JokerStateManager;
use crate::stage::Stage;
use serde::{Deserialize, Serialize};

/// Simple structure to hold hand scoring information
#[derive(Debug, Clone)]
pub struct HandScore {
    pub chips: u64,
    pub mult: f64,
}

/// Simple game event type for joker processing
#[derive(Debug, Clone)]
pub struct GameEvent {
    pub event_type: String,
    pub data: Option<serde_json::Value>,
}

/// Trait for joker identity and metadata.
///
/// This trait handles the basic identity information for a joker,
/// including its type, name, and descriptive information.
pub trait JokerIdentity: Send + Sync {
    /// Returns the unique type identifier for this joker.
    fn joker_type(&self) -> &'static str;

    /// Returns the display name of this joker.
    fn name(&self) -> &str;

    /// Returns a description of what this joker does.
    fn description(&self) -> &str;

    /// Returns the rarity level of this joker.
    fn rarity(&self) -> Rarity;

    /// Returns the base cost of this joker in the shop.
    fn base_cost(&self) -> u64;

    /// Returns whether this joker is a unique/legendary variant.
    fn is_unique(&self) -> bool {
        false
    }
}

/// Trait for joker lifecycle management.
///
/// This trait handles the lifecycle events of a joker, from purchase
/// through gameplay to sale or destruction.
pub trait JokerLifecycle: Send + Sync {
    /// Called when the joker is purchased from the shop.
    fn on_purchase(&mut self) {}

    /// Called when the joker is sold.
    fn on_sell(&mut self) {}

    /// Called when the joker is destroyed.
    fn on_destroy(&mut self) {}

    /// Called at the start of each round.
    fn on_round_start(&mut self) {}

    /// Called at the end of each round.
    fn on_round_end(&mut self) {}

    /// Called when another joker is added to the collection.
    fn on_joker_added(&mut self, _other_joker_type: &str) {}

    /// Called when another joker is removed from the collection.
    fn on_joker_removed(&mut self, _other_joker_type: &str) {}
}

/// Trait for joker gameplay mechanics.
///
/// This trait handles the core gameplay functionality of jokers,
/// including scoring and card interactions.
///
/// # State Management
///
/// The `process()` method takes a mutable `&mut self` reference, allowing jokers
/// to maintain internal state directly. This enables cleaner, more efficient code
/// compared to external state management.
///
/// ## Common Patterns
///
/// ### Per-Round State
/// ```rust,ignore
/// struct MyJoker {
///     triggered_this_round: bool,
/// }
///
/// impl JokerGameplay for MyJoker {
///     fn process(&mut self, stage: &Stage, context: &mut ProcessContext) -> ProcessResult {
///         if !self.triggered_this_round && self.should_trigger(context) {
///             self.triggered_this_round = true;
///             // Return effect
///             ProcessResult {
///                 mult_added: 5.0,
///                 ..Default::default()
///             }
///         } else {
///             ProcessResult::default()
///         }
///     }
/// }
/// ```
///
/// ### Accumulating State
/// ```rust,ignore
/// struct AccumulatingJoker {
///     accumulated_value: f64,
/// }
///
/// impl JokerGameplay for AccumulatingJoker {
///     fn process(&mut self, stage: &Stage, context: &mut ProcessContext) -> ProcessResult {
///         self.accumulated_value += 1.0;
///         ProcessResult {
///             mult_added: self.accumulated_value,
///             ..Default::default()
///         }
///     }
/// }
/// ```
///
/// ## Thread Safety
///
/// For jokers that need thread-safe state sharing, use interior mutability patterns
/// like `Mutex`, `RwLock`, or atomic types. The joker itself must remain `Send + Sync`.
pub trait JokerGameplay: Send + Sync {
    /// Processes the joker's effect during the specified stage.
    fn process(&mut self, stage: &Stage, context: &mut ProcessContext) -> ProcessResult;

    /// Checks if this joker can trigger based on the current game state.
    fn can_trigger(&self, stage: &Stage, context: &ProcessContext) -> bool;

    /// Gets the order priority for this joker's processing (higher = earlier).
    fn get_priority(&self, _stage: &Stage) -> i32 {
        0
    }
}

/// Trait for joker modifiers and effects.
///
/// This trait handles modifiers that jokers can apply to scoring,
/// hand size, and other game mechanics.
pub trait JokerModifiers: Send + Sync {
    /// Returns the chip multiplier this joker provides.
    fn get_chip_mult(&self) -> f64 {
        1.0
    }

    /// Returns the score multiplier this joker provides.
    fn get_score_mult(&self) -> f64 {
        1.0
    }

    /// Returns the hand size modifier this joker provides.
    fn get_hand_size_modifier(&self) -> i32 {
        0
    }

    /// Returns the discard modifier this joker provides.
    fn get_discard_modifier(&self) -> i32 {
        0
    }
}

/// Trait for joker state management.
///
/// This trait handles the internal state of jokers, including
/// serialization and state queries.
pub trait JokerState: Send + Sync {
    /// Returns whether this joker has any internal state.
    fn has_state(&self) -> bool {
        false
    }

    /// Serializes the joker's state to a value.
    fn serialize_state(&self) -> Option<serde_json::Value> {
        None
    }

    /// Deserializes the joker's state from a value.
    fn deserialize_state(&mut self, _value: serde_json::Value) -> Result<(), String> {
        Ok(())
    }

    /// Returns a debug representation of the joker's current state.
    fn debug_state(&self) -> String {
        "{}".to_string()
    }

    /// Resets the joker's state to its initial values.
    fn reset_state(&mut self) {}
}

/// Rarity levels for jokers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Legendary,
}

/// Context provided to jokers during processing.
///
/// This struct provides access to game state that jokers may need during their
/// processing phase, such as current hand score, played cards, and game events.
///
/// # State Management
///
/// With the `process()` method now taking `&mut self`, jokers can maintain their
/// own internal state directly. The `joker_state_manager` field is retained for
/// backward compatibility during the migration period, but new jokers should
/// use internal state instead.
///
/// ## Example: Direct state management
/// ```rust,ignore
/// struct CounterJoker {
///     counter: u32,
/// }
///
/// impl JokerGameplay for CounterJoker {
///     fn process(&mut self, stage: &Stage, context: &mut ProcessContext) -> ProcessResult {
///         self.counter += 1;  // Direct state update
///         ProcessResult {
///             mult_added: self.counter as f64,
///             ..Default::default()
///         }
///     }
/// }
/// ```
pub struct ProcessContext<'a> {
    pub hand_score: &'a mut HandScore,
    pub played_cards: &'a [Card],
    pub held_cards: &'a [Card],
    pub events: &'a mut Vec<GameEvent>,
    pub hand: &'a SelectHand,
    pub joker_state_manager: &'a JokerStateManager,
}

/// Result returned from joker processing.
pub struct ProcessResult {
    pub chips_added: u64,
    pub mult_added: f64,
    pub mult_multiplier: f64,
    pub retriggered: bool,
    pub message: Option<String>,
}

impl Default for ProcessResult {
    fn default() -> Self {
        Self {
            chips_added: 0,
            mult_added: 0.0,
            mult_multiplier: 1.0,
            retriggered: false,
            message: None,
        }
    }
}
