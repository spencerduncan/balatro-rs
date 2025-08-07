//! Retrigger Jokers implementation
//!
//! This module implements jokers that retrigger card effects.
//! These jokers cause cards to be scored multiple times, multiplying their effects.

use crate::{
    card::{Card, Value},
    hand::SelectHand,
    joker::{
        traits::{JokerState, ProcessContext, ProcessResult, Rarity},
        GameContext, Joker, JokerEffect, JokerGameplay, JokerId, JokerIdentity, JokerRarity,
    },
    stage::Stage,
};
use serde_json::json;

/// Dusk - Retriggers all played cards in the last hand of the round
#[derive(Debug, Clone)]
pub struct DuskJoker {
    id: JokerId,
    name: String,
    description: String,
    rarity: JokerRarity,
    cost: usize,
}

impl Default for DuskJoker {
    fn default() -> Self {
        Self::new()
    }
}

impl DuskJoker {
    pub fn new() -> Self {
        Self {
            id: JokerId::Dusk,
            name: "Dusk".to_string(),
            description: "Retriggers all played cards in final hand of round".to_string(),
            rarity: JokerRarity::Uncommon,
            cost: 6,
        }
    }
}

impl JokerIdentity for DuskJoker {
    fn joker_type(&self) -> &'static str {
        "dusk"
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

impl Joker for DuskJoker {
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
        // TODO: Check if this is the final hand of the round
        // For now, we can't detect this without additional game state
        // This would need to be implemented with proper game state tracking
        JokerEffect::new()
    }
}

impl JokerGameplay for DuskJoker {
    fn process(&mut self, stage: &Stage, _context: &mut ProcessContext) -> ProcessResult {
        if matches!(stage, Stage::Blind(_)) {
            // TODO: Check if this is the final hand
            // For now, return default - needs game state integration
            ProcessResult::default()
        } else {
            ProcessResult::default()
        }
    }

    fn can_trigger(&self, stage: &Stage, _context: &ProcessContext) -> bool {
        matches!(stage, Stage::Blind(_))
    }
}

/// Seltzer - Retriggers all played cards for 10 hands, then self-destructs
#[derive(Debug, Clone)]
pub struct SeltzerJoker {
    id: JokerId,
    name: String,
    description: String,
    rarity: JokerRarity,
    cost: usize,
    hands_remaining: u32,
}

impl Default for SeltzerJoker {
    fn default() -> Self {
        Self::new()
    }
}

impl SeltzerJoker {
    pub fn new() -> Self {
        Self {
            id: JokerId::Seltzer,
            name: "Seltzer".to_string(),
            description: "Retriggers all played cards for next 10 hands".to_string(),
            rarity: JokerRarity::Uncommon,
            cost: 5,
            hands_remaining: 10,
        }
    }
}

impl JokerIdentity for SeltzerJoker {
    fn joker_type(&self) -> &'static str {
        "seltzer"
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

impl Joker for SeltzerJoker {
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
        // Get current hands remaining from state
        let hands_remaining = context
            .joker_state_manager
            .get_state(self.id())
            .map(|state| state.accumulated_value as u32)
            .unwrap_or(10);

        if hands_remaining > 0 {
            // Decrease counter
            context
                .joker_state_manager
                .update_state(self.id(), |state| {
                    state.accumulated_value = (hands_remaining - 1) as f64;
                });

            let mut effect = JokerEffect::new()
                .with_retrigger(1) // Retrigger all cards once
                .with_message(format!("Seltzer: {} hands remaining", hands_remaining - 1));

            // Self-destruct if this was the last hand
            if hands_remaining == 1 {
                effect.destroy_self = true;
                effect.message = Some("Seltzer: Last hand! Self-destructing...".to_string());
            }

            effect
        } else {
            JokerEffect::new()
        }
    }

    fn on_created(&self, context: &mut GameContext) -> JokerEffect {
        // Initialize state with 10 hands
        context
            .joker_state_manager
            .update_state(self.id(), |state| {
                state.accumulated_value = 10.0;
            });
        JokerEffect::new()
    }
}

impl JokerState for SeltzerJoker {
    fn has_state(&self) -> bool {
        true
    }

    fn serialize_state(&self) -> Option<serde_json::Value> {
        Some(json!({
            "hands_remaining": self.hands_remaining
        }))
    }

    fn deserialize_state(&mut self, value: serde_json::Value) -> Result<(), String> {
        if let Some(hands) = value.get("hands_remaining").and_then(|v| v.as_u64()) {
            self.hands_remaining = hands as u32;
            Ok(())
        } else {
            Err("Invalid state format".to_string())
        }
    }
}

impl JokerGameplay for SeltzerJoker {
    fn process(&mut self, stage: &Stage, _context: &mut ProcessContext) -> ProcessResult {
        if matches!(stage, Stage::Blind(_)) && self.hands_remaining > 0 {
            self.hands_remaining = self.hands_remaining.saturating_sub(1);

            ProcessResult {
                retriggered: true,
                ..Default::default()
            }
        } else {
            ProcessResult::default()
        }
    }

    fn can_trigger(&self, stage: &Stage, _context: &ProcessContext) -> bool {
        matches!(stage, Stage::Blind(_)) && self.hands_remaining > 0
    }
}

/// Hanging Chad - Retriggers the first card scored twice
#[derive(Debug, Clone)]
pub struct HangingChadJoker {
    id: JokerId,
    name: String,
    description: String,
    rarity: JokerRarity,
    cost: usize,
}

impl Default for HangingChadJoker {
    fn default() -> Self {
        Self::new()
    }
}

impl HangingChadJoker {
    pub fn new() -> Self {
        Self {
            id: JokerId::Hanging,
            name: "Hanging Chad".to_string(),
            description: "Retriggers first card scored 2 times".to_string(),
            rarity: JokerRarity::Common,
            cost: 4,
        }
    }
}

impl JokerIdentity for HangingChadJoker {
    fn joker_type(&self) -> &'static str {
        "hanging_chad"
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

impl Joker for HangingChadJoker {
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

    fn on_card_scored(&self, context: &mut GameContext, card: &Card) -> JokerEffect {
        // Check if this is the first card scored
        // We track this by checking if we've already triggered this hand
        let has_triggered = context
            .joker_state_manager
            .get_state(self.id())
            .map(|state| state.accumulated_value > 0.5)
            .unwrap_or(false);

        if !has_triggered {
            // Mark as triggered for this hand
            context
                .joker_state_manager
                .update_state(self.id(), |state| {
                    state.accumulated_value = 1.0;
                });

            JokerEffect::new()
                .with_retrigger(2) // Retrigger twice
                .with_message(format!(
                    "Hanging Chad: First card ({:?}) retriggered!",
                    card.value
                ))
        } else {
            JokerEffect::new()
        }
    }

    fn on_hand_played(&self, context: &mut GameContext, _hand: &SelectHand) -> JokerEffect {
        // Reset the trigger state for the new hand
        context
            .joker_state_manager
            .update_state(self.id(), |state| {
                state.accumulated_value = 0.0;
            });
        JokerEffect::new()
    }
}

impl JokerState for HangingChadJoker {
    fn has_state(&self) -> bool {
        true
    }
}

impl JokerGameplay for HangingChadJoker {
    fn process(&mut self, stage: &Stage, _context: &mut ProcessContext) -> ProcessResult {
        if matches!(stage, Stage::Blind(_)) && !_context.played_cards.is_empty() {
            // Only retrigger the first card
            ProcessResult {
                retriggered: true,
                ..Default::default()
            }
        } else {
            ProcessResult::default()
        }
    }

    fn can_trigger(&self, stage: &Stage, context: &ProcessContext) -> bool {
        matches!(stage, Stage::Blind(_)) && !context.played_cards.is_empty()
    }
}

/// Sock and Buskin - Retriggers all face cards
#[derive(Debug, Clone)]
pub struct SockAndBuskinJoker {
    id: JokerId,
    name: String,
    description: String,
    rarity: JokerRarity,
    cost: usize,
}

impl Default for SockAndBuskinJoker {
    fn default() -> Self {
        Self::new()
    }
}

impl SockAndBuskinJoker {
    pub fn new() -> Self {
        Self {
            id: JokerId::SockAndBuskin,
            name: "Sock and Buskin".to_string(),
            description: "Retriggers all face cards".to_string(),
            rarity: JokerRarity::Uncommon,
            cost: 5,
        }
    }

    fn is_face_card(card: &Card) -> bool {
        matches!(card.value, Value::Jack | Value::Queen | Value::King)
    }
}

impl JokerIdentity for SockAndBuskinJoker {
    fn joker_type(&self) -> &'static str {
        "sock_and_buskin"
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

impl Joker for SockAndBuskinJoker {
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
                .with_retrigger(1)
                .with_message(format!("Sock and Buskin: {:?} retriggered!", card.value))
        } else {
            JokerEffect::new()
        }
    }
}

impl JokerGameplay for SockAndBuskinJoker {
    fn process(&mut self, stage: &Stage, _context: &mut ProcessContext) -> ProcessResult {
        if !matches!(stage, Stage::Blind(_)) {
            return ProcessResult::default();
        }

        let has_face_cards = _context.played_cards.iter().any(Self::is_face_card);

        if has_face_cards {
            ProcessResult {
                retriggered: true,
                ..Default::default()
            }
        } else {
            ProcessResult::default()
        }
    }

    fn can_trigger(&self, stage: &Stage, context: &ProcessContext) -> bool {
        matches!(stage, Stage::Blind(_)) && context.played_cards.iter().any(Self::is_face_card)
    }
}

/// Hack - Retriggers all played 2, 3, 4, or 5
#[derive(Debug, Clone)]
pub struct HackJoker {
    id: JokerId,
    name: String,
    description: String,
    rarity: JokerRarity,
    cost: usize,
}

impl Default for HackJoker {
    fn default() -> Self {
        Self::new()
    }
}

impl HackJoker {
    pub fn new() -> Self {
        Self {
            id: JokerId::Hack,
            name: "Hack".to_string(),
            description: "Retriggers all played 2s, 3s, 4s, and 5s".to_string(),
            rarity: JokerRarity::Uncommon,
            cost: 5,
        }
    }

    fn is_target_rank(card: &Card) -> bool {
        matches!(
            card.value,
            Value::Two | Value::Three | Value::Four | Value::Five
        )
    }
}

impl JokerIdentity for HackJoker {
    fn joker_type(&self) -> &'static str {
        "hack"
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

impl Joker for HackJoker {
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
        if Self::is_target_rank(card) {
            JokerEffect::new()
                .with_retrigger(1)
                .with_message(format!("Hack: {:?} retriggered!", card.value))
        } else {
            JokerEffect::new()
        }
    }
}

impl JokerGameplay for HackJoker {
    fn process(&mut self, stage: &Stage, _context: &mut ProcessContext) -> ProcessResult {
        if !matches!(stage, Stage::Blind(_)) {
            return ProcessResult::default();
        }

        let has_target_ranks = _context.played_cards.iter().any(Self::is_target_rank);

        if has_target_ranks {
            ProcessResult {
                retriggered: true,
                ..Default::default()
            }
        } else {
            ProcessResult::default()
        }
    }

    fn can_trigger(&self, stage: &Stage, context: &ProcessContext) -> bool {
        matches!(stage, Stage::Blind(_)) && context.played_cards.iter().any(Self::is_target_rank)
    }
}

/// Factory functions for creating retrigger jokers
pub fn create_dusk_joker() -> Box<dyn Joker> {
    Box::new(DuskJoker::new())
}

pub fn create_seltzer_joker() -> Box<dyn Joker> {
    Box::new(SeltzerJoker::new())
}

pub fn create_hanging_chad_joker() -> Box<dyn Joker> {
    Box::new(HangingChadJoker::new())
}

pub fn create_sock_and_buskin_joker() -> Box<dyn Joker> {
    Box::new(SockAndBuskinJoker::new())
}

pub fn create_hack_joker() -> Box<dyn Joker> {
    Box::new(HackJoker::new())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::{Card, Suit, Value};

    #[test]
    fn test_dusk() {
        let dusk = DuskJoker::new();

        // Test identity
        assert_eq!(dusk.joker_type(), "dusk");
        assert_eq!(JokerIdentity::name(&dusk), "Dusk");
        assert_eq!(dusk.base_cost(), 6);
        assert_eq!(dusk.rarity, JokerRarity::Uncommon);
    }

    #[test]
    fn test_seltzer() {
        let seltzer = SeltzerJoker::new();

        // Test identity
        assert_eq!(seltzer.joker_type(), "seltzer");
        assert_eq!(JokerIdentity::name(&seltzer), "Seltzer");
        assert_eq!(seltzer.base_cost(), 5);

        // Test state
        let mutable_seltzer = SeltzerJoker::new();
        assert!(mutable_seltzer.has_state());

        let state = JokerState::serialize_state(&mutable_seltzer).unwrap();
        assert_eq!(state["hands_remaining"], 10);
    }

    #[test]
    fn test_seltzer_countdown() {
        let seltzer = SeltzerJoker::new();
        let mut test_context = crate::joker::test_utils::TestContextBuilder::new().build();

        // Initialize state
        seltzer.on_created(&mut test_context);

        // Play a hand
        let hand = SelectHand::new(vec![]);
        let effect = seltzer.on_hand_played(&mut test_context, &hand);

        // Should retrigger with 9 hands remaining
        assert_eq!(effect.retrigger, 1);
        assert!(effect.message.unwrap().contains("9 hands remaining"));
    }

    #[test]
    fn test_hanging_chad() {
        let chad = HangingChadJoker::new();

        // Test identity
        assert_eq!(chad.joker_type(), "hanging_chad");
        assert_eq!(JokerIdentity::name(&chad), "Hanging Chad");
        assert_eq!(chad.base_cost(), 4);
        assert_eq!(chad.rarity, JokerRarity::Common);
    }

    #[test]
    fn test_hanging_chad_first_card_only() {
        let chad = HangingChadJoker::new();
        let mut test_context = crate::joker::test_utils::TestContextBuilder::new().build();

        // Reset state for new hand
        let hand = SelectHand::new(vec![]);
        chad.on_hand_played(&mut test_context, &hand);

        // First card should be retriggered
        let first_card = Card::new(Value::Ace, Suit::Spade);
        let effect1 = chad.on_card_scored(&mut test_context, &first_card);
        assert_eq!(effect1.retrigger, 2);

        // Second card should not be retriggered
        let second_card = Card::new(Value::King, Suit::Heart);
        let effect2 = chad.on_card_scored(&mut test_context, &second_card);
        assert_eq!(effect2.retrigger, 0);
    }

    #[test]
    fn test_sock_and_buskin() {
        let sock = SockAndBuskinJoker::new();

        // Test identity
        assert_eq!(sock.joker_type(), "sock_and_buskin");
        assert_eq!(JokerIdentity::name(&sock), "Sock and Buskin");
        assert_eq!(sock.base_cost(), 5);
        assert_eq!(sock.rarity, JokerRarity::Uncommon);
    }

    #[test]
    fn test_sock_and_buskin_face_cards() {
        let sock = SockAndBuskinJoker::new();
        let mut test_context = crate::joker::test_utils::TestContextBuilder::new().build();

        // Face cards should be retriggered
        let jack = Card::new(Value::Jack, Suit::Diamond);
        let effect_jack = sock.on_card_scored(&mut test_context, &jack);
        assert_eq!(effect_jack.retrigger, 1);

        let queen = Card::new(Value::Queen, Suit::Club);
        let effect_queen = sock.on_card_scored(&mut test_context, &queen);
        assert_eq!(effect_queen.retrigger, 1);

        let king = Card::new(Value::King, Suit::Heart);
        let effect_king = sock.on_card_scored(&mut test_context, &king);
        assert_eq!(effect_king.retrigger, 1);

        // Non-face cards should not be retriggered
        let ace = Card::new(Value::Ace, Suit::Spade);
        let effect_ace = sock.on_card_scored(&mut test_context, &ace);
        assert_eq!(effect_ace.retrigger, 0);

        let ten = Card::new(Value::Ten, Suit::Heart);
        let effect_ten = sock.on_card_scored(&mut test_context, &ten);
        assert_eq!(effect_ten.retrigger, 0);
    }

    #[test]
    fn test_face_card_detection() {
        assert!(SockAndBuskinJoker::is_face_card(&Card::new(
            Value::Jack,
            Suit::Heart
        )));
        assert!(SockAndBuskinJoker::is_face_card(&Card::new(
            Value::Queen,
            Suit::Diamond
        )));
        assert!(SockAndBuskinJoker::is_face_card(&Card::new(
            Value::King,
            Suit::Spade
        )));

        assert!(!SockAndBuskinJoker::is_face_card(&Card::new(
            Value::Ace,
            Suit::Club
        )));
        assert!(!SockAndBuskinJoker::is_face_card(&Card::new(
            Value::Ten,
            Suit::Heart
        )));
        assert!(!SockAndBuskinJoker::is_face_card(&Card::new(
            Value::Two,
            Suit::Diamond
        )));
    }

    #[test]
    fn test_hack() {
        let hack = HackJoker::new();

        // Test identity
        assert_eq!(hack.joker_type(), "hack");
        assert_eq!(JokerIdentity::name(&hack), "Hack");
        assert_eq!(hack.base_cost(), 5);
        assert_eq!(hack.rarity, JokerRarity::Uncommon);
    }

    #[test]
    fn test_hack_target_ranks() {
        let hack = HackJoker::new();
        let mut test_context = crate::joker::test_utils::TestContextBuilder::new().build();

        // Target ranks (2, 3, 4, 5) should be retriggered
        let two = Card::new(Value::Two, Suit::Diamond);
        let effect_two = hack.on_card_scored(&mut test_context, &two);
        assert_eq!(effect_two.retrigger, 1);
        assert!(effect_two.message.unwrap().contains("Two"));

        let three = Card::new(Value::Three, Suit::Club);
        let effect_three = hack.on_card_scored(&mut test_context, &three);
        assert_eq!(effect_three.retrigger, 1);
        assert!(effect_three.message.unwrap().contains("Three"));

        let four = Card::new(Value::Four, Suit::Heart);
        let effect_four = hack.on_card_scored(&mut test_context, &four);
        assert_eq!(effect_four.retrigger, 1);
        assert!(effect_four.message.unwrap().contains("Four"));

        let five = Card::new(Value::Five, Suit::Spade);
        let effect_five = hack.on_card_scored(&mut test_context, &five);
        assert_eq!(effect_five.retrigger, 1);
        assert!(effect_five.message.unwrap().contains("Five"));

        // Non-target ranks should not be retriggered
        let six = Card::new(Value::Six, Suit::Diamond);
        let effect_six = hack.on_card_scored(&mut test_context, &six);
        assert_eq!(effect_six.retrigger, 0);

        let ace = Card::new(Value::Ace, Suit::Spade);
        let effect_ace = hack.on_card_scored(&mut test_context, &ace);
        assert_eq!(effect_ace.retrigger, 0);

        let king = Card::new(Value::King, Suit::Heart);
        let effect_king = hack.on_card_scored(&mut test_context, &king);
        assert_eq!(effect_king.retrigger, 0);

        let ten = Card::new(Value::Ten, Suit::Club);
        let effect_ten = hack.on_card_scored(&mut test_context, &ten);
        assert_eq!(effect_ten.retrigger, 0);
    }

    #[test]
    fn test_target_rank_detection() {
        // Test target ranks (2, 3, 4, 5)
        assert!(HackJoker::is_target_rank(&Card::new(
            Value::Two,
            Suit::Heart
        )));
        assert!(HackJoker::is_target_rank(&Card::new(
            Value::Three,
            Suit::Diamond
        )));
        assert!(HackJoker::is_target_rank(&Card::new(
            Value::Four,
            Suit::Spade
        )));
        assert!(HackJoker::is_target_rank(&Card::new(
            Value::Five,
            Suit::Club
        )));

        // Test non-target ranks
        assert!(!HackJoker::is_target_rank(&Card::new(
            Value::Six,
            Suit::Heart
        )));
        assert!(!HackJoker::is_target_rank(&Card::new(
            Value::Seven,
            Suit::Diamond
        )));
        assert!(!HackJoker::is_target_rank(&Card::new(
            Value::Eight,
            Suit::Spade
        )));
        assert!(!HackJoker::is_target_rank(&Card::new(
            Value::Nine,
            Suit::Club
        )));
        assert!(!HackJoker::is_target_rank(&Card::new(
            Value::Ten,
            Suit::Heart
        )));
        assert!(!HackJoker::is_target_rank(&Card::new(
            Value::Jack,
            Suit::Diamond
        )));
        assert!(!HackJoker::is_target_rank(&Card::new(
            Value::Queen,
            Suit::Spade
        )));
        assert!(!HackJoker::is_target_rank(&Card::new(
            Value::King,
            Suit::Club
        )));
        assert!(!HackJoker::is_target_rank(&Card::new(
            Value::Ace,
            Suit::Heart
        )));
    }
}
