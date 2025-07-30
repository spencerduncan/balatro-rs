//! Integration tests for Skip Tags system
//!
//! These tests verify that the skip tag system works correctly with the Game struct
//! and that all shop enhancement tags function as expected.

use balatro_rs::game::Game;
use balatro_rs::skip_tags::tag_registry::global_registry;
use balatro_rs::skip_tags::{SkipTagId, TagEffectType};

/// Test fixture for creating a game with specific state
fn create_test_game() -> Game {
    let mut game = Game::default();
    game.start();
    game
}

#[test]
fn test_skip_tag_registry_creation() {
    let registry = global_registry();

    // Test that all shop enhancement tags are registered
    assert!(registry.get_tag(SkipTagId::Voucher).is_some());
    assert!(registry.get_tag(SkipTagId::Coupon).is_some());
    assert!(registry.get_tag(SkipTagId::D6).is_some());
    assert!(registry.get_tag(SkipTagId::Foil).is_some());
    assert!(registry.get_tag(SkipTagId::Holographic).is_some());
    assert!(registry.get_tag(SkipTagId::Polychrome).is_some());

    // Test getting all shop enhancement tags
    let shop_tags = registry.get_all_shop_enhancement_tags();
    assert_eq!(shop_tags.len(), 6);
}

#[test]
fn test_voucher_tag_basic_functionality() {
    let registry = global_registry();
    let voucher_tag = registry.get_tag(SkipTagId::Voucher).unwrap();

    // Test tag metadata
    assert_eq!(voucher_tag.id(), SkipTagId::Voucher);
    assert_eq!(voucher_tag.name(), "Voucher");
    assert_eq!(voucher_tag.effect_type(), TagEffectType::NextShopModifier);
    assert!(voucher_tag.description().contains("Voucher"));
    assert!(voucher_tag.description().contains("next shop"));
}

#[test]
fn test_voucher_tag_effect() {
    let game = create_test_game();
    let registry = global_registry();
    let voucher_tag = registry.get_tag(SkipTagId::Voucher).unwrap();

    let mut game = game;
    let result = voucher_tag.apply_effect(&mut game);

    // Should have no immediate money reward
    assert_eq!(result.money_reward, 0);
    // Should persist for next shop
    assert!(result.persist_tag);
    // Should have appropriate message
    assert!(result.message.as_ref().unwrap().contains("effect applied"));
}

#[test]
fn test_coupon_tag_basic_functionality() {
    let registry = global_registry();
    let coupon_tag = registry.get_tag(SkipTagId::Coupon).unwrap();

    // Test tag metadata
    assert_eq!(coupon_tag.id(), SkipTagId::Coupon);
    assert_eq!(coupon_tag.name(), "Coupon");
    assert_eq!(coupon_tag.effect_type(), TagEffectType::NextShopModifier);
    assert!(coupon_tag.description().contains("Initial items"));
    assert!(coupon_tag.description().contains("free"));
}

#[test]
fn test_coupon_tag_effect() {
    let game = create_test_game();
    let registry = global_registry();
    let coupon_tag = registry.get_tag(SkipTagId::Coupon).unwrap();

    let mut game = game;
    let result = coupon_tag.apply_effect(&mut game);

    // Should have no immediate money reward
    assert_eq!(result.money_reward, 0);
    // Should persist for next shop
    assert!(result.persist_tag);
    // Should have appropriate message
    assert!(result.message.as_ref().unwrap().contains("effect applied"));
}

#[test]
fn test_d6_tag_basic_functionality() {
    let registry = global_registry();
    let d6_tag = registry.get_tag(SkipTagId::D6).unwrap();

    // Test tag metadata
    assert_eq!(d6_tag.id(), SkipTagId::D6);
    assert_eq!(d6_tag.name(), "D6");
    assert_eq!(d6_tag.effect_type(), TagEffectType::NextShopModifier);
    assert!(d6_tag.description().contains("Rerolls"));
    assert!(d6_tag.description().contains("$0"));
}

#[test]
fn test_d6_tag_effect() {
    let game = create_test_game();
    let registry = global_registry();
    let d6_tag = registry.get_tag(SkipTagId::D6).unwrap();

    let mut game = game;
    let result = d6_tag.apply_effect(&mut game);

    // Should have no immediate money reward
    assert_eq!(result.money_reward, 0);
    // Should persist for next shop
    assert!(result.persist_tag);
    // Should have appropriate message
    assert!(result.message.as_ref().unwrap().contains("effect applied"));
}

#[test]
fn test_foil_tag_basic_functionality() {
    let registry = global_registry();
    let foil_tag = registry.get_tag(SkipTagId::Foil).unwrap();

    // Test tag metadata
    assert_eq!(foil_tag.id(), SkipTagId::Foil);
    assert_eq!(foil_tag.name(), "Foil");
    assert_eq!(foil_tag.effect_type(), TagEffectType::NextShopModifier);
    assert!(foil_tag.description().contains("Foil"));
    assert!(foil_tag.description().contains("+50 Chips"));
}

#[test]
fn test_foil_tag_effect() {
    let game = create_test_game();
    let registry = global_registry();
    let foil_tag = registry.get_tag(SkipTagId::Foil).unwrap();

    let mut game = game;
    let result = foil_tag.apply_effect(&mut game);

    // Should have no immediate money reward
    assert_eq!(result.money_reward, 0);
    // Should persist for next shop
    assert!(result.persist_tag);
    // Should have appropriate message
    assert!(result.message.as_ref().unwrap().contains("effect applied"));
}

#[test]
fn test_holographic_tag_basic_functionality() {
    let registry = global_registry();
    let holographic_tag = registry.get_tag(SkipTagId::Holographic).unwrap();

    // Test tag metadata
    assert_eq!(holographic_tag.id(), SkipTagId::Holographic);
    assert_eq!(holographic_tag.name(), "Holographic");
    assert_eq!(
        holographic_tag.effect_type(),
        TagEffectType::NextShopModifier
    );
    assert!(holographic_tag.description().contains("Holographic"));
    assert!(holographic_tag.description().contains("+10 Mult"));
}

#[test]
fn test_holographic_tag_effect() {
    let game = create_test_game();
    let registry = global_registry();
    let holographic_tag = registry.get_tag(SkipTagId::Holographic).unwrap();

    let mut game = game;
    let result = holographic_tag.apply_effect(&mut game);

    // Should have no immediate money reward
    assert_eq!(result.money_reward, 0);
    // Should persist for next shop
    assert!(result.persist_tag);
    // Should have appropriate message
    assert!(result.message.as_ref().unwrap().contains("effect applied"));
}

#[test]
fn test_polychrome_tag_basic_functionality() {
    let registry = global_registry();
    let polychrome_tag = registry.get_tag(SkipTagId::Polychrome).unwrap();

    // Test tag metadata
    assert_eq!(polychrome_tag.id(), SkipTagId::Polychrome);
    assert_eq!(polychrome_tag.name(), "Polychrome");
    assert_eq!(
        polychrome_tag.effect_type(),
        TagEffectType::NextShopModifier
    );
    assert!(polychrome_tag.description().contains("Polychrome"));
    assert!(polychrome_tag.description().contains("X1.5 Mult"));
}

#[test]
fn test_polychrome_tag_effect() {
    let game = create_test_game();
    let registry = global_registry();
    let polychrome_tag = registry.get_tag(SkipTagId::Polychrome).unwrap();

    let mut game = game;
    let result = polychrome_tag.apply_effect(&mut game);

    // Should have no immediate money reward
    assert_eq!(result.money_reward, 0);
    // Should persist for next shop
    assert!(result.persist_tag);
    // Should have appropriate message
    assert!(result.message.as_ref().unwrap().contains("effect applied"));
}

#[test]
fn test_shop_enhancement_game_integration() {
    let mut game = create_test_game();

    // Apply all shop enhancement tags
    game.apply_skip_tag_effect(SkipTagId::Voucher).unwrap();
    game.apply_skip_tag_effect(SkipTagId::Coupon).unwrap();
    game.apply_skip_tag_effect(SkipTagId::D6).unwrap();
    game.apply_skip_tag_effect(SkipTagId::Foil).unwrap();
    game.apply_skip_tag_effect(SkipTagId::Holographic).unwrap();
    game.apply_skip_tag_effect(SkipTagId::Polychrome).unwrap();

    // Verify all next shop modifiers are set
    let modifiers = &game.active_skip_tags.next_shop_modifiers;
    assert_eq!(modifiers.additional_vouchers, 1);
    assert!(modifiers.coupon_active);
    assert!(modifiers.free_rerolls);
    assert!(modifiers.foil_tag_active);
    assert!(modifiers.holographic_tag_active);
    assert!(modifiers.polychrome_tag_active);
}

#[test]
fn test_voucher_tag_stacking() {
    let mut game = create_test_game();

    // Apply voucher tag multiple times
    game.apply_skip_tag_effect(SkipTagId::Voucher).unwrap();
    game.apply_skip_tag_effect(SkipTagId::Voucher).unwrap();
    game.apply_skip_tag_effect(SkipTagId::Voucher).unwrap();

    // Should stack to 3 vouchers
    assert_eq!(
        game.active_skip_tags
            .next_shop_modifiers
            .additional_vouchers,
        3
    );
}

#[test]
fn test_edition_tags_can_coexist() {
    let mut game = create_test_game();

    // Apply multiple edition tags - should all be set
    game.apply_skip_tag_effect(SkipTagId::Foil).unwrap();
    game.apply_skip_tag_effect(SkipTagId::Holographic).unwrap();
    game.apply_skip_tag_effect(SkipTagId::Polychrome).unwrap();

    // All edition modifiers should be active
    let modifiers = &game.active_skip_tags.next_shop_modifiers;
    assert!(modifiers.foil_tag_active);
    assert!(modifiers.holographic_tag_active);
    assert!(modifiers.polychrome_tag_active);
}

#[test]
fn test_next_shop_modifiers_consumption() {
    let mut game = create_test_game();

    // Set up shop modifiers
    game.apply_skip_tag_effect(SkipTagId::Voucher).unwrap();
    game.apply_skip_tag_effect(SkipTagId::Coupon).unwrap();
    game.apply_skip_tag_effect(SkipTagId::D6).unwrap();

    // Verify modifiers are set
    let modifiers = &game.active_skip_tags.next_shop_modifiers;
    assert_eq!(modifiers.additional_vouchers, 1);
    assert!(modifiers.coupon_active);
    assert!(modifiers.free_rerolls);

    // Consume modifiers
    let modifiers = game.consume_next_shop_modifiers();

    // Verify modifiers are returned correctly
    assert_eq!(modifiers.additional_vouchers, 1);
    assert!(modifiers.coupon_active);
    assert!(modifiers.free_rerolls);

    // Verify modifiers are reset after consumption
    let reset_modifiers = &game.active_skip_tags.next_shop_modifiers;
    assert_eq!(reset_modifiers.additional_vouchers, 0);
    assert!(!reset_modifiers.coupon_active);
    assert!(!reset_modifiers.free_rerolls);
}

#[test]
fn test_blinds_skipped_tracking() {
    let mut game = create_test_game();

    // Initially no blinds skipped
    assert_eq!(game.get_blinds_skipped_count(), 0);

    // Increment blinds skipped
    game.increment_blinds_skipped();
    game.increment_blinds_skipped();

    assert_eq!(game.get_blinds_skipped_count(), 2);
}

#[test]
fn test_boss_blind_defeat_no_investment() {
    let mut game = create_test_game();

    // Test boss blind defeat with no investment tags
    let reward = game.handle_boss_blind_defeat();
    assert_eq!(reward, 0);
}

#[test]
fn test_all_shop_enhancement_tags_are_next_shop_modifiers() {
    let registry = global_registry();
    let shop_tags = registry.get_all_shop_enhancement_tags();

    for tag_id in shop_tags {
        let tag = registry.get_tag(tag_id).unwrap();
        assert_eq!(tag.effect_type(), TagEffectType::NextShopModifier);
        assert!(tag.can_apply(&create_test_game()));
    }
}

#[test]
fn test_all_shop_enhancement_tags_persist() {
    let registry = global_registry();
    let shop_tags = registry.get_all_shop_enhancement_tags();

    for tag_id in shop_tags {
        let tag = registry.get_tag(tag_id).unwrap();
        let mut test_game = create_test_game();
        let result = tag.apply_effect(&mut test_game);
        assert!(result.persist_tag, "Tag {} should persist", tag.name());
        assert_eq!(
            result.money_reward,
            0,
            "Tag {} should give no immediate money",
            tag.name()
        );
        assert!(
            result.message.is_some(),
            "Tag {} should have a message",
            tag.name()
        );
    }
}

#[test]
fn test_invalid_tag_id() {
    let mut game = create_test_game();

    // Try to apply a tag that's not implemented (Economy tag)
    let result = game.apply_skip_tag_effect(SkipTagId::Economy);
    assert!(result.is_err());

    if let Err(e) = result {
        assert!(matches!(
            e,
            balatro_rs::skip_tags::TagError::InvalidTagId(_)
        ));
    }
}

#[test]
fn test_tag_id_names() {
    assert_eq!(SkipTagId::Voucher.name(), "Voucher");
    assert_eq!(SkipTagId::Coupon.name(), "Coupon");
    assert_eq!(SkipTagId::D6.name(), "D6");
    assert_eq!(SkipTagId::Foil.name(), "Foil");
    assert_eq!(SkipTagId::Holographic.name(), "Holographic");
    assert_eq!(SkipTagId::Polychrome.name(), "Polychrome");
}

#[test]
fn test_active_skip_tags_default_state() {
    let game = create_test_game();

    // All shop modifiers should start inactive
    let modifiers = &game.active_skip_tags.next_shop_modifiers;
    assert_eq!(modifiers.additional_vouchers, 0);
    assert!(!modifiers.coupon_active);
    assert!(!modifiers.free_rerolls);
    assert!(!modifiers.foil_tag_active);
    assert!(!modifiers.holographic_tag_active);
    assert!(!modifiers.polychrome_tag_active);

    // Economic tag state should start at zero
    assert_eq!(game.active_skip_tags.investment_count, 0);
    assert_eq!(game.active_skip_tags.blinds_skipped, 0);
}

#[test]
fn test_shop_enhancement_tags_comprehensive() {
    let mut game = create_test_game();
    let initial_money = game.money;

    // Test each shop enhancement tag individually
    let shop_tag_ids = [
        SkipTagId::Voucher,
        SkipTagId::Coupon,
        SkipTagId::D6,
        SkipTagId::Foil,
        SkipTagId::Holographic,
        SkipTagId::Polychrome,
    ];

    for tag_id in shop_tag_ids {
        let result = game.apply_skip_tag_effect(tag_id).unwrap();

        // All shop enhancement tags should:
        // 1. Give no immediate money
        assert_eq!(
            result.money_reward, 0,
            "Tag {tag_id:?} should give no immediate money"
        );

        // 2. Persist for next shop
        assert!(result.persist_tag, "Tag {tag_id:?} should persist");

        // 3. Have a descriptive message
        assert!(
            result.message.is_some(),
            "Tag {tag_id:?} should have a message"
        );
        assert!(
            result.message.as_ref().unwrap().contains("effect applied"),
            "Tag {tag_id:?} message should contain 'effect applied'"
        );

        // 4. Not change money immediately
        assert_eq!(
            game.money, initial_money,
            "Tag {tag_id:?} should not change money immediately"
        );
    }

    // Verify all modifiers are now set
    let modifiers = &game.active_skip_tags.next_shop_modifiers;
    assert_eq!(modifiers.additional_vouchers, 1);
    assert!(modifiers.coupon_active);
    assert!(modifiers.free_rerolls);
    assert!(modifiers.foil_tag_active);
    assert!(modifiers.holographic_tag_active);
    assert!(modifiers.polychrome_tag_active);
}
