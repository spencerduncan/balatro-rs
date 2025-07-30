//! Tests for Issue #727: Voucher Upgrade Implementations
//!
//! This test file verifies the implementation of all 9 upgrade vouchers
//! and their integration with the factory system.
//!
//! New Upgrade Vouchers:
//! - GlowUp (upgrade of Hone)
//! - Liquidation (upgrade of Clearance Sale)
//! - Recyclomancy (upgrade of Wasteful)
//! - PlanetMerchant (base voucher)
//! - PlanetTycoon (upgrade of Planet Merchant)
//! - DirectorsCut (base voucher)
//! - Retcon (upgrade of Director's Cut)
//! - Palette (upgrade of Paint Brush)

#[cfg(test)]
mod tests {
    use balatro_rs::vouchers::{
        create_voucher, GameState, Voucher, VoucherEffect, VoucherId, VoucherTier,
        GlowUpVoucher, LiquidationVoucher, RecyclomancyVoucher, PlanetMerchantVoucher,
        PlanetTycoonVoucher, DirectorsCutVoucher, RetconVoucher, PaletteVoucher,
    };

    /// Test GlowUp voucher implementation (upgrade of Hone)
    #[test]
    fn test_glow_up_voucher() {
        let voucher = GlowUpVoucher;

        // Basic properties
        assert_eq!(voucher.id(), VoucherId::GlowUp);
        assert_eq!(voucher.tier(), VoucherTier::Upgraded);
        assert_eq!(voucher.prerequisite(), Some(VoucherId::Hone));
        assert_eq!(voucher.name(), "Glow Up");
        assert_eq!(voucher.description(), "Foil, Holographic, and Polychrome cards appear 4X more often");

        // Effects
        let effects = voucher.get_effects();
        assert_eq!(effects.len(), 1);
        match &effects[0] {
            VoucherEffect::PolychromeFrequencyMultiplier(multiplier) => {
                assert_eq!(*multiplier, 4.0);
            },
            _ => panic!("Expected PolychromeFrequencyMultiplier effect"),
        }

        // Purchase requirements
        let mut game_state = GameState::new();
        assert!(!voucher.can_purchase(&game_state)); // No prerequisite

        game_state.add_voucher(VoucherId::Hone);
        assert!(voucher.can_purchase(&game_state)); // Has prerequisite

        game_state.add_voucher(VoucherId::GlowUp);
        assert!(!voucher.can_purchase(&game_state)); // Already owned

        // Effect validation
        assert!(effects[0].validate().is_ok());
    }

    /// Test Liquidation voucher implementation (upgrade of Clearance Sale)
    #[test]
    fn test_liquidation_voucher() {
        let voucher = LiquidationVoucher;

        // Basic properties
        assert_eq!(voucher.id(), VoucherId::Liquidation);
        assert_eq!(voucher.tier(), VoucherTier::Upgraded);
        assert_eq!(voucher.prerequisite(), Some(VoucherId::ClearanceSale));
        assert_eq!(voucher.name(), "Liquidation");
        assert_eq!(voucher.description(), "All cards and packs in shop are 50% off");

        // Effects
        let effects = voucher.get_effects();
        assert_eq!(effects.len(), 1);
        match &effects[0] {
            VoucherEffect::ShopDiscountMultiplier(multiplier) => {
                assert_eq!(*multiplier, 0.5);
            },
            _ => panic!("Expected ShopDiscountMultiplier effect"),
        }

        // Shop effect categorization
        assert!(effects[0].affects_shop());
        assert!(!effects[0].affects_money());
        assert!(!effects[0].affects_hand());

        // Purchase requirements
        let mut game_state = GameState::new();
        assert!(!voucher.can_purchase(&game_state)); // No prerequisite

        game_state.add_voucher(VoucherId::ClearanceSale);
        assert!(voucher.can_purchase(&game_state)); // Has prerequisite

        // Effect validation
        assert!(effects[0].validate().is_ok());
    }

    /// Test Recyclomancy voucher implementation (upgrade of Wasteful)
    #[test]
    fn test_recyclomancy_voucher() {
        let voucher = RecyclomancyVoucher;

        // Basic properties
        assert_eq!(voucher.id(), VoucherId::Recyclomancy);
        assert_eq!(voucher.tier(), VoucherTier::Upgraded);
        assert_eq!(voucher.prerequisite(), Some(VoucherId::Wasteful));
        assert_eq!(voucher.name(), "Recyclomancy");
        assert_eq!(voucher.description(), "Permanently gain +1 discard each round");

        // Effects
        let effects = voucher.get_effects();
        assert_eq!(effects.len(), 1);
        match &effects[0] {
            VoucherEffect::DiscardIncrease(amount) => {
                assert_eq!(*amount, 1);
            },
            _ => panic!("Expected DiscardIncrease effect"),
        }

        // Hand effect categorization
        assert!(effects[0].affects_hand());
        assert!(!effects[0].affects_shop());
        assert!(!effects[0].affects_money());

        // Purchase requirements
        let mut game_state = GameState::new();
        assert!(!voucher.can_purchase(&game_state)); // No prerequisite

        game_state.add_voucher(VoucherId::Wasteful);
        assert!(voucher.can_purchase(&game_state)); // Has prerequisite

        // Effect validation
        assert!(effects[0].validate().is_ok());
    }

    /// Test PlanetMerchant voucher implementation (base voucher)
    #[test]
    fn test_planet_merchant_voucher() {
        let voucher = PlanetMerchantVoucher;

        // Basic properties
        assert_eq!(voucher.id(), VoucherId::PlanetMerchant);
        assert_eq!(voucher.tier(), VoucherTier::Base);
        assert_eq!(voucher.prerequisite(), None);
        assert_eq!(voucher.name(), "Planet Merchant");
        assert_eq!(voucher.description(), "Planet cards appear 2X more frequently in shop");

        // Effects
        let effects = voucher.get_effects();
        assert_eq!(effects.len(), 1);
        match &effects[0] {
            VoucherEffect::PlanetFrequencyMultiplier(multiplier) => {
                assert_eq!(*multiplier, 2.0);
            },
            _ => panic!("Expected PlanetFrequencyMultiplier effect"),
        }

        // Shop effect categorization
        assert!(effects[0].affects_shop());
        assert!(effects[0].is_permanent());

        // Purchase requirements (no prerequisites for base voucher)
        let game_state = GameState::new();
        assert!(voucher.can_purchase(&game_state));

        // Effect validation
        assert!(effects[0].validate().is_ok());
    }

    /// Test PlanetTycoon voucher implementation (upgrade of Planet Merchant)
    #[test]
    fn test_planet_tycoon_voucher() {
        let voucher = PlanetTycoonVoucher;

        // Basic properties
        assert_eq!(voucher.id(), VoucherId::PlanetTycoon);
        assert_eq!(voucher.tier(), VoucherTier::Upgraded);
        assert_eq!(voucher.prerequisite(), Some(VoucherId::PlanetMerchant));
        assert_eq!(voucher.name(), "Planet Tycoon");
        assert_eq!(voucher.description(), "Planet cards appear 4X more frequently in shop");

        // Effects
        let effects = voucher.get_effects();
        assert_eq!(effects.len(), 1);
        match &effects[0] {
            VoucherEffect::PlanetFrequencyMultiplier(multiplier) => {
                assert_eq!(*multiplier, 4.0);
            },
            _ => panic!("Expected PlanetFrequencyMultiplier effect"),
        }

        // Upgrade relationship verification
        let base_voucher = PlanetMerchantVoucher;
        let base_effects = base_voucher.get_effects();

        // Both have same effect type but upgraded has higher multiplier
        match (&base_effects[0], &effects[0]) {
            (VoucherEffect::PlanetFrequencyMultiplier(base_mult),
             VoucherEffect::PlanetFrequencyMultiplier(upgrade_mult)) => {
                assert!(*upgrade_mult > *base_mult, "Upgrade should have higher multiplier");
                assert_eq!(*upgrade_mult, *base_mult * 2.0, "Upgrade should be 2x base");
            },
            _ => panic!("Both vouchers should have PlanetFrequencyMultiplier effect"),
        }

        // Purchase requirements
        let mut game_state = GameState::new();
        assert!(!voucher.can_purchase(&game_state)); // No prerequisite

        game_state.add_voucher(VoucherId::PlanetMerchant);
        assert!(voucher.can_purchase(&game_state)); // Has prerequisite

        // Effect validation
        assert!(effects[0].validate().is_ok());
    }

    /// Test DirectorsCut voucher implementation (base voucher)
    #[test]
    fn test_directors_cut_voucher() {
        let voucher = DirectorsCutVoucher;

        // Basic properties
        assert_eq!(voucher.id(), VoucherId::DirectorsCut);
        assert_eq!(voucher.tier(), VoucherTier::Base);
        assert_eq!(voucher.prerequisite(), None);
        assert_eq!(voucher.name(), "Director's Cut");
        assert_eq!(voucher.description(), "Reroll Boss Blind 1 time per Ante, $10 per roll");

        // Effects
        let effects = voucher.get_effects();
        assert_eq!(effects.len(), 1);
        match &effects[0] {
            VoucherEffect::BossBlindRerollEnabled { unlimited, cost_per_roll } => {
                assert!(!*unlimited);
                assert_eq!(*cost_per_roll, 10);
            },
            _ => panic!("Expected BossBlindRerollEnabled effect"),
        }

        // Shop effect categorization
        assert!(effects[0].affects_shop());
        assert!(effects[0].is_permanent());

        // Purchase requirements (no prerequisites for base voucher)
        let game_state = GameState::new();
        assert!(voucher.can_purchase(&game_state));

        // Effect validation
        assert!(effects[0].validate().is_ok());
    }

    /// Test Retcon voucher implementation (upgrade of Director's Cut)
    #[test]
    fn test_retcon_voucher() {
        let voucher = RetconVoucher;

        // Basic properties
        assert_eq!(voucher.id(), VoucherId::Retcon);
        assert_eq!(voucher.tier(), VoucherTier::Upgraded);
        assert_eq!(voucher.prerequisite(), Some(VoucherId::DirectorsCut));
        assert_eq!(voucher.name(), "Retcon");
        assert_eq!(voucher.description(), "Reroll Boss Blinds unlimited times, $10 per roll");

        // Effects
        let effects = voucher.get_effects();
        assert_eq!(effects.len(), 1);
        match &effects[0] {
            VoucherEffect::BossBlindRerollEnabled { unlimited, cost_per_roll } => {
                assert!(*unlimited);
                assert_eq!(*cost_per_roll, 10);
            },
            _ => panic!("Expected BossBlindRerollEnabled effect"),
        }

        // Upgrade relationship verification
        let base_voucher = DirectorsCutVoucher;
        let base_effects = base_voucher.get_effects();

        // Both have same effect type but upgraded has unlimited rerolls
        match (&base_effects[0], &effects[0]) {
            (VoucherEffect::BossBlindRerollEnabled { unlimited: base_unlimited, cost_per_roll: base_cost },
             VoucherEffect::BossBlindRerollEnabled { unlimited: upgrade_unlimited, cost_per_roll: upgrade_cost }) => {
                assert!(!*base_unlimited, "Base should have limited rerolls");
                assert!(*upgrade_unlimited, "Upgrade should have unlimited rerolls");
                assert_eq!(*base_cost, *upgrade_cost, "Cost should be same");
            },
            _ => panic!("Both vouchers should have BossBlindRerollEnabled effect"),
        }

        // Purchase requirements
        let mut game_state = GameState::new();
        assert!(!voucher.can_purchase(&game_state)); // No prerequisite

        game_state.add_voucher(VoucherId::DirectorsCut);
        assert!(voucher.can_purchase(&game_state)); // Has prerequisite

        // Effect validation
        assert!(effects[0].validate().is_ok());
    }

    /// Test Palette voucher implementation (upgrade of Paint Brush)
    #[test]
    fn test_palette_voucher() {
        let voucher = PaletteVoucher;

        // Basic properties
        assert_eq!(voucher.id(), VoucherId::Palette);
        assert_eq!(voucher.tier(), VoucherTier::Upgraded);
        assert_eq!(voucher.prerequisite(), Some(VoucherId::PaintBrush));
        assert_eq!(voucher.name(), "Palette");
        assert_eq!(voucher.description(), "+1 hand size");

        // Effects
        let effects = voucher.get_effects();
        assert_eq!(effects.len(), 1);
        match &effects[0] {
            VoucherEffect::HandSizeIncrease(amount) => {
                assert_eq!(*amount, 1);
            },
            _ => panic!("Expected HandSizeIncrease effect"),
        }

        // Hand effect categorization
        assert!(effects[0].affects_hand());
        assert!(!effects[0].affects_shop());
        assert!(!effects[0].affects_money());

        // Purchase requirements
        let mut game_state = GameState::new();
        assert!(!voucher.can_purchase(&game_state)); // No prerequisite

        game_state.add_voucher(VoucherId::PaintBrush);
        assert!(voucher.can_purchase(&game_state)); // Has prerequisite

        // Effect validation
        assert!(effects[0].validate().is_ok());

        // Effect application
        let mut test_state = GameState::new();
        let original_hand_size = test_state.hand_size();
        voucher.apply_effect(&mut test_state);
        assert_eq!(test_state.hand_size(), original_hand_size + 1);
    }

    /// Test voucher factory integration for all upgrade vouchers
    #[test]
    fn test_voucher_factory_integration() {
        let upgrade_vouchers = [
            VoucherId::GlowUp,
            VoucherId::Liquidation,
            VoucherId::Recyclomancy,
            VoucherId::PlanetMerchant,
            VoucherId::PlanetTycoon,
            VoucherId::DirectorsCut,
            VoucherId::Retcon,
            VoucherId::Palette,
        ];

        for voucher_id in upgrade_vouchers {
            // Test that factory can create each voucher
            let voucher_instance = create_voucher(voucher_id);
            assert!(voucher_instance.is_some(), "Factory should create voucher for {voucher_id:?}");

            let voucher = voucher_instance.unwrap();

            // Verify factory creates correct voucher type
            assert_eq!(voucher.id(), voucher_id, "Factory created wrong voucher type");

            // Verify voucher has valid effects
            let effects = voucher.get_effects();
            assert!(!effects.is_empty(), "Voucher should have at least one effect");

            // Verify all effects are valid
            for effect in effects {
                assert!(effect.validate().is_ok(), "Effect should be valid for {voucher_id:?}");
            }

            // Verify voucher has name and description
            assert!(!voucher.name().is_empty(), "Voucher should have name");
            assert!(!voucher.description().is_empty(), "Voucher should have description");
        }
    }

    /// Test prerequisite validation for all upgrade vouchers
    #[test]
    fn test_prerequisite_validation() {
        let prerequisite_chains = [
            (VoucherId::GlowUp, VoucherId::Hone),
            (VoucherId::Liquidation, VoucherId::ClearanceSale),
            (VoucherId::Recyclomancy, VoucherId::Wasteful),
            (VoucherId::PlanetTycoon, VoucherId::PlanetMerchant),
            (VoucherId::Retcon, VoucherId::DirectorsCut),
            (VoucherId::Palette, VoucherId::PaintBrush),
        ];

        for (upgrade_id, base_id) in prerequisite_chains {
            let upgrade_voucher = create_voucher(upgrade_id).unwrap();

            // Verify prerequisite relationship
            assert_eq!(upgrade_voucher.prerequisite(), Some(base_id),
                      "Upgrade voucher {upgrade_id:?} should require {base_id:?}");

            // Test purchase validation
            let mut game_state = GameState::new();

            // Cannot purchase without prerequisite
            assert!(!upgrade_voucher.can_purchase(&game_state),
                   "Should not be able to purchase {upgrade_id:?} without {base_id:?}");

            // Can purchase with prerequisite
            game_state.add_voucher(base_id);
            assert!(upgrade_voucher.can_purchase(&game_state),
                   "Should be able to purchase {upgrade_id:?} with {base_id:?}");

            // Cannot purchase if already owned
            game_state.add_voucher(upgrade_id);
            assert!(!upgrade_voucher.can_purchase(&game_state),
                   "Should not be able to purchase {upgrade_id:?} if already owned");
        }

        // Test base vouchers have no prerequisites
        let base_vouchers = [VoucherId::PlanetMerchant, VoucherId::DirectorsCut];
        for base_id in base_vouchers {
            let base_voucher = create_voucher(base_id).unwrap();
            assert_eq!(base_voucher.prerequisite(), None,
                      "Base voucher {base_id:?} should have no prerequisites");

            let game_state = GameState::new();
            assert!(base_voucher.can_purchase(&game_state),
                   "Should be able to purchase base voucher {base_id:?}");
        }
    }

    /// Test effect application for all upgrade vouchers
    #[test]
    fn test_effect_application() {
        let upgrade_vouchers = [
            VoucherId::GlowUp,
            VoucherId::Liquidation,
            VoucherId::Recyclomancy,
            VoucherId::PlanetMerchant,
            VoucherId::PlanetTycoon,
            VoucherId::DirectorsCut,
            VoucherId::Retcon,
            VoucherId::Palette,
        ];

        for voucher_id in upgrade_vouchers {
            let voucher = create_voucher(voucher_id).unwrap();
            let mut game_state = GameState::new();
            let original_state = game_state.clone();

            // Apply effects
            voucher.apply_effect(&mut game_state);

            // Verify state is still valid after applying effects
            assert!(game_state.validate_state().is_ok(),
                   "Game state should be valid after applying {voucher_id:?} effects");

            // For hand size effects, verify they actually change the state
            if let Some(hand_increase) = voucher.get_effects().iter()
                .find_map(|e| e.hand_size_bonus()) {
                assert_eq!(game_state.hand_size(), original_state.hand_size() + hand_increase,
                          "Hand size should increase by {hand_increase} for {voucher_id:?}");
            }
        }
    }

    /// Test tier classification for all upgrade vouchers
    #[test]
    fn test_tier_classification() {
        let base_vouchers = [VoucherId::PlanetMerchant, VoucherId::DirectorsCut];
        let upgraded_vouchers = [
            VoucherId::GlowUp,
            VoucherId::Liquidation,
            VoucherId::Recyclomancy,
            VoucherId::PlanetTycoon,
            VoucherId::Retcon,
            VoucherId::Palette,
        ];

        // Test base vouchers
        for voucher_id in base_vouchers {
            let voucher = create_voucher(voucher_id).unwrap();
            assert_eq!(voucher.tier(), VoucherTier::Base,
                      "{voucher_id:?} should be base tier");
            assert!(voucher.tier().is_base());
            assert!(!voucher.tier().is_upgraded());
        }

        // Test upgraded vouchers
        for voucher_id in upgraded_vouchers {
            let voucher = create_voucher(voucher_id).unwrap();
            assert_eq!(voucher.tier(), VoucherTier::Upgraded,
                      "{voucher_id:?} should be upgraded tier");
            assert!(!voucher.tier().is_base());
            assert!(voucher.tier().is_upgraded());
        }
    }
}
