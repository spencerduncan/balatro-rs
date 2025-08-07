# CLAUDE.md - Core Library Package

## Directory Purpose

The `core` directory contains the main game engine library (`balatro-rs` crate) that implements the entire Balatro game logic and move generation system. This is the foundational crate that powers both the CLI application and Python bindings, providing a high-performance, thread-safe implementation optimized for reinforcement learning applications.

## Package Configuration

- **Crate Name**: `balatro-rs` (v0.0.1)
- **Edition**: 2021
- **License**: MIT
- **Repository**: https://github.com/evanofslack/balatro-rs

## Dependencies Architecture

### Core Dependencies (Always Included)
- `anyhow` (~1.0): Error handling
- `indexmap` (2.6.0): Ordered hash maps for deterministic iteration
- `itertools` (0.13.0): Iterator utilities
- `once_cell` (1.20): Lazy static initialization
- `rand` (~0.8.5) & `rand_chacha` (~0.3.1): RNG with reproducible seeds
- `strum` (0.26): Enum utilities with derive macros
- `thiserror` (~1.0.61): Error type derivation

### Optional Dependencies (Feature-Gated)
- `pyo3` (0.24.1): Python bindings (enabled with `python` feature)
- `serde`/`serde_json`/`toml`: Serialization (enabled with `serde` feature)
- `colored` (2.2.0): Terminal colors (enabled with `colored` feature)
- `regex` (~1.10.2): Pattern matching (enabled with `serde` feature)
- `tracing` (~0.1.40): Observability (optional)
- `uuid` (~1.9.1): Unique identifiers (optional)

## Feature Flags

```toml
[features]
default = ["serde", "python"]  # Standard build with serialization and Python support
python = ["dep:pyo3"]          # Enable Python bindings
serde = [...]                  # Enable serialization/deserialization
colored = ["dep:colored"]      # Enable colored terminal output
statistical_tests = []         # Enable slow/flaky statistical tests
integration_tests = []         # Enable slow integration tests
disabled-for-emergency = []    # Temporarily disabled tests
```

## Library Structure

### Public API Exports
- **Core Game**: `game`, `action`, `stage`, `generator`
- **Game Elements**: `card`, `deck`, `hand`, `joker`
- **Systems**: `consumables`, `vouchers`, `boss_blinds`, `skip_tags`
- **Infrastructure**: `rng`, `error`, `config`, `memory_monitor`

### Initialization System
```rust
pub fn initialize() -> Result<(), Box<dyn std::error::Error + Send + Sync>>
```
Must be called once at startup to:
- Initialize global joker registry
- Set up tarot card factory
- Prepare all factory systems
- Ensure thread-safe global state

## Workspace Integration

The core library serves as the foundation:
1. **CLI Application** (`../cli`): Uses with `colored` feature
2. **Python Bindings** (`../pylatro`): Uses with default features
3. **Workspace Structure**: Part of Cargo workspace with resolver v2

## Building and Development

### Build Commands
```bash
# Standard build
cargo build -p balatro-rs

# With specific features
cargo build -p balatro-rs --features colored

# Release build for performance
cargo build -p balatro-rs --release
```

### Testing
```bash
# Run all tests
cargo test -p balatro-rs

# Run benchmarks
cargo bench -p balatro-rs

# Run with specific features
cargo test -p balatro-rs --features statistical_tests
```

## Key Design Characteristics

1. **Performance-First**: Optimized for RL training with minimal allocations
2. **Thread-Safe**: Send + Sync bounds throughout for parallel training
3. **Feature-Modular**: Optional dependencies reduce binary size
4. **Deterministic**: Reproducible RNG for training consistency
5. **Zero-Copy**: Minimizes memory overhead for high-frequency operations

## Subdirectories

- **`src/`**: Main source code (see src/CLAUDE.md)
- **`benches/`**: Performance benchmarks (see benches/CLAUDE.md)
- **`examples/`**: Usage examples and patterns (see examples/CLAUDE.md)
- **`tests/`**: Integration tests (see tests/CLAUDE.md)

Each subdirectory has its own CLAUDE.md with detailed documentation.
