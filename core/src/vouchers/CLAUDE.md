# CLAUDE.md - Voucher System

## Directory Purpose

Vouchers are permanent upgrades that provide passive effects throughout the entire game run. They serve as a crucial progression system, allowing players to permanently modify game rules, create strategic builds through voucher combinations, and enable meta-progression.

## Voucher Types

### Shop Vouchers (8 total)
Modify shop mechanics:
- **Overstock**: +20% all item generation weights
- **ClearanceSale**: 50% discount on all shop items
- **Hone**: +30% joker generation weight
- **RerollSurplus**: Free shop rerolls
- **CrystalBall**: Adds consumable slot
- **Liquidation**: 25% discount on all operations
- **RerollGlut**: Base reroll cost reduced to 2
- **GlowUp**: 2 additional shop slots

### Gameplay Vouchers (13 total)
Modify core mechanics:
- **Grabber**: +1 hand size
- **NachoTong**: +1 hand size
- **Wasteful**: +1 hand size, -1 joker slot
- **SeedMoney**: +$5 at round start
- **MoneyTree**: $1 interest per $5 (max $25)
- **Hieroglyph**: -2 ante, -1 hand size
- **Petroglyph**: -1 ante
- **Antimatter**: +1 joker slot (free, rare)
- **MagicTrick**: Playing cards purchasable in shop
- **Illusion**: Playing cards in shop may have enhancements
- **Blank**: No effect placeholder
- **PaintBrush**: +1 hand size, +1 joker slot
- **TarotMerchant**: Tarot cards appear 2x more in shop

### Upgrade Vouchers (9 total)
Enhanced versions with prerequisites:
- **TarotTycoon**: Tarot cards appear 4x more (requires TarotMerchant)
- **Recyclomancy**: Tarot cards appear 2x more (requires TarotMerchant)
- **PlanetMerchant**: Planet cards appear 2x more
- **PlanetTycoon**: Planet cards appear 4x more (requires PlanetMerchant)
- **DirectorsCut**: Ante scales by 2 per round, +1 discard (requires Grabber)
- **Retcon**: Boss blinds can be rerolled
- **Palette**: +1 hand size, +1 joker slot (requires PaintBrush)

## Core Components

### Enums and Types
```rust
VoucherId      // 30 unique vouchers
VoucherTier    // Base or Upgraded
VoucherEffect  // 24 distinct effect types
VoucherCollection  // Manages owned vouchers
```

### Effect Categories
- **Hand Mechanics**: Hand size, discards, plays modifications
- **Economy**: Money gain, interest cap, shop discounts
- **Capacity**: Joker slots, consumable slots, shop slots
- **Progression**: Ante scaling, win requirements, blind scores
- **Shop Modifiers**: Playing cards, enhancements, pack options
- **Frequency Multipliers**: Tarot, Planet, Polychrome appearance rates
- **Special**: Boss blind rerolls, starting cards

## Validation System

### Constraints
- Hand size increases: max 50
- Joker slot changes: max 20
- Money gains: max 10,000
- Shop slots: max 20
- Discards/plays: max 50
- Multipliers: ≤10.0 for scaling, ≤1.0 for reductions

### Validation Process
```rust
fn validate_effect(&self, effect: &VoucherEffect) -> Result<(), String>
```
Ensures all effects remain within balanced bounds.

## Purchase Mechanics

### Purchase Flow
1. **Prerequisite Check**: Verify prerequisites via `VoucherCollection`
2. **Affordability Check**: Verify sufficient money (base cost: 10)
3. **Ownership Check**: Ensure not already owned (one-time purchase)
4. **Application**: Effects applied immediately upon purchase

### Effect Application
- **Immediate Effects**: Applied to GameState directly
- **System Effects**: Handled by respective systems
- **Permanent Effects**: Most effects persist for entire run
- **One-time Effects**: MoneyGain consumed immediately

## State Management

### VoucherCollection
```rust
pub struct VoucherCollection {
    owned_vouchers: HashSet<VoucherId>,
}
```
- Lightweight storage (only IDs)
- Integrated with save/load system
- Prerequisite checking support

### Effect Persistence
- Permanent modifiers remain active throughout run
- Cascading prerequisites ensure proper upgrade paths
- Validation system prevents invalid state combinations
- GameState conversion preserves voucher ownership

## Integration Points

### Shop System
- Displayed as `ShopItem::Voucher(VoucherId)`
- Generation weights modified by voucher effects
- Cost modifiers applied during generation

### Game Module
- Purchase handled via action system
- Effects integrated with configuration
- Persistent modifications to game rules

### Save/Load System
- VoucherCollection serialized with game state
- Effects reapplied on load
- Prerequisite trees maintained

## Architecture Patterns

### Trait-Based Design
```rust
pub trait Voucher: Send + Sync {
    fn voucher_id(&self) -> VoucherId;
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn tier(&self) -> VoucherTier;
    fn prerequisites(&self) -> Vec<VoucherId>;
    fn effects(&self) -> Vec<VoucherEffect>;
    fn base_cost(&self) -> u32;
    fn stacking(&self) -> VoucherStacking;
}
```

### Key Patterns
- **Prerequisite Trees**: Create progression depth
- **Effect Validation**: Ensures game balance
- **Stacking Rules**: Currently NoStacking for all vouchers
- **Simplified GameState**: Decouples vouchers from full game complexity

## Performance Characteristics

- **Effect application**: O(1) for most effects
- **Prerequisite checking**: O(n) for prerequisite chain
- **Memory overhead**: ~100 bytes per owned voucher
- **Thread safety**: All operations thread-safe

## Implementation Status

- **Complete**: All 30 base vouchers implemented
- **Framework**: Ready for additional vouchers
- **Validation**: Comprehensive effect validation
- **Integration**: Full shop and game system integration
