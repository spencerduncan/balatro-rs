# Balatro-RS Architecture Documentation

## Overview

Balatro-RS is a high-performance game engine and move generator for a simplified version of Balatro, implemented in Rust with Python bindings. The architecture is designed for reinforcement learning applications with emphasis on performance, correctness, and clean API design.

## Core Architecture

### Engine Design Pattern

The engine follows a clear separation of concerns between game control and state inspection:

```
┌─────────────────┐    ┌─────────────────┐
│   GameEngine    │    │   GameState     │
│   (Actions)     │────│  (Read-only)    │
│                 │    │                 │
│ • gen_actions() │    │ • score         │
│ • handle_action │    │ • money         │
│ • is_over       │    │ • available     │
│ • state         │────│ • joker_ids     │
└─────────────────┘    └─────────────────┘
```

### Key Principles

1. **Single Responsibility**: Each component has one clear purpose
2. **Immutable Snapshots**: GameState provides immutable views of game state
3. **Action-State Separation**: Actions modify state; state inspection is read-only
4. **Performance First**: Zero-copy access where possible, optimized for RL training

## Rust Core Library

### Game Module (`core/src/game.rs`)

Central coordinator managing:
- Game state transitions
- Action validation and execution
- Stage progression (Pre-blind, Blind, Post-blind, Shop)
- Score calculation and win/lose conditions

### Action System (`core/src/action.rs`)

Comprehensive action enumeration covering:
- Card play/discard/selection
- Joker purchase/sale/arrangement
- Shop interactions
- Consumable usage
- Stage transitions

### Generator Pattern (`core/src/generator.rs`)

Move generation optimized for AI applications:
- `gen_actions()`: Returns action iterator for dynamic programming
- `gen_action_space()`: Returns binary vector for ML frameworks
- Lazy evaluation for performance
- Deterministic ordering for reproducibility

### Joker System

Multi-pattern joker implementation:
- **Static Framework**: High-performance conditional jokers
- **Dynamic Implementation**: Complex jokers with custom logic
- **Registry System**: Thread-safe joker metadata and factory
- **State Management**: Persistent joker state for scaling effects

## Python Bindings (PyO3)

### Dual Framework Elimination

**Previous Architecture (Eliminated)**:
```python
# DEPRECATED: Confusing dual interface
engine = pylatro.GameEngine()
state = engine.state

# Both had action methods (confusing!)
actions1 = engine.gen_actions()  # GameEngine
actions2 = state.gen_actions()   # GameState (deprecated)
```

**Current Architecture**:
```python
# CLEAR: Single responsibility principle
engine = pylatro.GameEngine()  # For actions and control
state = engine.state           # For read-only access

# Clean separation
actions = engine.gen_actions()   # ✅ Actions via engine
score = state.score             # ✅ State via state object
```

### Performance Optimizations

- **Zero-copy where possible**: Direct memory access to Rust data
- **Lazy evaluation**: State snapshots created only when needed
- **Cached method results**: Expensive operations cached
- **Minimal Python/Rust crossings**: Batch operations when possible

## Threading and Concurrency

### Thread Safety

- **GameEngine**: Not thread-safe (single-threaded per instance)
- **GameState**: Immutable snapshots are thread-safe for reading
- **Joker Registry**: Thread-safe with RwLock protection
- **Static Data**: All static strings and definitions are thread-safe

### Parallel Training

```python
# Safe pattern for multi-threaded RL training
import threading
import pylatro

def train_worker(worker_id):
    # Each thread gets its own engine
    engine = pylatro.GameEngine()
    
    while training:
        # Independent game simulation
        actions = engine.gen_actions()
        # ... training logic
```

## Memory Management

### Rust Side

- **Stack allocation**: Prefer stack over heap where possible
- **Arena allocation**: For temporary game state during scoring
- **Reference counting**: Shared immutable data (cards, jokers)
- **Copy-on-write**: For expensive state updates

### Python Side

- **Minimal copying**: PyO3 handles memory efficiently
- **Reference management**: Python GC handles PyO3 objects
- **State snapshots**: Lightweight views into Rust data
- **Batch operations**: Reduce crossing overhead

## Error Handling

### Rust Error Propagation

```rust
// Comprehensive error types
pub enum GameError {
    InvalidAction(String),
    InvalidState(String),
    SerializationError(serde_json::Error),
    // ...
}

// Result-based APIs
impl Game {
    pub fn handle_action(&mut self, action: Action) -> Result<(), GameError> {
        self.validate_action(&action)?;
        self.apply_action(action)?;
        Ok(())
    }
}
```

### Python Error Translation

```python
# Rust errors become Python exceptions
try:
    engine.handle_action(invalid_action)
except pylatro.InvalidActionError as e:
    print(f"Action failed: {e}")
```

## Performance Characteristics

### Benchmarks

- **Action generation**: ~10μs for complex game states
- **Action execution**: ~1-5μs per action
- **State snapshot**: ~100ns (cached), ~1μs (fresh)
- **Memory usage**: ~1KB per joker, ~10KB base game state

### Optimization Strategies

1. **Hot path optimization**: Critical loops hand-optimized
2. **SIMD where applicable**: Vectorized operations for bulk processing
3. **Cache-friendly data layout**: Structs organized for cache efficiency
4. **Lazy evaluation**: Expensive computations deferred until needed

## Save/Load System

### Game State Serialization

```rust
// Versioned save format
#[derive(Serialize, Deserialize)]
pub struct SaveGame {
    pub version: u32,
    pub game_state: Game,
    pub metadata: SaveMetadata,
}

// Backwards compatibility
impl SaveGame {
    pub fn load_v1(data: &[u8]) -> Result<Game, SaveError> {
        // Migration logic for old saves
    }
}
```

### Python Integration

```python
# Save/load through engine
engine = pylatro.GameEngine()
save_data = engine.save_state()

# Restore later
new_engine = pylatro.GameEngine.from_save(save_data)
```

## Extension Points

### Adding New Jokers

1. **Define in JokerId enum**: Add new variant
2. **Implement Joker trait**: Custom logic or static framework
3. **Register in factory**: Enable creation and discovery
4. **Add tests**: Comprehensive test coverage

### Adding New Actions

1. **Extend Action enum**: New action types
2. **Update generator**: Include in move generation
3. **Implement in Game**: Handle action execution
4. **Python bindings**: Expose new actions

### Adding New Consumables

1. **Define Consumable trait**: Effect interface
2. **Implement specific types**: Tarot, Planet, Spectral
3. **Integrate with shop**: Purchase and usage
4. **Target system**: For card selection effects

## Testing Strategy

### Rust Tests

- **Unit tests**: Individual component testing
- **Integration tests**: Full game flow testing
- **Property tests**: Invariant checking with quickcheck
- **Performance tests**: Regression detection

### Python Tests

- **API tests**: Python binding correctness
- **Compatibility tests**: Backwards compatibility verification
- **Performance tests**: Python/Rust boundary overhead
- **RL integration tests**: Training environment validation

## Development Workflow

### Git Worktree Strategy

```bash
# Never develop in main workspace
cd /home/spduncan/balatro-rs-ws/balatro-rs  # Main repository

# Create feature worktree
git worktree add /home/spduncan/balatro-rs-ws/feature-branch feature/new-feature

# Develop in worktree
cd /home/spduncan/balatro-rs-ws/feature-branch
cargo test --all
```

### CI/CD Pipeline

1. **Formatting**: rustfmt, Black (Python)
2. **Linting**: Clippy, mypy
3. **Testing**: Unit, integration, performance
4. **Security**: Dependency scanning, audit
5. **Documentation**: Doc generation and validation

## Future Architecture Considerations

### Planned Improvements

1. **WebAssembly support**: Browser deployment
2. **Async support**: Non-blocking operations
3. **Plugin system**: Dynamic joker/consumable loading
4. **Network play**: Multi-player support
5. **GPU acceleration**: Parallel game simulation

### Scalability

- **Distributed training**: Multiple game engines
- **Batch processing**: Vectorized operations
- **Memory pooling**: Reduced allocation overhead
- **Hot reloading**: Dynamic code updates

This architecture provides a solid foundation for high-performance Balatro simulation while maintaining clean interfaces and extensibility for future development.