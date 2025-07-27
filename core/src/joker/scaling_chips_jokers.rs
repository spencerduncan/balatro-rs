//! Scaling Chips Jokers implementation
//!
//! This module implements jokers that accumulate chip bonuses over time
//! based on various game triggers. Implementations match joker.json specifications exactly.

use crate::{
    card::{Card, Suit, Value},
    hand::SelectHand,
    joker::{
        traits::{JokerState, ProcessContext, ProcessResult, Rarity},
        GameContext, Joker, JokerEffect, JokerGameplay, JokerId, JokerIdentity, JokerLifecycle,
        JokerRarity,
    },
    stage::Stage,
};
use serde_json;

/// Castle Joker - Gains chips per discarded card of rotating suit
/// Specification from joker.json: "This Joker gains {C:chips}+#1#{} Chips
/// per discarded {V:1}#2#{} card, suit changes every round"
#[derive(Debug, Clone)]
pub struct CastleJoker {
    id: JokerId,
    name: String,
    description: String,
    rarity: JokerRarity,
    cost: usize,
    accumulated_chips: u32,
    chips_per_card: u32,
    current_suit: Suit,
}

impl Default for CastleJoker {
    fn default() -> Self {
        Self::new()
    }
}

impl CastleJoker {
    pub fn new() -> Self {
        Self {
            id: JokerId::Reserved3, // Using Reserved3 as per existing implementation
            name: "Castle".to_string(),
            description:
                "Gains +3 Chips per discarded Spade card, suit changes every round (Currently: +0)"
                    .to_string(),
            rarity: JokerRarity::Rare,
            cost: 8,
            accumulated_chips: 0,
            chips_per_card: 3,
            current_suit: Suit::Spade, // Start with Spades
        }
    }

    fn update_description(&mut self) {
        let suit_name = match self.current_suit {
            Suit::Spade => "Spade",
            Suit::Club => "Club",
            Suit::Heart => "Heart",
            Suit::Diamond => "Diamond",
        };
        self.description = format!(
            "Gains +{} Chips per discarded {} card, suit changes every round (Currently: +{})",
            self.chips_per_card, suit_name, self.accumulated_chips
        );
    }

    pub fn on_cards_discarded(&mut self, cards: &[Card]) {
        // Count cards of the current suit
        let matching_cards = cards
            .iter()
            .filter(|card| card.suit == self.current_suit)
            .count();

        if matching_cards > 0 {
            self.accumulated_chips += (matching_cards as u32) * self.chips_per_card;
            self.update_description();
        }
    }

    fn rotate_suit(&mut self) {
        // Rotate through suits each round: Spade -> Club -> Heart -> Diamond -> Spade
        self.current_suit = match self.current_suit {
            Suit::Spade => Suit::Club,
            Suit::Club => Suit::Heart,
            Suit::Heart => Suit::Diamond,
            Suit::Diamond => Suit::Spade,
        };
        self.update_description();
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
        if self.accumulated_chips > 0 {
            JokerEffect::new()
                .with_chips(self.accumulated_chips as i32)
                .with_message(format!("Castle: +{} Chips", self.accumulated_chips))
        } else {
            JokerEffect::new()
        }
    }

    fn on_discard(&self, _context: &mut GameContext, cards: &[Card]) -> JokerEffect {
        // Check if any discarded cards match current suit
        let matching_cards = cards
            .iter()
            .filter(|card| card.suit == self.current_suit)
            .count();

        if matching_cards > 0 {
            JokerEffect::new().with_message(format!(
                "Castle: Gaining +{} chips from {} {} cards",
                matching_cards * (self.chips_per_card as usize),
                matching_cards,
                match self.current_suit {
                    Suit::Spade => "Spade",
                    Suit::Club => "Club",
                    Suit::Heart => "Heart",
                    Suit::Diamond => "Diamond",
                }
            ))
        } else {
            JokerEffect::new()
        }
    }
}

impl JokerGameplay for CastleJoker {
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

impl JokerLifecycle for CastleJoker {
    fn on_round_end(&mut self) {
        // Rotate suit at end of round as per joker.json specification
        self.rotate_suit();
    }
}

impl JokerState for CastleJoker {
    fn has_state(&self) -> bool {
        true
    }

    fn serialize_state(&self) -> Option<serde_json::Value> {
        Some(serde_json::json!({
            "accumulated_chips": self.accumulated_chips,
            "current_suit": self.current_suit as u8
        }))
    }

    fn deserialize_state(&mut self, value: serde_json::Value) -> Result<(), String> {
        if let (Some(chips), Some(suit_val)) = (
            value.get("accumulated_chips").and_then(|v| v.as_u64()),
            value.get("current_suit").and_then(|v| v.as_u64()),
        ) {
            self.accumulated_chips = chips as u32;
            self.current_suit = match suit_val {
                0 => Suit::Spade,
                1 => Suit::Club,
                2 => Suit::Heart,
                3 => Suit::Diamond,
                _ => return Err("Invalid suit value in Castle state".to_string()),
            };
            self.update_description();
            Ok(())
        } else {
            Err("Invalid state format for Castle".to_string())
        }
    }

    fn debug_state(&self) -> String {
        format!(
            "accumulated_chips: {}, current_suit: {:?}",
            self.accumulated_chips, self.current_suit
        )
    }

    fn reset_state(&mut self) {
        self.accumulated_chips = 0;
        self.current_suit = Suit::Spade;
        self.update_description();
    }
}

/// Wee Joker - Gains chips when 2s are played
/// Specification from joker.json: "This Joker gains {C:chips}+#2#{} Chips when each
/// played {C:attention}2{} is scored"
#[derive(Debug, Clone)]
pub struct WeeJoker {
    id: JokerId,
    name: String,
    description: String,
    rarity: JokerRarity,
    cost: usize,
    accumulated_chips: u32,
    chips_per_two: u32,
}

impl Default for WeeJoker {
    fn default() -> Self {
        Self::new()
    }
}

impl WeeJoker {
    pub fn new() -> Self {
        Self {
            id: JokerId::Wee,
            name: "Wee Joker".to_string(),
            description: "Gains +8 Chips when each played 2 is scored (Currently: +0)".to_string(),
            rarity: JokerRarity::Common,
            cost: 4,
            accumulated_chips: 0,
            chips_per_two: 8,
        }
    }

    fn update_description(&mut self) {
        self.description = format!(
            "Gains +{} Chips when each played 2 is scored (Currently: +{})",
            self.chips_per_two, self.accumulated_chips
        );
    }

    pub fn on_two_scored(&mut self) {
        self.accumulated_chips += self.chips_per_two;
        self.update_description();
    }
}

impl JokerIdentity for WeeJoker {
    fn joker_type(&self) -> &'static str {
        "wee"
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

impl Joker for WeeJoker {
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
                .with_message(format!("Wee Joker: +{} Chips", self.accumulated_chips))
        } else {
            JokerEffect::new()
        }
    }

    fn on_card_scored(&self, _context: &mut GameContext, card: &Card) -> JokerEffect {
        if card.value == Value::Two {
            JokerEffect::new().with_message(format!(
                "Wee Joker: 2 scored! Gaining +{} chips",
                self.chips_per_two
            ))
        } else {
            JokerEffect::new()
        }
    }
}

impl JokerGameplay for WeeJoker {
    fn process(&mut self, stage: &Stage, context: &mut ProcessContext) -> ProcessResult {
        if !matches!(stage, Stage::Blind(_)) {
            return ProcessResult::default();
        }

        // Check for 2s in played cards and accumulate
        let twos_count = context
            .played_cards
            .iter()
            .filter(|card| card.value == Value::Two)
            .count();

        if twos_count > 0 {
            let chips_to_add = (twos_count as u32) * self.chips_per_two;
            self.accumulated_chips += chips_to_add;
            self.update_description();

            ProcessResult {
                chips_added: chips_to_add as u64,
                mult_added: 0.0,
                mult_multiplier: 1.0,
                retriggered: false,
                message: Some(format!(
                    "Wee Joker: {twos_count} 2s scored, +{chips_to_add} chips!"
                )),
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
            && (context
                .played_cards
                .iter()
                .any(|card| card.value == Value::Two)
                || self.accumulated_chips > 0)
    }
}

impl JokerState for WeeJoker {
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
            Err("Invalid state format for Wee Joker".to_string())
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

/// Factory functions for creating scaling chips jokers
pub fn create_castle_joker() -> Box<dyn Joker> {
    Box::new(CastleJoker::new())
}

pub fn create_wee_joker() -> Box<dyn Joker> {
    Box::new(WeeJoker::new())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stage::Blind;

    #[test]
    fn test_castle_joker_identity() {
        let castle = CastleJoker::new();
        assert_eq!(castle.joker_type(), "castle");
        assert_eq!(JokerIdentity::name(&castle), "Castle");
        assert_eq!(JokerIdentity::rarity(&castle), Rarity::Rare);
        assert_eq!(castle.base_cost(), 8);
    }

    #[test]
    fn test_castle_suit_rotation() {
        let mut castle = CastleJoker::new();

        // Should start with Spades
        assert_eq!(castle.current_suit, Suit::Spade);
        assert!(castle.description.contains("Spade"));

        // After round end, should rotate to Clubs
        JokerLifecycle::on_round_end(&mut castle);
        assert_eq!(castle.current_suit, Suit::Club);
        assert!(castle.description.contains("Club"));

        // Continue rotation: Clubs -> Hearts -> Diamonds -> Spades
        JokerLifecycle::on_round_end(&mut castle);
        assert_eq!(castle.current_suit, Suit::Heart);

        JokerLifecycle::on_round_end(&mut castle);
        assert_eq!(castle.current_suit, Suit::Diamond);

        JokerLifecycle::on_round_end(&mut castle);
        assert_eq!(castle.current_suit, Suit::Spade);
    }

    #[test]
    fn test_castle_discard_accumulation() {
        let mut castle = CastleJoker::new();

        // Start with Spades, discard some spades
        let cards = vec![
            Card::new(Value::Ace, Suit::Spade),
            Card::new(Value::King, Suit::Spade),
            Card::new(Value::Queen, Suit::Heart), // Different suit - shouldn't count
        ];

        castle.on_cards_discarded(&cards);
        assert_eq!(castle.accumulated_chips, 6); // 2 spades * 3 chips each
        assert!(castle.description.contains("Currently: +6"));

        // Discard more spades
        let more_cards = vec![Card::new(Value::Jack, Suit::Spade)];
        castle.on_cards_discarded(&more_cards);
        assert_eq!(castle.accumulated_chips, 9); // Previous 6 + 3 more
    }

    #[test]
    fn test_castle_state_serialization() {
        let mut castle = CastleJoker::new();
        castle.accumulated_chips = 15;
        castle.current_suit = Suit::Diamond;
        castle.update_description();

        let state = JokerState::serialize_state(&castle).unwrap();
        assert_eq!(state["accumulated_chips"], 15);
        assert_eq!(state["current_suit"], 3); // Diamond = 3

        let mut new_castle = CastleJoker::new();
        JokerState::deserialize_state(&mut new_castle, state).unwrap();
        assert_eq!(new_castle.accumulated_chips, 15);
        assert_eq!(new_castle.current_suit, Suit::Diamond);
    }

    #[test]
    fn test_wee_joker_identity() {
        let wee = WeeJoker::new();
        assert_eq!(wee.joker_type(), "wee");
        assert_eq!(JokerIdentity::name(&wee), "Wee Joker");
        assert_eq!(JokerIdentity::rarity(&wee), Rarity::Common);
        assert_eq!(wee.base_cost(), 4);
    }

    #[test]
    fn test_wee_joker_two_scoring() {
        let mut wee = WeeJoker::new();
        let stage = Stage::Blind(Blind::Small);

        // Create test context with 2s
        let cards_with_twos = vec![
            Card::new(Value::Two, Suit::Heart),
            Card::new(Value::Two, Suit::Spade),
            Card::new(Value::King, Suit::Diamond), // Not a 2
        ];

        let held_cards = vec![];
        let mut events = vec![];
        let mut hand_score = crate::joker::traits::HandScore {
            chips: 0,
            mult: 0.0,
        };

        let joker_state_manager = crate::joker_state::JokerStateManager::new();

        let mut context = ProcessContext {
            hand_score: &mut hand_score,
            played_cards: &cards_with_twos,
            held_cards: &held_cards,
            events: &mut events,
            joker_state_manager: &joker_state_manager,
        };

        // Should trigger with 2s
        assert!(wee.can_trigger(&stage, &context));

        // Process should add chips for 2s
        let result = wee.process(&stage, &mut context);
        assert_eq!(result.chips_added, 16); // 2 twos * 8 chips each
        assert_eq!(wee.accumulated_chips, 16);
        assert!(wee.description.contains("Currently: +16"));
    }

    #[test]
    fn test_wee_joker_state_serialization() {
        let mut wee = WeeJoker::new();
        wee.accumulated_chips = 24;
        wee.update_description();

        let state = JokerState::serialize_state(&wee).unwrap();
        assert_eq!(state["accumulated_chips"], 24);

        let mut new_wee = WeeJoker::new();
        JokerState::deserialize_state(&mut new_wee, state).unwrap();
        assert_eq!(new_wee.accumulated_chips, 24);
        assert!(new_wee.description.contains("Currently: +24"));
    }

    #[test]
    fn test_wee_joker_gameplay_trait() {
        let mut wee = WeeJoker::new();
        let stage = Stage::Blind(Blind::Small);

        // Test with no 2s
        let cards_no_twos = vec![
            Card::new(Value::Ace, Suit::Heart),
            Card::new(Value::King, Suit::Diamond),
        ];

        let held_cards = vec![];
        let mut events = vec![];
        let mut hand_score = crate::joker::traits::HandScore {
            chips: 0,
            mult: 0.0,
        };

        let joker_state_manager = crate::joker_state::JokerStateManager::new();

        let mut context = ProcessContext {
            hand_score: &mut hand_score,
            played_cards: &cards_no_twos,
            held_cards: &held_cards,
            events: &mut events,
            joker_state_manager: &joker_state_manager,
        };

        // Should not trigger with no 2s and no accumulated chips
        assert!(!wee.can_trigger(&stage, &context));

        let result = wee.process(&stage, &mut context);
        assert_eq!(result.chips_added, 0);
        assert_eq!(wee.accumulated_chips, 0);
    }
}
