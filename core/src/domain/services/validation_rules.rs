//! # Validation Rules
//!
//! This module contains concrete implementations of validation rules
//! for the ActionValidator service.

use crate::action::Action;
use crate::domain::services::action_validator::{ValidationContext, ValidationRule};
use crate::domain::value_objects::ValidationResult;
use crate::game::Game;
#[cfg(test)]
use crate::joker::JokerId;
use crate::stage::Stage;

/// Rule to validate actions are appropriate for the current game stage
pub struct StageValidationRule;

impl ValidationRule for StageValidationRule {
    fn name(&self) -> &str {
        "StageValidation"
    }

    fn validate(
        &self,
        action: &Action,
        game: &Game,
        _context: &ValidationContext,
    ) -> ValidationResult {
        let current_stage = game.stage;

        // Check if action is valid for current stage
        match (current_stage, action) {
            (Stage::PreBlind(), Action::Play()) => {
                ValidationResult::invalid("Cannot play cards during PreBlind stage")
            }
            (Stage::Shop(), Action::Play()) => {
                ValidationResult::invalid("Cannot play cards during Shop stage")
            }
            (Stage::End(_), _) => ValidationResult::invalid("Game has ended, no actions allowed"),
            _ => ValidationResult::valid(),
        }
    }

    fn priority(&self) -> u32 {
        100 // High priority - check stage first
    }
}

/// Rule to validate that required resources are available
pub struct ResourceValidationRule;

impl ValidationRule for ResourceValidationRule {
    fn name(&self) -> &str {
        "ResourceValidation"
    }

    fn validate(
        &self,
        action: &Action,
        game: &Game,
        _context: &ValidationContext,
    ) -> ValidationResult {
        match action {
            Action::BuyJoker {
                joker_id: _,
                slot: _,
            } => {
                // Check if player has enough money (simplified check)
                if game.money < 0.0 {
                    return ValidationResult::invalid("Insufficient funds to buy joker");
                }
            }
            Action::SellJokers(jokers) => {
                // Check if the jokers exist to sell
                if jokers.is_empty() {
                    return ValidationResult::invalid("No jokers selected for selling");
                }
            }
            _ => {}
        }

        ValidationResult::valid()
    }

    fn applies_to(&self, action: &Action) -> bool {
        matches!(action, Action::BuyJoker { .. } | Action::SellJokers(_))
    }
}

/// Rule to validate action history constraints
pub struct ActionHistoryRule {
    _max_consecutive_skips: usize,
}

impl ActionHistoryRule {
    pub fn new(max_consecutive_skips: usize) -> Self {
        Self {
            _max_consecutive_skips: max_consecutive_skips,
        }
    }
}

impl ValidationRule for ActionHistoryRule {
    fn name(&self) -> &str {
        "ActionHistory"
    }

    fn validate(
        &self,
        action: &Action,
        _game: &Game,
        context: &ValidationContext,
    ) -> ValidationResult {
        // Simplified validation without action history tracking
        // In a real implementation, we would track action history separately
        if matches!(action, Action::NextRound()) {
            // For now, always allow NextRound actions
            // Real implementation would check consecutive patterns
        }

        // Check total action count limit
        if context.strict_mode && context.max_action_history > 0 {
            // Simplified check - real implementation would track actual history
            // For now, we just validate the limit exists
        }

        ValidationResult::valid()
    }
}

/// Composite rule that chains multiple rules
pub struct CompositeRule {
    rules: Vec<Box<dyn ValidationRule>>,
}

impl CompositeRule {
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    pub fn add_rule(mut self, rule: Box<dyn ValidationRule>) -> Self {
        self.rules.push(rule);
        self
    }
}

impl Default for CompositeRule {
    fn default() -> Self {
        Self::new()
    }
}

impl ValidationRule for CompositeRule {
    fn name(&self) -> &str {
        "Composite"
    }

    fn validate(
        &self,
        action: &Action,
        game: &Game,
        context: &ValidationContext,
    ) -> ValidationResult {
        for rule in &self.rules {
            let result = rule.validate(action, game, context);
            if result.is_invalid() {
                return result;
            }
        }
        ValidationResult::valid()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stage_validation_rule() {
        let rule = StageValidationRule;
        let context = ValidationContext::default();

        // Test PreBlind stage
        let mut game = Game::default();
        game.start(); // Sets to PreBlind

        let play_action = Action::Play();
        let result = rule.validate(&play_action, &game, &context);
        assert!(result.is_invalid());
        assert!(result.error().unwrap().reason().contains("PreBlind"));

        // Test valid action for PreBlind
        let next_action = Action::NextRound();
        let result = rule.validate(&next_action, &game, &context);
        assert!(result.is_valid());
    }

    #[test]
    fn test_resource_validation_rule() {
        let rule = ResourceValidationRule;
        let context = ValidationContext::default();

        let game = Game::default();

        // Test buying without money
        let buy_action = Action::BuyJoker {
            joker_id: JokerId::CreditCard,
            slot: 0,
        };
        let result = rule.validate(&buy_action, &game, &context);
        // This will pass because shop is empty in default game
        assert!(result.is_valid());

        // Test selling empty list
        let sell_action = Action::SellJokers(vec![]);
        let result = rule.validate(&sell_action, &game, &context);
        assert!(result.is_invalid());
    }

    #[test]
    fn test_action_history_rule() {
        let rule = ActionHistoryRule::new(3);
        let context = ValidationContext::default();

        let game = Game::default();

        // First action should be valid
        let next_action = Action::NextRound();
        let result = rule.validate(&next_action, &game, &context);
        assert!(result.is_valid());
    }

    #[test]
    fn test_composite_rule() {
        let composite = CompositeRule::new()
            .add_rule(Box::new(StageValidationRule))
            .add_rule(Box::new(ResourceValidationRule));

        let context = ValidationContext::default();
        let mut game = Game::default();
        game.start();

        // Test action that passes both rules
        let next_action = Action::NextRound();
        let result = composite.validate(&next_action, &game, &context);
        assert!(result.is_valid());

        // Test action that fails stage validation
        let play_action = Action::Play();
        let result = composite.validate(&play_action, &game, &context);
        assert!(result.is_invalid());
    }
}
