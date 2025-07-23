//! # Joker Traits System
//!
//! This module defines the modern trait-based architecture for jokers, replacing the
//! monolithic 25-method `Joker` trait with 5 focused, single-responsibility traits.
//! This design follows the Interface Segregation Principle and enables better
//! maintainability, testability, and selective implementation.
//!
//! ## Architecture Overview
//!
//! The joker system is built around five core traits, each handling a specific aspect:
//!
//! * [`JokerIdentity`] - Core metadata and identification (6 methods)
//! * [`JokerLifecycle`] - Event hooks and state transitions (7 methods)  
//! * [`JokerGameplay`] - Core game interactions and scoring (3 methods)
//! * [`JokerModifiers`] - Passive game rule modifications (4 methods)
//! * [`JokerState`] - Internal state management and persistence (5 methods)
//!
//! ## Key Benefits
//!
//! ### Selective Implementation
//! Jokers can now implement only the traits they actually need:
//! ```rust
//! // Simple joker only needs identity and gameplay
//! struct SimpleJoker;
//! impl JokerIdentity for SimpleJoker { /* ... */ }
//! impl JokerGameplay for SimpleJoker { /* ... */ }
//! // Other traits get default implementations automatically
//! ```
//!
//! ### Better Testing
//! Each aspect can be tested in complete isolation:
//! ```rust
//! fn test_identity_only() {
//!     let joker = TestJoker;
//!     assert_eq!(joker.name(), "Expected Name");
//!     // No need to mock gameplay or state
//! }
//! ```
//!
//! ### Improved Maintainability
//! Changes to one aspect don't affect others, and trait boundaries are clear.
//!
//! ## Migration from Monolithic Trait
//!
//! The original `Joker` trait is now a super trait that requires all five focused traits:
//! ```rust
//! pub trait Joker: JokerIdentity + JokerLifecycle + JokerGameplay +
//!                  JokerModifiers + JokerState + Send + Sync + Debug {}
//! ```
//!
//! This provides complete backward compatibility - all existing code continues to work
//! unchanged while new code can use the focused traits directly.
//!
//! ## Implementation Patterns
//!
//! ### Minimal Joker (Most Common)
//! ```rust
//! #[derive(Debug, Clone)]
//! struct PlusMultJoker;
//!
//! impl JokerIdentity for PlusMultJoker {
//!     fn joker_type(&self) -> &'static str { "plus_mult" }
//!     fn name(&self) -> &str { "+4 Mult" }
//!     fn description(&self) -> &str { "Gives +4 Mult when hand is played" }
//!     fn rarity(&self) -> Rarity { Rarity::Common }
//!     fn base_cost(&self) -> u64 { 3 }
//! }
//!
//! impl JokerGameplay for PlusMultJoker {
//!     fn process(&self, stage: &Stage, context: &mut ProcessContext) -> ProcessResult {
//!         if matches!(stage, Stage::Scoring) {
//!             ProcessResult { chips_added: 0, mult_added: 4.0, retriggered: false }
//!         } else {
//!             ProcessResult::default()
//!         }
//!     }
//!
//!     fn can_trigger(&self, stage: &Stage, _context: &ProcessContext) -> bool {
//!         matches!(stage, Stage::Scoring)
//!     }
//! }
//!
//! // Default implementations for unused traits
//! impl JokerLifecycle for PlusMultJoker {}
//! impl JokerModifiers for PlusMultJoker {}  
//! impl JokerState for PlusMultJoker {}
//! impl Joker for PlusMultJoker {}
//! ```
//!
//! ### Stateful Joker
//! ```rust
//! #[derive(Debug, Clone)]
//! struct CountingJoker {
//!     hands_played: u32,
//! }
//!
//! impl JokerIdentity for CountingJoker {
//!     fn joker_type(&self) -> &'static str { "counting" }
//!     fn name(&self) -> &str { "Counting Joker" }
//!     fn description(&self) -> &str { "Gains +1 Mult per hand played" }
//!     fn rarity(&self) -> Rarity { Rarity::Uncommon }
//!     fn base_cost(&self) -> u64 { 5 }
//! }
//!
//! impl JokerGameplay for CountingJoker {
//!     fn process(&self, stage: &Stage, context: &mut ProcessContext) -> ProcessResult {
//!         if matches!(stage, Stage::Scoring) {
//!             ProcessResult {
//!                 chips_added: 0,
//!                 mult_added: self.hands_played as f64,
//!                 retriggered: false
//!             }
//!         } else {
//!             ProcessResult::default()
//!         }
//!     }
//!
//!     fn can_trigger(&self, stage: &Stage, _context: &ProcessContext) -> bool {
//!         matches!(stage, Stage::Scoring)
//!     }
//! }
//!
//! impl JokerLifecycle for CountingJoker {
//!     fn on_round_end(&mut self) {
//!         self.hands_played += 1;
//!     }
//! }
//!
//! impl JokerState for CountingJoker {
//!     fn has_state(&self) -> bool { true }
//!     
//!     fn serialize_state(&self) -> Option<serde_json::Value> {
//!         Some(serde_json::json!({ "hands_played": self.hands_played }))
//!     }
//!     
//!     fn deserialize_state(&mut self, value: serde_json::Value) -> Result<(), String> {
//!         self.hands_played = value["hands_played"].as_u64()
//!             .ok_or("Missing hands_played")? as u32;
//!         Ok(())
//!     }
//! }
//!
//! // Default implementations
//! impl JokerModifiers for CountingJoker {}
//! impl Joker for CountingJoker {}
//! ```

use crate::card::Card;
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

/// Core identity and metadata for jokers.
///
/// This trait defines the fundamental identifying characteristics of a joker,
/// including its unique type identifier, display information, and economic properties.
/// All jokers must implement this trait as it provides the basic information
/// needed by the game engine, UI, and shop systems.
///
/// ## Design Philosophy
///
/// `JokerIdentity` focuses purely on static, immutable properties that define
/// what a joker is, not what it does. This separation allows for clean
/// categorization, searching, and display without needing to instantiate
/// complex gameplay logic.
///
/// ## Implementation Guidelines
///
/// - **Unique Types**: Each joker variant should have a unique `joker_type()` string
/// - **Descriptive Names**: Names should be clear and match the in-game display
/// - **Accurate Descriptions**: Descriptions should explain the joker's effect concisely
/// - **Balanced Costs**: Base costs should reflect the joker's power level and rarity
///
/// ## Example Implementation
///
/// ```rust
/// use crate::joker::traits::{JokerIdentity, Rarity};
///
/// #[derive(Debug, Clone)]
/// struct DoubleJoker;
///
/// impl JokerIdentity for DoubleJoker {
///     fn joker_type(&self) -> &'static str {
///         "double"
///     }
///
///     fn name(&self) -> &str {
///         "Double"
///     }
///
///     fn description(&self) -> &str {
///         "Double all Mult"
///     }
///
///     fn rarity(&self) -> Rarity {
///         Rarity::Rare
///     }
///
///     fn base_cost(&self) -> u64 {
///         8  // Higher cost for rare, powerful joker
///     }
///
///     fn is_unique(&self) -> bool {
///         false  // Can have multiple copies
///     }
/// }
/// ```
///
/// ## Usage Patterns
///
/// ### Shop Integration
/// ```rust
/// // Shop system uses identity for pricing and display
/// let joker = DoubleJoker;
/// let shop_price = joker.base_cost() * shop_multiplier;
/// let display_text = format!("{}: {}", joker.name(), joker.description());
/// ```
///
/// ### Collection Management
/// ```rust
/// // Game can query joker properties for deck management
/// if joker.is_unique() && collection.has_joker(joker.joker_type()) {
///     return Err("Cannot add duplicate unique joker");
/// }
/// ```
///
/// ### Filtering and Search
/// ```rust
/// // Filter jokers by rarity
/// let rare_jokers: Vec<_> = collection.iter()
///     .filter(|j| matches!(j.rarity(), Rarity::Rare))
///     .collect();
/// ```
pub trait JokerIdentity: Send + Sync {
    /// Returns the unique type identifier for this joker.
    ///
    /// This string must be unique across all joker types and is used for:
    /// - Save/load serialization
    /// - Joker registry lookups
    /// - Collection duplicate detection (for unique jokers)
    /// - Debug logging and error reporting
    ///
    /// **Convention**: Use lowercase snake_case (e.g., "plus_mult", "fibonacci_joker")
    ///
    /// # Examples
    /// ```rust
    /// # use crate::joker::traits::JokerIdentity;
    /// # struct MyJoker;
    /// # impl JokerIdentity for MyJoker {
    /// #     fn joker_type(&self) -> &'static str { "my_special_joker" }
    /// #     fn name(&self) -> &str { "My Joker" }
    /// #     fn description(&self) -> &str { "Does something special" }
    /// #     fn rarity(&self) -> Rarity { Rarity::Common }
    /// #     fn base_cost(&self) -> u64 { 3 }
    /// # }
    /// let joker = MyJoker;
    /// assert_eq!(joker.joker_type(), "my_special_joker");
    /// ```
    fn joker_type(&self) -> &'static str;

    /// Returns the human-readable display name of this joker.
    ///
    /// This name appears in the UI, shop, and collection screens. It should be
    /// concise but descriptive enough for players to identify the joker's purpose.
    ///
    /// # Examples  
    /// ```rust
    /// # use crate::joker::traits::JokerIdentity;
    /// # struct TheJoker;
    /// # impl JokerIdentity for TheJoker {
    /// #     fn joker_type(&self) -> &'static str { "the_joker" }
    /// #     fn name(&self) -> &str { "Joker" }
    /// #     fn description(&self) -> &str { "+4 Mult" }
    /// #     fn rarity(&self) -> Rarity { Rarity::Common }
    /// #     fn base_cost(&self) -> u64 { 3 }
    /// # }
    /// let joker = TheJoker;
    /// assert_eq!(joker.name(), "Joker");
    /// ```
    fn name(&self) -> &str;

    /// Returns a concise description of what this joker does.
    ///
    /// This description should clearly explain the joker's effect in terms
    /// players can understand. It appears in tooltips and help text.
    ///
    /// **Guidelines**:
    /// - Start with the primary effect ("+4 Mult")
    /// - Include trigger conditions if relevant ("when hand contains a Pair")
    /// - Keep it under 60 characters when possible
    /// - Use consistent terminology with other jokers
    ///
    /// # Examples
    /// ```rust
    /// # use crate::joker::traits::JokerIdentity;
    /// # struct MultJoker;
    /// # impl JokerIdentity for MultJoker {
    /// #     fn joker_type(&self) -> &'static str { "mult" }
    /// #     fn name(&self) -> &str { "Mult" }
    /// #     fn description(&self) -> &str { "+4 Mult" }
    /// #     fn rarity(&self) -> Rarity { Rarity::Common }
    /// #     fn base_cost(&self) -> u64 { 3 }
    /// # }
    /// let joker = MultJoker;
    /// assert_eq!(joker.description(), "+4 Mult");
    /// ```
    fn description(&self) -> &str;

    /// Returns the rarity tier of this joker.
    ///
    /// Rarity affects:
    /// - Appearance rate in shops
    /// - Default pricing (see `base_cost()`)
    /// - Visual styling and effects
    /// - Collection ordering
    ///
    /// # Rarity Guidelines
    /// - **Common**: Basic effects, widely available
    /// - **Uncommon**: More powerful or situational effects  
    /// - **Rare**: Significant game-changing effects
    /// - **Legendary**: Unique, build-defining effects
    fn rarity(&self) -> Rarity;

    /// Returns the base cost of this joker in the shop.
    ///
    /// This is the default price before any shop modifiers, discounts, or
    /// other pricing effects are applied. The actual purchase price may vary
    /// based on game state.
    ///
    /// **Pricing Guidelines**:
    /// - Common: 2-4 coins
    /// - Uncommon: 5-7 coins  
    /// - Rare: 8-12 coins
    /// - Legendary: 15+ coins
    ///
    /// # Examples
    /// ```rust
    /// # use crate::joker::traits::{JokerIdentity, Rarity};
    /// # struct ExpensiveJoker;
    /// # impl JokerIdentity for ExpensiveJoker {
    /// #     fn joker_type(&self) -> &'static str { "expensive" }
    /// #     fn name(&self) -> &str { "Expensive Joker" }
    /// #     fn description(&self) -> &str { "Very powerful effect" }
    /// #     fn rarity(&self) -> Rarity { Rarity::Legendary }
    /// #     fn base_cost(&self) -> u64 { 20 }
    /// # }
    /// let joker = ExpensiveJoker;
    /// assert_eq!(joker.base_cost(), 20);
    /// ```
    fn base_cost(&self) -> u64;

    /// Returns whether this joker is unique (limit one per collection).
    ///
    /// Unique jokers cannot be purchased if the player already owns one.
    /// This is typically used for legendary jokers with build-defining effects
    /// that would be overpowered if stacked.
    ///
    /// **Default**: `false` (non-unique, can have multiples)
    ///
    /// # Examples
    /// ```rust
    /// # use crate::joker::traits::JokerIdentity;
    /// # struct UniqueJoker;
    /// # impl JokerIdentity for UniqueJoker {
    /// #     fn joker_type(&self) -> &'static str { "unique" }
    /// #     fn name(&self) -> &str { "Unique Joker" }
    /// #     fn description(&self) -> &str { "One of a kind" }
    /// #     fn rarity(&self) -> Rarity { Rarity::Legendary }
    /// #     fn base_cost(&self) -> u64 { 25 }
    /// #     fn is_unique(&self) -> bool { true }
    /// # }
    /// let joker = UniqueJoker;
    /// assert!(joker.is_unique());
    /// ```
    fn is_unique(&self) -> bool {
        false
    }
}

/// Lifecycle event hooks for jokers.
///
/// This trait defines hooks for major lifecycle events in a joker's existence,
/// from initial purchase through ongoing gameplay to eventual sale or destruction.
/// Most simple jokers can use the default implementations, while complex jokers
/// can override specific events to implement state changes or side effects.
///
/// ## Event Ordering
///
/// The lifecycle events follow this typical order:
/// 1. `on_purchase()` - When bought from shop
/// 2. `on_round_start()` / `on_round_end()` - Every round
/// 3. `on_joker_added()` / `on_joker_removed()` - When collection changes
/// 4. `on_sell()` or `on_destroy()` - When removed from collection
///
/// ## Design Philosophy
///
/// Lifecycle hooks follow a "notification" pattern - they inform jokers about
/// state changes but don't return results. This keeps the lifecycle system
/// simple and predictable. For gameplay effects that return results, use
/// the [`JokerGameplay`] trait instead.
///
/// ## Implementation Guidelines
///
/// - **Stateless Jokers**: Use default implementations (no overrides needed)
/// - **State Tracking**: Override events where state changes occur
/// - **Side Effects**: Keep side effects minimal and predictable
/// - **Error Handling**: Lifecycle methods should not panic or fail
///
/// ## Example Implementations
///
/// ### Stateless Joker (Default)
/// ```rust
/// use crate::joker::traits::JokerLifecycle;
///
/// #[derive(Debug, Clone)]
/// struct SimpleJoker;
///
/// // Uses all default implementations - no overrides needed
/// impl JokerLifecycle for SimpleJoker {}
/// ```
///
/// ### Counter Joker (State Tracking)
/// ```rust
/// use crate::joker::traits::JokerLifecycle;
///
/// #[derive(Debug, Clone)]
/// struct CounterJoker {
///     rounds_survived: u32,
/// }
///
/// impl JokerLifecycle for CounterJoker {
///     fn on_purchase(&mut self) {
///         self.rounds_survived = 0;
///         println!("Counter joker purchased!");
///     }
///
///     fn on_round_end(&mut self) {
///         self.rounds_survived += 1;
///         println!("Round {} completed", self.rounds_survived);
///     }
///
///     fn on_sell(&mut self) {
///         println!("Sold after {} rounds", self.rounds_survived);
///     }
/// }
/// ```
///
/// ### Social Joker (Collection Awareness)
/// ```rust
/// use crate::joker::traits::JokerLifecycle;
///
/// #[derive(Debug, Clone)]
/// struct SocialJoker {
///     joker_friends: Vec<String>,
/// }
///
/// impl JokerLifecycle for SocialJoker {
///     fn on_joker_added(&mut self, other_joker_type: &str) {
///         self.joker_friends.push(other_joker_type.to_string());
///         println!("Made friends with {}", other_joker_type);
///     }
///
///     fn on_joker_removed(&mut self, other_joker_type: &str) {
///         self.joker_friends.retain(|t| t != other_joker_type);
///         println!("Lost friend {}", other_joker_type);
///     }
/// }
/// ```
///
/// ## Common Patterns
///
/// ### Initialization
/// ```rust
/// fn on_purchase(&mut self) {
///     // Initialize counters, states, or resources
///     self.times_triggered = 0;
///     self.bonus_accumulated = 0.0;
/// }
/// ```
///
/// ### Resource Management
/// ```rust
/// fn on_destroy(&mut self) {
///     // Clean up resources, log final state
///     println!("Joker destroyed with {} triggers", self.times_triggered);
/// }
/// ```
///
/// ### Dynamic Behavior
/// ```rust
/// fn on_round_start(&mut self) {
///     // Reset per-round state
///     self.triggered_this_round = false;
/// }
/// ```
pub trait JokerLifecycle: Send + Sync {
    /// Called when the joker is purchased from the shop.
    ///
    /// This is the first event in a joker's lifecycle, triggered immediately
    /// after the purchase transaction completes. Use this hook to:
    /// - Initialize internal state variables
    /// - Set up resources or data structures
    /// - Log purchase events
    /// - Trigger one-time setup effects
    ///
    /// **Default**: No-op (most jokers don't need purchase logic)
    ///
    /// # Example
    /// ```rust
    /// # use crate::joker::traits::JokerLifecycle;
    /// # struct TrackingJoker { purchase_time: u64 }
    /// # impl JokerLifecycle for TrackingJoker {
    /// fn on_purchase(&mut self) {
    ///     self.purchase_time = get_current_timestamp();
    ///     println!("Purchased tracking joker at {}", self.purchase_time);
    /// }
    /// # }
    /// ```
    fn on_purchase(&mut self) {}

    /// Called when the joker is sold back to the shop.
    ///
    /// This event occurs when the player voluntarily sells the joker for coins.
    /// Use this hook to:
    /// - Clean up resources
    /// - Log final statistics
    /// - Apply sell-triggered effects
    /// - Save performance metrics
    ///
    /// **Default**: No-op (most jokers don't need sell logic)
    ///
    /// # Example  
    /// ```rust
    /// # use crate::joker::traits::JokerLifecycle;
    /// # struct StatsJoker { hands_played: u32 }
    /// # impl JokerLifecycle for StatsJoker {
    /// fn on_sell(&mut self) {
    ///     println!("Sold joker after {} hands", self.hands_played);
    ///     // Could trigger bonus effect based on hands_played
    /// }
    /// # }
    /// ```
    fn on_sell(&mut self) {}

    /// Called when the joker is destroyed (removed without selling).
    ///
    /// This event occurs when the joker is removed through game effects
    /// (e.g., certain cards or penalties) rather than voluntary sale.
    /// Use this hook for cleanup and final effects.
    ///
    /// **Default**: No-op (most jokers don't need destruction logic)
    ///
    /// # Example
    /// ```rust
    /// # use crate::joker::traits::JokerLifecycle;
    /// # struct ExplodingJoker;
    /// # impl JokerLifecycle for ExplodingJoker {
    /// fn on_destroy(&mut self) {
    ///     println!("Joker destroyed - triggering explosion effect!");
    ///     // Could apply damage or other effects to remaining jokers
    /// }
    /// # }
    /// ```
    fn on_destroy(&mut self) {}

    /// Called at the start of each round (before cards are dealt).
    ///
    /// This is one of the most commonly used lifecycle hooks. It occurs
    /// before any hand is played in the round. Use this to:
    /// - Reset per-round state variables
    /// - Apply round-start bonuses
    /// - Update counters or trackers
    /// - Prepare for the new round's gameplay
    ///
    /// **Default**: No-op (stateless jokers don't need round tracking)
    ///
    /// # Example
    /// ```rust
    /// # use crate::joker::traits::JokerLifecycle;
    /// # struct RoundTracker { hands_this_round: u32 }
    /// # impl JokerLifecycle for RoundTracker {
    /// fn on_round_start(&mut self) {
    ///     self.hands_this_round = 0;
    ///     println!("New round started - resetting hand counter");
    /// }
    /// # }
    /// ```
    fn on_round_start(&mut self) {}

    /// Called at the end of each round (after scoring is complete).
    ///
    /// This hook is called after all hands have been played and scored
    /// for the round. Use this to:
    /// - Update persistent counters
    /// - Apply end-of-round bonuses
    /// - Analyze round performance
    /// - Persist state changes
    ///
    /// **Default**: No-op (stateless jokers don't need round tracking)
    ///
    /// # Example
    /// ```rust
    /// # use crate::joker::traits::JokerLifecycle;
    /// # struct GrowthJoker { rounds_completed: u32, bonus_mult: f64 }
    /// # impl JokerLifecycle for GrowthJoker {
    /// fn on_round_end(&mut self) {
    ///     self.rounds_completed += 1;
    ///     self.bonus_mult += 0.5;  // Grow stronger each round
    ///     println!("Round {} complete, bonus now {}",
    ///              self.rounds_completed, self.bonus_mult);
    /// }
    /// # }
    /// ```
    fn on_round_end(&mut self) {}

    /// Called when another joker is added to the collection.
    ///
    /// This event allows jokers to react to changes in the joker collection.
    /// The `other_joker_type` parameter is the type identifier of the newly
    /// added joker. Use this for:
    /// - Synergy effects with specific joker types
    /// - Collection-size-based bonuses
    /// - Tracking joker diversity
    ///
    /// **Default**: No-op (most jokers don't care about collection changes)
    ///
    /// # Parameters
    /// - `other_joker_type`: The `joker_type()` string of the added joker
    ///
    /// # Example
    /// ```rust
    /// # use crate::joker::traits::JokerLifecycle;
    /// # struct SynergyJoker { mult_buddies: u32 }
    /// # impl JokerLifecycle for SynergyJoker {
    /// fn on_joker_added(&mut self, other_joker_type: &str) {
    ///     if other_joker_type.contains("mult") {
    ///         self.mult_buddies += 1;
    ///         println!("Found mult buddy! Now have {}", self.mult_buddies);
    ///     }
    /// }
    /// # }
    /// ```
    fn on_joker_added(&mut self, _other_joker_type: &str) {}

    /// Called when another joker is removed from the collection.
    ///
    /// This event allows jokers to react when other jokers leave the collection
    /// (through sale, destruction, or other removal). Use this for:
    /// - Updating synergy tracking
    /// - Adjusting collection-based bonuses
    /// - Mourning lost joker friends
    ///
    /// **Default**: No-op (most jokers don't care about collection changes)
    ///
    /// # Parameters
    /// - `other_joker_type`: The `joker_type()` string of the removed joker
    ///
    /// # Example
    /// ```rust
    /// # use crate::joker::traits::JokerLifecycle;
    /// # struct LoyalJoker { departed_friends: u32, sadness_bonus: f64 }
    /// # impl JokerLifecycle for LoyalJoker {
    /// fn on_joker_removed(&mut self, other_joker_type: &str) {
    ///     self.departed_friends += 1;
    ///     self.sadness_bonus += 1.0;  // Gets stronger when alone
    ///     println!("Lost friend {} - sadness bonus now {}",
    ///              other_joker_type, self.sadness_bonus);
    /// }
    /// # }
    /// ```
    fn on_joker_removed(&mut self, _other_joker_type: &str) {}
}

/// Core gameplay interactions and scoring logic for jokers.
///
/// This trait defines the heart of joker functionality - how jokers interact
/// with the game during different stages to modify scoring, trigger effects,
/// and influence gameplay. This is the most performance-critical trait as it's
/// called frequently during scoring calculations.
///
/// ## Design Philosophy
///
/// `JokerGameplay` follows a stage-based processing model where jokers can
/// react to different game stages (dealing, playing, scoring, etc.). The trait
/// provides both effect processing (`process`) and conditional logic (`can_trigger`)
/// to enable complex, context-aware joker behavior.
///
/// ## Processing Stages
///
/// Jokers are processed during various game stages:
/// - `Stage::Dealing` - When cards are being dealt
/// - `Stage::Playing` - When player selects and plays cards
/// - `Stage::Scoring` - When calculating hand score (most common)
/// - `Stage::Discarding` - When cards are discarded
/// - Custom stages defined by the game
///
/// ## Performance Considerations
///
/// Since `process()` is called frequently during gameplay:
/// - Keep calculations lightweight and cache-friendly
/// - Use `can_trigger()` to short-circuit expensive processing
/// - Avoid allocations in hot paths
/// - Consider using `get_priority()` to optimize processing order
///
/// ## Implementation Guidelines
///
/// ### Simple Scoring Jokers (Most Common)
/// ```rust
/// use crate::joker::traits::{JokerGameplay, ProcessContext, ProcessResult};
/// use crate::stage::Stage;
///
/// #[derive(Debug, Clone)]
/// struct BasicMultJoker;
///
/// impl JokerGameplay for BasicMultJoker {
///     fn process(&self, stage: &Stage, _context: &mut ProcessContext) -> ProcessResult {
///         if matches!(stage, Stage::Scoring) {
///             ProcessResult {
///                 chips_added: 0,
///                 mult_added: 4.0,
///                 retriggered: false,
///             }
///         } else {
///             ProcessResult::default()
///         }
///     }
///
///     fn can_trigger(&self, stage: &Stage, _context: &ProcessContext) -> bool {
///         matches!(stage, Stage::Scoring)
///     }
/// }
/// ```
///
/// ### Conditional Jokers
/// ```rust
/// # use crate::joker::traits::{JokerGameplay, ProcessContext, ProcessResult};
/// # use crate::stage::Stage;
/// #[derive(Debug, Clone)]
/// struct PairBonusJoker;
///
/// impl JokerGameplay for PairBonusJoker {
///     fn process(&self, stage: &Stage, context: &mut ProcessContext) -> ProcessResult {
///         if !matches!(stage, Stage::Scoring) {
///             return ProcessResult::default();
///         }
///
///         // Check if played hand contains a pair
///         if self.hand_contains_pair(context.played_cards) {
///             ProcessResult {
///                 chips_added: 30,
///                 mult_added: 3.0,
///                 retriggered: false,
///             }
///         } else {
///             ProcessResult::default()
///         }
///     }
///
///     fn can_trigger(&self, stage: &Stage, context: &ProcessContext) -> bool {
///         matches!(stage, Stage::Scoring) && self.hand_contains_pair(context.played_cards)
///     }
/// }
///
/// impl PairBonusJoker {
///     fn hand_contains_pair(&self, cards: &[Card]) -> bool {
///         // Implementation to detect pairs
///         false // Simplified for example
///     }
/// }
/// ```
///
/// ### Complex State-Based Jokers
/// ```rust
/// # use crate::joker::traits::{JokerGameplay, ProcessContext, ProcessResult};
/// # use crate::stage::Stage;
/// # use std::cell::Cell;
/// #[derive(Debug)]
/// struct BuildingJoker {
///     times_triggered: Cell<u32>,
/// }
///
/// impl BuildingJoker {
///     fn new() -> Self {
///         Self { times_triggered: Cell::new(0) }
///     }
/// }
///
/// impl JokerGameplay for BuildingJoker {
///     fn process(&self, stage: &Stage, _context: &mut ProcessContext) -> ProcessResult {
///         if !matches!(stage, Stage::Scoring) {
///             return ProcessResult::default();
///         }
///
///         let current = self.times_triggered.get();
///         self.times_triggered.set(current + 1);
///         let bonus = current as f64 * 0.5; // Grows stronger over time
///
///         ProcessResult {
///             chips_added: 0,
///             mult_added: bonus,
///             retriggered: false,
///         }
///     }
///
///     fn can_trigger(&self, stage: &Stage, _context: &ProcessContext) -> bool {
///         matches!(stage, Stage::Scoring)
///     }
///
///     fn get_priority(&self, _stage: &Stage) -> i32 {
///         // Higher priority for stronger jokers
///         self.times_triggered.get() as i32
///     }
/// }
/// ```
///
/// ## Common Patterns
///
/// ### Stage-Specific Processing
/// ```rust
/// fn process(&self, stage: &Stage, context: &mut ProcessContext) -> ProcessResult {
///     match stage {
///         Stage::Scoring => {
///             // Main scoring effect
///             ProcessResult { mult_added: 5.0, ..Default::default() }
///         },
///         Stage::Discarding => {
///             // Bonus for discarding
///             context.hand_score.chips += 10;
///             ProcessResult::default()
///         },
///         _ => ProcessResult::default()
///     }
/// }
/// ```
///
/// ### Conditional Triggering
/// ```rust
/// fn can_trigger(&self, stage: &Stage, context: &ProcessContext) -> bool {
///     matches!(stage, Stage::Scoring)
///         && !context.played_cards.is_empty()
///         && self.some_internal_condition()
/// }
/// ```
///
/// ### Performance Optimization
/// ```rust
/// fn get_priority(&self, stage: &Stage) -> i32 {
///     match stage {
///         Stage::Scoring => 100,  // High priority for scoring
///         Stage::Playing => 50,   // Medium priority for play effects
///         _ => 0                  // Default priority otherwise
///     }
/// }
/// ```
pub trait JokerGameplay: Send + Sync {
    /// Processes the joker's effect during the specified game stage.
    ///
    /// This is the core method where jokers implement their gameplay effects.
    /// Called by the game engine during various stages, this method should:
    ///
    /// 1. Check if the joker should activate for the given stage
    /// 2. Apply the joker's effect (modify scoring, trigger events, etc.)
    /// 3. Return the appropriate `ProcessResult` describing what happened
    ///
    /// **Performance Note**: This method is called frequently during gameplay.
    /// Use `can_trigger()` for expensive condition checks and keep this method fast.
    ///
    /// # Parameters
    /// - `stage`: Current game stage (Scoring, Playing, Dealing, etc.)
    /// - `context`: Mutable game context with scoring info and card data
    ///
    /// # Returns
    /// `ProcessResult` with chips/mult added and whether cards were retriggered
    ///
    /// # Example
    /// ```rust
    /// # use crate::joker::traits::{JokerGameplay, ProcessContext, ProcessResult};
    /// # use crate::stage::Stage;
    /// # struct MyJoker;
    /// # impl JokerGameplay for MyJoker {
    /// fn process(&self, stage: &Stage, context: &mut ProcessContext) -> ProcessResult {
    ///     match stage {
    ///         Stage::Scoring => ProcessResult {
    ///             chips_added: 20,
    ///             mult_added: 2.0,
    ///             retriggered: false,
    ///         },
    ///         _ => ProcessResult::default(), // No effect for other stages
    ///     }
    /// }
    /// # fn can_trigger(&self, stage: &Stage, context: &ProcessContext) -> bool { true }
    /// # }
    /// ```
    fn process(&self, stage: &Stage, context: &mut ProcessContext) -> ProcessResult;

    /// Checks if this joker can trigger for the current stage and context.
    ///
    /// This method provides a fast check to determine if the joker should
    /// process during this stage. The game engine may use this for:
    /// - Performance optimization (skip inactive jokers)
    /// - UI indications (highlight active jokers)
    /// - Processing order optimization
    ///
    /// **Implementation Note**: This should be a fast, side-effect-free check.
    /// Expensive computations belong in `process()`, not here.
    ///
    /// # Parameters  
    /// - `stage`: Current game stage
    /// - `context`: Read-only game context for condition checking
    ///
    /// # Returns
    /// `true` if the joker should be processed, `false` otherwise
    ///
    /// # Example
    /// ```rust
    /// # use crate::joker::traits::{JokerGameplay, ProcessContext, ProcessResult};
    /// # use crate::stage::Stage;
    /// # struct ConditionalJoker { enabled: bool }
    /// # impl JokerGameplay for ConditionalJoker {
    /// fn can_trigger(&self, stage: &Stage, context: &ProcessContext) -> bool {
    ///     // Only trigger during scoring if enabled and cards were played
    ///     matches!(stage, Stage::Scoring)
    ///         && self.enabled
    ///         && !context.played_cards.is_empty()
    /// }
    /// # fn process(&self, stage: &Stage, context: &mut ProcessContext) -> ProcessResult {
    /// #     ProcessResult::default()
    /// # }
    /// # }
    /// ```
    fn can_trigger(&self, stage: &Stage, context: &ProcessContext) -> bool;

    /// Gets the processing priority for this joker in the given stage.
    ///
    /// Higher priority jokers are processed first. This is useful for:
    /// - Ensuring certain effects happen before others
    /// - Optimizing performance by processing high-impact jokers early
    /// - Creating predictable interaction patterns
    ///
    /// **Default**: `0` (neutral priority)
    ///
    /// **Common Patterns**:
    /// - Multiplier jokers: High priority (100+)
    /// - Additive bonuses: Medium priority (50-99)  
    /// - Conditional effects: Variable priority based on strength
    /// - Cosmetic effects: Low priority (negative values)
    ///
    /// # Parameters
    /// - `stage`: The stage for which priority is being queried
    ///
    /// # Returns
    /// Priority value (higher = processed earlier)
    ///
    /// # Example
    /// ```rust
    /// # use crate::joker::traits::{JokerGameplay, ProcessContext, ProcessResult};
    /// # use crate::stage::Stage;
    /// # struct PowerfulJoker { strength: u32 }
    /// # impl JokerGameplay for PowerfulJoker {
    /// fn get_priority(&self, stage: &Stage) -> i32 {
    ///     match stage {
    ///         Stage::Scoring => 100 + self.strength as i32, // Higher for stronger jokers
    ///         Stage::Dealing => 50,                          // Medium for dealing effects
    ///         _ => 0,                                        // Default for others
    ///     }
    /// }
    /// # fn process(&self, stage: &Stage, context: &mut ProcessContext) -> ProcessResult {
    /// #     ProcessResult::default()
    /// # }
    /// # fn can_trigger(&self, stage: &Stage, context: &ProcessContext) -> bool { true }
    /// # }
    /// ```
    fn get_priority(&self, _stage: &Stage) -> i32 {
        0
    }
}

/// Passive game rule modifications applied by jokers.
///
/// This trait handles permanent, passive modifications that jokers apply
/// to core game mechanics like scoring multipliers, hand size, and discard limits.
/// Unlike `JokerGameplay` which handles active effects during specific stages,
/// `JokerModifiers` provides constant rule changes that are always active.
///
/// ## Design Philosophy
///
/// Modifiers follow a "multiplicative stacking" approach where multiple jokers
/// with similar modifiers combine their effects. The base game queries all
/// jokers for their modifier values and applies them in sequence.
///
/// ## Common Modifier Types
///
/// - **Scoring Multipliers**: Multiply chip or mult values
/// - **Game Rule Changes**: Modify hand size or discard limits  
/// - **Economic Effects**: Affect shop prices or sell values
/// - **Mechanical Changes**: Alter deck size, card draw, etc.
///
/// ## Implementation Examples
///
/// ### Basic Multiplier Joker
/// ```rust
/// use crate::joker::traits::JokerModifiers;
///
/// #[derive(Debug, Clone)]
/// struct DoubleMultJoker;
///
/// impl JokerModifiers for DoubleMultJoker {
///     fn get_score_mult(&self) -> f64 {
///         2.0  // Doubles all scoring
///     }
///     
///     // Other modifiers use defaults
/// }
/// ```
///
/// ### Hand Size Modifier
/// ```rust
/// # use crate::joker::traits::JokerModifiers;
/// #[derive(Debug, Clone)]
/// struct BigHandJoker;
///
/// impl JokerModifiers for BigHandJoker {
///     fn get_hand_size_modifier(&self) -> i32 {
///         2  // Increases hand size by 2
///     }
/// }
/// ```
///
/// ### Multi-Effect Joker
/// ```rust
/// # use crate::joker::traits::JokerModifiers;
/// #[derive(Debug, Clone)]
/// struct PowerJoker;
///
/// impl JokerModifiers for PowerJoker {
///     fn get_chip_mult(&self) -> f64 {
///         1.5  // 50% more chips
///     }
///     
///     fn get_score_mult(&self) -> f64 {
///         1.25  // 25% score boost
///     }
///     
///     fn get_hand_size_modifier(&self) -> i32 {
///         1  // +1 hand size
///     }
/// }
/// ```
pub trait JokerModifiers: Send + Sync {
    /// Returns the chip multiplier this joker provides.
    ///
    /// This multiplier is applied to the base chip value during scoring.
    /// Multiple jokers with chip multipliers stack multiplicatively.
    ///
    /// **Default**: `1.0` (no modification)
    /// **Common values**: `1.5` (+50%), `2.0` (double), `0.5` (half)
    ///
    /// # Example
    /// ```rust
    /// # use crate::joker::traits::JokerModifiers;
    /// # struct ChipBoostJoker;
    /// # impl JokerModifiers for ChipBoostJoker {
    /// fn get_chip_mult(&self) -> f64 {
    ///     1.5  // 50% more chips
    /// }
    /// # }
    /// ```
    fn get_chip_mult(&self) -> f64 {
        1.0
    }

    /// Returns the score multiplier this joker provides.
    ///
    /// This multiplier is applied to the final calculated score.
    /// It's the most powerful type of modifier as it affects the complete score.
    ///
    /// **Default**: `1.0` (no modification)
    /// **Common values**: `1.25` (+25%), `2.0` (double), `3.0` (triple)
    ///
    /// # Example
    /// ```rust
    /// # use crate::joker::traits::JokerModifiers;
    /// # struct ScoreBoostJoker;
    /// # impl JokerModifiers for ScoreBoostJoker {
    /// fn get_score_mult(&self) -> f64 {
    ///     2.0  // Double all scoring
    /// }
    /// # }
    /// ```
    fn get_score_mult(&self) -> f64 {
        1.0
    }

    /// Returns the hand size modifier this joker provides.
    ///
    /// This value is added to the base hand size, allowing players to
    /// hold and play more cards. Positive values increase hand size,
    /// negative values decrease it.
    ///
    /// **Default**: `0` (no modification)
    /// **Common values**: `+1`, `+2`, `+5` for beneficial effects; `-1`, `-2` for penalties
    ///
    /// # Example
    /// ```rust
    /// # use crate::joker::traits::JokerModifiers;
    /// # struct BigHandJoker;
    /// # impl JokerModifiers for BigHandJoker {
    /// fn get_hand_size_modifier(&self) -> i32 {
    ///     3  // +3 hand size
    /// }
    /// # }
    /// ```
    fn get_hand_size_modifier(&self) -> i32 {
        0
    }

    /// Returns the discard limit modifier this joker provides.
    ///
    /// This value is added to the base number of discards allowed per round.
    /// More discards provide greater flexibility in hand management.
    ///
    /// **Default**: `0` (no modification)
    /// **Common values**: `+1`, `+2` for extra discards; `-1` for limitations
    ///
    /// # Example
    /// ```rust
    /// # use crate::joker::traits::JokerModifiers;
    /// # struct ExtraDiscardJoker;
    /// # impl JokerModifiers for ExtraDiscardJoker {
    /// fn get_discard_modifier(&self) -> i32 {
    ///     2  // +2 extra discards per round
    /// }
    /// # }
    /// ```
    fn get_discard_modifier(&self) -> i32 {
        0
    }
}

/// Internal state management and persistence for jokers.
///
/// This trait handles jokers that need to maintain internal state across
/// gameplay sessions. It provides serialization, deserialization, and state
/// validation capabilities for persistent joker data. Most simple jokers don't
/// need state and can use the default implementations.
///
/// ## When to Use State
///
/// Jokers need state when they:
/// - Track counters or statistics over time
/// - Remember previous game events
/// - Build up power or effects gradually
/// - Need to persist data across save/load cycles
///
/// ## State Design Guidelines
///
/// - **Keep state minimal**: Only store what's absolutely necessary
/// - **Use standard types**: Prefer basic types (numbers, strings) for serialization
/// - **Validate state**: Check deserialized state for corruption or invalid values
/// - **Handle migrations**: Support loading state from older game versions
///
/// ## Implementation Examples
///
/// ### Stateless Joker (Default)
/// ```rust
/// use crate::joker::traits::JokerState;
///
/// #[derive(Debug, Clone)]
/// struct SimpleJoker;
///
/// // Uses all default implementations - no custom state needed
/// impl JokerState for SimpleJoker {}
/// ```
///
/// ### Counter-Based Joker
/// ```rust
/// # use crate::joker::traits::JokerState;
/// # use serde_json::Value;
/// #[derive(Debug, Clone)]
/// struct CountingJoker {
///     times_triggered: u32,
///     total_bonus: f64,
/// }
///
/// impl JokerState for CountingJoker {
///     fn has_state(&self) -> bool {
///         true
///     }
///
///     fn serialize_state(&self) -> Option<Value> {
///         Some(serde_json::json!({
///             "times_triggered": self.times_triggered,
///             "total_bonus": self.total_bonus
///         }))
///     }
///
///     fn deserialize_state(&mut self, value: Value) -> Result<(), String> {
///         self.times_triggered = value["times_triggered"]
///             .as_u64().ok_or("Invalid times_triggered")? as u32;
///         self.total_bonus = value["total_bonus"]
///             .as_f64().ok_or("Invalid total_bonus")?;
///         Ok(())
///     }
///
///     fn debug_state(&self) -> String {
///         format!("triggers: {}, bonus: {:.1}", self.times_triggered, self.total_bonus)
///     }
///
///     fn reset_state(&mut self) {
///         self.times_triggered = 0;
///         self.total_bonus = 0.0;
///     }
/// }
/// ```
///
/// ### Complex State with Validation
/// ```rust
/// # use crate::joker::traits::JokerState;
/// # use serde_json::Value;
/// #[derive(Debug, Clone)]
/// struct ComplexJoker {
///     level: u32,
///     experience: u32,
///     abilities: Vec<String>,
/// }
///
/// impl JokerState for ComplexJoker {
///     fn has_state(&self) -> bool { true }
///
///     fn serialize_state(&self) -> Option<Value> {
///         Some(serde_json::json!({
///             "level": self.level,
///             "experience": self.experience,
///             "abilities": self.abilities,
///             "version": 1  // Include version for future migrations
///         }))
///     }
///
///     fn deserialize_state(&mut self, value: Value) -> Result<(), String> {
///         // Handle version migrations
///         let version = value["version"].as_u64().unwrap_or(0);
///         
///         self.level = value["level"].as_u64().ok_or("Missing level")? as u32;
///         self.experience = value["experience"].as_u64().ok_or("Missing experience")? as u32;
///         
///         if let Some(abilities) = value["abilities"].as_array() {
///             self.abilities = abilities.iter()
///                 .filter_map(|v| v.as_str().map(|s| s.to_string()))
///                 .collect();
///         }
///         
///         // Validate state after deserialization
///         if self.level > 100 {
///             return Err("Invalid level: too high".to_string());
///         }
///         
///         Ok(())
///     }
/// }
/// ```
pub trait JokerState: Send + Sync {
    /// Returns whether this joker has any internal state to manage.
    ///
    /// This is a performance optimization that allows the game engine to
    /// skip state-related operations for stateless jokers. Return `true`
    /// only if the joker actually needs to serialize/deserialize data.
    ///
    /// **Default**: `false` (no state)
    ///
    /// # Example
    /// ```rust
    /// # use crate::joker::traits::JokerState;
    /// # struct StatefulJoker { counter: u32 }
    /// # impl JokerState for StatefulJoker {
    /// fn has_state(&self) -> bool {
    ///     true  // This joker tracks internal counters
    /// }
    /// # }
    /// ```
    fn has_state(&self) -> bool {
        false
    }

    /// Serializes the joker's state to a JSON value for persistence.
    ///
    /// Called when the game needs to save the joker's state (during save
    /// operations, checkpoints, etc.). Return `None` if the joker has no
    /// state to serialize, or `Some(Value)` containing the state data.
    ///
    /// **Default**: `None` (no state to serialize)
    ///
    /// # Returns
    /// - `Some(Value)`: JSON-serializable state data
    /// - `None`: No state to serialize
    ///
    /// # Example
    /// ```rust
    /// # use crate::joker::traits::JokerState;
    /// # use serde_json::{Value, json};
    /// # struct CounterJoker { count: u32, bonus: f64 }
    /// # impl JokerState for CounterJoker {
    /// #   fn has_state(&self) -> bool { true }
    /// fn serialize_state(&self) -> Option<Value> {
    ///     Some(json!({
    ///         "count": self.count,
    ///         "bonus": self.bonus,
    ///         "version": 1  // Include version for future compatibility
    ///     }))
    /// }
    /// # }
    /// ```
    fn serialize_state(&self) -> Option<serde_json::Value> {
        None
    }

    /// Deserializes the joker's state from a JSON value.
    ///
    /// Called when loading saved games to restore the joker's internal state.
    /// The method should extract the relevant data from the JSON value and
    /// update the joker's internal fields. Validate the data and return
    /// an error if the state is corrupted or invalid.
    ///
    /// **Default**: No-op (always succeeds)
    ///
    /// # Parameters
    /// - `value`: JSON value containing the serialized state
    ///
    /// # Returns
    /// - `Ok(())`: State loaded successfully
    /// - `Err(String)`: Error message if loading failed
    ///
    /// # Example
    /// ```rust
    /// # use crate::joker::traits::JokerState;
    /// # use serde_json::Value;
    /// # struct CounterJoker { count: u32, bonus: f64 }
    /// # impl JokerState for CounterJoker {
    /// fn deserialize_state(&mut self, value: Value) -> Result<(), String> {
    ///     // Extract and validate count
    ///     self.count = value["count"]
    ///         .as_u64()
    ///         .ok_or("Missing or invalid count field")?
    ///         as u32;
    ///     
    ///     // Extract and validate bonus  
    ///     self.bonus = value["bonus"]
    ///         .as_f64()
    ///         .ok_or("Missing or invalid bonus field")?;
    ///     
    ///     // Validate ranges
    ///     if self.count > 1000 {
    ///         return Err("Count too high".to_string());
    ///     }
    ///     
    ///     Ok(())
    /// }
    /// # }
    /// ```
    fn deserialize_state(&mut self, _value: serde_json::Value) -> Result<(), String> {
        Ok(())
    }

    /// Returns a human-readable debug representation of the current state.
    ///
    /// Used for debugging, logging, and development tools. Should provide
    /// a concise but informative view of the joker's internal state.
    ///
    /// **Default**: `"{}"` (empty object notation)
    ///
    /// # Returns
    /// String describing the current state
    ///
    /// # Example
    /// ```rust
    /// # use crate::joker::traits::JokerState;
    /// # struct LevelingJoker { level: u32, xp: u32, abilities: Vec<String> }
    /// # impl JokerState for LevelingJoker {
    /// fn debug_state(&self) -> String {
    ///     format!("Level {}, XP: {}/{}, Abilities: {:?}",
    ///             self.level, self.xp, self.xp_needed(), self.abilities)
    /// }
    /// # fn xp_needed(&self) -> u32 { 100 }
    /// # }
    /// ```
    fn debug_state(&self) -> String {
        "{}".to_string()
    }

    /// Resets the joker's state to its initial/default values.
    ///
    /// Called when the joker needs to be reset (new game, special effects,
    /// debug commands, etc.). Should restore all internal state variables
    /// to their starting values.
    ///
    /// **Default**: No-op (nothing to reset)
    ///
    /// # Example
    /// ```rust
    /// # use crate::joker::traits::JokerState;
    /// # struct ProgressJoker {
    /// #     times_used: u32,
    /// #     accumulated_bonus: f64,
    /// #     achievements: Vec<String>
    /// # }
    /// # impl JokerState for ProgressJoker {
    /// fn reset_state(&mut self) {
    ///     self.times_used = 0;
    ///     self.accumulated_bonus = 0.0;
    ///     self.achievements.clear();
    /// }
    /// # }
    /// ```
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
pub struct ProcessContext<'a> {
    pub hand_score: &'a mut HandScore,
    pub played_cards: &'a [Card],
    pub held_cards: &'a [Card],
    pub events: &'a mut Vec<GameEvent>,
}

/// Result returned from joker processing.
pub struct ProcessResult {
    pub chips_added: u64,
    pub mult_added: f64,
    pub retriggered: bool,
}

impl Default for ProcessResult {
    fn default() -> Self {
        Self {
            chips_added: 0,
            mult_added: 0.0,
            retriggered: false,
        }
    }
}
