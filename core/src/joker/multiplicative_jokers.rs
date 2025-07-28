//! Multiplicative jokers implementation
//!
//! This module implements jokers that apply multiplicative effects (X mult)
//! to scoring. These jokers multiply the final score by a factor based on
//! various conditions.

use crate::{
    card::{Card, Suit, Value},
    hand::SelectHand,
    joker::traits::{JokerState, ProcessContext, ProcessResult, Rarity},
    joker::{
        GameContext, Joker, JokerEffect, JokerGameplay, JokerId, JokerIdentity, JokerLifecycle,
        JokerRarity,
    },
    stage::Stage,
};

/// Baron joker - X1.5 mult per King held in hand
#[derive(Debug, Clone)]
pub struct Baron {
    id: JokerId,
    name: String,
    description: String,
    rarity: JokerRarity,
    cost: usize,
}

impl Default for Baron {
    fn default() -> Self {
        Self::new()
    }
}

impl Baron {
    pub fn new() -> Self {
        Self {
            id: JokerId::BaronJoker,
            name: "Baron".to_string(),
            description: "Each King held in hand gives X1.5 Mult".to_string(),
            rarity: JokerRarity::Rare,
            cost: 8,
        }
    }

    fn count_kings_in_hand(hand: &SelectHand) -> usize {
        hand.cards()
            .iter()
            .filter(|card| matches!(card.value, Value::King))
            .count()
    }
}

impl JokerIdentity for Baron {
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

impl Joker for Baron {
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
        let king_count = Self::count_kings_in_hand(hand);
        if king_count == 0 {
            return JokerEffect::new();
        }

        // X1.5 per King, multiplicative: 1.5^n
        let multiplier = 1.5_f64.powi(king_count as i32);

        JokerEffect::new()
            .with_mult_multiplier(multiplier)
            .with_message(format!("Baron: X{multiplier} Mult ({king_count} Kings)"))
    }
}

impl JokerGameplay for Baron {
    fn process(&mut self, stage: &Stage, context: &mut ProcessContext) -> ProcessResult {
        if !matches!(stage, Stage::Blind(_)) {
            return ProcessResult::default();
        }

        let king_count = Self::count_kings_in_hand(context.hand);
        if king_count == 0 {
            return ProcessResult::default();
        }

        // X1.5 per King, multiplicative: 1.5^n
        let multiplier = 1.5_f64.powi(king_count as i32);

        ProcessResult {
            mult_multiplier: multiplier,
            message: Some(format!("Baron: X{multiplier} Mult ({king_count} Kings)")),
            ..Default::default()
        }
    }

    fn can_trigger(&self, stage: &Stage, _context: &ProcessContext) -> bool {
        matches!(stage, Stage::Blind(_))
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
            description: "Gives X0.5 Mult for each Steel Card in your full deck".to_string(),
            rarity: JokerRarity::Uncommon,
            cost: 6,
        }
    }

    // Note: Steel card counting is now done via GameContext.steel_cards_in_deck
    // in the on_hand_played method, not through ProcessContext
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

    fn on_hand_played(&self, context: &mut GameContext, _hand: &SelectHand) -> JokerEffect {
        let steel_count = context.steel_cards_in_deck;
        if steel_count == 0 {
            return JokerEffect::new();
        }

        // Steel Joker gives X0.5 Mult for each Steel Card per joker.json specification
        // This equals X(1.0 + 0.5 * steel_count)
        let multiplier = 1.0 + (steel_count as f64 * 0.5);

        JokerEffect::new()
            .with_mult_multiplier(multiplier)
            .with_message(format!(
                "Steel Joker: X{multiplier:.1} Mult ({steel_count} Steel cards)"
            ))
    }
}

impl JokerGameplay for SteelJoker {
    fn process(&mut self, stage: &Stage, _context: &mut ProcessContext) -> ProcessResult {
        if !matches!(stage, Stage::Blind(_)) {
            return ProcessResult::default();
        }

        // Note: This process method is for JokerGameplay trait compatibility
        // The main logic is in on_hand_played method which uses GameContext
        // ProcessContext doesn't have access to full deck information
        ProcessResult::default()
    }

    fn can_trigger(&self, stage: &Stage, _context: &ProcessContext) -> bool {
        matches!(stage, Stage::Blind(_))
    }
}

/// Ancient Joker - X1.5 mult per card of selected suit scored
/// The suit changes each round
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
            description: "Each card with selected suit gives X1.5 Mult when scored, suit changes at end of round".to_string(),
            rarity: JokerRarity::Rare,
            cost: 8,
            selected_suit: Suit::Spade, // Start with Spades
        }
    }

    fn count_suit_cards_scored(cards: &[Card], suit: Suit) -> usize {
        cards.iter().filter(|card| card.suit == suit).count()
    }

    fn rotate_suit(&mut self) {
        self.selected_suit = match self.selected_suit {
            Suit::Spade => Suit::Heart,
            Suit::Heart => Suit::Diamond,
            Suit::Diamond => Suit::Club,
            Suit::Club => Suit::Spade,
        };
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

    fn on_hand_played(&self, _context: &mut GameContext, hand: &SelectHand) -> JokerEffect {
        // Count cards of selected suit in the played hand
        let suit_count = hand
            .cards()
            .iter()
            .filter(|card| card.suit == self.selected_suit)
            .count();

        if suit_count == 0 {
            return JokerEffect::new();
        }

        // X1.5 per card of suit, multiplicative: 1.5^n
        let multiplier = 1.5_f64.powi(suit_count as i32);

        JokerEffect::new()
            .with_mult_multiplier(multiplier)
            .with_message(format!(
                "Ancient Joker: X{} Mult ({} {:?}s played)",
                multiplier, suit_count, self.selected_suit
            ))
    }

    fn on_round_end(&self, _context: &mut GameContext) -> JokerEffect {
        // Note: We can't mutate self here, so rotation happens in JokerLifecycle
        JokerEffect::new()
    }
}

impl JokerGameplay for AncientJoker {
    fn process(&mut self, stage: &Stage, context: &mut ProcessContext) -> ProcessResult {
        if !matches!(stage, Stage::Blind(_)) {
            return ProcessResult::default();
        }

        let suit_count = Self::count_suit_cards_scored(context.played_cards, self.selected_suit);
        if suit_count == 0 {
            return ProcessResult::default();
        }

        // X1.5 per card of suit, multiplicative: 1.5^n
        let multiplier = 1.5_f64.powi(suit_count as i32);

        ProcessResult {
            mult_multiplier: multiplier,
            message: Some(format!(
                "Ancient Joker: X{} Mult ({} {:?}s scored)",
                multiplier, suit_count, self.selected_suit
            )),
            ..Default::default()
        }
    }

    fn can_trigger(&self, stage: &Stage, _context: &ProcessContext) -> bool {
        matches!(stage, Stage::Blind(_))
    }
}

impl JokerLifecycle for AncientJoker {
    fn on_round_end(&mut self) {
        self.rotate_suit();
    }
}

impl JokerState for AncientJoker {
    fn has_state(&self) -> bool {
        true
    }

    fn serialize_state(&self) -> Option<serde_json::Value> {
        Some(serde_json::json!({
            "selected_suit": match self.selected_suit {
                Suit::Spade => "spade",
                Suit::Heart => "heart",
                Suit::Diamond => "diamond",
                Suit::Club => "club",
            }
        }))
    }

    fn deserialize_state(&mut self, value: serde_json::Value) -> Result<(), String> {
        if let Some(suit_str) = value.get("selected_suit").and_then(|v| v.as_str()) {
            self.selected_suit = match suit_str {
                "spade" => Suit::Spade,
                "heart" => Suit::Heart,
                "diamond" => Suit::Diamond,
                "club" => Suit::Club,
                _ => return Err("Invalid suit in state".to_string()),
            };
            Ok(())
        } else {
            Err("Missing selected_suit in state".to_string())
        }
    }

    fn debug_state(&self) -> String {
        format!("Selected suit: {:?}", self.selected_suit)
    }
}

/// The Duo - X2 mult if hand contains exactly a pair
#[derive(Debug, Clone)]
pub struct TheDuo {
    id: JokerId,
    name: String,
    description: String,
    rarity: JokerRarity,
    cost: usize,
}

impl Default for TheDuo {
    fn default() -> Self {
        Self::new()
    }
}

impl TheDuo {
    pub fn new() -> Self {
        Self {
            id: JokerId::TheDuo,
            name: "The Duo".to_string(),
            description: "X2 Mult if hand contains exactly a pair".to_string(),
            rarity: JokerRarity::Common,
            cost: 3,
        }
    }

    fn is_exactly_pair(_context: &ProcessContext) -> bool {
        // Check if the hand rank is exactly a pair
        // This would need to check the actual hand evaluation
        // For now, placeholder implementation
        false
    }
}

impl JokerIdentity for TheDuo {
    fn joker_type(&self) -> &'static str {
        "the_duo"
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

impl Joker for TheDuo {
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
        // TODO: Check if the hand rank is exactly a pair
        // For now, return no effect
        JokerEffect::new()
    }
}

impl JokerGameplay for TheDuo {
    fn process(&mut self, stage: &Stage, context: &mut ProcessContext) -> ProcessResult {
        if !matches!(stage, Stage::Blind(_)) {
            return ProcessResult::default();
        }

        if Self::is_exactly_pair(context) {
            ProcessResult {
                mult_multiplier: 2.0,
                message: Some("The Duo: X2 Mult (Pair)".to_string()),
                ..Default::default()
            }
        } else {
            ProcessResult::default()
        }
    }

    fn can_trigger(&self, stage: &Stage, _context: &ProcessContext) -> bool {
        matches!(stage, Stage::Blind(_))
    }
}

/// The Trio - X3 mult if hand contains exactly three of a kind
#[derive(Debug, Clone)]
pub struct TheTrio {
    id: JokerId,
    name: String,
    description: String,
    rarity: JokerRarity,
    cost: usize,
}

impl Default for TheTrio {
    fn default() -> Self {
        Self::new()
    }
}

impl TheTrio {
    pub fn new() -> Self {
        Self {
            id: JokerId::TheTrio,
            name: "The Trio".to_string(),
            description: "X3 Mult if hand contains exactly three of a kind".to_string(),
            rarity: JokerRarity::Uncommon,
            cost: 6,
        }
    }

    fn is_exactly_three_of_kind(_context: &ProcessContext) -> bool {
        // Check if the hand rank is exactly three of a kind
        // This would need to check the actual hand evaluation
        // For now, placeholder implementation
        false
    }
}

impl JokerIdentity for TheTrio {
    fn joker_type(&self) -> &'static str {
        "the_trio"
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

impl Joker for TheTrio {
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
        // TODO: Check if the hand rank is exactly three of a kind
        // For now, return no effect
        JokerEffect::new()
    }
}

impl JokerGameplay for TheTrio {
    fn process(&mut self, stage: &Stage, context: &mut ProcessContext) -> ProcessResult {
        if !matches!(stage, Stage::Blind(_)) {
            return ProcessResult::default();
        }

        if Self::is_exactly_three_of_kind(context) {
            ProcessResult {
                mult_multiplier: 3.0,
                message: Some("The Trio: X3 Mult (Three of a Kind)".to_string()),
                ..Default::default()
            }
        } else {
            ProcessResult::default()
        }
    }

    fn can_trigger(&self, stage: &Stage, _context: &ProcessContext) -> bool {
        matches!(stage, Stage::Blind(_))
    }
}

/// Factory functions for creating multiplicative jokers
pub fn create_baron() -> Box<dyn Joker> {
    Box::new(Baron::new())
}

pub fn create_steel_joker() -> Box<dyn Joker> {
    Box::new(SteelJoker::new())
}

pub fn create_ancient_joker() -> Box<dyn Joker> {
    Box::new(AncientJoker::new())
}

pub fn create_the_duo() -> Box<dyn Joker> {
    Box::new(TheDuo::new())
}

pub fn create_the_trio() -> Box<dyn Joker> {
    Box::new(TheTrio::new())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::joker_state::JokerStateManager;
    use crate::stage::Blind;

    #[test]
    fn test_steel_joker_identity() {
        let steel = SteelJoker::new();
        assert_eq!(steel.id(), JokerId::SteelJoker);
        assert_eq!(Joker::name(&steel), "Steel Joker");
        assert_eq!(
            Joker::description(&steel),
            "Gives X0.5 Mult for each Steel Card in your full deck"
        );
        assert_eq!(Joker::rarity(&steel), JokerRarity::Uncommon);
        assert_eq!(steel.cost(), 6);
    }

    #[test]
    fn test_steel_joker_multiplier_calculation() {
        use crate::hand::SelectHand;
        use crate::joker::test_utils::TestContextBuilder;

        let steel = SteelJoker::new();
        let select_hand = SelectHand::new(vec![]);

        // Test with 0 steel cards - should return no effect
        let mut context = TestContextBuilder::new()
            .with_steel_cards_in_deck(0)
            .build();
        let effect = steel.on_hand_played(&mut context, &select_hand);
        assert_eq!(effect.mult_multiplier, 0.0); // No multiplier effect
        assert!(effect.message.is_none());

        // Test with 1 steel card - should give X1.5 Mult (1.0 + 0.5)
        let mut context = TestContextBuilder::new()
            .with_steel_cards_in_deck(1)
            .build();
        let effect = steel.on_hand_played(&mut context, &select_hand);
        assert_eq!(effect.mult_multiplier, 1.5); // 1.0 + (1 * 0.5) = 1.5
        assert!(effect.message.is_some());

        // Test with 2 steel cards - should give X2.0 Mult (1.0 + 1.0)
        let mut context = TestContextBuilder::new()
            .with_steel_cards_in_deck(2)
            .build();
        let effect = steel.on_hand_played(&mut context, &select_hand);
        assert_eq!(effect.mult_multiplier, 2.0); // 1.0 + (2 * 0.5) = 2.0

        // Test with 3 steel cards - should give X2.5 Mult (1.0 + 1.5)
        let mut context = TestContextBuilder::new()
            .with_steel_cards_in_deck(3)
            .build();
        let effect = steel.on_hand_played(&mut context, &select_hand);
        assert_eq!(effect.mult_multiplier, 2.5); // 1.0 + (3 * 0.5) = 2.5
    }

    #[test]
    fn test_steel_joker_edge_cases() {
        use crate::hand::SelectHand;
        use crate::joker::test_utils::TestContextBuilder;

        let steel = SteelJoker::new();
        let select_hand = SelectHand::new(vec![]);

        // Test with maximum reasonable steel card count
        let mut context = TestContextBuilder::new()
            .with_steel_cards_in_deck(10)
            .build();
        let effect = steel.on_hand_played(&mut context, &select_hand);
        assert_eq!(effect.mult_multiplier, 6.0); // 1.0 + (10 * 0.5) = 6.0
        assert!(effect
            .message
            .as_ref()
            .unwrap()
            .contains("Steel Joker: X6.0 Mult (10 Steel cards)"));

        // Test message format consistency
        let mut context = TestContextBuilder::new()
            .with_steel_cards_in_deck(1)
            .build();
        let effect = steel.on_hand_played(&mut context, &select_hand);
        assert!(effect
            .message
            .as_ref()
            .unwrap()
            .contains("Steel Joker: X1.5 Mult (1 Steel cards)"));

        // Verify no other effects are set (chips, mult, money should be default)
        assert_eq!(effect.chips, 0);
        assert_eq!(effect.mult, 0);
        assert_eq!(effect.money, 0);
    }

    #[test]
    fn test_baron_no_kings() {
        let mut baron = Baron::new();
        let hand = SelectHand::new(vec![
            Card::new(Value::Ace, Suit::Heart),
            Card::new(Value::Two, Suit::Spade),
        ]);

        // Test JokerGameplay process method
        let mut hand_score = crate::joker::traits::HandScore {
            chips: 0,
            mult: 0.0,
        };
        let played_cards = vec![];
        let held_cards = vec![];
        let mut events = vec![];

        let state_manager = JokerStateManager::new();
        let mut context = ProcessContext {
            hand_score: &mut hand_score,
            played_cards: &played_cards,
            held_cards: &held_cards,
            events: &mut events,
            hand: &hand,
            joker_state_manager: &state_manager,
        };

        let stage = Stage::Blind(Blind::Small);
        let result = baron.process(&stage, &mut context);

        // With no kings, should return default (no multiplier)
        assert_eq!(result.mult_multiplier, 1.0);
    }

    #[test]
    fn test_baron_exponential_scaling() {
        // Test exponential scaling with multiple kings
        // 1 King: X1.5
        // 2 Kings: X2.25 (1.5^2)
        // 3 Kings: X3.375 (1.5^3)
        // 4 Kings: X5.0625 (1.5^4)
    }

    #[test]
    fn test_ancient_joker_suit_rotation() {
        let mut ancient = AncientJoker::new();
        assert_eq!(ancient.selected_suit, Suit::Spade);

        // Use the JokerLifecycle trait method directly
        use crate::joker::traits::JokerLifecycle;

        JokerLifecycle::on_round_end(&mut ancient);
        assert_eq!(ancient.selected_suit, Suit::Heart);

        JokerLifecycle::on_round_end(&mut ancient);
        assert_eq!(ancient.selected_suit, Suit::Diamond);

        JokerLifecycle::on_round_end(&mut ancient);
        assert_eq!(ancient.selected_suit, Suit::Club);

        JokerLifecycle::on_round_end(&mut ancient);
        assert_eq!(ancient.selected_suit, Suit::Spade);
    }
}
