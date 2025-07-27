//! Basic XMult Jokers implementation
//!
//! This module implements jokers that provide multiplicative mult bonuses (X mult).
//! These jokers apply mult_multiplier effects under various conditions.

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

/// Photograph Joker - First played face card gives X2 Mult when scored
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
            description: "First played face card gives X2 Mult when scored".to_string(),
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
                .with_mult_multiplier(2.0)
                .with_message("Photograph: X2 Mult (first face card)".to_string())
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
                    // Note: The actual X2 mult is applied via on_card_scored
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

/// Polished Joker - X1 Mult plus X0.25 Mult per Joker
#[derive(Debug, Clone)]
pub struct PolishedJoker {
    id: JokerId,
    name: String,
    description: String,
    rarity: JokerRarity,
    cost: usize,
}

impl Default for PolishedJoker {
    fn default() -> Self {
        Self::new()
    }
}

impl PolishedJoker {
    pub fn new() -> Self {
        Self {
            id: JokerId::PolishedJoker,
            name: "Polished Joker".to_string(),
            description: "X1 Mult, plus X0.25 Mult per Joker you have".to_string(),
            rarity: JokerRarity::Uncommon,
            cost: 6,
        }
    }

    fn calculate_multiplier(joker_count: usize) -> f64 {
        1.0 + (0.25 * joker_count as f64)
    }
}

impl JokerIdentity for PolishedJoker {
    fn joker_type(&self) -> &'static str {
        "polished"
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

impl Joker for PolishedJoker {
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
        let joker_count = context.jokers.len();
        let multiplier = Self::calculate_multiplier(joker_count);

        JokerEffect::new()
            .with_mult_multiplier(multiplier)
            .with_message(format!(
                "Polished Joker: X{multiplier} Mult ({joker_count} jokers)"
            ))
    }
}

impl JokerGameplay for PolishedJoker {
    fn process(&mut self, stage: &Stage, _context: &mut ProcessContext) -> ProcessResult {
        if !matches!(stage, Stage::Blind(_)) {
            return ProcessResult::default();
        }

        // Note: The actual multiplier calculation happens in on_hand_played
        // which has access to GameContext with joker count
        ProcessResult::default()
    }

    fn can_trigger(&self, stage: &Stage, _context: &ProcessContext) -> bool {
        matches!(stage, Stage::Blind(_))
    }
}

/// Rough Gem - +25 Chips for Clubs, Diamonds give +1 Mult, Spades/Hearts give X1.5 Mult
#[derive(Debug, Clone)]
pub struct RoughGemJoker {
    id: JokerId,
    name: String,
    description: String,
    rarity: JokerRarity,
    cost: usize,
}

impl Default for RoughGemJoker {
    fn default() -> Self {
        Self::new()
    }
}

impl RoughGemJoker {
    pub fn new() -> Self {
        Self {
            id: JokerId::RoughGem,
            name: "Rough Gem".to_string(),
            description: "Clubs: +25 Chips, Diamonds: +1 Mult, Spades/Hearts: X1.5 Mult"
                .to_string(),
            rarity: JokerRarity::Uncommon,
            cost: 7,
        }
    }
}

impl JokerIdentity for RoughGemJoker {
    fn joker_type(&self) -> &'static str {
        "rough_gem"
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

impl Joker for RoughGemJoker {
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
        match card.suit {
            Suit::Club => JokerEffect::new()
                .with_chips(25)
                .with_message("Rough Gem: +25 Chips (Club)".to_string()),
            Suit::Diamond => JokerEffect::new()
                .with_mult(1)
                .with_message("Rough Gem: +1 Mult (Diamond)".to_string()),
            Suit::Spade | Suit::Heart => JokerEffect::new()
                .with_mult_multiplier(1.5)
                .with_message(format!("Rough Gem: X1.5 Mult ({:?})", card.suit)),
        }
    }
}

impl JokerGameplay for RoughGemJoker {
    fn process(&mut self, stage: &Stage, context: &mut ProcessContext) -> ProcessResult {
        if !matches!(stage, Stage::Blind(_)) {
            return ProcessResult::default();
        }

        let mut chips_added = 0;
        let mut mult_added = 0.0;
        for card in context.played_cards {
            match card.suit {
                Suit::Club => chips_added += 25,
                Suit::Diamond => mult_added += 1.0,
                Suit::Spade | Suit::Heart => {} // X1.5 mult handled by on_card_scored
            }
        }

        ProcessResult {
            chips_added: chips_added as u64,
            mult_added,
            mult_multiplier: 1.0,
            retriggered: false,
            message: None,
        }
    }

    fn can_trigger(&self, stage: &Stage, context: &ProcessContext) -> bool {
        matches!(stage, Stage::Blind(_)) && !context.played_cards.is_empty()
    }
}

/// Bloodstone Joker - X1.5 Mult per unique suit in played hand
#[derive(Debug, Clone)]
pub struct BloodstoneJoker {
    id: JokerId,
    name: String,
    description: String,
    rarity: JokerRarity,
    cost: usize,
}

impl Default for BloodstoneJoker {
    fn default() -> Self {
        Self::new()
    }
}

impl BloodstoneJoker {
    pub fn new() -> Self {
        Self {
            id: JokerId::Bloodstone,
            name: "Bloodstone".to_string(),
            description: "X1.5 Mult per unique suit in played hand".to_string(),
            rarity: JokerRarity::Uncommon,
            cost: 7,
        }
    }

    fn count_unique_suits(cards: &[Card]) -> usize {
        let mut suits = std::collections::HashSet::new();
        for card in cards {
            suits.insert(card.suit);
        }
        suits.len()
    }
}

impl JokerIdentity for BloodstoneJoker {
    fn joker_type(&self) -> &'static str {
        "bloodstone"
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

impl Joker for BloodstoneJoker {
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
        // Use cards from context.hand instead of SelectHand
        let cards: Vec<Card> = context.hand.cards().to_vec();
        let unique_suits = Self::count_unique_suits(&cards);
        if unique_suits > 0 {
            let multiplier = 1.5_f64.powi(unique_suits as i32);
            JokerEffect::new()
                .with_mult_multiplier(multiplier)
                .with_message(format!(
                    "Bloodstone: X{multiplier} Mult ({unique_suits} unique suits)"
                ))
        } else {
            JokerEffect::new()
        }
    }
}

impl JokerGameplay for BloodstoneJoker {
    fn process(&mut self, stage: &Stage, _context: &mut ProcessContext) -> ProcessResult {
        if !matches!(stage, Stage::Blind(_)) {
            return ProcessResult::default();
        }

        // Note: The actual multiplier is applied via on_hand_played
        // This just tracks that we can trigger
        ProcessResult::default()
    }

    fn can_trigger(&self, stage: &Stage, context: &ProcessContext) -> bool {
        matches!(stage, Stage::Blind(_)) && !context.played_cards.is_empty()
    }
}

/// Misprint Joker - X1 to X23 Mult (random each hand)
#[derive(Debug, Clone)]
pub struct MisprintJoker {
    id: JokerId,
    name: String,
    description: String,
    rarity: JokerRarity,
    cost: usize,
}

impl Default for MisprintJoker {
    fn default() -> Self {
        Self::new()
    }
}

impl MisprintJoker {
    pub fn new() -> Self {
        Self {
            id: JokerId::Misprint,
            name: "Misprint".to_string(),
            description: "X1 to X23 Mult (random each hand)".to_string(),
            rarity: JokerRarity::Common,
            cost: 4,
        }
    }
}

impl JokerIdentity for MisprintJoker {
    fn joker_type(&self) -> &'static str {
        "misprint"
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

impl Joker for MisprintJoker {
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
        // Use game RNG to generate random multiplier between 1 and 23
        let multiplier = context.rng.gen_range(1..=23) as f64;

        JokerEffect::new()
            .with_mult_multiplier(multiplier)
            .with_message(format!("Misprint: X{multiplier} Mult"))
    }
}

impl JokerGameplay for MisprintJoker {
    fn process(&mut self, stage: &Stage, _context: &mut ProcessContext) -> ProcessResult {
        if !matches!(stage, Stage::Blind(_)) {
            return ProcessResult::default();
        }

        // Note: Random multiplier is generated in on_hand_played
        ProcessResult::default()
    }

    fn can_trigger(&self, stage: &Stage, _context: &ProcessContext) -> bool {
        matches!(stage, Stage::Blind(_))
    }
}

/// Factory functions for creating basic xmult jokers
pub fn create_photograph_joker() -> Box<dyn Joker> {
    Box::new(PhotographJoker::new())
}

pub fn create_polished_joker() -> Box<dyn Joker> {
    Box::new(PolishedJoker::new())
}

pub fn create_rough_gem_joker() -> Box<dyn Joker> {
    Box::new(RoughGemJoker::new())
}

pub fn create_bloodstone_joker() -> Box<dyn Joker> {
    Box::new(BloodstoneJoker::new())
}

pub fn create_misprint_joker() -> Box<dyn Joker> {
    Box::new(MisprintJoker::new())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stage::Blind;

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
    fn test_polished_joker_multiplier() {
        let polished = PolishedJoker::new();

        // Test identity
        assert_eq!(polished.joker_type(), "polished");
        assert_eq!(JokerIdentity::name(&polished), "Polished Joker");
        assert_eq!(JokerIdentity::rarity(&polished), Rarity::Uncommon);

        // Test multiplier calculation
        assert_eq!(PolishedJoker::calculate_multiplier(0), 1.0);
        assert_eq!(PolishedJoker::calculate_multiplier(1), 1.25);
        assert_eq!(PolishedJoker::calculate_multiplier(4), 2.0);
        assert_eq!(PolishedJoker::calculate_multiplier(8), 3.0);
    }

    #[test]
    fn test_rough_gem_suit_effects() {
        let mut rough_gem = RoughGemJoker::new();
        let stage = Stage::Blind(Blind::Small);

        // Test with different suits
        let cards = vec![
            Card::new(Value::Ace, Suit::Club),     // +25 chips
            Card::new(Value::King, Suit::Diamond), // +1 mult
            Card::new(Value::Queen, Suit::Spade),  // X1.5 mult
            Card::new(Value::Jack, Suit::Heart),   // X1.5 mult
        ];

        let played_cards = cards;
        let held_cards = vec![];
        let mut events = vec![];
        let mut hand_score = crate::joker::traits::HandScore {
            chips: 0,
            mult: 0.0,
        };

        let joker_state_manager = crate::joker_state::JokerStateManager::new();
        let hand = SelectHand::new(played_cards.clone());

        let mut context = ProcessContext {
            hand_score: &mut hand_score,
            played_cards: &played_cards,
            hand: &hand,
            held_cards: &held_cards,
            events: &mut events,
            joker_state_manager: &joker_state_manager,
        };

        let result = rough_gem.process(&stage, &mut context);
        assert_eq!(result.chips_added, 25);
        assert_eq!(result.mult_added, 1.0);
    }

    #[test]
    fn test_bloodstone_unique_suits() {
        // Test unique suit counting
        let cards1 = vec![
            Card::new(Value::Ace, Suit::Heart),
            Card::new(Value::King, Suit::Heart),
        ];
        assert_eq!(BloodstoneJoker::count_unique_suits(&cards1), 1);

        let cards2 = vec![
            Card::new(Value::Ace, Suit::Heart),
            Card::new(Value::King, Suit::Diamond),
            Card::new(Value::Queen, Suit::Spade),
            Card::new(Value::Jack, Suit::Club),
        ];
        assert_eq!(BloodstoneJoker::count_unique_suits(&cards2), 4);
    }

    #[test]
    fn test_misprint_joker() {
        let misprint = MisprintJoker::new();

        // Test identity
        assert_eq!(misprint.joker_type(), "misprint");
        assert_eq!(JokerIdentity::name(&misprint), "Misprint");
        assert_eq!(misprint.base_cost(), 4);

        // Test can trigger
        let stage = Stage::Blind(Blind::Small);
        let played_cards = vec![];
        let held_cards = vec![];
        let mut events = vec![];
        let mut hand_score = crate::joker::traits::HandScore {
            chips: 0,
            mult: 0.0,
        };

        let joker_state_manager = crate::joker_state::JokerStateManager::new();
        let hand = SelectHand::new(played_cards.clone());

        let context = ProcessContext {
            hand_score: &mut hand_score,
            played_cards: &played_cards,
            hand: &hand,
            held_cards: &held_cards,
            events: &mut events,
            joker_state_manager: &joker_state_manager,
        };

        assert!(misprint.can_trigger(&stage, &context));
    }
}
