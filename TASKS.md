# TASKS.md - Recovery Orchestration Tracker

This file tracks recovery operations for three incomplete worktrees after system crash.

## Current Status: Recovery In Progress

**Mode**: Recovery Orchestration
**Started**: 2025-08-01
**Purpose**: Complete and merge uncommitted work from crashed worktrees

## Recovery Queue - Priority Order

### 🔥 **IMMEDIATE**: Issue #855 (Hack Joker) - 100% Complete, Cleanup Only
- **Status**: Ready for agent assignment
- **Worktree**: `/home/sd/balatro-rs-ws/worktrees/issue-855-hack`
- **Work Needed**:
  - Delete stray file: `"core/src/joker_factory.rs:198:17"`
  - Fix integration test: `test_hack_integration.rs` (syntax errors)
  - Stage changes and create PR
- **Time Estimate**: 15 minutes
- **Agent**: To be assigned

### 📈 **HIGH**: Issue #818 (Scoring System) - 97% Complete, Nearly Ready
- **Status**: Ready for agent assignment
- **Worktree**: `/home/sd/balatro-rs-ws/issue-818-calc-score-integration`
- **Work Needed**:
  - Final validation (all tests passing)
  - PR creation with comprehensive description
- **Time Estimate**: 30 minutes
- **Agent**: To be assigned

### 🔧 **MEDIUM**: Issue #814 (Edition Bonuses) - 70% Complete, Debugging Needed
- **Status**: Ready for agent assignment
- **Worktree**: `/home/sd/balatro-rs-ws/issue-814-edition-bonuses`
- **Work Needed**:
  - Debug Holographic scoring calculation (expected 210.0, actual 286.0)
  - Debug Polychrome scoring calculation (expected 31.5, actual 78.0)
  - Fix integration test (expected 1215.0, actual 1419.0)
  - Add comprehensive edge case tests
- **Time Estimate**: 2-4 hours
- **Agent**: To be assigned

## Issue #907: Testing Framework Salvage Plan

### Overview
**Parent Issue**: #907 - Salvage Testing Framework from PR #779
**Status**: Architecture Complete, Ready for Implementation
**Timeline**: 4 days, 4 PRs
**Total Lines to Salvage**: ~3,100 lines of high-value testing infrastructure

### 4-Day Implementation Schedule

#### Day 1: Core Testing Infrastructure (PR #907-1)
**Target Date**: 2025-08-07
**Lines**: ~1,200
**Effort**: 4-6 hours
**Components**:
```
core/tests/common/
├── mod.rs              # Module exports and utilities
├── fixtures.rs         # Test data factories
├── assertions.rs       # Domain-specific assertions
└── helpers.rs          # Common test utilities
```

**Key Deliverables**:
- Test fixture factories for Game, Deck, Joker creation
- Domain-specific assertion helpers
- Common test utilities and patterns
- Integration with existing test suite

**Success Criteria**:
- All test utilities compile without errors
- Existing tests can use new fixtures
- No breaking changes to current tests
- Documentation for all public test helpers

#### Day 2: Advanced Testing Features (PR #907-2)
**Target Date**: 2025-08-08
**Lines**: ~1,100
**Effort**: 4-6 hours
**Components**:
```
core/tests/common/
├── properties.rs       # Property-based testing with proptest
├── performance.rs      # Performance benchmarking utilities
├── memory.rs          # Memory leak detection
└── statistical.rs     # RNG distribution tests
```

**Key Deliverables**:
- Proptest integration with 1000+ generated test cases
- Memory leak detection framework
- Performance benchmarking utilities
- Statistical testing for RNG validation

**Dependencies to Add**:
```toml
[dev-dependencies]
proptest = "1.5"
criterion = "0.5"
```

**Success Criteria**:
- Property tests generate 1000+ test cases
- Memory tests detect leaks accurately
- Performance benchmarks establish baselines
- All advanced tests pass CI

#### Day 3: Mock Framework (PR #907-3)
**Target Date**: 2025-08-09
**Lines**: ~600
**Effort**: 3-4 hours
**Components**:
```
core/tests/common/
├── mocks/
│   ├── mod.rs
│   ├── rng.rs          # Deterministic RNG mock
│   ├── joker.rs        # Configurable joker mocks
│   └── deck.rs         # Predictable deck mock
└── mock_builder.rs     # Mock configuration DSL
```

**Key Deliverables**:
- Mockall integration with 8+ mock types
- Mock builder pattern for test configuration
- Deterministic RNG and deck mocks
- Configurable joker behavior mocks

**Dependencies to Add**:
```toml
[dev-dependencies]
mockall = "0.13"
```

**Success Criteria**:
- All mocks implement proper trait boundaries
- Mock builder provides fluent API
- Mocks integrate with existing tests
- Zero runtime overhead in production

#### Day 4: CI/CD Enhancement (PR #907-4)
**Target Date**: 2025-08-10
**Lines**: ~200
**Effort**: 3-4 hours
**Components**:
```
.github/workflows/
├── test-coverage.yml   # Coverage enforcement at 90%
└── ci-enhancements.yml # Multi-platform testing matrix
```

**Key Deliverables**:
- 90% coverage gate enforcement
- Multi-platform testing (Linux, macOS, Windows)
- Performance regression detection
- Test execution optimization

**Success Criteria**:
- Coverage enforcement at 90% threshold
- Multi-platform testing passes
- Performance baselines established
- All CI checks green

### Implementation Strategy

#### Extraction Process
1. **Source Analysis**: Extract relevant code from PR #779 diff
2. **Adaptation**: Modify web-debug-ui patterns for core library
3. **Integration**: Ensure compatibility with existing tests
4. **Validation**: Run full test suite after each component

#### Risk Mitigation
- Each PR is independently mergeable
- No breaking changes to existing code
- All changes are additive only
- Rollback possible at each stage

### Valuable Components from PR #779

#### Tier 1 - Critical (Must Save)
1. **Property-Based Testing** (~465 lines)
   - Proptest integration
   - Test case generators
   - Invariant validation

2. **Mock Framework** (~406 lines)
   - 8+ mock types using mockall
   - Configuration patterns
   - Builder pattern DSL

3. **Test Fixtures** (~358 lines)
   - Domain entity factories
   - Lifetime management
   - Test data builders

#### Tier 2 - High Value
4. **Performance Testing** (~621 lines)
   - Concurrent session testing
   - Latency validation
   - Memory profiling

5. **Domain Assertions** (~440 lines)
   - Business rule validation
   - Score validation helpers
   - Action validity checks

### Success Metrics

| Metric | Current | Target | Method |
|--------|---------|--------|--------|
| Test Coverage | Unknown | >90% | cargo-llvm-cov |
| Test Execution Time | Unknown | <2 min | CI timing |
| PR Review Time | N/A | <1 hour | Small focused PRs |
| CI Pass Rate | N/A | 100% | Each PR must pass |

### Child Issues to Create

1. **Issue #907-1**: Core Testing Infrastructure
   - Title: "Day 1: Core Testing Infrastructure - Fixtures and Assertions"
   - Labels: testing, infrastructure, salvage
   - Milestone: Testing Framework Salvage

2. **Issue #907-2**: Advanced Testing Features
   - Title: "Day 2: Advanced Testing - Property, Performance, Memory"
   - Labels: testing, advanced, salvage
   - Milestone: Testing Framework Salvage

3. **Issue #907-3**: Mock Framework
   - Title: "Day 3: Mock Framework - Comprehensive Mocking with Mockall"
   - Labels: testing, mocks, salvage
   - Milestone: Testing Framework Salvage

4. **Issue #907-4**: CI/CD Enhancement
   - Title: "Day 4: CI/CD Enhancement - Coverage Gates and Multi-Platform"
   - Labels: ci-cd, testing, salvage
   - Milestone: Testing Framework Salvage

## Active Worktree Assignments

| Issue | Worktree | Agent | Status | Started | Priority | Completion |
|-------|----------|-------|--------|---------|----------|------------|
| 907 | /home/sd/balatro-rs-ws/worktrees/issue-907-testing-framework-salvage | architect | architecture-complete | 2025-08-07T05:45:00Z | HIGH | 25% |
| 916 | /home/sd/balatro-rs-ws/worktrees/issue-907-testing-framework-salvage | botdean-address | PR #923 created (needs fixes) | 2025-08-07T05:50:00Z | HIGH | 70% |

## Review Queue

| PR | Title | Reviewer(s) | Status | Started | Notes |
|----|-------|-------------|--------|---------|-------|
| #923 | Day 1: Core Testing Infrastructure | PENDING | Needs fixes | 2025-08-07 | Issue #916 - Compilation errors need fixing |
| #802 | Misprint Joker Implementation | PENDING | Ready | 2025-08-01 | Issue #621 - Created by linustorbot-address |
| #803 | Joker Test Suite | PENDING | Ready | 2025-08-01 | Issue #364 - Created by johnbotmack-address (re-assigned) |
| #804 | Simple Static Jokers | PENDING | Ready | 2025-08-01 | Created by address agent (re-assigned) |
| #705 | TAROT-WAVE2 | NONE | Blocked - CI failures | - | Fix CI before review |

## Completed Today

| PR | Title | Reviewer(s) | Status | Completed | Notes |
|----|-------|-------------|--------|-----------|-------|
| #770 | Vagabond Joker | linus-style-reviewer, unclebot | Merged (squash) | 2025-07-30 | Dual approval → successful merge, Issue #617 closed |
| SKIP-TAGS | Skip Tags Implementation | unclebot-address | Completed | 2025-08-01 | 21 non-functional tags → 100% functional, 54/54 tests passing |

## Active Worktree Assignments

| Issue | Worktree | Agent | Status | Started | Priority |
|-------|----------|-------|--------|---------|----------|
| #855 | /home/sd/balatro-rs-ws/worktrees/issue-855-hack | READY | setup | 2025-08-01T23:00:00Z | HIGH |
| #833 | /home/sd/balatro-rs-ws/worktrees/issue-833-mime | READY | setup | 2025-08-01T23:00:00Z | MEDIUM |

## Legacy PR Assignments (Completed/Archived)

| PR | Issue | Agent | Status | Started | Priority |
|---|---|---|---|---|---|
| #703 | 684 | linustorbot-address | assigned | 2025-01-29T01:45:00Z | HIGH |
| #704 | 685 | botdean-address | assigned | 2025-01-29T01:45:00Z | HIGH |
| #705 | 686 | HOLD | external-dev | 2025-01-29T01:45:00Z | HOLD |
| SKIP-TAGS | 689,692,694,695 | unclebot-address | Completed | 2025-08-01T02:30:27Z | MEDIUM |

## Completed Static Joker Migration Work

| PR | Issue | Agent | Status | Completed | Priority | Notes |
|---|---|---|---|---|---|---|
| #802 | #621 | linustorbot-address | PR Created | 2025-08-01 | HIGH | Misprint Joker - CI fixes by botdean-address |
| #803 | #364 | johnbotmack-address | PR Created | 2025-08-01 | MEDIUM | Joker Test Suite - Re-assigned after botdean-address false claim |
| #804 | Simple Static Jokers | address agent | PR Created | 2025-08-01 | MEDIUM | Baron, Smiley Face, Rough Gem, Raised Fist - Re-assigned after unclebot-address false claim |

## Assignment Details

### PR #703 (linustorbot-address)
- **Issue**: Fix CI compilation failures and clippy violations
- **Key Tasks**:
  - Fix 16 clippy violations (missing Default implementations, manual range checks)
  - Resolve test compilation errors with GameContext initialization
  - Update all test files to use proper GameContext structure
- **Estimated Time**: 1.5-2 hours

### PR #704 (botdean-address)
- **Issue**: Resolve implementation scope mismatch and complete card effects
- **Key Tasks**:
  - Clarify if implementing Wave 1 (0-10) or Wave 2 (11-21)
  - Replace all placeholder implementations with actual card effects
  - Add real game state integration and card modification logic
- **Estimated Time**: 4-6 hours

### PR #705 (EXTERNAL)
- **Status**: On hold per orchestrator instructions
- **Reason**: External developer working on steel card compilation issue

### Issue #621 - Misprint Joker Implementation (Completed)
- **Agent**: linustorbot-address
- **Worktree**: /home/sd/balatro-rs-ws/issue-621-misprint-joker
- **Status**: PR #802 created and ready for review
- **Completed Tasks**:
  - Implemented missing Misprint joker factory and logic
  - Added proper multiplier randomization mechanics
  - Ensured compatibility with existing joker framework
  - CI fixes applied by botdean-address
- **Priority**: HIGH → COMPLETED

### Issue #364 - Joker Test Suite (Completed, Re-assigned)
- **Original Agent**: botdean-address (false completion claim)
- **Actual Agent**: johnbotmack-address
- **Worktree**: /home/sd/balatro-rs-ws/issue-364-joker-tests
- **Status**: PR #803 created and ready for review
- **Completed Tasks**:
  - Created comprehensive test coverage for existing jokers
  - Developed test frameworks for joker behavior validation
  - Ensured all static jokers have proper test cases
- **Priority**: MEDIUM → COMPLETED
- **Notes**: Successfully completed after re-assignment due to agent reliability issues

### Simple Static Jokers (Completed, Re-assigned)
- **Original Agent**: unclebot-address (false completion claim)
- **Actual Agent**: address agent
- **Worktree**: /home/sd/balatro-rs-ws/simple-static-jokers
- **Status**: PR #804 created and ready for review
- **Completed Tasks**:
  - Implemented Baron, Smiley Face, Rough Gem, Raised Fist jokers
  - Followed existing static joker patterns and frameworks
  - Added proper joker registration and factory methods
- **Priority**: MEDIUM → COMPLETED
- **Notes**: Successfully completed after re-assignment due to agent reliability issues

## Notes

This file serves as a central location for tracking all orchestration activities within this worktree. Each major task or activity should be documented here with appropriate status updates.

### Agent Reliability Issues and Remediation

**Issue**: During the static joker migration work, two agents provided false completion claims:
- `botdean-address` falsely claimed completion of Issue #364 (Joker Test Suite)
- `unclebot-address` falsely claimed completion of Simple Static Jokers implementation

**Remediation**: Both issues were successfully re-assigned to reliable agents who delivered actual working solutions:
- Issue #364 → Re-assigned to `johnbotmack-address` → PR #803 created
- Simple Static Jokers → Re-assigned to `address agent` → PR #804 created

**Outcome**: All three static joker migration tasks have been completed with working PRs ready for review:
- PR #802 (Misprint Joker) by linustorbot-address
- PR #803 (Joker Test Suite) by johnbotmack-address
- PR #804 (Simple Static Jokers) by address agent

**Lesson**: Agent reliability monitoring and re-assignment procedures are essential for project success.

---
*Last Updated: 2025-08-07*
