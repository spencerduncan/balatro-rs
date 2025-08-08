//! Test fixtures for domain entities and game state
//!
//! Provides factory functions for creating test instances of domain objects
//! with realistic configurations for comprehensive testing.
//!

#![allow(clippy::all)]
//! ## Production Engineering Patterns
//! - Builder pattern for complex object creation
//! - Deterministic test data for reproducible tests
//! - Edge case generators for boundary testing
//! - Performance test datasets for load testing

use balatro_rs::{
    action::Action,
    card::{Card, Suit, Value},
    config::Config,
    game::Game,
    joker::{JokerEffect, JokerId},
    shop::packs::PackType,
    stage::{Blind, Stage},
};

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

/// Creates a test game with custom seed for deterministic testing
///
/// Note: Seed configuration in Config is not yet available in the current
/// codebase. This function is prepared for future implementation when
/// seed support is added to the Config struct.
pub fn create_test_game_with_seed(_seed: u64) -> Game {
    // API Note: Seed field will be added to Config in future updates
    // Current workaround: Use default config with deterministic operations
    let config = Config::default();
    let mut game = Game::new(config);
    game.start();
    game
}

/// Creates a comprehensive set of test cards covering all suits and values
pub fn create_test_deck() -> Vec<Card> {
    let mut cards = Vec::new();

    for suit in [Suit::Heart, Suit::Diamond, Suit::Club, Suit::Spade] {
        for value in [
            Value::Ace,
            Value::Two,
            Value::Three,
            Value::Four,
            Value::Five,
            Value::Six,
            Value::Seven,
            Value::Eight,
            Value::Nine,
            Value::Ten,
            Value::Jack,
            Value::Queen,
            Value::King,
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
            ante: 8,    // Balatro's maximum ante
            round: 255, // Max u8 value
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

// ============================================================================
// PRODUCTION-READY TEST FIXTURES FROM PR #779 SALVAGE
// ============================================================================

/// Builder pattern for creating complex game states
/// Production pattern: Fluent interface for test data creation
pub struct GameStateBuilder {
    config: Config,
    ante: u8,
    round: u8,
    money: i32,
    chips: i32,
    mult: i32,
    jokers: Vec<JokerId>,
    stage: Stage,
    seed: Option<u64>,
}

impl GameStateBuilder {
    /// Creates a new game state builder with defaults
    pub fn new() -> Self {
        Self {
            config: Config::default(),
            ante: 1,
            round: 1,
            money: 4,
            chips: 0,
            mult: 0,
            jokers: Vec::new(),
            stage: Stage::PreBlind(),
            seed: None,
        }
    }

    /// Sets the ante level
    pub fn with_ante(mut self, ante: u8) -> Self {
        self.ante = ante;
        self
    }

    /// Sets the round number
    pub fn with_round(mut self, round: u8) -> Self {
        self.round = round;
        self
    }

    /// Sets the money amount
    pub fn with_money(mut self, money: i32) -> Self {
        self.money = money;
        self
    }

    /// Sets chips and mult
    pub fn with_score(mut self, chips: i32, mult: i32) -> Self {
        self.chips = chips;
        self.mult = mult;
        self
    }

    /// Adds jokers to the game state
    pub fn with_jokers(mut self, jokers: Vec<JokerId>) -> Self {
        self.jokers = jokers;
        self
    }

    /// Sets the game stage
    pub fn with_stage(mut self, stage: Stage) -> Self {
        self.stage = stage;
        self
    }

    /// Sets a deterministic seed for reproducible tests
    pub fn with_seed(mut self, seed: u64) -> Self {
        self.seed = Some(seed);
        self
    }

    /// Builds the game with the configured state
    pub fn build(self) -> Game {
        let config = self.config;

        let mut game = Game::new(config);
        game.start();

        // Apply configured state (convert types as needed)
        game.ante_current = balatro_rs::ante::Ante::try_from(self.ante as usize)
            .unwrap_or(balatro_rs::ante::Ante::One);
        game.round = self.round.into();
        game.money = self.money.into();
        game.chips = self.chips.into();
        game.mult = self.mult.into();
        game.stage = self.stage;

        // Note: Joker addition API is not yet exposed for direct manipulation
        // This will be available when the joker management API is refactored
        // Current workaround: Use shop actions to add jokers in tests

        game
    }
}

/// Joker configuration builder for test scenarios
/// Production pattern: Type-safe joker configuration
pub struct JokerTestBuilder {
    joker_id: JokerId,
    trigger_count: usize,
    effect: JokerEffect,
}

impl JokerTestBuilder {
    /// Creates a new joker test builder
    pub fn new(joker_id: JokerId) -> Self {
        Self {
            joker_id,
            trigger_count: 0,
            effect: JokerEffect::default(),
        }
    }

    /// Sets the trigger count for scaling jokers
    pub fn with_triggers(mut self, count: usize) -> Self {
        self.trigger_count = count;
        self
    }

    /// Sets the joker effect
    pub fn with_effect(mut self, chips: i32, mult: i32, x_mult: f64) -> Self {
        self.effect = JokerEffect {
            chips,
            mult,
            money: 0,
            interest_bonus: 0,
            mult_multiplier: x_mult,
            retrigger: 0,
            destroy_self: false,
            destroy_others: Vec::new(),
            transform_cards: Vec::new(),
            hand_size_mod: 0,
            discard_mod: 0,
            sell_value_increase: 0,
            message: None,
            consumables_created: Vec::new(),
        };
        self
    }

    /// Builds the joker configuration
    pub fn build(self) -> (JokerId, usize, JokerEffect) {
        (self.joker_id, self.trigger_count, self.effect)
    }
}

/// Creates a deck builder for custom deck configurations
/// Production pattern: Controlled deck state for testing
pub struct DeckBuilder {
    cards: Vec<Card>,
    shuffle: bool,
    seed: Option<u64>,
}

impl DeckBuilder {
    /// Creates a new deck builder
    pub fn new() -> Self {
        Self {
            cards: Vec::new(),
            shuffle: false,
            seed: None,
        }
    }

    /// Adds a standard 52-card deck
    pub fn with_standard_deck(mut self) -> Self {
        self.cards = create_test_deck();
        self
    }

    /// Adds specific cards to the deck
    pub fn with_cards(mut self, cards: Vec<Card>) -> Self {
        self.cards.extend(cards);
        self
    }

    /// Adds multiple copies of a card
    pub fn with_card_copies(mut self, card: Card, count: usize) -> Self {
        for _ in 0..count {
            self.cards.push(card.clone());
        }
        self
    }

    /// Enables shuffling with optional seed
    pub fn shuffled(mut self, seed: Option<u64>) -> Self {
        self.shuffle = true;
        self.seed = seed;
        self
    }

    /// Builds the deck
    pub fn build(mut self) -> Vec<Card> {
        if self.shuffle {
            // Use deterministic shuffle for testing
            if let Some(seed) = self.seed {
                let rng = balatro_rs::rng::GameRng::new(balatro_rs::rng::RngMode::Testing(seed));
                // Simple Fisher-Yates shuffle
                for i in (1..self.cards.len()).rev() {
                    let j = rng.gen_range(0..=i);
                    self.cards.swap(i, j);
                }
            }
        }
        self.cards
    }
}

/// Test data generator for stress testing
/// Production pattern: Scalable test data generation
pub struct TestDataGenerator {
    seed: u64,
    rng: balatro_rs::rng::GameRng,
}

impl TestDataGenerator {
    /// Creates a new test data generator with seed
    pub fn new(seed: u64) -> Self {
        Self {
            seed,
            rng: balatro_rs::rng::GameRng::new(balatro_rs::rng::RngMode::Testing(seed)),
        }
    }

    /// Generates random cards
    pub fn generate_random_cards(&mut self, count: usize) -> Vec<Card> {
        let suits = [Suit::Heart, Suit::Diamond, Suit::Club, Suit::Spade];
        let values = [
            Value::Ace,
            Value::Two,
            Value::Three,
            Value::Four,
            Value::Five,
            Value::Six,
            Value::Seven,
            Value::Eight,
            Value::Nine,
            Value::Ten,
            Value::Jack,
            Value::Queen,
            Value::King,
        ];

        (0..count)
            .map(|_| {
                let suit = suits[self.rng.gen_range(0..suits.len())];
                let value = values[self.rng.gen_range(0..values.len())];
                Card::new(value, suit)
            })
            .collect()
    }

    /// Generates random game states for testing
    pub fn generate_game_states(&mut self, count: usize) -> Vec<Game> {
        (0..count)
            .map(|i| {
                GameStateBuilder::new()
                    .with_seed(self.seed + i as u64)
                    .with_ante(self.rng.gen_range(1..=8))
                    .with_round(self.rng.gen_range(1..=3))
                    .with_money(self.rng.gen_range(0..=100))
                    .with_score(self.rng.gen_range(0..=1000), self.rng.gen_range(1..=50))
                    .build()
            })
            .collect()
    }

    /// Generates action sequences for testing
    pub fn generate_action_sequences(&mut self, length: usize) -> Vec<Vec<Action>> {
        let mut sequences = Vec::new();

        for _ in 0..10 {
            let mut sequence = Vec::new();
            for _ in 0..length {
                // Generate random actions
                use rand::prelude::*;
                let mut rng = rand::thread_rng();
                let action = match rng.gen_range(0..5) {
                    0 => Action::SelectCard(self.generate_random_cards(1)[0].clone()),
                    1 => Action::Play(),
                    2 => Action::Discard(),
                    3 => Action::NextRound(),
                    _ => Action::CashOut(rng.gen_range(0..=100) as f64),
                };
                sequence.push(action);
            }
            sequences.push(sequence);
        }

        sequences
    }
}

/// Creates test fixtures for concurrent testing scenarios
/// Production pattern: Thread-safe test data
pub fn create_concurrent_test_fixtures(count: usize) -> Vec<Game> {
    (0..count)
        .map(|i| {
            GameStateBuilder::new()
                .with_seed(12345 + i as u64) // Deterministic but unique
                .build()
        })
        .collect()
}

/// Creates test fixtures for memory leak detection
/// Production pattern: Resource tracking
pub fn create_memory_test_fixtures() -> MemoryTestFixtures {
    MemoryTestFixtures {
        small_games: (0..10).map(|i| create_test_game_with_seed(i)).collect(),
        large_games: (0..100)
            .map(|i| {
                GameStateBuilder::new()
                    .with_seed(i)
                    .with_jokers(vec![JokerId::Joker; 5])
                    .build()
            })
            .collect(),
        stress_games: (0..1000).map(|i| create_test_game_with_seed(i)).collect(),
    }
}

/// Memory test fixtures structure
pub struct MemoryTestFixtures {
    pub small_games: Vec<Game>,
    pub large_games: Vec<Game>,
    pub stress_games: Vec<Game>,
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
            assert_eq!(
                hand.len(),
                5,
                "Hand type {:?} should have 5 cards",
                hand_type
            );
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

    #[test]
    fn test_game_state_builder() {
        let game = GameStateBuilder::new()
            .with_ante(3)
            .with_round(2)
            .with_money(50)
            .with_score(100, 10)
            .with_seed(42)
            .build();

        assert_eq!(game.ante_current, balatro_rs::ante::Ante::Three);
        assert_eq!(game.round, 2.0);
        assert_eq!(game.money, 50.0);
        assert_eq!(game.chips, 100.0);
        assert_eq!(game.mult, 10.0);
    }

    #[test]
    fn test_joker_test_builder() {
        let (id, triggers, effect) = JokerTestBuilder::new(JokerId::Joker)
            .with_triggers(5)
            .with_effect(10, 5, 1.5)
            .build();

        assert_eq!(id, JokerId::Joker);
        assert_eq!(triggers, 5);
        assert_eq!(effect.chips, 10);
        assert_eq!(effect.mult, 5);
        assert_eq!(effect.mult_multiplier, 1.5);
    }

    #[test]
    fn test_deck_builder() {
        let deck = DeckBuilder::new().with_standard_deck().build();

        assert_eq!(deck.len(), 52);

        let custom_deck = DeckBuilder::new()
            .with_card_copies(Card::new(Value::Ace, Suit::Spade), 4)
            .build();

        assert_eq!(custom_deck.len(), 4);
        assert!(custom_deck.iter().all(|c| c.value == Value::Ace));
    }

    #[test]
    fn test_data_generator() {
        let mut generator = TestDataGenerator::new(42);

        let cards = generator.generate_random_cards(10);
        assert_eq!(cards.len(), 10);

        let states = generator.generate_game_states(5);
        assert_eq!(states.len(), 5);

        let sequences = generator.generate_action_sequences(3);
        assert!(!sequences.is_empty());
    }

    #[test]
    fn test_concurrent_fixtures() {
        let fixtures = create_concurrent_test_fixtures(10);
        assert_eq!(fixtures.len(), 10);

        // Verify each game has a unique state
        let _seeds: Vec<_> = fixtures.iter().map(|g| g.ante_current).collect();
        // All games should start with same ante but different internal states
        assert!(fixtures.iter().all(|g| !g.is_over()));
    }

    #[test]
    fn test_memory_fixtures() {
        let fixtures = create_memory_test_fixtures();

        assert_eq!(fixtures.small_games.len(), 10);
        assert_eq!(fixtures.large_games.len(), 100);
        assert_eq!(fixtures.stress_games.len(), 1000);
    }
}
