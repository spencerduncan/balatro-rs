//! Consumable Purchase System with Overflow Handling
//!
//! This module implements configurable overflow handling for consumable purchases
//! when all slots are full. It follows Clean Code principles with the Strategy
//! pattern for different overflow behaviors.
//!
//! ## Key Principles Applied:
//! - Single Responsibility: Each strategy handles one overflow behavior
//! - Open/Closed: Easy to add new strategies without modifying existing code
//! - Strategy Pattern: Configurable behavior without complex conditionals
//! - Clean interfaces: Clear method signatures with meaningful names
//! - Error handling: Explicit error types for different failure modes
//!
//! ## Usage Examples
//!
//! ### Basic Purchase with Overflow
//! ```rust
//! use balatro_rs::game::Game;
//! use balatro_rs::config::{Config, ConsumableOverflowStrategy};
//! use balatro_rs::consumables::{ConsumablePurchaseHandler, ConsumableId};
//! use balatro_rs::stage::Stage;
//!
//! // Set up game with FIFO overflow strategy
//! let mut config = Config::new();
//! config.consumable_overflow_strategy = ConsumableOverflowStrategy::Fifo;
//! config.consumable_hand_capacity = 2;
//!
//! let mut game = Game::new(config);
//! game.stage = Stage::Shop();
//! game.money = 20.0;
//!
//! // Fill consumable hand to capacity
//! game.consumables_in_hand = vec![
//!     ConsumableId::TheFool,
//!     ConsumableId::TheMagician,
//! ];
//!
//! // Create purchase handler and buy a consumable
//! let handler = ConsumablePurchaseHandler::from_config(&game);
//! let result = handler.purchase_consumable(
//!     &mut game,
//!     ConsumableId::TheEmpress,
//!     3.0
//! );
//!
//! match result {
//!     Ok(purchase_result) => {
//!         if purchase_result.was_overflow() {
//!             println!("Purchased {} by removing {}",
//!                 purchase_result.purchased_consumable,
//!                 purchase_result.removed_consumable.unwrap()
//!             );
//!         } else {
//!             println!("Purchased {} in empty slot {}",
//!                 purchase_result.purchased_consumable,
//!                 purchase_result.placed_in_slot
//!             );
//!         }
//!     }
//!     Err(e) => println!("Purchase failed: {}", e),
//! }
//! ```
//!
//! ### Custom Overflow Strategy
//! ```rust
//! use balatro_rs::consumables::{OverflowStrategy, PurchaseError, ConsumableId};
//!
//! #[derive(Debug)]
//! struct RandomOverflowStrategy;
//!
//! impl OverflowStrategy for RandomOverflowStrategy {
//!     fn choose_slot_to_replace(
//!         &self,
//!         current_consumables: &[ConsumableId],
//!         _capacity: usize,
//!     ) -> Result<usize, PurchaseError> {
//!         use rand::Rng;
//!         let mut rng = rand::thread_rng();
//!         Ok(rng.gen_range(0..current_consumables.len()))
//!     }
//!
//!     fn strategy_name(&self) -> &'static str {
//!         "Random"
//!     }
//! }
//! ```

use crate::config::ConsumableOverflowStrategy;
use crate::consumables::ConsumableId;
use crate::game::Game;
use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;

/// Errors that can occur during consumable purchase operations
#[derive(Debug, Error, Clone, PartialEq)]
pub enum PurchaseError {
    #[error("Insufficient funds: need ${cost}, have ${available}")]
    InsufficientFunds { cost: f64, available: f64 },

    #[error("Invalid game state for purchase: {reason}")]
    InvalidGameState { reason: String },

    #[error("Overflow handling failed: {reason}")]
    OverflowHandlingFailed { reason: String },

    #[error("Consumable creation failed: {consumable_id:?}")]
    ConsumableCreationFailed { consumable_id: ConsumableId },

    #[error("No slots available and overflow disabled")]
    NoSlotsAvailable,
}

/// Result of a purchase operation with detailed information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PurchaseResult {
    /// The consumable that was purchased
    pub purchased_consumable: ConsumableId,
    /// The slot where the consumable was placed
    pub placed_in_slot: usize,
    /// The consumable that was removed due to overflow, if any
    pub removed_consumable: Option<ConsumableId>,
    /// The slot from which a consumable was removed, if any
    pub removed_from_slot: Option<usize>,
    /// Whether overflow handling was triggered
    pub overflow_occurred: bool,
    /// Cost of the purchase
    pub cost: f64,
    /// Remaining money after purchase
    pub remaining_money: f64,
}

impl PurchaseResult {
    /// Create a new purchase result for a successful purchase without overflow
    pub fn new_success(
        consumable: ConsumableId,
        slot: usize,
        cost: f64,
        remaining_money: f64,
    ) -> Self {
        Self {
            purchased_consumable: consumable,
            placed_in_slot: slot,
            removed_consumable: None,
            removed_from_slot: None,
            overflow_occurred: false,
            cost,
            remaining_money,
        }
    }

    /// Create a new purchase result for a successful purchase with overflow
    pub fn new_with_overflow(
        purchased: ConsumableId,
        placed_slot: usize,
        removed: ConsumableId,
        removed_slot: usize,
        cost: f64,
        remaining_money: f64,
    ) -> Self {
        Self {
            purchased_consumable: purchased,
            placed_in_slot: placed_slot,
            removed_consumable: Some(removed),
            removed_from_slot: Some(removed_slot),
            overflow_occurred: true,
            cost,
            remaining_money,
        }
    }

    /// Returns true if this purchase triggered overflow handling
    pub fn was_overflow(&self) -> bool {
        self.overflow_occurred
    }

    /// Get a human-readable description of what happened
    pub fn description(&self) -> String {
        if self.overflow_occurred {
            format!(
                "Purchased {} (slot {}), removed {} (slot {})",
                self.purchased_consumable,
                self.placed_in_slot,
                self.removed_consumable.as_ref().unwrap(),
                self.removed_from_slot.unwrap()
            )
        } else {
            format!(
                "Purchased {} (slot {})",
                self.purchased_consumable, self.placed_in_slot
            )
        }
    }
}

/// Trait defining the strategy for handling consumable overflow
///
/// This trait embodies the Strategy pattern, allowing different overflow
/// behaviors to be implemented cleanly and swapped at runtime.
pub trait OverflowStrategy: fmt::Debug + Send + Sync {
    /// Determine which slot should be used when all slots are full
    ///
    /// # Arguments
    /// * `current_consumables` - Current consumables in slots (index = slot)
    /// * `capacity` - Maximum number of slots available
    ///
    /// # Returns
    /// * `Ok(slot_index)` - The slot that should be replaced
    /// * `Err(PurchaseError)` - If overflow cannot be handled
    fn choose_slot_to_replace(
        &self,
        current_consumables: &[ConsumableId],
        capacity: usize,
    ) -> Result<usize, PurchaseError>;

    /// Get a human-readable name for this strategy
    fn strategy_name(&self) -> &'static str;
}

/// FIFO (First In, First Out) overflow strategy
///
/// Removes the consumable that was added earliest (index 0).
/// This maintains chronological order and feels natural to most players.
#[derive(Debug, Clone)]
pub struct FifoOverflowStrategy;

impl Default for FifoOverflowStrategy {
    fn default() -> Self {
        Self::new()
    }
}

impl FifoOverflowStrategy {
    pub fn new() -> Self {
        Self
    }
}

impl OverflowStrategy for FifoOverflowStrategy {
    fn choose_slot_to_replace(
        &self,
        current_consumables: &[ConsumableId],
        capacity: usize,
    ) -> Result<usize, PurchaseError> {
        // Validate inputs - defensive programming
        if current_consumables.is_empty() {
            return Err(PurchaseError::OverflowHandlingFailed {
                reason: "No consumables to replace".to_string(),
            });
        }

        if current_consumables.len() != capacity {
            return Err(PurchaseError::OverflowHandlingFailed {
                reason: format!(
                    "Expected {} consumables for full capacity, found {}",
                    capacity,
                    current_consumables.len()
                ),
            });
        }

        // FIFO always removes the first slot (oldest)
        Ok(0)
    }

    fn strategy_name(&self) -> &'static str {
        "FIFO"
    }
}

/// LIFO (Last In, First Out) overflow strategy
///
/// Removes the consumable that was added most recently (highest index).
/// This preserves older consumables that players may value more.
#[derive(Debug, Clone)]
pub struct LifoOverflowStrategy;

impl Default for LifoOverflowStrategy {
    fn default() -> Self {
        Self::new()
    }
}

impl LifoOverflowStrategy {
    pub fn new() -> Self {
        Self
    }
}

impl OverflowStrategy for LifoOverflowStrategy {
    fn choose_slot_to_replace(
        &self,
        current_consumables: &[ConsumableId],
        capacity: usize,
    ) -> Result<usize, PurchaseError> {
        // Validate inputs - defensive programming
        if current_consumables.is_empty() {
            return Err(PurchaseError::OverflowHandlingFailed {
                reason: "No consumables to replace".to_string(),
            });
        }

        if current_consumables.len() != capacity {
            return Err(PurchaseError::OverflowHandlingFailed {
                reason: format!(
                    "Expected {} consumables for full capacity, found {}",
                    capacity,
                    current_consumables.len()
                ),
            });
        }

        // LIFO removes the last slot (newest)
        Ok(current_consumables.len() - 1)
    }

    fn strategy_name(&self) -> &'static str {
        "LIFO"
    }
}

/// Factory for creating overflow strategies based on configuration
///
/// This factory follows the Factory pattern and makes it easy to add
/// new strategies without modifying existing code.
pub struct OverflowStrategyFactory;

impl OverflowStrategyFactory {
    /// Create an overflow strategy based on the configuration
    pub fn create_strategy(strategy_type: ConsumableOverflowStrategy) -> Box<dyn OverflowStrategy> {
        match strategy_type {
            ConsumableOverflowStrategy::Fifo => Box::new(FifoOverflowStrategy::new()),
            ConsumableOverflowStrategy::Lifo => Box::new(LifoOverflowStrategy::new()),
        }
    }
}

/// Main purchase handler that coordinates overflow handling
///
/// This struct acts as the Facade for the purchase system, providing
/// a clean interface while handling the complexity internally.
#[derive(Debug)]
pub struct ConsumablePurchaseHandler {
    overflow_strategy: Box<dyn OverflowStrategy>,
}

impl ConsumablePurchaseHandler {
    /// Create a new purchase handler with the specified overflow strategy
    pub fn new(strategy: Box<dyn OverflowStrategy>) -> Self {
        Self {
            overflow_strategy: strategy,
        }
    }

    /// Create a purchase handler from game configuration
    pub fn from_config(game: &Game) -> Self {
        let strategy =
            OverflowStrategyFactory::create_strategy(game.config.consumable_overflow_strategy);
        Self::new(strategy)
    }

    /// Attempt to purchase a consumable, handling overflow if necessary
    ///
    /// This is the main entry point for consumable purchases. It handles
    /// all the complexity of validation, overflow, and state updates.
    ///
    /// # Arguments
    /// * `game` - Mutable reference to the game state
    /// * `consumable_id` - The consumable to purchase
    /// * `cost` - The cost of the consumable
    ///
    /// # Returns
    /// * `Ok(PurchaseResult)` - Detailed information about the purchase
    /// * `Err(PurchaseError)` - Specific reason why purchase failed
    pub fn purchase_consumable(
        &self,
        game: &mut Game,
        consumable_id: ConsumableId,
        cost: f64,
    ) -> Result<PurchaseResult, PurchaseError> {
        // Step 1: Validate purchase preconditions
        self.validate_purchase_preconditions(game, cost)?;

        // Step 2: Determine where to place the consumable
        let placement = self.determine_placement(game, consumable_id)?;

        // Step 3: Execute the purchase
        self.execute_purchase(game, placement, cost)
    }

    /// Validate that a purchase can proceed
    fn validate_purchase_preconditions(&self, game: &Game, cost: f64) -> Result<(), PurchaseError> {
        // Check sufficient funds
        if game.money < cost {
            return Err(PurchaseError::InsufficientFunds {
                cost,
                available: game.money,
            });
        }

        // Check game state (must be in shop)
        if !matches!(game.stage, crate::stage::Stage::Shop()) {
            return Err(PurchaseError::InvalidGameState {
                reason: format!("Not in shop stage, currently in {:?}", game.stage),
            });
        }

        Ok(())
    }

    /// Determine where to place the new consumable
    fn determine_placement(
        &self,
        game: &Game,
        consumable_id: ConsumableId,
    ) -> Result<PlacementDecision, PurchaseError> {
        let current_count = game.consumables_in_hand.len();
        let capacity = game.config.consumable_hand_capacity;

        if current_count < capacity {
            // Simple case: we have empty slots
            Ok(PlacementDecision::EmptySlot {
                slot: current_count,
                consumable: consumable_id,
            })
        } else {
            // Overflow case: need to replace an existing consumable
            let slot_to_replace = self
                .overflow_strategy
                .choose_slot_to_replace(&game.consumables_in_hand, capacity)?;

            let replaced_consumable = game.consumables_in_hand[slot_to_replace];

            Ok(PlacementDecision::ReplaceSlot {
                slot: slot_to_replace,
                new_consumable: consumable_id,
                replaced_consumable,
            })
        }
    }

    /// Execute the actual purchase and state changes
    fn execute_purchase(
        &self,
        game: &mut Game,
        placement: PlacementDecision,
        cost: f64,
    ) -> Result<PurchaseResult, PurchaseError> {
        // Deduct money first (fail fast if there's a problem)
        game.money -= cost;

        match placement {
            PlacementDecision::EmptySlot { slot, consumable } => {
                // Add to empty slot
                game.consumables_in_hand.push(consumable);

                Ok(PurchaseResult::new_success(
                    consumable, slot, cost, game.money,
                ))
            }
            PlacementDecision::ReplaceSlot {
                slot,
                new_consumable,
                replaced_consumable,
            } => {
                // Replace existing consumable
                game.consumables_in_hand[slot] = new_consumable;

                Ok(PurchaseResult::new_with_overflow(
                    new_consumable,
                    slot,
                    replaced_consumable,
                    slot,
                    cost,
                    game.money,
                ))
            }
        }
    }
}

/// Internal enum for tracking placement decisions
#[derive(Debug, Clone)]
enum PlacementDecision {
    EmptySlot {
        slot: usize,
        consumable: ConsumableId,
    },
    ReplaceSlot {
        slot: usize,
        new_consumable: ConsumableId,
        replaced_consumable: ConsumableId,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{Config, ConsumableOverflowStrategy};
    use crate::consumables::ConsumableId;
    use crate::game::Game;
    use crate::stage::Stage;

    /// Helper function to create a test game with specified consumables
    fn create_test_game(
        consumables: Vec<ConsumableId>,
        money: f64,
        capacity: usize,
        strategy: ConsumableOverflowStrategy,
    ) -> Game {
        let mut config = Config::new();
        config.consumable_hand_capacity = capacity;
        config.consumable_overflow_strategy = strategy;

        Game {
            config,
            consumables_in_hand: consumables,
            money,
            stage: Stage::Shop(),
            ..Default::default()
        }
    }

    #[test]
    fn test_fifo_strategy_chooses_first_slot() {
        let strategy = FifoOverflowStrategy::new();
        let consumables = vec![ConsumableId::TheFool, ConsumableId::TheMagician];

        let result = strategy.choose_slot_to_replace(&consumables, 2);
        assert_eq!(result.unwrap(), 0); // Should choose first slot
    }

    #[test]
    fn test_lifo_strategy_chooses_last_slot() {
        let strategy = LifoOverflowStrategy::new();
        let consumables = vec![ConsumableId::TheFool, ConsumableId::TheMagician];

        let result = strategy.choose_slot_to_replace(&consumables, 2);
        assert_eq!(result.unwrap(), 1); // Should choose last slot
    }

    #[test]
    fn test_strategy_factory_creates_correct_strategy() {
        let fifo_strategy =
            OverflowStrategyFactory::create_strategy(ConsumableOverflowStrategy::Fifo);
        assert_eq!(fifo_strategy.strategy_name(), "FIFO");

        let lifo_strategy =
            OverflowStrategyFactory::create_strategy(ConsumableOverflowStrategy::Lifo);
        assert_eq!(lifo_strategy.strategy_name(), "LIFO");
    }

    #[test]
    fn test_purchase_without_overflow() {
        let mut game = create_test_game(
            vec![ConsumableId::TheFool],
            10.0,
            2,
            ConsumableOverflowStrategy::Fifo,
        );

        let handler = ConsumablePurchaseHandler::from_config(&game);
        let result = handler
            .purchase_consumable(&mut game, ConsumableId::TheMagician, 3.0)
            .unwrap();

        assert!(!result.was_overflow());
        assert_eq!(result.purchased_consumable, ConsumableId::TheMagician);
        assert_eq!(result.placed_in_slot, 1);
        assert_eq!(result.remaining_money, 7.0);
        assert_eq!(game.consumables_in_hand.len(), 2);
    }

    #[test]
    fn test_purchase_with_fifo_overflow() {
        let mut game = create_test_game(
            vec![ConsumableId::TheFool, ConsumableId::TheMagician],
            10.0,
            2,
            ConsumableOverflowStrategy::Fifo,
        );

        let handler = ConsumablePurchaseHandler::from_config(&game);
        let result = handler
            .purchase_consumable(&mut game, ConsumableId::TheEmpress, 3.0)
            .unwrap();

        assert!(result.was_overflow());
        assert_eq!(result.purchased_consumable, ConsumableId::TheEmpress);
        assert_eq!(result.placed_in_slot, 0);
        assert_eq!(result.removed_consumable, Some(ConsumableId::TheFool));
        assert_eq!(result.removed_from_slot, Some(0));
        assert_eq!(game.consumables_in_hand[0], ConsumableId::TheEmpress);
        assert_eq!(game.consumables_in_hand[1], ConsumableId::TheMagician);
    }

    #[test]
    fn test_purchase_with_lifo_overflow() {
        let mut game = create_test_game(
            vec![ConsumableId::TheFool, ConsumableId::TheMagician],
            10.0,
            2,
            ConsumableOverflowStrategy::Lifo,
        );

        let handler = ConsumablePurchaseHandler::from_config(&game);
        let result = handler
            .purchase_consumable(&mut game, ConsumableId::TheEmpress, 3.0)
            .unwrap();

        assert!(result.was_overflow());
        assert_eq!(result.purchased_consumable, ConsumableId::TheEmpress);
        assert_eq!(result.placed_in_slot, 1);
        assert_eq!(result.removed_consumable, Some(ConsumableId::TheMagician));
        assert_eq!(result.removed_from_slot, Some(1));
        assert_eq!(game.consumables_in_hand[0], ConsumableId::TheFool);
        assert_eq!(game.consumables_in_hand[1], ConsumableId::TheEmpress);
    }

    #[test]
    fn test_insufficient_funds() {
        let mut game = create_test_game(
            vec![],
            2.0, // Not enough for a 3.0 purchase
            2,
            ConsumableOverflowStrategy::Fifo,
        );

        let handler = ConsumablePurchaseHandler::from_config(&game);
        let result = handler.purchase_consumable(&mut game, ConsumableId::TheFool, 3.0);

        assert!(matches!(
            result,
            Err(PurchaseError::InsufficientFunds { .. })
        ));
    }

    #[test]
    fn test_invalid_game_state() {
        let mut game = create_test_game(vec![], 10.0, 2, ConsumableOverflowStrategy::Fifo);
        game.stage = Stage::PreBlind(); // Wrong stage

        let handler = ConsumablePurchaseHandler::from_config(&game);
        let result = handler.purchase_consumable(&mut game, ConsumableId::TheFool, 3.0);

        assert!(matches!(
            result,
            Err(PurchaseError::InvalidGameState { .. })
        ));
    }

    #[test]
    fn test_single_slot_overflow() {
        let mut game = create_test_game(
            vec![ConsumableId::TheFool],
            10.0,
            1, // Single slot
            ConsumableOverflowStrategy::Fifo,
        );

        let handler = ConsumablePurchaseHandler::from_config(&game);
        let result = handler
            .purchase_consumable(&mut game, ConsumableId::TheMagician, 3.0)
            .unwrap();

        assert!(result.was_overflow());
        assert_eq!(result.placed_in_slot, 0);
        assert_eq!(result.removed_from_slot, Some(0));
        assert_eq!(game.consumables_in_hand.len(), 1);
        assert_eq!(game.consumables_in_hand[0], ConsumableId::TheMagician);
    }
}
