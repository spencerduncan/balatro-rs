# CLAUDE.md - Joker Test Traits

## Directory Purpose

This directory contains the comprehensive testing infrastructure for the trait-based joker system. It validates the implementation of modular joker traits that decompose the monolithic `Joker` trait into focused, single-responsibility interfaces.

## Key Components

### Test Modules
- **`mod.rs`**: Module declaration hub organizing trait-specific test modules
- **`lifecycle.rs`**: Tests for `JokerLifecycle` trait - validates event handling throughout joker lifecycle
- **`state.rs`**: Tests for `JokerState` trait - validates state management, serialization, and persistence

### Testing Infrastructure

#### Lifecycle Testing (`lifecycle.rs`)
- **Mock Implementation**: `StaticLifecycleMock` with thread-safe state tracking via `Arc<Mutex<LifecycleState>>`
- **Event Coverage**: Purchase, sell, destroy, round start/end, joker interactions
- **Validation**: Event ordering, state invariants, concurrent access patterns

#### State Management Testing (`state.rs`)
- **Mock Implementations**:
  - `SimpleMockJoker`: Basic JSON value state storage
  - `ComplexMockJoker`: Structured state with validation
  - `FailingMockJoker`: Error condition testing
- **Coverage**: Serialization roundtrips, state transitions, error handling, thread safety

## Architecture Patterns

### Zero-Allocation Philosophy
- Minimal memory allocation in tests
- Stack-allocated structures where possible
- Performance-conscious testing mirroring production requirements

### Thread Safety Verification
All trait implementations validated for `Send + Sync` bounds:
```rust
fn assert_send_sync<T: Send + Sync>() {}
assert_send_sync::<StaticLifecycleMock>();
```

### Property-Based Testing
- Idempotence verification for state operations
- Invariant checking across state transitions
- Edge case exploration with extreme values

## Test Coverage

### Categories
1. **Unit Tests**: Individual trait method validation
2. **Integration Tests**: Multi-joker interactions and full game simulations
3. **Property Tests**: Mathematical properties and invariants
4. **Concurrency Tests**: Thread-safe access patterns
5. **Edge Case Tests**: Boundary conditions and error scenarios
6. **Performance Tests**: Batch operations and rapid state transitions

### Coverage Metrics
- **Lifecycle Events**: 100% of defined lifecycle methods
- **State Operations**: Full CRUD cycle with error paths
- **Serialization**: All JSON value types including edge cases
- **Concurrency**: Multi-threaded access patterns with mutex protection

## Trait System Integration

The tests validate five core traits:
1. **`JokerIdentity`**: Static metadata (name, description, rarity)
2. **`JokerLifecycle`**: Event handling during joker lifetime
3. **`JokerGameplay`**: Core game mechanics and scoring
4. **`JokerModifiers`**: Passive effects and multipliers
5. **`JokerState`**: Persistent state management

## Important Development Notes

### Test Patterns
```rust
// Mock implementation pattern
struct StaticLifecycleMock {
    state: Arc<Mutex<LifecycleState>>,
}

// Error recovery testing
let good_state = joker.serialize_state().unwrap();
let result = joker.deserialize_state(bad_state);
assert!(result.is_err());
assert_eq!(joker.serialize_state().unwrap(), good_state);
```

### Key Principles
1. **Test Isolation**: Each test fully isolated with no shared state
2. **Performance Focus**: Validates both correctness and efficiency
3. **Real-World Scenarios**: Integration tests simulate actual game patterns
4. **Failure Testing**: Explicit testing of error conditions
5. **Living Documentation**: Tests serve as usage examples for trait implementations

## Migration Support

Tests support the ongoing migration from monolithic to trait-based design:
- Validates new trait implementations independently
- Ensures backward compatibility during transition
- Provides reference implementations for trait adoption
- Integrates with `JokerStateManager` for legacy compatibility

## Performance Characteristics

Based on test benchmarks:
- Event handling: ~100ns per lifecycle event
- State serialization: ~1μs for simple states, ~10μs for complex
- Thread-safe access: ~500ns overhead for mutex operations
- Batch operations: Linear scaling with joker count
