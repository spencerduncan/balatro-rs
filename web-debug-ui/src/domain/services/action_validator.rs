//! ActionValidator Service
//!
//! The ActionValidator service encapsulates the business logic for validating
//! game actions against the current game state. It serves as a domain service
//! that coordinates validation rules and provides a clean interface for
//! action validation throughout the domain layer.

use crate::domain::{
    value_objects::{ValidationResult},
    Action,
};
use crate::domain::stubs::{Stage, Game, Blind, JokerId};

/// Service interface for validating game actions
///
/// ActionValidator defines the contract for validating actions against game state.
/// This enables different validation strategies and makes the domain layer
/// testable with mock implementations.
///
/// # Design Principles
///
/// - **Single Responsibility**: Only handles action validation
/// - **Strategy Pattern**: Different validators can implement different rules
/// - **Dependency Inversion**: Domain depends on this abstraction
///
/// # Examples
///
/// ```ignore
/// use balatro_domain::{ActionValidator, BalatroActionValidator};
/// use balatro_rs::Game;
///
/// let validator = BalatroActionValidator::new();
/// let game = Game::new(Default::default());
/// let action = Action::Play();
///
/// let result = validator.validate(&action, &game);
/// if result.is_valid() {
///     // Apply the action
/// }
/// ```
pub trait ActionValidator: Send + Sync {
    /// Validate an action against the current game state
    ///
    /// # Arguments
    ///
    /// * `action` - The action to validate
    /// * `state` - The current game state
    ///
    /// # Returns
    ///
    /// * `ValidationResult::Valid` - Action is valid and can be applied
    /// * `ValidationResult::Invalid` - Action cannot be applied with details
    fn validate(&self, action: &Action, state: &Game) -> ValidationResult;

    /// Get all actions that are currently valid for the given state
    ///
    /// # Arguments
    ///
    /// * `state` - The current game state
    ///
    /// # Returns
    ///
    /// * `Vec<Action>` - List of all valid actions for the current state
    fn get_available_actions(&self, state: &Game) -> Vec<Action>;

    /// Check if a specific action type is generally available in the current stage
    ///
    /// # Arguments
    ///
    /// * `action` - The action to check
    /// * `stage` - The current game stage
    ///
    /// # Returns
    ///
    /// * `true` - Action type is available in this stage
    /// * `false` - Action type is not available in this stage
    fn is_action_available_in_stage(&self, action: &Action, stage: &Stage) -> bool;
}

/// Concrete implementation of ActionValidator for Balatro game rules
///
/// BalatroActionValidator implements the specific business rules for validating
/// actions in the Balatro card game. It encapsulates knowledge of game stages,
/// card selection rules, and other domain-specific validation logic.
#[derive(Debug, Clone, Default)]
pub struct BalatroActionValidator {
    /// Whether to allow debug actions (for testing/development)
    allow_debug_actions: bool,
}

impl BalatroActionValidator {
    /// Create a new validator with default settings
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new validator with debug actions enabled
    pub fn with_debug_actions() -> Self {
        Self {
            allow_debug_actions: true,
        }
    }

    /// Enable or disable debug actions
    pub fn set_debug_actions(&mut self, enabled: bool) {
        self.allow_debug_actions = enabled;
    }

    // Private validation methods for different action types

    fn validate_play_action(&self, state: &Game) -> ValidationResult {
        // Must be in appropriate stage
        if !matches!(state.stage, Stage::Blind | Stage::PreBlind) {
            return ValidationResult::invalid("Cannot play cards outside of blind stages");
        }

        // Must have cards selected
        if state.available.selected.is_empty() {
            return ValidationResult::invalid("No cards selected for play");
        }

        // Must have plays remaining
        if state.plays <= 0.0 {
            return ValidationResult::invalid("No plays remaining");
        }

        ValidationResult::valid()
    }

    fn validate_discard_action(&self, state: &Game) -> ValidationResult {
        // Must be in appropriate stage
        if !matches!(state.stage, Stage::Blind | Stage::PreBlind) {
            return ValidationResult::invalid("Cannot discard cards outside of blind stages");
        }

        // Must have cards selected
        if state.available.selected.is_empty() {
            return ValidationResult::invalid("No cards selected for discard");
        }

        // Must have discards remaining
        if state.discards <= 0.0 {
            return ValidationResult::invalid("No discards remaining");
        }

        ValidationResult::valid()
    }

    fn validate_card_selection(&self, action: &Action, state: &Game) -> ValidationResult {
        match action {
            Action::SelectCard(card) => {
                // Card must be available for selection
                if !state.available.cards.contains(card) {
                    return ValidationResult::invalid("Card is not available for selection");
                }

                // Check if card is already selected
                if state.available.selected.contains(card) {
                    return ValidationResult::invalid("Card is already selected");
                }

                ValidationResult::valid()
            }
            Action::DeselectCard(card) => {
                // Card must be currently selected
                if !state.available.selected.contains(card) {
                    return ValidationResult::invalid("Card is not currently selected");
                }

                ValidationResult::valid()
            }
            _ => ValidationResult::valid(),
        }
    }

    fn validate_shop_action(&self, action: &Action, state: &Game) -> ValidationResult {
        // Shop actions are only valid in Shop stage
        if !matches!(state.stage, Stage::Shop) {
            return ValidationResult::invalid("Shop actions are only available in Shop stage");
        }

        match action {
            Action::RerollShop() => {
                if state.money < state.shop_reroll_cost {
                    return ValidationResult::invalid("Not enough money to reroll shop");
                }
                ValidationResult::valid()
            }
            Action::BuyJoker { slot, .. } => {
                if *slot >= state.shop.jokers.capacity() {
                    return ValidationResult::invalid("Invalid joker slot");
                }
                // Additional validation would check joker availability and cost
                ValidationResult::valid()
            }
            _ => ValidationResult::valid(),
        }
    }

    fn validate_round_progression(&self, action: &Action, state: &Game) -> ValidationResult {
        match action {
            Action::NextRound() => {
                if !matches!(state.stage, Stage::PostBlind) {
                    return ValidationResult::invalid("Can only advance round from PostBlind stage");
                }
                ValidationResult::valid()
            }
            Action::SelectBlind(_) => {
                if !matches!(state.stage, Stage::PreBlind) {
                    return ValidationResult::invalid("Can only select blind in PreBlind stage");
                }
                ValidationResult::valid()
            }
            _ => ValidationResult::valid(),
        }
    }
}

impl ActionValidator for BalatroActionValidator {
    fn validate(&self, action: &Action, state: &Game) -> ValidationResult {
        // First check if the action is available in the current stage
        if !self.is_action_available_in_stage(action, &state.stage) {
            return ValidationResult::invalid(format!(
                "Action {:?} is not available in stage {:?}",
                action, state.stage
            ));
        }

        // Then perform specific validation based on action type
        match action {
            Action::Play() => self.validate_play_action(state),
            Action::Discard() => self.validate_discard_action(state),
            Action::SelectCard(_) | Action::DeselectCard(_) => {
                self.validate_card_selection(action, state)
            }
            Action::RerollShop() | Action::BuyJoker { .. } => {
                self.validate_shop_action(action, state)
            }
            Action::NextRound() | Action::SelectBlind(_) => {
                self.validate_round_progression(action, state)
            }
            _ => {
                // For actions not specifically handled, perform basic availability check
                ValidationResult::valid()
            }
        }
    }

    fn get_available_actions(&self, state: &Game) -> Vec<Action> {
        let mut actions = Vec::new();

        match state.stage {
            Stage::PreBlind => {
                // Can select blind
                for blind in [
                    Blind::Small,
                    Blind::Big,
                    Blind::Boss,
                ] {
                    actions.push(Action::SelectBlind(blind));
                }

                // Can select/deselect cards
                for card in &state.available.cards {
                    if !state.available.selected.contains(card) {
                        actions.push(Action::SelectCard(*card));
                    }
                }

                for card in &state.available.selected {
                    actions.push(Action::DeselectCard(*card));
                }
            }
            Stage::Blind => {
                // Can play or discard if cards are selected
                if !state.available.selected.is_empty() {
                    if state.plays > 0.0 {
                        actions.push(Action::Play());
                    }
                    if state.discards > 0.0 {
                        actions.push(Action::Discard());
                    }
                }

                // Can select/deselect cards
                for card in &state.available.cards {
                    if !state.available.selected.contains(card) {
                        actions.push(Action::SelectCard(*card));
                    }
                }

                for card in &state.available.selected {
                    actions.push(Action::DeselectCard(*card));
                }
            }
            Stage::PostBlind => {
                // Can advance to next round
                actions.push(Action::NextRound());
            }
            Stage::Shop => {
                // Can reroll shop if player has money
                if state.money >= state.shop_reroll_cost {
                    actions.push(Action::RerollShop());
                }

                // Can buy jokers (simplified check)
                for slot in 0..state.shop.jokers.capacity() {
                    if let Some(_joker) = state.shop.jokers.get(slot) {
                        // In a full implementation, we'd check joker cost vs money
                        actions.push(Action::BuyJoker {
                            joker_id: JokerId::Joker, // Placeholder
                            slot,
                        });
                    }
                }

                // Eventually move to next round from shop
                actions.push(Action::NextRound());
            }
            Stage::End => {
                // No actions available when game is ended
            }
        }

        actions
    }

    fn is_action_available_in_stage(&self, action: &Action, stage: &Stage) -> bool {
        match (action, stage) {
            // Card selection is available in pre-blind and blind stages
            (Action::SelectCard(_) | Action::DeselectCard(_), Stage::PreBlind | Stage::Blind) => {
                true
            }

            // Play and discard are only available during blind stage
            (Action::Play() | Action::Discard(), Stage::Blind) => true,

            // Blind selection is only available in pre-blind stage
            (Action::SelectBlind(_), Stage::PreBlind) => true,

            // Shop actions are only available in shop stage
            (
                Action::RerollShop()
                | Action::BuyJoker { .. }
                | Action::BuyConsumable { .. }
                | Action::BuyVoucher { .. }
                | Action::BuyPack { .. },
                Stage::Shop,
            ) => true,

            // Round progression is available from post-blind and shop stages
            (Action::NextRound(), Stage::PostBlind | Stage::Shop) => true,

            // No actions are available when game is ended
            (_, Stage::End) => false,

            // Default to false for unmatched combinations
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::stubs::{Card, Config, Rank, Suit, Game, Stage};

    fn create_test_game() -> Game {
        Game::new(Config::default())
    }

    fn create_test_game_in_blind_stage() -> Game {
        let mut game = Game::new(Config::default());
        game.stage = Stage::Blind;
        game
    }

    #[test]
    fn balatro_validator_creation() {
        let validator = BalatroActionValidator::new();
        assert!(!validator.allow_debug_actions);

        let debug_validator = BalatroActionValidator::with_debug_actions();
        assert!(debug_validator.allow_debug_actions);
    }

    #[test]
    fn validate_play_action_with_no_cards_selected() {
        let validator = BalatroActionValidator::new();
        let game = create_test_game_in_blind_stage();
        let action = Action::Play();

        let result = validator.validate(&action, &game);
        assert!(result.is_invalid());

        if let ValidationResult::Invalid(error) = result {
            assert!(error.reason().contains("No cards selected for play"));
        }
    }

    #[test]
    fn validate_discard_action_with_no_cards_selected() {
        let validator = BalatroActionValidator::new();
        let game = create_test_game_in_blind_stage();
        let action = Action::Discard();

        let result = validator.validate(&action, &game);
        assert!(result.is_invalid());

        if let ValidationResult::Invalid(error) = result {
            assert!(error.reason().contains("No cards selected for discard"));
        }
    }

    #[test]
    fn validate_card_selection_with_available_card() {
        let validator = BalatroActionValidator::new();
        let game = create_test_game();

        if let Some(card) = game.available.cards.first() {
            let action = Action::SelectCard(*card);
            let result = validator.validate(&action, &game);
            assert!(result.is_valid());
        }
    }

    #[test]
    fn validate_card_deselection_with_unselected_card() {
        let validator = BalatroActionValidator::new();
        let game = create_test_game();

        if let Some(card) = game.available.cards.first() {
            let action = Action::DeselectCard(*card);
            let result = validator.validate(&action, &game);
            assert!(result.is_invalid());

            if let ValidationResult::Invalid(error) = result {
                assert!(error.reason().contains("not currently selected"));
            }
        }
    }

    #[test]
    fn shop_actions_only_valid_in_shop_stage() {
        let validator = BalatroActionValidator::new();
        let mut game = create_test_game();

        // Set stage to non-shop stage
        game.stage = Stage::Blind;

        let reroll_action = Action::RerollShop();
        let result = validator.validate(&reroll_action, &game);
        assert!(result.is_invalid());

        if let ValidationResult::Invalid(error) = result {
            assert!(error.reason().contains("not available in stage"));
        }
    }

    #[test]
    fn next_round_only_valid_in_post_blind_stage() {
        let validator = BalatroActionValidator::new();
        let mut game = create_test_game();

        // Set stage to non-post-blind stage
        game.stage = Stage::Blind;

        let action = Action::NextRound();
        let result = validator.validate(&action, &game);
        assert!(result.is_invalid());
    }

    #[test]
    fn get_available_actions_in_preblind_stage() {
        let validator = BalatroActionValidator::new();
        let mut game = create_test_game();
        game.stage = Stage::PreBlind;

        let actions = validator.get_available_actions(&game);

        // Should include blind selection actions
        assert!(actions
            .iter()
            .any(|a| matches!(a, Action::SelectBlind(_))));

        // Should include card selection actions
        assert!(actions.iter().any(|a| matches!(a, Action::SelectCard(_))));
    }

    #[test]
    fn get_available_actions_in_shop_stage() {
        let validator = BalatroActionValidator::new();
        let mut game = create_test_game();
        game.stage = Stage::Shop;
        game.money = 100.0; // Ensure enough money for shop actions

        let actions = validator.get_available_actions(&game);

        // Should include shop reroll if player has money
        assert!(actions.iter().any(|a| matches!(a, Action::RerollShop())));

        // Should eventually include next round
        assert!(actions.iter().any(|a| matches!(a, Action::NextRound())));
    }

    #[test]
    fn get_available_actions_in_end_stage() {
        let validator = BalatroActionValidator::new();
        let mut game = create_test_game();
        game.stage = Stage::End;

        let actions = validator.get_available_actions(&game);

        // Should have no available actions
        assert!(actions.is_empty());
    }

    #[test]
    fn is_action_available_in_stage_logic() {
        let validator = BalatroActionValidator::new();
        let card = Card {
            rank: Rank::Ace,
            suit: Suit::Spades,
        };

        // Card selection available in PreBlind and Blind
        assert!(validator.is_action_available_in_stage(&Action::SelectCard(card), &Stage::PreBlind));
        assert!(validator.is_action_available_in_stage(&Action::SelectCard(card), &Stage::Blind));
        assert!(!validator.is_action_available_in_stage(&Action::SelectCard(card), &Stage::Shop));

        // Play only available in Blind
        assert!(!validator.is_action_available_in_stage(&Action::Play(), &Stage::PreBlind));
        assert!(validator.is_action_available_in_stage(&Action::Play(), &Stage::Blind));
        assert!(!validator.is_action_available_in_stage(&Action::Play(), &Stage::Shop));

        // Shop actions only in Shop stage
        assert!(!validator.is_action_available_in_stage(&Action::RerollShop(), &Stage::Blind));
        assert!(validator.is_action_available_in_stage(&Action::RerollShop(), &Stage::Shop));

        // Nothing available in End stage
        assert!(!validator.is_action_available_in_stage(&Action::Play(), &Stage::End));
        assert!(!validator.is_action_available_in_stage(&Action::RerollShop(), &Stage::End));
    }

    #[test]
    fn trait_can_be_implemented() {
        let _validator = BalatroActionValidator::new();
        // This test ensures the trait can be implemented
    }

    #[test]
    fn validator_trait_is_object_safe() {
        let validator = BalatroActionValidator::new();
        let _trait_object: &dyn ActionValidator = &validator;
        // This test ensures the trait is object-safe (can be used as trait object)
    }

    #[test]
    fn validator_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<BalatroActionValidator>();
        // This test ensures implementations can be used across threads
    }

    #[test]
    fn validator_debug_actions_setting() {
        let mut validator = BalatroActionValidator::new();
        assert!(!validator.allow_debug_actions);

        validator.set_debug_actions(true);
        assert!(validator.allow_debug_actions);

        validator.set_debug_actions(false);
        assert!(!validator.allow_debug_actions);
    }

    #[test]
    fn validator_cloning_preserves_settings() {
        let mut validator = BalatroActionValidator::new();
        validator.set_debug_actions(true);

        let cloned = validator.clone();
        assert_eq!(validator.allow_debug_actions, cloned.allow_debug_actions);
    }
}