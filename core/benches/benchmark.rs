#![allow(clippy::field_reassign_with_default)]

use balatro_rs::{
    action::Action,
    card::{Card, Suit, Value},
    game::Game,
    hand::SelectHand,
    joker::{GameContext, Joker, JokerEffect, JokerId, JokerRarity},
    joker_effect_processor::{CacheConfig, JokerEffectProcessor},
    joker_impl::{TheJoker, GreedyJoker},
    joker_state::JokerStateManager,
    rng::GameRng,
    stage::Stage,
    static_joker::{StaticJoker, StaticContext},
    static_joker_factory::StaticJokerFactory,
};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::collections::HashMap;
use std::hint::black_box;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("run game gen actions", |b| b.iter(run_game_gen_actions));
    c.bench_function("hand evaluation performance", |b| {
        b.iter(|| black_box(benchmark_hand_evaluation()))
    });
    c.bench_function("hand evaluation batch", |b| {
        b.iter(|| black_box(benchmark_hand_evaluation_batch()))
    });

    // JokerEffectProcessor cache benchmarks
    c.bench_function("joker effect processing with cache", |b| {
        b.iter(|| black_box(benchmark_joker_effects_with_cache()))
    });
    c.bench_function("joker effect processing without cache", |b| {
        b.iter(|| black_box(benchmark_joker_effects_without_cache()))
    });

    // Cache performance comparison with different scenarios
    let mut group = c.benchmark_group("cache_comparison");
    for iterations in [10, 50, 100, 500].iter() {
        group.bench_with_input(
            BenchmarkId::new("with_cache", iterations),
            iterations,
            |b, &iterations| b.iter(|| benchmark_cache_scenario(iterations, true)),
        );
        group.bench_with_input(
            BenchmarkId::new("without_cache", iterations),
            iterations,
            |b, &iterations| b.iter(|| benchmark_cache_scenario(iterations, false)),
        );
    }
    group.finish();

    // StaticJoker Performance Benchmarks - Critical for Phase 2 validation
    bench_static_joker_performance(c);
}

/// Comprehensive StaticJoker performance benchmarks to validate 3-5x performance claims
/// This is production-critical for Phase 2 StaticJoker implementation validation
fn bench_static_joker_performance(c: &mut Criterion) {
    // Per-card joker benchmarks (suit-based jokers like Greedy)
    let mut per_card_group = c.benchmark_group("static_vs_dynamic_per_card");

    for iterations in [100, 500, 1000, 5000].iter() {
        per_card_group.bench_with_input(
            BenchmarkId::new("dynamic_greedy", iterations),
            iterations,
            |b, &iterations| b.iter(|| benchmark_dynamic_per_card_processing(iterations)),
        );
        per_card_group.bench_with_input(
            BenchmarkId::new("framework_static_greedy", iterations),
            iterations,
            |b, &iterations| b.iter(|| benchmark_framework_static_per_card_processing(iterations)),
        );
        per_card_group.bench_with_input(
            BenchmarkId::new("trait_static_greedy", iterations),
            iterations,
            |b, &iterations| b.iter(|| benchmark_trait_static_per_card_processing(iterations)),
        );
    }
    per_card_group.finish();

    // Per-hand joker benchmarks (hand-type jokers like Jolly)
    let mut per_hand_group = c.benchmark_group("static_vs_dynamic_per_hand");

    for iterations in [100, 500, 1000, 5000].iter() {
        per_hand_group.bench_with_input(
            BenchmarkId::new("dynamic_basic", iterations),
            iterations,
            |b, &iterations| b.iter(|| benchmark_dynamic_per_hand_processing(iterations)),
        );
        per_hand_group.bench_with_input(
            BenchmarkId::new("framework_static_jolly", iterations),
            iterations,
            |b, &iterations| b.iter(|| benchmark_framework_static_per_hand_processing(iterations)),
        );
        per_hand_group.bench_with_input(
            BenchmarkId::new("trait_static_basic", iterations),
            iterations,
            |b, &iterations| b.iter(|| benchmark_trait_static_per_hand_processing(iterations)),
        );
    }
    per_hand_group.finish();

    // Complex condition evaluation benchmarks
    c.bench_function("complex_condition_dynamic", |b| {
        b.iter(|| black_box(benchmark_complex_condition_dynamic()))
    });
    c.bench_function("complex_condition_static", |b| {
        b.iter(|| black_box(benchmark_complex_condition_static()))
    });

    // Memory allocation benchmarks
    c.bench_function("memory_allocation_dynamic", |b| {
        b.iter(|| black_box(benchmark_memory_allocation_dynamic()))
    });
    c.bench_function("memory_allocation_static", |b| {
        b.iter(|| black_box(benchmark_memory_allocation_static()))
    });

    // Batch processing simulation (RL training scenario)
    let mut batch_group = c.benchmark_group("batch_processing_rl_simulation");

    for batch_size in [50, 100, 500].iter() {
        batch_group.bench_with_input(
            BenchmarkId::new("dynamic_batch", batch_size),
            batch_size,
            |b, &batch_size| b.iter(|| benchmark_dynamic_batch_processing(batch_size)),
        );
        batch_group.bench_with_input(
            BenchmarkId::new("static_batch", batch_size),
            batch_size,
            |b, &batch_size| b.iter(|| benchmark_static_batch_processing(batch_size)),
        );
    }
    batch_group.finish();
}

// =============================================================================
// STATIC JOKER PERFORMANCE BENCHMARK IMPLEMENTATIONS
// =============================================================================

/// Benchmark dynamic per-card processing (suit-based jokers like Greedy)
fn benchmark_dynamic_per_card_processing(iterations: u32) -> u64 {
    let mut operations = 0u64;
    let greedy_joker = GreedyJoker;

    // Test cards mix (some diamonds, some not)
    let test_cards = vec![
        Card::new(Value::Ace, Suit::Diamond),   // Should trigger
        Card::new(Value::King, Suit::Heart),    // Should not trigger
        Card::new(Value::Queen, Suit::Diamond), // Should trigger
        Card::new(Value::Jack, Suit::Spade),    // Should not trigger
        Card::new(Value::Ten, Suit::Diamond),   // Should trigger
    ];

    for _ in 0..iterations {
        for card in &test_cards {
            let mut mutable_context = create_benchmark_game_context();
            let _effect = greedy_joker.on_card_scored(&mut mutable_context, card);
            operations += 1;
        }
    }

    operations
}

/// Benchmark framework static per-card processing
fn benchmark_framework_static_per_card_processing(iterations: u32) -> u64 {
    let mut operations = 0u64;
    let static_greedy = StaticJokerFactory::create_greedy_joker();

    // Test cards mix (some diamonds, some not)
    let test_cards = vec![
        Card::new(Value::Ace, Suit::Diamond),   // Should trigger
        Card::new(Value::King, Suit::Heart),    // Should not trigger
        Card::new(Value::Queen, Suit::Diamond), // Should trigger
        Card::new(Value::Jack, Suit::Spade),    // Should not trigger
        Card::new(Value::Ten, Suit::Diamond),   // Should trigger
    ];

    for _ in 0..iterations {
        for card in &test_cards {
            let mut mutable_context = create_benchmark_game_context();
            let _effect = static_greedy.on_card_scored(&mut mutable_context, card);
            operations += 1;
        }
    }

    operations
}

/// Test StaticJoker trait implementation for benchmarking
#[derive(Debug)]
struct BenchmarkStaticJoker;

impl StaticJoker for BenchmarkStaticJoker {
    const ID: JokerId = JokerId::GreedyJoker;
    const NAME: &'static str = "Benchmark Static Joker";
    const DESCRIPTION: &'static str = "Diamond cards give +3 Mult";
    const RARITY: JokerRarity = JokerRarity::Common;
    const TRIGGERS_PER_CARD: bool = true;

    fn check_card_condition(&self, card: &Card, _context: &StaticContext) -> bool {
        card.suit == Suit::Diamond
    }

    fn check_hand_condition(&self, _hand: &SelectHand, _context: &StaticContext) -> bool {
        false // Per-card joker doesn't use hand conditions
    }

    fn calculate_effect(&self, _context: &StaticContext) -> JokerEffect {
        JokerEffect::new().with_mult(3)
    }
}

/// Test StaticJoker trait implementation for per-hand benchmarking
#[derive(Debug)]
struct BenchmarkHandStaticJoker;

impl StaticJoker for BenchmarkHandStaticJoker {
    const ID: JokerId = JokerId::Joker;
    const NAME: &'static str = "Benchmark Hand Static Joker";
    const DESCRIPTION: &'static str = "+4 Mult per hand";
    const RARITY: JokerRarity = JokerRarity::Common;
    const TRIGGERS_PER_CARD: bool = false;

    fn check_card_condition(&self, _card: &Card, _context: &StaticContext) -> bool {
        false // Hand-based joker doesn't check individual cards
    }

    fn check_hand_condition(&self, _hand: &SelectHand, _context: &StaticContext) -> bool {
        true // Always triggers
    }

    fn calculate_effect(&self, _context: &StaticContext) -> JokerEffect {
        JokerEffect::new().with_mult(4)
    }
}

/// Benchmark trait-based static per-card processing (pure StaticJoker trait)
fn benchmark_trait_static_per_card_processing(iterations: u32) -> u64 {
    let mut operations = 0u64;
    let trait_joker = BenchmarkStaticJoker;
    let dynamic_joker = trait_joker.to_dynamic();

    // Test cards mix (some diamonds, some not)
    let test_cards = vec![
        Card::new(Value::Ace, Suit::Diamond),   // Should trigger
        Card::new(Value::King, Suit::Heart),    // Should not trigger
        Card::new(Value::Queen, Suit::Diamond), // Should trigger
        Card::new(Value::Jack, Suit::Spade),    // Should not trigger
        Card::new(Value::Ten, Suit::Diamond),   // Should trigger
    ];

    for _ in 0..iterations {
        for card in &test_cards {
            let mut mutable_context = create_benchmark_game_context();
            let _effect = dynamic_joker.on_card_scored(&mut mutable_context, card);
            operations += 1;
        }
    }

    operations
}

/// Benchmark dynamic per-hand processing (basic joker like TheJoker)
fn benchmark_dynamic_per_hand_processing(iterations: u32) -> u64 {
    let mut operations = 0u64;
    let basic_joker = TheJoker;

    // Test hands of various types
    let test_hands = create_test_hands();

    for _ in 0..iterations {
        for hand_cards in &test_hands {
            let mut mutable_context = create_benchmark_game_context();
            let hand = SelectHand::new(hand_cards.clone());
            let _effect = basic_joker.on_hand_played(&mut mutable_context, &hand);
            operations += 1;
        }
    }

    operations
}

/// Benchmark framework static per-hand processing
fn benchmark_framework_static_per_hand_processing(iterations: u32) -> u64 {
    let mut operations = 0u64;
    let static_jolly = StaticJokerFactory::create_jolly_joker();

    // Test hands of various types
    let test_hands = create_test_hands();

    for _ in 0..iterations {
        for hand_cards in &test_hands {
            let mut mutable_context = create_benchmark_game_context();
            let hand = SelectHand::new(hand_cards.clone());
            let _effect = static_jolly.on_hand_played(&mut mutable_context, &hand);
            operations += 1;
        }
    }

    operations
}

/// Benchmark trait-based static per-hand processing
fn benchmark_trait_static_per_hand_processing(iterations: u32) -> u64 {
    let mut operations = 0u64;
    let trait_joker = BenchmarkHandStaticJoker;
    let dynamic_joker = trait_joker.to_dynamic();

    // Test hands of various types
    let test_hands = create_test_hands();

    for _ in 0..iterations {
        for hand_cards in &test_hands {
            let mut mutable_context = create_benchmark_game_context();
            let hand = SelectHand::new(hand_cards.clone());
            let _effect = dynamic_joker.on_hand_played(&mut mutable_context, &hand);
            operations += 1;
        }
    }

    operations
}

/// Benchmark complex condition evaluation - dynamic approach
fn benchmark_complex_condition_dynamic() -> u64 {
    let mut operations = 0u64;

    // Simulate complex joker with multiple conditions
    let greedy_joker = GreedyJoker;

    // Complex scenario: 1000 cards with mixed suits
    for suit in [Suit::Diamond, Suit::Heart, Suit::Spade, Suit::Club].iter().cycle().take(250) {
        for value in [Value::Ace, Value::King, Value::Queen, Value::Jack].iter() {
            let mut mutable_context = create_benchmark_game_context();
            let card = Card::new(*value, *suit);
            let _effect = greedy_joker.on_card_scored(&mut mutable_context, &card);
            operations += 1;
        }
    }

    operations
}

/// Benchmark complex condition evaluation - static approach
fn benchmark_complex_condition_static() -> u64 {
    let mut operations = 0u64;

    // Simulate complex static joker with multiple conditions
    let static_greedy = StaticJokerFactory::create_greedy_joker();

    // Complex scenario: 1000 cards with mixed suits
    for suit in [Suit::Diamond, Suit::Heart, Suit::Spade, Suit::Club].iter().cycle().take(250) {
        for value in [Value::Ace, Value::King, Value::Queen, Value::Jack].iter() {
            let mut mutable_context = create_benchmark_game_context();
            let card = Card::new(*value, *suit);
            let _effect = static_greedy.on_card_scored(&mut mutable_context, &card);
            operations += 1;
        }
    }

    operations
}

/// Benchmark memory allocation patterns - dynamic jokers
fn benchmark_memory_allocation_dynamic() -> u64 {
    let mut operations = 0u64;

    // Create many dynamic jokers (simulates allocation overhead)
    for _ in 0..1000 {
        let _greedy = GreedyJoker::default();
        let _basic = TheJoker::default();
        operations += 2;
    }

    operations
}

/// Benchmark memory allocation patterns - static jokers
fn benchmark_memory_allocation_static() -> u64 {
    let mut operations = 0u64;

    // Create many static jokers (should be more efficient)
    for _ in 0..1000 {
        let _trait_joker = BenchmarkStaticJoker;
        let _hand_joker = BenchmarkHandStaticJoker;
        operations += 2;
    }

    operations
}

/// Benchmark batch processing - dynamic jokers (RL training simulation)
fn benchmark_dynamic_batch_processing(batch_size: u32) -> u64 {
    let mut operations = 0u64;

    let dynamic_jokers: Vec<Box<dyn Joker>> = vec![
        Box::new(GreedyJoker::default()),
        Box::new(TheJoker::default()),
    ];

    let test_cards = create_benchmark_cards();
    let test_hands = create_test_hands();

    for _ in 0..batch_size {
        // Process cards with all jokers
        for joker in &dynamic_jokers {
            for card in &test_cards {
                let mut mutable_context = create_benchmark_game_context();
                let _effect = joker.on_card_scored(&mut mutable_context, card);
                operations += 1;
            }
        }

        // Process hands with all jokers
        for joker in &dynamic_jokers {
            for hand_cards in &test_hands {
                let mut mutable_context = create_benchmark_game_context();
                let hand = SelectHand::new(hand_cards.clone());
                let _effect = joker.on_hand_played(&mut mutable_context, &hand);
                operations += 1;
            }
        }
    }

    operations
}

/// Benchmark batch processing - static jokers (RL training simulation)
fn benchmark_static_batch_processing(batch_size: u32) -> u64 {
    let mut operations = 0u64;

    let static_jokers: Vec<Box<dyn Joker>> = vec![
        StaticJokerFactory::create_greedy_joker(),
        StaticJokerFactory::create_joker(),
    ];

    let test_cards = create_benchmark_cards();
    let test_hands = create_test_hands();

    for _ in 0..batch_size {
        // Process cards with all jokers
        for joker in &static_jokers {
            for card in &test_cards {
                let mut mutable_context = create_benchmark_game_context();
                let _effect = joker.on_card_scored(&mut mutable_context, card);
                operations += 1;
            }
        }

        // Process hands with all jokers
        for joker in &static_jokers {
            for hand_cards in &test_hands {
                let mut mutable_context = create_benchmark_game_context();
                let hand = SelectHand::new(hand_cards.clone());
                let _effect = joker.on_hand_played(&mut mutable_context, &hand);
                operations += 1;
            }
        }
    }

    operations
}

/// Create a reusable benchmark game context
fn create_benchmark_game_context() -> GameContext<'static> {
    let joker_state_manager = std::sync::Arc::new(JokerStateManager::new());
    let stage = Box::leak(Box::new(Stage::PreBlind()));
    let hand = Box::leak(Box::new(balatro_rs::hand::Hand::new(vec![])));
    let hand_type_counts = Box::leak(Box::new(HashMap::new()));
    let rng = Box::leak(Box::new(GameRng::for_testing(42)));
    let empty_cards: Vec<Card> = Vec::new();
    let discarded = Box::leak(empty_cards.into_boxed_slice());
    let jokers: Vec<Box<dyn Joker>> = Vec::new();
    let jokers_ref = Box::leak(jokers.into_boxed_slice());
    let joker_state_manager_ref = Box::leak(Box::new(joker_state_manager));

    GameContext {
        chips: 100,
        mult: 4,
        money: 100,
        ante: 1,
        round: 1,
        stage,
        hands_played: 0,
        discards_used: 0,
        jokers: jokers_ref,
        hand,
        discarded,
        joker_state_manager: joker_state_manager_ref,
        hand_type_counts,
        cards_in_deck: 52,
        stone_cards_in_deck: 0,
        steel_cards_in_deck: 0,
        rng,
    }
}

fn run_game_gen_actions() {
    let mut g = Game::default();
    let bench_rng = GameRng::for_testing(999); // Use deterministic RNG for consistent benchmarks

    g.start();
    while !g.is_over() {
        // Get all available moves
        let actions: Vec<Action> = g.gen_actions().collect();
        if actions.is_empty() {
            break;
        }

        // Pick a random move and execute it using deterministic RNG
        let i = bench_rng.gen_range(0..actions.len());
        let action = actions[i].clone();
        let action_res = g.handle_action(action.clone());
        debug_assert!(action_res.is_ok());
    }
}

/// Benchmark hand evaluation with various hand types for performance testing
fn benchmark_hand_evaluation() -> u32 {
    let mut evaluations = 0u32;

    // Test various hand types for comprehensive performance measurement
    let test_hands = create_test_hands();

    for hand in test_hands {
        let select_hand = SelectHand::new(hand);
        let _result = select_hand.best_hand();
        evaluations += 1;
    }

    evaluations
}

/// Benchmark batch hand evaluation (1000 hands) to test performance at scale
fn benchmark_hand_evaluation_batch() -> u32 {
    let mut evaluations = 0u32;
    let test_hands = create_test_hands();

    // Evaluate 1000 hands to simulate RL training scenario
    for _ in 0..1000 {
        for hand in &test_hands {
            let select_hand = SelectHand::new(hand.clone());
            let _result = select_hand.best_hand();
            evaluations += 1;
        }
    }

    evaluations
}

/// Create a comprehensive set of test hands covering all hand types
fn create_test_hands() -> Vec<Vec<Card>> {
    vec![
        // Royal Flush
        vec![
            Card::new(Value::Ten, Suit::Spade),
            Card::new(Value::Jack, Suit::Spade),
            Card::new(Value::Queen, Suit::Spade),
            Card::new(Value::King, Suit::Spade),
            Card::new(Value::Ace, Suit::Spade),
        ],
        // Straight Flush
        vec![
            Card::new(Value::Five, Suit::Heart),
            Card::new(Value::Six, Suit::Heart),
            Card::new(Value::Seven, Suit::Heart),
            Card::new(Value::Eight, Suit::Heart),
            Card::new(Value::Nine, Suit::Heart),
        ],
        // Four of a Kind
        vec![
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::King, Suit::Spade),
            Card::new(Value::King, Suit::Club),
            Card::new(Value::King, Suit::Diamond),
            Card::new(Value::Two, Suit::Heart),
        ],
        // Full House
        vec![
            Card::new(Value::Queen, Suit::Heart),
            Card::new(Value::Queen, Suit::Spade),
            Card::new(Value::Queen, Suit::Club),
            Card::new(Value::Jack, Suit::Heart),
            Card::new(Value::Jack, Suit::Spade),
        ],
        // Flush
        vec![
            Card::new(Value::Two, Suit::Diamond),
            Card::new(Value::Four, Suit::Diamond),
            Card::new(Value::Six, Suit::Diamond),
            Card::new(Value::Eight, Suit::Diamond),
            Card::new(Value::Ten, Suit::Diamond),
        ],
        // Straight
        vec![
            Card::new(Value::Five, Suit::Heart),
            Card::new(Value::Six, Suit::Spade),
            Card::new(Value::Seven, Suit::Club),
            Card::new(Value::Eight, Suit::Diamond),
            Card::new(Value::Nine, Suit::Heart),
        ],
        // Three of a Kind
        vec![
            Card::new(Value::Seven, Suit::Heart),
            Card::new(Value::Seven, Suit::Spade),
            Card::new(Value::Seven, Suit::Club),
            Card::new(Value::Two, Suit::Heart),
            Card::new(Value::Five, Suit::Spade),
        ],
        // Two Pair
        vec![
            Card::new(Value::Jack, Suit::Heart),
            Card::new(Value::Jack, Suit::Spade),
            Card::new(Value::Three, Suit::Club),
            Card::new(Value::Three, Suit::Heart),
            Card::new(Value::Nine, Suit::Spade),
        ],
        // One Pair
        vec![
            Card::new(Value::Eight, Suit::Heart),
            Card::new(Value::Eight, Suit::Spade),
            Card::new(Value::Two, Suit::Club),
            Card::new(Value::Five, Suit::Heart),
            Card::new(Value::King, Suit::Spade),
        ],
        // High Card
        vec![
            Card::new(Value::Ace, Suit::Heart),
            Card::new(Value::Three, Suit::Spade),
            Card::new(Value::Five, Suit::Club),
            Card::new(Value::Seven, Suit::Heart),
            Card::new(Value::Nine, Suit::Diamond),
        ],
        // Balatro special hands - Flush Five
        vec![
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::King, Suit::Heart),
        ],
        // Balatro special hands - Flush House
        vec![
            Card::new(Value::Ace, Suit::Club),
            Card::new(Value::Ace, Suit::Club),
            Card::new(Value::Ace, Suit::Club),
            Card::new(Value::Two, Suit::Club),
            Card::new(Value::Two, Suit::Club),
        ],
    ]
}

/// Benchmark JokerEffectProcessor with cache enabled
fn benchmark_joker_effects_with_cache() -> u64 {
    let mut processor = JokerEffectProcessor::new();

    // Enable aggressive caching for benchmark
    let mut cache_config = CacheConfig::default();
    cache_config.max_entries = 10000;
    cache_config.ttl_seconds = 3600; // 1 hour
    cache_config.enabled = true;
    processor.set_cache_config(cache_config);

    benchmark_joker_processing(&mut processor, 100)
}

/// Benchmark JokerEffectProcessor with cache disabled
fn benchmark_joker_effects_without_cache() -> u64 {
    let mut processor = JokerEffectProcessor::new();

    // Disable caching
    let mut cache_config = CacheConfig::default();
    cache_config.enabled = false;
    processor.set_cache_config(cache_config);

    benchmark_joker_processing(&mut processor, 100)
}

/// Benchmark cache performance with different iteration counts
fn benchmark_cache_scenario(iterations: u32, cache_enabled: bool) -> u64 {
    let mut processor = JokerEffectProcessor::new();

    let mut cache_config = CacheConfig::default();
    cache_config.enabled = cache_enabled;
    if cache_enabled {
        cache_config.max_entries = 10000;
        cache_config.ttl_seconds = 3600;
    }
    processor.set_cache_config(cache_config);

    benchmark_joker_processing(&mut processor, iterations)
}

/// Core benchmark logic for joker effect processing
fn benchmark_joker_processing(processor: &mut JokerEffectProcessor, iterations: u32) -> u64 {
    let mut operations = 0u64;

    // Create realistic game context
    let joker_state_manager = std::sync::Arc::new(JokerStateManager::new());
    let stage = Stage::PreBlind();
    let hand = balatro_rs::hand::Hand::new(vec![]);
    let hand_type_counts = HashMap::new();
    let rng = GameRng::for_testing(42);

    let mut game_context = GameContext {
        chips: 100,
        mult: 4,
        money: 100,
        ante: 1,
        round: 1,
        stage: &stage,
        hands_played: 0,
        discards_used: 0,
        jokers: &[],
        hand: &hand,
        discarded: &[],
        joker_state_manager: &joker_state_manager,
        hand_type_counts: &hand_type_counts,
        cards_in_deck: 52,
        stone_cards_in_deck: 0,
        steel_cards_in_deck: 0,
        rng: &rng,
    };

    // Create test hands and cards
    let test_hands = create_benchmark_hands();
    let test_cards = create_benchmark_cards();
    let jokers: Vec<Box<dyn balatro_rs::joker::Joker>> = vec![];

    // Simulate realistic RL training scenario with repeated processing
    for _ in 0..iterations {
        // Process each hand multiple times (simulating repeated game states)
        for hand in &test_hands {
            let select_hand = SelectHand::new(hand.clone());
            let _result = processor.process_hand_effects(&jokers, &mut game_context, &select_hand);
            operations += 1;
        }

        // Process each card multiple times
        for card in &test_cards {
            let _result = processor.process_card_effects(&jokers, &mut game_context, card);
            operations += 1;
        }

        // Slightly modify context to create variety while maintaining cache hits
        game_context.hands_played = (game_context.hands_played + 1) % 5;
        game_context.money = 100 + (operations % 50) as i32;
    }

    operations
}

/// Create hands specifically designed for cache benchmarking
fn create_benchmark_hands() -> Vec<Vec<Card>> {
    vec![
        // High-value hands that would commonly occur in RL training
        vec![
            Card::new(Value::Ace, Suit::Heart),
            Card::new(Value::Ace, Suit::Spade),
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::King, Suit::Spade),
            Card::new(Value::Queen, Suit::Heart),
        ],
        vec![
            Card::new(Value::Ten, Suit::Club),
            Card::new(Value::Jack, Suit::Club),
            Card::new(Value::Queen, Suit::Club),
            Card::new(Value::King, Suit::Club),
            Card::new(Value::Ace, Suit::Club),
        ],
        vec![
            Card::new(Value::Seven, Suit::Diamond),
            Card::new(Value::Seven, Suit::Heart),
            Card::new(Value::Seven, Suit::Spade),
            Card::new(Value::Two, Suit::Club),
            Card::new(Value::Three, Suit::Heart),
        ],
        vec![
            Card::new(Value::Four, Suit::Heart),
            Card::new(Value::Five, Suit::Heart),
            Card::new(Value::Six, Suit::Heart),
            Card::new(Value::Seven, Suit::Heart),
            Card::new(Value::Eight, Suit::Heart),
        ],
    ]
}

/// Create cards specifically for cache benchmarking
fn create_benchmark_cards() -> Vec<Card> {
    vec![
        Card::new(Value::Ace, Suit::Heart),
        Card::new(Value::King, Suit::Spade),
        Card::new(Value::Queen, Suit::Diamond),
        Card::new(Value::Jack, Suit::Club),
        Card::new(Value::Ten, Suit::Heart),
        Card::new(Value::Nine, Suit::Spade),
        Card::new(Value::Eight, Suit::Diamond),
        Card::new(Value::Seven, Suit::Club),
    ]
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
