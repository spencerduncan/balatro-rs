# Boss Blinds Module

This module implements boss blind mechanics for the Balatro game engine.

## Overview

Boss blinds are special blind types that introduce unique challenges and mechanics beyond normal scoring requirements. They modify gameplay through various effects and typically offer greater rewards.

## Key Features

- **Unique Effects**: Each boss blind has special mechanics that change gameplay
- **Dynamic Interaction**: Effects can trigger on blind start, hand play, and blind end
- **Score Modification**: Can modify hand scoring through multipliers, bonuses, or penalties
- **State Tracking**: Maintains boss blind state throughout the game
- **Higher Stakes**: Increased difficulty with correspondingly higher rewards

## Architecture

The module provides a flexible framework for boss blind implementation:

- `BossBlind` trait defines the interface all boss blinds must implement
- `BossBlindId` enum identifies all available boss blinds
- `BossBlindState` tracks active boss blind and custom state data
- `HandModification` represents changes to hand scoring
- Event-driven system with hooks for different game phases

## Usage

```rust
use balatro_rs::boss_blinds::{BossBlindId, BossBlindState, HandModification};

// Create boss blind state
let mut state = BossBlindState::new();

// Activate a boss blind
state.activate(BossBlindId::BossBlindPlaceholder);
assert!(state.is_active());

// Create hand modifications
let modification = HandModification::multiply_score(0.5); // Halve score
let bonus = HandModification::add_score(100); // Add bonus points
```

## Boss Blind Lifecycle

1. **Selection**: Player chooses boss blind from available options
2. **Activation**: `on_blind_start()` applies initial effects
3. **Hand Playing**: `on_hand_played()` modifies each hand as it's played
4. **Completion**: `on_blind_end()` handles cleanup and final effects

## Integration Points

- **Blind Selection**: Integrates with stage system for blind choice
- **Hand Evaluation**: Modifies scoring through `HandModification`
- **Game State**: Tracks boss blind state and custom data
- **Difficulty Scaling**: Base score requirements scale with ante progression

## Future Expansion

The module is designed for easy extension with specific boss blind implementations in `implementations.rs`, each providing unique and challenging gameplay modifications.