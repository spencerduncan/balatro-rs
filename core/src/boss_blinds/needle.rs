//! The Needle Boss Blind Implementation
//!
//! The Needle limits the player to only 1 hand play during the blind.
//! This is a minimum ante 2 boss blind with 1x score requirement.
//!
//! ## Mechanics
//! - Player can only play 1 hand during the entire blind
//! - Forces careful hand selection and optimization
//! - Discards are still allowed normally
//! - Minimum ante: 2
//! - Score requirement: 1x base chips (same as normal blind)

use crate::boss_blinds::{BlindEffect, BossBlind, CounterType};
use crate::game::Game;

/// The Needle Boss Blind
///
/// Restricts the player to exactly 1 hand play during the entire blind.
/// This creates a high-stakes decision where the player must choose their single hand carefully.
#[derive(Debug)]
pub struct TheNeedle;

impl BossBlind for TheNeedle {
    fn name(&self) -> &str {
        "The Needle"
    }

    fn min_ante(&self) -> u32 {
        2
    }

    fn apply_effects(&self, game: &mut Game) {
        // Set the maximum number of plays to 1
        game.plays = 1.0;

        // Set custom state to track that play limit is active
        game.boss_blind_state.set_custom_state(
            "play_limit_active".to_string(),
            crate::boss_blinds::BossBlindData::Boolean(true),
        );

        // Store the maximum plays allowed
        game.boss_blind_state.set_custom_state(
            "max_plays".to_string(),
            crate::boss_blinds::BossBlindData::Integer(1),
        );
    }

    fn get_effects(&self) -> Vec<BlindEffect> {
        vec![BlindEffect::RestrictActions(
            "Only 1 hand play allowed".to_string(),
        )]
    }

    fn check_counters(&self, _game: &Game) -> Vec<CounterType> {
        // The Needle needs to track hands played to enforce the limit
        vec![CounterType::HandsPlayed]
    }
}

impl TheNeedle {
    /// Check if The Needle boss blind is currently active
    pub fn is_active(game: &Game) -> bool {
        if let Some(active_boss) = game.boss_blind_state.active_boss() {
            if matches!(active_boss, crate::boss_blinds::BossBlindId::TheNeedle) {
                if let Some(crate::boss_blinds::BossBlindData::Boolean(true)) =
                    game.boss_blind_state.get_custom_state("play_limit_active")
                {
                    return true;
                }
            }
        }
        false
    }

    /// Get the maximum number of plays allowed by The Needle
    pub fn get_max_plays(game: &Game) -> i64 {
        if Self::is_active(game) {
            if let Some(crate::boss_blinds::BossBlindData::Integer(max_plays)) =
                game.boss_blind_state.get_custom_state("max_plays")
            {
                return *max_plays;
            }
        }
        // Return a large number if not active (no limit)
        999
    }

    /// Check if the player can still play hands under The Needle's restriction
    pub fn can_play_hand(game: &Game) -> bool {
        if Self::is_active(game) {
            return game.plays > 0.0;
        }
        // If not active, normal play rules apply
        true
    }

    /// Check if The Needle's play limit has been reached
    pub fn play_limit_reached(game: &Game) -> bool {
        if Self::is_active(game) {
            return game.plays <= 0.0;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::boss_blinds::BossBlindId;
    use crate::config::Config;

    #[test]
    fn test_needle_boss_blind_properties() {
        let needle = TheNeedle;

        assert_eq!(needle.name(), "The Needle");
        assert_eq!(needle.min_ante(), 2);

        let effects = needle.get_effects();
        assert_eq!(effects.len(), 1);
        assert_eq!(
            effects[0],
            BlindEffect::RestrictActions("Only 1 hand play allowed".to_string())
        );
    }

    #[test]
    fn test_needle_apply_effects() {
        let needle = TheNeedle;
        let mut game = Game::new(Config::new());

        // Store initial plays (should be more than 1)
        let initial_plays = game.plays;
        assert!(initial_plays > 1.0);

        // Initially, The Needle should not be active
        assert!(!TheNeedle::is_active(&game));
        assert!(TheNeedle::can_play_hand(&game));

        // Activate The Needle
        game.boss_blind_state.activate(BossBlindId::TheNeedle);
        needle.apply_effects(&mut game);

        // Now The Needle should be active and plays should be limited to 1
        assert!(TheNeedle::is_active(&game));
        assert_eq!(game.plays, 1.0);
        assert_eq!(TheNeedle::get_max_plays(&game), 1);
        assert!(TheNeedle::can_play_hand(&game));
        assert!(!TheNeedle::play_limit_reached(&game));
    }

    #[test]
    fn test_needle_play_limit_enforcement() {
        let needle = TheNeedle;
        let mut game = Game::new(Config::new());

        // Activate The Needle
        game.boss_blind_state.activate(BossBlindId::TheNeedle);
        needle.apply_effects(&mut game);

        // Initially can play
        assert!(TheNeedle::can_play_hand(&game));
        assert!(!TheNeedle::play_limit_reached(&game));

        // Simulate playing the one allowed hand
        game.plays = 0.0;

        // Now should not be able to play more hands
        assert!(!TheNeedle::can_play_hand(&game));
        assert!(TheNeedle::play_limit_reached(&game));
    }

    #[test]
    fn test_needle_check_counters() {
        let needle = TheNeedle;
        let game = Game::new(Config::new());

        let counters = needle.check_counters(&game);
        assert_eq!(counters.len(), 1);
        assert_eq!(counters[0], CounterType::HandsPlayed);
    }

    #[test]
    fn test_needle_not_active_without_boss_blind() {
        let game = Game::new(Config::new());

        // Without activating the boss blind, it should not be active
        assert!(!TheNeedle::is_active(&game));
        assert!(TheNeedle::can_play_hand(&game));
        assert!(!TheNeedle::play_limit_reached(&game));
        // Should return large number when not active
        assert_eq!(TheNeedle::get_max_plays(&game), 999);
    }

    #[test]
    fn test_needle_deactivation() {
        let needle = TheNeedle;
        let mut game = Game::new(Config::new());

        // Activate The Needle
        game.boss_blind_state.activate(BossBlindId::TheNeedle);
        needle.apply_effects(&mut game);
        assert!(TheNeedle::is_active(&game));
        assert_eq!(game.plays, 1.0);

        // Deactivate boss blind
        game.boss_blind_state.deactivate();
        assert!(!TheNeedle::is_active(&game));
        assert!(TheNeedle::can_play_hand(&game));
    }
}
