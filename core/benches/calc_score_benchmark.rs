use balatro_rs::card::{Card, Edition, Enhancement, Seal, Suit, Value};
use balatro_rs::config::Config;
use balatro_rs::game::Game;
use balatro_rs::hand::{MadeHand, SelectHand};
use balatro_rs::rank::HandRank;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::hint::black_box;

/// Create a simple pair hand for baseline testing
fn create_simple_pair() -> MadeHand {
    let cards = vec![
        Card::new(Value::King, Suit::Heart),
        Card::new(Value::King, Suit::Spade),
        Card::new(Value::Five, Suit::Diamond),
        Card::new(Value::Seven, Suit::Club),
        Card::new(Value::Two, Suit::Heart),
    ];

    MadeHand {
        rank: HandRank::OnePair,
        hand: SelectHand::new(cards.clone()),
        all: cards,
    }
}

/// Create a flush with mixed enhancements for medium complexity
fn create_enhanced_flush() -> MadeHand {
    let mut cards = vec![
        Card::new(Value::Ace, Suit::Heart),
        Card::new(Value::King, Suit::Heart),
        Card::new(Value::Queen, Suit::Heart),
        Card::new(Value::Jack, Suit::Heart),
        Card::new(Value::Ten, Suit::Heart),
    ];

    // Add various enhancements
    cards[0].enhancement = Some(Enhancement::Bonus); // +30 chips
    cards[1].enhancement = Some(Enhancement::Mult); // +4 mult
    cards[2].enhancement = Some(Enhancement::Steel); // x1.5 mult
    cards[3].enhancement = Some(Enhancement::Gold); // +$3
    cards[4].edition = Edition::Foil; // +50 chips

    MadeHand {
        rank: HandRank::Flush,
        hand: SelectHand::new(cards.clone()),
        all: cards,
    }
}

/// Create worst-case scenario: max cards with all enhancements active
fn create_worst_case_hand() -> MadeHand {
    let mut cards = vec![
        Card::new(Value::Ace, Suit::Heart),
        Card::new(Value::Ace, Suit::Spade),
        Card::new(Value::Ace, Suit::Diamond),
        Card::new(Value::King, Suit::Heart),
        Card::new(Value::King, Suit::Spade),
    ];

    // Apply all possible enhancements and editions
    cards[0].enhancement = Some(Enhancement::Glass); // x2 mult, 25% destroy
    cards[0].edition = Edition::Polychrome; // x1.5 score
    cards[0].seal = Some(Seal::Gold); // +$3

    cards[1].enhancement = Some(Enhancement::Steel); // x1.5 mult
    cards[1].edition = Edition::Holographic; // +10 mult
    cards[1].seal = Some(Seal::Red); // retrigger

    cards[2].enhancement = Some(Enhancement::Lucky); // RNG effects
    cards[2].edition = Edition::Foil; // +50 chips

    cards[3].enhancement = Some(Enhancement::Stone); // +50 chips, ignore rank
    cards[3].edition = Edition::Negative; // +1 joker slot

    cards[4].enhancement = Some(Enhancement::Wild); // any suit for flush

    MadeHand {
        rank: HandRank::FullHouse,
        hand: SelectHand::new(cards.clone()),
        all: cards,
    }
}

/// Benchmark calc_score with simple hand (baseline)
fn bench_calc_score_simple(c: &mut Criterion) {
    let mut group = c.benchmark_group("calc_score/simple");

    group.bench_function("pair", |b| {
        let config = Config::default();
        let hand = create_simple_pair();

        b.iter(|| {
            let mut game = Game::new(config.clone());
            black_box(game.calc_score(black_box(hand.clone())))
        });
    });

    group.finish();
}

/// Benchmark calc_score with enhanced cards (medium complexity)
fn bench_calc_score_enhanced(c: &mut Criterion) {
    let mut group = c.benchmark_group("calc_score/enhanced");

    group.bench_function("flush_with_enhancements", |b| {
        let config = Config::default();
        let hand = create_enhanced_flush();

        b.iter(|| {
            let mut game = Game::new(config.clone());
            black_box(game.calc_score(black_box(hand.clone())))
        });
    });

    group.finish();
}

/// Benchmark calc_score worst case (all enhancements)
fn bench_calc_score_worst_case(c: &mut Criterion) {
    let mut group = c.benchmark_group("calc_score/worst_case");

    group.bench_function("full_house_all_enhancements", |b| {
        let config = Config::default();
        let hand = create_worst_case_hand();

        b.iter(|| {
            let mut game = Game::new(config.clone());
            black_box(game.calc_score(black_box(hand.clone())))
        });
    });

    group.finish();
}

/// Benchmark simulating RL training hot path (many iterations)
fn bench_calc_score_rl_simulation(c: &mut Criterion) {
    let mut group = c.benchmark_group("calc_score/rl_training");

    for batch_size in [100, 1000, 10000].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(batch_size),
            batch_size,
            |b, &batch_size| {
                let config = Config::default();
                let hands = [
                    create_simple_pair(),
                    create_enhanced_flush(),
                    create_worst_case_hand(),
                ];

                b.iter(|| {
                    let mut game = Game::new(config.clone());
                    let mut total = 0.0;

                    for i in 0..batch_size {
                        let hand = &hands[i % hands.len()];
                        total += game.calc_score(black_box(hand.clone()));
                    }

                    black_box(total)
                });
            },
        );
    }

    group.finish();
}

/// Benchmark different enhancement combinations
fn bench_enhancement_combinations(c: &mut Criterion) {
    let mut group = c.benchmark_group("calc_score/enhancements");

    // Test with no enhancements
    group.bench_function("no_enhancements", |b| {
        let cards = vec![
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::Queen, Suit::Spade),
            Card::new(Value::Jack, Suit::Diamond),
            Card::new(Value::Ten, Suit::Club),
            Card::new(Value::Nine, Suit::Heart),
        ];
        let hand = MadeHand {
            rank: HandRank::HighCard,
            hand: SelectHand::new(cards.clone()),
            all: cards,
        };
        let config = Config::default();

        b.iter(|| {
            let mut game = Game::new(config.clone());
            black_box(game.calc_score(black_box(hand.clone())))
        });
    });

    // Test with mixed enhancements
    group.bench_function("mixed_enhancements", |b| {
        let mut cards = vec![
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::Queen, Suit::Spade),
            Card::new(Value::Jack, Suit::Diamond),
            Card::new(Value::Ten, Suit::Club),
            Card::new(Value::Nine, Suit::Heart),
        ];

        cards[0].enhancement = Some(Enhancement::Bonus);
        cards[1].enhancement = Some(Enhancement::Mult);
        cards[2].enhancement = Some(Enhancement::Steel);
        cards[3].enhancement = Some(Enhancement::Glass);
        cards[4].enhancement = Some(Enhancement::Lucky);

        let hand = MadeHand {
            rank: HandRank::HighCard,
            hand: SelectHand::new(cards.clone()),
            all: cards,
        };
        let config = Config::default();

        b.iter(|| {
            let mut game = Game::new(config.clone());
            black_box(game.calc_score(black_box(hand.clone())))
        });
    });

    // Test with all Glass cards (worst case for RNG)
    group.bench_function("all_glass", |b| {
        let mut cards = vec![
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::Queen, Suit::Spade),
            Card::new(Value::Jack, Suit::Diamond),
            Card::new(Value::Ten, Suit::Club),
            Card::new(Value::Nine, Suit::Heart),
        ];

        for card in cards.iter_mut() {
            card.enhancement = Some(Enhancement::Glass);
        }

        let hand = MadeHand {
            rank: HandRank::HighCard,
            hand: SelectHand::new(cards.clone()),
            all: cards,
        };
        let config = Config::default();

        b.iter(|| {
            let mut game = Game::new(config.clone());
            black_box(game.calc_score(black_box(hand.clone())))
        });
    });

    group.finish();
}

/// Benchmark memory allocations per call
fn bench_calc_score_memory(c: &mut Criterion) {
    let mut group = c.benchmark_group("calc_score/memory");
    group.significance_level(0.1).sample_size(500);

    group.bench_function("allocation_count", |b| {
        let config = Config::default();
        let hand = create_enhanced_flush();

        b.iter(|| {
            let mut game = Game::new(config.clone());
            // This should ideally have ZERO allocations in the hot path
            black_box(game.calc_score(black_box(hand.clone())))
        });
    });

    group.finish();
}

/// Compare inline vs function call overhead
fn bench_inline_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("calc_score/inline_test");

    // Test with process_enhancement_effects as a function call
    group.bench_function("with_function_call", |b| {
        let config = Config::default();
        let hand = create_enhanced_flush();

        b.iter(|| {
            let mut game = Game::new(config.clone());
            black_box(game.calc_score(black_box(hand.clone())))
        });
    });

    // Note: To test inline version, we'd need a version with #[inline(always)]
    // This would be added after measuring if there's significant overhead

    group.finish();
}

criterion_group!(
    benches,
    bench_calc_score_simple,
    bench_calc_score_enhanced,
    bench_calc_score_worst_case,
    bench_calc_score_rl_simulation,
    bench_enhancement_combinations,
    bench_calc_score_memory,
    bench_inline_comparison
);

criterion_main!(benches);
