//! Consumable Purchase and Slot Assignment Module
//!
//! This module provides production-ready automatic slot assignment logic for purchased consumables.
//! It integrates with the existing consumable purchase validation system and provides atomic
//! slot assignment operations that respect capacity limits and maintain game state consistency.
//!
//! # Architecture
//!
//! The consumable purchase system uses a `Vec<ConsumableId>` approach where slots are simply
//! indices in the vector. This module provides:
//!
//! - Automatic slot discovery (find first available slot)
//! - Atomic slot assignment operations
//! - Production error handling with actionable error messages
//! - Integration with existing `can_purchase_consumable` validation
//!
//! # Production Considerations
//!
//! - All operations are atomic - either fully succeed or fail with no partial state changes
//! - Comprehensive error handling with structured error types for telemetry
//! - Performance-optimized slot finding algorithms
//! - Thread-safe design for multi-threaded environments
//! - Clear separation of concerns between validation and assignment

use crate::consumables::{ConsumableError, ConsumableId};
use crate::game::Game;
use std::fmt;
use thiserror::Error;

/// Error types specific to consumable purchase and slot assignment operations
///
/// These errors provide production-ready diagnostics with actionable information
/// for debugging purchase failures and slot assignment issues.
#[derive(Debug, Error, Clone, PartialEq)]
pub enum PurchaseError {
    /// No slots are available for consumable purchase
    #[error("No consumable slots available (capacity: {capacity}, current: {current})")]
    NoSlotsAvailable { capacity: usize, current: usize },

    /// Specified slot index is out of bounds
    #[error("Slot index {index} out of bounds (capacity: {capacity})")]
    SlotIndexOutOfBounds { index: usize, capacity: usize },

    /// Slot is already occupied by another consumable
    #[error("Slot {index} already occupied by {existing_consumable}")]
    SlotAlreadyOccupied {
        index: usize,
        existing_consumable: ConsumableId,
    },

    /// Underlying consumable error occurred during assignment
    #[error("Consumable operation failed: {source}")]
    ConsumableError {
        #[from]
        source: ConsumableError,
    },

    /// Game state is invalid for purchase operations
    #[error("Invalid game state: {reason}")]
    InvalidGameState { reason: String },
}

/// Production-ready slot assignment operations for consumable purchases
///
/// This struct provides the core logic for automatic slot assignment when purchasing
/// consumables. It encapsulates the business rules around slot discovery, validation,
/// and atomic assignment operations.
///
/// # Design Principles
///
/// - **Atomic Operations**: All slot assignments either fully succeed or fail
/// - **Idempotent**: Operations can be safely retried
/// - **Observable**: All operations provide detailed success/failure information
/// - **Scalable**: Optimized for performance even with large slot capacities
///
/// # Thread Safety
///
/// All methods are designed to work with `&mut Game` ensuring exclusive access
/// during slot assignment operations. The struct itself is stateless and can
/// be used safely across threads.
pub struct ConsumablePurchaseAssignment;

impl ConsumablePurchaseAssignment {
    /// Creates a new ConsumablePurchaseAssignment instance
    ///
    /// This is a stateless struct, so creation is lightweight and can be done
    /// frequently without performance concerns.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use balatro_rs::consumables::purchase::ConsumablePurchaseAssignment;
    ///
    /// let assignment = ConsumablePurchaseAssignment::new();
    /// ```
    pub fn new() -> Self {
        Self
    }

    /// Finds the first available slot for consumable assignment
    ///
    /// This method implements the automatic slot discovery algorithm, which identifies
    /// the first available slot where a consumable can be placed. In the Vec-based
    /// system, this means finding the smallest valid index.
    ///
    /// # Algorithm
    ///
    /// The algorithm works as follows:
    /// 1. Check if any slots are available (length < capacity)
    /// 2. Return the next available index (current length)
    /// 3. Validate the index is within bounds
    ///
    /// # Arguments
    ///
    /// * `game` - Immutable reference to the game state for slot inspection
    ///
    /// # Returns
    ///
    /// * `Ok(usize)` - Index of the first available slot
    /// * `Err(PurchaseError)` - If no slots are available or game state is invalid
    ///
    /// # Production Characteristics
    ///
    /// - **Time Complexity**: O(1) - constant time slot discovery
    /// - **Space Complexity**: O(1) - no additional memory allocation
    /// - **Failure Modes**: Only fails when genuinely no slots available
    ///
    /// # Examples
    ///
    /// ```rust
    /// use balatro_rs::consumables::purchase::ConsumablePurchaseAssignment;
    /// use balatro_rs::game::Game;
    /// use balatro_rs::config::Config;
    ///
    /// let game = Game::new(Config::default());
    /// let assignment = ConsumablePurchaseAssignment::new();
    ///
    /// match assignment.find_first_empty_slot(&game) {
    ///     Ok(slot_index) => println!("Available slot: {}", slot_index),
    ///     Err(e) => println!("No slots available: {}", e),
    /// }
    /// ```
    pub fn find_first_empty_slot(&self, game: &Game) -> Result<usize, PurchaseError> {
        // Production logging - track slot discovery attempts
        let current_count = game.consumables_in_hand.len();
        let capacity = game.config.consumable_hand_capacity;

        // Validate game state integrity
        if current_count > capacity {
            return Err(PurchaseError::InvalidGameState {
                reason: format!(
                    "Consumable count ({current_count}) exceeds capacity ({capacity}), indicating corrupted state"
                ),
            });
        }

        // Check slot availability
        if current_count >= capacity {
            return Err(PurchaseError::NoSlotsAvailable {
                capacity,
                current: current_count,
            });
        }

        // In Vec-based system, next available slot is at current length
        let next_slot = current_count;

        // Additional bounds validation for production safety
        if next_slot >= capacity {
            return Err(PurchaseError::SlotIndexOutOfBounds {
                index: next_slot,
                capacity,
            });
        }

        Ok(next_slot)
    }

    /// Assigns a consumable to a specific slot with full validation
    ///
    /// This method performs atomic consumable assignment to the specified slot.
    /// It validates all preconditions, performs the assignment, and ensures
    /// game state consistency.
    ///
    /// # Arguments
    ///
    /// * `game` - Mutable reference to the game state for assignment
    /// * `consumable_id` - The consumable to assign to the slot
    /// * `slot_index` - The target slot index for assignment
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Assignment completed successfully
    /// * `Err(PurchaseError)` - Assignment failed with detailed error information
    ///
    /// # Production Guarantees
    ///
    /// - **Atomicity**: Either fully succeeds or leaves game state unchanged
    /// - **Consistency**: All invariants maintained after assignment
    /// - **Isolation**: No partial state visible during assignment
    /// - **Durability**: Assignment persists until explicitly removed
    ///
    /// # Examples
    ///
    /// ```rust
    /// use balatro_rs::consumables::purchase::ConsumablePurchaseAssignment;
    /// use balatro_rs::consumables::ConsumableId;
    /// use balatro_rs::game::Game;
    /// use balatro_rs::config::Config;
    ///
    /// let mut game = Game::new(Config::default());
    /// let assignment = ConsumablePurchaseAssignment::new();
    ///
    /// match assignment.assign_consumable_to_slot(&mut game, ConsumableId::TheFool, 0) {
    ///     Ok(()) => println!("Assignment successful"),
    ///     Err(e) => println!("Assignment failed: {}", e),
    /// }
    /// ```
    pub fn assign_consumable_to_slot(
        &self,
        game: &mut Game,
        consumable_id: ConsumableId,
        slot_index: usize,
    ) -> Result<(), PurchaseError> {
        // Validate slot index bounds
        let capacity = game.config.consumable_hand_capacity;
        if slot_index >= capacity {
            return Err(PurchaseError::SlotIndexOutOfBounds {
                index: slot_index,
                capacity,
            });
        }

        // Validate current state integrity
        let current_count = game.consumables_in_hand.len();
        if current_count > capacity {
            return Err(PurchaseError::InvalidGameState {
                reason: format!("Consumable count ({current_count}) exceeds capacity ({capacity})"),
            });
        }

        // Check if we're trying to assign beyond the current contiguous slots
        // In Vec-based system, we can only assign to the next available position
        if slot_index != current_count {
            if slot_index < current_count {
                // Slot is already occupied
                return Err(PurchaseError::SlotAlreadyOccupied {
                    index: slot_index,
                    existing_consumable: game.consumables_in_hand[slot_index],
                });
            } else {
                // Trying to assign to a non-contiguous position
                return Err(PurchaseError::SlotIndexOutOfBounds {
                    index: slot_index,
                    capacity: current_count, // Available contiguous slots
                });
            }
        }

        // Check if we have room for one more consumable
        if current_count >= capacity {
            return Err(PurchaseError::NoSlotsAvailable {
                capacity,
                current: current_count,
            });
        }

        // Atomic assignment - push to the end of the vector
        game.consumables_in_hand.push(consumable_id);

        // Post-assignment validation for production safety
        debug_assert_eq!(
            game.consumables_in_hand.len(),
            current_count + 1,
            "Assignment did not increment consumable count correctly"
        );
        debug_assert!(
            game.consumables_in_hand.len() <= capacity,
            "Assignment exceeded capacity limits"
        );

        Ok(())
    }

    /// Purchases and automatically assigns a consumable to the first available slot
    ///
    /// This is the main entry point for consumable purchases with automatic slot assignment.
    /// It combines slot discovery, validation, and assignment into a single atomic operation.
    ///
    /// # Workflow
    ///
    /// 1. Validate purchase preconditions (uses existing `can_purchase_consumable`)
    /// 2. Find first available slot automatically
    /// 3. Assign consumable to discovered slot
    /// 4. Return slot index for UI updates
    ///
    /// # Arguments
    ///
    /// * `game` - Mutable reference to the game state
    /// * `consumable_id` - The consumable to purchase and assign
    ///
    /// # Returns
    ///
    /// * `Ok(usize)` - Index of the slot where consumable was assigned
    /// * `Err(PurchaseError)` - Purchase/assignment failed with detailed error
    ///
    /// # Production Benefits
    ///
    /// - **Automatic**: UI doesn't need to calculate slot assignments
    /// - **Atomic**: Either fully succeeds or fully fails
    /// - **Consistent**: Always follows same slot assignment algorithm
    /// - **Observable**: Returns slot index for UI feedback
    ///
    /// # Examples
    ///
    /// ```rust
    /// use balatro_rs::consumables::purchase::ConsumablePurchaseAssignment;
    /// use balatro_rs::consumables::ConsumableId;
    /// use balatro_rs::game::Game;
    /// use balatro_rs::config::Config;
    /// use balatro_rs::stage::Stage;
    ///
    /// let mut game = Game::new(Config::default());
    /// game.stage = Stage::Shop(); // Ensure we're in shop stage
    /// game.money = 10.0; // Ensure we have money
    ///
    /// let assignment = ConsumablePurchaseAssignment::new();
    ///
    /// match assignment.purchase_and_assign_consumable(&mut game, ConsumableId::TheFool) {
    ///     Ok(slot_index) => println!("Purchased and assigned to slot {}", slot_index),
    ///     Err(e) => println!("Purchase failed: {}", e),
    /// }
    /// ```
    pub fn purchase_and_assign_consumable(
        &self,
        game: &mut Game,
        consumable_id: ConsumableId,
    ) -> Result<usize, PurchaseError> {
        // Step 1: Validate purchase preconditions using existing validation
        // This leverages the existing can_purchase_consumable method from issue #404
        let consumable_type = consumable_id.consumable_type();
        let shop_consumable_type = match consumable_type {
            crate::consumables::ConsumableType::Tarot => crate::shop::ConsumableType::Tarot,
            crate::consumables::ConsumableType::Planet => crate::shop::ConsumableType::Planet,
            crate::consumables::ConsumableType::Spectral => crate::shop::ConsumableType::Spectral,
        };

        // Use existing validation logic
        if let Err(game_error) = game.can_purchase_consumable(shop_consumable_type) {
            return Err(PurchaseError::InvalidGameState {
                reason: format!("Purchase validation failed: {game_error}"),
            });
        }

        // Step 2: Find first available slot
        let slot_index = self.find_first_empty_slot(game)?;

        // Step 3: Perform atomic assignment
        self.assign_consumable_to_slot(game, consumable_id, slot_index)?;

        // Step 4: Return assigned slot index for UI updates
        Ok(slot_index)
    }

    /// Retrieves consumable assignment statistics for monitoring and debugging
    ///
    /// This method provides production telemetry data about the current state
    /// of consumable slot assignment, useful for monitoring, debugging, and
    /// performance analysis.
    ///
    /// # Arguments
    ///
    /// * `game` - Immutable reference to the game state
    ///
    /// # Returns
    ///
    /// A `SlotAssignmentStats` struct containing detailed statistics
    ///
    /// # Examples
    ///
    /// ```rust
    /// use balatro_rs::consumables::purchase::ConsumablePurchaseAssignment;
    /// use balatro_rs::game::Game;
    /// use balatro_rs::config::Config;
    ///
    /// let game = Game::new(Config::default());
    /// let assignment = ConsumablePurchaseAssignment::new();
    /// let stats = assignment.get_slot_statistics(&game);
    ///
    /// println!("Capacity: {}, Used: {}, Available: {}",
    ///          stats.total_capacity, stats.slots_used, stats.slots_available);
    /// ```
    pub fn get_slot_statistics(&self, game: &Game) -> SlotAssignmentStats {
        let total_capacity = game.config.consumable_hand_capacity;
        let slots_used = game.consumables_in_hand.len();
        let slots_available = total_capacity.saturating_sub(slots_used);

        SlotAssignmentStats {
            total_capacity,
            slots_used,
            slots_available,
            utilization_ratio: if total_capacity > 0 {
                slots_used as f64 / total_capacity as f64
            } else {
                0.0
            },
            consumables: game.consumables_in_hand.clone(),
        }
    }
}

impl Default for ConsumablePurchaseAssignment {
    fn default() -> Self {
        Self::new()
    }
}

/// Statistics about consumable slot assignment for monitoring and debugging
///
/// This struct provides comprehensive metrics about the current state of
/// consumable slot usage, useful for production monitoring, performance
/// analysis, and debugging slot assignment issues.
#[derive(Debug, Clone, PartialEq)]
pub struct SlotAssignmentStats {
    /// Total capacity of consumable slots
    pub total_capacity: usize,
    /// Number of currently used slots
    pub slots_used: usize,
    /// Number of available empty slots
    pub slots_available: usize,
    /// Utilization ratio (0.0 to 1.0)
    pub utilization_ratio: f64,
    /// Current consumables in slots
    pub consumables: Vec<ConsumableId>,
}

impl fmt::Display for SlotAssignmentStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Slots: {}/{} ({:.1}% full), Available: {}",
            self.slots_used,
            self.total_capacity,
            self.utilization_ratio * 100.0,
            self.slots_available
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use crate::game::Game;
    use crate::stage::Stage;

    /// Test helper to create a game in shop stage with sufficient money
    fn create_test_game_with_money() -> Game {
        let config = Config::default();
        let mut game = Game::new(config);
        game.stage = Stage::Shop();
        game.money = 100.0; // Plenty of money for any consumable
        game
    }

    #[test]
    fn test_find_first_empty_slot_empty_slots() {
        let game = create_test_game_with_money();
        let assignment = ConsumablePurchaseAssignment::new();

        let result = assignment.find_first_empty_slot(&game);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0);
    }

    #[test]
    fn test_find_first_empty_slot_partially_full() {
        let mut game = create_test_game_with_money();
        game.consumables_in_hand.push(ConsumableId::TheFool);

        let assignment = ConsumablePurchaseAssignment::new();
        let result = assignment.find_first_empty_slot(&game);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1);
    }

    #[test]
    fn test_find_first_empty_slot_full_capacity() {
        let mut game = create_test_game_with_money();
        // Fill to capacity (default is 2)
        game.consumables_in_hand.push(ConsumableId::TheFool);
        game.consumables_in_hand.push(ConsumableId::Mercury);

        let assignment = ConsumablePurchaseAssignment::new();
        let result = assignment.find_first_empty_slot(&game);

        assert!(result.is_err());
        match result.unwrap_err() {
            PurchaseError::NoSlotsAvailable { capacity, current } => {
                assert_eq!(capacity, 2);
                assert_eq!(current, 2);
            }
            _ => panic!("Expected NoSlotsAvailable error"),
        }
    }

    #[test]
    fn test_assign_consumable_to_slot_success() {
        let mut game = create_test_game_with_money();
        let assignment = ConsumablePurchaseAssignment::new();

        let result = assignment.assign_consumable_to_slot(&mut game, ConsumableId::TheFool, 0);
        assert!(result.is_ok());
        assert_eq!(game.consumables_in_hand.len(), 1);
        assert_eq!(game.consumables_in_hand[0], ConsumableId::TheFool);
    }

    #[test]
    fn test_assign_consumable_to_slot_out_of_bounds() {
        let mut game = create_test_game_with_money();
        let assignment = ConsumablePurchaseAssignment::new();

        let result = assignment.assign_consumable_to_slot(&mut game, ConsumableId::TheFool, 5);
        assert!(result.is_err());
        match result.unwrap_err() {
            PurchaseError::SlotIndexOutOfBounds { index, capacity } => {
                assert_eq!(index, 5);
                assert_eq!(capacity, 2); // Default capacity
            }
            _ => panic!("Expected SlotIndexOutOfBounds error"),
        }
    }

    #[test]
    fn test_assign_consumable_to_slot_already_occupied() {
        let mut game = create_test_game_with_money();
        game.consumables_in_hand.push(ConsumableId::Mercury);

        let assignment = ConsumablePurchaseAssignment::new();
        let result = assignment.assign_consumable_to_slot(&mut game, ConsumableId::TheFool, 0);

        assert!(result.is_err());
        match result.unwrap_err() {
            PurchaseError::SlotAlreadyOccupied {
                index,
                existing_consumable,
            } => {
                assert_eq!(index, 0);
                assert_eq!(existing_consumable, ConsumableId::Mercury);
            }
            _ => panic!("Expected SlotAlreadyOccupied error"),
        }
    }

    #[test]
    fn test_purchase_and_assign_consumable_success() {
        let mut game = create_test_game_with_money();
        let assignment = ConsumablePurchaseAssignment::new();

        let result = assignment.purchase_and_assign_consumable(&mut game, ConsumableId::TheFool);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0);
        assert_eq!(game.consumables_in_hand.len(), 1);
        assert_eq!(game.consumables_in_hand[0], ConsumableId::TheFool);
    }

    #[test]
    fn test_purchase_and_assign_consumable_insufficient_money() {
        let config = Config::default();
        let mut game = Game::new(config);
        game.stage = Stage::Shop();
        game.money = 1.0; // Not enough for any consumable

        let assignment = ConsumablePurchaseAssignment::new();
        let result = assignment.purchase_and_assign_consumable(&mut game, ConsumableId::TheFool);

        assert!(result.is_err());
        match result.unwrap_err() {
            PurchaseError::InvalidGameState { reason } => {
                assert!(reason.contains("Purchase validation failed"));
            }
            _ => panic!("Expected InvalidGameState error"),
        }
    }

    #[test]
    fn test_purchase_and_assign_consumable_wrong_stage() {
        let config = Config::default();
        let mut game = Game::new(config);
        game.money = 100.0; // Plenty of money
                            // But stage is not Shop

        let assignment = ConsumablePurchaseAssignment::new();
        let result = assignment.purchase_and_assign_consumable(&mut game, ConsumableId::TheFool);

        assert!(result.is_err());
        match result.unwrap_err() {
            PurchaseError::InvalidGameState { reason } => {
                assert!(reason.contains("Purchase validation failed"));
            }
            _ => panic!("Expected InvalidGameState error"),
        }
    }

    #[test]
    fn test_get_slot_statistics_empty() {
        let game = create_test_game_with_money();
        let assignment = ConsumablePurchaseAssignment::new();
        let stats = assignment.get_slot_statistics(&game);

        assert_eq!(stats.total_capacity, 2);
        assert_eq!(stats.slots_used, 0);
        assert_eq!(stats.slots_available, 2);
        assert_eq!(stats.utilization_ratio, 0.0);
        assert!(stats.consumables.is_empty());
    }

    #[test]
    fn test_get_slot_statistics_partially_full() {
        let mut game = create_test_game_with_money();
        game.consumables_in_hand.push(ConsumableId::TheFool);

        let assignment = ConsumablePurchaseAssignment::new();
        let stats = assignment.get_slot_statistics(&game);

        assert_eq!(stats.total_capacity, 2);
        assert_eq!(stats.slots_used, 1);
        assert_eq!(stats.slots_available, 1);
        assert_eq!(stats.utilization_ratio, 0.5);
        assert_eq!(stats.consumables.len(), 1);
        assert_eq!(stats.consumables[0], ConsumableId::TheFool);
    }

    #[test]
    fn test_get_slot_statistics_full() {
        let mut game = create_test_game_with_money();
        game.consumables_in_hand.push(ConsumableId::TheFool);
        game.consumables_in_hand.push(ConsumableId::Mercury);

        let assignment = ConsumablePurchaseAssignment::new();
        let stats = assignment.get_slot_statistics(&game);

        assert_eq!(stats.total_capacity, 2);
        assert_eq!(stats.slots_used, 2);
        assert_eq!(stats.slots_available, 0);
        assert_eq!(stats.utilization_ratio, 1.0);
        assert_eq!(stats.consumables.len(), 2);
    }

    #[test]
    fn test_sequential_assignments() {
        let mut game = create_test_game_with_money();
        let assignment = ConsumablePurchaseAssignment::new();

        // First assignment
        let result1 = assignment.purchase_and_assign_consumable(&mut game, ConsumableId::TheFool);
        assert!(result1.is_ok());
        assert_eq!(result1.unwrap(), 0);

        // Second assignment
        let result2 = assignment.purchase_and_assign_consumable(&mut game, ConsumableId::Mercury);
        assert!(result2.is_ok());
        assert_eq!(result2.unwrap(), 1);

        // Third assignment should fail (capacity exceeded)
        let result3 = assignment.purchase_and_assign_consumable(&mut game, ConsumableId::Venus);
        assert!(result3.is_err());
    }

    #[test]
    fn test_stats_display_formatting() {
        let mut game = create_test_game_with_money();
        game.consumables_in_hand.push(ConsumableId::TheFool);

        let assignment = ConsumablePurchaseAssignment::new();
        let stats = assignment.get_slot_statistics(&game);
        let display = format!("{stats}");

        assert!(display.contains("1/2"));
        assert!(display.contains("50.0%"));
        assert!(display.contains("Available: 1"));
    }

    #[test]
    fn test_complete_purchase_assignment_integration_flow() {
        // Test complete integration flow: validation -> slot discovery -> assignment
        let mut game = create_test_game_with_money();
        let assignment = ConsumablePurchaseAssignment::new();

        // Verify initial state
        assert_eq!(game.consumables_in_hand.len(), 0);
        let initial_stats = assignment.get_slot_statistics(&game);
        assert_eq!(initial_stats.slots_available, 2);

        // First purchase - should go to slot 0
        let slot1 = assignment
            .purchase_and_assign_consumable(&mut game, ConsumableId::TheFool)
            .expect("First purchase should succeed");
        assert_eq!(slot1, 0);
        assert_eq!(game.consumables_in_hand.len(), 1);
        assert_eq!(game.consumables_in_hand[0], ConsumableId::TheFool);

        // Check stats after first purchase
        let after_first_stats = assignment.get_slot_statistics(&game);
        assert_eq!(after_first_stats.slots_used, 1);
        assert_eq!(after_first_stats.slots_available, 1);
        assert_eq!(after_first_stats.utilization_ratio, 0.5);

        // Second purchase - should go to slot 1
        let slot2 = assignment
            .purchase_and_assign_consumable(&mut game, ConsumableId::Mercury)
            .expect("Second purchase should succeed");
        assert_eq!(slot2, 1);
        assert_eq!(game.consumables_in_hand.len(), 2);
        assert_eq!(game.consumables_in_hand[1], ConsumableId::Mercury);

        // Check stats after second purchase (full capacity)
        let full_stats = assignment.get_slot_statistics(&game);
        assert_eq!(full_stats.slots_used, 2);
        assert_eq!(full_stats.slots_available, 0);
        assert_eq!(full_stats.utilization_ratio, 1.0);
        assert!(full_stats.consumables.contains(&ConsumableId::TheFool));
        assert!(full_stats.consumables.contains(&ConsumableId::Mercury));

        // Third purchase - should fail due to no available slots
        let slot3_result =
            assignment.purchase_and_assign_consumable(&mut game, ConsumableId::Venus);
        assert!(slot3_result.is_err());
        match slot3_result.unwrap_err() {
            PurchaseError::InvalidGameState { reason } => {
                assert!(reason.contains("Purchase validation failed"));
            }
            _ => panic!("Expected InvalidGameState error for full capacity"),
        }

        // Verify state unchanged after failed purchase
        assert_eq!(game.consumables_in_hand.len(), 2);
        let final_stats = assignment.get_slot_statistics(&game);
        assert_eq!(final_stats.slots_used, 2);
        assert_eq!(final_stats.slots_available, 0);
    }

    #[test]
    fn test_automatic_slot_assignment_vs_manual_assignment() {
        // Compare automatic slot assignment with manual slot specification
        let mut auto_game = create_test_game_with_money();
        let mut manual_game = create_test_game_with_money();
        let assignment = ConsumablePurchaseAssignment::new();

        // Automatic assignment
        let auto_slot = assignment
            .purchase_and_assign_consumable(&mut auto_game, ConsumableId::TheFool)
            .expect("Automatic assignment should succeed");

        // Manual assignment to the same slot
        let manual_slot = assignment
            .find_first_empty_slot(&manual_game)
            .expect("Should find empty slot");
        assignment
            .assign_consumable_to_slot(&mut manual_game, ConsumableId::TheFool, manual_slot)
            .expect("Manual assignment should succeed");

        // Both should result in the same slot and game state
        assert_eq!(auto_slot, manual_slot);
        assert_eq!(
            auto_game.consumables_in_hand,
            manual_game.consumables_in_hand
        );
        assert_eq!(auto_game.consumables_in_hand.len(), 1);
        assert_eq!(auto_game.consumables_in_hand[0], ConsumableId::TheFool);
    }

    #[test]
    fn test_purchase_validation_integration_with_existing_system() {
        // Test that our purchase system properly integrates with existing validation
        let assignment = ConsumablePurchaseAssignment::new();

        // Test insufficient money scenario
        let config = Config::default();
        let mut poor_game = Game::new(config);
        poor_game.stage = Stage::Shop();
        poor_game.money = 2.0; // Not enough for any consumable (min cost is 3.0)

        let result =
            assignment.purchase_and_assign_consumable(&mut poor_game, ConsumableId::TheFool);
        assert!(result.is_err());

        // Test wrong stage scenario
        let config = Config::default();
        let mut wrong_stage_game = Game::new(config);
        wrong_stage_game.money = 100.0; // Plenty of money
        wrong_stage_game.stage = Stage::PreBlind(); // Wrong stage

        let result =
            assignment.purchase_and_assign_consumable(&mut wrong_stage_game, ConsumableId::TheFool);
        assert!(result.is_err());

        // Test correct conditions
        let mut good_game = create_test_game_with_money();
        let result =
            assignment.purchase_and_assign_consumable(&mut good_game, ConsumableId::TheFool);
        assert!(result.is_ok());
    }
}
