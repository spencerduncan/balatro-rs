/// Acceptance tests for joker scoring system integration (Issue #55)
/// These tests capture the expected behavior for the enhanced joker scoring pipeline
use balatro_rs::card::{Card, Suit, Value};
use balatro_rs::game::Game;
use balatro_rs::hand::SelectHand;
use balatro_rs::joker::{GameContext, Joker, JokerEffect, JokerId, JokerRarity};
use balatro_rs::joker_factory::JokerFactory;
use balatro_rs::stage::{Blind, Stage};
use std::time::Instant;

#[test]
fn test_joker_left_to_right_evaluation_order() {
    let mut game = Game::default();
    game.start();
    game.stage = Stage::Blind(Blind::Small);
    game.blind = Some(Blind::Small);

    // Create jokers that modify each other's effects
    // First joker adds +10 mult, second joker doubles mult bonuses
    let joker1 = Box::new(TestOrderJoker::new(1, 10, 0, 1.0));
    let joker2 = Box::new(TestOrderJoker::new(2, 0, 0, 2.0));

    // Add jokers in specific left-to-right order
    game.jokers = vec![joker1, joker2];

    let cards = vec![Card::new(Value::Ace, Suit::Heart)];
    let hand = SelectHand::new(cards).best_hand().unwrap();

    let score = game.calc_score(hand);

    // Expected: Joker1 adds +10 mult, then Joker2 doubles it to +20
    // Base: (5 + 11) * (1 + 20) = 16 * 21 = 336
    assert_eq!(score, 336.0);
}

#[test]
fn test_lifecycle_hooks_at_correct_phases() {
    let mut game = Game::default();
    game.start();
    game.stage = Stage::Blind(Blind::Small);
    game.blind = Some(Blind::Small);

    // Add joker that responds to different lifecycle events
    let joker = Box::new(TestLifecycleJoker::new());
    game.jokers = vec![joker];

    // Test blind start hook
    let initial_money = game.money;
    game.start_blind();
    assert!(
        game.money > initial_money,
        "on_blind_start should have added money"
    );

    // Test hand played hook
    let cards = vec![Card::new(Value::Ace, Suit::Heart)];
    let hand = SelectHand::new(cards).best_hand().unwrap();
    let score = game.calc_score(hand);

    // Should include effects from on_hand_played
    assert!(
        score > 16.0,
        "on_hand_played should have contributed to score"
    );
}

#[test]
fn test_retrigger_mechanics_with_stacking() {
    let mut game = Game::default();
    game.start();
    game.stage = Stage::Blind(Blind::Small);
    game.blind = Some(Blind::Small);

    // Add joker that triggers retriggers
    let joker = Box::new(TestRetriggerJoker::new(2)); // 2 additional triggers
    game.jokers = vec![joker];

    let cards = vec![Card::new(Value::Ace, Suit::Heart)];
    let hand = SelectHand::new(cards).best_hand().unwrap();
    let score = game.calc_score(hand);

    // Expected: Base effect (10 mult) * 3 total triggers = 30 mult
    // Base: (5 + 11) * (1 + 30) = 16 * 31 = 496
    assert_eq!(score, 496.0);
}

#[test]
fn test_score_breakdown_tracking() {
    let mut game = Game::default();
    game.start();
    game.stage = Stage::Blind(Blind::Small);
    game.blind = Some(Blind::Small);

    // Add multiple jokers with different effects
    let joker1 = JokerFactory::create(JokerId::Joker).expect("Can create joker");
    let joker2 = JokerFactory::create(JokerId::GreedyJoker).expect("Can create joker");
    game.jokers = vec![joker1, joker2];

    let cards = vec![
        Card::new(Value::Ace, Suit::Diamond),
        Card::new(Value::King, Suit::Heart),
    ];
    let hand = SelectHand::new(cards).best_hand().unwrap();

    // This test should pass once score breakdown is implemented
    let breakdown = game.calc_score_with_breakdown(hand);

    assert!(breakdown.base_chips > 0.0);
    assert!(breakdown.base_mult > 0.0);
    assert!(breakdown.card_chips > 0.0);
    assert!(breakdown.joker_contributions.len() > 0);
    assert!(breakdown.final_score > 0.0);
}

#[test]
fn test_debug_logging_for_joker_contributions() {
    let mut game = Game::default();
    game.start();
    game.stage = Stage::Blind(Blind::Small);
    game.blind = Some(Blind::Small);

    // Enable debug logging
    game.enable_debug_logging();

    let joker = JokerFactory::create(JokerId::Joker).expect("Can create joker");
    game.jokers = vec![joker];

    let cards = vec![Card::new(Value::Ace, Suit::Heart)];
    let hand = SelectHand::new(cards).best_hand().unwrap();

    let _score = game.calc_score(hand);

    // Check that debug messages were logged
    let debug_messages = game.get_debug_messages();
    assert!(!debug_messages.is_empty(), "Should have debug messages");
    assert!(
        debug_messages.iter().any(|msg| msg.contains("Joker")),
        "Should log joker effects"
    );
}

#[test]
fn test_performance_under_1ms() {
    let mut game = Game::default();
    game.start();
    game.stage = Stage::Blind(Blind::Small);
    game.blind = Some(Blind::Small);

    // Add maximum number of jokers (5 in Balatro)
    for _ in 0..5 {
        let joker = JokerFactory::create(JokerId::Joker).expect("Can create joker");
        game.jokers.push(joker);
    }

    let cards = vec![
        Card::new(Value::Ace, Suit::Heart),
        Card::new(Value::King, Suit::Spade),
        Card::new(Value::Queen, Suit::Diamond),
        Card::new(Value::Jack, Suit::Club),
        Card::new(Value::Ten, Suit::Heart),
    ];
    let hand = SelectHand::new(cards).best_hand().unwrap();

    let start = Instant::now();
    let _score = game.calc_score(hand);
    let duration = start.elapsed();

    assert!(
        duration.as_millis() < 1,
        "Scoring should complete in under 1ms"
    );
}

#[test]
fn test_killscreen_behavior() {
    let mut game = Game::default();
    game.start();
    game.stage = Stage::Blind(Blind::Small);
    game.blind = Some(Blind::Small);
    game.enable_debug_logging();

    // Create jokers with extreme multipliers that will cause killscreen
    // 1e200 * 1e200 = 1e400 which exceeds f64 max (~1.8e308) and becomes infinity
    let extreme_joker = Box::new(TestOrderJoker::new(1, 0, 0, 1e200));
    game.jokers.push(extreme_joker);

    // Add a second joker with the same extreme multiplier to ensure overflow
    let extreme_joker2 = Box::new(TestOrderJoker::new(2, 0, 0, 1e200));
    game.jokers.push(extreme_joker2);

    let cards = vec![Card::new(Value::Ace, Suit::Heart)];
    let hand = SelectHand::new(cards).best_hand().unwrap();

    let score = game.calc_score(hand);

    // Debug: print the score and debug messages to understand what's happening
    println!("Score: {}, is_finite: {}", score, score.is_finite());
    let debug_messages = game.get_debug_messages();
    for msg in debug_messages {
        println!("Debug: {}", msg);
    }

    // Score should be infinite (killscreen reached) OR we should have killscreen message
    let has_killscreen_msg = debug_messages.iter().any(|msg| msg.contains("KILLSCREEN"));

    // Test passes if either score is infinite OR we got a killscreen message
    assert!(
        !score.is_finite() || has_killscreen_msg,
        "Should reach killscreen either through infinite score ({}) or killscreen detection. Messages: {:?}", 
        score, debug_messages
    );
}

#[test]
fn test_complex_joker_interaction_scenario() {
    let mut game = Game::default();
    game.start();
    game.stage = Stage::Blind(Blind::Small);
    game.blind = Some(Blind::Small);

    // Complex scenario: Multiple jokers with different trigger conditions
    let joker1 = JokerFactory::create(JokerId::Joker).expect("Basic mult joker");
    let joker2 = JokerFactory::create(JokerId::GreedyJoker).expect("Diamond suit joker");
    let joker3 = Box::new(TestRetriggerJoker::new(1)); // Retrigger joker

    game.jokers = vec![joker1, joker2, joker3];

    let cards = vec![
        Card::new(Value::Ace, Suit::Diamond),  // Triggers GreedyJoker
        Card::new(Value::King, Suit::Diamond), // Triggers GreedyJoker
        Card::new(Value::Queen, Suit::Heart),
    ];
    let hand = SelectHand::new(cards).best_hand().unwrap();

    let score = game.calc_score(hand);

    // This is a complex calculation involving:
    // - Base hand chips/mult
    // - Card contributions
    // - Multiple joker effects
    // - Retrigger mechanics
    // The exact value depends on the final implementation
    assert!(
        score > 100.0,
        "Complex joker interactions should produce significant score"
    );
}

// Test helper jokers for acceptance testing

#[derive(Debug)]
struct TestOrderJoker {
    position: usize,
    mult_bonus: i32,
    chip_bonus: i32,
    mult_multiplier: f64,
}

impl TestOrderJoker {
    fn new(position: usize, mult_bonus: i32, chip_bonus: i32, mult_multiplier: f64) -> Self {
        Self {
            position,
            mult_bonus,
            chip_bonus,
            mult_multiplier,
        }
    }
}

impl Joker for TestOrderJoker {
    fn id(&self) -> JokerId {
        JokerId::Joker
    }
    fn name(&self) -> &str {
        "Test Order Joker"
    }
    fn description(&self) -> &str {
        "Tests left-to-right evaluation order"
    }
    fn rarity(&self) -> JokerRarity {
        JokerRarity::Common
    }

    fn on_hand_played(&self, _context: &mut GameContext, _hand: &SelectHand) -> JokerEffect {
        JokerEffect::new()
            .with_chips(self.chip_bonus)
            .with_mult(self.mult_bonus)
            .with_mult_multiplier(self.mult_multiplier as f64)
    }
}

#[derive(Debug)]
struct TestLifecycleJoker;

impl TestLifecycleJoker {
    fn new() -> Self {
        Self
    }
}

impl Joker for TestLifecycleJoker {
    fn id(&self) -> JokerId {
        JokerId::Joker
    }
    fn name(&self) -> &str {
        "Test Lifecycle Joker"
    }
    fn description(&self) -> &str {
        "Tests lifecycle hook integration"
    }
    fn rarity(&self) -> JokerRarity {
        JokerRarity::Common
    }

    fn on_blind_start(&self, _context: &mut GameContext) -> JokerEffect {
        JokerEffect::new().with_money(10)
    }

    fn on_hand_played(&self, _context: &mut GameContext, _hand: &SelectHand) -> JokerEffect {
        JokerEffect::new().with_mult(5)
    }
}

#[derive(Debug)]
struct TestRetriggerJoker {
    retrigger_count: u32,
}

impl TestRetriggerJoker {
    fn new(retrigger_count: u32) -> Self {
        Self { retrigger_count }
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
        "Tests retrigger mechanics"
    }
    fn rarity(&self) -> JokerRarity {
        JokerRarity::Common
    }

    fn on_hand_played(&self, _context: &mut GameContext, _hand: &SelectHand) -> JokerEffect {
        JokerEffect::new()
            .with_mult(10)
            .with_retrigger(self.retrigger_count)
    }
}
