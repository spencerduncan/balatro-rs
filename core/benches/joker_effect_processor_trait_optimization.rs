//! Benchmark comparing legacy vs trait-optimized JokerEffectProcessor performance
//!
//! This benchmark demonstrates the performance improvements achieved by the trait-specific
//! optimization paths in JokerEffectProcessor, as implemented for issue #431.

use balatro_rs::card::{Card, Suit, Value};
use balatro_rs::hand::SelectHand;
use balatro_rs::joker::{GameContext, Joker};
use balatro_rs::joker_effect_processor::JokerEffectProcessor;
use balatro_rs::joker_impl::{GreedyJoker, LustyJoker, TheJoker};
use balatro_rs::rank::HandRank;
use balatro_rs::rng::GameRng;
use balatro_rs::stage::Stage;
#[allow(deprecated)]
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::collections::HashMap;
use std::hint::black_box;

// Test data holder to maintain ownership
struct TestGameData {
    stage: Stage,
    hand: balatro_rs::hand::Hand,
    joker_state_manager: std::sync::Arc<balatro_rs::joker_state::JokerStateManager>,
    hand_type_counts: HashMap<HandRank, u32>,
    rng: GameRng,
}

impl TestGameData {
    fn new() -> Self {
        Self {
            stage: Stage::PreBlind(),
            hand: balatro_rs::hand::Hand::new(vec![]),
            joker_state_manager: std::sync::Arc::new(
                balatro_rs::joker_state::JokerStateManager::new(),
            ),
            hand_type_counts: HashMap::new(),
            rng: GameRng::for_testing(12345),
        }
    }

    fn create_context(&self) -> GameContext {
        GameContext {
            chips: 100,
            mult: 4,
            money: 100,
            ante: 1,
            round: 1,
            stage: &self.stage,
            hands_played: 0,
            discards_used: 0,
            hands_remaining: 4.0, // Standard hands remaining for testing
            is_final_hand: false,
            jokers: &[],
            hand: &self.hand,
            discarded: &[],
            joker_state_manager: &self.joker_state_manager,
            hand_type_counts: &self.hand_type_counts,
            cards_in_deck: 52,
            stone_cards_in_deck: 0, // BENCHMARK: Using standard deck composition
            steel_cards_in_deck: 0, // BENCHMARK: Using standard deck composition
            rng: &self.rng,
        }
    }
}

/// Create a test hand for benchmarking
fn create_test_hand() -> SelectHand {
    SelectHand::new(vec![
        Card::new(Value::Ace, Suit::Heart),
        Card::new(Value::King, Suit::Heart),
        Card::new(Value::Queen, Suit::Heart),
        Card::new(Value::Jack, Suit::Heart),
        Card::new(Value::Ten, Suit::Heart),
    ])
}

/// Create test jokers for benchmarking
fn create_test_jokers(count: usize) -> Vec<Box<dyn Joker>> {
    let mut jokers = Vec::new();

    for i in 0..count {
        match i % 3 {
            0 => jokers.push(Box::new(TheJoker) as Box<dyn Joker>),
            1 => jokers.push(Box::new(GreedyJoker) as Box<dyn Joker>),
            2 => jokers.push(Box::new(LustyJoker) as Box<dyn Joker>),
            _ => unreachable!(),
        }
    }

    jokers
}

/// Benchmark legacy hand effect processing
fn bench_legacy_hand_processing(c: &mut Criterion) {
    let mut group = c.benchmark_group("hand_processing_legacy");

    for joker_count in [1, 5, 10, 20, 50].iter() {
        group.bench_with_input(
            BenchmarkId::new("legacy", joker_count),
            joker_count,
            |b, &joker_count| {
                let mut processor = JokerEffectProcessor::new();
                let jokers = create_test_jokers(joker_count);
                let hand = create_test_hand();
                let test_data = TestGameData::new();

                b.iter(|| {
                    let mut game_context = test_data.create_context();

                    black_box(processor.process_hand_effects(
                        black_box(&jokers),
                        black_box(&mut game_context),
                        black_box(&hand),
                    ))
                });
            },
        );
    }

    group.finish();
}

/// Benchmark trait-optimized hand effect processing
fn bench_optimized_hand_processing(c: &mut Criterion) {
    let mut group = c.benchmark_group("hand_processing_optimized");

    for joker_count in [1, 5, 10, 20, 50].iter() {
        group.bench_with_input(
            BenchmarkId::new("optimized", joker_count),
            joker_count,
            |b, &joker_count| {
                let mut processor = JokerEffectProcessor::new();
                let jokers = create_test_jokers(joker_count);
                let hand = create_test_hand();
                let stage = Stage::PreBlind();
                let test_data = TestGameData::new();

                b.iter(|| {
                    let mut game_context = test_data.create_context();

                    black_box(processor.process_hand_effects_optimized(
                        black_box(&jokers),
                        black_box(&mut game_context),
                        black_box(&hand),
                        black_box(&stage),
                    ))
                });
            },
        );
    }

    group.finish();
}

/// Benchmark card effect processing comparison
fn bench_card_processing_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("card_processing_comparison");

    let jokers = create_test_jokers(10);
    let card = Card::new(Value::Ace, Suit::Diamond);
    let stage = Stage::PreBlind();

    group.bench_function("legacy_card_processing", |b| {
        let mut processor = JokerEffectProcessor::new();
        let test_data = TestGameData::new();

        b.iter(|| {
            let mut game_context = test_data.create_context();

            black_box(processor.process_card_effects(
                black_box(&jokers),
                black_box(&mut game_context),
                black_box(&card),
            ))
        });
    });

    group.bench_function("optimized_card_processing", |b| {
        let mut processor = JokerEffectProcessor::new();
        let test_data = TestGameData::new();

        b.iter(|| {
            let mut game_context = test_data.create_context();

            black_box(processor.process_card_effects_optimized(
                black_box(&jokers),
                black_box(&mut game_context),
                black_box(&card),
                black_box(&stage),
            ))
        });
    });

    group.finish();
}

/// Benchmark trait detection caching
fn bench_trait_detection_caching(c: &mut Criterion) {
    let mut group = c.benchmark_group("trait_detection");

    let jokers = create_test_jokers(10);

    group.bench_function("cold_trait_detection", |b| {
        b.iter(|| {
            let _processor = JokerEffectProcessor::new();

            // Cold cache - first detection of each joker
            for _joker in &jokers {
                // TODO: Fix - detect_joker_traits is now private
                // black_box(processor.detect_joker_traits(joker.as_ref()));
            }
        });
    });

    group.bench_function("warm_trait_detection", |b| {
        let _processor = JokerEffectProcessor::new();

        // Pre-warm the cache
        // TODO: Fix - detect_joker_traits is now private
        /*
        for joker in &jokers {
            processor.detect_joker_traits(joker.as_ref());
        }
        */

        b.iter(|| {
            // Warm cache - subsequent detections
            for _joker in &jokers {
                // TODO: Fix - detect_joker_traits is now private
                // black_box(processor.detect_joker_traits(joker.as_ref()));
            }
        });
    });

    group.finish();
}

/// Benchmark cache performance with different configurations
fn bench_cache_performance(c: &mut Criterion) {
    let mut group = c.benchmark_group("cache_performance");

    let jokers = create_test_jokers(5);
    let hand = create_test_hand();

    group.bench_function("cache_disabled", |b| {
        let mut processor = JokerEffectProcessor::new();
        // Disable caching
        let mut config = processor.context().cache_config.clone();
        config.enabled = false;
        processor.set_cache_config(config);

        let test_data = TestGameData::new();

        b.iter(|| {
            let mut game_context = test_data.create_context();

            black_box(processor.process_hand_effects(
                black_box(&jokers),
                black_box(&mut game_context),
                black_box(&hand),
            ))
        });
    });

    group.bench_function("cache_enabled", |b| {
        let mut processor = JokerEffectProcessor::new();
        // Cache is enabled by default
        let test_data = TestGameData::new();

        b.iter(|| {
            let mut game_context = test_data.create_context();

            black_box(processor.process_hand_effects(
                black_box(&jokers),
                black_box(&mut game_context),
                black_box(&hand),
            ))
        });
    });

    group.finish();
}

/// Comprehensive benchmark comparing all optimizations
fn bench_comprehensive_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("comprehensive_comparison");

    let jokers = create_test_jokers(20);
    let hand = create_test_hand();
    let stage = Stage::PreBlind();

    // Simulate a realistic game scenario with multiple hand plays
    group.bench_function("legacy_full_scenario", |b| {
        b.iter(|| {
            let mut processor = JokerEffectProcessor::new();
            let test_data = TestGameData::new();

            // Process 10 hands (simulating a blind)
            for _ in 0..10 {
                let mut game_context = test_data.create_context();

                black_box(processor.process_hand_effects(
                    black_box(&jokers),
                    black_box(&mut game_context),
                    black_box(&hand),
                ));
            }
        });
    });

    group.bench_function("optimized_full_scenario", |b| {
        b.iter(|| {
            let mut processor = JokerEffectProcessor::new();
            let test_data = TestGameData::new();

            // Process 10 hands (simulating a blind)
            for _ in 0..10 {
                let mut game_context = test_data.create_context();

                black_box(processor.process_hand_effects_optimized(
                    black_box(&jokers),
                    black_box(&mut game_context),
                    black_box(&hand),
                    black_box(&stage),
                ));
            }
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_legacy_hand_processing,
    bench_optimized_hand_processing,
    bench_card_processing_comparison,
    bench_trait_detection_caching,
    bench_cache_performance,
    bench_comprehensive_comparison
);

criterion_main!(benches);
