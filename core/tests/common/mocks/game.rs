//! Game State Mocks for Predictable Testing Scenarios
//!
//! Provides builders and utilities for creating deterministic game states,
//! scenario generators for various game situations, and state snapshot capabilities.

use super::MockRng;
use balatro_rs::{
    action::Action, card::Card, card::Suit, consumables::ConsumableType, deck::Deck, game::Game,
    joker::JokerId, rank::HandRank, stage::Stage, vouchers::VoucherId,
};
use std::collections::HashMap;

/// Builder for creating predictable game states
pub struct MockGameBuilder {
    pub money: i32,
    pub ante: u8,
    pub round: u8,
    pub score: i64,
    pub stage: Stage,
    pub hands_remaining: u8,
    pub discards_remaining: u8,
    pub jokers: Vec<JokerId>,
    pub hand: Vec<Card>,
    pub deck_cards: Vec<Card>,
    pub consumables: Vec<ConsumableType>,
    pub vouchers: Vec<VoucherId>,
    pub planet_levels: HashMap<HandRank, u8>,
    pub boss_blind: Option<String>, // Using String for blind type name
    pub rng: Option<MockRng>,
}

impl MockGameBuilder {
    /// Create a new game builder with sensible defaults
    pub fn new() -> Self {
        Self {
            money: 4,
            ante: 1,
            round: 1,
            score: 0,
            stage: Stage::PreBlind,
            hands_remaining: 4,
            discards_remaining: 3,
            jokers: Vec::new(),
            hand: Vec::new(),
            deck_cards: Vec::new(),
            consumables: Vec::new(),
            vouchers: Vec::new(),
            planet_levels: HashMap::new(),
            boss_blind: None,
            rng: None,
        }
    }

    /// Set the current money
    pub fn with_money(mut self, money: i32) -> Self {
        self.money = money;
        self
    }

    /// Set the current ante and round
    pub fn with_ante_round(mut self, ante: u8, round: u8) -> Self {
        self.ante = ante;
        self.round = round;
        self
    }

    /// Set the current score
    pub fn with_score(mut self, score: i64) -> Self {
        self.score = score;
        self
    }

    /// Set the current stage
    pub fn with_stage(mut self, stage: Stage) -> Self {
        self.stage = stage;
        self
    }

    /// Set hands and discards remaining
    pub fn with_hands_discards(mut self, hands: u8, discards: u8) -> Self {
        self.hands_remaining = hands;
        self.discards_remaining = discards;
        self
    }

    /// Add jokers to the game
    pub fn with_jokers(mut self, jokers: Vec<JokerId>) -> Self {
        self.jokers = jokers;
        self
    }

    /// Set the cards in hand
    pub fn with_hand(mut self, hand: Vec<Card>) -> Self {
        self.hand = hand;
        self
    }

    /// Set the deck cards
    pub fn with_deck(mut self, cards: Vec<Card>) -> Self {
        self.deck_cards = cards;
        self
    }

    /// Add consumables
    pub fn with_consumables(mut self, consumables: Vec<ConsumableType>) -> Self {
        self.consumables = consumables;
        self
    }

    /// Add vouchers
    pub fn with_vouchers(mut self, vouchers: Vec<VoucherId>) -> Self {
        self.vouchers = vouchers;
        self
    }

    /// Set planet card levels
    pub fn with_planet_level(mut self, hand_type: HandType, level: u8) -> Self {
        self.planet_levels.insert(hand_type, level);
        self
    }

    /// Set boss blind
    pub fn with_boss_blind(mut self, blind: BlindType) -> Self {
        self.boss_blind = Some(blind);
        self
    }

    /// Use a specific MockRng
    pub fn with_rng(mut self, rng: MockRng) -> Self {
        self.rng = Some(rng);
        self
    }

    /// Build the game instance
    pub fn build(self) -> Game {
        let mut game = if let Some(rng) = self.rng {
            // Use mock RNG if provided
            Game::new_with_seed(42) // Would need to modify Game to accept MockRng
        } else {
            Game::new()
        };

        // Apply all the settings
        // Note: This would require adding setter methods to Game or
        // accessing internals through a test-only interface
        self.apply_to_game(&mut game);

        game
    }

    /// Apply settings to an existing game
    fn apply_to_game(&self, game: &mut Game) {
        // This would need to be implemented with actual Game API
        // For now, this is a placeholder showing the intent

        // The actual implementation would set:
        // - game.money = self.money
        // - game.ante = self.ante
        // - game.round = self.round
        // - game.score = self.score
        // - game.stage = self.stage
        // - etc.
    }
}

/// Predefined game scenarios for common test cases
pub struct GameScenario;

impl GameScenario {
    /// Create a game at the start of a run
    pub fn new_run() -> MockGameBuilder {
        MockGameBuilder::new()
            .with_money(4)
            .with_ante_round(1, 1)
            .with_score(0)
            .with_stage(Stage::PreBlind)
            .with_hands_discards(4, 3)
    }

    /// Create a game in the middle of a blind
    pub fn mid_blind() -> MockGameBuilder {
        MockGameBuilder::new()
            .with_money(15)
            .with_ante_round(2, 2)
            .with_score(500)
            .with_stage(Stage::Blind)
            .with_hands_discards(2, 1)
            .with_jokers(vec![JokerId::Joker, JokerId::Scholar])
    }

    /// Create a game at the shop
    pub fn at_shop() -> MockGameBuilder {
        MockGameBuilder::new()
            .with_money(25)
            .with_ante_round(3, 1)
            .with_score(1500)
            .with_stage(Stage::Shop)
            .with_jokers(vec![JokerId::Baron, JokerId::Burglar, JokerId::IceCream])
    }

    /// Create a game facing a boss blind
    pub fn boss_blind() -> MockGameBuilder {
        MockGameBuilder::new()
            .with_money(30)
            .with_ante_round(4, 3)
            .with_score(5000)
            .with_stage(Stage::Blind)
            .with_boss_blind(BlindType::ThePsychic)
            .with_hands_discards(4, 3)
    }

    /// Create a winning scenario (high score, good jokers)
    pub fn winning_position() -> MockGameBuilder {
        MockGameBuilder::new()
            .with_money(100)
            .with_ante_round(8, 2)
            .with_score(50000)
            .with_stage(Stage::Blind)
            .with_jokers(vec![
                JokerId::Baron,
                JokerId::TribouilletBaron,
                JokerId::Blueprint,
                JokerId::Brainstorm,
                JokerId::Cavendish,
            ])
            .with_hands_discards(6, 4)
            .with_planet_level(HandType::Flush, 5)
            .with_planet_level(HandType::FullHouse, 4)
    }

    /// Create a losing scenario (low resources, bad position)
    pub fn losing_position() -> MockGameBuilder {
        MockGameBuilder::new()
            .with_money(0)
            .with_ante_round(5, 3)
            .with_score(2000)
            .with_stage(Stage::Blind)
            .with_hands_discards(1, 0)
            .with_boss_blind(BlindType::CrimsonHeart)
    }

    /// Create an edge case scenario (maximum values)
    pub fn edge_case_max() -> MockGameBuilder {
        MockGameBuilder::new()
            .with_money(999999)
            .with_ante_round(20, 3)
            .with_score(999999999)
            .with_hands_discards(99, 99)
            .with_jokers(vec![JokerId::Joker; 5])
    }

    /// Create a scenario for testing specific joker interactions
    pub fn joker_synergy_test() -> MockGameBuilder {
        MockGameBuilder::new()
            .with_money(50)
            .with_ante_round(3, 2)
            .with_jokers(vec![
                JokerId::Mime,        // Retrigger
                JokerId::Baron,       // Kings held in hand
                JokerId::SteelJoker,  // +X Mult for Steel cards
                JokerId::Holographic, // +X Mult
            ])
            .with_hand(vec![
                Card::new(Suit::Spades, Rank::King),
                Card::new(Suit::Hearts, Rank::King),
                Card::new(Suit::Diamonds, Rank::King),
                Card::new(Suit::Clubs, Rank::King),
                Card::new(Suit::Spades, Rank::Ace),
            ])
    }
}

/// State snapshot for saving and restoring game state
#[derive(Debug, Clone)]
pub struct StateSnapshot {
    money: i32,
    ante: u8,
    round: u8,
    score: i64,
    stage: Stage,
    hands_remaining: u8,
    discards_remaining: u8,
    joker_ids: Vec<JokerId>,
    hand_size: usize,
    deck_size: usize,
    timestamp: std::time::Instant,
    label: String,
}

impl StateSnapshot {
    /// Create a snapshot from a game state
    pub fn from_game(game: &Game, label: impl Into<String>) -> Self {
        // This would need to extract state from the actual Game
        // For now, returning a placeholder
        Self {
            money: 0,
            ante: 1,
            round: 1,
            score: 0,
            stage: Stage::PreBlind,
            hands_remaining: 4,
            discards_remaining: 3,
            joker_ids: Vec::new(),
            hand_size: 0,
            deck_size: 52,
            timestamp: std::time::Instant::now(),
            label: label.into(),
        }
    }

    /// Compare this snapshot with another
    pub fn diff(&self, other: &StateSnapshot) -> SnapshotDiff {
        SnapshotDiff {
            money_change: other.money - self.money,
            score_change: other.score - self.score,
            stage_changed: self.stage != other.stage,
            jokers_changed: self.joker_ids != other.joker_ids,
            hand_size_change: other.hand_size as i32 - self.hand_size as i32,
            deck_size_change: other.deck_size as i32 - self.deck_size as i32,
        }
    }
}

/// Difference between two snapshots
#[derive(Debug)]
pub struct SnapshotDiff {
    pub money_change: i32,
    pub score_change: i64,
    pub stage_changed: bool,
    pub jokers_changed: bool,
    pub hand_size_change: i32,
    pub deck_size_change: i32,
}

/// Manager for tracking state transitions during tests
pub struct StateTransitionTracker {
    snapshots: Vec<StateSnapshot>,
    max_snapshots: usize,
}

impl StateTransitionTracker {
    /// Create a new tracker
    pub fn new() -> Self {
        Self {
            snapshots: Vec::new(),
            max_snapshots: 100,
        }
    }

    /// Record a state snapshot
    pub fn record(&mut self, game: &Game, label: impl Into<String>) {
        if self.snapshots.len() >= self.max_snapshots {
            self.snapshots.remove(0);
        }
        self.snapshots.push(StateSnapshot::from_game(game, label));
    }

    /// Get all snapshots
    pub fn snapshots(&self) -> &[StateSnapshot] {
        &self.snapshots
    }

    /// Find snapshot by label
    pub fn find_by_label(&self, label: &str) -> Option<&StateSnapshot> {
        self.snapshots.iter().find(|s| s.label == label)
    }

    /// Get transition summary between two points
    pub fn transition_summary(&self, from_idx: usize, to_idx: usize) -> Option<SnapshotDiff> {
        let from = self.snapshots.get(from_idx)?;
        let to = self.snapshots.get(to_idx)?;
        Some(from.diff(to))
    }

    /// Export state history for debugging
    pub fn export_history(&self) -> String {
        let mut output = String::new();
        output.push_str("=== State Transition History ===\n");

        for (i, snapshot) in self.snapshots.iter().enumerate() {
            output.push_str(&format!(
                "[{}] {} - Money: {}, Score: {}, Stage: {:?}\n",
                i, snapshot.label, snapshot.money, snapshot.score, snapshot.stage
            ));
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_builder_basic() {
        let builder = MockGameBuilder::new()
            .with_money(100)
            .with_ante_round(5, 2)
            .with_score(10000)
            .with_jokers(vec![JokerId::Joker, JokerId::Baron]);

        // The builder is set up correctly
        assert_eq!(builder.money, 100);
        assert_eq!(builder.ante, 5);
        assert_eq!(builder.round, 2);
        assert_eq!(builder.score, 10000);
        assert_eq!(builder.jokers.len(), 2);
    }

    #[test]
    fn test_game_scenarios() {
        let new_run = GameScenario::new_run();
        assert_eq!(new_run.money, 4);
        assert_eq!(new_run.ante, 1);

        let mid_blind = GameScenario::mid_blind();
        assert_eq!(mid_blind.stage, Stage::Blind);
        assert_eq!(mid_blind.jokers.len(), 2);

        let shop = GameScenario::at_shop();
        assert_eq!(shop.stage, Stage::Shop);
        assert!(shop.money > 20);

        let boss = GameScenario::boss_blind();
        assert!(boss.boss_blind.is_some());

        let winning = GameScenario::winning_position();
        assert!(winning.money > 50);
        assert!(winning.score > 10000);
        assert!(winning.jokers.len() > 3);

        let losing = GameScenario::losing_position();
        assert_eq!(losing.money, 0);
        assert_eq!(losing.hands_remaining, 1);
        assert_eq!(losing.discards_remaining, 0);
    }

    #[test]
    fn test_state_snapshots() {
        let game = Game::new();
        let snapshot1 = StateSnapshot::from_game(&game, "initial");
        let snapshot2 = StateSnapshot::from_game(&game, "after_action");

        let diff = snapshot1.diff(&snapshot2);
        assert_eq!(diff.money_change, 0);
        assert_eq!(diff.score_change, 0);
        assert!(!diff.stage_changed);
    }

    #[test]
    fn test_state_transition_tracker() {
        let mut tracker = StateTransitionTracker::new();
        let game = Game::new();

        tracker.record(&game, "start");
        tracker.record(&game, "middle");
        tracker.record(&game, "end");

        assert_eq!(tracker.snapshots().len(), 3);
        assert!(tracker.find_by_label("middle").is_some());

        let history = tracker.export_history();
        assert!(history.contains("start"));
        assert!(history.contains("middle"));
        assert!(history.contains("end"));
    }
}
