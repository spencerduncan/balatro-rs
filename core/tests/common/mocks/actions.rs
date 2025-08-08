//! Action Mocks for Scripted Testing Sequences
//!
//! Provides tools for recording, replaying, and validating action sequences,
//! enabling deterministic test scenarios and regression testing.

use balatro_rs::{action::Action, game::Game, stage::Stage};
use std::collections::VecDeque;
use std::fmt;

/// Records and replays action sequences for testing
#[derive(Debug, Clone)]
pub struct ActionRecorder {
    /// Recorded actions in order
    actions: Vec<RecordedAction>,

    /// Maximum actions to record
    max_actions: usize,

    /// Whether to validate action legality
    validate_legality: bool,

    /// Track action outcomes
    track_outcomes: bool,
}

#[derive(Debug, Clone)]
struct RecordedAction {
    action: Action,
    timestamp: std::time::Instant,
    stage_before: Option<Stage>,
    stage_after: Option<Stage>,
    was_legal: bool,
    outcome: Option<ActionOutcome>,
}

#[derive(Debug, Clone)]
enum ActionOutcome {
    Success,
    Failed(String),
    StateChange(String),
}

impl ActionRecorder {
    /// Create a new action recorder
    pub fn new() -> Self {
        Self {
            actions: Vec::new(),
            max_actions: 1000,
            validate_legality: true,
            track_outcomes: false,
        }
    }

    /// Create a recorder with custom configuration
    pub fn with_config(max_actions: usize, validate: bool, track_outcomes: bool) -> Self {
        Self {
            actions: Vec::new(),
            max_actions,
            validate_legality: validate,
            track_outcomes,
        }
    }

    /// Record an action
    pub fn record(&mut self, action: Action) {
        if self.actions.len() >= self.max_actions {
            self.actions.remove(0);
        }

        self.actions.push(RecordedAction {
            action,
            timestamp: std::time::Instant::now(),
            stage_before: None,
            stage_after: None,
            was_legal: true,
            outcome: None,
        });
    }

    /// Record an action with context
    pub fn record_with_context(&mut self, action: Action, game: &Game) {
        let stage_before = game.get_stage();

        // Record the action
        self.record(action.clone());

        // Update context in the last recorded action
        if let Some(recorded) = self.actions.last_mut() {
            recorded.stage_before = Some(stage_before);
        }
    }

    /// Record outcome after action execution
    pub fn record_outcome(&mut self, stage_after: Stage, success: bool, message: Option<String>) {
        if let Some(recorded) = self.actions.last_mut() {
            recorded.stage_after = Some(stage_after);
            recorded.was_legal = success;

            if self.track_outcomes {
                recorded.outcome = Some(if success {
                    ActionOutcome::Success
                } else {
                    ActionOutcome::Failed(message.unwrap_or_else(|| "Unknown error".to_string()))
                });
            }
        }
    }

    /// Get all recorded actions
    pub fn get_actions(&self) -> Vec<Action> {
        self.actions.iter().map(|r| r.action.clone()).collect()
    }

    /// Clear all recorded actions
    pub fn clear(&mut self) {
        self.actions.clear();
    }

    /// Validate that all recorded actions were legal
    pub fn validate_sequence(&self) -> bool {
        if !self.validate_legality {
            return true;
        }

        self.actions.iter().all(|r| r.was_legal)
    }

    /// Export actions as a replayable script
    pub fn export_script(&self) -> ActionScript {
        ActionScript::from_actions(self.get_actions())
    }

    /// Get a summary of recorded actions
    pub fn summary(&self) -> String {
        let mut summary = format!("=== Action Recording Summary ===\n");
        summary.push_str(&format!("Total actions: {}\n", self.actions.len()));

        if self.validate_legality {
            let legal_count = self.actions.iter().filter(|a| a.was_legal).count();
            summary.push_str(&format!(
                "Legal actions: {}/{}\n",
                legal_count,
                self.actions.len()
            ));
        }

        // Count action types
        let mut action_counts = std::collections::HashMap::new();
        for recorded in &self.actions {
            let action_type = format!("{:?}", recorded.action)
                .split('(')
                .next()
                .unwrap_or("Unknown")
                .to_string();
            *action_counts.entry(action_type).or_insert(0) += 1;
        }

        summary.push_str("\nAction breakdown:\n");
        for (action_type, count) in action_counts {
            summary.push_str(&format!("  {}: {}\n", action_type, count));
        }

        summary
    }
}

/// A scripted sequence of actions for replay
#[derive(Debug, Clone)]
pub struct ActionScript {
    actions: VecDeque<Action>,
    original_count: usize,
    current_index: usize,
}

impl ActionScript {
    /// Create a new action script
    pub fn new(actions: Vec<Action>) -> Self {
        let count = actions.len();
        Self {
            actions: actions.into_iter().collect(),
            original_count: count,
            current_index: 0,
        }
    }

    /// Create from recorded actions
    pub fn from_actions(actions: Vec<Action>) -> Self {
        Self::new(actions)
    }

    /// Get the next action in the script
    pub fn next(&mut self) -> Option<Action> {
        let action = self.actions.pop_front();
        if action.is_some() {
            self.current_index += 1;
        }
        action
    }

    /// Peek at the next action without consuming it
    pub fn peek(&self) -> Option<&Action> {
        self.actions.front()
    }

    /// Check if there are more actions
    pub fn has_next(&self) -> bool {
        !self.actions.is_empty()
    }

    /// Get progress through the script
    pub fn progress(&self) -> (usize, usize) {
        (self.current_index, self.original_count)
    }

    /// Reset the script to the beginning
    pub fn reset(&mut self, actions: Vec<Action>) {
        self.actions = actions.into_iter().collect();
        self.current_index = 0;
    }
}

/// Validates action sequences for correctness
pub struct ActionValidator {
    rules: Vec<Box<dyn ValidationRule>>,
}

impl ActionValidator {
    /// Create a new validator
    pub fn new() -> Self {
        Self {
            rules: vec![
                Box::new(NoConsecutiveDuplicatesRule),
                Box::new(ValidStageTransitionsRule),
                Box::new(ResourceAvailabilityRule),
            ],
        }
    }

    /// Add a custom validation rule
    pub fn add_rule(&mut self, rule: Box<dyn ValidationRule>) {
        self.rules.push(rule);
    }

    /// Validate an action sequence
    pub fn validate(&self, actions: &[Action]) -> ValidationResult {
        let mut result = ValidationResult::success();

        for rule in &self.rules {
            let rule_result = rule.validate(actions);
            if !rule_result.is_valid {
                result.add_error(rule_result.errors.join("; "));
            }
        }

        result
    }
}

/// Trait for validation rules
pub trait ValidationRule: fmt::Debug {
    /// Validate the action sequence
    fn validate(&self, actions: &[Action]) -> ValidationResult;
}

/// Result of validation
#[derive(Debug)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

impl ValidationResult {
    fn success() -> Self {
        Self {
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    fn add_error(&mut self, error: String) {
        self.errors.push(error);
        self.is_valid = false;
    }

    fn add_warning(&mut self, warning: String) {
        self.warnings.push(warning);
    }
}

// Built-in validation rules

#[derive(Debug)]
struct NoConsecutiveDuplicatesRule;

impl ValidationRule for NoConsecutiveDuplicatesRule {
    fn validate(&self, actions: &[Action]) -> ValidationResult {
        let mut result = ValidationResult::success();

        for window in actions.windows(2) {
            if window[0] == window[1] {
                result.add_warning(format!(
                    "Consecutive duplicate action detected: {:?}",
                    window[0]
                ));
            }
        }

        result
    }
}

#[derive(Debug)]
struct ValidStageTransitionsRule;

impl ValidationRule for ValidStageTransitionsRule {
    fn validate(&self, actions: &[Action]) -> ValidationResult {
        let mut result = ValidationResult::success();

        // Check for invalid action combinations that indicate bad stage transitions
        for (i, action) in actions.iter().enumerate() {
            match action {
                Action::BuyJoker(_) | Action::BuyConsumable(_) | Action::BuyVoucher(_) => {
                    // These should only happen in shop stage
                    if i > 0 {
                        if let Some(prev) = actions.get(i - 1) {
                            match prev {
                                Action::PlayHand(_) | Action::Discard(_) => {
                                    result.add_error(format!(
                                        "Invalid transition: {:?} followed by {:?} (shop action outside shop)",
                                        prev, action
                                    ));
                                }
                                _ => {}
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        result
    }
}

#[derive(Debug)]
struct ResourceAvailabilityRule;

impl ValidationRule for ResourceAvailabilityRule {
    fn validate(&self, actions: &[Action]) -> ValidationResult {
        let mut result = ValidationResult::success();
        let mut money = 4; // Starting money

        for action in actions {
            match action {
                Action::BuyJoker(_) => {
                    if money < 3 {
                        result.add_warning(
                            "Potential insufficient funds for joker purchase".to_string(),
                        );
                    }
                    money -= 3; // Approximate cost
                }
                Action::BuyConsumable(_) => {
                    if money < 2 {
                        result.add_warning(
                            "Potential insufficient funds for consumable purchase".to_string(),
                        );
                    }
                    money -= 2; // Approximate cost
                }
                Action::Reroll => {
                    if money < 5 {
                        result.add_warning("Potential insufficient funds for reroll".to_string());
                    }
                    money -= 5;
                }
                _ => {}
            }
        }

        result
    }
}

/// Builder for creating action sequences
pub struct ActionSequence {
    actions: Vec<Action>,
}

impl ActionSequence {
    /// Create a new sequence builder
    pub fn new() -> Self {
        Self {
            actions: Vec::new(),
        }
    }

    /// Add an action to the sequence
    pub fn then(mut self, action: Action) -> Self {
        self.actions.push(action);
        self
    }

    /// Add multiple actions
    pub fn then_all(mut self, actions: Vec<Action>) -> Self {
        self.actions.extend(actions);
        self
    }

    /// Repeat an action multiple times
    pub fn repeat(mut self, action: Action, count: usize) -> Self {
        for _ in 0..count {
            self.actions.push(action.clone());
        }
        self
    }

    /// Build into a vector
    pub fn build(self) -> Vec<Action> {
        self.actions
    }

    /// Build into an ActionScript
    pub fn into_script(self) -> ActionScript {
        ActionScript::new(self.actions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_action_recorder() {
        let mut recorder = ActionRecorder::new();

        recorder.record(Action::SelectCard(0));
        recorder.record(Action::SelectCard(1));
        recorder.record(Action::PlayHand(vec![0, 1]));

        let actions = recorder.get_actions();
        assert_eq!(actions.len(), 3);

        let summary = recorder.summary();
        assert!(summary.contains("Total actions: 3"));
    }

    #[test]
    fn test_action_script() {
        let actions = vec![
            Action::SelectCard(0),
            Action::SelectCard(1),
            Action::PlayHand(vec![0, 1]),
        ];

        let mut script = ActionScript::new(actions.clone());

        assert_eq!(script.next(), Some(Action::SelectCard(0)));
        assert_eq!(script.peek(), Some(&Action::SelectCard(1)));
        assert!(script.has_next());

        let (current, total) = script.progress();
        assert_eq!(current, 1);
        assert_eq!(total, 3);
    }

    #[test]
    fn test_action_validator() {
        let validator = ActionValidator::new();

        // Valid sequence
        let valid_actions = vec![
            Action::SelectCard(0),
            Action::SelectCard(1),
            Action::PlayHand(vec![0, 1]),
        ];

        let result = validator.validate(&valid_actions);
        assert!(result.is_valid);

        // Sequence with consecutive duplicates
        let duplicate_actions = vec![
            Action::SelectCard(0),
            Action::SelectCard(0), // Duplicate
            Action::PlayHand(vec![0]),
        ];

        let result = validator.validate(&duplicate_actions);
        assert!(!result.warnings.is_empty());
    }

    #[test]
    fn test_action_sequence_builder() {
        let sequence = ActionSequence::new()
            .then(Action::SelectCard(0))
            .then(Action::SelectCard(1))
            .repeat(Action::Discard(vec![2]), 2)
            .build();

        assert_eq!(sequence.len(), 4);
        assert_eq!(sequence[3], Action::Discard(vec![2]));
    }

    #[test]
    fn test_validation_rules() {
        // Test NoConsecutiveDuplicatesRule
        let rule = NoConsecutiveDuplicatesRule;
        let actions = vec![Action::SelectCard(0), Action::SelectCard(0)];
        let result = rule.validate(&actions);
        assert!(!result.warnings.is_empty());

        // Test ValidStageTransitionsRule
        let rule = ValidStageTransitionsRule;
        let invalid_actions = vec![
            Action::PlayHand(vec![0]),
            Action::BuyJoker(0), // Shop action after play
        ];
        let result = rule.validate(&invalid_actions);
        assert!(!result.errors.is_empty());
    }
}
