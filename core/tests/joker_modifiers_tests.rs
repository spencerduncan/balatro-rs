//! Unit tests for the JokerModifiers trait
//!
//! This module tests all aspects of the JokerModifiers trait including:
//! - Default implementations
//! - Custom implementations
//! - Edge cases and boundary conditions
//! - Thread safety (Send + Sync)

use balatro_rs::joker::traits::JokerModifiers;

/// Test struct that uses default implementations
#[derive(Debug, Clone)]
struct DefaultModifierJoker;

impl JokerModifiers for DefaultModifierJoker {}

/// Test struct with custom chip multiplier
#[derive(Debug, Clone)]
struct ChipMultiplierJoker {
    chip_mult: f64,
}

impl ChipMultiplierJoker {
    fn new(chip_mult: f64) -> Self {
        Self { chip_mult }
    }
}

impl JokerModifiers for ChipMultiplierJoker {
    fn get_chip_mult(&self) -> f64 {
        self.chip_mult
    }
}

/// Test struct with custom score multiplier
#[derive(Debug, Clone)]
struct ScoreMultiplierJoker {
    score_mult: f64,
}

impl ScoreMultiplierJoker {
    fn new(score_mult: f64) -> Self {
        Self { score_mult }
    }
}

impl JokerModifiers for ScoreMultiplierJoker {
    fn get_score_mult(&self) -> f64 {
        self.score_mult
    }
}

/// Test struct with custom hand size modifier
#[derive(Debug, Clone)]
struct HandSizeModifierJoker {
    hand_size_mod: i32,
}

impl HandSizeModifierJoker {
    fn new(hand_size_mod: i32) -> Self {
        Self { hand_size_mod }
    }
}

impl JokerModifiers for HandSizeModifierJoker {
    fn get_hand_size_modifier(&self) -> i32 {
        self.hand_size_mod
    }
}

/// Test struct with custom discard modifier
#[derive(Debug, Clone)]
struct DiscardModifierJoker {
    discard_mod: i32,
}

impl DiscardModifierJoker {
    fn new(discard_mod: i32) -> Self {
        Self { discard_mod }
    }
}

impl JokerModifiers for DiscardModifierJoker {
    fn get_discard_modifier(&self) -> i32 {
        self.discard_mod
    }
}

/// Test struct with all modifiers customized
#[derive(Debug, Clone)]
struct AllModifiersJoker {
    chip_mult: f64,
    score_mult: f64,
    hand_size_mod: i32,
    discard_mod: i32,
}

impl AllModifiersJoker {
    fn new(chip_mult: f64, score_mult: f64, hand_size_mod: i32, discard_mod: i32) -> Self {
        Self {
            chip_mult,
            score_mult,
            hand_size_mod,
            discard_mod,
        }
    }
}

impl JokerModifiers for AllModifiersJoker {
    fn get_chip_mult(&self) -> f64 {
        self.chip_mult
    }

    fn get_score_mult(&self) -> f64 {
        self.score_mult
    }

    fn get_hand_size_modifier(&self) -> i32 {
        self.hand_size_mod
    }

    fn get_discard_modifier(&self) -> i32 {
        self.discard_mod
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_implementations() {
        let joker = DefaultModifierJoker;

        // Test default chip multiplier
        assert_eq!(joker.get_chip_mult(), 1.0);

        // Test default score multiplier
        assert_eq!(joker.get_score_mult(), 1.0);

        // Test default hand size modifier
        assert_eq!(joker.get_hand_size_modifier(), 0);

        // Test default discard modifier
        assert_eq!(joker.get_discard_modifier(), 0);
    }

    #[test]
    fn test_chip_multiplier_implementation() {
        // Test positive multiplier
        let joker_positive = ChipMultiplierJoker::new(2.5);
        assert_eq!(joker_positive.get_chip_mult(), 2.5);
        assert_eq!(joker_positive.get_score_mult(), 1.0); // Default
        assert_eq!(joker_positive.get_hand_size_modifier(), 0); // Default
        assert_eq!(joker_positive.get_discard_modifier(), 0); // Default

        // Test zero multiplier
        let joker_zero = ChipMultiplierJoker::new(0.0);
        assert_eq!(joker_zero.get_chip_mult(), 0.0);

        // Test fractional multiplier
        let joker_fractional = ChipMultiplierJoker::new(0.5);
        assert_eq!(joker_fractional.get_chip_mult(), 0.5);

        // Test large multiplier
        let joker_large = ChipMultiplierJoker::new(1000000.0);
        assert_eq!(joker_large.get_chip_mult(), 1000000.0);
    }

    #[test]
    fn test_score_multiplier_implementation() {
        // Test positive multiplier
        let joker_positive = ScoreMultiplierJoker::new(3.0);
        assert_eq!(joker_positive.get_score_mult(), 3.0);
        assert_eq!(joker_positive.get_chip_mult(), 1.0); // Default
        assert_eq!(joker_positive.get_hand_size_modifier(), 0); // Default
        assert_eq!(joker_positive.get_discard_modifier(), 0); // Default

        // Test zero multiplier
        let joker_zero = ScoreMultiplierJoker::new(0.0);
        assert_eq!(joker_zero.get_score_mult(), 0.0);

        // Test fractional multiplier
        let joker_fractional = ScoreMultiplierJoker::new(0.25);
        assert_eq!(joker_fractional.get_score_mult(), 0.25);

        // Test large multiplier
        let joker_large = ScoreMultiplierJoker::new(999999.99);
        assert_eq!(joker_large.get_score_mult(), 999999.99);
    }

    #[test]
    fn test_hand_size_modifier_implementation() {
        // Test positive modifier
        let joker_positive = HandSizeModifierJoker::new(5);
        assert_eq!(joker_positive.get_hand_size_modifier(), 5);
        assert_eq!(joker_positive.get_chip_mult(), 1.0); // Default
        assert_eq!(joker_positive.get_score_mult(), 1.0); // Default
        assert_eq!(joker_positive.get_discard_modifier(), 0); // Default

        // Test zero modifier
        let joker_zero = HandSizeModifierJoker::new(0);
        assert_eq!(joker_zero.get_hand_size_modifier(), 0);

        // Test negative modifier
        let joker_negative = HandSizeModifierJoker::new(-3);
        assert_eq!(joker_negative.get_hand_size_modifier(), -3);

        // Test large positive modifier
        let joker_large_positive = HandSizeModifierJoker::new(1000);
        assert_eq!(joker_large_positive.get_hand_size_modifier(), 1000);

        // Test large negative modifier
        let joker_large_negative = HandSizeModifierJoker::new(-1000);
        assert_eq!(joker_large_negative.get_hand_size_modifier(), -1000);
    }

    #[test]
    fn test_discard_modifier_implementation() {
        // Test positive modifier
        let joker_positive = DiscardModifierJoker::new(2);
        assert_eq!(joker_positive.get_discard_modifier(), 2);
        assert_eq!(joker_positive.get_chip_mult(), 1.0); // Default
        assert_eq!(joker_positive.get_score_mult(), 1.0); // Default
        assert_eq!(joker_positive.get_hand_size_modifier(), 0); // Default

        // Test zero modifier
        let joker_zero = DiscardModifierJoker::new(0);
        assert_eq!(joker_zero.get_discard_modifier(), 0);

        // Test negative modifier
        let joker_negative = DiscardModifierJoker::new(-1);
        assert_eq!(joker_negative.get_discard_modifier(), -1);

        // Test large positive modifier
        let joker_large_positive = DiscardModifierJoker::new(100);
        assert_eq!(joker_large_positive.get_discard_modifier(), 100);

        // Test large negative modifier
        let joker_large_negative = DiscardModifierJoker::new(-100);
        assert_eq!(joker_large_negative.get_discard_modifier(), -100);
    }

    #[test]
    fn test_all_modifiers_implementation() {
        let joker = AllModifiersJoker::new(2.5, 1.5, 3, -1);

        assert_eq!(joker.get_chip_mult(), 2.5);
        assert_eq!(joker.get_score_mult(), 1.5);
        assert_eq!(joker.get_hand_size_modifier(), 3);
        assert_eq!(joker.get_discard_modifier(), -1);
    }

    #[test]
    fn test_extreme_values() {
        // Test with extreme float values
        let extreme_joker = AllModifiersJoker::new(f64::MAX, f64::MIN_POSITIVE, i32::MAX, i32::MIN);

        assert_eq!(extreme_joker.get_chip_mult(), f64::MAX);
        assert_eq!(extreme_joker.get_score_mult(), f64::MIN_POSITIVE);
        assert_eq!(extreme_joker.get_hand_size_modifier(), i32::MAX);
        assert_eq!(extreme_joker.get_discard_modifier(), i32::MIN);
    }

    #[test]
    fn test_special_float_values() {
        // Test with infinity
        let infinity_joker = ChipMultiplierJoker::new(f64::INFINITY);
        assert_eq!(infinity_joker.get_chip_mult(), f64::INFINITY);

        // Test with negative infinity
        let neg_infinity_joker = ScoreMultiplierJoker::new(f64::NEG_INFINITY);
        assert_eq!(neg_infinity_joker.get_score_mult(), f64::NEG_INFINITY);

        // Test with NaN
        let nan_joker = ChipMultiplierJoker::new(f64::NAN);
        assert!(nan_joker.get_chip_mult().is_nan());
    }

    #[test]
    fn test_thread_safety() {
        // Test that JokerModifiers implementations are Send + Sync
        fn assert_send_sync<T: Send + Sync>() {}

        assert_send_sync::<DefaultModifierJoker>();
        assert_send_sync::<ChipMultiplierJoker>();
        assert_send_sync::<ScoreMultiplierJoker>();
        assert_send_sync::<HandSizeModifierJoker>();
        assert_send_sync::<DiscardModifierJoker>();
        assert_send_sync::<AllModifiersJoker>();
    }

    #[test]
    fn test_clone_behavior() {
        let original = AllModifiersJoker::new(2.0, 3.0, 1, -1);
        let cloned = original.clone();

        // Verify that cloned joker has same modifier values
        assert_eq!(original.get_chip_mult(), cloned.get_chip_mult());
        assert_eq!(original.get_score_mult(), cloned.get_score_mult());
        assert_eq!(
            original.get_hand_size_modifier(),
            cloned.get_hand_size_modifier()
        );
        assert_eq!(
            original.get_discard_modifier(),
            cloned.get_discard_modifier()
        );
    }

    #[test]
    fn test_mutable_modifier_changes() {
        let mut joker = ChipMultiplierJoker::new(1.0);
        assert_eq!(joker.get_chip_mult(), 1.0);

        // Modify the joker
        joker.chip_mult = 5.0;
        assert_eq!(joker.get_chip_mult(), 5.0);
    }

    #[test]
    fn test_zero_and_negative_multipliers() {
        // Zero multipliers could represent disabled effects
        let zero_chip = ChipMultiplierJoker::new(0.0);
        let zero_score = ScoreMultiplierJoker::new(0.0);

        assert_eq!(zero_chip.get_chip_mult(), 0.0);
        assert_eq!(zero_score.get_score_mult(), 0.0);

        // Negative multipliers could represent penalties
        let negative_chip = ChipMultiplierJoker::new(-0.5);
        let negative_score = ScoreMultiplierJoker::new(-2.0);

        assert_eq!(negative_chip.get_chip_mult(), -0.5);
        assert_eq!(negative_score.get_score_mult(), -2.0);
    }

    #[test]
    fn test_precision_edge_cases() {
        // Test very small positive values
        let tiny_positive = ChipMultiplierJoker::new(f64::EPSILON);
        assert_eq!(tiny_positive.get_chip_mult(), f64::EPSILON);

        // Test values just above and below 1.0
        let just_above_one = ScoreMultiplierJoker::new(1.0 + f64::EPSILON);
        let just_below_one = ScoreMultiplierJoker::new(1.0 - f64::EPSILON);

        assert_eq!(just_above_one.get_score_mult(), 1.0 + f64::EPSILON);
        assert_eq!(just_below_one.get_score_mult(), 1.0 - f64::EPSILON);
    }

    #[test]
    fn test_multiple_jokers_independence() {
        let joker1 = ChipMultiplierJoker::new(2.0);
        let joker2 = ChipMultiplierJoker::new(3.0);
        let joker3 = ScoreMultiplierJoker::new(4.0);

        // Verify each joker maintains its own state
        assert_eq!(joker1.get_chip_mult(), 2.0);
        assert_eq!(joker2.get_chip_mult(), 3.0);
        assert_eq!(joker3.get_score_mult(), 4.0);

        // Verify defaults are maintained for non-overridden methods
        assert_eq!(joker1.get_score_mult(), 1.0);
        assert_eq!(joker2.get_hand_size_modifier(), 0);
        assert_eq!(joker3.get_chip_mult(), 1.0);
    }

    /// Test that demonstrates how jokers with different modifiers could work together
    #[test]
    fn test_modifier_combination_scenarios() {
        let chip_joker = ChipMultiplierJoker::new(2.0);
        let score_joker = ScoreMultiplierJoker::new(1.5);
        let hand_joker = HandSizeModifierJoker::new(2);
        let discard_joker = DiscardModifierJoker::new(-1);

        // Simulate applying all modifiers (this is conceptual - actual implementation
        // would be in the game engine)
        let base_chips = 100.0;
        let base_score = 50.0;
        let base_hand_size = 8;
        let base_discards = 3;

        let modified_chips = base_chips * chip_joker.get_chip_mult();
        let modified_score = base_score * score_joker.get_score_mult();
        let modified_hand_size = base_hand_size + hand_joker.get_hand_size_modifier();
        let modified_discards = base_discards + discard_joker.get_discard_modifier();

        assert_eq!(modified_chips, 200.0);
        assert_eq!(modified_score, 75.0);
        assert_eq!(modified_hand_size, 10);
        assert_eq!(modified_discards, 2);
    }
}
