use balatro_rs::action::Action;
use balatro_rs::bounded_action_history::BoundedActionHistory;
use balatro_rs::config::Config;
/// Tests for memory leak detection and performance validation
///
/// These tests validate the memory optimizations implemented for issue #280
/// to ensure they prevent memory leaks in long-running training scenarios.
use balatro_rs::game::Game;
use balatro_rs::memory_monitor::MemoryConfig;
use std::time::Instant;

#[test]
fn test_bounded_action_history_memory_limit() {
    let mut history = BoundedActionHistory::with_capacity(100);

    // Add more actions than the limit
    for _i in 0..200 {
        history.push(Action::Play());
    }

    // Should not exceed the limit
    assert_eq!(history.len(), 100);
    assert_eq!(history.total_actions(), 200);

    // Memory usage should be bounded
    let stats = history.memory_stats();
    assert!(stats.estimated_bytes < 10000); // Should be reasonable
}

#[test]
fn test_action_history_doesnt_grow_unbounded() {
    let mut game = Game::new(Config::default());
    game.enable_rl_memory_monitoring(); // Use RL config with smaller limits

    // Simulate many actions
    for _ in 0..10000 {
        game.action_history.push(Action::Play());
    }

    // Action history should be bounded
    assert!(game.action_history.len() <= 5000); // RL config limit
    assert_eq!(game.action_history.total_actions(), 10000);
}

#[test]
fn test_memory_monitoring_configuration() {
    let mut game = Game::new(Config::default());

    // Test RL memory monitoring
    game.enable_rl_memory_monitoring();
    let config = game.memory_monitor.config();
    assert!(config.enable_monitoring);
    assert_eq!(config.max_action_history, 5000);
    assert!(config.warning_threshold_mb < 1024);

    // Test simulation memory monitoring
    game.enable_simulation_memory_monitoring();
    let config = game.memory_monitor.config();
    assert!(config.enable_monitoring);
    assert_eq!(config.max_action_history, 1000);
    assert!(config.warning_threshold_mb < 512);
}

#[test]
fn test_memory_stats_collection() {
    let mut game = Game::new(Config::default());
    game.enable_rl_memory_monitoring();

    // Generate some game state
    game.start();
    for _ in 0..100 {
        game.action_history.push(Action::Play());
    }

    // Get memory stats
    let stats = game.get_memory_stats();
    assert!(stats.is_some());

    let stats = stats.unwrap();
    assert!(stats.estimated_usage_bytes > 0);
    assert!(stats.total_actions >= 100);
    assert_eq!(stats.active_snapshots, 1);
}

#[test]
fn test_memory_safety_check() {
    let mut game = Game::new(Config::default());

    // With normal usage, should be safe
    assert!(game.check_memory_safety());

    // Enable strict monitoring
    let strict_config = MemoryConfig {
        enable_monitoring: true,
        warning_threshold_mb: 1, // Very low threshold
        critical_threshold_mb: 2,
        max_memory_mb: 4,
        ..MemoryConfig::default()
    };
    game.memory_monitor.update_config(strict_config);

    // Should still be safe for small usage
    assert!(game.check_memory_safety());
}

#[test]
fn test_memory_usage_estimation() {
    let mut game = Game::new(Config::default());

    // Enable monitoring to get memory stats
    game.enable_rl_memory_monitoring();

    // Get initial memory stats
    let initial_stats = game.get_memory_stats();
    assert!(initial_stats.is_some());

    // Add some data
    game.start();
    for _ in 0..1000 {
        game.action_history.push(Action::Play());
    }

    // Check memory safety and get updated stats
    assert!(game.check_memory_safety());
    let updated_stats = game.get_memory_stats();
    assert!(updated_stats.is_some());
}

#[test]
fn test_long_running_simulation_memory_stability() {
    let mut game = Game::new(Config::default());

    // Configure memory monitoring with a very short interval for testing
    let mut memory_config = MemoryConfig::for_simulation();
    memory_config.monitoring_interval_ms = 1; // Check every 1ms for testing
    game.memory_monitor.update_config(memory_config.clone());
    game.action_history.resize(memory_config.max_action_history);

    let start_time = Instant::now();
    let mut memory_measurements = Vec::new();

    // Simulate a long-running training session
    for i in 0..1000 {
        // Simulate some actions
        game.action_history.push(Action::Play());
        game.action_history.push(Action::Discard());

        // Collect memory stats every 100 iterations
        if i % 100 == 0 {
            // Small delay to ensure monitoring interval passes
            std::thread::sleep(std::time::Duration::from_millis(2));
            if let Some(stats) = game.get_memory_stats() {
                memory_measurements.push(stats.estimated_usage_bytes);
            }
        }
    }

    let elapsed = start_time.elapsed();
    println!("Long-running simulation took: {elapsed:?}");
    println!("Memory measurements: {memory_measurements:?}");

    // Memory should be stable (not growing unbounded)
    if memory_measurements.len() >= 2 {
        let first = memory_measurements[0];
        let last = *memory_measurements.last().unwrap();

        // Memory should not grow excessively (allow some variation)
        // For bytes, we allow growth up to 20x the initial size as action history fills up, then stabilizes
        assert!(
            last < first * 20,
            "Memory grew too much: {first} bytes -> {last} bytes"
        );

        // Most importantly, memory should stabilize (not continue growing unbounded)
        // Check that the last few measurements are similar (within 10% variation)
        if memory_measurements.len() >= 4 {
            let recent_measurements = &memory_measurements[memory_measurements.len() - 4..];
            let recent_max = *recent_measurements.iter().max().unwrap();
            let recent_min = *recent_measurements.iter().min().unwrap();
            let variation = (recent_max - recent_min) as f64 / recent_min as f64;
            assert!(
                variation < 0.1,
                "Memory should stabilize but is still varying: recent measurements {:?}, variation: {:.2}%",
                recent_measurements,
                variation * 100.0
            );
        }
    }

    // Final memory usage should be reasonable (less than 10MB in bytes)
    let final_stats = game.get_memory_stats().unwrap();
    assert!(
        final_stats.estimated_usage_bytes < 10 * 1024 * 1024,
        "Final memory usage too high: {} bytes",
        final_stats.estimated_usage_bytes
    );
}

#[test]
fn test_game_state_snapshot_memory_efficiency() {
    use balatro_rs::action::Action;

    let mut game = Game::new(Config::default());
    game.start();

    // Add some state
    for _ in 0..1000 {
        game.action_history.push(Action::Play());
    }

    // Create multiple snapshots (simulating frequent state access)
    let start_time = Instant::now();
    let mut snapshots = Vec::new();

    for _ in 0..100 {
        // This would previously cause expensive cloning
        let snapshot = format!("{:?}", game.stage);
        snapshots.push(snapshot);
    }

    let elapsed = start_time.elapsed();
    println!("Creating 100 snapshots took: {elapsed:?}");

    // Should be fast (under 10ms for basic operations)
    assert!(
        elapsed.as_millis() < 100,
        "Snapshot creation too slow: {elapsed:?}"
    );
}

#[test]
fn test_memory_report_generation() {
    let mut game = Game::new(Config::default());
    game.enable_rl_memory_monitoring();

    // Generate some state
    game.start();
    for _ in 0..100 {
        game.action_history.push(Action::Play());
    }

    // Get memory stats first
    let _ = game.get_memory_stats();

    // Generate report
    let report = game.generate_memory_report();
    assert!(!report.is_empty());
    assert!(report.contains("Memory Usage Report"));
    assert!(report.contains("Estimated Usage"));

    println!("Memory Report:\n{report}");
}

#[cfg(test)]
mod performance_benchmarks {
    use super::*;
    use std::time::Instant;

    #[test]
    fn benchmark_action_history_vs_vec() {
        const ITERATIONS: usize = 10000;

        // Test BoundedActionHistory
        let start = Instant::now();
        let mut bounded_history = BoundedActionHistory::with_capacity(1000);
        for _ in 0..ITERATIONS {
            bounded_history.push(Action::Play());
        }
        let bounded_time = start.elapsed();

        // Test regular Vec
        let start = Instant::now();
        let mut vec_history = Vec::new();
        for _ in 0..ITERATIONS {
            vec_history.push(Action::Play());
            if vec_history.len() > 1000 {
                vec_history.remove(0); // Simulate bounded behavior
            }
        }
        let vec_time = start.elapsed();

        println!("BoundedActionHistory: {bounded_time:?}");
        println!("Vec with manual bounds: {vec_time:?}");

        // BoundedActionHistory should be competitive or better
        // (Vec::remove(0) is O(n), our circular buffer is O(1))
        assert!(bounded_time < vec_time * 2); // Allow some overhead
    }

    #[test]
    fn benchmark_memory_monitoring_overhead() {
        const ITERATIONS: usize = 1000;

        let mut game = Game::new(Config::default());

        // Test without monitoring
        let start = Instant::now();
        for _ in 0..ITERATIONS {
            game.action_history.push(Action::Play());
        }
        let no_monitoring_time = start.elapsed();

        // Test with monitoring enabled
        game.enable_rl_memory_monitoring();
        let start = Instant::now();
        for _ in 0..ITERATIONS {
            game.action_history.push(Action::Play());
            if game.memory_monitor.should_check() {
                let _ = game.get_memory_stats();
            }
        }
        let with_monitoring_time = start.elapsed();

        println!("Without monitoring: {no_monitoring_time:?}");
        println!("With monitoring: {with_monitoring_time:?}");

        // Monitoring overhead should be reasonable for various environments
        // In high-load environments, we allow significant overhead since the
        // test's purpose is to verify functionality, not precise performance
        let max_overhead = if std::env::var("CI").is_ok() { 50 } else { 40 };
        assert!(
            with_monitoring_time < no_monitoring_time * max_overhead,
            "Monitoring overhead too high: {}x (limit: {}x)",
            with_monitoring_time.as_nanos() / no_monitoring_time.as_nanos().max(1),
            max_overhead
        );
    }
}
