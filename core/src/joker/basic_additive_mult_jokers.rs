//! Basic Additive Mult Jokers implementation
//!
//! This module implements jokers that provide additive mult bonuses (+X Mult).
//! These jokers add fixed amounts of mult to the score under various conditions.

use crate::{
    card::{Card, Value},
    hand::SelectHand,
    joker::{
        traits::{ProcessContext, ProcessResult, Rarity},
        GameContext, Joker, JokerEffect, JokerGameplay, JokerId, JokerIdentity, JokerRarity,
    },
    stage::Stage,
};

/// Basic Joker - +4 Mult per hand
#[derive(Debug, Clone)]
pub struct BasicJoker {
    id: JokerId,
    name: String,
    description: String,
    rarity: JokerRarity,
    cost: usize,
}

impl Default for BasicJoker {
    fn default() -> Self {
        Self::new()
    }
}

impl BasicJoker {
    pub fn new() -> Self {
        Self {
            id: JokerId::Joker,
            name: "Joker".to_string(),
            description: "+4 Mult".to_string(),
            rarity: JokerRarity::Common,
            cost: 2,
        }
    }
}

impl JokerIdentity for BasicJoker {
    fn joker_type(&self) -> &'static str {
        "joker"
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

impl Joker for BasicJoker {
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
        JokerEffect::new()
            .with_mult(4)
            .with_message("Joker: +4 Mult".to_string())
    }
}

impl JokerGameplay for BasicJoker {
    fn process(&mut self, stage: &Stage, _context: &mut ProcessContext) -> ProcessResult {
        if matches!(stage, Stage::Blind(_)) {
            ProcessResult {
                mult_added: 4.0,
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

/// Even Steven - Even cards (2, 4, 6, 8, 10) give +4 Mult when scored
#[derive(Debug, Clone)]
pub struct EvenStevenJoker {
    id: JokerId,
    name: String,
    description: String,
    rarity: JokerRarity,
    cost: usize,
}

impl Default for EvenStevenJoker {
    fn default() -> Self {
        Self::new()
    }
}

impl EvenStevenJoker {
    pub fn new() -> Self {
        Self {
            id: JokerId::EvenSteven,
            name: "Even Steven".to_string(),
            description: "Played cards with even rank give +4 Mult when scored".to_string(),
            rarity: JokerRarity::Common,
            cost: 4,
        }
    }

    fn is_even_card(card: &Card) -> bool {
        matches!(
            card.value,
            Value::Two | Value::Four | Value::Six | Value::Eight | Value::Ten
        )
    }
}

impl JokerIdentity for EvenStevenJoker {
    fn joker_type(&self) -> &'static str {
        "even_steven"
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

impl Joker for EvenStevenJoker {
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
        if Self::is_even_card(card) {
            JokerEffect::new()
                .with_mult(4)
                .with_message(format!("Even Steven: +4 Mult ({:?})", card.value))
        } else {
            JokerEffect::new()
        }
    }
}

impl JokerGameplay for EvenStevenJoker {
    fn process(&mut self, stage: &Stage, context: &mut ProcessContext) -> ProcessResult {
        if !matches!(stage, Stage::Blind(_)) {
            return ProcessResult::default();
        }

        let even_count = context
            .played_cards
            .iter()
            .filter(|card| Self::is_even_card(card))
            .count();

        ProcessResult {
            mult_added: (even_count * 4) as f64,
            ..Default::default()
        }
    }

    fn can_trigger(&self, stage: &Stage, context: &ProcessContext) -> bool {
        matches!(stage, Stage::Blind(_)) && context.played_cards.iter().any(Self::is_even_card)
    }
}

/// Scholar - Aces give +20 Chips and +4 Mult when scored
#[derive(Debug, Clone)]
pub struct ScholarJoker {
    id: JokerId,
    name: String,
    description: String,
    rarity: JokerRarity,
    cost: usize,
}

impl Default for ScholarJoker {
    fn default() -> Self {
        Self::new()
    }
}

impl ScholarJoker {
    pub fn new() -> Self {
        Self {
            id: JokerId::Scholar,
            name: "Scholar".to_string(),
            description: "Played Aces give +20 Chips and +4 Mult when scored".to_string(),
            rarity: JokerRarity::Common,
            cost: 4,
        }
    }
}

impl JokerIdentity for ScholarJoker {
    fn joker_type(&self) -> &'static str {
        "scholar"
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

impl Joker for ScholarJoker {
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
        if card.value == Value::Ace {
            JokerEffect::new()
                .with_chips(20)
                .with_mult(4)
                .with_message("Scholar: +20 Chips, +4 Mult (Ace)".to_string())
        } else {
            JokerEffect::new()
        }
    }
}

impl JokerGameplay for ScholarJoker {
    fn process(&mut self, stage: &Stage, context: &mut ProcessContext) -> ProcessResult {
        if !matches!(stage, Stage::Blind(_)) {
            return ProcessResult::default();
        }

        let ace_count = context
            .played_cards
            .iter()
            .filter(|card| card.value == Value::Ace)
            .count();

        ProcessResult {
            chips_added: (ace_count * 20) as u64,
            mult_added: (ace_count * 4) as f64,
            ..Default::default()
        }
    }

    fn can_trigger(&self, stage: &Stage, context: &ProcessContext) -> bool {
        matches!(stage, Stage::Blind(_))
            && context
                .played_cards
                .iter()
                .any(|card| card.value == Value::Ace)
    }
}

/// Half Joker - +20 Mult if played hand has 4 or fewer cards
#[derive(Debug, Clone)]
pub struct HalfJoker {
    id: JokerId,
    name: String,
    description: String,
    rarity: JokerRarity,
    cost: usize,
}

impl Default for HalfJoker {
    fn default() -> Self {
        Self::new()
    }
}

impl HalfJoker {
    pub fn new() -> Self {
        Self {
            id: JokerId::HalfJoker,
            name: "Half Joker".to_string(),
            description: "+20 Mult if played hand has 4 or fewer cards".to_string(),
            rarity: JokerRarity::Common,
            cost: 3,
        }
    }
}

impl JokerIdentity for HalfJoker {
    fn joker_type(&self) -> &'static str {
        "half_joker"
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

impl Joker for HalfJoker {
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
        if hand.len() <= 4 {
            JokerEffect::new()
                .with_mult(20)
                .with_message(format!("Half Joker: +20 Mult ({} cards)", hand.len()))
        } else {
            JokerEffect::new()
        }
    }
}

impl JokerGameplay for HalfJoker {
    fn process(&mut self, stage: &Stage, context: &mut ProcessContext) -> ProcessResult {
        if !matches!(stage, Stage::Blind(_)) {
            return ProcessResult::default();
        }

        if context.played_cards.len() <= 4 {
            ProcessResult {
                mult_added: 20.0,
                ..Default::default()
            }
        } else {
            ProcessResult::default()
        }
    }

    fn can_trigger(&self, stage: &Stage, context: &ProcessContext) -> bool {
        matches!(stage, Stage::Blind(_)) && context.played_cards.len() <= 4
    }
}

/// Walkie Talkie - Each played 10 or 4 gives +10 Chips and +4 Mult when scored
#[derive(Debug, Clone)]
pub struct WalkieJoker {
    id: JokerId,
    name: String,
    description: String,
    rarity: JokerRarity,
    cost: usize,
}

impl Default for WalkieJoker {
    fn default() -> Self {
        Self::new()
    }
}

impl WalkieJoker {
    pub fn new() -> Self {
        Self {
            id: JokerId::Walkie,
            name: "Walkie Talkie".to_string(),
            description: "Each played 10 or 4 gives +10 Chips and +4 Mult when scored".to_string(),
            rarity: JokerRarity::Common,
            cost: 3,
        }
    }

    fn is_ten_or_four(card: &Card) -> bool {
        matches!(card.value, Value::Ten | Value::Four)
    }
}

impl JokerIdentity for WalkieJoker {
    fn joker_type(&self) -> &'static str {
        "walkie"
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

impl Joker for WalkieJoker {
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
        if Self::is_ten_or_four(card) {
            JokerEffect::new()
                .with_chips(10)
                .with_mult(4)
                .with_message(format!(
                    "Walkie Talkie: +10 Chips, +4 Mult ({:?})",
                    card.value
                ))
        } else {
            JokerEffect::new()
        }
    }
}

impl JokerGameplay for WalkieJoker {
    fn process(&mut self, stage: &Stage, context: &mut ProcessContext) -> ProcessResult {
        if !matches!(stage, Stage::Blind(_)) {
            return ProcessResult::default();
        }

        let tens_and_fours_count = context
            .played_cards
            .iter()
            .filter(|card| Self::is_ten_or_four(card))
            .count();

        ProcessResult {
            chips_added: (tens_and_fours_count * 10) as u64,
            mult_added: (tens_and_fours_count * 4) as f64,
            ..Default::default()
        }
    }

    fn can_trigger(&self, stage: &Stage, context: &ProcessContext) -> bool {
        matches!(stage, Stage::Blind(_)) && context.played_cards.iter().any(Self::is_ten_or_four)
    }
}

/// Factory functions for creating basic additive mult jokers
pub fn create_basic_joker() -> Box<dyn Joker> {
    Box::new(BasicJoker::new())
}

pub fn create_even_steven_joker() -> Box<dyn Joker> {
    Box::new(EvenStevenJoker::new())
}

pub fn create_scholar_joker() -> Box<dyn Joker> {
    Box::new(ScholarJoker::new())
}

pub fn create_half_joker() -> Box<dyn Joker> {
    Box::new(HalfJoker::new())
}

pub fn create_walkie_joker() -> Box<dyn Joker> {
    Box::new(WalkieJoker::new())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::{Card, Suit};
    use crate::hand::SelectHand;
    use crate::stage::Blind;

    #[test]
    fn test_basic_joker() {
        let basic = BasicJoker::new();

        // Test identity
        assert_eq!(basic.joker_type(), "joker");
        assert_eq!(JokerIdentity::name(&basic), "Joker");
        assert_eq!(basic.base_cost(), 2);

        // Test effect
        let mut test_context = crate::joker::test_utils::TestContextBuilder::new()
            .with_chips(100)
            .with_mult(10)
            .build();
        let hand = SelectHand::new(vec![]);

        let effect = basic.on_hand_played(&mut test_context, &hand);
        assert_eq!(effect.mult(), 4);
    }

    #[test]
    fn test_even_steven() {
        let mut even_steven = EvenStevenJoker::new();

        // Test identity
        assert_eq!(even_steven.joker_type(), "even_steven");
        assert_eq!(JokerIdentity::name(&even_steven), "Even Steven");
        assert_eq!(even_steven.base_cost(), 4);

        // Test with even cards
        let stage = Stage::Blind(Blind::Small);
        let played_cards = vec![
            Card::new(Value::Two, Suit::Heart),
            Card::new(Value::Four, Suit::Diamond),
            Card::new(Value::Three, Suit::Spade), // Odd
        ];

        let held_cards = vec![];
        let mut events = vec![];
        let mut hand_score = crate::joker::traits::HandScore {
            chips: 0,
            mult: 0.0,
        };
        let hand = SelectHand::new(played_cards.clone());

        let joker_state_manager = crate::joker_state::JokerStateManager::new();

        let mut context = ProcessContext {
            hand_score: &mut hand_score,
            played_cards: &played_cards,
            held_cards: &held_cards,
            events: &mut events,
            hand: &hand,
            joker_state_manager: &joker_state_manager,
        };

        let result = even_steven.process(&stage, &mut context);
        assert_eq!(result.mult_added, 8.0); // 2 even cards * 4 mult
    }

    #[test]
    fn test_scholar() {
        let mut scholar = ScholarJoker::new();

        // Test with aces
        let stage = Stage::Blind(Blind::Small);
        let played_cards = vec![
            Card::new(Value::Ace, Suit::Heart),
            Card::new(Value::Ace, Suit::Diamond),
            Card::new(Value::King, Suit::Spade),
        ];

        let held_cards = vec![];
        let mut events = vec![];
        let mut hand_score = crate::joker::traits::HandScore {
            chips: 0,
            mult: 0.0,
        };
        let hand = SelectHand::new(played_cards.clone());

        let joker_state_manager = crate::joker_state::JokerStateManager::new();

        let mut context = ProcessContext {
            hand_score: &mut hand_score,
            played_cards: &played_cards,
            held_cards: &held_cards,
            events: &mut events,
            hand: &hand,
            joker_state_manager: &joker_state_manager,
        };

        let result = scholar.process(&stage, &mut context);
        assert_eq!(result.chips_added, 40); // 2 aces * 20 chips
        assert_eq!(result.mult_added, 8.0); // 2 aces * 4 mult
    }

    #[test]
    fn test_half_joker() {
        let mut half_joker = HalfJoker::new();

        // Test with 4 cards
        let stage = Stage::Blind(Blind::Small);
        let played_cards = vec![
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::King, Suit::Diamond),
            Card::new(Value::King, Suit::Spade),
            Card::new(Value::King, Suit::Club),
        ];

        let held_cards = vec![];
        let mut events = vec![];
        let mut hand_score = crate::joker::traits::HandScore {
            chips: 0,
            mult: 0.0,
        };
        let hand = SelectHand::new(played_cards.clone());

        let joker_state_manager = crate::joker_state::JokerStateManager::new();

        let mut context = ProcessContext {
            hand_score: &mut hand_score,
            played_cards: &played_cards,
            held_cards: &held_cards,
            events: &mut events,
            hand: &hand,
            joker_state_manager: &joker_state_manager,
        };

        let result = half_joker.process(&stage, &mut context);
        assert_eq!(result.mult_added, 20.0);

        // Test with 5 cards (should not trigger)
        let played_cards_5 = vec![
            Card::new(Value::Two, Suit::Heart),
            Card::new(Value::Three, Suit::Diamond),
            Card::new(Value::Four, Suit::Spade),
            Card::new(Value::Five, Suit::Club),
            Card::new(Value::Six, Suit::Heart),
        ];

        let hand_5 = SelectHand::new(played_cards_5.clone());
        let mut context_5 = ProcessContext {
            hand_score: &mut hand_score,
            played_cards: &played_cards_5,
            held_cards: &held_cards,
            events: &mut events,
            hand: &hand_5,
            joker_state_manager: &joker_state_manager,
        };

        let result_5 = half_joker.process(&stage, &mut context_5);
        assert_eq!(result_5.mult_added, 0.0);
    }

    #[test]
    fn test_walkie_talkie_tens_and_fours() {
        let mut walkie = WalkieJoker::new();

        // Test identity
        assert_eq!(walkie.joker_type(), "walkie");
        assert_eq!(JokerIdentity::name(&walkie), "Walkie Talkie");
        assert_eq!(walkie.base_cost(), 3);

        // Test with 10s and 4s
        let stage = Stage::Blind(Blind::Small);
        let played_cards = vec![
            Card::new(Value::Ten, Suit::Heart),
            Card::new(Value::Four, Suit::Diamond),
            Card::new(Value::King, Suit::Spade), // Should not trigger
        ];

        let held_cards = vec![];
        let mut events = vec![];
        let mut hand_score = crate::joker::traits::HandScore {
            chips: 0,
            mult: 0.0,
        };
        let hand = SelectHand::new(played_cards.clone());

        let joker_state_manager = crate::joker_state::JokerStateManager::new();

        let mut context = ProcessContext {
            hand_score: &mut hand_score,
            played_cards: &played_cards,
            held_cards: &held_cards,
            events: &mut events,
            hand: &hand,
            joker_state_manager: &joker_state_manager,
        };

        let result = walkie.process(&stage, &mut context);
        assert_eq!(result.chips_added, 20); // 2 cards (10 + 4) * 10 chips
        assert_eq!(result.mult_added, 8.0); // 2 cards (10 + 4) * 4 mult

        // Test with no 10s or 4s
        let no_trigger_cards = vec![
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::Queen, Suit::Diamond),
            Card::new(Value::Jack, Suit::Spade),
        ];

        let hand_no_trigger = SelectHand::new(no_trigger_cards.clone());
        let mut context_no_trigger = ProcessContext {
            hand_score: &mut hand_score,
            played_cards: &no_trigger_cards,
            held_cards: &held_cards,
            events: &mut events,
            hand: &hand_no_trigger,
            joker_state_manager: &joker_state_manager,
        };

        let result_no_trigger = walkie.process(&stage, &mut context_no_trigger);
        assert_eq!(result_no_trigger.chips_added, 0);
        assert_eq!(result_no_trigger.mult_added, 0.0);

        // Test is_ten_or_four helper
        assert!(WalkieJoker::is_ten_or_four(&Card::new(
            Value::Ten,
            Suit::Heart
        )));
        assert!(WalkieJoker::is_ten_or_four(&Card::new(
            Value::Four,
            Suit::Spade
        )));
        assert!(!WalkieJoker::is_ten_or_four(&Card::new(
            Value::King,
            Suit::Heart
        )));
        assert!(!WalkieJoker::is_ten_or_four(&Card::new(
            Value::Ace,
            Suit::Diamond
        )));
    }
}
