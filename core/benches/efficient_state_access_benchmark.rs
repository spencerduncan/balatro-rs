//! Performance benchmarks for efficient state access patterns
//!
//! Benchmarks the concurrent-safe data structures and optimized access patterns
//! implemented for issue #169.

use balatro_rs::concurrent_state::StateUpdate;
use balatro_rs::config::Config;
use balatro_rs::game::Game;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

pub fn efficient_state_access_benchmarks(c: &mut Criterion) {
    bench_concurrent_reads(c);
    bench_state_snapshots(c);
    bench_cached_action_generation(c);
    bench_batch_updates(c);
    bench_multithreaded_access(c);
}

/// Benchmark concurrent read access vs direct field access
fn bench_concurrent_reads(c: &mut Criterion) {
    let game = Game::new(Config::default());

    let mut group = c.benchmark_group("concurrent_reads");
    group.throughput(Throughput::Elements(1000));

    // Benchmark direct field access (baseline)
    group.bench_function("direct_field_access", |b| {
        b.iter(|| {
            for _ in 0..1000 {
                black_box(game.money);
                black_box(game.chips);
                black_box(game.score);
                black_box(&game.stage);
            }
        })
    });

    // Benchmark concurrent-safe access
    group.bench_function("concurrent_safe_access", |b| {
        b.iter(|| {
            for _ in 0..1000 {
                black_box(game.get_money_concurrent());
                black_box(game.get_chips_concurrent());
                black_box(game.get_score_concurrent());
                black_box(game.get_stage_concurrent());
            }
        })
    });

    group.finish();
}

/// Benchmark state snapshot generation for Python bindings optimization
fn bench_state_snapshots(c: &mut Criterion) {
    let game = Game::new(Config::default());

    let mut group = c.benchmark_group("state_snapshots");
    group.throughput(Throughput::Elements(100));

    // Benchmark lock-free snapshot generation
    group.bench_function("lock_free_snapshot", |b| {
        b.iter(|| {
            for _ in 0..100 {
                black_box(game.get_state_snapshot());
            }
        })
    });

    // Benchmark individual field access (simulating current Python binding approach)
    group.bench_function("individual_field_access", |b| {
        b.iter(|| {
            for _ in 0..100 {
                black_box(game.money);
                black_box(game.chips);
                black_box(game.score);
                black_box(format!("{:?}", game.stage));
                black_box(game.round);
                black_box(game.plays);
                black_box(game.discards);
            }
        })
    });

    group.finish();
}

/// Benchmark cached action generation vs regular generation
fn bench_cached_action_generation(c: &mut Criterion) {
    let mut game = Game::new(Config::default());
    game.enable_action_caching(Duration::from_millis(100));

    let mut group = c.benchmark_group("action_generation");
    group.throughput(Throughput::Elements(10));

    // Benchmark regular action generation
    group.bench_function("regular_generation", |b| {
        b.iter(|| {
            for _ in 0..10 {
                let actions: Vec<_> = black_box(game.gen_actions().collect());
                black_box(actions);
            }
        })
    });

    // Benchmark cached action generation (simulates repeated calls with same state)
    group.bench_function("cached_generation", |b| {
        b.iter(|| {
            for _ in 0..10 {
                let actions: Vec<_> = black_box(game.gen_actions_cached().collect());
                black_box(actions);
            }
        })
    });

    group.finish();
}

/// Benchmark batch state updates vs individual updates
fn bench_batch_updates(c: &mut Criterion) {
    let mut group = c.benchmark_group("batch_updates");
    group.throughput(Throughput::Elements(4));

    // Benchmark individual updates
    group.bench_function("individual_updates", |b| {
        b.iter(|| {
            let mut game = Game::new(Config::default());
            game.money = black_box(100);
            game.chips = black_box(50);
            game.mult = black_box(2);
            game.score = black_box(1000);
        })
    });

    // Benchmark batch updates
    group.bench_function("batch_updates", |b| {
        b.iter(|| {
            let mut game = Game::new(Config::default());
            let updates = vec![
                StateUpdate::Money(100),
                StateUpdate::Chips(50),
                StateUpdate::Mult(2),
                StateUpdate::Score(1000),
            ];
            black_box(game.apply_batch_updates(updates).unwrap());
        })
    });

    group.finish();
}

/// Benchmark multithreaded state access patterns
fn bench_multithreaded_access(c: &mut Criterion) {
    let mut group = c.benchmark_group("multithreaded_access");

    // Test with different thread counts
    for thread_count in [1, 2, 4, 8] {
        group.bench_with_input(
            BenchmarkId::new("concurrent_reads", thread_count),
            &thread_count,
            |b, &thread_count| {
                b.iter(|| {
                    let game = Arc::new(Game::new(Config::default()));
                    let mut handles = vec![];

                    for _ in 0..thread_count {
                        let game_clone = Arc::clone(&game);
                        let handle = thread::spawn(move || {
                            for _ in 0..100 {
                                black_box(game_clone.get_money_concurrent());
                                black_box(game_clone.get_state_snapshot());
                            }
                        });
                        handles.push(handle);
                    }

                    for handle in handles {
                        handle.join().unwrap();
                    }
                })
            },
        );
    }

    group.finish();
}

/// Benchmark memory allocation patterns for different access strategies
fn bench_memory_patterns(c: &mut Criterion) {
    let game = Game::new(Config::default());

    let mut group = c.benchmark_group("memory_patterns");
    group.throughput(Throughput::Elements(1000));

    // Benchmark allocation-heavy approach (simulating current Python bindings)
    group.bench_function("allocation_heavy", |b| {
        b.iter(|| {
            for _ in 0..1000 {
                // Simulate cloning entire game state
                let _cloned_game = black_box(game.clone());
            }
        })
    });

    // Benchmark allocation-light approach (using snapshots)
    group.bench_function("allocation_light", |b| {
        b.iter(|| {
            for _ in 0..1000 {
                let _snapshot = black_box(game.get_state_snapshot());
            }
        })
    });

    group.finish();
}

/// Benchmark state consistency validation
fn bench_state_validation(c: &mut Criterion) {
    let game = Game::new(Config::default());

    let mut group = c.benchmark_group("state_validation");
    group.throughput(Throughput::Elements(1000));

    group.bench_function("validate_consistency", |b| {
        b.iter(|| {
            for _ in 0..1000 {
                black_box(game.validate_state_consistency());
            }
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    efficient_state_access_benchmarks,
    bench_memory_patterns,
    bench_state_validation
);
criterion_main!(benches);
