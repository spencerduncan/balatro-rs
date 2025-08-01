//! The Serpent Boss Blind Implementation
//!
//! The Serpent forces the player to draw 3 cards from the deck after each play or discard.
//! This is a minimum ante 5 boss blind with 2x score requirement.
//!
//! ## Mechanics
//! - After each hand play, player draws 3 cards
//! - After each discard action, player draws 3 cards
//! - Can quickly fill the hand but also exhaust the deck
//! - Minimum ante: 5
//! - Score requirement: 2x base chips

use crate::boss_blinds::{BlindEffect, BossBlind, CounterType};
use crate::game::Game;

/// The Serpent Boss Blind
///
/// Forces the player to draw 3 cards from the deck after each play or discard action.
/// This creates tension between needing to play hands and risking running out of cards.
#[derive(Debug)]
pub struct TheSerpent;

impl BossBlind for TheSerpent {
    fn name(&self) -> &str {
        "The Serpent"
    }

    fn min_ante(&self) -> u32 {
        5
    }

    fn apply_effects(&self, game: &mut Game) {
        // Set custom state to track that forced draw is active
        game.boss_blind_state.set_custom_state(
            "forced_draw_active".to_string(),
            crate::boss_blinds::BossBlindData::Boolean(true),
        );

        // Set the number of cards to draw after each action
        game.boss_blind_state.set_custom_state(
            "cards_to_draw".to_string(),
            crate::boss_blinds::BossBlindData::Integer(3),
        );
    }

    fn get_effects(&self) -> Vec<BlindEffect> {
        vec![BlindEffect::SpecialRule(
            "Draw 3 cards after play or discard".to_string(),
        )]
    }

    fn check_counters(&self, _game: &Game) -> Vec<CounterType> {
        // The Serpent needs to track hands played and cards discarded to trigger draws
        vec![CounterType::HandsPlayed, CounterType::CardsDiscarded]
    }
}

impl TheSerpent {
    /// Check if The Serpent boss blind is currently active
    pub fn is_active(game: &Game) -> bool {
        if let Some(active_boss) = game.boss_blind_state.active_boss() {
            if matches!(active_boss, crate::boss_blinds::BossBlindId::TheSerpent) {
                if let Some(crate::boss_blinds::BossBlindData::Boolean(true)) =
                    game.boss_blind_state.get_custom_state("forced_draw_active")
                {
                    return true;
                }
            }
        }
        false
    }

    /// Get the number of cards to draw after each action
    pub fn get_cards_to_draw(game: &Game) -> i64 {
        if Self::is_active(game) {
            if let Some(crate::boss_blinds::BossBlindData::Integer(count)) =
                game.boss_blind_state.get_custom_state("cards_to_draw")
            {
                return *count;
            }
        }
        0
    }

    /// Check if forced drawing should occur after a play or discard action
    /// This would be called by the game engine after hand plays or discards
    pub fn should_trigger_forced_draw(game: &Game) -> bool {
        Self::is_active(game)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::boss_blinds::BossBlindId;
    use crate::config::Config;

    #[test]
    fn test_serpent_boss_blind_properties() {
        let serpent = TheSerpent;

        assert_eq!(serpent.name(), "The Serpent");
        assert_eq!(serpent.min_ante(), 5);

        let effects = serpent.get_effects();
        assert_eq!(effects.len(), 1);
        assert_eq!(
            effects[0],
            BlindEffect::SpecialRule("Draw 3 cards after play or discard".to_string())
        );
    }

    #[test]
    fn test_serpent_apply_effects() {
        let serpent = TheSerpent;
        let mut game = Game::new(Config::new());

        // Initially, The Serpent should not be active
        assert!(!TheSerpent::is_active(&game));
        assert_eq!(TheSerpent::get_cards_to_draw(&game), 0);

        // Activate The Serpent
        game.boss_blind_state.activate(BossBlindId::TheSerpent);
        serpent.apply_effects(&mut game);

        // Now The Serpent should be active
        assert!(TheSerpent::is_active(&game));
        assert_eq!(TheSerpent::get_cards_to_draw(&game), 3);
        assert!(TheSerpent::should_trigger_forced_draw(&game));
    }

    #[test]
    fn test_serpent_check_counters() {
        let serpent = TheSerpent;
        let game = Game::new(Config::new());

        let counters = serpent.check_counters(&game);
        assert_eq!(counters.len(), 2);
        assert!(counters.contains(&CounterType::HandsPlayed));
        assert!(counters.contains(&CounterType::CardsDiscarded));
    }

    #[test]
    fn test_serpent_not_active_without_boss_blind() {
        let game = Game::new(Config::new());

        // Without activating the boss blind, it should not be active
        assert!(!TheSerpent::is_active(&game));
        assert_eq!(TheSerpent::get_cards_to_draw(&game), 0);
        assert!(!TheSerpent::should_trigger_forced_draw(&game));
    }

    #[test]
    fn test_serpent_deactivation() {
        let serpent = TheSerpent;
        let mut game = Game::new(Config::new());

        // Activate The Serpent
        game.boss_blind_state.activate(BossBlindId::TheSerpent);
        serpent.apply_effects(&mut game);
        assert!(TheSerpent::is_active(&game));

        // Deactivate boss blind
        game.boss_blind_state.deactivate();
        assert!(!TheSerpent::is_active(&game));
        assert_eq!(TheSerpent::get_cards_to_draw(&game), 0);
    }
}
