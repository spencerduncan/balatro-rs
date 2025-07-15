use crate::card::Card;
use crate::error::GameError;
use crate::game::Game;
use crate::joker::JokerId;

// Re-export legacy shop for backward compatibility
pub use legacy::*;

// Re-export pack types for convenience
pub use packs::PackType;

// Legacy shop implementation
mod legacy;

// Pack system implementation
pub mod packs;

/// Enhanced shop trait for generating shop contents with weighted randomization
/// and support for various item types including jokers, consumables, vouchers, and packs.
///
/// This trait defines the core functionality for an advanced shop system that can:
/// - Generate shops with multiple item types using weighted probabilities
/// - Create booster packs with appropriate contents
/// - Handle reroll mechanics with cost considerations
/// - Adapt generation based on game state (ante, money, existing jokers, etc.)
///
/// # Design Philosophy
///
/// The shop generator follows a data-driven approach where game state influences
/// the probability weights for different item types. This allows for:
/// - Dynamic difficulty scaling based on ante progression
/// - Adaptive item generation based on player resources
/// - Extensible voucher effects that modify shop generation
///
/// # Implementation Example
///
/// ```rust,ignore
/// use balatro_rs::shop::{ShopGenerator, EnhancedShop, ItemWeights};
///
/// struct StandardShopGenerator;
///
/// impl ShopGenerator for StandardShopGenerator {
///     fn generate_shop(&self, game: &Game) -> EnhancedShop {
///         let weights = self.calculate_weights(game);
///         // Use weights to randomly select items for each slot
///         EnhancedShop::new()
///     }
/// }
/// ```
pub trait ShopGenerator {
    /// Generate a complete shop based on current game state.
    ///
    /// This method should create a new shop with items appropriate for the current
    /// game state. Factors to consider:
    /// - Current ante (affects item rarity and costs)
    /// - Player money (affects affordability considerations)
    /// - Existing jokers (avoid duplicates or create synergies)
    /// - Active vouchers (that modify shop generation)
    ///
    /// # Arguments
    /// * `game` - Current game state containing all relevant context
    ///
    /// # Returns
    /// A fully populated `EnhancedShop` ready for player interaction
    fn generate_shop(&self, game: &Game) -> EnhancedShop;

    /// Generate a specific booster pack with appropriate contents.
    ///
    /// Pack contents should vary based on pack type and game state:
    /// - Standard packs: Basic playing cards
    /// - Jumbo/Mega packs: More cards with potential enhancements
    /// - Spectral packs: Consumable cards with powerful effects
    ///
    /// # Arguments
    /// * `pack_type` - The type of pack to generate
    /// * `game` - Current game state for contextual generation
    ///
    /// # Returns
    /// A `Pack` containing appropriate items for the specified type
    fn generate_pack(&self, pack_type: PackType, game: &Game) -> Pack;

    /// Calculate generation weights based on current game state.
    ///
    /// Weights determine the relative probability of each item type appearing
    /// in shop generation. This method should consider:
    /// - Game progression (higher ante = more rare items)
    /// - Player resources (more money = higher cost items viable)
    /// - Existing build synergies (recommend complementary items)
    ///
    /// # Arguments
    /// * `game` - Current game state to analyze
    ///
    /// # Returns
    /// `ItemWeights` struct containing probability weights for each item type
    fn calculate_weights(&self, game: &Game) -> ItemWeights;

    /// Reroll the shop contents while preserving shop state.
    ///
    /// This method should generate a new shop while maintaining:
    /// - Reroll count and costs
    /// - Any active shop modifiers
    /// - Voucher effects on shop generation
    ///
    /// # Arguments
    /// * `current_shop` - The shop being rerolled
    /// * `game` - Current game state
    ///
    /// # Returns
    /// A new `EnhancedShop` with fresh contents but preserved state
    fn reroll_shop(&self, current_shop: &EnhancedShop, game: &Game) -> EnhancedShop;
}

/// Represents all possible items that can appear in the shop.
///
/// This enum encompasses every type of purchasable item in the enhanced shop system,
/// providing a unified interface for shop generation, display, and purchase mechanics.
///
/// # Item Categories
///
/// - **Jokers**: Permanent modifiers that affect scoring and game mechanics
/// - **Consumables**: Single-use cards with powerful immediate effects
/// - **Vouchers**: Permanent upgrades that modify game rules or shop behavior
/// - **Packs**: Containers with multiple items for batch purchasing
/// - **Playing Cards**: Individual cards that can be added to the player's deck
///
/// # Design Considerations
///
/// Each item type has different:
/// - **Cost structures**: Jokers use rarity-based pricing, packs have fixed costs
/// - **Availability**: Some items may be limited by game progression
/// - **Effects**: Items affect different aspects of the game (scoring, economy, deck composition)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ShopItem {
    /// A joker card identified by its ID.
    ///
    /// Jokers provide ongoing effects during gameplay and are typically
    /// the most expensive and impactful shop items.
    Joker(JokerId),

    /// A consumable card (Tarot, Planet, Spectral).
    ///
    /// Consumables provide immediate effects when used and are consumed
    /// in the process. They offer powerful but limited-use benefits.
    Consumable(ConsumableType),

    /// A voucher that provides permanent upgrades.
    ///
    /// Vouchers modify game rules permanently, such as increasing shop
    /// slots, reducing costs, or adding new mechanics.
    Voucher(VoucherId),

    /// A booster pack containing multiple cards.
    ///
    /// Packs provide value by containing multiple items at a potentially
    /// discounted rate compared to individual purchases.
    Pack(PackType),

    /// A playing card that can be added to the deck.
    ///
    /// Individual playing cards allow precise deck customization and
    /// can have special enhancements or properties.
    PlayingCard(Card),
}

impl ShopItem {
    /// Get the base cost of this item in coins.
    ///
    /// This provides the default cost before any modifiers are applied.
    /// Actual shop prices may differ due to vouchers, sales, or other effects.
    pub fn base_cost(&self) -> usize {
        match self {
            ShopItem::Joker(joker_id) => {
                // Would need to look up joker rarity, using placeholder for now
                match joker_id {
                    JokerId::Joker => 3, // Common joker base cost
                    _ => 3,              // Default for now - would implement proper rarity lookup
                }
            }
            ShopItem::Consumable(consumable_type) => match consumable_type {
                ConsumableType::Tarot => 3,
                ConsumableType::Planet => 3,
                ConsumableType::Spectral => 4,
            },
            ShopItem::Voucher(_) => 10, // Standard voucher cost
            ShopItem::Pack(pack_type) => pack_type.base_cost(),
            ShopItem::PlayingCard(_) => 2, // Standard playing card cost
        }
    }

    /// Get a human-readable name for this item.
    pub fn display_name(&self) -> String {
        match self {
            ShopItem::Joker(joker_id) => format!("{joker_id:?} Joker"),
            ShopItem::Consumable(consumable_type) => format!("{consumable_type:?} Card"),
            ShopItem::Voucher(voucher_id) => format!("{voucher_id:?} Voucher"),
            ShopItem::Pack(pack_type) => format!("{pack_type:?} Pack"),
            ShopItem::PlayingCard(card) => format!("{card}"),
        }
    }

    /// Check if this item type is affected by specific voucher effects.
    pub fn is_affected_by_voucher(&self, voucher: VoucherId) -> bool {
        match voucher {
            VoucherId::Overstock => true, // Affects all shop items
            VoucherId::ClearancePackage => matches!(self, ShopItem::Pack(_)),
            VoucherId::Coupon => matches!(self, ShopItem::Joker(_)),
            _ => false,
        }
    }
}

/// Types of consumable cards available in the shop
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ConsumableType {
    Tarot,
    Planet,
    Spectral,
}

/// Voucher identifiers for shop vouchers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum VoucherId {
    Overstock,
    ClearancePackage,
    Liquidation,
    Coupon,
    Poll,
    Hone,
    Glow,
    Reroll,
    // ... more vouchers to be added
}

/// Individual slot in the enhanced shop
#[derive(Debug, Clone)]
pub struct ShopSlot {
    /// The item in this slot
    pub item: ShopItem,
    /// Cost to purchase this item
    pub cost: usize,
    /// Whether this slot is currently available for purchase
    pub available: bool,
    /// Any special modifiers affecting this slot
    pub modifiers: Vec<SlotModifier>,
}

/// Special modifiers that can affect shop slots
#[derive(Debug, Clone)]
pub enum SlotModifier {
    /// Item costs 50% less
    HalfPrice,
    /// Item is free
    Free,
    /// Item gives bonus money when purchased
    Bonus(usize),
    /// Item is on sale
    Sale(f32), // Percentage discount (0.0 to 1.0)
}

/// Weights for different item types in shop generation
#[derive(Debug, Clone)]
pub struct ItemWeights {
    /// Weight for joker generation
    pub joker_weight: f32,
    /// Weight for consumable generation
    pub consumable_weight: f32,
    /// Weight for voucher generation  
    pub voucher_weight: f32,
    /// Weight for pack generation
    pub pack_weight: f32,
    /// Weight for playing card generation
    pub playing_card_weight: f32,
}

impl Default for ItemWeights {
    fn default() -> Self {
        Self {
            joker_weight: 50.0,
            consumable_weight: 20.0,
            voucher_weight: 10.0,
            pack_weight: 15.0,
            playing_card_weight: 5.0,
        }
    }
}

/// A booster pack containing multiple items
#[derive(Debug, Clone)]
pub struct Pack {
    /// Type of pack
    pub pack_type: PackType,
    /// Items contained in the pack
    pub contents: Vec<ShopItem>,
    /// Cost to purchase the pack
    pub cost: usize,
}

/// Enhanced shop with support for multiple item types and weighted generation
#[derive(Debug, Clone)]
pub struct EnhancedShop {
    /// All slots in the shop
    pub slots: Vec<ShopSlot>,
    /// Number of rerolls available
    pub rerolls_remaining: usize,
    /// Cost per reroll
    pub reroll_cost: usize,
    /// Generation weights used for this shop
    pub weights: ItemWeights,
}

impl EnhancedShop {
    /// Create a new empty enhanced shop
    pub fn new() -> Self {
        Self {
            slots: Vec::new(),
            rerolls_remaining: 3,
            reroll_cost: 5,
            weights: ItemWeights::default(),
        }
    }

    /// Check if a specific item is available in the shop
    pub fn has_item(&self, item: &ShopItem) -> bool {
        self.slots
            .iter()
            .any(|slot| slot.available && slot.item == *item)
    }

    /// Get the cost of a specific item if available
    pub fn get_item_cost(&self, item: &ShopItem) -> Option<usize> {
        self.slots
            .iter()
            .find(|slot| slot.available && slot.item == *item)
            .map(|slot| slot.cost)
    }

    /// Purchase an item from the shop
    pub fn purchase_item(&mut self, item: &ShopItem) -> Result<ShopItem, GameError> {
        let slot_index = self
            .slots
            .iter()
            .position(|slot| slot.available && slot.item == *item)
            .ok_or(GameError::InvalidAction)?;

        self.slots[slot_index].available = false;
        Ok(item.clone())
    }
}

impl Default for EnhancedShop {
    fn default() -> Self {
        Self::new()
    }
}

/// Interface for pack selection and opening mechanics
pub trait PackSelector {
    /// Select items from a pack based on pack type and game state
    fn select_from_pack(&self, pack: &Pack, game: &Game) -> Vec<ShopItem>;

    /// Get the maximum number of items that can be selected from a pack
    fn max_selections(&self, pack_type: PackType) -> usize;

    /// Check if a pack selection is valid
    fn is_valid_selection(&self, pack: &Pack, selected: &[ShopItem]) -> bool;
}

/// Interface for reroll mechanics
pub trait RerollMechanics {
    /// Calculate the cost of a reroll based on game state
    fn calculate_reroll_cost(&self, current_cost: usize, game: &Game) -> usize;

    /// Check if a reroll is available
    fn can_reroll(&self, shop: &EnhancedShop, game: &Game) -> bool;

    /// Apply voucher effects to reroll mechanics
    fn apply_voucher_effects(&self, base_cost: usize, vouchers: &[VoucherId]) -> usize;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::{Suit, Value};
    use crate::game::Game;

    /// Test implementation of ShopGenerator for testing purposes
    #[derive(Debug)]
    struct TestShopGenerator;

    impl ShopGenerator for TestShopGenerator {
        fn generate_shop(&self, _game: &Game) -> EnhancedShop {
            let mut shop = EnhancedShop::new();
            shop.slots.push(ShopSlot {
                item: ShopItem::Joker(JokerId::Joker),
                cost: 3,
                available: true,
                modifiers: vec![],
            });
            shop
        }

        fn generate_pack(&self, pack_type: PackType, _game: &Game) -> Pack {
            Pack {
                pack_type,
                contents: vec![ShopItem::PlayingCard(Card::new(Value::Ace, Suit::Heart))],
                cost: 4,
            }
        }

        fn calculate_weights(&self, _game: &Game) -> ItemWeights {
            ItemWeights::default()
        }

        fn reroll_shop(&self, _current_shop: &EnhancedShop, game: &Game) -> EnhancedShop {
            self.generate_shop(game)
        }
    }

    #[test]
    fn test_shop_generator_trait() {
        let generator = TestShopGenerator;
        let game = Game::new(crate::config::Config::default());

        let shop = generator.generate_shop(&game);
        assert_eq!(shop.slots.len(), 1);
        assert!(matches!(
            shop.slots[0].item,
            ShopItem::Joker(JokerId::Joker)
        ));
    }

    #[test]
    fn test_shop_item_enum() {
        let joker_item = ShopItem::Joker(JokerId::Joker);
        let consumable_item = ShopItem::Consumable(ConsumableType::Tarot);
        let voucher_item = ShopItem::Voucher(VoucherId::Overstock);
        let pack_item = ShopItem::Pack(PackType::Standard);

        // Test that different item types are not equal
        assert_ne!(joker_item, consumable_item);
        assert_ne!(consumable_item, voucher_item);
        assert_ne!(voucher_item, pack_item);
    }

    #[test]
    fn test_pack_type_enum() {
        let standard_pack = PackType::Standard;
        let buffoon_pack = PackType::Buffoon;
        let mega_pack = PackType::MegaBuffoon;

        assert_ne!(standard_pack, buffoon_pack);
        assert_ne!(buffoon_pack, mega_pack);
    }

    #[test]
    fn test_shop_slot_structure() {
        let slot = ShopSlot {
            item: ShopItem::Joker(JokerId::Joker),
            cost: 3,
            available: true,
            modifiers: vec![SlotModifier::HalfPrice],
        };

        assert_eq!(slot.cost, 3);
        assert!(slot.available);
        assert_eq!(slot.modifiers.len(), 1);
    }

    #[test]
    fn test_item_weights_default() {
        let weights = ItemWeights::default();
        assert_eq!(weights.joker_weight, 50.0);
        assert_eq!(weights.consumable_weight, 20.0);
        assert_eq!(weights.voucher_weight, 10.0);
        assert_eq!(weights.pack_weight, 15.0);
        assert_eq!(weights.playing_card_weight, 5.0);
    }

    #[test]
    fn test_enhanced_shop_creation() {
        let shop = EnhancedShop::new();
        assert_eq!(shop.slots.len(), 0);
        assert_eq!(shop.rerolls_remaining, 3);
        assert_eq!(shop.reroll_cost, 5);
    }

    #[test]
    fn test_enhanced_shop_has_item() {
        let mut shop = EnhancedShop::new();
        let item = ShopItem::Joker(JokerId::Joker);

        assert!(!shop.has_item(&item));

        shop.slots.push(ShopSlot {
            item: item.clone(),
            cost: 3,
            available: true,
            modifiers: vec![],
        });

        assert!(shop.has_item(&item));
    }

    #[test]
    fn test_enhanced_shop_purchase_item() {
        let mut shop = EnhancedShop::new();
        let item = ShopItem::Joker(JokerId::Joker);

        shop.slots.push(ShopSlot {
            item: item.clone(),
            cost: 3,
            available: true,
            modifiers: vec![],
        });

        let purchased = shop
            .purchase_item(&item)
            .expect("Should purchase successfully");
        assert_eq!(purchased, item);
        assert!(!shop.slots[0].available);
    }

    #[test]
    fn test_pack_generation() {
        let generator = TestShopGenerator;
        let game = Game::new(crate::config::Config::default());

        let pack = generator.generate_pack(PackType::Standard, &game);
        assert_eq!(pack.pack_type, PackType::Standard);
        assert_eq!(pack.contents.len(), 1);
        assert_eq!(pack.cost, 4);
    }

    #[test]
    fn test_shop_item_base_cost() {
        let joker_item = ShopItem::Joker(JokerId::Joker);
        let tarot_item = ShopItem::Consumable(ConsumableType::Tarot);
        let voucher_item = ShopItem::Voucher(VoucherId::Overstock);
        let pack_item = ShopItem::Pack(PackType::Standard);
        let card_item = ShopItem::PlayingCard(Card::new(Value::Ace, Suit::Heart));

        assert_eq!(joker_item.base_cost(), 3);
        assert_eq!(tarot_item.base_cost(), 3);
        assert_eq!(voucher_item.base_cost(), 10);
        assert_eq!(pack_item.base_cost(), 4);
        assert_eq!(card_item.base_cost(), 2);
    }

    #[test]
    fn test_shop_item_display_name() {
        let joker_item = ShopItem::Joker(JokerId::Joker);
        let tarot_item = ShopItem::Consumable(ConsumableType::Tarot);
        let voucher_item = ShopItem::Voucher(VoucherId::Overstock);

        assert_eq!(joker_item.display_name(), "Joker Joker");
        assert_eq!(tarot_item.display_name(), "Tarot Card");
        assert_eq!(voucher_item.display_name(), "Overstock Voucher");
    }

    #[test]
    fn test_shop_item_voucher_effects() {
        let joker_item = ShopItem::Joker(JokerId::Joker);
        let pack_item = ShopItem::Pack(PackType::Standard);
        let card_item = ShopItem::PlayingCard(Card::new(Value::Ace, Suit::Heart));

        // Overstock affects all items
        assert!(joker_item.is_affected_by_voucher(VoucherId::Overstock));
        assert!(pack_item.is_affected_by_voucher(VoucherId::Overstock));
        assert!(card_item.is_affected_by_voucher(VoucherId::Overstock));

        // ClearancePackage only affects packs
        assert!(!joker_item.is_affected_by_voucher(VoucherId::ClearancePackage));
        assert!(pack_item.is_affected_by_voucher(VoucherId::ClearancePackage));
        assert!(!card_item.is_affected_by_voucher(VoucherId::ClearancePackage));

        // Coupon only affects jokers
        assert!(joker_item.is_affected_by_voucher(VoucherId::Coupon));
        assert!(!pack_item.is_affected_by_voucher(VoucherId::Coupon));
        assert!(!card_item.is_affected_by_voucher(VoucherId::Coupon));
    }

    #[test]
    fn test_pack_type_costs() {
        assert_eq!(ShopItem::Pack(PackType::Standard).base_cost(), 4);
        assert_eq!(ShopItem::Pack(PackType::Buffoon).base_cost(), 4);
        assert_eq!(ShopItem::Pack(PackType::Arcana).base_cost(), 4);
        assert_eq!(ShopItem::Pack(PackType::Celestial).base_cost(), 4);
        assert_eq!(ShopItem::Pack(PackType::Spectral).base_cost(), 4);
        assert_eq!(ShopItem::Pack(PackType::MegaBuffoon).base_cost(), 8);
        assert_eq!(ShopItem::Pack(PackType::MegaArcana).base_cost(), 8);
        assert_eq!(ShopItem::Pack(PackType::MegaCelestial).base_cost(), 8);
    }

    #[test]
    fn test_consumable_type_costs() {
        assert_eq!(ShopItem::Consumable(ConsumableType::Tarot).base_cost(), 3);
        assert_eq!(ShopItem::Consumable(ConsumableType::Planet).base_cost(), 3);
        assert_eq!(
            ShopItem::Consumable(ConsumableType::Spectral).base_cost(),
            4
        );
    }

    #[test]
    fn test_enhanced_shop_get_item_cost() {
        let mut shop = EnhancedShop::new();
        let item = ShopItem::Joker(JokerId::Joker);

        // Item not in shop
        assert_eq!(shop.get_item_cost(&item), None);

        // Add item to shop
        shop.slots.push(ShopSlot {
            item: item.clone(),
            cost: 5,
            available: true,
            modifiers: vec![],
        });

        // Item in shop and available
        assert_eq!(shop.get_item_cost(&item), Some(5));

        // Mark item as unavailable
        shop.slots[0].available = false;
        assert_eq!(shop.get_item_cost(&item), None);
    }

    #[test]
    fn test_slot_modifiers() {
        let slot = ShopSlot {
            item: ShopItem::Joker(JokerId::Joker),
            cost: 10,
            available: true,
            modifiers: vec![
                SlotModifier::HalfPrice,
                SlotModifier::Sale(0.2), // 20% off
                SlotModifier::Bonus(5),
            ],
        };

        assert_eq!(slot.modifiers.len(), 3);
        assert!(matches!(slot.modifiers[0], SlotModifier::HalfPrice));
        assert!(matches!(slot.modifiers[1], SlotModifier::Sale(0.2)));
        assert!(matches!(slot.modifiers[2], SlotModifier::Bonus(5)));
    }
}
