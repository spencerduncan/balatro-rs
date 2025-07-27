//! Simplified performance benchmarks for JokerEffectProcessor
//!
//! This benchmark suite focuses on the public API and measures performance
//! improvements from the JokerEffectProcessor optimizations.

use balatro_rs::joker_effect_processor::JokerEffectProcessor;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

/// Basic processing benchmarks with different numbers of jokers
fn basic_processing_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("basic_processing");

    // Test processing with single joker
    group.bench_function("single_joker_processing", |b| {
        b.iter(|| {
            let mut processor = JokerEffectProcessor::new();
            let jokers = create_multiple_jokers(1);
            let mut game_context = create_test_game_context();
            let hand = create_test_hand();

            black_box(processor.process_hand_effects(&jokers, &mut game_context, &hand))
        });
    });

    // Test processing with multiple jokers
    group.bench_function("multiple_jokers_processing", |b| {
        b.iter(|| {
            let mut processor = JokerEffectProcessor::new();
            let jokers = create_multiple_jokers(5);
            let mut game_context = create_test_game_context();
            let hand = create_test_hand();

            black_box(processor.process_hand_effects(&jokers, &mut game_context, &hand))
        });
    });

    group.finish();
}

/// Benchmark cache performance
fn cache_performance_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("cache_performance");

    // Cache simulation - repeated processing with same inputs
    group.bench_function("repeated_processing", |b| {
        let mut processor = JokerEffectProcessor::new();
        let jokers = create_multiple_jokers(3);

        // Prime the cache by running once
        let mut game_context = create_test_game_context();
        let hand = create_test_hand();
        processor.process_hand_effects(&jokers, &mut game_context, &hand);

        b.iter(|| {
            // This should benefit from any internal caching
            let mut game_context = create_test_game_context();
            let hand = create_test_hand();
            black_box(processor.process_hand_effects(&jokers, &mut game_context, &hand))
        });
    });

    group.finish();
}

/// Benchmark processing with various scenarios
fn scaling_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("scaling");

    // Test different numbers of jokers
    for joker_count in [1, 5, 10, 20].iter() {
        group.throughput(Throughput::Elements(*joker_count as u64));

        group.bench_with_input(
            BenchmarkId::new("joker_count", joker_count),
            joker_count,
            |b, &joker_count| {
                b.iter(|| {
                    let mut processor = JokerEffectProcessor::new();
                    let jokers = create_multiple_jokers(joker_count);
                    let mut game_context = create_test_game_context();
                    let hand = create_test_hand();

                    black_box(processor.process_hand_effects(&jokers, &mut game_context, &hand))
                });
            },
        );
    }

    // Test card processing performance
    group.bench_function("card_processing", |b| {
        b.iter(|| {
            let mut processor = JokerEffectProcessor::new();
            let jokers = create_multiple_jokers(5);
            let mut game_context = create_test_game_context();
            let test_card = balatro_rs::card::Card::new(
                balatro_rs::card::Value::King,
                balatro_rs::card::Suit::Heart,
            );

            black_box(processor.process_card_effects(&jokers, &mut game_context, &test_card))
        });
    });

    // Performance targets:
    // - 10 jokers should be < 10μs
    // - 20 jokers should be < 50μs
    group.finish();
}

// Helper functions for creating test data

fn create_multiple_jokers(count: usize) -> Vec<Box<dyn balatro_rs::joker::Joker>> {
    let mut jokers = Vec::new();
    for _i in 0..count {
        if let Some(joker) =
            balatro_rs::joker_factory::JokerFactory::create(balatro_rs::joker::JokerId::Joker)
        {
            jokers.push(joker);
        }
    }
    jokers
}

fn create_test_game_context() -> balatro_rs::joker::GameContext<'static> {
    use std::collections::HashMap;

    // Create static references for the benchmark
    let stage = Box::leak(Box::new(balatro_rs::stage::Stage::PreBlind()));
    let hand = Box::leak(Box::new(balatro_rs::hand::Hand::new(vec![])));
    let jokers: &'static [Box<dyn balatro_rs::joker::Joker>] = Box::leak(Box::new([]));
    let discarded: &'static [balatro_rs::card::Card] = Box::leak(Box::new([]));
    let joker_state_manager = Box::leak(Box::new(std::sync::Arc::new(
        balatro_rs::joker_state::JokerStateManager::new(),
    )));
    let hand_type_counts = Box::leak(Box::new(HashMap::new()));
    let rng = Box::leak(Box::new(balatro_rs::rng::GameRng::for_testing(12345)));

    balatro_rs::joker::GameContext {
        chips: 100,
        mult: 4,
        money: 100,
        ante: 1,
        round: 1,
        stage,
        hands_played: 0,
        discards_used: 0,
        jokers,
        hand,
        discarded,
        joker_state_manager,
        hand_type_counts,
        cards_in_deck: 52,
        stone_cards_in_deck: 0,
        steel_cards_in_deck: 0,
        rng,
    }
}

fn create_test_hand() -> balatro_rs::hand::SelectHand {
    use balatro_rs::card::{Card, Suit, Value};
    balatro_rs::hand::SelectHand::new(vec![
        Card::new(Value::Ace, Suit::Spade),
        Card::new(Value::King, Suit::Spade),
        Card::new(Value::Queen, Suit::Spade),
        Card::new(Value::Jack, Suit::Spade),
        Card::new(Value::Ten, Suit::Spade),
    ])
}

criterion_group!(
    simple_benches,
    basic_processing_benchmarks,
    cache_performance_benchmarks,
    scaling_benchmarks
);
criterion_main!(simple_benches);
