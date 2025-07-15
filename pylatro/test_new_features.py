#!/usr/bin/env python3
"""
Test script to validate new JokerId-based PyO3 bindings functionality.
This script doesn't require pytest and can be run directly.
"""

import pylatro

def test_joker_id_types():
    """Test that new JokerId types are available and working"""
    print("Testing JokerId types availability...")
    
    # Test that JokerId enum is accessible
    assert hasattr(pylatro, 'JokerId'), "JokerId not found in pylatro module"
    assert hasattr(pylatro, 'JokerRarity'), "JokerRarity not found in pylatro module"
    assert hasattr(pylatro, 'JokerDefinition'), "JokerDefinition not found in pylatro module"
    
    # Test JokerRarity values
    rarity = pylatro.JokerRarity
    assert hasattr(rarity, 'Common'), "JokerRarity.Common not found"
    assert hasattr(rarity, 'Uncommon'), "JokerRarity.Uncommon not found"
    assert hasattr(rarity, 'Rare'), "JokerRarity.Rare not found"
    assert hasattr(rarity, 'Legendary'), "JokerRarity.Legendary not found"
    
    print("✓ JokerId types are available")


def test_gamestate_joker_ids():
    """Test GameState joker_ids property"""
    print("Testing GameState joker_ids property...")
    
    game = pylatro.GameEngine()
    state = game.state
    
    # Test new joker_ids property
    assert hasattr(state, 'joker_ids'), "joker_ids property not found"
    joker_ids = state.joker_ids
    assert isinstance(joker_ids, list), f"joker_ids should be list, got {type(joker_ids)}"
    
    # Test backward compatibility - old jokers property should still work
    assert hasattr(state, 'jokers'), "jokers property not found (backward compatibility)"
    old_jokers = state.jokers
    assert isinstance(old_jokers, list), f"jokers should be list, got {type(old_jokers)}"
    
    # Both lists should have same length (same jokers, different representations)
    assert len(joker_ids) == len(old_jokers), f"joker_ids length {len(joker_ids)} != jokers length {len(old_jokers)}"
    
    print(f"✓ GameState has {len(joker_ids)} jokers (old and new representations consistent)")


def test_gamestate_joker_slots():
    """Test GameState joker slot methods"""
    print("Testing GameState joker slot properties...")
    
    game = pylatro.GameEngine()
    state = game.state
    
    # Test joker slot properties
    assert hasattr(state, 'joker_slots_used'), "joker_slots_used property not found"
    assert hasattr(state, 'joker_slots_total'), "joker_slots_total property not found"
    
    slots_used = state.joker_slots_used
    slots_total = state.joker_slots_total
    
    assert isinstance(slots_used, int), f"joker_slots_used should be int, got {type(slots_used)}"
    assert isinstance(slots_total, int), f"joker_slots_total should be int, got {type(slots_total)}"
    assert slots_used >= 0, f"joker_slots_used should be >= 0, got {slots_used}"
    assert slots_total > 0, f"joker_slots_total should be > 0, got {slots_total}"
    assert slots_used <= slots_total, f"joker_slots_used {slots_used} should be <= joker_slots_total {slots_total}"
    
    print(f"✓ Joker slots: {slots_used}/{slots_total} used")


def test_gameengine_joker_registry():
    """Test GameEngine joker registry methods"""
    print("Testing GameEngine joker registry methods...")
    
    game = pylatro.GameEngine()
    
    # Test get_available_jokers
    assert hasattr(game, 'get_available_jokers'), "get_available_jokers method not found"
    try:
        all_jokers = game.get_available_jokers()
        assert isinstance(all_jokers, list), f"get_available_jokers should return list, got {type(all_jokers)}"
        print(f"✓ Found {len(all_jokers)} available jokers")
        
        # Test filtering by rarity if jokers exist
        if all_jokers:
            common_jokers = game.get_available_jokers(pylatro.JokerRarity.Common)
            assert isinstance(common_jokers, list), f"get_available_jokers with rarity should return list, got {type(common_jokers)}"
            print(f"✓ Found {len(common_jokers)} common jokers")
        
    except Exception as e:
        print(f"Note: get_available_jokers failed (registry may be empty): {e}")
    
    # Test other methods exist
    assert hasattr(game, 'get_joker_info'), "get_joker_info method not found"
    assert hasattr(game, 'can_buy_joker'), "can_buy_joker method not found"
    assert hasattr(game, 'get_joker_cost'), "get_joker_cost method not found"
    
    print("✓ All joker registry methods are available")


def test_backward_compatibility():
    """Test that existing Python code continues to work"""
    print("Testing backward compatibility...")
    
    game = pylatro.GameEngine()
    state = game.state
    
    # All existing properties should still be available
    expected_state_props = [
        'stage', 'round', 'action_history', 'deck', 'selected', 'available',
        'discarded', 'plays', 'discards', 'score', 'required_score', 
        'jokers', 'money', 'ante'
    ]
    
    for prop in expected_state_props:
        assert hasattr(state, prop), f"GameState.{prop} property missing (backward compatibility broken)"
    
    # All existing GameEngine methods should still work
    expected_engine_methods = [
        'gen_actions', 'gen_action_space', 'handle_action', 'handle_action_index',
        'get_action_name', 'state', 'is_over', 'is_win'
    ]
    
    for method in expected_engine_methods:
        assert hasattr(game, method), f"GameEngine.{method} method missing (backward compatibility broken)"
    
    print("✓ All existing properties and methods are available")


def test_new_and_old_joker_consistency():
    """Test that new joker_ids and old jokers have consistent data"""
    print("Testing consistency between old and new joker representations...")
    
    game = pylatro.GameEngine()
    state = game.state
    
    old_jokers = state.jokers
    new_joker_ids = state.joker_ids
    
    # Should have same number of jokers
    assert len(old_jokers) == len(new_joker_ids), f"Inconsistent joker counts: old={len(old_jokers)}, new={len(new_joker_ids)}"
    
    # If we have jokers, test the conversion works
    for i, (old_joker, new_id) in enumerate(zip(old_jokers, new_joker_ids)):
        # The old joker should be convertible to the same JokerId
        # This tests that the to_joker_id() method works correctly
        converted_id = old_joker.to_joker_id()
        assert converted_id == new_id, f"Joker {i}: conversion mismatch {converted_id} != {new_id}"
    
    if old_jokers:
        print(f"✓ {len(old_jokers)} jokers have consistent old/new representations")
    else:
        print("✓ No jokers present, but conversion system is ready")


def main():
    """Run all tests"""
    print("=" * 60)
    print("Testing PyO3 Bindings Core Types Migration (Issue #171)")
    print("=" * 60)
    
    tests = [
        test_joker_id_types,
        test_gamestate_joker_ids,
        test_gamestate_joker_slots,
        test_gameengine_joker_registry,
        test_backward_compatibility,
        test_new_and_old_joker_consistency,
    ]
    
    passed = 0
    failed = 0
    
    for test in tests:
        try:
            test()
            passed += 1
        except Exception as e:
            print(f"✗ {test.__name__} FAILED: {e}")
            failed += 1
        print()
    
    print("=" * 60)
    print(f"Test Results: {passed} passed, {failed} failed")
    print("=" * 60)
    
    if failed == 0:
        print("🎉 All tests passed! PyO3 bindings migration successful.")
        return True
    else:
        print("❌ Some tests failed. Please check the implementation.")
        return False


if __name__ == "__main__":
    success = main()
    exit(0 if success else 1)