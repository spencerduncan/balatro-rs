//! # Test Joker Mock Implementations
//!
//! This module provides comprehensive Test*Joker mock implementations for effect-based testing.
//! These test jokers are designed to isolate and validate specific effect types in the joker system.
//!
//! ## Available Test Jokers
//!
//! - **TestChipsJoker**: Tests chips-based effects with configurable chip bonuses
//! - **TestMultJoker**: Tests mult-based effects with configurable mult bonuses
//! - **TestXMultJoker**: Tests multiplicative effects with configurable mult multipliers
//! - **TestMoneyJoker**: Tests economic effects with configurable money and interest bonuses
//! - **TestRetriggerJoker**: Tests retrigger mechanics with configurable retrigger counts
//! - **TestSpecialJoker**: Tests complex effects including card transformation and destruction
//! - **TestScalingJoker**: Tests scaling effects that change based on game state
//!
//! ## Usage Examples
//!
//! ```rust
//! use balatro_rs::joker::test_jokers::*;
//!
//! // Test a simple chips bonus
//! let chips_joker = TestChipsJoker::new(25);
//!
//! // Test multiplicative effects
//! let xmult_joker = TestXMultJoker::new(1.5); // 50% mult multiplier
//!
//! // Test complex scaling
//! let scaling_joker = TestScalingJoker::new()
//!     .with_base_chips(10)
//!     .with_scaling_factor(2.0)
//!     .with_scaling_trigger(ScalingTrigger::HandsPlayed);
//! ```
//!
//! ## Performance Considerations
//!
//! These test jokers are optimized for fast execution in test environments:
//! - Minimal memory allocations
//! - Fast trait method implementations
//! - Cache-friendly data structures
//! - SIMD-friendly numerical operations where applicable

use crate::card::Card;
use crate::hand::SelectHand;
use crate::joker::{GameContext, Joker, JokerEffect, JokerId, JokerRarity};

// ================================================================================================
// TestChipsJoker - For chips-based effect testing
// ================================================================================================

/// Test joker for validating chips-based effects.
///
/// This joker provides configurable chip bonuses and can be used to test:
/// - Basic chip addition mechanics
/// - Conditional chip bonuses
/// - Hand-based vs card-based chip effects
/// - Chip scaling over time
#[derive(Debug, Clone)]
pub struct TestChipsJoker {
    /// Base chip bonus per activation
    pub chips: i32,
    /// Chip bonus per card scored (for card-based effects)
    pub chips_per_card: i32,
    /// Whether the joker is active (for conditional testing)
    pub active: bool,
    /// Custom joker ID (defaults to Reserved)
    pub joker_id: JokerId,
    /// Display name override
    pub name_override: Option<String>,
}

impl TestChipsJoker {
    /// Create a new TestChipsJoker with the specified chip bonus.
    pub fn new(chips: i32) -> Self {
        Self {
            chips,
            chips_per_card: 0,
            active: true,
            joker_id: JokerId::Reserved,
            name_override: None,
        }
    }

    /// Set chips per card for card-based effects.
    pub fn with_chips_per_card(mut self, chips_per_card: i32) -> Self {
        self.chips_per_card = chips_per_card;
        self
    }

    /// Set active state (for conditional testing).
    pub fn with_active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    /// Set custom joker ID.
    pub fn with_id(mut self, id: JokerId) -> Self {
        self.joker_id = id;
        self
    }

    /// Set custom display name.
    pub fn with_name(mut self, name: String) -> Self {
        self.name_override = Some(name);
        self
    }

    /// Create a scaling chips joker that increases over time.
    pub fn scaling(base_chips: i32, scaling_factor: f64) -> TestScalingChipsJoker {
        TestScalingChipsJoker {
            base_chips,
            scaling_factor,
            current_chips: base_chips,
        }
    }
}

impl Joker for TestChipsJoker {
    fn id(&self) -> JokerId {
        self.joker_id
    }

    fn name(&self) -> &str {
        self.name_override.as_deref().unwrap_or("Test Chips Joker")
    }

    fn description(&self) -> &str {
        "Test joker for chips-based effects"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Common
    }

    fn cost(&self) -> usize {
        3
    }

    fn on_hand_played(&self, _context: &mut GameContext, _hand: &SelectHand) -> JokerEffect {
        if self.active {
            JokerEffect::new().with_chips(self.chips)
        } else {
            JokerEffect::new()
        }
    }

    fn on_card_scored(&self, _context: &mut GameContext, _card: &Card) -> JokerEffect {
        if self.active && self.chips_per_card != 0 {
            JokerEffect::new().with_chips(self.chips_per_card)
        } else {
            JokerEffect::new()
        }
    }
}

// ================================================================================================
// TestMultJoker - For mult-based effect testing
// ================================================================================================

/// Test joker for validating mult-based effects.
///
/// This joker provides configurable mult bonuses and can be used to test:
/// - Basic mult addition mechanics
/// - Conditional mult bonuses
/// - Hand-based vs card-based mult effects
/// - Mult scaling over time
#[derive(Debug, Clone)]
pub struct TestMultJoker {
    /// Base mult bonus per activation
    pub mult: i32,
    /// Mult bonus per card scored (for card-based effects)
    pub mult_per_card: i32,
    /// Whether the joker is active (for conditional testing)
    pub active: bool,
    /// Custom joker ID (defaults to Reserved)
    pub joker_id: JokerId,
    /// Display name override
    pub name_override: Option<String>,
}

impl TestMultJoker {
    /// Create a new TestMultJoker with the specified mult bonus.
    pub fn new(mult: i32) -> Self {
        Self {
            mult,
            mult_per_card: 0,
            active: true,
            joker_id: JokerId::Reserved,
            name_override: None,
        }
    }

    /// Set mult per card for card-based effects.
    pub fn with_mult_per_card(mut self, mult_per_card: i32) -> Self {
        self.mult_per_card = mult_per_card;
        self
    }

    /// Set active state (for conditional testing).
    pub fn with_active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    /// Set custom joker ID.
    pub fn with_id(mut self, id: JokerId) -> Self {
        self.joker_id = id;
        self
    }

    /// Set custom display name.
    pub fn with_name(mut self, name: String) -> Self {
        self.name_override = Some(name);
        self
    }
}

impl Joker for TestMultJoker {
    fn id(&self) -> JokerId {
        self.joker_id
    }

    fn name(&self) -> &str {
        self.name_override.as_deref().unwrap_or("Test Mult Joker")
    }

    fn description(&self) -> &str {
        "Test joker for mult-based effects"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Common
    }

    fn cost(&self) -> usize {
        4
    }

    fn on_hand_played(&self, _context: &mut GameContext, _hand: &SelectHand) -> JokerEffect {
        if self.active {
            JokerEffect::new().with_mult(self.mult)
        } else {
            JokerEffect::new()
        }
    }

    fn on_card_scored(&self, _context: &mut GameContext, _card: &Card) -> JokerEffect {
        if self.active && self.mult_per_card != 0 {
            JokerEffect::new().with_mult(self.mult_per_card)
        } else {
            JokerEffect::new()
        }
    }
}

// ================================================================================================
// TestXMultJoker - For multiplicative effect testing
// ================================================================================================

/// Test joker for validating multiplicative effects.
///
/// This joker provides configurable mult multipliers and can be used to test:
/// - Mult multiplier mechanics
/// - Conditional multiplicative bonuses
/// - Stacking of multiple multipliers
/// - Performance of multiplicative calculations
#[derive(Debug, Clone)]
pub struct TestXMultJoker {
    /// Mult multiplier (1.0 = no change, 2.0 = double mult)
    pub mult_multiplier: f64,
    /// Whether the joker is active (for conditional testing)
    pub active: bool,
    /// Custom joker ID (defaults to Reserved)
    pub joker_id: JokerId,
    /// Display name override
    pub name_override: Option<String>,
    /// Whether to apply on hand played (default) or card scored
    pub applies_to_cards: bool,
}

impl TestXMultJoker {
    /// Create a new TestXMultJoker with the specified mult multiplier.
    ///
    /// # Arguments
    /// * `mult_multiplier` - Multiplier to apply (1.0 = no change, 2.0 = double)
    pub fn new(mult_multiplier: f64) -> Self {
        Self {
            mult_multiplier,
            active: true,
            joker_id: JokerId::Reserved,
            name_override: None,
            applies_to_cards: false,
        }
    }

    /// Set active state (for conditional testing).
    pub fn with_active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    /// Set custom joker ID.
    pub fn with_id(mut self, id: JokerId) -> Self {
        self.joker_id = id;
        self
    }

    /// Set custom display name.
    pub fn with_name(mut self, name: String) -> Self {
        self.name_override = Some(name);
        self
    }

    /// Make the multiplier apply to individual cards instead of hands.
    pub fn applies_to_cards(mut self) -> Self {
        self.applies_to_cards = true;
        self
    }
}

impl Joker for TestXMultJoker {
    fn id(&self) -> JokerId {
        self.joker_id
    }

    fn name(&self) -> &str {
        self.name_override.as_deref().unwrap_or("Test XMult Joker")
    }

    fn description(&self) -> &str {
        "Test joker for multiplicative effects"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Uncommon
    }

    fn cost(&self) -> usize {
        6
    }

    fn on_hand_played(&self, _context: &mut GameContext, _hand: &SelectHand) -> JokerEffect {
        if self.active && !self.applies_to_cards {
            JokerEffect::new().with_mult_multiplier(self.mult_multiplier)
        } else {
            JokerEffect::new()
        }
    }

    fn on_card_scored(&self, _context: &mut GameContext, _card: &Card) -> JokerEffect {
        if self.active && self.applies_to_cards {
            JokerEffect::new().with_mult_multiplier(self.mult_multiplier)
        } else {
            JokerEffect::new()
        }
    }
}

// ================================================================================================
// TestMoneyJoker - For economic effect testing
// ================================================================================================

/// Test joker for validating economic effects.
///
/// This joker provides configurable money and interest bonuses and can be used to test:
/// - Basic money generation
/// - Interest bonus mechanics
/// - Economic scaling effects
/// - Conditional economic bonuses
#[derive(Debug, Clone)]
pub struct TestMoneyJoker {
    /// Money to award per activation
    pub money: i32,
    /// Interest bonus to add to base interest calculation
    pub interest_bonus: i32,
    /// Whether the joker is active (for conditional testing)
    pub active: bool,
    /// Custom joker ID (defaults to Reserved)
    pub joker_id: JokerId,
    /// Display name override
    pub name_override: Option<String>,
    /// Whether to award money on hand played (default) or end of round
    pub awards_on_hand: bool,
}

impl TestMoneyJoker {
    /// Create a new TestMoneyJoker with the specified money bonus.
    pub fn new(money: i32) -> Self {
        Self {
            money,
            interest_bonus: 0,
            active: true,
            joker_id: JokerId::Reserved,
            name_override: None,
            awards_on_hand: true,
        }
    }

    /// Set interest bonus.
    pub fn with_interest_bonus(mut self, interest_bonus: i32) -> Self {
        self.interest_bonus = interest_bonus;
        self
    }

    /// Set active state (for conditional testing).
    pub fn with_active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    /// Set custom joker ID.
    pub fn with_id(mut self, id: JokerId) -> Self {
        self.joker_id = id;
        self
    }

    /// Set custom display name.
    pub fn with_name(mut self, name: String) -> Self {
        self.name_override = Some(name);
        self
    }

    /// Make the joker award money at end of round instead of on hand played.
    pub fn awards_at_round_end(mut self) -> Self {
        self.awards_on_hand = false;
        self
    }
}

impl Joker for TestMoneyJoker {
    fn id(&self) -> JokerId {
        self.joker_id
    }

    fn name(&self) -> &str {
        self.name_override.as_deref().unwrap_or("Test Money Joker")
    }

    fn description(&self) -> &str {
        "Test joker for economic effects"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Common
    }

    fn cost(&self) -> usize {
        5
    }

    fn on_hand_played(&self, _context: &mut GameContext, _hand: &SelectHand) -> JokerEffect {
        if self.active && self.awards_on_hand {
            JokerEffect::new()
                .with_money(self.money)
                .with_interest_bonus(self.interest_bonus)
        } else {
            JokerEffect::new()
        }
    }

    fn on_round_end(&self, _context: &mut GameContext) -> JokerEffect {
        if self.active && !self.awards_on_hand {
            JokerEffect::new()
                .with_money(self.money)
                .with_interest_bonus(self.interest_bonus)
        } else {
            JokerEffect::new()
        }
    }
}

// ================================================================================================
// TestRetriggerJoker - For retrigger effect testing
// ================================================================================================

/// Test joker for validating retrigger mechanics.
///
/// This joker provides configurable retrigger counts and can be used to test:
/// - Basic retrigger mechanics
/// - Conditional retrigger effects
/// - Retrigger stacking behavior
/// - Performance of retrigger processing
#[derive(Debug, Clone)]
pub struct TestRetriggerJoker {
    /// Number of retriggers to apply
    pub retrigger_count: u32,
    /// Base effect to apply with each trigger
    pub base_chips: i32,
    /// Base mult to apply with each trigger
    pub base_mult: i32,
    /// Whether the joker is active (for conditional testing)
    pub active: bool,
    /// Custom joker ID (defaults to Reserved)
    pub joker_id: JokerId,
    /// Display name override
    pub name_override: Option<String>,
}

impl TestRetriggerJoker {
    /// Create a new TestRetriggerJoker with the specified retrigger count.
    pub fn new(retrigger_count: u32) -> Self {
        Self {
            retrigger_count,
            base_chips: 0,
            base_mult: 0,
            active: true,
            joker_id: JokerId::Reserved,
            name_override: None,
        }
    }

    /// Set base effect to apply with each trigger.
    pub fn with_base_effect(mut self, chips: i32, mult: i32) -> Self {
        self.base_chips = chips;
        self.base_mult = mult;
        self
    }

    /// Set active state (for conditional testing).
    pub fn with_active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    /// Set custom joker ID.
    pub fn with_id(mut self, id: JokerId) -> Self {
        self.joker_id = id;
        self
    }

    /// Set custom display name.
    pub fn with_name(mut self, name: String) -> Self {
        self.name_override = Some(name);
        self
    }
}

impl Joker for TestRetriggerJoker {
    fn id(&self) -> JokerId {
        self.joker_id
    }

    fn name(&self) -> &str {
        self.name_override
            .as_deref()
            .unwrap_or("Test Retrigger Joker")
    }

    fn description(&self) -> &str {
        "Test joker for retrigger effects"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Rare
    }

    fn cost(&self) -> usize {
        8
    }

    fn on_hand_played(&self, _context: &mut GameContext, _hand: &SelectHand) -> JokerEffect {
        if self.active {
            JokerEffect::new()
                .with_chips(self.base_chips)
                .with_mult(self.base_mult)
                .with_retrigger(self.retrigger_count)
        } else {
            JokerEffect::new()
        }
    }

    fn on_card_scored(&self, _context: &mut GameContext, _card: &Card) -> JokerEffect {
        if self.active {
            JokerEffect::new()
                .with_chips(self.base_chips)
                .with_mult(self.base_mult)
                .with_retrigger(self.retrigger_count)
        } else {
            JokerEffect::new()
        }
    }
}

// ================================================================================================
// TestSpecialJoker - For complex effect testing
// ================================================================================================

/// Test joker for validating complex effects.
///
/// This joker provides configurable complex effects and can be used to test:
/// - Card transformation mechanics
/// - Joker destruction effects
/// - Self-destruction mechanics
/// - Complex multi-part effects
#[derive(Debug, Clone)]
pub struct TestSpecialJoker {
    /// Base effect chips/mult
    pub base_chips: i32,
    pub base_mult: i32,
    /// Whether to destroy self after activation
    pub destroys_self: bool,
    /// Other jokers to destroy
    pub destroys_others: Vec<JokerId>,
    /// Card transformations to apply
    pub card_transforms: Vec<(Card, Card)>,
    /// Whether the joker is active (for conditional testing)
    pub active: bool,
    /// Custom joker ID (defaults to Reserved)
    pub joker_id: JokerId,
    /// Display name override
    pub name_override: Option<String>,
}

impl TestSpecialJoker {
    /// Create a new TestSpecialJoker with basic effects.
    pub fn new() -> Self {
        Self {
            base_chips: 0,
            base_mult: 0,
            destroys_self: false,
            destroys_others: Vec::new(),
            card_transforms: Vec::new(),
            active: true,
            joker_id: JokerId::Reserved,
            name_override: None,
        }
    }

    /// Set base effect.
    pub fn with_base_effect(mut self, chips: i32, mult: i32) -> Self {
        self.base_chips = chips;
        self.base_mult = mult;
        self
    }

    /// Make the joker destroy itself after activation.
    pub fn destroys_self(mut self) -> Self {
        self.destroys_self = true;
        self
    }

    /// Add jokers to destroy when this effect is applied.
    pub fn destroys_others(mut self, others: Vec<JokerId>) -> Self {
        self.destroys_others = others;
        self
    }

    /// Add card transformations.
    pub fn with_card_transforms(mut self, transforms: Vec<(Card, Card)>) -> Self {
        self.card_transforms = transforms;
        self
    }

    /// Set active state (for conditional testing).
    pub fn with_active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    /// Set custom joker ID.
    pub fn with_id(mut self, id: JokerId) -> Self {
        self.joker_id = id;
        self
    }

    /// Set custom display name.
    pub fn with_name(mut self, name: String) -> Self {
        self.name_override = Some(name);
        self
    }
}

impl Default for TestSpecialJoker {
    fn default() -> Self {
        Self::new()
    }
}

impl Joker for TestSpecialJoker {
    fn id(&self) -> JokerId {
        self.joker_id
    }

    fn name(&self) -> &str {
        self.name_override
            .as_deref()
            .unwrap_or("Test Special Joker")
    }

    fn description(&self) -> &str {
        "Test joker for complex effects"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Legendary
    }

    fn cost(&self) -> usize {
        12
    }

    fn on_hand_played(&self, _context: &mut GameContext, _hand: &SelectHand) -> JokerEffect {
        if self.active {
            JokerEffect::new()
                .with_chips(self.base_chips)
                .with_mult(self.base_mult)
                .with_destroy_self(self.destroys_self)
                .with_destroy_others(self.destroys_others.clone())
                .with_transform_cards(self.card_transforms.clone())
        } else {
            JokerEffect::new()
        }
    }
}

// ================================================================================================
// TestScalingJoker - For scaling effect testing
// ================================================================================================

/// Defines what triggers scaling for TestScalingJoker.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScalingTrigger {
    /// Scale based on number of hands played
    HandsPlayed,
    /// Scale based on number of cards discarded
    CardsDiscarded,
    /// Scale based on current ante level
    AnteLevel,
    /// Scale based on current round number
    RoundNumber,
    /// Scale based on money earned
    MoneyEarned,
    /// Scale based on joker activations
    Activations,
}

/// Test joker for validating scaling effects.
///
/// This joker provides configurable scaling mechanics and can be used to test:
/// - Scaling based on game state
/// - Different scaling triggers
/// - Linear vs exponential scaling
/// - Performance of scaling calculations
#[derive(Debug, Clone)]
pub struct TestScalingJoker {
    /// Base effect values
    pub base_chips: i32,
    pub base_mult: i32,
    pub base_money: i32,
    /// Scaling configuration
    pub scaling_factor: f64,
    pub scaling_trigger: ScalingTrigger,
    /// Current scaled values (updated based on trigger) - now computed on-demand
    pub current_chips: i32,
    pub current_mult: i32,
    pub current_money: i32,
    /// Whether the joker is active (for conditional testing)
    pub active: bool,
    /// Custom joker ID (defaults to Reserved)
    pub joker_id: JokerId,
    /// Display name override
    pub name_override: Option<String>,
}

impl TestScalingJoker {
    /// Create a new TestScalingJoker with default scaling.
    pub fn new() -> Self {
        Self {
            base_chips: 1,
            base_mult: 1,
            base_money: 0,
            scaling_factor: 1.0,
            scaling_trigger: ScalingTrigger::Activations,
            current_chips: 1,
            current_mult: 1,
            current_money: 0,
            active: true,
            joker_id: JokerId::Reserved,
            name_override: None,
        }
    }

    /// Set base chip value.
    pub fn with_base_chips(mut self, base_chips: i32) -> Self {
        self.base_chips = base_chips;
        self.current_chips = base_chips;
        self
    }

    /// Set base mult value.
    pub fn with_base_mult(mut self, base_mult: i32) -> Self {
        self.base_mult = base_mult;
        self.current_mult = base_mult;
        self
    }

    /// Set base money value.
    pub fn with_base_money(mut self, base_money: i32) -> Self {
        self.base_money = base_money;
        self.current_money = base_money;
        self
    }

    /// Set scaling factor (multiplier applied each trigger).
    pub fn with_scaling_factor(mut self, scaling_factor: f64) -> Self {
        self.scaling_factor = scaling_factor;
        self
    }

    /// Set scaling trigger.
    pub fn with_scaling_trigger(mut self, scaling_trigger: ScalingTrigger) -> Self {
        self.scaling_trigger = scaling_trigger;
        self
    }

    /// Set active state (for conditional testing).
    pub fn with_active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    /// Set custom joker ID.
    pub fn with_id(mut self, id: JokerId) -> Self {
        self.joker_id = id;
        self
    }

    /// Set custom display name.
    pub fn with_name(mut self, name: String) -> Self {
        self.name_override = Some(name);
        self
    }

    /// Get the current scaling multiplier based on activations stored in state manager.
    /// This is used for testing and introspection.
    pub fn get_current_multiplier(&self, context: &GameContext) -> f64 {
        let activations = context
            .joker_state_manager
            .get_state(self.joker_id)
            .map(|state| state.accumulated_value as u32)
            .unwrap_or(0);

        match self.scaling_trigger {
            ScalingTrigger::Activations => 1.0 + (activations as f64 * self.scaling_factor),
            ScalingTrigger::HandsPlayed => 1.0 + (activations as f64 * self.scaling_factor),
            _ => 1.0 + (activations as f64 * self.scaling_factor),
        }
    }
}

impl Default for TestScalingJoker {
    fn default() -> Self {
        Self::new()
    }
}

impl Joker for TestScalingJoker {
    fn id(&self) -> JokerId {
        self.joker_id
    }

    fn name(&self) -> &str {
        self.name_override
            .as_deref()
            .unwrap_or("Test Scaling Joker")
    }

    fn description(&self) -> &str {
        "Test joker for scaling effects"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Rare
    }

    fn cost(&self) -> usize {
        10
    }

    fn on_hand_played(&self, context: &mut GameContext, _hand: &SelectHand) -> JokerEffect {
        if !self.active {
            return JokerEffect::new();
        }

        // Update activations count in state manager
        context
            .joker_state_manager
            .update_state(self.joker_id, |state| {
                state.accumulated_value = (state.accumulated_value + 1.0).min(1000.0);
                // Max 1000 activations
            });

        // Get current activation count
        let activations = context
            .joker_state_manager
            .get_state(self.joker_id)
            .map(|state| state.accumulated_value as u32)
            .unwrap_or(0);

        // Calculate current scaled values based on activations and scaling factor
        let scaling_multiplier = match self.scaling_trigger {
            ScalingTrigger::Activations => 1.0 + (activations as f64 * self.scaling_factor),
            ScalingTrigger::HandsPlayed => 1.0 + (activations as f64 * self.scaling_factor),
            ScalingTrigger::CardsDiscarded => {
                // For card discarding, we'd need access to discard count from context
                // For now, use activations as a proxy
                1.0 + (activations as f64 * self.scaling_factor)
            }
            ScalingTrigger::AnteLevel => {
                // For ante level, we'd need access to ante from context
                // For now, use a reasonable scaling based on activations
                let ante_equivalent = (activations / 10).max(1); // Every 10 activations = 1 ante level
                1.0 + (ante_equivalent as f64 * self.scaling_factor)
            }
            ScalingTrigger::RoundNumber => {
                // For round number, we'd need access to round from context
                // For now, use activations as proxy
                1.0 + (activations as f64 * self.scaling_factor)
            }
            ScalingTrigger::MoneyEarned => {
                // For money earned, we'd need access to money from context
                // For now, use activations as proxy
                1.0 + (activations as f64 * self.scaling_factor)
            }
        };

        let current_chips = ((self.base_chips as f64) * scaling_multiplier).round() as i32;
        let current_mult = ((self.base_mult as f64) * scaling_multiplier).round() as i32;
        let current_money = ((self.base_money as f64) * scaling_multiplier).round() as i32;

        JokerEffect::new()
            .with_chips(current_chips)
            .with_mult(current_mult)
            .with_money(current_money)
    }
}

// ================================================================================================
// TestScalingChipsJoker - Specialized scaling for chips only
// ================================================================================================

/// Specialized test joker for scaling chips effects.
#[derive(Debug, Clone)]
pub struct TestScalingChipsJoker {
    pub base_chips: i32,
    pub scaling_factor: f64,
    pub current_chips: i32,
}

impl Joker for TestScalingChipsJoker {
    fn id(&self) -> JokerId {
        JokerId::Reserved
    }

    fn name(&self) -> &str {
        "Test Scaling Chips Joker"
    }

    fn description(&self) -> &str {
        "Test joker for scaling chips effects"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Uncommon
    }

    fn cost(&self) -> usize {
        7
    }

    fn on_hand_played(&self, context: &mut GameContext, _hand: &SelectHand) -> JokerEffect {
        // Update activations count in state manager
        context
            .joker_state_manager
            .update_state(self.id(), |state| {
                state.accumulated_value = (state.accumulated_value + 1.0).min(1000.0);
                // Max 1000 activations
            });

        // Get current activation count
        let activations = context
            .joker_state_manager
            .get_state(self.id())
            .map(|state| state.accumulated_value as u32)
            .unwrap_or(0);

        // Calculate current scaled chips
        let current_chips = (self.base_chips as f64
            * (1.0 + activations as f64 * self.scaling_factor))
            .round() as i32;

        JokerEffect::new().with_chips(current_chips)
    }
}

// ================================================================================================
// Test Utilities and Builders
// ================================================================================================

/// Builder for creating collections of test jokers for comprehensive testing.
#[derive(Debug, Default)]
pub struct TestJokerBuilder {
    jokers: Vec<Box<dyn Joker>>,
}

impl TestJokerBuilder {
    /// Create a new TestJokerBuilder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a chips joker to the collection.
    pub fn add_chips_joker(mut self, chips: i32) -> Self {
        self.jokers.push(Box::new(TestChipsJoker::new(chips)));
        self
    }

    /// Add a mult joker to the collection.
    pub fn add_mult_joker(mut self, mult: i32) -> Self {
        self.jokers.push(Box::new(TestMultJoker::new(mult)));
        self
    }

    /// Add an xmult joker to the collection.
    pub fn add_xmult_joker(mut self, mult_multiplier: f64) -> Self {
        self.jokers
            .push(Box::new(TestXMultJoker::new(mult_multiplier)));
        self
    }

    /// Add a money joker to the collection.
    pub fn add_money_joker(mut self, money: i32) -> Self {
        self.jokers.push(Box::new(TestMoneyJoker::new(money)));
        self
    }

    /// Add a retrigger joker to the collection.
    pub fn add_retrigger_joker(mut self, retriggers: u32) -> Self {
        self.jokers
            .push(Box::new(TestRetriggerJoker::new(retriggers)));
        self
    }

    /// Add a scaling joker to the collection.
    pub fn add_scaling_joker(mut self, base_chips: i32, scaling_factor: f64) -> Self {
        self.jokers.push(Box::new(
            TestScalingJoker::new()
                .with_base_chips(base_chips)
                .with_scaling_factor(scaling_factor),
        ));
        self
    }

    /// Build the collection of test jokers.
    pub fn build(self) -> Vec<Box<dyn Joker>> {
        self.jokers
    }
}

/// Create a comprehensive test suite with all joker types.
pub fn create_comprehensive_test_suite() -> Vec<Box<dyn Joker>> {
    TestJokerBuilder::new()
        .add_chips_joker(10)
        .add_mult_joker(5)
        .add_xmult_joker(1.5)
        .add_money_joker(3)
        .add_retrigger_joker(2)
        .add_scaling_joker(1, 0.1)
        .build()
}

/// Create a performance test suite optimized for benchmarking.
pub fn create_performance_test_suite(count: usize) -> Vec<Box<dyn Joker>> {
    let mut jokers = Vec::with_capacity(count);

    for i in 0..count {
        let joker: Box<dyn Joker> = match i % 6 {
            0 => Box::new(TestChipsJoker::new((i % 20) as i32 + 1)),
            1 => Box::new(TestMultJoker::new((i % 15) as i32 + 1)),
            2 => Box::new(TestXMultJoker::new(1.0 + (i % 10) as f64 * 0.1)),
            3 => Box::new(TestMoneyJoker::new((i % 5) as i32 + 1)),
            4 => Box::new(TestRetriggerJoker::new((i % 3) as u32 + 1)),
            5 => Box::new(TestScalingJoker::new().with_base_chips((i % 10) as i32 + 1)),
            _ => unreachable!(),
        };
        jokers.push(joker);
    }

    jokers
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::{Card, Suit, Value};

    #[test]
    fn test_chips_joker_basic() {
        let joker = TestChipsJoker::new(15);
        assert_eq!(joker.name(), "Test Chips Joker");
        assert_eq!(joker.id(), JokerId::Reserved);

        // Mock context - in real tests you'd use proper context
        let mut context = create_mock_context();
        let hand = create_test_hand();

        let effect = joker.on_hand_played(&mut context, &hand);
        assert_eq!(effect.chips, 15);
        assert_eq!(effect.mult, 0);
    }

    #[test]
    fn test_mult_joker_basic() {
        let joker = TestMultJoker::new(8);
        let mut context = create_mock_context();
        let hand = create_test_hand();

        let effect = joker.on_hand_played(&mut context, &hand);
        assert_eq!(effect.chips, 0);
        assert_eq!(effect.mult, 8);
    }

    #[test]
    fn test_xmult_joker_basic() {
        let joker = TestXMultJoker::new(2.0);
        let mut context = create_mock_context();
        let hand = create_test_hand();

        let effect = joker.on_hand_played(&mut context, &hand);
        assert_eq!(effect.mult_multiplier, 2.0);
    }

    #[test]
    fn test_money_joker_basic() {
        let joker = TestMoneyJoker::new(5).with_interest_bonus(2);
        let mut context = create_mock_context();
        let hand = create_test_hand();

        let effect = joker.on_hand_played(&mut context, &hand);
        assert_eq!(effect.money, 5);
        assert_eq!(effect.interest_bonus, 2);
    }

    #[test]
    fn test_retrigger_joker_basic() {
        let joker = TestRetriggerJoker::new(3).with_base_effect(5, 2);
        let mut context = create_mock_context();
        let hand = create_test_hand();

        let effect = joker.on_hand_played(&mut context, &hand);
        assert_eq!(effect.retrigger, 3);
        assert_eq!(effect.chips, 5);
        assert_eq!(effect.mult, 2);
    }

    #[test]
    fn test_special_joker_basic() {
        let joker = TestSpecialJoker::new()
            .with_base_effect(10, 5)
            .destroys_self();

        let mut context = create_mock_context();
        let hand = create_test_hand();

        let effect = joker.on_hand_played(&mut context, &hand);
        assert_eq!(effect.chips, 10);
        assert_eq!(effect.mult, 5);
        assert!(effect.destroy_self);
    }

    #[test]
    fn test_scaling_joker_basic() {
        let joker = TestScalingJoker::new()
            .with_base_chips(5)
            .with_scaling_factor(0.2);

        let mut context = create_mock_context();

        // Initially no scaling
        assert_eq!(joker.get_current_multiplier(&context), 1.0);

        // Simulate 5 activations by calling on_hand_played 5 times
        let hand = SelectHand::new(vec![]);
        for _ in 0..5 {
            joker.on_hand_played(&mut context, &hand);
        }

        // Should now have scaling multiplier of 2.0 (1 + 5 * 0.2)
        assert_eq!(joker.get_current_multiplier(&context), 2.0);

        // The effect should give chips based on 6th activation
        let effect = joker.on_hand_played(&mut context, &hand);
        assert_eq!(effect.chips, 11); // 5 * (1 + 6 * 0.2) = 5 * 2.2 = 11 (6th activation)
    }

    #[test]
    fn test_builder_pattern() {
        let jokers = TestJokerBuilder::new()
            .add_chips_joker(10)
            .add_mult_joker(5)
            .add_xmult_joker(1.5)
            .build();

        assert_eq!(jokers.len(), 3);
    }

    #[test]
    fn test_performance_suite_creation() {
        let jokers = create_performance_test_suite(100);
        assert_eq!(jokers.len(), 100);

        // Verify variety
        let mut chip_jokers = 0;
        let mut mult_jokers = 0;
        let mut xmult_jokers = 0;

        for joker in &jokers {
            match joker.name() {
                "Test Chips Joker" => chip_jokers += 1,
                "Test Mult Joker" => mult_jokers += 1,
                "Test XMult Joker" => xmult_jokers += 1,
                _ => {}
            }
        }

        assert!(chip_jokers > 0);
        assert!(mult_jokers > 0);
        assert!(xmult_jokers > 0);
    }

    // Helper functions for testing
    fn create_mock_context() -> GameContext<'static> {
        use std::collections::HashMap;
        use std::sync::Arc;

        // Create static references for testing
        let stage = Box::leak(Box::new(crate::stage::Stage::PreBlind()));
        let hand = Box::leak(Box::new(crate::hand::Hand::new(vec![])));
        let jokers: &'static [Box<dyn Joker>] = Box::leak(Box::new([]));
        let discarded: &'static [Card] = Box::leak(Box::new([]));
        let joker_state_manager = Box::leak(Box::new(Arc::new(
            crate::joker_state::JokerStateManager::new(),
        )));
        let hand_type_counts = Box::leak(Box::new(HashMap::new()));
        let rng = Box::leak(Box::new(crate::rng::GameRng::for_testing(12345)));

        GameContext {
            chips: 100,
            mult: 4,
            money: 50,
            ante: 1,
            round: 1,
            stage,
            hands_played: 0,
            hands_remaining: 4.0,
            discards_used: 0,
            is_final_hand: false,
            jokers,
            hand,
            discarded,
            joker_state_manager,
            hand_type_counts,
            cards_in_deck: 52,
            stone_cards_in_deck: 0,
            steel_cards_in_deck: 0,
            enhanced_cards_in_deck: 0,
            rng,
        }
    }

    fn create_test_hand() -> SelectHand {
        SelectHand::new(vec![
            Card::new(Value::Ace, Suit::Spade),
            Card::new(Value::King, Suit::Spade),
            Card::new(Value::Queen, Suit::Spade),
            Card::new(Value::Jack, Suit::Spade),
            Card::new(Value::Ten, Suit::Spade),
        ])
    }
}
