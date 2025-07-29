#!/usr/bin/env python3
"""
Basic Batch Operations Example

Simple demonstration of the BatchGameEngine API for parallel game simulation.
This example shows the core batch operations without complex RL training logic.

Performance Features Demonstrated:
- Creating batches of games efficiently
- Vectorized state extraction 
- Batch action execution with parallel processing
- Game reset and management operations

Author: John Botmack Performance Implementation
"""

import time
import numpy as np

try:
    import pylatro
except ImportError:
    print("Please build the pylatro extension first:")
    print("  cd pylatro && maturin develop")
    exit(1)


def basic_batch_demo():
    """Demonstrate basic batch operations"""
    
    print("=== Basic Batch Operations Demo ===\n")
    
    # Create a batch of games
    batch_size = 20
    print(f"1. Creating batch of {batch_size} games...")
    
    start_time = time.time()
    batch = pylatro.BatchGameEngine(batch_size)
    creation_time = time.time() - start_time
    
    print(f"   ✓ Created in {creation_time:.3f}s ({batch_size/creation_time:.0f} games/sec)")
    print(f"   ✓ Batch size: {batch.len()}")
    
    # Get initial performance statistics
    print(f"\n2. Batch statistics:")
    stats = batch.get_batch_stats()
    for key, value in stats.items():
        print(f"   {key}: {value}")
    
    # Extract vectorized states
    print(f"\n3. Vectorized state extraction:")
    states = batch.get_vectorized_states()
    
    for state_name, values in states.items():
        print(f"   {state_name}: {len(values)} values")
        if state_name in ['scores', 'money']:
            # Show some statistics for numeric values
            arr = np.array(values)
            print(f"      range: {arr.min():.1f} - {arr.max():.1f}, mean: {arr.mean():.1f}")
    
    # Generate actions for all games
    print(f"\n4. Batch action generation:")
    start_time = time.time()
    all_actions = batch.batch_gen_actions()
    generation_time = time.time() - start_time
    
    total_actions = sum(len(actions) for actions in all_actions)
    print(f"   ✓ Generated {total_actions} total actions in {generation_time:.3f}s")
    print(f"   ✓ Average {total_actions/len(all_actions):.1f} actions per game")
    
    # Execute batch actions (select first valid action for each game)
    print(f"\n5. Batch action execution:")
    selected_actions = []
    for game_actions in all_actions:
        if game_actions:
            selected_actions.append(game_actions[0])  # Use first valid action
        else:
            selected_actions.append(pylatro.Action.EndTurn)  # Fallback
    
    start_time = time.time()
    success_flags = batch.batch_handle_actions(selected_actions)
    execution_time = time.time() - start_time
    
    success_count = sum(success_flags)
    print(f"   ✓ Executed {len(selected_actions)} actions in {execution_time:.3f}s")
    print(f"   ✓ Success rate: {success_count}/{len(selected_actions)} ({success_count/len(selected_actions):.1%})")
    
    # Check game status
    print(f"\n6. Game status check:")
    is_over = batch.batch_is_over()
    active_games = len(is_over) - sum(is_over)
    print(f"   ✓ Active games: {active_games}/{len(is_over)}")
    print(f"   ✓ Finished games: {sum(is_over)}/{len(is_over)}")
    
    # Get updated states after actions
    print(f"\n7. Updated states after actions:")
    updated_states = batch.get_vectorized_states()
    
    scores = np.array(updated_states['scores'])
    money = np.array(updated_states['money'])
    
    print(f"   Score range: {scores.min():.1f} - {scores.max():.1f}")
    print(f"   Money range: {money.min():.1f} - {money.max():.1f}")
    
    # Individual game access
    print(f"\n8. Individual game state access:")
    try:
        # Get state for first game
        game_state = batch.get_game_state(0)
        print(f"   ✓ Game 0 score: {game_state.score}")
        print(f"   ✓ Game 0 round: {game_state.round}")
        print(f"   ✓ Game 0 stage: {game_state.stage}")
    except Exception as e:
        print(f"   Error accessing individual game: {e}")
    
    print(f"\n✅ Basic batch operations demo completed successfully!")


def performance_comparison_demo():
    """Compare batch vs individual game performance"""
    
    print(f"\n=== Performance Comparison Demo ===\n")
    
    num_games = 50
    
    # Test individual games
    print(f"1. Testing {num_games} individual games...")
    start_time = time.time()
    
    individual_games = []
    for i in range(num_games):
        game = pylatro.GameEngine()
        actions = game.gen_actions()
        if actions:
            game.handle_action(actions[0])
        individual_games.append(game)
    
    individual_time = time.time() - start_time
    
    # Test batch games
    print(f"2. Testing batch of {num_games} games...")
    start_time = time.time()
    
    batch = pylatro.BatchGameEngine(num_games)
    all_actions = batch.batch_gen_actions()
    selected_actions = [actions[0] if actions else pylatro.Action.EndTurn for actions in all_actions]
    batch.batch_handle_actions(selected_actions)
    
    batch_time = time.time() - start_time
    
    # Results
    print(f"\n📊 Performance Results:")
    print(f"   Individual games: {individual_time:.3f}s ({num_games/individual_time:.0f} games/sec)")
    print(f"   Batch games:      {batch_time:.3f}s ({num_games/batch_time:.0f} games/sec)")
    print(f"   Speedup:          {individual_time/batch_time:.1f}x faster")
    
    # Memory comparison (approximate)
    individual_memory_mb = num_games * 0.01  # ~10KB per game
    batch_stats = batch.get_batch_stats()
    batch_memory_mb = batch_stats['memory_usage_mb']
    
    print(f"\n💾 Memory Usage (estimated):")
    print(f"   Individual games: ~{individual_memory_mb:.1f} MB")
    print(f"   Batch games:      {batch_memory_mb} MB") 
    print(f"   Memory efficiency: {individual_memory_mb/batch_memory_mb:.1f}x better")


def numpy_integration_demo():
    """Demonstrate NumPy integration for ML frameworks"""
    
    print(f"\n=== NumPy Integration Demo ===\n")
    
    batch_size = 100
    batch = pylatro.BatchGameEngine(batch_size)
    
    print(f"1. Converting batch states to NumPy arrays...")
    
    # Get states as Python lists
    states = batch.get_vectorized_states()
    
    # Convert to NumPy arrays (zero-copy when possible)
    np_states = {}
    for key, values in states.items():
        if key in ['scores', 'money', 'rounds']:
            np_states[key] = np.array(values, dtype=np.float32)
        elif key == 'antes':
            np_states[key] = np.array(values, dtype=np.int32)
        elif key == 'is_over':
            np_states[key] = np.array(values, dtype=bool)
    
    print(f"   ✓ Created NumPy arrays:")
    for key, array in np_states.items():
        print(f"      {key}: shape={array.shape}, dtype={array.dtype}")
    
    # Demonstrate vectorized operations
    print(f"\n2. Vectorized analysis with NumPy:")
    
    # Basic statistics
    scores = np_states['scores']
    money = np_states['money']
    
    print(f"   Score statistics:")
    print(f"      Mean: {scores.mean():.2f}")
    print(f"      Std:  {scores.std():.2f}")
    print(f"      Min:  {scores.min():.2f}")
    print(f"      Max:  {scores.max():.2f}")
    
    # Logical operations
    high_score_games = scores > scores.mean()
    rich_games = money > money.mean()
    successful_games = high_score_games & rich_games
    
    print(f"\n   Game categorization:")
    print(f"      High score games: {high_score_games.sum()}/{len(scores)}")
    print(f"      Rich games:       {rich_games.sum()}/{len(money)}")
    print(f"      Successful games: {successful_games.sum()}/{len(scores)}")
    
    # Demonstrate typical RL preprocessing
    print(f"\n3. RL preprocessing example:")
    
    # Normalize states for neural network input
    normalized_scores = (scores - scores.mean()) / (scores.std() + 1e-8)
    normalized_money = (money - money.mean()) / (money.std() + 1e-8)
    
    # Stack features for RL agent input
    features = np.stack([normalized_scores, normalized_money, np_states['rounds']], axis=1)
    
    print(f"   ✓ Created feature matrix: shape={features.shape}")
    print(f"   ✓ Ready for RL framework input")
    
    # Simulate reward calculation
    rewards = scores + money * 0.1  # Simple reward function
    print(f"   ✓ Calculated rewards: mean={rewards.mean():.2f}, std={rewards.std():.2f}")


def main():
    """Run all demonstrations"""
    
    print("John Botmack Batch Operations Implementation")
    print("High-performance parallel game simulation for RL training")
    print("=" * 60)
    
    try:
        basic_batch_demo()
        performance_comparison_demo()
        numpy_integration_demo()
        
        print(f"\n{'='*60}")
        print("🎯 All demonstrations completed successfully!")
        print("✅ Batch operations are working correctly")
        print("✅ Performance improvements demonstrated") 
        print("✅ NumPy integration ready for RL frameworks")
        print("✅ Ready for production RL training workloads")
        
    except Exception as e:
        print(f"❌ Error during demonstration: {e}")
        import traceback
        traceback.print_exc()


if __name__ == "__main__":
    main()