# Dual Framework Elimination Migration Guide

## Overview

This guide covers the architectural changes made to eliminate the "dual framework" design where both `GameState` and `GameEngine` provided overlapping action methods. The new design provides clear separation of concerns:

- **GameEngine**: Performs actions and controls game flow
- **GameState**: Provides read-only access to game state

## Background

### Previous Architecture (Dual Framework)

In the previous design, both `GameState` and `GameEngine` had action methods, leading to confusion and potential inconsistencies:

```python
# OLD - Both classes had action methods
engine = pylatro.GameEngine()
state = engine.state

# Could call actions on either object
actions1 = engine.gen_actions()    # Available on GameEngine
actions2 = state.gen_actions()     # Also available on GameState (confusing!)

# Could check game status on either
is_over1 = engine.is_over          # Available on GameEngine
is_over2 = state.is_over           # Also available on GameState (confusing!)
```

### Current Architecture (Single Framework)

The new architecture eliminates duplication and provides clear separation:

```python
# NEW - Clear separation of concerns
engine = pylatro.GameEngine()      # For actions and game control
state = engine.state               # For read-only state access

# Actions only on GameEngine
actions = engine.gen_actions()     # ✅ Clear and consistent
is_over = engine.is_over          # ✅ Clear and consistent

# State reading only on GameState
score = state.score               # ✅ Read-only state access
cards = state.available           # ✅ Read-only state access
jokers = state.joker_ids          # ✅ Read-only state access
```

## Benefits of the New Architecture

### 1. Clear Separation of Concerns

- **GameEngine**: Mutating operations (actions, game progression)
- **GameState**: Immutable operations (state inspection, data access)

### 2. Better Thread Safety

- `GameState` objects are immutable snapshots
- Concurrent read access to state is safe
- Mutations only happen through `GameEngine`

### 3. Improved API Clarity

- No confusion about which object to use for which operation
- Prevents accidental state mutations through wrong interface
- Better self-documenting code

### 4. Enhanced Memory Efficiency

- `GameState` objects can be optimized as lightweight snapshots
- Reduced memory overhead from duplicate method implementations
- Better cache locality for state access patterns

## Migration Steps

### Step 1: Update Function Signatures

Change function parameters from `GameState` to `GameEngine` where actions are needed:

```python
# BEFORE
def game_loop(game_state: pylatro.GameState):
    while not game_state.is_over:
        actions = game_state.gen_actions()
        if actions:
            game_state.handle_action(actions[0])

# AFTER
def game_loop(game_engine: pylatro.GameEngine):
    while not game_engine.is_over:
        actions = game_engine.gen_actions()
        if actions:
            game_engine.handle_action(actions[0])
```

### Step 2: Update Action Method Calls

Move all action-related calls to `GameEngine`:

```python
# BEFORE - Actions on GameState (deprecated)
def analyze_game(state: pylatro.GameState):
    actions = state.gen_actions()           # DEPRECATED
    action_count = len(actions)
    is_finished = state.is_over             # DEPRECATED
    
# AFTER - Actions on GameEngine
def analyze_game(engine: pylatro.GameEngine):
    actions = engine.gen_actions()          # ✅ Correct
    action_count = len(actions)
    is_finished = engine.is_over            # ✅ Correct
```

### Step 3: Use GameState for Read-Only Access

Access state properties through `engine.state`:

```python
# BEFORE - Mixed access patterns
def get_game_info(state: pylatro.GameState):
    return {
        'score': state.score,
        'money': state.money,
        'actions': state.gen_actions(),     # DEPRECATED
    }

# AFTER - Clear access patterns
def get_game_info(engine: pylatro.GameEngine):
    state = engine.state
    return {
        'score': state.score,              # ✅ Read from state
        'money': state.money,              # ✅ Read from state
        'actions': engine.gen_actions(),   # ✅ Actions from engine
    }
```

### Step 4: Update Class Hierarchies

If you have classes that store game references, update them:

```python
# BEFORE
class GameAnalyzer:
    def __init__(self, game_state: pylatro.GameState):
        self.game = game_state
    
    def analyze(self):
        actions = self.game.gen_actions()  # DEPRECATED
        return len(actions)

# AFTER
class GameAnalyzer:
    def __init__(self, game_engine: pylatro.GameEngine):
        self.engine = game_engine
    
    def analyze(self):
        actions = self.engine.gen_actions()  # ✅ Correct
        return len(actions)
```

## Common Migration Patterns

### Pattern 1: Game Loop Functions

```python
# OLD PATTERN
def run_simulation(state: pylatro.GameState) -> int:
    while not state.is_over:
        actions = state.gen_actions()
        if actions:
            state.handle_action(random.choice(actions))
    return state.score

# NEW PATTERN
def run_simulation(engine: pylatro.GameEngine) -> int:
    while not engine.is_over:
        actions = engine.gen_actions()
        if actions:
            engine.handle_action(random.choice(actions))
    return engine.state.score
```

### Pattern 2: State Analysis Functions

```python
# OLD PATTERN - Function needs both state and actions
def analyze_position(state: pylatro.GameState):
    score = state.score                    # Read state
    actions = state.gen_actions()          # Get actions (deprecated)
    return {'score': score, 'actions': len(actions)}

# NEW PATTERN - Use engine for complete access
def analyze_position(engine: pylatro.GameEngine):
    state = engine.state
    score = state.score                    # Read state
    actions = engine.gen_actions()         # Get actions
    return {'score': score, 'actions': len(actions)}
```

### Pattern 3: Separation of Concerns

```python
# OLD PATTERN - Mixed responsibilities
def process_game(state: pylatro.GameState):
    # Analysis (read-only)
    current_score = state.score
    
    # Actions (mutating) - deprecated usage
    actions = state.gen_actions()
    state.handle_action(actions[0])

# NEW PATTERN - Clear separation
def analyze_game(state: pylatro.GameState) -> dict:
    """Pure function for game analysis"""
    return {
        'score': state.score,
        'money': state.money,
        'cards': len(state.available)
    }

def execute_action(engine: pylatro.GameEngine):
    """Function for game mutations"""
    actions = engine.gen_actions()
    if actions:
        engine.handle_action(actions[0])

def process_game(engine: pylatro.GameEngine):
    """Orchestrator function"""
    analysis = analyze_game(engine.state)
    print(f"Current state: {analysis}")
    execute_action(engine)
```

## Backwards Compatibility

### Deprecation Warnings

The old API still works but shows deprecation warnings:

```python
# These methods work but show warnings:
engine = pylatro.GameEngine()
state = engine.state

actions = state.gen_actions()        # ⚠️ DEPRECATED: Shows warning
is_over = state.is_over             # ⚠️ DEPRECATED: Shows warning
```

### Error Messages

Some methods that were moved show clear error messages:

```python
# These methods fail with helpful error messages:
state.handle_action(action)         # ❌ ERROR: Use engine.handle_action()
state.handle_action_index(0)        # ❌ ERROR: Use engine.handle_action_index()
```

### Suppressing Warnings During Migration

```python
import warnings

# Temporarily suppress during transition
with warnings.catch_warnings():
    warnings.filterwarnings("ignore", category=DeprecationWarning)
    actions = state.gen_actions()  # No warning shown
```

## Validation and Testing

### Verify Migration Completeness

```python
def validate_migration(engine: pylatro.GameEngine):
    """Test that migration is complete - should run without warnings"""
    import warnings
    
    with warnings.catch_warnings(record=True) as w:
        warnings.simplefilter("always")
        
        # Test all operations
        actions = engine.gen_actions()
        is_over = engine.is_over
        state = engine.state
        score = state.score
        
        # Should have no deprecation warnings
        deprecation_warnings = [warning for warning in w 
                              if issubclass(warning.category, DeprecationWarning)]
        
        if deprecation_warnings:
            print(f"Migration incomplete: {len(deprecation_warnings)} warnings")
            for warning in deprecation_warnings:
                print(f"  - {warning.message}")
        else:
            print("Migration complete: No deprecation warnings")
```

### Test Equivalence

```python
def test_api_equivalence(engine: pylatro.GameEngine):
    """Ensure new API returns same data as old API"""
    state = engine.state
    
    # Suppress warnings for comparison
    with warnings.catch_warnings():
        warnings.simplefilter("ignore")
        
        # Compare action generation
        old_actions = state.gen_actions()
        new_actions = engine.gen_actions()
        assert old_actions == new_actions
        
        # Compare game status
        old_is_over = state.is_over
        new_is_over = engine.is_over
        assert old_is_over == new_is_over
```

## Performance Considerations

### Memory Usage

```python
# EFFICIENT: Access state only when needed
def efficient_pattern(engine: pylatro.GameEngine):
    # Get actions from engine
    actions = engine.gen_actions()
    
    # Only access state when needed for reading
    if len(actions) > 0:
        state = engine.state
        print(f"Score: {state.score}")

# INEFFICIENT: Unnecessary state access
def inefficient_pattern(engine: pylatro.GameEngine):
    state = engine.state  # Unnecessary early access
    actions = engine.gen_actions()
    print(f"Score: {state.score}")
```

### Action Processing

```python
# EFFICIENT: Batch action processing
def process_actions_efficiently(engine: pylatro.GameEngine):
    actions = engine.gen_actions()  # Single call
    
    # Process multiple actions
    for action in actions[:3]:
        engine.handle_action(action)

# INEFFICIENT: Repeated action generation
def process_actions_inefficiently(engine: pylatro.GameEngine):
    for _ in range(3):
        actions = engine.gen_actions()  # Repeated calls
        if actions:
            engine.handle_action(actions[0])
```

## Troubleshooting

### Common Issues

#### Issue 1: "AttributeError: 'GameState' object has no attribute 'handle_action'"

**Cause**: Trying to call action methods on GameState
**Solution**: Use GameEngine for actions

```python
# PROBLEM
state = engine.state
state.handle_action(action)  # ❌ Error

# SOLUTION
engine.handle_action(action)  # ✅ Correct
```

#### Issue 2: Function expects GameState but gets deprecation warnings

**Cause**: Function uses deprecated GameState action methods
**Solution**: Update function to accept GameEngine

```python
# PROBLEM
def my_function(state: pylatro.GameState):
    actions = state.gen_actions()  # ⚠️ Deprecated

# SOLUTION
def my_function(engine: pylatro.GameEngine):
    actions = engine.gen_actions()  # ✅ Correct
```

#### Issue 3: Type annotation conflicts

**Cause**: Code expects GameState but needs actions
**Solution**: Update type annotations and usage

```python
# PROBLEM
def analyze(game: pylatro.GameState) -> dict:
    return {
        'actions': game.gen_actions(),  # ⚠️ Deprecated
        'score': game.score
    }

# SOLUTION
def analyze(engine: pylatro.GameEngine) -> dict:
    state = engine.state
    return {
        'actions': engine.gen_actions(),  # ✅ Correct
        'score': state.score
    }
```

## Timeline and Migration Support

### Current Status
- **Backwards compatibility**: Active (old API works with warnings)
- **New API**: Fully available and recommended
- **Migration window**: 6-12 months

### Migration Phases

1. **Phase 1 (Current)**: Dual support with deprecation warnings
2. **Phase 2 (Next major)**: Old API deprecated but functional
3. **Phase 3 (Future major)**: Old API removed

### Getting Help

- **GitHub Issues**: Report migration problems
- **Examples**: See `pylatro/examples/simulation.py` for updated patterns
- **Tests**: See `test_backwards_compatibility.py` for migration verification

## Summary

The dual framework elimination provides:

1. **Clearer API**: Distinct purposes for GameEngine and GameState
2. **Better performance**: Reduced overhead and better optimization opportunities
3. **Improved safety**: Immutable state access and controlled mutations
4. **Enhanced maintainability**: Clearer code organization and fewer bugs

Follow this guide to migrate your code systematically and take advantage of the improved architecture while maintaining backwards compatibility during the transition period.