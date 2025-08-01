//! Integration tests for pack opening with consumable slot assignment
//!
//! Tests the functionality implemented in issue #408: Consumable Purchase - Pack Opening Integration

use balatro_rs::config::Config;
use balatro_rs::consumables::ConsumableId;
use balatro_rs::game::Game;
use balatro_rs::shop::{ConsumableType as ShopConsumableType, ShopItem};

#[test]
fn test_pack_opening_consumable_integration_basic() {
    // Create a game with default settings
    let mut game = Game::new(Config::default());

    // Verify initial state - consumable slots should be empty
    assert_eq!(game.consumable_slots.len(), 0);
    assert_eq!(game.consumable_slots.capacity(), 2); // Default capacity

    // Test processing a Tarot consumable from a pack
    let tarot_item = ShopItem::Consumable(ShopConsumableType::Tarot);
    let result = game.process_pack_item(tarot_item);

    // Should succeed in adding the consumable
    assert!(
        result.is_ok(),
        "Failed to process tarot consumable from pack"
    );

    // Verify the consumable was added to slots
    assert_eq!(game.consumable_slots.len(), 1);
    assert!(game.consumable_slots.available_slots() == 1);
}

#[test]
fn test_pack_opening_specific_consumable_integration() {
    let mut game = Game::new(Config::default());

    // Test processing a specific consumable from a pack
    let specific_consumable = ShopItem::SpecificConsumable(ConsumableId::TheFool);
    let result = game.process_pack_item(specific_consumable);

    // Should succeed in adding the specific consumable
    assert!(
        result.is_ok(),
        "Failed to process specific consumable from pack"
    );

    // Verify the consumable was added to slots
    assert_eq!(game.consumable_slots.len(), 1);
}

#[test]
fn test_pack_opening_spectral_consumable_integration() {
    let mut game = Game::new(Config::default());

    // Test processing a Spectral consumable from a pack
    let spectral_item = ShopItem::Consumable(ShopConsumableType::Spectral);
    let result = game.process_pack_item(spectral_item);

    // Should succeed in adding the spectral consumable
    assert!(
        result.is_ok(),
        "Failed to process spectral consumable from pack"
    );

    // Verify the consumable was added to slots
    assert_eq!(game.consumable_slots.len(), 1);
}

#[test]
fn test_pack_opening_overflow_scenario() {
    let mut game = Game::new(Config::default());

    // Fill up consumable slots to capacity (default is 2)
    let tarot1 = ShopItem::SpecificConsumable(ConsumableId::TheFool);
    let tarot2 = ShopItem::SpecificConsumable(ConsumableId::TheMagician);

    // Add first two consumables - should succeed
    assert!(game.process_pack_item(tarot1).is_ok());
    assert!(game.process_pack_item(tarot2).is_ok());
    assert_eq!(game.consumable_slots.len(), 2);
    assert!(game.consumable_slots.is_full());

    // Try to add a third consumable - should fail due to overflow
    let tarot3 = ShopItem::SpecificConsumable(ConsumableId::TheEmpress);
    let result = game.process_pack_item(tarot3);

    // Should fail because slots are full
    assert!(
        result.is_err(),
        "Expected overflow error when slots are full"
    );

    // Slots should still be full with original consumables
    assert_eq!(game.consumable_slots.len(), 2);
    assert!(game.consumable_slots.is_full());
}

#[test]
fn test_pack_opening_mixed_items() {
    let mut game = Game::new(Config::default());

    // Test processing non-consumable items from packs still works
    use balatro_rs::card::{Card, Suit, Value};
    use balatro_rs::joker::JokerId;

    // Process a playing card
    let card_item = ShopItem::PlayingCard(Card::new(Value::Ace, Suit::Heart));
    assert!(game.process_pack_item(card_item).is_ok());

    // Process a joker
    let joker_item = ShopItem::Joker(JokerId::Joker);
    assert!(game.process_pack_item(joker_item).is_ok());

    // Consumable slots should still be empty
    assert_eq!(game.consumable_slots.len(), 0);

    // Now add a consumable
    let consumable_item = ShopItem::SpecificConsumable(ConsumableId::TheFool);
    assert!(game.process_pack_item(consumable_item).is_ok());

    // Now consumable slots should have one item
    assert_eq!(game.consumable_slots.len(), 1);
}
