# CLAUDE.md - Skip Tags System

## Directory Purpose

The skip tags system is a reward mechanism that provides players with special benefits when they choose to skip blinds. It enhances strategic decision-making by offering various immediate rewards, shop modifiers, and game state modifications as incentives for skipping.

## Tag Categories

### Economic Tags (5 tags)
Immediate money rewards:
- **Economy**: Doubles current money (max +$40, $0 if negative)
- **Investment**: $25 after defeating next Boss Blind (stackable)
- **Garbage**: $1 per unused discard this run (retroactive)
- **Speed**: $5 per blind skipped this run (minimum $5)
- **Handy**: $1 per hand played this run (retroactive)

### Shop Enhancement Tags (6 tags)
Modify next shop experience:
- **Voucher**: Adds extra voucher to next shop (stackable)
- **Coupon**: Makes initial shop items free
- **D6**: Makes first reroll free
- **Foil**: Next base joker becomes Foil (+50 chips) and free
- **Holographic**: Next base joker becomes Holographic (+10 mult) and free
- **Polychrome**: Next base joker becomes Polychrome (×1.5 mult) and free

### Reward Tags (5 tags)
Pack/consumable rewards:
- **Charm**: Mega Tarot Pack
- **Ethereal**: Spectral Pack
- **Buffoon**: Mega Buffoon Pack
- **Standard**: Mega Standard Pack
- **Meteor**: Mega Planet Pack
- **Rare**: Rare Joker guaranteed in shop
- **Uncommon**: Uncommon or better Joker guaranteed
- **TopUp**: Creates consumables up to capacity

### Utility Tags (4 tags) [Currently disabled]
Special mechanics:
- **Double**: Duplicates another selected tag
- **Boss**: Re-rolls next Boss Blind
- **Orbital**: Upgrades random poker hand by 3 levels
- **Juggle**: +3 hand size for next round (stackable)

## Generation Mechanics

### Probability System
- **50% chance** to generate skip tags when skipping
- **Weighted rarity**:
  - Common: 1.0 weight
  - Uncommon: 0.6 weight
  - Rare: 0.3 weight
  - Legendary: 0.1 weight

### Selection Flow
1. Player skips blind → System rolls for tag generation
2. If successful, 1-3 tags offered based on weights
3. Player selects one tag for activation
4. Tag effect applies immediately or persists

### Stacking Rules
- **Stackable tags**: Voucher, Investment, Juggle accumulate effects
- **Non-stackable tags**: Only one instance active at a time
- **Persistence**: Shop enhancement tags persist until shop entry

## Integration Points

### Action System
```rust
Action::SkipBlind(Blind) → Triggers tag generation
Action::SelectSkipTag(SkipTagId) → Activates selected tag
```

### State Tracking
```rust
ActiveSkipTags {
    blinds_skipped: u32,              // Tracks total skips
    investment_count: u32,            // Tracks Investment tags
    next_shop_modifiers: NextShopModifiers,  // Pending shop effects
    active_instances: Vec<SkipTagInstance>,  // Active tag stack
}
```

### Key Features
- Automatic blind skip counting for economic tags
- Boss blind defeat triggers Investment payouts
- Shop entry consumes and applies modifiers
- Retroactive calculations for Garbage and Handy tags

## Architecture

### Registry Pattern
```rust
SkipTagRegistry {
    tags: RwLock<HashMap<SkipTagId, Arc<dyn SkipTag>>>
}
```
- Global singleton registry using `OnceLock`
- Thread-safe read/write access
- Dynamic tag registration at initialization

### State Management
- **ActiveSkipTags**: Persisted in game state for save/load
- **Shop Modifiers**: Stored until consumed by shop generation
- **Economic Counters**: Maintained throughout run

### Effect Application
1. **Immediate Effects**: Applied directly to game state
2. **Deferred Effects**: Stored for future trigger
3. **Conditional Effects**: Check game state before application

### Memory Management
- Tags use `Arc` for shared ownership without duplication
- Effect results are lightweight value types
- Zero-allocation patterns where possible

## Design Patterns

### Trait-Based Architecture
All tags implement the `SkipTag` trait:
```rust
pub trait SkipTag: Send + Sync {
    fn tag_id(&self) -> SkipTagId;
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn rarity(&self) -> SkipTagRarity;
    fn apply(&self, game: &mut Game) -> SkipTagEffect;
}
```

### Effect Composition
- Modular effect functions for reusability
- Clean separation between tag logic and game state
- Type safety with strongly-typed IDs and effects

## Performance Characteristics

- **Tag generation**: O(n) for weighted selection
- **Effect application**: O(1) for most effects
- **Memory overhead**: ~100 bytes per active tag
- **Thread safety**: All operations thread-safe

## Implementation Status

- **Complete**: Economic, Shop Enhancement, and Reward tags
- **Disabled**: Utility tags (Double, Boss, Orbital, Juggle)
- **Framework**: Ready for additional tag types
