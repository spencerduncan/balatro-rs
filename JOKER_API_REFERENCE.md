# Joker System API Reference

## Overview

The Balatro-RS joker system provides a modern, trait-based framework for implementing all 159 Balatro jokers. The system has been refactored from a monolithic 25-method trait into 5 focused traits that follow the Interface Segregation Principle, enabling better maintainability, testability, and selective implementation.

## Architecture

### New Focused Trait System

The joker system is built around five core traits, each handling a specific aspect of joker behavior:

1. **[`JokerIdentity`]** - Core metadata and identification (6 methods)
2. **[`JokerLifecycle`]** - Event hooks and state transitions (7 methods)  
3. **[`JokerGameplay`]** - Core game interactions and scoring (3 methods)
4. **[`JokerModifiers`]** - Passive game rule modifications (4 methods)
5. **[`JokerState`]** - Internal state management and persistence (5 methods)

### The Joker Super Trait

For backward compatibility, the complete `Joker` trait combines all focused traits:

```rust
/// Complete joker interface combining all aspects
pub trait Joker: 
    JokerIdentity + 
    JokerLifecycle + 
    JokerGameplay + 
    JokerModifiers + 
    JokerState + 
    Send + 
    Sync + 
    std::fmt::Debug 
{
    // Can add convenience methods here if needed
}
```

This provides complete backward compatibility - all existing code continues to work unchanged while new code can use the focused traits directly.

## Core Traits

### JokerIdentity

Handles core metadata and identification:

```rust
pub trait JokerIdentity: Send + Sync {
    /// Unique type identifier for this joker
    fn joker_type(&self) -> &'static str;
    
    /// Display name for the joker
    fn name(&self) -> &str;
    
    /// Description of the joker's effect
    fn description(&self) -> &str;
    
    /// Rarity level of the joker
    fn rarity(&self) -> Rarity;
    
    /// Base cost in the shop
    fn base_cost(&self) -> u64;
    
    /// Whether this joker is unique (limit one per collection)
    fn is_unique(&self) -> bool { false }
}
```

### JokerGameplay

Handles core game interactions and scoring:

```rust
pub trait JokerGameplay: Send + Sync {
    /// Processes the joker's effect during the specified stage
    fn process(&mut self, stage: &Stage, context: &mut ProcessContext) -> ProcessResult;
    
    /// Checks if this joker can trigger based on current game state
    fn can_trigger(&self, stage: &Stage, context: &ProcessContext) -> bool;
    
    /// Gets processing priority (higher = processed earlier)
    fn get_priority(&self, stage: &Stage) -> i32 { 0 }
}
```

### JokerModifiers

Handles passive game rule modifications:

```rust
pub trait JokerModifiers: Send + Sync {
    /// Returns the chip multiplier this joker provides
    fn get_chip_mult(&self) -> f64 { 1.0 }
    
    /// Returns the score multiplier this joker provides
    fn get_score_mult(&self) -> f64 { 1.0 }
    
    /// Returns the hand size modifier this joker provides
    fn get_hand_size_modifier(&self) -> i32 { 0 }
    
    /// Returns the discard modifier this joker provides
    fn get_discard_modifier(&self) -> i32 { 0 }
}
```

### JokerLifecycle

Handles lifecycle events and state transitions:

```rust
pub trait JokerLifecycle: Send + Sync {
    /// Called when the joker is purchased from the shop
    fn on_purchase(&mut self) {}
    
    /// Called when the joker is sold
    fn on_sell(&mut self) {}
    
    /// Called when the joker is destroyed
    fn on_destroy(&mut self) {}
    
    /// Called at the start of each round
    fn on_round_start(&mut self) {}
    
    /// Called at the end of each round
    fn on_round_end(&mut self) {}
    
    /// Called when another joker is added to the collection
    fn on_joker_added(&mut self, other_joker_type: &str) {}
    
    /// Called when another joker is removed from the collection
    fn on_joker_removed(&mut self, other_joker_type: &str) {}
}
```

### JokerState

Handles internal state management and persistence:

```rust
pub trait JokerState: Send + Sync {
    /// Returns whether this joker has any internal state
    fn has_state(&self) -> bool { false }
    
    /// Serializes the joker's state to a value
    fn serialize_state(&self) -> Option<serde_json::Value> { None }
    
    /// Deserializes the joker's state from a value
    fn deserialize_state(&mut self, value: serde_json::Value) -> Result<(), String> { Ok(()) }
    
    /// Returns a debug representation of the current state
    fn debug_state(&self) -> String { "{}".to_string() }
    
    /// Resets the joker's state to its initial values
    fn reset_state(&mut self) {}
}
```

## Trait Composition Patterns

### Implementation Strategies

The focused trait design enables three main implementation strategies:

#### 1. Selective Implementation (Recommended)

Implement only the traits your joker actually needs:

```rust
#[derive(Debug, Clone)]
struct SimpleMultJoker;

// Only implement traits this joker uses
impl JokerIdentity for SimpleMultJoker {
    fn joker_type(&self) -> &'static str { "simple_mult" }
    fn name(&self) -> &str { "Simple Mult" }
    fn description(&self) -> &str { "+4 Mult" }
    fn rarity(&self) -> Rarity { Rarity::Common }
    fn base_cost(&self) -> u64 { 3 }
}

impl JokerGameplay for SimpleMultJoker {
    fn process(&mut self, stage: &Stage, context: &mut ProcessContext) -> ProcessResult {
        if matches!(stage, Stage::Scoring) {
            ProcessResult { mult_added: 4.0, ..Default::default() }
        } else {
            ProcessResult::default()
        }
    }

    fn can_trigger(&self, stage: &Stage, _context: &ProcessContext) -> bool {
        matches!(stage, Stage::Scoring)
    }
}

// Default implementations for unused traits
impl JokerLifecycle for SimpleMultJoker {}
impl JokerModifiers for SimpleMultJoker {}
impl JokerState for SimpleMultJoker {}

// Still implements the complete Joker trait
impl Joker for SimpleMultJoker {}
```

#### 2. Full Implementation (Legacy Pattern)

Implement all traits explicitly for complex jokers:

```rust
#[derive(Debug, Clone)]
struct ComplexJoker {
    state: JokerInternalState,
}

impl JokerIdentity for ComplexJoker {
    // Identity implementation
}

impl JokerGameplay for ComplexJoker {
    // Gameplay implementation with complex logic
}

impl JokerLifecycle for ComplexJoker {
    // Lifecycle hooks with state management
}

impl JokerModifiers for ComplexJoker {
    // Dynamic modifiers based on state
}

impl JokerState for ComplexJoker {
    // Full state serialization/deserialization
}

impl Joker for ComplexJoker {}
```

#### 3. Trait-Specific Usage (Advanced)

Use specific traits directly for specialized functionality:

```rust
// Function that works with any joker identity
fn display_joker_info<T: JokerIdentity>(joker: &T) {
    println!("{}: {}", joker.name(), joker.description());
    println!("Rarity: {:?}, Cost: {}", joker.rarity(), joker.base_cost());
}

// Function that processes gameplay for any joker
fn process_joker_effect<T: JokerGameplay>(
    joker: &mut T, 
    stage: &Stage, 
    context: &mut ProcessContext
) -> ProcessResult {
    if joker.can_trigger(stage, context) {
        joker.process(stage, context)
    } else {
        ProcessResult::default()
    }
}
```

### Common Composition Patterns

#### Pattern 1: Simple Scoring Joker

Most basic jokers (60% of all jokers):

```rust
struct BasicJoker;

impl JokerIdentity for BasicJoker { /* basic info */ }
impl JokerGameplay for BasicJoker { /* scoring effect */ }
impl JokerLifecycle for BasicJoker {} // defaults
impl JokerModifiers for BasicJoker {} // defaults  
impl JokerState for BasicJoker {} // defaults
impl Joker for BasicJoker {}
```

**Required Traits**: Identity + Gameplay  
**Optional Traits**: All others use defaults  
**Complexity**: Low  
**Performance**: Excellent

#### Pattern 2: Stateful Growing Joker

Jokers that accumulate power over time:

```rust
struct GrowthJoker {
    power_level: f64,
}

impl JokerIdentity for GrowthJoker { /* basic info */ }

impl JokerGameplay for GrowthJoker {
    fn process(&mut self, stage: &Stage, context: &mut ProcessContext) -> ProcessResult {
        if matches!(stage, Stage::Scoring) {
            ProcessResult { mult_added: self.power_level, ..Default::default() }
        } else {
            ProcessResult::default()
        }
    }
}

impl JokerLifecycle for GrowthJoker {
    fn on_round_end(&mut self) {
        self.power_level += 0.5; // Grows each round
    }
}

impl JokerState for GrowthJoker {
    fn has_state(&self) -> bool { true }
    
    fn serialize_state(&self) -> Option<serde_json::Value> {
        Some(serde_json::json!({"power_level": self.power_level}))
    }
    
    fn deserialize_state(&mut self, value: serde_json::Value) -> Result<(), String> {
        self.power_level = value["power_level"].as_f64().ok_or("Invalid power_level")?;
        Ok(())
    }
}

impl JokerModifiers for GrowthJoker {} // defaults
impl Joker for GrowthJoker {}
```

**Required Traits**: Identity + Gameplay + Lifecycle + State  
**Optional Traits**: Modifiers  
**Complexity**: Medium  
**Performance**: Good

#### Pattern 3: Passive Modifier Joker

Jokers that permanently change game rules:

```rust
struct PassiveJoker;

impl JokerIdentity for PassiveJoker { /* basic info */ }

impl JokerModifiers for PassiveJoker {
    fn get_score_mult(&self) -> f64 {
        1.5 // Permanent 50% score boost
    }
    
    fn get_hand_size_modifier(&self) -> i32 {
        2 // +2 hand size
    }
}

impl JokerGameplay for PassiveJoker {} // defaults (no active effects)
impl JokerLifecycle for PassiveJoker {} // defaults
impl JokerState for PassiveJoker {} // defaults
impl Joker for PassiveJoker {}
```

**Required Traits**: Identity + Modifiers  
**Optional Traits**: All others use defaults  
**Complexity**: Low  
**Performance**: Excellent

#### Pattern 4: Social Joker (Collection Awareness)

Jokers that react to other jokers:

```rust
struct SocialJoker {
    friend_count: u32,
}

impl JokerIdentity for SocialJoker { /* basic info */ }

impl JokerGameplay for SocialJoker {
    fn process(&mut self, stage: &Stage, context: &mut ProcessContext) -> ProcessResult {
        if matches!(stage, Stage::Scoring) {
            // Stronger with more joker friends
            ProcessResult { 
                mult_added: self.friend_count as f64 * 2.0, 
                ..Default::default() 
            }
        } else {
            ProcessResult::default()
        }
    }
}

impl JokerLifecycle for SocialJoker {
    fn on_joker_added(&mut self, _other_joker_type: &str) {
        self.friend_count += 1;
    }
    
    fn on_joker_removed(&mut self, _other_joker_type: &str) {
        self.friend_count = self.friend_count.saturating_sub(1);
    }
}

impl JokerState for SocialJoker {
    fn has_state(&self) -> bool { true }
    // State implementation for friend_count
}

impl JokerModifiers for SocialJoker {} // defaults
impl Joker for SocialJoker {}
```

**Required Traits**: Identity + Gameplay + Lifecycle + State  
**Optional Traits**: Modifiers  
**Complexity**: Medium-High  
**Performance**: Good

### Migration Patterns

#### From Monolithic to Focused

**Old Pattern (Deprecated)**:
```rust
impl Joker for MyJoker {
    fn id(&self) -> JokerId { /* ... */ }
    fn name(&self) -> &str { /* ... */ }
    fn on_hand_played(&self, context: &mut GameContext, hand: &SelectHand) -> JokerEffect { /* ... */ }
    fn modify_mult(&self, context: &GameContext, base_mult: i32) -> i32 { /* ... */ }
    // 20+ other methods...
}
```

**New Pattern (Recommended)**:
```rust
impl JokerIdentity for MyJoker {
    fn joker_type(&self) -> &'static str { /* ... */ }
    fn name(&self) -> &str { /* ... */ }
    // Only identity-related methods
}

impl JokerGameplay for MyJoker {
    fn process(&mut self, stage: &Stage, context: &mut ProcessContext) -> ProcessResult { /* ... */ }
    // Only gameplay-related methods
}

// Other traits as needed
impl JokerLifecycle for MyJoker {}
impl JokerModifiers for MyJoker {}
impl JokerState for MyJoker {}
impl Joker for MyJoker {} // Still implements the super trait
```

## Migration Guide

### Quick Migration Steps

For developers migrating from the old monolithic `Joker` trait to the new focused traits:

#### Step 1: Analyze Your Current Implementation

Identify which categories of methods your joker actually uses:

```rust
// Old implementation - identify used methods
impl Joker for MyJoker {
    // ✅ IDENTITY METHODS - Migrate to JokerIdentity
    fn id(&self) -> JokerId { JokerId::Custom("my_joker".to_string()) }
    fn name(&self) -> &str { "My Joker" }
    fn description(&self) -> &str { "Does something cool" }
    fn rarity(&self) -> JokerRarity { JokerRarity::Common }
    fn cost(&self) -> usize { 5 }

    // ✅ GAMEPLAY METHODS - Migrate to JokerGameplay  
    fn on_hand_played(&self, context: &mut GameContext, hand: &SelectHand) -> JokerEffect {
        JokerEffect::new().with_mult(4)
    }

    // ✅ LIFECYCLE METHODS - Migrate to JokerLifecycle
    fn on_round_end(&self, context: &mut GameContext) -> JokerEffect {
        // Some round-end logic
        JokerEffect::new()
    }

    // ❌ UNUSED METHODS - Don't need to migrate these
    // (20+ default implementations you never overrode)
}
```

#### Step 2: Split Into Focused Traits

Create separate `impl` blocks for each category:

```rust
// ✅ NEW: JokerIdentity implementation
impl JokerIdentity for MyJoker {
    fn joker_type(&self) -> &'static str { "my_joker" }
    fn name(&self) -> &str { "My Joker" }
    fn description(&self) -> &str { "Does something cool" }
    fn rarity(&self) -> Rarity { Rarity::Common }
    fn base_cost(&self) -> u64 { 5 }
}

// ✅ NEW: JokerGameplay implementation  
impl JokerGameplay for MyJoker {
    fn process(&mut self, stage: &Stage, context: &mut ProcessContext) -> ProcessResult {
        if matches!(stage, Stage::Scoring) {
            ProcessResult { mult_added: 4.0, ..Default::default() }
        } else {
            ProcessResult::default()
        }
    }

    fn can_trigger(&self, stage: &Stage, _context: &ProcessContext) -> bool {
        matches!(stage, Stage::Scoring)
    }
}

// ✅ NEW: JokerLifecycle implementation
impl JokerLifecycle for MyJoker {
    fn on_round_end(&mut self) {
        // Migrate round-end logic here
    }
}

// ✅ NEW: Default implementations for unused traits
impl JokerModifiers for MyJoker {}
impl JokerState for MyJoker {}

// ✅ UNCHANGED: Still implements complete Joker trait
impl Joker for MyJoker {}
```

#### Step 3: Update Method Signatures

Several method signatures have changed in the new system:

| Old Method | New Method | Changes |
|------------|------------|---------|
| `fn id(&self) -> JokerId` | `fn joker_type(&self) -> &'static str` | Returns string instead of JokerId |
| `fn cost(&self) -> usize` | `fn base_cost(&self) -> u64` | Changed type and name |
| `fn on_hand_played(...) -> JokerEffect` | `fn process(...) -> ProcessResult` | Unified processing method |
| `fn on_card_scored(...) -> JokerEffect` | `fn process(...) -> ProcessResult` | Combined into process() |
| `fn modify_mult(...) -> i32` | `fn get_score_mult() -> f64` | Different calculation approach |

### Detailed Migration Examples

#### Example 1: Basic Scoring Joker

**Before (Old System)**:
```rust
impl Joker for TheJoker {
    fn id(&self) -> JokerId { JokerId::Joker }
    fn name(&self) -> &str { "Joker" }
    fn description(&self) -> &str { "+4 Mult" }
    fn rarity(&self) -> JokerRarity { JokerRarity::Common }
    fn cost(&self) -> usize { 3 }
    
    fn on_hand_played(&self, context: &mut GameContext, hand: &SelectHand) -> JokerEffect {
        JokerEffect::new().with_mult(4)
    }
    
    // 20+ other default methods...
}
```

**After (New System)**:
```rust
impl JokerIdentity for TheJoker {
    fn joker_type(&self) -> &'static str { "joker" }
    fn name(&self) -> &str { "Joker" }
    fn description(&self) -> &str { "+4 Mult" }
    fn rarity(&self) -> Rarity { Rarity::Common }
    fn base_cost(&self) -> u64 { 3 }
}

impl JokerGameplay for TheJoker {
    fn process(&mut self, stage: &Stage, _context: &mut ProcessContext) -> ProcessResult {
        if matches!(stage, Stage::Scoring) {
            ProcessResult { mult_added: 4.0, ..Default::default() }
        } else {
            ProcessResult::default()
        }
    }

    fn can_trigger(&self, stage: &Stage, _context: &ProcessContext) -> bool {
        matches!(stage, Stage::Scoring)
    }
}

impl JokerLifecycle for TheJoker {}
impl JokerModifiers for TheJoker {}
impl JokerState for TheJoker {}
impl Joker for TheJoker {}
```

**Benefits**: Only ~15 lines instead of 25+ methods, clearer intent, better testability.

#### Example 2: Stateful Joker Migration

**Before (Old System)**:
```rust
#[derive(Clone)]
struct IceCreamJoker {
    remaining_uses: u32,
}

impl Joker for IceCreamJoker {
    fn id(&self) -> JokerId { JokerId::IceCream }
    fn name(&self) -> &str { "Ice Cream" }
    fn description(&self) -> &str { "+100 Chips, -1 after each use" }
    fn rarity(&self) -> JokerRarity { JokerRarity::Common }
    fn cost(&self) -> usize { 5 }
    
    fn on_hand_played(&self, context: &mut GameContext, hand: &SelectHand) -> JokerEffect {
        if self.remaining_uses > 0 {
            self.remaining_uses -= 1;
            JokerEffect::new().with_chips(100)
        } else {
            JokerEffect::new()
        }
    }
    
    // State management scattered across multiple methods...
}
```

**After (New System)**:
```rust
#[derive(Debug, Clone)]
struct IceCreamJoker {
    remaining_uses: u32,
}

impl JokerIdentity for IceCreamJoker {
    fn joker_type(&self) -> &'static str { "ice_cream" }
    fn name(&self) -> &str { "Ice Cream" }
    fn description(&self) -> &str { "+100 Chips, -1 after each use" }
    fn rarity(&self) -> Rarity { Rarity::Common }
    fn base_cost(&self) -> u64 { 5 }
}

impl JokerGameplay for IceCreamJoker {
    fn process(&mut self, stage: &Stage, _context: &mut ProcessContext) -> ProcessResult {
        if matches!(stage, Stage::Scoring) && self.remaining_uses > 0 {
            self.remaining_uses -= 1;
            ProcessResult { chips_added: 100, ..Default::default() }
        } else {
            ProcessResult::default()
        }
    }

    fn can_trigger(&self, stage: &Stage, _context: &ProcessContext) -> bool {
        matches!(stage, Stage::Scoring) && self.remaining_uses > 0
    }
}

impl JokerState for IceCreamJoker {
    fn has_state(&self) -> bool { true }
    
    fn serialize_state(&self) -> Option<serde_json::Value> {
        Some(serde_json::json!({"remaining_uses": self.remaining_uses}))
    }
    
    fn deserialize_state(&mut self, value: serde_json::Value) -> Result<(), String> {
        self.remaining_uses = value["remaining_uses"]
            .as_u64().ok_or("Invalid remaining_uses")? as u32;
        Ok(())
    }
    
    fn debug_state(&self) -> String {
        format!("uses left: {}", self.remaining_uses)
    }
    
    fn reset_state(&mut self) {
        self.remaining_uses = 5; // Reset to initial value
    }
}

impl JokerLifecycle for IceCreamJoker {}
impl JokerModifiers for IceCreamJoker {}
impl Joker for IceCreamJoker {}
```

**Benefits**: Clear state management, better serialization, easier testing.

#### Example 3: Modifier Joker Migration

**Before (Old System)**:
```rust
impl Joker for HandSizeJoker {
    fn id(&self) -> JokerId { JokerId::Custom("hand_size".to_string()) }
    fn name(&self) -> &str { "Big Hand" }
    fn description(&self) -> &str { "+2 hand size" }
    fn rarity(&self) -> JokerRarity { JokerRarity::Uncommon }
    fn cost(&self) -> usize { 6 }
    
    fn modify_hand_size(&self, context: &GameContext, base_size: usize) -> usize {
        base_size + 2
    }
    
    // 20+ other default methods...
}
```

**After (New System)**:
```rust
impl JokerIdentity for HandSizeJoker {
    fn joker_type(&self) -> &'static str { "hand_size" }
    fn name(&self) -> &str { "Big Hand" }
    fn description(&self) -> &str { "+2 hand size" }
    fn rarity(&self) -> Rarity { Rarity::Uncommon }
    fn base_cost(&self) -> u64 { 6 }
}

impl JokerModifiers for HandSizeJoker {
    fn get_hand_size_modifier(&self) -> i32 {
        2  // +2 hand size
    }
}

impl JokerGameplay for HandSizeJoker {}
impl JokerLifecycle for HandSizeJoker {}
impl JokerState for HandSizeJoker {}
impl Joker for HandSizeJoker {}
```

**Benefits**: Extremely clean and focused, clear modifier intent.

### Migration Checklist

When migrating a joker, use this checklist:

- [ ] **Analyze** - Identify which old methods you actually override
- [ ] **Identity** - Implement `JokerIdentity` with updated method names/types
- [ ] **Gameplay** - Convert gameplay logic to `process()` and `can_trigger()`
- [ ] **Lifecycle** - Move lifecycle hooks to `JokerLifecycle` (if any)
- [ ] **Modifiers** - Convert modifier methods to `JokerModifiers` (if any)
- [ ] **State** - Implement proper state management in `JokerState` (if needed)
- [ ] **Defaults** - Add empty `impl` blocks for unused traits
- [ ] **Super Trait** - Add `impl Joker for YourJoker {}`
- [ ] **Test** - Verify all functionality still works
- [ ] **Performance** - Check that `can_trigger()` is efficient

### Backward Compatibility

The new system maintains **100% backward compatibility**:

- All existing `Box<dyn Joker>` code continues to work
- The old monolithic `Joker` trait still exists as a super trait
- No changes needed to game engine or other systems
- Migration can be done incrementally, joker by joker

### Testing Your Migration

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_migrated_joker() {
        let mut joker = MyJoker::new();
        
        // Test identity
        assert_eq!(joker.name(), "Expected Name");
        assert_eq!(joker.joker_type(), "expected_type");
        
        // Test gameplay
        let mut context = ProcessContext::new();
        let result = joker.process(&Stage::Scoring, &mut context);
        assert_eq!(result.mult_added, 4.0);
        
        // Test can_trigger
        assert!(joker.can_trigger(&Stage::Scoring, &context));
        assert!(!joker.can_trigger(&Stage::Dealing, &context));
        
        // Test state (if applicable)
        if joker.has_state() {
            let state = joker.serialize_state().unwrap();
            let mut joker2 = MyJoker::new();
            joker2.deserialize_state(state).unwrap();
            // Verify state was loaded correctly
        }
    }
}
```

## Best Practices & Common Pitfalls

### Best Practices

#### 1. Implement Only What You Need

**✅ Good**: Selective implementation
```rust
struct SimpleJoker;

impl JokerIdentity for SimpleJoker {
    // Only implement required identity methods
}

impl JokerGameplay for SimpleJoker {
    // Only implement the gameplay you actually use
}

// Default implementations for unused traits
impl JokerLifecycle for SimpleJoker {}
impl JokerModifiers for SimpleJoker {}
impl JokerState for SimpleJoker {}
impl Joker for SimpleJoker {}
```

**❌ Bad**: Over-implementation
```rust
impl JokerLifecycle for SimpleJoker {
    fn on_purchase(&mut self) {
        // Empty - why implement this if you don't use it?
    }
    
    fn on_sell(&mut self) {
        // Empty - unnecessary boilerplate
    }
    
    // ... many more empty methods
}
```

#### 2. Use Efficient can_trigger() Implementations

**✅ Good**: Fast, side-effect-free checks
```rust
impl JokerGameplay for ConditionalJoker {
    fn can_trigger(&self, stage: &Stage, context: &ProcessContext) -> bool {
        // Fast boolean checks only
        matches!(stage, Stage::Scoring) 
            && !context.played_cards.is_empty()
            && self.enabled  // Simple field check
    }
    
    fn process(&mut self, stage: &Stage, context: &mut ProcessContext) -> ProcessResult {
        // Expensive logic goes here, after can_trigger() passed
        let complex_calculation = self.calculate_bonus(context);
        ProcessResult { mult_added: complex_calculation, ..Default::default() }
    }
}
```

**❌ Bad**: Expensive or side-effect-having checks
```rust
impl JokerGameplay for SlowJoker {
    fn can_trigger(&self, stage: &Stage, context: &ProcessContext) -> bool {
        // ❌ Expensive computation in can_trigger
        let complex_result = self.complex_hand_analysis(context.played_cards);
        
        // ❌ Side effects in can_trigger  
        self.update_internal_cache(complex_result);
        
        complex_result > 0
    }
}
```

#### 3. Design State for Persistence

**✅ Good**: Clean, serializable state
```rust
#[derive(Debug, Clone)]
struct WellDesignedJoker {
    // Simple, serializable types
    level: u32,
    experience: u32,
    bonuses_applied: u32,
}

impl JokerState for WellDesignedJoker {
    fn serialize_state(&self) -> Option<serde_json::Value> {
        Some(serde_json::json!({
            "level": self.level,
            "experience": self.experience,
            "bonuses_applied": self.bonuses_applied,
            "version": 1  // Always include version for future migrations
        }))
    }
    
    fn deserialize_state(&mut self, value: serde_json::Value) -> Result<(), String> {
        // Validate all fields
        self.level = value["level"].as_u64().ok_or("Invalid level")? as u32;
        self.experience = value["experience"].as_u64().ok_or("Invalid experience")? as u32;
        self.bonuses_applied = value["bonuses_applied"].as_u64().ok_or("Invalid bonuses_applied")? as u32;
        
        // Validate ranges
        if self.level > 100 {
            return Err("Level too high".to_string());
        }
        
        Ok(())
    }
}
```

**❌ Bad**: Complex, hard-to-serialize state
```rust
#[derive(Debug, Clone)]
struct BadlyDesignedJoker {
    // ❌ Complex types that are hard to serialize
    callback_functions: Vec<Box<dyn Fn() -> i32>>,
    file_handles: Vec<File>,
    thread_handles: Vec<JoinHandle<()>>,
    
    // ❌ Pointer-based data structures
    linked_list_head: Option<Rc<RefCell<Node>>>,
}
```

#### 4. Use Priority Wisely

**✅ Good**: Logical priority ordering
```rust
impl JokerGameplay for PriorityJoker {
    fn get_priority(&self, stage: &Stage) -> i32 {
        match stage {
            Stage::Scoring => {
                // Higher priority for more impactful effects
                if self.is_multiplier_joker() {
                    100  // Multipliers should run early
                } else if self.is_additive_bonus() {
                    50   // Additive bonuses run after multipliers
                } else {
                    0    // Default priority
                }
            },
            _ => 0
        }
    }
}
```

**❌ Bad**: Random or inappropriate priorities
```rust
impl JokerGameplay for BadPriorityJoker {
    fn get_priority(&self, stage: &Stage) -> i32 {
        // ❌ Random high priority for no reason
        999  // This will interfere with other jokers
    }
}
```

#### 5. Handle Trait Interactions Properly

**✅ Good**: Coordinated trait implementations
```rust
struct CoordinatedJoker {
    power_level: f64,
}

impl JokerGameplay for CoordinatedJoker {
    fn process(&mut self, stage: &Stage, context: &mut ProcessContext) -> ProcessResult {
        if matches!(stage, Stage::Scoring) {
            ProcessResult { mult_added: self.power_level, ..Default::default() }
        } else {
            ProcessResult::default()
        }
    }
}

impl JokerLifecycle for CoordinatedJoker {
    fn on_round_end(&mut self) {
        // Lifecycle updates state...
        self.power_level += 0.5;
    }
}

impl JokerState for CoordinatedJoker {
    fn has_state(&self) -> bool { true }
    
    fn serialize_state(&self) -> Option<serde_json::Value> {
        // ...and state knows how to persist the updates
        Some(serde_json::json!({"power_level": self.power_level}))
    }
}
```

### Common Pitfalls

#### Pitfall 1: Forgetting the Super Trait

**❌ Problem**: Compile errors when using jokers polymorphically
```rust
struct MyJoker;

impl JokerIdentity for MyJoker { /* ... */ }
impl JokerGameplay for MyJoker { /* ... */ }
// ... other traits

// ❌ Forgot this!
// impl Joker for MyJoker {}

fn use_joker() {
    let joker: Box<dyn Joker> = Box::new(MyJoker); // ❌ Compile error!
}
```

**✅ Solution**: Always implement the super trait
```rust
impl Joker for MyJoker {} // ✅ Don't forget this line!
```

#### Pitfall 2: Inconsistent State Management

**❌ Problem**: State changes in gameplay that aren't persisted
```rust
impl JokerGameplay for InconsistentJoker {
    fn process(&mut self, stage: &Stage, context: &mut ProcessContext) -> ProcessResult {
        if matches!(stage, Stage::Scoring) {
            self.times_used += 1; // ❌ State change...
            ProcessResult { mult_added: self.times_used as f64, ..Default::default() }
        } else {
            ProcessResult::default()
        }
    }
}

impl JokerState for InconsistentJoker {
    fn has_state(&self) -> bool { false } // ❌ ...but claims no state!
    
    // serialize_state() returns None, so times_used is lost on save/load
}
```

**✅ Solution**: Consistent state management
```rust
impl JokerState for ConsistentJoker {
    fn has_state(&self) -> bool { true } // ✅ Claims state
    
    fn serialize_state(&self) -> Option<serde_json::Value> {
        Some(serde_json::json!({"times_used": self.times_used})) // ✅ Persists state
    }
}
```

#### Pitfall 3: Side Effects in can_trigger()

**❌ Problem**: Unpredictable behavior from side effects
```rust
impl JokerGameplay for UnpredictableJoker {
    fn can_trigger(&self, stage: &Stage, context: &ProcessContext) -> bool {
        if matches!(stage, Stage::Scoring) {
            // ❌ Side effect - modifies state during checking!
            self.last_checked = current_time();
            self.check_count += 1;
            true
        } else {
            false
        }
    }
}
```

**✅ Solution**: Pure functions for can_trigger()
```rust
impl JokerGameplay for PredictableJoker {
    fn can_trigger(&self, stage: &Stage, context: &ProcessContext) -> bool {
        // ✅ Pure function - no side effects
        matches!(stage, Stage::Scoring) && !context.played_cards.is_empty()
    }
    
    fn process(&mut self, stage: &Stage, context: &mut ProcessContext) -> ProcessResult {
        // ✅ Side effects belong in process()
        self.times_triggered += 1;
        ProcessResult { mult_added: 4.0, ..Default::default() }
    }
}
```

#### Pitfall 4: Method Signature Mismatches

**❌ Problem**: Using old method signatures
```rust
impl JokerIdentity for OutdatedJoker {
    fn id(&self) -> JokerId { // ❌ Old method signature
        JokerId::Custom("outdated".to_string())
    }
    
    fn cost(&self) -> usize { 5 } // ❌ Old method signature
}
```

**✅ Solution**: Use correct new signatures
```rust
impl JokerIdentity for ModernJoker {
    fn joker_type(&self) -> &'static str { // ✅ New method
        "modern"
    }
    
    fn base_cost(&self) -> u64 { 5 } // ✅ New method
}
```

#### Pitfall 5: Performance Anti-patterns

**❌ Problem**: Expensive operations in hot paths
```rust
impl JokerGameplay for SlowJoker {
    fn process(&mut self, stage: &Stage, context: &mut ProcessContext) -> ProcessResult {
        if matches!(stage, Stage::Scoring) {
            // ❌ Expensive operations on every call
            let all_cards: Vec<Card> = context.played_cards.iter()
                .chain(context.held_cards.iter())
                .cloned()
                .collect();
                
            let complex_analysis = self.analyze_entire_deck(&all_cards); // Expensive!
            
            ProcessResult { mult_added: complex_analysis, ..Default::default() }
        } else {
            ProcessResult::default()
        }
    }
}
```

**✅ Solution**: Optimize hot paths
```rust
impl JokerGameplay for FastJoker {
    fn process(&mut self, stage: &Stage, context: &mut ProcessContext) -> ProcessResult {
        if matches!(stage, Stage::Scoring) {
            // ✅ Simple, fast calculation
            let bonus = context.played_cards.len() as f64 * 2.0;
            ProcessResult { mult_added: bonus, ..Default::default() }
        } else {
            ProcessResult::default()
        }
    }
}
```

### Testing Best Practices

#### Unit Test Each Trait Separately

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_identity_trait() {
        let joker = MyJoker::new();
        
        // Test only identity methods
        assert_eq!(joker.joker_type(), "my_joker");
        assert_eq!(joker.name(), "My Joker");
        assert_eq!(joker.base_cost(), 5);
    }
    
    #[test]
    fn test_gameplay_trait() {
        let mut joker = MyJoker::new();
        let mut context = create_test_context();
        
        // Test can_trigger first
        assert!(joker.can_trigger(&Stage::Scoring, &context));
        assert!(!joker.can_trigger(&Stage::Dealing, &context));
        
        // Test process method
        let result = joker.process(&Stage::Scoring, &mut context);
        assert_eq!(result.mult_added, 4.0);
    }
    
    #[test]
    fn test_state_trait() {
        let mut joker = MyJoker::new();
        
        if joker.has_state() {
            // Test serialization round-trip
            let state = joker.serialize_state().unwrap();
            let mut joker2 = MyJoker::new();
            joker2.deserialize_state(state).unwrap();
            
            // Verify state was preserved
            assert_eq!(joker.debug_state(), joker2.debug_state());
        }
    }
}
```

### JokerEffect

Represents the effect a joker has on scoring:

```rust
pub struct JokerEffect {
    pub chips: i32,                                // Bonus chips to add
    pub mult: i32,                                 // Bonus mult to add  
    pub money: i32,                                // Money to award
    pub mult_multiplier: f32,                      // Multiplier to apply to mult
    pub retrigger: u32,                            // Number of times to retrigger effect
    pub destroy_self: bool,                        // Whether this joker destroys itself
    pub destroy_others: Vec<JokerId>,              // Other jokers to destroy
    pub transform_cards: Vec<(Card, Card)>,        // Cards to transform
    pub hand_size_mod: i32,                        // Temporary hand size modification
    pub discard_mod: i32,                          // Temporary discard count modification
    pub message: Option<String>,                   // Custom message to display
}

impl JokerEffect {
    pub fn new() -> Self;
    pub fn with_chips(self, chips: i32) -> Self;
    pub fn with_mult(self, mult: i32) -> Self;
    pub fn with_money(self, money: i32) -> Self;
    pub fn with_mult_multiplier(self, multiplier: f32) -> Self;
}
```

### GameContext

Provides access to game state for joker implementations:

```rust
pub struct GameContext<'a> {
    pub chips: i32,                      // Current chips
    pub mult: i32,                       // Current mult
    pub money: i32,                      // Current money
    pub ante: u8,                        // Current ante
    pub round: u32,                      // Current round
    pub stage: &'a Stage,                // Current game stage
    pub hands_played: u32,               // Hands played this round
    pub discards_used: u32,              // Discards used this round
    pub jokers: &'a [Box<dyn Joker>],    // All jokers in play
    pub hand: &'a Hand,                  // Cards in hand
    pub discarded: &'a [Card],           // Discarded cards
}
```

## Static Joker Framework

For simple conditional jokers (most common pattern):

### StaticJoker

```rust
pub struct StaticJoker {
    pub id: JokerId,
    pub name: &'static str,
    pub description: &'static str,
    pub rarity: JokerRarity,
    pub base_cost: Option<usize>,
    pub chips_bonus: Option<i32>,
    pub mult_bonus: Option<i32>,
    pub mult_multiplier: Option<f32>,
    pub condition: StaticCondition,
    pub per_card: bool,
}
```

### Builder Pattern

```rust
impl StaticJoker {
    pub fn builder(
        id: JokerId,
        name: &'static str,
        description: &'static str,
    ) -> StaticJokerBuilder;
}

impl StaticJokerBuilder {
    pub fn rarity(self, rarity: JokerRarity) -> Self;
    pub fn cost(self, cost: usize) -> Self;
    pub fn chips(self, chips: i32) -> Self;
    pub fn mult(self, mult: i32) -> Self;
    pub fn mult_multiplier(self, multiplier: f32) -> Self;
    pub fn condition(self, condition: StaticCondition) -> Self;
    pub fn per_card(self) -> Self;
    pub fn per_hand(self) -> Self;
    pub fn build(self) -> Result<StaticJoker, String>;
}
```

### Static Conditions

```rust
pub enum StaticCondition {
    Always,                           // Always trigger
    SuitScored(Suit),                // Specific suit scored
    RankScored(Value),               // Specific rank scored
    HandType(HandRank),              // Specific hand type played
    AnySuitScored(Vec<Suit>),        // Any of these suits scored
    AnyRankScored(Vec<Value>),       // Any of these ranks scored
}
```

## Factory Pattern

### JokerFactory

For dynamic joker creation:

```rust
pub struct JokerFactory;

impl JokerFactory {
    pub fn create(id: JokerId) -> Option<Box<dyn Joker>>;
    pub fn get_by_rarity(rarity: JokerRarity) -> Vec<JokerId>;
    pub fn get_all_implemented() -> Vec<JokerId>;
}
```

### StaticJokerFactory

Pre-configured static jokers:

```rust
pub struct StaticJokerFactory;

impl StaticJokerFactory {
    // Suit-based jokers
    pub fn create_greedy_joker() -> Box<dyn Joker>;      // +3 mult per Diamond
    pub fn create_lusty_joker() -> Box<dyn Joker>;       // +3 mult per Heart
    pub fn create_wrathful_joker() -> Box<dyn Joker>;    // +3 mult per Spade
    pub fn create_gluttonous_joker() -> Box<dyn Joker>;  // +3 mult per Club
    
    // Hand-type jokers
    pub fn create_jolly_joker() -> Box<dyn Joker>;       // +8 mult for Pair
    pub fn create_zany_joker() -> Box<dyn Joker>;        // +12 mult for Three of a Kind
    pub fn create_mad_joker() -> Box<dyn Joker>;         // +10 mult for Straight
    pub fn create_crazy_joker() -> Box<dyn Joker>;       // +12 mult for Flush
    pub fn create_droll_joker() -> Box<dyn Joker>;       // +10 mult for Full House
    
    // Rank-based jokers
    pub fn create_even_steven() -> Box<dyn Joker>;       // +4 mult for even ranks
    pub fn create_odd_todd() -> Box<dyn Joker>;          // +4 mult for odd ranks
    pub fn create_scholar() -> Box<dyn Joker>;           // +4 mult for Aces
    
    // And more...
}
```

## Registry System

### JokerRegistry

Thread-safe registry for joker definitions:

```rust
pub struct JokerRegistry;

impl JokerRegistry {
    pub fn register(definition: JokerDefinition);
    pub fn get_definition(id: &JokerId) -> Option<JokerDefinition>;
    pub fn get_all_definitions() -> Vec<JokerDefinition>;
    pub fn is_unlocked(id: &JokerId, context: &GameContext) -> bool;
}

pub struct JokerDefinition {
    pub id: JokerId,
    pub name: &'static str,
    pub description: &'static str,
    pub rarity: JokerRarity,
    pub base_cost: usize,
    pub unlock_condition: Option<UnlockCondition>,
}
```

## State Management

### JokerState

For jokers that maintain persistent state:

```rust
pub struct JokerState {
    pub accumulated_value: f64,
    pub triggers_remaining: Option<u32>,
    pub custom_data: HashMap<String, Value>,
}

impl JokerState {
    pub fn new() -> Self;
    pub fn with_accumulated_value(value: f64) -> Self;
    pub fn with_triggers(triggers: u32) -> Self;
    // Note: with_custom_data method is not implemented, use direct field access
    // Note: get_custom_data method is not implemented, use direct field access
}
```

### JokerStateManager

Thread-safe state management:

```rust
pub struct JokerStateManager;

impl JokerStateManager {
    pub fn new() -> Self;
    pub fn get_state(&self, joker_id: JokerId) -> Option<JokerState>;
    pub fn set_state(&self, joker_id: JokerId, state: JokerState);
    pub fn update_state<F>(&self, joker_id: JokerId, f: F)
    where F: FnOnce(&mut JokerState);
    pub fn remove_state(&self, joker_id: JokerId) -> Option<JokerState>;
    pub fn has_state(&self, joker_id: JokerId) -> bool;
    
    // Helper methods
    pub fn add_accumulated_value(&self, joker_id: JokerId, value: f64);
    pub fn use_trigger(&self, joker_id: JokerId) -> bool;
}
```

## Conditional Joker Framework

For complex conditional logic:

```rust
pub struct ConditionalJoker {
    pub id: JokerId,
    pub name: &'static str,
    pub description: &'static str,
    pub rarity: JokerRarity,
    pub condition: JokerCondition,
    pub effect: JokerEffect,
}

pub enum JokerCondition {
    // Money conditions
    MoneyLessThan(i32),
    MoneyGreaterThan(i32),
    
    // Hand conditions
    HandSizeExactly(usize),
    NoFaceCardsHeld,
    ContainsRank(Value),
    ContainsSuit(Suit),
    
    // Game conditions
    PlayedHandType(HandRank),
    
    // Composite conditions
    All(Vec<JokerCondition>),
    Any(Vec<JokerCondition>),
    Not(Box<JokerCondition>),
    Always,
}
```

## Enumerations

### JokerId

All 159 joker identifiers are defined in the `JokerId` enum. Major categories include:

- **Basic scoring jokers**: `Joker`, `GreedyJoker`, `LustyJoker`, etc.
- **Multiplicative jokers**: `HalfJoker`, `AbstractJoker`, `SteelJoker`, etc.
- **Conditional jokers**: `Banner`, `EvenSteven`, `OddTodd`, etc.
- **Scaling jokers**: `JokerStencil`, `FourFingers`, `MimeJoker`, etc.
- **Economic jokers**: `GoldenJoker`, `EggJoker`, `BusinessCard`, etc.
- **Special jokers**: `Joker`, `CertificateJoker`, `DNA`, etc.

### JokerRarity

```rust
pub enum JokerRarity {
    Common,      // Base cost: 3 coins
    Uncommon,    // Base cost: 6 coins  
    Rare,        // Base cost: 8 coins
    Legendary,   // Base cost: 20 coins
}
```

## PyO3 Integration

### Python Bindings

The joker system is exposed to Python through PyO3. Following the dual framework elimination, use `GameEngine` for actions and `GameState` for read-only access:

```python
# Create game engine (handles actions)
engine = pylatro.GameEngine()

# Access read-only state for joker information
state = engine.state

# Get joker IDs from state (new registry-based API)
for joker_id in state.joker_ids:
    joker_info = engine.get_joker_info(joker_id)
    if joker_info:
        print(f"{joker_info.name}: {joker_info.description}")

# Helper methods for easy access
joker_names = state.get_joker_names()
joker_descriptions = state.get_joker_descriptions()
for name, desc in zip(joker_names, joker_descriptions):
    print(f"{name}: {desc}")
```

### Migration from Legacy API

```python
# DEPRECATED: Old dual framework API (still works with warnings)
# state.gen_actions()     # Shows deprecation warning
# state.is_over          # Shows deprecation warning
# state.jokers()         # Shows deprecation warning

# CURRENT: New single framework API
engine = pylatro.GameEngine()
actions = engine.gen_actions()     # ✅ Use GameEngine for actions
is_over = engine.is_over          # ✅ Use GameEngine for game state
state = engine.state              # ✅ Use GameState for read-only access
joker_ids = state.joker_ids       # ✅ Use new registry-based API
```

## Error Handling

### Common Error Types

```rust
pub enum JokerError {
    InvalidCondition(String),
    StateNotFound(JokerId),
    SerializationError(serde_json::Error),
    ValidationError(String),
}
```

### Result Types

Most operations return `Result<T, JokerError>` for proper error handling:

```rust
// Building static jokers
let joker = StaticJoker::builder(id, name, desc)
    .condition(condition)
    .build()?; // Returns Result<StaticJoker, String>

// State operations  
let state = state_manager.get_state(&joker_id)
    .ok_or(JokerError::StateNotFound(joker_id))?;
```

## Thread Safety

All core joker types implement `Send + Sync`:

- **Joker trait**: All implementations must be thread-safe
- **JokerRegistry**: Uses `RwLock` for thread-safe access
- **JokerStateManager**: Uses `RwLock<HashMap>` for concurrent access
- **Static jokers**: Immutable and naturally thread-safe

## Performance Considerations

### Zero-Cost Abstractions

- **Static jokers**: Compile-time optimizations for simple conditions
- **Trait objects**: Minimal overhead for polymorphism
- **Condition checking**: O(1) for most conditions

### Memory Efficiency

- **Box<dyn Joker>**: Single allocation for polymorphic jokers
- **Static strings**: All names/descriptions are `&'static str`
- **State management**: Optional state only allocated when needed

### Optimization Tips

- Use static framework for simple jokers (fastest)
- Implement early returns in condition checking
- Batch effect applications when possible
- Cache expensive calculations in joker state