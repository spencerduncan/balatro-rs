# Python API Tutorial

This tutorial demonstrates how to use the Balatro-RS Python bindings with the new unified API design following the dual framework elimination.

## Quick Start

### Installation

```bash
cd pylatro
./setup.sh                    # Setup development environment
source .env/bin/activate       # Activate virtual environment
maturin develop               # Build and install
```

### Basic Game Loop

```python
import pylatro

# Create game engine
engine = pylatro.GameEngine()

# Simple game loop
while not engine.is_over:
    # Get available actions
    actions = engine.gen_actions()
    
    if actions:
        # Choose first action (or implement your strategy)
        action = actions[0]
        engine.handle_action(action)
        
        # Access game state for information
        state = engine.state
        print(f"Score: {state.score}, Money: {state.money}")

# Check final result
if engine.is_win:
    print(f"Victory! Final score: {engine.state.score}")
else:
    print(f"Game over. Final score: {engine.state.score}")
```

## Core Concepts

### GameEngine vs GameState

The API follows a clear separation:

- **GameEngine**: Controls the game (actions, progression)
- **GameState**: Provides read-only access to game data

```python
engine = pylatro.GameEngine()

# Actions and control via GameEngine
actions = engine.gen_actions()        # ✅ Get possible actions
engine.handle_action(actions[0])      # ✅ Execute action
is_over = engine.is_over             # ✅ Check game status

# Data access via GameState
state = engine.state                 # ✅ Get current state
score = state.score                  # ✅ Read score
money = state.money                  # ✅ Read money
cards = state.available              # ✅ Read available cards
```

### Action Types

The engine provides two ways to work with actions:

```python
# Method 1: Direct action objects (recommended for complex logic)
actions = engine.gen_actions()
for action in actions:
    action_name = engine.get_action_name(action)
    print(f"Available: {action_name}")

# Execute specific action
if actions:
    engine.handle_action(actions[0])

# Method 2: Action space (recommended for ML/RL)
action_space = engine.gen_action_space()  # Returns [0, 1, 0, 1, ...]
valid_indices = [i for i, valid in enumerate(action_space) if valid]

# Execute by index
if valid_indices:
    index = valid_indices[0]
    engine.handle_action_index(index)
```

## Game Configuration

### Basic Configuration

```python
# Create custom configuration
config = pylatro.Config()
config.ante_end = 5          # Play until ante 5
config.starting_money = 20   # Start with more money

# Create engine with config
engine = pylatro.GameEngine(config)
```

### Advanced Configuration

```python
config = pylatro.Config()

# Game difficulty
config.ante_end = 8                    # Higher ante for longer games
config.starting_money = 4              # Standard starting money

# Performance settings
config.max_action_history = 1000       # Limit history for memory

engine = pylatro.GameEngine(config)
```

## Working with Game State

### Basic State Information

```python
state = engine.state

# Game progress
print(f"Ante: {state.ante}")
print(f"Round: {state.round}")
print(f"Stage: {state.stage}")

# Resources
print(f"Score: {state.score}")
print(f"Required: {state.required_score}")
print(f"Money: {state.money}")

# Turn information
print(f"Plays remaining: {state.plays}")
print(f"Discards remaining: {state.discards}")
```

### Card Information

```python
state = engine.state

# Available cards (in hand)
hand_cards = state.available
print(f"Hand size: {len(hand_cards)}")

for card in hand_cards:
    print(f"Card: {card}")

# Discarded cards
discarded = state.discarded
print(f"Discarded: {len(discarded)} cards")

# Deck information
deck_info = state.deck
print(f"Deck: {deck_info}")
```

### Joker Information

```python
state = engine.state

# New joker API (recommended)
joker_ids = state.joker_ids
print(f"Jokers: {len(joker_ids)}")

for joker_id in joker_ids:
    # Get detailed information from engine
    joker_info = engine.get_joker_info(joker_id)
    if joker_info:
        print(f"Joker: {joker_info.name}")
        print(f"Description: {joker_info.description}")
        print(f"Rarity: {joker_info.rarity}")
        
        # Get cost information
        cost = engine.get_joker_cost(joker_id)
        can_buy = engine.can_buy_joker(joker_id)
        print(f"Cost: ${cost}, Can buy: {can_buy}")

# Helper methods for quick access
joker_names = state.get_joker_names()
joker_descriptions = state.get_joker_descriptions()

for name, desc in zip(joker_names, joker_descriptions):
    print(f"{name}: {desc}")
```

## Strategy Implementation

### Random Strategy

```python
import random

def random_strategy(engine):
    """Simple random action selection"""
    while not engine.is_over:
        actions = engine.gen_actions()
        if actions:
            action = random.choice(actions)
            engine.handle_action(action)
    
    return engine.state.score

# Run simulation
config = pylatro.Config()
config.ante_end = 3
engine = pylatro.GameEngine(config)
final_score = random_strategy(engine)
print(f"Random strategy score: {final_score}")
```

### Greedy Strategy

```python
def greedy_money_strategy(engine):
    """Prioritize actions that preserve/gain money"""
    while not engine.is_over:
        actions = engine.gen_actions()
        if not actions:
            continue
            
        # Simple heuristic: prefer playing cards over discarding
        play_actions = []
        other_actions = []
        
        for action in actions:
            action_name = engine.get_action_name(action)
            if "play" in action_name.lower():
                play_actions.append(action)
            else:
                other_actions.append(action)
        
        # Prefer playing cards
        if play_actions:
            engine.handle_action(play_actions[0])
        elif other_actions:
            engine.handle_action(other_actions[0])
    
    return engine.state.score

# Test greedy strategy
engine = pylatro.GameEngine()
score = greedy_money_strategy(engine)
print(f"Greedy strategy score: {score}")
```

### State-Based Strategy

```python
def state_based_strategy(engine):
    """Make decisions based on current game state"""
    while not engine.is_over:
        state = engine.state
        actions = engine.gen_actions()
        
        if not actions:
            continue
        
        # Decision logic based on state
        if state.money < 5:
            # Low money: be conservative
            action = actions[0]  # Take first available
        elif state.plays > 1:
            # Multiple plays: be aggressive
            # Look for play actions
            for action in actions:
                name = engine.get_action_name(action)
                if "play" in name.lower():
                    break
            else:
                action = actions[0]
        else:
            # Default case
            action = actions[0]
        
        engine.handle_action(action)
        
        # Optional: logging
        if len(actions) > 1:
            print(f"Chose action from {len(actions)} options")
    
    return engine.state.score

# Run state-based strategy
engine = pylatro.GameEngine()
score = state_based_strategy(engine)
print(f"State-based strategy score: {score}")
```

## Reinforcement Learning Integration

### Action Space for ML

```python
import numpy as np

def get_observation(engine):
    """Extract features for ML model"""
    state = engine.state
    
    # Basic features
    features = {
        'score': state.score,
        'money': state.money,
        'ante': state.ante,
        'plays': state.plays,
        'discards': state.discards,
        'hand_size': len(state.available),
        'joker_count': len(state.joker_ids),
    }
    
    # Action space
    action_space = engine.gen_action_space()
    features['action_mask'] = np.array(action_space, dtype=np.float32)
    
    return features

def ml_training_loop():
    """Example ML training pattern"""
    engine = pylatro.GameEngine()
    episode_rewards = []
    
    while not engine.is_over:
        # Get current observation
        obs = get_observation(engine)
        
        # Get valid actions
        action_space = obs['action_mask']
        valid_indices = np.where(action_space == 1)[0]
        
        if len(valid_indices) == 0:
            continue
        
        # Your ML model would predict action here
        # For demo, use random valid action
        action_index = np.random.choice(valid_indices)
        
        # Execute action
        engine.handle_action_index(action_index)
        
        # Calculate reward (example)
        new_obs = get_observation(engine)
        reward = new_obs['score'] - obs['score']  # Score difference
        episode_rewards.append(reward)
    
    return sum(episode_rewards)

# Run training episode
total_reward = ml_training_loop()
print(f"Episode reward: {total_reward}")
```

### Batch Processing

```python
def batch_simulation(num_games=100):
    """Run multiple games for statistics"""
    scores = []
    
    for i in range(num_games):
        config = pylatro.Config()
        config.ante_end = 3  # Shorter games for faster testing
        
        engine = pylatro.GameEngine(config)
        
        # Random play
        while not engine.is_over:
            actions = engine.gen_actions()
            if actions:
                import random
                engine.handle_action(random.choice(actions))
        
        scores.append(engine.state.score)
        
        if (i + 1) % 10 == 0:
            print(f"Completed {i + 1}/{num_games} games")
    
    # Statistics
    import statistics
    print(f"Average score: {statistics.mean(scores):.2f}")
    print(f"Best score: {max(scores)}")
    print(f"Worst score: {min(scores)}")
    
    return scores

# Run batch simulation
scores = batch_simulation(50)
```

## Error Handling

### Common Patterns

```python
def robust_game_loop(engine):
    """Game loop with error handling"""
    try:
        while not engine.is_over:
            actions = engine.gen_actions()
            
            if not actions:
                print("No actions available, continuing...")
                continue
            
            # Validate action before execution
            action = actions[0]
            try:
                engine.handle_action(action)
            except Exception as e:
                print(f"Action failed: {e}")
                # Continue with next iteration
                continue
                
    except KeyboardInterrupt:
        print("Game interrupted by user")
    except Exception as e:
        print(f"Unexpected error: {e}")
        raise
    
    return engine.state.score

# Use robust loop
engine = pylatro.GameEngine()
final_score = robust_game_loop(engine)
print(f"Final score: {final_score}")
```

## Performance Tips

### Efficient State Access

```python
# GOOD: Access state once, use multiple times
state = engine.state
score = state.score
money = state.money
jokers = state.joker_ids

# BAD: Multiple state accesses
score = engine.state.score     # Creates new snapshot
money = engine.state.money     # Creates new snapshot
jokers = engine.state.joker_ids # Creates new snapshot
```

### Batch Operations

```python
# GOOD: Batch action generation
actions = engine.gen_actions()
action_names = [engine.get_action_name(a) for a in actions]

# BAD: Individual calls in loop
action_names = []
for action in engine.gen_actions():
    name = engine.get_action_name(action)  # Separate call each time
    action_names.append(name)
```

### Memory Management

```python
def memory_efficient_simulation():
    """Minimize memory usage during long simulations"""
    engine = pylatro.GameEngine()
    
    # Don't store unnecessary state snapshots
    step_count = 0
    
    while not engine.is_over:
        actions = engine.gen_actions()
        if actions:
            engine.handle_action(actions[0])
            step_count += 1
            
            # Only access state when needed
            if step_count % 100 == 0:
                state = engine.state
                print(f"Step {step_count}: Score {state.score}")
    
    return engine.state.score  # Final state access only

score = memory_efficient_simulation()
```

## Migration from Legacy API

If you're upgrading from the old dual framework:

```python
# OLD WAY (deprecated)
def old_pattern():
    engine = pylatro.GameEngine()
    state = engine.state
    
    # These show deprecation warnings:
    actions = state.gen_actions()    # ⚠️ Warning
    is_over = state.is_over         # ⚠️ Warning
    
    # These fail with errors:
    # state.handle_action(action)   # ❌ Error

# NEW WAY (current)
def new_pattern():
    engine = pylatro.GameEngine()
    
    # Clear separation:
    actions = engine.gen_actions()   # ✅ Actions from engine
    is_over = engine.is_over        # ✅ Status from engine
    
    state = engine.state            # ✅ State for reading
    score = state.score             # ✅ Data from state
```

For detailed migration guidance, see [DUAL_FRAMEWORK_ELIMINATION_MIGRATION_GUIDE.md](DUAL_FRAMEWORK_ELIMINATION_MIGRATION_GUIDE.md).

## Next Steps

- Explore the `examples/` directory for more complex patterns
- Read the [API Reference](JOKER_API_REFERENCE.md) for complete method documentation
- Check the [Architecture Guide](ARCHITECTURE.md) for implementation details
- See the [Performance Guide](JOKER_PERFORMANCE_GUIDE.md) for optimization techniques

This tutorial covers the essential patterns for using the Balatro-RS Python API effectively. The unified design makes it easier to write correct, performant code for both research and production applications.