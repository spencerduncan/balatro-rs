use crate::error::GameError;
use crate::game::Game;
use crate::joker::JokerId;
use crate::card::Card;

// Re-export legacy shop for backward compatibility
pub use legacy::*;

// Legacy shop implementation
mod legacy;

/// Enhanced shop trait for generating shop contents with weighted randomization
/// and support for various item types including jokers, consumables, vouchers, and packs.
pub trait ShopGenerator {
    /// Generate a complete shop based on current game state
    fn generate_shop(&self, game: &Game) -> EnhancedShop;
    
    /// Generate a specific booster pack
    fn generate_pack(&self, pack_type: PackType, game: &Game) -> Pack;
    
    /// Calculate generation weights based on game state
    fn calculate_weights(&self, game: &Game) -> ItemWeights;
    
    /// Reroll the shop contents
    fn reroll_shop(&self, current_shop: &EnhancedShop, game: &Game) -> EnhancedShop;
}

/// Represents all possible items that can appear in the shop
#[derive(Debug, Clone, PartialEq)]
pub enum ShopItem {
    /// A joker card identified by its ID
    Joker(JokerId),
    /// A consumable card (Tarot, Planet, Spectral)
    Consumable(ConsumableType),
    /// A voucher that provides permanent upgrades
    Voucher(VoucherId),
    /// A booster pack containing multiple cards
    Pack(PackType),
    /// A playing card that can be added to the deck
    PlayingCard(Card),
}

/// Types of consumable cards available in the shop
#[derive(Debug, Clone, PartialEq)]
pub enum ConsumableType {
    Tarot,
    Planet,
    Spectral,
}

/// Types of booster packs available in the shop
#[derive(Debug, Clone, PartialEq)]
pub enum PackType {
    /// Standard pack with playing cards
    Standard,
    /// Jumbo pack with more playing cards
    Jumbo,
    /// Mega pack with even more playing cards  
    Mega,
    /// Spectral pack with spectral cards
    Spectral,
    /// Standard pack with enhanced cards
    Enhanced,
    /// Variety pack with mixed content
    Variety,
}

/// Voucher identifiers for shop vouchers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
        self.slots.iter().any(|slot| slot.available && slot.item == *item)
    }
    
    /// Get the cost of a specific item if available
    pub fn get_item_cost(&self, item: &ShopItem) -> Option<usize> {
        self.slots.iter()
            .find(|slot| slot.available && slot.item == *item)
            .map(|slot| slot.cost)
    }
    
    /// Purchase an item from the shop
    pub fn purchase_item(&mut self, item: &ShopItem) -> Result<ShopItem, GameError> {
        let slot_index = self.slots.iter()
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
    use crate::game::Game;
    use crate::card::{Value, Suit};

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
        assert!(matches!(shop.slots[0].item, ShopItem::Joker(JokerId::Joker)));
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
        let jumbo_pack = PackType::Jumbo;
        let mega_pack = PackType::Mega;
        
        assert_ne!(standard_pack, jumbo_pack);
        assert_ne!(jumbo_pack, mega_pack);
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
        
        let purchased = shop.purchase_item(&item).expect("Should purchase successfully");
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
}