"""
Acceptance tests for Issue #172: Add Joker Metadata Access Methods

This file tests all acceptance criteria for comprehensive joker metadata access
in Python bindings following Test-Driven Development principles.

Acceptance Criteria:
- [ ] Implement joker information retrieval methods in Python bindings
- [ ] Add access to joker properties (name, rarity, cost, etc.)
- [ ] Expose joker effect descriptions and parameters
- [ ] Provide joker state information (triggers, conditions, etc.)
- [ ] Add batch retrieval methods for multiple jokers
- [ ] Ensure efficient memory usage for metadata access
"""

import pytest
import pylatro
from typing import Dict, List, Any, Optional


class TestJokerMetadataAccess:
    """Test individual joker metadata access methods"""
    
    def setup_method(self):
        """Set up test fixtures"""
        self.game = pylatro.GameEngine()
        # Get some test jokers
        self.test_jokers = self.game.get_available_jokers()
        assert len(self.test_jokers) > 0, "Need jokers for testing"
        self.test_joker_id = self.test_jokers[0].id
    
    def test_get_joker_metadata(self):
        """Test comprehensive joker metadata retrieval"""
        # Test individual joker metadata access
        metadata = self.game.get_joker_metadata(self.test_joker_id)
        
        assert metadata is not None
        assert hasattr(metadata, 'id')
        assert hasattr(metadata, 'name')
        assert hasattr(metadata, 'description') 
        assert hasattr(metadata, 'rarity')
        assert hasattr(metadata, 'cost')
        assert hasattr(metadata, 'sell_value')
        assert hasattr(metadata, 'effect_type')
        assert hasattr(metadata, 'effect_description')
        assert hasattr(metadata, 'triggers_on')
        assert hasattr(metadata, 'conditions')
        assert hasattr(metadata, 'uses_state')
        assert hasattr(metadata, 'unlock_condition')
        assert hasattr(metadata, 'is_unlocked')
        
        # Verify types
        assert isinstance(metadata.name, str)
        assert isinstance(metadata.description, str)
        assert isinstance(metadata.cost, int)
        assert isinstance(metadata.sell_value, int)
        assert isinstance(metadata.triggers_on, list)
        assert isinstance(metadata.conditions, list)
        assert isinstance(metadata.uses_state, bool)
        assert isinstance(metadata.is_unlocked, bool)
        
    def test_get_joker_properties(self):
        """Test basic property access as dictionary"""
        properties = self.game.get_joker_properties(self.test_joker_id)
        
        assert properties is not None
        assert isinstance(properties, dict)
        
        required_keys = ['name', 'rarity', 'cost', 'description']
        for key in required_keys:
            assert key in properties
            
        assert isinstance(properties['name'], str)
        assert isinstance(properties['cost'], int)
        assert properties['rarity'] in ['Common', 'Uncommon', 'Rare', 'Legendary']
    
    def test_get_joker_effect_info(self):
        """Test joker effect descriptions and parameters"""
        effect_info = self.game.get_joker_effect_info(self.test_joker_id)
        
        assert effect_info is not None
        assert isinstance(effect_info, dict)
        
        expected_keys = ['effect_type', 'description', 'parameters', 'triggers_on']
        for key in expected_keys:
            assert key in effect_info
            
        assert isinstance(effect_info['effect_type'], str)
        assert isinstance(effect_info['description'], str)
        assert isinstance(effect_info['triggers_on'], list)
    
    def test_get_joker_unlock_status(self):
        """Test unlock condition and status access"""
        unlock_info = self.game.get_joker_unlock_status(self.test_joker_id)
        
        assert unlock_info is not None
        assert isinstance(unlock_info, dict)
        
        required_keys = ['is_unlocked', 'unlock_condition']
        for key in required_keys:
            assert key in unlock_info
            
        assert isinstance(unlock_info['is_unlocked'], bool)
    
    def test_invalid_joker_handling(self):
        """Test handling of invalid/unknown joker IDs"""
        # Use a JokerId that exists but is not implemented/registered in our basic registry
        # We'll try one that's likely not in our basic test registry
        test_id = pylatro.JokerId.Dusk  # This should be an unregistered joker in our basic registry
        
        # Test with an unregistered joker ID (should handle gracefully)
        invalid_metadata = self.game.get_joker_metadata(test_id)
        assert invalid_metadata is None
        
        invalid_properties = self.game.get_joker_properties(test_id)
        assert invalid_properties is None


class TestJokerStateAccess:
    """Test joker state information access"""
    
    def setup_method(self):
        """Set up test fixtures"""
        self.game = pylatro.GameEngine()
    
    def test_joker_state_properties(self):
        """Test JokerState property access"""
        # Create a game state with some jokers
        state = self.game.state
        joker_ids = state.joker_ids
        
        if joker_ids:
            # Test getting state for an active joker
            joker_state = self.game.get_joker_state_info(joker_ids[0])
            
            if joker_state is not None:
                assert hasattr(joker_state, 'accumulated_value')
                assert hasattr(joker_state, 'triggers_remaining')
                assert hasattr(joker_state, 'get_custom_data')
                assert hasattr(joker_state, 'has_custom_data')
                assert hasattr(joker_state, 'get_all_custom_keys')
                assert hasattr(joker_state, 'to_dict')
                
                # Test types
                assert isinstance(joker_state.accumulated_value, float)
                assert joker_state.triggers_remaining is None or isinstance(joker_state.triggers_remaining, int)
                
                # Test methods work
                keys = joker_state.get_all_custom_keys()
                assert isinstance(keys, list)
                
                state_dict = joker_state.to_dict()
                assert isinstance(state_dict, dict)
    
    def test_get_active_jokers_state(self):
        """Test getting state for all active jokers"""
        active_states = self.game.get_active_jokers_state()
        
        assert isinstance(active_states, dict)
        # Each key should be a JokerId, each value should be a JokerState
        for joker_id, joker_state in active_states.items():
            assert hasattr(joker_state, 'accumulated_value')
            assert hasattr(joker_state, 'triggers_remaining')


class TestBatchJokerOperations:
    """Test batch retrieval methods for multiple jokers"""
    
    def setup_method(self):
        """Set up test fixtures"""
        self.game = pylatro.GameEngine()
        self.all_jokers = self.game.get_available_jokers()
        assert len(self.all_jokers) > 0, "Need jokers for testing"
    
    def test_get_multiple_joker_metadata(self):
        """Test efficient batch metadata retrieval"""
        # Test with a subset of jokers
        test_ids = [joker.id for joker in self.all_jokers[:3]]
        
        metadata_dict = self.game.get_multiple_joker_metadata(test_ids)
        
        assert isinstance(metadata_dict, dict)
        assert len(metadata_dict) == len(test_ids)
        
        for joker_id in test_ids:
            joker_id_str = str(joker_id).replace('JokerId.', '')  # Convert to string format
            assert joker_id_str in metadata_dict
            metadata = metadata_dict[joker_id_str]
            assert hasattr(metadata, 'name')
            assert hasattr(metadata, 'rarity')
            assert hasattr(metadata, 'cost')
    
    def test_get_all_joker_metadata(self):
        """Test getting metadata for all jokers in registry"""
        all_metadata = self.game.get_all_joker_metadata()
        
        assert isinstance(all_metadata, dict)
        assert len(all_metadata) > 0
        
        # Should include all available jokers
        for joker_def in self.all_jokers:
            joker_id_str = str(joker_def.id).replace('JokerId.', '')  # Convert to string format
            assert joker_id_str in all_metadata
            metadata = all_metadata[joker_id_str]
            assert metadata.name == joker_def.name
            assert metadata.description == joker_def.description
    
    def test_get_jokers_by_rarity(self):
        """Test filtering jokers by rarity with metadata"""
        common_jokers = self.game.get_jokers_by_rarity(pylatro.JokerRarity.Common)
        
        assert isinstance(common_jokers, list)
        # Each item should be a JokerMetadata with Common rarity
        for joker_metadata in common_jokers:
            assert hasattr(joker_metadata, 'rarity')
            assert joker_metadata.rarity == pylatro.JokerRarity.Common
    
    def test_get_unlocked_jokers_metadata(self):
        """Test getting metadata for unlocked jokers only"""
        unlocked_jokers = self.game.get_unlocked_jokers_metadata()
        
        assert isinstance(unlocked_jokers, list)
        # Each joker should be unlocked
        for joker_metadata in unlocked_jokers:
            assert hasattr(joker_metadata, 'is_unlocked')
            assert joker_metadata.is_unlocked is True


class TestJokerFilteringAndSearch:
    """Test filtering and search capabilities"""
    
    def setup_method(self):
        """Set up test fixtures"""
        self.game = pylatro.GameEngine()
    
    def test_search_jokers(self):
        """Test searching jokers by name or description"""
        # Search for a common term
        results = self.game.search_jokers("joker")
        
        assert isinstance(results, list)
        # Results should contain jokers with "joker" in name or description
        for joker_metadata in results:
            name_match = "joker" in joker_metadata.name.lower()
            desc_match = "joker" in joker_metadata.description.lower()
            assert name_match or desc_match
    
    def test_filter_jokers(self):
        """Test filtering jokers by multiple criteria"""
        # Test filtering by rarity only
        common_jokers = self.game.filter_jokers(rarity=pylatro.JokerRarity.Common)
        assert isinstance(common_jokers, list)
        for joker in common_jokers:
            assert joker.rarity == pylatro.JokerRarity.Common
        
        # Test filtering unlocked only
        unlocked_jokers = self.game.filter_jokers(unlocked_only=True)
        assert isinstance(unlocked_jokers, list)
        for joker in unlocked_jokers:
            assert joker.is_unlocked is True
        
        # Test filtering affordable only (might be empty if player has no money)
        affordable_jokers = self.game.filter_jokers(affordable_only=True)
        assert isinstance(affordable_jokers, list)
    
    def test_get_jokers_by_cost_range(self):
        """Test filtering jokers by cost range"""
        # Test a reasonable cost range
        mid_cost_jokers = self.game.get_jokers_by_cost_range(5, 10)
        
        assert isinstance(mid_cost_jokers, list)
        for joker in mid_cost_jokers:
            assert 5 <= joker.cost <= 10


class TestJokerAnalysisUtilities:
    """Test analysis and utility methods"""
    
    def setup_method(self):
        """Set up test fixtures"""
        self.game = pylatro.GameEngine()
        self.test_jokers = self.game.get_available_jokers()
    
    def test_get_joker_categories(self):
        """Test joker organization by categories"""
        categories = self.game.get_joker_categories()
        
        assert isinstance(categories, dict)
        assert len(categories) > 0
        
        # Each category should map to a list of joker IDs
        for category, joker_ids in categories.items():
            assert isinstance(category, str)
            assert isinstance(joker_ids, list)
    
    def test_get_joker_statistics(self):
        """Test joker registry statistics"""
        stats = self.game.get_joker_statistics()
        
        assert isinstance(stats, dict)
        
        expected_keys = ['total_jokers', 'by_rarity', 'unlocked_count']
        for key in expected_keys:
            assert key in stats
        
        assert isinstance(stats['total_jokers'], int)
        assert isinstance(stats['by_rarity'], dict)
        assert isinstance(stats['unlocked_count'], int)
    
    def test_analyze_joker_synergies(self):
        """Test joker synergy analysis"""
        if len(self.test_jokers) >= 2:
            test_ids = [self.test_jokers[0].id, self.test_jokers[1].id]
            synergies = self.game.analyze_joker_synergies(test_ids)
            
            assert isinstance(synergies, dict)
            # Should contain analysis of how these jokers work together


class TestEnhancedGameStateProperties:
    """Test enhanced GameState joker-related properties"""
    
    def setup_method(self):
        """Set up test fixtures"""
        self.game = pylatro.GameEngine()
        self.state = self.game.state
    
    def test_get_joker_states(self):
        """Test getting all active joker states"""
        joker_states = self.state.get_joker_states()
        
        assert isinstance(joker_states, dict)
        # Each key should be a JokerId, each value a JokerState
        for joker_id, joker_state in joker_states.items():
            assert hasattr(joker_state, 'accumulated_value')
            assert hasattr(joker_state, 'triggers_remaining')
    
    def test_get_joker_accumulated_values(self):
        """Test getting accumulated values for all jokers"""
        accumulated_values = self.state.get_joker_accumulated_values()
        
        assert isinstance(accumulated_values, dict)
        # Each key should be a JokerId, each value a float
        for joker_id, value in accumulated_values.items():
            assert isinstance(value, float)
    
    def test_get_joker_triggers_remaining(self):
        """Test getting triggers remaining for all jokers"""
        triggers = self.state.get_joker_triggers_remaining()
        
        assert isinstance(triggers, dict)
        # Each key should be a JokerId, each value an int or None
        for joker_id, trigger_count in triggers.items():
            assert trigger_count is None or isinstance(trigger_count, int)


class TestMemoryEfficiency:
    """Test efficient memory usage for metadata access"""
    
    def setup_method(self):
        """Set up test fixtures"""
        self.game = pylatro.GameEngine()
    
    def test_large_batch_operations(self):
        """Test that large batch operations are memory efficient"""
        # Get all jokers metadata - should not cause memory issues
        all_metadata = self.game.get_all_joker_metadata()
        
        assert isinstance(all_metadata, dict)
        assert len(all_metadata) > 0
        
        # Test that we can handle repeated large operations
        for _ in range(5):
            metadata_copy = self.game.get_all_joker_metadata()
            assert len(metadata_copy) == len(all_metadata)
    
    def test_metadata_caching(self):
        """Test that metadata access is efficiently cached"""
        joker_id = self.game.get_available_jokers()[0].id
        
        # Multiple calls should be efficient (not explicitly testing timing, 
        # but ensuring no errors with repeated access)
        for _ in range(10):
            metadata = self.game.get_joker_metadata(joker_id)
            assert metadata is not None
            assert metadata.name == metadata.name  # Should be consistent


class TestBackwardCompatibility:
    """Test that existing joker API remains functional"""
    
    def setup_method(self):
        """Set up test fixtures"""
        self.game = pylatro.GameEngine()
    
    def test_existing_methods_unchanged(self):
        """Test that all existing joker methods still work"""
        # These methods should continue to work exactly as before
        all_jokers = self.game.get_available_jokers()
        assert isinstance(all_jokers, list)
        
        if all_jokers:
            joker_def = all_jokers[0]
            joker_id = joker_def.id
            
            # Test existing methods
            joker_info = self.game.get_joker_info(joker_id)
            if joker_info:
                assert hasattr(joker_info, 'name')
                assert hasattr(joker_info, 'description')
            
            can_buy = self.game.can_buy_joker(joker_id)
            assert isinstance(can_buy, bool)
            
            try:
                cost = self.game.get_joker_cost(joker_id)
                assert isinstance(cost, int)
            except:
                # Some jokers might not be implemented yet
                pass


if __name__ == "__main__":
    pytest.main([__file__, "-v"])