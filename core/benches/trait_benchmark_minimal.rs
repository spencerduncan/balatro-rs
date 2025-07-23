//! Minimal performance benchmarks for trait refactoring
//!
//! This benchmark suite measures the performance impact of the new trait system.

use balatro_rs::{
    card::{Card, Suit, Value},
    joker::JokerId,
    joker_registry,
};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

/// Minimal benchmark to test trait method dispatch
pub fn minimal_trait_benchmark(c: &mut Criterion) {
    c.bench_function("trait_method_dispatch", |b| {
        // Test basic trait method calls
        let test_card = Card::new(Value::Ace, Suit::Spade);

        b.iter(|| {
            // Test joker registry access
            if let Ok(joker) = joker_registry::registry::create_joker(&JokerId::Joker) {
                black_box(joker.id());
            }
            black_box(&test_card);
        });
    });
}

/// Test memory allocation patterns
pub fn memory_benchmark(c: &mut Criterion) {
    c.bench_function("card_creation", |b| {
        b.iter(|| {
            let cards = vec![
                Card::new(Value::Ace, Suit::Spade),
                Card::new(Value::King, Suit::Heart),
                Card::new(Value::Queen, Suit::Diamond),
                Card::new(Value::Jack, Suit::Club),
                Card::new(Value::Ten, Suit::Spade),
            ];
            black_box(cards);
        });
    });
}

criterion_group!(trait_benchmarks, minimal_trait_benchmark, memory_benchmark);
criterion_main!(trait_benchmarks);
