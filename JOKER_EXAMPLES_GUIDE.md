# Joker Examples Guide

This guide provides comprehensive examples demonstrating the joker trait system in balatro-rs. All examples are located in the `core/examples/` directory and can be run with `cargo run --example <name>`.

## Overview

The joker examples demonstrate various implementation patterns, performance optimizations, and best practices for creating jokers in balatro-rs. Each example focuses on specific aspects of joker development.

## Example Files

### 1. `joker_comprehensive_guide.rs`

**Purpose**: Complete overview of the joker system  
**Run with**: `cargo run --example joker_comprehensive_guide`

**Content**:
- Basic joker usage patterns
- Joker creation methods (direct, factory, builder)
- Game integration examples
- Static joker framework demonstration
- Performance optimization patterns
- Best practices for implementation

**Key Takeaways**:
- Use the factory pattern for dynamic joker creation
- Static jokers are efficient for simple conditional logic
- Always implement comprehensive testing
- Follow naming conventions that match Balatro

### 2. `new_joker_api.rs`

**Purpose**: Simple introduction to the new trait system  
**Run with**: `cargo run --example new_joker_api`

**Content**:
- Basic trait usage
- Factory creation examples
- Effect demonstration
- Rarity-based joker queries

**Key Takeaways**:
- The new trait system is clean and type-safe
- Factory methods provide easy joker creation
- Effects are composable through builder pattern

### 3. `trait_composition_examples.rs`

**Purpose**: Advanced joker implementations using multiple traits  
**Run with**: `cargo run --example trait_composition_examples`

**Content**:
- Complex scoring jokers with multiple conditions
- State-tracking jokers with lifecycle management
- Modifier jokers affecting game mechanics
- Event-driven jokers with multiple triggers
- Dynamic jokers that evolve over time

**Key Takeaways**:
- Complex jokers can combine multiple behavioral patterns
- State management enables persistent joker evolution
- Modifier traits can change core game mechanics
- Event-driven patterns provide reactive behavior

### 4. `edge_case_examples.rs`

**Purpose**: Defensive programming and error handling patterns  
**Run with**: `cargo run --example edge_case_examples`

**Content**:
- Self-destructing jokers with limited uses
- Validation jokers with complex conditions
- Overflow-safe accumulating jokers
- State corruption recovery mechanisms
- Anti-pattern demonstrations

**Key Takeaways**:
- Always validate inputs and state
- Use saturating arithmetic for scores
- Implement graceful fallbacks
- Handle state corruption gracefully
- Test edge cases thoroughly

### 5. `performance_examples.rs`

**Purpose**: Performance optimization patterns and comparisons  
**Run with**: `cargo run --example performance_examples`

**Content**:
- Efficient vs inefficient condition checking
- State access pattern comparisons
- Memory allocation optimization
- Early return patterns
- Batched operation strategies

**Key Takeaways**:
- Early returns eliminate unnecessary work
- Cache frequently accessed state values
- Avoid allocations in hot paths
- Use guard clauses for validation
- Batch operations when possible

### 6. `test_harness.rs`

**Purpose**: Comprehensive testing framework for joker validation  
**Run with**: `cargo run --example test_harness`

**Content**:
- Factory creation testing
- Trait implementation validation
- Effect generation testing
- State management verification
- Performance benchmarking

**Key Takeaways**:
- Automated testing catches regressions
- Performance testing ensures scalability
- State validation prevents corruption
- Factory testing verifies all joker types work

## Design Patterns

### 1. Factory Pattern

**When to use**: Dynamic joker creation, polymorphic collections

```rust
// Recommended approach for runtime joker creation
if let Some(joker) = JokerFactory::create(JokerId::Joker) {
    // Use the joker
}
```

**Benefits**:
- Centralized creation logic
- Easy polymorphic handling
- Consistent initialization

### 2. Static Joker Framework

**When to use**: Simple conditional jokers

```rust
// Efficient for basic conditional logic
let joker = StaticJoker::builder(id, name, description)
    .rarity(JokerRarity::Common)
    .mult(3)
    .condition(StaticCondition::SuitScored(Suit::Diamond))
    .per_card()
    .build()?;
```

**Benefits**:
- Compile-time optimizations
- Minimal runtime overhead
- Declarative syntax

### 3. Direct Implementation

**When to use**: Complex jokers requiring custom logic

```rust
impl Joker for CustomJoker {
    // Implement all required methods
    // Add custom logic for complex behavior
}
```

**Benefits**:
- Full control over behavior
- Complex state management
- Custom lifecycle hooks

### 4. State Management Patterns

#### Caching Pattern
```rust
// Cache frequently accessed values
let cached_value = context.joker_state_manager
    .get_accumulated_value(self.id())
    .unwrap_or(0.0);
```

#### Update Pattern
```rust
// Use closures for state updates
context.joker_state_manager.update_state(self.id(), |state| {
    state.accumulated_value += 1.0;
});
```

#### Recovery Pattern
```rust
// Handle state corruption gracefully
if self.validate_state(context, &state).is_err() {
    // Reset to known good state
    let recovery_state = self.initialize_state(context);
    context.joker_state_manager.update_state(self.id(), |s| *s = recovery_state);
}
```

## Best Practices

### Performance

1. **Early Returns**: Exit early from non-matching conditions
2. **Guard Clauses**: Validate inputs before expensive operations
3. **Caching**: Cache frequently accessed state values
4. **Avoid Allocations**: Use static strings and pre-allocated effects
5. **Batching**: Process multiple items together when possible

### Reliability

1. **Input Validation**: Always validate inputs and state
2. **Error Handling**: Provide graceful fallbacks for errors
3. **State Validation**: Implement comprehensive state validation
4. **Recovery Mechanisms**: Handle corruption and migration
5. **Testing**: Add comprehensive unit and integration tests

### Maintainability

1. **Clear Naming**: Use descriptive names matching Balatro conventions
2. **Documentation**: Document complex behavior and edge cases
3. **Modularity**: Group related jokers in modules
4. **Consistency**: Follow established patterns and conventions
5. **Version Management**: Handle state migration properly

### Implementation Guidelines

#### Trait Implementation Checklist

- [ ] Implement all required trait methods
- [ ] Use appropriate rarity and cost
- [ ] Add comprehensive documentation
- [ ] Handle edge cases gracefully
- [ ] Implement state validation if stateful
- [ ] Add unit tests for all behavior
- [ ] Follow naming conventions
- [ ] Optimize hot paths

#### Effect Creation Best Practices

```rust
// Prefer builder pattern for effects
JokerEffect::new()
    .with_mult(bonus)
    .with_message("Clear description".to_string())

// Use early returns for performance
if !condition_met {
    return JokerEffect::new(); // Empty effect
}

// Validate bounds
let safe_bonus = bonus.min(MAX_BONUS).max(0);
```

#### State Management Best Practices

```rust
// Initialize state properly
fn initialize_state(&self, context: &GameContext) -> JokerState {
    let mut state = JokerState::new();
    state.accumulated_value = 0.0;
    // Set initial custom data
    let _ = state.set_custom("version", 1u32);
    state
}

// Validate state integrity
fn validate_state(&self, _context: &GameContext, state: &JokerState) -> Result<(), String> {
    if state.accumulated_value < 0.0 {
        return Err("Accumulated value cannot be negative".to_string());
    }
    Ok(())
}
```

## Testing Strategies

### Unit Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_joker_basic_properties() {
        let joker = MyJoker;
        assert_eq!(joker.name(), "Expected Name");
        assert_eq!(joker.rarity(), JokerRarity::Common);
        assert!(joker.cost() > 0);
    }

    #[test]
    fn test_effect_generation() {
        let joker = MyJoker;
        // Test with various game states
        // Verify correct effects are generated
    }
}
```

### Integration Testing

Use the test harness example as a template for comprehensive testing:

1. **Factory Testing**: Verify all jokers can be created
2. **Trait Testing**: Validate all required methods work
3. **Effect Testing**: Ensure effects are generated correctly
4. **State Testing**: Verify state management works
5. **Performance Testing**: Benchmark critical paths

### Manual Testing

1. **Game Integration**: Test in actual game scenarios
2. **Edge Cases**: Test boundary conditions
3. **User Experience**: Verify descriptions are clear
4. **Balance**: Ensure effects are appropriately powerful

## Migration Guide

When updating joker implementations:

1. **Backup State**: Always backup existing save states
2. **Version Migration**: Implement proper state migration
3. **Backward Compatibility**: Maintain compatibility when possible
4. **Testing**: Test migration with real save data
5. **Documentation**: Document breaking changes

## Performance Guidelines

### Critical Paths

These methods are called frequently and should be optimized:

1. `on_card_scored()` - Called for each scoring card
2. `on_hand_played()` - Called for each played hand
3. State access methods - Called during scoring

### Optimization Techniques

1. **Early Returns**: Exit immediately for non-matching conditions
2. **Precomputation**: Calculate values once and cache
3. **Static Data**: Use const values for fixed bonuses
4. **Bit Operations**: Use bitwise operations for suit checks
5. **Avoid String Allocation**: Use static strings for messages

## Common Pitfalls

### Anti-Patterns to Avoid

1. **Complex Nested Conditions**: Use early returns instead
2. **Unnecessary Allocations**: Avoid string creation in hot paths
3. **State Access Patterns**: Don't repeatedly access state
4. **Missing Validation**: Always validate inputs and state
5. **Poor Error Handling**: Don't ignore validation failures
6. **Memory Leaks**: Clean up resources properly
7. **Race Conditions**: Use thread-safe patterns

### Debug Strategies

1. **Logging**: Add debug output for complex logic
2. **Assertions**: Use assertions for invariants
3. **State Inspection**: Implement debug state dumps
4. **Test Isolation**: Test jokers in isolation
5. **Performance Profiling**: Profile hot paths

## Conclusion

The joker examples provide a comprehensive foundation for implementing jokers in balatro-rs. By following the patterns and best practices demonstrated in these examples, developers can create efficient, reliable, and maintainable joker implementations.

### Key Takeaways

1. **Choose the Right Pattern**: Static framework for simple jokers, direct implementation for complex ones
2. **Optimize Hot Paths**: Focus on `on_card_scored` and `on_hand_played` performance
3. **Handle Edge Cases**: Implement robust validation and error handling
4. **Test Thoroughly**: Use automated testing to catch regressions
5. **Follow Conventions**: Maintain consistency with existing joker implementations

### Next Steps

1. Study the existing examples
2. Run the test harness to understand the testing approach
3. Implement your own jokers following the demonstrated patterns
4. Add comprehensive tests for your implementations
5. Optimize performance using the techniques shown in the examples

For more information, refer to the individual example files and the comprehensive trait documentation in the source code.