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

// ============= ECONOMIC TAG TESTS =============

#[test]
fn test_economy_tag_basic_functionality() {
    let registry = global_registry();
    let economy_tag = registry.get_tag(SkipTagId::Economy).unwrap();

    // Test tag metadata
    assert_eq!(economy_tag.id(), SkipTagId::Economy);
    assert_eq!(economy_tag.name(), "Economy");
    assert_eq!(economy_tag.effect_type(), TagEffectType::ImmediateReward);
    assert!(economy_tag.description().contains("Doubles your money"));
    assert!(economy_tag.description().contains("max +$40"));
    assert!(!economy_tag.stackable());
}

#[test]
fn test_economy_tag_positive_money() {
    use balatro_rs::skip_tags::SkipTagContext;

    let registry = global_registry();
    let economy_tag = registry.get_tag(SkipTagId::Economy).unwrap();

    // Test with $20 - should double to $40
    let mut game = create_test_game();
    game.money = 20.0;

    let context = SkipTagContext {
        game,
        skipped_blind: None,
        available_tags: vec![],
    };

    let result = economy_tag.activate(context);
    assert!(result.success);
    assert_eq!(result.game.money, 40.0); // $20 doubled
    assert!(result.message.as_ref().unwrap().contains("+$20"));
}

#[test]
fn test_economy_tag_max_cap() {
    use balatro_rs::skip_tags::SkipTagContext;

    let registry = global_registry();
    let economy_tag = registry.get_tag(SkipTagId::Economy).unwrap();

    // Test with $50 - should only give +$40 max
    let mut game = create_test_game();
    game.money = 50.0;

    let context = SkipTagContext {
        game,
        skipped_blind: None,
        available_tags: vec![],
    };

    let result = economy_tag.activate(context);
    assert!(result.success);
    assert_eq!(result.game.money, 90.0); // $50 + $40 max
    assert!(result.message.as_ref().unwrap().contains("+$40"));
}

#[test]
fn test_economy_tag_negative_money() {
    use balatro_rs::skip_tags::SkipTagContext;

    let registry = global_registry();
    let economy_tag = registry.get_tag(SkipTagId::Economy).unwrap();

    // Test with negative money - should give $0
    let mut game = create_test_game();
    game.money = -10.0;

    let context = SkipTagContext {
        game,
        skipped_blind: None,
        available_tags: vec![],
    };

    let result = economy_tag.activate(context);
    assert!(result.success);
    assert_eq!(result.game.money, -10.0); // No change
    assert!(result
        .message
        .as_ref()
        .unwrap()
        .contains("negative balance"));
}

#[test]
fn test_investment_tag_basic_functionality() {
    let registry = global_registry();
    let investment_tag = registry.get_tag(SkipTagId::Investment).unwrap();

    // Test tag metadata
    assert_eq!(investment_tag.id(), SkipTagId::Investment);
    assert_eq!(investment_tag.name(), "Investment");
    assert_eq!(
        investment_tag.effect_type(),
        TagEffectType::GameStateModifier
    );
    assert!(investment_tag.description().contains("$25"));
    assert!(investment_tag.description().contains("Boss Blind"));
    assert!(investment_tag.stackable()); // Investment is stackable
}

#[test]
fn test_investment_tag_activation() {
    use balatro_rs::skip_tags::SkipTagContext;

    let registry = global_registry();
    let investment_tag = registry.get_tag(SkipTagId::Investment).unwrap();

    let game = create_test_game();
    assert_eq!(game.active_skip_tags.investment_count, 0);

    let context = SkipTagContext {
        game,
        skipped_blind: None,
        available_tags: vec![],
    };

    let result = investment_tag.activate(context);
    assert!(result.success);
    assert_eq!(result.game.active_skip_tags.investment_count, 1);
    assert!(result.message.as_ref().unwrap().contains("$25"));
    assert!(result.message.as_ref().unwrap().contains("1 investment"));
}

#[test]
fn test_investment_tag_stacking() {
    use balatro_rs::skip_tags::SkipTagContext;

    let registry = global_registry();
    let investment_tag = registry.get_tag(SkipTagId::Investment).unwrap();

    let mut game = create_test_game();

    // Apply investment tag 3 times
    for i in 1..=3 {
        let context = SkipTagContext {
            game,
            skipped_blind: None,
            available_tags: vec![],
        };

        let result = investment_tag.activate(context);
        game = result.game;
        assert_eq!(game.active_skip_tags.investment_count, i);
        assert!(result
            .message
            .as_ref()
            .unwrap()
            .contains(&format!("${}", i * 25)));
    }
}

#[test]
fn test_investment_tag_boss_blind_payout() {
    let mut game = create_test_game();

    // Set up 3 investment tags
    game.active_skip_tags.investment_count = 3;
    let initial_money = game.money;

    // Defeat boss blind
    let reward = game.handle_boss_blind_defeat();
    assert_eq!(reward, 75); // 3 * $25
    assert_eq!(game.money, initial_money + 75.0);
    assert_eq!(game.active_skip_tags.investment_count, 0); // Reset after payout
}

#[test]
fn test_garbage_tag_basic_functionality() {
    let registry = global_registry();
    let garbage_tag = registry.get_tag(SkipTagId::Garbage).unwrap();

    // Test tag metadata
    assert_eq!(garbage_tag.id(), SkipTagId::Garbage);
    assert_eq!(garbage_tag.name(), "Garbage");
    assert_eq!(garbage_tag.effect_type(), TagEffectType::ImmediateReward);
    assert!(garbage_tag.description().contains("$1"));
    assert!(garbage_tag.description().contains("unused discard"));
    assert!(!garbage_tag.stackable());
}

#[test]
fn test_garbage_tag_with_unused_discards() {
    use balatro_rs::skip_tags::SkipTagContext;

    let registry = global_registry();
    let garbage_tag = registry.get_tag(SkipTagId::Garbage).unwrap();

    let mut game = create_test_game();
    game.config.discards = 4; // Starting discards
    game.discards = 3.0; // 3 unused discards remaining
    let initial_money = game.money;

    let context = SkipTagContext {
        game,
        skipped_blind: None,
        available_tags: vec![],
    };

    let result = garbage_tag.activate(context);
    assert!(result.success);
    assert_eq!(result.game.money, initial_money + 3.0); // +$3 for 3 unused
    assert!(result.message.as_ref().unwrap().contains("+$3"));
    assert!(result
        .message
        .as_ref()
        .unwrap()
        .contains("3 unused discards"));
}

#[test]
fn test_garbage_tag_no_unused_discards() {
    use balatro_rs::skip_tags::SkipTagContext;

    let registry = global_registry();
    let garbage_tag = registry.get_tag(SkipTagId::Garbage).unwrap();

    let mut game = create_test_game();
    game.config.discards = 4;
    game.discards = 0.0; // All discards used
    let initial_money = game.money;

    let context = SkipTagContext {
        game,
        skipped_blind: None,
        available_tags: vec![],
    };

    let result = garbage_tag.activate(context);
    assert!(result.success);
    assert_eq!(result.game.money, initial_money); // No reward
    assert!(result.message.as_ref().unwrap().contains("+$0"));
}

#[test]
fn test_speed_tag_basic_functionality() {
    let registry = global_registry();
    let speed_tag = registry.get_tag(SkipTagId::Speed).unwrap();

    // Test tag metadata
    assert_eq!(speed_tag.id(), SkipTagId::Speed);
    assert_eq!(speed_tag.name(), "Speed");
    assert_eq!(speed_tag.effect_type(), TagEffectType::ImmediateReward);
    assert!(speed_tag.description().contains("$5"));
    assert!(speed_tag.description().contains("Blind"));
    assert!(speed_tag.description().contains("skipped"));
    assert!(!speed_tag.stackable());
}

#[test]
fn test_speed_tag_minimum_payout() {
    use balatro_rs::skip_tags::SkipTagContext;

    let registry = global_registry();
    let speed_tag = registry.get_tag(SkipTagId::Speed).unwrap();

    let mut game = create_test_game();
    game.active_skip_tags.blinds_skipped = 0; // No blinds skipped
    let initial_money = game.money;

    let context = SkipTagContext {
        game,
        skipped_blind: None,
        available_tags: vec![],
    };

    let result = speed_tag.activate(context);
    assert!(result.success);
    assert_eq!(result.game.money, initial_money + 5.0); // Minimum $5
    assert!(result.message.as_ref().unwrap().contains("+$5"));
    assert!(result.message.as_ref().unwrap().contains("min $5"));
}

#[test]
fn test_speed_tag_multiple_blinds() {
    use balatro_rs::skip_tags::SkipTagContext;

    let registry = global_registry();
    let speed_tag = registry.get_tag(SkipTagId::Speed).unwrap();

    let mut game = create_test_game();
    game.active_skip_tags.blinds_skipped = 4; // 4 blinds skipped
    let initial_money = game.money;

    let context = SkipTagContext {
        game,
        skipped_blind: None,
        available_tags: vec![],
    };

    let result = speed_tag.activate(context);
    assert!(result.success);
    assert_eq!(result.game.money, initial_money + 20.0); // 4 * $5
    assert!(result.message.as_ref().unwrap().contains("+$20"));
    assert!(result
        .message
        .as_ref()
        .unwrap()
        .contains("4 blind(s) skipped"));
}

#[test]
fn test_handy_tag_basic_functionality() {
    let registry = global_registry();
    let handy_tag = registry.get_tag(SkipTagId::Handy).unwrap();

    // Test tag metadata
    assert_eq!(handy_tag.id(), SkipTagId::Handy);
    assert_eq!(handy_tag.name(), "Handy");
    assert_eq!(handy_tag.effect_type(), TagEffectType::ImmediateReward);
    assert!(handy_tag.description().contains("$1"));
    assert!(handy_tag.description().contains("hand played"));
    assert!(!handy_tag.stackable());
}

#[test]
fn test_handy_tag_with_played_hands() {
    use balatro_rs::skip_tags::SkipTagContext;

    let registry = global_registry();
    let handy_tag = registry.get_tag(SkipTagId::Handy).unwrap();

    let mut game = create_test_game();
    game.plays = 17.0; // 17 hands played (retroactive)
    let initial_money = game.money;

    let context = SkipTagContext {
        game,
        skipped_blind: None,
        available_tags: vec![],
    };

    let result = handy_tag.activate(context);
    assert!(result.success);
    assert_eq!(result.game.money, initial_money + 17.0); // +$17
    assert!(result.message.as_ref().unwrap().contains("+$17"));
    assert!(result.message.as_ref().unwrap().contains("17 hands played"));
}

#[test]
fn test_handy_tag_no_hands_played() {
    use balatro_rs::skip_tags::SkipTagContext;

    let registry = global_registry();
    let handy_tag = registry.get_tag(SkipTagId::Handy).unwrap();

    let mut game = create_test_game();
    game.plays = 0.0; // No hands played
    let initial_money = game.money;

    let context = SkipTagContext {
        game,
        skipped_blind: None,
        available_tags: vec![],
    };

    let result = handy_tag.activate(context);
    assert!(result.success);
    assert_eq!(result.game.money, initial_money); // No reward
    assert!(result.message.as_ref().unwrap().contains("+$0"));
}

#[test]
fn test_all_economic_tags_registered() {
    let registry = global_registry();

    // All economic tags should be registered
    assert!(registry.get_tag(SkipTagId::Economy).is_some());
    assert!(registry.get_tag(SkipTagId::Investment).is_some());
    assert!(registry.get_tag(SkipTagId::Garbage).is_some());
    assert!(registry.get_tag(SkipTagId::Speed).is_some());
    assert!(registry.get_tag(SkipTagId::Handy).is_some());
}

#[test]
fn test_economic_tags_game_integration() {
    let mut game = create_test_game();

    // Set up game state for testing
    game.money = 20.0;
    game.active_skip_tags.blinds_skipped = 2;
    game.discards = 2.0; // 2 unused discards
    game.plays = 10.0; // 10 hands played

    // Apply economic tags through game interface
    let economy_result = game.apply_skip_tag_effect(SkipTagId::Economy).unwrap();
    assert_eq!(economy_result.money_reward, 20); // Doubled $20

    let investment_result = game.apply_skip_tag_effect(SkipTagId::Investment).unwrap();
    assert_eq!(investment_result.money_reward, 0); // No immediate reward

    let garbage_result = game.apply_skip_tag_effect(SkipTagId::Garbage).unwrap();
    assert_eq!(garbage_result.money_reward, 2); // $2 for 2 unused discards

    let speed_result = game.apply_skip_tag_effect(SkipTagId::Speed).unwrap();
    assert_eq!(speed_result.money_reward, 10); // $10 for 2 blinds skipped

    let handy_result = game.apply_skip_tag_effect(SkipTagId::Handy).unwrap();
    assert_eq!(handy_result.money_reward, 10); // $10 for 10 hands played
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
fn test_charm_tag_implemented() {
    let mut game = create_test_game();

    // Charm tag is now implemented in PR #884 - test that it succeeds
    let result = game.apply_skip_tag_effect(SkipTagId::Charm);
    assert!(result.is_ok(), "Charm tag should now be implemented");

    // The Charm tag gives a free Mega Arcana Pack
    if let Ok(result) = result {
        assert!(result.success);
        // Just verify that we get a success message, don't check the exact content
        // since the implementation details may vary
        assert!(result.message.is_some(), "Should have a message");
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
