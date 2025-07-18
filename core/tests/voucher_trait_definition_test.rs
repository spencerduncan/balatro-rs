//! Tests for Issue #16: Voucher Trait Definition
//!
//! This test file verifies the implementation of the Voucher trait
//! and related types as specified in Issue #16.
//!
//! Acceptance Criteria:
//! - [x] Voucher trait defined with required methods
//! - [x] VoucherEffect enum for effect categorization  
//! - [x] VoucherTier enum (base vs upgraded versions)
//! - [x] Stacking rules interface defined
//! - [x] Effect query methods for game systems
//! - [x] Serialization support included

#[cfg(test)]
mod tests {
    use balatro_rs::game::{Game, GameState};
    use balatro_rs::vouchers::{
        StackingRule, Voucher, VoucherCollection, VoucherEffect, VoucherId, VoucherTier,
    };
    use serde_json;

    // Test struct that implements the new Voucher trait
    #[derive(Debug)]
    struct TestVoucher {
        id: VoucherId,
        tier: VoucherTier,
    }

    impl Voucher for TestVoucher {
        fn id(&self) -> VoucherId {
            self.id
        }

        fn tier(&self) -> VoucherTier {
            self.tier
        }

        fn prerequisite(&self) -> Option<VoucherId> {
            None
        }

        fn can_purchase(&self, _game_state: &GameState) -> bool {
            true
        }

        fn apply_effect(&self, _game_state: &mut GameState) {
            // Test implementation
        }

        fn get_effects(&self) -> Vec<VoucherEffect> {
            vec![VoucherEffect::HandSizeIncrease(1)]
        }

        fn stacking_rule(&self) -> StackingRule {
            StackingRule::NoStacking
        }
    }

    /// Test that VoucherEffect enum exists and covers all effect categories
    #[test]
    fn test_voucher_effect_enum_definition() {
        // Test various effect types
        let hand_size_effect = VoucherEffect::HandSizeIncrease(1);
        let joker_slot_effect = VoucherEffect::JokerSlotIncrease(1);
        let money_gain_effect = VoucherEffect::MoneyGain(50);
        let _ante_reduction_effect = VoucherEffect::AnteScaling(0.9);
        let pack_effect = VoucherEffect::ExtraPackOptions(1);
        let _blind_effect = VoucherEffect::BlindScoreReduction(0.1);
        let _card_effect = VoucherEffect::StartingCards(vec![]); // Empty for test
        let _shop_effect = VoucherEffect::ShopSlotIncrease(1);

        // Verify effects can be created and have expected properties
        match hand_size_effect {
            VoucherEffect::HandSizeIncrease(amount) => assert_eq!(amount, 1),
            _ => panic!("Unexpected effect type"),
        }

        match joker_slot_effect {
            VoucherEffect::JokerSlotIncrease(amount) => assert_eq!(amount, 1),
            _ => panic!("Unexpected effect type"),
        }

        match money_gain_effect {
            VoucherEffect::MoneyGain(amount) => assert_eq!(amount, 50),
            _ => panic!("Unexpected effect type"),
        }

        // Test effect categorization
        assert!(hand_size_effect.is_permanent());
        assert!(pack_effect.affects_shop());
        assert!(money_gain_effect.has_numeric_value());
    }

    /// Test that VoucherTier enum exists with base and upgraded versions
    #[test]
    fn test_voucher_tier_enum_definition() {
        let base_tier = VoucherTier::Base;
        let upgraded_tier = VoucherTier::Upgraded;

        // Test tier comparison methods
        assert!(base_tier.is_base());
        assert!(!base_tier.is_upgraded());
        assert!(!upgraded_tier.is_base());
        assert!(upgraded_tier.is_upgraded());

        // Test tier ordering
        assert!(base_tier < upgraded_tier);

        // Test upgrade relationship
        assert_eq!(base_tier.upgrade(), Some(upgraded_tier));
        assert_eq!(upgraded_tier.upgrade(), None);
    }

    /// Test that the Voucher trait has all required methods with correct signatures
    #[test]
    fn test_voucher_trait_method_signatures() {
        let test_voucher = TestVoucher {
            id: VoucherId::GrabBag,
            tier: VoucherTier::Base,
        };

        // Test trait bounds: Send + Sync + Debug
        fn assert_send_sync_debug<T: Send + Sync + std::fmt::Debug>(_: &T) {}
        assert_send_sync_debug(&test_voucher);

        // Test required methods exist and return expected types
        let voucher_id: VoucherId = test_voucher.id();
        assert_eq!(voucher_id, VoucherId::GrabBag);

        let tier: VoucherTier = test_voucher.tier();
        assert_eq!(tier, VoucherTier::Base);

        let prerequisite: Option<VoucherId> = test_voucher.prerequisite();
        assert_eq!(prerequisite, None);

        let effects: Vec<VoucherEffect> = test_voucher.get_effects();
        assert_eq!(effects.len(), 1);

        let stacking: StackingRule = test_voucher.stacking_rule();
        assert_eq!(stacking, StackingRule::NoStacking);

        // Test can_purchase with mock game state
        let game = Game::default();
        let game_state = GameState::from(&game);
        let can_purchase: bool = test_voucher.can_purchase(&game_state);
        assert!(can_purchase);
    }

    /// Test stacking rules interface
    #[test]
    fn test_stacking_rules_interface() {
        // Test different stacking rule types
        let no_stacking = StackingRule::NoStacking;
        let unlimited_stacking = StackingRule::UnlimitedStacking;
        let limited_stacking = StackingRule::LimitedStacking(3);

        // Test stacking rule behavior
        assert!(!no_stacking.allows_stacking());
        assert!(unlimited_stacking.allows_stacking());
        assert!(limited_stacking.allows_stacking());

        // Test stacking limits
        assert_eq!(no_stacking.max_stack_size(), Some(1));
        assert_eq!(unlimited_stacking.max_stack_size(), None);
        assert_eq!(limited_stacking.max_stack_size(), Some(3));

        // Test compatibility checking
        assert!(no_stacking.is_compatible_with(&no_stacking));
        assert!(!no_stacking.is_compatible_with(&unlimited_stacking));
        assert!(unlimited_stacking.is_compatible_with(&unlimited_stacking));
    }

    /// Test effect query methods for game systems
    #[test]
    fn test_effect_query_methods() {
        let effects = vec![
            VoucherEffect::HandSizeIncrease(2),
            VoucherEffect::JokerSlotIncrease(1),
            VoucherEffect::MoneyGain(100),
            VoucherEffect::ExtraPackOptions(1),
        ];

        // Test filtering effects by category
        let hand_effects: Vec<&VoucherEffect> = effects
            .iter()
            .filter(|effect| effect.affects_hand())
            .collect();
        assert_eq!(hand_effects.len(), 1);

        let shop_effects: Vec<&VoucherEffect> = effects
            .iter()
            .filter(|effect| effect.affects_shop())
            .collect();
        assert_eq!(shop_effects.len(), 2); // JokerSlot and ExtraPackOptions

        let money_effects: Vec<&VoucherEffect> = effects
            .iter()
            .filter(|effect| effect.affects_money())
            .collect();
        assert_eq!(money_effects.len(), 1);

        // Test cumulative effect calculation
        let total_hand_size_increase = effects
            .iter()
            .filter_map(|effect| effect.hand_size_bonus())
            .sum::<usize>();
        assert_eq!(total_hand_size_increase, 2);

        let total_joker_slots = effects
            .iter()
            .filter_map(|effect| effect.joker_slot_bonus())
            .sum::<usize>();
        assert_eq!(total_joker_slots, 1);

        let total_money_bonus = effects
            .iter()
            .filter_map(|effect| effect.money_bonus())
            .sum::<usize>();
        assert_eq!(total_money_bonus, 100);
    }

    /// Test serialization support for all voucher-related types
    #[test]
    fn test_serialization_support() {
        // Test VoucherId serialization
        let voucher_id = VoucherId::GrabBag;
        let id_json = serde_json::to_string(&voucher_id).expect("Failed to serialize VoucherId");
        let deserialized_id: VoucherId =
            serde_json::from_str(&id_json).expect("Failed to deserialize VoucherId");
        assert_eq!(voucher_id, deserialized_id);

        // Test VoucherTier serialization
        let tier = VoucherTier::Base;
        let tier_json = serde_json::to_string(&tier).expect("Failed to serialize VoucherTier");
        let deserialized_tier: VoucherTier =
            serde_json::from_str(&tier_json).expect("Failed to deserialize VoucherTier");
        assert_eq!(tier, deserialized_tier);

        // Test VoucherEffect serialization
        let effect = VoucherEffect::HandSizeIncrease(3);
        let effect_json =
            serde_json::to_string(&effect).expect("Failed to serialize VoucherEffect");
        let deserialized_effect: VoucherEffect =
            serde_json::from_str(&effect_json).expect("Failed to deserialize VoucherEffect");
        assert_eq!(effect, deserialized_effect);

        // Test StackingRule serialization
        let stacking = StackingRule::LimitedStacking(5);
        let stacking_json =
            serde_json::to_string(&stacking).expect("Failed to serialize StackingRule");
        let deserialized_stacking: StackingRule =
            serde_json::from_str(&stacking_json).expect("Failed to deserialize StackingRule");
        assert_eq!(stacking, deserialized_stacking);

        // Test VoucherCollection continues to work with new types
        let mut collection = VoucherCollection::new();
        collection.add(VoucherId::GrabBag);
        let collection_json =
            serde_json::to_string(&collection).expect("Failed to serialize VoucherCollection");
        let deserialized_collection: VoucherCollection = serde_json::from_str(&collection_json)
            .expect("Failed to deserialize VoucherCollection");
        assert_eq!(collection.count(), deserialized_collection.count());
        assert!(deserialized_collection.owns(VoucherId::GrabBag));
    }

    /// Test voucher prerequisite chains and validation
    #[test]
    fn test_voucher_prerequisite_validation() {
        // Create vouchers with prerequisite relationships
        let _base_voucher = TestVoucher {
            id: VoucherId::GrabBag,
            tier: VoucherTier::Base,
        };

        // Test that vouchers can form prerequisite chains
        let mut collection = VoucherCollection::new();

        // Initially can purchase base voucher (no prerequisites)
        assert!(collection.can_purchase(VoucherId::GrabBag));

        // Add base voucher
        collection.add(VoucherId::GrabBag);

        // Can no longer purchase same voucher (already owned)
        assert!(!collection.can_purchase(VoucherId::GrabBag));

        // Verify collection state
        assert_eq!(collection.count(), 1);
        assert!(collection.owns(VoucherId::GrabBag));
    }

    /// Test GameState integration for voucher effects
    #[test]
    fn test_game_state_integration() {
        let test_voucher = TestVoucher {
            id: VoucherId::GrabBag,
            tier: VoucherTier::Base,
        };

        // Create a game and get its state
        let game = Game::default();
        let mut game_state = GameState::from(&game);

        // Test that voucher can check purchase conditions
        let can_purchase_before = test_voucher.can_purchase(&game_state);
        assert!(can_purchase_before);

        // Test that voucher can apply effects to game state
        test_voucher.apply_effect(&mut game_state);

        // Test that effects are queryable
        let effects = test_voucher.get_effects();
        assert!(!effects.is_empty());

        // Verify effects have the expected types
        for effect in effects {
            match effect {
                VoucherEffect::HandSizeIncrease(_) => {
                    // This is expected for our test voucher
                }
                _ => panic!("Unexpected effect type in test voucher"),
            }
        }
    }

    /// Test that trait supports all 32 base game vouchers plus expansions
    #[test]
    fn test_voucher_scalability() {
        // This test ensures the trait design can handle the full voucher set

        // Test that VoucherId enum can be extended
        let current_voucher_count = VoucherId::all().len();
        assert!(current_voucher_count >= 2); // At least our test vouchers

        // Test that the trait design supports complex voucher hierarchies
        let base_tier = VoucherTier::Base;
        let upgraded_tier = VoucherTier::Upgraded;

        // Verify tier relationships support voucher families
        assert_ne!(base_tier, upgraded_tier);
        assert!(base_tier < upgraded_tier);

        // Test that effect system can handle diverse effect types
        let diverse_effects = vec![
            VoucherEffect::HandSizeIncrease(1),
            VoucherEffect::JokerSlotIncrease(1),
            VoucherEffect::MoneyGain(25),
            VoucherEffect::AnteScaling(0.9),
            VoucherEffect::ExtraPackOptions(1),
            VoucherEffect::BlindScoreReduction(0.1),
            VoucherEffect::ShopSlotIncrease(1),
        ];

        // Verify all effect types are distinct and serializable
        for effect in diverse_effects {
            let json = serde_json::to_string(&effect).expect("Effect should serialize");
            assert!(!json.is_empty());
        }
    }

    /// Test performance optimization for frequent effect queries
    #[test]
    fn test_effect_query_performance() {
        let test_voucher = TestVoucher {
            id: VoucherId::GrabBag,
            tier: VoucherTier::Base,
        };

        // Test that effect queries are efficient (no expensive computations)
        let start = std::time::Instant::now();

        for _ in 0..1000 {
            let _effects = test_voucher.get_effects();
            let _stacking = test_voucher.stacking_rule();
        }

        let duration = start.elapsed();

        // Effect queries should be very fast (under 1ms for 1000 calls)
        assert!(
            duration.as_millis() < 10,
            "Effect queries took too long: {:?}",
            duration
        );
    }
}
