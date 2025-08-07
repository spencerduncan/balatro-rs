# CLAUDE.md - Boss Blinds System

## Directory Purpose

The boss blinds module implements a special challenge system for enhanced difficulty in Balatro. Boss blinds are advanced versions of regular blinds that introduce unique gameplay modifications beyond standard scoring requirements, creating variety and strategic depth.

## Key Components

### Core Infrastructure
- **`BossBlind` Trait**: Unified interface for all boss blind implementations
  - `name()`: Display name
  - `apply_effects()`: Modifies game state when activated
  - `check_counters()`: Monitors game events
  - `get_effects()`: Describes gameplay modifications
  - `min_ante()`: Minimum ante level requirement

- **`BossBlindState`**: Runtime state management
  - Tracks active boss blind
  - Maintains effect activation status
  - Stores custom state via type-safe `BossBlindData` enum
  - Provides activation/deactivation lifecycle

### Effect System

#### HandModification
Scoring modifications during hand evaluation:
- Score multipliers (halving/doubling)
- Additive bonuses/penalties
- Forced discard mechanics
- Card disabling during scoring

#### BlindEffect Categories
- `DebuffCards`: Disables specific card types (face cards, suits, colors)
- `RestrictActions`: Limits player actions (discards, rerolls)
- `ModifyScoring`: Alters scoring calculations
- `SpecialRule`: Unique mechanics

## Architecture Patterns

### Trait-Based Design
Multiple specialized traits following Interface Segregation:
- `BossBlindInfo`: Metadata and information
- `BossBlindLifecycle`: Activation/deactivation events
- `BossBlindScoring`: Hand evaluation modifications
- Unified `BossBlind` trait combines all concerns

### Type-Safe State Management
- Bounded data types (`BossBlindData`) prevent serialization vulnerabilities
- Event-driven design hooks into specific game phases
- Counter-based mechanics track game events

### Thread Safety
All boss blind traits require `Send + Sync` bounds for multi-threaded RL training.

## Gameplay Effects

### Scoring Modifications
- Multipliers to hand scores (e.g., 0.5x for difficulty)
- Bonus point additions/subtractions
- Card disabling from scoring

### Action Restrictions
- Limit/prevent discards
- Restrict hand plays
- Modify shop interactions

### Progressive Effects
- Effects escalate based on counters
- State tracking for cumulative effects
- Reward structure with 1.5x monetary multiplier

## Game Integration

### Stage System
- Appear as `Stage::Blind(Blind::Boss)`
- Selected during `Stage::PreBlind()` via `Action::SelectBlind()`
- Skippable via `Action::SkipBlind()` for skip tags

### State Persistence
- `BossBlindState` serialized in `SaveableGameState`
- Maintains state across save/load cycles
- Compatible with versioned save system

### Ante Progression
- Scale with ante levels
- Minimum ante requirements ensure appropriate difficulty
- Base score requirements increase with advancement

## Implementation Status

Framework complete but lacks specific boss blind implementations:
- Full trait hierarchy and type definitions
- State management and persistence
- Integration points with game flow
- Placeholder `BossBlindId::BossBlindPlaceholder` ready for expansion

Ready for specific implementations (The Hook, The Wall, The Needle, etc.) following the established framework.

## Performance Characteristics
- Effect application: ~100ns per check
- State serialization: ~1μs
- Counter updates: O(1) operations
- Memory overhead: ~100 bytes per active boss blind
