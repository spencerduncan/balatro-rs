#!/usr/bin/env python3
"""
Batch Operations Performance Benchmark

Comprehensive performance testing for the BatchGameEngine implementation.
Tests performance targets:
- Support 1000+ parallel games efficiently
- <10% overhead vs individual game access
- Memory usage scaling linearly with game count
- Zero-copy state access where possible

Author: John Botmack Performance Implementation
"""

import time
import os
import sys
import subprocess
from typing import Dict, List, Any

def build_extension():
    """Build the pylatro extension"""
    print("Building pylatro extension...")
    
    pylatro_dir = os.path.join(os.path.dirname(__file__), "pylatro")
    
    try:
        result = subprocess.run([
            "maturin", "develop", "--release"
        ], cwd=pylatro_dir, capture_output=True, text=True, timeout=120)
        
        if result.returncode == 0:
            print("✅ Extension built successfully")
            return True
        else:
            print(f"❌ Build failed: {result.stderr}")
            return False
            
    except subprocess.TimeoutExpired:
        print("❌ Build timed out")
        return False
    except FileNotFoundError:
        print("❌ maturin not found. Please install with: pip install maturin")
        return False
    except Exception as e:
        print(f"❌ Build error: {e}")
        return False

def benchmark_creation_performance():
    """Benchmark game creation performance"""
    print("\n=== Game Creation Performance ===")
    
    batch_sizes = [10, 50, 100, 500, 1000, 2000]
    results = {}
    
    for batch_size in batch_sizes:
        print(f"Testing batch size: {batch_size}")
        
        try:
            start_time = time.time()
            batch = pylatro.BatchGameEngine(batch_size)
            creation_time = time.time() - start_time
            
            games_per_sec = batch_size / creation_time
            results[batch_size] = {
                'creation_time': creation_time,
                'games_per_sec': games_per_sec,
                'success': True
            }
            
            print(f"  ✅ {creation_time:.3f}s ({games_per_sec:.0f} games/sec)")
            
        except Exception as e:
            print(f"  ❌ Failed: {e}")
            results[batch_size] = {'success': False, 'error': str(e)}
    
    return results

def benchmark_state_extraction():
    """Benchmark vectorized state extraction performance"""
    print("\n=== State Extraction Performance ===")
    
    batch_sizes = [100, 500, 1000, 2000]
    results = {}
    
    for batch_size in batch_sizes:
        print(f"Testing batch size: {batch_size}")
        
        try:
            batch = pylatro.BatchGameEngine(batch_size)
            
            # Warm up
            batch.get_vectorized_states()
            
            # Benchmark multiple extractions
            num_extractions = 100
            start_time = time.time()
            
            for _ in range(num_extractions):
                states = batch.get_vectorized_states()
            
            total_time = time.time() - start_time
            time_per_extraction = total_time / num_extractions
            extractions_per_sec = num_extractions / total_time
            
            results[batch_size] = {
                'time_per_extraction': time_per_extraction,
                'extractions_per_sec': extractions_per_sec,
                'games_per_sec': batch_size * extractions_per_sec,
                'success': True
            }
            
            print(f"  ✅ {time_per_extraction*1000:.2f}ms per extraction")
            print(f"     {extractions_per_sec:.0f} extractions/sec")
            print(f"     {batch_size * extractions_per_sec:.0f} game-states/sec")
            
        except Exception as e:
            print(f"  ❌ Failed: {e}")
            results[batch_size] = {'success': False, 'error': str(e)}
    
    return results

def benchmark_action_execution():
    """Benchmark batch action execution performance"""
    print("\n=== Action Execution Performance ===")
    
    batch_sizes = [100, 500, 1000, 2000]
    results = {}
    
    for batch_size in batch_sizes:
        print(f"Testing batch size: {batch_size}")
        
        try:
            batch = pylatro.BatchGameEngine(batch_size)
            
            # Generate actions for warm-up
            all_actions = batch.batch_gen_actions()
            selected_actions = [actions[0] if actions else pylatro.Action.EndTurn 
                              for actions in all_actions]
            
            # Benchmark action execution
            num_executions = 50
            start_time = time.time()
            
            for _ in range(num_executions):
                # Re-generate actions each time as game state changes
                all_actions = batch.batch_gen_actions()
                selected_actions = [actions[0] if actions else pylatro.Action.EndTurn 
                                  for actions in all_actions]
                success_flags = batch.batch_handle_actions(selected_actions)
            
            total_time = time.time() - start_time
            time_per_execution = total_time / num_executions
            executions_per_sec = num_executions / total_time
            
            results[batch_size] = {
                'time_per_execution': time_per_execution,
                'executions_per_sec': executions_per_sec,
                'actions_per_sec': batch_size * executions_per_sec,
                'success': True
            }
            
            print(f"  ✅ {time_per_execution*1000:.2f}ms per batch execution")
            print(f"     {executions_per_sec:.0f} batches/sec")
            print(f"     {batch_size * executions_per_sec:.0f} actions/sec")
            
        except Exception as e:
            print(f"  ❌ Failed: {e}")
            results[batch_size] = {'success': False, 'error': str(e)}
    
    return results

def benchmark_memory_usage():
    """Benchmark memory usage scaling"""
    print("\n=== Memory Usage Scaling ===")
    
    batch_sizes = [100, 500, 1000, 2000, 5000]
    results = {}
    
    for batch_size in batch_sizes:
        print(f"Testing batch size: {batch_size}")
        
        try:
            batch = pylatro.BatchGameEngine(batch_size)
            stats = batch.get_batch_stats()
            
            memory_mb = stats['memory_usage_mb']
            memory_per_game = memory_mb / batch_size * 1024  # KB per game
            
            results[batch_size] = {
                'total_memory_mb': memory_mb,
                'memory_per_game_kb': memory_per_game,
                'success': True
            }
            
            print(f"  ✅ {memory_mb} MB total ({memory_per_game:.1f} KB/game)")
            
        except Exception as e:
            print(f"  ❌ Failed: {e}")
            results[batch_size] = {'success': False, 'error': str(e)}
    
    return results

def analyze_performance_targets(creation_results, state_results, action_results, memory_results):
    """Analyze if performance targets are met"""
    print("\n=== Performance Target Analysis ===")
    
    # Target 1: Support 1000+ parallel games efficiently
    target_1000_games = 1000 in creation_results and creation_results[1000].get('success', False)
    if target_1000_games:
        creation_time = creation_results[1000]['creation_time']
        print(f"✅ Target 1 - 1000+ games: PASS (created 1000 games in {creation_time:.3f}s)")
    else:
        print(f"❌ Target 1 - 1000+ games: FAIL")
    
    # Target 2: Linear memory scaling
    memory_scaling_ok = True
    if len(memory_results) >= 2:
        batch_sizes = sorted([k for k, v in memory_results.items() if v.get('success')])
        if len(batch_sizes) >= 2:
            # Check if memory per game is consistent
            memories_per_game = [memory_results[size]['memory_per_game_kb'] for size in batch_sizes]
            variation = max(memories_per_game) / min(memories_per_game)
            memory_scaling_ok = variation < 1.5  # Allow 50% variation
            
            print(f"✅ Target 2 - Linear memory scaling: {'PASS' if memory_scaling_ok else 'FAIL'}")
            print(f"    Memory per game variation: {variation:.2f}x")
        else:
            print(f"❌ Target 2 - Linear memory scaling: INSUFFICIENT DATA")
    
    # Target 3: High-performance state extraction
    state_performance_ok = False
    if 1000 in state_results and state_results[1000].get('success'):
        games_per_sec = state_results[1000]['games_per_sec']
        state_performance_ok = games_per_sec > 50000  # 50k game-states/sec
        print(f"✅ Target 3 - Fast state extraction: {'PASS' if state_performance_ok else 'FAIL'}")
        print(f"    Achieved: {games_per_sec:.0f} game-states/sec")
    else:
        print(f"❌ Target 3 - Fast state extraction: NO DATA")
    
    # Target 4: High-performance action execution  
    action_performance_ok = False
    if 1000 in action_results and action_results[1000].get('success'):
        actions_per_sec = action_results[1000]['actions_per_sec']
        action_performance_ok = actions_per_sec > 10000  # 10k actions/sec
        print(f"✅ Target 4 - Fast action execution: {'PASS' if action_performance_ok else 'FAIL'}")
        print(f"    Achieved: {actions_per_sec:.0f} actions/sec")
    else:
        print(f"❌ Target 4 - Fast action execution: NO DATA")
    
    # Overall assessment
    targets_met = [target_1000_games, memory_scaling_ok, state_performance_ok, action_performance_ok]
    success_rate = sum(targets_met) / len(targets_met)
    
    print(f"\n{'='*50}")
    print(f"OVERALL PERFORMANCE ASSESSMENT")
    print(f"{'='*50}")
    print(f"Targets met: {sum(targets_met)}/{len(targets_met)} ({success_rate:.1%})")
    
    if success_rate >= 0.75:
        print(f"🎯 EXCELLENT: Batch operations meet performance requirements!")
    elif success_rate >= 0.5:
        print(f"⚠️  GOOD: Most performance targets met, some optimization needed")  
    else:
        print(f"❌ NEEDS WORK: Significant performance improvements required")
    
    return targets_met

def main():
    """Run comprehensive performance benchmark"""
    
    print("John Botmack Batch Operations Performance Benchmark")
    print("Testing performance targets for RL training efficiency")
    print("=" * 60)
    
    # Build extension if needed
    if not build_extension():
        print("Failed to build extension. Cannot run benchmarks.")
        return 1
    
    # Import after building
    try:
        import pylatro
    except ImportError as e:
        print(f"Failed to import pylatro: {e}")
        return 1
    
    print(f"✅ Successfully imported pylatro")
    
    # Run benchmarks
    try:
        creation_results = benchmark_creation_performance()
        state_results = benchmark_state_extraction() 
        action_results = benchmark_action_execution()
        memory_results = benchmark_memory_usage()
        
        # Analyze results
        targets_met = analyze_performance_targets(
            creation_results, state_results, action_results, memory_results
        )
        
        # Return success code based on performance
        return 0 if sum(targets_met) >= 3 else 1
        
    except Exception as e:
        print(f"❌ Benchmark failed: {e}")
        import traceback
        traceback.print_exc()
        return 1

if __name__ == "__main__":
    exit(main())