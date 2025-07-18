# Python API Migration Guide

## Overview

As part of the unified joker trait system migration (Epic #150), the Python bindings are transitioning from the legacy `Jokers` enum-based API to a new `JokerId`-based registry system. This guide helps you migrate your existing Python code to the new API.

## Deprecation Timeline

| Version | Status | Description |
|---------|--------|-------------|
| **Current** | Deprecation Warnings | Old API shows warnings but continues to work |
| **Next Major** | Legacy Support | Old API available with warnings (6+ months) |  
| **Future Major** | Removal | Old API removed (12+ months from now) |

**Current Status**: We are in the **Deprecation Warnings** phase. Your existing code will continue to work but will show deprecation warnings.

## API Changes Summary

### Deprecated APIs
- ✅ `GameState.jokers` → Use `GameState.joker_ids` + `GameEngine.get_joker_info()`

### New APIs
- ✅ `GameState.joker_ids` - List of joker identifiers
- ✅ `GameEngine.get_joker_info(joker_id)` - Get joker metadata
- ✅ `GameEngine.get_available_jokers(rarity=None)` - Registry access
- ✅ `GameEngine.can_buy_joker(joker_id)` - Purchase validation
- ✅ `GameEngine.get_joker_cost(joker_id)` - Pricing information

### Helper Methods (New)
- ✅ `GameState.get_joker_names()` - Easy name access
- ✅ `GameState.get_joker_descriptions()` - Easy description access

## Migration Examples

### Basic Joker Information Access

**❌ Old Way (Deprecated)**
```python
import pylatro

game = pylatro.GameEngine()
state = game.state

# This will show deprecation warning
for joker in state.jokers:
    print(f"Joker: {joker.name()}")
    print(f"Description: {joker.desc()}")
    print(f"Cost: {joker.cost()}")
```

**✅ New Way (Recommended)**
```python
import pylatro

game = pylatro.GameEngine()
state = game.state

# Modern approach using registry
for joker_id in state.joker_ids:
    joker_info = game.get_joker_info(joker_id)
    if joker_info:
        print(f"Joker: {joker_info.name}")
        print(f"Description: {joker_info.description}")
        cost = game.get_joker_cost(joker_id)
        print(f"Cost: {cost}")
```

**✅ Quick Migration (Helper Methods)**
```python
import pylatro

game = pylatro.GameEngine()
state = game.state

# Easy migration using helper methods
joker_names = state.get_joker_names()
joker_descriptions = state.get_joker_descriptions()

for name, desc in zip(joker_names, joker_descriptions):
    print(f"Joker: {name}")
    print(f"Description: {desc}")
```

### Joker Count and Slots

**✅ No Change Required**
```python
# These properties remain the same
joker_count = len(state.joker_ids)  # or state.joker_slots_used
total_slots = state.joker_slots_total
```

### Shopping and Economics

**❌ Old Way**
```python
# Old way required manual cost calculation
for joker in state.jokers:
    name = joker.name()
    cost = joker.cost()
    # Manual affordability check
    can_afford = game.state.money >= cost
```

**✅ New Way**
```python
# New way provides integrated shopping methods
for joker_id in state.joker_ids:
    joker_info = game.get_joker_info(joker_id)
    cost = game.get_joker_cost(joker_id)
    can_buy = game.can_buy_joker(joker_id)  # Checks money AND slots
    
    print(f"{joker_info.name}: ${cost} (Can buy: {can_buy})")
```

### Registry and Discovery

**✅ New Capabilities**
```python
# These capabilities are new in the registry system
all_jokers = game.get_available_jokers()
common_jokers = game.get_available_jokers(pylatro.JokerRarity.Common)
legendary_jokers = game.get_available_jokers(pylatro.JokerRarity.Legendary)

print(f"Total jokers in registry: {len(all_jokers)}")
print(f"Common jokers: {len(common_jokers)}")
print(f"Legendary jokers: {len(legendary_jokers)}")
```

## Common Migration Patterns

### Pattern 1: List Comprehensions

**❌ Old**
```python
joker_names = [j.name() for j in state.jokers]
joker_costs = [j.cost() for j in state.jokers]
```

**✅ New (Option 1: Helper Methods)**
```python
joker_names = state.get_joker_names()
# For costs, use registry approach:
joker_costs = [game.get_joker_cost(jid) for jid in state.joker_ids]
```

**✅ New (Option 2: Registry)**
```python
joker_names = []
joker_costs = []
for joker_id in state.joker_ids:
    info = game.get_joker_info(joker_id)
    cost = game.get_joker_cost(joker_id)
    if info:
        joker_names.append(info.name)
        joker_costs.append(cost)
```

### Pattern 2: Conditional Logic

**❌ Old**
```python
for joker in state.jokers:
    if joker.name() == "Greedy Joker":
        print(f"Found Greedy Joker: {joker.desc()}")
```

**✅ New**
```python
for joker_id in state.joker_ids:
    info = game.get_joker_info(joker_id)
    if info and info.name == "Greedy Joker":
        print(f"Found Greedy Joker: {info.description}")
```

### Pattern 3: Rarity Filtering

**❌ Old (Not easily possible)**
```python
# Old system made rarity filtering difficult
```

**✅ New**
```python
# New system makes rarity filtering easy
for joker_id in state.joker_ids:
    info = game.get_joker_info(joker_id)
    if info and info.rarity == pylatro.JokerRarity.Legendary:
        print(f"Legendary joker: {info.name}")
```

## Machine Learning / RL Integration

### Observation Space Changes

**❌ Old**
```python
def get_joker_features(state):
    # Old approach required manual enumeration
    features = []
    for joker in state.jokers:
        features.append(joker.name())  # String features
    return features
```

**✅ New**
```python
def get_joker_features(state):
    # New approach provides stable numeric IDs
    return state.joker_ids  # Enum-based features, more ML-friendly
```

### Enhanced Metadata

**✅ New Capabilities**
```python
def get_detailed_joker_features(game, state):
    features = []
    for joker_id in state.joker_ids:
        info = game.get_joker_info(joker_id)
        if info:
            features.append({
                'id': joker_id,
                'name': info.name,
                'rarity': info.rarity,
                'cost': game.get_joker_cost(joker_id),
                'can_buy': game.can_buy_joker(joker_id)
            })
    return features
```

## Handling Deprecation Warnings

### Suppress Warnings During Migration

```python
import warnings

# Temporarily suppress during transition period
with warnings.catch_warnings():
    warnings.filterwarnings("ignore", category=DeprecationWarning)
    jokers = state.jokers  # Won't show warning
```

### Track Progress

```python
# Use warnings to track migration progress
import warnings

def old_function():
    with warnings.catch_warnings(record=True) as w:
        warnings.simplefilter("always")
        jokers = state.jokers
        
        if w:
            print(f"Migration needed: {len(w)} deprecated APIs used")
```

## Testing Your Migration

### Verify Equivalent Results

```python
def test_api_equivalence(game):
    """Ensure old and new APIs return equivalent data"""
    state = game.state
    
    # Suppress warnings for comparison
    with warnings.catch_warnings():
        warnings.simplefilter("ignore")
        old_jokers = state.jokers
    
    new_joker_ids = state.joker_ids
    
    # Should have same count
    assert len(old_jokers) == len(new_joker_ids)
    
    # Should have equivalent information
    for old_joker, new_id in zip(old_jokers, new_joker_ids):
        new_info = game.get_joker_info(new_id)
        assert old_joker.name() == new_info.name
        assert old_joker.cost() == game.get_joker_cost(new_id)
```

## Support and Resources

### Getting Help

1. **Issue Tracker**: Report migration problems at [GitHub Issues](https://github.com/spencerduncan/balatro-rs/issues)
2. **Examples**: See `test_backwards_compatibility.py` for working examples
3. **API Reference**: Check `JokerDefinition` class for complete metadata structure

### Migration Checklist

- [ ] Replace `state.jokers` with `state.joker_ids` + registry calls
- [ ] Update joker information access to use `get_joker_info()`
- [ ] Update cost calculations to use `get_joker_cost()`
- [ ] Update shopping logic to use `can_buy_joker()`
- [ ] Test for deprecation warnings
- [ ] Verify equivalent functionality
- [ ] Update documentation and comments

### Performance Notes

- ✅ **Registry calls are cached** - repeated `get_joker_info()` calls are fast
- ✅ **Enum comparisons are faster** - `JokerId` comparisons outperform string comparisons
- ✅ **Memory efficient** - New API reduces memory overhead
- ✅ **Thread safe** - Registry system supports concurrent access

## Timeline Reminders

- **Now**: Start migration, warnings appear
- **6 months**: Old API deprecated but supported
- **12 months**: Old API removed

**Action Required**: Begin migration now to avoid disruption when the old API is removed.