# Joker Performance Tuning Guide

## Overview

The joker system is designed for high-performance gaming and reinforcement learning applications. This guide covers optimization strategies, performance patterns, and benchmarking techniques to ensure your joker implementations run efficiently.

## Performance Fundamentals

### Critical Paths

Understanding which operations happen most frequently helps prioritize optimizations:

1. **Card Scoring** (highest frequency): Called for every scoring card
2. **Hand Playing** (high frequency): Called for every hand played
3. **State Updates** (medium frequency): Called when joker state changes
4. **Shop Operations** (low frequency): Called when entering shop

### Performance Targets

- **Card scoring**: < 1μs per joker per card
- **Hand playing**: < 10μs per joker per hand
- **Memory usage**: < 1KB per joker instance
- **State operations**: < 100ns for simple state access

## Optimization Strategies

### 1. Choose the Right Implementation Pattern

```rust
// FAST: Static jokers for simple conditions
let greedy = StaticJoker::builder(JokerId::GreedyJoker, "Greedy", "Description")
    .mult(3)
    .condition(StaticCondition::SuitScored(Suit::Diamond))
    .per_card()
    .build()?;

// MEDIUM: Manual implementation for complex logic
impl Joker for CustomJoker {
    fn on_card_scored(&self, context: &mut GameContext, card: &Card) -> JokerEffect {
        // Custom logic here
        if self.complex_condition(context, card) {
            JokerEffect::new().with_mult(self.calculate_bonus(card))
        } else {
            JokerEffect::new()
        }
    }
}

// SLOW: Avoid expensive operations in hot paths
impl Joker for BadJoker {
    fn on_card_scored(&self, context: &mut GameContext, card: &Card) -> JokerEffect {
        // DON'T: Expensive operations for every card
        let all_cards = context.hand.cards();
        let complex_analysis = analyze_all_combinations(&all_cards); // Slow!
        JokerEffect::new().with_mult(complex_analysis)
    }
}
```

### 2. Optimize Condition Checking

```rust
// FAST: Simple equality checks
impl StaticCondition {
    fn check_card(&self, card: &Card) -> bool {
        match self {
            StaticCondition::SuitScored(suit) => card.suit == *suit, // O(1)
            StaticCondition::RankScored(rank) => card.value == *rank, // O(1)
            _ => false,
        }
    }
}

// MEDIUM: Small set membership
impl StaticCondition {
    fn check_card(&self, card: &Card) -> bool {
        match self {
            StaticCondition::AnySuitScored(suits) => {
                suits.contains(&card.suit) // O(n) but n is small (≤4)
            }
            _ => false,
        }
    }
}

// SLOW: Avoid large iterations
fn bad_condition_check(context: &GameContext, target: Value) -> bool {
    // DON'T: Iterate through large collections repeatedly
    context.hand.cards().iter()
        .filter(|c| c.value == target)
        .count() >= 3 // Expensive for large hands
}
```

### 3. Early Returns

```rust
impl Joker for OptimizedJoker {
    fn on_card_scored(&self, context: &mut GameContext, card: &Card) -> JokerEffect {
        // Early return for non-matching conditions
        if card.suit != Suit::Diamond {
            return JokerEffect::new(); // Fast exit
        }
        
        // Only do expensive work when necessary
        let bonus = self.calculate_diamond_bonus(context, card);
        JokerEffect::new().with_mult(bonus)
    }
}
```

### 4. State Caching

```rust
impl Joker for CachingJoker {
    fn on_card_scored(&self, context: &mut GameContext, card: &Card) -> JokerEffect {
        let state_manager = JokerStateManager::new();
        
        // Check if we have cached result for this context
        if let Some(cached) = self.get_cached_result(context) {
            return cached; // O(1) cache hit
        }
        
        // Calculate and cache result
        let result = self.expensive_calculation(context, card);
        self.cache_result(context, &result);
        result
    }
    
    fn get_cached_result(&self, context: &GameContext) -> Option<JokerEffect> {
        // Implementation depends on what makes a good cache key
        // For example, money + hand size might be sufficient
        let cache_key = (context.money, context.hand.len());
        self.cache.get(&cache_key).cloned()
    }
}
```

### 5. Batch Operations

```rust
impl Joker for BatchedJoker {
    // Instead of processing each card individually...
    fn on_card_scored(&self, context: &mut GameContext, card: &Card) -> JokerEffect {
        // Store for batch processing
        self.pending_cards.push(card.clone());
        JokerEffect::new()
    }
    
    // Process all cards at once
    fn on_hand_played(&self, context: &mut GameContext, hand: &SelectHand) -> JokerEffect {
        let total_effect = self.process_all_pending_cards();
        self.pending_cards.clear();
        total_effect
    }
}
```

## Memory Optimization

### 1. Minimize Allocations

```rust
// GOOD: Use static strings and avoid allocations
pub struct StaticJoker {
    pub name: &'static str,        // No allocation
    pub description: &'static str, // No allocation
    // ...
}

// BAD: Unnecessary allocations
pub struct BadJoker {
    pub name: String,        // Heap allocation
    pub description: String, // Heap allocation
    // ...
}
```

### 2. Efficient State Management

```rust
// GOOD: Only allocate state when needed
impl JokerStateManager {
    pub fn get_state(&self, joker_id: &JokerId) -> Option<JokerState> {
        self.states.read().unwrap().get(joker_id).cloned()
    }
    
    pub fn ensure_state(&self, joker_id: JokerId) {
        let mut states = self.states.write().unwrap();
        states.entry(joker_id).or_insert_with(JokerState::new);
    }
}

// BAD: Pre-allocate all possible states
impl BadStateManager {
    pub fn new() -> Self {
        let mut states = HashMap::new();
        // DON'T: Allocate states for all 159 jokers
        for id in all_joker_ids() {
            states.insert(id, JokerState::new());
        }
        Self { states }
    }
}
```

### 3. Memory-Efficient Data Structures

```rust
// GOOD: Use appropriate data structures
use rustc_hash::FxHashMap; // Faster than std::HashMap for small keys

pub struct OptimizedStateManager {
    // Use FxHashMap for better performance with enum keys
    states: RwLock<FxHashMap<JokerId, JokerState>>,
}

// GOOD: Pack data efficiently
#[repr(C)]
pub struct PackedJokerEffect {
    chips: i16,           // Usually small values
    mult: i16,            // Usually small values  
    money: i16,           // Usually small values
    mult_multiplier: f32, // Needs full precision
}
```

## Profiling and Benchmarking

### 1. Micro-Benchmarks

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_greedy_joker(c: &mut Criterion) {
    let joker = StaticJokerFactory::create_greedy_joker();
    let diamond = Card::new(Value::Ace, Suit::Diamond);
    let heart = Card::new(Value::King, Suit::Heart);
    
    c.bench_function("greedy_joker_diamond", |b| {
        b.iter(|| {
            let context = create_benchmark_context();
            black_box(joker.on_card_scored(black_box(&mut context), black_box(&diamond)))
        })
    });
    
    c.bench_function("greedy_joker_non_diamond", |b| {
        b.iter(|| {
            let context = create_benchmark_context();
            black_box(joker.on_card_scored(black_box(&mut context), black_box(&heart)))
        })
    });
}

criterion_group!(benches, bench_greedy_joker);
criterion_main!(benches);
```

### 2. Integration Benchmarks

```rust
fn bench_full_hand_scoring(c: &mut Criterion) {
    let mut game = Game::new(Config::default());
    
    // Add multiple jokers
    game.add_joker(JokerId::GreedyJoker);
    game.add_joker(JokerId::JollyJoker);
    game.add_joker(JokerId::BasicJoker);
    
    let hand = vec![
        Card::new(Value::Ace, Suit::Diamond),
        Card::new(Value::Ace, Suit::Heart),
        Card::new(Value::King, Suit::Diamond),
        Card::new(Value::Queen, Suit::Spade),
        Card::new(Value::Jack, Suit::Club),
    ];
    
    c.bench_function("full_hand_scoring", |b| {
        b.iter(|| {
            black_box(game.play_hand(black_box(hand.clone())))
        })
    });
}
```

### 3. Memory Profiling

```rust
// Use for memory usage analysis
#[cfg(feature = "memory_profiling")]
fn profile_memory_usage() {
    let initial_memory = get_memory_usage();
    
    // Create many jokers
    let mut jokers = Vec::new();
    for _ in 0..1000 {
        jokers.push(StaticJokerFactory::create_greedy_joker());
    }
    
    let final_memory = get_memory_usage();
    println!("Memory per joker: {} bytes", 
             (final_memory - initial_memory) / 1000);
}
```

## Performance Patterns

### 1. Lazy Evaluation

```rust
impl Joker for LazyJoker {
    fn on_hand_played(&self, context: &mut GameContext, hand: &SelectHand) -> JokerEffect {
        // Only calculate if we know we'll use the result
        if !self.should_activate(context) {
            return JokerEffect::new();
        }
        
        // Expensive calculation only when needed
        let bonus = self.calculate_complex_bonus(context, hand);
        JokerEffect::new().with_mult(bonus)
    }
}
```

### 2. Pre-computation

```rust
impl Joker for PrecomputedJoker {
    fn new() -> Self {
        // Pre-compute lookup tables for expensive operations
        let lookup_table = Self::build_lookup_table();
        
        Self {
            lookup_table,
            // ... other fields
        }
    }
    
    fn on_card_scored(&self, context: &mut GameContext, card: &Card) -> JokerEffect {
        // O(1) lookup instead of O(n) calculation
        let bonus = self.lookup_table[&card.value];
        JokerEffect::new().with_mult(bonus)
    }
}
```

### 3. Incremental Updates

```rust
impl Joker for IncrementalJoker {
    fn on_card_scored(&self, context: &mut GameContext, card: &Card) -> JokerEffect {
        // Update running totals incrementally
        let state_manager = JokerStateManager::new();
        
        state_manager.update_state(&self.id(), |state| {
            match card.suit {
                Suit::Diamond => state.accumulated_value += 1,
                _ => {}
            }
        });
        
        JokerEffect::new()
    }
    
    fn on_hand_played(&self, context: &mut GameContext, hand: &SelectHand) -> JokerEffect {
        // Use pre-computed totals
        let state_manager = JokerStateManager::new();
        let diamond_count = state_manager.get_state(&self.id())
            .map(|s| s.accumulated_value)
            .unwrap_or(0.0);
        
        JokerEffect::new().with_mult(diamond_count * 3)
    }
}
```

## Anti-Patterns for Performance

### 1. Unnecessary Cloning

```rust
// BAD: Excessive cloning
fn bad_implementation(cards: &[Card]) -> Vec<Card> {
    cards.iter().cloned().collect() // Unnecessary allocation
}

// GOOD: Use references when possible
fn good_implementation(cards: &[Card]) -> usize {
    cards.len() // No allocation needed
}
```

### 2. String Operations in Hot Paths

```rust
// BAD: String formatting in performance-critical code
impl Joker for BadJoker {
    fn on_card_scored(&self, context: &mut GameContext, card: &Card) -> JokerEffect {
        let debug_info = format!("Processing card: {}", card); // Slow!
        log::debug!("{}", debug_info);
        // ... rest of implementation
    }
}

// GOOD: Use structured logging or conditional compilation
impl Joker for GoodJoker {
    fn on_card_scored(&self, context: &mut GameContext, card: &Card) -> JokerEffect {
        #[cfg(debug_assertions)]
        log::debug!("Processing card: {:?}", card); // Only in debug builds
        
        // ... rest of implementation
    }
}
```

### 3. Unnecessary HashMap Operations

```rust
// BAD: Creating HashMaps for simple lookups
fn bad_lookup(suit: Suit) -> i32 {
    let mut map = HashMap::new();
    map.insert(Suit::Diamond, 3);
    map.insert(Suit::Heart, 2);
    map.insert(Suit::Spade, 1);
    map.insert(Suit::Club, 0);
    map[&suit] // Expensive for simple lookup
}

// GOOD: Use match statements for simple mappings
fn good_lookup(suit: Suit) -> i32 {
    match suit {
        Suit::Diamond => 3,
        Suit::Heart => 2,
        Suit::Spade => 1,
        Suit::Club => 0,
    }
}
```

## Production Optimization Checklist

### Pre-Deployment

- [ ] Profile critical paths with real game data
- [ ] Benchmark joker combinations under load
- [ ] Test memory usage with maximum jokers (5+ slots)
- [ ] Verify no memory leaks during long sessions
- [ ] Test performance with complex joker interactions

### Runtime Monitoring

```rust
// Add performance monitoring hooks
pub struct PerformanceMonitor {
    card_scoring_times: Vec<Duration>,
    hand_playing_times: Vec<Duration>,
    memory_usage: usize,
}

impl PerformanceMonitor {
    pub fn report_card_scoring_time(&mut self, duration: Duration) {
        if duration > Duration::from_micros(10) {
            log::warn!("Slow card scoring: {:?}", duration);
        }
        self.card_scoring_times.push(duration);
    }
    
    pub fn get_average_card_scoring_time(&self) -> Duration {
        let total: Duration = self.card_scoring_times.iter().sum();
        total / self.card_scoring_times.len() as u32
    }
}
```

### Performance Regression Testing

```rust
#[test]
fn performance_regression_test() {
    let start = Instant::now();
    
    // Run standardized performance test
    let result = run_standard_game_simulation();
    
    let duration = start.elapsed();
    
    // Assert performance doesn't regress beyond acceptable threshold
    assert!(duration < Duration::from_millis(100), 
            "Performance regression detected: {:?}", duration);
    
    // Also check result correctness
    assert_eq!(result.final_score, EXPECTED_SCORE);
}
```

Following these optimization strategies will ensure your joker implementations maintain high performance even under heavy load in RL training scenarios.