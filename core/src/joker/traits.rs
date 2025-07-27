//! New trait definitions for the Joker system
//!
//! This module defines focused, single-responsibility traits that will eventually
//! replace the monolithic Joker trait. Each trait handles a specific aspect of
//! joker behavior, making the system more modular and maintainable.

use crate::card::Card;
use crate::hand::SelectHand;
use crate::joker_state::JokerStateManager;
use crate::stage::Stage;
use serde::{Deserialize, Serialize};

/// Simple structure to hold hand scoring information
#[derive(Debug, Clone)]
pub struct HandScore {
    pub chips: u64,
    pub mult: f64,
}

/// Simple game event type for joker processing
#[derive(Debug, Clone)]
pub struct GameEvent {
    pub event_type: String,
    pub data: Option<serde_json::Value>,
}

/// Trait for joker identity and metadata.
///
/// This trait handles the basic identity information for a joker,
/// including its type, name, and descriptive information.
pub trait JokerIdentity: Send + Sync {
    /// Returns the unique type identifier for this joker.
    fn joker_type(&self) -> &'static str;

    /// Returns the display name of this joker.
    fn name(&self) -> &str;

    /// Returns a description of what this joker does.
    fn description(&self) -> &str;

    /// Returns the rarity level of this joker.
    fn rarity(&self) -> Rarity;

    /// Returns the base cost of this joker in the shop.
    fn base_cost(&self) -> u64;

    /// Returns whether this joker is a unique/legendary variant.
    fn is_unique(&self) -> bool {
        false
    }
}

/// Trait for joker lifecycle management.
///
/// This trait handles the lifecycle events of a joker, from purchase
/// through gameplay to sale or destruction.
pub trait JokerLifecycle: Send + Sync {
    /// Called when the joker is purchased from the shop.
    fn on_purchase(&mut self) {}

    /// Called when the joker is sold.
    fn on_sell(&mut self) {}

    /// Called when the joker is destroyed.
    fn on_destroy(&mut self) {}

    /// Called at the start of each round.
    fn on_round_start(&mut self) {}

    /// Called at the end of each round.
    fn on_round_end(&mut self) {}

    /// Called when another joker is added to the collection.
    fn on_joker_added(&mut self, _other_joker_type: &str) {}

    /// Called when another joker is removed from the collection.
    fn on_joker_removed(&mut self, _other_joker_type: &str) {}
}

/// Trait for joker gameplay mechanics.
///
/// This trait handles the core gameplay functionality of jokers,
/// including scoring and card interactions.
///
/// # State Management
///
/// The `process()` method takes a mutable `&mut self` reference, allowing jokers
/// to maintain internal state directly. This enables cleaner, more efficient code
/// compared to external state management.
///
/// ## Common Patterns
///
/// ### Per-Round State
/// ```rust,ignore
/// struct MyJoker {
///     triggered_this_round: bool,
/// }
///
/// impl JokerGameplay for MyJoker {
///     fn process(&mut self, stage: &Stage, context: &mut ProcessContext) -> ProcessResult {
///         if !self.triggered_this_round && self.should_trigger(context) {
///             self.triggered_this_round = true;
///             // Return effect
///             ProcessResult {
///                 mult_added: 5.0,
///                 ..Default::default()
///             }
///         } else {
///             ProcessResult::default()
///         }
///     }
/// }
/// ```
///
/// ### Accumulating State
/// ```rust,ignore
/// struct AccumulatingJoker {
///     accumulated_value: f64,
/// }
///
/// impl JokerGameplay for AccumulatingJoker {
///     fn process(&mut self, stage: &Stage, context: &mut ProcessContext) -> ProcessResult {
///         self.accumulated_value += 1.0;
///         ProcessResult {
///             mult_added: self.accumulated_value,
///             ..Default::default()
///         }
///     }
/// }
/// ```
///
/// ## Thread Safety
///
/// For jokers that need thread-safe state sharing, use interior mutability patterns
/// like `Mutex`, `RwLock`, or atomic types. The joker itself must remain `Send + Sync`.
pub trait JokerGameplay: Send + Sync {
    /// Processes the joker's effect during the specified stage.
    fn process(&mut self, stage: &Stage, context: &mut ProcessContext) -> ProcessResult;

    /// Checks if this joker can trigger based on the current game state.
    fn can_trigger(&self, stage: &Stage, context: &ProcessContext) -> bool;

    /// Gets the order priority for this joker's processing (higher = earlier).
    fn get_priority(&self, _stage: &Stage) -> i32 {
        0
    }
}

/// Trait for joker modifiers and effects.
///
/// This trait handles modifiers that jokers can apply to scoring,
/// hand size, and other game mechanics.
pub trait JokerModifiers: Send + Sync {
    /// Returns the chip multiplier this joker provides.
    fn get_chip_mult(&self) -> f64 {
        1.0
    }

    /// Returns the score multiplier this joker provides.
    fn get_score_mult(&self) -> f64 {
        1.0
    }

    /// Returns the hand size modifier this joker provides.
    fn get_hand_size_modifier(&self) -> i32 {
        0
    }

    /// Returns the discard modifier this joker provides.
    fn get_discard_modifier(&self) -> i32 {
        0
    }
}

/// Trait for joker state management.
///
/// This trait handles the internal state of jokers, including
/// serialization and state queries.
pub trait JokerState: Send + Sync {
    /// Returns whether this joker has any internal state.
    fn has_state(&self) -> bool {
        false
    }

    /// Serializes the joker's state to a value.
    fn serialize_state(&self) -> Option<serde_json::Value> {
        None
    }

    /// Deserializes the joker's state from a value.
    fn deserialize_state(&mut self, _value: serde_json::Value) -> Result<(), String> {
        Ok(())
    }

    /// Returns a debug representation of the joker's current state.
    fn debug_state(&self) -> String {
        "{}".to_string()
    }

    /// Resets the joker's state to its initial values.
    fn reset_state(&mut self) {}
}

/// Rarity levels for jokers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Legendary,
}

/// Context provided to jokers during processing.
///
/// This struct provides access to game state that jokers may need during their
/// processing phase, such as current hand score, played cards, and game events.
///
/// # State Management
///
/// With the `process()` method now taking `&mut self`, jokers can maintain their
/// own internal state directly. The `joker_state_manager` field is retained for
/// backward compatibility during the migration period, but new jokers should
/// use internal state instead.
///
/// ## Example: Direct state management
/// ```rust,ignore
/// struct CounterJoker {
///     counter: u32,
/// }
///
/// impl JokerGameplay for CounterJoker {
///     fn process(&mut self, stage: &Stage, context: &mut ProcessContext) -> ProcessResult {
///         self.counter += 1;  // Direct state update
///         ProcessResult {
///             mult_added: self.counter as f64,
///             ..Default::default()
///         }
///     }
/// }
/// ```
pub struct ProcessContext<'a> {
    pub hand_score: &'a mut HandScore,
    pub played_cards: &'a [Card],
    pub held_cards: &'a [Card],
    pub events: &'a mut Vec<GameEvent>,
    pub hand: &'a SelectHand,
    pub joker_state_manager: &'a JokerStateManager,
}

/// Result returned from joker processing.
pub struct ProcessResult {
    pub chips_added: u64,
    pub mult_added: f64,
    pub mult_multiplier: f64,
    pub retriggered: bool,
    pub message: Option<String>,
}

impl Default for ProcessResult {
    fn default() -> Self {
        Self {
            chips_added: 0,
            mult_added: 0.0,
            mult_multiplier: 1.0,
            retriggered: false,
            message: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::{Card, Suit, Value};
    use crate::hand::SelectHand;
    use crate::joker_state::JokerStateManager;
    use crate::stage::{Blind, Stage};
    use serde_json::json;

    /// Zero-allocation mock implementation for testing JokerGameplay
    #[derive(Debug, Clone, Copy)]
    struct StaticGameplayJoker {
        joker_id: crate::joker::JokerId,
        base_chips: u64,
        base_mult: f64,
        trigger_condition: TriggerCondition,
        priority: i32,
        requires_state: bool,
    }

    /// Compile-time trigger conditions for testing
    #[derive(Debug, Clone, Copy, PartialEq)]
    enum TriggerCondition {
        Always,
        Never,
        OnShopStage,
        RequiresAce,
        RequiresPair,
        RequiresFlush,
        MinCards(usize),
    }

    impl StaticGameplayJoker {
        const fn new(joker_id: crate::joker::JokerId) -> Self {
            Self {
                joker_id,
                base_chips: 10,
                base_mult: 1.5,
                trigger_condition: TriggerCondition::Always,
                priority: 0,
                requires_state: false,
            }
        }

        const fn with_params(
            joker_id: crate::joker::JokerId,
            chips: u64,
            mult: f64,
            condition: TriggerCondition,
            priority: i32,
            requires_state: bool,
        ) -> Self {
            Self {
                joker_id,
                base_chips: chips,
                base_mult: mult,
                trigger_condition: condition,
                priority,
                requires_state,
            }
        }
    }

    impl JokerGameplay for StaticGameplayJoker {
        fn process(&mut self, stage: &Stage, context: &mut ProcessContext) -> ProcessResult {
            if !self.can_trigger(stage, context) {
                return ProcessResult::default();
            }

            if self.requires_state {
                let trigger_count: u32 = context
                    .joker_state_manager
                    .get_custom_data(self.joker_id, "trigger_count")
                    .ok()
                    .flatten()
                    .unwrap_or(0);

                let _ = context.joker_state_manager.set_custom_data(
                    self.joker_id,
                    "trigger_count",
                    json!(trigger_count + 1),
                );

                ProcessResult {
                    chips_added: self.base_chips * (trigger_count + 1) as u64,
                    mult_added: self.base_mult * (trigger_count + 1) as f64,
                    mult_multiplier: 1.0,
                    retriggered: trigger_count > 0,
                    message: None,
                }
            } else {
                ProcessResult {
                    chips_added: self.base_chips,
                    mult_added: self.base_mult,
                    mult_multiplier: 1.0,
                    retriggered: false,
                    message: None,
                }
            }
        }

        fn can_trigger(&self, stage: &Stage, context: &ProcessContext) -> bool {
            match self.trigger_condition {
                TriggerCondition::Always => true,
                TriggerCondition::Never => false,
                TriggerCondition::OnShopStage => matches!(stage, Stage::Shop()),
                TriggerCondition::RequiresAce => context
                    .played_cards
                    .iter()
                    .any(|card| card.value == Value::Ace),
                TriggerCondition::RequiresPair => {
                    let mut rank_counts = [0u8; 13];
                    for card in context.played_cards {
                        rank_counts[card.value as usize] += 1;
                    }
                    rank_counts.iter().any(|&count| count >= 2)
                }
                TriggerCondition::RequiresFlush => {
                    if context.played_cards.is_empty() {
                        return false;
                    }
                    let first_suit = context.played_cards[0].suit;
                    context
                        .played_cards
                        .iter()
                        .all(|card| card.suit == first_suit)
                }
                TriggerCondition::MinCards(min) => context.played_cards.len() >= min,
            }
        }

        fn get_priority(&self, _stage: &Stage) -> i32 {
            self.priority
        }
    }

    // Compile-time test data
    const TEST_JOKERS: &[StaticGameplayJoker] = &[
        StaticGameplayJoker::new(crate::joker::JokerId::Joker),
        StaticGameplayJoker::with_params(
            crate::joker::JokerId::GreedyJoker,
            50,
            2.0,
            TriggerCondition::RequiresAce,
            10,
            false,
        ),
        StaticGameplayJoker::with_params(
            crate::joker::JokerId::LustyJoker,
            0,
            1.0,
            TriggerCondition::OnShopStage,
            5,
            false,
        ),
        StaticGameplayJoker::with_params(
            crate::joker::JokerId::WrathfulJoker,
            5,
            0.5,
            TriggerCondition::Always,
            0,
            true,
        ),
        StaticGameplayJoker::with_params(
            crate::joker::JokerId::GluttonousJoker,
            100,
            10.0,
            TriggerCondition::Never,
            -10,
            false,
        ),
        StaticGameplayJoker::with_params(
            crate::joker::JokerId::JollyJoker,
            30,
            3.0,
            TriggerCondition::RequiresFlush,
            15,
            false,
        ),
        StaticGameplayJoker::with_params(
            crate::joker::JokerId::ZanyJoker,
            20,
            1.5,
            TriggerCondition::RequiresPair,
            8,
            false,
        ),
        StaticGameplayJoker::with_params(
            crate::joker::JokerId::MadJoker,
            15,
            2.5,
            TriggerCondition::MinCards(4),
            3,
            false,
        ),
    ];

    // Test card data - efficient functions that return static slices
    fn test_cards() -> &'static [Card] {
        use once_cell::sync::Lazy;
        static CARDS: Lazy<Vec<Card>> = Lazy::new(|| {
            vec![
                Card::new(Value::Ace, Suit::Spade),
                Card::new(Value::King, Suit::Heart),
                Card::new(Value::Queen, Suit::Diamond),
                Card::new(Value::Jack, Suit::Club),
                Card::new(Value::Ten, Suit::Spade),
            ]
        });
        &CARDS
    }

    fn flush_cards() -> &'static [Card] {
        use once_cell::sync::Lazy;
        static CARDS: Lazy<Vec<Card>> = Lazy::new(|| {
            vec![
                Card::new(Value::Ace, Suit::Heart),
                Card::new(Value::King, Suit::Heart),
                Card::new(Value::Queen, Suit::Heart),
                Card::new(Value::Jack, Suit::Heart),
                Card::new(Value::Ten, Suit::Heart),
            ]
        });
        &CARDS
    }

    fn pair_cards() -> &'static [Card] {
        use once_cell::sync::Lazy;
        static CARDS: Lazy<Vec<Card>> = Lazy::new(|| {
            vec![
                Card::new(Value::Ace, Suit::Heart),
                Card::new(Value::Ace, Suit::Spade),
                Card::new(Value::King, Suit::Diamond),
            ]
        });
        &CARDS
    }

    // Test fixtures to reduce repetitive setup
    struct TestFixture {
        hand_score: HandScore,
        events: Vec<GameEvent>,
        state_manager: JokerStateManager,
    }

    impl TestFixture {
        fn new() -> Self {
            Self {
                hand_score: HandScore {
                    chips: 50,
                    mult: 2.0,
                },
                events: Vec::new(),
                state_manager: JokerStateManager::new(),
            }
        }

        fn create_context<'a>(
            &'a mut self,
            played_cards: &'a [Card],
            held_cards: &'a [Card],
            hand: &'a SelectHand,
        ) -> ProcessContext<'a> {
            ProcessContext {
                hand_score: &mut self.hand_score,
                played_cards,
                held_cards,
                events: &mut self.events,
                hand,
                joker_state_manager: &self.state_manager,
            }
        }
    }

    #[test]
    fn test_basic_process() {
        let mut joker = TEST_JOKERS[0];
        let mut fixture = TestFixture::new();
        let hand = SelectHand::new(test_cards()[..3].to_vec());
        let mut context = fixture.create_context(&test_cards()[..3], &[], &hand);

        let result = joker.process(&Stage::Blind(Blind::Small), &mut context);
        assert_eq!(result.chips_added, 10);
        assert_eq!(result.mult_added, 1.5);
        assert!(!result.retriggered);
    }

    #[test]
    fn test_trigger_conditions() {
        let ace_joker = &TEST_JOKERS[1];
        let shop_joker = &TEST_JOKERS[2];
        let never_joker = &TEST_JOKERS[4];

        let mut fixture = TestFixture::new();
        let hand = SelectHand::new(test_cards()[..1].to_vec());
        let context = fixture.create_context(&test_cards()[..1], &[], &hand);

        // Test ace trigger (first card is an ace)
        assert!(ace_joker.can_trigger(&Stage::Blind(Blind::Small), &context));

        // Test shop stage trigger
        assert!(shop_joker.can_trigger(&Stage::Shop(), &context));
        assert!(!shop_joker.can_trigger(&Stage::Blind(Blind::Small), &context));

        // Test never trigger
        assert!(!never_joker.can_trigger(&Stage::Blind(Blind::Small), &context));
    }

    #[test]
    fn test_state_scaling() {
        let mut joker = TEST_JOKERS[3]; // state_scaler
        let mut fixture = TestFixture::new();
        let hand = SelectHand::new(vec![]);
        let mut context = fixture.create_context(&[], &[], &hand);

        // Test scaling over multiple triggers
        let expected = [(5, 0.5), (10, 1.0), (15, 1.5)];
        for (i, (chips, mult)) in expected.iter().enumerate() {
            let result = joker.process(&Stage::Blind(Blind::Small), &mut context);
            assert_eq!(result.chips_added, *chips);
            assert_eq!(result.mult_added, *mult);
            assert_eq!(result.retriggered, i > 0);
        }
    }

    #[test]
    fn test_priority_ordering() {
        let priorities: Vec<i32> = TEST_JOKERS
            .iter()
            .map(|j| j.get_priority(&Stage::Blind(Blind::Small)))
            .collect();

        assert_eq!(priorities[0], 0); // basic
        assert_eq!(priorities[1], 10); // ace_hunter
        assert_eq!(priorities[4], -10); // never_trigger
        assert_eq!(priorities[5], 15); // flush_master
    }

    #[test]
    fn test_card_conditions() {
        let flush_joker = &TEST_JOKERS[5];
        let pair_joker = &TEST_JOKERS[6];
        let big_hand_joker = &TEST_JOKERS[7];

        // Test flush
        let mut fixture_flush = TestFixture::new();
        let flush_hand = SelectHand::new(flush_cards().to_vec());
        let context_flush = fixture_flush.create_context(flush_cards(), &[], &flush_hand);
        assert!(flush_joker.can_trigger(&Stage::Blind(Blind::Small), &context_flush));

        // Test pair
        let mut fixture_pair = TestFixture::new();
        let pair_hand = SelectHand::new(pair_cards().to_vec());
        let context_pair = fixture_pair.create_context(pair_cards(), &[], &pair_hand);
        assert!(pair_joker.can_trigger(&Stage::Blind(Blind::Small), &context_pair));

        // Test min cards
        let mut fixture_big = TestFixture::new();
        let big_hand = SelectHand::new(test_cards()[..4].to_vec());
        let context_big = fixture_big.create_context(&test_cards()[..4], &[], &big_hand);
        assert!(big_hand_joker.can_trigger(&Stage::Blind(Blind::Small), &context_big));
    }

    #[test]
    fn test_scoring_invariants() {
        // Property: Processing should never produce negative scores
        for &joker in TEST_JOKERS {
            let mut joker_copy = joker;
            let mut fixture = TestFixture::new();
            fixture.hand_score.chips = 100;
            fixture.hand_score.mult = 2.0;

            let hand = SelectHand::new(test_cards().to_vec());
            let mut context = fixture.create_context(test_cards(), &[], &hand);

            let result = joker_copy.process(&Stage::Blind(Blind::Small), &mut context);

            // chips_added is u64, always non-negative by type
            assert!(result.mult_added >= 0.0);
            assert_eq!(fixture.hand_score.chips, 100); // Hand score unchanged
            assert_eq!(fixture.hand_score.mult, 2.0);
        }
    }

    #[test]
    fn test_thread_safety() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<StaticGameplayJoker>();
    }

    #[test]
    fn test_multi_joker_interactions() {
        // Test multiple jokers processing in sequence
        let mut fixture = TestFixture::new();

        // Test different jokers with their appropriate conditions
        let mut always_joker = TEST_JOKERS[0]; // priority 0, always triggers
        let mut ace_joker = TEST_JOKERS[1]; // priority 10, requires ace
        let mut flush_joker = TEST_JOKERS[5]; // priority 15, requires flush

        // Test with flush hand for flush joker
        let flush_hand = SelectHand::new(flush_cards().to_vec());
        let mut context = fixture.create_context(flush_cards(), &[], &flush_hand);

        let result1 = flush_joker.process(&Stage::Blind(Blind::Small), &mut context);
        assert_eq!(result1.chips_added, 30); // flush_master
        assert_eq!(result1.mult_added, 3.0);

        // Apply effects to hand score for cumulative testing
        fixture.hand_score.chips += result1.chips_added;
        fixture.hand_score.mult += result1.mult_added;

        // Test with ace hand for ace joker
        let ace_hand = SelectHand::new(test_cards()[..1].to_vec()); // First card is ace
        let mut context2 = fixture.create_context(&test_cards()[..1], &[], &ace_hand);
        let result2 = ace_joker.process(&Stage::Blind(Blind::Small), &mut context2);
        assert_eq!(result2.chips_added, 50); // ace_hunter
        assert_eq!(result2.mult_added, 2.0);

        // Test always joker with any hand
        let any_hand = SelectHand::new(test_cards().to_vec());
        let mut context3 = fixture.create_context(test_cards(), &[], &any_hand);
        let result3 = always_joker.process(&Stage::Blind(Blind::Small), &mut context3);
        assert_eq!(result3.chips_added, 10); // basic joker
        assert_eq!(result3.mult_added, 1.5);

        // Verify cumulative effects
        let total_chips_added = result1.chips_added + result2.chips_added + result3.chips_added;
        let total_mult_added = result1.mult_added + result2.mult_added + result3.mult_added;
        assert_eq!(total_chips_added, 90); // 30 + 50 + 10
        assert_eq!(total_mult_added, 6.5); // 3.0 + 2.0 + 1.5
    }

    #[test]
    fn test_multi_joker_state_interactions() {
        // Test jokers with shared state via JokerStateManager
        let mut fixture = TestFixture::new();
        let hand = SelectHand::new(vec![]);

        // Two state-based jokers
        let mut state_joker1 = TEST_JOKERS[3]; // state_scaler
        let mut state_joker2 = TEST_JOKERS[3]; // another state_scaler

        // First joker triggers and updates state
        let mut context1 = fixture.create_context(&[], &[], &hand);
        let result1 = state_joker1.process(&Stage::Blind(Blind::Small), &mut context1);
        assert_eq!(result1.chips_added, 5);
        assert_eq!(result1.mult_added, 0.5);

        // Second joker sees updated state
        let mut context2 = fixture.create_context(&[], &[], &hand);
        let result2 = state_joker2.process(&Stage::Blind(Blind::Small), &mut context2);
        assert_eq!(result2.chips_added, 10); // Should see incremented state
        assert_eq!(result2.mult_added, 1.0);

        // First joker again should see further updated state
        let mut context3 = fixture.create_context(&[], &[], &hand);
        let result3 = state_joker1.process(&Stage::Blind(Blind::Small), &mut context3);
        assert_eq!(result3.chips_added, 15);
        assert_eq!(result3.mult_added, 1.5);
        assert!(result3.retriggered);
    }

    #[test]
    fn test_edge_case_empty_hand() {
        // Edge case: no cards in hand
        let mut fixture = TestFixture::new();
        let empty_hand = SelectHand::new(vec![]);
        let mut context = fixture.create_context(&[], &[], &empty_hand);

        for &joker in TEST_JOKERS {
            let mut joker_copy = joker;
            let result = joker_copy.process(&Stage::Blind(Blind::Small), &mut context);

            // Jokers should handle empty hands gracefully
            match joker.trigger_condition {
                TriggerCondition::RequiresAce
                | TriggerCondition::RequiresPair
                | TriggerCondition::RequiresFlush
                | TriggerCondition::MinCards(_) => {
                    // These should not trigger
                    assert_eq!(result.chips_added, 0);
                    assert_eq!(result.mult_added, 0.0);
                }
                TriggerCondition::Always => {
                    // These should still work
                    assert!(result.chips_added > 0 || result.mult_added > 0.0);
                }
                _ => {}
            }
        }
    }

    #[test]
    fn test_edge_case_max_cards() {
        // Edge case: maximum allowed cards
        let max_cards = vec![
            Card::new(Value::Ace, Suit::Spade),
            Card::new(Value::King, Suit::Spade),
            Card::new(Value::Queen, Suit::Spade),
            Card::new(Value::Jack, Suit::Spade),
            Card::new(Value::Ten, Suit::Spade),
            Card::new(Value::Nine, Suit::Spade),
            Card::new(Value::Eight, Suit::Spade),
            Card::new(Value::Seven, Suit::Spade),
        ];

        let mut fixture = TestFixture::new();
        let hand = SelectHand::new(max_cards.clone());
        let mut context = fixture.create_context(&max_cards, &[], &hand);

        // MinCards joker should definitely trigger
        let mut min_cards_joker = TEST_JOKERS[7];
        let result = min_cards_joker.process(&Stage::Blind(Blind::Small), &mut context);
        assert_eq!(result.chips_added, 15);
        assert_eq!(result.mult_added, 2.5);
    }

    #[test]
    fn test_stage_specific_behaviors() {
        // Test jokers behave differently in different stages
        let mut fixture = TestFixture::new();
        let hand = SelectHand::new(test_cards().to_vec());
        let stages = [
            Stage::Blind(Blind::Small),
            Stage::Blind(Blind::Big),
            Stage::Blind(Blind::Boss),
            Stage::Shop(),
            Stage::PreBlind(),
            Stage::PostBlind(),
        ];

        for stage in &stages {
            let mut context = fixture.create_context(test_cards(), &[], &hand);

            // Shop-specific joker
            let mut shop_joker = TEST_JOKERS[2];
            let result = shop_joker.process(stage, &mut context);

            if matches!(stage, Stage::Shop()) {
                assert_eq!(result.chips_added, 0);
                assert_eq!(result.mult_added, 1.0);
            } else {
                assert_eq!(result.chips_added, 0);
                assert_eq!(result.mult_added, 0.0);
            }
        }
    }
}
