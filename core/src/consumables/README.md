# Consumables Module

This module implements consumable cards for the Balatro game engine.

## Overview

Consumables are single-use cards that provide various effects when played. There are three main types:

- **Tarot Cards**: Modify deck composition or provide temporary benefits
- **Planet Cards**: Permanently upgrade specific poker hands
- **Spectral Cards**: Powerful effects that often come with risks or costs

## Architecture

The module follows a trait-based design similar to the joker system:

- `Consumable` trait defines the interface all consumables must implement
- `ConsumableId` enum identifies all available consumables  
- `ConsumableType` categorizes consumables by their type
- Separate submodules for each consumable type (planned for future implementation)

## Usage

```rust
use balatro_rs::consumables::{ConsumableId, ConsumableType};

// Get all available consumables
let all_consumables = ConsumableId::all();

// Check consumable type
let tarot_type = ConsumableId::TarotPlaceholder.consumable_type();
assert_eq!(tarot_type, ConsumableType::Tarot);
```

## Future Expansion

This module is designed to be extended with specific consumable implementations:

- `tarot.rs` - Tarot card implementations
- `planet.rs` - Planet card implementations  
- `spectral.rs` - Spectral card implementations

Each implementation will provide concrete types that implement the `Consumable` trait.