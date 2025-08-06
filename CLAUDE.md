# CLAUDE.md - Balatro-RS Root

## Project Overview

Balatro-RS is a high-performance game engine and move generator for a simplified version of Balatro, written in Rust with Python bindings. The project is designed as a move generator for reinforcement learning applications.

**GitHub Repository**: https://github.com/spencerduncan/balatro-rs (fork of evanofslack/balatro-rs)

## Workspace Structure

The project uses a Cargo workspace with three members:

### Core Package (`core/`)
Main game engine library implementing:
- Complete game logic and state management
- Move generation for AI/RL
- Joker system with 140+ implementations
- Shop, consumables, vouchers, and boss blinds
- See `core/CLAUDE.md` for details

### CLI Package (`cli/`)
Simple command-line interface for playing:
- Text-based interactive gameplay
- Security-first input handling
- Direct integration with core engine
- See `cli/CLAUDE.md` for details

### Python Bindings (`pylatro/`)
Comprehensive Python API for RL/ML:
- PyO3-based FFI bindings
- Dual API design (control + observation)
- Extensive joker metadata queries
- OpenAI Gym environment support
- See `pylatro/CLAUDE.md` for details

## Build System

### Workspace Configuration
```toml
[workspace]
members = ["core", "pylatro", "cli"]
resolver = "2"
```

### Toolchain Requirements
- **Rust**: Stable with rustfmt and clippy
- **Python**: 3.8+ for pylatro bindings
- **Maturin**: For building Python extensions

## Development Workflow

### Pre-Commit Hooks
**MANDATORY**: Set up pre-commit hooks for code quality:
```bash
./scripts/setup-precommit.sh
```

Automatically runs:
- `cargo fmt` - Code formatting
- `cargo clippy` - Linting
- `cargo test` - Fast tests
- Security checks (cargo-audit, cargo-deny)

### Local CI Testing
Before pushing, replicate CI checks:
```bash
./scripts/test-ci-local.sh
```

### Building
```bash
# Build everything
cargo build --all

# Build specific package
cargo build -p balatro-rs
cargo build -p balatro-cli
cargo build -p pylatro

# Release builds
cargo build --all --release
```

### Testing
```bash
# Test everything
cargo test --all

# Run benchmarks
cargo bench

# Run with features
cargo test --all --features statistical_tests
```

## Documentation Files

### Core Documentation
- **README.md**: Project overview and getting started
- **CONTRIBUTING.md**: Contribution guidelines
- **SECURITY.md**: Security policy
- **PRE_COMMIT_GUIDE.md**: Pre-commit setup guide

### Technical Documentation
- **ADVANCED_JOKER_FRAMEWORK.md**: Joker system architecture
- **JOKER_EXAMPLES_GUIDE.md**: Implementation patterns
- **IMPLEMENTATION_BRIEF.md**: Strategy documentation
- **game_module_refactoring_epic_v2.md**: Refactoring plans

### Code Review Wisdom
- **botdean_wisdom.md**: Production engineering patterns
- **unclebob_wisdom.md**: Clean code principles
- **linus_wisdom.md**: Kernel-quality standards

## Project Standards

### Code Quality Requirements
1. **ALWAYS run rustfmt** before committing
2. **Zero clippy warnings** allowed
3. **All tests must pass** before merge
4. **Performance benchmarks** tracked
5. **Security scanning** via cargo-deny

### Architecture Principles
1. **Performance-first** for RL training
2. **Thread-safe** for parallel execution
3. **Zero-copy** where possible
4. **Deterministic** RNG for reproducibility
5. **Modular** with clear boundaries

## Key Directories

### Source Code
- `core/src/`: Game engine implementation
- `cli/src/`: CLI application
- `pylatro/src/`: Python bindings

### Testing & Benchmarks
- `core/tests/`: Integration tests
- `core/benches/`: Performance benchmarks
- `core/examples/`: Usage examples

### Configuration & Scripts
- `scripts/`: Automation scripts
- `examples/`: Joker definitions and demos
- `.github/`: CI/CD workflows

## Performance Targets

Critical metrics for RL training:
- **Action generation**: <10μs
- **State snapshots**: ~100ns cached
- **Memory per joker**: ~1KB
- **Hand evaluation**: O(n) single-pass

## Security

- Input validation on all boundaries
- No unsafe code in hot paths
- Dependency scanning via cargo-deny
- Regular security audits

## License

MIT License - See LICENSE file for details

## Getting Started

1. **Clone the repository**
2. **Run setup**: `./scripts/setup-precommit.sh`
3. **Build**: `cargo build --all`
4. **Test**: `cargo test --all`
5. **Play**: `cargo run -p balatro-cli`

For detailed information about specific components, refer to the CLAUDE.md files in each directory.
