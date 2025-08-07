# CLAUDE.md - Examples

## Directory Purpose

The examples directory provides comprehensive code demonstrations, patterns, and educational content for developers working with the balatro-rs engine. These examples serve as both documentation and learning resources.

## Example Organization

### Core Learning Examples
- `joker_comprehensive_guide.rs`: Complete joker system tutorial covering all aspects
- `test_harness.rs`: Testing patterns and validation strategies for jokers
- `new_joker_api.rs`: Demonstrates the latest joker API patterns

### Advanced Patterns
- `trait_composition_examples.rs`: Complex jokers using multiple traits (Aristocrat, Scholar, Magician patterns)
- `performance_examples.rs`: Optimization patterns comparing efficient vs inefficient implementations
- `edge_case_examples.rs`: Defensive programming for boundary conditions and error handling

### Specialized Examples
- `card_targeting_demo.rs`: Card selection and targeting mechanics

## Key Patterns Demonstrated

### Zero-Allocation Patterns
```rust
// Efficient: Stack allocation
let effect = JokerEffect {
    mult: 10.0,
    ..Default::default()
};

// Inefficient: Heap allocation
let effect = Box::new(JokerEffect::new());
```

### Early Return Optimization
```rust
// Efficient: Early return
if !self.can_trigger(context) {
    return ProcessResult::default();
}

// Inefficient: Nested logic
if self.can_trigger(context) {
    // Deep nesting...
}
```

### State Caching Strategies
```rust
struct CachedJoker {
    cached_value: Option<f64>,
    cache_valid: bool,
}

impl CachedJoker {
    fn get_value(&mut self) -> f64 {
        if !self.cache_valid {
            self.cached_value = Some(self.calculate());
            self.cache_valid = true;
        }
        self.cached_value.unwrap()
    }
}
```

## Trait Composition Examples

### Multi-Trait Jokers
```rust
// Aristocrat Pattern: Identity + Gameplay + State
struct AristocratJoker {
    mult_per_king: f64,
    total_kings: u32,
}

// Scholar Pattern: All 5 traits
struct ScholarJoker {
    knowledge_level: u32,
    books_read: Vec<BookId>,
}

// Magician Pattern: Lifecycle + special effects
struct MagicianJoker {
    tricks_performed: u32,
    audience_size: u32,
}
```

## Error Handling Examples

### Defensive Programming
```rust
// Boundary checking
if cards.is_empty() {
    return Err(JokerError::NoCards);
}

// Safe division
let average = if count > 0 {
    total / count
} else {
    0.0
};

// Overflow protection
let new_value = self.value.saturating_add(increment);
```

## Special Joker Patterns

### Self-Destructing Jokers
```rust
fn process(&mut self) -> ProcessResult {
    self.uses_remaining -= 1;
    if self.uses_remaining == 0 {
        return ProcessResult {
            destroy_self: true,
            ..Default::default()
        };
    }
    // Normal processing...
}
```

### Conditional Activation
```rust
fn can_trigger(&self, context: &GameContext) -> bool {
    context.money >= self.min_money
        && context.hand_type == HandType::Flush
        && context.ante > 3
}
```

## Running Examples

```bash
# Run specific example
cargo run --example joker_comprehensive_guide

# Run with output
cargo run --example performance_examples 2>&1 | less

# List all examples
ls core/examples/*.rs
```

## Educational Value

### Learning Progression
1. Start with `joker_comprehensive_guide.rs`
2. Study `trait_composition_examples.rs`
3. Analyze `performance_examples.rs`
4. Practice with `test_harness.rs`
5. Handle edge cases with `edge_case_examples.rs`

### Best Practices Demonstrated
- Clear variable naming
- Comprehensive documentation
- Error handling patterns
- Performance optimization
- Testing strategies
- Design patterns

## Integration Examples

Shows how to integrate with:
- Game state management
- Shop system
- Scoring pipeline
- Save/load system
- Python bindings
