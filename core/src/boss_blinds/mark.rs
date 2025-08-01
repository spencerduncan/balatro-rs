//! The Mark Boss Blind Implementation
//!
//! The Mark hides all face cards (Jacks, Queens, Kings) face-down until played.
//! This is a minimum ante 2 boss blind with 2x score requirement.
//!
//! ## Mechanics
//! - All face cards (Jack, Queen, King) are face-down and hidden
//! - Face-down cards cannot be identified by the player until played
//! - Face-down cards still function normally when played but are hidden during selection
//! - Minimum ante: 2
//! - Score requirement: 2x base chips

use crate::boss_blinds::{BlindEffect, BossBlind, CounterType};
use crate::card::{Card, Value};
use crate::game::Game;

/// The Mark Boss Blind
///
/// Hides all face cards face-down, making them unidentifiable until played.
/// This creates uncertainty and forces players to make decisions without full information.
#[derive(Debug)]
pub struct TheMark;

impl BossBlind for TheMark {
    fn name(&self) -> &str {
        "The Mark"
    }

    fn min_ante(&self) -> u32 {
        2
    }

    fn apply_effects(&self, game: &mut Game) {
        // Set custom state to track that face card hiding is active
        game.boss_blind_state.set_custom_state(
            "face_cards_hidden".to_string(),
            crate::boss_blinds::BossBlindData::Boolean(true),
        );

        // Create a list of card values that should be hidden
        let hidden_values = vec![Value::Jack as i64, Value::Queen as i64, Value::King as i64];

        game.boss_blind_state.set_custom_state(
            "hidden_card_values".to_string(),
            crate::boss_blinds::BossBlindData::IntegerList(hidden_values),
        );
    }

    fn get_effects(&self) -> Vec<BlindEffect> {
        vec![BlindEffect::SpecialRule(
            "Face cards are face-down".to_string(),
        )]
    }

    fn check_counters(&self, _game: &Game) -> Vec<CounterType> {
        // The Mark needs to track hands played and cards scored to manage hidden cards
        vec![CounterType::HandsPlayed, CounterType::CardsScored]
    }
}

impl TheMark {
    /// Check if The Mark boss blind is currently active
    pub fn is_active(game: &Game) -> bool {
        if let Some(active_boss) = game.boss_blind_state.active_boss() {
            if matches!(active_boss, crate::boss_blinds::BossBlindId::TheMark) {
                if let Some(crate::boss_blinds::BossBlindData::Boolean(true)) =
                    game.boss_blind_state.get_custom_state("face_cards_hidden")
                {
                    return true;
                }
            }
        }
        false
    }

    /// Check if a specific card should be hidden by The Mark
    pub fn is_card_hidden(game: &Game, card: &Card) -> bool {
        if !Self::is_active(game) {
            return false;
        }

        // Check if this card value is in the list of hidden values
        if let Some(crate::boss_blinds::BossBlindData::IntegerList(hidden_values)) =
            game.boss_blind_state.get_custom_state("hidden_card_values")
        {
            let card_value = card.value as i64;
            return hidden_values.contains(&card_value);
        }

        false
    }

    /// Check if a card is a face card that should be hidden
    pub fn is_face_card_hidden(card: &Card) -> bool {
        matches!(card.value, Value::Jack | Value::Queen | Value::King)
    }

    /// Get all cards that should be hidden by The Mark
    pub fn get_hidden_cards(game: &Game, cards: &[Card]) -> Vec<Card> {
        if !Self::is_active(game) {
            return vec![];
        }

        cards
            .iter()
            .filter(|card| Self::is_card_hidden(game, card))
            .cloned()
            .collect()
    }

    /// Get all visible (non-hidden) cards
    pub fn get_visible_cards(game: &Game, cards: &[Card]) -> Vec<Card> {
        if !Self::is_active(game) {
            return cards.to_vec();
        }

        cards
            .iter()
            .filter(|card| !Self::is_card_hidden(game, card))
            .cloned()
            .collect()
    }

    /// Check if face cards are currently hidden
    pub fn are_face_cards_hidden(game: &Game) -> bool {
        Self::is_active(game)
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
    fn test_mark_boss_blind_properties() {
        let mark = TheMark;

        assert_eq!(mark.name(), "The Mark");
        assert_eq!(mark.min_ante(), 2);

        let effects = mark.get_effects();
        assert_eq!(effects.len(), 1);
        assert_eq!(
            effects[0],
            BlindEffect::SpecialRule("Face cards are face-down".to_string())
        );
    }

    #[test]
    fn test_mark_apply_effects() {
        let mark = TheMark;
        let mut game = Game::new(Config::new());

        // Initially, The Mark should not be active
        assert!(!TheMark::is_active(&game));
        assert!(!TheMark::are_face_cards_hidden(&game));

        // Activate The Mark
        game.boss_blind_state.activate(BossBlindId::TheMark);
        mark.apply_effects(&mut game);

        // Now The Mark should be active
        assert!(TheMark::is_active(&game));
        assert!(TheMark::are_face_cards_hidden(&game));
    }

    #[test]
    fn test_mark_card_hiding() {
        let mark = TheMark;
        let mut game = Game::new(Config::new());

        // Create test cards
        let jack = create_test_card(Value::Jack);
        let queen = create_test_card(Value::Queen);
        let king = create_test_card(Value::King);
        let ace = create_test_card(Value::Ace);
        let ten = create_test_card(Value::Ten);

        // Without The Mark active, no cards should be hidden
        assert!(!TheMark::is_card_hidden(&game, &jack));
        assert!(!TheMark::is_card_hidden(&game, &queen));
        assert!(!TheMark::is_card_hidden(&game, &king));
        assert!(!TheMark::is_card_hidden(&game, &ace));

        // Activate The Mark
        game.boss_blind_state.activate(BossBlindId::TheMark);
        mark.apply_effects(&mut game);

        // Face cards should now be hidden
        assert!(TheMark::is_card_hidden(&game, &jack));
        assert!(TheMark::is_card_hidden(&game, &queen));
        assert!(TheMark::is_card_hidden(&game, &king));

        // Non-face cards should not be hidden
        assert!(!TheMark::is_card_hidden(&game, &ace));
        assert!(!TheMark::is_card_hidden(&game, &ten));
    }

    #[test]
    fn test_mark_face_card_identification() {
        // Test the static method for identifying face cards
        assert!(TheMark::is_face_card_hidden(&create_test_card(Value::Jack)));
        assert!(TheMark::is_face_card_hidden(&create_test_card(
            Value::Queen
        )));
        assert!(TheMark::is_face_card_hidden(&create_test_card(Value::King)));

        assert!(!TheMark::is_face_card_hidden(&create_test_card(Value::Ace)));
        assert!(!TheMark::is_face_card_hidden(&create_test_card(Value::Ten)));
        assert!(!TheMark::is_face_card_hidden(&create_test_card(Value::Two)));
    }

    #[test]
    fn test_mark_get_hidden_and_visible_cards() {
        let mark = TheMark;
        let mut game = Game::new(Config::new());

        let cards = vec![
            create_test_card(Value::Jack),
            create_test_card(Value::Queen),
            create_test_card(Value::King),
            create_test_card(Value::Ace),
            create_test_card(Value::Ten),
        ];

        // Without The Mark active, all cards should be visible
        let visible = TheMark::get_visible_cards(&game, &cards);
        let hidden = TheMark::get_hidden_cards(&game, &cards);
        assert_eq!(visible.len(), 5);
        assert_eq!(hidden.len(), 0);

        // Activate The Mark
        game.boss_blind_state.activate(BossBlindId::TheMark);
        mark.apply_effects(&mut game);

        // Face cards should be hidden, others visible
        let visible = TheMark::get_visible_cards(&game, &cards);
        let hidden = TheMark::get_hidden_cards(&game, &cards);
        assert_eq!(visible.len(), 2); // Ace and Ten
        assert_eq!(hidden.len(), 3); // Jack, Queen, King

        // Check specific cards
        assert!(hidden.iter().any(|c| matches!(c.value, Value::Jack)));
        assert!(hidden.iter().any(|c| matches!(c.value, Value::Queen)));
        assert!(hidden.iter().any(|c| matches!(c.value, Value::King)));

        assert!(visible.iter().any(|c| matches!(c.value, Value::Ace)));
        assert!(visible.iter().any(|c| matches!(c.value, Value::Ten)));
    }

    #[test]
    fn test_mark_check_counters() {
        let mark = TheMark;
        let game = Game::new(Config::new());

        let counters = mark.check_counters(&game);
        assert_eq!(counters.len(), 2);
        assert!(counters.contains(&CounterType::HandsPlayed));
        assert!(counters.contains(&CounterType::CardsScored));
    }

    #[test]
    fn test_mark_deactivation() {
        let mark = TheMark;
        let mut game = Game::new(Config::new());

        let jack = create_test_card(Value::Jack);

        // Activate The Mark
        game.boss_blind_state.activate(BossBlindId::TheMark);
        mark.apply_effects(&mut game);
        assert!(TheMark::is_card_hidden(&game, &jack));

        // Deactivate boss blind
        game.boss_blind_state.deactivate();
        assert!(!TheMark::is_active(&game));
        assert!(!TheMark::is_card_hidden(&game, &jack));
        assert!(!TheMark::are_face_cards_hidden(&game));
    }
}
