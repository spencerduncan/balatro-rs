# LINUSTORBOT_KERNEL_WISDOM.md - PR #712 Review

## Economic Tags Implementation - Critical Failures

### Date: 2025-07-31
### Component: Skip Tags System

### Anti-Patterns Found
1. **TODO-Driven Development**: Investment tag has TODO instead of implementation
2. **Hardcoded Values**: Speed tag uses 'blinds_skipped = 1' instead of actual count
3. **Disconnected State**: blinds_skipped counter exists but never incremented
4. **Type Confusion**: Unnecessary f64->i64->f64 casting for money
5. **Wrong Logic**: Garbage tag uses plays as discards available

### Missing Tests
- Zero test coverage for any economic tag
- No integration tests
- No edge case handling

### Review Verdict
**REJECTED** - Non-functional implementation with critical bugs
**Follow-up Issue**: #800 created for technical debt
