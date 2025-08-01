# Data-Oriented Game Module Refactoring Plan

## Executive Summary

This plan addresses the 70.97% performance regression from Phase 1's delegation patterns while incorporating critical feedback:
- **Phase 1 Failure**: Delegation patterns in hot paths caused severe performance degradation
- **Linus Principle**: No abstractions in hot paths, use data-oriented design, respect cache lines
- **Uncle Bob Adaptation**: Apply clean code principles through compile-time organization, not runtime abstractions

Our approach: Zero-overhead data-oriented design with compile-time module organization for maintainability.

## Core Design Principles

### 1. Data-Oriented Memory Layout
- Group data by access patterns, not conceptual boundaries
- Align frequently accessed data to cache lines (64 bytes)
- Separate hot data (accessed every frame) from cold data (accessed rarely)
- Use structure-of-arrays (SoA) for vectorizable operations

### 2. Zero-Overhead Hot Paths
- Direct field access for all performance-critical operations
- No function calls, virtual dispatch, or indirection in hot paths
- Inline all critical path functions
- Use const generics for compile-time configuration

### 3. Compile-Time Module Organization
- Modules provide organization at compile-time only
- No runtime module boundaries or delegation
- Use Rust's visibility rules for encapsulation without overhead
- Leverage trait implementations for code organization without vtables

### 4. Cache-Aware Design
- Pack related data into 64-byte cache lines
- Avoid false sharing between threads
- Use cache-oblivious algorithms where possible
- Profile cache misses and optimize access patterns

## Performance Validation Framework

### Continuous Performance Testing
```rust
// Every commit must pass these performance gates
#[bench]
fn hot_path_regression_test(b: &mut Bencher) {
    // Baseline: current performance
    // Threshold: 0.1% regression tolerance
    // Enforcement: Automatic rollback on failure
}
```

### Measurement Points
1. **Action Generation**: Target < 10μs per call
2. **Score Calculation**: Target < 5μs per hand
3. **State Access**: Target < 100ns for hot fields
4. **Cache Miss Rate**: Target < 5% L1 misses

## Implementation Stages

### Stage 1: Hot Path Identification and Baseline (Week 1)
**Goal**: Map all performance-critical paths and establish baselines

1. **Profile Current System**
   ```bash
   cargo build --release
   perf record --call-graph=dwarf target/release/balatro-cli
   perf report
   ```

2. **Identify Hot Paths**
   - Action generation pipeline (`gen_actions`, `gen_action_space`)
   - Score calculation (`calc_score`, joker effect processing)
   - State access patterns (field reads in inner loops)
   - Memory allocation patterns

3. **Create Performance Test Suite**
   ```rust
   mod perf_tests {
       #[bench]
       fn bench_action_generation_baseline(b: &mut Bencher) {
           let game = setup_complex_game_state();
           b.iter(|| game.gen_actions());
       }
       
       #[bench]
       fn bench_score_calculation_baseline(b: &mut Bencher) {
           let mut game = setup_scoring_scenario();
           b.iter(|| game.calc_score(test_hand()));
       }
   }
   ```

4. **Document Hot Path Invariants**
   ```rust
   // HOT PATH: This function is called 1M+ times per training episode
   // REQUIREMENT: Zero allocations, < 10μs execution time
   // INVARIANT: Direct field access only, no function calls
   ```

### Stage 2: Data-Oriented Memory Layout (Week 2)
**Goal**: Restructure Game struct for cache efficiency

1. **Current Layout Analysis**
   ```rust
   // PROBLEM: Random field ordering causes cache misses
   pub struct Game {
       config: Config,        // Cold: rarely accessed
       shop: Shop,           // Cold: only during shop stage
       deck: Deck,           // Warm: accessed during play
       available: Available,  // Hot: accessed every action
       discarded: Vec<Card>, // Warm: accessed during discard
       // ... 50+ fields with no access pattern grouping
   }
   ```

2. **New Cache-Aligned Layout**
   ```rust
   #[repr(C)]
   pub struct Game {
       // Cache Line 1: Hot Path Data (64 bytes)
       available: Available,     // 32 bytes
       plays: f64,              // 8 bytes
       discards: f64,           // 8 bytes
       chips: f64,              // 8 bytes
       mult: f64,               // 8 bytes
       
       // Cache Line 2: Scoring Data (64 bytes)
       score: f64,              // 8 bytes
       reward: f64,             // 8 bytes
       money: f64,              // 8 bytes
       hand_levels: [u32; 10],  // 40 bytes (fixed-size array)
       
       // Cache Line 3: Game State (64 bytes)
       stage: Stage,            // 8 bytes
       ante_current: Ante,      // 8 bytes
       round: f64,              // 8 bytes
       _padding: [u8; 40],      // Explicit padding
       
       // Cold Data: Separate allocation
       cold_data: Box<GameColdData>,
   }
   
   struct GameColdData {
       config: Config,
       shop: Shop,
       action_history: BoundedActionHistory,
       debug_manager: DebugManager,
       persistence_manager: PersistenceManager,
   }
   ```

3. **Structure-of-Arrays for Jokers**
   ```rust
   // BEFORE: Array of structs (poor cache usage)
   jokers: Vec<Box<dyn Joker>>,
   
   // AFTER: Structure of arrays (vectorizable)
   struct JokerData {
       ids: Vec<JokerId>,           // Contiguous IDs
       chips: Vec<i32>,             // Contiguous chip values
       mults: Vec<i32>,             // Contiguous mult values
       active_flags: BitVec,        // Packed boolean flags
       custom_data: Vec<JokerState>, // Only for complex jokers
   }
   ```

### Stage 3: Zero-Overhead Access Patterns (Week 3)
**Goal**: Eliminate all function call overhead in hot paths

1. **Direct Field Access**
   ```rust
   // BEFORE: Delegation pattern (70.97% regression)
   impl Game {
       pub fn is_pack_available(&self) -> bool {
           self.pack_manager.pack_inventory().is_empty()
       }
   }
   
   // AFTER: Direct access (zero overhead)
   impl Game {
       #[inline(always)]
       pub fn is_pack_available(&self) -> bool {
           self.pack_inventory.is_empty() // Direct field
       }
   }
   ```

2. **Inline Critical Functions**
   ```rust
   // Force inlining for hot path functions
   #[inline(always)]
   pub fn gen_actions(&self) -> ActionIterator {
       // Direct field access only
       match self.stage {
           Stage::Blind(_) if self.plays > 0.0 => {
               ActionIterator::new(&self.available, self.discards)
           }
           _ => ActionIterator::empty(),
       }
   }
   ```

3. **Const Generic Configuration**
   ```rust
   // Compile-time configuration without runtime overhead
   pub struct Game<const MAX_JOKERS: usize = 5> {
       joker_data: JokerData<MAX_JOKERS>,
   }
   ```

### Stage 4: Compile-Time Module Organization (Week 4)
**Goal**: Achieve clean code organization without runtime overhead

1. **Module Structure**
   ```rust
   // Modules provide compile-time organization only
   mod game {
       mod hot {
           // Performance-critical implementations
           pub(super) fn calc_score_inner(game: &mut Game) -> f64 {
               // Direct implementation, no delegation
           }
       }
       
       mod cold {
           // Non-critical convenience functions
           pub(super) fn save_game(game: &Game) -> Result<()> {
               // Can use abstractions here
           }
       }
   }
   ```

2. **Trait-Based Organization**
   ```rust
   // Traits for organization without vtables
   trait Scoring {
       fn calc_score(&mut self, hand: MadeHand) -> f64;
   }
   
   // Direct implementation, no dynamic dispatch
   impl Scoring for Game {
       #[inline(always)]
       fn calc_score(&mut self, hand: MadeHand) -> f64 {
           // Direct field manipulation
           self.chips * self.mult
       }
   }
   ```

3. **Visibility-Based Encapsulation**
   ```rust
   pub struct Game {
       // Public for hot path access
       pub available: Available,
       pub plays: f64,
       
       // Private with getter for cold paths only
       cold_data: Box<GameColdData>,
   }
   
   impl Game {
       // Cold path can have overhead
       pub fn config(&self) -> &Config {
           &self.cold_data.config
       }
   }
   ```

## Performance Validation Criteria

### Stage Gates
Each stage must pass before proceeding:

1. **Baseline Establishment**
   - All hot paths identified and benchmarked
   - Performance test suite achieving 100% hot path coverage
   - Documentation of performance requirements

2. **Memory Layout Validation**
   - Cache miss rate reduced by >50%
   - Memory usage reduced by >20%
   - No regression in any benchmark

3. **Access Pattern Validation**
   - All hot path functions inlined (verified in assembly)
   - Zero function call overhead in critical paths
   - 90% performance recovery vs Phase 1 baseline

4. **Final Validation**
   - All benchmarks within 0.1% of pre-Phase 1 performance
   - Code organization improved (measured by module cohesion)
   - Maintainability preserved through compile-time organization

### Rollback Criteria
Automatic rollback triggered by:
- Any benchmark regression >0.1%
- Cache miss rate increase >1%
- Memory allocation in hot paths
- Function calls in critical paths (detected via assembly analysis)

## Implementation Guidelines

### Do's
- ✅ Profile before optimizing
- ✅ Measure after every change
- ✅ Use direct field access in hot paths
- ✅ Align data to cache lines
- ✅ Separate hot and cold data
- ✅ Use compile-time polymorphism
- ✅ Document performance requirements

### Don'ts
- ❌ No function calls in hot paths
- ❌ No allocations in hot paths
- ❌ No virtual dispatch in critical code
- ❌ No delegation patterns for performance-critical operations
- ❌ No assumptions about compiler optimizations
- ❌ No large-scale changes without benchmarking

## Continuous Monitoring

### Performance Dashboard
```toml
[profile.bench]
debug = true
lto = true
codegen-units = 1

[profile.perf]
inherits = "release"
debug = true  # For profiling symbols
```

### Automated Checks
```yaml
# CI Pipeline
performance-validation:
  - run: cargo bench --bench hot_paths -- --save-baseline base
  - run: cargo bench --bench hot_paths -- --baseline base
  - fail-on: regression > 0.1%
```

### Assembly Verification
```bash
# Verify inlining and direct access
cargo asm balatro_rs::game::Game::gen_actions --release
# Should show: No CALL instructions, only direct memory access
```

## Success Metrics

### Technical Metrics
- **Performance Recovery**: >95% of pre-Phase 1 performance
- **Cache Efficiency**: <5% L1 cache miss rate
- **Memory Usage**: 20% reduction through better layout
- **Maintainability**: Module cohesion score >0.8

### Process Metrics
- **Detection Time**: <5 minutes for any regression
- **Rollback Time**: <2 minutes to safe state
- **Benchmark Coverage**: 100% of hot paths
- **Documentation**: All performance requirements documented

## Risk Mitigation

### Technical Risks
1. **Compiler Behavior Changes**
   - Mitigation: Assembly verification in CI
   - Fallback: Explicit assembly for critical sections

2. **Platform Differences**
   - Mitigation: Benchmark on all target platforms
   - Fallback: Platform-specific implementations

3. **Future Rust Changes**
   - Mitigation: Pin compiler version for releases
   - Fallback: Compatibility layer for new versions

### Process Risks
1. **Scope Creep**
   - Mitigation: Strict stage gates
   - Fallback: Incremental delivery

2. **Performance Regression**
   - Mitigation: Continuous benchmarking
   - Fallback: Automatic rollback

## Conclusion

This data-oriented refactoring plan addresses the Phase 1 performance regression through:
1. **Hot path preservation** with zero-overhead access
2. **Cache-aware data layout** for optimal memory performance
3. **Compile-time organization** for maintainability without runtime cost
4. **Continuous validation** to prevent future regressions

By following these principles, we can achieve both the performance requirements of a game engine and the maintainability needs of a complex codebase.
ENDOFFILE < /dev/null
