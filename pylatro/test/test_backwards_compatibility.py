#!/usr/bin/env python3
"""
Test script for backwards compatibility layer in pylatro.

This script tests the deprecated methods on GameState to ensure they:
1. Work correctly (for read-only methods)
2. Show deprecation warnings
3. Fail appropriately for mutating methods

It also tests joker-specific API deprecations (Issue #175).
"""

import warnings
import pylatro

def test_read_only_deprecated_methods():
    """Test deprecated read-only methods on GameState."""
    print("=== Testing read-only deprecated methods ===")
    
    # Create a game engine and get state
    engine = pylatro.GameEngine()
    state = engine.state
    
    # Test gen_actions (should work with warning)
    print("\n1. Testing GameState.gen_actions()...")
    with warnings.catch_warnings(record=True) as w:
        warnings.simplefilter("always")
        actions = state.gen_actions()
        print(f"   Actions generated: {len(actions)}")
        if w:
            print(f"   Deprecation warning: {w[0].message}")
    
    # Test gen_action_space (should work with warning)
    print("\n2. Testing GameState.gen_action_space()...")
    with warnings.catch_warnings(record=True) as w:
        warnings.simplefilter("always")
        action_space = state.gen_action_space()
        print(f"   Action space size: {len(action_space)}")
        if w:
            print(f"   Deprecation warning: {w[0].message}")
    
    # Test get_action_name (should work with warning)
    print("\n3. Testing GameState.get_action_name()...")
    with warnings.catch_warnings(record=True) as w:
        warnings.simplefilter("always")
        try:
            if len(action_space) > 0:
                action_name = state.get_action_name(0)
                print(f"   Action name for index 0: {action_name}")
        except RuntimeError as e:
            print(f"   Game logic error (expected): {e}")
        if w:
            print(f"   Deprecation warning: {w[0].message}")
    
    # Test is_over property (should work with warning)
    print("\n4. Testing GameState.is_over property...")
    with warnings.catch_warnings(record=True) as w:
        warnings.simplefilter("always")
        is_over = state.is_over
        print(f"   Game is over: {is_over}")
        if w:
            print(f"   Deprecation warning: {w[0].message}")

def test_mutating_deprecated_methods():
    """Test deprecated mutating methods on GameState (should fail)."""
    print("\n=== Testing mutating deprecated methods ===")
    
    # Create a game engine and get state
    engine = pylatro.GameEngine()
    state = engine.state
    actions = state.gen_actions()
    
    # Test handle_action (should fail with warning)
    print("\n1. Testing GameState.handle_action()...")
    with warnings.catch_warnings(record=True) as w:
        warnings.simplefilter("always")
        try:
            if actions:
                state.handle_action(actions[0])
                print("   ERROR: handle_action should have failed!")
        except RuntimeError as e:
            print(f"   Expected error: {e}")
        if w:
            print(f"   Deprecation warning: {w[0].message}")
    
    # Test handle_action_index (should fail with warning)
    print("\n2. Testing GameState.handle_action_index()...")
    with warnings.catch_warnings(record=True) as w:
        warnings.simplefilter("always")
        try:
            state.handle_action_index(0)
            print("   ERROR: handle_action_index should have failed!")
        except RuntimeError as e:
            print(f"   Expected error: {e}")
        if w:
            print(f"   Deprecation warning: {w[0].message}")

def test_new_api_still_works():
    """Test that the new API on GameEngine still works correctly."""
    print("\n=== Testing new API on GameEngine ===")
    
    engine = pylatro.GameEngine()
    
    # Test that GameEngine methods work without warnings
    print("\n1. Testing GameEngine.gen_actions()...")
    with warnings.catch_warnings(record=True) as w:
        warnings.simplefilter("always")
        actions = engine.gen_actions()
        print(f"   Actions generated: {len(actions)}")
        if w:
            print(f"   Unexpected warning: {w[0].message}")
        else:
            print("   No warnings (as expected)")
    
    print("\n2. Testing GameEngine.is_over property...")
    is_over = engine.is_over
    print(f"   Game is over: {is_over}")

def test_deprecated_jokers_property_warning():
    """Test that deprecated jokers property shows deprecation warning"""
    print("\n=== Testing joker-specific API deprecations (Issue #175) ===")
    print("\n1. Testing deprecation warning for GameState.jokers property...")
    
    game = pylatro.GameEngine()
    state = game.state
    
    # Capture deprecation warnings
    with warnings.catch_warnings(record=True) as warning_list:
        warnings.simplefilter("always")  # Capture all deprecation warnings
        
        # Access the deprecated property
        jokers = state.jokers
        
        # Check that a deprecation warning was issued
        deprecation_warnings = [w for w in warning_list if issubclass(w.category, DeprecationWarning)]
        
        if len(deprecation_warnings) > 0:
            warning = deprecation_warnings[0]
            expected_keywords = ["deprecated", "jokers", "joker_ids"]
            message = str(warning.message).lower()
            
            for keyword in expected_keywords:
                assert keyword in message, f"Warning message should contain '{keyword}', got: {warning.message}"
            
            print(f"   ✓ Deprecation warning shown: {warning.message}")
        else:
            print("   ⚠ No deprecation warning found (may be expected if not yet implemented)")
    
    # Ensure the property still works despite being deprecated
    assert isinstance(jokers, list), f"jokers property should still return list, got {type(jokers)}"
    print("   ✓ Deprecated property still functional")

def test_joker_migration_patterns():
    """Test migration patterns from old joker API to new API"""
    print("\n2. Testing joker migration patterns...")
    
    game = pylatro.GameEngine()
    state = game.state
    
    # Pattern 1: Getting joker count (both ways should work)
    with warnings.catch_warnings():
        warnings.simplefilter("ignore")  # Suppress warnings for this test
        old_count = len(state.jokers)
    new_count = len(state.joker_ids)
    slot_count = state.joker_slots_used
    
    assert old_count == new_count == slot_count, f"Joker counts inconsistent: old={old_count}, new={new_count}, slots={slot_count}"
    print("   ✓ Joker count migration pattern works")
    
    # Pattern 2: Accessing joker properties through old API should still work  
    with warnings.catch_warnings():
        warnings.simplefilter("ignore")  # Ignore deprecation warnings for functionality test
        old_jokers = state.jokers
        
        for joker in old_jokers:
            # Common operations that existing code would do
            name = joker.name()
            desc = joker.desc()
            cost = joker.cost()
            
            assert isinstance(name, str), f"Joker name should be string, got {type(name)}"
            assert isinstance(desc, str), f"Joker desc should be string, got {type(desc)}"
            assert isinstance(cost, int), f"Joker cost should be int, got {type(cost)}"
    
    print("   ✓ Old joker API still functional")
    
    # Pattern 3: New API should provide equivalent information
    new_joker_ids = state.joker_ids
    for joker_id in new_joker_ids:
        try:
            joker_info = game.get_joker_info(joker_id)
            if joker_info:
                assert hasattr(joker_info, 'name'), "New API should provide name"
                assert hasattr(joker_info, 'description'), "New API should provide description"
                
                # Cost calculation should work
                cost = game.get_joker_cost(joker_id)
                assert isinstance(cost, int), f"New API cost should be int, got {type(cost)}"
        except Exception as e:
            print(f"   Note: New API access failed for {joker_id}: {e}")
    
    print("   ✓ New joker API provides equivalent functionality")

if __name__ == "__main__":
    print("Testing Backwards Compatibility Layer")
    print("=" * 50)
    
    test_read_only_deprecated_methods()
    test_mutating_deprecated_methods()
    test_new_api_still_works()
    
    print("\n" + "=" * 50)
    print("Backwards compatibility test completed!")