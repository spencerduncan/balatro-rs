# CLAUDE.md - Joker Testing Infrastructure

## Directory Purpose

The joker testing infrastructure provides a comprehensive, multi-layered testing approach to validate the complex joker system implementation while maintaining high performance standards. Tests serve as both validation tools and living documentation for joker behavior.

## Test Organization

### Directory Structure
```
tests/
├── mod.rs                    # Test module hub
└── traits/                   # Trait-specific tests (see traits/CLAUDE.md)
    ├── lifecycle.rs          # JokerLifecycle trait tests
    ├── state.rs              # JokerState trait tests
    └── mod.rs                # Trait test organization
```

### Related Test Files
- `test_jokers.rs`: Mock joker implementations for testing
- `test_utils.rs`: Testing utilities and helpers
- `identity_tests.rs`: Zero-allocation identity trait tests
- `hand_composition_tests.rs`: Complex joker interaction tests

## Testing Methodologies

### Zero-Allocation Testing
- Compile-time constants and static data where possible
- Stack-allocated test structures to minimize heap pressure
- Performance-critical tests run 268x faster than heap-allocated versions

### Mock Implementation Hierarchy
```rust
TestChipsJoker      // Chips-based effects
TestMultJoker       // Mult-based effects
TestXMultJoker      // Multiplicative effects
TestMoneyJoker      // Economic effects
TestRetriggerJoker  // Retrigger mechanics
TestSpecialJoker    // Complex transformations
TestScalingJoker    // Scaling effects
```

### Thread-Safety Verification
- All implementations validated for `Send + Sync` bounds
- Concurrent access patterns tested with `Arc<Mutex<>>` wrappers
- Race condition testing for state mutations

### Property-Based Testing
- Idempotence verification for state operations
- Invariant checking across state transitions
- Edge case exploration with boundary values
- Statistical distribution validation for RNG features

## Test Coverage Areas

### Unit Test Coverage
- **Identity Tests**: Static metadata validation (name, description, rarity, cost)
- **Lifecycle Tests**: Event handling (purchase, sell, destroy, round events)
- **State Tests**: Serialization, deserialization, state persistence
- **Effect Tests**: Chips, mult, xmult, money, and special effect validation
- **Condition Tests**: Trigger condition evaluation and state dependencies

### Integration Test Coverage
- **Multi-Joker Interactions**: Tests for joker synergies and conflicts
- **Full Game Simulations**: End-to-end gameplay scenarios
- **Migration Compatibility**: Validates backward compatibility during trait migration
- **Performance Benchmarks**: Ensures operations meet timing requirements

### Edge Case Coverage
- Negative money values
- Empty hands and decks
- Maximum value boundaries (MAX_MULT = 1_000_000)
- Concurrent state modifications
- Error recovery scenarios

## Trait System Integration

### Trait-Specific Testing
Each trait has dedicated test coverage:
- `JokerIdentity`: Static property tests with zero allocations
- `JokerLifecycle`: Event sequence and state invariant tests
- `JokerGameplay`: Game event hook validation
- `JokerModifiers`: Passive effect calculation tests
- `JokerState`: Serialization roundtrip and error recovery tests

### Migration Support Testing
- Parallel testing of legacy and trait-based implementations
- Compatibility bridge validation
- State manager integration tests
- Factory pattern verification

## Performance Requirements

Enforces strict performance targets:
- Event handling: ~100ns per lifecycle event
- State serialization: ~1μs for simple states
- Effect calculation: ~10μs for complex game states
- Memory usage: ~1KB per joker instance

## Test Utilities

### Context Builders
```rust
TestContextBuilder::new()
    .with_chips(100)
    .with_mult(5)
    .with_money(50)
    .build();
```

### Mock Joker Builders
```rust
MockGameplayJoker::new()
    .with_hand_effect(JokerEffect::new().with_mult(10))
    .with_card_effect(JokerEffect::new().with_chips(5));
```

## Testing Principles

1. **Test as Documentation**: Test names clearly describe behavior
2. **Isolation**: Each test fully isolated with no shared state
3. **Performance Focus**: Validates both correctness and efficiency
4. **Real-World Scenarios**: Integration tests simulate actual game patterns
5. **Failure Testing**: Explicit testing of error conditions and recovery
6. **Continuous Verification**: Tests run on every change to prevent regressions

## Subdirectories

- **`traits/`**: Comprehensive testing for the trait-based joker system (see traits/CLAUDE.md)
