use balatro_rs::action::Action;
use balatro_rs::config::Config;
use balatro_rs::game::Game;
use balatro_rs::shop::packs::{Pack, PackType};
use balatro_rs::vouchers::VoucherId;

/// Test helper to create a game in shop stage with sufficient money
fn create_shop_game() -> Game {
    let mut game = Game::default();
    game.start();

    // For testing purposes, directly set the game to shop stage
    // This bypasses the complex game progression logic which is tested elsewhere
    use balatro_rs::stage::Stage;
    game.stage = Stage::Shop();

    // Ensure player has enough money for pack purchases
    game.money = 20.0;
    game
}

/// Test helper to get available pack actions from game
fn get_pack_actions(game: &Game) -> Vec<Action> {
    game.gen_actions()
        .filter(|action| matches!(action, Action::BuyPack { .. }))
        .collect()
}

/// Test helper to check if pack is available for purchase
fn has_pack_available(game: &Game, pack_type: PackType) -> bool {
    get_pack_actions(game)
        .iter()
        .any(|action| matches!(action, Action::BuyPack { pack_type: pt } if *pt == pack_type))
}

#[test]
fn test_standard_pack_choose_one_of_three_cards() {
    let mut game = create_shop_game();

    // Standard pack should be available for purchase
    assert!(has_pack_available(&game, PackType::Standard));

    // Buy standard pack
    let buy_action = Action::BuyPack {
        pack_type: PackType::Standard,
    };
    let result = game.handle_action(buy_action);
    assert!(result.is_ok(), "Should be able to buy standard pack");

    // Player should now have a pack in inventory
    assert_eq!(game.pack_inventory.len(), 1);
    assert_eq!(game.pack_inventory[0].pack_type, PackType::Standard);

    // Open the pack
    let open_action = Action::OpenPack { pack_id: 0 };
    let result = game.handle_action(open_action);
    assert!(result.is_ok(), "Should be able to open standard pack");

    // Pack should have 3 options (playing cards)
    let open_pack = game.open_pack.as_ref().expect("Pack should be opened");
    assert_eq!(
        open_pack.pack.options.len(),
        3,
        "Standard pack should have 3 options"
    );
    assert_eq!(
        open_pack.pack.choose_count, 1,
        "Should choose 1 from standard pack"
    );

    // All options should be playing cards
    for option in &open_pack.pack.options {
        assert!(
            matches!(option.item, balatro_rs::shop::ShopItem::PlayingCard(_)),
            "Standard pack should contain only playing cards"
        );
    }

    // Select first option
    let select_action = Action::SelectFromPack {
        pack_id: 0,
        option_index: 0,
    };
    let result = game.handle_action(select_action);
    assert!(result.is_ok(), "Should be able to select from pack");

    // Pack should be consumed after selection
    assert!(
        game.open_pack.is_none(),
        "Pack should be consumed after selection"
    );
    assert_eq!(
        game.pack_inventory.len(),
        0,
        "Pack should be removed from inventory"
    );
}

#[test]
fn test_buffoon_pack_choose_one_of_two_jokers() {
    let mut game = create_shop_game();

    // Buffoon pack should be available for purchase
    assert!(has_pack_available(&game, PackType::Buffoon));

    // Buy buffoon pack
    let buy_action = Action::BuyPack {
        pack_type: PackType::Buffoon,
    };
    let result = game.handle_action(buy_action);
    assert!(result.is_ok(), "Should be able to buy buffoon pack");

    // Open the pack
    let open_action = Action::OpenPack { pack_id: 0 };
    let result = game.handle_action(open_action);
    assert!(result.is_ok(), "Should be able to open buffoon pack");

    // Pack should have 2 options (jokers)
    let open_pack = game.open_pack.as_ref().expect("Pack should be opened");
    assert_eq!(
        open_pack.pack.options.len(),
        2,
        "Buffoon pack should have 2 options"
    );
    assert_eq!(
        open_pack.pack.choose_count, 1,
        "Should choose 1 from buffoon pack"
    );

    // All options should be jokers
    for option in &open_pack.pack.options {
        assert!(
            matches!(option.item, balatro_rs::shop::ShopItem::Joker(_)),
            "Buffoon pack should contain only jokers"
        );
    }
}

#[test]
fn test_arcana_pack_choose_one_of_tarot_cards() {
    let mut game = create_shop_game();

    // Arcana pack should be available for purchase
    assert!(has_pack_available(&game, PackType::Arcana));

    // Buy arcana pack
    let buy_action = Action::BuyPack {
        pack_type: PackType::Arcana,
    };
    let result = game.handle_action(buy_action);
    assert!(result.is_ok(), "Should be able to buy arcana pack");

    // Open the pack
    let open_action = Action::OpenPack { pack_id: 0 };
    let result = game.handle_action(open_action);
    assert!(result.is_ok(), "Should be able to open arcana pack");

    // Pack should have 2-3 options (tarot cards)
    let open_pack = game.open_pack.as_ref().expect("Pack should be opened");
    assert!(
        open_pack.pack.options.len() >= 2 && open_pack.pack.options.len() <= 3,
        "Arcana pack should have 2-3 options"
    );
    assert_eq!(
        open_pack.pack.choose_count, 1,
        "Should choose 1 from arcana pack"
    );

    // All options should be tarot consumables or Soul (special spectral card)
    for option in &open_pack.pack.options {
        assert!(
            matches!(
                option.item,
                balatro_rs::shop::ShopItem::Consumable(balatro_rs::shop::ConsumableType::Tarot)
                    | balatro_rs::shop::ShopItem::Consumable(
                        balatro_rs::shop::ConsumableType::Spectral
                    )
            ),
            "Arcana pack should contain only tarot cards or Soul spectral card"
        );
    }
}

#[test]
fn test_celestial_pack_choose_one_of_planet_cards() {
    let mut game = create_shop_game();

    // Celestial pack should be available for purchase
    assert!(has_pack_available(&game, PackType::Celestial));

    // Buy celestial pack
    let buy_action = Action::BuyPack {
        pack_type: PackType::Celestial,
    };
    let result = game.handle_action(buy_action);
    assert!(result.is_ok(), "Should be able to buy celestial pack");

    // Open the pack
    let open_action = Action::OpenPack { pack_id: 0 };
    let result = game.handle_action(open_action);
    assert!(result.is_ok(), "Should be able to open celestial pack");

    // Pack should have 2-3 options (planet cards)
    let open_pack = game.open_pack.as_ref().expect("Pack should be opened");
    assert!(
        open_pack.pack.options.len() >= 2 && open_pack.pack.options.len() <= 3,
        "Celestial pack should have 2-3 options"
    );
    assert_eq!(
        open_pack.pack.choose_count, 1,
        "Should choose 1 from celestial pack"
    );

    // All options should be planet consumables or Black Hole (special spectral card)
    for option in &open_pack.pack.options {
        assert!(
            matches!(
                option.item,
                balatro_rs::shop::ShopItem::Consumable(balatro_rs::shop::ConsumableType::Planet)
                    | balatro_rs::shop::ShopItem::Consumable(
                        balatro_rs::shop::ConsumableType::Spectral
                    )
            ),
            "Celestial pack should contain only planet cards or Black Hole spectral card"
        );
    }
}

#[test]
fn test_spectral_pack_choose_one_of_spectral_cards() {
    let mut game = create_shop_game();

    // Spectral pack should be available for purchase
    assert!(has_pack_available(&game, PackType::Spectral));

    // Buy spectral pack
    let buy_action = Action::BuyPack {
        pack_type: PackType::Spectral,
    };
    let result = game.handle_action(buy_action);
    assert!(result.is_ok(), "Should be able to buy spectral pack");

    // Open the pack
    let open_action = Action::OpenPack { pack_id: 0 };
    let result = game.handle_action(open_action);
    assert!(result.is_ok(), "Should be able to open spectral pack");

    // Pack should have 2-3 options (spectral cards)
    let open_pack = game.open_pack.as_ref().expect("Pack should be opened");
    assert!(
        open_pack.pack.options.len() >= 2 && open_pack.pack.options.len() <= 3,
        "Spectral pack should have 2-3 options"
    );
    assert_eq!(
        open_pack.pack.choose_count, 1,
        "Should choose 1 from spectral pack"
    );

    // All options should be spectral consumables
    for option in &open_pack.pack.options {
        assert!(
            matches!(
                option.item,
                balatro_rs::shop::ShopItem::Consumable(balatro_rs::shop::ConsumableType::Spectral)
            ),
            "Spectral pack should contain only spectral cards"
        );
    }
}

#[test]
fn test_mega_pack_variants_double_options() {
    let mut game = create_shop_game();

    // Ensure player has enough money for mega packs
    game.money = 50.0;

    // Test Mega Buffoon pack
    if has_pack_available(&game, PackType::MegaBuffoon) {
        let buy_action = Action::BuyPack {
            pack_type: PackType::MegaBuffoon,
        };
        let result = game.handle_action(buy_action);
        assert!(result.is_ok(), "Should be able to buy mega buffoon pack");

        let open_action = Action::OpenPack { pack_id: 0 };
        let result = game.handle_action(open_action);
        assert!(result.is_ok(), "Should be able to open mega buffoon pack");

        let open_pack = game.open_pack.as_ref().expect("Pack should be opened");
        assert_eq!(
            open_pack.pack.options.len(),
            4,
            "Mega buffoon pack should have 4 options"
        );

        // Clear pack for next test
        let select_action = Action::SelectFromPack {
            pack_id: 0,
            option_index: 0,
        };
        let _ = game.handle_action(select_action);
    }

    // Test Mega Arcana pack
    if has_pack_available(&game, PackType::MegaArcana) {
        let buy_action = Action::BuyPack {
            pack_type: PackType::MegaArcana,
        };
        let result = game.handle_action(buy_action);
        assert!(result.is_ok(), "Should be able to buy mega arcana pack");

        let open_action = Action::OpenPack {
            pack_id: game.pack_inventory.len() - 1,
        };
        let result = game.handle_action(open_action);
        assert!(result.is_ok(), "Should be able to open mega arcana pack");

        let open_pack = game.open_pack.as_ref().expect("Pack should be opened");
        assert!(
            open_pack.pack.options.len() >= 4 && open_pack.pack.options.len() <= 6,
            "Mega arcana pack should have 4-6 options"
        );
    }
}

#[test]
fn test_pack_skip_mechanics() {
    let mut game = create_shop_game();

    // Buy a pack
    let buy_action = Action::BuyPack {
        pack_type: PackType::Standard,
    };
    let result = game.handle_action(buy_action);
    assert!(result.is_ok(), "Should be able to buy pack");

    // Open the pack
    let open_action = Action::OpenPack { pack_id: 0 };
    let result = game.handle_action(open_action);
    assert!(result.is_ok(), "Should be able to open pack");

    // Pack should be skippable
    let open_pack = game.open_pack.as_ref().expect("Pack should be opened");
    assert!(open_pack.pack.can_skip, "Pack should be skippable");

    // Skip the pack
    let skip_action = Action::SkipPack { pack_id: 0 };
    let result = game.handle_action(skip_action);
    assert!(result.is_ok(), "Should be able to skip pack");

    // Pack should be consumed after skipping
    assert!(
        game.open_pack.is_none(),
        "Pack should be consumed after skipping"
    );
    assert_eq!(
        game.pack_inventory.len(),
        0,
        "Pack should be removed from inventory"
    );
}

#[test]
fn test_pack_costs() {
    let _game = create_shop_game();
    let config = Config::new();

    // Create pack instances to test costs
    let standard_pack = Pack::new(PackType::Standard, &config);
    let buffoon_pack = Pack::new(PackType::Buffoon, &config);
    let arcana_pack = Pack::new(PackType::Arcana, &config);
    let celestial_pack = Pack::new(PackType::Celestial, &config);
    let spectral_pack = Pack::new(PackType::Spectral, &config);

    // All basic packs should cost $4
    assert_eq!(standard_pack.cost, 4, "Standard pack should cost $4");
    assert_eq!(buffoon_pack.cost, 4, "Buffoon pack should cost $4");
    assert_eq!(arcana_pack.cost, 4, "Arcana pack should cost $4");
    assert_eq!(celestial_pack.cost, 4, "Celestial pack should cost $4");
    assert_eq!(spectral_pack.cost, 4, "Spectral pack should cost $4");

    // Mega packs should cost double
    let mega_buffoon_pack = Pack::new(PackType::MegaBuffoon, &config);
    let mega_arcana_pack = Pack::new(PackType::MegaArcana, &config);
    let mega_celestial_pack = Pack::new(PackType::MegaCelestial, &config);

    assert_eq!(
        mega_buffoon_pack.cost, 8,
        "Mega buffoon pack should cost $8"
    );
    assert_eq!(mega_arcana_pack.cost, 8, "Mega arcana pack should cost $8");
    assert_eq!(
        mega_celestial_pack.cost, 8,
        "Mega celestial pack should cost $8"
    );
}

#[test]
fn test_insufficient_funds_pack_purchase() {
    let mut game = create_shop_game();

    // Set player money to insufficient amount
    game.money = 2.0;

    // Try to buy a pack that costs $4
    let buy_action = Action::BuyPack {
        pack_type: PackType::Standard,
    };
    let result = game.handle_action(buy_action);
    assert!(
        result.is_err(),
        "Should not be able to buy pack with insufficient funds"
    );

    // Pack inventory should remain empty
    assert_eq!(game.pack_inventory.len(), 0, "No pack should be purchased");
}

#[test]
fn test_pack_move_generation_for_ai() {
    let game = create_shop_game();

    // Check that pack purchase actions are generated for AI
    let actions: Vec<Action> = game.gen_actions().collect();
    let pack_actions: Vec<&Action> = actions
        .iter()
        .filter(|action| matches!(action, Action::BuyPack { .. }))
        .collect();

    // Should have pack purchase actions available
    assert!(
        !pack_actions.is_empty(),
        "Should generate pack purchase actions for AI"
    );

    // Should have different pack types available
    let pack_types: std::collections::HashSet<PackType> = pack_actions
        .iter()
        .filter_map(|action| match action {
            Action::BuyPack { pack_type } => Some(*pack_type),
            _ => None,
        })
        .collect();

    assert!(
        pack_types.len() > 1,
        "Should have multiple pack types available"
    );
}

#[test]
fn test_grab_bag_voucher_adds_option() {
    // This test will be implemented when voucher system is integrated
    // For now, create placeholder test structure

    let mut game = create_shop_game();

    // Add Grab Bag voucher to player inventory
    game.vouchers.add(VoucherId::GrabBag);

    // Buy a pack
    let buy_action = Action::BuyPack {
        pack_type: PackType::Standard,
    };
    let result = game.handle_action(buy_action);
    assert!(result.is_ok(), "Should be able to buy pack");

    // Open the pack
    let open_action = Action::OpenPack { pack_id: 0 };
    let result = game.handle_action(open_action);
    assert!(result.is_ok(), "Should be able to open pack");

    // With Grab Bag voucher, pack should have +1 option
    let open_pack = game.open_pack.as_ref().expect("Pack should be opened");
    assert_eq!(
        open_pack.pack.options.len(),
        4,
        "Standard pack with Grab Bag should have 4 options"
    );
}

#[test]
fn test_pack_costs_use_config_values() {
    // Test that PackType uses config values for costs
    let mut custom_config = Config::new();
    custom_config.pack_standard_cost = 10; // Change from default 4
    custom_config.pack_buffoon_cost = 15; // Change from default 4

    // Test that PackType methods use config values
    assert_eq!(PackType::Standard.base_cost(&custom_config), 10);
    assert_eq!(PackType::Buffoon.base_cost(&custom_config), 15);

    // Compare with default config to ensure they're different
    let default_config = Config::new();
    assert_ne!(
        PackType::Standard.base_cost(&custom_config),
        PackType::Standard.base_cost(&default_config)
    );
    assert_ne!(
        PackType::Buffoon.base_cost(&custom_config),
        PackType::Buffoon.base_cost(&default_config)
    );

    // Test that Pack creation uses config costs
    let custom_pack = Pack::new(PackType::Standard, &custom_config);
    let default_pack = Pack::new(PackType::Standard, &default_config);
    assert_eq!(custom_pack.cost, 10);
    assert_eq!(default_pack.cost, 4);
    assert_ne!(custom_pack.cost, default_pack.cost);
}

#[test]
fn test_pack_option_counts_use_config_values() {
    // Test that PackType uses config values for option counts
    let mut custom_config = Config::new();
    custom_config.pack_standard_options = (5, 5); // Change from default (3, 3)
    custom_config.pack_buffoon_options = (4, 4); // Change from default (2, 2)

    // Test that PackType methods use config values
    assert_eq!(PackType::Standard.option_count(&custom_config), (5, 5));
    assert_eq!(PackType::Buffoon.option_count(&custom_config), (4, 4));

    // Compare with default config to ensure they're different
    let default_config = Config::new();
    assert_ne!(
        PackType::Standard.option_count(&custom_config),
        PackType::Standard.option_count(&default_config)
    );
    assert_ne!(
        PackType::Buffoon.option_count(&custom_config),
        PackType::Buffoon.option_count(&default_config)
    );
}

#[test]
fn test_default_config_values_match_original_hardcoded_values() {
    // This test ensures backward compatibility by verifying that default
    // config values match the original hardcoded values
    let config = Config::new();

    // Test pack costs match original hardcoded values
    assert_eq!(config.pack_standard_cost, 4);
    assert_eq!(config.pack_buffoon_cost, 4);
    assert_eq!(config.pack_consumable_cost, 4);
    assert_eq!(config.pack_mega_consumable_cost, 8);

    // Test enhancement rate matches original 10%
    assert_eq!(config.enhancement_rate, 0.1);

    // Test joker rarity weights match original 70/25/5
    assert_eq!(config.joker_rarity_weight_common, 70);
    assert_eq!(config.joker_rarity_weight_uncommon, 25);
    assert_eq!(config.joker_rarity_weight_rare, 5);

    // Test pack option counts match original values
    assert_eq!(config.pack_standard_options, (3, 3));
    assert_eq!(config.pack_buffoon_options, (2, 2));
    assert_eq!(config.pack_consumable_options, (2, 3));
}

#[test]
fn test_config_can_be_modified_at_runtime() {
    // Test that config changes affect pack behavior immediately
    let mut config = Config::new();

    // Create a pack with default config
    let original_pack = Pack::new(PackType::Standard, &config);
    assert_eq!(original_pack.cost, 4);

    // Modify config
    config.pack_standard_cost = 99;

    // Create a new pack with modified config
    let modified_pack = Pack::new(PackType::Standard, &config);
    assert_eq!(modified_pack.cost, 99);

    // Verify the change took effect
    assert_ne!(original_pack.cost, modified_pack.cost);
}

#[test]
fn test_all_pack_types_use_config_for_costs() {
    let mut custom_config = Config::new();
    custom_config.pack_standard_cost = 10;
    custom_config.pack_jumbo_cost = 12;
    custom_config.pack_mega_cost = 14;
    custom_config.pack_enhanced_cost = 11;
    custom_config.pack_variety_cost = 13;
    custom_config.pack_buffoon_cost = 15;
    custom_config.pack_consumable_cost = 16;
    custom_config.pack_mega_consumable_cost = 20;

    // Test that all pack types use config values
    let pack_types_and_expected_costs = [
        (PackType::Standard, 10),
        (PackType::Jumbo, 12),
        (PackType::Mega, 14),
        (PackType::Enhanced, 11),
        (PackType::Variety, 13),
        (PackType::Buffoon, 15),
        (PackType::Arcana, 16),
        (PackType::Celestial, 16),
        (PackType::Spectral, 16),
        (PackType::MegaBuffoon, 20),
        (PackType::MegaArcana, 20),
        (PackType::MegaCelestial, 20),
        (PackType::MegaSpectral, 20),
    ];

    for (pack_type, expected_cost) in pack_types_and_expected_costs {
        let pack = Pack::new(pack_type, &custom_config);
        assert_eq!(
            pack.cost, expected_cost,
            "Pack type {pack_type:?} should use config cost {expected_cost}"
        );

        // Also verify the base_cost method returns the expected value
        assert_eq!(
            pack_type.base_cost(&custom_config),
            expected_cost,
            "PackType {pack_type:?} base_cost should return {expected_cost}"
        );
    }
}
