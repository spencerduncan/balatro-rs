//! Tests for Planet Frequency Effects with Disabled Planet Cards
//!
//! This test file verifies that PlanetFrequencyMultiplier effects work correctly
//! even when the planet card system is disabled. The effects should be valid
//! and not cause runtime errors, even if they don't have practical impact.

#[cfg(test)]
mod tests {
    use balatro_rs::vouchers::{
        create_voucher, GameState, Voucher, VoucherEffect, VoucherId, PlanetMerchantVoucher, PlanetTycoonVoucher,
    };

    /// Test that PlanetFrequencyMultiplier effects have valid values
    #[test]
    fn test_planet_frequency_multiplier_validation() {
        // Test PlanetMerchant voucher
        let planet_merchant = PlanetMerchantVoucher;
        let merchant_effects = planet_merchant.get_effects();

        assert_eq!(merchant_effects.len(), 1);
        match &merchant_effects[0] {
            VoucherEffect::PlanetFrequencyMultiplier(multiplier) => {
                assert_eq!(*multiplier, 2.0, "PlanetMerchant should have 2x multiplier");
                assert!(multiplier.is_finite(), "Multiplier should be finite");
                assert!(*multiplier > 0.0, "Multiplier should be positive");
                assert!(*multiplier <= 10.0, "Multiplier should be within reasonable bounds");
            },
            _ => panic!("PlanetMerchant should have PlanetFrequencyMultiplier effect"),
        }

        // Test PlanetTycoon voucher
        let planet_tycoon = PlanetTycoonVoucher;
        let tycoon_effects = planet_tycoon.get_effects();

        assert_eq!(tycoon_effects.len(), 1);
        match &tycoon_effects[0] {
            VoucherEffect::PlanetFrequencyMultiplier(multiplier) => {
                assert_eq!(*multiplier, 4.0, "PlanetTycoon should have 4x multiplier");
                assert!(multiplier.is_finite(), "Multiplier should be finite");
                assert!(*multiplier > 0.0, "Multiplier should be positive");
                assert!(*multiplier <= 10.0, "Multiplier should be within reasonable bounds");
            },
            _ => panic!("PlanetTycoon should have PlanetFrequencyMultiplier effect"),
        }

        // Test that all effects pass validation
        for effect in merchant_effects.iter().chain(tycoon_effects.iter()) {
            assert!(effect.validate().is_ok(), "PlanetFrequencyMultiplier effect should be valid");
        }
    }

    /// Test that PlanetFrequencyMultiplier effects can be applied without errors
    #[test]
    fn test_planet_frequency_effect_application() {
        let planet_merchant = PlanetMerchantVoucher;
        let planet_tycoon = PlanetTycoonVoucher;

        let mut game_state = GameState::new();
        let original_state = game_state.clone();

        // Apply PlanetMerchant effects
        planet_merchant.apply_effect(&mut game_state);

        // Verify game state is still valid
        assert!(game_state.validate_state().is_ok(),
               "Game state should remain valid after PlanetMerchant effect");

        // Game state shouldn't change for frequency effects (they affect shop generation)
        assert_eq!(game_state.money(), original_state.money(), "Money shouldn't change");
        assert_eq!(game_state.hand_size(), original_state.hand_size(), "Hand size shouldn't change");
        assert_eq!(game_state.joker_slots(), original_state.joker_slots(), "Joker slots shouldn't change");

        // Apply PlanetTycoon effects (requires prerequisite)
        game_state.add_voucher(VoucherId::PlanetMerchant);
        planet_tycoon.apply_effect(&mut game_state);

        // Verify game state is still valid
        assert!(game_state.validate_state().is_ok(),
               "Game state should remain valid after PlanetTycoon effect");
    }

    /// Test that PlanetFrequencyMultiplier effects are categorized correctly
    #[test]
    fn test_planet_frequency_effect_categorization() {
        let planet_merchant = create_voucher(VoucherId::PlanetMerchant).unwrap();
        let planet_tycoon = create_voucher(VoucherId::PlanetTycoon).unwrap();

        let all_effects = planet_merchant.get_effects()
            .into_iter()
            .chain(planet_tycoon.get_effects())
            .collect::<Vec<_>>();

        for effect in &all_effects {
            // PlanetFrequencyMultiplier should affect shop
            assert!(effect.affects_shop(), "PlanetFrequencyMultiplier should affect shop");

            // Should not affect other systems directly
            assert!(!effect.affects_hand(), "PlanetFrequencyMultiplier should not affect hand");
            assert!(!effect.affects_money(), "PlanetFrequencyMultiplier should not affect money directly");

            // Should be permanent effect
            assert!(effect.is_permanent(), "PlanetFrequencyMultiplier should be permanent");

            // Should have numeric value
            assert!(effect.has_numeric_value(), "PlanetFrequencyMultiplier should have numeric value");
        }
    }

    /// Test upgrade relationship between Planet Merchant vouchers
    #[test]
    fn test_planet_voucher_upgrade_relationship() {
        let base_voucher = create_voucher(VoucherId::PlanetMerchant).unwrap();
        let upgrade_voucher = create_voucher(VoucherId::PlanetTycoon).unwrap();

        // Verify tier relationship
        assert!(base_voucher.tier().is_base(), "PlanetMerchant should be base tier");
        assert!(upgrade_voucher.tier().is_upgraded(), "PlanetTycoon should be upgraded tier");

        // Verify prerequisite relationship
        assert_eq!(upgrade_voucher.prerequisite(), Some(VoucherId::PlanetMerchant),
                  "PlanetTycoon should require PlanetMerchant");
        assert_eq!(base_voucher.prerequisite(), None,
                  "PlanetMerchant should have no prerequisites");

        // Verify upgrade effect is stronger
        let base_effects = base_voucher.get_effects();
        let upgrade_effects = upgrade_voucher.get_effects();

        match (&base_effects[0], &upgrade_effects[0]) {
            (VoucherEffect::PlanetFrequencyMultiplier(base_mult),
             VoucherEffect::PlanetFrequencyMultiplier(upgrade_mult)) => {
                assert!(upgrade_mult > base_mult,
                       "Upgrade multiplier ({upgrade_mult}) should be higher than base ({base_mult})");
                assert_eq!(*upgrade_mult, *base_mult * 2.0,
                          "Upgrade should be exactly 2x base multiplier");
            },
            _ => panic!("Both vouchers should have PlanetFrequencyMultiplier effect"),
        }
    }

    /// Test that Planet vouchers work with factory system
    #[test]
    fn test_planet_vouchers_factory_integration() {
        let planet_vouchers = [VoucherId::PlanetMerchant, VoucherId::PlanetTycoon];

        for voucher_id in planet_vouchers {
            // Test factory can create voucher
            let voucher = create_voucher(voucher_id);
            assert!(voucher.is_some(), "Factory should create {voucher_id:?}");

            let voucher = voucher.unwrap();

            // Verify correct voucher created
            assert_eq!(voucher.id(), voucher_id, "Factory should create correct voucher type");

            // Verify has valid planet frequency effect
            let effects = voucher.get_effects();
            assert_eq!(effects.len(), 1, "Planet voucher should have exactly one effect");

            match &effects[0] {
                VoucherEffect::PlanetFrequencyMultiplier(mult) => {
                    assert!(mult.is_finite() && *mult > 0.0, "Planet frequency multiplier should be valid");
                },
                _ => panic!("{voucher_id:?} should have PlanetFrequencyMultiplier effect"),
            }
        }
    }

    /// Test that Planet frequency effects handle edge cases properly
    #[test]
    fn test_planet_frequency_edge_cases() {
        let effects = [
            VoucherEffect::PlanetFrequencyMultiplier(2.0),
            VoucherEffect::PlanetFrequencyMultiplier(4.0),
        ];

        for effect in &effects {
            // Test serialization/deserialization works
            let serialized = serde_json::to_string(effect)
                .expect("PlanetFrequencyMultiplier should serialize");
            let deserialized: VoucherEffect = serde_json::from_str(&serialized)
                .expect("PlanetFrequencyMultiplier should deserialize");
            assert_eq!(*effect, deserialized, "Serialization should be round-trip safe");

            // Test validation passes
            assert!(effect.validate().is_ok(), "PlanetFrequencyMultiplier should validate");

            // Test that applying to game state doesn't crash
            let mut game_state = GameState::new();
            assert!(game_state.apply_voucher_effect(effect).is_ok(),
                   "Applying PlanetFrequencyMultiplier should not error");
        }
    }

    /// Test boundary conditions for Planet frequency multipliers
    #[test]
    fn test_planet_frequency_boundary_conditions() {
        // Test valid boundary values
        let valid_multipliers = [0.1, 1.0, 2.0, 4.0, 10.0];

        for multiplier in valid_multipliers {
            let effect = VoucherEffect::PlanetFrequencyMultiplier(multiplier);
            assert!(effect.validate().is_ok(),
                   "Multiplier {multiplier} should be valid");
        }

        // Test invalid values
        let invalid_multipliers = [0.0, -1.0, 11.0, f64::INFINITY, f64::NEG_INFINITY, f64::NAN];

        for multiplier in invalid_multipliers {
            let effect = VoucherEffect::PlanetFrequencyMultiplier(multiplier);
            assert!(effect.validate().is_err(),
                   "Multiplier {multiplier} should be invalid");
        }
    }

    /// Test documentation and naming consistency
    #[test]
    fn test_planet_voucher_documentation() {
        let planet_merchant = create_voucher(VoucherId::PlanetMerchant).unwrap();
        let planet_tycoon = create_voucher(VoucherId::PlanetTycoon).unwrap();

        // Test names are consistent
        assert_eq!(planet_merchant.name(), "Planet Merchant");
        assert_eq!(planet_tycoon.name(), "Planet Tycoon");

        // Test descriptions mention planet cards and frequency
        let merchant_desc = planet_merchant.description();
        let tycoon_desc = planet_tycoon.description();

        assert!(merchant_desc.contains("Planet"), "PlanetMerchant description should mention planets");
        assert!(merchant_desc.contains("2X"), "PlanetMerchant description should mention 2X multiplier");

        assert!(tycoon_desc.contains("Planet"), "PlanetTycoon description should mention planets");
        assert!(tycoon_desc.contains("4X"), "PlanetTycoon description should mention 4X multiplier");

        // Test IDs match expectations
        assert_eq!(format!("{}", planet_merchant.id()), "Planet Merchant");
        assert_eq!(format!("{}", planet_tycoon.id()), "Planet Tycoon");
    }
}
