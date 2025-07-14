#![cfg(feature = "python")]

use balatro_rs::{action::Action, card::*, game::Game, stage::Stage};

#[test]
fn test_pyclass_types_exist() {
    // This test ensures that PyO3 derive macros work correctly
    // The test will fail to compile if the pyclass attributes are broken

    // Test that Card can be created (has pyclass)
    let card = Card::new(Value::Ace, Suit::Heart);
    assert_eq!(card.value, Value::Ace);
    assert_eq!(card.suit, Suit::Heart);

    // Test that Action enum works with PyO3
    let action = Action::Play();
    match action {
        Action::Play() => assert!(true),
        _ => assert!(false),
    }

    // Test that Stage enum works
    let stage = Stage::PreBlind();
    assert!(matches!(stage, Stage::PreBlind()));
}

#[test]
fn test_game_with_python_feature() {
    // Test that Game works correctly with Python feature enabled
    let mut game = Game::default();
    game.start();

    // Verify game started correctly
    assert!(!game.is_over());
    assert!(matches!(game.stage, Stage::PreBlind()));

    // Test that we can generate actions
    let actions: Vec<Action> = game.gen_actions().collect();
    assert!(!actions.is_empty());
}

#[test]
fn test_pymethods_compilation() {
    // This test verifies that pymethods compile correctly
    // It doesn't test Python integration directly but ensures
    // the Rust side compiles with all PyO3 attributes

    let mut game = Game::default();
    game.start();

    // Test action space generation (used by Python)
    let space = game.gen_action_space();
    let vec = space.to_vec();
    assert!(!vec.is_empty());

    // Test that we can convert indices back to actions
    for (i, &valid) in vec.iter().enumerate() {
        if valid == 1 {
            let action = space.to_action(i, &game);
            assert!(action.is_ok());
            break;
        }
    }
}
