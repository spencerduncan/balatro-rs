//! Performance benchmarks for trait refactoring
//!
//! This benchmark suite measures the performance impact of the new trait system
//! and ensures that the refactoring doesn't introduce performance regressions.

#![allow(clippy::field_reassign_with_default)]

use balatro_rs::{
    action::Action,
    card::{Card, Suit, Value},
    game::Game,
    hand::SelectHand,
    joker::{GameContext, Joker, JokerEffect, JokerId},
    joker_effect_processor::{ConflictResolutionStrategy, JokerEffectProcessor, ProcessingContext},
    joker_registry,
    rank::HandRank,
    rng::GameRng,
    shop::Shop,
    stage::Stage,
};
#[allow(deprecated)]
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use std::collections::HashMap;
use std::hint::black_box;
use std::time::Instant;

/// Benchmark trait method dispatch overhead
pub fn trait_dispatch_benchmark(c: &mut Criterion) {
    // Initialize all systems before running benchmarks to avoid factory race conditions
    balatro_rs::initialize().expect("Failed to initialize core systems");

    let mut group = c.benchmark_group("trait_dispatch");

    // Create test jokers for benchmarking
    let joker_ids = vec![JokerId::Joker, JokerId::GreedyJoker, JokerId::LustyJoker];

    for joker_id in joker_ids {
        if let Ok(joker) = joker_registry::registry::create_joker(&joker_id) {
            group.bench_with_input(
                BenchmarkId::new("trait_method_call", format!("{joker_id:?}")),
                &joker,
                |b, joker| {
                    let test_data = TestGameData::new();
                    let test_card = Card::new(Value::Ace, Suit::Spade);

                    b.iter(|| {
                        let mut game_context = test_data.create_context();
                        // Benchmark trait method dispatch
                        black_box(joker.id());
                        black_box(joker.on_card_scored(&mut game_context, &test_card));
                        black_box(joker.on_hand_played(&mut game_context, &create_test_hand()));
                    });
                },
            );
        }
    }

    group.finish();
}

/// Benchmark JokerEffectProcessor performance
pub fn effect_processor_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("effect_processor");

    // Test different numbers of jokers
    let joker_counts = vec![1, 3, 5, 10];

    for count in joker_counts {
        group.bench_with_input(
            BenchmarkId::new("hand_processing", count),
            &count,
            |b, &count| {
                let jokers = create_test_jokers(count);
                let mut processor = JokerEffectProcessor::new();
                let test_data = TestGameData::new();
                let hand = create_test_hand();

                b.iter(|| {
                    let mut game_context = test_data.create_context();
                    let result = processor.process_hand_effects(
                        black_box(&jokers),
                        black_box(&mut game_context),
                        black_box(&hand),
                    );
                    black_box(result);
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("card_processing", count),
            &count,
            |b, &count| {
                let jokers = create_test_jokers(count);
                let mut processor = JokerEffectProcessor::new();
                let test_data = TestGameData::new();
                let test_card = Card::new(Value::King, Suit::Heart);

                b.iter(|| {
                    let mut game_context = test_data.create_context();
                    let result = processor.process_card_effects(
                        black_box(&jokers),
                        black_box(&mut game_context),
                        black_box(&test_card),
                    );
                    black_box(result);
                });
            },
        );
    }

    group.finish();
}

/// Benchmark effect processing with different conflict resolution strategies
pub fn conflict_resolution_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("conflict_resolution");

    let strategies = vec![
        ConflictResolutionStrategy::Sum,
        ConflictResolutionStrategy::Maximum,
        ConflictResolutionStrategy::Minimum,
        ConflictResolutionStrategy::FirstWins,
        ConflictResolutionStrategy::LastWins,
    ];

    for strategy in strategies {
        group.bench_with_input(
            BenchmarkId::new("strategy", format!("{strategy:?}")),
            &strategy,
            |b, strategy| {
                let jokers = create_test_jokers(5);
                let context = ProcessingContext::builder()
                    .resolution_strategy(strategy.clone())
                    .build();
                let mut processor = JokerEffectProcessor::with_context(context);
                let test_data = TestGameData::new();
                let hand = create_test_hand();

                b.iter(|| {
                    let mut game_context = test_data.create_context();
                    let result = processor.process_hand_effects(
                        black_box(&jokers),
                        black_box(&mut game_context),
                        black_box(&hand),
                    );
                    black_box(result);
                });
            },
        );
    }

    group.finish();
}

/// Benchmark memory allocation with different joker counts
pub fn memory_allocation_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_allocation");

    let joker_counts = vec![1, 5, 10, 20, 50];

    for count in joker_counts {
        group.throughput(Throughput::Elements(count as u64));
        group.bench_with_input(
            BenchmarkId::new("joker_creation", count),
            &count,
            |b, &count| {
                b.iter(|| {
                    let jokers = create_test_jokers(count);
                    black_box(jokers);
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("effect_accumulation", count),
            &count,
            |b, &count| {
                let jokers = create_test_jokers(count);
                let mut processor = JokerEffectProcessor::new();
                let test_data = TestGameData::new();
                let hand = create_test_hand();

                b.iter(|| {
                    let mut game_context = test_data.create_context();
                    // This will create and accumulate many effects
                    for _ in 0..10 {
                        let result =
                            processor.process_hand_effects(&jokers, &mut game_context, &hand);
                        black_box(result);
                    }
                });
            },
        );
    }

    group.finish();
}

/// Benchmark shop generation performance with jokers
pub fn shop_generation_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("shop_generation");

    group.bench_function("shop_creation", |b| {
        let mut game = Game::default();
        game.start();

        b.iter(|| {
            let shop = Shop::new();
            black_box(shop);
        });
    });

    group.bench_function("shop_with_jokers", |b| {
        let mut game = Game::default();
        game.start();

        // Add some jokers to the game state
        for joker_id in [JokerId::Joker, JokerId::GreedyJoker, JokerId::LustyJoker] {
            if let Ok(_joker) = joker_registry::registry::create_joker(&joker_id) {
                // game.add_joker(joker).ok(); // TODO: Fix - add_joker method doesn't exist
            }
        }

        b.iter(|| {
            let shop = Shop::new();
            black_box(shop);
        });
    });

    group.finish();
}

/// Benchmark action generation throughput
pub fn action_generation_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("action_generation");

    group.bench_function("actions_without_jokers", |b| {
        let mut game = Game::default();
        game.start();

        b.iter(|| {
            let actions: Vec<Action> = game.gen_actions().collect();
            black_box(actions);
        });
    });

    group.bench_function("actions_with_jokers", |b| {
        let mut game = Game::default();
        game.start();

        // Add jokers that might affect action generation
        for joker_id in [JokerId::Joker, JokerId::GreedyJoker] {
            if let Ok(_joker) = joker_registry::registry::create_joker(&joker_id) {
                // game.add_joker(joker).ok(); // TODO: Fix - add_joker method doesn't exist
            }
        }

        b.iter(|| {
            let actions: Vec<Action> = game.gen_actions().collect();
            black_box(actions);
        });
    });

    group.finish();
}

/// Benchmark effect processing with cache enabled vs disabled
pub fn cache_performance_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("cache_performance");

    group.bench_function("with_cache", |b| {
        let jokers = create_test_jokers(5);
        let mut processor = JokerEffectProcessor::new(); // Cache enabled by default
        let test_data = TestGameData::new();
        let hand = create_test_hand();

        b.iter(|| {
            let mut game_context = test_data.create_context();
            // Process the same hand multiple times to benefit from caching
            for _ in 0..10 {
                let result = processor.process_hand_effects(&jokers, &mut game_context, &hand);
                black_box(result);
            }
        });
    });

    group.bench_function("without_cache", |b| {
        let jokers = create_test_jokers(5);
        let context = ProcessingContext::builder().build();
        let mut processor = JokerEffectProcessor::with_context(context);
        // Disable cache
        let mut cache_config = balatro_rs::joker_effect_processor::CacheConfig::default();
        cache_config.enabled = false;
        processor.set_cache_config(cache_config);

        let test_data = TestGameData::new();
        let hand = create_test_hand();

        b.iter(|| {
            let mut game_context = test_data.create_context();
            // Process the same hand multiple times without cache benefit
            for _ in 0..10 {
                let result = processor.process_hand_effects(&jokers, &mut game_context, &hand);
                black_box(result);
            }
        });
    });

    group.finish();
}

/// Benchmark retriggering performance
pub fn retrigger_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("retrigger_performance");

    let retrigger_counts = vec![0, 1, 2, 5, 10];

    for retrigger_count in retrigger_counts {
        group.bench_with_input(
            BenchmarkId::new("retrigger_effects", retrigger_count),
            &retrigger_count,
            |b, &retrigger_count| {
                let _processor = JokerEffectProcessor::new();
                let joker_effects = vec![JokerEffect {
                    chips: 10,
                    mult: 2,
                    retrigger: retrigger_count,
                    ..Default::default()
                }];

                b.iter(|| {
                    // Simulate processing with retrigger effects
                    let start_time = Instant::now();
                    for effect in &joker_effects {
                        black_box(effect);
                    }
                    black_box(start_time.elapsed());
                });
            },
        );
    }

    group.finish();
}

// Helper functions for creating test data

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
            stone_cards_in_deck: 0,
            steel_cards_in_deck: 0,
            enhanced_cards_in_deck: 0,
            rng: &self.rng,
        }
    }
}

fn create_test_hand() -> SelectHand {
    SelectHand::new(vec![
        Card::new(Value::Ace, Suit::Spade),
        Card::new(Value::King, Suit::Spade),
        Card::new(Value::Queen, Suit::Spade),
        Card::new(Value::Jack, Suit::Spade),
        Card::new(Value::Ten, Suit::Spade),
    ])
}

fn create_test_jokers(count: usize) -> Vec<Box<dyn Joker>> {
    let joker_ids = [JokerId::Joker, JokerId::GreedyJoker, JokerId::LustyJoker];
    let mut jokers = Vec::new();

    for i in 0..count {
        let joker_id = &joker_ids[i % joker_ids.len()];
        if let Ok(joker) = joker_registry::registry::create_joker(joker_id) {
            jokers.push(joker);
        }
    }

    jokers
}

criterion_group!(
    trait_benchmarks,
    trait_dispatch_benchmark,
    effect_processor_benchmark,
    conflict_resolution_benchmark,
    memory_allocation_benchmark,
    shop_generation_benchmark,
    action_generation_benchmark,
    cache_performance_benchmark,
    retrigger_benchmark
);
criterion_main!(trait_benchmarks);
