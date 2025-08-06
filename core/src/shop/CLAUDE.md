# CLAUDE.md - Shop System

## Directory Purpose

The shop system serves as the primary economic interface in balatro-rs, providing strategic purchasing decisions during the shop phase. It implements a sophisticated weighted generation system that adapts to game progression and player resources.

## Key Components

### Core Structures
- **`EnhancedShop`**: Main shop container with slots, reroll mechanics, and generation weights
- **`ShopSlot`**: Individual shop position containing item, cost, availability, and modifiers
- **`ShopItem`**: Unified enum for all purchasable items (Joker, Consumable, Voucher, Pack, PlayingCard)
- **`ItemWeights`**: Probability weights for shop generation

### Generation Weights
Default distribution:
- Jokers: 50%
- Consumables: 20%
- Vouchers: 10%
- Packs: 15%
- Playing Cards: 5%

### Rarity Distribution
Standard joker rarity:
- Common: 70%
- Uncommon: 25%
- Rare: 4.5%
- Legendary: 0.5%

## Shop Mechanics

### Generation System
- **`WeightedGenerator`**: Statistical generation with rarity distributions
- **Ante-based scaling**: Higher antes increase rare item chances
- **Money-based adjustments**: Low money favors cheaper items

### Reroll System
- **Base cost**: 5 coins, increases by 5 per reroll
- **Cost progression**: 5 → 10 → 15 → 20 (linear)
- **Voucher effects**:
  - `RerollSurplus`: Free rerolls
  - `Liquidation`: 25% discount on rerolls
  - `ClearanceSale`: 50% discount on all items

### Refresh Process
1. Calculate weights based on game state
2. Generate new items for each slot
3. Apply cost modifiers from vouchers
4. Preserve reroll state

## Pack System

### Pack Types (13 variants)
**Playing Card Packs**:
- Standard (3), Jumbo (5), Mega (7), Enhanced (3-4)

**Joker Packs**:
- Buffoon (2), MegaBuffoon (4)

**Consumable Packs**:
- Arcana/MegaArcana: Tarot cards (2-3/4-6)
- Celestial/MegaCelestial: Planet cards (2-3/4-6)
- Spectral/MegaSpectral: Spectral cards (2-3/4-6)

**Mixed**:
- Variety: 3-5 mixed items

### Pack Mechanics
- **Selection**: Choose 1 item from generated options
- **Cost**: Standard packs cost 4, Mega packs cost 8
- **Skip Option**: All packs can be skipped without penalty

### Pack Opening Flow
1. Purchase pack from shop
2. Generate pack contents based on type
3. Present options with preview information
4. Player selects one item or skips
5. Selected item added to inventory

## Integration Points

### Voucher System
- **`Overstock`**: +20% all generation weights
- **`ClearanceSale`**: 50% discount on all items
- **`Liquidation`**: 25% discount on operations
- **`Hone`**: +30% joker generation weight

### Game State
- Reads current ante for difficulty scaling
- Monitors player money for affordability
- Tracks existing jokers (future: prevent duplicates)
- Uses game's RNG for consistent randomization

### Action System
- Generates `BuyJoker` actions with slot positioning
- Validates purchases against game constraints
- Integrates with move generation for AI/RL

## Architecture Highlights

### Design Patterns
- **Statistical Fairness**: Cryptographically secure RNG with well-defined distributions
- **Performance Optimization**: Weighted generation cached, O(n) operations
- **Extensibility**: Trait-based design allows custom generators
- **Configuration-Driven**: All parameters externalized to config
- **Backward Compatibility**: Legacy shop preserved while new system developed

### Pack Architecture
- **`PackType`** enum: Defines all pack variants
- **`Pack`** structure: Contains options and metadata
- **`PackOption`**: Provides item with preview information
- **`PackGenerator`** trait: Enables custom pack generation
- **`OpenPackState`**: Manages active pack selection

## Legacy System

Maintains backward compatibility:
- **`Shop`**: Legacy implementation with 2 joker slots
- **`JokerGenerator`**: Simple random joker generation
- Used for tests and gradual migration

## Performance Characteristics

- **Generation**: O(n) for n slots
- **Reroll**: O(n) regeneration
- **Weight calculation**: O(1) with caching
- **Memory**: ~1KB per shop instance

## Development Notes

The shop serves as a critical economic balancing mechanism, providing strategic depth through resource management and build customization while maintaining statistical fairness for RL training applications.
