# pylatro

Python bindings for the Rust Balatro engine using [PyO3](https://pyo3.rs), designed for reinforcement learning and AI applications.

## Overview

Pylatro provides high-performance Python bindings to the Rust-based Balatro game engine. The library is optimized for:

- **Reinforcement Learning**: Fast move generation and game simulation
- **AI Research**: Comprehensive game state access and action spaces
- **Performance**: Zero-copy data access where possible
- **Clean API**: Clear separation between game control and state inspection

## Quick Start

```python
import pylatro

# Create game with configuration
config = pylatro.Config()
config.ante_end = 3  # Play until ante 3
engine = pylatro.GameEngine(config)

# Game loop
while not engine.is_over:
    # Get available actions
    actions = engine.gen_actions()
    if actions:
        # Execute random action
        import random
        action = random.choice(actions)
        engine.handle_action(action)
    
    # Access game state
    state = engine.state
    print(f"Score: {state.score}, Money: {state.money}")

print(f"Game ended. Final score: {engine.state.score}")
```

## Architecture

### Dual Framework Elimination

The library uses a clean separation of concerns:

- **`GameEngine`**: Handles all game actions and mutations
  - `gen_actions()` - Get available actions
  - `handle_action()` - Execute actions
  - `is_over` - Check game status
  - `state` - Access current game state

- **`GameState`**: Provides read-only access to game state
  - `score`, `money`, `ante` - Game metrics
  - `available`, `joker_ids` - Card and joker information
  - `action_history` - Game history

### Migration from Legacy API

If you're upgrading from the previous dual framework:

```python
# OLD (deprecated but still works with warnings)
state = engine.state
actions = state.gen_actions()  # ⚠️ Shows deprecation warning

# NEW (recommended)
actions = engine.gen_actions()  # ✅ Clear and consistent
```

See [MIGRATION_GUIDE.md](MIGRATION_GUIDE.md) for complete migration details.

## Reinforcement Learning Integration

### Gymnasium Environment

```python
from pylatro.gym import BalatroEnv

env = BalatroEnv()
observation = env.reset()

for step in range(1000):
    action = env.action_space.sample()  # Random action
    observation, reward, done, info = env.step(action)
    
    if done:
        observation = env.reset()
```

### Action Space API

```python
# Get binary action space for ML frameworks
space = engine.gen_action_space()  # Returns [0, 1, 0, 1, ...] 

# Execute action by index
valid_indices = [i for i, valid in enumerate(space) if valid]
action_index = random.choice(valid_indices)
engine.handle_action_index(action_index)
```

## Installation

### Prerequisites

- Python 3.8+
- Rust toolchain (for development)

### Setup Development Environment

```bash
cd pylatro
./setup.sh                    # Create virtual environment
source .env/bin/activate       # Activate environment
maturin develop               # Build and install in dev mode
python examples/simulation.py # Test installation
```

### Running Tests

```bash
python test/main.py                    # Basic functionality tests
python test/test_backwards_compatibility.py  # API compatibility tests
python test/test_performance.py       # Performance benchmarks
```

## Features

- **Fast move generation**: Optimized for RL training loops
- **Complete game state access**: All game information available to AI
- **Action space generation**: Compatible with ML frameworks
- **Memory efficient**: Minimal Python/Rust boundary crossings
- **Thread safe**: Supports concurrent game simulations
- **Backwards compatible**: Gradual migration from legacy API

## Examples

See the `examples/` directory for complete usage patterns:
- `simulation.py` - Basic game simulation
- Advanced RL integration patterns

## Documentation

- [MIGRATION_GUIDE.md](MIGRATION_GUIDE.md) - Upgrading from legacy API
- [JOKER_API_MIGRATION_GUIDE.md](JOKER_API_MIGRATION_GUIDE.md) - Joker system migration
- [../DUAL_FRAMEWORK_ELIMINATION_MIGRATION_GUIDE.md](../DUAL_FRAMEWORK_ELIMINATION_MIGRATION_GUIDE.md) - Architectural changes

