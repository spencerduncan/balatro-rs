# CLAUDE.md - Integration Tests

## Directory Purpose

The tests directory contains comprehensive integration, unit, and acceptance tests ensuring correctness, performance, and backward compatibility of the engine. Tests follow professional standards including Uncle Bob's principles and kernel-quality requirements.

## Test Organization

### Core Functionality Tests
- `test.rs`: Basic integration tests
- `action_test.rs`: Action system validation
- `stage_test.rs`: Game stage transitions
- `error_test.rs`: Error handling verification

### Joker System Tests
- `test_basic_chips_jokers.rs`: Basic joker implementations
- `test_condition_based_jokers.rs`: Conditional joker logic
- `joker_scoring_integration_test.rs`: Scoring integration
- `joker_state_integration_tests.rs`: State management
- `joker_targeting_test.rs`: Targeting mechanics
- Multiple specialized files for each joker category

### Critical Bug Fix Tests
- `critical_bug_fixes_simple.rs`: Kernel-quality regression prevention
- Deterministic testing patterns
- Hash collision prevention
- Debug formatting validation

### Performance & Memory Tests
- `memory_leak_tests.rs`: Memory safety validation
- `optimization_validation.rs`: Performance regression checks
- `thread_safety_mutable_joker_tests.rs`: Concurrency safety

### Statistical Tests
- `rng_statistical_tests.rs`: RNG fairness validation
- Chi-square goodness of fit tests
- Permutation coverage validation
- Boolean probability distribution tests

### System Integration Tests
- `precommit_integration_test.rs`: Development workflow validation
- `save_game_compatibility_tests.rs`: Backward compatibility
- `pyo3_test.rs`: Python binding integration
- `module_structure_test.rs`: Architecture validation

### Feature-Specific Tests
- `consumable_trait_test.rs`: Consumables system
- `voucher_trait_definition_test.rs`: Voucher system
- `boss_blind_trait_test.rs`: Boss mechanics
- `skip_tags_integration_test.rs`: Skip tag system
- `pack_system_tests.rs`: Pack generation

## Testing Methodologies

### Test Categories
1. **Unit Testing**: Individual component validation
2. **Integration Testing**: Multi-component interaction
3. **Acceptance Testing**: Full game flow scenarios
4. **Regression Testing**: Critical bug prevention
5. **Performance Testing**: Benchmark validation
6. **Statistical Testing**: RNG and probability verification
7. **Security Testing**: Input validation and safety

### Testing Patterns

#### Deterministic Testing
```rust
#[test]
fn test_deterministic_behavior() {
    let mut game = Game::with_seed(12345);
    let result1 = game.play_hand();

    let mut game2 = Game::with_seed(12345);
    let result2 = game2.play_hand();

    assert_eq!(result1, result2);
}
```

#### Test Isolation
```rust
fn create_test_fixture() -> TestGame {
    TestGame {
        deck: create_standard_deck(),
        jokers: vec![],
        money: 100,
        ante: 1,
    }
}
```

#### Performance Assertions
```rust
#[test]
fn test_action_generation_performance() {
    let game = create_complex_game_state();
    let start = Instant::now();

    let _actions = game.gen_actions();

    assert!(start.elapsed() < Duration::from_micros(10));
}
```

## Statistical Testing

### RNG Validation
- Chi-square tests for uniform distribution
- Kolmogorov-Smirnov tests for continuous distributions
- Permutation tests for shuffle fairness
- Frequency analysis for bias detection

### Test Configuration
```rust
#[cfg(feature = "statistical_tests")]
#[test]
fn test_rng_distribution() {
    // Statistical tests behind feature flag
    // to avoid CI flakiness
}
```

## Best Practices

### Test Naming
```rust
#[test]
fn test_joker_greedy_adds_mult_per_diamond() {
    // Clear, descriptive test name
}
```

### Disabled Test Tracking
```rust
#[test]
#[ignore = "Waiting for Planet card implementation"]
fn test_planet_card_hand_upgrade() {
    // Clear documentation of why disabled
}
```

### Feature Flags
```rust
#[cfg(all(test, not(target_arch = "wasm32")))]
fn test_native_only_feature() {
    // Platform-specific tests
}
```

## Running Tests

```bash
# Run all tests
cargo test --all

# Run specific test file
cargo test test_basic_chips_jokers

# Run with output
cargo test -- --nocapture

# Run ignored tests
cargo test -- --ignored

# Run with specific features
cargo test --features statistical_tests
```

## Coverage Requirements

### Minimum Coverage
- Core functionality: 90%
- Joker system: 85%
- Error paths: 80%
- Edge cases: 75%

### Critical Paths
Must have 100% coverage:
- Save/load system
- RNG operations
- Money calculations
- Score calculations

## Continuous Integration

- Tests run on every commit
- Performance tests track regressions
- Statistical tests run nightly
- Coverage reports generated weekly
