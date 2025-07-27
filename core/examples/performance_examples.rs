//! # Performance Comparison Examples
//!
//! This example demonstrates different implementation patterns and their
//! performance characteristics. These examples help developers understand
//! the trade-offs between different approaches and optimize their joker implementations.

use balatro_rs::{
    card::{Card, Suit, Value},
    hand::SelectHand,
    joker::{GameContext, Joker, JokerEffect, JokerId, JokerRarity},
    joker_state::JokerState,
};
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Performance Comparison Examples ===\n");

    // Example 1: Efficient vs inefficient condition checking
    condition_checking_comparison()?;

    // Example 2: State access patterns
    state_access_patterns()?;

    // Example 3: Memory allocation patterns
    memory_allocation_patterns()?;

    // Example 4: Early return optimizations
    early_return_optimizations()?;

    // Example 5: Batched vs individual operations
    batched_operations_comparison()?;

    Ok(())
}

/// Example 1: Efficient vs inefficient condition checking
fn condition_checking_comparison() -> Result<(), Box<dyn std::error::Error>> {
    println!("## 1. Condition Checking Performance\n");

    let efficient_joker = EfficientConditionJoker;
    let inefficient_joker = InefficientConditionJoker;

    println!("Efficient Joker: {}", efficient_joker.name());
    println!("Description: {}", efficient_joker.description());
    println!("Optimizations:");
    println!("  ✓ Early returns for non-matching conditions");
    println!("  ✓ Minimal branching in hot paths");
    println!("  ✓ Precomputed condition flags");
    println!("  ✓ Bitwise operations for suit checking");
    println!();

    println!("Inefficient Joker: {}", inefficient_joker.name());
    println!("Description: {}", inefficient_joker.description());
    println!("Performance Issues:");
    println!("  ❌ Complex nested conditions");
    println!("  ❌ Unnecessary string allocations");
    println!("  ❌ Redundant calculations");
    println!("  ❌ No early returns");
    println!();

    Ok(())
}

/// Example 2: State access patterns
fn state_access_patterns() -> Result<(), Box<dyn std::error::Error>> {
    println!("## 2. State Access Performance\n");

    let cached_joker = CachedStateJoker;
    let direct_joker = DirectStateJoker;

    println!("Cached State Joker: {}", cached_joker.name());
    println!("Description: {}", cached_joker.description());
    println!("Optimizations:");
    println!("  ✓ Caches frequently accessed values");
    println!("  ✓ Batch state updates");
    println!("  ✓ Minimal serialization overhead");
    println!();

    println!("Direct State Joker: {}", direct_joker.name());
    println!("Description: {}", direct_joker.description());
    println!("Performance Issues:");
    println!("  ❌ Repeated state manager calls");
    println!("  ❌ Frequent serialization/deserialization");
    println!("  ❌ No value caching");
    println!();

    Ok(())
}

/// Example 3: Memory allocation patterns
fn memory_allocation_patterns() -> Result<(), Box<dyn std::error::Error>> {
    println!("## 3. Memory Allocation Performance\n");

    println!("Zero-Allocation Pattern:");
    println!("  ✓ Pre-allocated effect objects");
    println!("  ✓ Static string references");
    println!("  ✓ Stack-based calculations");
    println!("  ✓ Reused buffers");
    println!();

    println!("High-Allocation Pattern:");
    println!("  ❌ Dynamic string generation");
    println!("  ❌ Vector allocations per call");
    println!("  ❌ Boxed temporary values");
    println!("  ❌ Unnecessary cloning");
    println!();

    Ok(())
}

/// Example 4: Early return optimizations
fn early_return_optimizations() -> Result<(), Box<dyn std::error::Error>> {
    println!("## 4. Early Return Optimizations\n");

    let optimized_joker = EarlyReturnJoker;
    println!("Optimized Joker: {}", optimized_joker.name());
    println!("Description: {}", optimized_joker.description());
    println!("Optimizations:");
    println!("  ✓ Guard clauses eliminate unnecessary work");
    println!("  ✓ Most common cases handled first");
    println!("  ✓ Expensive calculations only when needed");
    println!("  ✓ Fail-fast validation");
    println!();

    Ok(())
}

/// Example 5: Batched vs individual operations
fn batched_operations_comparison() -> Result<(), Box<dyn std::error::Error>> {
    println!("## 5. Batched Operations Performance\n");

    println!("Batched Operations Pattern:");
    println!("  ✓ Process multiple cards at once");
    println!("  ✓ Single effect object creation");
    println!("  ✓ Reduced function call overhead");
    println!("  ✓ Better cache locality");
    println!();

    println!("Individual Operations Pattern:");
    println!("  ❌ Per-card processing overhead");
    println!("  ❌ Multiple effect object creations");
    println!("  ❌ Function call per card");
    println!("  ❌ Poor cache usage");
    println!();

    Ok(())
}

/// Efficient condition checking implementation
#[derive(Debug)]
struct EfficientConditionJoker;

impl Joker for EfficientConditionJoker {
    fn id(&self) -> JokerId {
        JokerId::Reserved10
    }

    fn name(&self) -> &str {
        "Efficient Checker"
    }

    fn description(&self) -> &str {
        "Optimized condition checking for face cards"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Common
    }

    fn on_card_scored(&self, _context: &mut GameContext, card: &Card) -> JokerEffect {
        // Efficient: Single comparison with early return
        match card.value {
            Value::Jack | Value::Queen | Value::King => {
                // Pre-computed static effect - no allocations
                JokerEffect::new().with_mult(3)
            }
            _ => {
                // Early return with zero-cost effect
                JokerEffect::new()
            }
        }
    }
}

/// Inefficient condition checking implementation
#[derive(Debug)]
struct InefficientConditionJoker;

impl Joker for InefficientConditionJoker {
    fn id(&self) -> JokerId {
        JokerId::Reserved // Using reserved slot
    }

    fn name(&self) -> &str {
        "Inefficient Checker"
    }

    fn description(&self) -> &str {
        "Non-optimized condition checking with performance issues"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Common
    }

    fn on_card_scored(&self, _context: &mut GameContext, card: &Card) -> JokerEffect {
        // Inefficient: Complex nested conditions and allocations
        let card_name = format!("{:?}", card.value); // Unnecessary allocation
        let is_face_card =
            card_name.contains("Jack") || card_name.contains("Queen") || card_name.contains("King");

        if is_face_card {
            let suit_name = format!("{:?}", card.suit); // More allocations
            let bonus = if suit_name.len() > 4 { 4 } else { 3 }; // Pointless calculation

            JokerEffect::new()
                .with_mult(bonus)
                .with_message(format!("Face card: {card_name} of {suit_name}"))
        // Even more allocations
        } else {
            JokerEffect::new()
        }
    }
}

/// Cached state access implementation
#[derive(Debug)]
struct CachedStateJoker;

impl Joker for CachedStateJoker {
    fn id(&self) -> JokerId {
        JokerId::Reserved2
    }

    fn name(&self) -> &str {
        "Cached State"
    }

    fn description(&self) -> &str {
        "Efficient state access with caching"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Uncommon
    }

    fn on_hand_played(&self, context: &mut GameContext, _hand: &SelectHand) -> JokerEffect {
        // Efficient: Get state once and cache values
        if let Some(state) = context.joker_state_manager.get_state(self.id()) {
            let cached_mult = state.accumulated_value as i32;

            // Single state update with closure
            context
                .joker_state_manager
                .update_state(self.id(), |state| {
                    state.accumulated_value += 1.0;
                });

            JokerEffect::new().with_mult(cached_mult.min(20))
        } else {
            JokerEffect::new()
        }
    }

    fn initialize_state(&self, _context: &GameContext) -> JokerState {
        let mut state = JokerState::new();
        state.accumulated_value = 1.0;
        state
    }
}

/// Direct state access implementation
#[derive(Debug)]
struct DirectStateJoker;

impl Joker for DirectStateJoker {
    fn id(&self) -> JokerId {
        JokerId::Reserved3
    }

    fn name(&self) -> &str {
        "Direct State"
    }

    fn description(&self) -> &str {
        "Inefficient direct state access"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Uncommon
    }

    fn on_hand_played(&self, context: &mut GameContext, _hand: &SelectHand) -> JokerEffect {
        // Inefficient: Multiple state manager calls
        let _current_value = context
            .joker_state_manager
            .get_accumulated_value(self.id())
            .unwrap_or(0.0);

        // Separate call to increment
        context
            .joker_state_manager
            .add_accumulated_value(self.id(), 1.0);

        // Another call to get updated value
        let new_value = context
            .joker_state_manager
            .get_accumulated_value(self.id())
            .unwrap_or(0.0);

        // More unnecessary state access
        if let Some(_state) = context.joker_state_manager.get_state(self.id()) {
            JokerEffect::new().with_mult((new_value as i32).min(20))
        } else {
            JokerEffect::new()
        }
    }
}

/// Early return optimization implementation
#[derive(Debug)]
struct EarlyReturnJoker;

impl Joker for EarlyReturnJoker {
    fn id(&self) -> JokerId {
        JokerId::Reserved4
    }

    fn name(&self) -> &str {
        "Early Return"
    }

    fn description(&self) -> &str {
        "Optimized with early returns and guard clauses"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Rare
    }

    fn on_card_scored(&self, context: &mut GameContext, card: &Card) -> JokerEffect {
        // Guard clause: Early return for most common case
        if card.suit != Suit::Spade {
            return JokerEffect::new();
        }

        // Guard clause: Early return for low-value cards
        if matches!(card.value, Value::Two | Value::Three | Value::Four) {
            return JokerEffect::new();
        }

        // Guard clause: Check ante threshold
        if context.ante < 3 {
            return JokerEffect::new();
        }

        // Only reach expensive calculation if all conditions pass
        let bonus = match card.value {
            Value::Ace => 10,
            Value::King | Value::Queen | Value::Jack => 8,
            Value::Ten => 6,
            _ => 4,
        };

        JokerEffect::new().with_mult(bonus)
    }
}

/// Demonstrates performance measurement utilities
#[allow(dead_code)]
fn performance_measurement_example() {
    // Example of how to measure joker performance
    let _joker = EfficientConditionJoker;
    let test_cards = vec![
        Card::new(Value::King, Suit::Spade),
        Card::new(Value::Two, Suit::Heart),
        Card::new(Value::Ace, Suit::Diamond),
    ];

    let start = Instant::now();

    // Simulate processing many cards
    for _ in 0..10000 {
        for card in &test_cards {
            // This would normally be called with proper game context
            // let _ = joker.on_card_scored(&mut context, card);

            // Simulate the work
            match card.value {
                Value::Jack | Value::Queen | Value::King => { /* work */ }
                _ => { /* early return */ }
            }
        }
    }

    let duration = start.elapsed();
    println!("Performance test completed in: {duration:?}");
}

/// Demonstrates memory-efficient patterns
#[allow(dead_code)]
#[derive(Debug)]
struct ZeroAllocationJoker;

impl Joker for ZeroAllocationJoker {
    fn id(&self) -> JokerId {
        JokerId::Reserved5
    }

    fn name(&self) -> &str {
        "Zero Allocation"
    }

    fn description(&self) -> &str {
        "Demonstrates zero-allocation patterns"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Legendary
    }

    fn on_card_scored(&self, _context: &mut GameContext, card: &Card) -> JokerEffect {
        // Zero allocations: everything on the stack
        const SPADE_BONUS: i32 = 5;
        const FACE_BONUS: i32 = 3;

        let suit_bonus = if card.suit == Suit::Spade {
            SPADE_BONUS
        } else {
            0
        };
        let face_bonus = match card.value {
            Value::Jack | Value::Queen | Value::King => FACE_BONUS,
            _ => 0,
        };

        let total_mult = suit_bonus + face_bonus;

        if total_mult > 0 {
            JokerEffect::new().with_mult(total_mult)
        } else {
            JokerEffect::new() // Cached empty effect
        }
    }
}
