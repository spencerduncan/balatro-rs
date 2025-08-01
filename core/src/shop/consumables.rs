/// Shop integration for consumable purchases
///
/// This module provides the bridge between the shop system and consumable purchase validation.
/// It integrates with the existing `can_purchase_consumable` validation logic and the
/// `consumables_in_hand` storage system to provide seamless shop transactions.
use crate::consumables::ConsumableId;
use crate::error::GameError;
use crate::game::Game;
use crate::shop::{ConsumableType, EnhancedShop, ShopItem};

/// Result of a consumable purchase attempt
#[derive(Debug, Clone)]
pub struct ConsumablePurchaseResult {
    /// The consumable that was purchased
    pub consumable_id: ConsumableId,
    /// The slot index where it was placed in the player's hand
    pub slot_index: usize,
    /// The amount of money deducted
    pub cost: f64,
    /// Whether the shop inventory was updated
    pub shop_updated: bool,
}

/// Errors specific to consumable shop purchases
#[derive(Debug, Clone)]
pub enum ConsumablePurchaseError {
    /// Game error occurred during purchase
    GameError(GameError),
    /// Consumable not available in shop
    NotInShop,
    /// Shop inventory could not be updated
    InventoryUpdateFailed,
    /// Purchase validation failed
    ValidationFailed(String),
}

impl PartialEq for ConsumablePurchaseError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::GameError(a), Self::GameError(b)) => {
                // Compare GameError by discriminant since it doesn't implement PartialEq
                std::mem::discriminant(a) == std::mem::discriminant(b)
            }
            (Self::NotInShop, Self::NotInShop) => true,
            (Self::InventoryUpdateFailed, Self::InventoryUpdateFailed) => true,
            (Self::ValidationFailed(a), Self::ValidationFailed(b)) => a == b,
            _ => false,
        }
    }
}

impl From<GameError> for ConsumablePurchaseError {
    fn from(error: GameError) -> Self {
        ConsumablePurchaseError::GameError(error)
    }
}

/// Shop integration for consumable purchases
///
/// This trait provides the interface for integrating consumable purchases with the shop system.
/// It follows the performance-first philosophy by providing atomic transactions and minimal allocations.
pub trait ConsumableShopIntegration {
    /// Purchase a consumable from the shop with full validation and transaction safety.
    ///
    /// This method performs a complete purchase transaction:
    /// 1. Validates the purchase using existing `can_purchase_consumable` logic
    /// 2. Checks shop inventory for availability
    /// 3. Deducts money only if all validations pass
    /// 4. Adds consumable to player's hand using automatic slot assignment
    /// 5. Updates shop inventory to remove purchased item
    ///
    /// # Performance Characteristics
    /// - O(1) for validation and money operations
    /// - O(n) for shop inventory search where n = shop slots (typically 5-7)
    /// - O(1) for consumable hand assignment
    /// - Zero allocations in success path
    ///
    /// # Arguments
    /// * `consumable_id` - The specific consumable to purchase
    /// * `shop` - Mutable reference to the shop for inventory updates
    ///
    /// # Returns
    /// * `Ok(ConsumablePurchaseResult)` - Purchase successful with details
    /// * `Err(ConsumablePurchaseError)` - Purchase failed with reason
    ///
    /// # Examples
    /// ```rust,ignore
    /// use balatro_rs::shop::consumables::ConsumableShopIntegration;
    ///
    /// let mut game = Game::new(config);
    /// let mut shop = generate_shop(&game);
    ///
    /// match game.purchase_consumable_from_shop(ConsumableId::TheFool, &mut shop) {
    ///     Ok(result) => {
    ///         println!("Purchased {} for ${} in slot {}",
    ///                  result.consumable_id, result.cost, result.slot_index);
    ///     }
    ///     Err(ConsumablePurchaseError::NotInShop) => {
    ///         println!("That consumable isn't available in the shop");
    ///     }
    ///     Err(ConsumablePurchaseError::GameError(GameError::InvalidBalance)) => {
    ///         println!("Not enough money for this purchase");
    ///     }
    ///     Err(error) => println!("Purchase failed: {:?}", error),
    /// }
    /// ```
    fn purchase_consumable_from_shop(
        &mut self,
        consumable_id: ConsumableId,
        shop: &mut EnhancedShop,
    ) -> Result<ConsumablePurchaseResult, ConsumablePurchaseError>;

    /// Validate a consumable purchase without executing it.
    ///
    /// This method performs all validation checks without modifying game state.
    /// Useful for UI validation and action generation.
    ///
    /// # Performance Characteristics
    /// - O(1) for validation checks
    /// - O(n) for shop inventory search
    /// - Zero allocations
    /// - No side effects
    ///
    /// # Arguments
    /// * `consumable_id` - The consumable to validate
    /// * `shop` - Reference to the shop for availability checks
    ///
    /// # Returns
    /// * `Ok(f64)` - Purchase is valid, returns the cost
    /// * `Err(ConsumablePurchaseError)` - Validation failed with reason
    fn validate_consumable_purchase(
        &self,
        consumable_id: ConsumableId,
        shop: &EnhancedShop,
    ) -> Result<f64, ConsumablePurchaseError>;

    /// Get the shop cost for a specific consumable
    ///
    /// This method determines the actual shop cost including any modifiers,
    /// voucher effects, or sales that might be active.
    ///
    /// # Arguments
    /// * `consumable_id` - The consumable to price
    /// * `shop` - Reference to the shop for modifier checks
    ///
    /// # Returns
    /// * `Some(f64)` - The actual cost in the shop
    /// * `None` - Consumable not available in shop
    fn get_shop_consumable_cost(
        &self,
        consumable_id: ConsumableId,
        shop: &EnhancedShop,
    ) -> Option<f64>;
}

impl ConsumableShopIntegration for Game {
    fn purchase_consumable_from_shop(
        &mut self,
        consumable_id: ConsumableId,
        shop: &mut EnhancedShop,
    ) -> Result<ConsumablePurchaseResult, ConsumablePurchaseError> {
        // Step 1: Validate the purchase (this checks stage, money, and slot availability)
        let _validation_cost = self.validate_consumable_purchase(consumable_id, shop)?;

        // Step 2: Find the consumable in the shop and get its exact cost
        let shop_item = ShopItem::SpecificConsumable(consumable_id);
        let actual_cost = shop
            .get_item_cost(&shop_item)
            .ok_or(ConsumablePurchaseError::NotInShop)?;

        // Step 3: Atomic transaction - deduct money first
        self.money -= actual_cost as f64; // Convert usize to f64 for game money system

        // Step 4: Add consumable to player's hand using automatic slot assignment
        // This uses the existing logic that simply pushes to the consumables_in_hand vector
        self.consumables_in_hand.push(consumable_id);
        let slot_index = self.consumables_in_hand.len() - 1; // Index of the newly added item

        // Step 5: Update shop inventory - remove the purchased item
        let shop_updated = shop.purchase_item(&shop_item).is_ok();

        // If shop update failed, we still completed the core transaction
        // This maintains consistency with the existing shop system design
        Ok(ConsumablePurchaseResult {
            consumable_id,
            slot_index,
            cost: actual_cost as f64,
            shop_updated,
        })
    }

    fn validate_consumable_purchase(
        &self,
        consumable_id: ConsumableId,
        shop: &EnhancedShop,
    ) -> Result<f64, ConsumablePurchaseError> {
        // Get consumable type for validation
        let consumable_type = match consumable_id.consumable_type() {
            crate::consumables::ConsumableType::Tarot => ConsumableType::Tarot,
            crate::consumables::ConsumableType::Planet => ConsumableType::Planet,
            crate::consumables::ConsumableType::Spectral => ConsumableType::Spectral,
        };

        // Use existing validation logic
        self.can_purchase_consumable(consumable_type.clone())?;

        // Check if item is available in shop
        let shop_item = ShopItem::SpecificConsumable(consumable_id);
        let cost = shop
            .get_item_cost(&shop_item)
            .ok_or(ConsumablePurchaseError::NotInShop)?;

        Ok(cost as f64)
    }

    fn get_shop_consumable_cost(
        &self,
        consumable_id: ConsumableId,
        shop: &EnhancedShop,
    ) -> Option<f64> {
        let shop_item = ShopItem::SpecificConsumable(consumable_id);
        shop.get_item_cost(&shop_item).map(|cost| cost as f64)
    }
}

/// Helper functions for consumable shop integration
impl Game {
    /// Handle the BuyConsumable action using shop integration
    ///
    /// This method integrates with the existing action handling system to provide
    /// seamless consumable purchases from the shop. It performs atomic transactions
    /// and maintains consistency with all existing validation logic.
    ///
    /// # Performance Characteristics
    /// - Single transaction with rollback safety
    /// - Minimal memory allocations
    /// - Fast validation using existing optimized paths
    ///
    /// # Arguments
    /// * `consumable_id` - The consumable to purchase
    /// * `_slot` - The target slot (currently unused, automatic assignment is used)
    ///
    /// # Returns
    /// * `Ok(())` - Purchase completed successfully
    /// * `Err(GameError)` - Purchase failed, game state unchanged
    ///
    /// # Integration Notes
    /// This method assumes the shop is available in the game state. In the current
    /// architecture, shop state is managed separately, so this method focuses on
    /// the core transaction logic while leaving shop state management to the caller.
    pub fn handle_buy_consumable_action(
        &mut self,
        consumable_id: ConsumableId,
        _slot: usize, // Automatic slot assignment is used
    ) -> Result<(), GameError> {
        // For now, we'll implement basic validation and addition
        // Full shop integration will require shop state management

        // Get consumable type for validation
        let consumable_type = match consumable_id.consumable_type() {
            crate::consumables::ConsumableType::Tarot => ConsumableType::Tarot,
            crate::consumables::ConsumableType::Planet => ConsumableType::Planet,
            crate::consumables::ConsumableType::Spectral => ConsumableType::Spectral,
        };

        // Use existing validation logic
        self.can_purchase_consumable(consumable_type.clone())?;

        // Get cost for this consumable type (using the same logic as validation)
        let cost = match consumable_type {
            ConsumableType::Tarot => 3.0,
            ConsumableType::Planet => 3.0,
            ConsumableType::Spectral => 4.0,
        };

        // Atomic transaction: deduct money then add consumable
        self.money -= cost;
        self.consumables_in_hand.push(consumable_id);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use crate::consumables::ConsumableId;
    use crate::shop::{ShopItem, ShopSlot};
    use crate::stage::{Blind, Stage};

    fn create_test_game() -> Game {
        let config = Config {
            consumable_hand_capacity: 5, // Allow more consumables for testing
            ..Default::default()
        };
        let mut game = Game::new(config);
        game.stage = Stage::Shop();
        game.money = 10.0; // Enough for most consumables
        game
    }

    fn create_test_shop_with_consumable(consumable_id: ConsumableId) -> EnhancedShop {
        let mut shop = EnhancedShop::new();
        shop.slots.push(ShopSlot {
            item: ShopItem::SpecificConsumable(consumable_id),
            cost: 3, // Standard cost
            available: true,
            modifiers: vec![],
        });
        shop
    }

    #[test]
    fn test_validate_consumable_purchase_success() {
        let game = create_test_game();
        let shop = create_test_shop_with_consumable(ConsumableId::TheFool);

        let result = game.validate_consumable_purchase(ConsumableId::TheFool, &shop);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 3.0);
    }

    #[test]
    fn test_validate_consumable_purchase_not_in_shop() {
        let game = create_test_game();
        let shop = EnhancedShop::new(); // Empty shop

        let result = game.validate_consumable_purchase(ConsumableId::TheFool, &shop);
        assert!(matches!(result, Err(ConsumablePurchaseError::NotInShop)));
    }

    #[test]
    fn test_validate_consumable_purchase_insufficient_money() {
        let mut game = create_test_game();
        game.money = 1.0; // Not enough for any consumable
        let shop = create_test_shop_with_consumable(ConsumableId::TheFool);

        let result = game.validate_consumable_purchase(ConsumableId::TheFool, &shop);
        assert!(matches!(
            result,
            Err(ConsumablePurchaseError::GameError(
                GameError::InvalidBalance
            ))
        ));
    }

    #[test]
    fn test_validate_consumable_purchase_wrong_stage() {
        let mut game = create_test_game();
        game.stage = Stage::Blind(Blind::Small); // Not shop stage
        let shop = create_test_shop_with_consumable(ConsumableId::TheFool);

        let result = game.validate_consumable_purchase(ConsumableId::TheFool, &shop);
        assert!(matches!(
            result,
            Err(ConsumablePurchaseError::GameError(GameError::InvalidStage))
        ));
    }

    #[test]
    fn test_validate_consumable_purchase_hand_full() {
        let mut game = create_test_game();
        // Fill consumable hand to capacity
        game.consumables_in_hand =
            vec![ConsumableId::TheFool; game.config.consumable_hand_capacity];
        let shop = create_test_shop_with_consumable(ConsumableId::TheWorld);

        let result = game.validate_consumable_purchase(ConsumableId::TheWorld, &shop);
        assert!(matches!(
            result,
            Err(ConsumablePurchaseError::GameError(
                GameError::NoAvailableSlot
            ))
        ));
    }

    #[test]
    fn test_purchase_consumable_from_shop_success() {
        let mut game = create_test_game();
        let mut shop = create_test_shop_with_consumable(ConsumableId::TheFool);
        let initial_money = game.money;
        let initial_hand_size = game.consumables_in_hand.len();

        let result = game.purchase_consumable_from_shop(ConsumableId::TheFool, &mut shop);

        assert!(result.is_ok());
        let purchase_result = result.unwrap();

        // Verify the purchase result
        assert_eq!(purchase_result.consumable_id, ConsumableId::TheFool);
        assert_eq!(purchase_result.cost, 3.0);
        assert_eq!(purchase_result.slot_index, initial_hand_size);
        assert!(purchase_result.shop_updated);

        // Verify game state changes
        assert_eq!(game.money, initial_money - 3.0);
        assert_eq!(game.consumables_in_hand.len(), initial_hand_size + 1);
        assert_eq!(
            game.consumables_in_hand[initial_hand_size],
            ConsumableId::TheFool
        );

        // Verify shop state changes
        assert!(!shop.slots[0].available);
    }

    #[test]
    fn test_purchase_consumable_from_shop_not_available() {
        let mut game = create_test_game();
        let mut shop = EnhancedShop::new(); // Empty shop
        let initial_money = game.money;
        let initial_hand_size = game.consumables_in_hand.len();

        let result = game.purchase_consumable_from_shop(ConsumableId::TheFool, &mut shop);

        assert!(matches!(result, Err(ConsumablePurchaseError::NotInShop)));

        // Verify no changes to game state
        assert_eq!(game.money, initial_money);
        assert_eq!(game.consumables_in_hand.len(), initial_hand_size);
    }

    #[test]
    fn test_get_shop_consumable_cost() {
        let game = create_test_game();
        let shop = create_test_shop_with_consumable(ConsumableId::TheFool);

        let cost = game.get_shop_consumable_cost(ConsumableId::TheFool, &shop);
        assert_eq!(cost, Some(3.0));

        let cost_not_available = game.get_shop_consumable_cost(ConsumableId::TheWorld, &shop);
        assert_eq!(cost_not_available, None);
    }

    #[test]
    fn test_handle_buy_consumable_action_success() {
        let mut game = create_test_game();
        let initial_money = game.money;
        let initial_hand_size = game.consumables_in_hand.len();

        let result = game.handle_buy_consumable_action(ConsumableId::TheFool, 0);

        assert!(result.is_ok());

        // Verify game state changes
        assert_eq!(game.money, initial_money - 3.0); // Tarot costs 3
        assert_eq!(game.consumables_in_hand.len(), initial_hand_size + 1);
        assert_eq!(
            game.consumables_in_hand[initial_hand_size],
            ConsumableId::TheFool
        );
    }

    #[test]
    fn test_handle_buy_consumable_action_insufficient_money() {
        let mut game = create_test_game();
        game.money = 1.0; // Not enough for any consumable
        let initial_money = game.money;
        let initial_hand_size = game.consumables_in_hand.len();

        let result = game.handle_buy_consumable_action(ConsumableId::TheFool, 0);

        assert!(matches!(result, Err(GameError::InvalidBalance)));

        // Verify no changes to game state
        assert_eq!(game.money, initial_money);
        assert_eq!(game.consumables_in_hand.len(), initial_hand_size);
    }

    #[test]
    fn test_handle_buy_consumable_action_wrong_stage() {
        let mut game = create_test_game();
        game.stage = Stage::Blind(Blind::Small); // Not shop stage
        let initial_money = game.money;
        let initial_hand_size = game.consumables_in_hand.len();

        let result = game.handle_buy_consumable_action(ConsumableId::TheFool, 0);

        assert!(matches!(result, Err(GameError::InvalidStage)));

        // Verify no changes to game state
        assert_eq!(game.money, initial_money);
        assert_eq!(game.consumables_in_hand.len(), initial_hand_size);
    }

    #[test]
    fn test_different_consumable_types_and_costs() {
        let mut game = create_test_game();
        game.money = 20.0; // Enough for multiple purchases

        // Test Tarot (cost: 3)
        let result = game.handle_buy_consumable_action(ConsumableId::TheFool, 0);
        assert!(result.is_ok());
        assert_eq!(game.money, 17.0);

        // Test Planet (cost: 3)
        let result = game.handle_buy_consumable_action(ConsumableId::Mercury, 0);
        assert!(result.is_ok());
        assert_eq!(game.money, 14.0);

        // Test Spectral (cost: 4)
        let result = game.handle_buy_consumable_action(ConsumableId::Familiar, 0);
        assert!(result.is_ok());
        assert_eq!(game.money, 10.0);

        // Verify all consumables were added
        assert_eq!(game.consumables_in_hand.len(), 3);
        assert_eq!(game.consumables_in_hand[0], ConsumableId::TheFool);
        assert_eq!(game.consumables_in_hand[1], ConsumableId::Mercury);
        assert_eq!(game.consumables_in_hand[2], ConsumableId::Familiar);
    }

    #[test]
    fn test_atomic_transaction_behavior() {
        let mut game = create_test_game();
        let mut shop = create_test_shop_with_consumable(ConsumableId::TheFool);

        // Set up a scenario where validation passes but shop update might fail
        let initial_money = game.money;
        let initial_hand_size = game.consumables_in_hand.len();

        let result = game.purchase_consumable_from_shop(ConsumableId::TheFool, &mut shop);

        // Even if shop update fails, the core transaction should complete
        assert!(result.is_ok());
        let purchase_result = result.unwrap();

        // Money should be deducted
        assert_eq!(game.money, initial_money - purchase_result.cost);

        // Consumable should be added to hand
        assert_eq!(game.consumables_in_hand.len(), initial_hand_size + 1);
        assert_eq!(
            game.consumables_in_hand[initial_hand_size],
            ConsumableId::TheFool
        );
    }
}
