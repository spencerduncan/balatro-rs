use balatro_rs::action::Action;
use balatro_rs::game::Game;
use balatro_rs::shop::packs::{Pack, PackType};

/// Test helper to create a game in shop stage with sufficient money
fn create_shop_game() -> Game {
    let mut game = Game::default();
    game.start();

    // For testing purposes, directly set the game to shop stage
    // This bypasses the complex game progression logic which is tested elsewhere
    use balatro_rs::stage::Stage;
    game.stage = Stage::Shop();

    // Ensure player has enough money for pack purchases
    game.money = 20;
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

    // All options should be tarot consumables
    for option in &open_pack.pack.options {
        assert!(
            matches!(
                option.item,
                balatro_rs::shop::ShopItem::Consumable(balatro_rs::shop::ConsumableType::Tarot)
            ),
            "Arcana pack should contain only tarot cards"
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

    // All options should be planet consumables
    for option in &open_pack.pack.options {
        assert!(
            matches!(
                option.item,
                balatro_rs::shop::ShopItem::Consumable(balatro_rs::shop::ConsumableType::Planet)
            ),
            "Celestial pack should contain only planet cards"
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
    game.money = 50;

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

    // Create pack instances to test costs
    let standard_pack = Pack::new(PackType::Standard);
    let buffoon_pack = Pack::new(PackType::Buffoon);
    let arcana_pack = Pack::new(PackType::Arcana);
    let celestial_pack = Pack::new(PackType::Celestial);
    let spectral_pack = Pack::new(PackType::Spectral);

    // All basic packs should cost $4
    assert_eq!(standard_pack.cost, 4, "Standard pack should cost $4");
    assert_eq!(buffoon_pack.cost, 4, "Buffoon pack should cost $4");
    assert_eq!(arcana_pack.cost, 4, "Arcana pack should cost $4");
    assert_eq!(celestial_pack.cost, 4, "Celestial pack should cost $4");
    assert_eq!(spectral_pack.cost, 4, "Spectral pack should cost $4");

    // Mega packs should cost double
    let mega_buffoon_pack = Pack::new(PackType::MegaBuffoon);
    let mega_arcana_pack = Pack::new(PackType::MegaArcana);
    let mega_celestial_pack = Pack::new(PackType::MegaCelestial);

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
    game.money = 2;

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

    // TODO: Add Grab Bag voucher to player inventory
    // game.vouchers.add(VoucherId::GrabBag);

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
    // TODO: Implement when voucher integration is complete
    // let open_pack = game.open_pack.as_ref().expect("Pack should be opened");
    // assert_eq!(open_pack.pack.options.len(), 4, "Standard pack with Grab Bag should have 4 options");
}
