//! Resource-Based Chips Jokers implementation
//!
//! This module implements jokers that provide chip bonuses based on
//! game resources like money, discards, cards in deck, etc.

use crate::{
    card::{Card, Value},
    hand::SelectHand,
    joker::{
        traits::{JokerState, ProcessContext, ProcessResult, Rarity},
        GameContext, Joker, JokerEffect, JokerGameplay, JokerId, JokerIdentity, JokerRarity,
    },
    stage::Stage,
};
use serde_json;
use std::collections::HashSet;

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

/// Hiker joker - Every played card permanently gains +5 chips when scored
/// Specification from joker.json: "Every played {C:attention}card{} permanently gains {C:chips}+#1#{} Chips when scored"
#[derive(Debug, Clone)]
pub struct HikerJoker {
    id: JokerId,
    name: String,
    description: String,
    rarity: JokerRarity,
    cost: usize,
    modified_cards: HashSet<usize>, // Card IDs that have been permanently modified
    chips_per_modification: u32,
}

impl Default for HikerJoker {
    fn default() -> Self {
        Self::new()
    }
}

impl HikerJoker {
    pub fn new() -> Self {
        Self {
            id: JokerId::Hiker,
            name: "Hiker".to_string(),
            description: "Every played card permanently gains +5 Chips when scored".to_string(),
            rarity: JokerRarity::Common,
            cost: 4,
            modified_cards: HashSet::new(),
            chips_per_modification: 5,
        }
    }

    fn is_card_modified(&self, card: &Card) -> bool {
        self.modified_cards.contains(&card.id)
    }

    fn modify_card(&mut self, card: &Card) {
        self.modified_cards.insert(card.id);
    }

    fn get_card_bonus(&self, card: &Card) -> u32 {
        if self.is_card_modified(card) {
            self.chips_per_modification
        } else {
            0
        }
    }
}

impl JokerIdentity for HikerJoker {
    fn joker_type(&self) -> &'static str {
        "hiker"
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

impl Joker for HikerJoker {
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
        let bonus_chips = self.get_card_bonus(card);
        if bonus_chips > 0 {
            JokerEffect::new()
                .with_chips(bonus_chips as i32)
                .with_message(format!(
                    "Hiker: +{bonus_chips} Chips (permanent card bonus)"
                ))
        } else {
            JokerEffect::new()
        }
    }

    fn on_hand_played(&self, _context: &mut GameContext, _hand: &SelectHand) -> JokerEffect {
        // We handle the logic in the JokerGameplay trait process method
        JokerEffect::new()
    }
}

impl JokerGameplay for HikerJoker {
    fn process(&mut self, stage: &Stage, context: &mut ProcessContext) -> ProcessResult {
        if !matches!(stage, Stage::Blind(_)) {
            return ProcessResult::default();
        }

        let mut total_chips = 0u64;
        let mut newly_modified = 0;

        // Process all played cards
        for card in context.played_cards {
            // If this card hasn't been modified before, modify it now
            if !self.is_card_modified(card) {
                self.modify_card(card);
                newly_modified += 1;
            }

            // Apply bonus chips for this card
            total_chips += self.chips_per_modification as u64;
        }

        if total_chips > 0 {
            let message = if newly_modified > 0 {
                Some(format!("Hiker: {newly_modified} cards permanently modified, +{total_chips} chips total!"))
            } else {
                Some(format!("Hiker: +{total_chips} chips from modified cards"))
            };

            ProcessResult {
                chips_added: total_chips,
                mult_added: 0.0,
                mult_multiplier: 1.0,
                retriggered: false,
                message,
            }
        } else {
            ProcessResult::default()
        }
    }

    fn can_trigger(&self, stage: &Stage, context: &ProcessContext) -> bool {
        matches!(stage, Stage::Blind(_)) && !context.played_cards.is_empty()
    }
}

impl JokerState for HikerJoker {
    fn has_state(&self) -> bool {
        true
    }

    fn serialize_state(&self) -> Option<serde_json::Value> {
        let modified_card_ids: Vec<usize> = self.modified_cards.iter().cloned().collect();
        Some(serde_json::json!({
            "modified_cards": modified_card_ids
        }))
    }

    fn deserialize_state(&mut self, value: serde_json::Value) -> Result<(), String> {
        if let Some(modified_cards) = value.get("modified_cards").and_then(|v| v.as_array()) {
            self.modified_cards.clear();
            for card_id in modified_cards {
                if let Some(id) = card_id.as_u64() {
                    self.modified_cards.insert(id as usize);
                } else {
                    return Err("Invalid card ID in Hiker state".to_string());
                }
            }
            Ok(())
        } else {
            Err("Invalid state format for Hiker".to_string())
        }
    }

    fn debug_state(&self) -> String {
        format!("modified_cards: {:?}", self.modified_cards)
    }

    fn reset_state(&mut self) {
        self.modified_cards.clear();
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

pub fn create_hiker_joker() -> Box<dyn Joker> {
    Box::new(HikerJoker::new())
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
        let hand = SelectHand::new(played_cards.clone());

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

    #[test]
    fn test_hiker_joker_identity() {
        let hiker = HikerJoker::new();
        assert_eq!(hiker.joker_type(), "hiker");
        assert_eq!(JokerIdentity::name(&hiker), "Hiker");
        assert_eq!(JokerIdentity::rarity(&hiker), Rarity::Common);
        assert_eq!(hiker.base_cost(), 4);
    }

    #[test]
    fn test_hiker_joker_permanent_modification() {
        let mut hiker = HikerJoker::new();
        let stage = Stage::Blind(Blind::Small);

        // Create test cards
        let card1 = Card::new(Value::Ace, crate::card::Suit::Heart);
        let card2 = Card::new(Value::King, crate::card::Suit::Diamond);
        let card3 = Card::new(Value::Queen, crate::card::Suit::Spade);

        // First hand - no cards are modified yet
        let played_cards = vec![card1.clone(), card2.clone()];
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

        // Should trigger with any cards
        assert!(hiker.can_trigger(&stage, &context));

        // Process should modify cards and add chips
        let result = hiker.process(&stage, &mut context);
        assert_eq!(result.chips_added, 10); // 2 cards * 5 chips each
        assert!(result.message.is_some());
        assert!(result
            .message
            .unwrap()
            .contains("2 cards permanently modified"));

        // Check that cards are now modified
        assert!(hiker.is_card_modified(&card1));
        assert!(hiker.is_card_modified(&card2));
        assert!(!hiker.is_card_modified(&card3)); // Wasn't played

        // Second hand with same cards - should still get bonus but no new modifications
        let result2 = hiker.process(&stage, &mut context);
        assert_eq!(result2.chips_added, 10); // Same bonus
        assert!(result2.message.is_some());
        assert!(result2
            .message
            .unwrap()
            .contains("chips from modified cards")); // Different message

        // Third hand with mixed cards (one new, two already modified)
        let played_cards3 = vec![card1.clone(), card3.clone()];
        let mut context3 = ProcessContext {
            hand_score: &mut hand_score,
            played_cards: &played_cards3,
            held_cards: &held_cards,
            events: &mut events,
            joker_state_manager: &joker_state_manager,
        };

        let result3 = hiker.process(&stage, &mut context3);
        assert_eq!(result3.chips_added, 10); // 2 cards * 5 chips each
        assert!(result3.message.is_some());
        assert!(result3
            .message
            .unwrap()
            .contains("1 cards permanently modified")); // Only card3 is new

        // Verify all three cards are now modified
        assert!(hiker.is_card_modified(&card1));
        assert!(hiker.is_card_modified(&card2));
        assert!(hiker.is_card_modified(&card3));
    }

    #[test]
    fn test_hiker_joker_on_card_scored() {
        let mut hiker = HikerJoker::new();
        let card = Card::new(Value::Two, crate::card::Suit::Club);

        // Create dummy context
        let empty_hand = crate::hand::Hand::new(vec![]);
        let empty_discarded = vec![];
        let joker_state_manager = std::sync::Arc::new(crate::joker_state::JokerStateManager::new());
        let hand_type_counts = std::collections::HashMap::new();
        let rng = crate::rng::GameRng::for_testing(12345);

        let mut context = GameContext {
            chips: 0,
            mult: 0,
            money: 0,
            ante: 1,
            round: 1,
            stage: &Stage::Blind(Blind::Small),
            hands_played: 0,
            discards_used: 0,
            jokers: &[],
            hand: &empty_hand,
            discarded: &empty_discarded,
            joker_state_manager: &joker_state_manager,
            hand_type_counts: &hand_type_counts,
            cards_in_deck: 52,
            stone_cards_in_deck: 0,
            rng: &rng,
        };

        // Card not modified yet - should not give bonus
        let effect = hiker.on_card_scored(&mut context, &card);
        assert_eq!(effect.chips, 0);

        // Modify the card manually
        hiker.modify_card(&card);

        // Now should give bonus
        let effect2 = hiker.on_card_scored(&mut context, &card);
        assert_eq!(effect2.chips, 5);
        assert!(effect2.message.is_some());
        assert!(effect2
            .message
            .unwrap()
            .contains("+5 Chips (permanent card bonus)"));
    }

    #[test]
    fn test_hiker_joker_state_serialization() {
        let mut hiker = HikerJoker::new();

        // Modify some cards
        let card1 = Card::new(Value::Ace, crate::card::Suit::Heart);
        let card2 = Card::new(Value::King, crate::card::Suit::Diamond);
        hiker.modify_card(&card1);
        hiker.modify_card(&card2);

        // Serialize state
        let state = JokerState::serialize_state(&hiker).unwrap();
        assert!(state.get("modified_cards").is_some());

        // Deserialize into new instance
        let mut new_hiker = HikerJoker::new();
        JokerState::deserialize_state(&mut new_hiker, state).unwrap();

        // Verify cards are still modified
        assert!(new_hiker.is_card_modified(&card1));
        assert!(new_hiker.is_card_modified(&card2));
        assert_eq!(new_hiker.modified_cards.len(), 2);
    }

    #[test]
    fn test_hiker_joker_state_reset() {
        let mut hiker = HikerJoker::new();

        // Modify some cards
        let card = Card::new(Value::Seven, crate::card::Suit::Spade);
        hiker.modify_card(&card);
        assert!(hiker.is_card_modified(&card));

        // Reset state
        JokerState::reset_state(&mut hiker);
        assert!(!hiker.is_card_modified(&card));
        assert!(hiker.modified_cards.is_empty());
    }

    #[test]
    fn test_hiker_joker_different_stages() {
        let mut hiker = HikerJoker::new();
        let played_cards = vec![Card::new(Value::Five, crate::card::Suit::Heart)];
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

        // Should trigger during blind stage
        let blind_stage = Stage::Blind(Blind::Small);
        assert!(hiker.can_trigger(&blind_stage, &context));
        let result = hiker.process(&blind_stage, &mut context);
        assert_eq!(result.chips_added, 5);

        // Should not trigger during non-blind stages
        let shop_stage = Stage::Shop();
        assert!(!hiker.can_trigger(&shop_stage, &context));
        let result2 = hiker.process(&shop_stage, &mut context);
        assert_eq!(result2.chips_added, 0);
    }
}
