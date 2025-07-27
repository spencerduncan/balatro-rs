//! Testing utilities for the Joker trait system.
//!
//! This module provides comprehensive testing infrastructure for joker implementations,
//! including mock jokers, test context builders, and assertion helpers.
//!
//! # Overview
//!
//! The testing utilities are organized around the different aspects of the Joker trait:
//! - **Identity**: Mock jokers for testing basic identity properties
//! - **Lifecycle**: Mock jokers for testing state lifecycle hooks
//! - **Gameplay**: Mock jokers for testing game event hooks
//! - **Modifiers**: Mock jokers for testing base value modification
//! - **State**: Mock jokers for testing serialization and validation
//!
//! # Usage Example
//!
//! ```rust
//! use balatro_rs::joker::test_utils::*;
//! use balatro_rs::joker::{Joker, JokerEffect, JokerId, JokerRarity};
//!
//! // Create a test context
//! let context = TestContextBuilder::new()
//!     .with_chips(100)
//!     .with_mult(5)
//!     .with_money(50)
//!     .build();
//!
//! // Use a mock joker for testing
//! let joker = MockGameplayJoker::new()
//!     .with_hand_effect(JokerEffect::new().with_mult(10))
//!     .with_card_effect(JokerEffect::new().with_chips(5));
//!
//! // Test the joker's behavior
//! let effect = joker.on_hand_played(&mut context, &hand);
//! assert_effect_mult(&effect, 10);
//! ```

use crate::card::{Card, Suit, Value};
use crate::hand::{Hand, SelectHand};
use crate::joker::{GameContext, Joker, JokerEffect, JokerId, JokerRarity};
use crate::joker_state::{JokerState, JokerStateManager};
use crate::rank::HandRank;
use crate::rng::GameRng;
use crate::stage::{Blind, Stage};
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::sync::Arc;

/// Mock joker for testing identity-related functionality.
///
/// This mock allows you to customize the basic identity properties
/// (id, name, description, rarity, cost) for testing purposes.
#[derive(Debug, Clone)]
pub struct MockIdentityJoker {
    pub id: JokerId,
    pub name: String,
    pub description: String,
    pub rarity: JokerRarity,
    pub cost: Option<usize>,
}

impl Default for MockIdentityJoker {
    fn default() -> Self {
        Self::new()
    }
}

impl MockIdentityJoker {
    /// Create a new mock identity joker with default values.
    pub fn new() -> Self {
        Self {
            id: JokerId::Joker,
            name: "Mock Joker".to_string(),
            description: "A mock joker for testing".to_string(),
            rarity: JokerRarity::Common,
            cost: None,
        }
    }

    /// Set the joker ID.
    pub fn with_id(mut self, id: JokerId) -> Self {
        self.id = id;
        self
    }

    /// Set the joker name.
    pub fn with_name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = name.into();
        self
    }

    /// Set the joker description.
    pub fn with_description<S: Into<String>>(mut self, description: S) -> Self {
        self.description = description.into();
        self
    }

    /// Set the joker rarity.
    pub fn with_rarity(mut self, rarity: JokerRarity) -> Self {
        self.rarity = rarity;
        self
    }

    /// Set a custom cost (overrides rarity-based pricing).
    pub fn with_cost(mut self, cost: usize) -> Self {
        self.cost = Some(cost);
        self
    }
}

impl Joker for MockIdentityJoker {
    fn id(&self) -> JokerId {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn rarity(&self) -> JokerRarity {
        self.rarity
    }

    fn cost(&self) -> usize {
        self.cost.unwrap_or(match self.rarity {
            JokerRarity::Common => 3,
            JokerRarity::Uncommon => 6,
            JokerRarity::Rare => 8,
            JokerRarity::Legendary => 20,
        })
    }
}

/// Mock joker for testing lifecycle-related functionality.
///
/// This mock allows you to specify effects for lifecycle events
/// (creation, activation, deactivation, cleanup).
#[derive(Debug, Clone)]
pub struct MockLifecycleJoker {
    pub id: JokerId,
    pub name: String,
    pub description: String,
    pub rarity: JokerRarity,
    pub on_created_effect: Option<JokerEffect>,
    pub on_activated_effect: Option<JokerEffect>,
    pub on_deactivated_effect: Option<JokerEffect>,
    pub on_cleanup_effect: Option<JokerEffect>,
}

impl Default for MockLifecycleJoker {
    fn default() -> Self {
        Self::new()
    }
}

impl MockLifecycleJoker {
    /// Create a new mock lifecycle joker.
    pub fn new() -> Self {
        Self {
            id: JokerId::Joker,
            name: "Mock Lifecycle Joker".to_string(),
            description: "A mock joker for testing lifecycle events".to_string(),
            rarity: JokerRarity::Common,
            on_created_effect: None,
            on_activated_effect: None,
            on_deactivated_effect: None,
            on_cleanup_effect: None,
        }
    }

    /// Set the effect for on_created.
    pub fn with_created_effect(mut self, effect: JokerEffect) -> Self {
        self.on_created_effect = Some(effect);
        self
    }

    /// Set the effect for on_activated.
    pub fn with_activated_effect(mut self, effect: JokerEffect) -> Self {
        self.on_activated_effect = Some(effect);
        self
    }

    /// Set the effect for on_deactivated.
    pub fn with_deactivated_effect(mut self, effect: JokerEffect) -> Self {
        self.on_deactivated_effect = Some(effect);
        self
    }

    /// Set the effect for on_cleanup.
    pub fn with_cleanup_effect(mut self, effect: JokerEffect) -> Self {
        self.on_cleanup_effect = Some(effect);
        self
    }
}

impl Joker for MockLifecycleJoker {
    fn id(&self) -> JokerId {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn rarity(&self) -> JokerRarity {
        self.rarity
    }

    fn on_created(&self, _context: &mut GameContext) -> JokerEffect {
        self.on_created_effect.clone().unwrap_or_default()
    }

    fn on_activated(&self, _context: &mut GameContext) -> JokerEffect {
        self.on_activated_effect.clone().unwrap_or_default()
    }

    fn on_deactivated(&self, _context: &mut GameContext) -> JokerEffect {
        self.on_deactivated_effect.clone().unwrap_or_default()
    }

    fn on_cleanup(&self, _context: &mut GameContext) -> JokerEffect {
        self.on_cleanup_effect.clone().unwrap_or_default()
    }
}

/// Mock joker for testing gameplay-related functionality.
///
/// This mock allows you to specify effects for various gameplay events
/// (hand played, card scored, blind start, etc.).
#[derive(Debug, Clone)]
pub struct MockGameplayJoker {
    pub id: JokerId,
    pub name: String,
    pub description: String,
    pub rarity: JokerRarity,
    pub on_hand_played_effect: Option<JokerEffect>,
    pub on_card_scored_effect: Option<JokerEffect>,
    pub on_blind_start_effect: Option<JokerEffect>,
    pub on_shop_open_effect: Option<JokerEffect>,
    pub on_discard_effect: Option<JokerEffect>,
    pub on_round_end_effect: Option<JokerEffect>,
}

impl Default for MockGameplayJoker {
    fn default() -> Self {
        Self::new()
    }
}

impl MockGameplayJoker {
    /// Create a new mock gameplay joker.
    pub fn new() -> Self {
        Self {
            id: JokerId::Joker,
            name: "Mock Gameplay Joker".to_string(),
            description: "A mock joker for testing gameplay events".to_string(),
            rarity: JokerRarity::Common,
            on_hand_played_effect: None,
            on_card_scored_effect: None,
            on_blind_start_effect: None,
            on_shop_open_effect: None,
            on_discard_effect: None,
            on_round_end_effect: None,
        }
    }

    /// Set the effect for on_hand_played.
    pub fn with_hand_effect(mut self, effect: JokerEffect) -> Self {
        self.on_hand_played_effect = Some(effect);
        self
    }

    /// Set the effect for on_card_scored.
    pub fn with_card_effect(mut self, effect: JokerEffect) -> Self {
        self.on_card_scored_effect = Some(effect);
        self
    }

    /// Set the effect for on_blind_start.
    pub fn with_blind_start_effect(mut self, effect: JokerEffect) -> Self {
        self.on_blind_start_effect = Some(effect);
        self
    }

    /// Set the effect for on_shop_open.
    pub fn with_shop_open_effect(mut self, effect: JokerEffect) -> Self {
        self.on_shop_open_effect = Some(effect);
        self
    }

    /// Set the effect for on_discard.
    pub fn with_discard_effect(mut self, effect: JokerEffect) -> Self {
        self.on_discard_effect = Some(effect);
        self
    }

    /// Set the effect for on_round_end.
    pub fn with_round_end_effect(mut self, effect: JokerEffect) -> Self {
        self.on_round_end_effect = Some(effect);
        self
    }
}

impl Joker for MockGameplayJoker {
    fn id(&self) -> JokerId {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn rarity(&self) -> JokerRarity {
        self.rarity
    }

    fn on_hand_played(&self, _context: &mut GameContext, _hand: &SelectHand) -> JokerEffect {
        self.on_hand_played_effect.clone().unwrap_or_default()
    }

    fn on_card_scored(&self, _context: &mut GameContext, _card: &Card) -> JokerEffect {
        self.on_card_scored_effect.clone().unwrap_or_default()
    }

    fn on_blind_start(&self, _context: &mut GameContext) -> JokerEffect {
        self.on_blind_start_effect.clone().unwrap_or_default()
    }

    fn on_shop_open(&self, _context: &mut GameContext) -> JokerEffect {
        self.on_shop_open_effect.clone().unwrap_or_default()
    }

    fn on_discard(&self, _context: &mut GameContext, _cards: &[Card]) -> JokerEffect {
        self.on_discard_effect.clone().unwrap_or_default()
    }

    fn on_round_end(&self, _context: &mut GameContext) -> JokerEffect {
        self.on_round_end_effect.clone().unwrap_or_default()
    }
}

/// Mock joker for testing modifier functionality.
///
/// This mock allows you to specify custom modifiers for base game values
/// (chips, mult, hand size, discards).
pub struct MockModifierJoker {
    pub id: JokerId,
    pub name: String,
    pub description: String,
    pub rarity: JokerRarity,
    pub chips_modifier: Option<Box<dyn Fn(i32) -> i32 + Send + Sync>>,
    pub mult_modifier: Option<Box<dyn Fn(i32) -> i32 + Send + Sync>>,
    pub hand_size_modifier: Option<Box<dyn Fn(usize) -> usize + Send + Sync>>,
    pub discards_modifier: Option<Box<dyn Fn(usize) -> usize + Send + Sync>>,
}

impl std::fmt::Debug for MockModifierJoker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MockModifierJoker")
            .field("id", &self.id)
            .field("name", &self.name)
            .field("description", &self.description)
            .field("rarity", &self.rarity)
            .field("chips_modifier", &self.chips_modifier.is_some())
            .field("mult_modifier", &self.mult_modifier.is_some())
            .field("hand_size_modifier", &self.hand_size_modifier.is_some())
            .field("discards_modifier", &self.discards_modifier.is_some())
            .finish()
    }
}

impl Default for MockModifierJoker {
    fn default() -> Self {
        Self::new()
    }
}

impl MockModifierJoker {
    /// Create a new mock modifier joker.
    pub fn new() -> Self {
        Self {
            id: JokerId::Joker,
            name: "Mock Modifier Joker".to_string(),
            description: "A mock joker for testing modifiers".to_string(),
            rarity: JokerRarity::Common,
            chips_modifier: None,
            mult_modifier: None,
            hand_size_modifier: None,
            discards_modifier: None,
        }
    }

    /// Set a chips modifier function.
    pub fn with_chips_modifier<F>(mut self, modifier: F) -> Self
    where
        F: Fn(i32) -> i32 + Send + Sync + 'static,
    {
        self.chips_modifier = Some(Box::new(modifier));
        self
    }

    /// Set a mult modifier function.
    pub fn with_mult_modifier<F>(mut self, modifier: F) -> Self
    where
        F: Fn(i32) -> i32 + Send + Sync + 'static,
    {
        self.mult_modifier = Some(Box::new(modifier));
        self
    }

    /// Set a hand size modifier function.
    pub fn with_hand_size_modifier<F>(mut self, modifier: F) -> Self
    where
        F: Fn(usize) -> usize + Send + Sync + 'static,
    {
        self.hand_size_modifier = Some(Box::new(modifier));
        self
    }

    /// Set a discards modifier function.
    pub fn with_discards_modifier<F>(mut self, modifier: F) -> Self
    where
        F: Fn(usize) -> usize + Send + Sync + 'static,
    {
        self.discards_modifier = Some(Box::new(modifier));
        self
    }
}

impl Joker for MockModifierJoker {
    fn id(&self) -> JokerId {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn rarity(&self) -> JokerRarity {
        self.rarity
    }

    fn modify_chips(&self, _context: &GameContext, base_chips: i32) -> i32 {
        if let Some(ref modifier) = self.chips_modifier {
            modifier(base_chips)
        } else {
            base_chips
        }
    }

    fn modify_mult(&self, _context: &GameContext, base_mult: i32) -> i32 {
        if let Some(ref modifier) = self.mult_modifier {
            modifier(base_mult)
        } else {
            base_mult
        }
    }

    fn modify_hand_size(&self, _context: &GameContext, base_size: usize) -> usize {
        if let Some(ref modifier) = self.hand_size_modifier {
            modifier(base_size)
        } else {
            base_size
        }
    }

    fn modify_discards(&self, _context: &GameContext, base_discards: usize) -> usize {
        if let Some(ref modifier) = self.discards_modifier {
            modifier(base_discards)
        } else {
            base_discards
        }
    }
}

/// Mock joker for testing state serialization functionality.
///
/// This mock allows you to specify custom serialization, deserialization,
/// validation, and state initialization behavior.
#[derive(Debug)]
pub struct MockStateJoker {
    pub id: JokerId,
    pub name: String,
    pub description: String,
    pub rarity: JokerRarity,
    pub custom_serialization: bool,
    pub custom_deserialization: bool,
    pub validation_should_fail: bool,
    pub initial_state: Option<JokerState>,
}

impl Default for MockStateJoker {
    fn default() -> Self {
        Self::new()
    }
}

impl MockStateJoker {
    /// Create a new mock state joker.
    pub fn new() -> Self {
        Self {
            id: JokerId::Joker,
            name: "Mock State Joker".to_string(),
            description: "A mock joker for testing state operations".to_string(),
            rarity: JokerRarity::Common,
            custom_serialization: false,
            custom_deserialization: false,
            validation_should_fail: false,
            initial_state: None,
        }
    }

    /// Enable custom serialization behavior.
    pub fn with_custom_serialization(mut self) -> Self {
        self.custom_serialization = true;
        self
    }

    /// Enable custom deserialization behavior.
    pub fn with_custom_deserialization(mut self) -> Self {
        self.custom_deserialization = true;
        self
    }

    /// Make validation fail for testing error cases.
    pub fn with_failing_validation(mut self) -> Self {
        self.validation_should_fail = true;
        self
    }

    /// Set a custom initial state.
    pub fn with_initial_state(mut self, state: JokerState) -> Self {
        self.initial_state = Some(state);
        self
    }
}

impl Joker for MockStateJoker {
    fn id(&self) -> JokerId {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn rarity(&self) -> JokerRarity {
        self.rarity
    }

    fn serialize_state(
        &self,
        _context: &GameContext,
        state: &JokerState,
    ) -> Result<JsonValue, serde_json::Error> {
        if self.custom_serialization {
            let mut custom_value = serde_json::to_value(state)?;
            custom_value["custom_serialization"] = JsonValue::Bool(true);
            Ok(custom_value)
        } else {
            serde_json::to_value(state)
        }
    }

    fn deserialize_state(
        &self,
        _context: &GameContext,
        data: &JsonValue,
    ) -> Result<JokerState, serde_json::Error> {
        if self.custom_deserialization {
            let mut state: JokerState = serde_json::from_value(data.clone())?;
            // Modify the state to indicate custom deserialization occurred
            state.accumulated_value += 1.0;
            Ok(state)
        } else {
            serde_json::from_value(data.clone())
        }
    }

    fn validate_state(&self, _context: &GameContext, _state: &JokerState) -> Result<(), String> {
        if self.validation_should_fail {
            Err("Validation failed as requested".to_string())
        } else {
            Ok(())
        }
    }

    fn initialize_state(&self, _context: &GameContext) -> JokerState {
        self.initial_state.clone().unwrap_or_default()
    }
}

/// Builder for creating test GameContext instances.
///
/// This builder makes it easy to create GameContext instances with specific
/// values for testing, without needing to set up a full game state.
pub struct TestContextBuilder {
    chips: i32,
    mult: i32,
    money: i32,
    ante: u8,
    round: u32,
    stage: Stage,
    hands_played: u32,
    discards_used: u32,
    hand: Hand,
    discarded: Vec<Card>,
    hand_type_counts: HashMap<HandRank, u32>,
    cards_in_deck: usize,
    stone_cards_in_deck: usize,
}

impl TestContextBuilder {
    /// Create a new test context builder with default values.
    pub fn new() -> Self {
        Self {
            chips: 10,
            mult: 1,
            money: 5,
            ante: 1,
            round: 1,
            stage: Stage::Blind(Blind::Small),
            hands_played: 0,
            discards_used: 0,
            hand: Hand::new(vec![]),
            discarded: Vec::new(),
            hand_type_counts: HashMap::new(),
            cards_in_deck: 52,
            stone_cards_in_deck: 0,
        }
    }

    /// Set the chips value.
    pub fn with_chips(mut self, chips: i32) -> Self {
        self.chips = chips;
        self
    }

    /// Set the mult value.
    pub fn with_mult(mut self, mult: i32) -> Self {
        self.mult = mult;
        self
    }

    /// Set the money value.
    pub fn with_money(mut self, money: i32) -> Self {
        self.money = money;
        self
    }

    /// Set the ante.
    pub fn with_ante(mut self, ante: u8) -> Self {
        self.ante = ante;
        self
    }

    /// Set the round.
    pub fn with_round(mut self, round: u32) -> Self {
        self.round = round;
        self
    }

    /// Set the stage.
    pub fn with_stage(mut self, stage: Stage) -> Self {
        self.stage = stage;
        self
    }

    /// Set the number of hands played.
    pub fn with_hands_played(mut self, hands_played: u32) -> Self {
        self.hands_played = hands_played;
        self
    }

    /// Set the number of discards used.
    pub fn with_discards_used(mut self, discards_used: u32) -> Self {
        self.discards_used = discards_used;
        self
    }

    /// Set the hand.
    pub fn with_hand(mut self, hand: Hand) -> Self {
        self.hand = hand;
        self
    }

    /// Set discarded cards.
    pub fn with_discarded(mut self, discarded: Vec<Card>) -> Self {
        self.discarded = discarded;
        self
    }

    /// Add a hand type count.
    pub fn with_hand_type_count(mut self, hand_rank: HandRank, count: u32) -> Self {
        self.hand_type_counts.insert(hand_rank, count);
        self
    }

    /// Set the number of cards in deck.
    pub fn with_cards_in_deck(mut self, count: usize) -> Self {
        self.cards_in_deck = count;
        self
    }

    /// Set the number of stone cards in deck.
    pub fn with_stone_cards_in_deck(mut self, count: usize) -> Self {
        self.stone_cards_in_deck = count;
        self
    }

    /// Build the GameContext.
    ///
    /// Note: This creates a minimal context suitable for testing.
    /// Some fields like jokers and joker_state_manager are created with
    /// minimal implementations.
    pub fn build(self) -> GameContext<'static> {
        let jokers: Vec<Box<dyn Joker>> = Vec::new();
        let joker_state_manager = Arc::new(JokerStateManager::new());
        let rng = GameRng::for_testing(42);

        // Convert to static references (this is unsafe but okay for tests)
        let stage_ref: &'static Stage = Box::leak(Box::new(self.stage));
        let jokers_ref: &'static [Box<dyn Joker>] = Box::leak(jokers.into_boxed_slice());
        let hand_ref: &'static Hand = Box::leak(Box::new(self.hand));
        let discarded_ref: &'static [Card] = Box::leak(self.discarded.into_boxed_slice());
        let hand_type_counts_ref: &'static HashMap<HandRank, u32> =
            Box::leak(Box::new(self.hand_type_counts));
        let rng_ref: &'static GameRng = Box::leak(Box::new(rng));

        GameContext {
            chips: self.chips,
            mult: self.mult,
            money: self.money,
            ante: self.ante,
            round: self.round,
            stage: stage_ref,
            hands_played: self.hands_played,
            discards_used: self.discards_used,
            jokers: jokers_ref,
            hand: hand_ref,
            discarded: discarded_ref,
            joker_state_manager: Box::leak(Box::new(joker_state_manager)),
            hand_type_counts: hand_type_counts_ref,
            cards_in_deck: self.cards_in_deck,
            stone_cards_in_deck: self.stone_cards_in_deck,
            steel_cards_in_deck: 0,
            rng: rng_ref,
        }
    }
}

impl Default for TestContextBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// Assertion helpers for JokerEffect validation

/// Assert that a JokerEffect has the expected chips value.
pub fn assert_effect_chips(effect: &JokerEffect, expected: i32) {
    assert_eq!(
        effect.chips, expected,
        "Expected chips: {}, got: {}",
        expected, effect.chips
    );
}

/// Assert that a JokerEffect has the expected mult value.
pub fn assert_effect_mult(effect: &JokerEffect, expected: i32) {
    assert_eq!(
        effect.mult, expected,
        "Expected mult: {}, got: {}",
        expected, effect.mult
    );
}

/// Assert that a JokerEffect has the expected money value.
pub fn assert_effect_money(effect: &JokerEffect, expected: i32) {
    assert_eq!(
        effect.money, expected,
        "Expected money: {}, got: {}",
        expected, effect.money
    );
}

/// Assert that a JokerEffect has the expected mult multiplier value.
pub fn assert_effect_mult_multiplier(effect: &JokerEffect, expected: f64) {
    assert!(
        (effect.mult_multiplier - expected).abs() < f64::EPSILON,
        "Expected mult_multiplier: {}, got: {}",
        expected,
        effect.mult_multiplier
    );
}

/// Assert that a JokerEffect has the expected retrigger count.
pub fn assert_effect_retrigger(effect: &JokerEffect, expected: u32) {
    assert_eq!(
        effect.retrigger, expected,
        "Expected retrigger: {}, got: {}",
        expected, effect.retrigger
    );
}

/// Assert that a JokerEffect has the expected destroy_self value.
pub fn assert_effect_destroy_self(effect: &JokerEffect, expected: bool) {
    assert_eq!(
        effect.destroy_self, expected,
        "Expected destroy_self: {}, got: {}",
        expected, effect.destroy_self
    );
}

/// Assert that a JokerEffect has the expected message.
pub fn assert_effect_message(effect: &JokerEffect, expected: Option<&str>) {
    match (effect.message.as_deref(), expected) {
        (Some(actual), Some(expected)) => assert_eq!(
            actual, expected,
            "Expected message: '{expected}', got: '{actual}'"
        ),
        (None, None) => (),
        (Some(actual), None) => panic!("Expected no message, got: '{actual}'"),
        (None, Some(expected)) => panic!("Expected message: '{expected}', got: None"),
    }
}

/// Assert that a JokerEffect is empty (all default values).
pub fn assert_effect_empty(effect: &JokerEffect) {
    let default_effect = JokerEffect::new();
    assert_eq!(effect.chips, default_effect.chips);
    assert_eq!(effect.mult, default_effect.mult);
    assert_eq!(effect.money, default_effect.money);
    assert!((effect.mult_multiplier - default_effect.mult_multiplier).abs() < f64::EPSILON);
    assert_eq!(effect.retrigger, default_effect.retrigger);
    assert_eq!(effect.destroy_self, default_effect.destroy_self);
    assert_eq!(effect.destroy_others, default_effect.destroy_others);
    assert_eq!(effect.transform_cards, default_effect.transform_cards);
    assert_eq!(effect.hand_size_mod, default_effect.hand_size_mod);
    assert_eq!(effect.discard_mod, default_effect.discard_mod);
    assert_eq!(
        effect.sell_value_increase,
        default_effect.sell_value_increase
    );
    assert_eq!(effect.message, default_effect.message);
}

/// Create a simple test card for testing purposes.
pub fn create_test_card(rank: Value, suit: Suit) -> Card {
    Card::new(rank, suit)
}

/// Create a simple test hand with specified cards.
pub fn create_test_hand(cards: Vec<Card>) -> Hand {
    Hand::new(cards)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::{Suit, Value};
    use crate::hand::SelectHand;
    use crate::joker_state::JokerState;
    use crate::rank::HandRank;
    use crate::stage::Stage;

    #[test]
    fn test_mock_identity_joker() {
        let joker = MockIdentityJoker::new()
            .with_id(JokerId::GreedyJoker)
            .with_name("Test Joker")
            .with_description("A test joker")
            .with_rarity(JokerRarity::Rare)
            .with_cost(15);

        assert_eq!(joker.id(), JokerId::GreedyJoker);
        assert_eq!(joker.name(), "Test Joker");
        assert_eq!(joker.description(), "A test joker");
        assert_eq!(joker.rarity(), JokerRarity::Rare);
        assert_eq!(joker.cost(), 15);
    }

    #[test]
    fn test_mock_identity_joker_default_cost() {
        let joker = MockIdentityJoker::new().with_rarity(JokerRarity::Legendary);
        assert_eq!(joker.cost(), 20); // Default legendary cost
    }

    #[test]
    fn test_mock_lifecycle_joker() {
        let created_effect = JokerEffect::new().with_chips(10);
        let activated_effect = JokerEffect::new().with_mult(5);
        let deactivated_effect = JokerEffect::new().with_money(3);
        let cleanup_effect = JokerEffect::new().with_message("Goodbye!".to_string());

        let joker = MockLifecycleJoker::new()
            .with_created_effect(created_effect.clone())
            .with_activated_effect(activated_effect.clone())
            .with_deactivated_effect(deactivated_effect.clone())
            .with_cleanup_effect(cleanup_effect.clone());

        let mut context = TestContextBuilder::new().build();

        let effect = joker.on_created(&mut context);
        assert_effect_chips(&effect, 10);

        let effect = joker.on_activated(&mut context);
        assert_effect_mult(&effect, 5);

        let effect = joker.on_deactivated(&mut context);
        assert_effect_money(&effect, 3);

        let effect = joker.on_cleanup(&mut context);
        assert_effect_message(&effect, Some("Goodbye!"));
    }

    #[test]
    fn test_mock_gameplay_joker() {
        let hand_effect = JokerEffect::new().with_mult(10);
        let card_effect = JokerEffect::new().with_chips(5);
        let blind_effect = JokerEffect::new().with_money(2);

        let joker = MockGameplayJoker::new()
            .with_hand_effect(hand_effect.clone())
            .with_card_effect(card_effect.clone())
            .with_blind_start_effect(blind_effect.clone());

        let mut context = TestContextBuilder::new().build();
        let test_card = create_test_card(Value::Ace, Suit::Spade);
        let hand = SelectHand::new(vec![test_card]);

        let effect = joker.on_hand_played(&mut context, &hand);
        assert_effect_mult(&effect, 10);

        let effect = joker.on_card_scored(&mut context, &test_card);
        assert_effect_chips(&effect, 5);

        let effect = joker.on_blind_start(&mut context);
        assert_effect_money(&effect, 2);
    }

    #[test]
    fn test_mock_modifier_joker() {
        let joker = MockModifierJoker::new()
            .with_chips_modifier(|chips| chips + 50)
            .with_mult_modifier(|mult| mult * 2)
            .with_hand_size_modifier(|size| size + 1)
            .with_discards_modifier(|discards| discards + 2);

        let context = TestContextBuilder::new().build();

        assert_eq!(joker.modify_chips(&context, 100), 150);
        assert_eq!(joker.modify_mult(&context, 5), 10);
        assert_eq!(joker.modify_hand_size(&context, 8), 9);
        assert_eq!(joker.modify_discards(&context, 3), 5);
    }

    #[test]
    fn test_mock_state_joker_serialization() {
        let joker = MockStateJoker::new().with_custom_serialization();
        let context = TestContextBuilder::new().build();
        let state = JokerState::new();

        let serialized = joker.serialize_state(&context, &state).unwrap();
        assert_eq!(serialized["custom_serialization"], JsonValue::Bool(true));
    }

    #[test]
    fn test_mock_state_joker_deserialization() {
        let joker = MockStateJoker::new().with_custom_deserialization();
        let context = TestContextBuilder::new().build();

        let state_data = serde_json::to_value(JokerState::new()).unwrap();
        let original_value = state_data["accumulated_value"].as_f64().unwrap_or(0.0);

        let deserialized = joker.deserialize_state(&context, &state_data).unwrap();
        assert_eq!(deserialized.accumulated_value, original_value + 1.0);
    }

    #[test]
    fn test_mock_state_joker_validation_success() {
        let joker = MockStateJoker::new();
        let context = TestContextBuilder::new().build();
        let state = JokerState::new();

        assert!(joker.validate_state(&context, &state).is_ok());
    }

    #[test]
    fn test_mock_state_joker_validation_failure() {
        let joker = MockStateJoker::new().with_failing_validation();
        let context = TestContextBuilder::new().build();
        let state = JokerState::new();

        assert!(joker.validate_state(&context, &state).is_err());
    }

    #[test]
    fn test_mock_state_joker_initial_state() {
        let mut initial_state = JokerState::new();
        initial_state.accumulated_value = 42.0;

        let joker = MockStateJoker::new().with_initial_state(initial_state.clone());
        let context = TestContextBuilder::new().build();

        let created_state = joker.initialize_state(&context);
        assert_eq!(created_state.accumulated_value, 42.0);
    }

    #[test]
    fn test_test_context_builder() {
        let context = TestContextBuilder::new()
            .with_chips(200)
            .with_mult(10)
            .with_money(100)
            .with_ante(5)
            .with_round(12)
            .with_stage(Stage::Shop())
            .with_hands_played(3)
            .with_discards_used(2)
            .with_hand_type_count(HandRank::OnePair, 5)
            .with_cards_in_deck(40)
            .with_stone_cards_in_deck(2)
            .build();

        assert_eq!(context.chips, 200);
        assert_eq!(context.mult, 10);
        assert_eq!(context.money, 100);
        assert_eq!(context.ante, 5);
        assert_eq!(context.round, 12);
        assert_eq!(*context.stage, Stage::Shop());
        assert_eq!(context.hands_played, 3);
        assert_eq!(context.discards_used, 2);
        assert_eq!(context.get_hand_type_count(HandRank::OnePair), 5);
        assert_eq!(context.cards_in_deck, 40);
        assert_eq!(context.stone_cards_in_deck, 2);
    }

    #[test]
    fn test_assertion_helpers() {
        let effect = JokerEffect::new()
            .with_chips(50)
            .with_mult(8)
            .with_money(12)
            .with_mult_multiplier(1.5)
            .with_retrigger(2)
            .with_message("Test message".to_string());

        assert_effect_chips(&effect, 50);
        assert_effect_mult(&effect, 8);
        assert_effect_money(&effect, 12);
        assert_effect_mult_multiplier(&effect, 1.5);
        assert_effect_retrigger(&effect, 2);
        assert_effect_destroy_self(&effect, false);
        assert_effect_message(&effect, Some("Test message"));
    }

    #[test]
    fn test_assert_effect_empty() {
        let empty_effect = JokerEffect::new();
        assert_effect_empty(&empty_effect);
    }

    #[test]
    #[should_panic(expected = "Expected chips: 10, got: 0")]
    fn test_assertion_failure_chips() {
        let effect = JokerEffect::new();
        assert_effect_chips(&effect, 10);
    }

    #[test]
    #[should_panic(expected = "Expected mult: 5, got: 0")]
    fn test_assertion_failure_mult() {
        let effect = JokerEffect::new();
        assert_effect_mult(&effect, 5);
    }

    #[test]
    #[should_panic(expected = "Expected money: 3, got: 0")]
    fn test_assertion_failure_money() {
        let effect = JokerEffect::new();
        assert_effect_money(&effect, 3);
    }

    #[test]
    fn test_create_test_card() {
        let card = create_test_card(Value::King, Suit::Heart);
        assert_eq!(card.value, Value::King);
        assert_eq!(card.suit, Suit::Heart);
    }

    #[test]
    fn test_create_test_hand() {
        let cards = vec![
            create_test_card(Value::Ace, Suit::Spade),
            create_test_card(Value::King, Suit::Heart),
            create_test_card(Value::Queen, Suit::Diamond),
        ];

        let hand = create_test_hand(cards.clone());
        assert_eq!(hand.cards().len(), 3);

        // Verify the cards are in the hand
        let hand_cards = hand.cards();
        assert!(hand_cards.contains(&cards[0]));
        assert!(hand_cards.contains(&cards[1]));
        assert!(hand_cards.contains(&cards[2]));
    }

    #[test]
    fn test_complex_joker_interaction() {
        // Test a complex scenario combining multiple mock jokers
        let mut context = TestContextBuilder::new()
            .with_chips(100)
            .with_mult(5)
            .with_money(50)
            .build();

        // Create a joker that provides bonus for specific cards
        let gameplay_joker =
            MockGameplayJoker::new().with_card_effect(JokerEffect::new().with_mult(3));

        // Create a modifier joker that doubles chips
        let modifier_joker = MockModifierJoker::new().with_chips_modifier(|chips| chips * 2);

        // Test the interaction
        let test_card = create_test_card(Value::Ace, Suit::Spade);
        let card_effect = gameplay_joker.on_card_scored(&mut context, &test_card);
        assert_effect_mult(&card_effect, 3);

        let modified_chips = modifier_joker.modify_chips(&context, 100);
        assert_eq!(modified_chips, 200);
    }

    #[test]
    fn test_lifecycle_and_state_interaction() {
        // Test the interaction between lifecycle and state jokers
        let mut initial_state = JokerState::new();
        initial_state.accumulated_value = 10.0;

        let state_joker = MockStateJoker::new().with_initial_state(initial_state.clone());

        let lifecycle_joker = MockLifecycleJoker::new()
            .with_created_effect(JokerEffect::new().with_money(5))
            .with_cleanup_effect(JokerEffect::new().with_message("Cleanup complete".to_string()));

        let mut context = TestContextBuilder::new().build();

        // Test state initialization
        let created_state = state_joker.initialize_state(&context);
        assert_eq!(created_state.accumulated_value, 10.0);

        // Test lifecycle events
        let created_effect = lifecycle_joker.on_created(&mut context);
        assert_effect_money(&created_effect, 5);

        let cleanup_effect = lifecycle_joker.on_cleanup(&mut context);
        assert_effect_message(&cleanup_effect, Some("Cleanup complete"));
    }
}
