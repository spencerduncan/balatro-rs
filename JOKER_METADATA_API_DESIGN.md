# Joker Metadata Access API Design - Issue #172

## Overview
This document outlines the comprehensive joker metadata access API design for Python bindings, implementing all acceptance criteria from issue #172.

## Design Principles
1. **Backward Compatibility**: Don't break existing APIs
2. **Consistency**: Follow existing naming patterns (`get_joker_*`)
3. **Performance**: Efficient batch operations and memory usage
4. **Comprehensive**: Cover all metadata aspects
5. **Pythonic**: Easy to use from Python/ML frameworks

## New Types to Expose

### 1. JokerMetadata (New Aggregation Type)
```python
@dataclass
class JokerMetadata:
    # Core properties
    id: JokerId
    name: str
    description: str
    rarity: JokerRarity
    cost: int
    sell_value: int
    
    # Effect information
    effect_type: str
    effect_description: str
    scaling_info: Optional[Dict[str, Any]]
    
    # Conditional information
    triggers_on: List[str]  # e.g., ["card_scored", "hand_played"]
    conditions: List[str]   # e.g., ["suit:Diamond", "rank:Ace"]
    
    # State information
    uses_state: bool
    max_triggers: Optional[int]
    persistent_data: bool
    
    # Unlock information
    unlock_condition: Optional[UnlockCondition]
    is_unlocked: bool
```

### 2. JokerState (Enhanced Exposure)
```python
class JokerState:
    # Existing properties exposed
    accumulated_value: float
    triggers_remaining: Optional[int]
    
    # New methods
    def get_custom_data(self, key: str) -> Optional[Any]
    def has_custom_data(self, key: str) -> bool
    def get_all_custom_keys(self) -> List[str]
    def to_dict(self) -> Dict[str, Any]
```

## Enhanced GameEngine Methods

### 1. Individual Joker Metadata Access
```python
class GameEngine:
    # Enhanced existing method with more comprehensive data
    def get_joker_metadata(self, joker_id: JokerId) -> Optional[JokerMetadata]:
        """Get comprehensive metadata for a specific joker."""
        
    def get_joker_properties(self, joker_id: JokerId) -> Optional[Dict[str, Any]]:
        """Get basic properties (name, rarity, cost) as dictionary."""
        
    def get_joker_effect_info(self, joker_id: JokerId) -> Optional[Dict[str, Any]]:
        """Get effect descriptions and parameters."""
        
    def get_joker_state_info(self, joker_id: JokerId) -> Optional[JokerState]:
        """Get current state information for an active joker."""
        
    def get_joker_unlock_status(self, joker_id: JokerId) -> Dict[str, Any]:
        """Get unlock condition and current unlock status."""
```

### 2. Batch Retrieval Methods
```python
class GameEngine:
    def get_multiple_joker_metadata(self, joker_ids: List[JokerId]) -> Dict[JokerId, JokerMetadata]:
        """Get metadata for multiple jokers efficiently."""
        
    def get_all_joker_metadata(self) -> Dict[JokerId, JokerMetadata]:
        """Get metadata for all jokers in the registry."""
        
    def get_jokers_by_rarity(self, rarity: JokerRarity, include_metadata: bool = True) -> List[JokerMetadata]:
        """Get all jokers of specific rarity with optional full metadata."""
        
    def get_unlocked_jokers_metadata(self) -> List[JokerMetadata]:
        """Get metadata for all currently unlocked jokers."""
        
    def get_active_jokers_state(self) -> Dict[JokerId, JokerState]:
        """Get state information for all active jokers."""
```

### 3. Filtering and Search Methods
```python
class GameEngine:
    def search_jokers(self, query: str) -> List[JokerMetadata]:
        """Search jokers by name or description."""
        
    def filter_jokers(self, 
                     rarity: Optional[JokerRarity] = None,
                     unlocked_only: bool = False,
                     affordable_only: bool = False,
                     triggers_on: Optional[List[str]] = None) -> List[JokerMetadata]:
        """Filter jokers by multiple criteria."""
        
    def get_jokers_by_cost_range(self, min_cost: int, max_cost: int) -> List[JokerMetadata]:
        """Get jokers within a cost range."""
```

### 4. Analysis and Utility Methods
```python
class GameEngine:
    def analyze_joker_synergies(self, joker_ids: List[JokerId]) -> Dict[str, Any]:
        """Analyze potential synergies between jokers."""
        
    def get_joker_categories(self) -> Dict[str, List[JokerId]]:
        """Get jokers organized by categories (scoring, economy, etc.)."""
        
    def get_joker_statistics(self) -> Dict[str, Any]:
        """Get statistics about joker registry (counts by rarity, etc.)."""
```

## Enhanced GameState Properties

### New State Properties
```python
class GameState:
    # Enhanced joker state access
    def get_joker_states(self) -> Dict[JokerId, JokerState]:
        """Get all active joker states."""
        
    def get_joker_accumulated_values(self) -> Dict[JokerId, float]:
        """Get accumulated values for all jokers."""
        
    def get_joker_triggers_remaining(self) -> Dict[JokerId, Optional[int]]:
        """Get remaining triggers for all jokers."""
```

## Implementation Strategy

### Phase 1: Core Metadata Types
1. Create `JokerMetadata` struct in Rust with PyO3 bindings
2. Enhance `JokerState` exposure to Python
3. Add core individual metadata access methods

### Phase 2: Batch Operations
1. Implement efficient batch retrieval in Rust
2. Add Python wrapper methods for batch operations
3. Optimize memory usage with lazy loading

### Phase 3: Advanced Features
1. Add filtering and search capabilities
2. Implement analysis and utility methods
3. Add comprehensive error handling

### Phase 4: Documentation and Examples
1. Create comprehensive Python examples
2. Add docstrings for all new methods
3. Update existing examples to showcase new API

## Memory Optimization Strategy

### 1. Lazy Loading
- Load full metadata only when requested
- Cache frequently accessed data
- Use weak references where appropriate

### 2. Batch Operations
- Single registry lock acquisition for batch operations
- Minimize data copying between Rust and Python
- Use views for read-only operations

### 3. Efficient Data Structures
- Pre-compute common groupings (by rarity, category)
- Use compact representations for large datasets
- Implement smart caching with TTL

## Backward Compatibility

### Existing API Preservation
- All existing methods remain unchanged
- New methods use consistent naming patterns
- Deprecated methods marked but still functional

### Migration Path
- Provide examples showing old vs new API usage
- Document performance benefits of new methods
- Gradual migration guidance in documentation

## Error Handling

### Consistent Error Types
- Use existing `GameError` types where applicable
- Provide meaningful error messages
- Handle lock contention gracefully

### Python Error Mapping
- Map Rust errors to appropriate Python exceptions
- Provide error context for debugging
- Handle missing/invalid joker IDs gracefully

## Testing Strategy

### Unit Tests
- Test each new method individually
- Verify error handling and edge cases
- Check memory usage patterns

### Integration Tests
- Test batch operations with real data
- Verify consistency across API methods
- Performance benchmarks for large datasets

### Python Tests
- Test PyO3 bindings work correctly
- Verify data types and conversions
- Check Python-specific error handling

This design provides comprehensive joker metadata access while maintaining performance, compatibility, and usability for ML/RL applications.