//! Steel Joker - Proper deck composition implementation
//!
//! This module implements the correct Steel Joker that scans deck composition
//! for Steel Cards instead of using destruction triggers or scaling mechanics.

use crate::{
    hand::SelectHand,
    joker::{
        traits::{ProcessContext, ProcessResult, Rarity},
        GameContext, Joker, JokerEffect, JokerGameplay, JokerId, JokerIdentity, JokerRarity,
    },
    stage::Stage,
};

/// Steel Joker - Gives X-Mult for each Steel Card in your full deck
///
/// According to joker.json: "Gives X#1# Mult for each Steel Card in your full deck"
/// This implementation scans the deck composition using GameContext.steel_cards_in_deck
/// instead of listening for destruction events.
#[derive(Debug, Clone)]
pub struct SteelJoker {
    id: JokerId,
    name: String,
    description: String,
    rarity: JokerRarity,
    cost: usize,
    /// X-Mult bonus per Steel Card (default: 0.25)
    mult_per_steel_card: f64,
}

impl Default for SteelJoker {
    fn default() -> Self {
        Self::new()
    }
}

impl SteelJoker {
    pub fn new() -> Self {
        Self::new_with_multiplier(0.25)
    }

    /// Create Steel Joker with custom multiplier per Steel Card
    pub fn new_with_multiplier(mult_per_steel_card: f64) -> Self {
        Self {
            id: JokerId::SteelJoker,
            name: "Steel Joker".to_string(),
            description: format!(
                "Gives X{mult_per_steel_card:.2} Mult for each Steel Card in your full deck"
            ),
            rarity: JokerRarity::Uncommon,
            cost: 6,
            mult_per_steel_card,
        }
    }

    /// Calculate the current X-Mult multiplier based on Steel Cards in deck
    fn calculate_multiplier(&self, steel_cards_count: usize) -> f64 {
        1.0 + (self.mult_per_steel_card * steel_cards_count as f64)
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

    /// Main joker logic: Scan deck composition for Steel Cards and apply X-Mult
    fn on_hand_played(&self, context: &mut GameContext, _hand: &SelectHand) -> JokerEffect {
        let steel_cards_count = context.steel_cards_in_deck;

        if steel_cards_count == 0 {
            // No Steel Cards in deck - return neutral multiplier (1.0 = no change)
            return JokerEffect::new().with_mult_multiplier(1.0);
        }

        let current_multiplier = self.calculate_multiplier(steel_cards_count);

        JokerEffect::new()
            .with_mult_multiplier(current_multiplier)
            .with_message(format!(
            "Steel Joker: X{current_multiplier:.2} Mult ({steel_cards_count} Steel cards in deck)"
        ))
    }
}

impl JokerGameplay for SteelJoker {
    fn process(&mut self, stage: &Stage, _context: &mut ProcessContext) -> ProcessResult {
        if !matches!(stage, Stage::Blind(_)) {
            return ProcessResult::default();
        }

        // Note: ProcessContext doesn't have access to game resources like steel_cards_in_deck
        // The main logic is in the Joker trait's on_hand_played method which has GameContext
        ProcessResult::default()
    }

    fn can_trigger(&self, stage: &Stage, _context: &ProcessContext) -> bool {
        // Steel Joker can always trigger during blinds (deck composition check happens in on_hand_played)
        matches!(stage, Stage::Blind(_))
    }
}

/// Factory function for creating the correct Steel Joker
pub fn create_steel_joker() -> Box<dyn Joker> {
    Box::new(SteelJoker::new())
}

/// Factory function for creating Steel Joker with custom multiplier
pub fn create_steel_joker_with_multiplier(mult_per_steel_card: f64) -> Box<dyn Joker> {
    Box::new(SteelJoker::new_with_multiplier(mult_per_steel_card))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        joker_state::JokerStateManager,
        rank::HandRank,
        rng::GameRng,
        stage::{Blind, Stage},
    };
    use std::collections::HashMap;

    fn create_test_context(steel_cards_in_deck: usize) -> GameContext<'static> {
        use crate::hand::Hand;
        use std::sync::{Arc, OnceLock};

        static STAGE: Stage = Stage::Blind(Blind::Small);
        static HAND: OnceLock<Hand> = OnceLock::new();
        let hand = HAND.get_or_init(|| Hand::new(Vec::new()));

        static HAND_TYPE_COUNTS: OnceLock<HashMap<HandRank, u32>> = OnceLock::new();
        let hand_type_counts = HAND_TYPE_COUNTS.get_or_init(HashMap::new);

        static JOKER_STATE_MANAGER: OnceLock<Arc<JokerStateManager>> = OnceLock::new();
        let joker_state_manager =
            JOKER_STATE_MANAGER.get_or_init(|| Arc::new(JokerStateManager::new()));

        static RNG: OnceLock<GameRng> = OnceLock::new();
        let rng = RNG.get_or_init(|| GameRng::for_testing(42));

        GameContext {
            chips: 100,
            mult: 5,
            money: 10,
            ante: 1,
            round: 1,
            stage: &STAGE,
            hands_played: 0,
            discards_used: 0,
            hands_remaining: 4.0,
            jokers: &[],
            hand,
            discarded: &[],
            hand_type_counts,
            joker_state_manager,
            cards_in_deck: 52,
            stone_cards_in_deck: 0,
            steel_cards_in_deck,
            rng,
        }
    }

    #[test]
    fn test_steel_joker_identity() {
        let steel = SteelJoker::new();

        assert_eq!(steel.joker_type(), "steel_joker");
        assert_eq!(JokerIdentity::name(&steel), "Steel Joker");
        assert_eq!(JokerIdentity::rarity(&steel), Rarity::Uncommon);
        assert_eq!(steel.base_cost(), 6);
        assert_eq!(steel.id(), JokerId::SteelJoker);
    }

    #[test]
    fn test_steel_joker_no_steel_cards() {
        let steel = SteelJoker::new();
        let mut context = create_test_context(0);
        let hand = SelectHand::new(vec![]);

        let effect = steel.on_hand_played(&mut context, &hand);

        // No Steel Cards = no effect
        assert_eq!(effect.mult_multiplier, 1.0);
        assert!(effect.message.is_none());
    }

    #[test]
    fn test_steel_joker_single_steel_card() {
        let steel = SteelJoker::new();
        let mut context = create_test_context(1);
        let hand = SelectHand::new(vec![]);

        let effect = steel.on_hand_played(&mut context, &hand);

        // 1 Steel Card = X1.25 Mult (1.0 + 0.25 * 1)
        assert_eq!(effect.mult_multiplier, 1.25);
        assert!(effect.message.is_some());
        assert!(effect.message.as_ref().unwrap().contains("X1.25"));
        assert!(effect.message.as_ref().unwrap().contains("1 Steel cards"));
    }

    #[test]
    fn test_steel_joker_multiple_steel_cards() {
        let steel = SteelJoker::new();
        let mut context = create_test_context(5);
        let hand = SelectHand::new(vec![]);

        let effect = steel.on_hand_played(&mut context, &hand);

        // 5 Steel Cards = X2.25 Mult (1.0 + 0.25 * 5)
        assert_eq!(effect.mult_multiplier, 2.25);
        assert!(effect.message.as_ref().unwrap().contains("X2.25"));
        assert!(effect.message.as_ref().unwrap().contains("5 Steel cards"));
    }

    #[test]
    fn test_steel_joker_custom_multiplier() {
        let steel = SteelJoker::new_with_multiplier(0.5);
        let mut context = create_test_context(3);
        let hand = SelectHand::new(vec![]);

        let effect = steel.on_hand_played(&mut context, &hand);

        // 3 Steel Cards with 0.5 multiplier = X2.5 Mult (1.0 + 0.5 * 3)
        assert_eq!(effect.mult_multiplier, 2.5);
        assert!(effect.message.as_ref().unwrap().contains("X2.50"));
        assert!(effect.message.as_ref().unwrap().contains("3 Steel cards"));
    }

    #[test]
    fn test_steel_joker_gameplay_trait() {
        let mut steel = SteelJoker::new();
        let stage = Stage::Blind(Blind::Small);

        // Test can_trigger
        let played_cards = vec![];
        let held_cards = vec![];
        let mut events = vec![];
        let mut hand_score = crate::joker::traits::HandScore {
            chips: 0,
            mult: 0.0,
        };

        let joker_state_manager = JokerStateManager::new();
        let hand = SelectHand::new(played_cards.clone());

        let context = ProcessContext {
            hand_score: &mut hand_score,
            played_cards: &played_cards,
            held_cards: &held_cards,
            events: &mut events,
            hand: &hand,
            joker_state_manager: &joker_state_manager,
        };

        assert!(steel.can_trigger(&stage, &context));

        // Test process (should return default since logic is in on_hand_played)
        let mut context_mut = context;
        let result = steel.process(&stage, &mut context_mut);
        assert_eq!(result.chips_added, 0);
        assert_eq!(result.mult_multiplier, 1.0);
    }

    #[test]
    fn test_calculate_multiplier() {
        let steel = SteelJoker::new_with_multiplier(0.3);

        assert_eq!(steel.calculate_multiplier(0), 1.0);
        assert_eq!(steel.calculate_multiplier(1), 1.3);
        assert_eq!(steel.calculate_multiplier(2), 1.6);
        assert_eq!(steel.calculate_multiplier(10), 4.0);
    }

    #[test]
    fn test_factory_functions() {
        let steel_default = create_steel_joker();
        assert_eq!(steel_default.id(), JokerId::SteelJoker);
        assert_eq!(steel_default.name(), "Steel Joker");

        let steel_custom = create_steel_joker_with_multiplier(0.4);
        assert_eq!(steel_custom.id(), JokerId::SteelJoker);
        assert_eq!(steel_custom.name(), "Steel Joker");
    }

    #[test]
    fn test_performance_many_steel_cards() {
        let steel = SteelJoker::new();
        let mut context = create_test_context(100); // Many Steel Cards
        let hand = SelectHand::new(vec![]);

        let start = std::time::Instant::now();
        let effect = steel.on_hand_played(&mut context, &hand);
        let duration = start.elapsed();

        // Should be extremely fast (deck composition scan is O(1))
        // Relaxed to 1000 microseconds for debug builds and CI environments
        assert!(duration.as_micros() < 1000);
        assert_eq!(effect.mult_multiplier, 26.0); // 1.0 + 0.25 * 100
    }
}
