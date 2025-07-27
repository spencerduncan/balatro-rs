use crate::card::Card;
use crate::game::Game;
use crate::joker::{JokerId, JokerRarity};
use crate::joker_factory::JokerFactory;
use crate::rng::GameRng;
use crate::shop::{
    EnhancedShop, ItemWeights, Pack, PackType, RerollMechanics, ShopGenerator, ShopItem, ShopSlot,
    VoucherId,
};

/// Weighted random generator for shop items with support for rarity-based
/// joker generation, voucher modifications, and ante-based scaling.
///
/// This generator implements statistical distributions based on Balatro's
/// shop mechanics, providing:
/// - Joker rarity weights (Common: 70%, Uncommon: 25%, Rare: 4.5%, Legendary: 0.5%)
/// - Ante-based difficulty scaling
/// - Voucher effect applications
/// - Performance-optimized caching
/// - Cryptographically secure RNG
#[derive(Debug, Clone)]
pub struct WeightedGenerator {
    // Note: Removed weight_cache due to f64 money not implementing Hash
    // TODO: Reimplement caching with ordered-float or discrete money units if needed
    /// Random number generator state
    rng: GameRng,
    /// Reroll mechanics implementation
    reroll_mechanics: StandardRerollMechanics,
}

/// Cache key for weight calculations based on game state
#[derive(Debug, Clone, PartialEq)]
struct CacheKey {
    ante: usize,
    money: f64,
    vouchers: Vec<VoucherId>,
}

/// Standard reroll mechanics implementation with cost escalation and voucher effects
#[derive(Debug, Clone)]
pub struct StandardRerollMechanics;

impl RerollMechanics for StandardRerollMechanics {
    /// Calculate the cost of a reroll based on current cost and game state
    ///
    /// Reroll costs follow this escalation pattern:
    /// - Base cost: 5 coins
    /// - First reroll: 5 coins
    /// - Second reroll: 10 coins (+5)
    /// - Third reroll: 15 coins (+5)
    /// - Each subsequent reroll: +5 coins
    ///
    /// This provides a reasonable escalation that discourages excessive rerolling
    /// while still allowing strategic use of the mechanic.
    fn calculate_reroll_cost(&self, current_cost: usize, game: &Game) -> usize {
        // Base reroll cost escalation: each reroll increases cost by 5
        let base_increase = 5;
        let new_cost = current_cost + base_increase;

        // Apply voucher effects after base calculation
        let shop_vouchers: Vec<VoucherId> = self.get_active_shop_vouchers(game);
        self.apply_voucher_effects(new_cost, &shop_vouchers)
    }

    /// Check if a reroll is available based on shop state and game resources
    fn can_reroll(&self, shop: &EnhancedShop, game: &Game) -> bool {
        // Check if rerolls are remaining (if limited)
        if shop.rerolls_remaining == 0 {
            return false;
        }

        // Check if player has enough money for the reroll
        if (shop.reroll_cost as f64) > game.money {
            return false;
        }

        true
    }

    /// Apply voucher effects to reroll cost
    fn apply_voucher_effects(&self, base_cost: usize, vouchers: &[VoucherId]) -> usize {
        let mut final_cost = base_cost as f64;

        for &voucher in vouchers {
            match voucher {
                VoucherId::Reroll => {
                    // Reroll voucher provides free rerolls or reduces cost
                    final_cost = 0.0; // Makes rerolls free
                }
                VoucherId::Liquidation => {
                    // Liquidation gives 20% discount on all shop operations
                    final_cost *= 0.8;
                }
                _ => {} // Other vouchers don't affect reroll costs
            }
        }

        // Ensure minimum cost of 0 (free rerolls are possible)
        (final_cost.round() as usize).max(0)
    }
}

impl StandardRerollMechanics {
    /// Create a new StandardRerollMechanics instance
    pub fn new() -> Self {
        Self
    }

    /// Extract active shop vouchers from game state
    /// TODO: Replace with actual voucher system integration when available
    fn get_active_shop_vouchers(&self, _game: &Game) -> Vec<VoucherId> {
        // For now, return empty list since voucher system is not fully implemented
        // In the future, this would extract vouchers from game.vouchers
        vec![]
    }
}

impl Default for StandardRerollMechanics {
    fn default() -> Self {
        Self::new()
    }
}

impl WeightedGenerator {
    /// Create a new weighted generator with cryptographically secure RNG
    pub fn new() -> Self {
        Self {
            rng: GameRng::secure(),
            reroll_mechanics: StandardRerollMechanics::new(),
        }
    }

    /// Create a new weighted generator with deterministic RNG for testing
    pub fn for_testing(seed: u64) -> Self {
        Self {
            rng: GameRng::for_testing(seed),
            reroll_mechanics: StandardRerollMechanics::new(),
        }
    }

    /// Convert Ante enum to numeric value for calculations
    fn ante_to_number(&self, ante: crate::ante::Ante) -> usize {
        match ante {
            crate::ante::Ante::Zero => 0,
            crate::ante::Ante::One => 1,
            crate::ante::Ante::Two => 2,
            crate::ante::Ante::Three => 3,
            crate::ante::Ante::Four => 4,
            crate::ante::Ante::Five => 5,
            crate::ante::Ante::Six => 6,
            crate::ante::Ante::Seven => 7,
            crate::ante::Ante::Eight => 8,
        }
    }

    /// Get joker rarity weights based on ante progression
    #[allow(dead_code)]
    fn get_joker_rarity_weights(&self, ante: usize) -> [f64; 4] {
        // Base weights from issue requirements
        let base_common = 70.0;
        let base_uncommon = 25.0;
        let base_rare = 4.5;
        let base_legendary = 0.5;

        // Ante scaling: higher antes increase rare chances
        let scaling_factor = 1.0 + (ante as f64 - 1.0) * 0.1;

        let scaled_rare = base_rare * scaling_factor;
        let scaled_legendary = base_legendary * scaling_factor;

        // Reduce common/uncommon proportionally
        let total_increased = (scaled_rare - base_rare) + (scaled_legendary - base_legendary);
        let reduction_factor = total_increased / (base_common + base_uncommon);

        let scaled_common = (base_common * (1.0 - reduction_factor)).max(40.0);
        let scaled_uncommon = (base_uncommon * (1.0 - reduction_factor)).max(15.0);

        [
            scaled_common,
            scaled_uncommon,
            scaled_rare,
            scaled_legendary,
        ]
    }

    /// Apply voucher effects to base weights
    #[allow(dead_code)]
    fn apply_voucher_effects(
        &self,
        mut weights: ItemWeights,
        vouchers: &[VoucherId],
    ) -> ItemWeights {
        for &voucher in vouchers {
            match voucher {
                VoucherId::Overstock => {
                    // Increases all weights by 20%
                    weights.joker_weight *= 1.2;
                    weights.consumable_weight *= 1.2;
                    weights.voucher_weight *= 1.2;
                    weights.pack_weight *= 1.2;
                    weights.playing_card_weight *= 1.2;
                }
                VoucherId::ClearancePackage => {
                    // Increases pack weight by 50%
                    weights.pack_weight *= 1.5;
                }
                VoucherId::Coupon => {
                    // Increases joker weight by 30%
                    weights.joker_weight *= 1.3;
                }
                _ => {} // Other vouchers don't affect generation weights
            }
        }
        weights
    }

    /// Generate a weighted random joker based on rarity distribution
    #[allow(dead_code)]
    fn generate_weighted_joker(&mut self, ante: usize) -> Option<JokerId> {
        let weights = self.get_joker_rarity_weights(ante);
        let rarities = [
            JokerRarity::Common,
            JokerRarity::Uncommon,
            JokerRarity::Rare,
            JokerRarity::Legendary,
        ];

        // Select rarity using weighted distribution
        let selected_rarity = self
            .rng
            .choose_weighted(&rarities, |r| match r {
                JokerRarity::Common => weights[0],
                JokerRarity::Uncommon => weights[1],
                JokerRarity::Rare => weights[2],
                JokerRarity::Legendary => weights[3],
            })
            .unwrap_or(&JokerRarity::Common);

        // Get available jokers for this rarity
        let available_jokers = self.get_jokers_by_rarity(*selected_rarity);
        if available_jokers.is_empty() {
            return None;
        }

        // Randomly select from available jokers of this rarity
        let joker_index = self.rng.gen_range(0..available_jokers.len());
        Some(available_jokers[joker_index])
    }

    /// Get all jokers of a specific rarity
    #[allow(dead_code)]
    fn get_jokers_by_rarity(&self, rarity: JokerRarity) -> Vec<JokerId> {
        JokerFactory::get_by_rarity(rarity)
    }

    /// Generate a random joker using weighted rarity distribution
    fn generate_random_joker(&self, game: &Game) -> Option<ShopItem> {
        let ante_number = self.ante_to_number(game.ante_current);
        let weights = self.get_joker_rarity_weights(ante_number);
        let rarities = [
            JokerRarity::Common,
            JokerRarity::Uncommon,
            JokerRarity::Rare,
            JokerRarity::Legendary,
        ];

        // Select rarity using weighted distribution
        let selected_rarity = self
            .rng
            .choose_weighted(&rarities, |r| match r {
                JokerRarity::Common => weights[0],
                JokerRarity::Uncommon => weights[1],
                JokerRarity::Rare => weights[2],
                JokerRarity::Legendary => weights[3],
            })
            .unwrap_or(&JokerRarity::Common);

        let available_jokers = self.get_jokers_by_rarity(*selected_rarity);
        if !available_jokers.is_empty() {
            let joker = self.rng.choose(&available_jokers).unwrap();
            return Some(ShopItem::Joker(*joker));
        }

        // Fallback to basic joker if weighted selection fails
        Some(ShopItem::Joker(JokerId::Joker))
    }

    /// Generate a random consumable
    fn generate_random_consumable(&self) -> Option<ShopItem> {
        use crate::shop::ConsumableType;
        let consumable_types = [
            ConsumableType::Tarot,
            ConsumableType::Planet,
            ConsumableType::Spectral,
        ];

        let random_type = self.rng.choose(&consumable_types).unwrap().clone();
        Some(ShopItem::Consumable(random_type))
    }

    /// Generate a random voucher
    fn generate_random_voucher(&self) -> Option<ShopItem> {
        let vouchers = [
            VoucherId::Overstock,
            VoucherId::ClearancePackage,
            VoucherId::Liquidation,
            VoucherId::Coupon,
            VoucherId::Poll,
            VoucherId::Hone,
            VoucherId::Glow,
            VoucherId::Reroll,
        ];

        let random_voucher = *self.rng.choose(&vouchers).unwrap();
        Some(ShopItem::Voucher(random_voucher))
    }

    /// Generate a random pack
    fn generate_random_pack(&self) -> Option<ShopItem> {
        let pack_types = [
            PackType::Standard,
            PackType::Buffoon,
            PackType::Arcana,
            PackType::Celestial,
            PackType::Spectral,
            PackType::MegaBuffoon,
            PackType::MegaArcana,
            PackType::MegaCelestial,
            PackType::MegaSpectral,
        ];

        let random_pack = *self.rng.choose(&pack_types).unwrap();
        Some(ShopItem::Pack(random_pack))
    }

    /// Generate a random playing card
    fn generate_random_playing_card(&self) -> Option<ShopItem> {
        use crate::card::{Suit, Value};

        let suits = [Suit::Heart, Suit::Diamond, Suit::Club, Suit::Spade];
        let values = [
            Value::Ace,
            Value::Two,
            Value::Three,
            Value::Four,
            Value::Five,
            Value::Six,
            Value::Seven,
            Value::Eight,
            Value::Nine,
            Value::Ten,
            Value::Jack,
            Value::Queen,
            Value::King,
        ];

        let random_suit = *self.rng.choose(&suits).unwrap();
        let random_value = *self.rng.choose(&values).unwrap();

        Some(ShopItem::PlayingCard(Card::new(random_value, random_suit)))
    }

    /// Calculate final cost after applying voucher modifiers
    fn calculate_final_cost(&self, base_cost: usize, vouchers: &[VoucherId]) -> usize {
        let mut final_cost = base_cost as f64;

        for &voucher in vouchers {
            match voucher {
                VoucherId::Liquidation => final_cost *= 0.8, // 20% discount on all items
                VoucherId::Coupon => final_cost *= 0.9, // 10% discount (applies to jokers specifically)
                _ => {}                                 // Other vouchers don't affect cost directly
            }
        }

        (final_cost.round() as usize).max(1) // Ensure minimum cost of 1
    }
}

impl ShopGenerator for WeightedGenerator {
    fn generate_shop(&self, game: &Game) -> EnhancedShop {
        let mut shop = EnhancedShop::new();
        let weights = self.calculate_weights(game);
        shop.weights = weights.clone();

        // Standard shop has 5 slots
        const SHOP_SLOTS: usize = 5;

        // Create weighted distribution for item types
        let item_weights = [
            weights.joker_weight,
            weights.consumable_weight,
            weights.voucher_weight,
            weights.pack_weight,
            weights.playing_card_weight,
        ];

        for _ in 0..SHOP_SLOTS {
            let item_type_index = self
                .rng
                .choose_weighted(&[0, 1, 2, 3, 4], |&i| item_weights[i])
                .unwrap_or(&0);

            let item = match item_type_index {
                0 => self.generate_random_joker(game),
                1 => self.generate_random_consumable(),
                2 => self.generate_random_voucher(),
                3 => self.generate_random_pack(),
                4 => self.generate_random_playing_card(),
                _ => self.generate_random_joker(game), // Fallback
            };

            if let Some(shop_item) = item {
                let base_cost = shop_item.base_cost(&game.config);
                // For now, use empty voucher list since voucher system is placeholder
                let shop_vouchers: Vec<VoucherId> = vec![];
                let final_cost = self.calculate_final_cost(base_cost, &shop_vouchers);

                shop.slots.push(ShopSlot {
                    item: shop_item,
                    cost: final_cost,
                    available: true,
                    modifiers: vec![],
                });
            }
        }

        shop
    }

    fn generate_pack(&self, pack_type: PackType, game: &Game) -> Pack {
        let cost = pack_type.base_cost(&game.config);

        let mut contents = Vec::new();

        // Generate pack contents based on pack type
        match pack_type {
            PackType::Standard => {
                // 3 playing cards
                for _ in 0..3 {
                    if let Some(card) = self.generate_random_playing_card() {
                        contents.push(card);
                    }
                }
            }
            PackType::Buffoon | PackType::MegaBuffoon => {
                // Buffoon: 2 jokers, MegaBuffoon: 4 jokers
                let num_jokers = if pack_type == PackType::Buffoon { 2 } else { 4 };
                for _ in 0..num_jokers {
                    if let Some(joker) = self.generate_random_joker(game) {
                        contents.push(joker);
                    }
                }
            }
            PackType::Arcana | PackType::MegaArcana => {
                // Arcana: 2-3 tarot cards, MegaArcana: 4-6 tarot cards
                let (min, max) = if pack_type == PackType::Arcana {
                    (2, 3)
                } else {
                    (4, 6)
                };
                let num_items = self.rng.gen_range(min..=max);
                for _ in 0..num_items {
                    contents.push(ShopItem::Consumable(crate::shop::ConsumableType::Tarot));
                }
            }
            PackType::Enhanced => {
                // 3-4 enhanced playing cards
                let num_items = self.rng.gen_range(3..=4);
                for _ in 0..num_items {
                    if let Some(card) = self.generate_random_playing_card() {
                        // TODO: Apply enhancement to card
                        contents.push(card);
                    }
                }
            }
            PackType::Variety => {
                // Mixed content - 3-5 items of various types
                let num_items = self.rng.gen_range(3..=5);
                for _ in 0..num_items {
                    // For now, just add playing cards
                    if let Some(card) = self.generate_random_playing_card() {
                        contents.push(card);
                    }
                }
            }
            PackType::Celestial | PackType::MegaCelestial => {
                // Celestial: 2-3 planet cards, MegaCelestial: 4-6 planet cards
                let (min, max) = if pack_type == PackType::Celestial {
                    (2, 3)
                } else {
                    (4, 6)
                };
                let num_items = self.rng.gen_range(min..=max);
                for _ in 0..num_items {
                    contents.push(ShopItem::Consumable(crate::shop::ConsumableType::Planet));
                }
            }
            PackType::Spectral | PackType::MegaSpectral => {
                // Spectral: 2-3 spectral cards, MegaSpectral: 4-6 spectral cards
                let (min, max) = if pack_type == PackType::Spectral {
                    (2, 3)
                } else {
                    (4, 6)
                };
                let num_items = self.rng.gen_range(min..=max);
                for _ in 0..num_items {
                    contents.push(ShopItem::Consumable(crate::shop::ConsumableType::Spectral));
                }
            }
            PackType::Jumbo => {
                // Jumbo pack: 5 playing cards
                for _ in 0..5 {
                    if let Some(card) = self.generate_random_playing_card() {
                        contents.push(card);
                    }
                }
            }
            PackType::Mega => {
                // Mega pack: 7 playing cards
                for _ in 0..7 {
                    if let Some(card) = self.generate_random_playing_card() {
                        contents.push(card);
                    }
                }
            }
        }

        // Ensure pack has at least one item
        if contents.is_empty() {
            if let Some(fallback_item) = self.generate_random_playing_card() {
                contents.push(fallback_item);
            }
        }

        Pack {
            pack_type,
            contents,
            cost,
        }
    }

    fn calculate_weights(&self, game: &Game) -> ItemWeights {
        let ante_number = self.ante_to_number(game.ante_current);
        let money = game.money;

        // For now, use empty voucher list since voucher system is placeholder
        // In future, would convert from game vouchers to shop vouchers
        let shop_vouchers: Vec<VoucherId> = vec![];

        // Create cache key for performance optimization (currently unused)
        let _cache_key = CacheKey {
            ante: ante_number,
            money,
            vouchers: shop_vouchers.clone(),
        };

        // Check cache first (in real implementation, would need mutable access to use cache)
        // For now, always calculate fresh weights

        // Start with default weights
        let mut weights = ItemWeights::default();

        // Apply ante-based scaling
        // Higher antes should favor higher-value items
        let ante_scale = 1.0 + (ante_number as f64 * 0.15);

        // Adjust weights based on ante progression
        if ante_number >= 3 {
            weights.voucher_weight *= ante_scale * 0.8; // Vouchers become more valuable
            weights.pack_weight *= ante_scale * 0.9; // Packs become more valuable
        }

        if ante_number >= 5 {
            weights.joker_weight *= ante_scale; // Jokers become even more important
            weights.playing_card_weight *= 0.7; // Playing cards less important in late game
        }

        // Apply money-based adjustments
        // If player has lots of money, bias toward expensive items
        if money >= 50.0 {
            weights.voucher_weight *= 1.3;
            weights.pack_weight *= 1.2;
        } else if money <= 20.0 {
            // If low on money, favor cheaper items
            weights.playing_card_weight *= 1.4;
            weights.consumable_weight *= 1.2;
        }

        // Apply voucher effects
        weights = self.apply_voucher_effects(weights, &shop_vouchers);

        weights
    }

    fn reroll_shop(&self, current_shop: &EnhancedShop, game: &Game) -> EnhancedShop {
        // Check if reroll is possible
        if !self.reroll_mechanics.can_reroll(current_shop, game) {
            // Return current shop unchanged if reroll not possible
            return current_shop.clone();
        }

        // Generate new shop contents while preserving shop state
        let mut new_shop = self.generate_shop(game);

        // Calculate new reroll cost using the mechanics
        let new_reroll_cost = self
            .reroll_mechanics
            .calculate_reroll_cost(current_shop.reroll_cost, game);

        // Preserve and update shop state from the current shop
        new_shop.reroll_cost = new_reroll_cost;

        // Handle reroll count - for now we'll use unlimited rerolls (common in Balatro)
        // In the future, this could be limited by vouchers or game settings
        new_shop.rerolls_remaining = current_shop.rerolls_remaining.saturating_sub(0); // Unlimited for now

        // Preserve generation weights from current shop for consistency
        new_shop.weights = current_shop.weights.clone();

        new_shop
    }
}

impl Default for WeightedGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use std::collections::HashMap;

    #[test]
    fn test_weighted_generator_creation() {
        let _generator = WeightedGenerator::new();
        // Generator successfully created - no cache to test since it was removed for f64 compatibility
    }

    #[test]
    fn test_joker_rarity_base_weights() {
        let generator = WeightedGenerator::new();
        let weights = generator.get_joker_rarity_weights(1);

        // Test base weights from issue requirements
        assert_eq!(weights[0], 70.0); // Common
        assert_eq!(weights[1], 25.0); // Uncommon
        assert_eq!(weights[2], 4.5); // Rare
        assert_eq!(weights[3], 0.5); // Legendary
    }

    #[test]
    fn test_ante_based_weight_scaling() {
        let generator = WeightedGenerator::new();

        let ante1_weights = generator.get_joker_rarity_weights(1);
        let ante5_weights = generator.get_joker_rarity_weights(5);

        // Higher ante should increase rare chances
        assert!(ante5_weights[2] > ante1_weights[2]); // Rare weight increased
        assert!(ante5_weights[3] > ante1_weights[3]); // Legendary weight increased

        // Common weight should decrease to compensate
        assert!(ante5_weights[0] < ante1_weights[0]);
    }

    #[test]
    fn test_voucher_effect_overstock() {
        let generator = WeightedGenerator::new();
        let base_weights = ItemWeights::default();
        let vouchers = vec![VoucherId::Overstock];

        let modified_weights = generator.apply_voucher_effects(base_weights.clone(), &vouchers);

        // All weights should increase by 20%
        assert_eq!(
            modified_weights.joker_weight,
            base_weights.joker_weight * 1.2
        );
        assert_eq!(
            modified_weights.consumable_weight,
            base_weights.consumable_weight * 1.2
        );
        assert_eq!(
            modified_weights.voucher_weight,
            base_weights.voucher_weight * 1.2
        );
        assert_eq!(modified_weights.pack_weight, base_weights.pack_weight * 1.2);
        assert_eq!(
            modified_weights.playing_card_weight,
            base_weights.playing_card_weight * 1.2
        );
    }

    #[test]
    fn test_voucher_effect_clearance_package() {
        let generator = WeightedGenerator::new();
        let base_weights = ItemWeights::default();
        let vouchers = vec![VoucherId::ClearancePackage];

        let modified_weights = generator.apply_voucher_effects(base_weights.clone(), &vouchers);

        // Only pack weight should increase by 50%
        assert_eq!(modified_weights.pack_weight, base_weights.pack_weight * 1.5);
        assert_eq!(modified_weights.joker_weight, base_weights.joker_weight);
        assert_eq!(
            modified_weights.consumable_weight,
            base_weights.consumable_weight
        );
    }

    #[test]
    fn test_voucher_effect_coupon() {
        let generator = WeightedGenerator::new();
        let base_weights = ItemWeights::default();
        let vouchers = vec![VoucherId::Coupon];

        let modified_weights = generator.apply_voucher_effects(base_weights.clone(), &vouchers);

        // Only joker weight should increase by 30%
        assert_eq!(
            modified_weights.joker_weight,
            base_weights.joker_weight * 1.3
        );
        assert_eq!(modified_weights.pack_weight, base_weights.pack_weight);
        assert_eq!(
            modified_weights.consumable_weight,
            base_weights.consumable_weight
        );
    }

    #[test]
    fn test_multiple_voucher_effects() {
        let generator = WeightedGenerator::new();
        let base_weights = ItemWeights::default();
        let vouchers = vec![VoucherId::Overstock, VoucherId::Coupon];

        let modified_weights = generator.apply_voucher_effects(base_weights.clone(), &vouchers);

        // Should apply both effects: 20% increase from Overstock, then 30% increase from Coupon
        let expected_joker_weight = base_weights.joker_weight * 1.2 * 1.3;
        assert_eq!(modified_weights.joker_weight, expected_joker_weight);
    }

    #[test]
    fn test_weighted_joker_generation() {
        let mut generator = WeightedGenerator::new();

        // Generate multiple jokers to test distribution
        let mut generated_jokers = Vec::new();
        for _ in 0..10 {
            if let Some(joker) = generator.generate_weighted_joker(1) {
                generated_jokers.push(joker);
            }
        }

        // Should generate some jokers
        assert!(!generated_jokers.is_empty());

        // All generated jokers should be valid and from the factory
        let all_jokers = [
            JokerFactory::get_by_rarity(JokerRarity::Common),
            JokerFactory::get_by_rarity(JokerRarity::Uncommon),
            JokerFactory::get_by_rarity(JokerRarity::Rare),
            JokerFactory::get_by_rarity(JokerRarity::Legendary),
        ]
        .concat();

        for joker in generated_jokers {
            assert!(all_jokers.contains(&joker));
        }
    }

    #[test]
    fn test_jokers_by_rarity() {
        let generator = WeightedGenerator::new();

        let common_jokers = generator.get_jokers_by_rarity(JokerRarity::Common);
        let uncommon_jokers = generator.get_jokers_by_rarity(JokerRarity::Uncommon);
        let _rare_jokers = generator.get_jokers_by_rarity(JokerRarity::Rare);
        let _legendary_jokers = generator.get_jokers_by_rarity(JokerRarity::Legendary);

        // Should have jokers for Common and Uncommon (currently implemented)
        assert!(!common_jokers.is_empty());
        assert!(!uncommon_jokers.is_empty());

        // Update test expectations based on current implementation
        // Both rare and legendary jokers have been implemented since this test was written
        // No longer asserting these are empty as jokers now exist for these rarities

        // Common should have more jokers than uncommon
        assert!(common_jokers.len() > uncommon_jokers.len());
    }

    #[test]
    fn test_cache_key_creation() {
        let key1 = CacheKey {
            ante: 1,
            money: 100.0,
            vouchers: vec![VoucherId::Overstock],
        };
        let key2 = CacheKey {
            ante: 1,
            money: 100.0,
            vouchers: vec![VoucherId::Overstock],
        };
        let key3 = CacheKey {
            ante: 2,
            money: 100.0,
            vouchers: vec![VoucherId::Overstock],
        };

        // Same keys should be equal
        assert_eq!(key1, key2);

        // Different keys should not be equal
        assert_ne!(key1, key3);
    }

    #[test]
    fn test_statistical_distribution_common_jokers() {
        let mut generator = WeightedGenerator::new();
        let mut rarity_counts = HashMap::new();
        let sample_size = 1000;

        // Get actual joker lists from factory for mapping
        let common_jokers = JokerFactory::get_by_rarity(JokerRarity::Common);
        let uncommon_jokers = JokerFactory::get_by_rarity(JokerRarity::Uncommon);
        let rare_jokers = JokerFactory::get_by_rarity(JokerRarity::Rare);
        let legendary_jokers = JokerFactory::get_by_rarity(JokerRarity::Legendary);

        // Generate large sample for statistical testing
        for _ in 0..sample_size {
            if let Some(joker) = generator.generate_weighted_joker(1) {
                let rarity = if common_jokers.contains(&joker) {
                    JokerRarity::Common
                } else if uncommon_jokers.contains(&joker) {
                    JokerRarity::Uncommon
                } else if rare_jokers.contains(&joker) {
                    JokerRarity::Rare
                } else if legendary_jokers.contains(&joker) {
                    JokerRarity::Legendary
                } else {
                    JokerRarity::Common // Fallback
                };
                *rarity_counts.entry(rarity).or_insert(0) += 1;
            }
        }

        let total_generated = rarity_counts.values().sum::<usize>() as f64;

        // Check if distribution is approximately correct (within 10% tolerance)
        if let Some(&common_count) = rarity_counts.get(&JokerRarity::Common) {
            let common_percentage = (common_count as f64 / total_generated) * 100.0;
            assert!(common_percentage > 60.0 && common_percentage < 80.0); // 70% ± 10%
        }

        if let Some(&uncommon_count) = rarity_counts.get(&JokerRarity::Uncommon) {
            let uncommon_percentage = (uncommon_count as f64 / total_generated) * 100.0;
            assert!(uncommon_percentage > 15.0 && uncommon_percentage < 35.0); // 25% ± 10%
        }
    }

    #[test]
    fn test_generate_shop_full_functionality() {
        let generator = WeightedGenerator::new();
        let game = Game::new(Config::default());
        let shop = generator.generate_shop(&game);

        // Should have exactly 5 slots
        assert_eq!(shop.slots.len(), 5);

        // All slots should be available
        for slot in &shop.slots {
            assert!(slot.available);
            assert!(slot.cost >= 1); // Minimum cost should be 1
        }

        // Check that shop has appropriate item types
        let item_types: Vec<_> = shop
            .slots
            .iter()
            .map(|slot| match &slot.item {
                ShopItem::Joker(_) => "joker",
                ShopItem::Consumable(_) => "consumable",
                ShopItem::Voucher(_) => "voucher",
                ShopItem::Pack(_) => "pack",
                ShopItem::PlayingCard(_) => "playing_card",
            })
            .collect();

        // Since shop generation is random, we just verify that items were generated
        assert_eq!(item_types.len(), 5, "Shop should have 5 items");

        // Shop should have weights calculated
        assert!(shop.weights.joker_weight > 0.0);
        assert!(shop.weights.consumable_weight > 0.0);
    }

    #[test]
    fn test_generate_pack_standard() {
        let generator = WeightedGenerator::new();
        let game = Game::new(Config::default());
        let pack = generator.generate_pack(PackType::Standard, &game);

        assert_eq!(pack.pack_type, PackType::Standard);
        assert_eq!(pack.cost, 4);
        assert_eq!(pack.contents.len(), 3); // Standard pack should have 3 cards

        // All items should be playing cards
        for item in &pack.contents {
            assert!(matches!(item, ShopItem::PlayingCard(_)));
        }
    }

    #[test]
    fn test_generate_pack_spectral() {
        let generator = WeightedGenerator::new();
        let game = Game::new(Config::default());
        let pack = generator.generate_pack(PackType::Spectral, &game);

        assert_eq!(pack.pack_type, PackType::Spectral);
        assert_eq!(pack.cost, 4);
        assert!(pack.contents.len() >= 2 && pack.contents.len() <= 3); // 2-3 items

        // All items should be spectral consumables
        for item in &pack.contents {
            assert!(matches!(
                item,
                ShopItem::Consumable(crate::shop::ConsumableType::Spectral)
            ));
        }
    }

    #[test]
    fn test_generate_pack_standard_content_count() {
        let generator = WeightedGenerator::new();
        let game = Game::new(Config::default());
        let pack = generator.generate_pack(PackType::Standard, &game);

        assert_eq!(pack.pack_type, PackType::Standard);
        assert_eq!(pack.cost, 4);
        assert_eq!(pack.contents.len(), 3); // Standard pack has 3 items

        // All items should be playing cards
        for item in &pack.contents {
            assert!(matches!(item, ShopItem::PlayingCard(_)));
        }
    }

    #[test]
    fn test_generate_pack_mega_buffoon() {
        let generator = WeightedGenerator::new();
        let game = Game::new(Config::default());
        let pack = generator.generate_pack(PackType::MegaBuffoon, &game);

        assert_eq!(pack.pack_type, PackType::MegaBuffoon);
        assert_eq!(pack.cost, 8);
        assert_eq!(pack.contents.len(), 4); // Mega Buffoon has 4 jokers

        // All items should be jokers
        for item in &pack.contents {
            assert!(matches!(item, ShopItem::Joker(_)));
        }
    }

    #[test]
    fn test_calculate_weights_low_money_adjustment() {
        let generator = WeightedGenerator::new();
        let game = Game::new(Config::default()); // Starts with 0 money
        let weights = generator.calculate_weights(&game);

        // With 0 money (low money), should favor cheaper items
        let default_weights = ItemWeights::default();
        assert_eq!(weights.joker_weight, default_weights.joker_weight); // No change
        assert_eq!(
            weights.consumable_weight,
            default_weights.consumable_weight * 1.2
        ); // 20% increase
        assert_eq!(
            weights.playing_card_weight,
            default_weights.playing_card_weight * 1.4
        ); // 40% increase
    }

    #[test]
    fn test_calculate_weights_high_money_adjustment() {
        let generator = WeightedGenerator::new();
        let mut game = Game::new(Config::default());
        game.money = 100.0; // High money
        let weights = generator.calculate_weights(&game);

        // With high money, should favor expensive items
        let default_weights = ItemWeights::default();
        assert_eq!(weights.voucher_weight, default_weights.voucher_weight * 1.3); // 30% increase
        assert_eq!(weights.pack_weight, default_weights.pack_weight * 1.2); // 20% increase
    }

    #[test]
    fn test_calculate_weights_high_ante_scaling() {
        use crate::ante::Ante;
        let generator = WeightedGenerator::new();
        let mut game = Game::new(Config::default());
        game.ante_current = Ante::Five; // High ante
        game.money = 30.0; // Medium money to avoid money-based adjustments
        let weights = generator.calculate_weights(&game);

        let default_weights = ItemWeights::default();
        let ante_scale = 1.0 + (5.0 * 0.15); // ante=5, scale=1.75

        // Should apply both ante progression effects (use approximate equality for floats)
        assert!(
            (weights.voucher_weight - default_weights.voucher_weight * ante_scale * 0.8).abs()
                < 0.001
        );
        assert!(
            (weights.pack_weight - default_weights.pack_weight * ante_scale * 0.9).abs() < 0.001
        );
        assert!((weights.joker_weight - default_weights.joker_weight * ante_scale).abs() < 0.001);
        assert!(
            (weights.playing_card_weight - default_weights.playing_card_weight * 0.7).abs() < 0.001
        );
    }

    #[test]
    fn test_reroll_shop_basic_functionality() {
        let generator = WeightedGenerator::new();
        let mut game = Game::new(Config::default());
        game.money = 50.0; // Ensure sufficient money for reroll

        let shop = EnhancedShop {
            slots: vec![],
            rerolls_remaining: 5,
            reroll_cost: 5,
            weights: ItemWeights::default(),
        };

        let new_shop = generator.reroll_shop(&shop, &game);

        // Should generate a new shop with 5 slots (standard shop)
        assert_eq!(new_shop.slots.len(), 5);

        // Should have updated reroll cost
        assert_eq!(new_shop.reroll_cost, 10); // 5 + 5 = 10
    }

    // StandardRerollMechanics tests
    #[test]
    fn test_standard_reroll_mechanics_creation() {
        let mechanics = StandardRerollMechanics::new();
        assert!(format!("{mechanics:?}").contains("StandardRerollMechanics"));

        let default_mechanics = StandardRerollMechanics;
        assert!(format!("{default_mechanics:?}").contains("StandardRerollMechanics"));
    }

    #[test]
    fn test_standard_reroll_mechanics_cost_calculation() {
        let mechanics = StandardRerollMechanics::new();
        let game = Game::new(Config::default());

        // Base cost escalation: each reroll increases cost by 5
        assert_eq!(mechanics.calculate_reroll_cost(5, &game), 10);
        assert_eq!(mechanics.calculate_reroll_cost(10, &game), 15);
        assert_eq!(mechanics.calculate_reroll_cost(15, &game), 20);
        assert_eq!(mechanics.calculate_reroll_cost(20, &game), 25);
    }

    #[test]
    fn test_standard_reroll_mechanics_can_reroll_sufficient_money() {
        let mechanics = StandardRerollMechanics::new();
        let mut game = Game::new(Config::default());
        game.money = 50.0;

        let shop = EnhancedShop {
            slots: vec![],
            rerolls_remaining: 5,
            reroll_cost: 10,
            weights: ItemWeights::default(),
        };

        assert!(mechanics.can_reroll(&shop, &game));
    }

    #[test]
    fn test_standard_reroll_mechanics_can_reroll_insufficient_money() {
        let mechanics = StandardRerollMechanics::new();
        let mut game = Game::new(Config::default());
        game.money = 5.0;

        let shop = EnhancedShop {
            slots: vec![],
            rerolls_remaining: 5,
            reroll_cost: 10, // More than available money
            weights: ItemWeights::default(),
        };

        assert!(!mechanics.can_reroll(&shop, &game));
    }

    #[test]
    fn test_standard_reroll_mechanics_can_reroll_no_rerolls_remaining() {
        let mechanics = StandardRerollMechanics::new();
        let mut game = Game::new(Config::default());
        game.money = 50.0;

        let shop = EnhancedShop {
            slots: vec![],
            rerolls_remaining: 0, // No rerolls left
            reroll_cost: 10,
            weights: ItemWeights::default(),
        };

        assert!(!mechanics.can_reroll(&shop, &game));
    }

    #[test]
    fn test_standard_reroll_mechanics_apply_voucher_effects_none() {
        let mechanics = StandardRerollMechanics::new();
        let vouchers: Vec<VoucherId> = vec![];

        // No vouchers should return base cost
        assert_eq!(mechanics.apply_voucher_effects(10, &vouchers), 10);
        assert_eq!(mechanics.apply_voucher_effects(15, &vouchers), 15);
    }

    #[test]
    fn test_standard_reroll_mechanics_apply_voucher_effects_reroll() {
        let mechanics = StandardRerollMechanics::new();
        let vouchers = vec![VoucherId::Reroll];

        // Reroll voucher should make rerolls free
        assert_eq!(mechanics.apply_voucher_effects(10, &vouchers), 0);
        assert_eq!(mechanics.apply_voucher_effects(25, &vouchers), 0);
    }

    #[test]
    fn test_standard_reroll_mechanics_apply_voucher_effects_liquidation() {
        let mechanics = StandardRerollMechanics::new();
        let vouchers = vec![VoucherId::Liquidation];

        // Liquidation gives 20% discount
        assert_eq!(mechanics.apply_voucher_effects(10, &vouchers), 8); // 10 * 0.8 = 8
        assert_eq!(mechanics.apply_voucher_effects(25, &vouchers), 20); // 25 * 0.8 = 20
    }

    #[test]
    fn test_standard_reroll_mechanics_apply_voucher_effects_multiple() {
        let mechanics = StandardRerollMechanics::new();
        let vouchers = vec![VoucherId::Liquidation, VoucherId::Reroll];

        // Reroll should override liquidation, making it free
        assert_eq!(mechanics.apply_voucher_effects(10, &vouchers), 0);
        assert_eq!(mechanics.apply_voucher_effects(25, &vouchers), 0);
    }

    #[test]
    fn test_standard_reroll_mechanics_apply_voucher_effects_other_vouchers() {
        let mechanics = StandardRerollMechanics::new();
        let vouchers = vec![VoucherId::Overstock, VoucherId::Coupon];

        // Other vouchers should not affect reroll cost
        assert_eq!(mechanics.apply_voucher_effects(10, &vouchers), 10);
    }

    #[test]
    fn test_weighted_generator_reroll_shop_with_mechanics() {
        let generator = WeightedGenerator::new();
        let mut game = Game::new(Config::default());
        game.money = 50.0;

        let current_shop = EnhancedShop {
            slots: vec![],
            rerolls_remaining: 5,
            reroll_cost: 10,
            weights: ItemWeights::default(),
        };

        let new_shop = generator.reroll_shop(&current_shop, &game);

        // Should generate new contents
        assert!(!new_shop.slots.is_empty());

        // Should calculate new reroll cost (10 + 5 = 15)
        assert_eq!(new_shop.reroll_cost, 15);

        // Should preserve rerolls remaining
        assert_eq!(new_shop.rerolls_remaining, 5);

        // Should preserve weights
        assert_eq!(
            new_shop.weights.joker_weight,
            current_shop.weights.joker_weight
        );
    }

    #[test]
    fn test_weighted_generator_reroll_shop_insufficient_money() {
        let generator = WeightedGenerator::new();
        let mut game = Game::new(Config::default());
        game.money = 5.0; // Less than reroll cost

        let current_shop = EnhancedShop {
            slots: vec![],
            rerolls_remaining: 5,
            reroll_cost: 10,
            weights: ItemWeights::default(),
        };

        let new_shop = generator.reroll_shop(&current_shop, &game);

        // Should return current shop unchanged when reroll not possible
        assert_eq!(new_shop.reroll_cost, current_shop.reroll_cost);
        assert_eq!(new_shop.rerolls_remaining, current_shop.rerolls_remaining);
    }

    #[test]
    fn test_weighted_generator_reroll_shop_no_rerolls_remaining() {
        let generator = WeightedGenerator::new();
        let mut game = Game::new(Config::default());
        game.money = 50.0;

        let current_shop = EnhancedShop {
            slots: vec![],
            rerolls_remaining: 0, // No rerolls left
            reroll_cost: 10,
            weights: ItemWeights::default(),
        };

        let new_shop = generator.reroll_shop(&current_shop, &game);

        // Should return current shop unchanged when no rerolls remaining
        assert_eq!(new_shop.reroll_cost, current_shop.reroll_cost);
        assert_eq!(new_shop.rerolls_remaining, current_shop.rerolls_remaining);
    }
}
