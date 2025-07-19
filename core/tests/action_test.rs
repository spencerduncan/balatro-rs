use balatro_rs::action::{Action, MoveDirection};
use balatro_rs::card::{Card, Rank, Suit};
use balatro_rs::joker::JokerId;
use balatro_rs::shop::packs::PackType;
use balatro_rs::stage::Blind;
use std::fmt::Write;

#[cfg(test)]
mod move_direction_tests {
    use super::*;

    #[test]
    fn test_move_direction_display() {
        assert_eq!(format!("{}", MoveDirection::Left), "left");
        assert_eq!(format!("{}", MoveDirection::Right), "right");
    }

    #[test]
    fn test_move_direction_debug() {
        assert_eq!(format!("{:?}", MoveDirection::Left), "Left");
        assert_eq!(format!("{:?}", MoveDirection::Right), "Right");
    }

    #[test]
    fn test_move_direction_equality() {
        assert_eq!(MoveDirection::Left, MoveDirection::Left);
        assert_eq!(MoveDirection::Right, MoveDirection::Right);
        assert_ne!(MoveDirection::Left, MoveDirection::Right);
    }

    #[test]
    fn test_move_direction_clone() {
        let left = MoveDirection::Left;
        let cloned_left = left.clone();
        assert_eq!(left, cloned_left);
    }

    #[test]
    fn test_move_direction_hash() {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        map.insert(MoveDirection::Left, "left_value");
        map.insert(MoveDirection::Right, "right_value");
        
        assert_eq!(map.get(&MoveDirection::Left), Some(&"left_value"));
        assert_eq!(map.get(&MoveDirection::Right), Some(&"right_value"));
    }
}

#[cfg(test)]
mod action_tests {
    use super::*;

    fn create_test_card() -> Card {
        Card::new(Rank::Ace, Suit::Spades)
    }

    #[test]
    fn test_action_select_card_display() {
        let card = create_test_card();
        let action = Action::SelectCard(card.clone());
        let display_string = format!("{}", action);
        assert!(display_string.contains("SelectCard:"));
        assert!(display_string.contains("Ace"));
        assert!(display_string.contains("♠"));
    }

    #[test]
    fn test_action_move_card_display() {
        let card = create_test_card();
        let action = Action::MoveCard(MoveDirection::Left, card.clone());
        let display_string = format!("{}", action);
        assert!(display_string.contains("MoveCard:"));
        assert!(display_string.contains("Ace"));
        assert!(display_string.contains("♠"));
        assert!(display_string.contains("left"));
    }

    #[test]
    fn test_action_play_display() {
        let action = Action::Play();
        assert_eq!(format!("{}", action), "Play");
    }

    #[test]
    fn test_action_discard_display() {
        let action = Action::Discard();
        assert_eq!(format!("{}", action), "Discard");
    }

    #[test]
    fn test_action_cash_out_display() {
        let action = Action::CashOut(150);
        assert_eq!(format!("{}", action), "CashOut: 150");
    }

    #[test]
    fn test_action_buy_joker_display() {
        let action = Action::BuyJoker { 
            joker_id: JokerId::Joker, 
            slot: 3 
        };
        let display_string = format!("{}", action);
        assert!(display_string.contains("BuyJoker:"));
        assert!(display_string.contains("Joker"));
        assert!(display_string.contains("slot 3"));
    }

    #[test]
    fn test_action_buy_pack_display() {
        let action = Action::BuyPack { 
            pack_type: PackType::Standard 
        };
        let display_string = format!("{}", action);
        assert!(display_string.contains("BuyPack:"));
        assert!(display_string.contains("Standard"));
    }

    #[test]
    fn test_action_open_pack_display() {
        let action = Action::OpenPack { pack_id: 42 };
        assert_eq!(format!("{}", action), "OpenPack: 42");
    }

    #[test]
    fn test_action_select_from_pack_display() {
        let action = Action::SelectFromPack { 
            pack_id: 10, 
            option_index: 2 
        };
        assert_eq!(format!("{}", action), "SelectFromPack: pack 10, option 2");
    }

    #[test]
    fn test_action_skip_pack_display() {
        let action = Action::SkipPack { pack_id: 7 };
        assert_eq!(format!("{}", action), "SkipPack: 7");
    }

    #[test]
    fn test_action_next_round_display() {
        let action = Action::NextRound();
        assert_eq!(format!("{}", action), "NextRound");
    }

    #[test]
    fn test_action_select_blind_display() {
        let action = Action::SelectBlind(Blind::Small);
        let display_string = format!("{}", action);
        assert!(display_string.contains("SelectBlind:"));
        assert!(display_string.contains("Small"));
    }

    #[test]
    fn test_action_equality() {
        let card1 = create_test_card();
        let card2 = create_test_card();
        
        assert_eq!(Action::Play(), Action::Play());
        assert_eq!(Action::Discard(), Action::Discard());
        assert_eq!(Action::SelectCard(card1.clone()), Action::SelectCard(card2));
        assert_ne!(Action::Play(), Action::Discard());
    }

    #[test]
    fn test_action_clone() {
        let action = Action::CashOut(100);
        let cloned_action = action.clone();
        assert_eq!(action, cloned_action);
    }

    #[test]
    fn test_action_debug() {
        let action = Action::Play();
        let debug_string = format!("{:?}", action);
        assert_eq!(debug_string, "Play()");
    }

    #[test]
    fn test_action_hash() {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        map.insert(Action::Play(), "play_value");
        map.insert(Action::Discard(), "discard_value");
        
        assert_eq!(map.get(&Action::Play()), Some(&"play_value"));
        assert_eq!(map.get(&Action::Discard()), Some(&"discard_value"));
    }

    #[test]
    fn test_all_action_variants() {
        let card = create_test_card();
        
        // Test that all action variants can be constructed without panicking
        let actions = vec![
            Action::SelectCard(card.clone()),
            Action::MoveCard(MoveDirection::Left, card),
            Action::Play(),
            Action::Discard(),
            Action::CashOut(100),
            Action::BuyJoker { joker_id: JokerId::Joker, slot: 0 },
            Action::BuyPack { pack_type: PackType::Standard },
            Action::OpenPack { pack_id: 1 },
            Action::SelectFromPack { pack_id: 1, option_index: 0 },
            Action::SkipPack { pack_id: 1 },
            Action::NextRound(),
            Action::SelectBlind(Blind::Small),
        ];
        
        // Verify all actions can be displayed and debugged
        for action in actions {
            let _ = format!("{}", action);
            let _ = format!("{:?}", action);
        }
    }

    #[test]
    fn test_action_edge_cases() {
        // Test edge case values
        let action_large_cash = Action::CashOut(usize::MAX);
        assert!(format!("{}", action_large_cash).contains(&format!("{}", usize::MAX)));
        
        let action_zero_cash = Action::CashOut(0);
        assert_eq!(format!("{}", action_zero_cash), "CashOut: 0");
    }
}

#[cfg(all(test, feature = "python"))]
mod python_action_tests {
    use super::*;

    #[test]
    fn test_action_repr() {
        let action = Action::Play();
        let repr_string = action.__repr__();
        assert_eq!(repr_string, "Action: Play");
    }

    #[test]
    fn test_action_repr_with_data() {
        let action = Action::CashOut(250);
        let repr_string = action.__repr__();
        assert_eq!(repr_string, "Action: CashOut: 250");
    }
}

#[cfg(all(test, feature = "serde"))]
mod serde_tests {
    use super::*;

    #[test]
    fn test_move_direction_serde() {
        let left = MoveDirection::Left;
        let serialized = serde_json::to_string(&left).unwrap();
        let deserialized: MoveDirection = serde_json::from_str(&serialized).unwrap();
        assert_eq!(left, deserialized);
    }

    #[test]
    fn test_action_serde() {
        let action = Action::Play();
        let serialized = serde_json::to_string(&action).unwrap();
        let deserialized: Action = serde_json::from_str(&serialized).unwrap();
        assert_eq!(action, deserialized);
    }

    #[test]
    fn test_complex_action_serde() {
        let action = Action::BuyJoker { 
            joker_id: JokerId::Joker, 
            slot: 5 
        };
        let serialized = serde_json::to_string(&action).unwrap();
        let deserialized: Action = serde_json::from_str(&serialized).unwrap();
        assert_eq!(action, deserialized);
    }
}