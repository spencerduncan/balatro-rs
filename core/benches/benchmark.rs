use balatro_rs::{
    action::Action,
    card::{Card, Suit, Value},
    game::Game,
    hand::SelectHand,
    rng::GameRng,
};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("run game gen actions", |b| b.iter(run_game_gen_actions));
    c.bench_function("hand evaluation performance", |b| {
        b.iter(|| black_box(benchmark_hand_evaluation()))
    });
    c.bench_function("hand evaluation batch", |b| {
        b.iter(|| black_box(benchmark_hand_evaluation_batch()))
    });
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

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
