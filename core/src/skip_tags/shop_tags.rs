//! Shop Enhancement Tags Implementation
//!
//! These tags modify the next shop experience, providing various benefits like
//! free rerolls, vouchers, and edition upgrades to shop items.

use super::{SkipTag, SkipTagId, TagEffectResult, TagEffectType};
use crate::game::Game;

/// Voucher Tag - Adds a free voucher to the next shop
#[derive(Debug, Clone)]
pub struct VoucherTag;

impl SkipTag for VoucherTag {
    fn tag_id(&self) -> SkipTagId {
        SkipTagId::Voucher
    }

    fn name(&self) -> &'static str {
        "Voucher"
    }

    fn effect_type(&self) -> TagEffectType {
        TagEffectType::NextShopModifier
    }

    fn description(&self) -> &'static str {
        "Adds a free Voucher to next shop"
    }

    fn apply_effect(&self, _game_state: &Game) -> TagEffectResult {
        TagEffectResult::with_persistence(
            0,
            "Voucher Tag: Next shop will have a free voucher".to_string(),
        )
    }
}

/// Coupon Tag - Next shop has +1 free reroll
#[derive(Debug, Clone)]
pub struct CouponTag;

impl SkipTag for CouponTag {
    fn tag_id(&self) -> SkipTagId {
        SkipTagId::Coupon
    }

    fn name(&self) -> &'static str {
        "Coupon"
    }

    fn effect_type(&self) -> TagEffectType {
        TagEffectType::NextShopModifier
    }

    fn description(&self) -> &'static str {
        "Initial items in next shop are free"
    }

    fn apply_effect(&self, _game_state: &Game) -> TagEffectResult {
        TagEffectResult::with_persistence(0, "Coupon Tag: Next shop items will be free".to_string())
    }
}

/// D6 Tag - All rerolls in next shop cost $0
#[derive(Debug, Clone)]
pub struct D6Tag;

impl SkipTag for D6Tag {
    fn tag_id(&self) -> SkipTagId {
        SkipTagId::D6
    }

    fn name(&self) -> &'static str {
        "D6"
    }

    fn effect_type(&self) -> TagEffectType {
        TagEffectType::NextShopModifier
    }

    fn description(&self) -> &'static str {
        "Rerolls in next shop cost $0"
    }

    fn apply_effect(&self, _game_state: &Game) -> TagEffectResult {
        TagEffectResult::with_persistence(0, "D6 Tag: Next shop rerolls will be free".to_string())
    }
}

/// Foil Tag - Adds Foil edition to a random joker in next shop
#[derive(Debug, Clone)]
pub struct FoilTag;

impl SkipTag for FoilTag {
    fn tag_id(&self) -> SkipTagId {
        SkipTagId::Foil
    }

    fn name(&self) -> &'static str {
        "Foil"
    }

    fn effect_type(&self) -> TagEffectType {
        TagEffectType::NextShopModifier
    }

    fn description(&self) -> &'static str {
        "Next shop has a Foil joker (+50 Chips when scored)"
    }

    fn apply_effect(&self, _game_state: &Game) -> TagEffectResult {
        TagEffectResult::with_persistence(
            0,
            "Foil Tag: Next shop will have a Foil joker".to_string(),
        )
    }
}

/// Holographic Tag - Adds Holographic edition to a random joker in next shop
#[derive(Debug, Clone)]
pub struct HolographicTag;

impl SkipTag for HolographicTag {
    fn tag_id(&self) -> SkipTagId {
        SkipTagId::Holographic
    }

    fn name(&self) -> &'static str {
        "Holographic"
    }

    fn effect_type(&self) -> TagEffectType {
        TagEffectType::NextShopModifier
    }

    fn description(&self) -> &'static str {
        "Next shop has a Holographic joker (+10 Mult when scored)"
    }

    fn apply_effect(&self, _game_state: &Game) -> TagEffectResult {
        TagEffectResult::with_persistence(
            0,
            "Holographic Tag: Next shop will have a Holographic joker".to_string(),
        )
    }
}

/// Polychrome Tag - Adds Polychrome edition to a random joker in next shop
#[derive(Debug, Clone)]
pub struct PolychromeTag;

impl SkipTag for PolychromeTag {
    fn tag_id(&self) -> SkipTagId {
        SkipTagId::Polychrome
    }

    fn name(&self) -> &'static str {
        "Polychrome"
    }

    fn effect_type(&self) -> TagEffectType {
        TagEffectType::NextShopModifier
    }

    fn description(&self) -> &'static str {
        "Next shop has a Polychrome joker (X1.5 Mult when scored)"
    }

    fn apply_effect(&self, _game_state: &Game) -> TagEffectResult {
        TagEffectResult::with_persistence(
            0,
            "Polychrome Tag: Next shop will have a Polychrome joker".to_string(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::Game;

    /// Helper function to create a test game
    fn create_test_game() -> Game {
        let mut game = Game::default();
        game.start();
        game
    }

    #[test]
    fn test_voucher_tag_basic_functionality() {
        let tag = VoucherTag;
        let game = create_test_game();

        // Test tag metadata
        assert_eq!(tag.tag_id(), SkipTagId::Voucher);
        assert_eq!(tag.name(), "Voucher");
        assert_eq!(tag.effect_type(), TagEffectType::NextShopModifier);
        assert!(tag.description().contains("Voucher"));
        assert!(tag.description().contains("next shop"));

        // Test effect application
        let result = tag.apply_effect(&game);
        assert_eq!(result.money_reward, 0);
        assert!(result.persist_tag);
        assert!(!result.messages.is_empty());
        assert!(result.messages[0].contains("Voucher Tag"));
    }

    #[test]
    fn test_coupon_tag_basic_functionality() {
        let tag = CouponTag;
        let game = create_test_game();

        // Test tag metadata
        assert_eq!(tag.tag_id(), SkipTagId::Coupon);
        assert_eq!(tag.name(), "Coupon");
        assert_eq!(tag.effect_type(), TagEffectType::NextShopModifier);
        assert!(tag.description().contains("Initial items"));
        assert!(tag.description().contains("free"));

        // Test effect application
        let result = tag.apply_effect(&game);
        assert_eq!(result.money_reward, 0);
        assert!(result.persist_tag);
        assert!(!result.messages.is_empty());
        assert!(result.messages[0].contains("Coupon Tag"));
    }

    #[test]
    fn test_d6_tag_basic_functionality() {
        let tag = D6Tag;
        let game = create_test_game();

        // Test tag metadata
        assert_eq!(tag.tag_id(), SkipTagId::D6);
        assert_eq!(tag.name(), "D6");
        assert_eq!(tag.effect_type(), TagEffectType::NextShopModifier);
        assert!(tag.description().contains("Rerolls"));
        assert!(tag.description().contains("$0"));

        // Test effect application
        let result = tag.apply_effect(&game);
        assert_eq!(result.money_reward, 0);
        assert!(result.persist_tag);
        assert!(!result.messages.is_empty());
        assert!(result.messages[0].contains("D6 Tag"));
    }

    #[test]
    fn test_foil_tag_basic_functionality() {
        let tag = FoilTag;
        let game = create_test_game();

        // Test tag metadata
        assert_eq!(tag.tag_id(), SkipTagId::Foil);
        assert_eq!(tag.name(), "Foil");
        assert_eq!(tag.effect_type(), TagEffectType::NextShopModifier);
        assert!(tag.description().contains("Foil"));
        assert!(tag.description().contains("+50 Chips"));

        // Test effect application
        let result = tag.apply_effect(&game);
        assert_eq!(result.money_reward, 0);
        assert!(result.persist_tag);
        assert!(!result.messages.is_empty());
        assert!(result.messages[0].contains("Foil Tag"));
    }

    #[test]
    fn test_holographic_tag_basic_functionality() {
        let tag = HolographicTag;
        let game = create_test_game();

        // Test tag metadata
        assert_eq!(tag.tag_id(), SkipTagId::Holographic);
        assert_eq!(tag.name(), "Holographic");
        assert_eq!(tag.effect_type(), TagEffectType::NextShopModifier);
        assert!(tag.description().contains("Holographic"));
        assert!(tag.description().contains("+10 Mult"));

        // Test effect application
        let result = tag.apply_effect(&game);
        assert_eq!(result.money_reward, 0);
        assert!(result.persist_tag);
        assert!(!result.messages.is_empty());
        assert!(result.messages[0].contains("Holographic Tag"));
    }

    #[test]
    fn test_polychrome_tag_basic_functionality() {
        let tag = PolychromeTag;
        let game = create_test_game();

        // Test tag metadata
        assert_eq!(tag.tag_id(), SkipTagId::Polychrome);
        assert_eq!(tag.name(), "Polychrome");
        assert_eq!(tag.effect_type(), TagEffectType::NextShopModifier);
        assert!(tag.description().contains("Polychrome"));
        assert!(tag.description().contains("X1.5 Mult"));

        // Test effect application
        let result = tag.apply_effect(&game);
        assert_eq!(result.money_reward, 0);
        assert!(result.persist_tag);
        assert!(!result.messages.is_empty());
        assert!(result.messages[0].contains("Polychrome Tag"));
    }

    #[test]
    fn test_all_shop_enhancement_tags_are_next_shop_modifiers() {
        let tags: Vec<Box<dyn SkipTag>> = vec![
            Box::new(VoucherTag),
            Box::new(CouponTag),
            Box::new(D6Tag),
            Box::new(FoilTag),
            Box::new(HolographicTag),
            Box::new(PolychromeTag),
        ];

        for tag in tags {
            assert_eq!(tag.effect_type(), TagEffectType::NextShopModifier);
            assert!(tag.can_apply(&create_test_game()));
        }
    }

    #[test]
    fn test_all_shop_enhancement_tags_persist() {
        let game = create_test_game();
        let tags: Vec<Box<dyn SkipTag>> = vec![
            Box::new(VoucherTag),
            Box::new(CouponTag),
            Box::new(D6Tag),
            Box::new(FoilTag),
            Box::new(HolographicTag),
            Box::new(PolychromeTag),
        ];

        for tag in tags {
            let result = tag.apply_effect(&game);
            assert!(result.persist_tag, "Tag {} should persist", tag.name());
            assert_eq!(
                result.money_reward,
                0,
                "Tag {} should give no immediate money",
                tag.name()
            );
            assert!(
                !result.messages.is_empty(),
                "Tag {} should have a message",
                tag.name()
            );
        }
    }
}
