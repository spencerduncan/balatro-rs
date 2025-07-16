// Integration tests for JokerStateManager integration with Game struct
// Tests for issue #165 - proper lifecycle management and state persistence

use balatro_rs::config::Config;
use balatro_rs::game::Game;
use balatro_rs::joker::compat::TheJoker;
use balatro_rs::joker::{JokerId, Jokers};
use balatro_rs::joker_state::JokerState;

#[test]
fn test_joker_removal_cleans_up_state() {
    // Test that removing a joker also removes its state
    let mut game = Game::new(Config::default());

    // Add a joker and give it some state
    let joker = Jokers::TheJoker(TheJoker {});
    game.jokers.push(joker);

    // Add some state for this joker
    game.joker_state_manager.set_state(
        JokerId::Joker,
        JokerState {
            accumulated_value: 100.0,
            triggers_remaining: Some(5),
            custom_data: std::collections::HashMap::new(),
        },
    );

    // Verify state exists
    assert!(game.joker_state_manager.get_state(JokerId::Joker).is_some());

    // Remove the joker - this should clean up state
    let result = game.remove_joker(0);
    assert!(result.is_ok());

    // Verify joker is removed
    assert_eq!(game.jokers.len(), 0);

    // Verify state is cleaned up
    assert!(game.joker_state_manager.get_state(JokerId::Joker).is_none());
}

#[test]
fn test_sell_joker_cleans_up_state() {
    // Test that selling a joker also removes its state
    let mut game = Game::new(Config::default());

    // Add a joker with state
    let joker = Jokers::TheJoker(TheJoker {});
    game.jokers.push(joker);
    game.joker_state_manager
        .set_state(JokerId::Joker, JokerState::default());

    let initial_money = game.money.load(std::sync::atomic::Ordering::Acquire);

    // Sell the joker
    let result = game.sell_joker(0);
    assert!(result.is_ok());

    // Verify joker is removed and money increased
    assert_eq!(game.jokers.len(), 0);
    assert!(game.money.load(std::sync::atomic::Ordering::Acquire) > initial_money);

    // Verify state is cleaned up
    assert!(game.joker_state_manager.get_state(JokerId::Joker).is_none());
}

#[test]
fn test_remove_joker_invalid_slot() {
    // Test error handling for invalid joker slot
    let mut game = Game::new(Config::default());

    // Try to remove joker from empty game
    let result = game.remove_joker(0);
    assert!(result.is_err());

    // Add one joker, try to remove slot 1 (out of bounds)
    game.jokers.push(Jokers::TheJoker(TheJoker {}));
    let result = game.remove_joker(1);
    assert!(result.is_err());
}

#[test]
fn test_validate_joker_state_consistency() {
    // Test that state validation catches inconsistencies
    let mut game = Game::new(Config::default());

    // Create state without corresponding joker
    game.joker_state_manager
        .set_state(JokerId::Joker, JokerState::default());

    // Validation should fail
    let result = game.validate_joker_state();
    assert!(result.is_err());

    // Add the joker to fix consistency
    game.jokers.push(Jokers::TheJoker(TheJoker {}));

    // Validation should now pass
    let result = game.validate_joker_state();
    assert!(result.is_ok());
}

#[test]
fn test_cleanup_joker_state() {
    // Test cleanup of orphaned state
    let mut game = Game::new(Config::default());

    // Add joker state without corresponding joker
    game.joker_state_manager
        .set_state(JokerId::Joker, JokerState::default());
    game.joker_state_manager
        .set_state(JokerId::LustyJoker, JokerState::default());

    // Add one actual joker (maps to JokerId::Joker)
    game.jokers.push(Jokers::TheJoker(TheJoker {}));

    // Cleanup should remove orphaned state
    game.cleanup_joker_state();

    // TheJoker state should remain (has corresponding joker)
    assert!(game.joker_state_manager.get_state(JokerId::Joker).is_some());

    // LustyJoker state should be removed (no corresponding joker)
    assert!(game
        .joker_state_manager
        .get_state(JokerId::LustyJoker)
        .is_none());
}

#[test]
fn test_reset_game_cleans_joker_state() {
    // Test that game reset properly cleans up all joker state
    let mut game = Game::new(Config::default());

    // Add jokers and state
    game.jokers.push(Jokers::TheJoker(TheJoker {}));
    game.jokers.push(Jokers::TheJoker(TheJoker {}));
    game.joker_state_manager
        .set_state(JokerId::Joker, JokerState::default());
    game.joker_state_manager
        .set_state(JokerId::GreedyJoker, JokerState::default());

    // Reset game
    game.reset_game();

    // All jokers and state should be cleared
    assert_eq!(game.jokers.len(), 0);
    assert!(game.joker_state_manager.get_state(JokerId::Joker).is_none());
    assert!(game
        .joker_state_manager
        .get_state(JokerId::GreedyJoker)
        .is_none());
}

#[test]
fn test_joker_state_integration_during_blind() {
    // Test that joker state persists and updates correctly during blind play
    let mut game = Game::new(Config::default());

    // Add a joker that accumulates value
    game.jokers.push(Jokers::TheJoker(TheJoker {}));

    // Initial state
    let initial_state = JokerState {
        accumulated_value: 10.0,
        triggers_remaining: Some(3),
        custom_data: std::collections::HashMap::new(),
    };
    game.joker_state_manager
        .set_state(JokerId::Joker, initial_state);

    // Simulate joker trigger that updates state
    game.joker_state_manager
        .update_state(JokerId::Joker, |state| {
            state.accumulated_value += 5.0;
            if let Some(ref mut triggers) = state.triggers_remaining {
                *triggers = triggers.saturating_sub(1);
            }
        });

    // Verify state was updated correctly
    let updated_state = game.joker_state_manager.get_state(JokerId::Joker).unwrap();
    assert_eq!(updated_state.accumulated_value, 15.0);
    assert_eq!(updated_state.triggers_remaining, Some(2));

    // State should persist across game operations
    game.round += 1;
    let persistent_state = game.joker_state_manager.get_state(JokerId::Joker).unwrap();
    assert_eq!(persistent_state.accumulated_value, 15.0);
}

#[test]
fn test_multiple_joker_state_management() {
    // Test managing state for multiple jokers simultaneously
    let mut game = Game::new(Config::default());

    // Add multiple jokers (using different IDs to test multiple state management)
    let joker_ids = vec![JokerId::Joker, JokerId::GreedyJoker, JokerId::LustyJoker];
    for &joker_id in &joker_ids {
        game.jokers.push(Jokers::TheJoker(TheJoker {}));
        game.joker_state_manager.set_state(
            joker_id,
            JokerState {
                accumulated_value: joker_id as u32 as f64 * 10.0,
                triggers_remaining: None,
                custom_data: std::collections::HashMap::new(),
            },
        );
    }

    // Verify all states exist and are correct
    for &joker_id in &joker_ids {
        let state = game.joker_state_manager.get_state(joker_id).unwrap();
        assert_eq!(state.accumulated_value, joker_id as u32 as f64 * 10.0);
    }

    // Remove middle joker
    game.jokers.remove(1); // Remove GreedyJoker
    game.joker_state_manager.remove_state(JokerId::GreedyJoker);

    // Verify correct states remain
    assert!(game.joker_state_manager.get_state(JokerId::Joker).is_some());
    assert!(game
        .joker_state_manager
        .get_state(JokerId::GreedyJoker)
        .is_none());
    assert!(game
        .joker_state_manager
        .get_state(JokerId::LustyJoker)
        .is_some());
}
