//! Test suite for shop voucher implementations from Issue #730
//!
//! Tests all 9 missing voucher implementations with comprehensive coverage:
//! - Overstock and Overstock Plus (shop slots)
//! - Clearance Sale (shop discount)
//! - Hone (polychrome frequency)
//! - Reroll Surplus and Reroll Glut (reroll costs)
//! - Crystal Ball and Omen Globe (consumable/spectral)
//! - Telescope (celestial packs)

#[cfg(test)]
mod tests {
    use super::super::{
        implementations::*, GameState, Voucher, VoucherEffect, VoucherId, VoucherTier,
    };

    /// Test helper to create basic game state for voucher testing
    fn create_test_game_state() -> GameState {
        GameState::new()
    }

    /// Test helper to create game state with existing vouchers
    fn create_game_state_with_vouchers(vouchers: Vec<VoucherId>) -> GameState {
        let mut state = GameState::new();
        for voucher in vouchers {
            state.add_voucher(voucher);
        }
        state
    }

    #[test]
    fn test_overstock_voucher_basic_properties() {
        let voucher = OverstockVoucher;

        assert_eq!(voucher.id(), VoucherId::Overstock);
        assert_eq!(voucher.tier(), VoucherTier::Base);
        assert_eq!(voucher.prerequisite(), None);
        assert_eq!(voucher.name(), "Overstock");
        assert_eq!(voucher.description(), "+1 card slot in shop");
        assert_eq!(voucher.cost(), 10);
    }

    #[test]
    fn test_overstock_voucher_effects() {
        let voucher = OverstockVoucher;
        let effects = voucher.get_effects();

        assert_eq!(effects.len(), 1);
        assert_eq!(effects[0], VoucherEffect::ShopSlotIncrease(1));

        // Test effect validation
        effects[0].validate().expect("Effect should be valid");
    }

    #[test]
    fn test_overstock_voucher_purchase_conditions() {
        let voucher = OverstockVoucher;
        let mut state = create_test_game_state();

        // Can purchase with sufficient funds
        assert!(voucher.can_purchase(&state));

        // Cannot purchase if already owned
        state.add_voucher(VoucherId::Overstock);
        assert!(!voucher.can_purchase(&state));

        // Cannot purchase with insufficient funds
        let mut poor_state = GameState::new();
        poor_state.spend_money(95).unwrap(); // Leave only $5
        assert!(!voucher.can_purchase(&poor_state));
    }

    #[test]
    fn test_overstock_plus_voucher_basic_properties() {
        let voucher = OverstockPlusVoucher;

        assert_eq!(voucher.id(), VoucherId::OverstockPlus);
        assert_eq!(voucher.tier(), VoucherTier::Upgraded);
        assert_eq!(voucher.prerequisite(), Some(VoucherId::Overstock));
        assert_eq!(voucher.name(), "Overstock Plus");
        assert_eq!(voucher.description(), "+2 card slots in shop");
        assert_eq!(voucher.cost(), 10);
    }

    #[test]
    fn test_overstock_plus_voucher_effects() {
        let voucher = OverstockPlusVoucher;
        let effects = voucher.get_effects();

        assert_eq!(effects.len(), 1);
        assert_eq!(effects[0], VoucherEffect::ShopSlotIncrease(2));

        // Test effect validation
        effects[0].validate().expect("Effect should be valid");
    }

    #[test]
    fn test_overstock_plus_voucher_prerequisites() {
        let voucher = OverstockPlusVoucher;

        // Cannot purchase without prerequisite
        let state = create_test_game_state();
        assert!(!voucher.can_purchase(&state));

        // Can purchase with prerequisite
        let state_with_prereq = create_game_state_with_vouchers(vec![VoucherId::Overstock]);
        assert!(voucher.can_purchase(&state_with_prereq));

        // Cannot purchase if already owned
        let state_owned =
            create_game_state_with_vouchers(vec![VoucherId::Overstock, VoucherId::OverstockPlus]);
        assert!(!voucher.can_purchase(&state_owned));
    }

    #[test]
    fn test_clearance_sale_voucher_basic_properties() {
        let voucher = ClearanceSaleVoucher;

        assert_eq!(voucher.id(), VoucherId::ClearanceSale);
        assert_eq!(voucher.tier(), VoucherTier::Base);
        assert_eq!(voucher.prerequisite(), None);
        assert_eq!(voucher.name(), "Clearance Sale");
        assert_eq!(voucher.description(), "All items in shop 50% off");
        assert_eq!(voucher.cost(), 10);
    }

    #[test]
    fn test_clearance_sale_voucher_effects() {
        let voucher = ClearanceSaleVoucher;
        let effects = voucher.get_effects();

        assert_eq!(effects.len(), 1);
        assert_eq!(effects[0], VoucherEffect::ShopDiscountPercent(50.0));

        // Test effect validation
        effects[0].validate().expect("Effect should be valid");

        // Test affect shop check
        assert!(effects[0].affects_shop());
    }

    #[test]
    fn test_hone_voucher_basic_properties() {
        let voucher = HoneVoucher;

        assert_eq!(voucher.id(), VoucherId::Hone);
        assert_eq!(voucher.tier(), VoucherTier::Base);
        assert_eq!(voucher.prerequisite(), None);
        assert_eq!(voucher.name(), "Hone");
        assert_eq!(
            voucher.description(),
            "Foil/Holo/Polychrome cards appear 2X more"
        );
        assert_eq!(voucher.cost(), 10);
    }

    #[test]
    fn test_hone_voucher_effects() {
        let voucher = HoneVoucher;
        let effects = voucher.get_effects();

        assert_eq!(effects.len(), 1);
        assert_eq!(
            effects[0],
            VoucherEffect::PolychromeFrequencyMultiplier(2.0)
        );

        // Test effect validation
        effects[0].validate().expect("Effect should be valid");
    }

    #[test]
    fn test_reroll_surplus_voucher_basic_properties() {
        let voucher = RerollSurplusVoucher;

        assert_eq!(voucher.id(), VoucherId::RerollSurplus);
        assert_eq!(voucher.tier(), VoucherTier::Base);
        assert_eq!(voucher.prerequisite(), None);
        assert_eq!(voucher.name(), "Reroll Surplus");
        assert_eq!(voucher.description(), "Rerolls cost $1 less");
        assert_eq!(voucher.cost(), 10);
    }

    #[test]
    fn test_reroll_surplus_voucher_effects() {
        let voucher = RerollSurplusVoucher;
        let effects = voucher.get_effects();

        assert_eq!(effects.len(), 1);
        assert_eq!(effects[0], VoucherEffect::RerollCostReduction(1));

        // Test effect validation
        effects[0].validate().expect("Effect should be valid");

        // Test affect shop check
        assert!(effects[0].affects_shop());
    }

    #[test]
    fn test_crystal_ball_voucher_basic_properties() {
        let voucher = CrystalBallVoucher;

        assert_eq!(voucher.id(), VoucherId::CrystalBall);
        assert_eq!(voucher.tier(), VoucherTier::Base);
        assert_eq!(voucher.prerequisite(), None);
        assert_eq!(voucher.name(), "Crystal Ball");
        assert_eq!(voucher.description(), "+1 consumable slot");
        assert_eq!(voucher.cost(), 10);
    }

    #[test]
    fn test_crystal_ball_voucher_effects() {
        let voucher = CrystalBallVoucher;
        let effects = voucher.get_effects();

        assert_eq!(effects.len(), 1);
        assert_eq!(effects[0], VoucherEffect::ConsumableSlotIncrease(1));

        // Test effect validation
        effects[0].validate().expect("Effect should be valid");

        // Test affect shop check
        assert!(effects[0].affects_shop());
    }

    #[test]
    fn test_telescope_voucher_basic_properties() {
        let voucher = TelescopeVoucher;

        assert_eq!(voucher.id(), VoucherId::Telescope);
        assert_eq!(voucher.tier(), VoucherTier::Base);
        assert_eq!(voucher.prerequisite(), None);
        assert_eq!(voucher.name(), "Telescope");
        assert_eq!(
            voucher.description(),
            "Celestial packs have 1 more planet card"
        );
        assert_eq!(voucher.cost(), 10);
    }

    #[test]
    fn test_telescope_voucher_effects() {
        let voucher = TelescopeVoucher;
        let effects = voucher.get_effects();

        assert_eq!(effects.len(), 1);
        assert_eq!(effects[0], VoucherEffect::CelestialPackBonus(1));

        // Test effect validation
        effects[0].validate().expect("Effect should be valid");
    }

    #[test]
    fn test_reroll_glut_voucher_basic_properties() {
        let voucher = RerollGlutVoucher;

        assert_eq!(voucher.id(), VoucherId::RerollGlut);
        assert_eq!(voucher.tier(), VoucherTier::Upgraded);
        assert_eq!(voucher.prerequisite(), Some(VoucherId::RerollSurplus));
        assert_eq!(voucher.name(), "Reroll Glut");
        assert_eq!(voucher.description(), "Rerolls cost $2 less");
        assert_eq!(voucher.cost(), 10);
    }

    #[test]
    fn test_reroll_glut_voucher_effects() {
        let voucher = RerollGlutVoucher;
        let effects = voucher.get_effects();

        assert_eq!(effects.len(), 1);
        assert_eq!(effects[0], VoucherEffect::RerollCostReduction(2));

        // Test effect validation
        effects[0].validate().expect("Effect should be valid");

        // Test affect shop check
        assert!(effects[0].affects_shop());
    }

    #[test]
    fn test_reroll_glut_voucher_prerequisites() {
        let voucher = RerollGlutVoucher;

        // Cannot purchase without prerequisite
        let state = create_test_game_state();
        assert!(!voucher.can_purchase(&state));

        // Can purchase with prerequisite
        let state_with_prereq = create_game_state_with_vouchers(vec![VoucherId::RerollSurplus]);
        assert!(voucher.can_purchase(&state_with_prereq));

        // Cannot purchase if already owned
        let state_owned =
            create_game_state_with_vouchers(vec![VoucherId::RerollSurplus, VoucherId::RerollGlut]);
        assert!(!voucher.can_purchase(&state_owned));
    }

    #[test]
    fn test_omen_globe_voucher_basic_properties() {
        let voucher = OmenGlobeVoucher;

        assert_eq!(voucher.id(), VoucherId::OmenGlobe);
        assert_eq!(voucher.tier(), VoucherTier::Upgraded);
        assert_eq!(voucher.prerequisite(), Some(VoucherId::CrystalBall));
        assert_eq!(voucher.name(), "Omen Globe");
        assert_eq!(
            voucher.description(),
            "Spectral packs may contain Planet cards"
        );
        assert_eq!(voucher.cost(), 10);
    }

    #[test]
    fn test_omen_globe_voucher_effects() {
        let voucher = OmenGlobeVoucher;
        let effects = voucher.get_effects();

        assert_eq!(effects.len(), 1);
        assert_eq!(effects[0], VoucherEffect::SpectralPackPlanetChance(0.25));

        // Test effect validation
        effects[0].validate().expect("Effect should be valid");
    }

    #[test]
    fn test_omen_globe_voucher_prerequisites() {
        let voucher = OmenGlobeVoucher;

        // Cannot purchase without prerequisite
        let state = create_test_game_state();
        assert!(!voucher.can_purchase(&state));

        // Can purchase with prerequisite
        let state_with_prereq = create_game_state_with_vouchers(vec![VoucherId::CrystalBall]);
        assert!(voucher.can_purchase(&state_with_prereq));

        // Cannot purchase if already owned
        let state_owned =
            create_game_state_with_vouchers(vec![VoucherId::CrystalBall, VoucherId::OmenGlobe]);
        assert!(!voucher.can_purchase(&state_owned));
    }

    #[test]
    fn test_voucher_effect_application() {
        let mut state = create_test_game_state();

        // Test applying effects doesn't crash
        let overstock = OverstockVoucher;
        overstock.apply_effect(&mut state);

        let clearance = ClearanceSaleVoucher;
        clearance.apply_effect(&mut state);

        let hone = HoneVoucher;
        hone.apply_effect(&mut state);

        // State should remain valid after all applications
        state.validate_state().expect("State should remain valid");
    }

    #[test]
    fn test_all_voucher_effects_validation() {
        // Test that all new effects validate properly
        let effects = vec![
            VoucherEffect::ShopSlotIncrease(1),
            VoucherEffect::ShopSlotIncrease(2),
            VoucherEffect::ShopDiscountPercent(50.0),
            VoucherEffect::PolychromeFrequencyMultiplier(2.0),
            VoucherEffect::RerollCostReduction(1),
            VoucherEffect::RerollCostReduction(2),
            VoucherEffect::ConsumableSlotIncrease(1),
            VoucherEffect::CelestialPackBonus(1),
            VoucherEffect::SpectralPackPlanetChance(0.25),
        ];

        for effect in effects {
            effect
                .validate()
                .unwrap_or_else(|_| panic!("Effect {effect:?} should validate"));
        }
    }

    #[test]
    fn test_invalid_effect_validation() {
        // Test effects that should fail validation
        let invalid_effects = vec![
            VoucherEffect::ShopDiscountPercent(101.0), // Over 100%
            VoucherEffect::ShopDiscountPercent(-10.0), // Negative
            VoucherEffect::PolychromeFrequencyMultiplier(11.0), // Over limit
            VoucherEffect::RerollCostReduction(15),    // Over limit
            VoucherEffect::ConsumableSlotIncrease(15), // Over limit
            VoucherEffect::CelestialPackBonus(10),     // Over limit
            VoucherEffect::SpectralPackPlanetChance(1.5), // Over 1.0
            VoucherEffect::SpectralPackPlanetChance(-0.1), // Negative
        ];

        for effect in invalid_effects {
            assert!(
                effect.validate().is_err(),
                "Effect {effect:?} should fail validation"
            );
        }
    }

    #[test]
    fn test_upgrade_relationships() {
        // Test that upgraded vouchers have correct base relationships
        let upgrade_pairs = vec![
            (VoucherId::Overstock, VoucherId::OverstockPlus),
            (VoucherId::RerollSurplus, VoucherId::RerollGlut),
            (VoucherId::CrystalBall, VoucherId::OmenGlobe),
        ];

        for (base, upgrade) in upgrade_pairs {
            let base_prereqs = base.prerequisites();
            let upgrade_prereqs = upgrade.prerequisites();

            // Base vouchers should have no prerequisites
            assert!(
                base_prereqs.is_empty(),
                "Base voucher {base:?} should have no prerequisites"
            );

            // Upgraded vouchers should require their base
            assert_eq!(
                upgrade_prereqs.len(),
                1,
                "Upgrade voucher {upgrade:?} should have exactly one prerequisite"
            );
            assert_eq!(
                upgrade_prereqs[0], base,
                "Upgrade voucher {upgrade:?} should require base {base:?}"
            );
        }
    }

    #[test]
    fn test_consistent_pricing() {
        let vouchers: Vec<Box<dyn Voucher>> = vec![
            Box::new(OverstockVoucher),
            Box::new(OverstockPlusVoucher),
            Box::new(ClearanceSaleVoucher),
            Box::new(HoneVoucher),
            Box::new(RerollSurplusVoucher),
            Box::new(CrystalBallVoucher),
            Box::new(TelescopeVoucher),
            Box::new(RerollGlutVoucher),
            Box::new(OmenGlobeVoucher),
        ];

        // All vouchers should cost $10 as per issue requirements
        for voucher in vouchers {
            assert_eq!(
                voucher.cost(),
                10,
                "Voucher {} should cost $10",
                voucher.name()
            );
        }
    }

    /// Integration test to ensure all vouchers work together
    #[test]
    fn test_voucher_integration() {
        let mut state = create_test_game_state();

        // Purchase base vouchers first
        let base_vouchers: Vec<Box<dyn Voucher>> = vec![
            Box::new(OverstockVoucher),
            Box::new(ClearanceSaleVoucher),
            Box::new(HoneVoucher),
            Box::new(RerollSurplusVoucher),
            Box::new(CrystalBallVoucher),
            Box::new(TelescopeVoucher),
        ];

        for voucher in base_vouchers {
            assert!(
                voucher.can_purchase(&state),
                "Should be able to purchase {}",
                voucher.name()
            );
            state.spend_money(voucher.cost()).unwrap();
            state.add_voucher(voucher.id());
            voucher.apply_effect(&mut state);
        }

        // Now purchase upgrade vouchers
        let upgrade_vouchers: Vec<Box<dyn Voucher>> = vec![
            Box::new(OverstockPlusVoucher),
            Box::new(RerollGlutVoucher),
            Box::new(OmenGlobeVoucher),
        ];

        // Create a new state with prerequisites for upgrade vouchers
        let mut rich_state = GameState::new();
        rich_state.add_voucher(VoucherId::Overstock);
        rich_state.add_voucher(VoucherId::RerollSurplus);
        rich_state.add_voucher(VoucherId::CrystalBall);

        for voucher in upgrade_vouchers {
            assert!(
                voucher.can_purchase(&rich_state),
                "Should be able to purchase upgrade {}",
                voucher.name()
            );
            rich_state.spend_money(voucher.cost()).unwrap();
            rich_state.add_voucher(voucher.id());
            voucher.apply_effect(&mut rich_state);
        }

        // Final state should be valid
        rich_state
            .validate_state()
            .expect("Final state should be valid");
    }
}
