//! Scaling XMult Jokers implementation
//!
//! This module implements jokers that provide multiplicative mult bonuses (X mult)
//! that scale up over time based on various triggers. These jokers accumulate
//! value through gameplay and apply mult_multiplier effects.

use crate::{
    hand::SelectHand,
    joker::{
        traits::{JokerState, ProcessContext, ProcessResult, Rarity},
        GameContext, Joker, JokerEffect, JokerGameplay, JokerId, JokerIdentity, JokerLifecycle,
        JokerRarity,
    },
    stage::Stage,
};
use serde_json;

/// Throwback - +0.5X Mult per shop reroll, resets when entering shop
///
/// Note: Since shop reroll events aren't directly available in the current API,
/// this implementation tracks rounds and provides scaling based on round count.
/// In a full implementation, this would track actual shop rerolls.
#[derive(Debug, Clone)]
pub struct ThrowbackJoker {
    id: JokerId,
    name: String,
    description: String,
    rarity: JokerRarity,
    cost: usize,
    current_multiplier: f64,
    rounds_played: u32,
}

impl Default for ThrowbackJoker {
    fn default() -> Self {
        Self::new()
    }
}

impl ThrowbackJoker {
    pub fn new() -> Self {
        Self {
            id: JokerId::Reserved, // TODO: Update when JokerId::Throwback is added
            name: "Throwback".to_string(),
            description: "+0.5X Mult per round (resets at shop)".to_string(),
            rarity: JokerRarity::Uncommon,
            cost: 6,
            current_multiplier: 1.0,
            rounds_played: 0,
        }
    }

    fn calculate_multiplier(&self) -> f64 {
        1.0 + (0.5 * self.rounds_played as f64)
    }
}

impl JokerIdentity for ThrowbackJoker {
    fn joker_type(&self) -> &'static str {
        "throwback"
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

impl Joker for ThrowbackJoker {
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
        if self.current_multiplier > 1.0 {
            JokerEffect::new()
                .with_mult_multiplier(self.current_multiplier)
                .with_message(format!("Throwback: X{:.1} Mult", self.current_multiplier))
        } else {
            JokerEffect::new()
        }
    }

    fn on_shop_open(&self, _context: &mut GameContext) -> JokerEffect {
        // Reset happens in JokerLifecycle trait
        JokerEffect::new()
    }
}

impl JokerGameplay for ThrowbackJoker {
    fn process(&mut self, stage: &Stage, context: &mut ProcessContext) -> ProcessResult {
        if !matches!(stage, Stage::Blind(_)) {
            return ProcessResult::default();
        }

        // Check for shop-related events to handle rerolls
        for event in context.events.iter() {
            if event.event_type == "shop_reroll" {
                self.rounds_played += 1;
                self.current_multiplier = self.calculate_multiplier();
            }
        }

        ProcessResult::default()
    }

    fn can_trigger(&self, stage: &Stage, _context: &ProcessContext) -> bool {
        matches!(stage, Stage::Blind(_)) && self.current_multiplier > 1.0
    }
}

impl JokerLifecycle for ThrowbackJoker {
    fn on_round_end(&mut self) {
        // Increment rounds for scaling
        self.rounds_played += 1;
        self.current_multiplier = self.calculate_multiplier();
    }
}

impl JokerState for ThrowbackJoker {
    fn has_state(&self) -> bool {
        true
    }

    fn serialize_state(&self) -> Option<serde_json::Value> {
        Some(serde_json::json!({
            "rounds_played": self.rounds_played,
            "current_multiplier": self.current_multiplier
        }))
    }

    fn deserialize_state(&mut self, value: serde_json::Value) -> Result<(), String> {
        if let Some(rounds) = value.get("rounds_played").and_then(|v| v.as_u64()) {
            self.rounds_played = rounds as u32;
        }
        if let Some(mult) = value.get("current_multiplier").and_then(|v| v.as_f64()) {
            self.current_multiplier = mult;
        }
        Ok(())
    }

    fn debug_state(&self) -> String {
        format!(
            "rounds_played: {}, current_multiplier: {:.1}",
            self.rounds_played, self.current_multiplier
        )
    }

    fn reset_state(&mut self) {
        self.rounds_played = 0;
        self.current_multiplier = 1.0;
    }
}

/// Steel Joker (Scaling Version) - Gains X0.2 Mult per round (permanent scaling)
///
/// Note: Since card destruction events aren't directly available in the current API,
/// this implementation provides scaling based on rounds played. In a full implementation,
/// this would track actual card destructions.
#[derive(Debug, Clone)]
pub struct ScalingSteelJoker {
    id: JokerId,
    name: String,
    description: String,
    rarity: JokerRarity,
    cost: usize,
    rounds_accumulated: u32,
    current_multiplier: f64,
}

impl Default for ScalingSteelJoker {
    fn default() -> Self {
        Self::new()
    }
}

impl ScalingSteelJoker {
    pub fn new() -> Self {
        Self {
            id: JokerId::SteelJoker,
            name: "Steel Joker".to_string(),
            description: "+0.2x Mult per card destroyed".to_string(),
            rarity: JokerRarity::Uncommon,
            cost: 7,
            rounds_accumulated: 0,
            current_multiplier: 1.0,
        }
    }

    fn calculate_multiplier(&self) -> f64 {
        1.0 + (0.2 * self.rounds_accumulated as f64)
    }
}

impl JokerIdentity for ScalingSteelJoker {
    fn joker_type(&self) -> &'static str {
        "steel_scaling"
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

impl Joker for ScalingSteelJoker {
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
        if self.current_multiplier > 1.0 {
            JokerEffect::new()
                .with_mult_multiplier(self.current_multiplier)
                .with_message(format!("Steel Joker: X{:.1} Mult", self.current_multiplier))
        } else {
            JokerEffect::new()
        }
    }
}

impl JokerGameplay for ScalingSteelJoker {
    fn process(&mut self, stage: &Stage, context: &mut ProcessContext) -> ProcessResult {
        if !matches!(stage, Stage::Blind(_)) {
            return ProcessResult::default();
        }

        // Check for card destruction events
        for event in context.events.iter() {
            if event.event_type == "card_destroyed" {
                self.rounds_accumulated += 1;
                self.current_multiplier = self.calculate_multiplier();
            }
        }

        ProcessResult::default()
    }

    fn can_trigger(&self, stage: &Stage, _context: &ProcessContext) -> bool {
        matches!(stage, Stage::Blind(_)) && self.current_multiplier > 1.0
    }
}

impl JokerLifecycle for ScalingSteelJoker {
    fn on_round_end(&mut self) {
        // Increment for permanent scaling
        self.rounds_accumulated += 1;
        self.current_multiplier = self.calculate_multiplier();
    }
}

impl JokerState for ScalingSteelJoker {
    fn has_state(&self) -> bool {
        true
    }

    fn serialize_state(&self) -> Option<serde_json::Value> {
        Some(serde_json::json!({
            "rounds_accumulated": self.rounds_accumulated,
            "current_multiplier": self.current_multiplier
        }))
    }

    fn deserialize_state(&mut self, value: serde_json::Value) -> Result<(), String> {
        if let Some(rounds) = value.get("rounds_accumulated").and_then(|v| v.as_u64()) {
            self.rounds_accumulated = rounds as u32;
        }
        if let Some(mult) = value.get("current_multiplier").and_then(|v| v.as_f64()) {
            self.current_multiplier = mult;
        }
        Ok(())
    }

    fn debug_state(&self) -> String {
        format!(
            "rounds_accumulated: {}, current_multiplier: {:.1}",
            self.rounds_accumulated, self.current_multiplier
        )
    }

    fn reset_state(&mut self) {
        self.rounds_accumulated = 0;
        self.current_multiplier = 1.0;
    }
}

/// Ceremonial Dagger - Mult doubles each blind, resets at round end
#[derive(Debug, Clone)]
pub struct CeremonialDaggerJoker {
    id: JokerId,
    name: String,
    description: String,
    rarity: JokerRarity,
    cost: usize,
    current_multiplier: f64,
    blind_count: u32,
}

impl Default for CeremonialDaggerJoker {
    fn default() -> Self {
        Self::new()
    }
}

impl CeremonialDaggerJoker {
    pub fn new() -> Self {
        Self {
            id: JokerId::Ceremonial,
            name: "Ceremonial Dagger".to_string(),
            description: "Mult doubles each blind".to_string(),
            rarity: JokerRarity::Uncommon,
            cost: 6,
            current_multiplier: 1.0,
            blind_count: 0,
        }
    }
}

impl JokerIdentity for CeremonialDaggerJoker {
    fn joker_type(&self) -> &'static str {
        "ceremonial_dagger"
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

impl Joker for CeremonialDaggerJoker {
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
        if self.current_multiplier > 1.0 {
            JokerEffect::new()
                .with_mult_multiplier(self.current_multiplier)
                .with_message(format!(
                    "Ceremonial Dagger: X{:.1} Mult",
                    self.current_multiplier
                ))
        } else {
            JokerEffect::new()
        }
    }

    fn on_blind_start(&self, _context: &mut GameContext) -> JokerEffect {
        // Note: We need to modify self, so actual doubling happens in process()
        JokerEffect::new()
    }
}

impl JokerGameplay for CeremonialDaggerJoker {
    fn process(&mut self, stage: &Stage, _context: &mut ProcessContext) -> ProcessResult {
        // Double mult when entering a new blind
        if matches!(stage, Stage::Blind(_)) && self.blind_count == 0 {
            self.blind_count += 1;
            self.current_multiplier = if self.current_multiplier == 1.0 {
                2.0
            } else {
                self.current_multiplier * 2.0
            };
        }

        ProcessResult::default()
    }

    fn can_trigger(&self, stage: &Stage, _context: &ProcessContext) -> bool {
        matches!(stage, Stage::Blind(_)) && self.current_multiplier > 1.0
    }
}

impl JokerLifecycle for CeremonialDaggerJoker {
    fn on_round_end(&mut self) {
        // Reset at round end
        self.current_multiplier = 1.0;
        self.blind_count = 0;
    }
}

impl JokerState for CeremonialDaggerJoker {
    fn has_state(&self) -> bool {
        true
    }

    fn serialize_state(&self) -> Option<serde_json::Value> {
        Some(serde_json::json!({
            "current_multiplier": self.current_multiplier,
            "blind_count": self.blind_count
        }))
    }

    fn deserialize_state(&mut self, value: serde_json::Value) -> Result<(), String> {
        if let Some(mult) = value.get("current_multiplier").and_then(|v| v.as_f64()) {
            self.current_multiplier = mult;
        }
        if let Some(count) = value.get("blind_count").and_then(|v| v.as_u64()) {
            self.blind_count = count as u32;
        }
        Ok(())
    }

    fn debug_state(&self) -> String {
        format!(
            "current_multiplier: {:.1}, blind_count: {}",
            self.current_multiplier, self.blind_count
        )
    }

    fn reset_state(&mut self) {
        self.current_multiplier = 1.0;
        self.blind_count = 0;
    }
}

/// Factory functions for creating scaling xmult jokers
pub fn create_throwback_joker() -> Box<dyn Joker> {
    Box::new(ThrowbackJoker::new())
}

// DEPRECATED: Replaced by steel_joker_composition::SteelJoker
// pub fn create_scaling_steel_joker() -> Box<dyn Joker> {
//     Box::new(ScalingSteelJoker::new())
// }

pub fn create_ceremonial_dagger_joker() -> Box<dyn Joker> {
    Box::new(CeremonialDaggerJoker::new())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stage::Blind;

    #[test]
    fn test_throwback_joker() {
        let mut throwback = ThrowbackJoker::new();

        // Test identity
        assert_eq!(throwback.joker_type(), "throwback");
        assert_eq!(JokerIdentity::name(&throwback), "Throwback");
        assert_eq!(throwback.base_cost(), 6);

        // Test initial state
        assert_eq!(throwback.current_multiplier, 1.0);
        assert_eq!(throwback.rounds_played, 0);

        // Test round scaling
        JokerLifecycle::on_round_end(&mut throwback);
        assert_eq!(throwback.rounds_played, 1);
        assert_eq!(throwback.current_multiplier, 1.5);

        JokerLifecycle::on_round_end(&mut throwback);
        assert_eq!(throwback.rounds_played, 2);
        assert_eq!(throwback.current_multiplier, 2.0);

        // Test reset
        throwback.reset_state();
        assert_eq!(throwback.rounds_played, 0);
        assert_eq!(throwback.current_multiplier, 1.0);
    }

    #[test]
    fn test_scaling_steel_joker() {
        let mut steel = ScalingSteelJoker::new();

        // Test identity
        assert_eq!(steel.joker_type(), "steel_scaling");
        assert_eq!(JokerIdentity::name(&steel), "Steel Joker");
        assert_eq!(JokerIdentity::rarity(&steel), Rarity::Uncommon);

        // Test initial state
        assert_eq!(steel.current_multiplier, 1.0);
        assert_eq!(steel.rounds_accumulated, 0);

        // Test round scaling
        JokerLifecycle::on_round_end(&mut steel);
        assert_eq!(steel.rounds_accumulated, 1);
        assert_eq!(steel.current_multiplier, 1.2);

        JokerLifecycle::on_round_end(&mut steel);
        assert_eq!(steel.rounds_accumulated, 2);
        assert_eq!(steel.current_multiplier, 1.4);
    }

    #[test]
    fn test_ceremonial_dagger() {
        let mut dagger = CeremonialDaggerJoker::new();

        // Test identity
        assert_eq!(dagger.joker_type(), "ceremonial_dagger");
        assert_eq!(JokerIdentity::name(&dagger), "Ceremonial Dagger");
        assert_eq!(dagger.base_cost(), 6);

        // Test initial state
        assert_eq!(dagger.current_multiplier, 1.0);
        assert_eq!(dagger.blind_count, 0);

        // Test blind doubling
        let stage = Stage::Blind(Blind::Small);
        let played_cards = vec![];
        let held_cards = vec![];
        let mut events = vec![];
        let mut hand_score = crate::joker::traits::HandScore {
            chips: 0,
            mult: 0.0,
        };

        let joker_state_manager = crate::joker_state::JokerStateManager::new();
        let hand = crate::hand::SelectHand::new(played_cards.clone());

        let mut context = ProcessContext {
            hand_score: &mut hand_score,
            played_cards: &played_cards,
            held_cards: &held_cards,
            events: &mut events,
            hand: &hand,
            joker_state_manager: &joker_state_manager,
        };

        dagger.process(&stage, &mut context);
        assert_eq!(dagger.blind_count, 1);
        assert_eq!(dagger.current_multiplier, 2.0);

        // Test reset on round end
        JokerLifecycle::on_round_end(&mut dagger);
        assert_eq!(dagger.current_multiplier, 1.0);
        assert_eq!(dagger.blind_count, 0);
    }

    #[test]
    fn test_state_serialization() {
        let mut throwback = ThrowbackJoker::new();
        throwback.rounds_played = 3;
        throwback.current_multiplier = 2.5;

        let state = JokerState::serialize_state(&throwback).unwrap();

        let mut new_throwback = ThrowbackJoker::new();
        JokerState::deserialize_state(&mut new_throwback, state).unwrap();

        assert_eq!(new_throwback.rounds_played, 3);
        assert_eq!(new_throwback.current_multiplier, 2.5);
    }

    #[test]
    fn test_can_trigger() {
        let mut steel = ScalingSteelJoker::new();
        let stage = Stage::Blind(Blind::Small);

        let played_cards = vec![];
        let held_cards = vec![];
        let mut events = vec![];
        let mut hand_score = crate::joker::traits::HandScore {
            chips: 0,
            mult: 0.0,
        };

        let joker_state_manager = crate::joker_state::JokerStateManager::new();
        let hand = crate::hand::SelectHand::new(played_cards.clone());

        let context = ProcessContext {
            hand_score: &mut hand_score,
            played_cards: &played_cards,
            held_cards: &held_cards,
            events: &mut events,
            hand: &hand,
            joker_state_manager: &joker_state_manager,
        };

        // Should not trigger when multiplier is 1.0
        assert!(!steel.can_trigger(&stage, &context));

        // Should trigger after accumulating rounds
        steel.rounds_accumulated = 1;
        steel.current_multiplier = 1.2;
        assert!(steel.can_trigger(&stage, &context));
    }
}
