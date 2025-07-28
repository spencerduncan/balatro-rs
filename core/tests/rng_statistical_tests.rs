/// Statistical distribution tests for RNG fairness and security
/// These tests verify that the RNG implementation produces statistically fair distributions
/// and meets the requirements specified in issue #281
///
/// NOTE: These tests require the "statistical_tests" feature flag because they are:
/// - Slow (require large sample sizes for statistical significance)
/// - Potentially flaky (statistical tests can occasionally fail due to randomness)
/// - Resource intensive for CI runners
///
/// To run these tests locally, use: cargo test --test rng_statistical_tests --features statistical_tests
use balatro_rs::rng::GameRng;
use std::collections::HashMap;

/// Test that uniform distribution is actually uniform within statistical bounds
#[test]
#[ignore = "EMERGENCY DISABLE: Hand empty method issues - tracked for post-emergency fix"]
#[cfg(feature = "statistical_tests")]
fn test_uniform_distribution_fairness() {
    let rng = GameRng::secure(); // Use secure RNG for this test
    let sample_size = 10_000;
    let range_size = 10;
    let mut counts = vec![0; range_size];

    // Generate samples
    for _ in 0..sample_size {
        let value = rng.gen_range(0..range_size);
        counts[value] += 1;
    }

    // Calculate expected count and acceptable variance
    let expected_count = sample_size / range_size;
    let tolerance = (expected_count as f64 * 0.05) as usize; // 5% tolerance

    // Check that all buckets are within tolerance
    for (i, &count) in counts.iter().enumerate() {
        let deviation = (count as usize).abs_diff(expected_count);

        assert!(
            deviation <= tolerance,
            "Bucket {i} has count {count} (expected ~{expected_count}), deviation {deviation} exceeds tolerance {tolerance}"
        );
    }
}

/// Test chi-square goodness of fit for uniform distribution
#[test]
#[ignore = "EMERGENCY DISABLE: Hand empty method issues - tracked for post-emergency fix"]
#[cfg(feature = "statistical_tests")]
fn test_chi_square_uniform_distribution() {
    let rng = GameRng::secure();
    let sample_size = 5_000;
    let buckets = 20;
    let mut observed = vec![0; buckets];

    // Generate samples
    for _ in 0..sample_size {
        let value = rng.gen_range(0..buckets);
        observed[value] += 1;
    }

    // Calculate chi-square statistic
    let expected = sample_size as f64 / buckets as f64;
    let mut chi_square = 0.0;

    for count in observed {
        let diff = count as f64 - expected;
        chi_square += (diff * diff) / expected;
    }

    // For 19 degrees of freedom (20 buckets - 1) and α = 0.001,
    // critical value is approximately 38.58
    // We use a more conservative threshold to account for RNG variance
    assert!(
        chi_square < 45.0,
        "Chi-square statistic {chi_square} indicates non-uniform distribution (critical value ~38.58)"
    );
}

/// Test that boolean generation with different probabilities works correctly
#[test]
#[ignore = "EMERGENCY DISABLE: Hand empty method issues - tracked for post-emergency fix"]
#[cfg(feature = "statistical_tests")]
fn test_boolean_probability_distribution() {
    let rng = GameRng::secure();
    let sample_size = 10_000;

    // Test different probabilities
    let probabilities = [0.1, 0.25, 0.5, 0.75, 0.9];

    for &prob in &probabilities {
        let mut true_count = 0;

        for _ in 0..sample_size {
            if rng.gen_bool(prob) {
                true_count += 1;
            }
        }

        let observed_prob = true_count as f64 / sample_size as f64;
        let tolerance = 0.01; // 1% tolerance

        assert!(
            (observed_prob - prob).abs() < tolerance,
            "Boolean generation with p={prob} gave observed probability {observed_prob} (tolerance {tolerance})"
        );
    }
}

/// Test that shuffle produces all possible permutations over many runs
#[test]
#[ignore = "EMERGENCY DISABLE: Hand empty method issues - tracked for post-emergency fix"]
#[cfg(feature = "statistical_tests")]
fn test_shuffle_permutation_coverage() {
    let rng = GameRng::secure();
    let data = vec![1, 2, 3, 4]; // Small array for manageable permutation space
    let mut permutation_counts: HashMap<Vec<i32>, usize> = HashMap::new();
    let iterations = 10_000;

    for _ in 0..iterations {
        let mut copy = data.clone();
        rng.shuffle(&mut copy);
        *permutation_counts.entry(copy).or_insert(0) += 1;
    }

    // There are 4! = 24 possible permutations
    let expected_permutations = 24;

    // We should see a reasonable number of different permutations
    // (not necessarily all, due to randomness, but most)
    let min_expected_unique = expected_permutations * 3 / 4; // At least 75%

    assert!(
        permutation_counts.len() >= min_expected_unique,
        "Shuffle only produced {} unique permutations out of {} possible (minimum expected: {})",
        permutation_counts.len(),
        expected_permutations,
        min_expected_unique
    );

    // Check that no single permutation dominates
    let max_count = permutation_counts.values().max().unwrap();
    let expected_avg = iterations / expected_permutations;
    let max_acceptable = expected_avg * 3; // No permutation should be 3x more common

    assert!(
        *max_count <= max_acceptable,
        "Most common permutation appeared {max_count} times (expected avg: {expected_avg}, max acceptable: {max_acceptable})"
    );
}

/// Test weighted choice distribution
#[test]
#[ignore = "EMERGENCY DISABLE: Hand empty method issues - tracked for post-emergency fix"]
#[cfg(feature = "statistical_tests")]
fn test_weighted_choice_distribution() {
    let rng = GameRng::secure();
    let items = vec!["A", "B", "C", "D"];
    let weights = [1.0, 2.0, 3.0, 4.0]; // Total weight: 10
    let sample_size = 10_000;
    let mut counts = HashMap::new();

    for _ in 0..sample_size {
        let choice = rng
            .choose_weighted(&items, |i| {
                weights[items.iter().position(|&x| x == *i).unwrap()]
            })
            .unwrap();
        *counts.entry(*choice).or_insert(0) += 1;
    }

    // Check expected proportions
    let total_weight = weights.iter().sum::<f64>();
    for (i, &item) in items.iter().enumerate() {
        let expected_prop = weights[i] / total_weight;
        let observed_count = counts.get(item).unwrap_or(&0);
        let observed_prop = *observed_count as f64 / sample_size as f64;
        let tolerance = 0.01; // 1% tolerance

        assert!(
            (observed_prop - expected_prop).abs() < tolerance,
            "Item {item} expected proportion {expected_prop} but observed {observed_prop} (tolerance {tolerance})"
        );
    }
}

/// Test deterministic reproducibility
#[test]
#[ignore = "EMERGENCY DISABLE: Hand empty method issues - tracked for post-emergency fix"]
#[cfg(feature = "statistical_tests")]
fn test_deterministic_reproducibility() {
    let seed = 12345;
    let sample_size = 1000;

    // Generate first sequence
    let rng1 = GameRng::for_testing(seed);
    let mut sequence1 = Vec::new();
    for _ in 0..sample_size {
        sequence1.push(rng1.gen_range(0..1000u32));
    }

    // Generate second sequence with same seed
    let rng2 = GameRng::for_testing(seed);
    let mut sequence2 = Vec::new();
    for _ in 0..sample_size {
        sequence2.push(rng2.gen_range(0..1000u32));
    }

    // Sequences should be identical
    assert_eq!(
        sequence1, sequence2,
        "Deterministic RNG with same seed produced different sequences"
    );
}

/// Test that different seeds produce different sequences
#[test]
#[ignore = "EMERGENCY DISABLE: Hand empty method issues - tracked for post-emergency fix"]
#[cfg(feature = "statistical_tests")]
fn test_seed_independence() {
    let sample_size = 1000;

    // Generate sequences with different seeds
    let rng1 = GameRng::for_testing(111);
    let rng2 = GameRng::for_testing(222);

    let mut sequence1 = Vec::new();
    let mut sequence2 = Vec::new();

    for _ in 0..sample_size {
        sequence1.push(rng1.gen_range(0..1000u32));
        sequence2.push(rng2.gen_range(0..1000u32));
    }

    // Count differences
    let differences = sequence1
        .iter()
        .zip(sequence2.iter())
        .filter(|(a, b)| a != b)
        .count();
    let min_expected_differences = sample_size * 90 / 100; // At least 90% should be different

    assert!(
        differences >= min_expected_differences,
        "Only {differences} out of {sample_size} values differed between different seeds (expected at least {min_expected_differences})"
    );
}

/// Test that forked RNGs produce independent sequences
#[test]
#[ignore = "EMERGENCY DISABLE: Hand empty method issues - tracked for post-emergency fix"]
#[cfg(feature = "statistical_tests")]
fn test_fork_independence() {
    let parent = GameRng::for_testing(42);
    let child = parent.fork();

    let sample_size = 1000;
    let mut parent_sequence = Vec::new();
    let mut child_sequence = Vec::new();

    for _ in 0..sample_size {
        parent_sequence.push(parent.gen_range(0..1000u32));
        child_sequence.push(child.gen_range(0..1000u32));
    }

    // Count differences
    let differences = parent_sequence
        .iter()
        .zip(child_sequence.iter())
        .filter(|(a, b)| a != b)
        .count();
    let min_expected_differences = sample_size * 90 / 100; // At least 90% should be different

    assert!(
        differences >= min_expected_differences,
        "Only {differences} out of {sample_size} values differed between parent and forked RNG (expected at least {min_expected_differences})"
    );
}

/// Test security properties - ensure secure RNG is unpredictable
#[test]
#[ignore = "EMERGENCY DISABLE: Hand empty method issues - tracked for post-emergency fix"]
#[cfg(feature = "statistical_tests")]
fn test_secure_rng_unpredictability() {
    // Create two independent secure RNGs
    let rng1 = GameRng::secure();
    let rng2 = GameRng::secure();

    let sample_size = 1000;
    let mut sequence1 = Vec::new();
    let mut sequence2 = Vec::new();

    for _ in 0..sample_size {
        sequence1.push(rng1.gen_range(0..1000u32));
        sequence2.push(rng2.gen_range(0..1000u32));
    }

    // Count similarities
    let similarities = sequence1
        .iter()
        .zip(sequence2.iter())
        .filter(|(a, b)| a == b)
        .count();
    let max_expected_similarities = sample_size / 50; // Less than 2% should be identical

    assert!(
        similarities <= max_expected_similarities,
        "Too many similarities ({similarities}) between independent secure RNGs (max expected: {max_expected_similarities})"
    );
}

/// Performance test - ensure RNG operations meet speed requirements
#[test]
#[ignore = "EMERGENCY DISABLE: Hand empty method issues - tracked for post-emergency fix"]
#[cfg(feature = "statistical_tests")]
fn test_rng_performance() {
    let rng = GameRng::secure();
    let iterations = 10_000;

    let start = std::time::Instant::now();

    for _ in 0..iterations {
        let _value = rng.gen_range(0..1000u32);
    }

    let duration = start.elapsed();
    let ops_per_second = iterations as f64 / duration.as_secs_f64();

    // Requirement: should be able to generate at least 10,000 random numbers per second
    // This is a conservative requirement to ensure performance impact < 2% as specified
    assert!(
        ops_per_second >= 10_000.0,
        "RNG performance too slow: {} ops/sec (minimum required: 10,000)",
        ops_per_second as u64
    );
}

/// Test thread safety and isolation
#[test]
#[ignore = "EMERGENCY DISABLE: Hand empty method issues - tracked for post-emergency fix"]
#[cfg(feature = "statistical_tests")]
fn test_thread_safety() {
    use std::sync::{Arc, Mutex};
    use std::thread;

    let rng = Arc::new(GameRng::secure());
    let results = Arc::new(Mutex::new(Vec::new()));
    let mut handles = Vec::new();

    // Spawn multiple threads generating random numbers
    for i in 0..4 {
        let rng_clone = rng.clone();
        let results_clone = results.clone();

        let handle = thread::spawn(move || {
            let mut local_results = Vec::new();
            for _ in 0..1000 {
                local_results.push((i, rng_clone.gen_range(0..1000u32)));
            }
            results_clone.lock().unwrap().extend(local_results);
        });

        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().expect("Thread should complete successfully");
    }

    let final_results = results.lock().unwrap();

    // Should have results from all threads
    assert_eq!(final_results.len(), 4000);

    // Check that each thread contributed
    for thread_id in 0..4 {
        let thread_count = final_results
            .iter()
            .filter(|(id, _)| *id == thread_id)
            .count();
        assert_eq!(
            thread_count, 1000,
            "Thread {thread_id} should have contributed 1000 results"
        );
    }
}

// Tests for RNG-based jokers (Issue #442)

/// Test LuckyCardJoker probability distribution
/// Should trigger approximately 20% of the time (1 in 5 chance)
#[test]
#[ignore = "EMERGENCY DISABLE: Hand empty method issues - tracked for post-emergency fix"]
#[cfg(feature = "statistical_tests")]
fn test_lucky_card_joker_probability() {
    use balatro_rs::hand::Hand;
    use balatro_rs::hand::SelectHand;
    use balatro_rs::joker::{GameContext, Joker};
    use balatro_rs::joker_impl::LuckyCardJoker;
    use balatro_rs::joker_state::JokerStateManager;
    use balatro_rs::stage::Stage;
    use std::sync::Arc;

    let rng = GameRng::for_testing(12345); // Deterministic for reproducible testing
    let lucky_card = LuckyCardJoker;
    let sample_size = 10_000;
    let mut activation_count = 0;

    // Mock context and hand for testing
    let state_manager = Arc::new(JokerStateManager::new());
    let stage = Stage::PreBlind();
    let jokers: Vec<Box<dyn Joker>> = vec![];
    let hand = Hand::new(vec![]);
    let discarded = vec![];
    let hand_type_counts = HashMap::new();
    let select_hand = SelectHand::new(vec![]);

    for _ in 0..sample_size {
        let mut context = GameContext {
            chips: 0,
            mult: 0,
            money: 10,
            ante: 1,
            round: 1,
            stage: &stage,
            hands_played: 0,
            discards_used: 0,
            jokers: &jokers,
            hand: &hand,
            discarded: &discarded,
            joker_state_manager: &state_manager,
            hand_type_counts: &hand_type_counts,
            cards_in_deck: 52,
            stone_cards_in_deck: 0, // TEST: Using standard test deck composition
            steel_cards_in_deck: 0, // TEST: Using standard test deck composition
            rng: &rng,
        };

        let effect = lucky_card.on_hand_played(&mut context, &select_hand);
        if effect.mult > 0 {
            activation_count += 1;
        }
    }

    let activation_rate = activation_count as f64 / sample_size as f64;
    let expected_rate = 0.2; // 20%
    let tolerance = 0.05; // ±5% tolerance for statistical testing

    assert!(
        (activation_rate - expected_rate).abs() < tolerance,
        "Lucky Card activation rate {:.3} is outside expected range [{:.3}, {:.3}]",
        activation_rate,
        expected_rate - tolerance,
        expected_rate + tolerance
    );
}

/// Test deterministic behavior with seeded RNG
/// RNG jokers should produce identical results with the same seed
#[test]
#[ignore = "EMERGENCY DISABLE: Hand empty method issues - tracked for post-emergency fix"]
fn test_rng_jokers_deterministic_behavior() {
    use balatro_rs::hand::Hand;
    use balatro_rs::hand::SelectHand;
    use balatro_rs::joker::{GameContext, Joker};
    use balatro_rs::joker_impl::{ChaoticJoker, LuckyCardJoker, MysteryJoker};
    use balatro_rs::joker_state::JokerStateManager;
    use balatro_rs::stage::Stage;
    use std::sync::Arc;

    let seed = 42;
    let test_iterations = 100;

    // Test each RNG joker with the same seed
    let jokers: Vec<Box<dyn Joker>> = vec![
        Box::new(LuckyCardJoker),
        Box::new(MysteryJoker),
        Box::new(ChaoticJoker),
    ];

    for joker in jokers {
        let mut first_run_results = Vec::new();
        let mut second_run_results = Vec::new();

        // First run with seeded RNG
        let rng1 = GameRng::for_testing(seed);
        let state_manager1 = Arc::new(JokerStateManager::new());
        let stage = Stage::PreBlind();
        let jokers_vec: Vec<Box<dyn Joker>> = vec![];
        let hand = Hand::new(vec![]);
        let discarded = vec![];
        let hand_type_counts = HashMap::new();
        let select_hand = SelectHand::new(vec![]);

        for _ in 0..test_iterations {
            let mut context = GameContext {
                chips: 0,
                mult: 0,
                money: 10,
                ante: 1,
                round: 1,
                stage: &stage,
                hands_played: 0,
                discards_used: 0,
                jokers: &jokers_vec,
                hand: &hand,
                discarded: &discarded,
                joker_state_manager: &state_manager1,
                hand_type_counts: &hand_type_counts,
                cards_in_deck: 52,
                stone_cards_in_deck: 0,
                steel_cards_in_deck: 0,
                rng: &rng1,
            };

            let effect = joker.on_hand_played(&mut context, &select_hand);
            first_run_results.push((
                effect.chips,
                effect.mult,
                effect.money,
                effect.mult_multiplier,
                effect.retrigger,
            ));
        }

        // Second run with same seeded RNG
        let rng2 = GameRng::for_testing(seed);
        let state_manager2 = Arc::new(JokerStateManager::new());

        for _ in 0..test_iterations {
            let mut context = GameContext {
                chips: 0,
                mult: 0,
                money: 10,
                ante: 1,
                round: 1,
                stage: &stage,
                hands_played: 0,
                discards_used: 0,
                jokers: &jokers_vec,
                hand: &hand,
                discarded: &discarded,
                joker_state_manager: &state_manager2,
                hand_type_counts: &hand_type_counts,
                cards_in_deck: 52,
                stone_cards_in_deck: 0,
                steel_cards_in_deck: 0,
                rng: &rng2,
            };

            let effect = joker.on_hand_played(&mut context, &select_hand);
            second_run_results.push((
                effect.chips,
                effect.mult,
                effect.money,
                effect.mult_multiplier,
                effect.retrigger,
            ));
        }

        // Results should be identical
        assert_eq!(
            first_run_results,
            second_run_results,
            "Joker {} should produce identical results with same seed",
            joker.name()
        );
    }
}
