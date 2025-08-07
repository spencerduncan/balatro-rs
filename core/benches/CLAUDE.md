# CLAUDE.md - Benchmarks

## Directory Purpose

The benchmarks directory provides comprehensive performance testing using the Criterion framework to ensure RL training efficiency remains optimal. All benchmarks are statistically rigorous and track critical performance metrics.

## Benchmark Organization

### Core Performance
- `benchmark.rs`: Main performance suite covering action generation (~10μs target), hand evaluation, and joker processing
- `state_benchmark.rs`: State management operations (~100ns cached, ~1μs fresh snapshots)
- `actionspace_benchmark.rs`: RL-critical action space generation (<10μs for complex states)

### Joker System Performance
- `trait_benchmark.rs`: New 5-trait joker architecture overhead measurements
- `trait_benchmark_minimal.rs`: Isolated trait dispatch baseline measurements
- `trait_performance.rs`: Real-world trait performance in hot paths

### Effect Processing
- `effect_processor_benchmark.rs`: Complete joker effect processing pipeline
- `effect_processor_benchmark_simple.rs`: Single joker processing patterns
- `effect_processor_benchmark_minimal.rs`: Framework overhead baseline
- `joker_effect_processor_trait_optimization.rs`: Trait-specific optimizations

### Specialized Benchmarks
- `tarot_performance.rs`: Consumable card system performance

## Performance Targets

### Critical Metrics
- **Action generation**: <10μs for complex game states
- **State snapshots**: ~100ns cached, ~1μs fresh
- **Hand evaluation**: O(n) single-pass target
- **Memory per joker**: ~1KB maximum
- **Effect processing**: <5μs per joker chain

### Regression Thresholds
- >5% performance degradation fails CI
- Memory usage increases >10% flagged
- Allocation count increases reviewed

## Benchmarking Approach

### Statistical Rigor
- Uses Criterion for confidence intervals
- Regression detection with baseline comparisons
- HTML reports generated for visualization
- Warmup iterations for JIT stabilization

### Optimization Workflow
1. Profile current implementation
2. Establish baseline measurements
3. Implement optimizations
4. Verify improvements
5. Check for regressions

## Running Benchmarks

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark
cargo bench trait_benchmark

# Generate HTML report
cargo bench -- --save-baseline master

# Compare against baseline
cargo bench -- --baseline master
```

## Key Insights

### Performance Characteristics
- Trait dispatch adds ~10ns overhead (acceptable)
- Static jokers 10x faster than dynamic
- Caching reduces evaluation time by 70-90%
- Zero-allocation patterns critical for hot paths

### Memory Patterns
- Stack allocation preferred for temporary data
- Arena allocators for batch operations
- Reference counting for shared state
- Bounded collections prevent unbounded growth

## Continuous Monitoring

- CI runs benchmarks on every PR
- Performance dashboard tracks trends
- Alerts on regression detection
- Monthly performance reviews
