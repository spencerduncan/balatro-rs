#!/usr/bin/env python3
"""
Test script for backwards compatibility layer in pylatro.

This script tests the deprecated methods on GameState to ensure they:
1. Work correctly (for read-only methods)
2. Show deprecation warnings
3. Fail appropriately for mutating methods
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

if __name__ == "__main__":
    print("Testing Backwards Compatibility Layer")
    print("=" * 50)
    
    test_read_only_deprecated_methods()
    test_mutating_deprecated_methods()
    test_new_api_still_works()
    
    print("\n" + "=" * 50)
    print("Backwards compatibility test completed!")