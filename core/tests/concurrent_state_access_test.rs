//! Test Suite for Efficient State Access Patterns
//!
//! Tests concurrent-safe data structures and optimized access patterns
//! for AI/RL applications as specified in issue #169.

use balatro_rs::action::Action;
use balatro_rs::concurrent_state::StateUpdate;
use balatro_rs::config::Config;
use balatro_rs::game::Game;
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

/// Test concurrent read access to game state from multiple threads
#[test]
fn test_concurrent_state_reads() {
    let game = Arc::new(Game::new(Config::default()));
    let mut handles = vec![];

    // Spawn 8 threads that concurrently read game state
    for thread_id in 0..8 {
        let game_clone = Arc::clone(&game);
        let handle = thread::spawn(move || {
            let mut read_count = 0;
            let start = Instant::now();

            // Perform 1000 concurrent state reads per thread
            while start.elapsed() < Duration::from_millis(100) {
                // Test concurrent access to frequently read state
                let _money = game_clone.get_money_concurrent();
                let _chips = game_clone.get_chips_concurrent();
                let _score = game_clone.get_score_concurrent();
                let _stage = game_clone.get_stage_concurrent();

                read_count += 1;
            }

            (thread_id, read_count)
        });
        handles.push(handle);
    }

    // Collect results - should complete without deadlocks
    let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();

    // Verify all threads completed successfully
    assert_eq!(results.len(), 8);
    for (thread_id, read_count) in results {
        assert!(
            read_count > 100,
            "Thread {} only performed {} reads",
            thread_id,
            read_count
        );
    }
}

/// Test cached action generation for repeated state queries
#[test]
fn test_cached_action_generation() {
    let mut game = Game::new(Config::default());

    // Enable action caching
    game.enable_action_caching(Duration::from_millis(50));

    // First call should cache the result
    let start = Instant::now();
    let actions1: Vec<Action> = game.gen_actions_cached().collect();
    let first_duration = start.elapsed();

    // Second call should be faster due to caching
    let start = Instant::now();
    let actions2: Vec<Action> = game.gen_actions_cached().collect();
    let second_duration = start.elapsed();

    // Verify results are identical
    assert_eq!(actions1, actions2);

    // Second call should be significantly faster (cache hit)
    assert!(
        second_duration < first_duration / 2,
        "Cached call took {}μs, original took {}μs",
        second_duration.as_micros(),
        first_duration.as_micros()
    );
}

/// Test batch state updates for high-throughput scenarios
#[test]
fn test_batch_state_updates() {
    let mut game = Game::new(Config::default());

    // Prepare batch updates
    let updates = vec![
        StateUpdate::Money(100),
        StateUpdate::Chips(50),
        StateUpdate::Mult(2),
        StateUpdate::Score(1000),
    ];

    // Apply batch updates atomically
    let result = game.apply_batch_updates(updates);
    assert!(result.is_ok(), "Batch updates failed: {:?}", result);

    // Verify all updates were applied
    assert_eq!(game.money, 100);
    assert_eq!(game.chips, 50);
    assert_eq!(game.mult, 2);
    assert_eq!(game.score, 1000);
}

/// Test lock-free state snapshots for Python bindings
#[test]
fn test_lock_free_state_snapshot() {
    let game = Game::new(Config::default());

    // Get lock-free snapshot (should not block)
    let snapshot = game.get_state_snapshot();

    // Verify snapshot contains essential state
    assert_eq!(snapshot.money, game.money);
    assert_eq!(snapshot.chips, game.chips);
    assert_eq!(snapshot.score, game.score);
    assert_eq!(snapshot.stage, format!("{:?}", game.stage));
    assert_eq!(snapshot.round, game.round);

    // Snapshot should be independent of original game
    let snapshot2 = game.get_state_snapshot();
    assert_eq!(snapshot.money, snapshot2.money);
}

/// Test that state consistency is maintained across concurrent operations
#[test]
fn test_concurrent_state_consistency() {
    let game = Arc::new(Game::new(Config::default()));
    let mut handles = vec![];

    // Spawn readers and writers concurrently
    for i in 0..4 {
        let game_clone = Arc::clone(&game);
        let handle = thread::spawn(move || {
            for _ in 0..100 {
                // Mix reads and writes
                if i % 2 == 0 {
                    // Reader thread
                    let _snapshot = game_clone.get_state_snapshot();
                    let _money = game_clone.get_money_concurrent();
                } else {
                    // Writer thread would require &mut access
                    // For now just test concurrent reads
                    let _snapshot = game_clone.get_state_snapshot();
                }

                thread::sleep(Duration::from_millis(1));
            }
        });
        handles.push(handle);
    }

    // Wait for all operations to complete
    for handle in handles {
        handle.join().unwrap();
    }

    // Game should remain in valid state
    assert!(game.validate_state_consistency());
}

/// Test performance benchmarks for state access operations
#[test]
fn test_state_access_performance() {
    let game = Game::new(Config::default());

    // Benchmark direct field access
    let start = Instant::now();
    for _ in 0..10000 {
        let _money = game.money;
        let _chips = game.chips;
        let _score = game.score;
    }
    let direct_duration = start.elapsed();

    // Benchmark concurrent-safe access
    let start = Instant::now();
    for _ in 0..10000 {
        let _money = game.get_money_concurrent();
        let _chips = game.get_chips_concurrent();
        let _score = game.get_score_concurrent();
    }
    let concurrent_duration = start.elapsed();

    // Concurrent access should be reasonably fast (< 10x slower than direct)
    assert!(
        concurrent_duration < direct_duration * 10,
        "Concurrent access too slow: {}μs vs {}μs",
        concurrent_duration.as_micros(),
        direct_duration.as_micros()
    );

    println!(
        "Direct access: {}μs, Concurrent access: {}μs",
        direct_duration.as_micros(),
        concurrent_duration.as_micros()
    );
}
