# Testing Framework Salvage Plan from PR #779

## Overview
Salvaging valuable testing infrastructure from closed PR #779 (~4,920 lines of testing code) across 4 focused PRs for Issue #907.

## Component Analysis from PR #779

### Tier 1 - Critical Components (MUST SAVE)
1. **Property-Based Testing** (~465 lines)
   - Proptest integration for comprehensive validation
   - Arbitrary trait implementations for domain types
   - Property generators for game states

2. **Mock Framework** (~406 lines)
   - 8+ mock types using mockall
   - Repository, StateNotifier, MetricsCollector mocks
   - Deterministic testing helpers

3. **Test Fixtures** (~358 lines)
   - Domain entity factories
   - Edge case scenario generators
   - Performance test data sets

### Tier 2 - High Value Components
1. **Performance Testing** (~621 lines)
   - Concurrent session testing
   - Memory monitoring utilities
   - Load testing framework

2. **Domain Assertions** (~440 lines)
   - Business rule validation helpers
   - Game state assertions
   - Action validation utilities

## 4-Day Implementation Plan

### Day 1: Core Testing Infrastructure (4-6 hours)
**Goal**: Establish foundational testing utilities (~1,200 lines)

**Files to Create**:
- `core/tests/common/mod.rs` - Module exports
- `core/tests/common/fixtures.rs` - Test data factories
- `core/tests/common/assertions.rs` - Domain assertions
- `core/tests/common/helpers.rs` - Utility functions

**Key Components**:
- Game state fixtures
- Card and hand generators
- Action builders
- Basic validation assertions
- Test scenario helpers

**Integration Example**:
- Create `core/tests/test_infrastructure_demo.rs` to showcase usage

### Day 2: Advanced Testing (4-6 hours)
**Goal**: Add property-based and performance testing (~1,100 lines)

**Files to Create**:
- `core/tests/common/properties.rs` - Property generators
- `core/tests/common/performance.rs` - Performance utilities
- `core/tests/common/statistical.rs` - Statistical test helpers

**Key Components**:
- Proptest arbitrary implementations
- Performance metrics collection
- Memory monitoring
- Load test configurations
- Statistical distribution tests

**Dependencies to Add**:
```toml
[dev-dependencies]
proptest = "1.0"
criterion = "0.5"
```

### Day 3: Mock Framework (3-4 hours)
**Goal**: Implement comprehensive mocking (~600 lines)

**Files to Create**:
- `core/tests/common/mocks.rs` - Mock implementations
- `core/tests/common/builders.rs` - Mock builders

**Key Components**:
- Game repository mocks
- RNG mocks for deterministic testing
- Joker factory mocks
- Session manager mocks
- Performance profiler mocks

**Dependencies to Add**:
```toml
[dev-dependencies]
mockall = "0.11"
```

### Day 4: CI/CD Enhancement (3-4 hours)
**Goal**: Enhance CI pipeline with quality gates (~200 lines + config)

**Files to Modify**:
- `.github/workflows/ci.yml` - Already enhanced with timeouts
- `deny.toml` - Dependency checks (if not exists)
- `codecov.yml` - Coverage configuration

**Key Components**:
- Coverage enforcement (90% target)
- Performance regression detection
- Test retry mechanism (already added)
- Memory leak detection
- Security scanning

## Migration Strategy

### Phase 1: Core Infrastructure (Day 1)
1. Extract fixtures from web-debug-ui to core
2. Adapt for balatro-rs types
3. Remove web-specific dependencies
4. Add comprehensive test coverage

### Phase 2: Advanced Features (Day 2)
1. Port property generators
2. Implement performance utilities
3. Add statistical helpers
4. Create benchmark examples

### Phase 3: Mocking (Day 3)
1. Define mock traits
2. Implement mock builders
3. Create deterministic test helpers
4. Add mock usage examples

### Phase 4: CI/CD (Day 4)
1. Enhance workflow configuration
2. Add quality gates
3. Implement coverage tracking
4. Add performance baselines

## Success Criteria

### Day 1 Success:
- [ ] Core test infrastructure compiles
- [ ] Fixtures cover all major game types
- [ ] Assertions work for domain validation
- [ ] Demo test passes

### Day 2 Success:
- [ ] Property tests generate valid inputs
- [ ] Performance utilities measure correctly
- [ ] Statistical tests validate distributions

### Day 3 Success:
- [ ] Mock framework integrates cleanly
- [ ] Deterministic testing works
- [ ] Mock builders simplify test setup

### Day 4 Success:
- [ ] CI pipeline runs with new features
- [ ] Coverage reporting works
- [ ] Quality gates enforce standards

## Risk Mitigation

1. **Dependency Conflicts**: Use exact versions, test compatibility
2. **Performance Impact**: Keep test utilities in dev-dependencies
3. **Breaking Changes**: Maintain backward compatibility
4. **CI Complexity**: Add timeouts and retry mechanisms (done)

## Benefits

1. **Improved Test Quality**: Comprehensive testing patterns
2. **Faster Development**: Reusable test utilities
3. **Better Coverage**: Property-based testing finds edge cases
4. **Performance Validation**: Catch regressions early
5. **Deterministic Testing**: Reproducible test failures

## Timeline

- **Day 1 (Today)**: Core infrastructure + fixtures
- **Day 2 (Tomorrow)**: Advanced testing features
- **Day 3**: Mock framework
- **Day 4**: CI/CD enhancements

## Notes

- Focus on balatro-rs core library, not web-debug-ui
- Maintain compatibility with existing tests
- Document all new utilities thoroughly
- Create examples for each component
