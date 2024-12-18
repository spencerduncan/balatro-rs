use pyo3::prelude::*;

const DEFAULT_ROUND_START: usize = 0;
const DEFAULT_PLAYS: usize = 4;
const DEFAULT_DISCARDS: usize = 4;
const DEFAULT_MONEY_START: usize = 0;
const DEFAULT_REWARD_BASE: usize = 0;
const DEFAULT_MONEY_PER_HAND: usize = 1;
const DEFAULT_INTEREST_RATE: f32 = 0.2;
const DEFAULT_INTEREST_MAX: usize = 5;
const DEFAULT_HAND_SIZE: usize = 8;
const DEFAULT_BASE_MULT: usize = 0;
const DEFAULT_BASE_CHIPS: usize = 0;
const DEFAULT_BASE_SCORE: usize = 0;
const DEFAULT_ANTE_START: usize = 1;
const DEFAULT_ANTE_END: usize = 8;
const DEFAULT_JOKER_SLOTS: usize = 5;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "python", pyclass)]
#[derive(Debug, Clone)]
pub struct Config {
    pub round_start: usize,
    pub plays: usize,
    pub discards: usize,
    pub money_start: usize,
    pub reward_base: usize,
    pub money_per_hand: usize,
    pub interest_rate: f32,
    pub interest_max: usize,
    pub hand_size: usize,
    pub base_mult: usize,
    pub base_chips: usize,
    pub base_score: usize,
    pub ante_start: usize,
    pub ante_end: usize,
    pub joker_slots: usize,
}

impl Config {
    pub fn new() -> Self {
        return Config {
            round_start: DEFAULT_ROUND_START,
            plays: DEFAULT_PLAYS,
            discards: DEFAULT_DISCARDS,
            money_start: DEFAULT_MONEY_START,
            reward_base: DEFAULT_REWARD_BASE,
            money_per_hand: DEFAULT_MONEY_PER_HAND,
            interest_rate: DEFAULT_INTEREST_RATE,
            interest_max: DEFAULT_INTEREST_MAX,
            hand_size: DEFAULT_HAND_SIZE,
            base_mult: DEFAULT_BASE_MULT,
            base_chips: DEFAULT_BASE_CHIPS,
            base_score: DEFAULT_BASE_SCORE,
            ante_start: DEFAULT_ANTE_START,
            ante_end: DEFAULT_ANTE_END,
            joker_slots: DEFAULT_JOKER_SLOTS,
        };
    }
}

impl Default for Config {
    fn default() -> Self {
        return Self::new();
    }
}

#[cfg(feature = "python")]
#[pymethods]
impl Config {
    #[new]
    fn py_new() -> Self {
        Self::default()
    }

    #[setter]
    fn ante_end(&mut self, i: usize) {
        self.ante_end = i;
    }
}
