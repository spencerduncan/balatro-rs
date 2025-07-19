//! Integration tests for Boss Blind Trait Definition (Issue #28)
//!
//! Tests the BossBlind trait, BlindEffect enum, CounterType enum, and related functionality
//! following TDD Red-Green-Refactor cycle.

use balatro_rs::boss_blinds::{BlindEffect, BossBlind, CounterType};
use balatro_rs::game::Game;
use serde_json;

/// Mock implementation for testing the BossBlind trait
#[derive(Debug)]
struct TestBossBlind {
    name: &'static str,
    effects: Vec<BlindEffect>,
    min_ante: u32,
}

impl TestBossBlind {
    fn new(name: &'static str, effects: Vec<BlindEffect>, min_ante: u32) -> Self {
        Self {
            name,
            effects,
            min_ante,
        }
    }
}

impl BossBlind for TestBossBlind {
    fn name(&self) -> &str {
        self.name
    }

    fn apply_effects(&self, _game_state: &mut Game) {
        // Test implementation - would apply actual effects in real implementation
    }

    fn check_counters(&self, _game_state: &Game) -> Vec<CounterType> {
        // Test implementation - would check actual counters in real implementation
        vec![CounterType::HandsPlayed, CounterType::CardsScored]
    }

    fn get_effects(&self) -> Vec<BlindEffect> {
        self.effects.clone()
    }

    fn min_ante(&self) -> u32 {
        self.min_ante
    }
}

#[test]
fn test_boss_blind_trait_basic_functionality() {
    let effects = vec![
        BlindEffect::ModifyScoring("score * 0.5".to_string()),
        BlindEffect::SpecialRule("Hearts are disabled".to_string()),
    ];

    let boss_blind = TestBossBlind::new("Test Boss", effects.clone(), 3);

    // Test basic trait methods
    assert_eq!(boss_blind.name(), "Test Boss");
    assert_eq!(boss_blind.min_ante(), 3);
    assert_eq!(boss_blind.get_effects(), effects);

    // Test that it implements required traits
    assert!(std::ptr::eq(
        &boss_blind as &dyn std::fmt::Debug,
        &boss_blind as &dyn std::fmt::Debug
    ));
}

#[test]
fn test_boss_blind_trait_with_game_state() {
    let mut game = Game::default();
    let boss_blind = TestBossBlind::new("State Test Boss", vec![], 1);

    // Test apply_effects doesn't panic
    boss_blind.apply_effects(&mut game);

    // Test check_counters returns expected values
    let counters = boss_blind.check_counters(&game);
    assert!(!counters.is_empty());
    assert!(counters.contains(&CounterType::HandsPlayed));
    assert!(counters.contains(&CounterType::CardsScored));
}

#[test]
fn test_blind_effect_enum_variants() {
    // Test all required BlindEffect variants exist and can be created
    let debuff_effect = BlindEffect::DebuffCards("face_cards".to_string());
    let restrict_effect = BlindEffect::RestrictActions("no_discards".to_string());
    let scoring_effect = BlindEffect::ModifyScoring("chips * 2".to_string());
    let special_effect = BlindEffect::SpecialRule("All cards are red".to_string());

    // Test that effects can be matched
    match debuff_effect {
        BlindEffect::DebuffCards(condition) => assert_eq!(condition, "face_cards"),
        _ => panic!("Wrong variant"),
    }

    match restrict_effect {
        BlindEffect::RestrictActions(restriction) => assert_eq!(restriction, "no_discards"),
        _ => panic!("Wrong variant"),
    }

    match scoring_effect {
        BlindEffect::ModifyScoring(modifier) => assert_eq!(modifier, "chips * 2"),
        _ => panic!("Wrong variant"),
    }

    match special_effect {
        BlindEffect::SpecialRule(rule) => assert_eq!(rule, "All cards are red"),
        _ => panic!("Wrong variant"),
    }
}

#[test]
fn test_counter_type_enum_variants() {
    // Test all required CounterType variants exist
    let hands_counter = CounterType::HandsPlayed;
    let cards_counter = CounterType::CardsScored;
    let money_counter = CounterType::MoneySpent;

    // Test that counters can be matched and used in collections
    let counters = vec![hands_counter, cards_counter, money_counter];
    assert_eq!(counters.len(), 3);

    // Test that counters implement required traits for collections
    assert!(counters.contains(&CounterType::HandsPlayed));
    assert!(counters.contains(&CounterType::CardsScored));
    assert!(counters.contains(&CounterType::MoneySpent));
}

#[test]
fn test_blind_effect_serialization() {
    let effect = BlindEffect::ModifyScoring("mult * 1.5".to_string());

    // Test serialization
    let serialized = serde_json::to_string(&effect).expect("Should serialize");
    assert!(!serialized.is_empty());

    // Test deserialization
    let deserialized: BlindEffect = serde_json::from_str(&serialized).expect("Should deserialize");

    match deserialized {
        BlindEffect::ModifyScoring(modifier) => assert_eq!(modifier, "mult * 1.5"),
        _ => panic!("Wrong variant after deserialization"),
    }
}

#[test]
fn test_counter_type_serialization() {
    let counter = CounterType::HandsPlayed;

    // Test serialization
    let serialized = serde_json::to_string(&counter).expect("Should serialize");
    assert!(!serialized.is_empty());

    // Test deserialization
    let deserialized: CounterType = serde_json::from_str(&serialized).expect("Should deserialize");
    assert_eq!(deserialized, CounterType::HandsPlayed);
}

#[test]
fn test_boss_blind_trait_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}

    // This test ensures the trait requires Send + Sync
    // It will compile only if BossBlind trait has Send + Sync bounds
    assert_send_sync::<Box<dyn BossBlind>>();
}

#[test]
fn test_complex_boss_blind_scenario() {
    let mut game = Game::default();

    // Create a complex boss blind with multiple effects
    let effects = vec![
        BlindEffect::DebuffCards("clubs".to_string()),
        BlindEffect::RestrictActions("max_1_discard".to_string()),
        BlindEffect::ModifyScoring("score * 0.75".to_string()),
        BlindEffect::SpecialRule("First hand must be a pair".to_string()),
    ];

    let boss_blind = TestBossBlind::new("The Punishment", effects.clone(), 5);

    // Test all methods work together
    assert_eq!(boss_blind.name(), "The Punishment");
    assert_eq!(boss_blind.min_ante(), 5);
    assert_eq!(boss_blind.get_effects().len(), 4);

    // Apply effects (should not panic)
    boss_blind.apply_effects(&mut game);

    // Check counters
    let counters = boss_blind.check_counters(&game);
    assert!(!counters.is_empty());
}

#[test]
fn test_boss_blind_trait_object_usage() {
    // Test that the trait can be used as a trait object
    let boss_blind: Box<dyn BossBlind> = Box::new(TestBossBlind::new(
        "Trait Object Test",
        vec![BlindEffect::SpecialRule("Test rule".to_string())],
        2,
    ));

    assert_eq!(boss_blind.name(), "Trait Object Test");
    assert_eq!(boss_blind.min_ante(), 2);
    assert_eq!(boss_blind.get_effects().len(), 1);

    // Test that we can store multiple boss blinds as trait objects
    let boss_blinds: Vec<Box<dyn BossBlind>> = vec![
        Box::new(TestBossBlind::new("Boss 1", vec![], 1)),
        Box::new(TestBossBlind::new("Boss 2", vec![], 2)),
        Box::new(TestBossBlind::new("Boss 3", vec![], 3)),
    ];

    assert_eq!(boss_blinds.len(), 3);
    assert_eq!(boss_blinds[0].name(), "Boss 1");
    assert_eq!(boss_blinds[1].name(), "Boss 2");
    assert_eq!(boss_blinds[2].name(), "Boss 3");
}
