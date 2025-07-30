//! Shop Enhancement Skip Tags
//!
//! Implementation of skip tags that modify the next shop experience:
//! - Voucher: Adds a voucher to the next shop (stackable)
//! - Coupon: Makes initial items free in next shop
//! - D6: Makes first reroll free in next shop
//! - Foil: Makes next base edition joker Foil and free
//! - Holographic: Makes next base edition joker Holographic and free
//! - Polychrome: Makes next base edition joker Polychrome and free

use super::{SkipTag, SkipTagContext, SkipTagId, SkipTagResult, TagEffectType, TagRarity};

/// Voucher Tag: Adds a Voucher to the next Shop (stackable)
#[derive(Debug)]
pub struct VoucherTag;

impl SkipTag for VoucherTag {
    fn id(&self) -> SkipTagId {
        SkipTagId::Voucher
    }

    fn name(&self) -> &'static str {
        "Voucher"
    }

    fn description(&self) -> &'static str {
        "Add a Voucher to the next shop"
    }

    fn effect_type(&self) -> TagEffectType {
        TagEffectType::NextShopModifier
    }

    fn rarity(&self) -> TagRarity {
        TagRarity::Uncommon
    }

    fn stackable(&self) -> bool {
        true
    }

    fn activate(&self, context: SkipTagContext) -> SkipTagResult {
        // TODO: Add voucher to next shop modifiers
        SkipTagResult {
            game: context.game,
            additional_tags: vec![],
            success: true,
            message: Some("Next shop will have an additional voucher".to_string()),
        }
    }
}

/// Coupon Tag: Initial jokers, consumables, and packs are free in next shop
#[derive(Debug)]
pub struct CouponTag;

impl SkipTag for CouponTag {
    fn id(&self) -> SkipTagId {
        SkipTagId::Coupon
    }

    fn name(&self) -> &'static str {
        "Coupon"
    }

    fn description(&self) -> &'static str {
        "Initial items are free in the next shop"
    }

    fn effect_type(&self) -> TagEffectType {
        TagEffectType::NextShopModifier
    }

    fn rarity(&self) -> TagRarity {
        TagRarity::Uncommon
    }

    fn stackable(&self) -> bool {
        false
    }

    fn activate(&self, context: SkipTagContext) -> SkipTagResult {
        // TODO: Set coupon modifier for next shop
        SkipTagResult {
            game: context.game,
            additional_tags: vec![],
            success: true,
            message: Some("Initial items will be free in next shop".to_string()),
        }
    }
}

/// D6 Tag: Rerolls start at $0 in next shop
#[derive(Debug)]
pub struct D6Tag;

impl SkipTag for D6Tag {
    fn id(&self) -> SkipTagId {
        SkipTagId::D6
    }

    fn name(&self) -> &'static str {
        "D6"
    }

    fn description(&self) -> &'static str {
        "Rerolls start at $0 in the next shop"
    }

    fn effect_type(&self) -> TagEffectType {
        TagEffectType::NextShopModifier
    }

    fn rarity(&self) -> TagRarity {
        TagRarity::Common
    }

    fn stackable(&self) -> bool {
        false
    }

    fn activate(&self, context: SkipTagContext) -> SkipTagResult {
        // TODO: Set free reroll modifier for next shop
        SkipTagResult {
            game: context.game,
            additional_tags: vec![],
            success: true,
            message: Some("First reroll will be free in next shop".to_string()),
        }
    }
}

/// Foil Tag: Next base edition joker becomes Foil (+50 Chips) and free
#[derive(Debug)]
pub struct FoilTag;

impl SkipTag for FoilTag {
    fn id(&self) -> SkipTagId {
        SkipTagId::Foil
    }

    fn name(&self) -> &'static str {
        "Foil"
    }

    fn description(&self) -> &'static str {
        "Next base edition joker becomes Foil (+50 Chips) and free"
    }

    fn effect_type(&self) -> TagEffectType {
        TagEffectType::NextShopModifier
    }

    fn rarity(&self) -> TagRarity {
        TagRarity::Rare
    }

    fn stackable(&self) -> bool {
        false
    }

    fn activate(&self, context: SkipTagContext) -> SkipTagResult {
        // TODO: Set foil joker modifier for next shop
        SkipTagResult {
            game: context.game,
            additional_tags: vec![],
            success: true,
            message: Some("Next base edition joker will become Foil and free".to_string()),
        }
    }
}

/// Holographic Tag: Next base edition joker becomes Holographic (+10 Mult) and free
#[derive(Debug)]
pub struct HolographicTag;

impl SkipTag for HolographicTag {
    fn id(&self) -> SkipTagId {
        SkipTagId::Holographic
    }

    fn name(&self) -> &'static str {
        "Holographic"
    }

    fn description(&self) -> &'static str {
        "Next base edition joker becomes Holographic (+10 Mult) and free"
    }

    fn effect_type(&self) -> TagEffectType {
        TagEffectType::NextShopModifier
    }

    fn rarity(&self) -> TagRarity {
        TagRarity::Rare
    }

    fn stackable(&self) -> bool {
        false
    }

    fn activate(&self, context: SkipTagContext) -> SkipTagResult {
        // TODO: Set holographic joker modifier for next shop
        SkipTagResult {
            game: context.game,
            additional_tags: vec![],
            success: true,
            message: Some("Next base edition joker will become Holographic and free".to_string()),
        }
    }
}

/// Polychrome Tag: Next base edition joker becomes Polychrome (X1.5 Mult) and free
#[derive(Debug)]
pub struct PolychromeTag;

impl SkipTag for PolychromeTag {
    fn id(&self) -> SkipTagId {
        SkipTagId::Polychrome
    }

    fn name(&self) -> &'static str {
        "Polychrome"
    }

    fn description(&self) -> &'static str {
        "Next base edition joker becomes Polychrome (X1.5 Mult) and free"
    }

    fn effect_type(&self) -> TagEffectType {
        TagEffectType::NextShopModifier
    }

    fn rarity(&self) -> TagRarity {
        TagRarity::Rare
    }

    fn stackable(&self) -> bool {
        false
    }

    fn activate(&self, context: SkipTagContext) -> SkipTagResult {
        // TODO: Set polychrome joker modifier for next shop
        SkipTagResult {
            game: context.game,
            additional_tags: vec![],
            success: true,
            message: Some("Next base edition joker will become Polychrome and free".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::Game;
    use crate::stage::Blind;

    fn create_test_context() -> SkipTagContext {
        SkipTagContext {
            game: Game::default(),
            skipped_blind: Some(Blind::Small),
            available_tags: vec![],
        }
    }

    #[test]
    fn test_voucher_tag_properties() {
        let tag = VoucherTag;
        assert_eq!(tag.id(), SkipTagId::Voucher);
        assert_eq!(tag.name(), "Voucher");
        assert_eq!(tag.effect_type(), TagEffectType::NextShopModifier);
        assert_eq!(tag.rarity(), TagRarity::Uncommon);
        assert!(tag.stackable());
    }

    #[test]
    fn test_voucher_tag_activation() {
        let tag = VoucherTag;
        let context = create_test_context();

        let result = tag.activate(context);

        assert!(result.success);
        assert!(result.message.unwrap().contains("voucher"));
    }

    #[test]
    fn test_coupon_tag_properties() {
        let tag = CouponTag;
        assert_eq!(tag.id(), SkipTagId::Coupon);
        assert_eq!(tag.name(), "Coupon");
        assert_eq!(tag.effect_type(), TagEffectType::NextShopModifier);
        assert_eq!(tag.rarity(), TagRarity::Uncommon);
        assert!(!tag.stackable());
    }

    #[test]
    fn test_d6_tag_properties() {
        let tag = D6Tag;
        assert_eq!(tag.id(), SkipTagId::D6);
        assert_eq!(tag.name(), "D6");
        assert_eq!(tag.effect_type(), TagEffectType::NextShopModifier);
        assert_eq!(tag.rarity(), TagRarity::Common);
        assert!(!tag.stackable());
    }

    #[test]
    fn test_foil_tag_properties() {
        let tag = FoilTag;
        assert_eq!(tag.id(), SkipTagId::Foil);
        assert_eq!(tag.name(), "Foil");
        assert_eq!(tag.effect_type(), TagEffectType::NextShopModifier);
        assert_eq!(tag.rarity(), TagRarity::Rare);
        assert!(!tag.stackable());
    }

    #[test]
    fn test_holographic_tag_properties() {
        let tag = HolographicTag;
        assert_eq!(tag.id(), SkipTagId::Holographic);
        assert_eq!(tag.name(), "Holographic");
        assert_eq!(tag.effect_type(), TagEffectType::NextShopModifier);
        assert_eq!(tag.rarity(), TagRarity::Rare);
        assert!(!tag.stackable());
    }

    #[test]
    fn test_polychrome_tag_properties() {
        let tag = PolychromeTag;
        assert_eq!(tag.id(), SkipTagId::Polychrome);
        assert_eq!(tag.name(), "Polychrome");
        assert_eq!(tag.effect_type(), TagEffectType::NextShopModifier);
        assert_eq!(tag.rarity(), TagRarity::Rare);
        assert!(!tag.stackable());
    }

    #[test]
    fn test_all_shop_tags_activation() {
        let voucher_result = VoucherTag.activate(create_test_context());
        assert!(voucher_result.success);

        let coupon_result = CouponTag.activate(create_test_context());
        assert!(coupon_result.success);

        let d6_result = D6Tag.activate(create_test_context());
        assert!(d6_result.success);

        let foil_result = FoilTag.activate(create_test_context());
        assert!(foil_result.success);

        let holographic_result = HolographicTag.activate(create_test_context());
        assert!(holographic_result.success);

        let polychrome_result = PolychromeTag.activate(create_test_context());
        assert!(polychrome_result.success);
    }
}
