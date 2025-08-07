# CLAUDE.md - Joker System

## Directory Purpose

The joker system implements the core gameplay mechanic of special cards that provide scoring bonuses, modifiers, and special effects. It represents a sophisticated evolution from a monolithic design to a highly modular, trait-based architecture supporting multiple implementation patterns.

## System Architecture

### Architectural Layers

1. **Core Trait Layer** (`mod.rs`, `traits.rs`)
   - Central `Joker` trait defining fundamental interface
   - Focused single-responsibility traits
   - ~1400 lines of comprehensive documentation

2. **Implementation Patterns Layer**
   - **Static Framework** (`static_joker.rs`): High-performance, compile-time jokers
   - **Conditional Framework** (`conditional.rs`): Declarative condition-based jokers
   - **Advanced Framework** (`advanced_traits.rs`, `advanced_conditions.rs`): Rich context and sophisticated conditions
   - **Direct Implementation**: Custom jokers with full control

3. **Factory & Registry Layer** (`joker_factory.rs`, `joker_registry.rs`)
   - Centralized joker creation and registration
   - Thread-safe registry with RwLock protection
   - Support for both static and dynamic creation

4. **Compatibility Bridge Layer** (`compatibility_bridge.rs`, `compat.rs`)
   - Seamless integration between old and new systems
   - Zero-overhead adapters for legacy jokers
   - Mixed collections supporting both paradigms

## Core Components

### JokerId Enum (194 variants)
- Comprehensive enumeration of all joker types
- Unique identifier across the system
- Supports serialization/deserialization for save games

### JokerEffect Struct
Encapsulates all possible effects:
- **Scoring**: chips, mult, mult_multiplier
- **Resources**: money, interest_bonus
- **Special**: retrigger, destroy_self, transform_cards
- **Modifiers**: hand_size_mod, discard_mod
- **Creation**: consumable generation support

### GameContext Struct
Provides comprehensive game state access:
- Current scoring state (chips, mult, money)
- Game progression (ante, round, stage)
- Hand information (cards, discards, hand types)
- Deck composition (stone/steel/enhanced cards)
- Joker state management
- Secure RNG access

## Trait-Based Design

### Core Trait Hierarchy
```
Joker (monolithic, being phased out)
├── JokerIdentity (metadata)
├── JokerLifecycle (state transitions)
├── JokerGameplay (scoring logic)
├── JokerModifiers (game modifiers)
└── JokerState (persistence)
```

### Advanced Trait Extensions
```
AdvancedJokerGameplay
├── AdvancedJokerIdentity (enhanced metadata)
├── JokerProcessor (effect processing)
└── InternalJokerState (sophisticated state)
```

### Key Features
- **Send + Sync Bounds**: Thread safety for concurrent RL training
- **Mutable Processing**: Internal state management support
- **Lifecycle Hooks**: Comprehensive event system
- **State Management**: Built-in serialization/deserialization

## Factory & Registry System

### JokerFactory
Static factory with specialized delegates:
- `StaticJokerFactory` for high-performance jokers
- `ScalingJokerFactory` for scaling effects
- Direct instantiation for custom implementations
- Supports ~142 joker implementations

### JokerRegistry
Thread-safe registration system:
- Stores joker definitions and factory functions
- Provides querying by rarity, unlock conditions
- Centralized cost calculation based on rarity

## Implementation Categories

### Basic Jokers (15+ implementations)
- **Additive Mult**: Joker, Greedy, Lusty, Wrathful
- **Additive Chips**: Banner, Blue Joker, Scary Face
- **Multiplicative**: Steel, Baron, Ancient, Photograph

### Conditional Jokers (20+ implementations)
- **Money-based**: Business Card, Egg, Burglar
- **Hand-type**: Jolly (Pair), Zany (Three of a Kind)
- **Card-based**: Even Steven, Odd Todd, Faceless

### Scaling Jokers (15+ implementations)
- **Chips Scaling**: Castle, Wee, Stuntman, Hiker
- **Mult Scaling**: Spare Trousers, Green Joker, Red Card
- **XMult Scaling**: Throwback, Ceremonial Dagger

### Special Mechanics (10+ implementations)
- **Hand Modification**: Four Fingers, Shortcut
- **Retrigger**: Dusk, Seltzer, Hanging Chad
- **Economy**: Delayed Gratification, Rocket, To the Moon
- **RNG-based**: Fortune Teller, Vagabond, Oops All Sixes

## Migration Strategy

### Phase 1: Compatibility Layer
- `LegacyJokerAdapter` wraps old implementations
- `MixedJokerCollection` handles both paradigms
- Zero performance overhead for legacy code

### Phase 2: Trait Decomposition
- Split monolithic trait into focused traits
- Implement traits independently for new jokers
- Gradual conversion of existing jokers

### Phase 3: Advanced Features
- `EnhancedJoker` with sophisticated conditions
- Condition caching for performance
- State-dependent and temporal conditions

## Game System Integration

### Scoring Integration
```rust
pub fn process_joker_effects(&mut self, hand: &MadeHand) -> AccumulatedEffects {
    let context = self.create_game_context();

    // Process per-hand effects
    for joker in &mut self.jokers {
        let effect = joker.on_hand_played(&mut context, hand);
        self.apply_effect(effect);
    }

    // Process per-card effects
    for card in hand.scoring_cards() {
        for joker in &mut self.jokers {
            let effect = joker.on_card_scored(&mut context, card);
            self.apply_effect(effect);
        }
    }
}
```

### State Persistence
- Jokers serialize/deserialize through `JokerState`
- Version migration support for save compatibility
- Custom state validation per joker type

## Performance Characteristics

### Memory Usage
- Base joker: ~24 bytes (optimal struct packing)
- With state: ~1KB per joker
- Condition cache: ~100 bytes per cached result

### Processing Performance
- Static jokers: ~100ns per evaluation
- Conditional jokers: ~500ns per evaluation
- Advanced jokers: ~1-5μs per evaluation
- Cache hit rate: 70-90% for temporal conditions

### Thread Safety
- All jokers are Send + Sync
- Registry uses RwLock for concurrent access
- State manager provides thread-safe state access
- No global mutable state

## Design Patterns

1. **Builder Pattern**: JokerEffect, EnhancedJoker construction
2. **Adapter Pattern**: Legacy compatibility wrappers
3. **Factory Pattern**: Centralized joker creation
4. **Strategy Pattern**: Pluggable condition evaluation
5. **Observer Pattern**: Lifecycle event hooks
6. **Composite Pattern**: Complex condition composition

## Testing Infrastructure

See `tests/CLAUDE.md` for comprehensive testing documentation.

### Test Coverage
- Identity tests for all joker metadata
- Behavior tests for effect calculation
- Integration tests for game flow
- Performance benchmarks for hot paths
- Thread safety verification

## Future Extension Points

1. **Dynamic Joker Loading**: Plugin system for custom jokers
2. **Visual Effects**: Animation and UI integration hooks
3. **Balancing Framework**: Automatic cost/effect tuning
4. **ML Integration**: Joker value estimation for AI
5. **Mod Support**: User-created joker definitions

## Subdirectories

- **`tests/`**: Comprehensive testing infrastructure (see tests/CLAUDE.md)
