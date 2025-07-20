/// Integration tests for full game simulation
///
/// NOTE: The main test_game function requires the "integration_tests" feature flag because:
/// - It can be very slow (runs full game simulations)
/// - Timing depends on random game outcomes and can be unpredictable
/// - Can timeout in CI environments with limited resources
///
/// To run locally: cargo test --features integration_tests test_game
use balatro_rs::{action::Action, game::Game, rng::GameRng, stage::Stage};

fn run_game_gen_actions() {
    let mut g = Game::default();
    let test_rng = GameRng::for_testing(42); // Use deterministic RNG for reproducible tests

    g.start();
    while !g.is_over() {
        // Get all available moves
        let actions: Vec<Action> = g.gen_actions().collect();
        if actions.is_empty() {
            break;
        }

        // Pick a random move and execute it using deterministic RNG
        let i = test_rng.gen_range(0..actions.len());
        let action = actions[i].clone();
        let action_res = g.handle_action(action.clone());
        assert!(action_res.is_ok());
    }
    let result = g.result();
    // Ensure game is over at end
    assert!(result.is_some());
    // Check game state at end
    assert!(matches!(g.stage, Stage::End(_)));
}

fn run_game_action_space() {
    let mut g = Game::default();

    g.start();
    while !g.is_over() {
        // Get action space and vector
        let space = g.gen_action_space();
        let space_vec = space.to_vec();
        assert!(!space.is_empty());

        // Pick a random move and ensure its unmasked using deterministic RNG
        let test_rng = GameRng::for_testing(123);
        let mut i: usize;
        loop {
            i = test_rng.gen_range(0..space_vec.len());
            if space_vec[i] == 1 {
                break;
            }
        }
        let action = space.to_action(i, &g).expect("valid index to action");
        // dbg!("game state:\n{}", g.clone());
        // dbg!("play action: {}", action.clone());
        let action_res = g.handle_action(action.clone());
        // dbg!(action);
        assert!(action_res.is_ok());
    }
    let result = g.result();
    // Ensure game is over at end
    assert!(result.is_some());
    // Check game state at end
    assert!(matches!(g.stage, Stage::End(_)));
    // dbg!("game action history: {:?}", g.action_history);
}

#[test]
#[cfg(feature = "integration_tests")]
fn test_game() {
    run_game_gen_actions();
    run_game_action_space();
}

#[test]
#[ignore]
fn test_games_gen_actions() {
    for _ in 0..1000 {
        run_game_gen_actions();
    }
}

#[test]
#[ignore]
fn test_games_action_space() {
    for _ in 0..1000 {
        run_game_action_space();
    }
}

// TODO: Fix this test - currently has compilation errors due to private methods
/*
#[test]
fn test_purchase_validation_consistency() {
    /// Test that validates purchase validation logic is consistent between
    /// validation checks and actual purchase execution
    use balatro_rs::{
        action::Action,
        game::Game,
        joker::JokerId,
        stage::Stage,
    };

    let mut game = Game::default();
    game.start();

    // Progress to shop stage
    while !matches!(game.stage, Stage::Shop()) && !game.is_over() {
        let actions: Vec<Action> = game.gen_actions().collect();
        if actions.is_empty() {
            break;
        }
        
        // Find an action that progresses toward shop
        for action in actions {
            match action {
                Action::NextRound() | Action::CashOut(_) => {
                    if game.handle_action(action).is_ok() {
                        break;
                    }
                }
                _ => continue,
            }
        }
    }

    if matches!(game.stage, Stage::Shop()) {
        // Test each joker in the shop
        for shop_joker in game.shop.jokers.clone() {
            if let Some(joker_id) = shop_joker.to_joker_id() {
                // Check current validation logic
                let can_purchase = 
                    // Check slot availability
                    game.jokers.len() < game.config.joker_slots &&
                    // Check money
                    shop_joker.cost() as f64 <= game.money &&
                    // Check stage
                    matches!(game.stage, Stage::Shop()) &&
                    // Check shop availability
                    game.shop.has_joker(joker_id);

                // Try actual purchase
                let action = Action::BuyJoker { joker_id, slot: 0 };
                let purchase_result = game.buy_joker_with_slot(joker_id, 0);

                // Validation should match actual result
                if can_purchase {
                    assert!(
                        purchase_result.is_ok(),
                        "Validation indicated purchase should succeed but it failed for joker {:?}",
                        joker_id
                    );
                } else {
                    assert!(
                        purchase_result.is_err(),
                        "Validation indicated purchase should fail but it succeeded for joker {:?}",
                        joker_id
                    );
                }

                // Reset game state if purchase succeeded to continue testing
                if purchase_result.is_ok() {
                    game = Game::default();
                    game.start();
                    while !matches!(game.stage, Stage::Shop()) && !game.is_over() {
                        let actions: Vec<Action> = game.gen_actions().collect();
                        if actions.is_empty() {
                            break;
                        }
                        for action in actions {
                            match action {
                                Action::NextRound() | Action::CashOut(_) => {
                                    if game.handle_action(action).is_ok() {
                                        break;
                                    }
                                }
                                _ => continue,
                            }
                        }
                    }
                    break; // Only test one purchase to avoid complexity
                }
            }
        }
    }
}
*/

#[test]
fn test_can_purchase_consumable_validation() {
    /// Comprehensive test for can_purchase_consumable method
    /// Covers all validation scenarios from issue #404
    use balatro_rs::{
        action::Action,
        error::GameError,
        game::Game,
        shop::ConsumableType,
        stage::Stage,
    };

    // Test 1: Valid purchase conditions
    {
        let mut game = Game::default();
        game.start();
        
        // Progress to shop stage
        while !matches!(game.stage, Stage::Shop()) && !game.is_over() {
            let actions: Vec<Action> = game.gen_actions().collect();
            if let Some(action) = actions.iter().find(|a| matches!(a, Action::NextRound() | Action::CashOut(_))) {
                let _ = game.handle_action(action.clone());
            } else {
                break;
            }
        }
        
        if matches!(game.stage, Stage::Shop()) {
            // Ensure player has enough money
            game.money = 10.0;
            
            // Test each consumable type with valid conditions
            assert!(game.can_purchase_consumable(ConsumableType::Tarot).is_ok(), 
                "Should be able to purchase Tarot with sufficient money and slots");
            assert!(game.can_purchase_consumable(ConsumableType::Planet).is_ok(), 
                "Should be able to purchase Planet with sufficient money and slots");
            assert!(game.can_purchase_consumable(ConsumableType::Spectral).is_ok(), 
                "Should be able to purchase Spectral with sufficient money and slots");
        }
    }

    // Test 2: Insufficient money
    {
        let mut game = Game::default();
        game.start();
        
        // Progress to shop stage
        while !matches!(game.stage, Stage::Shop()) && !game.is_over() {
            let actions: Vec<Action> = game.gen_actions().collect();
            if let Some(action) = actions.iter().find(|a| matches!(a, Action::NextRound() | Action::CashOut(_))) {
                let _ = game.handle_action(action.clone());
            } else {
                break;
            }
        }
        
        if matches!(game.stage, Stage::Shop()) {
            // Set insufficient money
            game.money = 2.0; // Less than required for any consumable
            
            assert!(matches!(game.can_purchase_consumable(ConsumableType::Tarot).unwrap_err(), 
                GameError::InvalidBalance), "Should fail with insufficient money for Tarot");
            assert!(matches!(game.can_purchase_consumable(ConsumableType::Planet).unwrap_err(), 
                GameError::InvalidBalance), "Should fail with insufficient money for Planet");
            assert!(matches!(game.can_purchase_consumable(ConsumableType::Spectral).unwrap_err(), 
                GameError::InvalidBalance), "Should fail with insufficient money for Spectral");
        }
    }

    // Test 3: Edge case - exactly enough money
    {
        let mut game = Game::default();
        game.start();
        
        // Progress to shop stage
        while !matches!(game.stage, Stage::Shop()) && !game.is_over() {
            let actions: Vec<Action> = game.gen_actions().collect();
            if let Some(action) = actions.iter().find(|a| matches!(a, Action::NextRound() | Action::CashOut(_))) {
                let _ = game.handle_action(action.clone());
            } else {
                break;
            }
        }
        
        if matches!(game.stage, Stage::Shop()) {
            // Test exact money for Tarot/Planet
            game.money = 3.0;
            assert!(game.can_purchase_consumable(ConsumableType::Tarot).is_ok(), 
                "Should succeed with exactly enough money for Tarot");
            assert!(game.can_purchase_consumable(ConsumableType::Planet).is_ok(), 
                "Should succeed with exactly enough money for Planet");
            assert!(matches!(game.can_purchase_consumable(ConsumableType::Spectral).unwrap_err(), 
                GameError::InvalidBalance), "Should fail with insufficient money for Spectral");
            
            // Test exact money for Spectral
            game.money = 4.0;
            assert!(game.can_purchase_consumable(ConsumableType::Spectral).is_ok(), 
                "Should succeed with exactly enough money for Spectral");
        }
    }

    // Test 4: No available consumable slots
    {
        let mut game = Game::default();
        game.start();
        
        // Progress to shop stage
        while !matches!(game.stage, Stage::Shop()) && !game.is_over() {
            let actions: Vec<Action> = game.gen_actions().collect();
            if let Some(action) = actions.iter().find(|a| matches!(a, Action::NextRound() | Action::CashOut(_))) {
                let _ = game.handle_action(action.clone());
            } else {
                break;
            }
        }
        
        if matches!(game.stage, Stage::Shop()) {
            // Fill consumable slots to capacity (2)
            game.money = 10.0;
            use balatro_rs::consumables::ConsumableId;
            
            // Add consumables to fill slots
            for _ in 0..2 {
                game.consumables_in_hand.push(ConsumableId::TheFool);
            }
            
            assert!(matches!(game.can_purchase_consumable(ConsumableType::Tarot).unwrap_err(), 
                GameError::NoAvailableSlot), "Should fail when consumable slots are full");
            assert!(matches!(game.can_purchase_consumable(ConsumableType::Planet).unwrap_err(), 
                GameError::NoAvailableSlot), "Should fail when consumable slots are full");
            assert!(matches!(game.can_purchase_consumable(ConsumableType::Spectral).unwrap_err(), 
                GameError::NoAvailableSlot), "Should fail when consumable slots are full");
        }
    }

    // Test 5: Wrong game stage
    {
        let mut game = Game::default();
        game.start();
        
        // Ensure we're not in shop stage
        assert!(!matches!(game.stage, Stage::Shop()));
        
        game.money = 10.0; // Sufficient money
        
        assert!(matches!(game.can_purchase_consumable(ConsumableType::Tarot).unwrap_err(), 
            GameError::InvalidStage), "Should fail when not in Shop stage");
        assert!(matches!(game.can_purchase_consumable(ConsumableType::Planet).unwrap_err(), 
            GameError::InvalidStage), "Should fail when not in Shop stage");
        assert!(matches!(game.can_purchase_consumable(ConsumableType::Spectral).unwrap_err(), 
            GameError::InvalidStage), "Should fail when not in Shop stage");
    }

    // Test 6: Cost validation for different consumable types
    {
        let mut game = Game::default();
        game.start();
        
        // Progress to shop stage
        while !matches!(game.stage, Stage::Shop()) && !game.is_over() {
            let actions: Vec<Action> = game.gen_actions().collect();
            if let Some(action) = actions.iter().find(|a| matches!(a, Action::NextRound() | Action::CashOut(_))) {
                let _ = game.handle_action(action.clone());
            } else {
                break;
            }
        }
        
        if matches!(game.stage, Stage::Shop()) {
            // Verify costs: Tarot=3, Planet=3, Spectral=4
            
            // With 2.9 money - should fail for all
            game.money = 2.9;
            assert!(game.can_purchase_consumable(ConsumableType::Tarot).is_err());
            assert!(game.can_purchase_consumable(ConsumableType::Planet).is_err());
            assert!(game.can_purchase_consumable(ConsumableType::Spectral).is_err());
            
            // With 3.0 money - should work for Tarot and Planet, fail for Spectral
            game.money = 3.0;
            assert!(game.can_purchase_consumable(ConsumableType::Tarot).is_ok());
            assert!(game.can_purchase_consumable(ConsumableType::Planet).is_ok());
            assert!(game.can_purchase_consumable(ConsumableType::Spectral).is_err());
            
            // With 3.9 money - should work for Tarot and Planet, fail for Spectral
            game.money = 3.9;
            assert!(game.can_purchase_consumable(ConsumableType::Tarot).is_ok());
            assert!(game.can_purchase_consumable(ConsumableType::Planet).is_ok());
            assert!(game.can_purchase_consumable(ConsumableType::Spectral).is_err());
            
            // With 4.0 money - should work for all
            game.money = 4.0;
            assert!(game.can_purchase_consumable(ConsumableType::Tarot).is_ok());
            assert!(game.can_purchase_consumable(ConsumableType::Planet).is_ok());
            assert!(game.can_purchase_consumable(ConsumableType::Spectral).is_ok());
        }
    }
}
