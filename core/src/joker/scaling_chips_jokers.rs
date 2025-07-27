//! Scaling Chips Jokers implementation
//!
//! This module implements jokers that accumulate chip bonuses over time
//! based on various game triggers.

use crate::{
    card::Card,
    hand::SelectHand,
    joker::{
        traits::{JokerState, ProcessContext, ProcessResult, Rarity},
        GameContext, Joker, JokerEffect, JokerGameplay, JokerId, JokerIdentity, JokerLifecycle,
        JokerRarity,
    },
    stage::Stage,
};
use serde_json;

/// Square Joker - Gains +4 chips per hand played with exactly 4 cards
/// This is a true scaling joker that accumulates value over time
#[derive(Debug, Clone)]
pub struct SquareJoker {
    id: JokerId,
    name: String,
    description: String,
    rarity: JokerRarity,
    cost: usize,
    accumulated_chips: u32,
}

impl Default for SquareJoker {
    fn default() -> Self {
        Self::new()
    }
}

impl SquareJoker {
    pub fn new() -> Self {
        Self {
            id: JokerId::Square,
            name: "Square Joker".to_string(),
            description: "Gains +4 Chips per 4-card hand played (Currently: +0)".to_string(),
            rarity: JokerRarity::Common,
            cost: 4,
            accumulated_chips: 0,
        }
    }

    fn update_description(&mut self) {
        self.description = format!(
            "Gains +4 Chips per 4-card hand played (Currently: +{})",
            self.accumulated_chips
        );
    }
}

impl JokerIdentity for SquareJoker {
    fn joker_type(&self) -> &'static str {
        "square"
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn rarity(&self) -> Rarity {
        match self.rarity {
            JokerRarity::Common => Rarity::Common,
            JokerRarity::Uncommon => Rarity::Uncommon,
            JokerRarity::Rare => Rarity::Rare,
            JokerRarity::Legendary => Rarity::Legendary,
        }
    }

    fn base_cost(&self) -> u64 {
        self.cost as u64
    }
}

impl Joker for SquareJoker {
    fn id(&self) -> JokerId {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn rarity(&self) -> JokerRarity {
        self.rarity
    }

    fn cost(&self) -> usize {
        self.cost
    }

    fn on_hand_played(&self, _context: &mut GameContext, hand: &SelectHand) -> JokerEffect {
        // Check if hand has exactly 4 cards
        if hand.len() == 4 {
            // Note: The state update happens in JokerGameplay::process
            // This just returns the current effect
            JokerEffect::new()
                .with_chips(self.accumulated_chips as i32)
                .with_message(format!(
                    "Square Joker: +{} Chips (4-card hand)",
                    self.accumulated_chips
                ))
        } else {
            // Still provide accumulated chips even if not triggering
            if self.accumulated_chips > 0 {
                JokerEffect::new()
                    .with_chips(self.accumulated_chips as i32)
                    .with_message(format!("Square Joker: +{} Chips", self.accumulated_chips))
            } else {
                JokerEffect::new()
            }
        }
    }
}

impl JokerGameplay for SquareJoker {
    fn process(&mut self, stage: &Stage, context: &mut ProcessContext) -> ProcessResult {
        if !matches!(stage, Stage::Blind(_)) {
            return ProcessResult::default();
        }

        // Count cards in the played hand
        let card_count = context.played_cards.len();

        // If exactly 4 cards, increment accumulated value
        if card_count == 4 {
            self.accumulated_chips += 4;
            self.update_description();

            ProcessResult {
                chips_added: 4,
                mult_added: 0.0,
                mult_multiplier: 1.0,
                retriggered: false,
                message: None,
            }
        } else if self.accumulated_chips > 0 {
            // Provide accumulated chips even when not incrementing
            ProcessResult {
                chips_added: self.accumulated_chips as u64,
                mult_added: 0.0,
                mult_multiplier: 1.0,
                retriggered: false,
                message: None,
            }
        } else {
            ProcessResult::default()
        }
    }

    fn can_trigger(&self, stage: &Stage, context: &ProcessContext) -> bool {
        matches!(stage, Stage::Blind(_))
            && (context.played_cards.len() == 4 || self.accumulated_chips > 0)
    }
}

impl JokerState for SquareJoker {
    fn has_state(&self) -> bool {
        true
    }

    fn serialize_state(&self) -> Option<serde_json::Value> {
        Some(serde_json::json!({
            "accumulated_chips": self.accumulated_chips
        }))
    }

    fn deserialize_state(&mut self, value: serde_json::Value) -> Result<(), String> {
        if let Some(chips) = value.get("accumulated_chips").and_then(|v| v.as_u64()) {
            self.accumulated_chips = chips as u32;
            self.update_description();
            Ok(())
        } else {
            Err("Invalid state format for Square Joker".to_string())
        }
    }

    fn debug_state(&self) -> String {
        format!("accumulated_chips: {}", self.accumulated_chips)
    }

    fn reset_state(&mut self) {
        self.accumulated_chips = 0;
        self.update_description();
    }
}

/// Marble Joker - Gains +50 chips per joker sold
#[derive(Debug, Clone)]
pub struct MarbleJoker {
    id: JokerId,
    name: String,
    description: String,
    rarity: JokerRarity,
    cost: usize,
    chips_per_joker_sold: u32,
    accumulated_chips: u32,
}

impl Default for MarbleJoker {
    fn default() -> Self {
        Self::new()
    }
}

impl MarbleJoker {
    pub fn new() -> Self {
        Self {
            id: JokerId::MarbleJoker,
            name: "Marble Joker".to_string(),
            description: "Gains +50 Chips per Joker sold (Currently: +0)".to_string(),
            rarity: JokerRarity::Rare,
            cost: 8,
            chips_per_joker_sold: 50,
            accumulated_chips: 0,
        }
    }

    fn update_description(&mut self) {
        self.description = format!(
            "Gains +50 Chips per Joker sold (Currently: +{})",
            self.accumulated_chips
        );
    }

    pub fn on_joker_sold(&mut self) {
        self.accumulated_chips += self.chips_per_joker_sold;
        self.update_description();
    }
}

impl JokerIdentity for MarbleJoker {
    fn joker_type(&self) -> &'static str {
        "marble"
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn rarity(&self) -> Rarity {
        match self.rarity {
            JokerRarity::Common => Rarity::Common,
            JokerRarity::Uncommon => Rarity::Uncommon,
            JokerRarity::Rare => Rarity::Rare,
            JokerRarity::Legendary => Rarity::Legendary,
        }
    }

    fn base_cost(&self) -> u64 {
        self.cost as u64
    }
}

impl Joker for MarbleJoker {
    fn id(&self) -> JokerId {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn rarity(&self) -> JokerRarity {
        self.rarity
    }

    fn cost(&self) -> usize {
        self.cost
    }

    fn on_hand_played(&self, _context: &mut GameContext, _hand: &SelectHand) -> JokerEffect {
        if self.accumulated_chips > 0 {
            JokerEffect::new()
                .with_chips(self.accumulated_chips as i32)
                .with_message(format!("Marble Joker: +{} Chips", self.accumulated_chips))
        } else {
            JokerEffect::new()
        }
    }
}

impl JokerGameplay for MarbleJoker {
    fn process(&mut self, stage: &Stage, _context: &mut ProcessContext) -> ProcessResult {
        if !matches!(stage, Stage::Blind(_)) {
            return ProcessResult::default();
        }

        if self.accumulated_chips > 0 {
            ProcessResult {
                chips_added: self.accumulated_chips as u64,
                mult_added: 0.0,
                mult_multiplier: 1.0,
                retriggered: false,
                message: None,
            }
        } else {
            ProcessResult::default()
        }
    }

    fn can_trigger(&self, stage: &Stage, _context: &ProcessContext) -> bool {
        matches!(stage, Stage::Blind(_)) && self.accumulated_chips > 0
    }
}

impl JokerState for MarbleJoker {
    fn has_state(&self) -> bool {
        true
    }

    fn serialize_state(&self) -> Option<serde_json::Value> {
        Some(serde_json::json!({
            "accumulated_chips": self.accumulated_chips
        }))
    }

    fn deserialize_state(&mut self, value: serde_json::Value) -> Result<(), String> {
        if let Some(chips) = value.get("accumulated_chips").and_then(|v| v.as_u64()) {
            self.accumulated_chips = chips as u32;
            self.update_description();
            Ok(())
        } else {
            Err("Invalid state format for Marble Joker".to_string())
        }
    }

    fn debug_state(&self) -> String {
        format!("accumulated_chips: {}", self.accumulated_chips)
    }

    fn reset_state(&mut self) {
        self.accumulated_chips = 0;
        self.update_description();
    }
}

/// Castle - Gains +300 chips per discard used this round (resets each round)
#[derive(Debug, Clone)]
pub struct CastleJoker {
    id: JokerId,
    name: String,
    description: String,
    rarity: JokerRarity,
    cost: usize,
    chips_per_discard: u32,
    current_round_chips: u32,
    max_chips: u32,
}

impl Default for CastleJoker {
    fn default() -> Self {
        Self::new()
    }
}

impl CastleJoker {
    pub fn new() -> Self {
        Self {
            id: JokerId::Reserved3,
            name: "Castle".to_string(),
            description: "+300 Chips per discard used this round (Currently: +0, Max: 1200)"
                .to_string(),
            rarity: JokerRarity::Rare,
            cost: 8,
            chips_per_discard: 300,
            current_round_chips: 0,
            max_chips: 1200, // Max 4 discards
        }
    }

    fn update_description(&mut self) {
        self.description = format!(
            "+300 Chips per discard used this round (Currently: +{}, Max: {})",
            self.current_round_chips, self.max_chips
        );
    }

    pub fn on_cards_discarded(&mut self, num_cards: usize) {
        if num_cards > 0 && self.current_round_chips < self.max_chips {
            self.current_round_chips =
                (self.current_round_chips + self.chips_per_discard).min(self.max_chips);
            self.update_description();
        }
    }
}

impl JokerIdentity for CastleJoker {
    fn joker_type(&self) -> &'static str {
        "castle"
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn rarity(&self) -> Rarity {
        match self.rarity {
            JokerRarity::Common => Rarity::Common,
            JokerRarity::Uncommon => Rarity::Uncommon,
            JokerRarity::Rare => Rarity::Rare,
            JokerRarity::Legendary => Rarity::Legendary,
        }
    }

    fn base_cost(&self) -> u64 {
        self.cost as u64
    }
}

impl Joker for CastleJoker {
    fn id(&self) -> JokerId {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn rarity(&self) -> JokerRarity {
        self.rarity
    }

    fn cost(&self) -> usize {
        self.cost
    }

    fn on_hand_played(&self, _context: &mut GameContext, _hand: &SelectHand) -> JokerEffect {
        if self.current_round_chips > 0 {
            JokerEffect::new()
                .with_chips(self.current_round_chips as i32)
                .with_message(format!("Castle: +{} Chips", self.current_round_chips))
        } else {
            JokerEffect::new()
        }
    }

    fn on_discard(&self, _context: &mut GameContext, cards: &[Card]) -> JokerEffect {
        // Note: State update happens through on_cards_discarded
        // This is just for immediate feedback
        if !cards.is_empty() && self.current_round_chips < self.max_chips {
            JokerEffect::new().with_message("Castle: Gaining chips from discard".to_string())
        } else {
            JokerEffect::new()
        }
    }

    fn on_round_end(&self, _context: &mut GameContext) -> JokerEffect {
        // Note: State reset happens in JokerLifecycle
        JokerEffect::new()
    }
}

impl JokerGameplay for CastleJoker {
    fn process(&mut self, stage: &Stage, _context: &mut ProcessContext) -> ProcessResult {
        if !matches!(stage, Stage::Blind(_)) {
            return ProcessResult::default();
        }

        if self.current_round_chips > 0 {
            ProcessResult {
                chips_added: self.current_round_chips as u64,
                mult_added: 0.0,
                mult_multiplier: 1.0,
                retriggered: false,
                message: None,
            }
        } else {
            ProcessResult::default()
        }
    }

    fn can_trigger(&self, stage: &Stage, _context: &ProcessContext) -> bool {
        matches!(stage, Stage::Blind(_)) && self.current_round_chips > 0
    }
}

impl JokerLifecycle for CastleJoker {
    fn on_round_end(&mut self) {
        self.current_round_chips = 0;
        self.update_description();
    }
}

impl JokerState for CastleJoker {
    fn has_state(&self) -> bool {
        true
    }

    fn serialize_state(&self) -> Option<serde_json::Value> {
        Some(serde_json::json!({
            "current_round_chips": self.current_round_chips
        }))
    }

    fn deserialize_state(&mut self, value: serde_json::Value) -> Result<(), String> {
        if let Some(chips) = value.get("current_round_chips").and_then(|v| v.as_u64()) {
            self.current_round_chips = chips as u32;
            self.update_description();
            Ok(())
        } else {
            Err("Invalid state format for Castle".to_string())
        }
    }

    fn debug_state(&self) -> String {
        format!("current_round_chips: {}", self.current_round_chips)
    }

    fn reset_state(&mut self) {
        self.current_round_chips = 0;
        self.update_description();
    }
}

/// Factory functions for creating scaling chips jokers
pub fn create_square_joker() -> Box<dyn Joker> {
    Box::new(SquareJoker::new())
}

pub fn create_marble_joker() -> Box<dyn Joker> {
    Box::new(MarbleJoker::new())
}

pub fn create_castle_joker() -> Box<dyn Joker> {
    Box::new(CastleJoker::new())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::{Suit, Value};
    use crate::stage::Blind;

    #[test]
    fn test_square_joker_scaling() {
        let mut square = SquareJoker::new();

        // Test identity
        assert_eq!(square.joker_type(), "square");
        assert_eq!(JokerIdentity::name(&square), "Square Joker");
        assert_eq!(square.base_cost(), 4);

        // Test initial state
        assert_eq!(square.accumulated_chips, 0);
        assert!(JokerIdentity::description(&square).contains("Currently: +0"));

        // Test state management
        assert!(JokerState::has_state(&square));
        let state = JokerState::serialize_state(&square).unwrap();
        assert_eq!(state["accumulated_chips"], 0);
    }

    #[test]
    fn test_marble_joker_accumulation() {
        let mut marble = MarbleJoker::new();

        // Test identity
        assert_eq!(marble.joker_type(), "marble");
        assert_eq!(JokerIdentity::name(&marble), "Marble Joker");
        assert_eq!(JokerIdentity::rarity(&marble), Rarity::Rare);
        assert_eq!(marble.base_cost(), 8);

        // Simulate selling jokers
        marble.on_joker_sold();
        assert_eq!(marble.accumulated_chips, 50);
        assert!(JokerIdentity::description(&marble).contains("Currently: +50"));

        marble.on_joker_sold();
        assert_eq!(marble.accumulated_chips, 100);

        // Test serialization
        let state = JokerState::serialize_state(&marble).unwrap();
        assert_eq!(state["accumulated_chips"], 100);
    }

    #[test]
    fn test_castle_round_reset() {
        let mut castle = CastleJoker::new();

        // Test identity
        assert_eq!(castle.joker_type(), "castle");
        assert_eq!(JokerIdentity::name(&castle), "Castle");
        assert_eq!(JokerIdentity::rarity(&castle), Rarity::Rare);

        // Simulate discards
        castle.on_cards_discarded(3);
        assert_eq!(castle.current_round_chips, 300);

        castle.on_cards_discarded(2);
        assert_eq!(castle.current_round_chips, 600);

        // Test max cap
        castle.on_cards_discarded(10);
        castle.on_cards_discarded(10);
        assert_eq!(castle.current_round_chips, 1200); // Capped at max

        // Test round reset
        JokerLifecycle::on_round_end(&mut castle);
        assert_eq!(castle.current_round_chips, 0);
    }

    #[test]
    fn test_square_joker_gameplay_trait() {
        let mut square = SquareJoker::new();
        let stage = Stage::Blind(Blind::Small);

        // Create test context with 4 cards
        let cards = vec![
            Card::new(Value::Ace, Suit::Heart),
            Card::new(Value::King, Suit::Diamond),
            Card::new(Value::Queen, Suit::Spade),
            Card::new(Value::Jack, Suit::Club),
        ];

        let played_cards = cards;
        let held_cards = vec![];
        let mut events = vec![];
        let mut hand_score = crate::joker::traits::HandScore {
            chips: 0,
            mult: 0.0,
        };

        let joker_state_manager = crate::joker_state::JokerStateManager::new();

        let mut context = ProcessContext {
            hand_score: &mut hand_score,
            played_cards: &played_cards,
            held_cards: &held_cards,
            events: &mut events,
            joker_state_manager: &joker_state_manager,
        };

        // Test can_trigger with 4 cards
        assert!(square.can_trigger(&stage, &context));

        // Test process
        let result = square.process(&stage, &mut context);
        assert_eq!(result.chips_added, 4);
        assert_eq!(square.accumulated_chips, 4);

        // Process again
        let result = square.process(&stage, &mut context);
        assert_eq!(result.chips_added, 4);
        assert_eq!(square.accumulated_chips, 8);
    }
}
