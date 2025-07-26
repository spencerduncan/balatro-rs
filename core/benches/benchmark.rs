use balatro_rs::{
    action::Action,
    card::{Card, Suit, Value},
    game::Game,
    hand::SelectHand,
    joker::GameContext,
    joker_effect_processor::{CacheConfig, JokerEffectProcessor},
    joker_state::JokerStateManager,
    rng::GameRng,
    stage::Stage,
};
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::collections::HashMap;

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
