# Balatro-RS Test Infrastructure

## Overview

The Balatro-RS testing framework provides comprehensive testing capabilities for the game engine, including unit tests, integration tests, performance benchmarks, and CI/CD integration. This document explains how to use and extend the testing infrastructure.

## Table of Contents

- [Test Organization](#test-organization)
- [Writing Tests](#writing-tests)
- [Running Tests](#running-tests)
- [Coverage Analysis](#coverage-analysis)
- [Performance Testing](#performance-testing)
- [CI/CD Integration](#cicd-integration)
- [Debugging Tests](#debugging-tests)
- [Best Practices](#best-practices)

## Test Organization

```
core/tests/
├── common/           # Shared test utilities
│   ├── mod.rs       # Module exports
│   └── mocks/       # Mock implementations
│       ├── mod.rs   # Mock module
│       ├── game.rs  # Game mocks
│       ├── rng.rs   # RNG mocks
│       └── actions.rs # Action mocks
├── unit/            # Unit tests (if separated)
├── integration/     # Integration tests
└── README.md        # This file
```

### Test Categories

1. **Unit Tests**: Test individual components in isolation
2. **Integration Tests**: Test component interactions
3. **Statistical Tests**: Verify RNG distributions
4. **Performance Tests**: Benchmark critical paths
5. **Security Tests**: Validate input handling and boundaries

## Writing Tests

### Using the Test Framework

```rust
// Import test utilities
use balatro_rs::test_utils::{TestBuilder, MockBuilder};
use balatro_rs::test_fixtures::{standard_deck, basic_jokers};

#[test]
fn test_game_initialization() {
    // Use TestBuilder for game setup
    let game = TestBuilder::new()
        .with_seed(12345)
        .with_deck(standard_deck())
        .with_jokers(basic_jokers())
        .build();

    assert_eq!(game.state.round, 0);
    assert_eq!(game.state.money, 4);
}
```

### Using Mocks

```rust
use crate::common::mocks::{MockGame, MockRng};

#[test]
fn test_with_mocked_rng() {
    let mut rng = MockRng::new();
    rng.expect_next_f64()
        .times(1)
        .returning(|| 0.5);

    let mut game = MockGame::new();
    game.expect_draw_card()
        .with(eq(5))
        .returning(|_| Ok(()));

    // Test logic using mocks
    game.draw_card(5).unwrap();
}
```

### Property-Based Testing

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_score_always_positive(
        chips in 0..10000i32,
        mult in 0..100i32
    ) {
        let score = calculate_score(chips, mult);
        prop_assert!(score >= 0);
    }
}
```

### Async Testing

```rust
#[tokio::test]
async fn test_async_game_loop() {
    let game = GameEngine::new_async().await;

    let result = game.process_turn_async().await;
    assert!(result.is_ok());
}
```

## Running Tests

### Local Development

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_game_initialization

# Run tests with output
cargo test -- --nocapture

# Run tests in release mode
cargo test --release

# Run tests for specific package
cargo test -p balatro-rs

# Run with specific features
cargo test --features "async,benchmark"

# Run ignored tests
cargo test -- --ignored

# Run tests in parallel (default) or sequentially
cargo test -- --test-threads=1
```

### Coverage Testing

```bash
# Generate coverage report (HTML)
./scripts/test-with-coverage.sh --html

# Generate LCOV report for CI
./scripts/test-with-coverage.sh --lcov

# Set custom threshold
./scripts/test-with-coverage.sh --threshold 80

# Exclude patterns
./scripts/test-with-coverage.sh --exclude "*/generated/*,*/vendor/*"
```

### Performance Testing

```bash
# Run benchmarks
cargo bench

# Run specific benchmark
cargo bench bench_joker_scoring

# Compare with baseline
cargo bench -- --baseline main

# Save benchmark results
cargo bench -- --save-baseline my-feature
```

## Coverage Analysis

### Coverage Tools

The project supports two coverage tools:

1. **cargo-llvm-cov** (Recommended)
   - Better integration with LLVM toolchain
   - More accurate source mapping
   - Faster execution

2. **cargo-tarpaulin**
   - Cross-platform support
   - Docker integration
   - Historical compatibility

### Coverage Reports

```bash
# Terminal output
cargo llvm-cov --workspace

# HTML report
cargo llvm-cov --html --open

# LCOV for codecov
cargo llvm-cov --lcov --output-path lcov.info

# JSON for processing
cargo llvm-cov --json --output-path coverage.json
```

### Coverage Exclusions

Add to source files:
```rust
// Exclude function from coverage
#[cfg(not(tarpaulin_include))]
fn debug_only_function() { }

// Exclude module
#[cfg_attr(tarpaulin, skip)]
mod test_helpers;
```

## Performance Testing

### Writing Benchmarks

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_joker_scoring(c: &mut Criterion) {
    let game = setup_game();

    c.bench_function("joker_scoring", |b| {
        b.iter(|| {
            game.calculate_joker_score(black_box(&jokers))
        })
    });
}

criterion_group!(benches, bench_joker_scoring);
criterion_main!(benches);
```

### Performance Regression Detection

The CI pipeline automatically detects performance regressions:

1. Runs benchmarks on PR branch
2. Compares with main branch baseline
3. Fails if regression > 10%
4. Generates comparison report

## CI/CD Integration

### GitHub Actions Workflow

The project uses a comprehensive CI pipeline:

1. **Test Matrix**: Tests across multiple Rust versions and OS
2. **Coverage Collection**: Automated coverage reporting
3. **Performance Checks**: Regression detection
4. **Parallel Execution**: Tests split across workers

### Environment Variables

```bash
# CI detection
CI=true

# Coverage settings
CARGO_INCREMENTAL=0
RUSTFLAGS="-C instrument-coverage"
LLVM_PROFILE_FILE="balatro-%p-%m.profraw"

# Test settings
RUST_BACKTRACE=1
RUST_TEST_THREADS=4
```

### CI-Specific Tests

```rust
#[test]
#[cfg_attr(not(ci), ignore)]
fn test_ci_environment() {
    // Test that only runs in CI
    assert!(std::env::var("CI").is_ok());
}

#[test]
fn test_ci_integration() {
    // Verify CI configuration
    if std::env::var("CI").is_ok() {
        // CI-specific assertions
        assert!(std::env::var("GITHUB_ACTIONS").is_ok());
    }
}
```

## Debugging Tests

### Debug Output

```rust
#[test]
fn test_with_debug_output() {
    env_logger::init(); // Initialize logging

    log::debug!("Starting test");

    let game = setup_game();
    println!("Game state: {:?}", game);

    // Use dbg! macro for quick debugging
    let result = dbg!(game.process_action(action));

    assert!(result.is_ok());
}
```

### Test Failure Investigation

```bash
# Run single test with backtrace
RUST_BACKTRACE=full cargo test test_name -- --exact

# Run with debug logging
RUST_LOG=debug cargo test

# Generate test binary for debugging
cargo test --no-run
# Then debug with gdb/lldb
```

### Using Test Snapshots

```rust
use insta::assert_snapshot;

#[test]
fn test_game_state_snapshot() {
    let game = setup_game();

    // Snapshot testing for complex outputs
    assert_snapshot!(game.to_string());
}
```

## Best Practices

### 1. Test Naming

```rust
// Good: Descriptive, specific
#[test]
fn test_joker_scoring_with_multiplier_bonus() { }

// Bad: Generic, unclear
#[test]
fn test1() { }
```

### 2. Test Independence

```rust
// Each test should be independent
#[test]
fn test_independent_1() {
    let game = TestBuilder::new().build(); // Fresh state
    // ...
}

#[test]
fn test_independent_2() {
    let game = TestBuilder::new().build(); // Fresh state
    // ...
}
```

### 3. Use Test Fixtures

```rust
mod fixtures {
    pub fn standard_game() -> Game {
        TestBuilder::new()
            .with_standard_config()
            .build()
    }

    pub fn endgame_scenario() -> Game {
        TestBuilder::new()
            .with_round(8)
            .with_score(100000)
            .build()
    }
}
```

### 4. Test Coverage Goals

- **Unit Tests**: Aim for 80%+ coverage
- **Integration Tests**: Cover critical paths
- **Edge Cases**: Test boundary conditions
- **Error Paths**: Test failure scenarios

### 5. Performance Test Guidelines

- Benchmark hot paths only
- Use stable inputs for consistency
- Run multiple iterations
- Compare against baselines
- Document expected performance

### 6. Mock Best Practices

```rust
// Configure mocks explicitly
let mut mock = MockGame::new();
mock.expect_method()
    .times(1)  // Explicit call count
    .with(eq(expected_param))  // Parameter validation
    .returning(|_| Ok(()));  // Return value

// Verify mock expectations
mock.checkpoint();  // Explicit verification point
```

### 7. Statistical Testing

```rust
#[test]
fn test_rng_distribution() {
    const SAMPLES: usize = 10000;
    let mut counts = [0; 6];

    for _ in 0..SAMPLES {
        let roll = rng.gen_range(1..=6);
        counts[roll - 1] += 1;
    }

    // Chi-squared test for uniform distribution
    let expected = SAMPLES as f64 / 6.0;
    let chi_squared: f64 = counts.iter()
        .map(|&count| {
            let diff = count as f64 - expected;
            diff * diff / expected
        })
        .sum();

    // 5 degrees of freedom, 95% confidence
    assert!(chi_squared < 11.07);
}
```

## Troubleshooting

### Common Issues

1. **Flaky Tests**
   - Use fixed seeds for RNG
   - Avoid time-dependent logic
   - Mock external dependencies

2. **Slow Tests**
   - Use `#[ignore]` for slow tests
   - Run in parallel when possible
   - Profile test execution

3. **Coverage Gaps**
   - Check excluded patterns
   - Verify test execution
   - Add missing test cases

4. **CI Failures**
   - Check environment differences
   - Verify dependency versions
   - Review CI logs carefully

## Contributing

When adding new tests:

1. Follow existing patterns
2. Add documentation for complex tests
3. Ensure tests are deterministic
4. Include both positive and negative cases
5. Update this README if adding new patterns

## Resources

- [Rust Testing Book](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Criterion.rs Documentation](https://bheisler.github.io/criterion.rs/book/)
- [cargo-nextest](https://nexte.st/)
- [cargo-llvm-cov](https://github.com/taiki-e/cargo-llvm-cov)
- [Proptest](https://proptest-rs.github.io/proptest/)
