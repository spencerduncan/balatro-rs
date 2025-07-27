//! Basic XMult Jokers implementation
//!
//! This module implements jokers that provide multiplicative mult bonuses (X mult).
//! These jokers apply mult_multiplier effects under various conditions.
//!
//! Implements the 5 jokers from Issue #192:
//! - Photograph: X2 mult for first face card
//! - Ancient Joker: X1.5 mult for selected suit, changes each round
//! - Steel Joker: X0.2 mult per Steel card in deck
//! - Baron: X1.5 mult per King held in hand
//! - The Idol: X mult for specific rank+suit, changes each round

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
use rand::Rng;
use serde_json;

/// Photograph Joker - First played face card gives X1.5 Mult when scored
#[derive(Debug, Clone)]
pub struct PhotographJoker {
    id: JokerId,
    name: String,
    description: String,
    rarity: JokerRarity,
    cost: usize,
    face_card_played: bool,
}

impl Default for PhotographJoker {
    fn default() -> Self {
        Self::new()
    }
}

impl PhotographJoker {
    pub fn new() -> Self {
        Self {
            id: JokerId::Photograph,
            name: "Photograph".to_string(),
            description: "First played face card gives X1.5 Mult when scored".to_string(),
            rarity: JokerRarity::Common,
            cost: 5,
            face_card_played: false,
        }
    }

    fn is_face_card(card: &Card) -> bool {
        matches!(card.value, Value::Jack | Value::Queen | Value::King)
    }
}

impl JokerIdentity for PhotographJoker {
    fn joker_type(&self) -> &'static str {
        "photograph"
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

impl Joker for PhotographJoker {
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

    fn on_card_scored(&self, _context: &mut GameContext, card: &Card) -> JokerEffect {
        if !self.face_card_played && Self::is_face_card(card) {
            // Note: State update happens in JokerGameplay::process
            JokerEffect::new()
                .with_mult_multiplier(1.5)
                .with_message("Photograph: X1.5 Mult (first face card)".to_string())
        } else {
            JokerEffect::new()
        }
    }
}

impl JokerGameplay for PhotographJoker {
    fn process(&mut self, stage: &Stage, context: &mut ProcessContext) -> ProcessResult {
        if !matches!(stage, Stage::Blind(_)) {
            return ProcessResult::default();
        }

        // Check if we have face cards and haven't triggered yet
        if !self.face_card_played {
            for card in context.played_cards {
                if Self::is_face_card(card) {
                    self.face_card_played = true;
                    // Note: The actual X1.5 mult is applied via on_card_scored
                    // This just tracks the state
                    return ProcessResult::default();
                }
            }
        }

        ProcessResult::default()
    }

    fn can_trigger(&self, stage: &Stage, context: &ProcessContext) -> bool {
        matches!(stage, Stage::Blind(_))
            && !self.face_card_played
            && context.played_cards.iter().any(Self::is_face_card)
    }
}

impl JokerLifecycle for PhotographJoker {
    fn on_round_start(&mut self) {
        self.face_card_played = false;
    }
}

impl JokerState for PhotographJoker {
    fn has_state(&self) -> bool {
        true
    }

    fn serialize_state(&self) -> Option<serde_json::Value> {
        Some(serde_json::json!({
            "face_card_played": self.face_card_played
        }))
    }

    fn deserialize_state(&mut self, value: serde_json::Value) -> Result<(), String> {
        if let Some(played) = value.get("face_card_played").and_then(|v| v.as_bool()) {
            self.face_card_played = played;
            Ok(())
        } else {
            Err("Invalid state format for Photograph".to_string())
        }
    }

    fn debug_state(&self) -> String {
        format!("face_card_played: {}", self.face_card_played)
    }

    fn reset_state(&mut self) {
        self.face_card_played = false;
    }
}

/// Ancient Joker - X1.5 mult for selected suit, changes each round
#[derive(Debug, Clone)]
pub struct AncientJoker {
    id: JokerId,
    name: String,
    description: String,
    rarity: JokerRarity,
    cost: usize,
    selected_suit: Suit,
}

impl Default for AncientJoker {
    fn default() -> Self {
        Self::new()
    }
}

impl AncientJoker {
    pub fn new() -> Self {
        Self {
            id: JokerId::AncientJoker,
            name: "Ancient Joker".to_string(),
            description: "Each played card with selected suit gives X1.5 Mult when scored, suit changes at end of round".to_string(),
            rarity: JokerRarity::Rare,
            cost: 8,
            selected_suit: Suit::Heart, // Default to Hearts
        }
    }

    fn random_suit() -> Suit {
        match rand::thread_rng().gen_range(0..4) {
            0 => Suit::Heart,
            1 => Suit::Diamond,
            2 => Suit::Club,
            _ => Suit::Spade,
        }
    }
}

impl JokerIdentity for AncientJoker {
    fn joker_type(&self) -> &'static str {
        "ancient_joker"
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn rarity(&self) -> Rarity {
        Rarity::Rare
    }

    fn base_cost(&self) -> u64 {
        self.cost as u64
    }
}

impl Joker for AncientJoker {
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

    fn on_card_scored(&self, _context: &mut GameContext, card: &Card) -> JokerEffect {
        if card.suit == self.selected_suit {
            JokerEffect::new()
                .with_mult_multiplier(1.5)
                .with_message(format!(
                    "Ancient Joker: X1.5 Mult ({:?})",
                    self.selected_suit
                ))
        } else {
            JokerEffect::new()
        }
    }
}

impl JokerGameplay for AncientJoker {
    fn process(&mut self, stage: &Stage, _context: &mut ProcessContext) -> ProcessResult {
        if !matches!(stage, Stage::Blind(_)) {
            return ProcessResult::default();
        }

        // The actual multiplier is applied via on_card_scored
        ProcessResult::default()
    }

    fn can_trigger(&self, stage: &Stage, context: &ProcessContext) -> bool {
        matches!(stage, Stage::Blind(_))
            && context
                .played_cards
                .iter()
                .any(|card| card.suit == self.selected_suit)
    }
}

impl JokerLifecycle for AncientJoker {
    fn on_round_end(&mut self) {
        // Change suit at end of round
        self.selected_suit = Self::random_suit();
    }
}

impl JokerState for AncientJoker {
    fn has_state(&self) -> bool {
        true
    }

    fn serialize_state(&self) -> Option<serde_json::Value> {
        Some(serde_json::json!({
            "selected_suit": self.selected_suit
        }))
    }

    fn deserialize_state(&mut self, value: serde_json::Value) -> Result<(), String> {
        if let Some(suit_str) = value.get("selected_suit").and_then(|v| v.as_str()) {
            self.selected_suit = match suit_str {
                "Heart" => Suit::Heart,
                "Diamond" => Suit::Diamond,
                "Club" => Suit::Club,
                "Spade" => Suit::Spade,
                _ => return Err("Invalid suit in Ancient Joker state".to_string()),
            };
            Ok(())
        } else {
            Err("Invalid state format for Ancient Joker".to_string())
        }
    }

    fn debug_state(&self) -> String {
        format!("selected_suit: {:?}", self.selected_suit)
    }

    fn reset_state(&mut self) {
        self.selected_suit = Self::random_suit();
    }
}

/// Steel Joker - X0.2 mult per Steel card in deck
#[derive(Debug, Clone)]
pub struct SteelJoker {
    id: JokerId,
    name: String,
    description: String,
    rarity: JokerRarity,
    cost: usize,
}

impl Default for SteelJoker {
    fn default() -> Self {
        Self::new()
    }
}

impl SteelJoker {
    pub fn new() -> Self {
        Self {
            id: JokerId::SteelJoker,
            name: "Steel Joker".to_string(),
            description: "Gives X0.2 Mult for each Steel Card in your full deck".to_string(),
            rarity: JokerRarity::Uncommon,
            cost: 6,
        }
    }
}

impl JokerIdentity for SteelJoker {
    fn joker_type(&self) -> &'static str {
        "steel_joker"
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
    }

    fn base_cost(&self) -> u64 {
        self.cost as u64
    }
}

impl Joker for SteelJoker {
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
        // TODO: Count Steel cards in full deck when enhancement system is available
        // For now, use placeholder implementation like the existing Steel joker
        let steel_count = 0; // Placeholder - no steel cards for now

        if steel_count > 0 {
            let multiplier = 1.0 + (0.2 * steel_count as f64);
            JokerEffect::new()
                .with_mult_multiplier(multiplier)
                .with_message(format!(
                    "Steel Joker: X{multiplier:.1} Mult ({steel_count} Steel cards)"
                ))
        } else {
            JokerEffect::new()
        }
    }
}

impl JokerGameplay for SteelJoker {
    fn process(&mut self, stage: &Stage, _context: &mut ProcessContext) -> ProcessResult {
        if !matches!(stage, Stage::Blind(_)) {
            return ProcessResult::default();
        }

        // The actual multiplier is applied via on_hand_played
        ProcessResult::default()
    }

    fn can_trigger(&self, stage: &Stage, _context: &ProcessContext) -> bool {
        matches!(stage, Stage::Blind(_))
    }
}

/// Baron - X1.5 mult per King held in hand
#[derive(Debug, Clone)]
pub struct BaronJoker {
    id: JokerId,
    name: String,
    description: String,
    rarity: JokerRarity,
    cost: usize,
}

impl Default for BaronJoker {
    fn default() -> Self {
        Self::new()
    }
}

impl BaronJoker {
    pub fn new() -> Self {
        Self {
            id: JokerId::BaronJoker,
            name: "Baron".to_string(),
            description: "Each King held in hand gives X1.5 Mult".to_string(),
            rarity: JokerRarity::Rare,
            cost: 8,
        }
    }

    fn count_kings_in_hand(cards: &[Card]) -> usize {
        cards
            .iter()
            .filter(|card| card.value == Value::King)
            .count()
    }
}

impl JokerIdentity for BaronJoker {
    fn joker_type(&self) -> &'static str {
        "baron"
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn rarity(&self) -> Rarity {
        Rarity::Rare
    }

    fn base_cost(&self) -> u64 {
        self.cost as u64
    }
}

impl Joker for BaronJoker {
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

    fn on_hand_played(&self, context: &mut GameContext, _hand: &SelectHand) -> JokerEffect {
        // Count Kings in held cards (cards in hand that are NOT played)
        let held_cards: Vec<Card> = context.hand.cards().to_vec();
        let king_count = Self::count_kings_in_hand(&held_cards);

        if king_count > 0 {
            let multiplier = 1.5_f64.powi(king_count as i32);
            JokerEffect::new()
                .with_mult_multiplier(multiplier)
                .with_message(format!(
                    "Baron: X{multiplier:.1} Mult ({king_count} Kings held)"
                ))
        } else {
            JokerEffect::new()
        }
    }
}

impl JokerGameplay for BaronJoker {
    fn process(&mut self, stage: &Stage, _context: &mut ProcessContext) -> ProcessResult {
        if !matches!(stage, Stage::Blind(_)) {
            return ProcessResult::default();
        }

        // The actual multiplier is applied via on_hand_played
        ProcessResult::default()
    }

    fn can_trigger(&self, stage: &Stage, context: &ProcessContext) -> bool {
        matches!(stage, Stage::Blind(_)) && Self::count_kings_in_hand(context.held_cards) > 0
    }
}

/// The Idol - X mult for specific rank+suit, changes each round
#[derive(Debug, Clone)]
pub struct TheIdolJoker {
    id: JokerId,
    name: String,
    description: String,
    rarity: JokerRarity,
    cost: usize,
    selected_value: Value,
    selected_suit: Suit,
    multiplier: f64,
}

impl Default for TheIdolJoker {
    fn default() -> Self {
        Self::new()
    }
}

impl TheIdolJoker {
    pub fn new() -> Self {
        let (value, suit) = Self::random_card();
        Self {
            id: JokerId::TheIdol,
            name: "The Idol".to_string(),
            description: "Each played card of specific rank and suit gives X Mult when scored, card changes every round".to_string(),
            rarity: JokerRarity::Uncommon,
            cost: 6,
            selected_value: value,
            selected_suit: suit,
            multiplier: 2.0, // Default X2 mult, will vary
        }
    }

    fn random_card() -> (Value, Suit) {
        let value = match rand::thread_rng().gen_range(0..13) {
            0 => Value::Ace,
            1 => Value::Two,
            2 => Value::Three,
            3 => Value::Four,
            4 => Value::Five,
            5 => Value::Six,
            6 => Value::Seven,
            7 => Value::Eight,
            8 => Value::Nine,
            9 => Value::Ten,
            10 => Value::Jack,
            11 => Value::Queen,
            _ => Value::King,
        };

        let suit = match rand::thread_rng().gen_range(0..4) {
            0 => Suit::Heart,
            1 => Suit::Diamond,
            2 => Suit::Club,
            _ => Suit::Spade,
        };

        (value, suit)
    }

    fn random_multiplier() -> f64 {
        // Generate random multiplier between 1.5 and 3.0
        1.5 + (rand::thread_rng().gen::<f64>() * 1.5)
    }
}

impl JokerIdentity for TheIdolJoker {
    fn joker_type(&self) -> &'static str {
        "the_idol"
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
    }

    fn base_cost(&self) -> u64 {
        self.cost as u64
    }
}

impl Joker for TheIdolJoker {
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

    fn on_card_scored(&self, _context: &mut GameContext, card: &Card) -> JokerEffect {
        if card.value == self.selected_value && card.suit == self.selected_suit {
            JokerEffect::new()
                .with_mult_multiplier(self.multiplier)
                .with_message(format!(
                    "The Idol: X{:.1} Mult ({:?} of {:?})",
                    self.multiplier, self.selected_value, self.selected_suit
                ))
        } else {
            JokerEffect::new()
        }
    }
}

impl JokerGameplay for TheIdolJoker {
    fn process(&mut self, stage: &Stage, _context: &mut ProcessContext) -> ProcessResult {
        if !matches!(stage, Stage::Blind(_)) {
            return ProcessResult::default();
        }

        // The actual multiplier is applied via on_card_scored
        ProcessResult::default()
    }

    fn can_trigger(&self, stage: &Stage, context: &ProcessContext) -> bool {
        matches!(stage, Stage::Blind(_))
            && context
                .played_cards
                .iter()
                .any(|card| card.value == self.selected_value && card.suit == self.selected_suit)
    }
}

impl JokerLifecycle for TheIdolJoker {
    fn on_round_end(&mut self) {
        // Change card and multiplier at end of round
        let (value, suit) = Self::random_card();
        self.selected_value = value;
        self.selected_suit = suit;
        self.multiplier = Self::random_multiplier();
    }
}

impl JokerState for TheIdolJoker {
    fn has_state(&self) -> bool {
        true
    }

    fn serialize_state(&self) -> Option<serde_json::Value> {
        Some(serde_json::json!({
            "selected_value": self.selected_value,
            "selected_suit": self.selected_suit,
            "multiplier": self.multiplier
        }))
    }

    fn deserialize_state(&mut self, value: serde_json::Value) -> Result<(), String> {
        let value_str = value
            .get("selected_value")
            .and_then(|v| v.as_str())
            .ok_or("Missing selected_value")?;

        let suit_str = value
            .get("selected_suit")
            .and_then(|v| v.as_str())
            .ok_or("Missing selected_suit")?;

        let multiplier = value
            .get("multiplier")
            .and_then(|v| v.as_f64())
            .ok_or("Missing multiplier")?;

        self.selected_value = match value_str {
            "Ace" => Value::Ace,
            "Two" => Value::Two,
            "Three" => Value::Three,
            "Four" => Value::Four,
            "Five" => Value::Five,
            "Six" => Value::Six,
            "Seven" => Value::Seven,
            "Eight" => Value::Eight,
            "Nine" => Value::Nine,
            "Ten" => Value::Ten,
            "Jack" => Value::Jack,
            "Queen" => Value::Queen,
            "King" => Value::King,
            _ => return Err("Invalid value in The Idol state".to_string()),
        };

        self.selected_suit = match suit_str {
            "Heart" => Suit::Heart,
            "Diamond" => Suit::Diamond,
            "Club" => Suit::Club,
            "Spade" => Suit::Spade,
            _ => return Err("Invalid suit in The Idol state".to_string()),
        };

        self.multiplier = multiplier;
        Ok(())
    }

    fn debug_state(&self) -> String {
        format!(
            "selected_card: {:?} of {:?}, multiplier: {:.1}",
            self.selected_value, self.selected_suit, self.multiplier
        )
    }

    fn reset_state(&mut self) {
        let (value, suit) = Self::random_card();
        self.selected_value = value;
        self.selected_suit = suit;
        self.multiplier = Self::random_multiplier();
    }
}

/// Factory functions for creating basic xmult jokers
pub fn create_photograph_joker() -> Box<dyn Joker> {
    Box::new(PhotographJoker::new())
}

pub fn create_ancient_joker() -> Box<dyn Joker> {
    Box::new(AncientJoker::new())
}

pub fn create_steel_joker() -> Box<dyn Joker> {
    Box::new(SteelJoker::new())
}

pub fn create_baron_joker() -> Box<dyn Joker> {
    Box::new(BaronJoker::new())
}

pub fn create_the_idol_joker() -> Box<dyn Joker> {
    Box::new(TheIdolJoker::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_photograph_joker() {
        let mut photograph = PhotographJoker::new();

        // Test identity
        assert_eq!(photograph.joker_type(), "photograph");
        assert_eq!(JokerIdentity::name(&photograph), "Photograph");
        assert_eq!(photograph.base_cost(), 5);

        // Test state management
        assert!(!photograph.face_card_played);
        photograph.face_card_played = true;
        assert!(photograph.face_card_played);

        // Test round reset
        JokerLifecycle::on_round_start(&mut photograph);
        assert!(!photograph.face_card_played);
    }

    #[test]
    fn test_ancient_joker() {
        let mut ancient = AncientJoker::new();

        // Test identity
        assert_eq!(ancient.joker_type(), "ancient_joker");
        assert_eq!(JokerIdentity::name(&ancient), "Ancient Joker");
        assert_eq!(JokerIdentity::rarity(&ancient), Rarity::Rare);

        // Test suit changes on round end
        let _initial_suit = ancient.selected_suit;
        JokerLifecycle::on_round_end(&mut ancient);
        // May or may not change due to randomness, but should be valid
        assert!(matches!(
            ancient.selected_suit,
            Suit::Heart | Suit::Diamond | Suit::Club | Suit::Spade
        ));
    }

    #[test]
    fn test_steel_joker() {
        let steel = SteelJoker::new();

        // Test identity
        assert_eq!(steel.joker_type(), "steel_joker");
        assert_eq!(JokerIdentity::name(&steel), "Steel Joker");
        assert_eq!(JokerIdentity::rarity(&steel), Rarity::Uncommon);
    }

    #[test]
    fn test_baron_joker() {
        let baron = BaronJoker::new();

        // Test identity
        assert_eq!(baron.joker_type(), "baron");
        assert_eq!(JokerIdentity::name(&baron), "Baron");
        assert_eq!(JokerIdentity::rarity(&baron), Rarity::Rare);

        // Test king counting
        let cards_with_kings = vec![
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::Queen, Suit::Diamond),
            Card::new(Value::King, Suit::Spade),
        ];
        assert_eq!(BaronJoker::count_kings_in_hand(&cards_with_kings), 2);

        let cards_no_kings = vec![
            Card::new(Value::Ace, Suit::Heart),
            Card::new(Value::Queen, Suit::Diamond),
        ];
        assert_eq!(BaronJoker::count_kings_in_hand(&cards_no_kings), 0);
    }

    #[test]
    fn test_the_idol_joker() {
        let mut idol = TheIdolJoker::new();

        // Test identity
        assert_eq!(idol.joker_type(), "the_idol");
        assert_eq!(JokerIdentity::name(&idol), "The Idol");
        assert_eq!(JokerIdentity::rarity(&idol), Rarity::Uncommon);

        // Test card changes on round end
        let _initial_value = idol.selected_value;
        let _initial_suit = idol.selected_suit;
        let _initial_multiplier = idol.multiplier;

        JokerLifecycle::on_round_end(&mut idol);

        // Values should be valid (may or may not have changed due to randomness)
        assert!(matches!(
            idol.selected_value,
            Value::Ace
                | Value::Two
                | Value::Three
                | Value::Four
                | Value::Five
                | Value::Six
                | Value::Seven
                | Value::Eight
                | Value::Nine
                | Value::Ten
                | Value::Jack
                | Value::Queen
                | Value::King
        ));
        assert!(matches!(
            idol.selected_suit,
            Suit::Heart | Suit::Diamond | Suit::Club | Suit::Spade
        ));
        assert!(idol.multiplier >= 1.5 && idol.multiplier <= 3.0);
    }
}
