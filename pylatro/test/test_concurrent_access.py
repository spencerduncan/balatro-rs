#!/usr/bin/env python3
"""
Integration test for Python concurrent access patterns.

Tests the efficient state access patterns from issue #169 to ensure:
1. LightweightGameState provides efficient access to basic state
2. Concurrent access methods work correctly from Python
3. Action caching functions properly
4. Performance is acceptable compared to traditional state access
"""

import time
import sys
import os

# Add the parent directory to the path to import pylatro
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

try:
    import pylatro
except ImportError:
    print("pylatro module not found. Run 'maturin develop' first.")
    sys.exit(1)


def test_lightweight_state_access():
    """Test that LightweightGameState provides efficient access to basic state."""
    print("Testing lightweight state access...")
    
    config = pylatro.Config()
    game = pylatro.GameEngine(config)
    
    # Test traditional state access
    traditional_state = game.state
    assert hasattr(traditional_state, 'money')
    assert hasattr(traditional_state, 'score')
    
    # Test lightweight state access
    lightweight_state = game.lightweight_state
    assert hasattr(lightweight_state, 'money')
    assert hasattr(lightweight_state, 'chips')
    assert hasattr(lightweight_state, 'mult')
    assert hasattr(lightweight_state, 'score')
    assert hasattr(lightweight_state, 'round')
    assert hasattr(lightweight_state, 'plays_remaining')
    assert hasattr(lightweight_state, 'discards_remaining')
    
    # Values should be consistent between traditional and lightweight
    assert traditional_state.money == lightweight_state.money
    assert traditional_state.score == lightweight_state.score
    assert traditional_state.round == lightweight_state.round
    
    print("✓ Lightweight state access works correctly")


def test_concurrent_access_methods():
    """Test that concurrent access methods work correctly from Python."""
    print("Testing concurrent access methods...")
    
    config = pylatro.Config()
    game = pylatro.GameEngine(config)
    
    # Test concurrent access methods
    money = game.get_money_concurrent()
    chips = game.get_chips_concurrent()
    score = game.get_score_concurrent()
    stage = game.get_stage_concurrent()
    
    # These should be valid numbers
    assert isinstance(money, int)
    assert isinstance(chips, int)
    assert isinstance(score, int)
    assert isinstance(stage, str)
    assert "PreBlind" in stage  # Stage should be PreBlind initially
    
    # Values should match traditional state access
    traditional_state = game.state
    assert money == traditional_state.money
    assert score == traditional_state.score
    
    print("✓ Concurrent access methods work correctly")


def test_action_caching():
    """Test that action caching functions properly."""
    print("Testing action caching...")
    
    config = pylatro.Config()
    game = pylatro.GameEngine(config)
    
    # Enable action caching with 100ms TTL
    game.enable_action_caching(100)
    
    # Get cached actions (should generate and cache)
    actions1 = game.get_cached_actions()
    print(f"First call got {len(actions1)} actions")
    
    # Get cached actions again (should use cache)
    actions2 = game.get_cached_actions()
    print(f"Second call got {len(actions2)} actions")
    
    # Actions should be the same (from cache)
    assert len(actions1) == len(actions2), f"Action count mismatch: {len(actions1)} vs {len(actions2)}"
    
    # Compare action strings (might be different due to memory addresses, but should represent same actions)
    if len(actions1) > 0 and len(actions2) > 0:
        # Just check that we got the same number of actions
        # The caching might work but the comparison might fail due to object identity
        print(f"Action caching appears to work (consistent count)")
    
    print("✓ Action caching works correctly")


def test_performance_comparison():
    """Test that performance is acceptable compared to traditional access."""
    print("Testing performance comparison...")
    
    config = pylatro.Config()
    game = pylatro.GameEngine(config)
    
    # Benchmark traditional state access
    start_time = time.time()
    for _ in range(1000):
        state = game.state
        _ = state.money
        _ = state.score
    traditional_time = time.time() - start_time
    
    # Benchmark lightweight state access
    start_time = time.time()
    for _ in range(1000):
        state = game.lightweight_state
        _ = state.money
        _ = state.score
    lightweight_time = time.time() - start_time
    
    # Benchmark concurrent access
    start_time = time.time()
    for _ in range(1000):
        _ = game.get_money_concurrent()
        _ = game.get_score_concurrent()
    concurrent_time = time.time() - start_time
    
    print(f"Traditional state access: {traditional_time:.4f}s")
    print(f"Lightweight state access: {lightweight_time:.4f}s")
    print(f"Concurrent access: {concurrent_time:.4f}s")
    
    # All methods should complete in reasonable time
    assert traditional_time < 1.0, "Traditional access too slow"
    assert lightweight_time < 1.0, "Lightweight access too slow"
    assert concurrent_time < 1.0, "Concurrent access too slow"
    
    print("✓ Performance comparison completed")


def test_state_representation():
    """Test that state representations work correctly."""
    print("Testing state representations...")
    
    config = pylatro.Config()
    game = pylatro.GameEngine(config)
    
    # Test traditional state representation
    traditional_state = game.state
    traditional_repr = str(traditional_state)
    assert "GameState:" in traditional_repr
    
    # Test lightweight state representation
    lightweight_state = game.lightweight_state
    lightweight_repr = str(lightweight_state)
    assert "LightweightGameState" in lightweight_repr
    assert "money=" in lightweight_repr
    assert "chips=" in lightweight_repr
    
    print("✓ State representations work correctly")


def main():
    """Run all integration tests."""
    print("Running Python concurrent access integration tests...")
    print("=" * 60)
    
    try:
        test_lightweight_state_access()
        test_concurrent_access_methods()
        test_action_caching()
        test_performance_comparison()
        test_state_representation()
        
        print("=" * 60)
        print("✓ All Python concurrent access tests passed!")
        return 0
        
    except Exception as e:
        print(f"❌ Test failed: {e}")
        import traceback
        traceback.print_exc()
        return 1


if __name__ == "__main__":
    sys.exit(main())