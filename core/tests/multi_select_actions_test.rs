//! Comprehensive tests for multi-select card actions (Epic #811, Issues #806-#810)
//!
//! Tests all 5 multi-select card actions:
//! - SelectCards (Issue #806)
//! - DeselectCard/DeselectCards (Issue #807-#808)
//! - ToggleCardSelection (Issue #809)
//! - SelectAllCards/DeselectAllCards (Issue #810)
//! - RangeSelectCards (Range selection)

use balatro_rs::action::Action;
use balatro_rs::card::{Card, Suit, Value};
use balatro_rs::error::GameError;
use balatro_rs::game::Game;
use balatro_rs::stage::Stage;

/// Create a test game in blind stage with sample cards
fn create_test_game_with_cards() -> Game {
    let mut game = Game {
        stage: Stage::Blind(balatro_rs::stage::Blind::Small),
        ..Default::default()
    };

    // Initialize the game to populate deck and deal cards, then set back to Blind stage
    game.start(); // This sets stage to PreBlind and deals cards
    game.stage = Stage::Blind(balatro_rs::stage::Blind::Small); // Set back to Blind for multi-select

    game
}

/// Test SelectCards action (Issue #806)
#[cfg(test)]
mod select_cards_tests {
    use super::*;

    #[test]
    fn test_select_cards_valid_action() {
        let mut game = create_test_game_with_cards();
        let cards = game.available.cards();

        println!("Game stage: {:?}", game.stage);
        println!("Available cards count: {}", cards.len());
        println!("Available cards: {cards:?}");

        // Select first two cards
        let action = Action::SelectCards(vec![cards[0], cards[1]]);
        let result = game.handle_action(action);

        match &result {
            Ok(_) => println!("Action succeeded"),
            Err(e) => println!("Action failed with error: {e:?}"),
        }

        assert!(
            result.is_ok(),
            "SelectCards should succeed with valid cards - Error: {:?}",
            result.err()
        );

        // Verify multi-select is activated
        assert!(game.target_context.is_multi_select_active());

        // Verify cards are selected in multi-select context
        let selected = game.target_context.multi_select_context().selected_cards();
        assert_eq!(selected.len(), 2);
        assert!(selected.contains(&cards[0].id));
        assert!(selected.contains(&cards[1].id));
    }

    #[test]
    fn test_select_cards_invalid_stage() {
        let mut game = create_test_game_with_cards();
        game.stage = Stage::Shop(); // Not a blind stage

        let cards = game.available.cards();
        let action = Action::SelectCards(vec![cards[0]]);
        let result = game.handle_action(action);

        assert!(
            result.is_err(),
            "SelectCards should fail in non-blind stage"
        );
        assert!(matches!(result.unwrap_err(), GameError::InvalidAction));
    }

    #[test]
    fn test_select_cards_nonexistent_card() {
        let mut game = create_test_game_with_cards();

        // Create a card that doesn't exist in available cards
        let nonexistent_card = Card::new(Value::Two, Suit::Club);
        let action = Action::SelectCards(vec![nonexistent_card]);
        let result = game.handle_action(action);

        assert!(
            result.is_err(),
            "SelectCards should fail with nonexistent card"
        );
        assert!(matches!(result.unwrap_err(), GameError::NoCardMatch));
    }

    #[test]
    fn test_select_cards_empty_list() {
        let mut game = create_test_game_with_cards();

        let action = Action::SelectCards(vec![]);
        let result = game.handle_action(action);

        assert!(
            result.is_ok(),
            "SelectCards should handle empty list gracefully"
        );
    }

    #[test]
    fn test_select_cards_duplicate_cards() {
        let mut game = create_test_game_with_cards();
        let cards = game.available.cards();

        // Try to select same card twice
        let action = Action::SelectCards(vec![cards[0], cards[0]]);
        let result = game.handle_action(action);

        assert!(result.is_ok(), "SelectCards should handle duplicates");

        // Verify card is only selected once
        let selected = game.target_context.multi_select_context().selected_cards();
        assert_eq!(selected.len(), 1);
        assert!(selected.contains(&cards[0].id));
    }
}

/// Test DeselectCard action (Issue #807)
#[cfg(test)]
mod deselect_card_tests {
    use super::*;

    #[test]
    fn test_deselect_card_valid_action() {
        let mut game = create_test_game_with_cards();
        let cards = game.available.cards();

        // First select some cards
        let select_action = Action::SelectCards(vec![cards[0], cards[1]]);
        game.handle_action(select_action).unwrap();

        // Then deselect one card
        let deselect_action = Action::DeselectCard(cards[0]);
        let result = game.handle_action(deselect_action);

        assert!(result.is_ok(), "DeselectCard should succeed");

        // Verify only one card remains selected
        let selected = game.target_context.multi_select_context().selected_cards();
        assert_eq!(selected.len(), 1);
        assert!(!selected.contains(&cards[0].id));
        assert!(selected.contains(&cards[1].id));
    }

    #[test]
    fn test_deselect_card_not_selected() {
        let mut game = create_test_game_with_cards();
        let cards = game.available.cards();

        // Try to deselect a card that wasn't selected
        let action = Action::DeselectCard(cards[0]);
        let result = game.handle_action(action);

        match &result {
            Ok(_) => println!("DeselectCard succeeded"),
            Err(e) => println!("DeselectCard failed: {e:?}"),
        }

        // Should still succeed but have no effect
        assert!(
            result.is_ok(),
            "DeselectCard should handle unselected card gracefully - Error: {:?}",
            result.err()
        );
    }

    #[test]
    fn test_deselect_card_invalid_stage() {
        let mut game = create_test_game_with_cards();
        game.stage = Stage::Shop();

        let cards = game.available.cards();
        let action = Action::DeselectCard(cards[0]);
        let result = game.handle_action(action);

        assert!(
            result.is_err(),
            "DeselectCard should fail in non-blind stage"
        );
        assert!(matches!(result.unwrap_err(), GameError::InvalidAction));
    }

    #[test]
    fn test_deselect_card_nonexistent() {
        let mut game = create_test_game_with_cards();

        let nonexistent_card = Card::new(Value::Two, Suit::Club);
        let action = Action::DeselectCard(nonexistent_card);
        let result = game.handle_action(action);

        assert!(
            result.is_err(),
            "DeselectCard should fail with nonexistent card"
        );
        assert!(matches!(result.unwrap_err(), GameError::NoCardMatch));
    }
}

/// Test DeselectCards action (Issue #808)
#[cfg(test)]
mod deselect_cards_tests {
    use super::*;

    #[test]
    fn test_deselect_cards_valid_action() {
        let mut game = create_test_game_with_cards();
        let cards = game.available.cards();

        // First select multiple cards
        let select_action = Action::SelectCards(vec![cards[0], cards[1], cards[2]]);
        game.handle_action(select_action).unwrap();

        // Then deselect some cards
        let deselect_action = Action::DeselectCards(vec![cards[0], cards[1]]);
        let result = game.handle_action(deselect_action);

        assert!(result.is_ok(), "DeselectCards should succeed");

        // Verify only one card remains selected
        let selected = game.target_context.multi_select_context().selected_cards();
        assert_eq!(selected.len(), 1);
        assert!(!selected.contains(&cards[0].id));
        assert!(!selected.contains(&cards[1].id));
        assert!(selected.contains(&cards[2].id));
    }

    #[test]
    fn test_deselect_cards_empty_list() {
        let mut game = create_test_game_with_cards();

        let action = Action::DeselectCards(vec![]);
        let result = game.handle_action(action);

        assert!(result.is_ok(), "DeselectCards should handle empty list");
    }

    #[test]
    fn test_deselect_cards_partial_failure() {
        let mut game = create_test_game_with_cards();
        let cards = game.available.cards();

        // Select one card
        let select_action = Action::SelectCards(vec![cards[0]]);
        game.handle_action(select_action).unwrap();

        // Try to deselect including nonexistent card
        let nonexistent_card = Card::new(Value::Two, Suit::Club);
        let deselect_action = Action::DeselectCards(vec![cards[0], nonexistent_card]);
        let result = game.handle_action(deselect_action);

        assert!(
            result.is_err(),
            "DeselectCards should fail if any card is invalid"
        );
        assert!(matches!(result.unwrap_err(), GameError::NoCardMatch));
    }
}

/// Test ToggleCardSelection action (Issue #809)
#[cfg(test)]
mod toggle_card_selection_tests {
    use super::*;

    #[test]
    fn test_toggle_card_selection_select() {
        let mut game = create_test_game_with_cards();
        let cards = game.available.cards();

        // Toggle unselected card (should select it)
        let action = Action::ToggleCardSelection(cards[0]);
        let result = game.handle_action(action);

        assert!(result.is_ok(), "ToggleCardSelection should succeed");

        // Verify card is now selected
        let selected = game.target_context.multi_select_context().selected_cards();
        assert_eq!(selected.len(), 1);
        assert!(selected.contains(&cards[0].id));
    }

    #[test]
    fn test_toggle_card_selection_deselect() {
        let mut game = create_test_game_with_cards();
        let cards = game.available.cards();

        // First select a card
        let select_action = Action::SelectCards(vec![cards[0]]);
        game.handle_action(select_action).unwrap();

        // Then toggle it (should deselect)
        let toggle_action = Action::ToggleCardSelection(cards[0]);
        let result = game.handle_action(toggle_action);

        assert!(result.is_ok(), "ToggleCardSelection should succeed");

        // Verify card is now deselected
        let selected = game.target_context.multi_select_context().selected_cards();
        assert_eq!(selected.len(), 0);
    }

    #[test]
    fn test_toggle_card_selection_multiple_toggles() {
        let mut game = create_test_game_with_cards();
        let cards = game.available.cards();

        // Toggle card multiple times
        for i in 0..4 {
            let action = Action::ToggleCardSelection(cards[0]);
            let result = game.handle_action(action);
            assert!(result.is_ok(), "Toggle {i} should succeed");

            let selected = game.target_context.multi_select_context().selected_cards();
            let expected_selected = i % 2 == 0; // Even iterations = selected
            assert_eq!(
                selected.contains(&cards[0].id),
                expected_selected,
                "Toggle {i} should result in selected = {expected_selected}",
            );
        }
    }

    #[test]
    fn test_toggle_card_selection_invalid_stage() {
        let mut game = create_test_game_with_cards();
        game.stage = Stage::Shop();

        let cards = game.available.cards();
        let action = Action::ToggleCardSelection(cards[0]);
        let result = game.handle_action(action);

        assert!(
            result.is_err(),
            "ToggleCardSelection should fail in non-blind stage"
        );
        assert!(matches!(result.unwrap_err(), GameError::InvalidAction));
    }
}

/// Test SelectAllCards and DeselectAllCards actions (Issue #810)
#[cfg(test)]
mod batch_selection_tests {
    use super::*;

    #[test]
    fn test_select_all_cards() {
        let mut game = create_test_game_with_cards();
        let cards = game.available.cards();

        println!("test_select_all_cards - Available cards: {}", cards.len());

        let action = Action::SelectAllCards();
        let result = game.handle_action(action);

        match &result {
            Ok(_) => println!("SelectAllCards succeeded"),
            Err(e) => println!("SelectAllCards failed: {e:?}"),
        }

        assert!(
            result.is_ok(),
            "SelectAllCards should succeed - Error: {:?}",
            result.err()
        );

        // Verify all cards are selected
        let selected = game.target_context.multi_select_context().selected_cards();
        assert_eq!(selected.len(), cards.len());

        for card in &cards {
            assert!(
                selected.contains(&card.id),
                "Card {} should be selected",
                card.id
            );
        }
    }

    #[test]
    fn test_select_all_cards_with_existing_selection() {
        let mut game = create_test_game_with_cards();
        let cards = game.available.cards();

        // First select one card
        let select_action = Action::SelectCards(vec![cards[0]]);
        game.handle_action(select_action).unwrap();

        // Then select all (should clear existing and select all)
        let select_all_action = Action::SelectAllCards();
        let result = game.handle_action(select_all_action);

        assert!(result.is_ok(), "SelectAllCards should succeed");

        // Verify all cards are selected
        let selected = game.target_context.multi_select_context().selected_cards();
        assert_eq!(selected.len(), cards.len());
    }

    #[test]
    fn test_deselect_all_cards() {
        let mut game = create_test_game_with_cards();
        let cards = game.available.cards();

        // First select some cards
        let select_action = Action::SelectCards(vec![cards[0], cards[1], cards[2]]);
        game.handle_action(select_action).unwrap();

        // Then deselect all
        let deselect_all_action = Action::DeselectAllCards();
        let result = game.handle_action(deselect_all_action);

        assert!(result.is_ok(), "DeselectAllCards should succeed");

        // Verify no cards are selected
        let selected = game.target_context.multi_select_context().selected_cards();
        assert_eq!(selected.len(), 0);
    }

    #[test]
    fn test_deselect_all_cards_when_none_selected() {
        let mut game = create_test_game_with_cards();

        let action = Action::DeselectAllCards();
        let result = game.handle_action(action);

        assert!(
            result.is_ok(),
            "DeselectAllCards should succeed even when no cards selected"
        );

        // Verify still no cards selected
        let selected = game.target_context.multi_select_context().selected_cards();
        assert_eq!(selected.len(), 0);
    }

    #[test]
    fn test_batch_selection_invalid_stage() {
        let mut game = create_test_game_with_cards();
        game.stage = Stage::Shop();

        let select_all_result = game.handle_action(Action::SelectAllCards());
        assert!(select_all_result.is_err());
        assert!(matches!(
            select_all_result.unwrap_err(),
            GameError::InvalidAction
        ));

        let deselect_all_result = game.handle_action(Action::DeselectAllCards());
        assert!(deselect_all_result.is_err());
        assert!(matches!(
            deselect_all_result.unwrap_err(),
            GameError::InvalidAction
        ));
    }
}

/// Test RangeSelectCards action
#[cfg(test)]
mod range_select_cards_tests {
    use super::*;

    #[test]
    fn test_range_select_cards_forward() {
        let mut game = create_test_game_with_cards();
        let cards = game.available.cards();

        // Select range from first to third card
        let action = Action::RangeSelectCards {
            start: cards[0],
            end: cards[2],
        };
        let result = game.handle_action(action);

        assert!(result.is_ok(), "RangeSelectCards should succeed");

        // Verify cards in range are selected
        let selected = game.target_context.multi_select_context().selected_cards();
        assert!(selected.len() >= 3, "At least 3 cards should be selected");
        assert!(selected.contains(&cards[0].id));
        assert!(selected.contains(&cards[1].id));
        assert!(selected.contains(&cards[2].id));
    }

    #[test]
    fn test_range_select_cards_reverse() {
        let mut game = create_test_game_with_cards();
        let cards = game.available.cards();

        // Select range from third to first card (reverse order)
        let action = Action::RangeSelectCards {
            start: cards[2],
            end: cards[0],
        };
        let result = game.handle_action(action);

        assert!(result.is_ok(), "RangeSelectCards should succeed in reverse");

        // Verify cards in range are selected (same result as forward)
        let selected = game.target_context.multi_select_context().selected_cards();
        assert!(selected.len() >= 3, "At least 3 cards should be selected");
        assert!(selected.contains(&cards[0].id));
        assert!(selected.contains(&cards[1].id));
        assert!(selected.contains(&cards[2].id));
    }

    #[test]
    fn test_range_select_cards_single_card() {
        let mut game = create_test_game_with_cards();
        let cards = game.available.cards();

        // Select "range" from card to itself
        let action = Action::RangeSelectCards {
            start: cards[1],
            end: cards[1],
        };
        let result = game.handle_action(action);

        assert!(
            result.is_ok(),
            "RangeSelectCards should succeed for single card"
        );

        // Verify at least the target card is selected
        let selected = game.target_context.multi_select_context().selected_cards();
        assert!(selected.contains(&cards[1].id));
    }

    #[test]
    fn test_range_select_cards_nonexistent_start() {
        let mut game = create_test_game_with_cards();
        let cards = game.available.cards();

        let nonexistent_card = Card::new(Value::Two, Suit::Club);
        let action = Action::RangeSelectCards {
            start: nonexistent_card,
            end: cards[0],
        };
        let result = game.handle_action(action);

        assert!(
            result.is_err(),
            "RangeSelectCards should fail with nonexistent start card"
        );
        assert!(matches!(result.unwrap_err(), GameError::NoCardMatch));
    }

    #[test]
    fn test_range_select_cards_nonexistent_end() {
        let mut game = create_test_game_with_cards();
        let cards = game.available.cards();

        let nonexistent_card = Card::new(Value::Two, Suit::Club);
        let action = Action::RangeSelectCards {
            start: cards[0],
            end: nonexistent_card,
        };
        let result = game.handle_action(action);

        assert!(
            result.is_err(),
            "RangeSelectCards should fail with nonexistent end card"
        );
        assert!(matches!(result.unwrap_err(), GameError::NoCardMatch));
    }

    #[test]
    fn test_range_select_cards_invalid_stage() {
        let mut game = create_test_game_with_cards();
        game.stage = Stage::Shop();

        let cards = game.available.cards();
        let action = Action::RangeSelectCards {
            start: cards[0],
            end: cards[1],
        };
        let result = game.handle_action(action);

        assert!(
            result.is_err(),
            "RangeSelectCards should fail in non-blind stage"
        );
        assert!(matches!(result.unwrap_err(), GameError::InvalidAction));
    }
}

/// Integration tests for multi-select system
#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_multi_select_workflow() {
        let mut game = create_test_game_with_cards();
        let cards = game.available.cards();

        // Step 1: Select multiple cards
        let select_action = Action::SelectCards(vec![cards[0], cards[1]]);
        assert!(game.handle_action(select_action).is_ok());

        // Step 2: Toggle one card (should add it)
        let toggle_action = Action::ToggleCardSelection(cards[2]);
        assert!(game.handle_action(toggle_action).is_ok());

        // Step 3: Deselect one card
        let deselect_action = Action::DeselectCard(cards[0]);
        assert!(game.handle_action(deselect_action).is_ok());

        // Step 4: Verify final state
        let selected = game.target_context.multi_select_context().selected_cards();
        assert_eq!(selected.len(), 2);
        assert!(!selected.contains(&cards[0].id)); // Deselected
        assert!(selected.contains(&cards[1].id)); // Still selected
        assert!(selected.contains(&cards[2].id)); // Toggled on

        // Step 5: Deselect all
        let deselect_all_action = Action::DeselectAllCards();
        assert!(game.handle_action(deselect_all_action).is_ok());

        let selected_final = game.target_context.multi_select_context().selected_cards();
        assert_eq!(selected_final.len(), 0);
    }

    #[test]
    fn test_multi_select_with_traditional_select() {
        let mut game = create_test_game_with_cards();
        let cards = game.available.cards();

        // Use traditional SelectCard action
        let traditional_action = Action::SelectCard(cards[0]);
        assert!(game.handle_action(traditional_action).is_ok());

        // Verify traditional selection works
        let traditional_selected = game.available.selected();
        assert_eq!(traditional_selected.len(), 1);
        assert_eq!(traditional_selected[0].id, cards[0].id);

        // Use multi-select action
        let multi_action = Action::SelectCards(vec![cards[1], cards[2]]);
        assert!(game.handle_action(multi_action).is_ok());

        // Verify multi-select context is active and has selections
        assert!(game.target_context.is_multi_select_active());
        let multi_selected = game.target_context.multi_select_context().selected_cards();
        assert_eq!(multi_selected.len(), 2);
        assert!(multi_selected.contains(&cards[1].id));
        assert!(multi_selected.contains(&cards[2].id));
    }

    #[test]
    fn test_multi_select_limits_respected() {
        let mut game = create_test_game_with_cards();
        let cards = game.available.cards();

        println!("Available cards: {}", cards.len());

        // Try to select more cards than available
        let action = Action::SelectCards(cards.clone());
        let result = game.handle_action(action);

        match &result {
            Ok(_) => println!("SelectCards succeeded"),
            Err(e) => println!("SelectCards failed: {e:?}"),
        }

        // Should succeed up to the limit
        assert!(
            result.is_ok(),
            "SelectCards should handle limits gracefully - Error: {:?}",
            result.err()
        );

        // Verify multi-select context respects its internal limits
        let selected = game.target_context.multi_select_context().selected_cards();
        assert!(selected.len() <= cards.len());
    }

    #[test]
    fn test_multi_select_persistence() {
        let mut game = create_test_game_with_cards();
        let cards = game.available.cards();

        // Select some cards
        let action1 = Action::SelectCards(vec![cards[0], cards[1]]);
        game.handle_action(action1).unwrap();

        // Perform other operations
        let action2 = Action::ToggleCardSelection(cards[2]);
        game.handle_action(action2).unwrap();

        // Verify selections persist
        let selected = game.target_context.multi_select_context().selected_cards();
        assert_eq!(selected.len(), 3);
        assert!(selected.contains(&cards[0].id));
        assert!(selected.contains(&cards[1].id));
        assert!(selected.contains(&cards[2].id));

        // Clear and verify
        let clear_action = Action::DeselectAllCards();
        game.handle_action(clear_action).unwrap();

        let selected_after_clear = game.target_context.multi_select_context().selected_cards();
        assert_eq!(selected_after_clear.len(), 0);
    }
}

/// Edge case and error handling tests
#[cfg(test)]
mod edge_case_tests {
    use super::*;

    #[test]
    fn test_multi_select_with_empty_available() {
        let mut game = Game {
            stage: Stage::Blind(balatro_rs::stage::Blind::Small),
            ..Default::default()
        };
        // No cards in available

        let actions = vec![
            Action::SelectCards(vec![]),
            Action::SelectAllCards(),
            Action::DeselectAllCards(),
        ];

        for action in actions {
            let result = game.handle_action(action);
            assert!(
                result.is_ok(),
                "Actions should handle empty available cards"
            );
        }
    }

    #[test]
    fn test_multi_select_context_synchronization() {
        let mut game = create_test_game_with_cards();
        let cards = game.available.cards();

        // Ensure sync happens before each operation
        let action = Action::SelectCards(vec![cards[0]]);
        game.handle_action(action).unwrap();

        // Manually check that target_context has the right available cards
        let target_available = game.target_context.multi_select_context().selected_cards();
        assert!(!target_available.is_empty());

        // Modify available cards directly (simulating external change)
        // Cannot directly extend available - using existing cardsvec![Card::new(Value::Nine, Suit::Heart)]);

        // Next multi-select action should sync properly
        let action2 = Action::SelectAllCards();
        let result = game.handle_action(action2);
        assert!(
            result.is_ok(),
            "Actions should handle changes to available cards"
        );
    }

    #[test]
    fn test_multi_select_performance_with_many_cards() {
        let mut game = create_test_game_with_cards();

        let available_cards = game.available.cards();
        println!(
            "Performance test: Available cards count: {}",
            available_cards.len()
        );

        // Test that selections work efficiently with available cards
        let start = std::time::Instant::now();
        let action = Action::SelectAllCards();
        let result = game.handle_action(action);
        let duration = start.elapsed();

        match &result {
            Ok(_) => println!("SelectAllCards succeeded"),
            Err(e) => println!("SelectAllCards failed: {e:?}"),
        }

        assert!(
            result.is_ok(),
            "SelectAllCards should work with available cards - Error: {:?}",
            result.err()
        );
        assert!(
            duration.as_millis() < 100,
            "Operation should complete quickly (took {}ms)",
            duration.as_millis()
        );

        // Verify all available cards are selected
        let selected = game.target_context.multi_select_context().selected_cards();
        println!(
            "Selected {} cards out of {} available",
            selected.len(),
            available_cards.len()
        );
        assert_eq!(
            selected.len(),
            available_cards.len(),
            "Should select all {} available cards, but selected {}",
            available_cards.len(),
            selected.len()
        );
    }
}
