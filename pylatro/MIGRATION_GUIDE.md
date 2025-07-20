# Python API Migration Guide

This guide helps you migrate from the deprecated GameState action methods to the new GameEngine-only API and covers the f64 numeric type migration.

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

## f64 Numeric Type Migration

### Overview

In addition to the API changes above, all numeric game values have been migrated from mixed types (usize, i32) to f64 for Lua compatibility. This change is largely transparent to Python users since Python numbers are already floating point.

### What Changed

#### Rust Side Changes
- `Game.{chips, mult, score, money, round}`: `usize` → `f64`
- `JokerEffect` numeric fields: `i32/f32` → `f64`
- All game calculations now use f64 precision

#### Python Side Impact
```python
# Python users benefit from seamless integration
engine = pylatro.GameEngine()
state = engine.state

# All numeric values are now consistently f64 (float in Python)
score = state.score     # Always float, now with full precision
chips = state.chips     # Now f64, preserves fractional values
money = state.money     # Now f64, no more truncation
mult = state.mult       # Now f64, supports fractional multipliers

# Fractional values now work correctly
engine.chips = 1000.5   # Previously truncated, now preserved
engine.mult = 2.25      # Fractional multipliers now supported
```

### Benefits for Python Users

#### Preserved Precision
```python
# Before: Fractional values were truncated
engine.money = 100.75
assert state.money == 100  # Truncated to integer

# After: Full precision preserved
engine.money = 100.75
assert state.money == 100.75  # Exact value preserved
```

#### Larger Number Support
```python
# Support for very large scores beyond 32-bit integers
engine.score = 1e15  # 1 quadrillion
assert state.score == 1e15  # Works correctly

# Scientific notation support
very_large_score = 9.223372036854776e18
engine.score = very_large_score
assert state.score == very_large_score
```

#### Consistent Floating Point Operations
```python
# Arithmetic operations are now consistent
state = engine.state
chips = state.chips  # f64
mult = state.mult    # f64

# Direct floating point arithmetic
expected_score = chips * mult  # No type conversion needed
actual_score = state.score

# Floating point comparison (use appropriate epsilon)
assert abs(actual_score - expected_score) < 1e-10
```

### Migration Steps for Python Code

#### Step 1: Remove Integer Casting (if any)
```python
# OLD - unnecessary integer casting
chips = int(state.chips)
money = int(state.money)

# NEW - use values directly as floats
chips = state.chips  # Already float
money = state.money  # Already float
```

#### Step 2: Update Equality Comparisons
```python
# OLD - exact equality may fail due to floating point precision
assert state.score == 1000.0

# NEW - use appropriate epsilon for floating point comparison
import math
assert math.isclose(state.score, 1000.0, rel_tol=1e-10)

# Or use pytest.approx for testing
import pytest
assert state.score == pytest.approx(1000.0)
```

#### Step 3: Handle Special Values
```python
import math

# Check for special floating point values
if math.isnan(state.score):
    print("Score is NaN")
elif math.isinf(state.score):
    print("Score is infinite")
else:
    print(f"Score: {state.score}")
```

### Common Patterns

#### Display Formatting
```python
# Format for display (integer values without decimals)
def format_score(score):
    if score == int(score):
        return f"{int(score):,}"  # "1,000"
    else:
        return f"{score:,.1f}"    # "1,000.5"

# Format money with consistent decimals
def format_money(amount):
    return f"${amount:.2f}"  # "$100.50"

# Format multipliers
def format_mult(mult):
    if mult == int(mult):
        return f"x{int(mult)}"    # "x5"
    else:
        return f"x{mult:.2f}"     # "x2.25"
```

#### Numerical Stability
```python
import math

def safe_divide(a, b):
    """Safe division with floating point checks"""
    if abs(b) < 1e-10:  # Avoid division by near-zero
        return 0.0
    result = a / b
    if math.isinf(result) or math.isnan(result):
        return 0.0  # Or handle appropriately
    return result
```

#### Range Validation
```python
def validate_score(score):
    """Validate score is reasonable"""
    if math.isnan(score) or math.isinf(score):
        return False
    if score < 0 or score > 1e15:  # Reasonable score range
        return False
    return True
```

### Testing f64 Migration

#### Test Precision Preservation
```python
def test_fractional_values():
    engine = pylatro.GameEngine()
    
    # Test fractional values are preserved
    engine.chips = 1000.25
    engine.mult = 2.5
    engine.money = 100.75
    
    state = engine.state
    assert state.chips == 1000.25
    assert state.mult == 2.5
    assert state.money == 100.75

def test_large_numbers():
    engine = pylatro.GameEngine()
    
    # Test large number support
    large_score = 5e12
    engine.score = large_score
    
    state = engine.state
    assert state.score == large_score
```

#### Test Arithmetic Precision
```python
import math

def test_arithmetic_precision():
    engine = pylatro.GameEngine()
    
    # Set up known values
    engine.chips = 1000.0
    engine.mult = 2.5
    
    # Calculate expected score
    expected = 1000.0 * 2.5  # 2500.0
    
    # Compare with small epsilon
    state = engine.state
    calculated = state.chips * state.mult
    assert math.isclose(calculated, expected, rel_tol=1e-10)
```

### Troubleshooting

#### Issue: Floating point comparison failures
```python
# Problem
assert state.score == 1000.0  # May fail due to precision

# Solution
import math
assert math.isclose(state.score, 1000.0, rel_tol=1e-10)
```

#### Issue: Unexpected decimal places in display
```python
# Problem
print(f"Score: {state.score}")  # "Score: 1000.0"

# Solution
def display_score(score):
    if score == int(score):
        return f"{int(score)}"  # "1000"
    else:
        return f"{score:.1f}"   # "1000.5"
```

### Backward Compatibility

The f64 migration maintains full backward compatibility:
- All existing Python code continues to work
- No changes required for basic usage
- Performance impact is minimal
- Type conversions are handled automatically

### Summary

The f64 migration benefits Python users by:
- ✅ **Preserving Precision**: Fractional values no longer truncated
- ✅ **Supporting Large Numbers**: Beyond 32-bit integer limits
- ✅ **Consistent Types**: All game values are now float
- ✅ **Seamless Integration**: Python's native float compatibility
- ✅ **No Breaking Changes**: Existing code continues to work

For most Python users, this migration is transparent and provides improved precision and range for game calculations.