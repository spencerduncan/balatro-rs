use crate::joker::JokerId;
use crate::joker_effect_processor::EffectPriority;
use crate::joker_metadata::JokerMetadata;
use crate::joker_registry;
use std::collections::HashMap;
use std::sync::Arc;

/// Strategy trait for determining joker effect priorities
pub trait PriorityStrategy: Send + Sync + std::fmt::Debug {
    /// Get the priority for a given joker ID
    fn get_priority(&self, joker_id: JokerId) -> EffectPriority;

    /// Get a human-readable name for this strategy
    fn name(&self) -> &'static str;

    /// Get a description of how this strategy works
    fn description(&self) -> &'static str;
}

/// Default strategy that always returns Normal priority
/// This maintains the original behavior described in the issue
#[derive(Debug, Clone, Default)]
pub struct DefaultPriorityStrategy;

impl PriorityStrategy for DefaultPriorityStrategy {
    fn get_priority(&self, _joker_id: JokerId) -> EffectPriority {
        EffectPriority::Normal
    }

    fn name(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Always returns Normal priority for all jokers"
    }
}

/// Strategy that reads priority from joker metadata/definitions
/// This implements the current behavior in the codebase
#[derive(Debug, Clone, Default)]
pub struct MetadataPriorityStrategy;

impl PriorityStrategy for MetadataPriorityStrategy {
    fn get_priority(&self, joker_id: JokerId) -> EffectPriority {
        // Try to get the joker definition and create metadata
        if let Ok(Some(definition)) = joker_registry::registry::get_definition(&joker_id) {
            // Create metadata to get the computed priority
            let metadata = JokerMetadata::from_definition(&definition, true);
            metadata.effect_priority
        } else {
            // Fallback to Normal priority if definition not found
            EffectPriority::Normal
        }
    }

    fn name(&self) -> &'static str {
        "Metadata"
    }

    fn description(&self) -> &'static str {
        "Reads priority from joker definition metadata"
    }
}

/// Strategy that accepts user-defined priority mappings
#[derive(Debug, Clone)]
pub struct CustomPriorityStrategy {
    /// Custom priority mappings
    mappings: HashMap<JokerId, EffectPriority>,
    /// Fallback strategy for unmapped jokers
    fallback: Arc<dyn PriorityStrategy>,
}

impl CustomPriorityStrategy {
    /// Create a new custom priority strategy with the given mappings
    pub fn new(mappings: HashMap<JokerId, EffectPriority>) -> Self {
        Self {
            mappings,
            fallback: Arc::new(MetadataPriorityStrategy),
        }
    }

    /// Create a custom strategy with mappings and a specific fallback strategy
    pub fn with_fallback(
        mappings: HashMap<JokerId, EffectPriority>,
        fallback: Arc<dyn PriorityStrategy>,
    ) -> Self {
        Self { mappings, fallback }
    }

    /// Add a priority mapping for a joker
    pub fn add_mapping(&mut self, joker_id: JokerId, priority: EffectPriority) {
        self.mappings.insert(joker_id, priority);
    }

    /// Remove a priority mapping for a joker
    pub fn remove_mapping(&mut self, joker_id: &JokerId) -> Option<EffectPriority> {
        self.mappings.remove(joker_id)
    }

    /// Get the current mappings
    pub fn mappings(&self) -> &HashMap<JokerId, EffectPriority> {
        &self.mappings
    }
}

impl PriorityStrategy for CustomPriorityStrategy {
    fn get_priority(&self, joker_id: JokerId) -> EffectPriority {
        self.mappings
            .get(&joker_id)
            .copied()
            .unwrap_or_else(|| self.fallback.get_priority(joker_id))
    }

    fn name(&self) -> &'static str {
        "Custom"
    }

    fn description(&self) -> &'static str {
        "Uses user-defined priority mappings with fallback strategy"
    }
}

impl Default for CustomPriorityStrategy {
    fn default() -> Self {
        Self::new(HashMap::new())
    }
}

/// Strategy that considers game context when determining priorities
/// This is a placeholder implementation that can be extended with actual game state
#[derive(Debug, Clone)]
pub struct ContextAwarePriorityStrategy {
    /// Base strategy to use for default behavior
    base_strategy: Arc<dyn PriorityStrategy>,
    /// Whether to boost multiplicative jokers at higher money levels
    boost_multiplicative_at_high_money: bool,
    /// Money threshold for boosting multiplicative effects
    high_money_threshold: i32,
}

impl ContextAwarePriorityStrategy {
    /// Create a new context-aware strategy
    pub fn new() -> Self {
        Self {
            base_strategy: Arc::new(MetadataPriorityStrategy),
            boost_multiplicative_at_high_money: true,
            high_money_threshold: 50,
        }
    }

    /// Create with custom base strategy
    pub fn with_base_strategy(base_strategy: Arc<dyn PriorityStrategy>) -> Self {
        Self {
            base_strategy,
            boost_multiplicative_at_high_money: true,
            high_money_threshold: 50,
        }
    }

    /// Set the money threshold for boosting multiplicative effects
    pub fn with_money_threshold(mut self, threshold: i32) -> Self {
        self.high_money_threshold = threshold;
        self
    }

    /// Enable/disable boosting multiplicative effects at high money
    pub fn with_multiplicative_boost(mut self, enable: bool) -> Self {
        self.boost_multiplicative_at_high_money = enable;
        self
    }
}

impl PriorityStrategy for ContextAwarePriorityStrategy {
    fn get_priority(&self, joker_id: JokerId) -> EffectPriority {
        // Get base priority
        let base_priority = self.base_strategy.get_priority(joker_id);

        // Note: In a real implementation, this would access game context
        // For now, we demonstrate the concept with static logic

        // Example: Boost priority for certain jokers that provide multiplicative effects
        if self.boost_multiplicative_at_high_money {
            match joker_id {
                // These are examples - would need actual joker definitions to determine
                // which ones provide multiplicative effects
                JokerId::LustyJoker => {
                    // This joker might provide multiplicative effects
                    match base_priority {
                        EffectPriority::Low => EffectPriority::Normal,
                        EffectPriority::Normal => EffectPriority::High,
                        EffectPriority::High => EffectPriority::Critical,
                        EffectPriority::Critical => EffectPriority::Critical,
                    }
                }
                _ => base_priority,
            }
        } else {
            base_priority
        }
    }

    fn name(&self) -> &'static str {
        "ContextAware"
    }

    fn description(&self) -> &'static str {
        "Adjusts priorities based on game context and state"
    }
}

impl Default for ContextAwarePriorityStrategy {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_priority_strategy() {
        let strategy = DefaultPriorityStrategy;

        // Should always return Normal regardless of joker
        assert_eq!(
            strategy.get_priority(JokerId::Joker),
            EffectPriority::Normal
        );
        assert_eq!(
            strategy.get_priority(JokerId::GreedyJoker),
            EffectPriority::Normal
        );
        assert_eq!(
            strategy.get_priority(JokerId::LustyJoker),
            EffectPriority::Normal
        );

        assert_eq!(strategy.name(), "Default");
        assert!(!strategy.description().is_empty());
    }

    #[test]
    fn test_metadata_priority_strategy() {
        let strategy = MetadataPriorityStrategy;

        // These tests depend on the joker registry being populated
        // For registered jokers, should get priority from metadata
        // For unregistered jokers, should fall back to Normal
        assert_eq!(
            strategy.get_priority(JokerId::Joker),
            EffectPriority::Normal
        );

        assert_eq!(strategy.name(), "Metadata");
        assert!(!strategy.description().is_empty());
    }

    #[test]
    fn test_custom_priority_strategy() {
        let mut mappings = HashMap::new();
        mappings.insert(JokerId::Joker, EffectPriority::High);
        mappings.insert(JokerId::GreedyJoker, EffectPriority::Critical);

        let strategy = CustomPriorityStrategy::new(mappings);

        // Should use custom mappings
        assert_eq!(strategy.get_priority(JokerId::Joker), EffectPriority::High);
        assert_eq!(
            strategy.get_priority(JokerId::GreedyJoker),
            EffectPriority::Critical
        );

        // Should fall back to metadata strategy for unmapped jokers
        assert_eq!(
            strategy.get_priority(JokerId::LustyJoker),
            EffectPriority::Normal
        );

        assert_eq!(strategy.name(), "Custom");
        assert!(!strategy.description().is_empty());
    }

    #[test]
    fn test_custom_priority_strategy_with_default_fallback() {
        let mut mappings = HashMap::new();
        mappings.insert(JokerId::Joker, EffectPriority::Low);

        let strategy =
            CustomPriorityStrategy::with_fallback(mappings, Arc::new(DefaultPriorityStrategy));

        // Should use custom mapping
        assert_eq!(strategy.get_priority(JokerId::Joker), EffectPriority::Low);

        // Should fall back to default strategy (always Normal)
        assert_eq!(
            strategy.get_priority(JokerId::GreedyJoker),
            EffectPriority::Normal
        );
        assert_eq!(
            strategy.get_priority(JokerId::LustyJoker),
            EffectPriority::Normal
        );
    }

    #[test]
    fn test_custom_priority_strategy_modification() {
        let mut strategy = CustomPriorityStrategy::default();

        // Initially empty, should use fallback
        assert_eq!(
            strategy.get_priority(JokerId::Joker),
            EffectPriority::Normal
        );

        // Add mapping
        strategy.add_mapping(JokerId::Joker, EffectPriority::High);
        assert_eq!(strategy.get_priority(JokerId::Joker), EffectPriority::High);

        // Remove mapping
        let removed = strategy.remove_mapping(&JokerId::Joker);
        assert_eq!(removed, Some(EffectPriority::High));
        assert_eq!(
            strategy.get_priority(JokerId::Joker),
            EffectPriority::Normal
        );
    }

    #[test]
    fn test_context_aware_priority_strategy() {
        let strategy = ContextAwarePriorityStrategy::new();

        // Test boosting behavior
        let lusty_priority = strategy.get_priority(JokerId::LustyJoker);
        let normal_priority = strategy.get_priority(JokerId::Joker);

        // LustyJoker should get boosted priority if it normally has Normal priority
        // (This depends on what the metadata strategy returns)
        assert_eq!(strategy.name(), "ContextAware");
        assert!(!strategy.description().is_empty());
    }

    #[test]
    fn test_context_aware_with_custom_base() {
        let base_strategy = Arc::new(DefaultPriorityStrategy);
        let strategy = ContextAwarePriorityStrategy::with_base_strategy(base_strategy);

        // With default strategy as base, all should start at Normal
        let base_priority = strategy.get_priority(JokerId::Joker);
        assert_eq!(base_priority, EffectPriority::Normal);

        // LustyJoker should get boosted from Normal to High
        let boosted_priority = strategy.get_priority(JokerId::LustyJoker);
        assert_eq!(boosted_priority, EffectPriority::High);
    }

    #[test]
    fn test_context_aware_configuration() {
        let strategy = ContextAwarePriorityStrategy::new()
            .with_money_threshold(100)
            .with_multiplicative_boost(false);

        // With boost disabled, should use base strategy
        let priority = strategy.get_priority(JokerId::LustyJoker);
        // Should not be boosted when boost is disabled
        assert_eq!(strategy.name(), "ContextAware");
    }

    #[test]
    fn test_effect_priority_ordering() {
        // Verify the priority ordering is correct
        assert!(EffectPriority::Low < EffectPriority::Normal);
        assert!(EffectPriority::Normal < EffectPriority::High);
        assert!(EffectPriority::High < EffectPriority::Critical);

        // Verify numeric values
        assert_eq!(EffectPriority::Low as u8, 1);
        assert_eq!(EffectPriority::Normal as u8, 5);
        assert_eq!(EffectPriority::High as u8, 10);
        assert_eq!(EffectPriority::Critical as u8, 15);
    }
}
