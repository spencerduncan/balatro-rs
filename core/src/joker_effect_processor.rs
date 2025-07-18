use crate::card::Card;
use crate::hand::SelectHand;
use crate::joker::{GameContext, Joker, JokerEffect, JokerId};
use std::collections::HashMap;

/// Priority level for effect processing
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum EffectPriority {
    /// Lowest priority - applied first
    Low = 1,
    /// Normal priority - default for most jokers
    Normal = 5,
    /// High priority - applied after normal effects
    High = 10,
    /// Critical priority - applied last (e.g., multiplicative effects)
    Critical = 15,
}

impl Default for EffectPriority {
    fn default() -> Self {
        Self::Normal
    }
}

/// Strategy for resolving conflicts between competing effects
#[derive(Debug, Clone, PartialEq)]
pub enum ConflictResolutionStrategy {
    /// Sum all numeric effects (default)
    Sum,
    /// Take the maximum value for each effect type
    Maximum,
    /// Take the minimum value for each effect type
    Minimum,
    /// Use first joker's effect (by priority order)
    FirstWins,
    /// Use last joker's effect (by priority order)
    LastWins,
    // TODO: Add Custom resolution function support later
    // Custom(fn(&[JokerEffect]) -> JokerEffect),
}

impl Default for ConflictResolutionStrategy {
    fn default() -> Self {
        Self::Sum
    }
}

/// Context for effect processing operations
#[derive(Debug, Clone)]
pub struct ProcessingContext {
    /// Processing mode (immediate vs delayed)
    pub processing_mode: ProcessingMode,
    /// Current resolution strategy
    pub resolution_strategy: ConflictResolutionStrategy,
    /// Whether to validate effects before processing
    pub validate_effects: bool,
    /// Maximum number of retriggered effects to prevent infinite loops
    pub max_retriggered_effects: u32,
}

impl Default for ProcessingContext {
    fn default() -> Self {
        Self {
            processing_mode: ProcessingMode::Immediate,
            resolution_strategy: ConflictResolutionStrategy::default(),
            validate_effects: true,
            max_retriggered_effects: 100,
        }
    }
}

/// Processing mode for effects
#[derive(Debug, Clone, PartialEq)]
pub enum ProcessingMode {
    /// Process effects immediately when collected
    Immediate,
    /// Collect effects and process them later (for batching)
    Delayed,
}

/// Result of effect processing operation
#[derive(Debug, Clone)]
pub struct ProcessingResult {
    /// Final accumulated effect
    pub accumulated_effect: JokerEffect,
    /// Number of jokers that contributed effects
    pub jokers_processed: usize,
    /// Number of retriggered effects
    pub retriggered_count: u32,
    /// Any errors encountered during processing
    pub errors: Vec<EffectProcessingError>,
    /// Performance metrics
    pub processing_time_micros: u64,
}

/// Errors that can occur during effect processing
#[derive(Debug, Clone, PartialEq)]
pub enum EffectProcessingError {
    /// Effect validation failed
    ValidationFailed(String),
    /// Too many retriggered effects (infinite loop protection)
    TooManyRetriggers(u32),
    /// Conflicting effects couldn't be resolved
    ConflictResolutionFailed(String),
    /// Invalid joker state
    InvalidJokerState(JokerId, String),
}

/// Weighted effect with priority and source information
#[derive(Debug, Clone)]
pub struct WeightedEffect {
    /// The joker effect
    pub effect: JokerEffect,
    /// Priority for processing order
    pub priority: EffectPriority,
    /// Source joker that generated this effect
    pub source_joker_id: JokerId,
    /// Whether this effect was retriggered
    pub is_retriggered: bool,
}

/// Main processor for joker effects with accumulation and conflict resolution
#[derive(Debug, Clone)]
pub struct JokerEffectProcessor {
    /// Current processing context
    context: ProcessingContext,
    /// Cache for performance optimization
    effect_cache: HashMap<String, ProcessingResult>,
}

impl JokerEffectProcessor {
    /// Create a new effect processor with default settings
    pub fn new() -> Self {
        Self {
            context: ProcessingContext::default(),
            effect_cache: HashMap::new(),
        }
    }

    /// Create a processor with custom context
    pub fn with_context(context: ProcessingContext) -> Self {
        Self {
            context,
            effect_cache: HashMap::new(),
        }
    }

    /// Process effects when a hand is played
    pub fn process_hand_effects(
        &mut self,
        jokers: &[Box<dyn Joker>],
        game_context: &mut GameContext,
        hand: &SelectHand,
    ) -> ProcessingResult {
        let start_time = std::time::Instant::now();

        // Collect effects from all jokers
        let mut weighted_effects = Vec::new();

        for joker in jokers {
            let effect = joker.on_hand_played(game_context, hand);
            if !self.is_empty_effect(&effect) {
                weighted_effects.push(WeightedEffect {
                    effect,
                    priority: self.get_joker_priority(joker.id()),
                    source_joker_id: joker.id(),
                    is_retriggered: false,
                });
            }
        }

        // Process the collected effects
        self.process_weighted_effects(weighted_effects, start_time.elapsed().as_micros() as u64)
    }

    /// Process effects when individual cards are scored
    pub fn process_card_effects(
        &mut self,
        jokers: &[Box<dyn Joker>],
        game_context: &mut GameContext,
        card: &Card,
    ) -> ProcessingResult {
        let start_time = std::time::Instant::now();

        // Collect effects from all jokers for this card
        let mut weighted_effects = Vec::new();

        for joker in jokers {
            let effect = joker.on_card_scored(game_context, card);
            if !self.is_empty_effect(&effect) {
                weighted_effects.push(WeightedEffect {
                    effect,
                    priority: self.get_joker_priority(joker.id()),
                    source_joker_id: joker.id(),
                    is_retriggered: false,
                });
            }
        }

        // Process the collected effects
        self.process_weighted_effects(weighted_effects, start_time.elapsed().as_micros() as u64)
    }

    /// Process a collection of weighted effects
    fn process_weighted_effects(
        &mut self,
        mut weighted_effects: Vec<WeightedEffect>,
        base_processing_time: u64,
    ) -> ProcessingResult {
        let mut errors = Vec::new();
        let mut retriggered_count = 0;

        // Handle retriggering
        let mut i = 0;
        let original_length = weighted_effects.len();
        while i < original_length && retriggered_count < self.context.max_retriggered_effects {
            let retrigger_count = weighted_effects[i].effect.retrigger;

            // Process retriggers for this effect
            for _ in 0..retrigger_count {
                if retriggered_count >= self.context.max_retriggered_effects {
                    errors.push(EffectProcessingError::TooManyRetriggers(
                        self.context.max_retriggered_effects,
                    ));
                    break;
                }

                let mut retriggered_effect = weighted_effects[i].clone();
                retriggered_effect.is_retriggered = true;
                weighted_effects.push(retriggered_effect);
                retriggered_count += 1;
            }

            i += 1;
        }

        // Sort by priority (higher priority applied later)
        weighted_effects.sort_by_key(|we| we.priority);

        // Validate effects if enabled
        if self.context.validate_effects {
            for weighted_effect in &weighted_effects {
                if let Err(validation_error) = self.validate_effect(&weighted_effect.effect) {
                    errors.push(EffectProcessingError::ValidationFailed(validation_error));
                }
            }
        }

        // Accumulate effects based on resolution strategy
        let accumulated_effect = self.accumulate_effects(&weighted_effects);

        ProcessingResult {
            accumulated_effect,
            jokers_processed: weighted_effects.len(),
            retriggered_count,
            errors,
            processing_time_micros: base_processing_time
                + std::time::Instant::now().elapsed().as_micros() as u64,
        }
    }

    /// Accumulate multiple effects into a single effect using the current resolution strategy
    fn accumulate_effects(&self, weighted_effects: &[WeightedEffect]) -> JokerEffect {
        if weighted_effects.is_empty() {
            return JokerEffect::new();
        }

        let effects: Vec<JokerEffect> = weighted_effects
            .iter()
            .map(|we| we.effect.clone())
            .collect();

        match &self.context.resolution_strategy {
            ConflictResolutionStrategy::Sum => self.sum_effects(&effects),
            ConflictResolutionStrategy::Maximum => self.max_effects(&effects),
            ConflictResolutionStrategy::Minimum => self.min_effects(&effects),
            ConflictResolutionStrategy::FirstWins => effects[0].clone(),
            ConflictResolutionStrategy::LastWins => effects[effects.len() - 1].clone(),
        }
    }

    /// Sum all numeric effects together
    fn sum_effects(&self, effects: &[JokerEffect]) -> JokerEffect {
        let mut result = JokerEffect::new();

        // Start mult_multiplier at 1.0 for proper multiplication
        result.mult_multiplier = 1.0;

        for effect in effects {
            result.chips += effect.chips;
            result.mult += effect.mult;
            result.money += effect.money;
            result.hand_size_mod += effect.hand_size_mod;
            result.discard_mod += effect.discard_mod;
            result.sell_value_increase += effect.sell_value_increase;

            // Multiplicative effects are multiplied together
            // Only multiply if the effect has a non-default multiplier
            if effect.mult_multiplier != 0.0 {
                result.mult_multiplier *= effect.mult_multiplier;
            }

            // Boolean effects - any true makes result true
            result.destroy_self = result.destroy_self || effect.destroy_self;

            // Append vectors
            result.destroy_others.extend(&effect.destroy_others);
            result.transform_cards.extend(&effect.transform_cards);

            // Take last non-empty message
            if effect.message.is_some() {
                result.message = effect.message.clone();
            }
        }

        // If no multiplicative effects were applied, set back to default (0.0)
        if result.mult_multiplier == 1.0 {
            result.mult_multiplier = 0.0;
        }

        result
    }

    /// Take maximum values for all effects
    fn max_effects(&self, effects: &[JokerEffect]) -> JokerEffect {
        let mut result = effects[0].clone();

        for effect in &effects[1..] {
            result.chips = result.chips.max(effect.chips);
            result.mult = result.mult.max(effect.mult);
            result.money = result.money.max(effect.money);
            result.hand_size_mod = result.hand_size_mod.max(effect.hand_size_mod);
            result.discard_mod = result.discard_mod.max(effect.discard_mod);
            result.sell_value_increase = result.sell_value_increase.max(effect.sell_value_increase);
            result.mult_multiplier = result.mult_multiplier.max(effect.mult_multiplier);
        }

        result
    }

    /// Take minimum values for all effects
    fn min_effects(&self, effects: &[JokerEffect]) -> JokerEffect {
        let mut result = effects[0].clone();

        for effect in &effects[1..] {
            result.chips = result.chips.min(effect.chips);
            result.mult = result.mult.min(effect.mult);
            result.money = result.money.min(effect.money);
            result.hand_size_mod = result.hand_size_mod.min(effect.hand_size_mod);
            result.discard_mod = result.discard_mod.min(effect.discard_mod);
            result.sell_value_increase = result.sell_value_increase.min(effect.sell_value_increase);
            result.mult_multiplier = result.mult_multiplier.min(effect.mult_multiplier);
        }

        result
    }

    /// Check if an effect is empty (no-op)
    fn is_empty_effect(&self, effect: &JokerEffect) -> bool {
        effect.chips == 0
            && effect.mult == 0
            && effect.money == 0
            && effect.mult_multiplier == 0.0  // Default trait gives 0.0 for f32
            && effect.retrigger == 0
            && !effect.destroy_self
            && effect.destroy_others.is_empty()
            && effect.transform_cards.is_empty()
            && effect.hand_size_mod == 0
            && effect.discard_mod == 0
            && effect.sell_value_increase == 0
            && effect.message.is_none()
    }

    /// Get processing priority for a joker (can be customized)
    fn get_joker_priority(&self, _joker_id: JokerId) -> EffectPriority {
        // Default implementation - can be extended to read from joker metadata
        EffectPriority::Normal
    }

    /// Validate a single effect
    fn validate_effect(&self, effect: &JokerEffect) -> Result<(), String> {
        // Basic validation rules
        if effect.mult_multiplier < 0.0 {
            return Err("Mult multiplier cannot be negative".to_string());
        }

        if effect.retrigger > 10 {
            return Err("Too many retriggers - maximum is 10".to_string());
        }

        // Additional validation can be added here
        Ok(())
    }

    /// Clear the effect cache (useful for testing or memory management)
    pub fn clear_cache(&mut self) {
        self.effect_cache.clear();
    }

    /// Update processing context
    pub fn set_context(&mut self, context: ProcessingContext) {
        self.context = context;
    }

    /// Get current processing context
    pub fn context(&self) -> &ProcessingContext {
        &self.context
    }
}

impl Default for JokerEffectProcessor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_effect_detection() {
        let processor = JokerEffectProcessor::new();
        let empty_effect = JokerEffect::new();
        assert!(processor.is_empty_effect(&empty_effect));

        let non_empty_effect = JokerEffect::new().with_chips(5);
        assert!(!processor.is_empty_effect(&non_empty_effect));
    }

    #[test]
    fn test_sum_effects() {
        let processor = JokerEffectProcessor::new();
        let effects = vec![
            JokerEffect::new().with_chips(10).with_mult(2),
            JokerEffect::new().with_chips(5).with_mult(3),
        ];

        let result = processor.sum_effects(&effects);
        assert_eq!(result.chips, 15);
        assert_eq!(result.mult, 5);
    }

    #[test]
    fn test_multiplicative_effects() {
        let processor = JokerEffectProcessor::new();
        let effects = vec![
            JokerEffect::new().with_mult_multiplier(1.5),
            JokerEffect::new().with_mult_multiplier(2.0),
        ];

        let result = processor.sum_effects(&effects);
        assert_eq!(result.mult_multiplier, 3.0); // 1.5 * 2.0
    }

    #[test]
    fn test_effect_validation() {
        let processor = JokerEffectProcessor::new();

        let valid_effect = JokerEffect::new().with_mult_multiplier(1.5);
        assert!(processor.validate_effect(&valid_effect).is_ok());

        let invalid_effect = JokerEffect::new().with_mult_multiplier(-1.0);
        assert!(processor.validate_effect(&invalid_effect).is_err());
    }

    #[test]
    fn test_complex_effect_accumulation() {
        let processor = JokerEffectProcessor::new();

        // Create multiple effects with different properties
        let effects = vec![
            WeightedEffect {
                effect: JokerEffect::new().with_chips(10).with_mult(2),
                priority: EffectPriority::Low,
                source_joker_id: JokerId::Joker,
                is_retriggered: false,
            },
            WeightedEffect {
                effect: JokerEffect::new().with_chips(5).with_mult(3).with_money(1),
                priority: EffectPriority::Normal,
                source_joker_id: JokerId::GreedyJoker,
                is_retriggered: false,
            },
            WeightedEffect {
                effect: JokerEffect::new().with_mult_multiplier(1.5).with_money(2),
                priority: EffectPriority::High,
                source_joker_id: JokerId::LustyJoker,
                is_retriggered: false,
            },
        ];

        let result = processor.accumulate_effects(&effects);

        // Verify accumulation
        assert_eq!(result.chips, 15); // 10 + 5
        assert_eq!(result.mult, 5); // 2 + 3
        assert_eq!(result.money, 3); // 1 + 2
        assert_eq!(result.mult_multiplier, 1.5); // Only one multiplier
    }

    #[test]
    fn test_conflict_resolution_strategies() {
        let processor = JokerEffectProcessor::new();

        let effects = vec![
            JokerEffect::new().with_chips(10).with_mult(5),
            JokerEffect::new().with_chips(20).with_mult(3),
            JokerEffect::new().with_chips(5).with_mult(8),
        ];

        // Test Sum strategy (default)
        let sum_result = processor.sum_effects(&effects);
        assert_eq!(sum_result.chips, 35); // 10 + 20 + 5
        assert_eq!(sum_result.mult, 16); // 5 + 3 + 8

        // Test Maximum strategy
        let max_result = processor.max_effects(&effects);
        assert_eq!(max_result.chips, 20); // max(10, 20, 5)
        assert_eq!(max_result.mult, 8); // max(5, 3, 8)

        // Test Minimum strategy
        let min_result = processor.min_effects(&effects);
        assert_eq!(min_result.chips, 5); // min(10, 20, 5)
        assert_eq!(min_result.mult, 3); // min(5, 3, 8)
    }

    #[test]
    fn test_priority_ordering() {
        let mut weighted_effects = vec![
            WeightedEffect {
                effect: JokerEffect::new().with_chips(10),
                priority: EffectPriority::High,
                source_joker_id: JokerId::Joker,
                is_retriggered: false,
            },
            WeightedEffect {
                effect: JokerEffect::new().with_chips(20),
                priority: EffectPriority::Low,
                source_joker_id: JokerId::GreedyJoker,
                is_retriggered: false,
            },
            WeightedEffect {
                effect: JokerEffect::new().with_chips(30),
                priority: EffectPriority::Critical,
                source_joker_id: JokerId::LustyJoker,
                is_retriggered: false,
            },
        ];

        // Sort by priority (higher priority applied later)
        weighted_effects.sort_by_key(|we| we.priority);

        // Verify order: Low, High, Critical
        assert_eq!(weighted_effects[0].priority, EffectPriority::Low);
        assert_eq!(weighted_effects[1].priority, EffectPriority::High);
        assert_eq!(weighted_effects[2].priority, EffectPriority::Critical);
    }

    #[test]
    fn test_retriggering_logic() {
        let mut processor = JokerEffectProcessor::new();

        let weighted_effects = vec![
            WeightedEffect {
                effect: JokerEffect::new().with_chips(10).with_mult(2),
                priority: EffectPriority::Normal,
                source_joker_id: JokerId::Joker,
                is_retriggered: false,
            },
            WeightedEffect {
                effect: JokerEffect {
                    chips: 5,
                    mult: 1,
                    retrigger: 2, // This effect should retrigger 2 times
                    ..Default::default()
                },
                priority: EffectPriority::Normal,
                source_joker_id: JokerId::GreedyJoker,
                is_retriggered: false,
            },
        ];

        let result = processor.process_weighted_effects(weighted_effects, 0);

        // Should have processed 2 original + 2 retriggered = 4 total
        assert_eq!(result.jokers_processed, 4);
        assert_eq!(result.retriggered_count, 2);

        // Accumulated effect should include retriggered effects
        // Original: 10+5=15 chips, 2+1=3 mult
        // Retriggered: +5+5=10 chips, +1+1=2 mult
        // Total: 25 chips, 5 mult
        assert_eq!(result.accumulated_effect.chips, 25);
        assert_eq!(result.accumulated_effect.mult, 5);
    }

    #[test]
    fn test_retrigger_limit_protection() {
        let mut processor = JokerEffectProcessor::new();

        // Set a low retrigger limit for testing
        processor.context.max_retriggered_effects = 3;

        let weighted_effects = vec![WeightedEffect {
            effect: JokerEffect {
                chips: 10,
                retrigger: 10, // Would cause 10 retriggers, but limit is 3
                ..Default::default()
            },
            priority: EffectPriority::Normal,
            source_joker_id: JokerId::Joker,
            is_retriggered: false,
        }];

        let result = processor.process_weighted_effects(weighted_effects, 0);

        // Should have hit the limit
        assert_eq!(result.retriggered_count, 3);
        assert!(!result.errors.is_empty());
        assert!(matches!(
            result.errors[0],
            EffectProcessingError::TooManyRetriggers(3)
        ));
    }

    #[test]
    fn test_effect_validation_in_processing() {
        let mut processor = JokerEffectProcessor::new();
        processor.context.validate_effects = true;

        let weighted_effects = vec![
            WeightedEffect {
                effect: JokerEffect::new().with_chips(10),
                priority: EffectPriority::Normal,
                source_joker_id: JokerId::Joker,
                is_retriggered: false,
            },
            WeightedEffect {
                effect: JokerEffect::new().with_mult_multiplier(-1.0), // Invalid
                priority: EffectPriority::Normal,
                source_joker_id: JokerId::GreedyJoker,
                is_retriggered: false,
            },
        ];

        let result = processor.process_weighted_effects(weighted_effects, 0);

        // Should have validation errors
        assert!(!result.errors.is_empty());
        assert!(matches!(
            result.errors[0],
            EffectProcessingError::ValidationFailed(_)
        ));
    }

    #[test]
    fn test_empty_effects_handling() {
        let processor = JokerEffectProcessor::new();

        let weighted_effects = vec![WeightedEffect {
            effect: JokerEffect::new(), // Empty effect
            priority: EffectPriority::Normal,
            source_joker_id: JokerId::Joker,
            is_retriggered: false,
        }];

        let result = processor.accumulate_effects(&weighted_effects);

        // Should produce an empty effect
        assert!(processor.is_empty_effect(&result));
    }

    #[test]
    fn test_multiplicative_effects_combination() {
        let processor = JokerEffectProcessor::new();

        let effects = vec![
            JokerEffect::new().with_mult_multiplier(1.5),
            JokerEffect::new().with_mult_multiplier(2.0),
            JokerEffect::new().with_mult_multiplier(1.2),
        ];

        let result = processor.sum_effects(&effects);

        // Should multiply together: 1.5 * 2.0 * 1.2 = 3.6
        assert!((result.mult_multiplier - 3.6).abs() < 0.001);
    }

    #[test]
    fn test_processing_context_modification() {
        let mut processor = JokerEffectProcessor::new();

        // Test context modification
        let mut new_context = ProcessingContext::default();
        new_context.processing_mode = ProcessingMode::Delayed;
        new_context.resolution_strategy = ConflictResolutionStrategy::Maximum;
        new_context.validate_effects = false;

        processor.set_context(new_context.clone());

        assert_eq!(processor.context().processing_mode, ProcessingMode::Delayed);
        assert_eq!(
            processor.context().resolution_strategy,
            ConflictResolutionStrategy::Maximum
        );
        assert!(!processor.context().validate_effects);
    }

    #[test]
    fn test_processing_result_structure() {
        let mut processor = JokerEffectProcessor::new();

        let weighted_effects = vec![WeightedEffect {
            effect: JokerEffect::new().with_chips(10).with_mult(5),
            priority: EffectPriority::Normal,
            source_joker_id: JokerId::Joker,
            is_retriggered: false,
        }];

        let result = processor.process_weighted_effects(weighted_effects, 0);

        // Verify structure
        assert_eq!(result.jokers_processed, 1);
        assert_eq!(result.retriggered_count, 0);
        assert!(result.errors.is_empty());
        // Processing time can be 0 on fast systems
        assert_eq!(result.accumulated_effect.chips, 10);
        assert_eq!(result.accumulated_effect.mult, 5);
    }

    #[test]
    fn test_cache_functionality() {
        let mut processor = JokerEffectProcessor::new();

        // The cache is currently simple - just test that clear works
        processor.clear_cache();
        // Cache should be empty after clear (no direct way to verify size)
        // This test ensures the method exists and doesn't panic
    }
}
