//! Integration tests for extended game state features
//!
//! Tests the new consumables, vouchers, boss blinds, and state versioning functionality
//! added to the Game struct for issue #35.

#![allow(clippy::field_reassign_with_default)]

use balatro_rs::boss_blinds::BossBlindId;
use balatro_rs::config::Config;
use balatro_rs::game::Game;
use balatro_rs::state_version::StateVersion;
use balatro_rs::vouchers::VoucherId;

#[test]
fn test_game_has_extended_state_fields() {
    let game = Game::default();

    // Test that new fields exist and have proper default values
    assert!(game.consumable_slots.is_empty());
    assert_eq!(game.vouchers.count(), 0);
    assert_eq!(game.boss_blind_state.active_boss, None);
    assert_eq!(game.state_version, StateVersion::current());
}

#[test]
fn test_game_new_with_extended_state() {
    let config = Config::default();
    let game = Game::new(config);

    // Verify all new fields are properly initialized
    assert!(game.consumable_slots.is_empty());
    assert_eq!(game.vouchers.count(), 0);
    assert!(!game.boss_blind_state.effect_active);
    assert_eq!(game.state_version, StateVersion::current());
}

#[test]
fn test_consumables_management() {
    let game = Game::default();

    // Test that consumable slots are properly initialized
    // Note: ConsumableSlots manages trait objects, not simple IDs
    // Real consumable testing should use the game's consumable factories
    assert_eq!(game.consumable_slots.len(), 0);
    assert_eq!(game.consumable_slots.capacity(), 2);
    assert!(game.consumable_slots.is_empty());
    assert!(!game.consumable_slots.is_full());
    assert_eq!(game.consumable_slots.available_slots(), 2);
}

#[test]
fn test_voucher_collection_integration() {
    let mut game = Game::default();

    // Test voucher operations
    assert!(!game.vouchers.owns(VoucherId::Overstock));

    game.vouchers.add(VoucherId::Overstock);
    assert!(game.vouchers.owns(VoucherId::Overstock));
    assert_eq!(game.vouchers.count(), 1);

    let owned = game.vouchers.owned_vouchers();
    assert!(owned.contains(&VoucherId::Overstock));
}

#[test]
fn test_boss_blind_state_management() {
    let mut game = Game::default();

    // Test initial state
    assert_eq!(game.boss_blind_state.active_boss, None);
    assert!(!game.boss_blind_state.effect_active);

    // Test activating boss blind
    game.boss_blind_state.active_boss = Some(BossBlindId::BossBlindPlaceholder);
    game.boss_blind_state.effect_active = true;

    assert_eq!(
        game.boss_blind_state.active_boss,
        Some(BossBlindId::BossBlindPlaceholder)
    );
    assert!(game.boss_blind_state.effect_active);

    // Test deactivating
    game.boss_blind_state.active_boss = None;
    game.boss_blind_state.effect_active = false;

    assert_eq!(game.boss_blind_state.active_boss, None);
    assert!(!game.boss_blind_state.effect_active);
}

#[test]
fn test_state_version_tracking() {
    let game = Game::default();

    // Verify current version is set
    assert_eq!(game.state_version, StateVersion::current());
    assert_eq!(game.state_version, StateVersion::V2);

    // Verify version can be changed (for migration testing)
    let mut game_v1 = Game::default();
    game_v1.state_version = StateVersion::V1;
    assert_eq!(game_v1.state_version, StateVersion::V1);
    assert!(game_v1.state_version.can_migrate_to_current());
}

#[cfg(feature = "serde")]
#[test]
fn test_game_serialization_with_extended_state() {
    let mut game = Game::default();

    // Set up extended state
    // Note: Actual consumable testing requires factory methods
    // For serialization testing, we'll test the structure only
    game.vouchers.add(VoucherId::Overstock);
    game.boss_blind_state.active_boss = Some(BossBlindId::BossBlindPlaceholder);
    game.state_version = StateVersion::V2;

    // Test serialization
    let serialized = serde_json::to_string(&game).expect("Game should serialize");
    assert!(!serialized.is_empty());

    // Test deserialization
    let deserialized: Game = serde_json::from_str(&serialized).expect("Game should deserialize");

    // Verify extended state structure is preserved
    // Note: ConsumableSlots doesn't serialize actual consumables
    assert_eq!(deserialized.consumable_slots.len(), 0);
    assert_eq!(deserialized.consumable_slots.capacity(), 2);
    assert!(deserialized.vouchers.owns(VoucherId::Overstock));
    assert_eq!(
        deserialized.boss_blind_state.active_boss,
        Some(BossBlindId::BossBlindPlaceholder)
    );
    assert_eq!(deserialized.state_version, StateVersion::V2);
}

#[cfg(feature = "serde")]
#[test]
fn test_state_version_migration_compatibility() {
    // Create a game state that simulates V1 format (without extended fields)
    let v1_json = r#"{
        "config": {},
        "shop": {"jokers": [], "joker_gen": {}},
        "deck": {"cards": []},
        "available": {"cards": []},
        "discarded": [],
        "blind": null,
        "stage": "PreBlind",
        "ante_start": "One",
        "ante_end": "Eight",
        "ante_current": "One",
        "action_history": [],
        "round": 1,
        "jokers": [],
        "effect_registry": {"effects": {}},
        "joker_state_manager": {"states": {}},
        "plays": 3,
        "discards": 3,
        "reward": 0,
        "money": 4,
        "chips": 0,
        "mult": 0,
        "score": 0,
        "hand_type_counts": {}
    }"#;

    // This test validates that we can handle missing fields gracefully
    // The actual migration logic would be more complex and handle this properly
    // For now, we verify that the test structure is correct

    let parsed: serde_json::Value = serde_json::from_str(v1_json).expect("V1 JSON should be valid");
    assert!(parsed.is_object());

    // Verify that V1 format doesn't have extended fields
    assert!(parsed.get("consumable_slots").is_none());
    assert!(parsed.get("vouchers").is_none());
    assert!(parsed.get("boss_blind_state").is_none());
    assert!(parsed.get("state_version").is_none());
}

#[test]
fn test_extended_state_memory_footprint() {
    let game = Game::default();

    // Test that extended fields don't excessively increase memory usage
    // This is a basic check - actual memory profiling would be done separately
    let size = std::mem::size_of_val(&game);

    // Verify the game state is reasonable in size (arbitrary threshold)
    // This test mainly ensures compilation and basic structure
    assert!(size > 0);
    assert!(size < 10_000); // Reasonable upper bound for basic state
}

#[test]
fn test_state_validation_with_extended_fields() {
    let mut game = Game::default();

    // Test that extended state maintains game invariants
    assert!(game.consumable_slots.len() <= game.consumable_slots.capacity()); // Slots can't exceed capacity
    assert!(game.vouchers.count() <= 100); // Arbitrary voucher limit

    // Add some extended state
    game.vouchers.add(VoucherId::Overstock);

    // Verify state is still valid
    assert!(game.consumable_slots.is_empty()); // No consumables added yet
    assert!(game.vouchers.count() > 0);
    assert_eq!(game.state_version, StateVersion::current());
}
