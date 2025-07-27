//! # Trait Composition Examples
//!
//! This example demonstrates how to create complex jokers using multiple traits
//! and trait composition patterns. These examples show advanced joker implementations
//! that combine different behaviors and demonstrate the flexibility of the trait system.

use balatro_rs::{
    card::{Card, Suit, Value},
    hand::SelectHand,
    joker::{GameContext, Joker, JokerEffect, JokerId, JokerRarity},
    joker_state::JokerState,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Trait Composition Examples ===\n");

    // Example 1: Complex scoring joker with multiple conditions
    complex_scoring_joker()?;

    // Example 2: State-tracking joker with lifecycle management
    state_tracking_joker()?;

    // Example 3: Modifier joker that affects game mechanics
    modifier_joker()?;

    // Example 4: Event-driven joker with multiple triggers
    event_driven_joker()?;

    // Example 5: Dynamic joker that evolves over time
    dynamic_joker()?;

    Ok(())
}

/// Example 1: Complex scoring joker that combines multiple scoring conditions
fn complex_scoring_joker() -> Result<(), Box<dyn std::error::Error>> {
    println!("## 1. Complex Scoring Joker - The Aristocrat\n");

    let joker = AristocratJoker;
    println!("Name: {}", joker.name());
    println!("Description: {}", joker.description());
    println!("Features:");
    println!("  - Per-card scoring based on face cards");
    println!("  - Hand-type bonus for royal combinations");
    println!("  - Accumulating mult bonus over time");
    println!("  - Shop interaction for value increases");
    println!();

    Ok(())
}

/// Example 2: State-tracking joker with complex lifecycle management
fn state_tracking_joker() -> Result<(), Box<dyn std::error::Error>> {
    println!("## 2. State-Tracking Joker - The Scholar\n");

    let joker = ScholarJoker;
    println!("Name: {}", joker.name());
    println!("Description: {}", joker.description());
    println!("Features:");
    println!("  - Tracks hand types played this game");
    println!("  - Provides escalating bonuses");
    println!("  - Stores persistent state across rounds");
    println!("  - Custom serialization logic");
    println!();

    Ok(())
}

/// Example 3: Modifier joker that affects core game mechanics
fn modifier_joker() -> Result<(), Box<dyn std::error::Error>> {
    println!("## 3. Modifier Joker - The Magician\n");

    let joker = MagicianJoker;
    println!("Name: {}", joker.name());
    println!("Description: {}", joker.description());
    println!("Features:");
    println!("  - Modifies hand size (+2 cards)");
    println!("  - Affects discard count (+1 discard)");
    println!("  - Alters base scoring multipliers");
    println!("  - Demonstrates modifier trait usage");
    println!();

    Ok(())
}

/// Example 4: Event-driven joker with multiple trigger conditions
fn event_driven_joker() -> Result<(), Box<dyn std::error::Error>> {
    println!("## 4. Event-Driven Joker - The Reactor\n");

    let joker = ReactorJoker;
    println!("Name: {}", joker.name());
    println!("Description: {}", joker.description());
    println!("Features:");
    println!("  - Responds to blind start events");
    println!("  - Reacts to shop interactions");
    println!("  - Triggers on discard actions");
    println!("  - Provides end-of-round effects");
    println!();

    Ok(())
}

/// Example 5: Dynamic joker that evolves its behavior over time
fn dynamic_joker() -> Result<(), Box<dyn std::error::Error>> {
    println!("## 5. Dynamic Joker - The Shapeshifter\n");

    let joker = ShapeshifterJoker;
    println!("Name: {}", joker.name());
    println!("Description: {}", joker.description());
    println!("Features:");
    println!("  - Changes behavior based on ante progression");
    println!("  - Adapts to game state dynamically");
    println!("  - Multiple scoring modes");
    println!("  - Demonstrates complex state evolution");
    println!();

    Ok(())
}

/// The Aristocrat Joker - Complex scoring with multiple conditions
#[derive(Debug)]
struct AristocratJoker;

impl Joker for AristocratJoker {
    fn id(&self) -> JokerId {
        JokerId::Reserved // Using reserved slot for examples
    }

    fn name(&self) -> &str {
        "The Aristocrat"
    }

    fn description(&self) -> &str {
        "+3 Mult per face card, +10 Mult for royal flush, grows with shops visited"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Rare
    }

    fn cost(&self) -> usize {
        12 // Custom pricing for rare joker
    }

    // Per-card scoring for face cards
    fn on_card_scored(&self, context: &mut GameContext, card: &Card) -> JokerEffect {
        let is_face_card = matches!(card.value, Value::Jack | Value::Queen | Value::King);

        if is_face_card {
            // Get accumulated bonus from shops visited
            let shops_bonus = context
                .joker_state_manager
                .get_accumulated_value(self.id())
                .unwrap_or(0.0) as i32;

            JokerEffect::new().with_mult(3 + shops_bonus)
        } else {
            JokerEffect::new()
        }
    }

    // Hand-type bonus (simplified for public API)
    fn on_hand_played(&self, _context: &mut GameContext, _hand: &SelectHand) -> JokerEffect {
        // Provide a base bonus for playing any hand
        // In a real implementation, this would check hand types via game context
        JokerEffect::new().with_mult(5)
    }

    // Accumulate value when visiting shops
    fn on_shop_open(&self, context: &mut GameContext) -> JokerEffect {
        // Increment accumulated value for future bonuses
        context
            .joker_state_manager
            .add_accumulated_value(self.id(), 1.0);

        JokerEffect::new().with_message("The Aristocrat gains power...".to_string())
    }
}

/// The Scholar Joker - Tracks hand types and provides escalating bonuses
#[derive(Debug)]
struct ScholarJoker;

impl Joker for ScholarJoker {
    fn id(&self) -> JokerId {
        JokerId::Reserved2
    }

    fn name(&self) -> &str {
        "The Scholar"
    }

    fn description(&self) -> &str {
        "+2 Mult per unique hand type played this game"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Uncommon
    }

    fn on_hand_played(&self, context: &mut GameContext, _hand: &SelectHand) -> JokerEffect {
        // Count unique hand types played
        let unique_types = context.hand_type_counts.len() as i32;

        // Bonus scales with unique hand types discovered
        let bonus = unique_types * 2;

        JokerEffect::new()
            .with_mult(bonus)
            .with_message(format!("Scholar: {unique_types} unique hands discovered!"))
    }

    fn initialize_state(&self, _context: &GameContext) -> JokerState {
        let mut state = JokerState::new();
        // Initialize with zero hand types discovered
        let _ = state.set_custom("unique_hand_types", 0i32);
        state
    }

    fn validate_state(&self, _context: &GameContext, state: &JokerState) -> Result<(), String> {
        // Validate that unique hand types is non-negative
        if let Ok(Some(types)) = state.get_custom::<i32>("unique_hand_types") {
            if types < 0 {
                return Err("Unique hand types cannot be negative".to_string());
            }
        }
        Ok(())
    }
}

/// The Magician Joker - Modifies core game mechanics
#[derive(Debug)]
struct MagicianJoker;

impl Joker for MagicianJoker {
    fn id(&self) -> JokerId {
        JokerId::Reserved3
    }

    fn name(&self) -> &str {
        "The Magician"
    }

    fn description(&self) -> &str {
        "+2 hand size, +1 discard, x1.1 Mult"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Legendary
    }

    // Modify hand size
    fn modify_hand_size(&self, _context: &GameContext, base_size: usize) -> usize {
        base_size + 2
    }

    // Modify discard count
    fn modify_discards(&self, _context: &GameContext, base_discards: usize) -> usize {
        base_discards + 1
    }

    // Apply multiplicative bonus to scoring
    fn on_hand_played(&self, _context: &mut GameContext, _hand: &SelectHand) -> JokerEffect {
        JokerEffect::new().with_mult_multiplier(1.1)
    }

    fn on_activated(&self, _context: &mut GameContext) -> JokerEffect {
        JokerEffect::new().with_message("Magic enhances your abilities!".to_string())
    }
}

/// The Reactor Joker - Responds to multiple game events
#[derive(Debug)]
struct ReactorJoker;

impl Joker for ReactorJoker {
    fn id(&self) -> JokerId {
        JokerId::Reserved4
    }

    fn name(&self) -> &str {
        "The Reactor"
    }

    fn description(&self) -> &str {
        "Provides different bonuses based on game events"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Rare
    }

    // Energy burst at blind start
    fn on_blind_start(&self, _context: &mut GameContext) -> JokerEffect {
        JokerEffect::new()
            .with_chips(20)
            .with_message("Reactor: Energy surge!".to_string())
    }

    // Scoring based on current energy level
    fn on_hand_played(&self, context: &mut GameContext, _hand: &SelectHand) -> JokerEffect {
        // Energy decreases with each hand played
        let energy = 10 - (context.hands_played as i32).min(10);
        if energy > 0 {
            JokerEffect::new().with_mult(energy)
        } else {
            JokerEffect::new()
        }
    }

    // Gain money from discarding
    fn on_discard(&self, _context: &mut GameContext, cards: &[Card]) -> JokerEffect {
        let discard_bonus = cards.len() as i32;
        JokerEffect::new().with_money(discard_bonus)
    }

    // Recharge at shop
    fn on_shop_open(&self, _context: &mut GameContext) -> JokerEffect {
        JokerEffect::new().with_message("Reactor: Recharging...".to_string())
    }

    // End of round stabilization
    fn on_round_end(&self, _context: &mut GameContext) -> JokerEffect {
        JokerEffect::new()
            .with_mult(5)
            .with_message("Reactor: Stabilizing...".to_string())
    }
}

/// The Shapeshifter Joker - Changes behavior dynamically
#[derive(Debug)]
struct ShapeshifterJoker;

impl Joker for ShapeshifterJoker {
    fn id(&self) -> JokerId {
        JokerId::Reserved5
    }

    fn name(&self) -> &str {
        "The Shapeshifter"
    }

    fn description(&self) -> &str {
        "Adapts behavior based on ante: Early=Chips, Mid=Mult, Late=Money"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Legendary
    }

    fn on_hand_played(&self, context: &mut GameContext, _hand: &SelectHand) -> JokerEffect {
        match context.ante {
            1..=2 => {
                // Early game: Focus on chips
                JokerEffect::new()
                    .with_chips(30)
                    .with_message("Shapeshifter: Chip form!".to_string())
            }
            3..=5 => {
                // Mid game: Focus on mult
                JokerEffect::new()
                    .with_mult(8)
                    .with_message("Shapeshifter: Mult form!".to_string())
            }
            _ => {
                // Late game: Focus on economy
                JokerEffect::new()
                    .with_money(3)
                    .with_message("Shapeshifter: Economic form!".to_string())
            }
        }
    }

    fn on_card_scored(&self, context: &mut GameContext, card: &Card) -> JokerEffect {
        // Adapt per-card effects based on game state
        let adaptation_bonus = match (context.ante, context.round % 3) {
            (1..=2, 0) => JokerEffect::new().with_chips(5), // Chip focus, round mod 0
            (1..=2, _) => JokerEffect::new().with_chips(2), // Chip focus, other rounds
            (3..=5, 0) => JokerEffect::new().with_mult(2),  // Mult focus, round mod 0
            (3..=5, _) => JokerEffect::new().with_mult(1),  // Mult focus, other rounds
            (_, 0) => JokerEffect::new().with_money(1),     // Money focus, round mod 0
            _ => JokerEffect::new(),                        // No bonus other times
        };

        // Add suit-based conditional bonus
        match card.suit {
            Suit::Spade if context.ante >= 6 => JokerEffect::new()
                .with_mult(3)
                .with_chips(adaptation_bonus.chips)
                .with_money(adaptation_bonus.money),
            _ => adaptation_bonus,
        }
    }

    fn initialize_state(&self, context: &GameContext) -> JokerState {
        let mut state = JokerState::new();
        // Track the current form based on ante
        let form = match context.ante {
            1..=2 => "chip",
            3..=5 => "mult",
            _ => "economy",
        };
        let _ = state.set_custom("current_form", form.to_string());
        state
    }
}
