# CLAUDE.md - Examples Directory

## Directory Purpose

The examples directory contains demonstration and reference implementations showcasing the joker system's capabilities and best practices. It serves as both documentation and validation for the joker framework.

## Key Files

### joker_definitions.toml
Comprehensive TOML schema demonstration (496 lines):
- Schema version 1.0.0 format
- Shows all joker definition patterns:
  - Basic scoring jokers
  - Conditional jokers
  - Dynamic stateful jokers
  - Money-based jokers
  - Multiplicative jokers
  - Retrigger jokers
  - Legendary jokers
- Includes complex conditions:
  - Suit-based conditions
  - Rank-based conditions
  - Composite conditions
- Demonstrates state management and behavior lifecycle patterns

## TOML Schema Structure

### Basic Joker Definition
```toml
[[jokers]]
id = "joker"
name = "Joker"
description = "{C:mult}+4{} Mult"
rarity = "common"
cost = 2

[jokers.behavior]
type = "additive_mult"
mult = 4
```

### Conditional Joker
```toml
[[jokers]]
id = "greedy_joker"
name = "Greedy Joker"
description = "{C:mult}+3{} Mult for each {C:diamond}Diamond{} scored"
rarity = "common"
cost = 3

[jokers.behavior]
type = "per_card"
mult_per_card = 3

[jokers.behavior.condition]
type = "suit"
suit = "diamonds"
```

### Stateful Joker
```toml
[[jokers]]
id = "spare_trousers"
name = "Spare Trousers"
description = "Gains {C:mult}+1{} Mult if hand contains {C:attention}Two Pair{}"
rarity = "uncommon"
cost = 5

[jokers.behavior]
type = "scaling_mult"
mult_per_trigger = 1
max_mult = 100

[jokers.behavior.condition]
type = "hand_type"
hand_type = "two_pair"
```

## Usage Patterns

### Schema Validation
- Version field ensures compatibility
- Clear separation of metadata and behavior
- Extensible condition system
- Type-safe effect definitions

### Joker Categories Demonstrated
1. **Basic Scoring**: Simple additive/multiplicative effects
2. **Conditional**: Effects that require specific conditions
3. **Dynamic**: Jokers that change state over time
4. **Money-based**: Economic effect jokers
5. **Multiplicative**: XMult and compound effects
6. **Retrigger**: Card replay mechanics
7. **Legendary**: Unique, powerful effects

## Related Examples

Additional examples in `core/examples/`:
- `joker_comprehensive_guide.rs`: Complete joker system demonstration
- `performance_examples.rs`: Optimization techniques
- `trait_composition_examples.rs`: Advanced trait patterns
- `edge_case_examples.rs`: Error handling
- `new_joker_api.rs`: Current API usage
- `test_harness.rs`: Testing utilities
- `card_targeting_demo.rs`: Targeting system

## Running Examples

```bash
# View TOML schema
cat examples/joker_definitions.toml

# Run Rust examples
cargo run --example joker_comprehensive_guide
cargo run --example performance_examples
```

## Best Practices Shown

1. **Clear Naming**: Descriptive IDs and names
2. **Rich Descriptions**: Formatted text with color codes
3. **Balanced Costs**: Rarity-appropriate pricing
4. **Condition Composition**: Complex trigger conditions
5. **State Management**: Proper initialization and limits
6. **Type Safety**: Strongly-typed behavior definitions
