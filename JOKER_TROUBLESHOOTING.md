# Joker System Troubleshooting Guide

## Common Issues and Solutions

### Compilation Issues

#### Issue: "Trait `Joker` is not implemented"

```rust
error[E0277]: the trait `Joker` is not implemented for `MyJoker`
```

**Cause**: Missing trait implementation.

**Solution**: Implement all required trait methods:

```rust
impl Joker for MyJoker {
    fn id(&self) -> JokerId { JokerId::MyJoker }
    fn name(&self) -> &str { "My Joker" }
    fn description(&self) -> &str { "Description" }
    fn rarity(&self) -> JokerRarity { JokerRarity::Common }
    
    // Optional: Override lifecycle methods as needed
    fn on_card_scored(&self, context: &mut GameContext, card: &Card) -> JokerEffect {
        // Your implementation
        JokerEffect::new()
    }
}
```

#### Issue: "Cannot move out of borrowed content"

```rust
error[E0507]: cannot move out of `*context.jokers` which is behind a shared reference
```

**Cause**: Trying to move data from borrowed references.

**Solution**: Use references instead of moving:

```rust
// BAD: Trying to move from borrowed reference
let joker = context.jokers[0]; // Error!

// GOOD: Use references
let joker = &context.jokers[0];

// GOOD: Clone if you need ownership
let joker_id = context.jokers[0].id(); // Copy the ID instead
```

#### Issue: "Static joker build failure"

```rust
error: "HandType conditions require per_hand, not per_card"
```

**Cause**: Logical mismatch between condition and application mode.

**Solution**: Match condition types with appropriate modes:

```rust
// BAD: Hand conditions with per_card
StaticJoker::builder(id, name, desc)
    .condition(StaticCondition::HandType(HandRank::Pair))
    .per_card() // Error! Hand types are per-hand
    .build()

// GOOD: Hand conditions with per_hand
StaticJoker::builder(id, name, desc)
    .condition(StaticCondition::HandType(HandRank::Pair))
    .per_hand() // Correct!
    .build()

// GOOD: Card conditions with per_card
StaticJoker::builder(id, name, desc)
    .condition(StaticCondition::SuitScored(Suit::Diamond))
    .per_card() // Correct!
    .build()
```

### Runtime Issues

#### Issue: Joker not triggering

**Symptoms**: Joker exists in game but doesn't apply effects.

**Debug Steps**:

1. **Check condition logic**:
```rust
// Add debug logging to condition checking
fn check_card_condition(&self, card: &Card) -> bool {
    let result = match &self.condition {
        StaticCondition::SuitScored(suit) => card.suit == *suit,
        // ... other conditions
    };
    
    #[cfg(debug_assertions)]
    eprintln!("Joker {} condition check: card={:?}, result={}", 
              self.name(), card, result);
    
    result
}
```

2. **Verify joker is in collection**:
```rust
// Check if joker is actually in the game
let joker_count = context.jokers.iter()
    .filter(|j| j.id() == JokerId::MyJoker)
    .count();
eprintln!("Found {} instances of MyJoker", joker_count);
```

3. **Check lifecycle method**:
```rust
// Ensure you're implementing the right lifecycle method
impl Joker for MyJoker {
    // For per-card effects, implement this:
    fn on_card_scored(&self, context: &mut GameContext, card: &Card) -> JokerEffect {
        // Your logic here
    }
    
    // For per-hand effects, implement this:
    fn on_hand_played(&self, context: &mut GameContext, hand: &SelectHand) -> JokerEffect {
        // Your logic here
    }
}
```

#### Issue: Incorrect effect calculation

**Symptoms**: Joker triggers but bonus is wrong.

**Debug Steps**:

1. **Trace effect calculation**:
```rust
fn on_card_scored(&self, context: &mut GameContext, card: &Card) -> JokerEffect {
    let effect = if card.suit == Suit::Diamond {
        JokerEffect::new().with_mult(3)
    } else {
        JokerEffect::new()
    };
    
    #[cfg(debug_assertions)]
    eprintln!("Joker {} effect: {:?}", self.name(), effect);
    
    effect
}
```

2. **Check effect application order**:
```rust
// Effects are applied in joker order (left to right)
// Make sure your joker is in the expected position
for (i, joker) in context.jokers.iter().enumerate() {
    eprintln!("Slot {}: {}", i, joker.name());
}
```

#### Issue: State not persisting

**Symptoms**: Joker state resets unexpectedly.

**Solutions**:

1. **Check state manager usage**:
```rust
// WRONG: Creating new state manager each time
fn on_card_scored(&self, context: &mut GameContext, card: &Card) -> JokerEffect {
    let state_manager = JokerStateManager::new(); // Creates new instance!
    // State won't persist
}

// CORRECT: Use shared state manager
lazy_static! {
    static ref STATE_MANAGER: JokerStateManager = JokerStateManager::new();
}

fn on_card_scored(&self, context: &mut GameContext, card: &Card) -> JokerEffect {
    STATE_MANAGER.update_state(&self.id(), |state| {
        state.accumulated_value += 1;
    });
    JokerEffect::new()
}
```

2. **Verify state initialization**:
```rust
// Check if state exists before trying to use it
fn on_blind_start(&self, context: &mut GameContext) -> JokerEffect {
    let state_manager = JokerStateManager::new();
    if state_manager.has_state(self.id()) {
        // State exists, operations will work
    }
    JokerEffect::new()
}
```

### Performance Issues

#### Issue: Slow card scoring

**Symptoms**: Game lag during hand scoring.

**Debug Steps**:

1. **Profile joker performance**:
```rust
use std::time::Instant;

fn on_card_scored(&self, context: &mut GameContext, card: &Card) -> JokerEffect {
    let start = Instant::now();
    
    let effect = self.calculate_effect(context, card);
    
    let duration = start.elapsed();
    if duration > Duration::from_micros(10) {
        eprintln!("Slow joker {}: {:?}", self.name(), duration);
    }
    
    effect
}
```

2. **Check for expensive operations**:
```rust
// BAD: O(n²) operations in card scoring
fn on_card_scored(&self, context: &mut GameContext, card: &Card) -> JokerEffect {
    for joker in context.jokers {
        for other_card in context.hand.cards() {
            // Expensive nested loop for every card!
        }
    }
    JokerEffect::new()
}

// GOOD: Pre-compute expensive operations
fn on_hand_played(&self, context: &mut GameContext, hand: &SelectHand) -> JokerEffect {
    // Do expensive work once per hand
    let precomputed = self.expensive_calculation(context);
    self.cache_result(precomputed);
    JokerEffect::new()
}

fn on_card_scored(&self, context: &mut GameContext, card: &Card) -> JokerEffect {
    // Use cached result
    let cached = self.get_cached_result();
    JokerEffect::new().with_mult(cached * self.card_multiplier(card))
}
```

#### Issue: Memory leaks

**Symptoms**: Memory usage grows over time.

**Solutions**:

1. **Check for state cleanup**:
```rust
fn on_round_end(&self, context: &mut GameContext) -> JokerEffect {
    // Clean up temporary state
    self.clear_temporary_data();
    JokerEffect::new()
}
```

2. **Avoid unnecessary allocations**:
```rust
// BAD: Creating new vectors repeatedly
fn bad_implementation(&self, cards: &[Card]) -> i32 {
    let diamonds: Vec<&Card> = cards.iter()
        .filter(|c| c.suit == Suit::Diamond)
        .collect(); // Unnecessary allocation
    diamonds.len() as i32
}

// GOOD: Use iterators directly
fn good_implementation(&self, cards: &[Card]) -> i32 {
    cards.iter()
        .filter(|c| c.suit == Suit::Diamond)
        .count() as i32 // No allocation
}
```

### Integration Issues

#### Issue: PyO3 binding problems

**Symptoms**: Python integration fails or crashes.

**Solutions**:

1. **Check trait object safety**:
```rust
// Ensure your joker trait implementations are object-safe
impl Joker for MyJoker {
    // All methods use &self, not Self
    // No generic parameters on methods
    // No associated types
}
```

2. **Verify serialization**:
```rust
// Ensure joker types can be serialized for Python
#[derive(Serialize, Deserialize)]
pub struct SerializableJoker {
    id: JokerId,
    state: Option<JokerState>,
}
```

#### Issue: Game engine integration

**Symptoms**: Jokers don't integrate properly with game flow.

**Solutions**:

1. **Check action handling**:
```rust
// Ensure BuyJoker actions create jokers correctly
match action {
    Action::BuyJoker { joker_id, slot } => {
        if let Some(joker) = JokerFactory::create(joker_id) {
            self.jokers.insert(slot, joker);
        }
    }
    // ... handle other actions
}
```

2. **Verify lifecycle calls**:
```rust
// Game engine should call joker methods at appropriate times
fn play_hand(&mut self, hand: SelectHand) -> GameResult {
    // Call on_hand_played for all jokers
    for joker in &self.jokers {
        let effect = joker.on_hand_played(&mut self.context, &hand);
        self.apply_effect(effect);
    }
    
    // Score each card
    for card in hand.cards() {
        for joker in &self.jokers {
            let effect = joker.on_card_scored(&mut self.context, &card);
            self.apply_effect(effect);
        }
    }
    
    // Calculate final score...
}
```

## Testing and Debugging Tools

### Unit Test Helpers

```rust
// Helper for creating test contexts
pub fn create_test_context() -> GameContext<'static> {
    GameContext {
        chips: 0,
        mult: 1,
        money: 50,
        ante: 1,
        round: 1,
        stage: &Stage::Blind,
        hands_played: 0,
        discards_used: 0,
        jokers: &[],
        hand: &Hand::new(),
        discarded: &[],
    }
}

// Helper for creating test hands
pub fn create_diamond_pair() -> SelectHand {
    SelectHand::new(vec![
        Card::new(Value::Ace, Suit::Diamond),
        Card::new(Value::Ace, Suit::Heart),
    ])
}

// Test template
#[test]
fn test_joker_behavior() {
    let joker = MyJoker::new();
    let mut context = create_test_context();
    let card = Card::new(Value::Ace, Suit::Diamond);
    
    let effect = joker.on_card_scored(&mut context, &card);
    
    assert_eq!(effect.mult, 3);
    assert_eq!(effect.chips, 0);
}
```

### Debug Macros

```rust
// Debug macro for joker effects
#[cfg(debug_assertions)]
macro_rules! debug_effect {
    ($joker:expr, $effect:expr) => {
        eprintln!("Joker '{}' produced effect: chips={}, mult={}, money={}", 
                 $joker.name(), $effect.chips, $effect.mult, $effect.money);
    };
}

// Usage in joker implementation
fn on_card_scored(&self, context: &mut GameContext, card: &Card) -> JokerEffect {
    let effect = JokerEffect::new().with_mult(3);
    debug_effect!(self, effect);
    effect
}
```

### Performance Monitoring

```rust
// Simple performance counter
pub struct PerformanceCounter {
    calls: AtomicUsize,
    total_time: AtomicU64,
}

impl PerformanceCounter {
    pub fn time_call<F, R>(&self, f: F) -> R
    where F: FnOnce() -> R {
        let start = Instant::now();
        let result = f();
        let duration = start.elapsed();
        
        self.calls.fetch_add(1, Ordering::Relaxed);
        self.total_time.fetch_add(duration.as_nanos() as u64, Ordering::Relaxed);
        
        result
    }
    
    pub fn average_time(&self) -> Duration {
        let calls = self.calls.load(Ordering::Relaxed);
        let total = self.total_time.load(Ordering::Relaxed);
        
        if calls > 0 {
            Duration::from_nanos(total / calls as u64)
        } else {
            Duration::from_nanos(0)
        }
    }
}
```

## Best Practices for Avoiding Issues

### Code Review Checklist

- [ ] **Trait implementation**: All required methods implemented
- [ ] **Condition logic**: Matches expected behavior and is efficient
- [ ] **State management**: Proper use of state manager if needed
- [ ] **Performance**: No expensive operations in hot paths
- [ ] **Memory**: No unnecessary allocations or leaks
- [ ] **Testing**: Unit tests cover all code paths
- [ ] **Documentation**: Clear description of joker behavior

### Development Workflow

1. **Start with tests**: Write tests before implementing joker logic
2. **Use static framework**: For simple jokers, prefer static framework
3. **Profile early**: Benchmark performance-critical jokers
4. **Debug incrementally**: Add logging to trace execution
5. **Test integration**: Verify joker works in full game context

### Common Anti-Patterns

- **Don't** implement expensive logic in `on_card_scored`
- **Don't** mutate context directly (use effects instead)
- **Don't** assume joker ordering (make position-independent)
- **Don't** use unwrap() in production code
- **Don't** create new state managers in each method call

Following these guidelines will help you avoid common pitfalls and create reliable, performant joker implementations.