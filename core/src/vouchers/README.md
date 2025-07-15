# Vouchers Module

This module implements voucher cards for the Balatro game engine.

## Overview

Vouchers are permanent upgrades that can be purchased from the shop. Unlike consumables, vouchers provide passive effects that persist throughout the entire run.

## Key Features

- **Permanent Effects**: Voucher effects remain active once purchased
- **One-Time Purchase**: Each voucher can only be bought once per run
- **Prerequisites**: Some vouchers require other vouchers to be owned first
- **Passive Application**: Effects are automatically applied without player intervention

## Architecture

The module provides a clean trait-based system:

- `Voucher` trait defines the interface all vouchers must implement
- `VoucherId` enum identifies all available vouchers
- `VoucherCollection` manages owned vouchers and prerequisite checking
- Effects integrate with the game's configuration system

## Usage

```rust
use balatro_rs::vouchers::{VoucherId, VoucherCollection};

// Create a voucher collection
let mut vouchers = VoucherCollection::new();

// Add a voucher
vouchers.add(VoucherId::VoucherPlaceholder);

// Check ownership
assert!(vouchers.owns(VoucherId::VoucherPlaceholder));

// Check if voucher can be purchased (prerequisites met)
assert!(!vouchers.can_purchase(VoucherId::VoucherPlaceholder)); // Already owned
```

## Integration Points

- **Shop System**: Vouchers appear in shop alongside jokers and consumables
- **Configuration**: Voucher effects modify game configuration permanently
- **Save System**: Voucher ownership persists between sessions
- **Prerequisites**: Complex voucher trees with dependencies

## Future Expansion

The module is designed for easy extension with specific voucher implementations in `implementations.rs`.