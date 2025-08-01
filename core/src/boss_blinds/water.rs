//! The Water Boss Blind Implementation
//!
//! The Water forces the player to start with 0 discards available.
//! This is a minimum ante 2 boss blind with 2x score requirement.
//!
//! ## Mechanics
//! - Player starts the blind with 0 discards instead of the usual 3
//! - Must make do with the cards dealt initially
//! - Forces careful hand management without discard flexibility
//! - Minimum ante: 2
//! - Score requirement: 2x base chips

use crate::boss_blinds::{BlindEffect, BossBlind, CounterType};
use crate::game::Game;

/// The Water Boss Blind
///
/// Removes all discard actions by setting discards to 0 at the start of the blind.
/// This forces players to work with their initial hand and draws without the safety net of discards.
#[derive(Debug)]
pub struct TheWater;

impl BossBlind for TheWater {
    fn name(&self) -> &str {
        "The Water"
    }

    fn min_ante(&self) -> u32 {
        2
    }

    fn apply_effects(&self, game: &mut Game) {
        // Set discards to 0
        game.discards = 0.0;

        // Set custom state to track that no discards are allowed
        game.boss_blind_state.set_custom_state(
            "no_discards_active".to_string(),
            crate::boss_blinds::BossBlindData::Boolean(true),
        );

        // Store the original discard count would have been
        game.boss_blind_state.set_custom_state(
            "original_discards".to_string(),
            crate::boss_blinds::BossBlindData::Integer(0),
        );
    }

    fn get_effects(&self) -> Vec<BlindEffect> {
        vec![BlindEffect::RestrictActions(
            "Start with 0 discards".to_string(),
        )]
    }

    fn check_counters(&self, _game: &Game) -> Vec<CounterType> {
        // The Water needs to track discards to ensure they remain at 0
        vec![CounterType::CardsDiscarded]
    }
}

impl TheWater {
    /// Check if The Water boss blind is currently active
    pub fn is_active(game: &Game) -> bool {
        if let Some(active_boss) = game.boss_blind_state.active_boss() {
            if matches!(active_boss, crate::boss_blinds::BossBlindId::TheWater) {
                if let Some(crate::boss_blinds::BossBlindData::Boolean(true)) =
                    game.boss_blind_state.get_custom_state("no_discards_active")
                {
                    return true;
                }
            }
        }
        false
    }

    /// Check if discard actions are blocked by The Water
    pub fn are_discards_blocked(game: &Game) -> bool {
        Self::is_active(game)
    }

    /// Check if the player can discard cards (should always be false when The Water is active)
    pub fn can_discard(game: &Game) -> bool {
        if Self::is_active(game) {
            return false;
        }
        // If not active, check normal discard availability
        game.discards > 0.0
    }

    /// Get the number of discards available (should be 0 when The Water is active)
    pub fn get_available_discards(game: &Game) -> f64 {
        if Self::is_active(game) {
            return 0.0;
        }
        game.discards
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::boss_blinds::BossBlindId;
    use crate::config::Config;

    #[test]
    fn test_water_boss_blind_properties() {
        let water = TheWater;

        assert_eq!(water.name(), "The Water");
        assert_eq!(water.min_ante(), 2);

        let effects = water.get_effects();
        assert_eq!(effects.len(), 1);
        assert_eq!(
            effects[0],
            BlindEffect::RestrictActions("Start with 0 discards".to_string())
        );
    }

    #[test]
    fn test_water_apply_effects() {
        let water = TheWater;
        let mut game = Game::new(Config::new());

        // Store initial discards (should be more than 0)
        let initial_discards = game.discards;
        assert!(initial_discards > 0.0);

        // Initially, The Water should not be active
        assert!(!TheWater::is_active(&game));
        assert!(!TheWater::are_discards_blocked(&game));
        assert!(TheWater::can_discard(&game));

        // Activate The Water
        game.boss_blind_state.activate(BossBlindId::TheWater);
        water.apply_effects(&mut game);

        // Now The Water should be active and discards should be 0
        assert!(TheWater::is_active(&game));
        assert_eq!(game.discards, 0.0);
        assert!(TheWater::are_discards_blocked(&game));
        assert!(!TheWater::can_discard(&game));
        assert_eq!(TheWater::get_available_discards(&game), 0.0);
    }

    #[test]
    fn test_water_discard_blocking() {
        let water = TheWater;
        let mut game = Game::new(Config::new());

        // Set some discards initially
        game.discards = 3.0;

        // Activate The Water
        game.boss_blind_state.activate(BossBlindId::TheWater);
        water.apply_effects(&mut game);

        // Discards should now be blocked
        assert!(TheWater::are_discards_blocked(&game));
        assert!(!TheWater::can_discard(&game));
        assert_eq!(game.discards, 0.0);
        assert_eq!(TheWater::get_available_discards(&game), 0.0);
    }

    #[test]
    fn test_water_check_counters() {
        let water = TheWater;
        let game = Game::new(Config::new());

        let counters = water.check_counters(&game);
        assert_eq!(counters.len(), 1);
        assert_eq!(counters[0], CounterType::CardsDiscarded);
    }

    #[test]
    fn test_water_not_active_without_boss_blind() {
        let mut game = Game::new(Config::new());

        // Set some discards
        game.discards = 3.0;

        // Without activating the boss blind, it should not be active
        assert!(!TheWater::is_active(&game));
        assert!(!TheWater::are_discards_blocked(&game));
        assert!(TheWater::can_discard(&game));
        assert_eq!(TheWater::get_available_discards(&game), 3.0);
    }

    #[test]
    fn test_water_deactivation() {
        let water = TheWater;
        let mut game = Game::new(Config::new());

        // Set some initial discards
        game.discards = 3.0;

        // Activate The Water
        game.boss_blind_state.activate(BossBlindId::TheWater);
        water.apply_effects(&mut game);
        assert!(TheWater::is_active(&game));
        assert_eq!(game.discards, 0.0);

        // Deactivate boss blind
        game.boss_blind_state.deactivate();
        assert!(!TheWater::is_active(&game));
        assert!(!TheWater::are_discards_blocked(&game));

        // Note: game.discards will still be 0.0 because deactivation doesn't restore the original value
        // In a real implementation, the game would need to handle restoration of discards
    }

    #[test]
    fn test_water_with_zero_initial_discards() {
        let water = TheWater;
        let mut game = Game::new(Config::new());

        // Start with 0 discards
        game.discards = 0.0;

        // Activate The Water
        game.boss_blind_state.activate(BossBlindId::TheWater);
        water.apply_effects(&mut game);

        // Should still work correctly
        assert!(TheWater::is_active(&game));
        assert_eq!(game.discards, 0.0);
        assert!(!TheWater::can_discard(&game));
    }
}
