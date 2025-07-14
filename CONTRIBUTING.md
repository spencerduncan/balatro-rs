# Contributing to balatro-rs

Thank you for your interest in contributing to balatro-rs! This document provides guidelines and standards for contributing to this project.

## Development Setup

### Prerequisites
- Rust stable (see `rust-toolchain.toml`)
- Python 3.8+ (for pylatro development)
- Git

### Setting Up Your Development Environment

1. Fork and clone the repository:
```bash
git clone https://github.com/your-username/balatro-rs.git
cd balatro-rs
```

2. Install Rust toolchain:
```bash
# The rust-toolchain.toml file will automatically install the correct version
rustup show
```

3. Build the project:
```bash
cargo build --all
```

4. Run tests:
```bash
cargo test --all
```

## Coding Standards

### Rust Code Style

1. **Formatting**: All code must be formatted with `rustfmt`
   ```bash
   cargo fmt --all
   ```

2. **Linting**: Code must pass `clippy` without warnings
   ```bash
   cargo clippy --all -- -D warnings
   ```

3. **Documentation**: 
   - All public APIs must have documentation comments
   - Use `///` for public items
   - Include examples where appropriate

4. **Testing**:
   - Write unit tests for new functionality
   - Integration tests go in `tests/` directory
   - Aim for high test coverage
   - Run tests before submitting PR: `cargo test --all`

### Commit Messages

Follow conventional commit format:
- `feat:` New features
- `fix:` Bug fixes
- `docs:` Documentation changes
- `test:` Test additions or modifications
- `refactor:` Code refactoring
- `perf:` Performance improvements
- `chore:` Maintenance tasks

Example: `feat: add tarot card consumables system`

### Pull Request Process

1. Create a new branch for your feature/fix:
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. Make your changes following the coding standards

3. Ensure all tests pass:
   ```bash
   cargo test --all
   cargo fmt --all -- --check
   cargo clippy --all -- -D warnings
   ```

4. Update documentation if needed

5. Submit a pull request with:
   - Clear description of changes
   - Reference to any related issues
   - Test results

### CI/CD Pipeline

All pull requests must pass the following CI checks:
- Rust formatting (`cargo fmt`)
- Clippy linting (`cargo clippy`)
- All tests (`cargo test`)
- Code coverage reporting
- Build verification for all workspace members

The CI pipeline runs in Docker containers for consistency. See `.github/workflows/` for details.

## Project Structure

```
balatro-rs/
├── core/           # Main game engine library
├── cli/            # Command-line interface
├── pylatro/        # Python bindings
└── .github/        # CI/CD workflows
```

## Performance Considerations

This project is designed for reinforcement learning applications, so performance is critical:
- Minimize allocations in hot paths
- Use iterators where possible
- Profile before optimizing
- Benchmark significant changes

## Questions?

If you have questions about contributing, please:
1. Check existing issues and discussions
2. Open a new issue with your question
3. Tag it with the `question` label

Thank you for contributing to balatro-rs!