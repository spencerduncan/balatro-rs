//! Special mechanic jokers using the new trait system
//!
//! This module implements jokers with unique mechanics that don't fit standard patterns.
//! These jokers use the new 5-trait system for modular, maintainable implementations.

use crate::card::{Card, Value};
use crate::hand::SelectHand;
use crate::joker::traits::{
    JokerGameplay, JokerIdentity, JokerLifecycle, JokerModifiers, JokerState as JokerStateTrait,
    ProcessContext, ProcessResult, Rarity,
};
use crate::joker::{GameContext, Joker, JokerEffect, JokerId, JokerRarity};
use crate::stage::Stage;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// ErosionJoker: +4 Mult for each card below 52 in deck
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ErosionJoker;

impl JokerIdentity for ErosionJoker {
    fn joker_type(&self) -> &'static str {
        "Erosion"
    }

    fn name(&self) -> &str {
        "Erosion"
    }

    fn description(&self) -> &str {
        "+4 Mult for each card below 52 in deck"
    }

    fn rarity(&self) -> Rarity {
        Rarity::Common
    }

    fn base_cost(&self) -> u64 {
        6
    }
}

impl JokerLifecycle for ErosionJoker {}

impl JokerGameplay for ErosionJoker {
    fn process(&mut self, _stage: &Stage, _context: &mut ProcessContext) -> ProcessResult {
        // Calculate cards missing from full deck (52)
        // Note: This would need access to deck size information
        // For now, use a placeholder calculation
        let mult_bonus = 4.0; // Placeholder

        ProcessResult {
            chips_added: 0,
            mult_added: mult_bonus,
            mult_multiplier: 1.0,
            retriggered: false,
            message: None,
        }
    }

    fn can_trigger(&self, _stage: &Stage, _context: &ProcessContext) -> bool {
        true // Always active
    }
}

impl JokerModifiers for ErosionJoker {}

impl JokerStateTrait for ErosionJoker {}

impl Joker for ErosionJoker {
    fn id(&self) -> JokerId {
        JokerId::Erosion
    }

    fn name(&self) -> &str {
        JokerIdentity::name(self)
    }

    fn description(&self) -> &str {
        JokerIdentity::description(self)
    }

    fn rarity(&self) -> JokerRarity {
        match JokerIdentity::rarity(self) {
            Rarity::Common => JokerRarity::Common,
            Rarity::Uncommon => JokerRarity::Uncommon,
            Rarity::Rare => JokerRarity::Rare,
            Rarity::Legendary => JokerRarity::Legendary,
        }
    }

    fn on_hand_played(&self, context: &mut GameContext, _hand: &SelectHand) -> JokerEffect {
        // Calculate cards missing from full deck (52)
        let cards_in_deck = context.cards_in_deck;
        let cards_missing = 52_i32.saturating_sub(cards_in_deck as i32);
        let mult_bonus = cards_missing * 4;

        JokerEffect::new().with_mult(mult_bonus)
    }
}

/// FigureJoker: $3 when face card played, face cards give +0 Chips
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FigureJoker;

impl JokerIdentity for FigureJoker {
    fn joker_type(&self) -> &'static str {
        "Figure"
    }

    fn name(&self) -> &str {
        "Figure"
    }

    fn description(&self) -> &str {
        "$3 when face card played, face cards give +0 Chips"
    }

    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
    }

    fn base_cost(&self) -> u64 {
        8
    }
}

impl JokerLifecycle for FigureJoker {}

impl JokerGameplay for FigureJoker {
    fn process(&mut self, _stage: &Stage, context: &mut ProcessContext) -> ProcessResult {
        let mut _money_earned = 0;

        // Award $3 for each face card played
        for card in context.played_cards {
            if matches!(card.value, Value::Jack | Value::Queen | Value::King) {
                _money_earned += 3;
            }
        }

        ProcessResult {
            chips_added: 0,
            mult_added: 0.0,
            mult_multiplier: 1.0,
            retriggered: false,
            message: None,
        }
    }

    fn can_trigger(&self, _stage: &Stage, context: &ProcessContext) -> bool {
        // Can trigger if any face cards are played
        context
            .played_cards
            .iter()
            .any(|card| matches!(card.value, Value::Jack | Value::Queen | Value::King))
    }
}

impl JokerModifiers for FigureJoker {}

impl JokerStateTrait for FigureJoker {}

impl Joker for FigureJoker {
    fn id(&self) -> JokerId {
        // Using a placeholder ID since Figure doesn't exist in the enum
        // This would need to be added to JokerId enum
        JokerId::Reserved
    }

    fn name(&self) -> &str {
        JokerIdentity::name(self)
    }

    fn description(&self) -> &str {
        JokerIdentity::description(self)
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Uncommon
    }

    fn on_card_scored(&self, _context: &mut GameContext, card: &Card) -> JokerEffect {
        if matches!(card.value, Value::Jack | Value::Queen | Value::King) {
            // Award money for face cards
            JokerEffect::new().with_money(3)
        } else {
            JokerEffect::new()
        }
    }

    fn modify_chips(&self, _context: &crate::joker::GameContext, base_chips: i32) -> i32 {
        // Face cards give +0 chips (override to 0)
        // This would need more complex logic to identify which cards are face cards
        // For now, just return base_chips
        base_chips
    }
}

/// FlowerPotJoker: +3 Mult if poker hand contains Diamond, Spade, Heart, Club
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FlowerPotJoker;

impl JokerIdentity for FlowerPotJoker {
    fn joker_type(&self) -> &'static str {
        "FlowerPot"
    }

    fn name(&self) -> &str {
        "Flower Pot"
    }

    fn description(&self) -> &str {
        "+3 Mult if poker hand contains Diamond, Spade, Heart, Club"
    }

    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
    }

    fn base_cost(&self) -> u64 {
        7
    }
}

impl JokerLifecycle for FlowerPotJoker {}

impl JokerGameplay for FlowerPotJoker {
    fn process(&mut self, _stage: &Stage, _context: &mut ProcessContext) -> ProcessResult {
        // Check if all 4 suits are present
        let mut suits = HashSet::new();
        for card in _context.played_cards {
            suits.insert(card.suit);
        }

        if suits.len() >= 4 {
            ProcessResult {
                chips_added: 0,
                mult_added: 3.0,
                mult_multiplier: 1.0,
                retriggered: false,
                message: None,
            }
        } else {
            ProcessResult::default()
        }
    }

    fn can_trigger(&self, _stage: &Stage, context: &ProcessContext) -> bool {
        // Check if all 4 suits are present in played cards
        let mut suits = HashSet::new();
        for card in context.played_cards {
            suits.insert(card.suit);
        }
        suits.len() >= 4
    }
}

impl JokerModifiers for FlowerPotJoker {}

impl JokerStateTrait for FlowerPotJoker {}

impl Joker for FlowerPotJoker {
    fn id(&self) -> JokerId {
        JokerId::Reserved2 // Placeholder, would need to add to enum
    }

    fn name(&self) -> &str {
        JokerIdentity::name(self)
    }

    fn description(&self) -> &str {
        JokerIdentity::description(self)
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Uncommon
    }

    fn on_hand_played(&self, _context: &mut GameContext, hand: &SelectHand) -> JokerEffect {
        // Check if all 4 suits are present
        let mut suits = HashSet::new();
        for card in hand.cards() {
            suits.insert(card.suit);
        }

        if suits.len() >= 4 {
            JokerEffect::new().with_mult(3)
        } else {
            JokerEffect::new()
        }
    }
}

/// BlueprintJoker: Copies ability of joker to the right
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BlueprintJoker {
    pub copied_effects: Vec<JokerEffect>,
}

impl BlueprintJoker {
    pub fn new() -> Self {
        Self {
            copied_effects: Vec::new(),
        }
    }
}

impl JokerIdentity for BlueprintJoker {
    fn joker_type(&self) -> &'static str {
        "Blueprint"
    }

    fn name(&self) -> &str {
        "Blueprint"
    }

    fn description(&self) -> &str {
        "Copies ability of joker to the right"
    }

    fn rarity(&self) -> Rarity {
        Rarity::Rare
    }

    fn base_cost(&self) -> u64 {
        10
    }
}

impl JokerLifecycle for BlueprintJoker {}

impl JokerGameplay for BlueprintJoker {
    fn process(&mut self, _stage: &Stage, _context: &mut ProcessContext) -> ProcessResult {
        // This would need complex logic to find the joker to the right
        // and copy its effects. For now, return default.
        ProcessResult::default()
    }

    fn can_trigger(&self, _stage: &Stage, _context: &ProcessContext) -> bool {
        // Would need to check if there's a joker to the right
        false
    }
}

impl JokerModifiers for BlueprintJoker {}

impl JokerStateTrait for BlueprintJoker {
    fn has_state(&self) -> bool {
        true
    }

    fn serialize_state(&self) -> Option<serde_json::Value> {
        serde_json::to_value(&self.copied_effects).ok()
    }

    fn deserialize_state(&mut self, value: serde_json::Value) -> Result<(), String> {
        self.copied_effects = serde_json::from_value(value)
            .map_err(|e| format!("Failed to deserialize Blueprint state: {e}"))?;
        Ok(())
    }

    fn debug_state(&self) -> String {
        format!("copied_effects: {:?}", self.copied_effects)
    }

    fn reset_state(&mut self) {
        self.copied_effects.clear();
    }
}

impl Joker for BlueprintJoker {
    fn id(&self) -> JokerId {
        JokerId::Blueprint
    }

    fn name(&self) -> &str {
        JokerIdentity::name(self)
    }

    fn description(&self) -> &str {
        JokerIdentity::description(self)
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Rare
    }

    fn on_hand_played(&self, _context: &mut GameContext, _hand: &SelectHand) -> JokerEffect {
        // Complex logic needed to find joker to the right and copy its effect
        // This would require access to the full joker collection and position tracking
        // For now, return empty effect
        JokerEffect::new()
    }
}

/// BraidedDeckJoker: Jokers to the right of first joker do not trigger
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BraidedDeckJoker;

impl JokerIdentity for BraidedDeckJoker {
    fn joker_type(&self) -> &'static str {
        "BraidedDeck"
    }

    fn name(&self) -> &str {
        "Braided Deck"
    }

    fn description(&self) -> &str {
        "Jokers to the right of first joker do not trigger"
    }

    fn rarity(&self) -> Rarity {
        Rarity::Rare
    }

    fn base_cost(&self) -> u64 {
        12
    }
}

impl JokerLifecycle for BraidedDeckJoker {}

impl JokerGameplay for BraidedDeckJoker {
    fn process(&mut self, _stage: &Stage, _context: &mut ProcessContext) -> ProcessResult {
        // This joker doesn't add effects itself, it prevents others from triggering
        ProcessResult::default()
    }

    fn can_trigger(&self, _stage: &Stage, _context: &ProcessContext) -> bool {
        true // Always active to suppress other jokers
    }

    fn get_priority(&self, _stage: &Stage) -> i32 {
        1000 // High priority to execute before other jokers
    }
}

impl JokerModifiers for BraidedDeckJoker {}

impl JokerStateTrait for BraidedDeckJoker {}

impl Joker for BraidedDeckJoker {
    fn id(&self) -> JokerId {
        JokerId::Reserved3 // Placeholder, would need to add to enum
    }

    fn name(&self) -> &str {
        JokerIdentity::name(self)
    }

    fn description(&self) -> &str {
        JokerIdentity::description(self)
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Rare
    }

    fn on_hand_played(&self, _context: &mut GameContext, _hand: &SelectHand) -> JokerEffect {
        // This joker affects the processing order, not direct effects
        JokerEffect::new()
    }
}

/// FourofaKindJoker: Jokers gain X4 Mult if 4 jokers, X3 if 3
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FourofaKindJoker;

impl JokerIdentity for FourofaKindJoker {
    fn joker_type(&self) -> &'static str {
        "FourofaKind"
    }

    fn name(&self) -> &str {
        "Four of a Kind"
    }

    fn description(&self) -> &str {
        "Jokers gain X4 Mult if 4 jokers, X3 if 3"
    }

    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
    }

    fn base_cost(&self) -> u64 {
        8
    }
}

impl JokerLifecycle for FourofaKindJoker {}

impl JokerGameplay for FourofaKindJoker {
    fn process(&mut self, _stage: &Stage, _context: &mut ProcessContext) -> ProcessResult {
        // This would need access to the joker collection to count jokers
        // For now, return default
        ProcessResult::default()
    }

    fn can_trigger(&self, _stage: &Stage, _context: &ProcessContext) -> bool {
        // Would need to check joker count
        true
    }
}

impl JokerModifiers for FourofaKindJoker {
    fn get_score_mult(&self) -> f64 {
        // This would need context to determine joker count
        // Return base multiplier for now
        1.0
    }
}

impl JokerStateTrait for FourofaKindJoker {}

impl Joker for FourofaKindJoker {
    fn id(&self) -> JokerId {
        JokerId::Reserved4 // Placeholder, would need to add to enum
    }

    fn name(&self) -> &str {
        JokerIdentity::name(self)
    }

    fn description(&self) -> &str {
        JokerIdentity::description(self)
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Uncommon
    }

    fn on_hand_played(&self, context: &mut GameContext, _hand: &SelectHand) -> JokerEffect {
        let joker_count = context.jokers.len();

        let multiplier = match joker_count {
            4 => 4.0,
            3 => 3.0,
            _ => 1.0,
        };

        if multiplier > 1.0 {
            JokerEffect::new().with_mult_multiplier(multiplier)
        } else {
            JokerEffect::new()
        }
    }
}

/// TheOrderJoker: Gain X3 Mult if poker hand is Straight, Flush, or Straight Flush
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TheOrderJoker;

impl JokerIdentity for TheOrderJoker {
    fn joker_type(&self) -> &'static str {
        "TheOrder"
    }

    fn name(&self) -> &str {
        "The Order"
    }

    fn description(&self) -> &str {
        "Gain X3 Mult if poker hand is Straight, Flush, or Straight Flush"
    }

    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
    }

    fn base_cost(&self) -> u64 {
        7
    }
}

impl JokerLifecycle for TheOrderJoker {}

impl JokerGameplay for TheOrderJoker {
    fn process(&mut self, _stage: &Stage, _context: &mut ProcessContext) -> ProcessResult {
        // Check hand rank in played cards
        // This would need access to hand evaluation logic
        ProcessResult::default()
    }

    fn can_trigger(&self, _stage: &Stage, _context: &ProcessContext) -> bool {
        // Would need to check if hand is Straight, Flush, or Straight Flush
        false
    }
}

impl JokerModifiers for TheOrderJoker {}

impl JokerStateTrait for TheOrderJoker {}

impl Joker for TheOrderJoker {
    fn id(&self) -> JokerId {
        JokerId::TheOrder
    }

    fn name(&self) -> &str {
        JokerIdentity::name(self)
    }

    fn description(&self) -> &str {
        JokerIdentity::description(self)
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Uncommon
    }

    fn on_hand_played(&self, _context: &mut GameContext, hand: &SelectHand) -> JokerEffect {
        // Check if hand is Straight, Flush, or Straight Flush
        let is_qualifying_hand = hand.is_straight().is_some()
            || hand.is_flush().is_some()
            || hand.is_straight_flush().is_some();

        if is_qualifying_hand {
            JokerEffect::new().with_mult_multiplier(3.0)
        } else {
            JokerEffect::new()
        }
    }
}

/// PhotographJoker: First played face card gives X2 Mult when scored
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PhotographJoker {
    pub face_card_triggered: bool,
}

impl PhotographJoker {
    pub fn new() -> Self {
        Self {
            face_card_triggered: false,
        }
    }
}

impl JokerIdentity for PhotographJoker {
    fn joker_type(&self) -> &'static str {
        "Photograph"
    }

    fn name(&self) -> &str {
        "Photograph"
    }

    fn description(&self) -> &str {
        "First played face card gives X2 Mult when scored"
    }

    fn rarity(&self) -> Rarity {
        Rarity::Common
    }

    fn base_cost(&self) -> u64 {
        5
    }
}

impl JokerLifecycle for PhotographJoker {
    fn on_round_start(&mut self) {
        // Reset internal state for the new round
        self.face_card_triggered = false;
    }
}

impl JokerGameplay for PhotographJoker {
    fn process(&mut self, _stage: &Stage, context: &mut ProcessContext) -> ProcessResult {
        // Use internal state directly - much cleaner!
        if !self.face_card_triggered {
            // Check if any played cards are face cards
            for card in context.played_cards {
                if matches!(card.value, Value::Jack | Value::Queen | Value::King) {
                    // Mark as triggered for this round
                    self.face_card_triggered = true;

                    // First face card gives X2 Mult
                    // Since we can't multiply existing mult directly, we'll add the current mult value
                    // The game system should handle this as a multiplicative effect
                    return ProcessResult {
                        chips_added: 0,
                        mult_added: context.hand_score.mult, // Double by adding current mult
                        mult_multiplier: 1.0,
                        retriggered: false,
                        message: None,
                    };
                }
            }
        }
        ProcessResult::default()
    }

    fn can_trigger(&self, _stage: &Stage, context: &ProcessContext) -> bool {
        // Use internal state directly - much cleaner!
        !self.face_card_triggered
            && context
                .played_cards
                .iter()
                .any(|card| matches!(card.value, Value::Jack | Value::Queen | Value::King))
    }
}

impl JokerModifiers for PhotographJoker {}

impl JokerStateTrait for PhotographJoker {
    fn has_state(&self) -> bool {
        true
    }

    fn serialize_state(&self) -> Option<serde_json::Value> {
        serde_json::to_value(self.face_card_triggered).ok()
    }

    fn deserialize_state(&mut self, value: serde_json::Value) -> Result<(), String> {
        self.face_card_triggered = serde_json::from_value(value)
            .map_err(|e| format!("Failed to deserialize Photograph state: {e}"))?;
        Ok(())
    }

    fn debug_state(&self) -> String {
        format!("face_card_triggered: {}", self.face_card_triggered)
    }

    fn reset_state(&mut self) {
        self.face_card_triggered = false;
    }
}

impl Joker for PhotographJoker {
    fn id(&self) -> JokerId {
        JokerId::Photograph
    }

    fn name(&self) -> &str {
        JokerIdentity::name(self)
    }

    fn description(&self) -> &str {
        JokerIdentity::description(self)
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Common
    }

    fn on_card_scored(&self, _context: &mut GameContext, card: &Card) -> JokerEffect {
        if !self.face_card_triggered
            && matches!(card.value, Value::Jack | Value::Queen | Value::King)
        {
            JokerEffect::new().with_mult_multiplier(2.0)
        } else {
            JokerEffect::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::{Card, Suit as CardSuit, Value};
    use crate::hand::SelectHand;
    // Note: These imports are used throughout the tests despite CI warnings
    #[allow(unused_imports)]
    use crate::joker::traits::{
        JokerGameplay, JokerIdentity, JokerLifecycle, JokerModifiers,
        JokerState as JokerStateTrait, Rarity,
    };
    use crate::joker_state::JokerStateManager;
    use crate::stage::{Blind, Stage};
    use std::sync::Arc;

    /// Helper function to create a test card
    fn create_card(suit: CardSuit, value: Value) -> Card {
        Card::new(value, suit)
    }

    /// Helper function to create a test blind stage
    fn create_blind_stage() -> Stage {
        Stage::Blind(Blind::Small)
    }

    #[test]
    fn test_photograph_triggers_on_first_face_card() {
        let mut joker = PhotographJoker::new();
        let state_manager = Arc::new(JokerStateManager::new());

        let mut hand_score = crate::joker::traits::HandScore {
            chips: 100,
            mult: 5.0,
        };
        let played_cards = vec![
            create_card(CardSuit::Heart, Value::Jack),
            create_card(CardSuit::Spade, Value::Ten),
        ];
        let held_cards = vec![];
        let mut events = vec![];
        let test_hand = SelectHand::new(played_cards.clone());

        let mut context = crate::joker::traits::ProcessContext {
            hand_score: &mut hand_score,
            played_cards: &played_cards,
            held_cards: &held_cards,
            events: &mut events,
            hand: &test_hand,
            joker_state_manager: &state_manager,
        };

        // First face card should trigger
        let blind_stage = create_blind_stage();
        let result = joker.process(&blind_stage, &mut context);
        assert_eq!(result.mult_added, 5.0); // Should double the current mult

        // Verify state was updated internally
        assert!(joker.face_card_triggered);
    }

    #[test]
    fn test_photograph_does_not_trigger_twice() {
        let mut joker = PhotographJoker::new();
        // Pre-set the triggered state using internal state
        joker.face_card_triggered = true;

        let state_manager = Arc::new(JokerStateManager::new());

        let mut hand_score = crate::joker::traits::HandScore {
            chips: 100,
            mult: 5.0,
        };
        let played_cards = vec![create_card(CardSuit::Heart, Value::King)];
        let held_cards = vec![];
        let mut events = vec![];
        let test_hand = SelectHand::new(played_cards.clone());

        let mut context = crate::joker::traits::ProcessContext {
            hand_score: &mut hand_score,
            played_cards: &played_cards,
            held_cards: &held_cards,
            events: &mut events,
            hand: &test_hand,
            joker_state_manager: &state_manager,
        };

        // Should not trigger again
        let blind_stage = create_blind_stage();
        let result = joker.process(&blind_stage, &mut context);
        assert_eq!(result.mult_added, 0.0);
    }

    #[test]
    fn test_photograph_can_trigger_checks_state() {
        let mut joker = PhotographJoker::new();
        let state_manager = Arc::new(JokerStateManager::new());

        let mut hand_score = crate::joker::traits::HandScore {
            chips: 100,
            mult: 5.0,
        };
        let played_cards = vec![create_card(CardSuit::Heart, Value::Queen)];
        let held_cards = vec![];
        let mut events = vec![];
        let test_hand = SelectHand::new(played_cards.clone());

        let context = crate::joker::traits::ProcessContext {
            hand_score: &mut hand_score,
            played_cards: &played_cards,
            held_cards: &held_cards,
            events: &mut events,
            hand: &test_hand,
            joker_state_manager: &state_manager,
        };

        // Should be able to trigger initially
        let blind_stage = create_blind_stage();
        assert!(joker.can_trigger(&blind_stage, &context));

        // Set triggered state using internal state
        joker.face_card_triggered = true;

        // Should not be able to trigger after state is set
        assert!(!joker.can_trigger(&blind_stage, &context));
    }

    #[test]
    fn test_photograph_no_face_cards() {
        let mut joker = PhotographJoker::new();
        let state_manager = Arc::new(JokerStateManager::new());

        let mut hand_score = crate::joker::traits::HandScore {
            chips: 100,
            mult: 5.0,
        };
        let played_cards = vec![
            create_card(CardSuit::Heart, Value::Ten),
            create_card(CardSuit::Spade, Value::Nine),
        ];
        let held_cards = vec![];
        let mut events = vec![];
        let test_hand = SelectHand::new(played_cards.clone());

        let mut context = crate::joker::traits::ProcessContext {
            hand_score: &mut hand_score,
            played_cards: &played_cards,
            held_cards: &held_cards,
            events: &mut events,
            hand: &test_hand,
            joker_state_manager: &state_manager,
        };

        // Should not trigger without face cards
        let blind_stage = create_blind_stage();
        let result = joker.process(&blind_stage, &mut context);
        assert_eq!(result.mult_added, 0.0);

        // State should remain untriggered
        assert!(!joker.face_card_triggered);
    }

    #[test]
    fn test_photograph_round_reset() {
        let mut joker = PhotographJoker::new();

        // Simulate being triggered
        joker.face_card_triggered = true;

        // Round reset should clear the flag
        joker.on_round_start();
        assert!(!joker.face_card_triggered);

        // NOTE: In actual game flow, the Game engine should also reset
        // the state in JokerStateManager by calling reset methods
    }
}
