use crate::joker::{Joker, JokerId, JokerRarity, JokerEffect, GameContext};
use crate::joker_state::JokerState;
use crate::rank::HandRank;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Triggers that can cause a scaling joker to increment its value
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ScalingTrigger {
    /// Triggered when a specific hand type is played
    HandPlayed(HandRank),
    /// Triggered when any card is discarded
    CardDiscarded,
    /// Triggered when money is gained
    MoneyGained,
    /// Triggered when a blind is completed
    BlindCompleted,
    /// Triggered when the shop is rerolled
    ShopReroll,
    /// Triggered when a joker is sold
    JokerSold,
    /// Triggered when a card is destroyed
    CardDestroyed,
    /// Triggered when a consumable is used
    ConsumableUsed,
}

impl fmt::Display for ScalingTrigger {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScalingTrigger::HandPlayed(hand_rank) => write!(f, "{} played", hand_rank),
            ScalingTrigger::CardDiscarded => write!(f, "card discarded"),
            ScalingTrigger::MoneyGained => write!(f, "money gained"),
            ScalingTrigger::BlindCompleted => write!(f, "blind completed"),
            ScalingTrigger::ShopReroll => write!(f, "shop reroll"),
            ScalingTrigger::JokerSold => write!(f, "joker sold"),
            ScalingTrigger::CardDestroyed => write!(f, "card destroyed"),
            ScalingTrigger::ConsumableUsed => write!(f, "consumable used"),
        }
    }
}

/// Conditions that can reset a scaling joker's accumulated value
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ResetCondition {
    /// Reset when a specific hand type is played
    HandPlayed(HandRank),
    /// Reset at the end of each round
    RoundEnd,
    /// Reset at the end of each ante
    AnteEnd,
    /// Reset when money is spent
    MoneySpent,
    /// Reset when entering the shop
    ShopEntered,
    /// Reset when a joker is purchased
    JokerPurchased,
    /// Never reset (permanent accumulation)
    Never,
}

impl fmt::Display for ResetCondition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ResetCondition::HandPlayed(hand_rank) => write!(f, "reset on {} played", hand_rank),
            ResetCondition::RoundEnd => write!(f, "reset at round end"),
            ResetCondition::AnteEnd => write!(f, "reset at ante end"),
            ResetCondition::MoneySpent => write!(f, "reset when money spent"),
            ResetCondition::ShopEntered => write!(f, "reset when entering shop"),
            ResetCondition::JokerPurchased => write!(f, "reset when joker purchased"),
            ResetCondition::Never => write!(f, "never resets"),
        }
    }
}

/// Framework for jokers that accumulate value over time based on triggers
#[derive(Debug, Clone)]
pub struct ScalingJoker {
    /// Unique identifier for this joker
    pub id: JokerId,
    /// Display name
    pub name: String,
    /// Description of the joker's effect
    pub description: String,
    /// Rarity level
    pub rarity: JokerRarity,
    /// Base value added per trigger (before scaling)
    pub base_value: f64,
    /// Amount to increment per trigger
    pub increment: f64,
    /// What triggers this joker to scale
    pub trigger: ScalingTrigger,
    /// Maximum value this joker can reach (None = unlimited)
    pub max_value: Option<f64>,
    /// Condition that resets the accumulated value
    pub reset_condition: Option<ResetCondition>,
    /// Whether this joker provides chips, mult, or other effects
    pub effect_type: ScalingEffectType,
}

/// Types of effects that scaling jokers can provide
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ScalingEffectType {
    /// Provides chips based on accumulated value
    Chips,
    /// Provides mult based on accumulated value
    Mult,
    /// Provides mult multiplier based on accumulated value
    MultMultiplier,
    /// Provides money based on accumulated value
    Money,
    /// Custom effect (requires override of effect calculation)
    Custom,
}

impl ScalingJoker {
    /// Create a new scaling joker
    pub fn new(
        id: JokerId,
        name: String,
        description: String,
        rarity: JokerRarity,
        base_value: f64,
        increment: f64,
        trigger: ScalingTrigger,
        effect_type: ScalingEffectType,
    ) -> Self {
        Self {
            id,
            name,
            description,
            rarity,
            base_value,
            increment,
            trigger,
            max_value: None,
            reset_condition: None,
            effect_type,
        }
    }

    /// Set the maximum value for this scaling joker
    pub fn with_max_value(mut self, max_value: f64) -> Self {
        self.max_value = Some(max_value);
        self
    }

    /// Set the reset condition for this scaling joker
    pub fn with_reset_condition(mut self, reset_condition: ResetCondition) -> Self {
        self.reset_condition = Some(reset_condition);
        self
    }

    /// Get the current accumulated value from the state manager
    fn get_accumulated_value(&self, context: &GameContext) -> f64 {
        context
            .joker_state_manager
            .get_accumulated_value(self.id)
            .unwrap_or(self.base_value)
    }

    /// Increment the accumulated value and apply max cap if set
    fn increment_value(&self, context: &mut GameContext) {
        let current_value = self.get_accumulated_value(context);
        let new_value = current_value + self.increment;
        
        let final_value = match self.max_value {
            Some(max) => new_value.min(max),
            None => new_value,
        };

        context
            .joker_state_manager
            .update_state(self.id, |state| {
                state.accumulated_value = final_value;
            });
    }

    /// Reset the accumulated value to base value
    fn reset_value(&self, context: &mut GameContext) {
        context
            .joker_state_manager
            .update_state(self.id, |state| {
                state.accumulated_value = self.base_value;
            });
    }

    /// Check if a reset condition is met and reset if necessary
    fn check_and_apply_reset(&self, context: &mut GameContext, event: &ScalingEvent) {
        if let Some(reset_condition) = self.reset_condition {
            let should_reset = match (reset_condition, event) {
                (ResetCondition::HandPlayed(reset_hand), ScalingEvent::HandPlayed(played_hand)) => {
                    reset_hand == *played_hand
                }
                (ResetCondition::RoundEnd, ScalingEvent::RoundEnd) => true,
                (ResetCondition::AnteEnd, ScalingEvent::AnteEnd) => true,
                (ResetCondition::MoneySpent, ScalingEvent::MoneySpent) => true,
                (ResetCondition::ShopEntered, ScalingEvent::ShopEntered) => true,
                (ResetCondition::JokerPurchased, ScalingEvent::JokerPurchased) => true,
                _ => false,
            };

            if should_reset {
                self.reset_value(context);
            }
        }
    }

    /// Check if the scaling trigger is met and increment if necessary
    fn check_and_apply_trigger(&self, context: &mut GameContext, event: &ScalingEvent) {
        let should_trigger = match (self.trigger, event) {
            (ScalingTrigger::HandPlayed(trigger_hand), ScalingEvent::HandPlayed(played_hand)) => {
                trigger_hand == *played_hand
            }
            (ScalingTrigger::CardDiscarded, ScalingEvent::CardDiscarded) => true,
            (ScalingTrigger::MoneyGained, ScalingEvent::MoneyGained) => true,
            (ScalingTrigger::BlindCompleted, ScalingEvent::BlindCompleted) => true,
            (ScalingTrigger::ShopReroll, ScalingEvent::ShopReroll) => true,
            (ScalingTrigger::JokerSold, ScalingEvent::JokerSold) => true,
            (ScalingTrigger::CardDestroyed, ScalingEvent::CardDestroyed) => true,
            (ScalingTrigger::ConsumableUsed, ScalingEvent::ConsumableUsed) => true,
            _ => false,
        };

        if should_trigger {
            self.increment_value(context);
        }
    }

    /// Process a scaling event (both reset and trigger logic)
    pub fn process_event(&self, context: &mut GameContext, event: &ScalingEvent) {
        // Check reset first (so we don't trigger and then immediately reset)
        self.check_and_apply_reset(context, event);
        
        // Then check trigger
        self.check_and_apply_trigger(context, event);
    }

    /// Calculate the effect based on current accumulated value
    pub fn calculate_effect(&self, context: &GameContext) -> JokerEffect {
        let value = self.get_accumulated_value(context);
        
        match self.effect_type {
            ScalingEffectType::Chips => JokerEffect::new().with_chips(value as i32),
            ScalingEffectType::Mult => JokerEffect::new().with_mult(value as i32),
            ScalingEffectType::MultMultiplier => JokerEffect::new().with_mult_multiplier(value),
            ScalingEffectType::Money => JokerEffect::new().with_money(value as i32),
            ScalingEffectType::Custom => JokerEffect::new(), // Override in specific implementations
        }
    }

    /// Get a formatted description including current value
    pub fn get_dynamic_description(&self, context: &GameContext) -> String {
        let current_value = self.get_accumulated_value(context);
        let effect_desc = match self.effect_type {
            ScalingEffectType::Chips => format!("Currently: +{} Chips", current_value as i32),
            ScalingEffectType::Mult => format!("Currently: +{} Mult", current_value as i32),
            ScalingEffectType::MultMultiplier => format!("Currently: X{:.1} Mult", current_value),
            ScalingEffectType::Money => format!("Currently: +${}", current_value as i32),
            ScalingEffectType::Custom => format!("Current value: {:.1}", current_value),
        };

        format!("{}\n{}", self.description, effect_desc)
    }
}

/// Events that can affect scaling jokers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScalingEvent {
    HandPlayed(HandRank),
    CardDiscarded,
    MoneyGained,
    BlindCompleted,
    ShopReroll,
    JokerSold,
    CardDestroyed,
    ConsumableUsed,
    RoundEnd,
    AnteEnd,
    MoneySpent,
    ShopEntered,
    JokerPurchased,
}

impl Joker for ScalingJoker {
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

    fn on_hand_played(&self, context: &mut GameContext, hand: &crate::hand::SelectHand) -> JokerEffect {
        // Process scaling event
        let hand_rank = match hand.best_hand() {
            Ok(made_hand) => made_hand.rank,
            Err(_) => HandRank::HighCard, // Fallback to high card if evaluation fails
        };
        self.process_event(context, &ScalingEvent::HandPlayed(hand_rank));
        
        // Return current effect
        self.calculate_effect(context)
    }

    fn on_discard(&self, context: &mut GameContext, _cards: &[crate::card::Card]) -> JokerEffect {
        // Process scaling event for each discarded card
        self.process_event(context, &ScalingEvent::CardDiscarded);
        
        // Return current effect (if applicable to discard phase)
        JokerEffect::new()
    }

    fn on_shop_open(&self, context: &mut GameContext) -> JokerEffect {
        // Process scaling event
        self.process_event(context, &ScalingEvent::ShopEntered);
        
        // Return current effect (if applicable to shop phase)
        JokerEffect::new()
    }

    fn on_round_end(&self, context: &mut GameContext) -> JokerEffect {
        // Process scaling event
        self.process_event(context, &ScalingEvent::RoundEnd);
        
        // Return current effect (if applicable to round end)
        JokerEffect::new()
    }

    fn initialize_state(&self, _context: &GameContext) -> JokerState {
        JokerState::with_accumulated_value(self.base_value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::joker_state::JokerStateManager;
    use std::sync::Arc;

    fn create_test_context() -> GameContext<'static> {
        // This is a simplified test context - in real tests we'd need proper initialization
        todo!("Implement test context creation")
    }

    #[test]
    fn test_scaling_joker_creation() {
        let joker = ScalingJoker::new(
            JokerId::GreedyJoker,
            "Test Scaling Joker".to_string(),
            "+1 Mult per hand played".to_string(),
            JokerRarity::Common,
            1.0,
            1.0,
            ScalingTrigger::HandPlayed(HandRank::HighCard),
            ScalingEffectType::Mult,
        );

        assert_eq!(joker.id, JokerId::GreedyJoker);
        assert_eq!(joker.name, "Test Scaling Joker");
        assert_eq!(joker.base_value, 1.0);
        assert_eq!(joker.increment, 1.0);
        assert_eq!(joker.max_value, None);
        assert_eq!(joker.reset_condition, None);
    }

    #[test]
    fn test_scaling_joker_with_options() {
        let joker = ScalingJoker::new(
            JokerId::GreedyJoker,
            "Test Joker".to_string(),
            "Test description".to_string(),
            JokerRarity::Common,
            0.0,
            2.0,
            ScalingTrigger::CardDiscarded,
            ScalingEffectType::Chips,
        )
        .with_max_value(20.0)
        .with_reset_condition(ResetCondition::RoundEnd);

        assert_eq!(joker.max_value, Some(20.0));
        assert_eq!(joker.reset_condition, Some(ResetCondition::RoundEnd));
    }

    #[test]
    fn test_scaling_trigger_display() {
        assert_eq!(
            format!("{}", ScalingTrigger::HandPlayed(HandRank::Pair)),
            "Pair played"
        );
        assert_eq!(format!("{}", ScalingTrigger::CardDiscarded), "card discarded");
        assert_eq!(format!("{}", ScalingTrigger::MoneyGained), "money gained");
    }

    #[test]
    fn test_reset_condition_display() {
        assert_eq!(
            format!("{}", ResetCondition::HandPlayed(HandRank::Flush)),
            "reset on Flush played"
        );
        assert_eq!(format!("{}", ResetCondition::RoundEnd), "reset at round end");
        assert_eq!(format!("{}", ResetCondition::Never), "never resets");
    }
}