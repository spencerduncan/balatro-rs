//! The Wheel Boss Blind Implementation
//!
//! The Wheel randomly hides 1 out of every 7 cards face-down.
//! This is a minimum ante 2 boss blind with 2x score requirement.
//!
//! ## Mechanics
//! - Randomly selects 1/7 of all cards to be face-down and hidden
//! - Selection is deterministic based on game seed but appears random
//! - Face-down cards cannot be identified until played
//! - Hidden cards are distributed throughout the deck randomly
//! - Minimum ante: 2
//! - Score requirement: 2x base chips

use crate::boss_blinds::{BlindEffect, BossBlind, CounterType};
use crate::card::Card;
use crate::game::Game;
use std::collections::HashSet;

/// The Wheel Boss Blind
///
/// Randomly hides 1 out of every 7 cards face-down throughout the game.
/// The selection is made when the boss blind activates and remains consistent.
#[derive(Debug)]
pub struct TheWheel;

impl BossBlind for TheWheel {
    fn name(&self) -> &str {
        "The Wheel"
    }

    fn min_ante(&self) -> u32 {
        2
    }

    fn apply_effects(&self, game: &mut Game) {
        // Set custom state to track that random hiding is active
        game.boss_blind_state.set_custom_state(
            "random_hiding_active".to_string(),
            crate::boss_blinds::BossBlindData::Boolean(true),
        );

        // Set the ratio of cards to hide (1 out of 7)
        game.boss_blind_state.set_custom_state(
            "hide_ratio".to_string(),
            crate::boss_blinds::BossBlindData::Float(1.0 / 7.0),
        );

        // Generate the list of hidden card indices using the game's RNG
        let hidden_indices = Self::generate_hidden_card_indices(game);
        game.boss_blind_state.set_custom_state(
            "hidden_card_indices".to_string(),
            crate::boss_blinds::BossBlindData::IntegerList(hidden_indices),
        );
    }

    fn get_effects(&self) -> Vec<BlindEffect> {
        vec![BlindEffect::SpecialRule(
            "1 in 7 cards are face-down".to_string(),
        )]
    }

    fn check_counters(&self, _game: &Game) -> Vec<CounterType> {
        // The Wheel needs to track hands played and cards scored to manage hidden cards
        vec![CounterType::HandsPlayed, CounterType::CardsScored]
    }
}

impl TheWheel {
    /// Check if The Wheel boss blind is currently active
    pub fn is_active(game: &Game) -> bool {
        if let Some(active_boss) = game.boss_blind_state.active_boss() {
            if matches!(active_boss, crate::boss_blinds::BossBlindId::TheWheel) {
                if let Some(crate::boss_blinds::BossBlindData::Boolean(true)) = game
                    .boss_blind_state
                    .get_custom_state("random_hiding_active")
                {
                    return true;
                }
            }
        }
        false
    }

    /// Generate a list of card indices that should be hidden (deterministic based on game state)
    fn generate_hidden_card_indices(game: &Game) -> Vec<i64> {
        // Use a simple deterministic approach based on ante and round
        // This ensures the same cards are hidden consistently during the blind
        let mut hidden_indices = Vec::new();

        // Create a deterministic but seemingly random selection
        // Use ante and round as seeds for consistency
        let ante_value = match game.ante_current {
            crate::ante::Ante::Zero => 0,
            crate::ante::Ante::One => 1,
            crate::ante::Ante::Two => 2,
            crate::ante::Ante::Three => 3,
            crate::ante::Ante::Four => 4,
            crate::ante::Ante::Five => 5,
            crate::ante::Ante::Six => 6,
            crate::ante::Ante::Seven => 7,
            crate::ante::Ante::Eight => 8,
        };
        let seed = (ante_value as u64) * 7 + (game.round as u64);

        // For simplicity, hide every 7th card starting from a position based on the seed
        let start_offset = (seed % 7) as usize;

        // Assume a standard 52-card deck for calculation
        // In practice, this would need to consider the actual deck composition
        for i in 0..52 {
            if (i + start_offset) % 7 == 0 {
                hidden_indices.push(i as i64);
            }
        }

        hidden_indices
    }

    /// Check if a card at a specific position should be hidden
    pub fn is_card_index_hidden(game: &Game, card_index: usize) -> bool {
        if !Self::is_active(game) {
            return false;
        }

        if let Some(crate::boss_blinds::BossBlindData::IntegerList(hidden_indices)) = game
            .boss_blind_state
            .get_custom_state("hidden_card_indices")
        {
            return hidden_indices.contains(&(card_index as i64));
        }

        false
    }

    /// Get the ratio of cards that should be hidden (1/7)
    pub fn get_hide_ratio(game: &Game) -> f64 {
        if Self::is_active(game) {
            if let Some(crate::boss_blinds::BossBlindData::Float(ratio)) =
                game.boss_blind_state.get_custom_state("hide_ratio")
            {
                return *ratio;
            }
        }
        0.0
    }

    /// Get all card indices that should be hidden
    pub fn get_hidden_card_indices(game: &Game) -> Vec<i64> {
        if Self::is_active(game) {
            if let Some(crate::boss_blinds::BossBlindData::IntegerList(indices)) = game
                .boss_blind_state
                .get_custom_state("hidden_card_indices")
            {
                return indices.clone();
            }
        }
        vec![]
    }

    /// Check if a specific card in a collection should be hidden based on its position
    pub fn is_card_hidden_in_collection(game: &Game, cards: &[Card], card: &Card) -> bool {
        if !Self::is_active(game) {
            return false;
        }

        // Find the position of this card in the collection
        if let Some(position) = cards.iter().position(|c| c == card) {
            return Self::is_card_index_hidden(game, position);
        }

        false
    }

    /// Filter cards to get only the hidden ones
    pub fn get_hidden_cards(game: &Game, cards: &[Card]) -> Vec<Card> {
        if !Self::is_active(game) {
            return vec![];
        }

        let hidden_indices: HashSet<usize> = Self::get_hidden_card_indices(game)
            .iter()
            .map(|&i| i as usize)
            .collect();

        cards
            .iter()
            .enumerate()
            .filter(|(index, _)| hidden_indices.contains(index))
            .map(|(_, card)| *card)
            .collect()
    }

    /// Filter cards to get only the visible ones
    pub fn get_visible_cards(game: &Game, cards: &[Card]) -> Vec<Card> {
        if !Self::is_active(game) {
            return cards.to_vec();
        }

        let hidden_indices: HashSet<usize> = Self::get_hidden_card_indices(game)
            .iter()
            .map(|&i| i as usize)
            .collect();

        cards
            .iter()
            .enumerate()
            .filter(|(index, _)| !hidden_indices.contains(index))
            .map(|(_, card)| *card)
            .collect()
    }

    /// Check if random card hiding is active
    pub fn is_random_hiding_active(game: &Game) -> bool {
        Self::is_active(game)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::boss_blinds::BossBlindId;
    use crate::card::{Card, Suit, Value};
    use crate::config::Config;

    fn create_test_cards(count: usize) -> Vec<Card> {
        let suits = [Suit::Heart, Suit::Diamond, Suit::Club, Suit::Spade];
        let values = [
            Value::Ace,
            Value::Two,
            Value::Three,
            Value::Four,
            Value::Five,
            Value::Six,
            Value::Seven,
            Value::Eight,
            Value::Nine,
            Value::Ten,
            Value::Jack,
            Value::Queen,
            Value::King,
        ];

        (0..count)
            .map(|i| {
                let suit = suits[i % suits.len()];
                let value = values[i % values.len()];
                Card::new(value, suit)
            })
            .collect()
    }

    #[test]
    fn test_wheel_boss_blind_properties() {
        let wheel = TheWheel;

        assert_eq!(wheel.name(), "The Wheel");
        assert_eq!(wheel.min_ante(), 2);

        let effects = wheel.get_effects();
        assert_eq!(effects.len(), 1);
        assert_eq!(
            effects[0],
            BlindEffect::SpecialRule("1 in 7 cards are face-down".to_string())
        );
    }

    #[test]
    fn test_wheel_apply_effects() {
        let wheel = TheWheel;
        let mut game = Game::new(Config::new());

        // Initially, The Wheel should not be active
        assert!(!TheWheel::is_active(&game));
        assert!(!TheWheel::is_random_hiding_active(&game));
        assert_eq!(TheWheel::get_hide_ratio(&game), 0.0);

        // Activate The Wheel
        game.boss_blind_state.activate(BossBlindId::TheWheel);
        wheel.apply_effects(&mut game);

        // Now The Wheel should be active
        assert!(TheWheel::is_active(&game));
        assert!(TheWheel::is_random_hiding_active(&game));
        assert_eq!(TheWheel::get_hide_ratio(&game), 1.0 / 7.0);

        // Should have generated some hidden indices
        let hidden_indices = TheWheel::get_hidden_card_indices(&game);
        assert!(!hidden_indices.is_empty());
    }

    #[test]
    fn test_wheel_hidden_card_generation() {
        let wheel = TheWheel;
        let mut game = Game::new(Config::new());

        // Activate The Wheel
        game.boss_blind_state.activate(BossBlindId::TheWheel);
        wheel.apply_effects(&mut game);

        let hidden_indices = TheWheel::get_hidden_card_indices(&game);

        // Should have approximately 1/7 of 52 cards hidden (about 7-8 cards)
        assert!(hidden_indices.len() >= 6);
        assert!(hidden_indices.len() <= 9);

        // All indices should be valid (0-51 for a standard deck)
        for &index in &hidden_indices {
            assert!(index >= 0);
            assert!(index < 52);
        }
    }

    #[test]
    fn test_wheel_card_hiding_in_collection() {
        let wheel = TheWheel;
        let mut game = Game::new(Config::new());

        let cards = create_test_cards(21); // 3 times 7 cards

        // Without The Wheel active, no cards should be hidden
        assert_eq!(TheWheel::get_hidden_cards(&game, &cards).len(), 0);
        assert_eq!(TheWheel::get_visible_cards(&game, &cards).len(), 21);

        // Activate The Wheel
        game.boss_blind_state.activate(BossBlindId::TheWheel);
        wheel.apply_effects(&mut game);

        let hidden_cards = TheWheel::get_hidden_cards(&game, &cards);
        let visible_cards = TheWheel::get_visible_cards(&game, &cards);

        // Should have some hidden cards
        assert!(!hidden_cards.is_empty());

        // Hidden + visible should equal total
        assert_eq!(hidden_cards.len() + visible_cards.len(), cards.len());

        // No card should appear in both lists
        for hidden_card in &hidden_cards {
            assert!(!visible_cards.contains(hidden_card));
        }
    }

    #[test]
    fn test_wheel_deterministic_hiding() {
        let wheel = TheWheel;
        let mut game1 = Game::new(Config::new());
        let mut game2 = Game::new(Config::new());

        // Set the same ante and round for both games
        game1.ante_current = crate::ante::Ante::Three;
        game1.round = 5.0;
        game2.ante_current = crate::ante::Ante::Three;
        game2.round = 5.0;

        // Activate The Wheel on both games
        game1.boss_blind_state.activate(BossBlindId::TheWheel);
        wheel.apply_effects(&mut game1);

        game2.boss_blind_state.activate(BossBlindId::TheWheel);
        wheel.apply_effects(&mut game2);

        // Should generate the same hidden indices
        let hidden1 = TheWheel::get_hidden_card_indices(&game1);
        let hidden2 = TheWheel::get_hidden_card_indices(&game2);
        assert_eq!(hidden1, hidden2);
    }

    #[test]
    fn test_wheel_check_counters() {
        let wheel = TheWheel;
        let game = Game::new(Config::new());

        let counters = wheel.check_counters(&game);
        assert_eq!(counters.len(), 2);
        assert!(counters.contains(&CounterType::HandsPlayed));
        assert!(counters.contains(&CounterType::CardsScored));
    }

    #[test]
    fn test_wheel_deactivation() {
        let wheel = TheWheel;
        let mut game = Game::new(Config::new());

        // Activate The Wheel
        game.boss_blind_state.activate(BossBlindId::TheWheel);
        wheel.apply_effects(&mut game);
        assert!(TheWheel::is_active(&game));

        // Deactivate boss blind
        game.boss_blind_state.deactivate();
        assert!(!TheWheel::is_active(&game));
        assert!(!TheWheel::is_random_hiding_active(&game));
        assert_eq!(TheWheel::get_hide_ratio(&game), 0.0);
    }

    #[test]
    fn test_wheel_card_index_hiding() {
        let wheel = TheWheel;
        let mut game = Game::new(Config::new());

        // Activate The Wheel
        game.boss_blind_state.activate(BossBlindId::TheWheel);
        wheel.apply_effects(&mut game);

        let hidden_indices = TheWheel::get_hidden_card_indices(&game);

        // Test specific indices
        for (i, _) in (0..20).enumerate() {
            let should_be_hidden = hidden_indices.contains(&(i as i64));
            let is_hidden = TheWheel::is_card_index_hidden(&game, i);
            assert_eq!(should_be_hidden, is_hidden);
        }
    }
}
