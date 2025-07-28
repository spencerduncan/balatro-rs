// Tests for consumable trait functionality
// Fixed: ConsumableSlots now implements Clone, Serialize, and Deserialize
// Fixed: Added get_mock_id() and get_real_id() methods to Consumable trait

use balatro_rs::consumables::{
    Consumable, ConsumableEffect, ConsumableError, ConsumableId, ConsumableSlots, ConsumableType,
    Target, TargetType,
};
use balatro_rs::game::Game;

#[test]
fn test_consumable_trait_object_compatibility() {
    #[derive(Debug)]
    struct MockConsumable;

    impl Consumable for MockConsumable {
        fn consumable_type(&self) -> ConsumableType {
            ConsumableType::Tarot
        }

        fn can_use(&self, _game_state: &Game, _target: &Target) -> bool {
            true
        }

        fn use_effect(
            &self,
            _game_state: &mut Game,
            _target: Target,
        ) -> Result<(), ConsumableError> {
            Ok(())
        }

        fn get_description(&self) -> String {
            "Mock consumable for testing".to_string()
        }

        fn get_target_type(&self) -> TargetType {
            TargetType::None
        }

        fn get_effect_category(&self) -> ConsumableEffect {
            ConsumableEffect::Enhancement
        }

        fn get_mock_id(&self) -> u32 {
            42 // Fixed mock ID for basic testing
        }
    }

    // Test trait object compatibility with Send + Sync + Debug bounds
    let consumable: Box<dyn Consumable + Send + Sync> = Box::new(MockConsumable);

    // Test basic trait methods work through trait object
    assert_eq!(consumable.consumable_type(), ConsumableType::Tarot);
    assert_eq!(consumable.get_target_type(), TargetType::None);
    assert_eq!(
        consumable.get_effect_category(),
        ConsumableEffect::Enhancement
    );
    assert_eq!(consumable.get_description(), "Mock consumable for testing");
}

#[test]
fn test_consumable_effect_enum_categories() {
    // Test all effect categories are properly defined
    let enhancement = ConsumableEffect::Enhancement;
    let destruction = ConsumableEffect::Destruction;
    let generation = ConsumableEffect::Generation;
    let modification = ConsumableEffect::Modification;
    let utility = ConsumableEffect::Utility;

    // Test Debug implementation
    assert!(format!("{:?}", enhancement).contains("Enhancement"));
    assert!(format!("{:?}", destruction).contains("Destruction"));
    assert!(format!("{:?}", generation).contains("Generation"));
    assert!(format!("{:?}", modification).contains("Modification"));
    assert!(format!("{:?}", utility).contains("Utility"));
}

#[test]
fn test_target_type_definitions() {
    // Test all target types are properly defined
    let none = TargetType::None;
    let cards = TargetType::Cards(2);
    let hand_type = TargetType::HandType;
    let joker = TargetType::Joker;
    let deck = TargetType::Deck;
    let shop = TargetType::Shop;

    // Test Debug implementation
    assert!(format!("{:?}", none).contains("None"));
    assert!(format!("{:?}", cards).contains("Cards"));
    assert!(format!("{:?}", hand_type).contains("HandType"));
    assert!(format!("{:?}", joker).contains("Joker"));
    assert!(format!("{:?}", deck).contains("Deck"));
    assert!(format!("{:?}", shop).contains("Shop"));
}

#[test]
fn test_target_validation() {
    let game = Game::default();

    // Test different target types
    let no_target = Target::None;
    let card_targets = Target::cards_in_hand(vec![0, 1]);
    let hand_target = Target::HandType(balatro_rs::rank::HandRank::OnePair);
    let joker_target = Target::Joker(0);
    let deck_target = Target::Deck;
    let shop_target = Target::Shop(2);

    assert!(matches!(no_target, Target::None));
    assert!(matches!(card_targets, Target::Cards(_)));
    assert!(matches!(hand_target, Target::HandType(_)));
    assert!(matches!(joker_target, Target::Joker(_)));
    assert!(matches!(deck_target, Target::Deck));
    assert!(matches!(shop_target, Target::Shop(_)));

    // Test validation methods
    assert!(no_target.is_valid(&game));
    // Card targets would need game state validation
    assert!(hand_target.is_valid(&game));
    assert!(deck_target.is_valid(&game));
    assert!(shop_target.is_valid(&game));
}

#[test]
fn test_consumable_error_types() {
    // Test error type definitions
    let invalid_target = ConsumableError::InvalidTarget("Mock error".to_string());
    let insufficient_resources = ConsumableError::InsufficientResources;
    let invalid_game_state = ConsumableError::InvalidGameState("Mock state error".to_string());
    let effect_failed = ConsumableError::EffectFailed("Mock effect error".to_string());

    // Test error display
    assert!(invalid_target.to_string().contains("Invalid target"));
    assert!(insufficient_resources
        .to_string()
        .contains("Insufficient resources"));
    assert!(invalid_game_state
        .to_string()
        .contains("Invalid game state"));
    assert!(effect_failed.to_string().contains("Effect failed"));
}

#[test]
fn test_consumable_type_existing_variants() {
    // Test existing ConsumableType variants still work
    let tarot = ConsumableType::Tarot;
    let planet = ConsumableType::Planet;
    let spectral = ConsumableType::Spectral;

    assert_eq!(tarot.to_string(), "Tarot");
    assert_eq!(planet.to_string(), "Planet");
    assert_eq!(spectral.to_string(), "Spectral");
}

// TODO: Add async test when tokio dependency is available
// #[tokio::test]
// async fn test_async_effect_application() {
//     // Async support will be added in future iterations
// }

#[test]
fn test_enhanced_consumable_trait_methods() {
    #[derive(Debug)]
    struct EnhancedMockConsumable;

    impl Consumable for EnhancedMockConsumable {
        fn consumable_type(&self) -> ConsumableType {
            ConsumableType::Planet
        }

        fn can_use(&self, game_state: &Game, target: &Target) -> bool {
            // Mock validation logic
            match target {
                Target::None => true,
                Target::Cards(cards) => {
                    !cards.indices.is_empty()
                        && cards.indices.len() <= game_state.available.cards().len()
                }
                Target::HandType(_) => true,
                Target::Joker(_) => game_state.jokers.len() > 0,
                Target::Deck => true,
                Target::Shop(_) => true,
            }
        }

        fn use_effect(
            &self,
            _game_state: &mut Game,
            _target: Target,
        ) -> Result<(), ConsumableError> {
            Ok(())
        }

        fn get_description(&self) -> String {
            "Enhanced mock consumable with validation".to_string()
        }

        fn get_target_type(&self) -> TargetType {
            TargetType::HandType
        }

        fn get_effect_category(&self) -> ConsumableEffect {
            ConsumableEffect::Modification
        }

        fn get_mock_id(&self) -> u32 {
            42 // Fixed mock ID for enhanced testing
        }
    }

    let consumable = EnhancedMockConsumable;
    let game = Game::default();

    // Test validation methods
    assert!(consumable.can_use(&game, &Target::None));
    assert!(consumable.can_use(
        &game,
        &Target::HandType(balatro_rs::rank::HandRank::OnePair)
    ));

    // Test metadata methods
    assert_eq!(consumable.get_target_type(), TargetType::HandType);
    assert_eq!(
        consumable.get_effect_category(),
        ConsumableEffect::Modification
    );
    assert!(consumable.get_description().contains("Enhanced mock"));
}

#[test]
fn test_target_type_method() {
    // Test target_type() method for all variants
    assert_eq!(Target::None.target_type(), TargetType::None);
    assert_eq!(
        Target::cards_in_hand(vec![0, 1, 2]).target_type(),
        TargetType::Cards(3)
    );
    assert_eq!(
        Target::cards_in_hand(vec![]).target_type(),
        TargetType::Cards(0)
    );
    assert_eq!(
        Target::HandType(balatro_rs::rank::HandRank::OnePair).target_type(),
        TargetType::HandType
    );
    assert_eq!(Target::Joker(5).target_type(), TargetType::Joker);
    assert_eq!(Target::Deck.target_type(), TargetType::Deck);
    assert_eq!(Target::Shop(3).target_type(), TargetType::Shop);
}

#[test]
fn test_target_is_valid_type_method() {
    // Test is_valid_type() method for matching types
    assert!(Target::None.is_valid_type(TargetType::None));
    assert!(Target::cards_in_hand(vec![0, 1]).is_valid_type(TargetType::Cards(2)));
    assert!(Target::cards_in_hand(vec![]).is_valid_type(TargetType::Cards(0)));
    assert!(
        Target::HandType(balatro_rs::rank::HandRank::FullHouse).is_valid_type(TargetType::HandType)
    );
    assert!(Target::Joker(0).is_valid_type(TargetType::Joker));
    assert!(Target::Deck.is_valid_type(TargetType::Deck));
    assert!(Target::Shop(2).is_valid_type(TargetType::Shop));

    // Test is_valid_type() method for mismatched types
    assert!(!Target::None.is_valid_type(TargetType::Cards(1)));
    assert!(!Target::cards_in_hand(vec![0]).is_valid_type(TargetType::Cards(2))); // Wrong count
    assert!(!Target::cards_in_hand(vec![0, 1]).is_valid_type(TargetType::HandType));
    assert!(!Target::HandType(balatro_rs::rank::HandRank::OnePair).is_valid_type(TargetType::Joker));
    assert!(!Target::Joker(0).is_valid_type(TargetType::Deck));
    assert!(!Target::Deck.is_valid_type(TargetType::Shop));
    assert!(!Target::Shop(1).is_valid_type(TargetType::None));
}

#[test]
fn test_target_card_count_method() {
    // Test card_count() method for all variants
    assert_eq!(Target::None.card_count(), 0);
    assert_eq!(Target::cards_in_hand(vec![]).card_count(), 0);
    assert_eq!(Target::cards_in_hand(vec![0]).card_count(), 1);
    assert_eq!(Target::cards_in_hand(vec![0, 1, 2]).card_count(), 3);
    assert_eq!(
        Target::HandType(balatro_rs::rank::HandRank::Straight).card_count(),
        0
    );
    assert_eq!(Target::Joker(10).card_count(), 0);
    assert_eq!(Target::Deck.card_count(), 0);
    assert_eq!(Target::Shop(5).card_count(), 0);
}

#[test]
fn test_target_all_variants_comprehensive() {
    use balatro_rs::rank::HandRank;

    // Test all Target variants can be created and used
    let targets = vec![
        Target::None,
        Target::cards_in_hand(vec![]),
        Target::cards_in_hand(vec![0, 1, 2]),
        Target::HandType(HandRank::HighCard),
        Target::HandType(HandRank::OnePair),
        Target::HandType(HandRank::TwoPair),
        Target::HandType(HandRank::ThreeOfAKind),
        Target::HandType(HandRank::Straight),
        Target::HandType(HandRank::Flush),
        Target::HandType(HandRank::FullHouse),
        Target::HandType(HandRank::FourOfAKind),
        Target::HandType(HandRank::StraightFlush),
        Target::HandType(HandRank::RoyalFlush),
        Target::HandType(HandRank::FiveOfAKind),
        Target::HandType(HandRank::FlushHouse),
        Target::HandType(HandRank::FlushFive),
        Target::Joker(0),
        Target::Joker(5),
        Target::Deck,
        Target::Shop(0),
        Target::Shop(1),
        Target::Shop(5),
    ];

    // Test all variants can be debugged
    for target in &targets {
        let debug_string = format!("{:?}", target);
        assert!(!debug_string.is_empty());
    }

    // Test all variants have consistent target_type() -> is_valid_type() behavior
    for target in &targets {
        let target_type = target.target_type();
        assert!(target.is_valid_type(target_type));
    }
}

#[test]
fn test_target_type_all_variants_comprehensive() {
    // Test all TargetType variants
    let target_types = vec![
        TargetType::None,
        TargetType::Cards(0),
        TargetType::Cards(1),
        TargetType::Cards(5),
        TargetType::HandType,
        TargetType::Joker,
        TargetType::Deck,
        TargetType::Shop,
    ];

    // Test all variants can be debugged
    for target_type in &target_types {
        let debug_string = format!("{:?}", target_type);
        assert!(!debug_string.is_empty());
    }

    // Test Hash and Eq traits work properly
    use std::collections::HashSet;
    let set: HashSet<TargetType> = target_types.into_iter().collect();
    assert!(set.len() >= 7); // At least 7 unique variants
}

#[test]
fn test_shop_target_specific_functionality() {
    // Test Shop-specific functionality
    let shop_targets = vec![
        Target::Shop(0),
        Target::Shop(1),
        Target::Shop(2),
        Target::Shop(10),
    ];

    for target in &shop_targets {
        assert_eq!(target.target_type(), TargetType::Shop);
        assert!(target.is_valid_type(TargetType::Shop));
        assert_eq!(target.card_count(), 0);
    }

    let game = Game::default();
    for target in &shop_targets {
        assert!(target.is_valid(&game));
    }
}

// Tests for ConsumableSlots basic functionality
#[test]
fn test_consumable_slots_basic_functionality() {
    // Test default creation
    let slots = ConsumableSlots::new();
    assert_eq!(slots.capacity(), 2);
    assert_eq!(slots.len(), 0);
    assert!(slots.is_empty());
    assert!(!slots.is_full());
    assert_eq!(slots.available_slots(), 2);

    // Test custom capacity creation
    let large_slots = ConsumableSlots::with_capacity(5);
    assert_eq!(large_slots.capacity(), 5);
    assert_eq!(large_slots.len(), 0);
    assert!(large_slots.is_empty());
    assert!(!large_slots.is_full());
    assert_eq!(large_slots.available_slots(), 5);
}

#[test]
fn test_consumable_slots_default_implementation() {
    let default_slots = ConsumableSlots::default();
    let new_slots = ConsumableSlots::new();

    // Both should have same capacity
    assert_eq!(default_slots.capacity(), new_slots.capacity());
    assert_eq!(default_slots.len(), new_slots.len());
    assert_eq!(default_slots.is_empty(), new_slots.is_empty());
    assert_eq!(default_slots.available_slots(), new_slots.available_slots());
}

#[test]
fn test_consumable_slots_edge_cases() {
    // Test zero capacity (edge case)
    let zero_slots = ConsumableSlots::with_capacity(0);
    assert_eq!(zero_slots.capacity(), 0);
    assert_eq!(zero_slots.len(), 0);
    assert!(zero_slots.is_empty());
    assert!(zero_slots.is_full()); // Zero capacity means full by definition
    assert_eq!(zero_slots.available_slots(), 0);

    // Test single capacity
    let single_slot = ConsumableSlots::with_capacity(1);
    assert_eq!(single_slot.capacity(), 1);
    assert_eq!(single_slot.len(), 0);
    assert!(single_slot.is_empty());
    assert!(!single_slot.is_full());
    assert_eq!(single_slot.available_slots(), 1);

    // Test large capacity
    let large_slots = ConsumableSlots::with_capacity(100);
    assert_eq!(large_slots.capacity(), 100);
    assert_eq!(large_slots.len(), 0);
    assert!(large_slots.is_empty());
    assert!(!large_slots.is_full());
    assert_eq!(large_slots.available_slots(), 100);
}

#[test]
fn test_consumable_slots_capacity_calculations() {
    let slots = ConsumableSlots::with_capacity(10);

    // Verify capacity calculations are consistent
    assert_eq!(slots.available_slots(), slots.capacity() - slots.len());
    assert_eq!(slots.is_empty(), slots.len() == 0);
    assert_eq!(slots.is_full(), slots.len() == slots.capacity());
}

#[test]
fn test_consumable_slots_debug_trait() {
    let slots = ConsumableSlots::new();
    let debug_output = format!("{:?}", slots);

    // Should contain the struct name and key fields
    assert!(debug_output.contains("ConsumableSlots"));
    assert!(debug_output.contains("capacity"));
    assert!(debug_output.contains("slots"));
}

#[test]
fn test_consumable_slots_clone() {
    let original = ConsumableSlots::with_capacity(3);
    let cloned = original.clone();

    // Verify clone has same properties
    assert_eq!(original.capacity(), cloned.capacity());
    assert_eq!(original.len(), cloned.len());
    assert_eq!(original.is_empty(), cloned.is_empty());
    assert_eq!(original.is_full(), cloned.is_full());
    assert_eq!(original.available_slots(), cloned.available_slots());
}

#[test]
fn test_consumable_slots_serde_compatibility() {
    use serde_json;

    let original = ConsumableSlots::with_capacity(3);

    // Test serialization
    let serialized = serde_json::to_string(&original);
    assert!(serialized.is_ok());

    // Test deserialization
    let json = serialized.unwrap();
    let deserialized: Result<ConsumableSlots, _> = serde_json::from_str(&json);
    assert!(deserialized.is_ok());

    let restored = deserialized.unwrap();
    assert_eq!(original.capacity(), restored.capacity());
    assert_eq!(original.len(), restored.len());
    assert_eq!(original.is_empty(), restored.is_empty());
    assert_eq!(original.is_full(), restored.is_full());
    assert_eq!(original.available_slots(), restored.available_slots());
}

#[test]
fn test_consumable_slots_thread_safety() {
    use std::sync::Arc;
    use std::thread;

    let slots = Arc::new(ConsumableSlots::new());
    let slots_clone = Arc::clone(&slots);

    // Test that ConsumableSlots can be safely shared between threads
    let handle = thread::spawn(move || {
        let capacity = slots_clone.capacity();
        let len = slots_clone.len();
        (capacity, len)
    });

    let (capacity, len) = handle.join().unwrap();
    assert_eq!(capacity, 2);
    assert_eq!(len, 0);
}

#[test]
fn test_consumable_slots_memory_efficiency() {
    // Test that slots are properly allocated only to capacity
    let small_slots = ConsumableSlots::with_capacity(2);
    let large_slots = ConsumableSlots::with_capacity(100);

    // Both should start empty regardless of capacity
    assert_eq!(small_slots.len(), 0);
    assert_eq!(large_slots.len(), 0);

    // Capacity should be exactly what was requested
    assert_eq!(small_slots.capacity(), 2);
    assert_eq!(large_slots.capacity(), 100);
}

// Tests for Issue #295 - Slot Addition and Removal Operations

#[test]
fn test_add_consumable_to_empty_slots() {
    let mut slots = ConsumableSlots::new();

    // Create mock consumables
    let consumable1 = Box::new(MockConsumableForSlots { id: 1 });
    let consumable2 = Box::new(MockConsumableForSlots { id: 2 });

    // Add first consumable
    let result1 = slots.add_consumable(consumable1);
    assert!(result1.is_ok());
    assert_eq!(result1.unwrap(), 0);
    assert_eq!(slots.len(), 1);
    assert!(!slots.is_empty());
    assert!(!slots.is_full());

    // Add second consumable
    let result2 = slots.add_consumable(consumable2);
    assert!(result2.is_ok());
    assert_eq!(result2.unwrap(), 1);
    assert_eq!(slots.len(), 2);
    assert!(!slots.is_empty());
    assert!(slots.is_full());
}

#[test]
fn test_add_consumable_when_full() {
    let mut slots = ConsumableSlots::new();

    // Fill all slots
    slots
        .add_consumable(Box::new(MockConsumableForSlots { id: 1 }))
        .unwrap();
    slots
        .add_consumable(Box::new(MockConsumableForSlots { id: 2 }))
        .unwrap();

    // Try to add one more
    let consumable3 = Box::new(MockConsumableForSlots { id: 3 });
    let result = slots.add_consumable(consumable3);

    assert!(result.is_err());
    match result.unwrap_err() {
        balatro_rs::consumables::SlotError::NoEmptySlots { capacity } => {
            assert_eq!(capacity, 2);
        }
        _ => panic!("Expected NoEmptySlots error"),
    }
}

#[test]
fn test_remove_consumable_valid_index() {
    let mut slots = ConsumableSlots::new();

    // Add consumables
    slots
        .add_consumable(Box::new(MockConsumableForSlots { id: 1 }))
        .unwrap();
    slots
        .add_consumable(Box::new(MockConsumableForSlots { id: 2 }))
        .unwrap();

    // Remove first consumable
    let result = slots.remove_consumable(0);
    assert!(result.is_ok());
    let removed = result.unwrap();
    assert_eq!(removed.get_mock_id(), 1);
    assert_eq!(slots.len(), 1);

    // Remove second consumable
    let result2 = slots.remove_consumable(1);
    assert!(result2.is_ok());
    let removed2 = result2.unwrap();
    assert_eq!(removed2.get_mock_id(), 2);
    assert_eq!(slots.len(), 0);
    assert!(slots.is_empty());
}

#[test]
fn test_remove_consumable_invalid_index() {
    let mut slots = ConsumableSlots::new();
    slots
        .add_consumable(Box::new(MockConsumableForSlots { id: 1 }))
        .unwrap();

    // Try to remove from out of bounds index
    let result = slots.remove_consumable(5);
    assert!(result.is_err());
    match result.unwrap_err() {
        balatro_rs::consumables::SlotError::IndexOutOfBounds { index, capacity } => {
            assert_eq!(index, 5);
            assert_eq!(capacity, 2);
        }
        _ => panic!("Expected IndexOutOfBounds error"),
    }
}

#[test]
fn test_remove_consumable_from_empty_slot() {
    let mut slots = ConsumableSlots::new();
    slots
        .add_consumable(Box::new(MockConsumableForSlots { id: 1 }))
        .unwrap();

    // Try to remove from empty slot (index 1)
    let result = slots.remove_consumable(1);
    assert!(result.is_err());
    match result.unwrap_err() {
        balatro_rs::consumables::SlotError::SlotEmpty { index } => {
            assert_eq!(index, 1);
        }
        _ => panic!("Expected SlotEmpty error"),
    }
}

#[test]
fn test_get_consumable_valid_access() {
    let mut slots = ConsumableSlots::new();
    slots
        .add_consumable(Box::new(MockConsumableForSlots { id: 42 }))
        .unwrap();

    // Test immutable access
    let consumable_ref = slots.get_consumable(0);
    assert!(consumable_ref.is_some());
    assert_eq!(consumable_ref.unwrap().get_mock_id(), 42);

    // Test mutable access
    let consumable_mut = slots.get_consumable_mut(0);
    assert!(consumable_mut.is_some());
    assert_eq!(consumable_mut.unwrap().get_mock_id(), 42);
}

#[test]
fn test_get_consumable_invalid_access() {
    let mut slots = ConsumableSlots::new();
    slots
        .add_consumable(Box::new(MockConsumableForSlots { id: 1 }))
        .unwrap();

    // Test out of bounds access
    assert!(slots.get_consumable(5).is_none());
    assert!(slots.get_consumable_mut(5).is_none());

    // Test empty slot access
    assert!(slots.get_consumable(1).is_none());
    assert!(slots.get_consumable_mut(1).is_none());
}

#[test]
fn test_find_empty_slot() {
    let mut slots = ConsumableSlots::new();

    // Initially, first slot should be empty
    assert_eq!(slots.find_empty_slot(), Some(0));

    // Add one consumable
    slots
        .add_consumable(Box::new(MockConsumableForSlots { id: 1 }))
        .unwrap();
    assert_eq!(slots.find_empty_slot(), Some(1));

    // Fill all slots
    slots
        .add_consumable(Box::new(MockConsumableForSlots { id: 2 }))
        .unwrap();
    assert_eq!(slots.find_empty_slot(), None);

    // Remove one and check
    slots.remove_consumable(0).unwrap();
    assert_eq!(slots.find_empty_slot(), Some(0));
}

#[test]
fn test_clear_slot() {
    let mut slots = ConsumableSlots::new();
    slots
        .add_consumable(Box::new(MockConsumableForSlots { id: 1 }))
        .unwrap();
    slots
        .add_consumable(Box::new(MockConsumableForSlots { id: 2 }))
        .unwrap();

    // Clear first slot
    let result = slots.clear_slot(0);
    assert!(result.is_ok());
    assert_eq!(slots.len(), 1);
    assert!(slots.get_consumable(0).is_none());
    assert!(slots.get_consumable(1).is_some());

    // Clear already empty slot (should still succeed)
    let result2 = slots.clear_slot(0);
    assert!(result2.is_ok());
    assert_eq!(slots.len(), 1);
}

#[test]
fn test_clear_slot_out_of_bounds() {
    let mut slots = ConsumableSlots::new();

    let result = slots.clear_slot(5);
    assert!(result.is_err());
    match result.unwrap_err() {
        balatro_rs::consumables::SlotError::IndexOutOfBounds { index, capacity } => {
            assert_eq!(index, 5);
            assert_eq!(capacity, 2);
        }
        _ => panic!("Expected IndexOutOfBounds error"),
    }
}

#[test]
fn test_consumable_slots_iterator() {
    let mut slots = ConsumableSlots::with_capacity(4);

    // Add some consumables with gaps
    slots
        .add_consumable(Box::new(MockConsumableForSlots { id: 10 }))
        .unwrap();
    slots
        .add_consumable(Box::new(MockConsumableForSlots { id: 20 }))
        .unwrap();
    slots
        .add_consumable(Box::new(MockConsumableForSlots { id: 30 }))
        .unwrap();

    // Remove middle one to create gap
    slots.remove_consumable(1).unwrap();

    // Iterator should skip empty slots
    let ids: Vec<u32> = slots.iter().map(|c| c.get_mock_id()).collect();
    assert_eq!(ids, vec![10, 30]);
    assert_eq!(ids.len(), 2);
}

#[test]
fn test_consumable_slots_iterator_empty() {
    let slots = ConsumableSlots::new();
    let count = slots.iter().count();
    assert_eq!(count, 0);
}

#[test]
fn test_slot_operations_with_custom_capacity() {
    let mut slots = ConsumableSlots::with_capacity(5);

    // Fill 3 slots
    for i in 1..=3 {
        slots
            .add_consumable(Box::new(MockConsumableForSlots { id: i }))
            .unwrap();
    }

    assert_eq!(slots.len(), 3);
    assert_eq!(slots.available_slots(), 2);
    assert!(!slots.is_full());

    // Find next empty slot
    assert_eq!(slots.find_empty_slot(), Some(3));

    // Remove middle slot
    slots.remove_consumable(1).unwrap();
    assert_eq!(slots.len(), 2);
    assert_eq!(slots.find_empty_slot(), Some(1)); // Should find gap
}

#[test]
fn test_slot_error_debug_display() {
    use balatro_rs::consumables::SlotError;

    let error1 = SlotError::IndexOutOfBounds {
        index: 5,
        capacity: 2,
    };
    let error2 = SlotError::NoEmptySlots { capacity: 3 };
    let error3 = SlotError::SlotEmpty { index: 1 };

    // Test debug output
    assert!(format!("{:?}", error1).contains("IndexOutOfBounds"));
    assert!(format!("{:?}", error2).contains("NoEmptySlots"));
    assert!(format!("{:?}", error3).contains("SlotEmpty"));

    // Test display output
    assert!(error1.to_string().contains("out of bounds"));
    assert!(error2.to_string().contains("No empty slots"));
    assert!(error3.to_string().contains("already empty"));
}

// Mock consumable for slot testing
#[derive(Debug)]
struct MockConsumableForSlots {
    id: u32,
}

impl MockConsumableForSlots {
    // Keep this helper method for convenience
    fn _get_mock_id(&self) -> u32 {
        self.id
    }
}

impl Consumable for MockConsumableForSlots {
    fn consumable_type(&self) -> ConsumableType {
        ConsumableType::Tarot
    }

    fn can_use(&self, _game_state: &Game, _target: &Target) -> bool {
        true
    }

    fn use_effect(&self, _game_state: &mut Game, _target: Target) -> Result<(), ConsumableError> {
        Ok(())
    }

    fn get_description(&self) -> String {
        format!("Mock consumable {}", self.id)
    }

    fn get_target_type(&self) -> TargetType {
        TargetType::None
    }

    fn get_effect_category(&self) -> ConsumableEffect {
        ConsumableEffect::Utility
    }

    fn get_mock_id(&self) -> u32 {
        self.id
    }
}

// Integration tests with actual consumable implementations
#[test]
fn test_integration_consumable_slots_with_real_consumables() {
    use balatro_rs::consumables::ConsumableId;

    let mut slots = ConsumableSlots::new();

    // Create instances of different consumable types
    let tarot_consumable = Box::new(RealConsumableWrapper {
        id: ConsumableId::TheFool,
        consumable_type: ConsumableType::Tarot,
    });

    let planet_consumable = Box::new(RealConsumableWrapper {
        id: ConsumableId::Mercury,
        consumable_type: ConsumableType::Planet,
    });

    // Test adding different types
    let result1 = slots.add_consumable(tarot_consumable);
    assert!(result1.is_ok());
    assert_eq!(result1.unwrap(), 0);

    let result2 = slots.add_consumable(planet_consumable);
    assert!(result2.is_ok());
    assert_eq!(result2.unwrap(), 1);

    // Test access
    assert!(slots.get_consumable(0).is_some());
    assert!(slots.get_consumable(1).is_some());

    // Test type checking
    assert_eq!(
        slots.get_consumable(0).unwrap().consumable_type(),
        ConsumableType::Tarot
    );
    assert_eq!(
        slots.get_consumable(1).unwrap().consumable_type(),
        ConsumableType::Planet
    );

    // Test iterator with mixed types
    let types: Vec<ConsumableType> = slots.iter().map(|c| c.consumable_type()).collect();
    assert_eq!(types, vec![ConsumableType::Tarot, ConsumableType::Planet]);
}

#[test]
fn test_integration_slot_operations_with_spectral_cards() {
    let mut slots = ConsumableSlots::with_capacity(3);

    // Add spectral cards
    let familiar = Box::new(RealConsumableWrapper {
        id: ConsumableId::Familiar,
        consumable_type: ConsumableType::Spectral,
    });

    let grim = Box::new(RealConsumableWrapper {
        id: ConsumableId::Grim,
        consumable_type: ConsumableType::Spectral,
    });

    let incantation = Box::new(RealConsumableWrapper {
        id: ConsumableId::Incantation,
        consumable_type: ConsumableType::Spectral,
    });

    // Fill slots
    slots.add_consumable(familiar).unwrap();
    slots.add_consumable(grim).unwrap();
    slots.add_consumable(incantation).unwrap();

    assert_eq!(slots.len(), 3);
    assert!(slots.is_full());

    // Remove middle card
    let removed = slots.remove_consumable(1).unwrap();
    assert_eq!(removed.get_real_id(), ConsumableId::Grim);

    // Verify gap created
    assert_eq!(slots.len(), 2);
    assert!(slots.get_consumable(1).is_none());
    assert!(slots.get_consumable(0).is_some());
    assert!(slots.get_consumable(2).is_some());

    // Add new card to fill gap
    let new_spectral = Box::new(RealConsumableWrapper {
        id: ConsumableId::SpectralPlaceholder,
        consumable_type: ConsumableType::Spectral,
    });

    let new_index = slots.add_consumable(new_spectral).unwrap();
    assert_eq!(new_index, 1); // Should fill the gap
}

#[test]
fn test_integration_mixed_consumable_types() {
    let mut slots = ConsumableSlots::with_capacity(4);

    // Add variety of consumable types
    let consumables = vec![
        Box::new(RealConsumableWrapper {
            id: ConsumableId::TheMagician,
            consumable_type: ConsumableType::Tarot,
        }),
        Box::new(RealConsumableWrapper {
            id: ConsumableId::Venus,
            consumable_type: ConsumableType::Planet,
        }),
        Box::new(RealConsumableWrapper {
            id: ConsumableId::Familiar,
            consumable_type: ConsumableType::Spectral,
        }),
        Box::new(RealConsumableWrapper {
            id: ConsumableId::TheEmperor,
            consumable_type: ConsumableType::Tarot,
        }),
    ];

    // Add all consumables
    for (i, consumable) in consumables.into_iter().enumerate() {
        let result = slots.add_consumable(consumable);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), i);
    }

    // Test iteration over mixed types
    let types: Vec<ConsumableType> = slots.iter().map(|c| c.consumable_type()).collect();
    assert_eq!(
        types,
        vec![
            ConsumableType::Tarot,
            ConsumableType::Planet,
            ConsumableType::Spectral,
            ConsumableType::Tarot
        ]
    );

    // Test targeted removal
    slots.remove_consumable(2).unwrap(); // Remove Spectral
    assert_eq!(slots.len(), 3);

    // Verify remaining types
    let remaining_types: Vec<ConsumableType> = slots.iter().map(|c| c.consumable_type()).collect();
    assert_eq!(
        remaining_types,
        vec![
            ConsumableType::Tarot,
            ConsumableType::Planet,
            ConsumableType::Tarot
        ]
    );
}

#[test]
fn test_integration_consumable_descriptions() {
    let mut slots = ConsumableSlots::new();

    let consumable = Box::new(RealConsumableWrapper {
        id: ConsumableId::TheHighPriestess,
        consumable_type: ConsumableType::Tarot,
    });

    slots.add_consumable(consumable).unwrap();

    // Test that description method works through trait object
    let stored_consumable = slots.get_consumable(0).unwrap();
    let description = stored_consumable.get_description();
    assert!(!description.is_empty());
    assert!(description.contains("High Priestess"));
}

#[test]
fn test_integration_consumable_target_types() {
    let mut slots = ConsumableSlots::with_capacity(3);

    // Add consumables with different target types
    let consumables = vec![
        (ConsumableId::TheFool, TargetType::None),
        (ConsumableId::TheMagician, TargetType::Cards(2)),
        (ConsumableId::Mercury, TargetType::HandType),
    ];

    for (id, _expected_target) in consumables {
        let consumable = Box::new(RealConsumableWrapper {
            id,
            consumable_type: id.consumable_type(),
        });

        slots.add_consumable(consumable).unwrap();
    }

    // Verify target types are preserved
    assert_eq!(
        slots.get_consumable(0).unwrap().get_target_type(),
        TargetType::None
    );
    assert_eq!(
        slots.get_consumable(1).unwrap().get_target_type(),
        TargetType::Cards(2)
    );
    assert_eq!(
        slots.get_consumable(2).unwrap().get_target_type(),
        TargetType::HandType
    );
}

// Real consumable wrapper for integration testing
#[derive(Debug)]
struct RealConsumableWrapper {
    id: ConsumableId,
    consumable_type: ConsumableType,
}

impl RealConsumableWrapper {
    // Keep this helper method for convenience
    fn _get_real_id(&self) -> ConsumableId {
        self.id
    }
}

impl Consumable for RealConsumableWrapper {
    fn consumable_type(&self) -> ConsumableType {
        self.consumable_type
    }

    fn can_use(&self, _game_state: &Game, _target: &Target) -> bool {
        true
    }

    fn use_effect(&self, _game_state: &mut Game, _target: Target) -> Result<(), ConsumableError> {
        Ok(())
    }

    fn get_description(&self) -> String {
        format!("Real consumable: {}", self.id)
    }

    fn get_target_type(&self) -> TargetType {
        match self.id {
            ConsumableId::TheFool => TargetType::None,
            ConsumableId::TheMagician => TargetType::Cards(2),
            ConsumableId::TheHighPriestess => TargetType::None,
            ConsumableId::TheEmperor => TargetType::None,
            ConsumableId::TheHierophant => TargetType::Cards(2),
            ConsumableId::Mercury => TargetType::HandType,
            ConsumableId::Venus => TargetType::HandType,
            ConsumableId::Earth => TargetType::HandType,
            ConsumableId::Mars => TargetType::HandType,
            ConsumableId::Jupiter => TargetType::HandType,
            ConsumableId::Familiar => TargetType::Cards(1),
            ConsumableId::Grim => TargetType::Cards(1),
            ConsumableId::Incantation => TargetType::Cards(1),
            _ => TargetType::None,
        }
    }

    fn get_effect_category(&self) -> ConsumableEffect {
        match self.consumable_type {
            ConsumableType::Tarot => ConsumableEffect::Enhancement,
            ConsumableType::Planet => ConsumableEffect::Modification,
            ConsumableType::Spectral => ConsumableEffect::Generation,
        }
    }

    fn get_real_id(&self) -> ConsumableId {
        self.id
    }
}
