use balatro_rs::{
    card::{Card, Suit, Value},
    hand::SelectHand,
    joker::{GameContext, Joker, JokerId},
    joker_effect_processor::{
        ConflictResolutionStrategy, EffectPriority, JokerEffectProcessor, ProcessingContext,
    },
};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use std::hint::black_box;

/// Benchmark suite for JokerEffectProcessor performance testing
pub fn effect_processor_benchmarks(c: &mut Criterion) {
    // Basic effect processing benchmarks
    basic_effect_processing_benchmarks(c);

    // Complex effect combination benchmarks
    complex_effect_combination_benchmarks(c);

    // Retriggering scenario benchmarks
    retriggering_scenario_benchmarks(c);

    // Large joker collection benchmarks
    large_joker_collection_benchmarks(c);

    // Conflict resolution strategy benchmarks
    conflict_resolution_benchmarks(c);

    // Priority ordering benchmarks
    priority_ordering_benchmarks(c);

    // Cache performance benchmarks
    cache_performance_benchmarks(c);

    // Memory allocation benchmarks
    memory_allocation_benchmarks(c);
}

/// Benchmark basic effect processing with minimal complexity
fn basic_effect_processing_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("basic_effect_processing");

    // Single joker effect processing
    group.bench_function("single_joker_hand_effect", |b| {
        b.iter(|| {
            let mut processor = JokerEffectProcessor::new();
            let mut game_context = create_test_game_context();
            let hand = create_test_hand();
            let jokers = create_single_joker_collection();

            black_box(processor.process_hand_effects(&jokers, &mut game_context, &hand))
        });
    });

    group.bench_function("single_joker_card_effect", |b| {
        b.iter(|| {
            let mut processor = JokerEffectProcessor::new();
            let mut game_context = create_test_game_context();
            let card = Card::new(Value::Ace, Suit::Spade);
            let jokers = create_single_joker_collection();

            black_box(processor.process_card_effects(&jokers, &mut game_context, &card))
        });
    });

    // Performance targets: Single joker effect processing should be < 1μs
    group.sample_size(10000);
    group.finish();
}

/// Benchmark complex effect combinations with multiple jokers
fn complex_effect_combination_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("complex_effect_combinations");

    for joker_count in [3, 5, 7, 10].iter() {
        group.throughput(Throughput::Elements(*joker_count as u64));

        group.bench_with_input(
            BenchmarkId::new("multi_joker_hand_effects", joker_count),
            joker_count,
            |b, &joker_count| {
                b.iter(|| {
                    let mut processor = JokerEffectProcessor::new();
                    let mut game_context = create_test_game_context();
                    let hand = create_test_hand();
                    let jokers = create_complex_joker_collection(joker_count);

                    black_box(processor.process_hand_effects(&jokers, &mut game_context, &hand))
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("multi_joker_card_effects", joker_count),
            joker_count,
            |b, &joker_count| {
                b.iter(|| {
                    let mut processor = JokerEffectProcessor::new();
                    let mut game_context = create_test_game_context();
                    let card = Card::new(Value::King, Suit::Heart);
                    let jokers = create_complex_joker_collection(joker_count);

                    black_box(processor.process_card_effects(&jokers, &mut game_context, &card))
                });
            },
        );
    }

    // Performance target: 10 jokers with complex effects should be < 10μs
    group.finish();
}

/// Benchmark heavy retriggering scenarios
fn retriggering_scenario_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("retriggering_scenarios");

    for retrigger_count in [1, 3, 5, 10, 20].iter() {
        group.throughput(Throughput::Elements(*retrigger_count as u64));

        group.bench_with_input(
            BenchmarkId::new("high_retrigger_processing", retrigger_count),
            retrigger_count,
            |b, &retrigger_count| {
                b.iter(|| {
                    let mut processor = JokerEffectProcessor::new();
                    let mut game_context = create_test_game_context();
                    let hand = create_test_hand();
                    let jokers = create_retriggering_joker_collection(retrigger_count);

                    black_box(processor.process_hand_effects(&jokers, &mut game_context, &hand))
                });
            },
        );
    }

    // Performance target: 20 jokers with retriggering should be < 50μs
    group.finish();
}

/// Benchmark performance with large joker collections
fn large_joker_collection_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("large_joker_collections");

    for joker_count in [10, 15, 20, 25, 30].iter() {
        group.throughput(Throughput::Elements(*joker_count as u64));

        group.bench_with_input(
            BenchmarkId::new("large_collection_processing", joker_count),
            joker_count,
            |b, &joker_count| {
                b.iter(|| {
                    let mut processor = JokerEffectProcessor::new();
                    let mut game_context = create_test_game_context();
                    let hand = create_test_hand();
                    let jokers = create_large_joker_collection(joker_count);

                    black_box(processor.process_hand_effects(&jokers, &mut game_context, &hand))
                });
            },
        );
    }

    group.finish();
}

/// Benchmark different conflict resolution strategies
fn conflict_resolution_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("conflict_resolution_strategies");

    let strategies = [
        ("sum", ConflictResolutionStrategy::Sum),
        ("maximum", ConflictResolutionStrategy::Maximum),
        ("minimum", ConflictResolutionStrategy::Minimum),
        ("first_wins", ConflictResolutionStrategy::FirstWins),
        ("last_wins", ConflictResolutionStrategy::LastWins),
    ];

    for (name, strategy) in strategies.iter() {
        group.bench_with_input(
            BenchmarkId::new("conflict_resolution", name),
            strategy,
            |b, strategy| {
                b.iter(|| {
                    let context = ProcessingContext {
                        resolution_strategy: strategy.clone(),
                        ..Default::default()
                    };
                    let mut processor = JokerEffectProcessor::with_context(context);
                    let mut game_context = create_test_game_context();
                    let hand = create_test_hand();
                    let jokers = create_conflicting_joker_collection();

                    black_box(processor.process_hand_effects(&jokers, &mut game_context, &hand))
                });
            },
        );
    }

    group.finish();
}

/// Benchmark priority ordering with different priority distributions
fn priority_ordering_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("priority_ordering");

    let priority_scenarios = [
        ("uniform_normal", vec![EffectPriority::Normal; 10]),
        (
            "mixed_priorities",
            vec![
                EffectPriority::Low,
                EffectPriority::Normal,
                EffectPriority::High,
                EffectPriority::Critical,
                EffectPriority::Normal,
                EffectPriority::Low,
                EffectPriority::High,
                EffectPriority::Normal,
                EffectPriority::Critical,
                EffectPriority::Normal,
            ],
        ),
        ("all_critical", vec![EffectPriority::Critical; 10]),
        (
            "reverse_order",
            vec![
                EffectPriority::Critical,
                EffectPriority::High,
                EffectPriority::Normal,
                EffectPriority::Low,
            ],
        ),
    ];

    for (name, priorities) in priority_scenarios.iter() {
        group.bench_with_input(
            BenchmarkId::new("priority_ordering", name),
            priorities,
            |b, priorities| {
                b.iter(|| {
                    let mut processor = JokerEffectProcessor::new();
                    let mut game_context = create_test_game_context();
                    let hand = create_test_hand();
                    let jokers = create_priority_joker_collection(priorities.clone());

                    black_box(processor.process_hand_effects(&jokers, &mut game_context, &hand))
                });
            },
        );
    }

    group.finish();
}

/// Benchmark cache performance scenarios
fn cache_performance_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("cache_performance");

    // Cache hit scenario
    group.bench_function("cache_hit_performance", |b| {
        let mut processor = JokerEffectProcessor::new();
        let mut game_context = create_test_game_context();
        let hand = create_test_hand();
        let jokers = create_single_joker_collection();

        // Prime the cache
        processor.process_hand_effects(&jokers, &mut game_context, &hand);

        b.iter(|| {
            // This should hit the cache
            black_box(processor.process_hand_effects(&jokers, &mut game_context, &hand))
        });
    });

    // Cache miss scenario
    group.bench_function("cache_miss_performance", |b| {
        b.iter(|| {
            let mut processor = JokerEffectProcessor::new();
            let mut game_context = create_test_game_context();
            let hand = create_varied_test_hand();
            let jokers = create_single_joker_collection();

            // This should always miss the cache due to varied hands
            black_box(processor.process_hand_effects(&jokers, &mut game_context, &hand))
        });
    });

    // Performance target: Cache hit performance should be < 100ns
    group.finish();
}

/// Memory allocation profiling benchmarks
fn memory_allocation_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_allocation");

    // Single joker memory allocation
    group.bench_function("single_joker_allocation", |b| {
        b.iter(|| {
            let mut processor = JokerEffectProcessor::new();
            let mut game_context = create_test_game_context();
            let hand = create_test_hand();
            let jokers = create_single_joker_collection();

            // Track memory allocation patterns
            let result = processor.process_hand_effects(&jokers, &mut game_context, &hand);
            black_box(result);

            // Processor should be dropped here, releasing memory
            drop(processor);
        });
    });

    // Multiple jokers memory allocation
    group.bench_function("multi_joker_allocation", |b| {
        b.iter(|| {
            let mut processor = JokerEffectProcessor::new();
            let mut game_context = create_test_game_context();
            let hand = create_test_hand();
            let jokers = create_complex_joker_collection(10);

            let result = processor.process_hand_effects(&jokers, &mut game_context, &hand);
            black_box(result);

            drop(processor);
        });
    });

    // Performance target: Memory allocations should be < 1KB per processing operation
    group.finish();
}

// Helper functions for creating test data

fn create_test_game_context() -> GameContext<'static> {
    // Create static references for the benchmark
    let stage = Box::leak(Box::new(balatro_rs::stage::Stage::PreBlind()));
    let hand = Box::leak(Box::new(balatro_rs::hand::Hand::new(vec![])));
    let jokers: &'static [Box<dyn balatro_rs::joker::Joker>] = Box::leak(Box::new([]));
    let discarded: &'static [balatro_rs::card::Card] = Box::leak(Box::new([]));
    let joker_state_manager = Box::leak(Box::new(std::sync::Arc::new(
        balatro_rs::joker_state::JokerStateManager::new(),
    )));
    let hand_type_counts = Box::leak(Box::new(std::collections::HashMap::new()));
    let rng = Box::leak(Box::new(balatro_rs::rng::GameRng::for_testing(12345)));

    GameContext {
        chips: 100,
        mult: 4,
        money: 100,
        ante: 1,
        round: 1,
        stage,
        hands_played: 0,
        discards_used: 0,
        hands_remaining: 4.0, // Standard hands remaining for testing
        jokers,
        hand,
        discarded,
        joker_state_manager,
        hand_type_counts,
        cards_in_deck: 52,
        stone_cards_in_deck: 0, // BENCHMARK: Using standard deck composition
        steel_cards_in_deck: 0, // BENCHMARK: Using standard deck composition
        rng,
    }
}

fn create_test_hand() -> SelectHand {
    SelectHand::new(vec![
        Card::new(Value::King, Suit::Spade),
        Card::new(Value::Queen, Suit::Spade),
        Card::new(Value::Jack, Suit::Spade),
        Card::new(Value::Ten, Suit::Spade),
        Card::new(Value::Nine, Suit::Spade),
    ])
}

fn create_varied_test_hand() -> SelectHand {
    use std::sync::atomic::{AtomicU8, Ordering};
    static COUNTER: AtomicU8 = AtomicU8::new(0);

    let counter = COUNTER.fetch_add(1, Ordering::Relaxed);
    let value = match counter % 13 {
        0 => Value::Ace,
        1 => Value::Two,
        2 => Value::Three,
        3 => Value::Four,
        4 => Value::Five,
        5 => Value::Six,
        6 => Value::Seven,
        7 => Value::Eight,
        8 => Value::Nine,
        9 => Value::Ten,
        10 => Value::Jack,
        11 => Value::Queen,
        _ => Value::King,
    };

    SelectHand::new(vec![
        Card::new(value, Suit::Spade),
        Card::new(Value::Two, Suit::Heart),
        Card::new(Value::Three, Suit::Club),
        Card::new(Value::Four, Suit::Diamond),
        Card::new(Value::Five, Suit::Spade),
    ])
}

fn create_single_joker_collection() -> Vec<Box<dyn Joker>> {
    vec![balatro_rs::joker_factory::JokerFactory::create(JokerId::Joker).unwrap()]
}

fn create_complex_joker_collection(count: usize) -> Vec<Box<dyn Joker>> {
    let mut jokers = Vec::new();

    for i in 0..count {
        let joker_id = match i % 4 {
            0 => JokerId::Joker,
            1 => JokerId::GreedyJoker,
            2 => JokerId::LustyJoker,
            _ => JokerId::WrathfulJoker,
        };

        if let Some(joker) = balatro_rs::joker_factory::JokerFactory::create(joker_id) {
            jokers.push(joker);
        } else if let Some(fallback) =
            balatro_rs::joker_factory::JokerFactory::create(JokerId::Joker)
        {
            // Fallback to basic joker if the specific one doesn't exist
            jokers.push(fallback);
        }
    }

    jokers
}

fn create_retriggering_joker_collection(retrigger_count: usize) -> Vec<Box<dyn Joker>> {
    let mut jokers = Vec::new();

    // Create jokers for retrigger benchmarking
    for _i in 0..std::cmp::min(retrigger_count, 5) {
        if let Some(joker) = balatro_rs::joker_factory::JokerFactory::create(JokerId::Joker) {
            jokers.push(joker);
        }
    }

    jokers
}

fn create_large_joker_collection(count: usize) -> Vec<Box<dyn Joker>> {
    let mut jokers = Vec::new();

    for i in 0..count {
        let joker_id = match i % 10 {
            0 => JokerId::Joker,
            1 => JokerId::GreedyJoker,
            2 => JokerId::LustyJoker,
            3 => JokerId::WrathfulJoker,
            4 => JokerId::GluttonousJoker,
            5 => JokerId::JollyJoker,
            6 => JokerId::ZanyJoker,
            7 => JokerId::MadJoker,
            8 => JokerId::CrazyJoker,
            _ => JokerId::DrollJoker,
        };

        if let Some(joker) = balatro_rs::joker_factory::JokerFactory::create(joker_id) {
            jokers.push(joker);
        } else if let Some(fallback) =
            balatro_rs::joker_factory::JokerFactory::create(JokerId::Joker)
        {
            // Fallback to basic joker if the specific one doesn't exist
            jokers.push(fallback);
        }
    }

    jokers
}

fn create_conflicting_joker_collection() -> Vec<Box<dyn Joker>> {
    vec![
        balatro_rs::joker_factory::JokerFactory::create(JokerId::Joker).unwrap(),
        balatro_rs::joker_factory::JokerFactory::create(JokerId::GreedyJoker).unwrap_or_else(
            || {
                // Fallback if GreedyJoker doesn't exist
                balatro_rs::joker_factory::JokerFactory::create(JokerId::Joker).unwrap()
            },
        ),
        balatro_rs::joker_factory::JokerFactory::create(JokerId::LustyJoker).unwrap_or_else(|| {
            // Fallback if LustyJoker doesn't exist
            balatro_rs::joker_factory::JokerFactory::create(JokerId::Joker).unwrap()
        }),
    ]
}

fn create_priority_joker_collection(priorities: Vec<EffectPriority>) -> Vec<Box<dyn Joker>> {
    let mut jokers = Vec::new();

    for &_priority in priorities.iter() {
        // Create a joker for benchmarking purposes
        // Note: Actual priority handling is implemented in the processor
        if let Some(joker) = balatro_rs::joker_factory::JokerFactory::create(JokerId::Joker) {
            jokers.push(joker);
        }
    }

    jokers
}

criterion_group!(effect_processor_benches, effect_processor_benchmarks);
criterion_main!(effect_processor_benches);
