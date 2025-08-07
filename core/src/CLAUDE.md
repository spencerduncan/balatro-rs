# CLAUDE.md - Core Game Engine

## Directory Purpose

The core/src directory contains the complete balatro-rs game engine, architected as a state machine-based system with clear separation of concerns across multiple subsystems. It provides a high-performance move generator optimized for reinforcement learning applications.

## Architecture Overview

### State Machine Design
The engine follows a state machine pattern with progression through well-defined stages:
```
PreBlind → Blind (Small/Big/Boss) → PostBlind → Shop → (repeat or End)
```
Each stage has specific valid actions, enforced through the generator system.

## Module Organization

### Core Systems

#### Game Logic (`game/`)
- Central orchestrator coordinating all subsystems
- State management and flow control
- Action handling and validation
- See `game/CLAUDE.md` for details

#### Action System (`action.rs`, `generator.rs`, `space.rs`)
- Comprehensive action enumeration for all operations
- Move generator providing valid actions per state
- 81-dimensional binary action space for RL frameworks

#### Card Systems (`card.rs`, `deck.rs`, `hand.rs`, `available.rs`)
- Immutable card representation with enhancements
- Deck management with shuffling and drawing
- O(n) hand evaluation using optimized analysis
- Available cards tracking with selection state

#### Joker Subsystem (`joker/`)
- Multi-pattern implementation (static, dynamic, scaling)
- Thread-safe registry with global registration
- State management for persistent effects
- Effect processor for score modification
- See `joker/CLAUDE.md` for details

### Economy Systems

#### Shop System (`shop/`)
- Weighted generation with rarity distributions
- Pack system with selection mechanics
- See `shop/CLAUDE.md` for details

#### Consumables (`consumables/`)
- Tarot, Planet, Spectral card implementations
- Effect processing and targeting system
- See `consumables/CLAUDE.md` for details

#### Vouchers (`vouchers/`)
- Permanent game modifiers
- Prerequisite system for upgrades
- See `vouchers/CLAUDE.md` for details

#### Skip Tags (`skip_tags/`)
- Strategic blind skipping rewards
- Economic and utility effects
- See `skip_tags/CLAUDE.md` for details

### Special Systems

#### Boss Blinds (`boss_blinds/`)
- Enhanced difficulty modifiers
- Special effects and challenges
- See `boss_blinds/CLAUDE.md` for details

## Key Data Structures

### Immutable Value Types
```rust
Card        // Copy-semantic card representation
Action      // Enumerated action space
HandRank    // Poker hand classifications
```

### State Management
```rust
Available              // Hand cards with selection
JokerStateMap         // Persistent joker state
MultiSelectContext    // Batch selection operations
BoundedActionHistory  // Fixed-size circular buffer
```

### Registry Systems
```rust
JokerRegistry    // Thread-safe joker definitions
TarotFactory     // Consumable card generation
SkipTagRegistry  // Skip tag effects
```

## Integration Patterns

### Event-Driven Effect Processing
Phased approach for joker effects:
1. Pre-scoring phase (setup)
2. Scoring phase (calculate)
3. Post-scoring phase (cleanup/triggers)

### Trait-Based Extensibility
Multiple trait hierarchies enable clean separation:
- Joker traits: Identity, Gameplay, Lifecycle, State
- Pack traits: PackGenerator, PackValidator
- Tag traits: SkipTag with various effects

### Factory Pattern Usage
- `StaticJokerFactory`: Compile-time joker registration
- `PackFactory`: Dynamic pack content generation
- `ConsumableFactory`: Card effect instantiation

## Performance Characteristics

### Memory Optimization
- Target: ~1KB per joker, ~10KB base game state
- Zero-copy patterns where possible
- Bounded collections for predictable memory

### Concurrency Model
- Game instances: Single-threaded, not Send+Sync
- Registries: Thread-safe with RwLock protection
- RNG: Arc<Mutex<>> wrapped for safe sharing
- Immutable snapshots: Safe read-only views

### Performance Metrics
- Action generation: ~10μs for complex states
- Hand evaluation: O(n) single-pass algorithm
- Action execution: 1-5μs per action
- State snapshot: ~100ns cached, ~1μs fresh

## Error Handling

Dual-error approach for security:
- `DeveloperError`: Detailed errors for debugging
- `UserError`: Sanitized generic errors for production
- `ErrorSanitizer`: Environment-based conversion

## RNG System

Multiple modes for different use cases:
- **Secure mode**: ChaCha20 CSPRNG for production
- **Deterministic mode**: Seeded for reproducible games
- **Testing mode**: Known sequences for unit tests
- **Audit logging**: All RNG operations tracked

## Serialization

- Versioned save format with forward compatibility
- Selective serialization skipping transient state
- PyO3 bindings for Python integration
- Automatic migration of old saves

## RL Training Optimization

- Vectorized action space for batch processing
- Stateless generators with no allocation
- Deterministic RNG for reproducible training
- Minimal copying with direct mutation

## Subdirectories

- `boss_blinds/`: Boss blind challenge system
- `consumables/`: Single-use card effects
- `game/`: Core game orchestration
- `joker/`: Joker implementation and management
- `shop/`: Economic and shop systems
- `skip_tags/`: Skip tag reward system
- `vouchers/`: Permanent upgrade system

Each subdirectory has its own CLAUDE.md with detailed documentation.
