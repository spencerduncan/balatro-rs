//! Performance benchmarks for trait refactoring
//! 
//! This benchmark suite measures the performance impact of the new trait system
//! and ensures that the refactoring doesn't introduce performance regressions.

use balatro_rs::{
    action::Action,
    card::{Card, Suit, Value},
    game::Game,
    hand::SelectHand,
    joker::{GameContext, Joker, JokerEffect, JokerId},
    joker_effect_processor::{JokerEffectProcessor, ProcessingContext, ConflictResolutionStrategy},
    joker_registry,
    rng::GameRng,
    shop::Shop,
    stage::Stage,
};
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
use std::collections::HashMap;
use std::time::Instant;

/// Benchmark trait method dispatch overhead
pub fn trait_dispatch_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("trait_dispatch");
    
    // Create test jokers for benchmarking
    let joker_ids = vec![
        JokerId::Joker,
        JokerId::GreedyJoker,
        JokerId::LustyJoker,
    ];
    
    for joker_id in joker_ids {
        if let Ok(Some(definition)) = joker_registry::registry::get_definition(&joker_id) {
            let joker = definition.create();
            
            group.bench_with_input(
                BenchmarkId::new("trait_method_call", format!("{:?}", joker_id)),
                &joker,
                |b, joker| {
                    let mut game_context = create_test_game_context();
                    let test_card = Card::new(Value::Ace, Suit::Spade);
                    
                    b.iter(|| {
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
                let mut game_context = create_test_game_context();
                let hand = create_test_hand();
                
                b.iter(|| {
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
                let mut game_context = create_test_game_context();
                let test_card = Card::new(Value::King, Suit::Heart);
                
                b.iter(|| {
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
            BenchmarkId::new("strategy", format!("{:?}", strategy)),
            &strategy,
            |b, strategy| {
                let jokers = create_test_jokers(5);
                let context = ProcessingContext::builder()
                    .resolution_strategy(strategy.clone())
                    .build();
                let mut processor = JokerEffectProcessor::with_context(context);
                let mut game_context = create_test_game_context();
                let hand = create_test_hand();
                
                b.iter(|| {
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
                let mut game_context = create_test_game_context();
                let hand = create_test_hand();
                
                b.iter(|| {
                    // This will create and accumulate many effects
                    for _ in 0..10 {
                        let result = processor.process_hand_effects(&jokers, &mut game_context, &hand);
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
            let shop = Shop::new(&game);
            black_box(shop);
        });
    });
    
    group.bench_function("shop_with_jokers", |b| {
        let mut game = Game::default();
        game.start();
        
        // Add some jokers to the game state
        for joker_id in [JokerId::Joker, JokerId::GreedyJoker, JokerId::LustyJoker] {
            if let Ok(Some(definition)) = joker_registry::registry::get_definition(&joker_id) {
                let joker = definition.create();
                game.add_joker(joker).ok(); // Ignore errors for benchmark
            }
        }
        
        b.iter(|| {
            let shop = Shop::new(&game);
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
            if let Ok(Some(definition)) = joker_registry::registry::get_definition(&joker_id) {
                let joker = definition.create();
                game.add_joker(joker).ok(); // Ignore errors for benchmark
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
        let mut game_context = create_test_game_context();
        let hand = create_test_hand();
        
        b.iter(|| {
            // Process the same hand multiple times to benefit from caching
            for _ in 0..10 {
                let result = processor.process_hand_effects(&jokers, &mut game_context, &hand);
                black_box(result);
            }
        });
    });
    
    group.bench_function("without_cache", |b| {
        let jokers = create_test_jokers(5);
        let context = ProcessingContext::builder()
            .build();
        let mut processor = JokerEffectProcessor::with_context(context);
        // Disable cache
        let mut cache_config = balatro_rs::joker_effect_processor::CacheConfig::default();
        cache_config.enabled = false;
        processor.set_cache_config(cache_config);
        
        let mut game_context = create_test_game_context();
        let hand = create_test_hand();
        
        b.iter(|| {
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
                let mut processor = JokerEffectProcessor::new();
                let joker_effects = vec![
                    JokerEffect {
                        chips: 10,
                        mult: 2,
                        retrigger: retrigger_count,
                        ..Default::default()
                    }
                ];
                
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

fn create_test_game_context() -> GameContext<'static> {
    GameContext {
        chips: 100,
        mult: 4,
        money: 100,
        ante: 1,
        round: 1,
        stage: &Stage::PreBlind(),
        hands_played: 0,
        discards_used: 0,
        jokers: &[],
        hand: &balatro_rs::hand::Hand::new(vec![]),
        discarded: &[],
        joker_state_manager: &std::sync::Arc::new(balatro_rs::joker_state::JokerStateManager::new()),
        hand_type_counts: &HashMap::new(),
        cards_in_deck: 52,
        stone_cards_in_deck: 0,
        rng: &GameRng::for_testing(12345),
    }
}

fn create_test_hand() -> SelectHand {
    SelectHand {
        cards: vec![
            Card::new(Value::Ace, Suit::Spade),
            Card::new(Value::King, Suit::Spade),
            Card::new(Value::Queen, Suit::Spade),
            Card::new(Value::Jack, Suit::Spade),
            Card::new(Value::Ten, Suit::Spade),
        ],
    }
}

fn create_test_jokers(count: usize) -> Vec<Box<dyn Joker>> {
    let joker_ids = vec![JokerId::Joker, JokerId::GreedyJoker, JokerId::LustyJoker];
    let mut jokers = Vec::new();
    
    for i in 0..count {
        let joker_id = &joker_ids[i % joker_ids.len()];
        if let Ok(Some(definition)) = joker_registry::registry::get_definition(joker_id) {
            jokers.push(definition.create());
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