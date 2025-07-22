use crate::card::{Suit, Value as Rank};
use crate::joker::JokerRarity;
use crate::rank::HandRank;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// TOML schema for defining jokers declaratively
///
/// This module provides a comprehensive schema for defining jokers in TOML format,
/// enabling data-driven joker creation without code compilation.
///
/// # Schema Design Principles
///
/// 1. **Declarative**: All joker behavior defined through configuration
/// 2. **Composable**: Complex behaviors built from simple primitives
/// 3. **Extensible**: Easy to add new condition and action types
/// 4. **Validatable**: Strong typing with comprehensive validation
/// 5. **Performant**: Efficient parsing and runtime evaluation

/// Root configuration for joker definitions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JokerConfig {
    /// Schema version for backwards compatibility
    pub schema_version: String,

    /// List of joker definitions
    pub jokers: Vec<TomlJokerDefinition>,
}

/// Complete joker definition in TOML format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TomlJokerDefinition {
    /// Unique joker identifier (maps to JokerId enum)
    pub id: String,

    /// Display name
    pub name: String,

    /// Effect description
    pub description: String,

    /// Rarity level
    pub rarity: TomlJokerRarity,

    /// Shop cost (optional, defaults to rarity-based pricing)
    pub cost: Option<usize>,

    /// Effect configuration
    pub effect: TomlJokerEffect,

    /// Optional state configuration for stateful jokers
    pub state: Option<TomlJokerState>,

    /// Optional behavior configuration for lifecycle hooks
    pub behavior: Option<TomlJokerBehavior>,
}

/// Joker rarity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TomlJokerRarity {
    Common,
    Uncommon,
    Rare,
    Legendary,
}

impl From<TomlJokerRarity> for JokerRarity {
    fn from(rarity: TomlJokerRarity) -> Self {
        match rarity {
            TomlJokerRarity::Common => JokerRarity::Common,
            TomlJokerRarity::Uncommon => JokerRarity::Uncommon,
            TomlJokerRarity::Rare => JokerRarity::Rare,
            TomlJokerRarity::Legendary => JokerRarity::Legendary,
        }
    }
}

/// Effect types that jokers can have
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum TomlJokerEffect {
    /// Simple scoring bonus (always active)
    Scoring {
        /// Chips bonus per trigger
        #[serde(default)]
        chips: i32,

        /// Mult bonus per trigger
        #[serde(default)]
        mult: i32,

        /// Money bonus per trigger
        #[serde(default)]
        money: i32,

        /// Mult multiplier (1.0 = no change, 2.0 = double)
        #[serde(default = "default_mult_multiplier")]
        mult_multiplier: f64,

        /// Whether this joker provides per-card or per-hand effects
        #[serde(default)]
        per_card: bool,
    },

    /// Conditional effect based on game state or hand
    Conditional {
        /// Condition that must be met
        condition: TomlJokerCondition,

        /// Action to take when condition is met
        action: TomlJokerAction,

        /// Whether to check condition per card or per hand
        #[serde(default)]
        per_card: bool,
    },

    /// Dynamic effect that changes based on joker state
    Dynamic {
        /// Base effect values
        base_effect: TomlJokerAction,

        /// Modifications based on state
        state_modifiers: Vec<TomlStateModifier>,
    },

    /// Special complex effect with custom behavior
    Special {
        /// Special effect type identifier
        special_type: String,

        /// Custom parameters for the special effect
        parameters: HashMap<String, TomlValue>,
    },
}

/// Default mult multiplier (no change)
fn default_mult_multiplier() -> f64 {
    1.0
}

/// Conditions that can trigger joker effects
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum TomlJokerCondition {
    /// Always true (unconditional)
    Always,

    /// Money conditions
    MoneyLessThan {
        amount: i32,
    },
    MoneyGreaterThan {
        amount: i32,
    },
    MoneyEqual {
        amount: i32,
    },

    /// Card property conditions
    SuitScored {
        suit: TomlSuit,
    },
    RankScored {
        rank: TomlRank,
    },
    FaceCardScored,
    NumberCardScored,

    /// Hand composition conditions
    HandType {
        hand_type: TomlHandRank,
    },
    HandSize {
        size: usize,
    },
    NoFaceCards,
    AllSameSuit,
    AllSameRank,

    /// Game state conditions
    Round {
        round: u32,
    },
    Ante {
        ante: u8,
    },
    HandsPlayed {
        count: u32,
    },
    DiscardsUsed {
        count: u32,
    },

    /// Composite conditions
    All {
        conditions: Vec<TomlJokerCondition>,
    },
    Any {
        conditions: Vec<TomlJokerCondition>,
    },
    Not {
        condition: Box<TomlJokerCondition>,
    },

    /// State-based conditions
    StateValue {
        field: String,
        operator: TomlComparisonOperator,
        value: TomlValue,
    },
}

/// Actions that jokers can perform
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum TomlJokerAction {
    /// Add scoring bonuses
    AddScore {
        #[serde(default)]
        chips: i32,
        #[serde(default)]
        mult: i32,
        #[serde(default)]
        money: i32,
        #[serde(default = "default_mult_multiplier")]
        mult_multiplier: f64,
    },

    /// Modify state values
    ModifyState {
        field: String,
        operation: TomlStateOperation,
        value: TomlValue,
    },

    /// Calculate value based on formula
    Calculate {
        formula: String,
        result_type: TomlResultType,
    },

    /// Trigger retrigger effects
    Retrigger { count: u32 },

    /// Destroy jokers
    Destroy { target: TomlDestroyTarget },

    /// Multiple actions in sequence
    Sequence { actions: Vec<TomlJokerAction> },
}

/// State operation types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TomlStateOperation {
    Set,
    Add,
    Subtract,
    Multiply,
    Divide,
    Increment,
    Decrement,
}

/// Comparison operators for conditions
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TomlComparisonOperator {
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
}

/// Result types for calculated actions
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TomlResultType {
    Chips,
    Mult,
    Money,
    MultMultiplier,
}

/// Destruction targets
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TomlDestroyTarget {
    Self_,
    Other { joker_id: String },
    Random { count: usize },
}

/// State modifiers for dynamic effects
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TomlStateModifier {
    /// State field to check
    pub state_field: String,

    /// Multiplier to apply to base effect based on state value
    pub multiplier: f64,
}

/// State configuration for stateful jokers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TomlJokerState {
    /// Field definitions with default values
    pub fields: HashMap<String, TomlValue>,

    /// Whether state persists across rounds/antes
    #[serde(default = "default_true")]
    pub persistent: bool,
}

/// Default true value
fn default_true() -> bool {
    true
}

/// Behavior configuration for lifecycle hooks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TomlJokerBehavior {
    /// Action when hand is played
    pub on_hand_played: Option<TomlJokerAction>,

    /// Action when card is scored
    pub on_card_scored: Option<TomlJokerAction>,

    /// Action when blind starts
    pub on_blind_start: Option<TomlJokerAction>,

    /// Action when shop opens
    pub on_shop_open: Option<TomlJokerAction>,

    /// Action when cards are discarded
    pub on_discard: Option<TomlJokerAction>,

    /// Action when round ends
    pub on_round_end: Option<TomlJokerAction>,

    /// Action when joker is created
    pub on_created: Option<TomlJokerAction>,

    /// Action when joker is activated
    pub on_activated: Option<TomlJokerAction>,

    /// Action when joker is deactivated
    pub on_deactivated: Option<TomlJokerAction>,

    /// Action when joker is cleaned up
    pub on_cleanup: Option<TomlJokerAction>,
}

/// Generic value type for parameters and state
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TomlValue {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Array(Vec<TomlValue>),
    Object(HashMap<String, TomlValue>),
}

/// Suit representation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TomlSuit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

impl From<TomlSuit> for Suit {
    fn from(suit: TomlSuit) -> Self {
        match suit {
            TomlSuit::Hearts => Suit::Heart,
            TomlSuit::Diamonds => Suit::Diamond,
            TomlSuit::Clubs => Suit::Club,
            TomlSuit::Spades => Suit::Spade,
        }
    }
}

/// Rank representation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TomlRank {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

impl From<TomlRank> for Rank {
    fn from(rank: TomlRank) -> Self {
        match rank {
            TomlRank::Ace => Rank::Ace,
            TomlRank::Two => Rank::Two,
            TomlRank::Three => Rank::Three,
            TomlRank::Four => Rank::Four,
            TomlRank::Five => Rank::Five,
            TomlRank::Six => Rank::Six,
            TomlRank::Seven => Rank::Seven,
            TomlRank::Eight => Rank::Eight,
            TomlRank::Nine => Rank::Nine,
            TomlRank::Ten => Rank::Ten,
            TomlRank::Jack => Rank::Jack,
            TomlRank::Queen => Rank::Queen,
            TomlRank::King => Rank::King,
        }
    }
}

/// Hand rank representation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TomlHandRank {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
    RoyalFlush,
}

impl From<TomlHandRank> for HandRank {
    fn from(hand_rank: TomlHandRank) -> Self {
        match hand_rank {
            TomlHandRank::HighCard => HandRank::HighCard,
            TomlHandRank::Pair => HandRank::OnePair,
            TomlHandRank::TwoPair => HandRank::TwoPair,
            TomlHandRank::ThreeOfAKind => HandRank::ThreeOfAKind,
            TomlHandRank::Straight => HandRank::Straight,
            TomlHandRank::Flush => HandRank::Flush,
            TomlHandRank::FullHouse => HandRank::FullHouse,
            TomlHandRank::FourOfAKind => HandRank::FourOfAKind,
            TomlHandRank::StraightFlush => HandRank::StraightFlush,
            TomlHandRank::RoyalFlush => HandRank::RoyalFlush,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_scoring_joker_schema() {
        let toml_str = r#"
            schema_version = "1.0.0"
            
            [[jokers]]
            id = "joker"
            name = "Joker"
            description = "+4 Mult"
            rarity = "common"
            cost = 2
            
            [jokers.effect]
            type = "scoring"
            mult = 4
        "#;

        let config: JokerConfig = toml::from_str(toml_str).expect("Failed to parse TOML");
        assert_eq!(config.jokers.len(), 1);
        assert_eq!(config.jokers[0].name, "Joker");
    }

    #[test]
    fn test_conditional_joker_schema() {
        let toml_str = r#"
            schema_version = "1.0.0"
            
            [[jokers]]
            id = "greedy_joker"
            name = "Greedy Joker"
            description = "+3 Mult per Diamond"
            rarity = "common"
            per_card = true
            
            [jokers.effect]
            type = "conditional"
            per_card = true
            
            [jokers.effect.condition]
            type = "suit_scored"
            suit = "diamonds"
            
            [jokers.effect.action]
            type = "add_score"
            mult = 3
        "#;

        let config: JokerConfig = toml::from_str(toml_str).expect("Failed to parse TOML");
        assert_eq!(config.jokers.len(), 1);
        assert_eq!(config.jokers[0].name, "Greedy Joker");
    }

    #[test]
    fn test_dynamic_joker_schema() {
        let toml_str = r#"
            schema_version = "1.0.0"
            
            [[jokers]]
            id = "ice_cream"
            name = "Ice Cream"
            description = "+100 Chips, -5 Chips per hand played"
            rarity = "common"
            
            [jokers.effect]
            type = "dynamic"
            
            [jokers.effect.base_effect]
            type = "add_score"
            chips = 100
            
            [[jokers.effect.state_modifiers]]
            state_field = "hands_played"
            multiplier = -5.0
            
            [jokers.state]
            persistent = true
            
            [jokers.state.fields]
            hands_played = 0
            
            [jokers.behavior]
            
            [jokers.behavior.on_hand_played]
            type = "modify_state"
            field = "hands_played"
            operation = "increment"
            value = 1
        "#;

        let config: JokerConfig = toml::from_str(toml_str).expect("Failed to parse TOML");
        assert_eq!(config.jokers.len(), 1);
        assert_eq!(config.jokers[0].name, "Ice Cream");
    }
}
