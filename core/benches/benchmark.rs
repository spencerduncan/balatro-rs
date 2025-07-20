use balatro_rs::{action::Action, game::Game, rng::GameRng};
use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("run game gen actions", |b| b.iter(run_game_gen_actions));
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

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
