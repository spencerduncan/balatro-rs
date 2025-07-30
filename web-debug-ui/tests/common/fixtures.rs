//! Test fixtures for domain entities and game state
//!
//! Provides factory functions for creating test instances of domain objects
//! with realistic configurations for comprehensive testing.

use balatro_rs::{
    action::Action,
    card::{Card, Suit, Value},
    config::Config,
    game::Game,
    hand::Hand,
    joker::{JokerId, GameContext},
    joker_state::JokerStateManager,
    rank::HandRank,
    rng::GameRng,
    stage::{Stage, Blind},
    shop::packs::PackType,
};
use std::collections::HashMap;
use std::sync::Arc;

/// Creates a default game configuration for testing
pub fn default_game_config() -> Config {
    Config::default()
}

/// Creates a deterministic game instance for testing
pub fn create_test_game() -> Game {
    let mut game = Game::default();
    game.start();
    game
}

/// Creates a test game with custom RNG seed
pub fn create_test_game_with_seed(seed: u64) -> Game {
    let mut game = Game::default();
    // Note: Game doesn't directly expose RNG configuration
    // This is a placeholder for future RNG configuration
    game.start();
    game
}

/// Creates a test game in a specific stage
pub fn create_test_game_in_stage(stage: Stage) -> Game {
    let mut game = create_test_game();
    // Note: Direct stage manipulation would require extending Game API
    // For now, we'll progress the game to the desired stage through actions
    game
}

/// Creates a comprehensive set of test cards covering all suits and values
pub fn create_test_deck() -> Vec<Card> {
    let mut cards = Vec::new();

    for suit in [Suit::Heart, Suit::Diamond, Suit::Club, Suit::Spade] {
        for value in [
            Value::Ace, Value::Two, Value::Three, Value::Four, Value::Five,
            Value::Six, Value::Seven, Value::Eight, Value::Nine, Value::Ten,
            Value::Jack, Value::Queen, Value::King
        ] {
            cards.push(Card::new(value, suit));
        }
    }

    cards
}

/// Creates a specific hand type for testing
pub fn create_test_hand(hand_type: TestHandType) -> Vec<Card> {
    match hand_type {
        TestHandType::RoyalFlush => vec![
            Card::new(Value::Ten, Suit::Spade),
            Card::new(Value::Jack, Suit::Spade),
            Card::new(Value::Queen, Suit::Spade),
            Card::new(Value::King, Suit::Spade),
            Card::new(Value::Ace, Suit::Spade),
        ],
        TestHandType::StraightFlush => vec![
            Card::new(Value::Five, Suit::Heart),
            Card::new(Value::Six, Suit::Heart),
            Card::new(Value::Seven, Suit::Heart),
            Card::new(Value::Eight, Suit::Heart),
            Card::new(Value::Nine, Suit::Heart),
        ],
        TestHandType::FourOfAKind => vec![
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::King, Suit::Spade),
            Card::new(Value::King, Suit::Club),
            Card::new(Value::King, Suit::Diamond),
            Card::new(Value::Two, Suit::Heart),
        ],
        TestHandType::FullHouse => vec![
            Card::new(Value::Queen, Suit::Heart),
            Card::new(Value::Queen, Suit::Spade),
            Card::new(Value::Queen, Suit::Club),
            Card::new(Value::Jack, Suit::Heart),
            Card::new(Value::Jack, Suit::Spade),
        ],
        TestHandType::Flush => vec![
            Card::new(Value::Two, Suit::Diamond),
            Card::new(Value::Four, Suit::Diamond),
            Card::new(Value::Six, Suit::Diamond),
            Card::new(Value::Eight, Suit::Diamond),
            Card::new(Value::Ten, Suit::Diamond),
        ],
        TestHandType::Straight => vec![
            Card::new(Value::Five, Suit::Heart),
            Card::new(Value::Six, Suit::Spade),
            Card::new(Value::Seven, Suit::Club),
            Card::new(Value::Eight, Suit::Diamond),
            Card::new(Value::Nine, Suit::Heart),
        ],
        TestHandType::ThreeOfAKind => vec![
            Card::new(Value::Seven, Suit::Heart),
            Card::new(Value::Seven, Suit::Spade),
            Card::new(Value::Seven, Suit::Club),
            Card::new(Value::Two, Suit::Heart),
            Card::new(Value::Five, Suit::Spade),
        ],
        TestHandType::TwoPair => vec![
            Card::new(Value::Jack, Suit::Heart),
            Card::new(Value::Jack, Suit::Spade),
            Card::new(Value::Three, Suit::Club),
            Card::new(Value::Three, Suit::Heart),
            Card::new(Value::Nine, Suit::Spade),
        ],
        TestHandType::OnePair => vec![
            Card::new(Value::Eight, Suit::Heart),
            Card::new(Value::Eight, Suit::Spade),
            Card::new(Value::Two, Suit::Club),
            Card::new(Value::Five, Suit::Heart),
            Card::new(Value::King, Suit::Spade),
        ],
        TestHandType::HighCard => vec![
            Card::new(Value::Ace, Suit::Heart),
            Card::new(Value::Three, Suit::Spade),
            Card::new(Value::Five, Suit::Club),
            Card::new(Value::Seven, Suit::Heart),
            Card::new(Value::Nine, Suit::Diamond),
        ],
        TestHandType::Empty => vec![],
        TestHandType::Single => vec![Card::new(Value::Ace, Suit::Spade)],
    }
}

/// Enumeration of test hand types for fixtures
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TestHandType {
    RoyalFlush,
    StraightFlush,
    FourOfAKind,
    FullHouse,
    Flush,
    Straight,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
    Empty,
    Single,
}

/// Creates a set of test actions covering all action types
pub fn create_test_actions() -> Vec<Action> {
    let test_card = Card::new(Value::Ace, Suit::Spade);

    vec![
        Action::SelectCard(test_card),
        Action::Play(),
        Action::Discard(),
        Action::CashOut(100.0),
        Action::BuyJoker {
            joker_id: JokerId::Joker,
            slot: 0,
        },
        Action::BuyPack {
            pack_type: PackType::Standard,
        },
        Action::OpenPack { pack_id: 1 },
        Action::SelectFromPack {
            pack_id: 1,
            option_index: 0,
        },
        Action::SkipPack { pack_id: 1 },
        Action::NextRound(),
        Action::SelectBlind(Blind::Small),
    ]
}

/// Creates a realistic game context for joker testing
/// Returns a closure that creates the context with proper lifetimes
pub fn create_test_game_context_factory() -> impl Fn() -> (
    Arc<JokerStateManager>,
    Stage,
    Hand,
    HashMap<HandRank, u32>,
    GameRng,
    Vec<Card>,
) {
    move || {
        let joker_state_manager = Arc::new(JokerStateManager::new());
        let stage = Stage::PreBlind();
        let hand = Hand::new(vec![]);
        let hand_type_counts = HashMap::new();
        let rng = GameRng::for_testing(42);
        let discarded = vec![];

        (joker_state_manager, stage, hand, hand_type_counts, rng, discarded)
    }
}

/// Creates a test game context with proper lifetimes for a given test
/// This macro helps create GameContext with proper lifetime management
#[macro_export]
macro_rules! with_test_game_context {
    ($test_code:expr) => {{
        let joker_state_manager = std::sync::Arc::new(balatro_rs::joker_state::JokerStateManager::new());
        let stage = balatro_rs::stage::Stage::PreBlind();
        let hand = balatro_rs::hand::Hand::new(vec![]);
        let hand_type_counts = std::collections::HashMap::new();
        let rng = balatro_rs::rng::GameRng::for_testing(42);
        let discarded = vec![];
        let jokers: Vec<Box<dyn balatro_rs::joker::Joker>> = vec![];

        let context = balatro_rs::joker::GameContext {
            chips: 100,
            mult: 4,
            money: 100,
            ante: 1,
            round: 1,
            stage: &stage,
            hands_played: 0,
            discards_used: 0,
            hands_remaining: 4.0,
            jokers: &jokers,
            hand: &hand,
            discarded: &discarded,
            joker_state_manager: &joker_state_manager,
            hand_type_counts: &hand_type_counts,
            cards_in_deck: 52,
            stone_cards_in_deck: 0,
            steel_cards_in_deck: 0,
            rng: &rng,
        };

        $test_code(context)
    }};
}

/// Creates edge case test scenarios for comprehensive testing
pub fn create_edge_case_scenarios() -> Vec<TestScenario> {
    vec![
        TestScenario {
            name: "Zero money",
            chips: 0,
            mult: 1,
            money: 0,
            ante: 1,
            round: 1,
        },
        TestScenario {
            name: "Maximum values",
            chips: i32::MAX,
            mult: i32::MAX,
            money: i32::MAX,
            ante: 8, // Balatro's maximum ante
            round: 999,
        },
        TestScenario {
            name: "High ante",
            chips: 1000,
            mult: 10,
            money: 50,
            ante: 8,
            round: 1,
        },
        TestScenario {
            name: "Low resources",
            chips: 1,
            mult: 1,
            money: 1,
            ante: 1,
            round: 1,
        },
    ]
}

/// Test scenario for edge case testing
#[derive(Debug, Clone)]
pub struct TestScenario {
    pub name: &'static str,
    pub chips: i32,
    pub mult: i32,
    pub money: i32,
    pub ante: u8,
    pub round: u8,
}

/// Creates performance test data sets
pub fn create_performance_test_data() -> PerformanceTestData {
    PerformanceTestData {
        small_dataset: create_test_hands_batch(10),
        medium_dataset: create_test_hands_batch(100),
        large_dataset: create_test_hands_batch(1000),
        stress_dataset: create_test_hands_batch(10000),
    }
}

/// Performance test data structure
pub struct PerformanceTestData {
    pub small_dataset: Vec<Vec<Card>>,
    pub medium_dataset: Vec<Vec<Card>>,
    pub large_dataset: Vec<Vec<Card>>,
    pub stress_dataset: Vec<Vec<Card>>,
}

/// Creates a batch of test hands for performance testing
fn create_test_hands_batch(count: usize) -> Vec<Vec<Card>> {
    let hand_types = [
        TestHandType::RoyalFlush,
        TestHandType::StraightFlush,
        TestHandType::FourOfAKind,
        TestHandType::FullHouse,
        TestHandType::Flush,
        TestHandType::Straight,
        TestHandType::ThreeOfAKind,
        TestHandType::TwoPair,
        TestHandType::OnePair,
        TestHandType::HighCard,
    ];

    (0..count)
        .map(|i| create_test_hand(hand_types[i % hand_types.len()]))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_test_game() {
        let game = create_test_game();
        assert!(!game.is_over());
    }

    #[test]
    fn test_create_test_deck() {
        let deck = create_test_deck();
        assert_eq!(deck.len(), 52);

        // Verify all suits and values are present
        let mut suits = std::collections::HashSet::new();
        let mut values = std::collections::HashSet::new();

        for card in deck {
            suits.insert(card.suit);
            values.insert(card.value);
        }

        assert_eq!(suits.len(), 4);
        assert_eq!(values.len(), 13);
    }

    #[test]
    fn test_create_test_hand_types() {
        for hand_type in [
            TestHandType::RoyalFlush,
            TestHandType::StraightFlush,
            TestHandType::FourOfAKind,
            TestHandType::FullHouse,
            TestHandType::Flush,
            TestHandType::Straight,
            TestHandType::ThreeOfAKind,
            TestHandType::TwoPair,
            TestHandType::OnePair,
            TestHandType::HighCard,
        ] {
            let hand = create_test_hand(hand_type);
            assert_eq!(hand.len(), 5, "Hand type {:?} should have 5 cards", hand_type);
        }

        assert_eq!(create_test_hand(TestHandType::Empty).len(), 0);
        assert_eq!(create_test_hand(TestHandType::Single).len(), 1);
    }

    #[test]
    fn test_create_test_actions() {
        let actions = create_test_actions();
        assert!(!actions.is_empty());

        // Verify all actions can be displayed without panicking
        for action in actions {
            let _ = format!("{:?}", action);
            let _ = format!("{}", action);
        }
    }

    #[test]
    fn test_create_edge_case_scenarios() {
        let scenarios = create_edge_case_scenarios();
        assert!(!scenarios.is_empty());

        for scenario in scenarios {
            assert!(!scenario.name.is_empty());
            assert!(scenario.ante > 0);
            assert!(scenario.round > 0);
        }
    }

    #[test]
    fn test_performance_test_data() {
        let data = create_performance_test_data();

        assert!(!data.small_dataset.is_empty());
        assert!(!data.medium_dataset.is_empty());
        assert!(!data.large_dataset.is_empty());
        assert!(!data.stress_dataset.is_empty());

        assert!(data.medium_dataset.len() > data.small_dataset.len());
        assert!(data.large_dataset.len() > data.medium_dataset.len());
        assert!(data.stress_dataset.len() > data.large_dataset.len());
    }
}
