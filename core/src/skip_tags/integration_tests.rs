//! Integration tests for the skip tags system
//!
//! Tests the full skip tags workflow: skipping blinds, getting tags, and activating them

use super::*;
use crate::action::Action;
use crate::game::Game;
use crate::skip_tags::tag_registry::global_registry;
use crate::stage::{Blind, Stage};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skip_blind_generates_tags() {
        let mut game = Game {
            stage: Stage::PreBlind(),
            ..Default::default()
        };

        // Skip a blind using the public Action API
        let result = game.handle_action(Action::SkipBlind(Blind::Small));

        // Should succeed
        assert!(result.is_ok());

        // Should move to PostBlind stage
        assert!(matches!(game.stage, Stage::PostBlind()));

        // May have generated a tag (50% chance in implementation)
        // Test doesn't verify specific tag generation due to randomness
        // but the system should handle it correctly either way
    }

    #[test]
    fn test_skip_tag_selection_workflow() {
        let mut game = Game::default();

        // Manually set up a tag selection scenario with a shop enhancement tag
        game.available_skip_tags
            .push(SkipTagInstance::new(SkipTagId::Voucher));
        game.pending_tag_selection = true;

        // Select the Voucher tag
        let result = game.handle_action(Action::SelectSkipTag(SkipTagId::Voucher));

        // Should succeed
        assert!(result.is_ok());

        // Should no longer have pending selection
        assert!(!game.pending_tag_selection);

        // Should have the tag in active tags
        assert!(!game.active_skip_tags.is_empty());
    }

    // Utility tags (Double, Boss, Orbital, Juggle) are now enabled

    #[test]
    fn test_double_tag_duplication() {
        let game = Game::default();

        // Set up Double tag context with available tags to duplicate
        let context = SkipTagContext {
            game,
            skipped_blind: Some(Blind::Small),
            available_tags: vec![SkipTagId::Boss, SkipTagId::Orbital],
        };

        let double_tag = utility_tags::DoubleTag;
        let result = double_tag.activate(context);

        assert!(result.success);
        assert_eq!(result.additional_tags.len(), 1);
        assert!(result.additional_tags.contains(&SkipTagId::Boss)); // First non-Double tag
    }

    #[test]
    fn test_boss_tag_activation() {
        let game = Game::default();
        let context = SkipTagContext {
            game,
            skipped_blind: Some(Blind::Boss),
            available_tags: vec![],
        };

        let boss_tag = utility_tags::BossTag;
        let result = boss_tag.activate(context);

        assert!(result.success);
        assert!(result.message.unwrap().contains("Boss Blind"));
    }

    #[test]
    fn test_orbital_tag_activation() {
        let game = Game::default();
        let context = SkipTagContext {
            game,
            skipped_blind: Some(Blind::Small),
            available_tags: vec![],
        };

        let orbital_tag = utility_tags::OrbitalTag;
        let result = orbital_tag.activate(context);

        assert!(result.success);
        let message = result.message.unwrap();
        assert!(message.contains("Upgraded"));
        assert!(message.contains("3 levels"));
    }

    #[test]
    fn test_juggle_tag_stacking() {
        let game = Game::default();
        let context = SkipTagContext {
            game,
            skipped_blind: Some(Blind::Small),
            available_tags: vec![],
        };

        let juggle_tag = utility_tags::JuggleTag;
        let result = juggle_tag.activate(context);

        assert!(result.success);
        assert!(result.message.unwrap().contains("+3 hand size"));

        // Test that juggle tags are stackable
        assert!(juggle_tag.stackable());
    }

    #[test]
    fn test_tag_registry_initialization() {
        let registry = global_registry();

        // Should have all 6 shop enhancement tags registered
        assert!(registry.is_registered(SkipTagId::Voucher));
        assert!(registry.is_registered(SkipTagId::Coupon));
        assert!(registry.is_registered(SkipTagId::D6));
        assert!(registry.is_registered(SkipTagId::Foil));
        assert!(registry.is_registered(SkipTagId::Holographic));
        assert!(registry.is_registered(SkipTagId::Polychrome));

        // Should be able to get implementations
        assert!(registry.get_tag(SkipTagId::Voucher).is_some());
        assert!(registry.get_tag(SkipTagId::Coupon).is_some());
        assert!(registry.get_tag(SkipTagId::D6).is_some());
        assert!(registry.get_tag(SkipTagId::Foil).is_some());
        assert!(registry.get_tag(SkipTagId::Holographic).is_some());
        assert!(registry.get_tag(SkipTagId::Polychrome).is_some());
    }

    #[test]
    fn test_tag_rarity_weights() {
        let registry = global_registry();
        let weighted_tags = registry.get_weighted_tags();

        // Should have all 23 tags with weights (6 shop + 5 economic + 4 utility + 8 reward)
        assert_eq!(weighted_tags.len(), 23);

        // Each tag should have appropriate weight based on rarity
        for (tag_id, weight) in weighted_tags {
            let tag_impl = registry.get_tag(tag_id).unwrap();
            assert_eq!(weight, tag_impl.rarity().weight());
            assert!(weight > 0.0);
        }
    }

    #[test]
    fn test_invalid_tag_selection() {
        let mut game = Game::default();

        // Try to select a tag when no selection is pending
        let result = game.handle_action(Action::SelectSkipTag(SkipTagId::Voucher));

        // Should fail
        assert!(result.is_err());
    }

    #[test]
    fn test_skip_blind_wrong_stage() {
        let mut game = Game {
            stage: Stage::PostBlind(), // Wrong stage for skipping
            ..Default::default()
        };

        let result = game.handle_action(Action::SkipBlind(Blind::Small));

        // Should fail
        assert!(result.is_err());
    }

    #[test]
    fn test_comprehensive_tag_properties() {
        let registry = global_registry();

        // Test Voucher tag properties
        let voucher_tag = registry.get_tag(SkipTagId::Voucher).unwrap();
        assert_eq!(voucher_tag.name(), "Voucher");
        assert_eq!(voucher_tag.effect_type(), TagEffectType::NextShopModifier);
        assert_eq!(voucher_tag.rarity(), TagRarity::Uncommon);
        assert!(voucher_tag.stackable());
        assert!(voucher_tag.selectable());

        // Test Coupon tag properties
        let coupon_tag = registry.get_tag(SkipTagId::Coupon).unwrap();
        assert_eq!(coupon_tag.name(), "Coupon");
        assert_eq!(coupon_tag.effect_type(), TagEffectType::NextShopModifier);
        assert_eq!(coupon_tag.rarity(), TagRarity::Uncommon);
        assert!(!coupon_tag.stackable());

        // Test D6 tag properties
        let d6_tag = registry.get_tag(SkipTagId::D6).unwrap();
        assert_eq!(d6_tag.name(), "D6");
        assert_eq!(d6_tag.effect_type(), TagEffectType::NextShopModifier);
        assert_eq!(d6_tag.rarity(), TagRarity::Common);
        assert!(!d6_tag.stackable());

        // Test Foil tag properties
        let foil_tag = registry.get_tag(SkipTagId::Foil).unwrap();
        assert_eq!(foil_tag.name(), "Foil");
        assert_eq!(foil_tag.effect_type(), TagEffectType::NextShopModifier);
        assert_eq!(foil_tag.rarity(), TagRarity::Rare);
        assert!(!foil_tag.stackable());
    }
}
