//! Advanced Joker Traits with Enhanced Condition Support
//!
//! This module extends the base trait system to support advanced joker conditions
//! while maintaining backward compatibility with existing implementations.
//!
//! The advanced traits provide:
//! - Rich context access for sophisticated condition evaluation
//! - Performance optimizations through condition caching
//! - State-dependent and temporal conditions
//! - Flexible composition patterns

use crate::card::Card;
use crate::hand::SelectHand;
use crate::joker::advanced_conditions::{
    AdvancedCondition, AdvancedEvaluationContext, ConditionCache, GameHistory,
};
use crate::joker::traits::{ProcessResult, Rarity};
use crate::joker::JokerId;
use crate::joker_state::JokerStateManager;
use crate::stage::Stage;
use std::fmt::Debug;

/// Enhanced joker gameplay trait with advanced condition support
///
/// This trait extends the basic `JokerGameplay` trait to support advanced
/// condition evaluation with rich context access and performance optimization.
pub trait AdvancedJokerGameplay: Send + Sync + Debug {
    /// Get the joker's identity information
    fn identity(&self) -> &dyn JokerIdentity;

    /// Get the advanced condition that determines when this joker triggers
    ///
    /// This returns the sophisticated condition that controls when the joker
    /// should activate. Unlike simple boolean checks, these conditions can:
    /// - Access joker internal state
    /// - Depend on game history and temporal patterns
    /// - Use performance optimizations like caching
    /// - Compose complex logical expressions
    fn get_trigger_condition(&self) -> &AdvancedCondition;

    /// Process the joker's effect with advanced context
    ///
    /// This method is called when the joker's condition is satisfied and it
    /// needs to apply its effect. The advanced context provides rich access
    /// to game state, history, and optimization systems.
    ///
    /// # Arguments
    /// * `context` - Rich evaluation context with full game state access
    ///
    /// # Returns
    /// The result of processing, including any effects to apply
    fn process_advanced(&mut self, context: &mut AdvancedEvaluationContext) -> ProcessResult;

    /// Get the processing priority for this joker
    ///
    /// Higher priority jokers are processed first. This allows for
    /// sophisticated interaction patterns between jokers.
    fn get_processing_priority(&self, _stage: &Stage) -> i32 {
        0
    }

    /// Check if this joker should be processed in the current context
    ///
    /// This method evaluates the joker's trigger condition using the advanced
    /// evaluation system. It handles caching, optimization, and complex logic.
    fn should_process(&self, context: &mut AdvancedEvaluationContext) -> bool {
        self.get_trigger_condition().evaluate(context)
    }

    /// Update joker internal state based on game events
    ///
    /// This method allows jokers to maintain sophisticated internal state
    /// that can be used in condition evaluation. Called automatically
    /// by the framework when relevant game events occur.
    fn update_internal_state(&mut self, event: &GameEvent) {
        // Default implementation does nothing
        let _ = event;
    }

    /// Get a hash representing the joker's current state for cache optimization
    ///
    /// This is used by the condition caching system to determine when
    /// cached results are still valid. Jokers with mutable state should
    /// include that state in the hash.
    fn state_hash(&self) -> u64 {
        // Default implementation: hash based on joker ID only
        self.identity().joker_type().as_ptr() as u64
    }
}

/// Enhanced joker identity trait with advanced metadata
///
/// Extends the basic identity information with additional metadata
/// needed for advanced condition evaluation and optimization.
pub trait AdvancedJokerIdentity: Send + Sync + Debug {
    /// Basic identity information
    fn joker_type(&self) -> &'static str;
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn rarity(&self) -> Rarity;
    fn base_cost(&self) -> u64;

    /// Advanced metadata for optimization and categorization
    fn category_tags(&self) -> &[&'static str] {
        &[]
    }

    fn is_state_dependent(&self) -> bool {
        false
    }

    fn is_temporal(&self) -> bool {
        false
    }

    fn cache_lifetime_hint(&self) -> Option<u32> {
        None
    }

    /// Performance characteristics for optimization
    fn evaluation_cost_estimate(&self) -> EvaluationCost {
        EvaluationCost::Cheap
    }
}

/// Alias for backward compatibility
pub use AdvancedJokerIdentity as JokerIdentity;

/// Evaluation cost estimates for optimization
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum EvaluationCost {
    /// Very cheap evaluation (simple comparisons)
    Cheap,
    /// Moderate cost (requires some computation)
    Moderate,
    /// Expensive evaluation (complex logic, state access)
    Expensive,
    /// Very expensive (should be cached aggressively)
    VeryExpensive,
}

/// Game events that jokers can respond to for state updates
#[derive(Debug, Clone)]
pub enum GameEvent {
    /// A hand was played
    HandPlayed {
        hand_type: crate::rank::HandRank,
        cards: Vec<Card>,
    },
    /// Cards were discarded
    CardsDiscarded { cards: Vec<Card> },
    /// A new round started
    RoundStarted { round_number: u32 },
    /// A blind was defeated
    BlindDefeated { blind_type: String },
    /// A joker was purchased
    JokerPurchased { joker_id: JokerId },
    /// A joker was sold
    JokerSold { joker_id: JokerId },
    /// Money was gained
    MoneyGained { amount: i32, source: String },
    /// Stage changed
    StageChanged { from: Stage, to: Stage },
}

/// Enhanced joker implementation that combines multiple trait capabilities
///
/// This struct provides a complete implementation that supports both
/// basic and advanced joker functionality, making it easy to create
/// sophisticated jokers with minimal boilerplate.
#[derive(Debug)]
pub struct EnhancedJoker {
    /// Basic identity information
    pub identity: Box<dyn AdvancedJokerIdentity>,

    /// Advanced trigger condition
    pub trigger_condition: AdvancedCondition,

    /// Processing function for when the joker triggers
    pub processor: Box<dyn JokerProcessor>,

    /// Processing priority (higher = earlier)
    pub priority: i32,

    /// Internal state for condition evaluation
    pub internal_state: InternalJokerState,
}

/// Internal state that jokers can maintain
#[derive(Debug, Default)]
pub struct InternalJokerState {
    /// Generic state storage
    pub data: std::collections::HashMap<String, serde_json::Value>,

    /// Counters for various events
    pub counters: std::collections::HashMap<String, u64>,

    /// Flags for boolean state
    pub flags: std::collections::HashMap<String, bool>,

    /// State version for cache invalidation
    pub version: u64,
}

impl InternalJokerState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn increment_counter(&mut self, key: &str) -> u64 {
        let new_value = self.counters.get(key).unwrap_or(&0) + 1;
        self.counters.insert(key.to_string(), new_value);
        self.version += 1;
        new_value
    }

    pub fn set_flag(&mut self, key: &str, value: bool) {
        self.flags.insert(key.to_string(), value);
        self.version += 1;
    }

    pub fn set_data(&mut self, key: &str, value: serde_json::Value) {
        self.data.insert(key.to_string(), value);
        self.version += 1;
    }

    pub fn get_counter(&self, key: &str) -> u64 {
        self.counters.get(key).copied().unwrap_or(0)
    }

    pub fn get_flag(&self, key: &str) -> bool {
        self.flags.get(key).copied().unwrap_or(false)
    }

    pub fn get_data(&self, key: &str) -> Option<&serde_json::Value> {
        self.data.get(key)
    }
}

/// Trait for joker processing logic
pub trait JokerProcessor: Send + Sync + Debug {
    fn process(
        &self,
        context: &mut AdvancedEvaluationContext,
        state: &mut InternalJokerState,
    ) -> ProcessResult;
}

impl EnhancedJoker {
    /// Create a new builder for constructing an enhanced joker
    pub fn builder() -> EnhancedJokerBuilder {
        EnhancedJokerBuilder::new()
    }
}

impl AdvancedJokerGameplay for EnhancedJoker {
    fn identity(&self) -> &dyn JokerIdentity {
        self.identity.as_ref()
    }

    fn get_trigger_condition(&self) -> &AdvancedCondition {
        &self.trigger_condition
    }

    fn process_advanced(&mut self, context: &mut AdvancedEvaluationContext) -> ProcessResult {
        self.processor.process(context, &mut self.internal_state)
    }

    fn get_processing_priority(&self, _stage: &Stage) -> i32 {
        self.priority
    }

    fn update_internal_state(&mut self, event: &GameEvent) {
        // Update counters based on event type
        match event {
            GameEvent::HandPlayed { .. } => {
                self.internal_state.increment_counter("hands_played");
            }
            GameEvent::CardsDiscarded { cards } => {
                self.internal_state.increment_counter("cards_discarded");
                self.internal_state
                    .set_data("last_discard_count", serde_json::json!(cards.len()));
            }
            GameEvent::RoundStarted { round_number } => {
                self.internal_state
                    .set_data("current_round", serde_json::json!(round_number));
                // Reset per-round counters
                self.internal_state
                    .counters
                    .insert("hands_this_round".to_string(), 0);
                self.internal_state
                    .counters
                    .insert("discards_this_round".to_string(), 0);
            }
            GameEvent::MoneyGained { amount, .. } => {
                let total = self.internal_state.get_counter("total_money_seen") + *amount as u64;
                self.internal_state
                    .counters
                    .insert("total_money_seen".to_string(), total);
            }
            _ => {} // Other events don't affect this joker's state
        }
    }

    fn state_hash(&self) -> u64 {
        // Include internal state version in hash for cache invalidation
        let base_hash = self.identity().joker_type().as_ptr() as u64;
        base_hash ^ self.internal_state.version
    }
}

/// Builder for creating enhanced jokers with fluent API
pub struct EnhancedJokerBuilder {
    identity: Option<Box<dyn AdvancedJokerIdentity>>,
    condition: Option<AdvancedCondition>,
    processor: Option<Box<dyn JokerProcessor>>,
    priority: i32,
}

impl EnhancedJokerBuilder {
    pub fn new() -> Self {
        Self {
            identity: None,
            condition: None,
            processor: None,
            priority: 0,
        }
    }

    pub fn identity(mut self, identity: Box<dyn AdvancedJokerIdentity>) -> Self {
        self.identity = Some(identity);
        self
    }

    pub fn condition(mut self, condition: AdvancedCondition) -> Self {
        self.condition = Some(condition);
        self
    }

    pub fn processor(mut self, processor: Box<dyn JokerProcessor>) -> Self {
        self.processor = Some(processor);
        self
    }

    pub fn priority(mut self, priority: i32) -> Self {
        self.priority = priority;
        self
    }

    pub fn build(self) -> Result<EnhancedJoker, &'static str> {
        Ok(EnhancedJoker {
            identity: self.identity.ok_or("Identity is required")?,
            trigger_condition: self.condition.ok_or("Condition is required")?,
            processor: self.processor.ok_or("Processor is required")?,
            priority: self.priority,
            internal_state: InternalJokerState::new(),
        })
    }
}

impl Default for EnhancedJokerBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Framework integration utilities
pub struct AdvancedJokerFramework;

impl AdvancedJokerFramework {
    /// Create an advanced evaluation context from basic components
    #[allow(clippy::too_many_arguments)]
    pub fn create_evaluation_context<'a>(
        game_context: &'a crate::joker::GameContext,
        stage: &'a Stage,
        hand: Option<&'a SelectHand>,
        card: Option<&'a Card>,
        joker_state_manager: &'a JokerStateManager,
        evaluating_joker_id: JokerId,
        condition_cache: &'a mut ConditionCache,
        game_history: &'a GameHistory,
    ) -> AdvancedEvaluationContext<'a> {
        AdvancedEvaluationContext {
            game_context,
            stage,
            hand,
            card,
            joker_state_manager,
            evaluating_joker_id,
            condition_cache,
            game_history,
        }
    }

    /// Process a collection of advanced jokers with proper ordering and optimization
    pub fn process_jokers(
        jokers: &mut [Box<dyn AdvancedJokerGameplay>],
        context: &mut AdvancedEvaluationContext,
    ) -> Vec<ProcessResult> {
        // Sort by priority (higher first)
        let mut indexed_jokers: Vec<(usize, i32)> = jokers
            .iter()
            .enumerate()
            .map(|(i, joker)| (i, joker.get_processing_priority(context.stage)))
            .collect();

        indexed_jokers.sort_by(|a, b| b.1.cmp(&a.1));

        let mut results = Vec::new();

        for (index, _priority) in indexed_jokers {
            let joker = &mut jokers[index];

            // Update context to reflect which joker we're evaluating
            context.evaluating_joker_id = JokerId::Joker; // TODO: Get from joker identity

            if joker.should_process(context) {
                let result = joker.process_advanced(context);
                results.push(result);
            }
        }

        results
    }

    /// Update all jokers with a game event
    pub fn broadcast_event(jokers: &mut [Box<dyn AdvancedJokerGameplay>], event: &GameEvent) {
        for joker in jokers.iter_mut() {
            joker.update_internal_state(event);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::joker::advanced_conditions::AdvancedConditionBuilder;
    use crate::joker::traits::Rarity;
    use crate::stage::{Blind, Stage};
    use serde_json::json;

    // Mock identity for testing
    #[derive(Debug)]
    struct MockJokerIdentity {
        joker_type: &'static str,
        name: String,
        description: String,
        rarity: Rarity,
        cost: u64,
    }

    impl AdvancedJokerIdentity for MockJokerIdentity {
        fn joker_type(&self) -> &'static str {
            self.joker_type
        }
        fn name(&self) -> &str {
            &self.name
        }
        fn description(&self) -> &str {
            &self.description
        }
        fn rarity(&self) -> Rarity {
            self.rarity
        }
        fn base_cost(&self) -> u64 {
            self.cost
        }

        fn is_state_dependent(&self) -> bool {
            true
        }
        fn evaluation_cost_estimate(&self) -> EvaluationCost {
            EvaluationCost::Moderate
        }
    }

    // Mock processor for testing
    #[derive(Debug)]
    struct MockProcessor {
        chips_bonus: u64,
        mult_bonus: f64,
    }

    impl JokerProcessor for MockProcessor {
        fn process(
            &self,
            _context: &mut AdvancedEvaluationContext,
            state: &mut InternalJokerState,
        ) -> ProcessResult {
            state.increment_counter("triggers");

            ProcessResult {
                chips_added: self.chips_bonus,
                mult_added: self.mult_bonus,
                mult_multiplier: 1.0,
                retriggered: false,
                message: Some(format!("Triggered {} times", state.get_counter("triggers"))),
            }
        }
    }

    #[test]
    fn test_enhanced_joker_builder() {
        let identity = Box::new(MockJokerIdentity {
            joker_type: "test_joker",
            name: "Test Joker".to_string(),
            description: "A joker for testing".to_string(),
            rarity: Rarity::Common,
            cost: 5,
        });

        let condition = AdvancedConditionBuilder::hands_played_this_round(3);
        let processor = Box::new(MockProcessor {
            chips_bonus: 20,
            mult_bonus: 2.0,
        });

        let joker = EnhancedJokerBuilder::new()
            .identity(identity)
            .condition(condition)
            .processor(processor)
            .priority(10)
            .build();

        assert!(joker.is_ok());
        let joker = joker.unwrap();

        assert_eq!(joker.identity().name(), "Test Joker");
        assert_eq!(
            joker.get_processing_priority(&Stage::Blind(Blind::Small)),
            10
        );
    }

    #[test]
    fn test_internal_joker_state() {
        let mut state = InternalJokerState::new();

        // Test counters
        assert_eq!(state.get_counter("test"), 0);
        state.increment_counter("test");
        assert_eq!(state.get_counter("test"), 1);

        // Test flags
        assert!(!state.get_flag("active"));
        state.set_flag("active", true);
        assert!(state.get_flag("active"));

        // Test data
        state.set_data("config", json!({"level": 5}));
        let data = state.get_data("config").unwrap();
        assert_eq!(data["level"], 5);

        // Test version increment
        let initial_version = state.version;
        state.increment_counter("test");
        assert!(state.version > initial_version);
    }

    #[test]
    fn test_evaluation_cost_ordering() {
        use EvaluationCost::*;

        // Cheaper costs should be less than expensive ones
        assert!(Cheap < Moderate);
        assert!(Moderate < Expensive);
        assert!(Expensive < VeryExpensive);
    }

    #[test]
    fn test_game_event_handling() {
        let mut enhanced_joker = create_test_joker();

        // Test hand played event
        let event = GameEvent::HandPlayed {
            hand_type: crate::rank::HandRank::HighCard,
            cards: vec![],
        };

        enhanced_joker.update_internal_state(&event);
        assert_eq!(enhanced_joker.internal_state.get_counter("hands_played"), 1);

        // Test round started event
        let event = GameEvent::RoundStarted { round_number: 5 };
        enhanced_joker.update_internal_state(&event);

        assert_eq!(
            enhanced_joker.internal_state.get_data("current_round"),
            Some(&json!(5))
        );
        assert_eq!(
            enhanced_joker
                .internal_state
                .get_counter("hands_this_round"),
            0
        );
    }

    fn create_test_joker() -> EnhancedJoker {
        let identity = Box::new(MockJokerIdentity {
            joker_type: "test",
            name: "Test".to_string(),
            description: "Test joker".to_string(),
            rarity: Rarity::Common,
            cost: 3,
        });

        let condition =
            AdvancedConditionBuilder::legacy(crate::joker::conditional::JokerCondition::Always);

        let processor = Box::new(MockProcessor {
            chips_bonus: 10,
            mult_bonus: 1.0,
        });

        EnhancedJokerBuilder::new()
            .identity(identity)
            .condition(condition)
            .processor(processor)
            .build()
            .unwrap()
    }
}
