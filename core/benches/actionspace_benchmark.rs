use balatro_rs::{config::Config, game::Game, space::ActionSpace};
use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

pub fn actionspace_benchmarks(c: &mut Criterion) {
    let config = Config::default();

    // Benchmark ActionSpace creation
    c.bench_function("actionspace_creation", |b| {
        b.iter(|| black_box(ActionSpace::from(config.clone())));
    });

    // Benchmark to_vec() method (old implementation behavior)
    c.bench_function("actionspace_to_vec_original", |b| {
        let actionspace = ActionSpace::from(config.clone());
        b.iter(|| black_box(actionspace.to_vec()));
    });

    // Benchmark to_vec_cached() method (new optimized implementation)
    c.bench_function("actionspace_to_vec_cached", |b| {
        let mut actionspace = ActionSpace::from(config.clone());
        b.iter(|| black_box(actionspace.to_vec_cached().clone()));
    });

    // Benchmark iterator method (zero-copy access)
    c.bench_function("actionspace_iter", |b| {
        let actionspace = ActionSpace::from(config.clone());
        b.iter(|| black_box(actionspace.iter().collect::<Vec<_>>()));
    });

    // Benchmark iterator sum (typical RL usage pattern)
    c.bench_function("actionspace_iter_sum", |b| {
        let actionspace = ActionSpace::from(config.clone());
        b.iter(|| black_box(actionspace.iter().sum::<usize>()));
    });

    // Benchmark is_empty() method
    c.bench_function("actionspace_is_empty", |b| {
        let actionspace = ActionSpace::from(config.clone());
        b.iter(|| black_box(actionspace.is_empty()));
    });

    // Benchmark repeated to_vec() calls (simulates RL training scenario)
    c.bench_function("actionspace_repeated_to_vec", |b| {
        let actionspace = ActionSpace::from(config.clone());
        b.iter(|| {
            for _ in 0..100 {
                black_box(actionspace.to_vec());
            }
        });
    });

    // Benchmark repeated to_vec_cached() calls
    c.bench_function("actionspace_repeated_to_vec_cached", |b| {
        let mut actionspace = ActionSpace::from(config.clone());
        b.iter(|| {
            for _ in 0..100 {
                black_box(actionspace.to_vec_cached().clone());
            }
        });
    });

    // Benchmark typical RL training workflow: real game scenario
    c.bench_function("actionspace_rl_workflow_original", |b| {
        b.iter(|| {
            let mut game = Game::default();
            game.start();

            // Generate action space in a real game scenario (typical RL setup)
            let actionspace = game.gen_action_space();

            // Access the action space multiple times (typical RL training)
            for _ in 0..10 {
                black_box(actionspace.to_vec());
            }
        });
    });

    // Benchmark RL workflow with optimized methods
    c.bench_function("actionspace_rl_workflow_optimized", |b| {
        b.iter(|| {
            let mut game = Game::default();
            game.start();

            // Generate action space in a real game scenario
            let mut actionspace = game.gen_action_space();

            // Access the action space multiple times using cached version
            for _ in 0..10 {
                black_box(actionspace.to_vec_cached().clone());
            }
        });
    });

    // Benchmark memory allocation patterns
    c.bench_function("actionspace_memory_allocation", |b| {
        let actionspace = ActionSpace::from(config.clone());
        b.iter(|| {
            // This tests memory allocation overhead of the old approach
            let _vec1 = actionspace.to_vec();
            let _vec2 = actionspace.to_vec();
            let _vec3 = actionspace.to_vec();
            black_box((_vec1, _vec2, _vec3));
        });
    });

    // Benchmark From trait conversion
    c.bench_function("actionspace_from_trait", |b| {
        let actionspace = ActionSpace::from(config.clone());
        b.iter(|| {
            let vec: Vec<usize> = actionspace.clone().into();
            black_box(vec);
        });
    });

    // Benchmark From trait performance comparison
    c.bench_function("actionspace_real_world_access", |b| {
        b.iter(|| {
            let mut game = Game::default();
            game.start();

            let mut actionspace = game.gen_action_space();

            // Test different access patterns
            black_box(actionspace.to_vec_cached().clone());
            black_box(actionspace.to_vec());
            black_box(actionspace.iter().sum::<usize>());
        });
    });
}

criterion_group!(benches, actionspace_benchmarks);
criterion_main!(benches);
