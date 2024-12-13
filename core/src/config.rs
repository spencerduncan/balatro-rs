const DEFAULT_ROUND_START: usize = 0;
const DEFAULT_PLAYS: usize = 4;
const DEFAULT_DISCARDS: usize = 4;
const DEFAULT_MONEY_START: usize = 0;
const DEFAULT_REWARD_BASE: usize = 0;
const DEFAULT_MONEY_PER_HAND: usize = 1;
const DEFAULT_INTEREST_RATE: f32 = 0.2;
const DEFAULT_INTEREST_MAX: usize = 5;
const DEFAULT_HAND_SIZE: usize = 8;
const DEFAULT_BASE_MULT: usize = 1;
const DEFAULT_BASE_CHIPS: usize = 0;
const DEFAULT_BASE_SCORE: usize = 0;
const DEFAULT_ANTE_START: usize = 1;
const DEFAULT_ANTE_END: usize = 8;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct Config {
    pub(crate) round_start: usize,
    pub(crate) plays: usize,
    pub(crate) discards: usize,
    pub(crate) money_start: usize,
    pub(crate) reward_base: usize,
    pub(crate) money_per_hand: usize,
    pub(crate) interest_rate: f32,
    pub(crate) interest_max: usize,
    pub(crate) hand_size: usize,
    pub(crate) base_mult: usize,
    pub(crate) base_chips: usize,
    pub(crate) base_score: usize,
    pub(crate) ante_start: usize,
    pub(crate) ante_end: usize,
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
        };
    }
}

impl Default for Config {
    fn default() -> Self {
        return Self::new();
    }
}
