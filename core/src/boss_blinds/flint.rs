//! The Flint Boss Blind Implementation
//!
//! The Flint halves base chips and mult for all played hands.
//! This is a minimum ante 2 boss blind with 2x score requirement.
//!
//! ## Mechanics
//! - All base chips are halved during scoring
//! - All base mult is halved during scoring
//! - Joker effects are not affected, only base scoring values
//! - Minimum ante: 2
//! - Score requirement: 2x base chips

use crate::boss_blinds::{BlindEffect, BossBlind, CounterType};
use crate::game::Game;

/// The Flint Boss Blind
///
/// Reduces all base chips and mult values by half during hand scoring.
/// This significantly reduces the effectiveness of base scoring while leaving joker effects intact.
#[derive(Debug)]
pub struct TheFlint;

impl BossBlind for TheFlint {
    fn name(&self) -> &str {
        "The Flint"
    }

    fn min_ante(&self) -> u32 {
        2
    }

    fn apply_effects(&self, game: &mut Game) {
        // Set custom state to track that halving is active
        game.boss_blind_state.set_custom_state(
            "halving_active".to_string(),
            crate::boss_blinds::BossBlindData::Boolean(true),
        );

        // Store the halving multiplier
        game.boss_blind_state.set_custom_state(
            "chips_multiplier".to_string(),
            crate::boss_blinds::BossBlindData::Float(0.5),
        );

        game.boss_blind_state.set_custom_state(
            "mult_multiplier".to_string(),
            crate::boss_blinds::BossBlindData::Float(0.5),
        );
    }

    fn get_effects(&self) -> Vec<BlindEffect> {
        vec![BlindEffect::ModifyScoring(
            "Base chips and mult halved".to_string(),
        )]
    }

    fn check_counters(&self, _game: &Game) -> Vec<CounterType> {
        // The Flint needs to track hands played to apply halving during scoring
        vec![CounterType::HandsPlayed, CounterType::CardsScored]
    }
}

impl TheFlint {
    /// Check if The Flint boss blind is currently active
    pub fn is_active(game: &Game) -> bool {
        if let Some(active_boss) = game.boss_blind_state.active_boss() {
            if matches!(active_boss, crate::boss_blinds::BossBlindId::TheFlint) {
                if let Some(crate::boss_blinds::BossBlindData::Boolean(true)) =
                    game.boss_blind_state.get_custom_state("halving_active")
                {
                    return true;
                }
            }
        }
        false
    }

    /// Get the chips multiplier applied by The Flint (should be 0.5)
    pub fn get_chips_multiplier(game: &Game) -> f64 {
        if Self::is_active(game) {
            if let Some(crate::boss_blinds::BossBlindData::Float(multiplier)) =
                game.boss_blind_state.get_custom_state("chips_multiplier")
            {
                return *multiplier;
            }
        }
        1.0 // No modification if not active
    }

    /// Get the mult multiplier applied by The Flint (should be 0.5)
    pub fn get_mult_multiplier(game: &Game) -> f64 {
        if Self::is_active(game) {
            if let Some(crate::boss_blinds::BossBlindData::Float(multiplier)) =
                game.boss_blind_state.get_custom_state("mult_multiplier")
            {
                return *multiplier;
            }
        }
        1.0 // No modification if not active
    }

    /// Apply The Flint's halving effect to base chips
    pub fn apply_chips_halving(game: &Game, base_chips: f64) -> f64 {
        if Self::is_active(game) {
            return base_chips * Self::get_chips_multiplier(game);
        }
        base_chips
    }

    /// Apply The Flint's halving effect to base mult
    pub fn apply_mult_halving(game: &Game, base_mult: f64) -> f64 {
        if Self::is_active(game) {
            return base_mult * Self::get_mult_multiplier(game);
        }
        base_mult
    }

    /// Check if scoring values should be halved
    pub fn should_halve_scoring(game: &Game) -> bool {
        Self::is_active(game)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::boss_blinds::BossBlindId;
    use crate::config::Config;

    #[test]
    fn test_flint_boss_blind_properties() {
        let flint = TheFlint;

        assert_eq!(flint.name(), "The Flint");
        assert_eq!(flint.min_ante(), 2);

        let effects = flint.get_effects();
        assert_eq!(effects.len(), 1);
        assert_eq!(
            effects[0],
            BlindEffect::ModifyScoring("Base chips and mult halved".to_string())
        );
    }

    #[test]
    fn test_flint_apply_effects() {
        let flint = TheFlint;
        let mut game = Game::new(Config::new());

        // Initially, The Flint should not be active
        assert!(!TheFlint::is_active(&game));
        assert_eq!(TheFlint::get_chips_multiplier(&game), 1.0);
        assert_eq!(TheFlint::get_mult_multiplier(&game), 1.0);

        // Activate The Flint
        game.boss_blind_state.activate(BossBlindId::TheFlint);
        flint.apply_effects(&mut game);

        // Now The Flint should be active
        assert!(TheFlint::is_active(&game));
        assert_eq!(TheFlint::get_chips_multiplier(&game), 0.5);
        assert_eq!(TheFlint::get_mult_multiplier(&game), 0.5);
        assert!(TheFlint::should_halve_scoring(&game));
    }

    #[test]
    fn test_flint_scoring_modifications() {
        let flint = TheFlint;
        let mut game = Game::new(Config::new());

        // Test values
        let base_chips = 100.0;
        let base_mult = 4.0;

        // Without The Flint active, values should not change
        assert_eq!(TheFlint::apply_chips_halving(&game, base_chips), base_chips);
        assert_eq!(TheFlint::apply_mult_halving(&game, base_mult), base_mult);

        // Activate The Flint
        game.boss_blind_state.activate(BossBlindId::TheFlint);
        flint.apply_effects(&mut game);

        // Now values should be halved
        assert_eq!(TheFlint::apply_chips_halving(&game, base_chips), 50.0);
        assert_eq!(TheFlint::apply_mult_halving(&game, base_mult), 2.0);
    }

    #[test]
    fn test_flint_check_counters() {
        let flint = TheFlint;
        let game = Game::new(Config::new());

        let counters = flint.check_counters(&game);
        assert_eq!(counters.len(), 2);
        assert!(counters.contains(&CounterType::HandsPlayed));
        assert!(counters.contains(&CounterType::CardsScored));
    }

    #[test]
    fn test_flint_not_active_without_boss_blind() {
        let game = Game::new(Config::new());

        // Without activating the boss blind, it should not be active
        assert!(!TheFlint::is_active(&game));
        assert!(!TheFlint::should_halve_scoring(&game));
        assert_eq!(TheFlint::get_chips_multiplier(&game), 1.0);
        assert_eq!(TheFlint::get_mult_multiplier(&game), 1.0);
    }

    #[test]
    fn test_flint_deactivation() {
        let flint = TheFlint;
        let mut game = Game::new(Config::new());

        // Activate The Flint
        game.boss_blind_state.activate(BossBlindId::TheFlint);
        flint.apply_effects(&mut game);
        assert!(TheFlint::is_active(&game));

        // Deactivate boss blind
        game.boss_blind_state.deactivate();
        assert!(!TheFlint::is_active(&game));
        assert!(!TheFlint::should_halve_scoring(&game));
        assert_eq!(TheFlint::get_chips_multiplier(&game), 1.0);
        assert_eq!(TheFlint::get_mult_multiplier(&game), 1.0);
    }

    #[test]
    fn test_flint_various_scoring_values() {
        let flint = TheFlint;
        let mut game = Game::new(Config::new());

        // Activate The Flint
        game.boss_blind_state.activate(BossBlindId::TheFlint);
        flint.apply_effects(&mut game);

        // Test various values
        assert_eq!(TheFlint::apply_chips_halving(&game, 0.0), 0.0);
        assert_eq!(TheFlint::apply_chips_halving(&game, 10.0), 5.0);
        assert_eq!(TheFlint::apply_chips_halving(&game, 1.0), 0.5);

        assert_eq!(TheFlint::apply_mult_halving(&game, 0.0), 0.0);
        assert_eq!(TheFlint::apply_mult_halving(&game, 8.0), 4.0);
        assert_eq!(TheFlint::apply_mult_halving(&game, 3.0), 1.5);
    }
}
