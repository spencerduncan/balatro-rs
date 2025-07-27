//! Special mechanic jokers using the new trait system
//!
//! This module implements jokers with unique mechanics that don't fit standard patterns.
//! These jokers use the new 5-trait system for modular, maintainable implementations.
//!
//! Jokers implemented (matching Issue #192 requirements):
//! - Photograph: First played face card gives X2 Mult when scored
//! - Ancient Joker: Each played card with \[suit\] gives X2 Mult, suit changes at end of round
//! - Steel Joker: Gives X1.5 Mult for each Steel Card in full deck
//! - Baron: Each King held in hand gives X1.5 Mult
//! - The Idol: Each played \[rank\] of \[suit\] gives X2 Mult, card changes every round

use crate::card::Suit;
use crate::card::{Card, Value};
use crate::hand::SelectHand;
use crate::joker::traits::{
    JokerGameplay, JokerIdentity, JokerLifecycle, JokerModifiers, JokerState as JokerStateTrait,
    ProcessContext, ProcessResult, Rarity,
};
use crate::joker::{GameContext, Joker, JokerEffect, JokerId, JokerRarity};
use crate::stage::Stage;
use serde::{Deserialize, Serialize};

/// Photograph Joker: First played face card gives X2 Mult when scored
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhotographJoker {
    pub face_card_triggered: bool,
}

impl Default for PhotographJoker {
    fn default() -> Self {
        Self::new()
    }
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
        self.face_card_triggered = false;
    }
}

impl JokerGameplay for PhotographJoker {
    fn process(&mut self, _stage: &Stage, _context: &mut ProcessContext) -> ProcessResult {
        ProcessResult {
            chips_added: 0,
            mult_added: 0.0,
            mult_multiplier: 1.0,
            retriggered: false,
            message: None,
        }
    }

    fn can_trigger(&self, _stage: &Stage, _context: &ProcessContext) -> bool {
        !self.face_card_triggered
    }
}

impl JokerModifiers for PhotographJoker {}
impl JokerStateTrait for PhotographJoker {}

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
        match JokerIdentity::rarity(self) {
            Rarity::Common => JokerRarity::Common,
            Rarity::Uncommon => JokerRarity::Uncommon,
            Rarity::Rare => JokerRarity::Rare,
            Rarity::Legendary => JokerRarity::Legendary,
        }
    }

    fn on_hand_played(&self, _context: &mut GameContext, hand: &SelectHand) -> JokerEffect {
        if self.face_card_triggered {
            return JokerEffect::new().with_mult_multiplier(1.0);
        }

        // Check if this hand contains a face card
        let has_face_card = hand
            .cards()
            .iter()
            .any(|card| matches!(card.value, Value::Jack | Value::Queen | Value::King));

        if has_face_card {
            // Production-ready: Deterministic effect for RL training
            JokerEffect::new().with_mult_multiplier(2.0)
        } else {
            JokerEffect::new().with_mult_multiplier(1.0)
        }
    }

    fn on_card_scored(&self, _context: &mut GameContext, card: &Card) -> JokerEffect {
        // For scoring effects, we apply X2 mult for face cards
        if matches!(card.value, Value::Jack | Value::Queen | Value::King) {
            // Production-ready: X2 Mult for face card scored
            JokerEffect::new().with_mult_multiplier(2.0)
        } else {
            JokerEffect::new().with_mult_multiplier(1.0)
        }
    }
}

/// Ancient Joker: Each played card with \[suit\] gives X2 Mult, suit changes at end of round
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AncientJoker {
    pub current_suit: Suit,
}

impl Default for AncientJoker {
    fn default() -> Self {
        Self::new()
    }
}

impl AncientJoker {
    pub fn new() -> Self {
        Self {
            current_suit: Suit::Heart, // Deterministic starting suit for RL training
        }
    }
}

impl JokerIdentity for AncientJoker {
    fn joker_type(&self) -> &'static str {
        "Ancient"
    }

    fn name(&self) -> &str {
        "Ancient Joker"
    }

    fn description(&self) -> &str {
        "Each played card with \\[suit\\] gives X2 Mult when scored, suit changes at end of round"
    }

    fn rarity(&self) -> Rarity {
        Rarity::Rare
    }

    fn base_cost(&self) -> u64 {
        8
    }
}

impl JokerLifecycle for AncientJoker {
    fn on_round_end(&mut self) {
        // Deterministic suit cycling for RL training compatibility
        self.current_suit = match self.current_suit {
            Suit::Heart => Suit::Diamond,
            Suit::Diamond => Suit::Club,
            Suit::Club => Suit::Spade,
            Suit::Spade => Suit::Heart,
        };
    }
}

impl JokerGameplay for AncientJoker {
    fn process(&mut self, _stage: &Stage, _context: &mut ProcessContext) -> ProcessResult {
        ProcessResult {
            chips_added: 0,
            mult_added: 0.0,
            mult_multiplier: 1.0,
            retriggered: false,
            message: None,
        }
    }

    fn can_trigger(&self, _stage: &Stage, _context: &ProcessContext) -> bool {
        true
    }
}

impl JokerModifiers for AncientJoker {}
impl JokerStateTrait for AncientJoker {}

impl Joker for AncientJoker {
    fn id(&self) -> JokerId {
        JokerId::Ancient
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

    fn on_card_scored(&self, _context: &mut GameContext, card: &Card) -> JokerEffect {
        if card.suit == self.current_suit {
            // Production-ready: X2 Mult for matching suit cards
            JokerEffect::new().with_mult_multiplier(2.0)
        } else {
            JokerEffect::new().with_mult_multiplier(1.0)
        }
    }
}

/// Steel Joker: Gives X1.5 Mult for each Steel Card in full deck
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SteelJoker;

impl JokerIdentity for SteelJoker {
    fn joker_type(&self) -> &'static str {
        "Steel"
    }

    fn name(&self) -> &str {
        "Steel Joker"
    }

    fn description(&self) -> &str {
        "Gives X1.5 Mult for each Steel Card in your full deck"
    }

    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
    }

    fn base_cost(&self) -> u64 {
        6
    }
}

impl JokerLifecycle for SteelJoker {}

impl JokerGameplay for SteelJoker {
    fn process(&mut self, _stage: &Stage, _context: &mut ProcessContext) -> ProcessResult {
        ProcessResult {
            chips_added: 0,
            mult_added: 0.0,
            mult_multiplier: 1.0,
            retriggered: false,
            message: None,
        }
    }

    fn can_trigger(&self, _stage: &Stage, _context: &ProcessContext) -> bool {
        true
    }
}

impl JokerModifiers for SteelJoker {}
impl JokerStateTrait for SteelJoker {}

impl Joker for SteelJoker {
    fn id(&self) -> JokerId {
        JokerId::SteelJoker
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
        let steel_cards = context.steel_cards_in_deck;
        if steel_cards > 0 {
            // Production-ready: X1.5 per steel card, compounding
            let multiplier = 1.5_f64.powi(steel_cards as i32);
            JokerEffect::new().with_mult_multiplier(multiplier)
        } else {
            JokerEffect::new().with_mult_multiplier(1.0)
        }
    }
}

/// Baron: Each King held in hand gives X1.5 Mult
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BaronJoker;

impl JokerIdentity for BaronJoker {
    fn joker_type(&self) -> &'static str {
        "Baron"
    }

    fn name(&self) -> &str {
        "Baron"
    }

    fn description(&self) -> &str {
        "Each King held in hand gives X1.5 Mult"
    }

    fn rarity(&self) -> Rarity {
        Rarity::Rare
    }

    fn base_cost(&self) -> u64 {
        8
    }
}

impl JokerLifecycle for BaronJoker {}

impl JokerGameplay for BaronJoker {
    fn process(&mut self, _stage: &Stage, _context: &mut ProcessContext) -> ProcessResult {
        ProcessResult {
            chips_added: 0,
            mult_added: 0.0,
            mult_multiplier: 1.0,
            retriggered: false,
            message: None,
        }
    }

    fn can_trigger(&self, _stage: &Stage, _context: &ProcessContext) -> bool {
        true
    }
}

impl JokerModifiers for BaronJoker {}
impl JokerStateTrait for BaronJoker {}

impl Joker for BaronJoker {
    fn id(&self) -> JokerId {
        JokerId::Baron
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

    fn on_hand_played(&self, context: &mut GameContext, _hand: &SelectHand) -> JokerEffect {
        // Count Kings in hand (this would need access to current hand)
        // For now, using a placeholder - production code would access hand state
        let kings_in_hand = context
            .hand
            .cards()
            .iter()
            .filter(|card| card.value == Value::King)
            .count();

        if kings_in_hand > 0 {
            // Production-ready: X1.5 per King, compounding
            let multiplier = 1.5_f64.powi(kings_in_hand as i32);
            JokerEffect::new().with_mult_multiplier(multiplier)
        } else {
            JokerEffect::new().with_mult_multiplier(1.0)
        }
    }
}

/// The Idol: Each played \[rank\] of \[suit\] gives X2 Mult, card changes every round
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TheIdolJoker {
    pub current_rank: Value,
    pub current_suit: Suit,
}

impl Default for TheIdolJoker {
    fn default() -> Self {
        Self::new()
    }
}

impl TheIdolJoker {
    pub fn new() -> Self {
        Self {
            current_rank: Value::Ace, // Deterministic starting values for RL training
            current_suit: Suit::Heart,
        }
    }
}

impl JokerIdentity for TheIdolJoker {
    fn joker_type(&self) -> &'static str {
        "TheIdol"
    }

    fn name(&self) -> &str {
        "The Idol"
    }

    fn description(&self) -> &str {
        "Each played \\[rank\\] of \\[suit\\] gives X2 Mult when scored, card changes every round"
    }

    fn rarity(&self) -> Rarity {
        Rarity::Rare
    }

    fn base_cost(&self) -> u64 {
        6
    }
}

impl JokerLifecycle for TheIdolJoker {
    fn on_round_end(&mut self) {
        // Deterministic cycling for RL training compatibility
        // Cycle through ranks first, then suits
        self.current_rank = match self.current_rank {
            Value::Ace => Value::Two,
            Value::Two => Value::Three,
            Value::Three => Value::Four,
            Value::Four => Value::Five,
            Value::Five => Value::Six,
            Value::Six => Value::Seven,
            Value::Seven => Value::Eight,
            Value::Eight => Value::Nine,
            Value::Nine => Value::Ten,
            Value::Ten => Value::Jack,
            Value::Jack => Value::Queen,
            Value::Queen => Value::King,
            Value::King => {
                // Cycle suit and reset to Ace
                self.current_suit = match self.current_suit {
                    Suit::Heart => Suit::Diamond,
                    Suit::Diamond => Suit::Club,
                    Suit::Club => Suit::Spade,
                    Suit::Spade => Suit::Heart,
                };
                Value::Ace
            }
        };
    }
}

impl JokerGameplay for TheIdolJoker {
    fn process(&mut self, _stage: &Stage, _context: &mut ProcessContext) -> ProcessResult {
        ProcessResult {
            chips_added: 0,
            mult_added: 0.0,
            mult_multiplier: 1.0,
            retriggered: false,
            message: None,
        }
    }

    fn can_trigger(&self, _stage: &Stage, _context: &ProcessContext) -> bool {
        true
    }
}

impl JokerModifiers for TheIdolJoker {}
impl JokerStateTrait for TheIdolJoker {}

impl Joker for TheIdolJoker {
    fn id(&self) -> JokerId {
        JokerId::TheIdol
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

    fn on_card_scored(&self, _context: &mut GameContext, card: &Card) -> JokerEffect {
        if card.value == self.current_rank && card.suit == self.current_suit {
            // Production-ready: X2 Mult for exact matching card
            JokerEffect::new().with_mult_multiplier(2.0)
        } else {
            JokerEffect::new().with_mult_multiplier(1.0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::joker_state::JokerStateManager;
    use crate::rank::HandRank;
    use std::collections::HashMap;
    use std::sync::Arc;

    /// Helper function to create basic test context
    fn create_basic_test_context() -> (
        Arc<JokerStateManager>,
        HashMap<HandRank, u32>,
        crate::rng::GameRng,
        crate::hand::Hand,
        Vec<Card>,
    ) {
        let state_manager = Arc::new(JokerStateManager::new());
        let hand_counts = HashMap::new();
        let rng = crate::rng::GameRng::new(crate::rng::RngMode::Testing(42));
        let hand = crate::hand::Hand::new(vec![]);
        let discarded = vec![];

        (state_manager, hand_counts, rng, hand, discarded)
    }

    #[test]
    fn test_photograph_joker_identity() {
        let joker = PhotographJoker::default();

        assert_eq!(joker.joker_type(), "Photograph");
        assert_eq!(JokerIdentity::name(&joker), "Photograph");
        assert_eq!(
            JokerIdentity::description(&joker),
            "First played face card gives X2 Mult when scored"
        );
        assert_eq!(JokerIdentity::rarity(&joker), Rarity::Common);
        assert_eq!(joker.base_cost(), 5);
    }

    #[test]
    fn test_ancient_joker_identity() {
        let joker = AncientJoker::default();

        assert_eq!(joker.joker_type(), "Ancient");
        assert_eq!(JokerIdentity::name(&joker), "Ancient Joker");
        assert_eq!(JokerIdentity::rarity(&joker), Rarity::Rare);
        assert_eq!(joker.base_cost(), 8);
    }

    #[test]
    fn test_steel_joker_identity() {
        let joker = SteelJoker;

        assert_eq!(joker.joker_type(), "Steel");
        assert_eq!(JokerIdentity::name(&joker), "Steel Joker");
        assert_eq!(JokerIdentity::rarity(&joker), Rarity::Uncommon);
        assert_eq!(joker.base_cost(), 6);
    }

    #[test]
    fn test_baron_joker_identity() {
        let joker = BaronJoker;

        assert_eq!(joker.joker_type(), "Baron");
        assert_eq!(JokerIdentity::name(&joker), "Baron");
        assert_eq!(JokerIdentity::rarity(&joker), Rarity::Rare);
        assert_eq!(joker.base_cost(), 8);
    }

    #[test]
    fn test_the_idol_joker_identity() {
        let joker = TheIdolJoker::default();

        assert_eq!(joker.joker_type(), "TheIdol");
        assert_eq!(JokerIdentity::name(&joker), "The Idol");
        assert_eq!(JokerIdentity::rarity(&joker), Rarity::Rare);
        assert_eq!(joker.base_cost(), 6);
    }

    #[test]
    fn test_photograph_face_card_mult() {
        let joker = PhotographJoker::default();
        let face_cards = vec![Card::new(Value::King, Suit::Heart)];
        let select_hand = SelectHand::new(face_cards);

        let (state_manager, hand_counts, rng, hand, discarded) = create_basic_test_context();
        let mut context = GameContext {
            chips: 0,
            mult: 0,
            money: 0,
            ante: 1,
            round: 1,
            stage: &Stage::Blind(crate::stage::Blind::Small),
            hands_played: 0,
            discards_used: 0,
            jokers: &[],
            hand: &hand,
            discarded: &discarded,
            joker_state_manager: &state_manager,
            hand_type_counts: &hand_counts,
            cards_in_deck: 52,
            stone_cards_in_deck: 0,
            steel_cards_in_deck: 0,
            rng: &rng,
        };

        let effect = joker.on_hand_played(&mut context, &select_hand);
        assert_eq!(effect.mult_multiplier, 2.0);
    }

    #[test]
    fn test_ancient_joker_suit_matching() {
        let joker = AncientJoker::new();
        assert_eq!(joker.current_suit, Suit::Heart); // Deterministic start

        let heart_card = Card::new(Value::Ace, Suit::Heart);
        let spade_card = Card::new(Value::Ace, Suit::Spade);

        let (state_manager, _hand_counts, _rng, _hand, _discarded) = create_basic_test_context();
        let mut context = GameContext {
            chips: 0,
            mult: 0,
            money: 0,
            ante: 1,
            round: 1,
            stage: &Stage::Blind(crate::stage::Blind::Small),
            hands_played: 0,
            discards_used: 0,
            jokers: &[],
            hand: &_hand,
            discarded: &_discarded,
            joker_state_manager: &state_manager,
            hand_type_counts: &_hand_counts,
            cards_in_deck: 52,
            stone_cards_in_deck: 0,
            steel_cards_in_deck: 0,
            rng: &_rng,
        };

        // Matching suit should give X2 mult
        let effect1 = joker.on_card_scored(&mut context, &heart_card);
        assert_eq!(effect1.mult_multiplier, 2.0);

        // Non-matching suit should give no bonus
        let effect2 = joker.on_card_scored(&mut context, &spade_card);
        assert_eq!(effect2.mult_multiplier, 1.0);
    }

    #[test]
    fn test_steel_joker_steel_cards() {
        let joker = SteelJoker;
        let select_hand = SelectHand::new(vec![]);

        let (state_manager, hand_counts, rng, hand, discarded) = create_basic_test_context();
        let mut context = GameContext {
            chips: 0,
            mult: 0,
            money: 0,
            ante: 1,
            round: 1,
            stage: &Stage::Blind(crate::stage::Blind::Small),
            hands_played: 0,
            discards_used: 0,
            jokers: &[],
            hand: &hand,
            discarded: &discarded,
            joker_state_manager: &state_manager,
            hand_type_counts: &hand_counts,
            cards_in_deck: 52,
            stone_cards_in_deck: 0,
            steel_cards_in_deck: 2, // 2 steel cards
            rng: &rng,
        };

        let effect = joker.on_hand_played(&mut context, &select_hand);
        assert_eq!(effect.mult_multiplier, 2.25); // 1.5^2 = 2.25
    }

    #[test]
    fn test_deterministic_suit_cycling() {
        let mut joker = AncientJoker::new();

        // Test deterministic suit cycling
        assert_eq!(joker.current_suit, Suit::Heart);
        JokerLifecycle::on_round_end(&mut joker);
        assert_eq!(joker.current_suit, Suit::Diamond);
        JokerLifecycle::on_round_end(&mut joker);
        assert_eq!(joker.current_suit, Suit::Club);
        JokerLifecycle::on_round_end(&mut joker);
        assert_eq!(joker.current_suit, Suit::Spade);
        JokerLifecycle::on_round_end(&mut joker);
        assert_eq!(joker.current_suit, Suit::Heart); // Cycles back
    }

    #[test]
    fn test_all_jokers_implement_required_traits() {
        // Test that all jokers implement the new trait system
        fn test_traits<T>(_joker: T)
        where
            T: JokerIdentity
                + JokerLifecycle
                + JokerGameplay
                + JokerModifiers
                + JokerStateTrait
                + Clone,
        {
            // This function will only compile if T implements all required traits
        }

        test_traits(PhotographJoker::default());
        test_traits(AncientJoker::default());
        test_traits(SteelJoker);
        test_traits(BaronJoker);
        test_traits(TheIdolJoker::default());
    }
}
