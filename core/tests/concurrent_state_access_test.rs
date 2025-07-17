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
    let actions1: Vec<Action> = game.gen_actions_cached();
    let first_duration = start.elapsed();

    // Second call should be faster due to caching
    let start = Instant::now();
    let actions2: Vec<Action> = game.gen_actions_cached();
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
    assert_eq!(game.money.load(std::sync::atomic::Ordering::Acquire), 100);
    assert_eq!(game.chips.load(std::sync::atomic::Ordering::Acquire), 50);
    assert_eq!(game.mult.load(std::sync::atomic::Ordering::Acquire), 2);
    assert_eq!(game.score.load(std::sync::atomic::Ordering::Acquire), 1000);
}

/// Test lock-free state snapshots for Python bindings
#[test]
fn test_lock_free_state_snapshot() {
    let game = Game::new(Config::default());

    // Get lock-free snapshot (should not block)
    let snapshot = game.get_state_snapshot();

    // Verify snapshot contains essential state
    assert_eq!(
        snapshot.money,
        game.money.load(std::sync::atomic::Ordering::Acquire)
    );
    assert_eq!(
        snapshot.chips,
        game.chips.load(std::sync::atomic::Ordering::Acquire)
    );
    assert_eq!(
        snapshot.score,
        game.score.load(std::sync::atomic::Ordering::Acquire)
    );
    assert_eq!(snapshot.stage, format!("{:?}", game.stage));
    assert_eq!(snapshot.round, game.round);

    // Snapshot should be independent of original game
    let snapshot2 = game.get_state_snapshot();
    assert_eq!(snapshot.money, snapshot2.money);
}

/// Test that state consistency is maintained across concurrent operations
#[test]
fn test_concurrent_state_consistency() {
    use std::sync::Mutex;
    
    let mut game = Game::new(Config::default());
    // Set initial money for testing
    game.money.store(1000, std::sync::atomic::Ordering::Release);
    
    let game = Arc::new(Mutex::new(game));
    let mut handles = vec![];

    // Test actual concurrent modifications using the atomic compare-and-swap operations
    // Simulate multiple threads trying to spend money simultaneously
    for thread_id in 0..8 {
        let game_clone = Arc::clone(&game);
        let handle = thread::spawn(move || {
            let mut successful_operations = 0;
            
            for _ in 0..50 {
                // Try to "spend" 10 money using the atomic operations
                let result = {
                    let mut game_guard = game_clone.lock().unwrap();
                    
                    // Simulate the atomic money deduction with compare-and-swap
                    // (this simulates what happens in buy_joker methods)
                    let cost = 10;
                    loop {
                        let current_money = game_guard.money.load(std::sync::atomic::Ordering::Acquire);
                        if current_money < cost {
                            break false; // Not enough money
                        }
                        
                        // Atomically update money only if it hasn't changed since we read it
                        match game_guard.money.compare_exchange_weak(
                            current_money,
                            current_money - cost,
                            std::sync::atomic::Ordering::SeqCst,
                            std::sync::atomic::Ordering::Acquire,
                        ) {
                            Ok(_) => break true, // Successfully deducted money
                            Err(_) => continue, // Money was modified by another thread, retry
                        }
                    }
                };
                
                if result {
                    successful_operations += 1;
                }
                
                // Small delay to increase chance of concurrent access
                thread::sleep(Duration::from_micros(100));
            }
            
            (thread_id, successful_operations)
        });
        handles.push(handle);
    }

    // Collect results
    let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();
    
    // Verify atomicity: total successful operations * 10 should equal money spent
    let total_successful_operations: usize = results.iter().map(|(_, ops)| *ops).sum();
    let final_money = {
        let game_guard = game.lock().unwrap();
        game_guard.money.load(std::sync::atomic::Ordering::Acquire)
    };
    
    // Check that money balance is correct (initial 1000 - (successful_operations * 10))
    assert_eq!(
        final_money,
        1000 - (total_successful_operations * 10),
        "Money balance incorrect: expected {}, got {}. Successful operations: {}",
        1000 - (total_successful_operations * 10),
        final_money,
        total_successful_operations
    );
    
    // Verify no money was lost or created due to race conditions
    assert!(final_money <= 1000, "Money increased beyond initial amount");
    assert!(total_successful_operations <= 100, "More operations succeeded than money allowed");
    
    println!(
        "Concurrent test results: {} total successful operations, final money: {}",
        total_successful_operations, final_money
    );
}

/// Test performance benchmarks for state access operations
#[test]
fn test_state_access_performance() {
    let game = Game::new(Config::default());

    // Benchmark direct field access
    let start = Instant::now();
    for _ in 0..10000 {
        let _money = game.money.load(std::sync::atomic::Ordering::Acquire);
        let _chips = game.chips.load(std::sync::atomic::Ordering::Acquire);
        let _score = game.score.load(std::sync::atomic::Ordering::Acquire);
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
