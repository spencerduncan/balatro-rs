# CLAUDE.md - Python Bindings Package

## Directory Purpose

The pylatro package provides comprehensive Python bindings for the Balatro game engine, enabling integration with Python-based reinforcement learning frameworks and data science workflows. It exposes both game control APIs and extensive metadata/query capabilities.

## Architecture

### Core Components

#### GameEngine Class
Primary interface for game control:
- Mutable game instance management
- Action generation and execution
- Joker metadata and state queries
- Backwards compatibility layer

#### GameState Class
Read-only game state snapshot:
- Immutable view of game data
- Lazy evaluation for expensive operations
- Deprecated mutable methods for migration

#### LazyGameStateSnapshot
Performance-optimized state storage:
- Immediate access to scalar values (no allocation)
- Lazy computation of vector fields (on-demand cloning)
- Memory-efficient caching with `OnceLock`

## Key Features

### Comprehensive Joker API
- 30+ methods for joker queries, metadata, and state management
- Batch operations for efficient bulk retrieval
- Filter and search capabilities
- State analysis and accumulated value tracking

### Performance Optimizations
- Lazy evaluation reduces allocation overhead
- Batch operations avoid N+1 query patterns
- Zero-copy for immutable data where possible
- Efficient enum conversion for actions

### Security Hardening
- Input validation with length limits (256 char keys, 8KB values)
- Safe JSON-to-Python conversion with error handling
- Protected registry access with proper locking
- Thread-safe operation guarantees

## Dependencies

```toml
[dependencies]
pyo3 = {version = "0.24.1", features = ["auto-initialize"]}
balatro-rs = {path = "../core/"}
serde_json = "1.0"  # For custom data serialization

[lib]
crate-type = ["cdylib"]  # Python extension module
```

## Build System

### Maturin Setup
```bash
# Install maturin
pip install maturin

# Development build
maturin develop

# Release build
maturin build --release
```

### Virtual Environment
```bash
# Create venv
python -m venv .env
source .env/bin/activate

# Install development dependencies
pip install -r requirements-dev.txt
```

## Usage Examples

### Basic Game Loop
```python
import pylatro

# Initialize with optional config
engine = pylatro.GameEngine()
state = engine.state  # Read-only snapshot

while not engine.is_over:
    # Option 1: Dynamic action list
    actions = engine.gen_actions()
    if actions:
        engine.handle_action(actions[0])

    # Option 2: Static action space (for RL)
    space = engine.gen_action_space()
    valid_indices = [i for i, v in enumerate(space) if v == 1]
    if valid_indices:
        engine.handle_action_index(valid_indices[0])
```

### Joker Metadata Queries
```python
# Get comprehensive joker information
metadata = engine.get_joker_metadata(pylatro.JokerId.Joker)
all_jokers = engine.get_all_joker_metadata()

# Filter and search jokers
affordable = engine.filter_jokers(
    rarity=pylatro.JokerRarity.Common,
    affordable_only=True,
    unlocked_only=True
)

# Analyze joker state
state_info = engine.get_joker_state_info(joker_id)
accumulated = engine.get_joker_accumulated_value(joker_id)
```

### Performance Patterns
```python
# Batch operations (efficient)
metadata_dict = engine.get_multiple_joker_metadata([id1, id2, id3])

# Avoid N+1 queries
all_metadata = engine.get_all_joker_metadata()  # Single registry lock
```

## Core Library Integration

### Direct FFI Bindings
- Maps Rust types directly to Python via PyO3
- Zero-copy for immutable data where possible
- Efficient enum conversion for actions

### State Management
- `GameEngine` owns mutable `Game` instance
- `GameState` provides immutable snapshots
- Lazy evaluation reduces allocation overhead

### Thread Safety
- Each Python thread gets independent engine instances
- Registry access protected with proper locking
- No shared mutable state between threads

## Project Structure

```
pylatro/
├── Cargo.toml           # Rust package configuration
├── pyproject.toml       # Python package configuration
├── src/
│   └── lib.rs          # Main bindings implementation
├── examples/
│   └── simulation.py   # Example usage
├── gym/
│   └── balatro_env.py  # OpenAI Gym environment
└── test/
    └── main.py         # Python tests
```

## API Design

### Dual API Approach
- **Control API** (`GameEngine`): Mutable game operations
- **Observation API** (`GameState`): Immutable state queries
- Clear separation of concerns for safety

### Backwards Compatibility
- Deprecation warnings guide API migration
- Old methods redirect or error appropriately
- Clear migration path from mutable to immutable

### RL-Friendly Design
- Static action spaces for neural networks
- Efficient state snapshots for training
- Batch operations for parallel environments

## Testing

### Python Tests
```bash
cd pylatro
python test/main.py
```

### Rust Tests
```bash
cargo test -p pylatro
```

## Performance Characteristics

- **State Snapshot**: ~100ns for cached, ~1μs for fresh
- **Action Generation**: ~10μs for complex states
- **Batch Metadata**: O(n) with single lock acquisition
- **Memory Overhead**: Minimal with lazy evaluation

## Security Features

- Input length validation and sanitization
- Protected registry access with proper locking
- Safe JSON conversion with error handling
- No arbitrary code execution paths
