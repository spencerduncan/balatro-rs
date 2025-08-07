# CLAUDE.md - Game Engine Core

## Directory Purpose

The `game` module serves as the **central orchestration engine** for Balatro, managing all game state, coordinating subsystems, and enforcing game rules. It implements the core game loop, action processing, and state transitions.

## Main Components

### Core Game Struct (`mod.rs`)
The monolithic `Game` struct (60+ fields) containing:
- **Core State**: deck, available cards, discarded pile, blind progression, ante tracking
- **Resource Management**: money, chips, mult, score, plays, discards
- **Systems Integration**: jokers, shop, vouchers, consumables, boss blinds
- **Meta Tracking**: action history, hand type counts, hand levels, enhancement counts
- **Infrastructure**: RNG, debug manager, memory monitor, persistence manager

### Active Modules

#### `persistence.rs` - Save/Load System
- `SaveableGameState` struct for JSON serialization
- `PersistenceManager` for save/load operations
- Version validation and joker recreation on load
- Excludes non-serializable fields (RNG, debug)

#### `packs.rs` - Booster Pack Management
- `PackManager` handles pack inventory
- Pack opening, selection, and skipping logic
- Integration with shop system for pack purchases

#### `debug.rs` - Development & Monitoring
- `DebugManager` for debug logging and memory monitoring
- Memory usage tracking for RL training scenarios
- Performance monitoring and reporting
- Configurable memory limits for different modes

### Placeholder Modules (TODOs)
- `cards.rs`, `flow.rs`, `jokers.rs`, `scoring.rs`, `shop.rs`
- Currently empty, planned for future refactoring

## State Machine Architecture

### Stage Progression
```
PreBlind → Blind → PostBlind → Shop → PreBlind (loop)
                         ↓
                        End (game over)
```

### Stage-Specific Actions

#### PreBlind
- `SelectBlind`: Choose Small/Big/Boss blind
- `SkipBlind`: Skip for reward and potential skip tag

#### Blind
- `SelectCard`: Add cards to selection (max limit enforced)
- `Play`: Execute selected cards as hand
- `Discard`: Discard selected cards
- `MoveCard`: Reorder cards left/right

#### PostBlind
- `CashOut`: Collect rewards after blind

#### Shop
- `BuyJoker`: Purchase with slot selection
- `BuyVoucher`: One-time purchase effects
- `BuyPack`: Purchase booster packs
- `RerollShop`: Refresh shop items (cost increases)
- `NextRound`: Advance to next blind

## Action System

### Central Dispatcher
```rust
pub fn handle_action(&mut self, action: Action) -> Result<(), GameError>
```
- Records all actions in bounded history
- Pattern matches action types to handlers
- Validates stage compatibility
- Returns specific error types

### Action Validation
- Stage compatibility checked before execution
- Invalid stage/action combinations return `GameError::InvalidStage`
- State consistency maintained through atomic updates
- Validation before state mutation

## Integration Patterns

### Move Generation (`generator.rs` extension)
- `gen_actions()`: Returns iterator of all valid actions
- `gen_action_space()`: Returns binary vector for RL frameworks
- Stage-aware action filtering ensures only legal moves
- Optimized for performance with minimal allocations

### Joker System
Complex multi-pattern integration:
- **Effect Processing**: `JokerEffectProcessor` handles accumulated effects
- **State Management**: `JokerStateManager` tracks persistent joker state
- **Scoring Integration**: `calc_score()` applies joker effects in correct order
- **Factory Pattern**: `JokerFactory` for joker creation/recreation

### Shop System
Delegated responsibility pattern:
- Shop maintains its own item inventory
- Game coordinates purchases and money transactions
- Reroll cost scaling managed by Game
- Pack purchases generate items for inventory

### Scoring Pipeline
Multi-stage scoring calculation:
1. Base hand rank scoring (chips + mult)
2. Card chip contributions
3. Joker effect accumulation
4. Multiplier application
5. Score validation against blind requirements

### Persistence Layer
Clean separation of concerns:
- `PersistenceManager` handles all save/load logic
- Versioned save format for backward compatibility
- Joker recreation through factory pattern
- Non-serializable fields excluded from saves

## Memory Management

### Bounded Resources
- Action history limited to prevent unbounded growth
- Debug message buffer with MAX_DEBUG_MESSAGES limit
- Memory monitoring for RL training scenarios

### Memory Configurations
- **Default**: Standard gameplay
- **RL Training**: Optimized for parallel training
- **Simulation**: Minimal memory footprint

## Error Handling

### Comprehensive Error Types
```rust
pub enum GameError {
    InvalidStage,
    InvalidAction,
    InsufficientFunds,
    SlotsFull,
    // ... more
}
```

### Error Philosophy
- Validation before state mutation
- Atomic operations to prevent partial updates
- Clear error messages for debugging

## Performance Optimizations

### Cached Values
- Enhancement counts (stone/steel cards)
- Iterator-based action generation (lazy evaluation)
- Minimal allocations in hot paths

### Hot Path Optimizations
- Action generation uses iterators
- State snapshots use copy-on-write
- Joker effects processed in batch

## Design Patterns

1. **State Machine**: Stage-based game flow control
2. **Command Pattern**: Action enum with handlers
3. **Factory Pattern**: Joker and pack creation
4. **Observer Pattern**: Debug logging and monitoring
5. **Strategy Pattern**: Different memory configurations
6. **Delegation**: Subsystem-specific logic in dedicated managers

## Extension Points

### Current Hooks
- Voucher system for game modifiers
- Boss blind state for special effects
- Skip tag system for blind skipping rewards
- Consumable integration (partially implemented)

### Future Extensions
- Hand level system for planet cards
- Enhanced boss blind effects
- Expanded voucher modifiers
- Complete consumable system

## Development Guidelines

### Adding New Actions
1. Extend `Action` enum
2. Update generator in `gen_actions_*`
3. Implement handler in `handle_action`
4. Add Python bindings if needed

### Modifying Game State
1. Update `Game` struct fields
2. Update `SaveableGameState` for persistence
3. Update initialization in `new()` and `default()`
4. Add migration logic if breaking change

### Performance Considerations
- Keep action generation lazy (iterators)
- Cache expensive calculations
- Minimize allocations in hot paths
- Profile with benchmarks

## Important Notes

The game engine serves as the **authoritative source of truth** for game state and the **central coordinator** for all gameplay systems, ensuring consistent rule enforcement and state management throughout the game lifecycle.
