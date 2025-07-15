## Review Lessons - PR #127
**Scope**: Static Joker Implementation
**Date**: 2025-07-15

### Positive Patterns Observed
- Excellent use of the StaticJoker builder pattern with proper method chaining
- Comprehensive unit test coverage for all implemented jokers
- Clear separation between fully implemented and placeholder jokers
- Proper use of TODO comments with explanatory context
- Consistent naming conventions following established patterns
- Good documentation with clear descriptions for each joker

### Anti-Patterns Identified
1. **Missing Factory Integration**: New jokers implemented in StaticJokerFactory but not integrated into main JokerFactory
   - **Solution**: Always ensure new jokers are added to JokerFactory::create() method
   - **Also update**: get_by_rarity() and get_all_implemented() methods

2. **Unused Imports**: Import of `crate::joker::Joker` in test module not used
   - **Solution**: Run `cargo clippy` before committing to catch unused imports

3. **Placeholder Implementation Issues**: Using `StaticCondition::Always` for unimplemented features
   - **Solution**: Consider creating dedicated placeholder conditions or clearer warnings
   - **Risk**: Unconditional bonuses could affect game balance during testing

### Review Process Insights
- Parallel analysis tasks effectively identified multiple issues simultaneously
- CI failure investigation revealed both environment issues and actual code problems
- Checking factory integration is critical for new feature implementations
- Style compliance checks should include unused import warnings

### Recommendations
1. **Pre-commit Checklist for Joker PRs**:
   - [ ] All new jokers added to JokerFactory::create()
   - [ ] Rarity lists updated in JokerFactory
   - [ ] No unused imports (run `cargo clippy`)
   - [ ] Integration tests verify factory creation
   - [ ] CI checks pass locally before push

2. **Placeholder Implementation Strategy**:
   - Create a `StaticCondition::NotImplemented` variant
   - Add runtime warnings for placeholder jokers
   - Document current behavior explicitly in code comments

3. **Testing Improvements**:
   - Add integration tests that verify JokerFactory can create all jokers
   - Test that all JokerId enum values have implementations
   - Verify rarity lists are complete and accurate

### Technical Debt Identified
- Framework lacks support for advanced conditions (hand size, discard count, joker interactions)
- No systematic way to ensure all JokerIds are implemented in factory
- Python dependency issues in test environment complicate local testing