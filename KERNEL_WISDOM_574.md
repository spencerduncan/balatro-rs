## LINUSTORBOT_KERNEL_WISDOM.md - PR #574
**Date**: 2025-07-23
**Component**: consumables/JokerTarget implementation

### Anti-Patterns Found (Learn From Others' Mistakes)
None - This code actually follows best practices.

### Good Patterns (Rare But Worth Noting)
**Validation Pattern**:
- Pattern: Single validation method that all other methods call
- Why It Works: Prevents inconsistent validation, ensures bounds checks happen
- Correct Implementation: validate() checks everything, get_joker() just calls it

**Error Design**:
- Pattern: Typed errors with structured data
- Why It Works: Makes debugging at 3 AM possible
- Implementation: JokerTargetError enum with specific failure reasons

**Bounds Checking**:
- Pattern: Check array bounds BEFORE access
- Why It Works: Prevents panics and segfaults
- Implementation: if self.slot >= game.jokers.len() BEFORE game.jokers[self.slot]

### Educational Success
- Developer understands: Proper error handling
- Developer understands: Validation patterns
- Developer understands: Test-driven development (22 tests)

### Minor Issues
- Misleading method name: active_joker_at_slot() doesn't create active target
- TODO stub: is_joker_active() always returns true
- Both acknowledged and not critical

### Production Readiness
- MTTR Impact: Error messages will reduce debug time
- Monitoring: Errors ready for telemetry integration
- Test Coverage: Comprehensive edge case testing
