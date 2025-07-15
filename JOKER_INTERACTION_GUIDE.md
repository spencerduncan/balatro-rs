# Joker Interaction Guide

## Overview

This guide explains how jokers interact with the game engine, each other, and different game events. Understanding these interactions is crucial for implementing complex jokers and predicting game behavior.

## Game Engine Integration

### Lifecycle Integration

Jokers integrate with the game through a well-defined lifecycle that mirrors Balatro's game flow:

```
Blind Start → Hand Selection → Hand Played → Card Scoring → Discard → Round End
     ↓              ↓              ↓              ↓           ↓         ↓
on_blind_start  (selection)  on_hand_played  on_card_scored  on_discard  on_round_end
```

### Event Sequence

1. **Blind Start**: `on_blind_start()` called for all jokers
2. **Hand Play**: `on_hand_played()` called when hand is played
3. **Card Scoring**: `on_card_scored()` called for each scoring card
4. **Discard**: `on_discard()` called when cards are discarded
5. **Round End**: `on_round_end()` called at round completion
6. **Shop**: `on_shop_open()` called when entering shop

### Action Integration

Jokers integrate with the action system:

```rust
// Actions that affect jokers
pub enum Action {
    BuyJoker { joker_id: JokerId, slot: usize },
    SellJoker(usize),           // Sell joker at slot
    ReorderJokers(Vec<usize>),  // Reorder joker slots
    // ... other actions
}
```

## Joker-to-Joker Interactions

### Execution Order

Jokers execute in **left-to-right order** (slot 0 to slot N), which can create important interactions:

```rust
// Example: Jokers affecting each other
// Slot 0: Steel Joker (+0.5 mult multiplier per Steel card)
// Slot 1: Greedy Joker (+3 mult per Diamond)
// 
// With 2 Steel Diamonds:
// 1. Steel Joker: +1.0 mult multiplier (2 × 0.5)
// 2. Greedy Joker: +6 mult (2 × 3)
// 3. Final calculation: (base_mult + 6) × 2.0
```

### Context Sharing

All jokers share the same `GameContext`, allowing complex interactions:

```rust
// Example: Banner Joker checks remaining discards
fn on_hand_played(&self, context: &mut GameContext, hand: &SelectHand) -> JokerEffect {
    let remaining_discards = context.stage.discards - context.discards_used;
    if remaining_discards > 0 {
        JokerEffect::new().with_chips(40)
    } else {
        JokerEffect::new()
    }
}
```

### State Dependencies

Jokers can depend on state from other jokers:

```rust
// Example: Jokers that scale based on other jokers
fn on_hand_played(&self, context: &mut GameContext, hand: &SelectHand) -> JokerEffect {
    let joker_count = context.jokers.len();
    JokerEffect::new().with_mult(joker_count as i32 * 2)
}
```

## Scoring Integration

### Effect Application Order

Effects are applied in a specific order to ensure consistent behavior:

1. **Chips calculation**: Base chips + chip bonuses
2. **Mult calculation**: Base mult + mult bonuses  
3. **Mult multiplication**: Apply mult multipliers
4. **Final score**: chips × mult

### Per-Card vs Per-Hand Effects

Understanding when effects apply is crucial:

```rust
// Per-card effect (applies to each scoring card)
impl Joker for GreedyJoker {
    fn on_card_scored(&self, context: &mut GameContext, card: &Card) -> JokerEffect {
        if card.suit == Suit::Diamond {
            JokerEffect::new().with_mult(3) // +3 per Diamond
        } else {
            JokerEffect::new()
        }
    }
}

// Per-hand effect (applies once per hand)
impl Joker for JollyJoker {
    fn on_hand_played(&self, context: &mut GameContext, hand: &SelectHand) -> JokerEffect {
        if hand.is_pair().is_some() {
            JokerEffect::new().with_mult(8) // +8 for any Pair
        } else {
            JokerEffect::new()
        }
    }
}
```

### Cumulative Effects

Multiple jokers can provide the same type of bonus:

```rust
// Example hand with 3 Diamonds and multiple jokers:
// - Greedy Joker: +9 mult (3 × 3)
// - Jolly Joker: +8 mult (Pair bonus)
// - Basic Joker: +4 mult (always)
// Total: +21 mult
```

## Game State Interactions

### Money Interactions

Many jokers interact with the player's money:

```rust
// Banner Joker: Bonus based on remaining discards
if context.discards_used < max_discards {
    JokerEffect::new().with_chips(40)
}

// Economic jokers that award money
JokerEffect::new().with_money(5)
```

### Ante/Round Interactions

Some jokers scale with game progression:

```rust
// Jokers that get stronger over time
let bonus = context.ante as i32 * 2; // +2 mult per ante
JokerEffect::new().with_mult(bonus)
```

### Hand Size Interactions

Jokers can modify hand size and other game parameters:

```rust
fn modify_hand_size(&self, context: &GameContext, base_size: usize) -> usize {
    base_size + 2 // +2 hand size
}

fn modify_discards(&self, context: &GameContext, base_discards: usize) -> usize {
    base_discards + 1 // +1 discard
}
```

## Complex Interaction Patterns

### Conditional Scaling

Jokers that become more powerful based on game state:

```rust
// Example: Joker that scales with money
fn on_hand_played(&self, context: &mut GameContext, hand: &SelectHand) -> JokerEffect {
    let money_bonus = (context.money / 10) as i32; // +1 mult per $10
    JokerEffect::new().with_mult(money_bonus)
}
```

### Trigger Limits

Some jokers have limited uses:

```rust
// Example: Joker with 3 triggers
fn on_hand_played(&self, context: &mut GameContext, hand: &SelectHand) -> JokerEffect {
    let state_manager = JokerStateManager::new();
    
    if state_manager.decrement_triggers(&self.id()) {
        JokerEffect::new().with_mult(50) // Big bonus but limited uses
    } else {
        JokerEffect::new()
    }
}
```

### Synergy Effects

Jokers designed to work together:

```rust
// Example: Jokers that synergize with specific cards
// - Flower Pot: +3 mult if hand contains Hearts
// - Lusty Joker: +3 mult per Heart
// Together: Powerful Heart-based strategy
```

## Common Interaction Patterns

### 1. Multiplicative Scaling

```rust
// Pattern: Percentage-based multipliers
fn on_hand_played(&self, context: &mut GameContext, hand: &SelectHand) -> JokerEffect {
    let multiplier = 1.5; // +50% mult
    JokerEffect::new().with_mult_multiplier(multiplier)
}
```

### 2. Conditional Activation

```rust
// Pattern: Activate only under specific conditions
fn on_hand_played(&self, context: &mut GameContext, hand: &SelectHand) -> JokerEffect {
    if context.money < 10 {
        JokerEffect::new().with_chips(100) // Bonus when poor
    } else {
        JokerEffect::new()
    }
}
```

### 3. Accumulation Over Time

```rust
// Pattern: Build up value over multiple rounds
fn on_round_end(&self, context: &mut GameContext) -> JokerEffect {
    let state_manager = JokerStateManager::new();
    state_manager.increment_accumulated(&self.id(), 2); // +2 each round
    JokerEffect::new()
}

fn on_hand_played(&self, context: &mut GameContext, hand: &SelectHand) -> JokerEffect {
    let state_manager = JokerStateManager::new();
    let accumulated = state_manager.get_state(&self.id())
        .map(|s| s.accumulated_value)
        .unwrap_or(0.0);
    
    JokerEffect::new().with_mult(accumulated)
}
```

### 4. Resource Management

```rust
// Pattern: Jokers that manage limited resources
fn on_hand_played(&self, context: &mut GameContext, hand: &SelectHand) -> JokerEffect {
    if self.can_trigger() {
        self.consume_charge();
        JokerEffect::new().with_chips(200)
    } else {
        JokerEffect::new()
    }
}
```

## Anti-Patterns to Avoid

### 1. Order Dependencies

**Bad**: Jokers that break when reordered
```rust
// DON'T: Assume specific joker positions
let other_joker = context.jokers[0]; // Fragile!
```

**Good**: Jokers that work regardless of order
```rust
// DO: Search for specific joker types
let steel_jokers = context.jokers.iter()
    .filter(|j| j.id() == JokerId::SteelJoker)
    .count();
```

### 2. Hidden State Mutations

**Bad**: Modifying context in unexpected ways
```rust
// DON'T: Secretly modify money without returning money effect
context.money += 10; // Hidden side effect!
```

**Good**: Explicit effect declarations
```rust
// DO: Return money effects explicitly
JokerEffect::new().with_money(10)
```

### 3. Expensive Operations

**Bad**: Slow operations in hot paths
```rust
// DON'T: Expensive calculations every card
fn on_card_scored(&self, context: &mut GameContext, card: &Card) -> JokerEffect {
    let expensive_calculation = complex_analysis(context); // Slow!
    JokerEffect::new().with_mult(expensive_calculation)
}
```

**Good**: Cache expensive calculations
```rust
// DO: Cache in joker state when possible
let cached_value = state_manager.get_cached_value(&self.id())
    .unwrap_or_else(|| self.calculate_and_cache(context));
```

## Testing Interactions

### Unit Testing Patterns

```rust
#[test]
fn test_joker_interaction() {
    let mut context = create_test_context();
    let greedy = GreedyJoker;
    let jolly = JollyJoker;
    
    // Test individual effects
    let card = Card::new(Value::Ace, Suit::Diamond);
    let greedy_effect = greedy.on_card_scored(&mut context, &card);
    assert_eq!(greedy_effect.mult, 3);
    
    // Test combined effects
    let hand = create_pair_hand_with_diamonds();
    let jolly_effect = jolly.on_hand_played(&mut context, &hand);
    assert_eq!(jolly_effect.mult, 8);
}
```

### Integration Testing

```rust
#[test]
fn test_full_scoring_interaction() {
    let mut game = Game::new(Config::default());
    
    // Add jokers
    game.add_joker(JokerId::GreedyJoker);
    game.add_joker(JokerId::JollyJoker);
    
    // Play hand and verify total effect
    let hand = vec![
        Card::new(Value::Ace, Suit::Diamond),
        Card::new(Value::Ace, Suit::Heart),
    ];
    
    let result = game.play_hand(hand);
    
    // Verify combined effects: Pair bonus + Diamond bonus
    assert!(result.final_score > base_pair_score + diamond_bonus);
}
```

## Debugging Interactions

### Effect Tracing

```rust
// Enable effect tracing for debugging
impl Joker for DebugJoker {
    fn on_card_scored(&self, context: &mut GameContext, card: &Card) -> JokerEffect {
        let effect = self.calculate_effect(card);
        
        #[cfg(debug_assertions)]
        eprintln!("Joker {} applied effect {:?} to card {:?}", 
                 self.name(), effect, card);
        
        effect
    }
}
```

### State Inspection

```rust
// Inspect joker state for debugging
#[cfg(debug_assertions)]
fn debug_joker_state(jokers: &[Box<dyn Joker>]) {
    for joker in jokers {
        if let Some(state) = JokerStateManager::get_state(&joker.id()) {
            eprintln!("Joker {} state: {:?}", joker.name(), state);
        }
    }
}
```

This interaction system provides a powerful foundation for implementing complex joker behaviors while maintaining predictable and testable game mechanics.