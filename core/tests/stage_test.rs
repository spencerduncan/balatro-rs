use balatro_rs::stage::{Blind, End, Stage};
use std::collections::HashMap;

#[cfg(test)]
mod blind_tests {
    use super::*;

    #[test]
    fn test_blind_reward() {
        assert_eq!(Blind::Small.reward(), 3);
        assert_eq!(Blind::Big.reward(), 4);
        assert_eq!(Blind::Boss.reward(), 5);
    }

    #[test]
    fn test_blind_next() {
        assert_eq!(Blind::Small.next(), Blind::Big);
        assert_eq!(Blind::Big.next(), Blind::Boss);
        assert_eq!(Blind::Boss.next(), Blind::Small);
    }

    #[test]
    fn test_blind_next_cycle() {
        // Test a complete cycle
        let mut current = Blind::Small;
        assert_eq!(current, Blind::Small);
        
        current = current.next();
        assert_eq!(current, Blind::Big);
        
        current = current.next();
        assert_eq!(current, Blind::Boss);
        
        current = current.next();
        assert_eq!(current, Blind::Small); // Back to start
    }

    #[test]
    fn test_blind_display() {
        assert_eq!(format!("{}", Blind::Small), "Small Blind");
        assert_eq!(format!("{}", Blind::Big), "Big Blind");
        assert_eq!(format!("{}", Blind::Boss), "Boss Blind");
    }

    #[test]
    fn test_blind_debug() {
        assert_eq!(format!("{:?}", Blind::Small), "Small");
        assert_eq!(format!("{:?}", Blind::Big), "Big");
        assert_eq!(format!("{:?}", Blind::Boss), "Boss");
    }

    #[test]
    fn test_blind_equality() {
        assert_eq!(Blind::Small, Blind::Small);
        assert_eq!(Blind::Big, Blind::Big);
        assert_eq!(Blind::Boss, Blind::Boss);
        assert_ne!(Blind::Small, Blind::Big);
        assert_ne!(Blind::Big, Blind::Boss);
        assert_ne!(Blind::Boss, Blind::Small);
    }

    #[test]
    fn test_blind_ordering() {
        assert!(Blind::Small < Blind::Big);
        assert!(Blind::Big < Blind::Boss);
        assert!(Blind::Small < Blind::Boss);
        
        // Test partial ordering
        assert!(Blind::Small <= Blind::Small);
        assert!(Blind::Small <= Blind::Big);
        assert!(Blind::Big >= Blind::Small);
    }

    #[test]
    fn test_blind_clone() {
        let blind = Blind::Big;
        let cloned_blind = blind.clone();
        assert_eq!(blind, cloned_blind);
    }

    #[test]
    fn test_blind_copy() {
        let blind = Blind::Boss;
        let copied_blind = blind; // Should be a copy due to Copy trait
        assert_eq!(blind, copied_blind);
        
        // Both should still be usable
        assert_eq!(blind.reward(), 5);
        assert_eq!(copied_blind.reward(), 5);
    }

    #[test]
    fn test_blind_hash() {
        let mut map = HashMap::new();
        map.insert(Blind::Small, "small_value");
        map.insert(Blind::Big, "big_value");
        map.insert(Blind::Boss, "boss_value");
        
        assert_eq!(map.get(&Blind::Small), Some(&"small_value"));
        assert_eq!(map.get(&Blind::Big), Some(&"big_value"));
        assert_eq!(map.get(&Blind::Boss), Some(&"boss_value"));
    }

    #[test]
    fn test_blind_reward_consistency() {
        // Test that rewards increase with blind difficulty
        assert!(Blind::Small.reward() < Blind::Big.reward());
        assert!(Blind::Big.reward() < Blind::Boss.reward());
    }

    #[test]
    fn test_all_blind_variants() {
        let blinds = vec![Blind::Small, Blind::Big, Blind::Boss];
        
        for blind in blinds {
            // Test that all methods work for all variants
            let _ = blind.reward();
            let _ = blind.next();
            let _ = format!("{}", blind);
            let _ = format!("{:?}", blind);
        }
    }
}

#[cfg(test)]
mod end_tests {
    use super::*;

    #[test]
    fn test_end_debug() {
        assert_eq!(format!("{:?}", End::Win), "Win");
        assert_eq!(format!("{:?}", End::Lose), "Lose");
    }

    #[test]
    fn test_end_equality() {
        assert_eq!(End::Win, End::Win);
        assert_eq!(End::Lose, End::Lose);
        assert_ne!(End::Win, End::Lose);
    }

    #[test]
    fn test_end_ordering() {
        // Win should be ordered before Lose (based on the enum definition)
        assert!(End::Win < End::Lose);
        assert!(End::Win <= End::Win);
        assert!(End::Lose >= End::Lose);
    }

    #[test]
    fn test_end_clone() {
        let end = End::Win;
        let cloned_end = end.clone();
        assert_eq!(end, cloned_end);
    }

    #[test]
    fn test_end_copy() {
        let end = End::Lose;
        let copied_end = end; // Should be a copy due to Copy trait
        assert_eq!(end, copied_end);
    }

    #[test]
    fn test_end_hash() {
        let mut map = HashMap::new();
        map.insert(End::Win, "win_value");
        map.insert(End::Lose, "lose_value");
        
        assert_eq!(map.get(&End::Win), Some(&"win_value"));
        assert_eq!(map.get(&End::Lose), Some(&"lose_value"));
    }
}

#[cfg(test)]
mod stage_tests {
    use super::*;

    #[test]
    fn test_stage_is_blind() {
        assert!(!Stage::PreBlind().is_blind());
        assert!(Stage::Blind(Blind::Small).is_blind());
        assert!(Stage::Blind(Blind::Big).is_blind());
        assert!(Stage::Blind(Blind::Boss).is_blind());
        assert!(!Stage::PostBlind().is_blind());
        assert!(!Stage::Shop().is_blind());
        assert!(!Stage::End(End::Win).is_blind());
        assert!(!Stage::End(End::Lose).is_blind());
    }

    #[test]
    fn test_stage_debug() {
        assert_eq!(format!("{:?}", Stage::PreBlind()), "PreBlind()");
        assert_eq!(format!("{:?}", Stage::Blind(Blind::Small)), "Blind(Small)");
        assert_eq!(format!("{:?}", Stage::PostBlind()), "PostBlind()");
        assert_eq!(format!("{:?}", Stage::Shop()), "Shop()");
        assert_eq!(format!("{:?}", Stage::End(End::Win)), "End(Win)");
    }

    #[test]
    fn test_stage_equality() {
        assert_eq!(Stage::PreBlind(), Stage::PreBlind());
        assert_eq!(Stage::PostBlind(), Stage::PostBlind());
        assert_eq!(Stage::Shop(), Stage::Shop());
        assert_eq!(Stage::Blind(Blind::Small), Stage::Blind(Blind::Small));
        assert_eq!(Stage::End(End::Win), Stage::End(End::Win));
        
        assert_ne!(Stage::PreBlind(), Stage::PostBlind());
        assert_ne!(Stage::Blind(Blind::Small), Stage::Blind(Blind::Big));
        assert_ne!(Stage::End(End::Win), Stage::End(End::Lose));
    }

    #[test]
    fn test_stage_ordering() {
        // Test basic ordering relationships
        assert!(Stage::PreBlind() < Stage::PostBlind());
        assert!(Stage::Blind(Blind::Small) < Stage::Blind(Blind::Big));
        assert!(Stage::End(End::Win) < Stage::End(End::Lose));
    }

    #[test]
    fn test_stage_clone() {
        let stage = Stage::Blind(Blind::Boss);
        let cloned_stage = stage.clone();
        assert_eq!(stage, cloned_stage);
    }

    #[test]
    fn test_stage_copy() {
        let stage = Stage::Shop();
        let copied_stage = stage; // Should be a copy due to Copy trait
        assert_eq!(stage, copied_stage);
    }

    #[test]
    fn test_stage_hash() {
        let mut map = HashMap::new();
        map.insert(Stage::PreBlind(), "pre_blind");
        map.insert(Stage::Blind(Blind::Small), "small_blind");
        map.insert(Stage::PostBlind(), "post_blind");
        map.insert(Stage::Shop(), "shop");
        map.insert(Stage::End(End::Win), "win");
        
        assert_eq!(map.get(&Stage::PreBlind()), Some(&"pre_blind"));
        assert_eq!(map.get(&Stage::Blind(Blind::Small)), Some(&"small_blind"));
        assert_eq!(map.get(&Stage::Shop()), Some(&"shop"));
    }

    #[test]
    fn test_all_stage_variants() {
        let stages = vec![
            Stage::PreBlind(),
            Stage::Blind(Blind::Small),
            Stage::Blind(Blind::Big),
            Stage::Blind(Blind::Boss),
            Stage::PostBlind(),
            Stage::Shop(),
            Stage::End(End::Win),
            Stage::End(End::Lose),
        ];
        
        for stage in stages {
            // Test that all methods work for all variants
            let _ = stage.is_blind();
            let _ = format!("{:?}", stage);
        }
    }

    #[test]
    fn test_stage_blind_extraction() {
        // Test pattern matching to extract blind from stage
        match Stage::Blind(Blind::Boss) {
            Stage::Blind(blind) => {
                assert_eq!(blind, Blind::Boss);
                assert_eq!(blind.reward(), 5);
            }
            _ => panic!("Expected Blind stage"),
        }
    }

    #[test]
    fn test_stage_end_extraction() {
        // Test pattern matching to extract end from stage
        match Stage::End(End::Win) {
            Stage::End(end) => {
                assert_eq!(end, End::Win);
            }
            _ => panic!("Expected End stage"),
        }
    }
}

#[cfg(all(test, feature = "python"))]
mod python_stage_tests {
    use super::*;

    #[test]
    fn test_stage_int_conversion() {
        assert_eq!(Stage::PreBlind().int(), 0);
        assert_eq!(Stage::Blind(Blind::Small).int(), 1);
        assert_eq!(Stage::Blind(Blind::Big).int(), 2);
        assert_eq!(Stage::Blind(Blind::Boss).int(), 3);
        assert_eq!(Stage::PostBlind().int(), 4);
        assert_eq!(Stage::Shop().int(), 5);
        assert_eq!(Stage::End(End::Win).int(), 6);
        assert_eq!(Stage::End(End::Lose).int(), 7);
    }

    #[test]
    fn test_stage_int_unique_values() {
        let stages = vec![
            Stage::PreBlind(),
            Stage::Blind(Blind::Small),
            Stage::Blind(Blind::Big),
            Stage::Blind(Blind::Boss),
            Stage::PostBlind(),
            Stage::Shop(),
            Stage::End(End::Win),
            Stage::End(End::Lose),
        ];
        
        let int_values: Vec<usize> = stages.iter().map(|s| s.int()).collect();
        
        // Verify all values are unique
        let mut sorted_values = int_values.clone();
        sorted_values.sort();
        sorted_values.dedup();
        
        assert_eq!(int_values.len(), sorted_values.len(), "Int values should be unique");
    }

    #[test]
    fn test_stage_int_sequential() {
        // Test that int values are sequential from 0 to 7
        let expected_sequence = vec![0, 1, 2, 3, 4, 5, 6, 7];
        let stages = vec![
            Stage::PreBlind(),
            Stage::Blind(Blind::Small),
            Stage::Blind(Blind::Big),
            Stage::Blind(Blind::Boss),
            Stage::PostBlind(),
            Stage::Shop(),
            Stage::End(End::Win),
            Stage::End(End::Lose),
        ];
        
        let int_values: Vec<usize> = stages.iter().map(|s| s.int()).collect();
        let mut sorted_values = int_values;
        sorted_values.sort();
        
        assert_eq!(sorted_values, expected_sequence);
    }
}

#[cfg(all(test, feature = "serde"))]
mod serde_tests {
    use super::*;

    #[test]
    fn test_blind_serde() {
        let blind = Blind::Boss;
        let serialized = serde_json::to_string(&blind).unwrap();
        let deserialized: Blind = serde_json::from_str(&serialized).unwrap();
        assert_eq!(blind, deserialized);
    }

    #[test]
    fn test_end_serde() {
        let end = End::Win;
        let serialized = serde_json::to_string(&end).unwrap();
        let deserialized: End = serde_json::from_str(&serialized).unwrap();
        assert_eq!(end, deserialized);
    }

    #[test]
    fn test_stage_serde() {
        let stages = vec![
            Stage::PreBlind(),
            Stage::Blind(Blind::Boss),
            Stage::PostBlind(),
            Stage::Shop(),
            Stage::End(End::Lose),
        ];
        
        for stage in stages {
            let serialized = serde_json::to_string(&stage).unwrap();
            let deserialized: Stage = serde_json::from_str(&serialized).unwrap();
            assert_eq!(stage, deserialized);
        }
    }
}

#[cfg(test)]
mod comprehensive_stage_tests {
    use super::*;

    #[test]
    fn test_game_flow_progression() {
        // Test a typical game flow progression
        let mut current_blind = Blind::Small;
        
        // Start with small blind
        assert_eq!(current_blind.reward(), 3);
        assert!(Stage::Blind(current_blind).is_blind());
        
        // Progress to big blind
        current_blind = current_blind.next();
        assert_eq!(current_blind, Blind::Big);
        assert_eq!(current_blind.reward(), 4);
        
        // Progress to boss blind
        current_blind = current_blind.next();
        assert_eq!(current_blind, Blind::Boss);
        assert_eq!(current_blind.reward(), 5);
        
        // Cycle back to small blind
        current_blind = current_blind.next();
        assert_eq!(current_blind, Blind::Small);
    }

    #[test]
    fn test_stage_type_safety() {
        // Test that stages with different inner types are properly distinct
        let small_blind_stage = Stage::Blind(Blind::Small);
        let big_blind_stage = Stage::Blind(Blind::Big);
        let win_end_stage = Stage::End(End::Win);
        let lose_end_stage = Stage::End(End::Lose);
        
        assert_ne!(small_blind_stage, big_blind_stage);
        assert_ne!(win_end_stage, lose_end_stage);
        
        // Test that is_blind correctly identifies blind stages
        assert!(small_blind_stage.is_blind());
        assert!(big_blind_stage.is_blind());
        assert!(!win_end_stage.is_blind());
        assert!(!lose_end_stage.is_blind());
    }

    #[test]
    fn test_blind_reward_total() {
        // Test total rewards for a complete blind cycle
        let total_reward = Blind::Small.reward() + Blind::Big.reward() + Blind::Boss.reward();
        assert_eq!(total_reward, 12); // 3 + 4 + 5
    }
}