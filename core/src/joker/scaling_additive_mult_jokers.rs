//! Scaling Additive Mult Jokers implementation
//!
//! This module implements jokers that provide additive mult bonuses that scale over time.
//! These jokers accumulate mult based on various game triggers.

use crate::{
    card::{Card, Value},
    hand::SelectHand,
    joker::{
        traits::{JokerState as JokerStateTrait, ProcessContext, ProcessResult, Rarity},
        GameContext, Joker, JokerEffect, JokerGameplay, JokerId, JokerIdentity, JokerLifecycle,
        JokerRarity,
    },
    rank::HandRank,
    stage::Stage,
};
use serde_json::json;

/// Spare Trousers - +2 Mult per Two Pair hand played
#[derive(Debug, Clone)]
pub struct SpareTrousersJoker {
    id: JokerId,
    name: String,
    description: String,
    rarity: JokerRarity,
    cost: usize,
    two_pairs_played: u32,
}

impl Default for SpareTrousersJoker {
    fn default() -> Self {
        Self::new()
    }
}

impl SpareTrousersJoker {
    pub fn new() -> Self {
        Self {
            id: JokerId::Trousers,
            name: "Spare Trousers".to_string(),
            description: "+2 Mult per Two Pair hand played".to_string(),
            rarity: JokerRarity::Uncommon,
            cost: 6,
            two_pairs_played: 0,
        }
    }
}

impl JokerIdentity for SpareTrousersJoker {
    fn joker_type(&self) -> &'static str {
        "spare_trousers"
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

impl Joker for SpareTrousersJoker {
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

    fn on_hand_played(&self, context: &mut GameContext, hand: &SelectHand) -> JokerEffect {
        // Check if the hand is a Two Pair
        let is_two_pair = hand
            .best_hand()
            .map(|made_hand| made_hand.rank == HandRank::TwoPair)
            .unwrap_or(false);

        if is_two_pair {
            // Update state to increment two pairs count
            context
                .joker_state_manager
                .update_state(self.id(), |state| {
                    state.accumulated_value += 1.0;
                });

            let current_count = context
                .joker_state_manager
                .get_state(self.id())
                .map(|state| state.accumulated_value as u32)
                .unwrap_or(0);

            JokerEffect::new()
                .with_mult((current_count * 2) as i32)
                .with_message(format!(
                    "Spare Trousers: {} Two Pairs played, +{} Mult",
                    current_count,
                    current_count * 2
                ))
        } else {
            // Still provide the accumulated mult even if not Two Pair
            let current_count = context
                .joker_state_manager
                .get_state(self.id())
                .map(|state| state.accumulated_value as u32)
                .unwrap_or(0);

            if current_count > 0 {
                JokerEffect::new().with_mult((current_count * 2) as i32)
            } else {
                JokerEffect::new()
            }
        }
    }
}

impl JokerStateTrait for SpareTrousersJoker {
    fn has_state(&self) -> bool {
        true
    }

    fn serialize_state(&self) -> Option<serde_json::Value> {
        Some(json!({
            "two_pairs_played": self.two_pairs_played
        }))
    }

    fn deserialize_state(&mut self, value: serde_json::Value) -> Result<(), String> {
        if let Some(count) = value.get("two_pairs_played").and_then(|v| v.as_u64()) {
            self.two_pairs_played = count as u32;
            Ok(())
        } else {
            Err("Invalid state format".to_string())
        }
    }
}

impl JokerGameplay for SpareTrousersJoker {
    fn process(&mut self, stage: &Stage, _context: &mut ProcessContext) -> ProcessResult {
        if matches!(stage, Stage::Blind(_)) {
            ProcessResult {
                mult_added: (self.two_pairs_played * 2) as f64,
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

/// Fortune Teller - +1 Mult per Tarot card used
#[derive(Debug, Clone)]
pub struct FortuneTellerJoker {
    id: JokerId,
    name: String,
    description: String,
    rarity: JokerRarity,
    cost: usize,
    tarots_used: u32,
}

impl Default for FortuneTellerJoker {
    fn default() -> Self {
        Self::new()
    }
}

impl FortuneTellerJoker {
    pub fn new() -> Self {
        Self {
            id: JokerId::Reserved10, // FortuneTeller
            name: "Fortune Teller".to_string(),
            description: "+1 Mult per Tarot card used".to_string(),
            rarity: JokerRarity::Common,
            cost: 3,
            tarots_used: 0,
        }
    }
}

impl JokerIdentity for FortuneTellerJoker {
    fn joker_type(&self) -> &'static str {
        "fortune_teller"
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

impl Joker for FortuneTellerJoker {
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
        // Get current tarot count from state
        let tarot_count = context
            .joker_state_manager
            .get_state(self.id())
            .map(|state| state.accumulated_value as u32)
            .unwrap_or(0);

        if tarot_count > 0 {
            JokerEffect::new()
                .with_mult(tarot_count as i32)
                .with_message(format!("Fortune Teller: +{} Mult", tarot_count))
        } else {
            JokerEffect::new()
        }
    }

    // TODO: Add on_consumable_used when implemented in trait system
}

impl JokerStateTrait for FortuneTellerJoker {
    fn has_state(&self) -> bool {
        true
    }

    fn serialize_state(&self) -> Option<serde_json::Value> {
        Some(json!({
            "tarots_used": self.tarots_used
        }))
    }

    fn deserialize_state(&mut self, value: serde_json::Value) -> Result<(), String> {
        if let Some(count) = value.get("tarots_used").and_then(|v| v.as_u64()) {
            self.tarots_used = count as u32;
            Ok(())
        } else {
            Err("Invalid state format".to_string())
        }
    }
}

impl JokerGameplay for FortuneTellerJoker {
    fn process(&mut self, stage: &Stage, _context: &mut ProcessContext) -> ProcessResult {
        if matches!(stage, Stage::Blind(_)) {
            ProcessResult {
                mult_added: self.tarots_used as f64,
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

/// Green Joker - +1 Mult per hand played, -1 per discard
#[derive(Debug, Clone)]
pub struct GreenJoker {
    id: JokerId,
    name: String,
    description: String,
    rarity: JokerRarity,
    cost: usize,
    current_mult: i32,
}

impl Default for GreenJoker {
    fn default() -> Self {
        Self::new()
    }
}

impl GreenJoker {
    pub fn new() -> Self {
        Self {
            id: JokerId::GreenJoker,
            name: "Green Joker".to_string(),
            description: "+1 Mult per hand played, -1 per discard".to_string(),
            rarity: JokerRarity::Common,
            cost: 3,
            current_mult: 0,
        }
    }
}

impl JokerIdentity for GreenJoker {
    fn joker_type(&self) -> &'static str {
        "green_joker"
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

impl Joker for GreenJoker {
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
        // Increment mult by 1
        context
            .joker_state_manager
            .update_state(self.id(), |state| {
                state.accumulated_value += 1.0;
            });

        let current_mult = context
            .joker_state_manager
            .get_state(self.id())
            .map(|state| state.accumulated_value as i32)
            .unwrap_or(0);

        if current_mult > 0 {
            JokerEffect::new()
                .with_mult(current_mult)
                .with_message(format!("Green Joker: +{} Mult", current_mult))
        } else {
            JokerEffect::new()
        }
    }

    fn on_discard(&self, context: &mut GameContext, _cards: &[Card]) -> JokerEffect {
        // Decrement mult by 1, but don't go below 0
        context
            .joker_state_manager
            .update_state(self.id(), |state| {
                state.accumulated_value = (state.accumulated_value - 1.0).max(0.0);
            });

        JokerEffect::new()
    }
}

impl JokerStateTrait for GreenJoker {
    fn has_state(&self) -> bool {
        true
    }

    fn serialize_state(&self) -> Option<serde_json::Value> {
        Some(json!({
            "current_mult": self.current_mult
        }))
    }

    fn deserialize_state(&mut self, value: serde_json::Value) -> Result<(), String> {
        if let Some(mult) = value.get("current_mult").and_then(|v| v.as_i64()) {
            self.current_mult = mult as i32;
            Ok(())
        } else {
            Err("Invalid state format".to_string())
        }
    }
}

impl JokerGameplay for GreenJoker {
    fn process(&mut self, stage: &Stage, _context: &mut ProcessContext) -> ProcessResult {
        if matches!(stage, Stage::Blind(_)) && self.current_mult > 0 {
            ProcessResult {
                mult_added: self.current_mult as f64,
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

/// Ride the Bus - +1 Mult per hand played this round (no face cards)
#[derive(Debug, Clone)]
pub struct RideTheBusJoker {
    id: JokerId,
    name: String,
    description: String,
    rarity: JokerRarity,
    cost: usize,
    hands_this_round: u32,
}

impl Default for RideTheBusJoker {
    fn default() -> Self {
        Self::new()
    }
}

impl RideTheBusJoker {
    pub fn new() -> Self {
        Self {
            id: JokerId::Reserved5, // RideTheBus
            name: "Ride the Bus".to_string(),
            description: "+1 Mult per hand played this round (no face cards)".to_string(),
            rarity: JokerRarity::Common,
            cost: 3,
            hands_this_round: 0,
        }
    }

    fn has_face_cards(hand: &SelectHand) -> bool {
        hand.cards()
            .into_iter()
            .any(|card| matches!(card.value, Value::Jack | Value::Queen | Value::King))
    }
}

impl JokerIdentity for RideTheBusJoker {
    fn joker_type(&self) -> &'static str {
        "ride_the_bus"
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

impl JokerLifecycle for RideTheBusJoker {
    fn on_round_end(&mut self) {
        // Reset counter at the end of each round
        self.hands_this_round = 0;
    }
}

impl Joker for RideTheBusJoker {
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

    fn on_hand_played(&self, context: &mut GameContext, hand: &SelectHand) -> JokerEffect {
        if Self::has_face_cards(hand) {
            // Reset the counter if face cards are played
            context
                .joker_state_manager
                .update_state(self.id(), |state| {
                    state.accumulated_value = 0.0;
                });
            JokerEffect::new().with_message("Ride the Bus: Reset! (Face card played)".to_string())
        } else {
            // Increment the counter
            context
                .joker_state_manager
                .update_state(self.id(), |state| {
                    state.accumulated_value += 1.0;
                });

            let current_count = context
                .joker_state_manager
                .get_state(self.id())
                .map(|state| state.accumulated_value as u32)
                .unwrap_or(0);

            JokerEffect::new()
                .with_mult(current_count as i32)
                .with_message(format!("Ride the Bus: +{} Mult", current_count))
        }
    }

    fn on_round_end(&self, context: &mut GameContext) -> JokerEffect {
        // Reset state at round end
        context
            .joker_state_manager
            .update_state(self.id(), |state| {
                state.accumulated_value = 0.0;
            });
        JokerEffect::new()
    }
}

impl JokerStateTrait for RideTheBusJoker {
    fn has_state(&self) -> bool {
        true
    }

    fn serialize_state(&self) -> Option<serde_json::Value> {
        Some(json!({
            "hands_this_round": self.hands_this_round
        }))
    }

    fn deserialize_state(&mut self, value: serde_json::Value) -> Result<(), String> {
        if let Some(hands) = value.get("hands_this_round").and_then(|v| v.as_u64()) {
            self.hands_this_round = hands as u32;
            Ok(())
        } else {
            Err("Invalid state format".to_string())
        }
    }
}

impl JokerGameplay for RideTheBusJoker {
    fn process(&mut self, stage: &Stage, context: &mut ProcessContext) -> ProcessResult {
        if matches!(stage, Stage::Blind(_)) {
            // Check if current hand has face cards
            let has_face = context
                .played_cards
                .iter()
                .any(|card| matches!(card.value, Value::Jack | Value::Queen | Value::King));

            if has_face {
                // Reset counter
                self.hands_this_round = 0;
                ProcessResult::default()
            } else {
                // Increment and apply mult
                self.hands_this_round += 1;
                ProcessResult {
                    mult_added: self.hands_this_round as f64,
                    ..Default::default()
                }
            }
        } else {
            ProcessResult::default()
        }
    }

    fn can_trigger(&self, stage: &Stage, _context: &ProcessContext) -> bool {
        matches!(stage, Stage::Blind(_))
    }
}

/// Red Card - +3 Mult per pack skipped
#[derive(Debug, Clone)]
pub struct RedCardJoker {
    id: JokerId,
    name: String,
    description: String,
    rarity: JokerRarity,
    cost: usize,
    packs_skipped: u32,
}

impl Default for RedCardJoker {
    fn default() -> Self {
        Self::new()
    }
}

impl RedCardJoker {
    pub fn new() -> Self {
        Self {
            id: JokerId::Reserved6, // RedCard (pack skipping version)
            name: "Red Card".to_string(),
            description: "+3 Mult per pack skipped".to_string(),
            rarity: JokerRarity::Common,
            cost: 4,
            packs_skipped: 0,
        }
    }
}

impl JokerIdentity for RedCardJoker {
    fn joker_type(&self) -> &'static str {
        "red_card"
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

impl Joker for RedCardJoker {
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
        // Get current pack skip count from state
        let packs_skipped = context
            .joker_state_manager
            .get_state(self.id())
            .map(|state| state.accumulated_value as u32)
            .unwrap_or(0);

        if packs_skipped > 0 {
            JokerEffect::new()
                .with_mult((packs_skipped * 3) as i32)
                .with_message(format!("Red Card: +{} Mult", packs_skipped * 3))
        } else {
            JokerEffect::new()
        }
    }

    // TODO: Add on_pack_skipped when implemented in trait system
}

impl JokerStateTrait for RedCardJoker {
    fn has_state(&self) -> bool {
        true
    }

    fn serialize_state(&self) -> Option<serde_json::Value> {
        Some(json!({
            "packs_skipped": self.packs_skipped
        }))
    }

    fn deserialize_state(&mut self, value: serde_json::Value) -> Result<(), String> {
        if let Some(packs) = value.get("packs_skipped").and_then(|v| v.as_u64()) {
            self.packs_skipped = packs as u32;
            Ok(())
        } else {
            Err("Invalid state format".to_string())
        }
    }
}

impl JokerGameplay for RedCardJoker {
    fn process(&mut self, stage: &Stage, _context: &mut ProcessContext) -> ProcessResult {
        if matches!(stage, Stage::Blind(_)) && self.packs_skipped > 0 {
            ProcessResult {
                mult_added: (self.packs_skipped * 3) as f64,
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

/// Factory functions for creating scaling additive mult jokers
pub fn create_spare_trousers_joker() -> Box<dyn Joker> {
    Box::new(SpareTrousersJoker::new())
}

pub fn create_fortune_teller_joker() -> Box<dyn Joker> {
    Box::new(FortuneTellerJoker::new())
}

pub fn create_green_joker() -> Box<dyn Joker> {
    Box::new(GreenJoker::new())
}

pub fn create_ride_the_bus_joker() -> Box<dyn Joker> {
    Box::new(RideTheBusJoker::new())
}

pub fn create_red_card_joker() -> Box<dyn Joker> {
    Box::new(RedCardJoker::new())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::{Card, Suit, Value};
    use crate::hand::SelectHand;
    use crate::joker::test_utils::TestContextBuilder;

    #[test]
    fn test_spare_trousers() {
        let joker = SpareTrousersJoker::new();

        // Test identity
        assert_eq!(joker.joker_type(), "spare_trousers");
        assert_eq!(JokerIdentity::name(&joker), "Spare Trousers");
        assert_eq!(joker.base_cost(), 6);
        assert_eq!(joker.rarity, JokerRarity::Uncommon);
    }

    #[test]
    fn test_spare_trousers_two_pair_accumulation() {
        let joker = SpareTrousersJoker::new();
        let mut test_context = TestContextBuilder::new().build();

        // Create a two pair hand (2s and 3s)
        let two_pair_hand = SelectHand::new(vec![
            Card::new(Value::Two, Suit::Heart),
            Card::new(Value::Two, Suit::Diamond),
            Card::new(Value::Three, Suit::Spade),
            Card::new(Value::Three, Suit::Club),
            Card::new(Value::King, Suit::Heart),
        ]);

        // First two pair should give +2 mult
        let effect1 = joker.on_hand_played(&mut test_context, &two_pair_hand);
        assert_eq!(effect1.mult, 2);

        // Second two pair should give +4 mult (2 two pairs * 2)
        let effect2 = joker.on_hand_played(&mut test_context, &two_pair_hand);
        assert_eq!(effect2.mult, 4);

        // Non-two pair hand should still give accumulated mult
        let regular_hand = SelectHand::new(vec![
            Card::new(Value::Ace, Suit::Heart),
            Card::new(Value::King, Suit::Diamond),
            Card::new(Value::Queen, Suit::Spade),
            Card::new(Value::Jack, Suit::Club),
            Card::new(Value::Ten, Suit::Heart),
        ]);

        let effect3 = joker.on_hand_played(&mut test_context, &regular_hand);
        assert_eq!(effect3.mult, 4); // Still 4 mult from 2 two pairs
    }

    #[test]
    fn test_green_joker() {
        let joker = GreenJoker::new();

        // Test identity
        assert_eq!(joker.joker_type(), "green_joker");
        assert_eq!(JokerIdentity::name(&joker), "Green Joker");
        assert_eq!(joker.base_cost(), 3);
        assert_eq!(joker.rarity, JokerRarity::Common);
    }

    #[test]
    fn test_green_joker_increment_decrement() {
        let joker = GreenJoker::new();
        let mut test_context = TestContextBuilder::new().build();

        let hand = SelectHand::new(vec![]);

        // First hand played gives +1 mult
        let effect1 = joker.on_hand_played(&mut test_context, &hand);
        assert_eq!(effect1.mult, 1);

        // Second hand played gives +2 mult
        let effect2 = joker.on_hand_played(&mut test_context, &hand);
        assert_eq!(effect2.mult, 2);

        // Discard decrements by 1
        let cards = vec![Card::new(Value::Two, Suit::Heart)];
        joker.on_discard(&mut test_context, &cards);

        // Next hand should give +2 mult (was 2, minus 1 from discard = 1, then increment on play = 2)
        let effect3 = joker.on_hand_played(&mut test_context, &hand);
        assert_eq!(effect3.mult, 2);

        // Multiple discards shouldn't go below 0
        joker.on_discard(&mut test_context, &cards);
        joker.on_discard(&mut test_context, &cards);

        let effect4 = joker.on_hand_played(&mut test_context, &hand);
        assert_eq!(effect4.mult, 1); // Was 0, increment on play = 1
    }

    #[test]
    fn test_ride_the_bus() {
        let joker = RideTheBusJoker::new();

        // Test identity
        assert_eq!(joker.joker_type(), "ride_the_bus");
        assert_eq!(JokerIdentity::name(&joker), "Ride the Bus");
        assert_eq!(joker.base_cost(), 3);
        assert_eq!(joker.rarity, JokerRarity::Common);
    }

    #[test]
    fn test_ride_the_bus_no_face_cards() {
        let joker = RideTheBusJoker::new();
        let mut test_context = TestContextBuilder::new().build();

        // Hand without face cards
        let no_face_hand = SelectHand::new(vec![
            Card::new(Value::Two, Suit::Heart),
            Card::new(Value::Three, Suit::Diamond),
            Card::new(Value::Four, Suit::Spade),
            Card::new(Value::Five, Suit::Club),
            Card::new(Value::Six, Suit::Heart),
        ]);

        // First hand gives +1 mult
        let effect1 = joker.on_hand_played(&mut test_context, &no_face_hand);
        assert_eq!(effect1.mult, 1);

        // Second hand gives +2 mult
        let effect2 = joker.on_hand_played(&mut test_context, &no_face_hand);
        assert_eq!(effect2.mult, 2);
    }

    #[test]
    fn test_ride_the_bus_reset_on_face_card() {
        let joker = RideTheBusJoker::new();
        let mut test_context = TestContextBuilder::new().build();

        // Build up some mult
        let no_face_hand = SelectHand::new(vec![
            Card::new(Value::Two, Suit::Heart),
            Card::new(Value::Three, Suit::Diamond),
        ]);

        joker.on_hand_played(&mut test_context, &no_face_hand);
        joker.on_hand_played(&mut test_context, &no_face_hand);

        // Hand with face card should reset
        let face_hand = SelectHand::new(vec![
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::Queen, Suit::Diamond),
        ]);

        let effect = joker.on_hand_played(&mut test_context, &face_hand);
        assert_eq!(effect.mult, 0);
        assert!(effect.message.unwrap().contains("Reset!"));

        // Next hand should start from 1 again
        let effect_after = joker.on_hand_played(&mut test_context, &no_face_hand);
        assert_eq!(effect_after.mult, 1);
    }

    #[test]
    fn test_fortune_teller() {
        let joker = FortuneTellerJoker::new();

        // Test identity
        assert_eq!(joker.joker_type(), "fortune_teller");
        assert_eq!(JokerIdentity::name(&joker), "Fortune Teller");
        assert_eq!(joker.base_cost(), 3);
        assert_eq!(joker.rarity, JokerRarity::Common);
    }

    #[test]
    fn test_red_card() {
        let joker = RedCardJoker::new();

        // Test identity
        assert_eq!(joker.joker_type(), "red_card");
        assert_eq!(JokerIdentity::name(&joker), "Red Card");
        assert_eq!(joker.base_cost(), 4);
        assert_eq!(joker.rarity, JokerRarity::Common);
    }

    #[test]
    fn test_red_card_pack_skipping() {
        let joker = RedCardJoker::new();
        let mut test_context = TestContextBuilder::new().build();

        let hand = SelectHand::new(vec![]);

        // Initially no mult
        let effect1 = joker.on_hand_played(&mut test_context, &hand);
        assert_eq!(effect1.mult, 0);

        // Skip a pack - manually update state since on_pack_skipped is not implemented yet
        test_context
            .joker_state_manager
            .update_state(joker.id(), |state| {
                state.accumulated_value += 1.0;
            });

        // Should now give +3 mult
        let effect2 = joker.on_hand_played(&mut test_context, &hand);
        assert_eq!(effect2.mult, 3);

        // Skip another pack - manually update state since on_pack_skipped is not implemented yet
        test_context
            .joker_state_manager
            .update_state(joker.id(), |state| {
                state.accumulated_value += 1.0;
            });

        // Should now give +6 mult
        let effect3 = joker.on_hand_played(&mut test_context, &hand);
        assert_eq!(effect3.mult, 6);
    }
}
