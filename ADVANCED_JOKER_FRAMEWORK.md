# Advanced Joker Condition Framework

## Overview

The Advanced Joker Condition Framework is a sophisticated extension to the existing joker system that supports complex conditional logic, performance optimization, and rich game state access while maintaining full backward compatibility.

## Key Features

### 🚀 **Advanced Conditions**
- **State-Dependent Conditions**: Access joker internal state for complex logic
- **Temporal Conditions**: "After X triggers", "During Y phase", sequence-based patterns
- **Composite Conditions**: Complex AND/OR/NOT logic with short-circuiting
- **Performance-Optimized**: Built-in caching and evaluation cost estimation

### 🔧 **Enhanced Trait System**
- **Rich Context Access**: Full game state, history, and optimization systems
- **Flexible Processing**: Sophisticated condition evaluation with context awareness
- **Performance Tuning**: Evaluation cost estimates and priority-based processing
- **Thread Safety**: Full Send + Sync support for concurrent access

### 🔄 **Backward Compatibility**
- **Zero Breaking Changes**: All existing jokers continue to work unchanged
- **Migration Path**: Clear upgrade path from legacy to advanced system
- **Mixed Collections**: Old and new jokers work together seamlessly
- **Performance Maintained**: No overhead for simple legacy jokers

## Architecture

### Core Components

#### 1. Advanced Conditions (`AdvancedCondition`)
```rust
// Simple conditions
AdvancedCondition::HandsPlayedThisRound(3)
AdvancedCondition::JokerStateGreaterThan { joker_id, state_key: "power", threshold: 10.0 }

// Complex compositions
AdvancedCondition::FastAnd {
    conditions: vec![
        AdvancedCondition::DuringStage(Stage::Blind(Blind::Big)),
        AdvancedCondition::MoneyGreaterThan(100),
        AdvancedCondition::JokerTriggeredCount { joker_id, count: 5 },
    ],
    short_circuit: true,
}
```

#### 2. Enhanced Traits
- **`AdvancedJokerGameplay`**: Core gameplay with rich context access
- **`AdvancedJokerIdentity`**: Enhanced identity with performance metadata
- **`JokerProcessor`**: Flexible processing logic with state management

#### 3. Performance Optimization
- **`ConditionCache`**: Automatic result caching for expensive conditions
- **`EvaluationCost`**: Performance cost estimation for optimization
- **Priority Processing**: Higher priority jokers process first

#### 4. Compatibility Bridge
- **`LegacyJokerAdapter`**: Seamless integration for existing jokers
- **`MixedJokerCollection`**: Collections handling both old and new jokers
- **Automatic Migration**: Tools for upgrading legacy jokers

## Usage Examples

### Creating an Advanced Joker
```rust
use balatro_rs::joker::{
    AdvancedConditionBuilder, EnhancedJokerBuilder, EvaluationCost
};

// Define a sophisticated joker condition
let condition = AdvancedConditionBuilder::fast_and(vec![
    AdvancedConditionBuilder::hands_played_this_round(2),
    AdvancedConditionBuilder::joker_state_gt(JokerId::Joker, "power", 5.0),
    AdvancedConditionBuilder::during_stage(Stage::Blind(Blind::Big)),
]);

// Create the enhanced joker
let joker = EnhancedJokerBuilder::new()
    .identity(Box::new(TestJokerIdentity::new("Power Joker")))
    .condition(condition)
    .processor(Box::new(PowerJokerProcessor::new(50, 3.0)))
    .priority(10) // High priority processing
    .build()?;
```

### Upgrading Legacy Jokers
```rust
use balatro_rs::joker::{CompatibilityBridge, ConditionalJoker};

// Existing conditional joker
let legacy_joker = ConditionalJoker::new(
    JokerId::Banner,
    "Banner",
    "+40 chips when money < 50",
    JokerRarity::Common,
    JokerCondition::MoneyLessThan(50),
    JokerEffect::new().with_chips(40),
);

// Upgrade to advanced framework
let enhanced_joker = CompatibilityBridge::upgrade_conditional_joker(legacy_joker)?;
```

### Mixed Collections
```rust
use balatro_rs::joker::{CompatibilityBridge, MixedJokerCollection};

// Create collection with both legacy and advanced jokers
let mixed_collection = CompatibilityBridge::create_mixed_collection(
    vec![Box::new(LegacyJoker::new())],      // Legacy jokers
    vec![Box::new(enhanced_joker)],          // Advanced jokers
);

// Process all jokers with proper priority ordering
let results = mixed_collection.process_all(&mut context);
```

## Performance Characteristics

### Evaluation Costs
- **Cheap**: Simple comparisons (money, counters) - ~10ns
- **Moderate**: State access, basic logic - ~100ns
- **Expensive**: Complex conditions, multiple state queries - ~1μs
- **Very Expensive**: Intensive computation - ~10μs+

### Caching Strategy
- Automatic caching for expensive conditions
- Cache invalidation based on context and state changes
- Configurable cache lifetime hints
- Statistics tracking for performance monitoring

### Processing Optimization
- Priority-based evaluation ordering
- Short-circuit evaluation for composite conditions
- Minimal allocations in hot paths
- Zero-cost abstractions for simple cases

## Migration Guide

### Phase 1: No Changes Required
All existing jokers continue to work without modification.

### Phase 2: Gradual Enhancement
```rust
// Enhance specific jokers with advanced conditions
let enhanced_condition = AdvancedConditionBuilder::fast_and(vec![
    AdvancedConditionBuilder::legacy(existing_condition),
    AdvancedConditionBuilder::joker_state_gt(joker_id, "power", 10.0),
]);

let adapter = LegacyJokerAdapter::with_condition(existing_joker, enhanced_condition);
```

### Phase 3: Full Migration
```rust
// Replace with fully advanced implementation
impl AdvancedJokerGameplay for MyJoker {
    fn get_trigger_condition(&self) -> &AdvancedCondition {
        &self.sophisticated_condition
    }

    fn process_advanced(&mut self, context: &mut AdvancedEvaluationContext) -> ProcessResult {
        // Rich context access for complex logic
        if context.game_history.hands_played_this_round > 2 {
            self.internal_state.increment_counter("powerful_triggers");
            ProcessResult {
                chips_added: self.calculate_dynamic_bonus(context),
                mult_added: self.get_scaling_multiplier(),
                ..Default::default()
            }
        } else {
            ProcessResult::default()
        }
    }
}
```

## Framework Benefits

### For Simple Jokers
- **Zero Overhead**: No performance cost for basic conditions
- **Easy Upgrade**: Automatic adapter generation
- **Familiar API**: Existing patterns continue to work

### for Complex Jokers
- **Rich Context**: Access to full game state and history
- **State Management**: Built-in internal state with serialization
- **Performance**: Automatic optimization and caching
- **Composition**: Complex condition logic with clean syntax

### For the Game Engine
- **Maintainability**: Clear separation of concerns
- **Performance**: Optimized evaluation with minimal overhead
- **Extensibility**: Easy to add new condition types
- **Debugging**: Rich debugging and introspection capabilities

## Implementation Quality

### Kernel-Level Standards
- **Memory Safety**: Zero unsafe code, proper lifetime management
- **Performance**: Efficient algorithms, minimal allocations
- **Error Handling**: Comprehensive error paths with proper cleanup
- **Thread Safety**: Full Send + Sync support for concurrent access

### Testing Coverage
- **Unit Tests**: Every condition type and trait method
- **Integration Tests**: Complex scenarios and interaction patterns
- **Performance Tests**: Benchmarks and regression detection
- **Compatibility Tests**: Legacy joker integration verification

### Documentation
- **API Documentation**: Comprehensive docs for all public APIs
- **Examples**: Real-world usage patterns and best practices
- **Migration Guide**: Clear path from legacy to advanced system
- **Performance Guide**: Optimization strategies and cost analysis

## Future Extensions

### Planned Features
- **Custom Condition Types**: Plugin system for domain-specific conditions
- **Visual Debugging**: Tools for condition evaluation visualization
- **Analytics Integration**: Performance metrics and usage statistics
- **Hot Reloading**: Dynamic condition updates during development

### Extension Points
- **Condition Evaluators**: Custom evaluation logic via traits
- **State Serialization**: Custom state formats and migration
- **Performance Monitoring**: Pluggable metrics collection
- **Event System**: Rich event handling for complex interactions

## Conclusion

The Advanced Joker Condition Framework represents a significant enhancement to the joker system while maintaining the reliability and simplicity that makes the existing system work. It provides a clear path forward for implementing sophisticated joker behaviors while ensuring that simple jokers remain simple and performant.

The framework follows kernel-quality development principles: correct by design, efficient by default, and maintainable for the long term. Every existing joker continues to work unchanged, while new jokers can leverage the full power of the advanced system.

**Key Takeaway**: This is evolution, not revolution. We've enhanced what works while fixing what was limiting, creating a system that scales from simple conditional logic to sophisticated game mechanics without sacrificing performance or maintainability.
