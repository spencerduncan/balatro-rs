use balatro_rs::{
    card::{Card, Suit, Value},
    hand::SelectHand,
    joker::{GameContext, Joker, JokerEffect, JokerId},
    joker_effect_processor::{ConflictResolutionStrategy, JokerEffectProcessor, ProcessingContext},
};
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

/// Minimal benchmark focusing on public JokerEffectProcessor API
pub fn effect_processor_benchmarks(c: &mut Criterion) {
    hand_effects_benchmarks(c);
    card_effects_benchmarks(c);
    conflict_resolution_benchmarks(c);
    scaling_benchmarks(c);
}

/// Test hand effects processing performance  
fn hand_effects_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("hand_effects");

    group.bench_function("empty_joker_collection", |b| {
        b.iter(|| {
            let mut processor = JokerEffectProcessor::new();
            let mut game_context = GameContext::new();
            let hand = create_test_hand();
            let jokers: Vec<Box<dyn Joker>> = Vec::new();

            black_box(processor.process_hand_effects(&jokers, &mut game_context, &hand))
        });
    });

    group.bench_function("single_test_joker", |b| {
        b.iter(|| {
            let mut processor = JokerEffectProcessor::new();
            let mut game_context = GameContext::new();
            let hand = create_test_hand();
            let jokers: Vec<Box<dyn Joker>> = vec![Box::new(create_test_joker())];

            black_box(processor.process_hand_effects(&jokers, &mut game_context, &hand))
        });
    });

    // Performance target: Single joker effect processing should be < 1μs
    group.sample_size(10000);
    group.finish();
}

/// Test card effects processing performance
fn card_effects_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("card_effects");

    group.bench_function("single_card_empty_jokers", |b| {
        b.iter(|| {
            let mut processor = JokerEffectProcessor::new();
            let mut game_context = GameContext::new();
            let card = Card::new(Value::Ace, Suit::Spade);
            let jokers: Vec<Box<dyn Joker>> = Vec::new();

            black_box(processor.process_card_effects(&jokers, &mut game_context, &card))
        });
    });

    group.bench_function("single_card_single_joker", |b| {
        b.iter(|| {
            let mut processor = JokerEffectProcessor::new();
            let mut game_context = GameContext::new();
            let card = Card::new(Value::King, Suit::Heart);
            let jokers: Vec<Box<dyn Joker>> = vec![Box::new(create_test_joker())];

            black_box(processor.process_card_effects(&jokers, &mut game_context, &card))
        });
    });

    group.finish();
}

/// Benchmark different conflict resolution strategies with test jokers
fn conflict_resolution_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("conflict_resolution");

    let strategies = [
        ("sum", ConflictResolutionStrategy::Sum),
        ("maximum", ConflictResolutionStrategy::Maximum),
        ("minimum", ConflictResolutionStrategy::Minimum),
        ("first_wins", ConflictResolutionStrategy::FirstWins),
        ("last_wins", ConflictResolutionStrategy::LastWins),
    ];

    for (name, strategy) in strategies.iter() {
        group.bench_with_input(
            BenchmarkId::new("strategy", name),
            strategy,
            |b, strategy| {
                b.iter(|| {
                    let mut context = ProcessingContext::default();
                    context.conflict_resolution = strategy.clone();
                    let mut processor = JokerEffectProcessor::with_context(context);
                    let mut game_context = GameContext::new();
                    let hand = create_test_hand();

                    // Create multiple test jokers that would create conflicts
                    let jokers: Vec<Box<dyn Joker>> = vec![
                        Box::new(create_test_joker_with_effect(10, 2)),
                        Box::new(create_test_joker_with_effect(15, 3)),
                        Box::new(create_test_joker_with_effect(8, 4)),
                    ];

                    black_box(processor.process_hand_effects(&jokers, &mut game_context, &hand))
                });
            },
        );
    }

    group.finish();
}

/// Test scaling performance with different numbers of jokers
fn scaling_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("scaling");

    for joker_count in [1, 3, 5, 10, 15, 20].iter() {
        group.throughput(Throughput::Elements(*joker_count as u64));

        group.bench_with_input(
            BenchmarkId::new("joker_count", joker_count),
            joker_count,
            |b, &joker_count| {
                b.iter(|| {
                    let mut processor = JokerEffectProcessor::new();
                    let mut game_context = GameContext::new();
                    let hand = create_test_hand();
                    let jokers = create_test_joker_collection(joker_count);

                    black_box(processor.process_hand_effects(&jokers, &mut game_context, &hand))
                });
            },
        );
    }

    // Performance targets:
    // - 10 jokers should be < 10μs
    // - 20 jokers should be < 50μs
    group.finish();
}

// Helper functions for creating test data

fn create_test_hand() -> SelectHand {
    SelectHand::new(vec![
        Card::new(Value::King, Suit::Spade),
        Card::new(Value::Queen, Suit::Spade),
        Card::new(Value::Jack, Suit::Spade),
        Card::new(Value::Ten, Suit::Spade),
        Card::new(Value::Nine, Suit::Spade),
    ])
}

fn create_test_joker() -> TestJoker {
    TestJoker {
        id: JokerId::Joker,
        chips: 4,
        mult: 0,
    }
}

fn create_test_joker_with_effect(chips: i32, mult: i32) -> TestJoker {
    TestJoker {
        id: JokerId::Joker,
        chips,
        mult,
    }
}

fn create_test_joker_collection(count: usize) -> Vec<Box<dyn Joker>> {
    let mut jokers = Vec::new();

    for i in 0..count {
        let chips = (i % 10) as i32 + 1;
        let mult = i as i32;
        jokers.push(Box::new(create_test_joker_with_effect(chips, mult)));
    }

    jokers
}

/// Simple test joker implementation for benchmarking
#[derive(Debug, Clone)]
struct TestJoker {
    id: JokerId,
    chips: i32,
    mult: i32,
}

impl Joker for TestJoker {
    fn id(&self) -> JokerId {
        self.id
    }

    fn name(&self) -> &str {
        "Test Joker"
    }

    fn description(&self) -> &str {
        "A simple test joker for benchmarking"
    }

    fn rarity(&self) -> balatro_rs::joker::JokerRarity {
        balatro_rs::joker::JokerRarity::Common
    }

    fn cost(&self) -> usize {
        3
    }

    fn on_hand_played(&self, _context: &mut GameContext, _hand: &SelectHand) -> JokerEffect {
        JokerEffect::new()
            .with_chips(self.chips)
            .with_mult(self.mult)
    }

    fn on_card_scored(&self, _context: &mut GameContext, _card: &Card) -> JokerEffect {
        JokerEffect::new()
            .with_chips(self.chips / 2)
            .with_mult(self.mult / 2)
    }
}

criterion_group!(effect_processor_benches, effect_processor_benchmarks);
criterion_main!(effect_processor_benches);
