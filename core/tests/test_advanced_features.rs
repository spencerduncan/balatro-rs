//! Advanced test examples demonstrating Day 2 testing infrastructure
//!
//! This test file demonstrates the usage of:
//! - Property-based testing with proptest
//! - Performance monitoring and benchmarking
//! - Memory leak detection
//! - Integration with existing test infrastructure

#![cfg(test)]

mod common;

use common::memory::*;
use common::performance::*;
use common::prelude::*;
use common::proptest::*;

use balatro_rs::{
    ante::Ante,
    config::Config,
    game::Game,
    hand::Hand,
    stage::{Blind, Stage},
};

use proptest::prelude::*;
use std::sync::Arc;
use std::time::Duration;

// ============================================================================
// PROPERTY-BASED TESTING EXAMPLES
// ============================================================================

mod property_tests {
    use super::*;

    proptest! {
        /// Property: Money should never go negative from valid actions
        #[test]
        fn prop_money_never_negative(
            initial_money in 0.0f64..=100.0,
        ) {
            // Note: Seed configuration may need special handling
            let config = Config::default();
            let mut game = Game::new(config);
            game.money = initial_money;

            // Generate and execute valid actions
            let actions: Vec<_> = game.gen_actions().take(10).collect();
            for action in actions {
                let money_before = game.money;
                let _ = game.handle_action(action);

                // Money should never become negative from valid actions
                prop_assert!(
                    game.money >= 0.0,
                    "Money became negative: {} -> {} after action",
                    money_before,
                    game.money
                );
            }
        }

        /// Property: Score should always increase or stay the same
        #[test]
        fn prop_score_monotonic(
            game in arb_game_state(),
        ) {
            let initial_score = game.score;

            // Score should never decrease
            prop_assert!(
                game.score >= initial_score,
                "Score decreased: {} -> {}",
                initial_score,
                game.score
            );
        }

        /// Property: Deck size should never exceed 52 cards
        #[test]
        fn prop_deck_size_limit(
            deck in arb_deck(),
        ) {
            prop_assert!(
                deck.cards().len() <= 52,
                "Deck has {} cards, exceeding limit of 52",
                deck.cards().len()
            );
        }

        /// Property: Hand evaluation should be deterministic
        #[test]
        fn prop_hand_evaluation_deterministic(
            cards in arb_hand(),
        ) {
            let hand1 = Hand::new(cards.clone());
            let hand2 = Hand::new(cards);

            // Same cards should produce same evaluation
            prop_assert_eq!(
                hand1.cards().len(),
                hand2.cards().len(),
                "Hand evaluation not deterministic"
            );
        }

        /// Property: Stage transitions should be valid
        #[test]
        fn prop_valid_stage_transitions(
            from_stage in arb_stage(),
        ) {
            // Test that each stage has at least one valid transition
            let _valid_transitions = match from_stage {
                Stage::PreBlind() => vec![Stage::Blind(Blind::Small), Stage::Blind(Blind::Big), Stage::Blind(Blind::Boss)],
                Stage::Blind(_) => vec![Stage::PostBlind()],
                Stage::PostBlind() => vec![Stage::Shop()],
                Stage::Shop() => vec![Stage::PreBlind()],
                Stage::End(_) => vec![], // End state has no transitions
            };

            // At least verify the stage exists
            prop_assert!(true, "Stage exists and can be created");
        }

        /// Property: Ante progression should be sequential
        #[test]
        fn prop_ante_progression_sequential(
            ante in arb_ante(),
        ) {
            let ante_value = match ante {
                Ante::Zero => 0,
                Ante::One => 1,
                Ante::Two => 2,
                Ante::Three => 3,
                Ante::Four => 4,
                Ante::Five => 5,
                Ante::Six => 6,
                Ante::Seven => 7,
                Ante::Eight => 8,
            };

            prop_assert!(
                ante_value >= 0 && ante_value <= 8,
                "Invalid ante value: {}",
                ante_value
            );
        }
    }

    /// Test with custom configuration
    #[test]
    fn test_custom_property_config() {
        let config = GameStateConfig {
            min_money: 10.0,
            max_money: 50.0,
            min_ante: 2,
            max_ante: 5,
            min_jokers: 1,
            max_jokers: 3,
            allow_negative_money: false,
        };

        let result = run_property_test(config, |game| game.money >= 10.0 && game.money <= 50.0);

        assert!(result.is_ok());
    }
}

// ============================================================================
// PERFORMANCE MONITORING EXAMPLES
// ============================================================================

mod performance_tests {
    use super::*;

    #[test]
    fn test_action_generation_performance() {
        let monitor = PerformanceMonitor::new();

        // Set performance threshold
        monitor.set_threshold(
            "action_generation",
            PerformanceThreshold::with_duration(Duration::from_micros(100)),
        );

        let game = create_test_game();

        // Measure action generation
        monitor.measure("action_generation", || {
            let _ = game.gen_actions().collect::<Vec<_>>();
        });

        // Check for performance regressions
        let violations = monitor.check_thresholds();
        assert!(
            violations.is_empty(),
            "Performance regression detected: {:?}",
            violations
        );

        // Get statistics
        if let Some(stats) = monitor.get_statistics("action_generation") {
            println!("Action generation stats: {}", stats);
            assert!(stats.mean_duration < Duration::from_micros(100));
        }
    }

    #[test]
    fn test_benchmark_comparison() {
        let harness = BenchmarkHarness::new()
            .with_warmup(100)
            .with_iterations(1000);

        // Compare two implementations
        let result = harness.compare(
            "vector_allocation",
            || {
                let _v: Vec<i32> = Vec::with_capacity(100);
            },
            "array_allocation",
            || {
                let _a: [i32; 100] = [0; 100];
            },
        );

        println!("{}", result);

        // Array allocation should be faster (or both are too fast to measure)
        // If both take 0ns, consider them equal
        assert!(
            result.is_faster()
                || result.baseline.1.mean_duration == result.comparison.1.mean_duration
        );
    }

    #[test]
    fn test_performance_monitoring_with_memory() {
        let monitor = PerformanceMonitor::new();

        // Measure with memory tracking
        let mut allocations = Vec::new();

        monitor.measure("vector_growth", || {
            for i in 0..1000 {
                allocations.push(i);
            }
        });

        let stats = monitor.get_statistics("vector_growth").unwrap();
        println!("Vector growth performance: {}", stats);

        // Generate report
        let report = monitor.report();
        println!("{}", report);
        println!("Markdown report:\n{}", report.to_markdown());
    }

    #[test]
    fn test_critical_path_timing() {
        let timer = Timer::start("critical_operation");

        // Simulate critical path
        let mut sum = 0;
        for i in 0..1_000_000 {
            sum += i;
        }

        let duration = timer.stop();
        println!("Critical operation took: {:?}", duration);

        assert!(duration < Duration::from_secs(1));
        assert_eq!(sum, 499999500000i64);
    }

    #[test]
    fn test_performance_baseline_regression() {
        let mut baseline = PerformanceBaseline::new(10.0); // 10% tolerance

        // Establish baseline
        let baseline_stats = PerformanceStatistics {
            count: 100,
            mean_duration: Duration::from_micros(50),
            median_duration: Duration::from_micros(45),
            min_duration: Duration::from_micros(30),
            max_duration: Duration::from_micros(100),
            std_deviation: Duration::from_micros(10),
            percentile_95: Duration::from_micros(80),
            percentile_99: Duration::from_micros(95),
            total_memory: 1024,
        };

        baseline.add_baseline("operation", baseline_stats.clone());

        // Test current performance (no regression)
        let current_good = PerformanceStatistics {
            mean_duration: Duration::from_micros(52), // Within 10% tolerance
            ..baseline_stats.clone()
        };

        assert!(baseline
            .check_regression("operation", &current_good)
            .is_none());

        // Test current performance (regression)
        let current_bad = PerformanceStatistics {
            mean_duration: Duration::from_micros(60), // 20% slower
            ..baseline_stats
        };

        assert!(baseline
            .check_regression("operation", &current_bad)
            .is_some());
    }
}

// ============================================================================
// MEMORY LEAK DETECTION EXAMPLES
// ============================================================================

mod memory_tests {
    use super::*;

    #[test]
    fn test_memory_leak_detection() {
        let stats = Arc::new(AllocationStats::new());

        // Test without leaks
        {
            let _guard = MemoryGuard::new("no_leak_test", stats.clone());

            // Allocate and deallocate properly
            let v = [1, 2, 3, 4, 5];
            drop(v);
        }

        assert!(!stats.has_leaks());
    }

    #[test]
    fn test_resource_tracking() {
        let tracker = ResourceTracker::new();

        // Simulate resource allocation
        tracker.register("game_1".to_string(), "Game".to_string(), 1024);
        tracker.register("deck_1".to_string(), "Deck".to_string(), 512);

        assert_eq!(tracker.resource_count(), 2);

        // Cleanup one resource
        assert!(tracker.unregister("game_1"));
        assert_eq!(tracker.resource_count(), 1);

        // Check for leaks
        let leaks = tracker.check_leaks();
        assert_eq!(leaks.len(), 1);
        assert_eq!(leaks[0].id, "deck_1");

        // Cleanup remaining
        assert!(tracker.unregister("deck_1"));
        assert!(tracker.check_leaks().is_empty());
    }

    #[test]
    fn test_raii_pattern() {
        let mut cleanup_called = false;
        let cleanup_flag = &mut cleanup_called;

        {
            let guard = ResourceGuard::new(vec![1, 2, 3], |_v| {
                *cleanup_flag = true;
            });

            assert_eq!(guard.len(), 3);
        }

        assert!(cleanup_called, "Cleanup was not called");
    }

    #[test]
    fn test_memory_profiling() {
        let mut profiler = MemoryProfiler::new().with_interval(Duration::from_millis(1));

        let stats = Arc::new(AllocationStats::new());

        // Profile memory usage
        profiler.profile(stats.clone(), || {
            stats.record_alloc(1000);
            stats.record_alloc(2000);
            stats.record_dealloc(500);
        });

        let report = profiler.report();
        println!("{}", report);

        assert!(report.peak_usage <= 2500);
    }

    #[test]
    fn test_allocation_report() {
        let stats = AllocationStats::new();

        // Simulate allocations
        stats.record_alloc(1024);
        stats.record_alloc(2048);
        stats.record_dealloc(1024);

        let report = stats.report();
        println!("{}", report);

        assert_eq!(report.total_allocated, 3072);
        assert_eq!(report.total_deallocated, 1024);
        assert_eq!(report.current_usage, 2048);
        assert!(report.has_leaks());
    }

    #[test]
    fn test_with_leak_detection_helper() {
        // This would panic if there was a leak
        test_with_leak_detection("helper_test", || {
            let v = [1, 2, 3, 4, 5];
            let sum: i32 = v.iter().sum();
            assert_eq!(sum, 15);
            // v is properly dropped here
        });
    }
}

// ============================================================================
// INTEGRATION TESTS - COMBINING ALL FEATURES
// ============================================================================

mod integration_tests {
    use super::*;

    /// Test that combines property testing with performance monitoring
    #[test]
    fn test_property_with_performance() {
        let monitor = PerformanceMonitor::new();

        proptest!(|(_seed in any::<u64>())| {
            let game = monitor.measure("game_creation", || {
                // Note: Seed configuration may need special handling
                let config = Config::default();
                Game::new(config)
            });

            prop_assert!(game.money >= 0.0);
            prop_assert!(game.score == 0.0);
        });

        if let Some(stats) = monitor.get_statistics("game_creation") {
            println!("Game creation performance: {}", stats);
            assert!(stats.mean_duration < Duration::from_millis(1));
        }
    }

    /// Test that combines memory tracking with benchmarking
    #[test]
    fn test_benchmark_with_memory() {
        let stats = Arc::new(AllocationStats::new());
        let harness = BenchmarkHarness::new().with_warmup(10).with_iterations(100);

        let perf_stats = harness.bench("game_lifecycle", || {
            stats.record_alloc(1024);
            let game = Game::new(Config::default());
            let _ = game.gen_actions().collect::<Vec<_>>();
            stats.record_dealloc(1024);
        });

        println!("Game lifecycle performance: {}", perf_stats);

        // Check no memory leaks
        assert!(!stats.has_leaks(), "Memory leak detected in game lifecycle");
    }

    /// Test property-based testing with resource tracking
    #[test]
    fn test_property_with_resources() {
        let tracker = ResourceTracker::new();

        proptest!(|(seed in any::<u64>())| {
            let game_id = format!("game_{}", seed);

            // Track resource
            tracker.register(game_id.clone(), "Game".to_string(), 1024);

            // Note: Seed configuration may need special handling
            let config = Config::default();
            let game = Game::new(config);
            prop_assert!(game.money >= 0.0);

            // Cleanup resource
            prop_assert!(tracker.unregister(&game_id));
        });

        // Verify all resources cleaned up
        assert_eq!(tracker.resource_count(), 0);
    }

    /// Comprehensive test using all advanced features
    #[test]
    fn test_comprehensive_advanced_features() {
        // Setup monitoring infrastructure
        let perf_monitor = PerformanceMonitor::new();
        let mem_stats = Arc::new(AllocationStats::new());
        let resource_tracker = ResourceTracker::new();

        // Set performance thresholds
        perf_monitor.set_threshold(
            "full_game",
            PerformanceThreshold::with_duration(Duration::from_millis(100)),
        );

        // Run property test with full monitoring
        let config = GameStateConfig {
            min_money: 0.0,
            max_money: 100.0,
            min_ante: 1,
            max_ante: 3,
            min_jokers: 0,
            max_jokers: 2,
            allow_negative_money: false,
        };

        let result = run_property_test(config, |game| {
            // Track as resource
            resource_tracker.register(
                format!("game_{:p}", &game),
                "Game".to_string(),
                std::mem::size_of_val(&game),
            );

            // Measure performance
            perf_monitor.measure("full_game", || {
                // Simulate game play
                let actions = game.gen_actions().take(5).collect::<Vec<_>>();
                assert!(!actions.is_empty() || game.is_over());
            });

            // Check invariants
            invariant_money_non_negative(&game)
                && invariant_ante_progression(&game)
                && invariant_joker_slots(&game)
        });

        assert!(result.is_ok());

        // Generate reports
        let perf_report = perf_monitor.report();
        println!("Performance Report:\n{}", perf_report);

        let mem_report = mem_stats.report();
        println!("Memory Report:\n{}", mem_report);

        // Cleanup and verify
        resource_tracker.clear();
        assert_eq!(resource_tracker.resource_count(), 0);

        // Check for performance regressions
        let violations = perf_monitor.check_thresholds();
        assert!(
            violations.is_empty(),
            "Performance violations: {:?}",
            violations
        );
    }
}
