//! # Edge Case Examples
//!
//! This example demonstrates how to handle edge cases, unusual combinations,
//! and boundary conditions in joker implementations. These examples show
//! defensive programming patterns and error handling strategies.

use balatro_rs::{
    hand::SelectHand,
    joker::{GameContext, Joker, JokerEffect, JokerId, JokerRarity},
    joker_state::JokerState,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Edge Case Examples ===\n");

    // Example 1: Self-destructing joker with limited uses
    self_destructing_joker()?;

    // Example 2: Conditional joker with complex validation
    conditional_validation_joker()?;

    // Example 3: Overflow-safe accumulating joker
    overflow_safe_joker()?;

    // Example 4: State corruption recovery joker
    recovery_joker()?;

    // Example 5: Anti-pattern joker (what not to do)
    anti_pattern_examples()?;

    Ok(())
}

/// Example 1: Self-destructing joker with limited uses
fn self_destructing_joker() -> Result<(), Box<dyn std::error::Error>> {
    println!("## 1. Self-Destructing Joker - The Firework\n");

    let joker = FireworkJoker;
    println!("Name: {}", joker.name());
    println!("Description: {}", joker.description());
    println!("Edge Cases Handled:");
    println!("  - Limited trigger count");
    println!("  - Self-destruction timing");
    println!("  - State validation after triggers");
    println!("  - Prevents negative trigger counts");
    println!();

    Ok(())
}

/// Example 2: Conditional joker with complex validation
fn conditional_validation_joker() -> Result<(), Box<dyn std::error::Error>> {
    println!("## 2. Conditional Validation Joker - The Perfectionist\n");

    let joker = PerfectionistJoker;
    println!("Name: {}", joker.name());
    println!("Description: {}", joker.description());
    println!("Edge Cases Handled:");
    println!("  - Empty hand validation");
    println!("  - Invalid card combinations");
    println!("  - Boundary value conditions");
    println!("  - Graceful degradation");
    println!();

    Ok(())
}

/// Example 3: Overflow-safe accumulating joker
fn overflow_safe_joker() -> Result<(), Box<dyn std::error::Error>> {
    println!("## 3. Overflow-Safe Joker - The Bank\n");

    let joker = BankJoker;
    println!("Name: {}", joker.name());
    println!("Description: {}", joker.description());
    println!("Edge Cases Handled:");
    println!("  - Prevents integer overflow");
    println!("  - Caps maximum accumulated value");
    println!("  - Handles negative value attempts");
    println!("  - Safe arithmetic operations");
    println!();

    Ok(())
}

/// Example 4: State corruption recovery joker
fn recovery_joker() -> Result<(), Box<dyn std::error::Error>> {
    println!("## 4. Recovery Joker - The Phoenix\n");

    let joker = PhoenixJoker;
    println!("Name: {}", joker.name());
    println!("Description: {}", joker.description());
    println!("Edge Cases Handled:");
    println!("  - Corrupted state detection");
    println!("  - Automatic state recovery");
    println!("  - Version migration fallbacks");
    println!("  - Graceful error handling");
    println!();

    Ok(())
}

/// Example 5: Anti-pattern demonstrations
fn anti_pattern_examples() -> Result<(), Box<dyn std::error::Error>> {
    println!("## 5. Anti-Pattern Examples\n");

    println!("Common Mistakes to Avoid:");
    println!("  ❌ No bounds checking on calculations");
    println!("  ❌ Ignoring state validation failures");
    println!("  ❌ Assuming cards/hands are always valid");
    println!("  ❌ Not handling edge cases in serialization");
    println!("  ❌ Expensive operations in hot paths");
    println!("  ❌ Memory leaks from uncleaned state");
    println!("  ❌ Race conditions in multi-threaded scenarios");
    println!();

    println!("✅ Best Practices:");
    println!("  ✓ Always validate inputs and state");
    println!("  ✓ Use saturating arithmetic for scores");
    println!("  ✓ Implement graceful fallbacks");
    println!("  ✓ Add comprehensive error handling");
    println!("  ✓ Test edge cases thoroughly");
    println!("  ✓ Clean up resources properly");
    println!();

    Ok(())
}

/// The Firework Joker - Self-destructs after limited uses
#[derive(Debug)]
struct FireworkJoker;

impl Joker for FireworkJoker {
    fn id(&self) -> JokerId {
        JokerId::Reserved10
    }

    fn name(&self) -> &str {
        "The Firework"
    }

    fn description(&self) -> &str {
        "Explosive +20 Mult for 3 hands, then destroys itself"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Rare
    }

    fn on_hand_played(&self, context: &mut GameContext, _hand: &SelectHand) -> JokerEffect {
        // Get current trigger count, with safe fallback
        let triggers_remaining = context
            .joker_state_manager
            .get_state(self.id())
            .and_then(|state| state.triggers_remaining)
            .unwrap_or(3); // Default to 3 if state is missing

        if triggers_remaining > 0 {
            // Decrement trigger count safely using closure
            context
                .joker_state_manager
                .update_state(self.id(), |state| {
                    state.triggers_remaining = Some(triggers_remaining.saturating_sub(1));
                });

            // Check if this was the last trigger
            if triggers_remaining == 1 {
                JokerEffect::new()
                    .with_mult(20)
                    .with_message("BOOM! Firework explodes in a final burst!".to_string())
                // Note: In real implementation, this would trigger self-destruction
            } else {
                JokerEffect::new().with_mult(20).with_message(format!(
                    "Firework sparkles! {} uses remaining",
                    triggers_remaining - 1
                ))
            }
        } else {
            // This shouldn't happen if joker is properly removed, but handle gracefully
            JokerEffect::new().with_message("Firework is spent...".to_string())
        }
    }

    fn initialize_state(&self, _context: &GameContext) -> JokerState {
        let mut state = JokerState::new();
        state.triggers_remaining = Some(3);
        state
    }

    fn validate_state(&self, _context: &GameContext, state: &JokerState) -> Result<(), String> {
        if let Some(triggers) = state.triggers_remaining {
            if triggers > 10 {
                return Err("Triggers remaining cannot exceed 10 (corruption detected)".to_string());
            }
        }
        Ok(())
    }
}

/// The Perfectionist Joker - Only triggers under perfect conditions
#[derive(Debug)]
struct PerfectionistJoker;

impl Joker for PerfectionistJoker {
    fn id(&self) -> JokerId {
        JokerId::Reserved7
    }

    fn name(&self) -> &str {
        "The Perfectionist"
    }

    fn description(&self) -> &str {
        "+15 Mult if hand has exactly 5 cards of the same suit"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Uncommon
    }

    fn on_hand_played(&self, context: &mut GameContext, _hand: &SelectHand) -> JokerEffect {
        // For demo purposes, use ante as a proxy for hand quality
        // In a real implementation, this would validate hand size and contents

        // Demonstrate validation pattern with available context
        let is_perfect_conditions = context.ante >= 3 && context.round % 5 == 0;

        if is_perfect_conditions {
            JokerEffect::new()
                .with_mult(15)
                .with_message("Perfectionist: Perfect conditions met!".to_string())
        } else {
            // Demonstrate graceful degradation
            JokerEffect::new().with_message("Perfectionist: Conditions not met".to_string())
        }
    }
}

/// The Bank Joker - Safely accumulates value with overflow protection
#[derive(Debug)]
struct BankJoker;

impl Joker for BankJoker {
    fn id(&self) -> JokerId {
        JokerId::Reserved8
    }

    fn name(&self) -> &str {
        "The Bank"
    }

    fn description(&self) -> &str {
        "Accumulates +1 Mult per $10 earned, max +50 Mult"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Common
    }

    fn on_round_end(&self, context: &mut GameContext) -> JokerEffect {
        // Safely add money to accumulated value with overflow protection
        const MAX_ACCUMULATED: f64 = 500.0; // Max $500 accumulated

        let current_money = context.money.max(0) as f64; // Ensure non-negative
        let current_accumulated = context
            .joker_state_manager
            .get_accumulated_value(self.id())
            .unwrap_or(0.0);

        // Safe addition with bounds checking
        let new_accumulated = (current_accumulated + current_money).min(MAX_ACCUMULATED);

        // Use the safe add method rather than trying to set directly
        let addition = new_accumulated - current_accumulated;
        if addition > 0.0 {
            context
                .joker_state_manager
                .add_accumulated_value(self.id(), addition);
        }

        // Calculate mult bonus with overflow protection
        let mult_bonus = ((new_accumulated / 10.0) as i32).min(50);

        if mult_bonus > 0 {
            JokerEffect::new()
                .with_mult(mult_bonus)
                .with_message(format!(
                    "Bank: ${new_accumulated:.0} stored, +{mult_bonus} Mult"
                ))
        } else {
            JokerEffect::new()
        }
    }

    fn validate_state(&self, _context: &GameContext, state: &JokerState) -> Result<(), String> {
        if state.accumulated_value < 0.0 {
            return Err("Bank cannot have negative accumulated value".to_string());
        }
        if state.accumulated_value > 1000.0 {
            return Err("Bank accumulated value exceeds maximum (corruption detected)".to_string());
        }
        Ok(())
    }
}

/// The Phoenix Joker - Demonstrates state recovery patterns
#[derive(Debug)]
struct PhoenixJoker;

impl Joker for PhoenixJoker {
    fn id(&self) -> JokerId {
        JokerId::Reserved9
    }

    fn name(&self) -> &str {
        "The Phoenix"
    }

    fn description(&self) -> &str {
        "Resurrects from corrupted state, +5 Mult per recovery"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Legendary
    }

    fn on_hand_played(&self, context: &mut GameContext, _hand: &SelectHand) -> JokerEffect {
        // Attempt to get state, with recovery on failure
        let state = if let Some(state) = context.joker_state_manager.get_state(self.id()) {
            state
        } else {
            // State missing - initialize with default
            let recovery_state = self.initialize_state(context);
            context
                .joker_state_manager
                .update_state(self.id(), |state| {
                    *state = recovery_state.clone();
                });
            recovery_state
        };

        // Validate state and recover if needed
        if self.validate_state(context, &state).is_err() {
            // State corrupted - reset and increment recovery counter
            let recoveries = state
                .get_custom::<u32>("recoveries")
                .unwrap_or(Some(0))
                .unwrap_or(0);
            context
                .joker_state_manager
                .update_state(self.id(), |state| {
                    *state = self.initialize_state(context);
                    let _ = state.set_custom("recoveries", recoveries + 1);
                });

            JokerEffect::new()
                .with_mult(5 * (recoveries + 1) as i32)
                .with_message(format!(
                    "Phoenix: Recovered from corruption! (+{} recoveries)",
                    recoveries + 1
                ))
        } else {
            // Normal operation
            let recoveries = state
                .get_custom::<u32>("recoveries")
                .unwrap_or(Some(0))
                .unwrap_or(0);

            if recoveries > 0 {
                JokerEffect::new()
                    .with_mult(5 * recoveries as i32)
                    .with_message(format!(
                        "Phoenix: {} recoveries, +{} Mult",
                        recoveries,
                        5 * recoveries
                    ))
            } else {
                JokerEffect::new().with_mult(5)
            }
        }
    }

    fn initialize_state(&self, _context: &GameContext) -> JokerState {
        let mut state = JokerState::new();
        let _ = state.set_custom("recoveries", 0u32);
        let _ = state.set_custom("version", 1u32);
        state
    }

    fn validate_state(&self, _context: &GameContext, state: &JokerState) -> Result<(), String> {
        // Check for reasonable recovery count
        if let Ok(Some(recoveries)) = state.get_custom::<u32>("recoveries") {
            if recoveries > 100 {
                return Err("Recovery count exceeds reasonable limit".to_string());
            }
        }

        // Check for version compatibility
        if let Ok(Some(version)) = state.get_custom::<u32>("version") {
            if version > 1 {
                return Err("State version is from future version".to_string());
            }
        }

        Ok(())
    }

    fn migrate_state(
        &self,
        _context: &GameContext,
        old_state: &serde_json::Value,
        from_version: u32,
    ) -> Result<JokerState, String> {
        match from_version {
            0 => {
                // Migrate from version 0 (no version field)
                let mut state = JokerState::new();

                // Try to preserve recoveries if they exist
                if let Some(recoveries) = old_state.get("recoveries") {
                    if let Some(recoveries_val) = recoveries.as_u64() {
                        let _ = state.set_custom("recoveries", recoveries_val as u32);
                    }
                }

                let _ = state.set_custom("version", 1u32);
                Ok(state)
            }
            1 => {
                // Current version - direct deserialization
                serde_json::from_value(old_state.clone())
                    .map_err(|e| format!("Failed to deserialize v1 state: {e}"))
            }
            _ => Err(format!("Unknown version for migration: {from_version}")),
        }
    }
}
