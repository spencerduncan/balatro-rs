//! The Serpent Boss Blind Implementation
//!
//! The Serpent forces the player to draw exactly 3 cards after each play or discard action.
//! This is a minimum ante 5 boss blind with 2x score requirement.
//!
//! ## Mechanics
//! - After any Play or Discard action, force draw exactly 3 cards
//! - This happens immediately after the action is completed
//! - Effect is active throughout the entire blind
//! - Can lead to hand overflow if not managed properly
//! - Minimum ante: 5
//! - Score requirement: 2x base chips

use crate::boss_blinds::{BlindEffect, BossBlind, CounterType};
use crate::game::Game;

/// The Serpent Boss Blind
///
/// Forces the player to draw exactly 3 cards after each play or discard action.
/// This creates a unique challenge where players must manage their hand size carefully,
/// as the forced drawing can lead to hand overflow situations.
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
        // Set custom state to track that forced drawing is active
        game.boss_blind_state.set_custom_state(
            "forced_draw_active".to_string(),
            crate::boss_blinds::BossBlindData::Boolean(true),
        );

        // Set the number of cards to force draw after each action
        game.boss_blind_state.set_custom_state(
            "forced_draw_count".to_string(),
            crate::boss_blinds::BossBlindData::Integer(3),
        );
    }

    fn get_effects(&self) -> Vec<BlindEffect> {
        vec![BlindEffect::SpecialRule(
            "Draw 3 cards after each play or discard".to_string(),
        )]
    }

    fn check_counters(&self, _game: &Game) -> Vec<CounterType> {
        // The Serpent needs to track hands played and cards discarded
        // to trigger the forced drawing effect
        vec![CounterType::HandsPlayed, CounterType::CardsDiscarded]
    }
}

impl TheSerpent {
    /// Check if The Serpent boss blind is currently active and forcing draws
    pub fn is_active_and_forcing_draws(game: &Game) -> bool {
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

    /// Get the number of cards to force draw (should be 3 for The Serpent)
    pub fn get_forced_draw_count(game: &Game) -> usize {
        if Self::is_active_and_forcing_draws(game) {
            if let Some(crate::boss_blinds::BossBlindData::Integer(count)) =
                game.boss_blind_state.get_custom_state("forced_draw_count")
            {
                return *count as usize;
            }
        }
        0
    }

    /// Apply The Serpent's forced drawing effect after play or discard actions
    ///
    /// This should be called by the game engine immediately after a play or discard
    /// action is completed when The Serpent boss blind is active.
    ///
    /// # Parameters
    /// * `game` - Mutable reference to the game state
    ///
    /// # Returns
    /// * `usize` - The number of cards actually drawn (may be less than 3 if deck is empty)
    pub fn force_draw_cards(game: &mut Game) -> usize {
        if !Self::is_active_and_forcing_draws(game) {
            return 0;
        }

        let cards_to_draw = Self::get_forced_draw_count(game);
        if cards_to_draw == 0 {
            return 0;
        }

        // Get the number of cards in deck before drawing
        let cards_in_deck = game.deck.cards().len();
        let actual_draw_count = cards_to_draw.min(cards_in_deck);

        if actual_draw_count > 0 {
            // Draw cards directly from deck to available using public fields
            if let Some(drawn_cards) = game.deck.draw(actual_draw_count) {
                game.available.extend(drawn_cards);
                // Update target context with new available cards (if sync method is public)
                // Note: The private sync_target_context() method would normally be called here
                // but we're working within boss blind constraints
            }
        }

        actual_draw_count
    }

    /// Check if a forced draw should be triggered after the given action
    ///
    /// The Serpent triggers forced draws after Play and Discard actions
    pub fn should_trigger_forced_draw(action: &crate::action::Action) -> bool {
        matches!(
            action,
            crate::action::Action::Play() | crate::action::Action::Discard()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::action::Action;
    use crate::boss_blinds::BossBlindId;
    use crate::card::{Card, Suit, Value};
    use crate::config::Config;

    #[allow(dead_code)]
    fn create_test_card(value: Value) -> Card {
        Card::new(value, Suit::Heart)
    }

    #[test]
    fn test_serpent_boss_blind_properties() {
        let serpent = TheSerpent;

        assert_eq!(serpent.name(), "The Serpent");
        assert_eq!(serpent.min_ante(), 5);

        let effects = serpent.get_effects();
        assert_eq!(effects.len(), 1);
        assert_eq!(
            effects[0],
            BlindEffect::SpecialRule("Draw 3 cards after each play or discard".to_string())
        );

        let counters = serpent.check_counters(&Game::new(Config::new()));
        assert_eq!(counters.len(), 2);
        assert!(counters.contains(&CounterType::HandsPlayed));
        assert!(counters.contains(&CounterType::CardsDiscarded));
    }

    #[test]
    fn test_serpent_apply_effects() {
        let serpent = TheSerpent;
        let mut game = Game::new(Config::new());

        // Initially, no boss blind should be active
        assert!(!TheSerpent::is_active_and_forcing_draws(&game));
        assert_eq!(TheSerpent::get_forced_draw_count(&game), 0);

        // Activate The Serpent
        game.boss_blind_state.activate(BossBlindId::TheSerpent);
        serpent.apply_effects(&mut game);

        // Now The Serpent should be active and forcing draws
        assert!(TheSerpent::is_active_and_forcing_draws(&game));
        assert_eq!(TheSerpent::get_forced_draw_count(&game), 3);
    }

    #[test]
    fn test_should_trigger_forced_draw() {
        let play_action = Action::Play();
        let discard_action = Action::Discard();
        let other_action = Action::NextRound();

        assert!(TheSerpent::should_trigger_forced_draw(&play_action));
        assert!(TheSerpent::should_trigger_forced_draw(&discard_action));
        assert!(!TheSerpent::should_trigger_forced_draw(&other_action));
    }

    #[test]
    fn test_force_draw_cards_inactive() {
        let mut game = Game::new(Config::new());

        // When The Serpent is not active, no cards should be drawn
        let drawn = TheSerpent::force_draw_cards(&mut game);
        assert_eq!(drawn, 0);
    }

    #[test]
    fn test_force_draw_cards_active() {
        let serpent = TheSerpent;
        let mut game = Game::new(Config::new());

        // Get initial available card count
        let initial_available = game.available.cards().len();

        // Activate The Serpent
        game.boss_blind_state.activate(BossBlindId::TheSerpent);
        serpent.apply_effects(&mut game);

        // Ensure we have cards in the deck to draw
        assert!(game.deck.cards().len() >= 3);

        // Force draw cards
        let drawn = TheSerpent::force_draw_cards(&mut game);
        assert_eq!(drawn, 3);

        // Verify that cards were added to available
        assert_eq!(game.available.cards().len(), initial_available + 3);
    }

    #[test]
    fn test_force_draw_cards_empty_deck() {
        let serpent = TheSerpent;
        let mut game = Game::new(Config::new());

        // Activate The Serpent
        game.boss_blind_state.activate(BossBlindId::TheSerpent);
        serpent.apply_effects(&mut game);

        // Empty the deck by manually drawing all cards
        let deck_size = game.deck.cards().len();
        if deck_size > 0 {
            if let Some(drawn_cards) = game.deck.draw(deck_size) {
                game.available.extend(drawn_cards);
            }
        }

        // Verify deck is empty
        assert_eq!(game.deck.cards().len(), 0);

        // Try to force draw - should draw 0 cards
        let drawn = TheSerpent::force_draw_cards(&mut game);
        assert_eq!(drawn, 0);
    }

    #[test]
    fn test_force_draw_cards_partial_deck() {
        let serpent = TheSerpent;
        let mut game = Game::new(Config::new());

        // Activate The Serpent
        game.boss_blind_state.activate(BossBlindId::TheSerpent);
        serpent.apply_effects(&mut game);

        // Ensure we only have 2 cards left in deck
        let deck_size = game.deck.cards().len();
        if deck_size > 2 {
            if let Some(drawn_cards) = game.deck.draw(deck_size - 2) {
                game.available.extend(drawn_cards);
            }
        }

        // Verify we have exactly 2 cards in deck
        assert_eq!(game.deck.cards().len(), 2);

        let initial_available = game.available.cards().len();

        // Try to force draw 3 cards - should only draw 2
        let drawn = TheSerpent::force_draw_cards(&mut game);
        assert_eq!(drawn, 2);

        // Verify that only 2 cards were added to available
        assert_eq!(game.available.cards().len(), initial_available + 2);
        assert_eq!(game.deck.cards().len(), 0);
    }
}
