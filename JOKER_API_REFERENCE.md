# Joker System API Reference

## Overview

The Balatro-RS joker system provides a comprehensive framework for implementing all 150 Balatro jokers with multiple implementation patterns optimized for different use cases.

## Core API

### Joker Trait

The central trait that all jokers implement:

```rust
pub trait Joker: Send + Sync + std::fmt::Debug {
    // Identity methods
    fn id(&self) -> JokerId;
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn rarity(&self) -> JokerRarity;
    fn cost(&self) -> usize;

    // Lifecycle hooks (all have default implementations)
    fn on_hand_played(&self, context: &mut GameContext, hand: &SelectHand) -> JokerEffect;
    fn on_card_scored(&self, context: &mut GameContext, card: &Card) -> JokerEffect;
    fn on_blind_start(&self, context: &mut GameContext) -> JokerEffect;
    fn on_shop_open(&self, context: &mut GameContext) -> JokerEffect;
    fn on_discard(&self, context: &mut GameContext, cards: &[Card]) -> JokerEffect;
    fn on_round_end(&self, context: &mut GameContext) -> JokerEffect;
    
    // Modifier hooks (all have default implementations)
    fn modify_chips(&self, context: &GameContext, base_chips: i32) -> i32;
    fn modify_mult(&self, context: &GameContext, base_mult: i32) -> i32;
    fn modify_hand_size(&self, context: &GameContext, base_size: usize) -> usize;
    fn modify_discards(&self, context: &GameContext, base_discards: usize) -> usize;
}
```

### JokerEffect

Represents the effect a joker has on scoring:

```rust
pub struct JokerEffect {
    pub chips: i32,           // Bonus chips to add
    pub mult: i32,            // Bonus mult to add  
    pub money: i32,           // Money to award
    pub mult_multiplier: f32, // Multiplier to apply to mult
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
    pub fn all_jokers() -> Vec<JokerId>;
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
    pub accumulated_value: i32,
    pub triggers_remaining: Option<u32>,
    pub custom_data: serde_json::Value,
}

impl JokerState {
    pub fn new() -> Self;
    pub fn with_accumulated_value(value: i32) -> Self;
    pub fn with_triggers_remaining(triggers: u32) -> Self;
    pub fn with_custom_data<T: Serialize>(data: T) -> Result<Self, serde_json::Error>;
    pub fn get_custom_data<T: DeserializeOwned>(&self) -> Result<T, serde_json::Error>;
}
```

### JokerStateManager

Thread-safe state management:

```rust
pub struct JokerStateManager;

impl JokerStateManager {
    pub fn new() -> Self;
    pub fn get_state(&self, joker_id: &JokerId) -> Option<JokerState>;
    pub fn set_state(&self, joker_id: JokerId, state: JokerState);
    pub fn update_state<F>(&self, joker_id: &JokerId, f: F) -> bool
    where F: FnOnce(&mut JokerState);
    pub fn clear_state(&self, joker_id: &JokerId) -> bool;
    pub fn has_state(&self, joker_id: &JokerId) -> bool;
    
    // Helper methods
    pub fn increment_accumulated(&self, joker_id: &JokerId, amount: i32);
    pub fn decrement_triggers(&self, joker_id: &JokerId) -> bool;
    pub fn reset_triggers(&self, joker_id: &JokerId, count: u32);
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

All 186 joker identifiers are defined in the `JokerId` enum. Major categories include:

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

The joker system is exposed to Python through PyO3:

```python
# Get jokers from game state
jokers = game_state.jokers()

# Each joker exposes basic information
for joker in jokers:
    print(f"{joker.name}: {joker.description}")
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