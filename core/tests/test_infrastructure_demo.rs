//! Demonstration of the new test infrastructure from PR #779 salvage
//!
//! This file shows how to use the production-ready test infrastructure
//! that was salvaged from PR #779 as part of issue #916 (Day 1).

// Import the test infrastructure
mod common;

use common::prelude::*;
use common::builders::*;
use common::performance::*;
use common::snapshot::*;
use common::environment::*;
use common::patterns::*;

use balatro_rs::{
    action::Action,
    stage::{Stage, Blind},
    joker::JokerId,
};
use std::time::Duration;

#[test]
fn test_basic_game_creation_with_builder() {
    // Use the builder pattern for complex game state setup
    let game = GameStateBuilder::new()
        .with_ante(2)
        .with_round(3)
        .with_money(50)
        .with_score(100, 10)
        .with_seed(42) // Deterministic testing
        .build();

    // Use domain assertions
    assert_game_running(&game);
    assert_ante_level(&game, 2);
    assert_round_number(&game, 3);
    assert_money_in_range(&game, 50, 50);
}

#[test]
fn test_deterministic_game_with_seed() {
    // Create two games with the same seed
    let game1 = create_test_game_with_seed(12345);
    let game2 = create_test_game_with_seed(12345);

    // They should have identical initial states
    assert_game_state_equals(&game1, &game2);

    // Verify deterministic action generation
    assert_actions_deterministic(12345, 5);
}

#[test]
fn test_test_environment_configuration() {
    // Create a performance testing environment
    let env = TestEnvironment::performance();
    assert!(env.performance_tracking);
    assert_eq!(env.timeout, Duration::from_secs(60));

    // Create a debug environment
    let debug_env = TestEnvironment::debug();
    assert!(debug_env.enable_logging);

    // Create game from environment
    let game = env.create_game();
    assert_game_running(&game);
}

#[test]
fn test_seed_manager_for_parameterized_tests() {
    let mut seed_manager = SeedManager::new(1000);

    // Get unique seeds for different tests
    let seed1 = seed_manager.get_seed("test_joker_effects");
    let seed2 = seed_manager.get_seed("test_scoring");
    assert_ne!(seed1, seed2);

    // Get the same seed when requested again
    let seed1_again = seed_manager.get_seed("test_joker_effects");
    assert_eq!(seed1, seed1_again);

    // Get seed sequence for parameterized testing
    let seeds = seed_manager.get_seed_sequence("test_hands", 5);
    assert_eq!(seeds.len(), 5);
}

#[test]
fn test_game_state_snapshot_for_regression() {
    let game = GameStateBuilder::new()
        .with_ante(3)
        .with_money(75)
        .with_seed(999)
        .build();

    // Take a snapshot of the game state
    let snapshot = GameStateSnapshot::from(&game);

    // Verify exact match
    assert_game_state_snapshot(&game, &snapshot, None);

    // Test with tolerance for money variations
    let tolerance = StateTolerance {
        money_tolerance: 10,
        score_tolerance: 100,
        strict_stage: false,
    };
    assert_game_state_snapshot(&game, &snapshot, Some(tolerance));
}

#[test]
fn test_performance_monitoring() {
    let mut monitor = PerformanceMonitor::new("game_creation_perf");

    // Measure game creation performance
    monitor.measure("create_simple_game", || {
        let _game = create_test_game();
    });

    monitor.measure("create_complex_game", || {
        let _game = GameStateBuilder::new()
            .with_ante(5)
            .with_jokers(vec![JokerId::Joker; 3])
            .build();
    });

    // Check that operations complete within time limits
    assert_action_completes_within(
        || {
            let _game = create_test_game();
        },
        Duration::from_millis(100),
        "Game creation"
    );
}

#[test]
fn test_test_fixture_with_setup_teardown() {
    let mut fixture = TestFixture::new(|| {
        GameStateBuilder::new()
            .with_money(100)
            .build()
    })
    .with_teardown(|game| {
        // Cleanup code - reset state
        game.money = 0;
    });

    // Setup the fixture
    let game = fixture.setup_mut();
    assert_eq!(game.money, 100);

    // Teardown happens automatically when fixture is dropped
}

#[test]
fn test_parameterized_test_pattern() {
    // Define test parameters
    let ante_levels = vec![1, 2, 3, 4, 5, 6, 7, 8];

    // Run parameterized test
    let results = run_parameterized_test(
        "test_valid_ante_levels",
        ante_levels,
        |&ante| {
            if ante >= 1 && ante <= 8 {
                Ok(())
            } else {
                Err(format!("Invalid ante level: {}", ante))
            }
        }
    );

    // All should pass
    assert!(results.iter().all(|r| r.is_ok()));
}

#[test]
fn test_game_state_recorder_for_debugging() {
    let mut recorder = GameStateRecorder::new(10);
    let mut game = create_test_game();

    // Record initial state
    recorder.record(&game, 0, None);

    // Simulate some actions
    let actions = vec![
        Action::SelectBlind(Blind::Small),
        Action::Play,
    ];

    for (i, action) in actions.iter().enumerate() {
        if game.gen_actions().any(|a| &a == action) {
            game.handle_action(action.clone()).ok();
            recorder.record(&game, i + 1, Some(action.clone()));
        }
    }

    // Find specific state
    let state = recorder.find_state(|s| s.step == 1);
    assert!(state.is_some());
}

#[test]
fn test_business_rule_validations() {
    let game = create_test_game();

    // Validate business rules
    assert_money_never_negative(&game);
    assert_ante_progression_valid(1, 2);
    assert_round_progression_valid(3, 1, true); // Round resets on ante change

    // Validate state transitions
    assert_valid_state_transition(&Stage::PreBlind, &Stage::Blind(Blind::Small));
    assert_valid_state_transition(&Stage::Shop, &Stage::PreBlind);
}

#[test]
fn test_edge_case_scenarios() {
    let scenarios = create_edge_case_scenarios();

    for scenario in scenarios {
        println!("Testing scenario: {}", scenario.name);

        let game = GameStateBuilder::new()
            .with_ante(scenario.ante)
            .with_round(scenario.round)
            .with_money(scenario.money)
            .with_score(scenario.chips, scenario.mult)
            .build();

        // Ensure edge cases don't break invariants
        if scenario.money >= 0 {
            assert_money_never_negative(&game);
        }
    }
}

#[test]
fn test_deck_builder_pattern() {
    use balatro_rs::card::{Card, Suit, Value};

    // Build a custom deck
    let deck = DeckBuilder::new()
        .with_standard_deck()
        .shuffled(Some(42)) // Deterministic shuffle
        .build();

    assert_eq!(deck.len(), 52);

    // Build a deck with specific cards
    let aces_deck = DeckBuilder::new()
        .with_card_copies(Card::new(Value::Ace, Suit::Spade), 4)
        .with_card_copies(Card::new(Value::Ace, Suit::Heart), 4)
        .build();

    assert_eq!(aces_deck.len(), 8);
}

#[test]
fn test_test_validator_for_invariants() {
    let validator = TestValidator::standard();
    let game = create_test_game();

    // Validate standard invariants
    let result = validator.validate(&game);
    assert!(result.is_ok());

    // Create custom validator
    let custom_validator = TestValidator::new()
        .add_invariant(|game| {
            if game.ante > 5 {
                Err("Ante too high for this test".to_string())
            } else {
                Ok(())
            }
        });

    let result = custom_validator.validate(&game);
    assert!(result.is_ok());
}

#[test]
fn test_concurrent_test_fixtures() {
    let fixtures = create_concurrent_test_fixtures(5);

    assert_eq!(fixtures.len(), 5);

    // Each game should be independent but deterministic
    for (i, game) in fixtures.iter().enumerate() {
        assert!(!game.is_over());
        // Each game has a unique seed based on index
        println!("Game {} ante: {}", i, game.ante);
    }
}

#[test]
fn test_memory_test_fixtures() {
    let fixtures = create_memory_test_fixtures();

    // Small set for quick tests
    assert_eq!(fixtures.small_games.len(), 10);

    // Large set for stress testing
    assert_eq!(fixtures.large_games.len(), 100);

    // Stress set for memory leak detection
    assert_eq!(fixtures.stress_games.len(), 1000);

    // All games should be valid
    for game in &fixtures.small_games {
        assert!(!game.is_over());
    }
}
