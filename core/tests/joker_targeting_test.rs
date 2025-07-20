use balatro_rs::consumables::{JokerTarget, JokerTargetError, Target};
use balatro_rs::game::Game;
use balatro_rs::joker::{Joker, JokerId, JokerRarity};

/// Mock joker for testing purposes
#[derive(Debug, Clone)]
struct MockJoker {
    id: JokerId,
    name: &'static str,
}

impl MockJoker {
    fn new(id: JokerId, name: &'static str) -> Self {
        Self { id, name }
    }
}

impl Joker for MockJoker {
    fn id(&self) -> JokerId {
        self.id
    }

    fn name(&self) -> &str {
        self.name
    }

    fn description(&self) -> &str {
        "Mock joker for testing"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Common
    }
}

/// Helper function to create a game with mock jokers
fn create_game_with_jokers(joker_count: usize) -> Game {
    let mut game = Game::default();

    // Add mock jokers to the game
    for i in 0..joker_count {
        let joker_id = match i % 3 {
            0 => JokerId::Joker,
            1 => JokerId::GreedyJoker,
            _ => JokerId::LustyJoker,
        };
        let joker = Box::new(MockJoker::new(joker_id, "Mock Joker"));
        game.jokers.push(joker);
    }

    game
}

#[test]
fn test_joker_target_new() {
    let target = JokerTarget::new(2);

    assert_eq!(target.slot, 2);
    assert!(!target.require_active);
    assert_eq!(target.joker_type, None);
}

#[test]
fn test_joker_target_active_joker() {
    let target = JokerTarget::active_joker(1);

    assert_eq!(target.slot, 1);
    assert!(target.require_active);
    assert_eq!(target.joker_type, None);
}

#[test]
fn test_joker_target_joker_of_type() {
    let target = JokerTarget::joker_of_type(3, JokerId::Joker);

    assert_eq!(target.slot, 3);
    assert!(!target.require_active);
    assert_eq!(target.joker_type, Some(JokerId::Joker));
}

#[test]
fn test_joker_target_validate_valid_slot() {
    let game = create_game_with_jokers(3);
    let target = JokerTarget::new(1);

    let result = target.validate(&game);
    assert!(result.is_ok());
}

#[test]
fn test_joker_target_validate_empty_slot() {
    let game = create_game_with_jokers(2);
    let target = JokerTarget::new(3); // Slot 3 doesn't exist (only 0, 1)

    let result = target.validate(&game);
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        JokerTargetError::EmptySlot { slot: 3 }
    ));
}

#[test]
fn test_joker_target_validate_wrong_joker_type() {
    let game = create_game_with_jokers(3);
    // Slot 0 has Joker, but we're expecting GreedyJoker
    let target = JokerTarget::joker_of_type(0, JokerId::GreedyJoker);

    let result = target.validate(&game);
    assert!(result.is_err());
    match result.unwrap_err() {
        JokerTargetError::WrongJokerType { expected, actual } => {
            assert_eq!(expected, JokerId::GreedyJoker);
            assert_eq!(actual, JokerId::Joker);
        }
        _ => panic!("Expected WrongJokerType error"),
    }
}

#[test]
fn test_joker_target_validate_correct_joker_type() {
    let game = create_game_with_jokers(3);
    // Slot 0 has Joker, and we're expecting Joker
    let target = JokerTarget::joker_of_type(0, JokerId::Joker);

    let result = target.validate(&game);
    assert!(result.is_ok());
}

#[test]
fn test_joker_target_get_joker_valid() {
    let game = create_game_with_jokers(3);
    let target = JokerTarget::new(1);

    let result = target.get_joker(&game);
    assert!(result.is_ok());
    let joker = result.unwrap();
    assert_eq!(joker.id(), JokerId::GreedyJoker); // Second joker has GreedyJoker
}

#[test]
fn test_joker_target_get_joker_invalid() {
    let game = create_game_with_jokers(2);
    let target = JokerTarget::new(5); // Invalid slot

    let result = target.get_joker(&game);
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        JokerTargetError::EmptySlot { slot: 5 }
    ));
}

#[test]
fn test_joker_target_is_slot_occupied() {
    let game = create_game_with_jokers(3);

    let target_valid = JokerTarget::new(1);
    let target_invalid = JokerTarget::new(5);

    assert!(target_valid.is_slot_occupied(&game));
    assert!(!target_invalid.is_slot_occupied(&game));
}

#[test]
fn test_joker_target_error_display() {
    let empty_slot_error = JokerTargetError::EmptySlot { slot: 2 };
    let inactive_joker_error = JokerTargetError::InactiveJoker { slot: 1 };
    let wrong_type_error = JokerTargetError::WrongJokerType {
        expected: JokerId::Joker,
        actual: JokerId::GreedyJoker,
    };

    assert!(empty_slot_error
        .to_string()
        .contains("Joker slot 2 is empty"));
    assert!(inactive_joker_error
        .to_string()
        .contains("Joker at slot 1 is not active"));
    assert!(wrong_type_error.to_string().contains("Expected joker type"));
    assert!(wrong_type_error.to_string().contains("but found"));
}

#[test]
fn test_joker_target_equality_and_clone() {
    let target1 = JokerTarget::new(2);
    let target2 = JokerTarget::new(2);
    let target3 = JokerTarget::active_joker(2);

    assert_eq!(target1, target2);
    assert_ne!(target1, target3);

    let cloned = target1.clone();
    assert_eq!(target1, cloned);
}

#[test]
fn test_joker_target_serialization() {
    use serde_json;

    let target = JokerTarget::joker_of_type(3, JokerId::Joker);

    // Test serialization
    let serialized = serde_json::to_string(&target);
    assert!(serialized.is_ok());

    // Test deserialization
    let json = serialized.unwrap();
    let deserialized: Result<JokerTarget, _> = serde_json::from_str(&json);
    assert!(deserialized.is_ok());

    let restored = deserialized.unwrap();
    assert_eq!(target, restored);
}

// Tests for new Target enum methods

#[test]
fn test_target_as_joker_target() {
    let joker_target = Target::Joker(2);
    let card_target = Target::Cards(vec![0, 1]);
    let none_target = Target::None;

    // Test conversion from Target::Joker
    let result = joker_target.as_joker_target();
    assert!(result.is_some());
    let joker_target_struct = result.unwrap();
    assert_eq!(joker_target_struct.slot, 2);
    assert!(!joker_target_struct.require_active);
    assert_eq!(joker_target_struct.joker_type, None);

    // Test conversion from non-joker targets
    assert!(card_target.as_joker_target().is_none());
    assert!(none_target.as_joker_target().is_none());
}

#[test]
fn test_target_joker_at_slot() {
    let target = Target::joker_at_slot(5);

    assert!(matches!(target, Target::Joker(5)));
    assert_eq!(
        target.target_type(),
        balatro_rs::consumables::TargetType::Joker
    );
}

#[test]
fn test_target_active_joker_at_slot() {
    let target = Target::active_joker_at_slot(3);

    // Note: This currently returns Target::Joker(3) since Target enum doesn't store active requirement
    // For full active joker validation, users should use JokerTarget::active_joker directly
    assert!(matches!(target, Target::Joker(3)));
    assert_eq!(
        target.target_type(),
        balatro_rs::consumables::TargetType::Joker
    );
}

#[test]
fn test_target_joker_methods_integration() {
    let game = create_game_with_jokers(4);

    // Test the workflow: Target -> JokerTarget -> validation
    let target = Target::joker_at_slot(2);
    let joker_target = target.as_joker_target().unwrap();
    let validation_result = joker_target.validate(&game);

    assert!(validation_result.is_ok());

    // Test getting the joker
    let joker = joker_target.get_joker(&game).unwrap();
    assert_eq!(joker.id(), JokerId::LustyJoker); // Third joker (index 2) has LustyJoker
}

#[test]
fn test_target_joker_invalid_slot() {
    let game = create_game_with_jokers(2);

    let target = Target::joker_at_slot(5);
    let joker_target = target.as_joker_target().unwrap();
    let validation_result = joker_target.validate(&game);

    assert!(validation_result.is_err());
    assert!(matches!(
        validation_result.unwrap_err(),
        JokerTargetError::EmptySlot { slot: 5 }
    ));
}

#[test]
fn test_joker_target_edge_cases() {
    let empty_game = Game::default();

    // Test with empty game (no jokers)
    let target = JokerTarget::new(0);
    let result = target.validate(&empty_game);
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        JokerTargetError::EmptySlot { slot: 0 }
    ));

    // Test is_slot_occupied with empty game
    assert!(!target.is_slot_occupied(&empty_game));
}

#[test]
fn test_joker_target_comprehensive_validation() {
    let game = create_game_with_jokers(5);

    // Test various targeting scenarios
    let test_cases = vec![
        (JokerTarget::new(0), true, "Basic targeting slot 0"),
        (JokerTarget::new(4), true, "Basic targeting last slot"),
        (JokerTarget::new(5), false, "Targeting beyond bounds"),
        (JokerTarget::active_joker(0), true, "Active joker slot 0"),
        (
            JokerTarget::active_joker(10),
            false,
            "Active joker beyond bounds",
        ),
        (
            JokerTarget::joker_of_type(0, JokerId::Joker),
            true,
            "Correct type slot 0",
        ),
        (
            JokerTarget::joker_of_type(1, JokerId::Joker),
            false,
            "Wrong type slot 1",
        ),
        (
            JokerTarget::joker_of_type(2, JokerId::LustyJoker),
            true,
            "Correct type slot 2",
        ),
    ];

    for (target, should_pass, description) in test_cases {
        let result = target.validate(&game);
        if should_pass {
            assert!(result.is_ok(), "Failed: {}", description);
        } else {
            assert!(result.is_err(), "Should have failed: {}", description);
        }
    }
}

#[test]
fn test_joker_target_with_different_game_states() {
    // Test with various joker configurations
    let single_joker_game = create_game_with_jokers(1);
    let many_jokers_game = create_game_with_jokers(10);

    // Test single joker game
    let target = JokerTarget::new(0);
    assert!(target.validate(&single_joker_game).is_ok());
    assert!(!target.validate(&Game::default()).is_ok()); // Empty game

    let invalid_target = JokerTarget::new(1);
    assert!(!invalid_target.validate(&single_joker_game).is_ok());
    assert!(invalid_target.validate(&many_jokers_game).is_ok());
}

#[test]
fn test_joker_target_debug_output() {
    let target = JokerTarget::joker_of_type(2, JokerId::Joker);
    let debug_output = format!("{:?}", target);

    assert!(debug_output.contains("JokerTarget"));
    assert!(debug_output.contains("slot: 2"));
    assert!(debug_output.contains("require_active"));
    assert!(debug_output.contains("joker_type"));
}
