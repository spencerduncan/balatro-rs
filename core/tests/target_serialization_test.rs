use balatro_rs::consumables::{Target, TargetType};
use balatro_rs::rank::HandRank;
use serde_json;

#[test]
fn test_target_type_serialization() {
    // Test all TargetType variants can be serialized and deserialized
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

    for target_type in target_types {
        // Serialize to JSON
        let json = serde_json::to_string(&target_type).expect("Failed to serialize TargetType");
        assert!(!json.is_empty());

        // Deserialize from JSON
        let deserialized: TargetType =
            serde_json::from_str(&json).expect("Failed to deserialize TargetType");

        // Verify round-trip integrity
        assert_eq!(target_type, deserialized);
    }
}

#[test]
fn test_target_serialization() {
    // Test all Target variants can be serialized and deserialized
    let targets = vec![
        Target::None,
        Target::cards_in_hand(vec![]),
        Target::cards_in_hand(vec![0]),
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
        Target::Joker(10),
        Target::Deck,
        Target::Shop(0),
        Target::Shop(1),
        Target::Shop(5),
        Target::Shop(10),
    ];

    for target in targets {
        // Serialize to JSON
        let json = serde_json::to_string(&target).expect("Failed to serialize Target");
        assert!(!json.is_empty());

        // Deserialize from JSON
        let deserialized: Target =
            serde_json::from_str(&json).expect("Failed to deserialize Target");

        // Verify round-trip integrity
        assert_eq!(target, deserialized);
    }
}

#[test]
fn test_target_type_json_format_stability() {
    // Test that serialized format is stable for save/load compatibility
    let test_cases = vec![
        (TargetType::None, r#""None""#),
        (TargetType::Cards(3), r#"{"Cards":3}"#),
        (TargetType::HandType, r#""HandType""#),
        (TargetType::Joker, r#""Joker""#),
        (TargetType::Deck, r#""Deck""#),
        (TargetType::Shop, r#""Shop""#),
    ];

    for (target_type, expected_json) in test_cases {
        // Test serialization produces expected format
        let json = serde_json::to_string(&target_type).expect("Failed to serialize");
        assert_eq!(json, expected_json);

        // Test deserialization from expected format
        let deserialized: TargetType =
            serde_json::from_str(expected_json).expect("Failed to deserialize");
        assert_eq!(target_type, deserialized);
    }
}

#[test]
fn test_target_json_format_stability() {
    // Test that serialized format is stable for save/load compatibility
    let test_cases = vec![
        (Target::None, r#""None""#),
        (Target::cards_in_hand(vec![0, 1]), r#"{"Cards":{"indices":[0,1],"collection":"Hand","min_cards":2,"max_cards":2}}"#),
        (
            Target::HandType(HandRank::OnePair),
            r#"{"HandType":"OnePair"}"#,
        ),
        (Target::Joker(3), r#"{"Joker":3}"#),
        (Target::Deck, r#""Deck""#),
        (Target::Shop(2), r#"{"Shop":2}"#),
    ];

    for (target, expected_json) in test_cases {
        // Test serialization produces expected format
        let json = serde_json::to_string(&target).expect("Failed to serialize");
        assert_eq!(json, expected_json);

        // Test deserialization from expected format
        let deserialized: Target =
            serde_json::from_str(expected_json).expect("Failed to deserialize");
        assert_eq!(target, deserialized);
    }
}

#[test]
fn test_complex_target_serialization() {
    // Test serialization of complex Target variants
    let complex_targets = vec![
        Target::cards_in_hand(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]),
        Target::cards_in_hand((0..100).collect()),
        Target::HandType(HandRank::FlushFive),
        Target::Joker(999),
        Target::Shop(999),
    ];

    for target in complex_targets {
        // Serialize to JSON
        let json = serde_json::to_string(&target).expect("Failed to serialize complex Target");

        // Deserialize from JSON
        let deserialized: Target =
            serde_json::from_str(&json).expect("Failed to deserialize complex Target");

        // Verify round-trip integrity
        assert_eq!(target, deserialized);
    }
}

#[test]
fn test_backwards_compatibility_deserialization() {
    // Test that old serialized data can still be deserialized
    // This ensures save/load compatibility when the enum is extended in the future
    let legacy_json_examples = vec![
        (r#""None""#, Target::None),
        (r#"{"Cards":{"indices":[0],"collection":"Hand","min_cards":1,"max_cards":1}}"#, Target::cards_in_hand(vec![0])),
        (
            r#"{"HandType":"FullHouse"}"#,
            Target::HandType(HandRank::FullHouse),
        ),
        (r#"{"Joker":0}"#, Target::Joker(0)),
        (r#""Deck""#, Target::Deck),
        // Note: Shop variant is new, so no legacy examples
    ];

    for (json, expected_target) in legacy_json_examples {
        let deserialized: Target =
            serde_json::from_str(json).expect("Failed to deserialize legacy JSON");
        assert_eq!(deserialized, expected_target);
    }
}

#[test]
fn test_serialization_error_handling() {
    // Test that invalid JSON produces appropriate errors
    let invalid_json_cases = vec![
        r#"{"InvalidVariant":123}"#,
        r#"{"Cards":"invalid"}"#,
        r#"{"HandType":123}"#,
        r#"{"Joker":"invalid"}"#,
        r#"{"Shop":"invalid"}"#,
        r#""InvalidString""#,
        r#"null"#,
        r#"123"#,
    ];

    for invalid_json in invalid_json_cases {
        let result: Result<Target, _> = serde_json::from_str(invalid_json);
        assert!(
            result.is_err(),
            "Expected error for invalid JSON: {}",
            invalid_json
        );
    }
}

#[test]
fn test_nested_serialization_with_game_state() {
    // Test serialization when Target is embedded in larger structures
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct MockConsumableAction {
        consumable_id: u32,
        target: Target,
        target_type: TargetType,
    }

    let actions = vec![
        MockConsumableAction {
            consumable_id: 1,
            target: Target::None,
            target_type: TargetType::None,
        },
        MockConsumableAction {
            consumable_id: 2,
            target: Target::cards_in_hand(vec![0, 1]),
            target_type: TargetType::Cards(2),
        },
        MockConsumableAction {
            consumable_id: 3,
            target: Target::Shop(1),
            target_type: TargetType::Shop,
        },
    ];

    for action in actions {
        // Serialize to JSON
        let json = serde_json::to_string(&action).expect("Failed to serialize action");

        // Deserialize from JSON
        let deserialized: MockConsumableAction =
            serde_json::from_str(&json).expect("Failed to deserialize action");

        // Verify round-trip integrity
        assert_eq!(action, deserialized);
    }
}
