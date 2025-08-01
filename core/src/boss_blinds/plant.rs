//! The Plant Boss Blind Implementation
//!
//! The Plant debuffs all face cards (Jacks, Queens, Kings) by making them score 0 base points.
//! This is a minimum ante 4 boss blind with 2x score requirement.
//!
//! ## Mechanics
//! - All face cards (Jack, Queen, King) contribute 0 chips during scoring
//! - Face cards still trigger joker effects but contribute no base score
//! - Effect is active throughout the entire blind
//! - Minimum ante: 4
//! - Score requirement: 2x base chips

use crate::boss_blinds::{BlindEffect, BossBlind, CounterType};
use crate::card::{Card, Value};
use crate::game::Game;

/// The Plant Boss Blind
///
/// Debuffs all face cards (Jacks, Queens, Kings) by making them contribute 0 chips during scoring.
/// Face cards still exist and can trigger joker effects, but they provide no base scoring value.
#[derive(Debug)]
pub struct ThePlant;

impl BossBlind for ThePlant {
    fn name(&self) -> &str {
        "The Plant"
    }

    fn min_ante(&self) -> u32 {
        4
    }

    fn apply_effects(&self, game: &mut Game) {
        // Set custom state to track that face cards are debuffed
        game.boss_blind_state.set_custom_state(
            "face_cards_debuffed".to_string(),
            crate::boss_blinds::BossBlindData::Boolean(true),
        );
    }

    fn get_effects(&self) -> Vec<BlindEffect> {
        vec![BlindEffect::DebuffCards("face_cards".to_string())]
    }

    fn check_counters(&self, _game: &Game) -> Vec<CounterType> {
        // The Plant doesn't need to track any specific counters
        // The face card debuffing is applied during scoring evaluation
        vec![CounterType::CardsScored]
    }
}

impl ThePlant {
    /// Check if a card is a face card that should be debuffed by The Plant
    pub fn is_face_card_debuffed(card: &Card) -> bool {
        matches!(card.value, Value::Jack | Value::Queen | Value::King)
    }

    /// Check if The Plant boss blind is currently active and debuffing face cards
    pub fn is_active_and_debuffing(game: &Game) -> bool {
        if let Some(active_boss) = game.boss_blind_state.active_boss() {
            if matches!(active_boss, crate::boss_blinds::BossBlindId::ThePlant) {
                if let Some(crate::boss_blinds::BossBlindData::Boolean(true)) = game
                    .boss_blind_state
                    .get_custom_state("face_cards_debuffed")
                {
                    return true;
                }
            }
        }
        false
    }

    /// Apply The Plant's debuffing effect to a card's base scoring
    ///
    /// Returns true if the card's base scoring should be zeroed out
    pub fn should_debuff_card_scoring(game: &Game, card: &Card) -> bool {
        Self::is_active_and_debuffing(game) && Self::is_face_card_debuffed(card)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::boss_blinds::BossBlindId;
    use crate::card::{Card, Suit, Value};
    use crate::config::Config;

    fn create_test_card(value: Value) -> Card {
        Card::new(value, Suit::Heart)
    }

    #[test]
    fn test_is_face_card_debuffed() {
        // Face cards should be debuffed
        assert!(ThePlant::is_face_card_debuffed(&create_test_card(
            Value::Jack
        )));
        assert!(ThePlant::is_face_card_debuffed(&create_test_card(
            Value::Queen
        )));
        assert!(ThePlant::is_face_card_debuffed(&create_test_card(
            Value::King
        )));

        // Non-face cards should not be debuffed
        assert!(!ThePlant::is_face_card_debuffed(&create_test_card(
            Value::Ace
        )));
        assert!(!ThePlant::is_face_card_debuffed(&create_test_card(
            Value::Two
        )));
        assert!(!ThePlant::is_face_card_debuffed(&create_test_card(
            Value::Ten
        )));
    }

    #[test]
    fn test_plant_boss_blind_properties() {
        let plant = ThePlant;

        assert_eq!(plant.name(), "The Plant");
        assert_eq!(plant.min_ante(), 4);

        let effects = plant.get_effects();
        assert_eq!(effects.len(), 1);
        assert_eq!(
            effects[0],
            BlindEffect::DebuffCards("face_cards".to_string())
        );
    }

    #[test]
    fn test_plant_apply_effects() {
        let plant = ThePlant;
        let mut game = Game::new(Config::new());

        // Initially, no boss blind should be active
        assert!(!ThePlant::is_active_and_debuffing(&game));

        // Activate The Plant
        game.boss_blind_state.activate(BossBlindId::ThePlant);
        plant.apply_effects(&mut game);

        // Now The Plant should be active and debuffing
        assert!(ThePlant::is_active_and_debuffing(&game));
    }

    #[test]
    fn test_should_debuff_card_scoring() {
        let plant = ThePlant;
        let mut game = Game::new(Config::new());

        let jack = create_test_card(Value::Jack);
        let queen = create_test_card(Value::Queen);
        let king = create_test_card(Value::King);
        let ace = create_test_card(Value::Ace);
        let ten = create_test_card(Value::Ten);

        // Initially, no cards should be debuffed
        assert!(!ThePlant::should_debuff_card_scoring(&game, &jack));
        assert!(!ThePlant::should_debuff_card_scoring(&game, &queen));
        assert!(!ThePlant::should_debuff_card_scoring(&game, &king));
        assert!(!ThePlant::should_debuff_card_scoring(&game, &ace));
        assert!(!ThePlant::should_debuff_card_scoring(&game, &ten));

        // Activate The Plant
        game.boss_blind_state.activate(BossBlindId::ThePlant);
        plant.apply_effects(&mut game);

        // Face cards should now be debuffed
        assert!(ThePlant::should_debuff_card_scoring(&game, &jack));
        assert!(ThePlant::should_debuff_card_scoring(&game, &queen));
        assert!(ThePlant::should_debuff_card_scoring(&game, &king));

        // Non-face cards should still not be debuffed
        assert!(!ThePlant::should_debuff_card_scoring(&game, &ace));
        assert!(!ThePlant::should_debuff_card_scoring(&game, &ten));
    }

    #[test]
    fn test_check_counters() {
        let plant = ThePlant;
        let game = Game::new(Config::new());

        let counters = plant.check_counters(&game);
        assert_eq!(counters.len(), 1);
        assert_eq!(counters[0], CounterType::CardsScored);
    }
}
