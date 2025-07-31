# balatro-rs

[![CI](https://github.com/spencerduncan/balatro-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/spencerduncan/balatro-rs/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/spencerduncan/balatro-rs/branch/main/graph/badge.svg)](https://codecov.io/gh/spencerduncan/balatro-rs)

Game engine and move generator for a simplified version of [balatro](https://www.playbalatro.com/), written in rust with python bindings

## ⚠️ Breaking Changes in v1.x

**f64 Migration (In Progress)**: Core game state fields have been migrated to f64 for Lua compatibility. JokerEffect system migration is still in progress.

- `Game.{chips, mult, score, money, round}`: `usize` → `f64` ✅ COMPLETE
- `JokerEffect` numeric fields: `i32/f32` → `f64` ❌ IN PROGRESS
- Save files are automatically migrated ✅
- **See breaking changes above for current migration status**

## Overview

This library implements a subset of balatro's rules allowing execution of games or simulations. It provides an exhaustive list of actions a user could take at any given stage, as well as an engine to execute those actions and progress the game.

The goal of providing all valid actions is to serve as a move generator. This would be the first step to apply reinforcement learning for balatro.

## Example

```rust
use balatro_rs::{action::Action, game::Game, stage::Stage};
use rand::Rng;

fn main() {
    let mut g = Game::default();
    g.start();
    while !g.is_over() {
        // Get all available moves
        let actions: Vec<Action> = g.gen_moves().collect();
        if actions.len() == 0 {
            break;
        }

        // Pick a random move and execute it
        let i = rand::thread_rng().gen_range(0..actions.len());
        let action = actions[i].clone();
        g.handle_action(action);
    }
    let result = g.result();
}
```

## Features

This library does not implement all aspects of balatro and likely never will.

The follow features are implemented (including move generation)
- [x] identification and scoring of poker hands
- [x] playing/discarding/reordering of cards
- [x] blind pass/fail and game win/lose conditions
- [x] money/interest generation
- [x] ante progression (up to ante 8)
- [x] blind progression (small, big, boss)
- [x] stage transition (pre-blind, blind, post-blind, shop)
- [x] buying/selling/using jokers (very partial support)

The following features are missing and may or may not be added
- [ ] buying/selling/using tarots
- [ ] buying/selling/using planets
- [ ] buying/selling/using spectrals
- [ ] boss blind modifiers
- [ ] skip blind/tags
- [ ] card enhancements, foils and seals
- [ ] joker foils
- [ ] alternative decks
- [ ] alternative stakes

## Development

### Quick Start

1. Clone the repository and set up pre-commit hooks:
```bash
git clone https://github.com/spencerduncan/balatro-rs.git
cd balatro-rs
./scripts/setup-precommit.sh  # Install automated code quality checks
```

2. Build and test:
```bash
cargo build --all
cargo test --all
```

### Code Quality (Automated)

This project uses pre-commit hooks to maintain code quality:

- **Formatting**: Rust code is automatically formatted with `rustfmt`
- **Linting**: Zero warnings policy with `clippy`
- **Testing**: Core tests run before each commit
- **Security**: Dependency vulnerability scanning

The hooks run automatically on `git commit`. See `PRE_COMMIT_GUIDE.md` for details.

## Python bindings

This library uses [pyo3](https://pyo3.rs) to provide python bindings with a clean, performance-oriented API. The Python interface follows a clear separation of concerns:

- **GameEngine**: For performing actions and controlling game flow
- **GameState**: For read-only access to game state (immutable snapshots)

### Quick Python Example

```python
import pylatro

# Create game engine
engine = pylatro.GameEngine()

# Game loop using the new unified API
while not engine.is_over:
    actions = engine.gen_actions()        # Actions from engine
    if actions:
        engine.handle_action(actions[0])  # Execute via engine

    # Access read-only state when needed
    state = engine.state
    print(f"Score: {state.score}, Money: {state.money}")

print(f"Final score: {engine.state.score}")
```

### Migration from Legacy API

If you're upgrading from the previous "dual framework" design where both `GameState` and `GameEngine` had action methods:

- **Use `GameEngine`** for all actions: `gen_actions()`, `handle_action()`, `is_over`, etc.
- **Use `GameState`** for read-only access: `score`, `money`, `available`, `joker_ids`, etc.
- **Backwards compatibility**: Old API still works but shows deprecation warnings

The new unified API provides better performance and clearer semantics. The old API still works with deprecation warnings for backwards compatibility.

For more details on the python work and reinforcement learning applications, check the [/pylatro](https://github.com/spencerduncan/balatro-rs/tree/main/pylatro) directory.
# Trigger CI
# CI trigger
