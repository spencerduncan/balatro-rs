# Scaling Joker Framework Design

## Overview

The Scaling Joker Framework provides a comprehensive system for implementing jokers that accumulate value over time based on various game triggers. This addresses issue #60 by implementing 15 scaling jokers with proper state persistence, trigger handling, and performance optimization.

## Architecture

### Core Components

1. **ScalingJoker Struct** (`scaling_joker.rs`)
   - Base framework for all scaling jokers
   - Integrates with `JokerStateManager` for persistence
   - Supports configurable triggers, effects, and reset conditions

2. **ScalingTrigger Enum**
   - Defines all possible trigger conditions
   - Includes hand types, game events, and resource changes
   - Extensible for future trigger types

3. **ScalingEffectType Enum**
   - Defines types of effects (chips, mult, money, etc.)
   - Supports multiplicative effects for exponential scaling
   - Custom effect type for complex behaviors

4. **ResetCondition Enum**
   - Defines when accumulated values reset
   - Supports round-based, ante-based, and event-based resets
   - Optional (some jokers never reset)

### Implementation Patterns

#### Pattern 1: Basic Scaling Joker
For simple scaling behaviors, use the `ScalingJoker` struct directly:

```rust
let spare_trousers = ScalingJoker::new(
    JokerId::Trousers,
    "Spare Trousers".to_string(),
    "+2 Mult per Two Pair hand played".to_string(),
    JokerRarity::Uncommon,
    0.0,  // Base value
    2.0,  // Increment per trigger
    ScalingTrigger::HandPlayed(HandRank::TwoPair),
    ScalingEffectType::Mult,
);
```

#### Pattern 2: Custom Logic Scaling Joker
For complex behaviors requiring custom logic, implement the `Joker` trait directly:

```rust
#[derive(Debug, Clone)]
pub struct GreenJoker {
    base: ScalingJoker,
}

impl Joker for GreenJoker {
    fn on_hand_played(&self, context: &mut GameContext, hand: &SelectHand) -> JokerEffect {
        // Custom increment logic
        context.joker_state_manager.update_state(self.id(), |state| {
            state.accumulated_value += 1.0;
        });
        // Custom effect calculation
        let current_value = context.joker_state_manager
            .get_accumulated_value(self.id()).unwrap_or(0.0);
        JokerEffect::new().with_mult(current_value as i32)
    }
    
    fn on_discard(&self, context: &mut GameContext, cards: &[Card]) -> JokerEffect {
        // Custom decrement logic
        context.joker_state_manager.update_state(self.id(), |state| {
            state.accumulated_value = (state.accumulated_value - 1.0).max(0.0);
        });
        JokerEffect::new()
    }
}
```

## Implemented Scaling Jokers

### 1. Spare Trousers
- **Effect**: +2 Mult per Two Pair hand played
- **Trigger**: HandPlayed(TwoPair)
- **Reset**: Never
- **Rarity**: Uncommon

### 2. Square Joker
- **Effect**: +4 Chips per 4-card hand played
- **Trigger**: Custom logic (checks hand size)
- **Reset**: Never
- **Rarity**: Common

### 3. Bull
- **Effect**: +2 Chips per $1 owned
- **Trigger**: Dynamic (based on current money)
- **Reset**: N/A (not accumulated)
- **Rarity**: Common

### 4. Bootstraps
- **Effect**: +2 Mult per $5 owned
- **Trigger**: Dynamic (based on current money ÷ 5)
- **Reset**: N/A (not accumulated)
- **Rarity**: Uncommon

### 5. Fortune Teller
- **Effect**: +1 Mult per Tarot card used
- **Trigger**: ConsumableUsed
- **Reset**: Never
- **Rarity**: Common

### 6. Ceremonial Dagger
- **Effect**: Mult doubles when blind starts
- **Trigger**: BlindStart (custom logic)
- **Reset**: RoundEnd
- **Rarity**: Uncommon

### 7. Banner
- **Effect**: +30 Chips per discard remaining
- **Trigger**: Dynamic (based on remaining discards)
- **Reset**: RoundEnd
- **Rarity**: Common

### 8. Throwback
- **Effect**: +0.5X Mult per shop reroll
- **Trigger**: ShopReroll
- **Reset**: ShopEntered
- **Rarity**: Uncommon

### 9. Green Joker
- **Effect**: +1 Mult per hand, -1 per discard
- **Trigger**: Custom logic (dual triggers)
- **Reset**: Never
- **Rarity**: Common

### 10. Red Card
- **Effect**: +3 Mult per pack skipped
- **Trigger**: Custom logic (pack skipping)
- **Reset**: Never
- **Rarity**: Common

### 11. Steel Joker (Scaling Version)
- **Effect**: +0.2X Mult per card destroyed
- **Trigger**: CardDestroyed
- **Reset**: Never
- **Rarity**: Uncommon

### 12. Mystic Summit
- **Effect**: +15 Mult per unique hand type played
- **Trigger**: Custom logic (tracks unique hand types)
- **Reset**: Never
- **Rarity**: Rare

### 13. Marble Joker (Scaling Version)
- **Effect**: +50 Chips per joker sold
- **Trigger**: JokerSold
- **Reset**: Never
- **Rarity**: Rare

### 14. Loyalty Card
- **Effect**: +1 Mult per blind completed this ante
- **Trigger**: BlindCompleted
- **Reset**: AnteEnd
- **Rarity**: Common

### 15. Castle
- **Effect**: +300 Chips per discard used this round
- **Trigger**: CardDiscarded
- **Reset**: RoundEnd
- **Max Value**: 1200 (4 discards maximum)
- **Rarity**: Rare

## State Management

### Persistence
All scaling jokers use the `JokerStateManager` for state persistence:
- Accumulated values are stored in `JokerState.accumulated_value`
- Custom data can be stored using `set_custom_data()`
- State is automatically saved/loaded with game saves
- Thread-safe for concurrent access

### Performance Optimizations
1. **Efficient State Updates**: Use `update_state()` for atomic updates
2. **Bulk Operations**: Use `bulk_update()` for multiple jokers
3. **Memory Management**: Use `compact_states()` to remove inactive jokers
4. **Validation**: Built-in validation prevents invalid states

## Integration Points

### Game Event System
Scaling jokers integrate with the game through well-defined hooks:
- `on_hand_played()` - Most common trigger point
- `on_discard()` - For discard-based scaling
- `on_shop_open()` - For shop-related triggers
- `on_round_end()` - For cleanup and resets
- `on_blind_start()` - For blind-start triggers

### Factory Functions
Use factory functions for consistent joker creation:
```rust
// Get all scaling jokers
let all_jokers = create_all_scaling_jokers();

// Get specific joker by ID
let spare_trousers = get_scaling_joker_by_id(JokerId::Trousers);

// Get custom implementation jokers
let green_joker = get_custom_scaling_joker_by_id(JokerId::GreenJoker);
```

## Testing Strategy

### Unit Tests
- Test framework components in isolation
- Verify trigger logic and effect calculations
- Test reset conditions and max value caps
- Validate factory functions and ID mapping

### Integration Tests
- Test joker behavior in actual game context
- Verify state persistence across sessions
- Test performance with multiple scaling jokers
- Validate interaction with other game systems

### Performance Tests
- Measure impact on scoring performance
- Test memory usage with many active jokers
- Benchmark state update operations
- Validate thread safety under load

## Future Extensions

### Additional Triggers
The framework is designed to be extensible:
- Card purchase triggers
- Blind defeat triggers
- Ante progression triggers
- Custom game mode triggers

### Enhanced Effects
- Complex multiplicative stacking
- Conditional effect application
- Cross-joker interactions
- Dynamic effect types

### Advanced Features
- Joker evolution based on accumulated value
- Synergy bonuses between scaling jokers
- Meta-progression across runs
- Conditional reset logic

## Best Practices

### For Implementers
1. Always use the `JokerStateManager` for persistence
2. Implement proper error handling for state operations
3. Use appropriate effect types for the desired behavior
4. Document custom logic clearly
5. Add comprehensive tests for new scaling jokers

### For Game Integration
1. Process scaling events in the correct order
2. Apply resets before triggers in the same frame
3. Validate state consistency after updates
4. Use bulk operations for performance
5. Monitor memory usage in long-running games

## File Structure

```
core/src/
├── scaling_joker.rs           # Core framework
├── scaling_joker_impl.rs      # Basic implementations
├── scaling_joker_custom.rs    # Custom logic implementations
└── lib.rs                     # Module exports

core/tests/
└── scaling_joker_tests.rs     # Comprehensive test suite
```

This design provides a robust, extensible foundation for scaling jokers while maintaining performance and consistency with the existing Balatro-RS architecture.