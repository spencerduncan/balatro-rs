# CLAUDE.md - Consumables System

## Directory Purpose

The consumables module implements single-use cards that provide various effects in Balatro. This comprehensive system is designed with production-ready quality, extensive error handling, and trait-based architecture.

## Consumable Types

### Tarot Cards (22 cards)
Major Arcana cards (0-XXI) from The Fool to The World
- **Effects**: Card enhancement, money gain, suit/rank conversion, joker creation
- **Cost**: Typically 3-4 in shop
- **Implementation**: Complete with all 22 cards

### Planet Cards (12 cards)
Named after celestial bodies (Mercury, Venus, Earth, etc.)
- **Effects**: Permanently upgrade specific poker hands
- **Mapping**: Each planet corresponds to a specific hand type
- **Status**: Currently disabled (`planet.rs.disabled`) pending hand level system

### Spectral Cards (18 cards)
High-risk/high-reward cards with powerful effects
- **Categories**:
  - Destructive: Familiar, Grim, Incantation, Immolate
  - Enhancement: Talisman, Aura
  - Seal Application: Deja Vu, Trance, Medium
  - Joker Manipulation: Ankh, Hex, Wraith, The Soul
  - Transformation: Sigil, Ouija, Ectoplasm
  - System-wide: Black Hole
- **Pools**: Regular (16 cards), Special (Soul, Black Hole), All

## Key Components

### Core Infrastructure (`mod.rs`)
- **`Consumable` Trait**: Interface for all consumables
- **`ConsumableId` Enum**: Identifies all ~50+ consumable cards
- **`ConsumableType` Enum**: Categorizes cards (Tarot/Planet/Spectral)
- **`ConsumableEffect` Enum**: Effect categories
- **`ConsumableSlots`**: Manages inventory with capacity limits

### Targeting System
```rust
pub struct CardTarget {
    pub indices: Vec<usize>,
    pub collection: CardCollection,
}

pub struct JokerTarget {
    pub slot: usize,
    pub joker_type: Option<JokerId>,
}
```

### Effect Tracking
```rust
pub struct TarotEffect {
    pub enhanced_cards: Vec<CardEnhancement>,
    pub created_consumables: Vec<ConsumableId>,
    pub money_change: i32,
    pub cards_added: Vec<Card>,
    pub cards_removed: Vec<usize>,
    pub jokers_created: Vec<JokerId>,
    pub description: String,
}
```

## Architecture Patterns

### Trait-Based Design
```rust
pub trait Consumable: Send + Sync + fmt::Debug {
    fn consumable_type(&self) -> ConsumableType;
    fn can_use(&self, game_state: &Game, target: &Target) -> bool;
    fn use_effect(&self, game_state: &mut Game, target: Target) -> Result<(), ConsumableError>;
    fn get_description(&self) -> String;
    fn get_target_type(&self) -> TargetType;
    fn get_effect_category(&self) -> ConsumableEffect;
}
```

### Factory Pattern
- `TarotFactory` for creating tarot cards by ID
- `create_spectral_card()` for spectral cards
- Global factory instance with `get_tarot_factory()`

### Validation-First Approach
- All targets validated before effects
- Comprehensive error types (`ConsumableError`, `TargetValidationError`)
- Production-ready error messages with debugging context

## Game System Integration

### State Access
- `game.available`: Hand cards
- `game.deck`: Deck cards
- `game.discarded`: Discard pile
- `game.jokers`: Joker slots
- `game.money`: Economy
- `game.rng`: Randomization
- `game.hand_type_counts`: Hand levels

### Move Generation
- `Target::get_available_targets()` generates valid targets
- Integration with action system for consumable usage
- Validation ensures only legal moves generated

## Implementation Status

### Complete
- Tarot card system (all 22 cards)
- Spectral card framework (18 cards)
- Targeting and validation system
- Factory pattern implementation
- Error handling infrastructure

### Pending
- Planet cards (awaiting hand level system)
- Full seal system implementation
- Hand size modification support
- Joker edition support

## Performance Characteristics

Based on implementation:
- **Effect Application**: <1ms target
- **Target Generation**: Efficient combinatorial algorithms
- **Memory Usage**: Minimal allocations
- **Thread Safety**: Send + Sync throughout

## Testing Strategy

Comprehensive test coverage:
- Unit tests for each consumable
- Validation tests for targeting
- Integration tests for state modifications
- Factory pattern tests
- Serialization tests
- Edge case and boundary tests
- Production failure mode testing

## Design Decisions

1. **Trait-Based Architecture**: Easy extension with new consumable types
2. **Comprehensive Validation**: Every operation validated before execution
3. **Production Quality**: Error handling, thread safety, performance targets
4. **Separation of Concerns**: Clear separation between targeting, validation, effects
5. **Factory Pattern**: Centralized creation for consistency
6. **Effect Categorization**: Helps with UI/UX and game balance
