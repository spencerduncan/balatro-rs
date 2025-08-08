//! Demonstration of the new test infrastructure from PR #779 salvage
//!
//! This file demonstrates the Day 1 test infrastructure implementation
//! from issue #916. Additional features will be added in Day 2-4 as per
//! the salvage plan.

// Import the test infrastructure
mod common;

#[cfg(test)]
mod tests {
    use super::common::fixtures::create_edge_case_scenarios;
    use super::common::prelude::*;

    #[test]
    fn test_basic_game_creation() {
        // Demonstrates basic test fixture usage
        let game = create_test_game();
        assert_game_running(&game);
    }

    #[test]
    fn test_deterministic_game_creation() {
        // Demonstrates seeded game creation
        // Note: seed parameter in Config not yet available
        let game1 = create_test_game();
        let game2 = create_test_game();

        // Both games start in similar state
        assert_game_running(&game1);
        assert_game_running(&game2);
    }

    #[test]
    fn test_game_state_builder() {
        // Demonstrates the builder pattern for test setup
        let game = GameStateBuilder::new().with_ante(2).with_money(50).build();

        assert_eq!(game.ante_current, balatro_rs::ante::Ante::Two);
        assert_eq!(game.money, 50.0);
    }

    #[test]
    fn test_hand_fixtures() {
        // Demonstrates test hand creation
        let royal_flush = create_test_hand(TestHandType::RoyalFlush);
        assert_eq!(royal_flush.len(), 5);

        let pair = create_test_hand(TestHandType::OnePair);
        assert_eq!(pair.len(), 5);
    }

    #[test]
    fn test_edge_case_scenarios() {
        // Demonstrates edge case testing
        let scenarios = create_edge_case_scenarios();
        assert!(!scenarios.is_empty());

        for scenario in scenarios {
            // Validate scenario properties
            assert!(scenario.ante >= 1 && scenario.ante <= 8);
        }
    }
}
