use balatro_rs::consumables::{
    tarot::{Judgement, Justice, TarotFactory, Temperance},
    Consumable, ConsumableId, Target,
};
use balatro_rs::game::Game;
use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

/// Benchmark tarot card creation performance
///
/// Tests that all tarot cards can be created in well under 1ms
fn benchmark_tarot_creation(c: &mut Criterion) {
    c.bench_function("tarot_factory_creation", |b| {
        b.iter(|| {
            let factory = TarotFactory::new();
            let cards = ConsumableId::tarot_cards();
            for card_id in black_box(cards) {
                // Unwrap is acceptable in benchmarks for performance measurement
                let _card = factory.create_tarot(card_id).unwrap();
            }
        })
    });
}

/// Benchmark individual tarot card creation
fn benchmark_individual_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("individual_tarot_creation");

    let cards = vec![
        ("Justice", ConsumableId::Justice),
        ("TheHangedMan", ConsumableId::TheHangedMan),
        ("Death", ConsumableId::Death),
        ("Temperance", ConsumableId::Temperance),
        ("TheDevil", ConsumableId::TheDevil),
        ("TheTower", ConsumableId::TheTower),
        ("TheStar", ConsumableId::TheStar),
        ("TheMoon", ConsumableId::TheMoon),
        ("TheSun", ConsumableId::TheSun),
        ("Judgement", ConsumableId::Judgement),
        ("TheWorld", ConsumableId::TheWorld),
    ];

    for (name, id) in cards {
        group.bench_function(name, |b| {
            let factory = TarotFactory::new();
            b.iter(|| {
                // Unwrap is acceptable in benchmarks for performance measurement
                let _card = factory.create_tarot(black_box(id)).unwrap();
            })
        });
    }

    group.finish();
}

/// Benchmark tarot card validation performance
///
/// Tests can_use method performance to ensure it's fast enough for real-time use
fn benchmark_tarot_validation(c: &mut Criterion) {
    let game = create_test_game();
    let target = Target::None;

    let mut group = c.benchmark_group("tarot_validation");

    // Test each tarot card's validation performance
    group.bench_function("justice_validation", |b| {
        let justice = Justice::new();
        b.iter(|| {
            let _result = justice.can_use(black_box(&game), black_box(&target));
        })
    });

    group.bench_function("temperance_validation", |b| {
        let temperance = Temperance::new();
        b.iter(|| {
            let _result = temperance.can_use(black_box(&game), black_box(&target));
        })
    });

    group.bench_function("judgement_validation", |b| {
        let judgement = Judgement::new();
        b.iter(|| {
            let _result = judgement.can_use(black_box(&game), black_box(&target));
        })
    });

    group.finish();
}

/// Benchmark tarot card effect application performance
///
/// This is the critical path - effects must complete in <1ms
fn benchmark_tarot_effects(c: &mut Criterion) {
    let mut group = c.benchmark_group("tarot_effects");
    group.measurement_time(std::time::Duration::from_millis(100));

    // Test Temperance effect (no actual modification, just validation)
    group.bench_function("temperance_effect", |b| {
        b.iter(|| {
            let mut game = create_test_game();
            let temperance = Temperance::new();
            let target = Target::None;
            let _result = temperance.use_effect(black_box(&mut game), black_box(target));
        })
    });

    // Test Justice validation chain
    group.bench_function("justice_validation_chain", |b| {
        b.iter(|| {
            let game = create_test_game();
            let justice = Justice::new();
            let target = Target::cards_in_hand(vec![0]);
            let _result = justice.can_use(black_box(&game), black_box(&target));
        })
    });

    group.finish();
}

/// Create a minimal test game for benchmarking
///
/// Uses the simplest possible game state to minimize overhead
/// and focus on tarot card performance
fn create_test_game() -> Game {
    // This is a placeholder - in a real implementation we'd create
    // a minimal game state optimized for benchmarking
    Game::default()
}

criterion_group!(
    benches,
    benchmark_tarot_creation,
    benchmark_individual_creation,
    benchmark_tarot_validation,
    benchmark_tarot_effects
);

criterion_main!(benches);
