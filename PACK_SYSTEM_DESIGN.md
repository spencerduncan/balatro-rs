# Pack System Design Specification

## Overview
Implementation of comprehensive pack system for Balatro-rs, extending the existing enhanced shop architecture to support all required pack types with choose-from-multiple mechanics.

## Design Principles
1. **Extend existing architecture** - Build upon enhanced shop system in `core/src/shop/mod.rs`
2. **Follow established patterns** - Mirror joker/consumable patterns for consistency
3. **TDD approach** - Write tests first to ensure requirements are met
4. **Integration focused** - Seamlessly integrate with existing game systems

## Pack Types and Requirements

### Standard Pack
- **Contents**: 3 playing cards (choose 1)
- **Cost**: $4
- **Enhancement chance**: 10% for enhanced cards
- **Selection**: Choose 1 of 3 options

### Buffoon Pack  
- **Contents**: 2 joker cards (choose 1)
- **Cost**: $4
- **Rarity distribution**: Based on current ante
- **Selection**: Choose 1 of 2 options

### Arcana Pack
- **Contents**: 2-3 Tarot cards (choose 1)
- **Cost**: $4
- **Card pool**: All available Tarot cards
- **Selection**: Choose 1 of 2-3 options

### Celestial Pack
- **Contents**: 2-3 Planet cards (choose 1)  
- **Cost**: $4
- **Card pool**: All available Planet cards
- **Selection**: Choose 1 of 2-3 options

### Spectral Pack
- **Contents**: 2-3 Spectral cards (choose 1)
- **Cost**: $4
- **Card pool**: All available Spectral cards
- **Selection**: Choose 1 of 2-3 options

### Mega Pack Variants
- **Mega Buffoon**: 4 joker cards (choose 1)
- **Mega Arcana**: 4-6 Tarot cards (choose 1)
- **Mega Celestial**: 4-6 Planet cards (choose 1)
- **Cost multiplier**: 2x base pack cost

## Data Structures

### Updated PackType Enum
```rust
#[derive(Debug, Clone, PartialEq)]
pub enum PackType {
    Standard,
    Buffoon,
    Arcana,
    Celestial,
    Spectral,
    MegaBuffoon,
    MegaArcana,
    MegaCelestial,
}
```

### Pack Structure
```rust
pub struct Pack {
    pub pack_type: PackType,
    pub options: Vec<PackOption>,
    pub choose_count: usize,
    pub can_skip: bool,
}

pub struct PackOption {
    pub item: ShopItem,
    pub preview_info: String,
}
```

### New Actions
```rust
// Add to Action enum:
BuyPack { pack_type: PackType },
OpenPack { pack_id: usize },
SelectFromPack { pack_id: usize, option_index: usize },
SkipPack { pack_id: usize },
```

## Implementation Strategy

### Phase 1: Foundation (RED - Tests First)
1. **Write acceptance tests** for each pack type
2. **Add pack actions** to Action enum  
3. **Create packs.rs module** with basic structures
4. **Update game state** to track pack inventory

### Phase 2: Generation (GREEN - Minimal Implementation)
1. **Implement pack generators** for each type
2. **Add purchase mechanics** to shop system
3. **Create pack opening state** in game
4. **Basic selection mechanics**

### Phase 3: Enhancement (REFACTOR - Polish)
1. **Add Grab Bag voucher** support (+1 option)
2. **Implement skip mechanics**
3. **Optimize pack generation** algorithms
4. **Add UI state management**

## File Structure

```
core/src/shop/
├── mod.rs           # Enhanced shop architecture (existing)
├── legacy.rs        # Current shop implementation (existing)  
└── packs.rs         # New pack system implementation
```

## Integration Points

### Game State Extensions
```rust
// Add to Game struct:
pub pack_inventory: Vec<Pack>,
pub open_pack: Option<OpenPackState>,

pub struct OpenPackState {
    pub pack: Pack,
    pub pack_id: usize,
}
```

### Action Handlers
- Extend `Game::handle_action()` with pack actions
- Add pack move generation to `Generator` trait
- Update `GameError` with pack-specific errors

### Shop Integration  
- Modify shop generation to include packs
- Add pack availability logic based on ante
- Implement pack cost modifiers from vouchers

## Voucher Integration

### Grab Bag Voucher
- **Effect**: +1 option to all packs
- **Implementation**: Modify pack generation to add extra option
- **Cost**: $10

### Pack-affecting Vouchers
- **Clearance Sale**: Packs cost 50% less
- **Bulk Discount**: Buy 2 packs, get 1 free
- **Premium Packs**: All packs become Mega variants

## Testing Strategy

### Acceptance Tests
```rust
#[test]
fn test_standard_pack_choose_one_of_three_cards() { }

#[test] 
fn test_buffoon_pack_choose_one_of_two_jokers() { }

#[test]
fn test_arcana_pack_choose_one_of_tarot_cards() { }

#[test]
fn test_mega_pack_double_options() { }

#[test]
fn test_grab_bag_voucher_adds_option() { }

#[test]
fn test_pack_skip_mechanics() { }
```

### Integration Tests
- Pack purchase flow
- Pack opening state management  
- Pack selection validation
- Voucher effects on packs
- Move generation for AI/RL

## Error Handling

### New GameError Variants
```rust
InvalidPackSelection,
PackNotOwned,
PackAlreadyOpened,
InsufficientFundsForPack,
PackNotAvailable,
```

## Performance Considerations

1. **Lazy generation** - Generate pack contents only when opened
2. **Pool-based selection** - Pre-compute available items for efficiency
3. **Memory management** - Clean up opened packs promptly
4. **Cache pack costs** - Avoid recalculation with voucher effects

## Balance Considerations

1. **Pack availability** - Gate premium packs behind ante progression
2. **Cost scaling** - Adjust pack costs based on game state
3. **Rarity distribution** - Ensure appropriate power level progression
4. **Voucher synergy** - Pack vouchers should be meaningful but not overpowered

## Future Extensions

1. **Custom pack types** - User-defined pack configurations
2. **Pack crafting** - Create packs from owned items
3. **Pack trading** - Exchange packs between game modes
4. **Pack achievements** - Track pack-related statistics