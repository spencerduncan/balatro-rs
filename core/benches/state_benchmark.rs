use balatro_rs::joker::JokerId;
use balatro_rs::joker_state::{JokerState, JokerStateManager};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use std::sync::Arc;

pub fn state_benchmarks(c: &mut Criterion) {
    bench_state_access(c);
    bench_state_updates(c);
    bench_concurrent_access(c);
    bench_bulk_operations(c);
    bench_memory_usage(c);
}

fn bench_state_access(c: &mut Criterion) {
    let manager = Arc::new(JokerStateManager::new());

    // Pre-populate with various jokers
    for i in 0..100 {
        let joker_id = match i % 5 {
            0 => JokerId::Joker,
            1 => JokerId::GreedyJoker,
            2 => JokerId::LustyJoker,
            3 => JokerId::SteelJoker,
            _ => JokerId::AbstractJoker,
        };
        manager.set_state(joker_id, JokerState::with_accumulated_value(i as f64));
    }

    let mut group = c.benchmark_group("state_access");

    group.bench_function("get_state", |b| {
        b.iter(|| manager.get_state(JokerId::Joker))
    });

    group.bench_function("get_accumulated_value", |b| {
        b.iter(|| manager.get_accumulated_value(JokerId::GreedyJoker))
    });

    group.bench_function("has_triggers", |b| {
        b.iter(|| manager.has_triggers(JokerId::SteelJoker))
    });

    group.finish();
}

fn bench_state_updates(c: &mut Criterion) {
    let manager = Arc::new(JokerStateManager::new());

    let mut group = c.benchmark_group("state_updates");

    group.bench_function("add_accumulated_value", |b| {
        b.iter(|| {
            manager.add_accumulated_value(JokerId::Joker, 1.0);
        })
    });

    group.bench_function("use_trigger", |b| {
        b.iter(|| {
            manager.set_state(JokerId::LustyJoker, JokerState::with_triggers(10));
            manager.use_trigger(JokerId::LustyJoker);
        })
    });

    group.bench_function("set_custom_data", |b| {
        b.iter(|| {
            manager
                .set_custom_data(JokerId::AbstractJoker, "test_key", 42)
                .unwrap();
        })
    });

    group.bench_function("update_state", |b| {
        b.iter(|| {
            manager.update_state(JokerId::SteelJoker, |state| {
                state.accumulated_value += 1.0;
            });
        })
    });

    group.finish();
}

fn bench_concurrent_access(c: &mut Criterion) {
    use std::thread;

    let manager = Arc::new(JokerStateManager::new());

    // Pre-populate
    for i in 0..50 {
        let joker_id = match i % 3 {
            0 => JokerId::Joker,
            1 => JokerId::GreedyJoker,
            _ => JokerId::LustyJoker,
        };
        manager.set_state(joker_id, JokerState::with_accumulated_value(i as f64));
    }

    let mut group = c.benchmark_group("concurrent_access");

    group.bench_function("parallel_reads", |b| {
        b.iter(|| {
            let handles: Vec<_> = (0..4)
                .map(|_| {
                    let manager = Arc::clone(&manager);
                    thread::spawn(move || {
                        for _ in 0..10 {
                            let _ = manager.get_state(JokerId::Joker);
                            let _ = manager.get_accumulated_value(JokerId::GreedyJoker);
                            let _ = manager.has_triggers(JokerId::LustyJoker);
                        }
                    })
                })
                .collect();

            for handle in handles {
                handle.join().unwrap();
            }
        })
    });

    group.finish();
}

fn bench_bulk_operations(c: &mut Criterion) {
    let manager = Arc::new(JokerStateManager::new());

    let mut group = c.benchmark_group("bulk_operations");
    group.throughput(Throughput::Elements(100));

    group.bench_function("bulk_state_creation", |b| {
        b.iter(|| {
            for i in 0..100 {
                let joker_id = match i % 10 {
                    0 => JokerId::Joker,
                    1 => JokerId::GreedyJoker,
                    2 => JokerId::LustyJoker,
                    3 => JokerId::SteelJoker,
                    4 => JokerId::AbstractJoker,
                    5 => JokerId::HalfJoker,
                    6 => JokerId::MysticalJoker,
                    7 => JokerId::FibonacciJoker,
                    8 => JokerId::ScaryFace,
                    _ => JokerId::RoughGem,
                };
                manager.set_state(joker_id, JokerState::with_accumulated_value(i as f64));
            }
        })
    });

    group.bench_function("bulk_value_updates", |b| {
        // Pre-populate
        for i in 0..100 {
            let joker_id = match i % 5 {
                0 => JokerId::Joker,
                1 => JokerId::GreedyJoker,
                2 => JokerId::LustyJoker,
                3 => JokerId::SteelJoker,
                _ => JokerId::AbstractJoker,
            };
            manager.set_state(joker_id, JokerState::with_accumulated_value(0.0));
        }

        b.iter(|| {
            for i in 0..100 {
                let joker_id = match i % 5 {
                    0 => JokerId::Joker,
                    1 => JokerId::GreedyJoker,
                    2 => JokerId::LustyJoker,
                    3 => JokerId::SteelJoker,
                    _ => JokerId::AbstractJoker,
                };
                manager.add_accumulated_value(joker_id, 1.0);
            }
        })
    });

    group.finish();
}

fn bench_memory_usage(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_usage");

    for size in [10, 100, 1000].iter() {
        group.bench_with_input(
            BenchmarkId::new("state_manager_with_jokers", size),
            size,
            |b, &size| {
                b.iter(|| {
                    let manager = JokerStateManager::new();

                    for i in 0..size {
                        let joker_id = match i % 15 {
                            0 => JokerId::Joker,
                            1 => JokerId::GreedyJoker,
                            2 => JokerId::LustyJoker,
                            3 => JokerId::SteelJoker,
                            4 => JokerId::AbstractJoker,
                            5 => JokerId::HalfJoker,
                            6 => JokerId::MysticalJoker,
                            7 => JokerId::FibonacciJoker,
                            8 => JokerId::ScaryFace,
                            9 => JokerId::RoughGem,
                            10 => JokerId::Banner,
                            11 => JokerId::EvenSteven,
                            12 => JokerId::OddTodd,
                            13 => JokerId::Scholar,
                            _ => JokerId::Runner,
                        };

                        let mut state = JokerState::with_accumulated_value(i as f64);
                        state.set_custom("level", i).unwrap();
                        state.set_custom("multiplier", i as f64 * 1.5).unwrap();

                        manager.set_state(joker_id, state);
                    }

                    // Simulate some operations
                    for i in 0..size / 10 {
                        let joker_id = match i % 5 {
                            0 => JokerId::Joker,
                            1 => JokerId::GreedyJoker,
                            2 => JokerId::LustyJoker,
                            3 => JokerId::SteelJoker,
                            _ => JokerId::AbstractJoker,
                        };
                        let _ = manager.get_accumulated_value(joker_id);
                        manager.add_accumulated_value(joker_id, 1.0);
                    }
                })
            },
        );
    }

    group.finish();
}

criterion_group!(state_benches, state_benchmarks);
criterion_main!(state_benches);
