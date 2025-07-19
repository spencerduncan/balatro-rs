import sys
import os

# Add the parent directory to the path so we can import pylatro
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

import pylatro


def test_get_joker_metadata():
    """Test getting comprehensive joker metadata"""
    engine = pylatro.GameEngine()
    
    # Test getting metadata for existing joker
    metadata = engine.get_joker_metadata(pylatro.JokerId.Joker)
    assert metadata is not None
    assert metadata.id == pylatro.JokerId.Joker
    assert metadata.name == "Joker"
    assert metadata.description == "+4 Mult"
    assert metadata.rarity == pylatro.JokerRarity.Common
    assert metadata.cost == 3  # Common joker cost
    assert metadata.sell_value == 1  # Half of cost
    assert metadata.effect_type == "additive_mult"
    assert metadata.is_unlocked == True  # Currently always true
    
    # Test getting metadata for non-existent joker
    # Skip this test for now since we don't have an invalid JokerId to test with
    # TODO: Add test for non-existent joker when registry supports it


def test_get_joker_properties():
    """Test getting basic joker properties as dictionary"""
    engine = pylatro.GameEngine()
    
    # Test getting properties for existing joker
    props = engine.get_joker_properties(pylatro.JokerId.Joker)
    assert props is not None
    assert isinstance(props, dict)
    assert props["name"] == "Joker"
    assert props["description"] == "+4 Mult"
    assert props["cost"] == 3
    assert props["sell_value"] == 1
    assert "rarity" in props
    assert "id" in props
    
    # Test getting properties for non-existent joker
    # Skip this test for now since we don't have an invalid JokerId to test with
    # TODO: Add test for non-existent joker when registry supports it


def test_get_joker_effect_info():
    """Test getting joker effect information"""
    engine = pylatro.GameEngine()
    
    # Test basic mult joker
    effect_info = engine.get_joker_effect_info(pylatro.JokerId.Joker)
    assert effect_info is not None
    assert effect_info["effect_type"] == "additive_mult"
    assert effect_info["effect_description"] == "+4 Mult"
    assert isinstance(effect_info["triggers_on"], list)
    assert "passive" in effect_info["triggers_on"]
    assert isinstance(effect_info["conditions"], list)
    assert "always" in effect_info["conditions"]
    assert effect_info["uses_state"] == False
    assert effect_info["max_triggers"] is None
    assert effect_info["persistent_data"] == False
    
    # Test suit-based joker
    effect_info = engine.get_joker_effect_info(pylatro.JokerId.GreedyJoker)
    assert effect_info is not None
    assert "card_scored" in effect_info["triggers_on"]
    assert "suit:diamonds" in effect_info["conditions"]
    
    # Test non-existent joker
    # Skip this test for now since we don't have an invalid JokerId to test with
    # TODO: Add test for non-existent joker when registry supports it


def test_get_joker_unlock_status():
    """Test getting joker unlock status and conditions"""
    engine = pylatro.GameEngine()
    
    # Test basic joker (no unlock condition)
    unlock_info = engine.get_joker_unlock_status(pylatro.JokerId.Joker)
    assert unlock_info is not None
    assert unlock_info["is_unlocked"] == True
    assert unlock_info["unlock_condition"] is None
    
    # Test joker with unlock condition
    # Note: Need to find a joker with actual unlock condition
    # For now, all jokers return is_unlocked=True
    
    # Test non-existent joker
    # Skip this test for now since we don't have an invalid JokerId to test with
    # TODO: Add test for non-existent joker when registry supports it


def test_metadata_for_different_rarities():
    """Test metadata for jokers of different rarities"""
    engine = pylatro.GameEngine()
    
    # Common joker
    metadata = engine.get_joker_metadata(pylatro.JokerId.Joker)
    assert metadata.rarity == pylatro.JokerRarity.Common
    assert metadata.cost == 3
    
    # TODO: Add tests for Uncommon, Rare, and Legendary jokers
    # when they are available in the registry


def test_effect_type_detection():
    """Test that effect types are correctly detected"""
    engine = pylatro.GameEngine()
    
    # Additive mult
    metadata = engine.get_joker_metadata(pylatro.JokerId.Joker)
    assert metadata.effect_type == "additive_mult"
    
    # Test Ice Cream joker
    metadata = engine.get_joker_metadata(pylatro.JokerId.IceCream)
    if metadata:  # Ice Cream might not be registered yet
        # Effect type is detected as "additive_chips" based on description
        assert metadata.effect_type == "additive_chips"
        assert metadata.uses_state == True
        assert metadata.persistent_data == True


def test_get_multiple_joker_metadata():
    """Test getting metadata for multiple jokers at once"""
    engine = pylatro.GameEngine()
    
    # Test with valid joker IDs
    joker_ids = [pylatro.JokerId.Joker, pylatro.JokerId.GreedyJoker, pylatro.JokerId.IceCream]
    metadata_list = engine.get_multiple_joker_metadata(joker_ids)
    
    assert len(metadata_list) == 3
    assert metadata_list[0] is not None
    assert metadata_list[0].id == pylatro.JokerId.Joker
    assert metadata_list[0].name == "Joker"
    
    # Test with known valid IDs
    # (Skipping test with invalid IDs for now since we don't have an invalid JokerId to test with)
    known_ids = [pylatro.JokerId.Joker, pylatro.JokerId.GreedyJoker]
    known_results = engine.get_multiple_joker_metadata(known_ids)
    
    assert len(known_results) == 2
    assert known_results[0] is not None
    assert known_results[1] is not None
    
    # Test with empty list
    empty_results = engine.get_multiple_joker_metadata([])
    assert empty_results == []


def test_get_all_joker_metadata():
    """Test getting metadata for all registered jokers"""
    engine = pylatro.GameEngine()
    
    # Get all joker metadata
    all_metadata = engine.get_all_joker_metadata()
    
    # Should have at least one joker (The Joker)
    assert len(all_metadata) > 0
    
    # Check that all returned items are valid JokerMetadata
    for metadata in all_metadata:
        assert hasattr(metadata, 'id')
        assert hasattr(metadata, 'name')
        assert hasattr(metadata, 'description')
        assert hasattr(metadata, 'rarity')
        assert hasattr(metadata, 'cost')
        assert hasattr(metadata, 'effect_type')
    
    # Verify specific joker is in the list
    joker_names = [m.name for m in all_metadata]
    assert "Joker" in joker_names
    
    # Check that costs match rarity
    for metadata in all_metadata:
        if metadata.rarity == pylatro.JokerRarity.Common:
            assert metadata.cost == 3
        elif metadata.rarity == pylatro.JokerRarity.Uncommon:
            assert metadata.cost == 6
        elif metadata.rarity == pylatro.JokerRarity.Rare:
            assert metadata.cost == 8
        elif metadata.rarity == pylatro.JokerRarity.Legendary:
            assert metadata.cost == 20


if __name__ == "__main__":
    test_get_joker_metadata()
    test_get_joker_properties()
    test_get_joker_effect_info()
    test_get_joker_unlock_status()
    test_metadata_for_different_rarities()
    test_effect_type_detection()
    test_get_multiple_joker_metadata()
    test_get_all_joker_metadata()
    print("All tests passed!")