#[cfg(feature = "python")]
use pyo3::prelude::*;

const DEFAULT_ROUND_START: usize = 0;
const DEFAULT_PLAYS: usize = 4;
const DEFAULT_DISCARDS: usize = 4;
const DEFAULT_MONEY_START: usize = 0;
const DEFAULT_MONEY_MAX: usize = 500;
const DEFAULT_REWARD_BASE: usize = 0;
const DEFAULT_MONEY_PER_HAND: usize = 1;
const DEFAULT_INTEREST_RATE: f64 = 0.2;
const DEFAULT_INTEREST_MAX: usize = 5;
const DEFAULT_BASE_MULT: usize = 0;
const DEFAULT_BASE_CHIPS: usize = 0;
const DEFAULT_BASE_SCORE: usize = 0;
const DEFAULT_ANTE_START: usize = 1;
const DEFAULT_ANTE_END: usize = 8;
const DEFAULT_JOKER_SLOTS: usize = 5;
const DEFAULT_JOKER_SLOTS_MAX: usize = 10;
const DEFAULT_AVAILABLE: usize = 8;
const DEFAULT_AVAILABLE_MAX: usize = 24; // arbitrary
const DEFAULT_STORE_CONSUMABLE_SLOTS_MAX: usize = 4;
const DEFAULT_CONSUMABLE_HAND_CAPACITY: usize = 2;
const DEFAULT_DECK_MAX: usize = 100;
const DEFAULT_DISCARDED_MAX: usize = 100;
const DEFAULT_SELECTED_MAX: usize = 5;

// Pack system configuration defaults
const DEFAULT_PACK_STANDARD_COST: usize = 4;
const DEFAULT_PACK_JUMBO_COST: usize = 6;
const DEFAULT_PACK_MEGA_COST: usize = 8;
const DEFAULT_PACK_ENHANCED_COST: usize = 6;
const DEFAULT_PACK_VARIETY_COST: usize = 5;
const DEFAULT_PACK_BUFFOON_COST: usize = 4;
const DEFAULT_PACK_CONSUMABLE_COST: usize = 4; // For Arcana, Celestial, Spectral
const DEFAULT_PACK_MEGA_CONSUMABLE_COST: usize = 8; // For Mega variants

const DEFAULT_ENHANCEMENT_RATE: f64 = 0.1; // 10% chance for card enhancement

// Joker rarity weight defaults (out of 100)
const DEFAULT_JOKER_RARITY_WEIGHT_COMMON: u32 = 70;
const DEFAULT_JOKER_RARITY_WEIGHT_UNCOMMON: u32 = 25;
const DEFAULT_JOKER_RARITY_WEIGHT_RARE: u32 = 5;

// Pack option count defaults (min, max)
const DEFAULT_PACK_STANDARD_OPTIONS: (usize, usize) = (3, 3);
const DEFAULT_PACK_JUMBO_OPTIONS: (usize, usize) = (5, 5);
const DEFAULT_PACK_MEGA_OPTIONS: (usize, usize) = (7, 7);
const DEFAULT_PACK_ENHANCED_OPTIONS: (usize, usize) = (3, 4);
const DEFAULT_PACK_VARIETY_OPTIONS: (usize, usize) = (3, 5);
const DEFAULT_PACK_BUFFOON_OPTIONS: (usize, usize) = (2, 2);
const DEFAULT_PACK_CONSUMABLE_OPTIONS: (usize, usize) = (2, 3); // For Arcana, Celestial, Spectral
const DEFAULT_PACK_MEGA_BUFFOON_OPTIONS: (usize, usize) = (4, 4);
const DEFAULT_PACK_MEGA_CONSUMABLE_OPTIONS: (usize, usize) = (4, 6); // For Mega variants

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "python", pyclass)]
#[derive(Debug, Clone)]
pub struct Config {
    pub round_start: usize,
    pub plays: usize,
    pub discards: usize,
    pub money_start: usize,
    pub money_max: usize,
    pub reward_base: usize,
    pub money_per_hand: usize,
    pub interest_rate: f64,
    pub interest_max: usize,
    pub base_mult: usize,
    pub base_chips: usize,
    pub base_score: usize,
    pub ante_start: usize,
    pub ante_end: usize,
    pub joker_slots: usize,
    pub joker_slots_max: usize,
    pub selected_max: usize,
    pub available: usize,
    pub available_max: usize,
    pub store_consumable_slots_max: usize,
    pub consumable_hand_capacity: usize,
    pub deck_max: usize,
    pub discarded_max: usize,

    // Pack system configuration
    pub pack_standard_cost: usize,
    pub pack_jumbo_cost: usize,
    pub pack_mega_cost: usize,
    pub pack_enhanced_cost: usize,
    pub pack_variety_cost: usize,
    pub pack_buffoon_cost: usize,
    pub pack_consumable_cost: usize,
    pub pack_mega_consumable_cost: usize,

    pub enhancement_rate: f64,

    pub joker_rarity_weight_common: u32,
    pub joker_rarity_weight_uncommon: u32,
    pub joker_rarity_weight_rare: u32,

    pub pack_standard_options: (usize, usize),
    pub pack_jumbo_options: (usize, usize),
    pub pack_mega_options: (usize, usize),
    pub pack_enhanced_options: (usize, usize),
    pub pack_variety_options: (usize, usize),
    pub pack_buffoon_options: (usize, usize),
    pub pack_consumable_options: (usize, usize),
    pub pack_mega_buffoon_options: (usize, usize),
    pub pack_mega_consumable_options: (usize, usize),
}

impl Config {
    pub fn new() -> Self {
        Config {
            round_start: DEFAULT_ROUND_START,
            plays: DEFAULT_PLAYS,
            discards: DEFAULT_DISCARDS,
            money_start: DEFAULT_MONEY_START,
            money_max: DEFAULT_MONEY_MAX,
            reward_base: DEFAULT_REWARD_BASE,
            money_per_hand: DEFAULT_MONEY_PER_HAND,
            interest_rate: DEFAULT_INTEREST_RATE,
            interest_max: DEFAULT_INTEREST_MAX,
            base_mult: DEFAULT_BASE_MULT,
            base_chips: DEFAULT_BASE_CHIPS,
            base_score: DEFAULT_BASE_SCORE,
            ante_start: DEFAULT_ANTE_START,
            ante_end: DEFAULT_ANTE_END,
            joker_slots: DEFAULT_JOKER_SLOTS,
            joker_slots_max: DEFAULT_JOKER_SLOTS_MAX,
            selected_max: DEFAULT_SELECTED_MAX,
            available: DEFAULT_AVAILABLE,
            available_max: DEFAULT_AVAILABLE_MAX,
            store_consumable_slots_max: DEFAULT_STORE_CONSUMABLE_SLOTS_MAX,
            consumable_hand_capacity: DEFAULT_CONSUMABLE_HAND_CAPACITY,
            deck_max: DEFAULT_DECK_MAX,
            discarded_max: DEFAULT_DISCARDED_MAX,

            // Pack system defaults
            pack_standard_cost: DEFAULT_PACK_STANDARD_COST,
            pack_jumbo_cost: DEFAULT_PACK_JUMBO_COST,
            pack_mega_cost: DEFAULT_PACK_MEGA_COST,
            pack_enhanced_cost: DEFAULT_PACK_ENHANCED_COST,
            pack_variety_cost: DEFAULT_PACK_VARIETY_COST,
            pack_buffoon_cost: DEFAULT_PACK_BUFFOON_COST,
            pack_consumable_cost: DEFAULT_PACK_CONSUMABLE_COST,
            pack_mega_consumable_cost: DEFAULT_PACK_MEGA_CONSUMABLE_COST,

            enhancement_rate: DEFAULT_ENHANCEMENT_RATE,

            joker_rarity_weight_common: DEFAULT_JOKER_RARITY_WEIGHT_COMMON,
            joker_rarity_weight_uncommon: DEFAULT_JOKER_RARITY_WEIGHT_UNCOMMON,
            joker_rarity_weight_rare: DEFAULT_JOKER_RARITY_WEIGHT_RARE,

            pack_standard_options: DEFAULT_PACK_STANDARD_OPTIONS,
            pack_jumbo_options: DEFAULT_PACK_JUMBO_OPTIONS,
            pack_mega_options: DEFAULT_PACK_MEGA_OPTIONS,
            pack_enhanced_options: DEFAULT_PACK_ENHANCED_OPTIONS,
            pack_variety_options: DEFAULT_PACK_VARIETY_OPTIONS,
            pack_buffoon_options: DEFAULT_PACK_BUFFOON_OPTIONS,
            pack_consumable_options: DEFAULT_PACK_CONSUMABLE_OPTIONS,
            pack_mega_buffoon_options: DEFAULT_PACK_MEGA_BUFFOON_OPTIONS,
            pack_mega_consumable_options: DEFAULT_PACK_MEGA_CONSUMABLE_OPTIONS,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "python")]
#[pymethods]
impl Config {
    #[new]
    fn py_new() -> Self {
        Self::default()
    }

    #[getter]
    fn get_ante_end(&mut self) -> usize {
        self.ante_end
    }

    #[setter]
    fn set_ante_end(&mut self, i: usize) {
        self.ante_end = i;
    }

    #[getter]
    fn get_plays(&mut self) -> usize {
        self.plays
    }

    #[setter]
    fn set_plays(&mut self, i: usize) {
        self.plays = i;
    }

    #[getter]
    fn get_discards(&mut self) -> usize {
        self.discards
    }

    #[setter]
    fn set_discards(&mut self, i: usize) {
        self.discards = i;
    }

    #[getter]
    fn get_selected_max(&mut self) -> usize {
        self.selected_max
    }

    #[setter]
    fn set_selected_max(&mut self, i: usize) {
        self.selected_max = i;
    }

    #[getter]
    fn get_deck_max(&mut self) -> usize {
        self.deck_max
    }

    #[setter]
    fn set_deck_max(&mut self, i: usize) {
        self.deck_max = i;
    }

    #[getter]
    fn get_discarded_max(&mut self) -> usize {
        self.discarded_max
    }

    #[setter]
    fn set_discarded_max(&mut self, i: usize) {
        self.discarded_max = i;
    }

    #[getter]
    fn get_available_max(&mut self) -> usize {
        self.available_max
    }

    #[setter]
    fn set_available_max(&mut self, i: usize) {
        use crate::math_safe::validate_array_size;

        // Validate that the size won't cause security issues
        validate_array_size(i, "available_max").expect("Invalid available_max value");

        self.available_max = i;
    }

    #[getter]
    fn get_joker_slots(&mut self) -> usize {
        self.joker_slots
    }

    #[setter]
    fn set_joker_slots(&mut self, i: usize) {
        self.joker_slots = i;
    }

    #[getter]
    fn get_joker_slots_max(&mut self) -> usize {
        self.joker_slots_max
    }

    #[setter]
    fn set_joker_slots_max(&mut self, i: usize) {
        self.joker_slots_max = i;
    }

    #[getter]
    fn get_money_max(&mut self) -> usize {
        self.money_max
    }

    #[setter]
    fn set_money_max(&mut self, i: usize) {
        self.money_max = i;
    }
    #[getter]
    fn get_stage_max(&self) -> usize {
        8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_new() {
        let config = Config::new();

        // Test all fields match expected defaults
        assert_eq!(config.round_start, DEFAULT_ROUND_START);
        assert_eq!(config.plays, DEFAULT_PLAYS);
        assert_eq!(config.discards, DEFAULT_DISCARDS);
        assert_eq!(config.money_start, DEFAULT_MONEY_START);
        assert_eq!(config.money_max, DEFAULT_MONEY_MAX);
        assert_eq!(config.reward_base, DEFAULT_REWARD_BASE);
        assert_eq!(config.money_per_hand, DEFAULT_MONEY_PER_HAND);
        assert_eq!(config.interest_rate, DEFAULT_INTEREST_RATE);
        assert_eq!(config.interest_max, DEFAULT_INTEREST_MAX);
        assert_eq!(config.base_mult, DEFAULT_BASE_MULT);
        assert_eq!(config.base_chips, DEFAULT_BASE_CHIPS);
        assert_eq!(config.base_score, DEFAULT_BASE_SCORE);
        assert_eq!(config.ante_start, DEFAULT_ANTE_START);
        assert_eq!(config.ante_end, DEFAULT_ANTE_END);
        assert_eq!(config.joker_slots, DEFAULT_JOKER_SLOTS);
        assert_eq!(config.joker_slots_max, DEFAULT_JOKER_SLOTS_MAX);
        assert_eq!(config.selected_max, DEFAULT_SELECTED_MAX);
        assert_eq!(config.available, DEFAULT_AVAILABLE);
        assert_eq!(config.available_max, DEFAULT_AVAILABLE_MAX);
        assert_eq!(
            config.store_consumable_slots_max,
            DEFAULT_STORE_CONSUMABLE_SLOTS_MAX
        );
        assert_eq!(
            config.consumable_hand_capacity,
            DEFAULT_CONSUMABLE_HAND_CAPACITY
        );
        assert_eq!(config.deck_max, DEFAULT_DECK_MAX);
        assert_eq!(config.discarded_max, DEFAULT_DISCARDED_MAX);
    }

    #[test]
    fn test_config_default() {
        let config = Config::default();
        let new_config = Config::new();

        // Default should be identical to new()
        assert_eq!(config.round_start, new_config.round_start);
        assert_eq!(config.plays, new_config.plays);
        assert_eq!(config.discards, new_config.discards);
        assert_eq!(config.money_start, new_config.money_start);
        assert_eq!(config.money_max, new_config.money_max);
        assert_eq!(config.reward_base, new_config.reward_base);
        assert_eq!(config.money_per_hand, new_config.money_per_hand);
        assert_eq!(config.interest_rate, new_config.interest_rate);
        assert_eq!(config.interest_max, new_config.interest_max);
        assert_eq!(config.base_mult, new_config.base_mult);
        assert_eq!(config.base_chips, new_config.base_chips);
        assert_eq!(config.base_score, new_config.base_score);
        assert_eq!(config.ante_start, new_config.ante_start);
        assert_eq!(config.ante_end, new_config.ante_end);
        assert_eq!(config.joker_slots, new_config.joker_slots);
        assert_eq!(config.joker_slots_max, new_config.joker_slots_max);
        assert_eq!(config.selected_max, new_config.selected_max);
        assert_eq!(config.available, new_config.available);
        assert_eq!(config.available_max, new_config.available_max);
        assert_eq!(
            config.store_consumable_slots_max,
            new_config.store_consumable_slots_max
        );
        assert_eq!(
            config.consumable_hand_capacity,
            new_config.consumable_hand_capacity
        );
        assert_eq!(config.deck_max, new_config.deck_max);
        assert_eq!(config.discarded_max, new_config.discarded_max);
    }

    #[test]
    fn test_default_constants() {
        // Test that all constants have reasonable values
        assert_eq!(DEFAULT_ROUND_START, 0);
        assert_eq!(DEFAULT_PLAYS, 4);
        assert_eq!(DEFAULT_DISCARDS, 4);
        assert_eq!(DEFAULT_MONEY_START, 0);
        assert_eq!(DEFAULT_MONEY_MAX, 500);
        assert_eq!(DEFAULT_REWARD_BASE, 0);
        assert_eq!(DEFAULT_MONEY_PER_HAND, 1);
        assert_eq!(DEFAULT_INTEREST_RATE, 0.2);
        assert_eq!(DEFAULT_INTEREST_MAX, 5);
        assert_eq!(DEFAULT_BASE_MULT, 0);
        assert_eq!(DEFAULT_BASE_CHIPS, 0);
        assert_eq!(DEFAULT_BASE_SCORE, 0);
        assert_eq!(DEFAULT_ANTE_START, 1);
        assert_eq!(DEFAULT_ANTE_END, 8);
        assert_eq!(DEFAULT_JOKER_SLOTS, 5);
        assert_eq!(DEFAULT_JOKER_SLOTS_MAX, 10);
        assert_eq!(DEFAULT_AVAILABLE, 8);
        assert_eq!(DEFAULT_AVAILABLE_MAX, 24);
        assert_eq!(DEFAULT_STORE_CONSUMABLE_SLOTS_MAX, 4);
        assert_eq!(DEFAULT_CONSUMABLE_HAND_CAPACITY, 2);
        assert_eq!(DEFAULT_DECK_MAX, 100);
        assert_eq!(DEFAULT_DISCARDED_MAX, 100);
        assert_eq!(DEFAULT_SELECTED_MAX, 5);
    }

    #[test]
    fn test_config_boundary_values() {
        let config = Config::new();

        // Test boundary conditions and logical constraints
        assert!(config.ante_start <= config.ante_end);
        assert!(config.joker_slots <= config.joker_slots_max);
        assert!(config.available <= config.available_max);
        assert!(config.money_start <= config.money_max);
        assert!(config.selected_max <= config.available);

        // Test reasonable minimums
        assert!(config.plays > 0);
        assert!(config.discards > 0);
        assert!(config.joker_slots > 0);
        assert!(config.available > 0);
        assert!(config.selected_max > 0);

        // Test reasonable maximums
        assert!(config.money_max < 1000); // Not too high
        assert!(config.deck_max >= 52); // At least a full deck
        assert!(config.available_max >= config.available);
    }

    #[test]
    fn test_config_interest_rate_bounds() {
        let config = Config::new();

        // Interest rate should be between 0 and 1 (0% to 100%)
        assert!(config.interest_rate >= 0.0);
        assert!(config.interest_rate <= 1.0);

        // Specific check for the default value
        assert_eq!(config.interest_rate, 0.2); // 20%
    }

    #[test]
    fn test_config_clone() {
        let config1 = Config::new();
        let config2 = config1.clone();

        // Cloned config should be identical
        assert_eq!(config1.plays, config2.plays);
        assert_eq!(config1.discards, config2.discards);
        assert_eq!(config1.money_max, config2.money_max);
        assert_eq!(config1.joker_slots, config2.joker_slots);
        assert_eq!(config1.interest_rate, config2.interest_rate);
    }

    #[test]
    fn test_config_debug() {
        let config = Config::new();
        let debug_str = format!("{config:?}");

        // Debug output should contain key fields
        assert!(debug_str.contains("Config"));
        assert!(debug_str.contains("plays"));
        assert!(debug_str.contains("discards"));
        assert!(debug_str.contains("money_max"));
    }

    #[cfg(feature = "python")]
    #[test]
    fn test_python_bindings() {
        let mut config = Config::py_new();

        // Test getters
        assert_eq!(config.get_ante_end(), DEFAULT_ANTE_END);
        assert_eq!(config.get_plays(), DEFAULT_PLAYS);
        assert_eq!(config.get_discards(), DEFAULT_DISCARDS);
        assert_eq!(config.get_selected_max(), DEFAULT_SELECTED_MAX);
        assert_eq!(config.get_deck_max(), DEFAULT_DECK_MAX);
        assert_eq!(config.get_discarded_max(), DEFAULT_DISCARDED_MAX);
        assert_eq!(config.get_available_max(), DEFAULT_AVAILABLE_MAX);
        assert_eq!(config.get_joker_slots(), DEFAULT_JOKER_SLOTS);
        assert_eq!(config.get_joker_slots_max(), DEFAULT_JOKER_SLOTS_MAX);
        assert_eq!(config.get_money_max(), DEFAULT_MONEY_MAX);
        assert_eq!(config.get_stage_max(), 8);

        // Test setters
        config.set_ante_end(10);
        assert_eq!(config.get_ante_end(), 10);

        config.set_plays(6);
        assert_eq!(config.get_plays(), 6);

        config.set_discards(3);
        assert_eq!(config.get_discards(), 3);

        config.set_selected_max(7);
        assert_eq!(config.get_selected_max(), 7);

        config.set_deck_max(150);
        assert_eq!(config.get_deck_max(), 150);

        config.set_discarded_max(200);
        assert_eq!(config.get_discarded_max(), 200);

        config.set_available_max(30);
        assert_eq!(config.get_available_max(), 30);

        config.set_joker_slots(8);
        assert_eq!(config.get_joker_slots(), 8);

        config.set_joker_slots_max(15);
        assert_eq!(config.get_joker_slots_max(), 15);

        config.set_money_max(1000);
        assert_eq!(config.get_money_max(), 1000);
    }

    #[cfg(feature = "python")]
    #[test]
    fn test_python_boundary_validation() {
        let mut config = Config::py_new();

        // Test setting extreme values (should work since no validation exists yet)
        config.set_plays(0);
        assert_eq!(config.get_plays(), 0);

        config.set_discards(1000);
        assert_eq!(config.get_discards(), 1000);

        // Test that stage_max is always 8 regardless of other settings
        assert_eq!(config.get_stage_max(), 8);
        config.set_ante_end(20);
        assert_eq!(config.get_stage_max(), 8); // Should still be 8
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialization() {
        let config = Config::new();

        // Test JSON serialization
        let json = serde_json::to_string(&config).expect("Should serialize to JSON");
        assert!(json.contains("plays"));
        assert!(json.contains("discards"));
        assert!(json.contains("money_max"));

        // Test deserialization
        let deserialized: Config =
            serde_json::from_str(&json).expect("Should deserialize from JSON");
        assert_eq!(config.plays, deserialized.plays);
        assert_eq!(config.discards, deserialized.discards);
        assert_eq!(config.money_max, deserialized.money_max);
        assert_eq!(config.interest_rate, deserialized.interest_rate);
    }

    #[test]
    fn test_config_modification() {
        let mut config = Config::new();

        // Test that we can modify config fields
        config.plays = 6;
        config.discards = 2;
        config.money_max = 1000;
        config.joker_slots = 8;

        assert_eq!(config.plays, 6);
        assert_eq!(config.discards, 2);
        assert_eq!(config.money_max, 1000);
        assert_eq!(config.joker_slots, 8);

        // Other fields should remain unchanged
        assert_eq!(config.ante_start, DEFAULT_ANTE_START);
        assert_eq!(config.ante_end, DEFAULT_ANTE_END);
    }

    #[test]
    fn test_config_zero_edge_cases() {
        let mut config = Config::new();

        // Test edge case where some values are zero
        config.plays = 0;
        config.discards = 0;
        config.money_start = 0;
        config.reward_base = 0;

        assert_eq!(config.plays, 0);
        assert_eq!(config.discards, 0);
        assert_eq!(config.money_start, 0);
        assert_eq!(config.reward_base, 0);
    }

    #[test]
    fn test_config_max_values() {
        let mut config = Config::new();

        // Test with maximum reasonable values
        config.money_max = usize::MAX;
        config.deck_max = usize::MAX;
        config.available_max = usize::MAX;

        assert_eq!(config.money_max, usize::MAX);
        assert_eq!(config.deck_max, usize::MAX);
        assert_eq!(config.available_max, usize::MAX);
    }

    #[test]
    fn test_interest_rate_edge_cases() {
        let mut config = Config::new();

        // Test interest rate edge cases
        config.interest_rate = 0.0; // 0% interest
        assert_eq!(config.interest_rate, 0.0);

        config.interest_rate = 1.0; // 100% interest
        assert_eq!(config.interest_rate, 1.0);

        config.interest_rate = 0.5; // 50% interest
        assert_eq!(config.interest_rate, 0.5);
    }

    #[test]
    fn test_ante_progression() {
        let config = Config::new();

        // Test ante progression makes sense
        assert!(config.ante_start >= 1); // Should start from at least ante 1
        assert!(config.ante_end > config.ante_start); // End should be greater than start
        assert!(config.ante_end <= 8); // Standard Balatro ends at ante 8
    }

    #[test]
    fn test_joker_slot_constraints() {
        let config = Config::new();

        // Test joker slot constraints
        assert!(config.joker_slots <= config.joker_slots_max);
        assert!(config.joker_slots > 0); // Should have at least one slot
        assert!(config.joker_slots_max >= config.joker_slots); // Max should be at least current
    }

    #[test]
    fn test_deck_and_hand_constraints() {
        let config = Config::new();

        // Test deck and hand size constraints
        assert!(config.selected_max <= config.available); // Can't select more than available
        assert!(config.available > 0); // Should have cards available
        assert!(config.deck_max >= 52); // Should accommodate at least a standard deck
        assert!(config.selected_max <= 5); // Standard poker hand max
    }
}
