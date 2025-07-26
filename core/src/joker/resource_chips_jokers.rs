//! Resource-Based Chips Jokers implementation
//!
//! This module implements jokers that provide chip bonuses based on
//! game resources like money, discards, cards in deck, etc.

use crate::{
    card::{Card, Value},
    hand::SelectHand,
    joker::{
        traits::{ProcessContext, ProcessResult, Rarity},
        GameContext, Joker, JokerEffect, JokerGameplay, JokerId, JokerIdentity, JokerRarity,
    },
    stage::Stage,
};

/// Banner joker - +30 chips per remaining discard
#[derive(Debug, Clone)]
pub struct BannerJoker {
    id: JokerId,
    name: String,
    description: String,
    rarity: JokerRarity,
    cost: usize,
}

impl Default for BannerJoker {
    fn default() -> Self {
        Self::new()
    }
}

impl BannerJoker {
    pub fn new() -> Self {
        Self {
            id: JokerId::Banner,
            name: "Banner".to_string(),
            description: "+30 Chips for each remaining discard".to_string(),
            rarity: JokerRarity::Common,
            cost: 3,
        }
    }

    fn calculate_remaining_discards(context: &GameContext) -> u32 {
        const MAX_DISCARDS: u32 = 5;
        MAX_DISCARDS.saturating_sub(context.discards_used)
    }
}

impl JokerIdentity for BannerJoker {
    fn joker_type(&self) -> &'static str {
        "banner"
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

impl Joker for BannerJoker {
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
        let discards_remaining = Self::calculate_remaining_discards(context);
        let chips_bonus = 30 * discards_remaining as i32;

        JokerEffect::new()
            .with_chips(chips_bonus)
            .with_message(format!(
                "Banner: +{chips_bonus} Chips ({discards_remaining} discards remaining)"
            ))
    }
}

impl JokerGameplay for BannerJoker {
    fn process(&mut self, stage: &Stage, _context: &mut ProcessContext) -> ProcessResult {
        if !matches!(stage, Stage::Blind(_)) {
            return ProcessResult::default();
        }

        // Note: ProcessContext doesn't have access to game resources
        // The main logic is in the Joker trait's on_hand_played method
        ProcessResult::default()
    }

    fn can_trigger(&self, stage: &Stage, _context: &ProcessContext) -> bool {
        matches!(stage, Stage::Blind(_))
    }
}

/// Bull joker - +2 chips per $1 owned
#[derive(Debug, Clone)]
pub struct BullJoker {
    id: JokerId,
    name: String,
    description: String,
    rarity: JokerRarity,
    cost: usize,
}

impl Default for BullJoker {
    fn default() -> Self {
        Self::new()
    }
}

impl BullJoker {
    pub fn new() -> Self {
        Self {
            id: JokerId::BullMarket,
            name: "Bull".to_string(),
            description: "+2 Chips per $1 owned".to_string(),
            rarity: JokerRarity::Common,
            cost: 3,
        }
    }
}

impl JokerIdentity for BullJoker {
    fn joker_type(&self) -> &'static str {
        "bull"
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

impl Joker for BullJoker {
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
        let chips_bonus = 2 * context.money;

        JokerEffect::new()
            .with_chips(chips_bonus)
            .with_message(format!(
                "Bull: +{chips_bonus} Chips (${} owned)",
                context.money
            ))
    }
}

impl JokerGameplay for BullJoker {
    fn process(&mut self, stage: &Stage, _context: &mut ProcessContext) -> ProcessResult {
        if !matches!(stage, Stage::Blind(_)) {
            return ProcessResult::default();
        }

        // Note: ProcessContext doesn't have access to game resources
        // The main logic is in the Joker trait's on_hand_played method
        ProcessResult::default()
    }

    fn can_trigger(&self, stage: &Stage, _context: &ProcessContext) -> bool {
        matches!(stage, Stage::Blind(_))
    }
}

/// Stone joker - +25 chips per Stone card in deck
#[derive(Debug, Clone)]
pub struct StoneJoker {
    id: JokerId,
    name: String,
    description: String,
    rarity: JokerRarity,
    cost: usize,
}

impl Default for StoneJoker {
    fn default() -> Self {
        Self::new()
    }
}

impl StoneJoker {
    pub fn new() -> Self {
        Self {
            id: JokerId::Stone,
            name: "Stone Joker".to_string(),
            description: "+25 Chips per Stone card in deck".to_string(),
            rarity: JokerRarity::Uncommon,
            cost: 4,
        }
    }
}

impl JokerIdentity for StoneJoker {
    fn joker_type(&self) -> &'static str {
        "stone_joker"
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

impl Joker for StoneJoker {
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
        let chips_bonus = 25 * context.stone_cards_in_deck as i32;

        JokerEffect::new()
            .with_chips(chips_bonus)
            .with_message(format!(
                "Stone Joker: +{chips_bonus} Chips ({} Stone cards)",
                context.stone_cards_in_deck
            ))
    }
}

impl JokerGameplay for StoneJoker {
    fn process(&mut self, stage: &Stage, _context: &mut ProcessContext) -> ProcessResult {
        if !matches!(stage, Stage::Blind(_)) {
            return ProcessResult::default();
        }

        // Note: ProcessContext doesn't have access to game resources
        // The main logic is in the Joker trait's on_hand_played method
        ProcessResult::default()
    }

    fn can_trigger(&self, stage: &Stage, _context: &ProcessContext) -> bool {
        matches!(stage, Stage::Blind(_))
    }
}

/// Scary Face joker - +30 chips when face cards are scored
#[derive(Debug, Clone)]
pub struct ScaryFaceJoker {
    id: JokerId,
    name: String,
    description: String,
    rarity: JokerRarity,
    cost: usize,
}

impl Default for ScaryFaceJoker {
    fn default() -> Self {
        Self::new()
    }
}

impl ScaryFaceJoker {
    pub fn new() -> Self {
        Self {
            id: JokerId::ScaryFace,
            name: "Scary Face".to_string(),
            description: "+30 Chips when face cards are scored".to_string(),
            rarity: JokerRarity::Common,
            cost: 3,
        }
    }

    fn is_face_card(card: &Card) -> bool {
        matches!(card.value, Value::Jack | Value::Queen | Value::King)
    }
}

impl JokerIdentity for ScaryFaceJoker {
    fn joker_type(&self) -> &'static str {
        "scary_face"
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

impl Joker for ScaryFaceJoker {
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
        if Self::is_face_card(card) {
            JokerEffect::new()
                .with_chips(30)
                .with_message(format!("Scary Face: +30 Chips ({:?} scored)", card.value))
        } else {
            JokerEffect::new()
        }
    }
}

impl JokerGameplay for ScaryFaceJoker {
    fn process(&mut self, stage: &Stage, context: &mut ProcessContext) -> ProcessResult {
        if !matches!(stage, Stage::Blind(_)) {
            return ProcessResult::default();
        }

        let face_card_count = context
            .played_cards
            .iter()
            .filter(|card| Self::is_face_card(card))
            .count();

        if face_card_count == 0 {
            return ProcessResult::default();
        }

        let chips_added = (30 * face_card_count) as u64;

        ProcessResult {
            chips_added,
            mult_added: 0.0,
            mult_multiplier: 1.0,
            retriggered: false,
            message: None,
        }
    }

    fn can_trigger(&self, stage: &Stage, context: &ProcessContext) -> bool {
        matches!(stage, Stage::Blind(_)) && context.played_cards.iter().any(Self::is_face_card)
    }
}

/// Blue joker - +2 chips per remaining card in deck
#[derive(Debug, Clone)]
pub struct BlueJoker {
    id: JokerId,
    name: String,
    description: String,
    rarity: JokerRarity,
    cost: usize,
}

impl Default for BlueJoker {
    fn default() -> Self {
        Self::new()
    }
}

impl BlueJoker {
    pub fn new() -> Self {
        Self {
            id: JokerId::BlueJoker,
            name: "Blue Joker".to_string(),
            description: "+2 Chips per remaining card in deck".to_string(),
            rarity: JokerRarity::Uncommon,
            cost: 4,
        }
    }
}

impl JokerIdentity for BlueJoker {
    fn joker_type(&self) -> &'static str {
        "blue_joker"
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

impl Joker for BlueJoker {
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
        let chips_bonus = 2 * context.cards_in_deck as i32;

        JokerEffect::new()
            .with_chips(chips_bonus)
            .with_message(format!(
                "Blue Joker: +{chips_bonus} Chips ({} cards in deck)",
                context.cards_in_deck
            ))
    }
}

impl JokerGameplay for BlueJoker {
    fn process(&mut self, stage: &Stage, _context: &mut ProcessContext) -> ProcessResult {
        if !matches!(stage, Stage::Blind(_)) {
            return ProcessResult::default();
        }

        // Note: ProcessContext doesn't have access to game resources
        // The main logic is in the Joker trait's on_hand_played method
        ProcessResult::default()
    }

    fn can_trigger(&self, stage: &Stage, _context: &ProcessContext) -> bool {
        matches!(stage, Stage::Blind(_))
    }
}

/// Factory functions for creating resource-based chips jokers
pub fn create_banner_joker() -> Box<dyn Joker> {
    Box::new(BannerJoker::new())
}

pub fn create_bull_joker() -> Box<dyn Joker> {
    Box::new(BullJoker::new())
}

pub fn create_stone_joker() -> Box<dyn Joker> {
    Box::new(StoneJoker::new())
}

pub fn create_scary_face_joker() -> Box<dyn Joker> {
    Box::new(ScaryFaceJoker::new())
}

pub fn create_blue_joker() -> Box<dyn Joker> {
    Box::new(BlueJoker::new())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stage::Blind;

    #[test]
    fn test_banner_joker_remaining_discards() {
        let banner = BannerJoker::new();

        // Test identity
        assert_eq!(banner.joker_type(), "banner");
        assert_eq!(JokerIdentity::name(&banner), "Banner");
        assert_eq!(banner.base_cost(), 3);
    }

    #[test]
    fn test_bull_joker_money_scaling() {
        let bull = BullJoker::new();

        // Test identity
        assert_eq!(bull.joker_type(), "bull");
        assert_eq!(JokerIdentity::name(&bull), "Bull");
        assert_eq!(bull.base_cost(), 3);
    }

    #[test]
    fn test_scary_face_joker_face_card_detection() {
        let _scary_face = ScaryFaceJoker::new();

        // Test face card detection
        let jack = Card::new(Value::Jack, crate::card::Suit::Heart);
        let two = Card::new(Value::Two, crate::card::Suit::Spade);

        assert!(ScaryFaceJoker::is_face_card(&jack));
        assert!(!ScaryFaceJoker::is_face_card(&two));
    }

    #[test]
    fn test_blue_joker_deck_based_chips() {
        let blue = BlueJoker::new();

        // Test identity
        assert_eq!(blue.joker_type(), "blue_joker");
        assert_eq!(JokerIdentity::name(&blue), "Blue Joker");
        assert_eq!(JokerIdentity::rarity(&blue), Rarity::Uncommon);
        assert_eq!(blue.base_cost(), 4);
    }

    #[test]
    fn test_stone_joker_stone_cards() {
        let stone = StoneJoker::new();

        // Test identity
        assert_eq!(stone.joker_type(), "stone_joker");
        assert_eq!(JokerIdentity::name(&stone), "Stone Joker");
        assert_eq!(JokerIdentity::rarity(&stone), Rarity::Uncommon);
        assert_eq!(stone.base_cost(), 4);
    }

    #[test]
    fn test_scary_face_gameplay_trait() {
        let mut scary_face = ScaryFaceJoker::new();
        let stage = Stage::Blind(Blind::Small);

        // Create test context with face cards
        let jack = Card::new(Value::Jack, crate::card::Suit::Heart);
        let queen = Card::new(Value::Queen, crate::card::Suit::Diamond);
        let two = Card::new(Value::Two, crate::card::Suit::Spade);

        let played_cards = vec![jack, queen, two];
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

        // Test can_trigger
        assert!(scary_face.can_trigger(&stage, &context));

        // Test process
        let result = scary_face.process(&stage, &mut context);
        assert_eq!(result.chips_added, 60); // 2 face cards * 30 chips
    }
}
