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

## Active Worktree Assignments

| Issue | Worktree | Agent | Status | Started | Priority | Completion |
|-------|----------|-------|--------|---------|----------|------------|

## Review Queue

| PR | Title | Reviewer(s) | Status | Started | Notes |
|----|-------|-------------|--------|---------|-------|
| #705 | TAROT-WAVE2 | NONE | ❌ **BLOCKED** - CI failures | - | Fix CI before review |

## Completed Today

| PR | Title | Reviewer(s) | Status | Completed | Notes |
|----|-------|-------------|--------|-----------|-------|
| #770 | Vagabond Joker | linus-style-reviewer, unclebot | ✅ **MERGED** (squash) | 2025-07-30 | Dual approval → successful merge, Issue #617 closed |

## Active PR Assignments

| PR | Issue | Agent | Status | Started | Priority |
|---|---|---|---|---|---|
| #703 | 684 | linustorbot-address | assigned | 2025-01-29T01:45:00Z | HIGH |
| #704 | 685 | botdean-address | assigned | 2025-01-29T01:45:00Z | HIGH |
| #705 | 686 | HOLD | external-dev | 2025-01-29T01:45:00Z | HOLD |

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

## Notes

This file serves as a central location for tracking all orchestration activities within this worktree. Each major task or activity should be documented here with appropriate status updates.

---
*Last Updated: 2025-08-06*
