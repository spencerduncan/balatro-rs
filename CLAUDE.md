# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

balatro-rs is a game engine and move generator for a simplified version of Balatro, written in Rust with Python bindings. The project is designed as a move generator for reinforcement learning applications.

**GitHub Repository**: https://github.com/spencerduncan/balatro-rs (fork of the original evanofslack/balatro-rs)

## Development Commands

### Building and Testing
```bash
# Build entire workspace
cargo build
cargo build --release

# Test entire workspace
cargo test
cargo test --verbose

# Run benchmarks (core library)
cargo bench

# Lint and format
cargo clippy --all -- -D warnings
cargo fmt
```

### Working with specific packages
```bash
# Core library
cargo build -p balatro-rs
cargo test -p balatro-rs

# CLI application
cargo run -p balatro-cli

# Python bindings
cargo build -p pylatro
```

### Python Extension Development
```bash
cd balatro-rs/pylatro
source .env/bin/activate  # or run ./setup.sh first time
maturin develop          # Build and install in dev mode
python test/main.py      # Run tests
python examples/simulation.py  # Run example
```

## Architecture

### Workspace Structure
- **balatro-rs/** - Root workspace directory
  - **core/** - Main game engine library (`balatro-rs` crate)
  - **cli/** - Command-line interface application
  - **pylatro/** - Python bindings using PyO3

### Core Game Architecture

The game engine follows a state machine design with these key components:

1. **Game** (`game.rs`) - Central game state and logic coordinator
2. **Action** (`action.rs`) - Enumeration of all possible game actions
3. **Stage** (`stage.rs`) - Game flow states (PreBlind, Blind, PostBlind, Shop, End)
4. **Generator** (`generator.rs`) - Move generation for AI/RL applications
5. **Joker System** (`joker/`) - Modular trait-based joker architecture

### New Trait-Based Joker Architecture

The joker system has been refactored into 5 focused, single-responsibility traits that replace the monolithic `Joker` trait:

#### 1. **JokerIdentity** (6 methods)
Core identity and metadata for jokers:
- `joker_type()` - Unique type identifier
- `name()` - Display name
- `description()` - Effect description
- `rarity()` - Rarity level (Common, Uncommon, Rare, Legendary)
- `base_cost()` - Shop cost
- `is_unique()` - Legendary variant flag

#### 2. **JokerLifecycle** (7 methods)
Lifecycle event management:
- `on_purchase()` - When bought from shop
- `on_sell()` - When sold
- `on_destroy()` - When destroyed
- `on_round_start()` - Beginning of each round
- `on_round_end()` - End of each round
- `on_joker_added()` - When another joker joins
- `on_joker_removed()` - When another joker leaves

#### 3. **JokerGameplay** (3 methods)
Core gameplay mechanics:
- `process()` - Core effect processing during game stages
- `can_trigger()` - Trigger condition checking
- `get_priority()` - Processing order priority (higher = earlier)

#### 4. **JokerModifiers** (4 methods)
Game state modifications:
- `get_chip_mult()` - Chip multiplier (default 1.0)
- `get_score_mult()` - Score multiplier (default 1.0)
- `get_hand_size_modifier()` - Hand size changes (default 0)
- `get_discard_modifier()` - Discard count changes (default 0)

#### 5. **JokerState** (5 methods)
State management and persistence:
- `has_state()` - State presence check
- `serialize_state()` - State serialization to JSON
- `deserialize_state()` - State deserialization from JSON
- `debug_state()` - Debug representation
- `reset_state()` - State reset to initial values

### Trait Composition Patterns

**Single Trait Implementation:**
```rust
impl JokerIdentity for MyJoker {
    fn joker_type(&self) -> &'static str { "my_joker" }
    fn name(&self) -> &str { "My Joker" }
    fn description(&self) -> &str { "+1 Mult" }
    fn rarity(&self) -> Rarity { Rarity::Common }
    fn base_cost(&self) -> u64 { 3 }
}
```

**Multiple Trait Implementation:**
```rust
impl JokerIdentity for ComplexJoker { /* ... */ }
impl JokerGameplay for ComplexJoker { /* ... */ }
impl JokerState for ComplexJoker { /* ... */ }
```

**Default Implementations:**
All traits provide sensible defaults for optional methods, enabling minimal implementations for simple jokers.

### Key Design Patterns

1. **Move Generator Pattern**: The engine provides two APIs for action enumeration:
   - `gen_actions()` - Returns iterator of valid Action enums
   - `gen_action_space()` - Returns binary vector for RL frameworks

2. **State Machine**: Game progression through well-defined stages with specific valid actions per stage

3. **Immutable Action History**: All actions are recorded for replay and analysis

4. **Trait Composition**: Jokers implement only the traits they need, promoting modularity and single responsibility

5. **Backward Compatibility**: Legacy `Joker` trait remains available during transition period

### Python Integration

PyO3 bindings expose a thin wrapper around Rust types:
- `GameEngine` - Main game interface
- `GameState` - Read-only game state snapshot
- `Action` - Direct mapping to Rust Action enum

All computation stays in Rust; Python is purely for interfacing with ML frameworks.

## Joker Implementation Guidelines

### When to Use Which Traits

**Choose traits based on joker functionality:**

1. **All jokers must implement `JokerIdentity`** - This is mandatory for basic joker information
2. **Simple scoring jokers:** `JokerIdentity` + `JokerGameplay` 
3. **Jokers with modifiers:** `JokerIdentity` + `JokerModifiers`
4. **Stateful jokers:** `JokerIdentity` + `JokerState` + relevant gameplay traits
5. **Event-driven jokers:** `JokerIdentity` + `JokerLifecycle` + relevant traits
6. **Complex jokers:** Implement all 5 traits for maximum flexibility

### Trait Selection Decision Tree

```
Does the joker need basic info? → Yes → Implement JokerIdentity (mandatory)
    ↓
Does it trigger during gameplay? → Yes → Implement JokerGameplay
    ↓
Does it modify base values? → Yes → Implement JokerModifiers  
    ↓
Does it have internal state? → Yes → Implement JokerState
    ↓
Does it respond to lifecycle events? → Yes → Implement JokerLifecycle
```

### Common Implementation Patterns

#### Pattern 1: Simple Scoring Joker
```rust
#[derive(Debug)]
struct GreedyJoker;

impl JokerIdentity for GreedyJoker {
    fn joker_type(&self) -> &'static str { "greedy" }
    fn name(&self) -> &str { "Greedy Joker" }
    fn description(&self) -> &str { "+3 Mult per Diamond scored" }
    fn rarity(&self) -> Rarity { Rarity::Common }
    fn base_cost(&self) -> u64 { 3 }
}

impl JokerGameplay for GreedyJoker {
    fn process(&mut self, stage: &Stage, context: &mut ProcessContext) -> ProcessResult {
        if *stage == Stage::Blind {
            let diamond_count = context.played_cards
                .iter()
                .filter(|card| card.suit == Suit::Diamond)
                .count();
            ProcessResult {
                mult_added: (diamond_count * 3) as f64,
                ..Default::default()
            }
        } else {
            ProcessResult::default()
        }
    }
    
    fn can_trigger(&self, stage: &Stage, _context: &ProcessContext) -> bool {
        *stage == Stage::Blind
    }
}
```

#### Pattern 2: Modifier Joker
```rust
#[derive(Debug)]
struct FourFingersJoker;

impl JokerIdentity for FourFingersJoker {
    fn joker_type(&self) -> &'static str { "four_fingers" }
    fn name(&self) -> &str { "Four Fingers" }
    fn description(&self) -> &str { "All Flushes and Straights can be made with 4 cards" }
    fn rarity(&self) -> Rarity { Rarity::Uncommon }
    fn base_cost(&self) -> u64 { 6 }
}

impl JokerModifiers for FourFingersJoker {
    fn get_hand_size_modifier(&self) -> i32 {
        -1  // Effectively allows 4-card hands for Flushes/Straights
    }
}
```

#### Pattern 3: Stateful Joker
```rust
#[derive(Debug)]
struct ScalingJoker {
    level: u32,
}

impl JokerIdentity for ScalingJoker {
    fn joker_type(&self) -> &'static str { "scaling" }
    fn name(&self) -> &str { "Scaling Joker" }
    fn description(&self) -> &str { "Gains +1 Mult per round" }
    fn rarity(&self) -> Rarity { Rarity::Rare }
    fn base_cost(&self) -> u64 { 8 }
}

impl JokerGameplay for ScalingJoker {
    fn process(&mut self, stage: &Stage, _context: &mut ProcessContext) -> ProcessResult {
        if *stage == Stage::Blind {
            ProcessResult {
                mult_added: self.level as f64,
                ..Default::default()
            }
        } else {
            ProcessResult::default()
        }
    }
    
    fn can_trigger(&self, stage: &Stage, _context: &ProcessContext) -> bool {
        *stage == Stage::Blind
    }
}

impl JokerLifecycle for ScalingJoker {
    fn on_round_end(&mut self) {
        self.level += 1;
    }
}

impl JokerState for ScalingJoker {
    fn has_state(&self) -> bool { true }
    
    fn serialize_state(&self) -> Option<serde_json::Value> {
        Some(serde_json::json!({ "level": self.level }))
    }
    
    fn deserialize_state(&mut self, value: serde_json::Value) -> Result<(), String> {
        if let Some(level) = value.get("level").and_then(|v| v.as_u64()) {
            self.level = level as u32;
            Ok(())
        } else {
            Err("Invalid state format".to_string())
        }
    }
}
```

### Performance Considerations

1. **Hot Path Optimization:**
   - `JokerGameplay::process()` is called most frequently - optimize this first
   - Use early returns for non-matching conditions
   - Cache expensive calculations in joker state

2. **Memory Efficiency:**
   - Keep trait implementations lightweight
   - Use `Copy` types where possible
   - Avoid unnecessary allocations in `process()`

3. **State Management:**
   - Only implement `JokerState` for jokers that actually need state
   - Use efficient serialization formats
   - Validate state during deserialization

4. **Thread Safety:**
   - All traits maintain `Send + Sync` bounds
   - Use interior mutability patterns when needed (`RefCell`, `Mutex`)
   - Consider atomic operations for simple counters

## Git Work Trees for Parallel Development

**IMPORTANT: Always develop in a work tree, never in the main balatro-rs directory.** The workspace uses git work trees for all development to enable parallel feature development and maintain clean separation between different issues.

**Directory Structure:**
- Main git repository: `/home/spduncan/balatro-rs-ws/balatro-rs/`
- All work trees: `/home/spduncan/balatro-rs-ws/[work-tree-name]/`

### Working with Work Trees

**Development Workflow:**
1. **Never develop directly in the main balatro-rs directory** - it should remain on the main branch
2. **Always create or use an appropriate work tree** for your issue
3. **Each work tree is an independent checkout** with its own branch and working directory
4. **All work trees must be created in `/home/spduncan/balatro-rs-ws/`** - do not use `../worktrees` or `./balatro-rs/` paths

```bash
# List all work trees
git worktree list

# Create a new work tree for an issue (from within balatro-rs directory)
cd /home/spduncan/balatro-rs-ws/balatro-rs
git worktree add /home/spduncan/balatro-rs-ws/fix-issue-3-dependencies fix/issue-3-dependencies

# Switch to the work tree for development
cd /home/spduncan/balatro-rs-ws/fix-issue-3-dependencies

# Each work tree operates independently
cargo build
cargo test

# Create feature branch if not already created
git checkout -b fix/issue-3-dependencies

# Push branch and set upstream
git push -u origin fix/issue-3-dependencies
```

**Work Tree Naming Convention:**
- Infrastructure/fixes: `fix-issue-[number]-[short-description]`
- Features: `[feature-name]-feature` (e.g., `consumables-feature`)
- Joker implementations: `joker-[type]` (e.g., `joker-scoring`)

## Future Extension Plans

The repository includes detailed plans for extending the engine:
- **mvp_plan.md** - Comprehensive 78-task plan for implementing missing Balatro features
- **joker_plan.md** - Specific joker implementation details

Key planned features:
- Consumables (Tarot/Planet/Spectral cards)
- Voucher system
- Enhanced shop mechanics
- Boss blind effects

## Important Implementation Notes

1. The project implements a **subset** of Balatro rules optimized for RL training
2. Performance is critical - move generation must be fast for training
3. All new features should maintain backward compatibility with save states
4. Statistical correctness is essential for RNG-based mechanics
5. Memory safety is paramount for Python bindings - avoid unnecessary copying

## Trait-Based Testing Strategy

### Testing Individual Traits

**Test each trait in isolation:**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_joker_identity() {
        let joker = MyJoker::new();
        assert_eq!(joker.joker_type(), "my_joker");
        assert_eq!(joker.name(), "My Joker");
        assert_eq!(joker.rarity(), Rarity::Common);
        assert_eq!(joker.base_cost(), 3);
    }
    
    #[test]
    fn test_joker_gameplay() {
        let mut joker = MyJoker::new();
        let stage = Stage::Blind;
        let mut context = create_test_context();
        
        let result = joker.process(&stage, &mut context);
        assert_eq!(result.mult_added, 5.0);
        
        assert!(joker.can_trigger(&stage, &context));
    }
    
    #[test]
    fn test_joker_state_serialization() {
        let mut joker = StatefulJoker::new();
        joker.level = 10;
        
        let serialized = joker.serialize_state().unwrap();
        let mut new_joker = StatefulJoker::new();
        new_joker.deserialize_state(serialized).unwrap();
        
        assert_eq!(new_joker.level, 10);
    }
}
```

### Integration Testing

**Test trait interactions:**

```rust
#[test]
fn test_multiple_trait_interactions() {
    let mut joker = ComplexJoker::new();
    
    // Test identity
    assert_eq!(joker.name(), "Complex Joker");
    
    // Test lifecycle
    joker.on_purchase();
    assert_eq!(joker.purchase_count, 1);
    
    // Test gameplay with state
    let stage = Stage::Blind;
    let mut context = create_test_context();
    let result = joker.process(&stage, &mut context);
    
    // Verify state changed during processing
    assert!(joker.has_state());
    
    // Test state persistence
    let state = joker.serialize_state().unwrap();
    let mut restored_joker = ComplexJoker::new();
    restored_joker.deserialize_state(state).unwrap();
    
    assert_eq!(restored_joker.internal_counter, joker.internal_counter);
}
```

### Benchmark Requirements

**Performance testing for trait-based jokers:**

```rust
#[cfg(test)]
mod benches {
    use super::*;
    use test::Bencher;
    
    #[bench]
    fn bench_joker_process_hot_path(b: &mut Bencher) {
        let mut joker = PerformanceCriticalJoker::new();
        let stage = Stage::Blind;
        let mut context = create_test_context();
        
        b.iter(|| {
            joker.process(&stage, &mut context)
        });
    }
    
    #[bench]
    fn bench_trait_composition_overhead(b: &mut Bencher) {
        let jokers: Vec<Box<dyn JokerGameplay>> = vec![
            Box::new(SimpleJoker::new()),
            Box::new(ComplexJoker::new()),
        ];
        
        let stage = Stage::Blind;
        let mut context = create_test_context();
        
        b.iter(|| {
            for joker in &jokers {
                if joker.can_trigger(&stage, &context) {
                    // Simulate trait method calls
                    test::black_box(joker.get_priority(&stage));
                }
            }
        });
    }
}
```

### Testing Utilities

**Use the provided test utilities:**

```rust
// Available in core/src/joker/test_utils.rs
use crate::joker::test_utils::*;

#[test]
fn test_with_utilities() {
    let mut joker = MyJoker::new();
    let mut test_game = create_test_game_state();
    let test_hand = create_test_hand(&[Card::new(Rank::Ace, Suit::Spades)]);
    
    // Test with realistic game context
    let result = test_joker_with_hand(&mut joker, &test_hand, &test_game);
    assert_eq!(result.mult_added, 4.0);
}
```

## Common Tasks

### Adding New Jokers

1. **Define the joker struct:**
   ```rust
   #[derive(Debug)]
   pub struct NewJoker {
       // Add fields for stateful jokers
       counter: u32,
   }
   ```

2. **Implement required traits:**
   ```rust
   impl JokerIdentity for NewJoker { /* ... */ }
   impl JokerGameplay for NewJoker { /* ... */ }
   // Add other traits as needed
   ```

3. **Add to the joker factory:**
   ```rust
   // In joker_factory.rs
   pub fn create_joker(id: JokerId) -> Box<dyn Joker> {
       match id {
           JokerId::NewJoker => Box::new(NewJoker::new()),
           // ... other jokers
       }
   }
   ```

4. **Add tests:**
   ```rust
   #[cfg(test)]
   mod tests {
       #[test]
       fn test_new_joker() { /* ... */ }
   }
   ```

### Modifying Existing Jokers

1. **Identify affected traits:**
   - Changing scoring logic → `JokerGameplay`
   - Adding state → `JokerState`
   - Modifying lifecycle → `JokerLifecycle`

2. **Update implementations:**
   ```rust
   impl JokerGameplay for ExistingJoker {
       fn process(&mut self, stage: &Stage, context: &mut ProcessContext) -> ProcessResult {
           // Updated logic here
       }
   }
   ```

3. **Update tests:**
   - Test new behavior
   - Ensure backward compatibility
   - Add regression tests

### Debugging Trait Interactions

1. **Use debug output:**
   ```rust
   impl JokerState for DebuggableJoker {
       fn debug_state(&self) -> String {
           format!("counter: {}, active: {}", self.counter, self.active)
       }
   }
   ```

2. **Test trait combinations:**
   ```rust
   #[test]
   fn test_trait_debug() {
       let joker = DebuggableJoker::new();
       println!("State: {}", joker.debug_state());
       
       // Test specific trait behavior
       assert!(joker.has_state());
   }
   ```

3. **Use integration tests:**
   - Test full joker lifecycle
   - Verify state transitions
   - Check for trait method conflicts

### Migration from Legacy Joker Trait

1. **Identify joker responsibilities:**
   - Which methods are actually used?
   - What state needs to be preserved?
   - Which lifecycle events matter?

2. **Split into focused traits:**
   ```rust
   // Old monolithic implementation
   impl Joker for OldJoker { /* 20+ methods */ }
   
   // New focused implementations
   impl JokerIdentity for NewJoker { /* 6 methods */ }
   impl JokerGameplay for NewJoker { /* 3 methods */ }
   // Only implement what's needed
   ```

3. **Test compatibility:**
   - Ensure same game behavior
   - Verify state serialization compatibility
   - Test performance impact

## Testing Requirements

When modifying the codebase:

### Core Testing
1. Run `cargo test --all` to ensure no regressions
2. For new features, add integration tests showing full game flow
3. For RNG features, include statistical distribution tests
4. For Python bindings, test memory safety and round-trip serialization

### Trait-Specific Testing
5. **Individual trait testing:** Test each trait implementation in isolation
6. **Trait composition testing:** Test interactions between multiple traits on the same joker
7. **State management testing:** For stateful jokers, test serialization/deserialization round-trips
8. **Performance testing:** Benchmark hot paths, especially `JokerGameplay::process()`
9. **Integration testing:** Test jokers in realistic game scenarios using test utilities