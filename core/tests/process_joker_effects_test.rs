/// Comprehensive tests for process_joker_effects function (Issue #370)
/// These tests document the current behavior before refactoring to ensure backward compatibility
use balatro_rs::card::{Card, Suit, Value};
use balatro_rs::game::Game;
use balatro_rs::hand::SelectHand;
use balatro_rs::joker::{GameContext, Joker, JokerEffect, JokerId, JokerRarity};
use balatro_rs::stage::{Blind, Stage};

#[test]
fn test_empty_joker_list() {
    let mut game = Game::default();
    game.start();
    game.stage = Stage::Blind(Blind::Small);
    game.blind = Some(Blind::Small);

    // Ensure no jokers
    game.jokers.clear();

    let cards = vec![Card::new(Value::Ace, Suit::Heart)];
    let hand = SelectHand::new(cards).best_hand().unwrap();

    let (chips, mult, money, mult_multiplier, messages) = game.process_joker_effects(&hand);

    // With no jokers, should return zero values
    assert_eq!(chips, 0);
    assert_eq!(mult, 0);
    assert_eq!(money, 0);
    assert_eq!(mult_multiplier, 1.0); // Default multiplier
    assert_eq!(messages.len(), 0);
}

#[test]
fn test_single_hand_level_joker() {
    let mut game = Game::default();
    game.start();
    game.stage = Stage::Blind(Blind::Small);
    game.blind = Some(Blind::Small);

    // Add joker that only affects hand-level
    let joker = Box::new(TestHandLevelJoker::new(10, 5, 2, 1.5, 0));
    game.jokers = vec![joker];

    let cards = vec![Card::new(Value::Ace, Suit::Heart)];
    let hand = SelectHand::new(cards).best_hand().unwrap();

    let (chips, mult, money, mult_multiplier, messages) = game.process_joker_effects(&hand);

    // Should include hand-level effects
    assert_eq!(chips, 10);
    assert_eq!(mult, 5);
    assert_eq!(money, 2);
    assert_eq!(mult_multiplier, 1.5);

    // Debug messages are generated in debug builds
    #[cfg(debug_assertions)]
    {
        assert!(messages.len() > 0);
        assert!(messages[0].contains("Hand effects"));
    }

    #[cfg(not(debug_assertions))]
    {
        assert_eq!(messages.len(), 0);
    }
}

#[test]
fn test_single_card_level_joker() {
    let mut game = Game::default();
    game.start();
    game.stage = Stage::Blind(Blind::Small);
    game.blind = Some(Blind::Small);

    // Add joker that only affects card-level
    let joker = Box::new(TestCardLevelJoker::new(5, 3, 1, 1.0, 0));
    game.jokers = vec![joker];

    // Create a pair so both cards are included in the made hand
    let cards = vec![
        Card::new(Value::Ace, Suit::Heart),
        Card::new(Value::Ace, Suit::Spade),
    ];
    let hand = SelectHand::new(cards).best_hand().unwrap();

    let (chips, mult, money, mult_multiplier, messages) = game.process_joker_effects(&hand);

    // Should include effects from both cards (2 cards * joker effects)
    assert_eq!(chips, 10); // 5 chips * 2 cards
    assert_eq!(mult, 6); // 3 mult * 2 cards
    assert_eq!(money, 2); // 1 money * 2 cards
    assert_eq!(mult_multiplier, 1.0); // No multiplier

    // Debug messages are generated in debug builds (one per card)
    #[cfg(debug_assertions)]
    {
        assert!(messages.len() > 0);
    }

    #[cfg(not(debug_assertions))]
    {
        assert_eq!(messages.len(), 0);
    }
}

#[test]
fn test_mixed_hand_and_card_level_jokers() {
    let mut game = Game::default();
    game.start();
    game.stage = Stage::Blind(Blind::Small);
    game.blind = Some(Blind::Small);

    // Add both hand-level and card-level jokers
    let hand_joker = Box::new(TestHandLevelJoker::new(10, 5, 2, 1.5, 0));
    let card_joker = Box::new(TestCardLevelJoker::new(3, 2, 1, 1.0, 0));
    game.jokers = vec![hand_joker, card_joker];

    // Create a pair
    let cards = vec![
        Card::new(Value::King, Suit::Heart),
        Card::new(Value::King, Suit::Diamond),
    ];
    let hand = SelectHand::new(cards).best_hand().unwrap();

    let (chips, mult, money, mult_multiplier, messages) = game.process_joker_effects(&hand);

    // Should include both hand-level and card-level effects
    // Hand: 10 chips, 5 mult, 2 money
    // Cards: 3 chips * 2 cards = 6, 2 mult * 2 cards = 4, 1 money * 2 cards = 2
    assert_eq!(chips, 16); // 10 + 6
    assert_eq!(mult, 9); // 5 + 4
    assert_eq!(money, 4); // 2 + 2
    assert_eq!(mult_multiplier, 1.5); // From hand joker

    #[cfg(debug_assertions)]
    {
        assert!(messages.len() > 0);
    }
}

#[test]
fn test_retrigger_effects() {
    let mut game = Game::default();
    game.start();
    game.stage = Stage::Blind(Blind::Small);
    game.blind = Some(Blind::Small);

    // Add joker with retrigger
    let joker = Box::new(TestRetriggerJoker::new(10, 5, 2, 2)); // 2 retriggers
    game.jokers = vec![joker];

    let cards = vec![Card::new(Value::Ace, Suit::Heart)];
    let hand = SelectHand::new(cards).best_hand().unwrap();

    let (chips, mult, money, mult_multiplier, _messages) = game.process_joker_effects(&hand);

    // With 2 retriggers, effect triggers 3 times total (1 + 2)
    assert_eq!(chips, 30); // 10 * 3
    assert_eq!(mult, 15); // 5 * 3
    assert_eq!(money, 6); // 2 * 3
    assert_eq!(mult_multiplier, 1.0); // No multiplier
}

#[test]
fn test_mult_multiplier_accumulation() {
    let mut game = Game::default();
    game.start();
    game.stage = Stage::Blind(Blind::Small);
    game.blind = Some(Blind::Small);

    // Add multiple jokers with mult multipliers
    let joker1 = Box::new(TestHandLevelJoker::new(0, 0, 0, 2.0, 0));
    let joker2 = Box::new(TestHandLevelJoker::new(0, 0, 0, 1.5, 0));
    game.jokers = vec![joker1, joker2];

    let cards = vec![Card::new(Value::Ace, Suit::Heart)];
    let hand = SelectHand::new(cards).best_hand().unwrap();

    let (_chips, _mult, _money, mult_multiplier, _messages) = game.process_joker_effects(&hand);

    // Multipliers accumulate multiplicatively: 1.0 * 2.0 * 1.5 = 3.0
    assert_eq!(mult_multiplier, 3.0);
}

#[test]
fn test_zero_mult_multiplier_edge_case() {
    let mut game = Game::default();
    game.start();
    game.stage = Stage::Blind(Blind::Small);
    game.blind = Some(Blind::Small);

    // Add joker with 0.0 mult_multiplier (should be treated as no multiplier)
    let joker = Box::new(TestHandLevelJoker::new(10, 5, 2, 0.0, 0));
    game.jokers = vec![joker];

    let cards = vec![Card::new(Value::Ace, Suit::Heart)];
    let hand = SelectHand::new(cards).best_hand().unwrap();

    let (chips, mult, money, mult_multiplier, _messages) = game.process_joker_effects(&hand);

    // Zero mult_multiplier should be treated as 1.0 (no effect)
    assert_eq!(chips, 10);
    assert_eq!(mult, 5);
    assert_eq!(money, 2);
    assert_eq!(mult_multiplier, 1.0); // Not 0.0
}

#[test]
fn test_large_number_handling() {
    let mut game = Game::default();
    game.start();
    game.stage = Stage::Blind(Blind::Small);
    game.blind = Some(Blind::Small);

    // Add joker with very large values
    let joker = Box::new(TestHandLevelJoker::new(
        1_000_000, 500_000, 100_000, 100.0, 0,
    ));
    game.jokers = vec![joker];

    let cards = vec![Card::new(Value::Ace, Suit::Heart)];
    let hand = SelectHand::new(cards).best_hand().unwrap();

    let (chips, mult, money, mult_multiplier, _messages) = game.process_joker_effects(&hand);

    // Should handle large numbers without panic
    assert_eq!(chips, 1_000_000);
    assert_eq!(mult, 500_000);
    assert_eq!(money, 100_000);
    assert_eq!(mult_multiplier, 100.0);
}

#[test]
fn test_killscreen_detection() {
    let mut game = Game::default();
    game.start();
    game.stage = Stage::Blind(Blind::Small);
    game.blind = Some(Blind::Small);

    // Add card-level joker with very high mult_multiplier that causes infinity
    // KILLSCREEN messages are only generated for card-level effects
    let joker = Box::new(TestCardLevelJoker::new(0, 0, 0, f64::MAX, 0));
    let joker2 = Box::new(TestCardLevelJoker::new(0, 0, 0, 2.0, 0));
    game.jokers = vec![joker, joker2];

    let cards = vec![Card::new(Value::Ace, Suit::Heart)];
    let hand = SelectHand::new(cards).best_hand().unwrap();

    let (_chips, _mult, _money, mult_multiplier, messages) = game.process_joker_effects(&hand);

    // Should detect killscreen (infinity)
    assert!(mult_multiplier.is_infinite());
    assert!(messages.iter().any(|msg| msg.contains("KILLSCREEN")));
}

#[test]
fn test_debug_message_generation() {
    let mut game = Game::default();
    game.start();
    game.stage = Stage::Blind(Blind::Small);
    game.blind = Some(Blind::Small);

    // Add joker that generates debug messages
    let joker = Box::new(TestDebugJoker::new(10, 5, 2));
    game.jokers = vec![joker];

    let cards = vec![Card::new(Value::Ace, Suit::Heart)];
    let hand = SelectHand::new(cards).best_hand().unwrap();

    let (chips, mult, money, _mult_multiplier, messages) = game.process_joker_effects(&hand);

    assert_eq!(chips, 10);
    assert_eq!(mult, 5);
    assert_eq!(money, 2);

    // Debug messages should be generated in debug builds
    #[cfg(debug_assertions)]
    {
        assert!(!messages.is_empty());
        assert!(messages.iter().any(|msg| msg.contains("Hand effects")));
    }
}

#[test]
fn test_multiple_jokers_with_retriggers() {
    let mut game = Game::default();
    game.start();
    game.stage = Stage::Blind(Blind::Small);
    game.blind = Some(Blind::Small);

    // Add multiple jokers with different retrigger counts
    let joker1 = Box::new(TestRetriggerJoker::new(5, 2, 1, 1)); // 1 retrigger
    let joker2 = Box::new(TestRetriggerJoker::new(3, 3, 0, 2)); // 2 retriggers
    game.jokers = vec![joker1, joker2];

    let cards = vec![Card::new(Value::Ace, Suit::Heart)];
    let hand = SelectHand::new(cards).best_hand().unwrap();

    let (chips, mult, money, _mult_multiplier, _messages) = game.process_joker_effects(&hand);

    // Joker1: 5 chips * 2 (1+1) = 10, 2 mult * 2 = 4, 1 money * 2 = 2
    // Joker2: 3 chips * 3 (1+2) = 9, 3 mult * 3 = 9, 0 money * 3 = 0
    assert_eq!(chips, 19); // 10 + 9
    assert_eq!(mult, 13); // 4 + 9
    assert_eq!(money, 2); // 2 + 0
}

#[test]
fn test_joker_evaluation_order() {
    let mut game = Game::default();
    game.start();
    game.stage = Stage::Blind(Blind::Small);
    game.blind = Some(Blind::Small);

    // Add jokers in specific order
    let joker1 = Box::new(TestHandLevelJoker::new(10, 0, 0, 2.0, 0));
    let joker2 = Box::new(TestHandLevelJoker::new(5, 0, 0, 3.0, 0));
    let joker3 = Box::new(TestHandLevelJoker::new(1, 0, 0, 1.5, 0));
    game.jokers = vec![joker1, joker2, joker3];

    let cards = vec![Card::new(Value::Ace, Suit::Heart)];
    let hand = SelectHand::new(cards).best_hand().unwrap();

    let (chips, _mult, _money, mult_multiplier, _messages) = game.process_joker_effects(&hand);

    // Jokers are evaluated in order
    assert_eq!(chips, 16); // 10 + 5 + 1
                           // Multipliers accumulate: 1.0 * 2.0 * 3.0 * 1.5 = 9.0
    assert_eq!(mult_multiplier, 9.0);
}

// Test helper jokers for comprehensive testing

/// Test joker that provides hand-level effects only
#[derive(Debug)]
struct TestHandLevelJoker {
    chips: i32,
    mult: i32,
    money: i32,
    mult_multiplier: f64,
    retrigger: u32,
}

impl TestHandLevelJoker {
    fn new(chips: i32, mult: i32, money: i32, mult_multiplier: f64, retrigger: u32) -> Self {
        Self {
            chips,
            mult,
            money,
            mult_multiplier,
            retrigger,
        }
    }
}

impl Joker for TestHandLevelJoker {
    fn id(&self) -> JokerId {
        JokerId::Joker
    }

    fn name(&self) -> &str {
        "Test Hand Level Joker"
    }

    fn description(&self) -> &str {
        "Test joker for hand-level effects"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Common
    }

    fn on_hand_played(&self, _context: &mut GameContext, _hand: &SelectHand) -> JokerEffect {
        JokerEffect {
            chips: self.chips,
            mult: self.mult,
            money: self.money,
            mult_multiplier: self.mult_multiplier,
            retrigger: self.retrigger,
            ..Default::default()
        }
    }
}

/// Test joker that provides card-level effects only
#[derive(Debug)]
struct TestCardLevelJoker {
    chips: i32,
    mult: i32,
    money: i32,
    mult_multiplier: f64,
    retrigger: u32,
}

impl TestCardLevelJoker {
    fn new(chips: i32, mult: i32, money: i32, mult_multiplier: f64, retrigger: u32) -> Self {
        Self {
            chips,
            mult,
            money,
            mult_multiplier,
            retrigger,
        }
    }
}

impl Joker for TestCardLevelJoker {
    fn id(&self) -> JokerId {
        JokerId::Joker
    }

    fn name(&self) -> &str {
        "Test Card Level Joker"
    }

    fn description(&self) -> &str {
        "Test joker for card-level effects"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Common
    }

    fn on_card_scored(&self, _context: &mut GameContext, _card: &Card) -> JokerEffect {
        JokerEffect {
            chips: self.chips,
            mult: self.mult,
            money: self.money,
            mult_multiplier: self.mult_multiplier,
            retrigger: self.retrigger,
            ..Default::default()
        }
    }
}

/// Test joker that provides retrigger effects
#[derive(Debug)]
struct TestRetriggerJoker {
    chips: i32,
    mult: i32,
    money: i32,
    retrigger_count: u32,
}

impl TestRetriggerJoker {
    fn new(chips: i32, mult: i32, money: i32, retrigger_count: u32) -> Self {
        Self {
            chips,
            mult,
            money,
            retrigger_count,
        }
    }
}

impl Joker for TestRetriggerJoker {
    fn id(&self) -> JokerId {
        JokerId::Joker
    }

    fn name(&self) -> &str {
        "Test Retrigger Joker"
    }

    fn description(&self) -> &str {
        "Test joker for retrigger mechanics"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Common
    }

    fn on_hand_played(&self, _context: &mut GameContext, _hand: &SelectHand) -> JokerEffect {
        JokerEffect {
            chips: self.chips,
            mult: self.mult,
            money: self.money,
            retrigger: self.retrigger_count,
            ..Default::default()
        }
    }
}

/// Test joker for debug message generation
#[derive(Debug)]
struct TestDebugJoker {
    chips: i32,
    mult: i32,
    money: i32,
}

impl TestDebugJoker {
    fn new(chips: i32, mult: i32, money: i32) -> Self {
        Self { chips, mult, money }
    }
}

impl Joker for TestDebugJoker {
    fn id(&self) -> JokerId {
        JokerId::Joker
    }

    fn name(&self) -> &str {
        "TestDebugJoker"
    }

    fn description(&self) -> &str {
        "Test joker for debug message generation"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Common
    }

    fn on_hand_played(&self, _context: &mut GameContext, _hand: &SelectHand) -> JokerEffect {
        JokerEffect {
            chips: self.chips,
            mult: self.mult,
            money: self.money,
            ..Default::default()
        }
    }
}
