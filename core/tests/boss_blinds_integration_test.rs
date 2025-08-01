//! Comprehensive Integration Tests for Boss Blinds
//!
//! Tests all boss blind implementations to ensure they work correctly with the
//! game engine and follow the established patterns.

use balatro_rs::boss_blinds::{
    BlindEffect, BossBlind, BossBlindId, CounterType, TheFlint, TheMark, TheNeedle, ThePlant,
    TheSerpent, TheWater, TheWheel,
};
use balatro_rs::card::{Card, Suit, Value};
use balatro_rs::config::Config;
use balatro_rs::game::Game;

/// Helper function to create a test card with the specified value
fn create_test_card(value: Value) -> Card {
    Card::new(value, Suit::Heart)
}

/// Helper function to create multiple test cards
fn create_test_cards(values: &[Value]) -> Vec<Card> {
    values.iter().map(|&v| create_test_card(v)).collect()
}

#[test]
fn test_all_boss_blinds_basic_properties() {
    // Test The Plant
    let plant = ThePlant;
    assert_eq!(plant.name(), "The Plant");
    assert_eq!(plant.min_ante(), 4);
    assert!(!plant.get_effects().is_empty());

    // Test The Serpent
    let serpent = TheSerpent;
    assert_eq!(serpent.name(), "The Serpent");
    assert_eq!(serpent.min_ante(), 5);
    assert!(!serpent.get_effects().is_empty());

    // Test The Needle
    let needle = TheNeedle;
    assert_eq!(needle.name(), "The Needle");
    assert_eq!(needle.min_ante(), 2);
    assert!(!needle.get_effects().is_empty());

    // Test The Water
    let water = TheWater;
    assert_eq!(water.name(), "The Water");
    assert_eq!(water.min_ante(), 2);
    assert!(!water.get_effects().is_empty());

    // Test The Flint
    let flint = TheFlint;
    assert_eq!(flint.name(), "The Flint");
    assert_eq!(flint.min_ante(), 2);
    assert!(!flint.get_effects().is_empty());

    // Test The Mark
    let mark = TheMark;
    assert_eq!(mark.name(), "The Mark");
    assert_eq!(mark.min_ante(), 2);
    assert!(!mark.get_effects().is_empty());

    // Test The Wheel
    let wheel = TheWheel;
    assert_eq!(wheel.name(), "The Wheel");
    assert_eq!(wheel.min_ante(), 2);
    assert!(!wheel.get_effects().is_empty());
}

#[test]
fn test_boss_blind_id_enum_completeness() {
    let all_ids = BossBlindId::all();

    // Should have all 7 boss blinds
    assert_eq!(all_ids.len(), 7);

    // Check that all expected boss blinds are present
    assert!(all_ids.contains(&BossBlindId::ThePlant));
    assert!(all_ids.contains(&BossBlindId::TheSerpent));
    assert!(all_ids.contains(&BossBlindId::TheNeedle));
    assert!(all_ids.contains(&BossBlindId::TheWater));
    assert!(all_ids.contains(&BossBlindId::TheFlint));
    assert!(all_ids.contains(&BossBlindId::TheMark));
    assert!(all_ids.contains(&BossBlindId::TheWheel));
}

#[test]
fn test_boss_blind_id_display_names() {
    assert_eq!(format!("{}", BossBlindId::ThePlant), "The Plant");
    assert_eq!(format!("{}", BossBlindId::TheSerpent), "The Serpent");
    assert_eq!(format!("{}", BossBlindId::TheNeedle), "The Needle");
    assert_eq!(format!("{}", BossBlindId::TheWater), "The Water");
    assert_eq!(format!("{}", BossBlindId::TheFlint), "The Flint");
    assert_eq!(format!("{}", BossBlindId::TheMark), "The Mark");
    assert_eq!(format!("{}", BossBlindId::TheWheel), "The Wheel");
}

#[test]
fn test_boss_blind_score_requirements() {
    // The Needle should have 1x score (300), others should have 2x score (600)
    assert_eq!(BossBlindId::ThePlant.base_score_requirement(), 600);
    assert_eq!(BossBlindId::TheSerpent.base_score_requirement(), 600);
    assert_eq!(BossBlindId::TheNeedle.base_score_requirement(), 300);
    assert_eq!(BossBlindId::TheWater.base_score_requirement(), 600);
    assert_eq!(BossBlindId::TheFlint.base_score_requirement(), 600);
    assert_eq!(BossBlindId::TheMark.base_score_requirement(), 600);
    assert_eq!(BossBlindId::TheWheel.base_score_requirement(), 600);
}

#[test]
fn test_boss_blind_reward_multipliers() {
    // The Needle should have lower reward (1.5x), others should have 2.0x
    assert_eq!(BossBlindId::ThePlant.reward_multiplier(), 2.0);
    assert_eq!(BossBlindId::TheSerpent.reward_multiplier(), 2.0);
    assert_eq!(BossBlindId::TheNeedle.reward_multiplier(), 1.5);
    assert_eq!(BossBlindId::TheWater.reward_multiplier(), 2.0);
    assert_eq!(BossBlindId::TheFlint.reward_multiplier(), 2.0);
    assert_eq!(BossBlindId::TheMark.reward_multiplier(), 2.0);
    assert_eq!(BossBlindId::TheWheel.reward_multiplier(), 2.0);
}

#[test]
fn test_the_plant_integration() {
    let plant = ThePlant;
    let mut game = Game::new(Config::new());

    // Test face card detection
    let jack = create_test_card(Value::Jack);
    let ace = create_test_card(Value::Ace);

    assert!(ThePlant::is_face_card_debuffed(&jack));
    assert!(!ThePlant::is_face_card_debuffed(&ace));

    // Test activation
    assert!(!ThePlant::is_active_and_debuffing(&game));

    game.boss_blind_state.activate(BossBlindId::ThePlant);
    plant.apply_effects(&mut game);

    assert!(ThePlant::is_active_and_debuffing(&game));
    assert!(ThePlant::should_debuff_card_scoring(&game, &jack));
    assert!(!ThePlant::should_debuff_card_scoring(&game, &ace));
}

#[test]
fn test_the_serpent_integration() {
    let serpent = TheSerpent;
    let mut game = Game::new(Config::new());

    // Test activation
    assert!(!TheSerpent::is_active(&game));
    assert_eq!(TheSerpent::get_cards_to_draw(&game), 0);

    game.boss_blind_state.activate(BossBlindId::TheSerpent);
    serpent.apply_effects(&mut game);

    assert!(TheSerpent::is_active(&game));
    assert_eq!(TheSerpent::get_cards_to_draw(&game), 3);
    assert!(TheSerpent::should_trigger_forced_draw(&game));
}

#[test]
fn test_the_needle_integration() {
    let needle = TheNeedle;
    let mut game = Game::new(Config::new());

    let initial_plays = game.plays;
    assert!(initial_plays > 1.0);

    // Test activation
    assert!(!TheNeedle::is_active(&game));
    assert!(TheNeedle::can_play_hand(&game));

    game.boss_blind_state.activate(BossBlindId::TheNeedle);
    needle.apply_effects(&mut game);

    assert!(TheNeedle::is_active(&game));
    assert_eq!(game.plays, 1.0);
    assert_eq!(TheNeedle::get_max_plays(&game), 1);
    assert!(TheNeedle::can_play_hand(&game));

    // Simulate playing the hand
    game.plays = 0.0;
    assert!(!TheNeedle::can_play_hand(&game));
    assert!(TheNeedle::play_limit_reached(&game));
}

#[test]
fn test_the_water_integration() {
    let water = TheWater;
    let mut game = Game::new(Config::new());

    // Set some initial discards
    game.discards = 3.0;

    // Test activation
    assert!(!TheWater::is_active(&game));
    assert!(TheWater::can_discard(&game));

    game.boss_blind_state.activate(BossBlindId::TheWater);
    water.apply_effects(&mut game);

    assert!(TheWater::is_active(&game));
    assert_eq!(game.discards, 0.0);
    assert!(TheWater::are_discards_blocked(&game));
    assert!(!TheWater::can_discard(&game));
    assert_eq!(TheWater::get_available_discards(&game), 0.0);
}

#[test]
fn test_the_flint_integration() {
    let flint = TheFlint;
    let mut game = Game::new(Config::new());

    // Test activation
    assert!(!TheFlint::is_active(&game));
    assert_eq!(TheFlint::get_chips_multiplier(&game), 1.0);
    assert_eq!(TheFlint::get_mult_multiplier(&game), 1.0);

    game.boss_blind_state.activate(BossBlindId::TheFlint);
    flint.apply_effects(&mut game);

    assert!(TheFlint::is_active(&game));
    assert_eq!(TheFlint::get_chips_multiplier(&game), 0.5);
    assert_eq!(TheFlint::get_mult_multiplier(&game), 0.5);
    assert!(TheFlint::should_halve_scoring(&game));

    // Test scoring modifications
    assert_eq!(TheFlint::apply_chips_halving(&game, 100.0), 50.0);
    assert_eq!(TheFlint::apply_mult_halving(&game, 4.0), 2.0);
}

#[test]
fn test_the_mark_integration() {
    let mark = TheMark;
    let mut game = Game::new(Config::new());

    let cards = create_test_cards(&[
        Value::Jack,
        Value::Queen,
        Value::King,
        Value::Ace,
        Value::Ten,
    ]);

    // Test activation
    assert!(!TheMark::is_active(&game));
    assert!(!TheMark::are_face_cards_hidden(&game));

    let visible = TheMark::get_visible_cards(&game, &cards);
    let hidden = TheMark::get_hidden_cards(&game, &cards);
    assert_eq!(visible.len(), 5);
    assert_eq!(hidden.len(), 0);

    game.boss_blind_state.activate(BossBlindId::TheMark);
    mark.apply_effects(&mut game);

    assert!(TheMark::is_active(&game));
    assert!(TheMark::are_face_cards_hidden(&game));

    let visible = TheMark::get_visible_cards(&game, &cards);
    let hidden = TheMark::get_hidden_cards(&game, &cards);
    assert_eq!(visible.len(), 2); // Ace and Ten
    assert_eq!(hidden.len(), 3); // Jack, Queen, King

    // Test specific card hiding
    assert!(TheMark::is_card_hidden(&game, &cards[0])); // Jack
    assert!(TheMark::is_card_hidden(&game, &cards[1])); // Queen
    assert!(TheMark::is_card_hidden(&game, &cards[2])); // King
    assert!(!TheMark::is_card_hidden(&game, &cards[3])); // Ace
    assert!(!TheMark::is_card_hidden(&game, &cards[4])); // Ten
}

#[test]
fn test_the_wheel_integration() {
    let wheel = TheWheel;
    let mut game = Game::new(Config::new());

    // Test activation
    assert!(!TheWheel::is_active(&game));
    assert!(!TheWheel::is_random_hiding_active(&game));
    assert_eq!(TheWheel::get_hide_ratio(&game), 0.0);

    game.boss_blind_state.activate(BossBlindId::TheWheel);
    wheel.apply_effects(&mut game);

    assert!(TheWheel::is_active(&game));
    assert!(TheWheel::is_random_hiding_active(&game));
    assert_eq!(TheWheel::get_hide_ratio(&game), 1.0 / 7.0);

    // Should have generated hidden indices
    let hidden_indices = TheWheel::get_hidden_card_indices(&game);
    assert!(!hidden_indices.is_empty());

    // Test card hiding with a collection
    let cards = create_test_cards(&[
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
        Value::Ace,
        Value::Two,
    ]);

    let visible = TheWheel::get_visible_cards(&game, &cards);
    let hidden = TheWheel::get_hidden_cards(&game, &cards);

    // Hidden + visible should equal total
    assert_eq!(visible.len() + hidden.len(), cards.len());
}

#[test]
fn test_boss_blind_effect_types() {
    // Test that each boss blind has the expected effect types
    let plant = ThePlant;
    let effects = plant.get_effects();
    assert_eq!(effects.len(), 1);
    assert!(matches!(effects[0], BlindEffect::DebuffCards(_)));

    let serpent = TheSerpent;
    let effects = serpent.get_effects();
    assert_eq!(effects.len(), 1);
    assert!(matches!(effects[0], BlindEffect::SpecialRule(_)));

    let needle = TheNeedle;
    let effects = needle.get_effects();
    assert_eq!(effects.len(), 1);
    assert!(matches!(effects[0], BlindEffect::RestrictActions(_)));

    let water = TheWater;
    let effects = water.get_effects();
    assert_eq!(effects.len(), 1);
    assert!(matches!(effects[0], BlindEffect::RestrictActions(_)));

    let flint = TheFlint;
    let effects = flint.get_effects();
    assert_eq!(effects.len(), 1);
    assert!(matches!(effects[0], BlindEffect::ModifyScoring(_)));

    let mark = TheMark;
    let effects = mark.get_effects();
    assert_eq!(effects.len(), 1);
    assert!(matches!(effects[0], BlindEffect::SpecialRule(_)));

    let wheel = TheWheel;
    let effects = wheel.get_effects();
    assert_eq!(effects.len(), 1);
    assert!(matches!(effects[0], BlindEffect::SpecialRule(_)));
}

#[test]
fn test_boss_blind_counter_tracking() {
    let game = Game::new(Config::new());

    // Test that each boss blind tracks appropriate counters
    let plant = ThePlant;
    let counters = plant.check_counters(&game);
    assert!(counters.contains(&CounterType::CardsScored));

    let serpent = TheSerpent;
    let counters = serpent.check_counters(&game);
    assert!(counters.contains(&CounterType::HandsPlayed));
    assert!(counters.contains(&CounterType::CardsDiscarded));

    let needle = TheNeedle;
    let counters = needle.check_counters(&game);
    assert!(counters.contains(&CounterType::HandsPlayed));

    let water = TheWater;
    let counters = water.check_counters(&game);
    assert!(counters.contains(&CounterType::CardsDiscarded));

    let flint = TheFlint;
    let counters = flint.check_counters(&game);
    assert!(counters.contains(&CounterType::HandsPlayed));
    assert!(counters.contains(&CounterType::CardsScored));

    let mark = TheMark;
    let counters = mark.check_counters(&game);
    assert!(counters.contains(&CounterType::HandsPlayed));
    assert!(counters.contains(&CounterType::CardsScored));

    let wheel = TheWheel;
    let counters = wheel.check_counters(&game);
    assert!(counters.contains(&CounterType::HandsPlayed));
    assert!(counters.contains(&CounterType::CardsScored));
}

#[test]
fn test_boss_blind_deactivation() {
    let mut game = Game::new(Config::new());

    // Test that all boss blinds can be properly deactivated
    let boss_blinds: Vec<(BossBlindId, Box<dyn BossBlind>)> = vec![
        (BossBlindId::ThePlant, Box::new(ThePlant)),
        (BossBlindId::TheSerpent, Box::new(TheSerpent)),
        (BossBlindId::TheNeedle, Box::new(TheNeedle)),
        (BossBlindId::TheWater, Box::new(TheWater)),
        (BossBlindId::TheFlint, Box::new(TheFlint)),
        (BossBlindId::TheMark, Box::new(TheMark)),
        (BossBlindId::TheWheel, Box::new(TheWheel)),
    ];

    for (id, boss_blind) in boss_blinds {
        // Activate the boss blind
        game.boss_blind_state.activate(id);
        boss_blind.apply_effects(&mut game);

        // Verify it's active
        assert!(game.boss_blind_state.is_active());
        assert_eq!(game.boss_blind_state.active_boss(), Some(id));

        // Deactivate
        game.boss_blind_state.deactivate();

        // Verify it's no longer active
        assert!(!game.boss_blind_state.is_active());
        assert_eq!(game.boss_blind_state.active_boss(), None);
    }
}

#[test]
fn test_boss_blind_trait_object_usage() {
    // Test that all boss blinds can be used as trait objects
    let boss_blinds: Vec<Box<dyn BossBlind>> = vec![
        Box::new(ThePlant),
        Box::new(TheSerpent),
        Box::new(TheNeedle),
        Box::new(TheWater),
        Box::new(TheFlint),
        Box::new(TheMark),
        Box::new(TheWheel),
    ];

    let mut game = Game::new(Config::new());

    for boss_blind in boss_blinds {
        // Test that all methods work through trait objects
        assert!(!boss_blind.name().is_empty());
        assert!(boss_blind.min_ante() > 0);
        assert!(!boss_blind.get_effects().is_empty());

        // Apply effects should not panic
        boss_blind.apply_effects(&mut game);

        // Check counters should return something
        let counters = boss_blind.check_counters(&game);
        assert!(!counters.is_empty());
    }
}

#[test]
fn test_boss_blind_serialization_compatibility() {
    // Test that all BossBlindId variants can be serialized and deserialized
    let all_ids = BossBlindId::all();

    for id in all_ids {
        let serialized = serde_json::to_string(&id).expect("Should serialize");
        assert!(!serialized.is_empty());

        let deserialized: BossBlindId =
            serde_json::from_str(&serialized).expect("Should deserialize");
        assert_eq!(id, deserialized);
    }
}

#[test]
fn test_multiple_boss_blinds_not_active_simultaneously() {
    let mut game = Game::new(Config::new());

    // Activate The Plant
    game.boss_blind_state.activate(BossBlindId::ThePlant);
    ThePlant.apply_effects(&mut game);
    assert!(ThePlant::is_active_and_debuffing(&game));

    // Activate The Serpent (should replace The Plant)
    game.boss_blind_state.activate(BossBlindId::TheSerpent);
    TheSerpent.apply_effects(&mut game);

    // Only The Serpent should be active now
    assert!(!ThePlant::is_active_and_debuffing(&game));
    assert!(TheSerpent::is_active(&game));
}
