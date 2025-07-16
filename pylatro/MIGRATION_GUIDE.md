# Python API Migration Guide

This guide helps you migrate from the deprecated GameState action methods to the new GameEngine-only API.

## Overview

Following the PyO3 bindings core types update (Issue #171), the Python API has been clarified to separate concerns:

- **GameEngine**: For performing actions and controlling the game
- **GameState**: For reading game state only (immutable snapshot)

## What Changed

### Before (Deprecated)
```python
# OLD API - DEPRECATED but still works with warnings
engine = pylatro.GameEngine()
state = engine.state  # Get GameState

# These methods were available on GameState but are now deprecated:
actions = state.gen_actions()           # DEPRECATED: Shows warning
space = state.gen_action_space()        # DEPRECATED: Shows warning  
name = state.get_action_name(0)         # DEPRECATED: Shows warning
is_over = state.is_over                 # DEPRECATED: Shows warning

# These methods fail on GameState:
state.handle_action(action)             # DEPRECATED: Fails with error
state.handle_action_index(0)            # DEPRECATED: Fails with error
```

### After (Current)
```python
# NEW API - Recommended approach
engine = pylatro.GameEngine()

# Use GameEngine for all actions:
actions = engine.gen_actions()          # ✅ Correct
space = engine.gen_action_space()       # ✅ Correct
name = engine.get_action_name(0)        # ✅ Correct
is_over = engine.is_over                # ✅ Correct
engine.handle_action(action)            # ✅ Correct
engine.handle_action_index(0)           # ✅ Correct

# Use GameState only for reading state:
state = engine.state                    # ✅ Correct
score = state.score                     # ✅ Correct
cards = state.available                 # ✅ Correct
jokers = state.jokers                   # ✅ Correct
```

## Migration Steps

### Step 1: Update Function Signatures
Change function parameters from `GameState` to `GameEngine`:

```python
# OLD
def action_loop(game: pylatro.GameState):
    pass

# NEW  
def action_loop(game: pylatro.GameEngine):
    pass
```

### Step 2: Update Action Calls
Move action method calls from GameState to GameEngine:

```python
# OLD
def action_loop(game: pylatro.GameState):
    if game.is_over:
        return
    actions = game.gen_actions()
    game.handle_action(actions[0])

# NEW
def action_loop(game: pylatro.GameEngine):
    if game.is_over:
        return  
    actions = game.gen_actions()
    game.handle_action(actions[0])
```

### Step 3: Access State When Needed
Use `engine.state` to access GameState properties:

```python
# When you need to read game state:
engine = pylatro.GameEngine()
state = engine.state

# Read state properties
score = state.score
cards = state.available
jokers = state.jokers
history = state.action_history
```

## Common Migration Patterns

### Pattern 1: Game Loop Function
```python
# OLD - function takes GameState
def run_game_loop(game_state: pylatro.GameState):
    while not game_state.is_over:
        actions = game_state.gen_actions()
        if actions:
            game_state.handle_action(actions[0])
    return game_state.score

# NEW - function takes GameEngine  
def run_game_loop(game_engine: pylatro.GameEngine):
    while not game_engine.is_over:
        actions = game_engine.gen_actions()
        if actions:
            game_engine.handle_action(actions[0])
    return game_engine.state.score
```

### Pattern 2: State Inspection with Actions
```python
# OLD - mixed state reading and actions on GameState
def analyze_and_act(game_state: pylatro.GameState):
    if game_state.money > 10:  # Read state
        actions = game_state.gen_actions()  # Get actions
        game_state.handle_action(actions[0])  # Perform action

# NEW - use GameEngine for actions, access state when needed
def analyze_and_act(game_engine: pylatro.GameEngine):
    if game_engine.state.money > 10:  # Read state via engine.state
        actions = game_engine.gen_actions()  # Get actions from engine
        game_engine.handle_action(actions[0])  # Perform action on engine
```

### Pattern 3: Passing State vs Engine
```python
# OLD - passing GameState around
def main():
    engine = pylatro.GameEngine()
    game_state = engine.state
    process_game(game_state)  # Pass state

def process_game(state: pylatro.GameState):
    # Work with state
    pass

# NEW - pass GameEngine for action capability, or GameState for read-only
def main():
    engine = pylatro.GameEngine()
    process_game(engine)  # Pass engine for full capability

def process_game(engine: pylatro.GameEngine):
    # Can perform actions AND read state
    state = engine.state  # Get state when needed for reading
    pass

# OR for read-only functions:
def analyze_game(state: pylatro.GameState):
    # Only read state, no actions
    pass

# Call with:
analyze_game(engine.state)
```

## Backwards Compatibility

The backwards compatibility layer ensures existing code continues to work:

### What Still Works (with warnings)
- `GameState.gen_actions()` - Shows deprecation warning but works
- `GameState.gen_action_space()` - Shows deprecation warning but works  
- `GameState.get_action_name()` - Shows deprecation warning but works
- `GameState.is_over` - Shows deprecation warning but works

### What Fails (with helpful errors)
- `GameState.handle_action()` - Fails with clear migration guidance
- `GameState.handle_action_index()` - Fails with clear migration guidance

### Deprecation Timeline
- **Version 1.x**: Backwards compatibility layer active (current)
- **Version 2.0**: Deprecated methods will be removed
- **Migration window**: Migrate code during 1.x versions

## Testing Your Migration

After migrating your code:

1. **Check for warnings**: Run your code and look for deprecation warnings
2. **Update function signatures**: Ensure all function parameters use `GameEngine` where actions are needed
3. **Test functionality**: Verify your game logic still works correctly
4. **Run tests**: Use the backwards compatibility test to verify migration

```python
# Test that your migration is complete
python test/test_backwards_compatibility.py
```

## Need Help?

- **Issues**: Report migration problems on [GitHub](https://github.com/spencerduncan/balatro-rs/issues)
- **Examples**: See `examples/simulation.py` for before/after migration
- **Tests**: See `test/test_backwards_compatibility.py` for API usage examples

## Summary

The key changes for migration:

1. **Use `GameEngine` for actions** - All game-modifying operations
2. **Use `GameState` for reading** - All state inspection (accessed via `engine.state`)
3. **Update function signatures** - Change parameters from `GameState` to `GameEngine` where actions are needed
4. **No breaking changes** - Old API still works but shows deprecation warnings

This separation provides a clearer, more maintainable API while ensuring existing code continues to work during the migration period.