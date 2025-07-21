use crate::card::Card;
use crate::hand::SelectHand;
use crate::joker::{GameContext, Joker, JokerEffect, JokerId};
use crate::joker_metadata::JokerMetadata;
use crate::joker_registry;
use crate::priority_strategy::{MetadataPriorityStrategy, PriorityStrategy};
pub use crate::priority_strategy::{ContextAwarePriorityStrategy, CustomPriorityStrategy, DefaultPriorityStrategy};
#[cfg(feature = "python")]
use pyo3::pyclass;
use std::collections::HashMap;
use std::sync::Arc;
use std::hash::{Hash, Hasher};
use std::time::{Duration, Instant};

/// Priority level for effect processing
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[cfg_attr(feature = "python", pyclass(eq))]
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
#[derive(Debug, Clone, PartialEq, Hash)]
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

/// Configuration for effect cache behavior
#[derive(Debug, Clone)]
pub struct CacheConfig {
    /// Maximum number of cache entries
    pub max_entries: usize,
    /// Time-to-live for cache entries in seconds
    pub ttl_seconds: u64,
    /// Whether caching is enabled
    pub enabled: bool,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            max_entries: 1000,
            ttl_seconds: 300, // 5 minutes
            enabled: true,
        }
    }
}

/// Cache metrics for performance monitoring
#[derive(Debug, Clone, Default)]
pub struct CacheMetrics {
    /// Total number of cache lookups
    pub total_lookups: u64,
    /// Number of cache hits
    pub hits: u64,
    /// Number of cache misses
    pub misses: u64,
    /// Total time saved by cache hits (in microseconds)
    pub time_saved_micros: u64,
    /// Number of cache evictions due to size limits
    pub evictions: u64,
    /// Number of cache expiries due to TTL
    pub expiries: u64,
}

impl CacheMetrics {
    /// Calculate cache hit ratio
    pub fn hit_ratio(&self) -> f64 {
        if self.total_lookups == 0 {
            0.0
        } else {
            self.hits as f64 / self.total_lookups as f64
        }
    }

    /// Calculate average time saved per hit in microseconds
    pub fn avg_time_saved_per_hit(&self) -> f64 {
        if self.hits == 0 {
            0.0
        } else {
            self.time_saved_micros as f64 / self.hits as f64
        }
    }
}

/// Cache entry with expiration tracking
#[derive(Debug, Clone)]
struct CacheEntry {
    /// Cached processing result
    result: ProcessingResult,
    /// When this entry was created
    created_at: Instant,
    /// When this entry was last accessed
    last_accessed: Instant,
}

impl CacheEntry {
    fn new(result: ProcessingResult) -> Self {
        let now = Instant::now();
        Self {
            result,
            created_at: now,
            last_accessed: now,
        }
    }

    fn is_expired(&self, ttl: Duration) -> bool {
        self.created_at.elapsed() > ttl
    }

    fn touch(&mut self) {
        self.last_accessed = Instant::now();
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
    /// Strategy for determining joker effect priorities
    pub priority_strategy: Arc<dyn PriorityStrategy>,
    /// Cache configuration
    pub cache_config: CacheConfig,
}

impl Default for ProcessingContext {
    fn default() -> Self {
        Self {
            processing_mode: ProcessingMode::Immediate,
            resolution_strategy: ConflictResolutionStrategy::default(),
            validate_effects: true,
            max_retriggered_effects: 100,
            priority_strategy: Arc::new(MetadataPriorityStrategy::default()),
            cache_config: CacheConfig::default(),
        }
    }
}

impl ProcessingContext {
    /// Create a new builder for ProcessingContext.
    ///
    /// This provides a fluent API for configuring ProcessingContext instances.
    ///
    /// # Examples
    ///
    /// ```
    /// # use balatro_rs::joker_effect_processor::{ProcessingContext, ProcessingMode, ConflictResolutionStrategy};
    /// let context = ProcessingContext::builder()
    ///     .processing_mode(ProcessingMode::Delayed)
    ///     .resolution_strategy(ConflictResolutionStrategy::Maximum)
    ///     .validate_effects(false)
    ///     .max_retriggered_effects(50)
    ///     .build();
    /// ```
    pub fn builder() -> ProcessingContextBuilder {
        ProcessingContextBuilder::new()
    }
    
    /// Set the priority strategy for this context
    pub fn with_priority_strategy(mut self, strategy: Arc<dyn PriorityStrategy>) -> Self {
        self.priority_strategy = strategy;
        self
    }
}

/// Builder for creating ProcessingContext instances with a fluent API.
///
/// The ProcessingContextBuilder provides a convenient way to configure
/// ProcessingContext instances using method chaining. All configuration
/// options are optional, with sensible defaults being used for unspecified
/// fields.
///
/// # Examples
///
/// ## Basic usage with all options
///
/// ```
/// # use balatro_rs::joker_effect_processor::{ProcessingContext, ProcessingMode, ConflictResolutionStrategy};
/// let context = ProcessingContext::builder()
///     .processing_mode(ProcessingMode::Delayed)
///     .resolution_strategy(ConflictResolutionStrategy::Maximum)
///     .validate_effects(false)
///     .max_retriggered_effects(50)
///     .build();
/// ```
///
/// ## Partial configuration (uses defaults for unspecified fields)
///
/// ```
/// # use balatro_rs::joker_effect_processor::{ProcessingContext, ProcessingMode};
/// let context = ProcessingContext::builder()
///     .processing_mode(ProcessingMode::Delayed)
///     .validate_effects(false)
///     .build();
/// ```
///
/// ## Builder reuse
///
/// ```
/// # use balatro_rs::joker_effect_processor::{ProcessingContext, ProcessingMode, ConflictResolutionStrategy};
/// let base_builder = ProcessingContext::builder()
///     .processing_mode(ProcessingMode::Delayed)
///     .resolution_strategy(ConflictResolutionStrategy::Maximum);
///
/// let context1 = base_builder.clone().validate_effects(true).build();
/// let context2 = base_builder.validate_effects(false).build();
/// ```
#[derive(Debug, Clone)]
pub struct ProcessingContextBuilder {
    processing_mode: ProcessingMode,
    resolution_strategy: ConflictResolutionStrategy,
    validate_effects: bool,
    max_retriggered_effects: u32,
    priority_strategy: Arc<dyn PriorityStrategy>,
}

impl ProcessingContextBuilder {
    /// Create a new builder with default values.
    ///
    /// The builder starts with the same default values as [`ProcessingContext::default()`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use balatro_rs::joker_effect_processor::ProcessingContextBuilder;
    /// let builder = ProcessingContextBuilder::new();
    /// let context = builder.build();
    /// ```
    pub fn new() -> Self {
        let default_context = ProcessingContext::default();
        Self {
            processing_mode: default_context.processing_mode,
            resolution_strategy: default_context.resolution_strategy,
            validate_effects: default_context.validate_effects,
            max_retriggered_effects: default_context.max_retriggered_effects,
            priority_strategy: default_context.priority_strategy,
        }
    }

    /// Set the processing mode for joker effects.
    ///
    /// The processing mode determines when effects are applied:
    /// - `ProcessingMode::Immediate`: Effects are applied as soon as they are generated
    /// - `ProcessingMode::Delayed`: Effects are collected and applied in batch
    ///
    /// # Arguments
    ///
    /// * `mode` - The processing mode to use
    ///
    /// # Examples
    ///
    /// ```
    /// # use balatro_rs::joker_effect_processor::{ProcessingContext, ProcessingMode};
    /// let context = ProcessingContext::builder()
    ///     .processing_mode(ProcessingMode::Delayed)
    ///     .build();
    /// ```
    pub fn processing_mode(mut self, mode: ProcessingMode) -> Self {
        self.processing_mode = mode;
        self
    }

    /// Set the conflict resolution strategy for combining multiple effects.
    ///
    /// When multiple jokers produce conflicting effects, the resolution strategy
    /// determines how they are combined:
    /// - `ConflictResolutionStrategy::Sum`: Add all effect values together
    /// - `ConflictResolutionStrategy::Maximum`: Use the highest effect value
    /// - `ConflictResolutionStrategy::Minimum`: Use the lowest effect value
    ///
    /// # Arguments
    ///
    /// * `strategy` - The conflict resolution strategy to use
    ///
    /// # Examples
    ///
    /// ```
    /// # use balatro_rs::joker_effect_processor::{ProcessingContext, ConflictResolutionStrategy};
    /// let context = ProcessingContext::builder()
    ///     .resolution_strategy(ConflictResolutionStrategy::Maximum)
    ///     .build();
    /// ```
    pub fn resolution_strategy(mut self, strategy: ConflictResolutionStrategy) -> Self {
        self.resolution_strategy = strategy;
        self
    }

    /// Set whether to validate effects during processing.
    ///
    /// When enabled, each effect is validated against predefined rules before
    /// being applied. This can help catch invalid effects but adds processing overhead.
    ///
    /// # Arguments
    ///
    /// * `validate` - Whether to enable effect validation
    ///
    /// # Examples
    ///
    /// ```
    /// # use balatro_rs::joker_effect_processor::ProcessingContext;
    /// // Enable validation for debugging
    /// let debug_context = ProcessingContext::builder()
    ///     .validate_effects(true)
    ///     .build();
    ///
    /// // Disable validation for performance
    /// let production_context = ProcessingContext::builder()
    ///     .validate_effects(false)
    ///     .build();
    /// ```
    pub fn validate_effects(mut self, validate: bool) -> Self {
        self.validate_effects = validate;
        self
    }

    /// Set the maximum number of retriggered effects allowed.
    ///
    /// This prevents infinite loops from jokers that retrigger other jokers.
    /// When the limit is reached, additional retriggers are ignored and an error
    /// is recorded.
    ///
    /// # Arguments
    ///
    /// * `max` - The maximum number of retriggered effects (0 disables retriggering)
    ///
    /// # Examples
    ///
    /// ```
    /// # use balatro_rs::joker_effect_processor::ProcessingContext;
    /// // Conservative limit for safety
    /// let safe_context = ProcessingContext::builder()
    ///     .max_retriggered_effects(10)
    ///     .build();
    ///
    /// // Higher limit for complex interactions
    /// let complex_context = ProcessingContext::builder()
    ///     .max_retriggered_effects(200)
    ///     .build();
    /// ```
    pub fn max_retriggered_effects(mut self, max: u32) -> Self {
        self.max_retriggered_effects = max;
        self
    }
    
    /// Set the priority strategy
    pub fn priority_strategy(mut self, strategy: Arc<dyn PriorityStrategy>) -> Self {
        self.priority_strategy = strategy;
        self
    }

    /// Build the final ProcessingContext instance.
    ///
    /// Consumes the builder and returns a configured ProcessingContext.
    ///
    /// # Examples
    ///
    /// ```
    /// # use balatro_rs::joker_effect_processor::{ProcessingContext, ProcessingMode};
    /// let context = ProcessingContext::builder()
    ///     .processing_mode(ProcessingMode::Delayed)
    ///     .validate_effects(false)
    ///     .build();
    /// ```
    pub fn build(self) -> ProcessingContext {
        ProcessingContext {
            processing_mode: self.processing_mode,
            resolution_strategy: self.resolution_strategy,
            validate_effects: self.validate_effects,
            max_retriggered_effects: self.max_retriggered_effects,
            priority_strategy: self.priority_strategy,
            cache_config: CacheConfig::default(),
        }
    }
}

impl Default for ProcessingContextBuilder {
    fn default() -> Self {
        Self::new()
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
    effect_cache: HashMap<String, CacheEntry>,
    /// Cache performance metrics
    cache_metrics: CacheMetrics,
}

impl JokerEffectProcessor {
    /// Create a new effect processor with default settings
    pub fn new() -> Self {
        Self {
            context: ProcessingContext::default(),
            effect_cache: HashMap::new(),
            cache_metrics: CacheMetrics::default(),
        }
    }

    /// Create a processor with custom context
    pub fn with_context(context: ProcessingContext) -> Self {
        Self {
            context,
            effect_cache: HashMap::new(),
            cache_metrics: CacheMetrics::default(),
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

        // Generate cache key and check cache
        let cache_key = self.generate_hand_cache_key(jokers, game_context, hand);
        if let Some(cached_result) = self.check_cache(&cache_key) {
            return cached_result;
        }

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
        let result = self
            .process_weighted_effects(weighted_effects, start_time.elapsed().as_micros() as u64);

        // Store result in cache
        self.store_in_cache(cache_key, result.clone());

        result
    }

    /// Process effects when individual cards are scored
    pub fn process_card_effects(
        &mut self,
        jokers: &[Box<dyn Joker>],
        game_context: &mut GameContext,
        card: &Card,
    ) -> ProcessingResult {
        let start_time = std::time::Instant::now();

        // Generate cache key and check cache
        let cache_key = self.generate_card_cache_key(jokers, game_context, card);
        if let Some(cached_result) = self.check_cache(&cache_key) {
            return cached_result;
        }

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
        let result = self
            .process_weighted_effects(weighted_effects, start_time.elapsed().as_micros() as u64);

        // Store result in cache
        self.store_in_cache(cache_key, result.clone());

        result
    }

    /// Process a collection of weighted effects
    fn process_weighted_effects(
        &mut self,
        mut weighted_effects: Vec<WeightedEffect>,
        base_processing_time: u64,
    ) -> ProcessingResult {
        let mut errors = Vec::new();

        // Handle retriggering
        let (retriggered_count, retrigger_errors) = self.process_retriggers(&mut weighted_effects);
        errors.extend(retrigger_errors);

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

    /// Process retrigger effects for weighted effects
    ///
    /// This method handles the creation of retriggered copies of effects that have
    /// retrigger counts. It enforces a maximum retrigger limit to prevent infinite loops.
    ///
    /// # Arguments
    /// * `weighted_effects` - The vector of weighted effects to process retriggers for
    ///
    /// # Returns
    /// A tuple containing:
    /// * `u32` - The total number of retriggered effects created
    /// * `Vec<EffectProcessingError>` - Any errors encountered during retrigger processing
    fn process_retriggers(
        &self,
        weighted_effects: &mut Vec<WeightedEffect>,
    ) -> (u32, Vec<EffectProcessingError>) {
        let mut errors = Vec::new();
        let mut retriggered_count = 0;

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

        (retriggered_count, errors)
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
            && effect.mult_multiplier == 0.0  // Default trait gives 0.0 for f64
            && effect.retrigger == 0
            && !effect.destroy_self
            && effect.destroy_others.is_empty()
            && effect.transform_cards.is_empty()
            && effect.hand_size_mod == 0
            && effect.discard_mod == 0
            && effect.sell_value_increase == 0
            && effect.message.is_none()
    }

    /// Get processing priority for a joker using the configured strategy
    fn get_joker_priority(&self, joker_id: JokerId) -> EffectPriority {
        self.context.priority_strategy.get_priority(joker_id)
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

    /// Generate a deterministic cache key for hand effect processing
    fn generate_hand_cache_key(
        &self,
        jokers: &[Box<dyn Joker>],
        game_context: &GameContext,
        hand: &SelectHand,
    ) -> String {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();

        // Hash joker states
        for joker in jokers {
            joker.id().hash(&mut hasher);
            // Hash joker-specific state (you might need to add a method to get hashable state)
            // For now, we'll use the joker ID as a proxy
        }

        // Hash relevant game context
        game_context.money.hash(&mut hasher);
        game_context.mult.hash(&mut hasher);
        game_context.chips.hash(&mut hasher);
        game_context.hands_played.hash(&mut hasher);
        game_context.discards_used.hash(&mut hasher);
        game_context.ante.hash(&mut hasher);
        game_context.round.hash(&mut hasher);

        // Hash hand composition
        for card in hand.cards() {
            card.value.hash(&mut hasher);
            card.suit.hash(&mut hasher);
        }

        // Hash processing context settings that affect results
        self.context.resolution_strategy.hash(&mut hasher);
        self.context.max_retriggered_effects.hash(&mut hasher);

        format!("hand_{:x}", hasher.finish())
    }

    /// Generate a deterministic cache key for card effect processing
    fn generate_card_cache_key(
        &self,
        jokers: &[Box<dyn Joker>],
        game_context: &GameContext,
        card: &Card,
    ) -> String {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();

        // Hash joker states
        for joker in jokers {
            joker.id().hash(&mut hasher);
        }

        // Hash relevant game context
        game_context.money.hash(&mut hasher);
        game_context.mult.hash(&mut hasher);
        game_context.chips.hash(&mut hasher);
        game_context.ante.hash(&mut hasher);
        game_context.round.hash(&mut hasher);

        // Hash card
        card.value.hash(&mut hasher);
        card.suit.hash(&mut hasher);

        // Hash processing context settings
        self.context.resolution_strategy.hash(&mut hasher);
        self.context.max_retriggered_effects.hash(&mut hasher);

        format!("card_{:x}", hasher.finish())
    }

    /// Check cache for existing result and update metrics
    fn check_cache(&mut self, cache_key: &str) -> Option<ProcessingResult> {
        if !self.context.cache_config.enabled {
            return None;
        }

        self.cache_metrics.total_lookups += 1;

        // Check if entry exists and is not expired
        if let Some(entry) = self.effect_cache.get_mut(cache_key) {
            let ttl = Duration::from_secs(self.context.cache_config.ttl_seconds);

            if entry.is_expired(ttl) {
                // Remove expired entry
                self.effect_cache.remove(cache_key);
                self.cache_metrics.misses += 1;
                self.cache_metrics.expiries += 1;
                None
            } else {
                // Cache hit - update access time and metrics
                entry.touch();
                self.cache_metrics.hits += 1;
                self.cache_metrics.time_saved_micros += entry.result.processing_time_micros;
                Some(entry.result.clone())
            }
        } else {
            // Cache miss
            self.cache_metrics.misses += 1;
            None
        }
    }

    /// Store result in cache with eviction if necessary
    fn store_in_cache(&mut self, cache_key: String, result: ProcessingResult) {
        if !self.context.cache_config.enabled {
            return;
        }

        // Check if we need to evict entries to stay within size limits
        while self.effect_cache.len() >= self.context.cache_config.max_entries {
            self.evict_oldest_entry();
        }

        // Store the new entry
        let entry = CacheEntry::new(result);
        self.effect_cache.insert(cache_key, entry);
    }

    /// Evict the oldest (least recently accessed) cache entry
    fn evict_oldest_entry(&mut self) {
        let mut oldest_key = None;
        let mut oldest_time = Instant::now();

        for (key, entry) in &self.effect_cache {
            if entry.last_accessed < oldest_time {
                oldest_time = entry.last_accessed;
                oldest_key = Some(key.clone());
            }
        }

        if let Some(key) = oldest_key {
            self.effect_cache.remove(&key);
            self.cache_metrics.evictions += 1;
        }
    }

    /// Clean up expired cache entries
    fn cleanup_expired_entries(&mut self) {
        let ttl = Duration::from_secs(self.context.cache_config.ttl_seconds);
        let mut expired_keys = Vec::new();

        for (key, entry) in &self.effect_cache {
            if entry.is_expired(ttl) {
                expired_keys.push(key.clone());
            }
        }

        for key in expired_keys {
            self.effect_cache.remove(&key);
            self.cache_metrics.expiries += 1;
        }
    }

    /// Clear the effect cache (useful for testing or memory management)
    pub fn clear_cache(&mut self) {
        self.effect_cache.clear();
        // Reset metrics when clearing cache
        self.cache_metrics = CacheMetrics::default();
    }

    /// Update processing context
    pub fn set_context(&mut self, context: ProcessingContext) {
        self.context = context;
    }

    /// Get current processing context
    pub fn context(&self) -> &ProcessingContext {
        &self.context
    }

    /// Get cache performance metrics
    pub fn cache_metrics(&self) -> &CacheMetrics {
        &self.cache_metrics
    }

    /// Get current cache size
    pub fn cache_size(&self) -> usize {
        self.effect_cache.len()
    }

    /// Update cache configuration
    pub fn set_cache_config(&mut self, config: CacheConfig) {
        self.context.cache_config = config;
        // Clear cache if caching was disabled
        if !self.context.cache_config.enabled {
            self.clear_cache();
        }
    }

    /// Perform cache maintenance (cleanup expired entries)
    pub fn maintain_cache(&mut self) {
        self.cleanup_expired_entries();
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
    fn test_comprehensive_priority_system() {
        let processor = JokerEffectProcessor::new();

        // Test with jokers that are actually registered in the registry
        assert_eq!(
            processor.get_joker_priority(JokerId::Joker),
            EffectPriority::Normal
        );
        assert_eq!(
            processor.get_joker_priority(JokerId::GreedyJoker),
            EffectPriority::Normal
        );
        assert_eq!(
            processor.get_joker_priority(JokerId::LustyJoker),
            EffectPriority::Normal
        );

        // Test fallback behavior for unregistered jokers
        // These jokers exist in the enum but aren't registered in the registry
        assert_eq!(
            processor.get_joker_priority(JokerId::IceCream),
            EffectPriority::Normal
        );
        assert_eq!(
            processor.get_joker_priority(JokerId::EggJoker),
            EffectPriority::Normal
        );
        assert_eq!(
            processor.get_joker_priority(JokerId::SpaceJoker),
            EffectPriority::Normal
        );
    }

    #[test]
    fn test_priority_based_effect_ordering() {
        let processor = JokerEffectProcessor::new();

        // Create effects with manually assigned priorities to demonstrate the system works
        let mut weighted_effects = vec![
            WeightedEffect {
                effect: JokerEffect::new().with_chips(50),
                priority: EffectPriority::High, // High priority
                source_joker_id: JokerId::EggJoker,
                is_retriggered: false,
            },
            WeightedEffect {
                effect: JokerEffect::new().with_chips(10),
                priority: EffectPriority::Low, // Low priority
                source_joker_id: JokerId::IceCream,
                is_retriggered: false,
            },
            WeightedEffect {
                effect: JokerEffect::new().with_chips(30),
                priority: EffectPriority::Normal, // Normal priority
                source_joker_id: JokerId::Joker,
                is_retriggered: false,
            },
        ];

        // Sort by priority (lower values processed first)
        weighted_effects.sort_by_key(|we| we.priority);

        // Verify the ordering: Low (1) -> Normal (5) -> High (10)
        assert_eq!(weighted_effects[0].source_joker_id, JokerId::IceCream);
        assert_eq!(weighted_effects[0].priority, EffectPriority::Low);

        assert_eq!(weighted_effects[1].source_joker_id, JokerId::Joker);
        assert_eq!(weighted_effects[1].priority, EffectPriority::Normal);

        assert_eq!(weighted_effects[2].source_joker_id, JokerId::EggJoker);
        assert_eq!(weighted_effects[2].priority, EffectPriority::High);

        // Test that the accumulation works correctly with priority ordering
        let result = processor.accumulate_effects(&weighted_effects);

        // All chips should be summed: 10 + 30 + 50 = 90
        assert_eq!(result.chips, 90);
    }

    #[test]
    fn test_priority_system_with_multiplicative_effects() {
        let processor = JokerEffectProcessor::new();

        // Test that multiplicative effects would get critical priority
        // (We test the priority assignment logic, even though these specific jokers
        // might not actually have multiplicative effects in the current implementation)

        // Create effects simulating what would happen with multiplicative jokers
        let weighted_effects = vec![
            WeightedEffect {
                effect: JokerEffect::new().with_chips(10),
                priority: EffectPriority::Normal, // Additive effect
                source_joker_id: JokerId::Joker,
                is_retriggered: false,
            },
            WeightedEffect {
                effect: JokerEffect::new().with_mult_multiplier(2.0),
                priority: EffectPriority::Critical, // Multiplicative effect
                source_joker_id: JokerId::SpaceJoker,
                is_retriggered: false,
            },
            WeightedEffect {
                effect: JokerEffect::new().with_mult(5),
                priority: EffectPriority::High, // High priority additive
                source_joker_id: JokerId::EggJoker,
                is_retriggered: false,
            },
        ];

        let result = processor.accumulate_effects(&weighted_effects);

        // Additive effects: 10 chips, 5 mult
        // Multiplicative effect: 2.0x mult multiplier
        assert_eq!(result.chips, 10);
        assert_eq!(result.mult, 5);
        assert_eq!(result.mult_multiplier, 2.0);
    }

    #[test]
    fn test_priority_assignment_logic_directly() {
        // Test the priority assignment logic directly by importing and testing
        // the determine_effect_priority function from joker_metadata
        use crate::joker_metadata::determine_effect_priority;

        // Test multiplicative effects get Critical priority
        assert_eq!(
            determine_effect_priority(&JokerId::Joker, "multiplicative_mult", "X2 Mult"),
            EffectPriority::Critical
        );

        // Test destructive effects get Critical priority
        assert_eq!(
            determine_effect_priority(&JokerId::Joker, "special", "destroy all cards"),
            EffectPriority::Critical
        );

        // Test economy effects get High priority
        assert_eq!(
            determine_effect_priority(&JokerId::Joker, "economy", "Earn $5 when played"),
            EffectPriority::High
        );

        // Test hand modification effects get High priority
        assert_eq!(
            determine_effect_priority(&JokerId::Joker, "hand_modification", "+1 hand size"),
            EffectPriority::High
        );

        // Test specific joker overrides
        assert_eq!(
            determine_effect_priority(
                &JokerId::IceCream,
                "conditional_chips",
                "conditional effect"
            ),
            EffectPriority::Low
        );

        assert_eq!(
            determine_effect_priority(&JokerId::EggJoker, "special", "affects sell values"),
            EffectPriority::High
        );

        // Test standard jokers get Normal priority
        assert_eq!(
            determine_effect_priority(&JokerId::Joker, "additive_mult", "+4 Mult"),
            EffectPriority::Normal
        );
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
    fn test_cache_basic_functionality() {
        let mut processor = JokerEffectProcessor::new();

        // Test that cache starts empty
        assert_eq!(processor.cache_size(), 0);
        assert_eq!(processor.cache_metrics().total_lookups, 0);

        // Test clear cache functionality
        processor.clear_cache();
        assert_eq!(processor.cache_size(), 0);
        assert_eq!(processor.cache_metrics().total_lookups, 0);
    }

    #[test]
    fn test_cache_key_generation() {
        use crate::card::{Value, Suit};
        use crate::hand::SelectHand;
        use crate::joker::{GameContext, JokerId};
        use std::collections::HashMap;

        let processor = JokerEffectProcessor::new();

        // Create test data
        let mut game_context = GameContext {
            chips: 100,
            mult: 4,
            money: 100,
            ante: 1,
            round: 1,
            stage: &crate::stage::Stage::PreBlind(),
            hands_played: 0,
            discards_used: 0,
            jokers: &[],
            hand: &crate::hand::Hand::new(vec![]),
            discarded: &[],
            joker_state_manager: &std::sync::Arc::new(crate::joker_state::JokerStateManager::new()),
            hand_type_counts: &HashMap::new(),
            cards_in_deck: 52,
            stone_cards_in_deck: 0,
            rng: &balatro_rs::rng::GameRng::secure(),
        };

        let hand = SelectHand {
            cards: vec![
                Card { rank: Value::Ace, suit: Suit::Hearts },
                Card { rank: Value::King, suit: Suit::Hearts },
            ],
        };
        
        let card = Card { rank: Value::Queen, suit: Suit::Spades };
        
        let jokers: Vec<Box<dyn crate::joker::Joker>> = vec![];

        // Test that cache keys are deterministic
        let key1 = processor.generate_hand_cache_key(&jokers, &game_context, &hand);
        let key2 = processor.generate_hand_cache_key(&jokers, &game_context, &hand);
        assert_eq!(key1, key2);

        let card_key1 = processor.generate_card_cache_key(&jokers, &game_context, &card);
        let card_key2 = processor.generate_card_cache_key(&jokers, &game_context, &card);
        assert_eq!(card_key1, card_key2);

        // Test that different inputs produce different keys
        game_context.money = 200;
        let key3 = processor.generate_hand_cache_key(&jokers, &game_context, &hand);
        assert_ne!(key1, key3);
    }

    #[test]
    fn test_cache_hit_miss_metrics() {
        let mut processor = JokerEffectProcessor::new();

        // Create a dummy cache entry
        let cache_key = "test_key".to_string();
        let test_result = ProcessingResult {
            accumulated_effect: JokerEffect::new(),
            jokers_processed: 1,
            retriggered_count: 0,
            errors: vec![],
            processing_time_micros: 100,
        };

        // Store in cache
        processor.store_in_cache(cache_key.clone(), test_result.clone());
        assert_eq!(processor.cache_size(), 1);

        // Test cache hit
        let cached = processor.check_cache(&cache_key);
        assert!(cached.is_some());
        assert_eq!(processor.cache_metrics().hits, 1);
        assert_eq!(processor.cache_metrics().misses, 0);
        assert_eq!(processor.cache_metrics().total_lookups, 1);
        assert_eq!(processor.cache_metrics().time_saved_micros, 100);

        // Test cache miss
        let missed = processor.check_cache("nonexistent_key");
        assert!(missed.is_none());
        assert_eq!(processor.cache_metrics().hits, 1);
        assert_eq!(processor.cache_metrics().misses, 1);
        assert_eq!(processor.cache_metrics().total_lookups, 2);

        // Test hit ratio calculation
        assert!((processor.cache_metrics().hit_ratio() - 0.5).abs() < 0.001);
    }

    #[test]
    fn test_cache_expiration() {
        let mut processor = JokerEffectProcessor::new();

        // Set very short TTL for testing
        let mut config = CacheConfig::default();
        config.ttl_seconds = 0; // Immediate expiration
        processor.set_cache_config(config);

        let cache_key = "test_key".to_string();
        let test_result = ProcessingResult {
            accumulated_effect: JokerEffect::new(),
            jokers_processed: 1,
            retriggered_count: 0,
            errors: vec![],
            processing_time_micros: 100,
        };

        // Store in cache
        processor.store_in_cache(cache_key.clone(), test_result);
        assert_eq!(processor.cache_size(), 1);

        // Sleep a tiny bit to ensure expiration
        std::thread::sleep(std::time::Duration::from_millis(1));

        // Should be expired now
        let cached = processor.check_cache(&cache_key);
        assert!(cached.is_none());
        assert_eq!(processor.cache_metrics().expiries, 1);
        assert_eq!(processor.cache_size(), 0);
    }

    #[test]
    fn test_cache_size_limits() {
        let mut processor = JokerEffectProcessor::new();

        // Set very small cache size for testing
        let mut config = CacheConfig::default();
        config.max_entries = 2;
        processor.set_cache_config(config);

        let test_result = ProcessingResult {
            accumulated_effect: JokerEffect::new(),
            jokers_processed: 1,
            retriggered_count: 0,
            errors: vec![],
            processing_time_micros: 100,
        };

        // Fill cache to limit
        processor.store_in_cache("key1".to_string(), test_result.clone());
        processor.store_in_cache("key2".to_string(), test_result.clone());
        assert_eq!(processor.cache_size(), 2);

        // Adding one more should trigger eviction
        processor.store_in_cache("key3".to_string(), test_result);
        assert_eq!(processor.cache_size(), 2);
        assert_eq!(processor.cache_metrics().evictions, 1);
    }

    #[test]
    fn test_cache_disabled() {
        let mut processor = JokerEffectProcessor::new();

        // Disable caching
        let mut config = CacheConfig::default();
        config.enabled = false;
        processor.set_cache_config(config);

        let cache_key = "test_key".to_string();
        let test_result = ProcessingResult {
            accumulated_effect: JokerEffect::new(),
            jokers_processed: 1,
            retriggered_count: 0,
            errors: vec![],
            processing_time_micros: 100,
        };

        // Attempt to store in cache - should be ignored
        processor.store_in_cache(cache_key.clone(), test_result);
        assert_eq!(processor.cache_size(), 0);

        // Cache lookup should return None
        let cached = processor.check_cache(&cache_key);
        assert!(cached.is_none());
        assert_eq!(processor.cache_metrics().total_lookups, 0);
    }

    #[test]
    fn test_cache_metrics_calculation() {
        let mut processor = JokerEffectProcessor::new();

        let test_result = ProcessingResult {
            accumulated_effect: JokerEffect::new(),
            jokers_processed: 1,
            retriggered_count: 0,
            errors: vec![],
            processing_time_micros: 150,
        };

        // Store and retrieve multiple times
        processor.store_in_cache("key1".to_string(), test_result.clone());
        processor.check_cache("key1"); // Hit
        processor.check_cache("key1"); // Hit
        processor.check_cache("key2"); // Miss

        let metrics = processor.cache_metrics();
        assert_eq!(metrics.hits, 2);
        assert_eq!(metrics.misses, 1);
        assert_eq!(metrics.total_lookups, 3);
        assert_eq!(metrics.time_saved_micros, 300); // 150 * 2 hits
        assert!((metrics.hit_ratio() - 2.0 / 3.0).abs() < 0.001);
        assert!((metrics.avg_time_saved_per_hit() - 150.0).abs() < 0.001);
    }

    #[test]
    fn test_cache_maintenance() {
        let mut processor = JokerEffectProcessor::new();

        // Set short TTL
        let mut config = CacheConfig::default();
        config.ttl_seconds = 0;
        processor.set_cache_config(config);

        let test_result = ProcessingResult {
            accumulated_effect: JokerEffect::new(),
            jokers_processed: 1,
            retriggered_count: 0,
            errors: vec![],
            processing_time_micros: 100,
        };

        // Store some entries
        processor.store_in_cache("key1".to_string(), test_result.clone());
        processor.store_in_cache("key2".to_string(), test_result);
        assert_eq!(processor.cache_size(), 2);

        // Wait for expiration
        std::thread::sleep(std::time::Duration::from_millis(1));

        // Maintenance should clean up expired entries
        processor.maintain_cache();
        assert_eq!(processor.cache_size(), 0);
        assert_eq!(processor.cache_metrics().expiries, 2);
    }

    #[test]
    fn test_cache_performance_improvement() {
        use crate::card::{Value, Suit};
        use crate::joker::{GameContext, JokerId};
        use crate::hand::SelectHand;
        use crate::joker::{GameContext, JokerId};
        use std::collections::HashMap;
        use std::time::Instant;

        // This test demonstrates cache performance benefits
        // Note: In a real benchmark, you'd use a proper benchmarking framework

        let mut processor_with_cache = JokerEffectProcessor::new();
        let mut processor_without_cache = JokerEffectProcessor::new();

        // Disable cache for one processor
        let mut config = CacheConfig::default();
        config.enabled = false;
        processor_without_cache.set_cache_config(config);
        
        // Helper function to create fresh GameContext instances
        let create_game_context = || {
            GameContext {
                chips: 100,
                mult: 4,
                money: 100,
                ante: 1,
                round: 1,
                stage: &crate::stage::Stage::PreBlind(),
                hands_played: 0,
                discards_used: 0,
                jokers: &[],
                hand: &crate::hand::Hand::new(vec![]),
                discarded: &[],
                joker_state_manager: &std::sync::Arc::new(crate::joker_state::JokerStateManager::new()),
                hand_type_counts: &HashMap::new(),
                cards_in_deck: 52,
                stone_cards_in_deck: 0,
                rng: &balatro_rs::rng::GameRng::secure(),
            }
        }
        };

        let hand = SelectHand {
            cards: vec![
                Card { rank: Value::Ace, suit: Suit::Hearts },
                Card { rank: Value::King, suit: Suit::Hearts },
                Card { rank: Value::Queen, suit: Suit::Hearts },
                Card { rank: Value::Jack, suit: Suit::Hearts },
                Card { rank: Value::Ten, suit: Suit::Hearts },
            ],
        };

        let jokers: Vec<Box<dyn crate::joker::Joker>> = vec![];

        // Simulate repeated effect processing (would be common in RL training)
        let iterations = 100;

        // Test with cache
        let start_cached = Instant::now();
        for _ in 0..iterations {
            let mut game_context = create_game_context();
            processor_with_cache.process_hand_effects(&jokers, &mut game_context, &hand);
        }
        let cached_duration = start_cached.elapsed();

        // Test without cache
        let start_uncached = Instant::now();
        for _ in 0..iterations {
            let mut game_context = create_game_context();
            processor_without_cache.process_hand_effects(&jokers, &mut game_context, &hand);
        }
        let uncached_duration = start_uncached.elapsed();

        // Verify cache was effective
        let metrics = processor_with_cache.cache_metrics();
        assert!(metrics.hits > 0, "Cache should have recorded hits");
        assert!(metrics.hit_ratio() > 0.5, "Hit ratio should be significant");

        // Performance improvement should be measurable
        // Note: This is a simple demonstration - in practice, you'd need
        // more complex joker processing to see significant differences
        println!("Cached processing: {:?}", cached_duration);
        println!("Uncached processing: {:?}", uncached_duration);
        println!("Cache hit ratio: {:.2}%", metrics.hit_ratio() * 100.0);
        println!("Total time saved: {}μs", metrics.time_saved_micros);

        // The test passes if caching infrastructure works correctly
        assert!(metrics.total_lookups > 0);
    }

    #[test]
    fn test_cache_integration_with_processing() {
        use crate::card::{Value, Suit};
        use crate::hand::SelectHand;
        use crate::joker::{GameContext, JokerId};
        use std::collections::HashMap;

        let mut processor = JokerEffectProcessor::new();

        let mut game_context = GameContext {
            chips: 100,
            mult: 4,
            money: 100,
            ante: 1,
            round: 1,
            stage: &crate::stage::Stage::PreBlind(),
            hands_played: 0,
            discards_used: 0,
            jokers: &[],
            hand: &crate::hand::Hand::new(vec![]),
            discarded: &[],
            joker_state_manager: &std::sync::Arc::new(crate::joker_state::JokerStateManager::new()),
            hand_type_counts: &HashMap::new(),
            cards_in_deck: 52,
            stone_cards_in_deck: 0,
            rng: &balatro_rs::rng::GameRng::secure(),
        };

        let hand = SelectHand {
            cards: vec![
                Card { rank: Value::Ace, suit: Suit::Hearts },
                Card { rank: Value::King, suit: Suit::Hearts },
            ],
        };
        
        let card = Card { rank: Value::Queen, suit: Suit::Spades };
        let jokers: Vec<Box<dyn crate::joker::Joker>> = vec![];

        // First call should miss cache and store result
        let result1 = processor.process_hand_effects(&jokers, &mut game_context, &hand);
        assert_eq!(processor.cache_metrics().misses, 1);
        assert_eq!(processor.cache_metrics().hits, 0);
        assert_eq!(processor.cache_size(), 1);

        // Second call with same input should hit cache
        let result2 = processor.process_hand_effects(&jokers, &mut game_context, &hand);
        assert_eq!(processor.cache_metrics().misses, 1);
        assert_eq!(processor.cache_metrics().hits, 1);

        // Results should be identical
        assert_eq!(result1.jokers_processed, result2.jokers_processed);
        assert_eq!(result1.retriggered_count, result2.retriggered_count);

        // Test card effects caching
        let card_result1 = processor.process_card_effects(&jokers, &mut game_context, &card);
        assert_eq!(processor.cache_metrics().misses, 2); // New cache miss for card effects
        assert_eq!(processor.cache_size(), 2); // Now have both hand and card cache entries

        let card_result2 = processor.process_card_effects(&jokers, &mut game_context, &card);
        assert_eq!(processor.cache_metrics().hits, 2); // Cache hit for card effects

        // Card results should be identical
        assert_eq!(card_result1.jokers_processed, card_result2.jokers_processed);
    }

    #[test]
    fn test_processing_context_builder_default() {
        let builder = ProcessingContextBuilder::new();
        let context = builder.build();
        let default_context = ProcessingContext::default();

        assert_eq!(context.processing_mode, default_context.processing_mode);
        assert_eq!(
            context.resolution_strategy,
            default_context.resolution_strategy
        );
        assert_eq!(context.validate_effects, default_context.validate_effects);
        assert_eq!(
            context.max_retriggered_effects,
            default_context.max_retriggered_effects
        );
    }

    #[test]
    fn test_processing_context_builder_fluent_api() {
        let context = ProcessingContext::builder()
            .processing_mode(ProcessingMode::Delayed)
            .resolution_strategy(ConflictResolutionStrategy::Maximum)
            .validate_effects(false)
            .max_retriggered_effects(50)
            .build();

        assert_eq!(context.processing_mode, ProcessingMode::Delayed);
        assert_eq!(
            context.resolution_strategy,
            ConflictResolutionStrategy::Maximum
        );
        assert!(!context.validate_effects);
        assert_eq!(context.max_retriggered_effects, 50);
    }

    #[test]
    fn test_processing_context_builder_partial_configuration() {
        let context = ProcessingContext::builder()
            .processing_mode(ProcessingMode::Delayed)
            .validate_effects(false)
            .build();

        // Should use default values for unset fields
        assert_eq!(context.processing_mode, ProcessingMode::Delayed);
        assert_eq!(context.resolution_strategy, ConflictResolutionStrategy::Sum);
        assert!(!context.validate_effects);
        assert_eq!(context.max_retriggered_effects, 100);
    }

    #[test]
    fn test_processing_context_builder_chaining() {
        let builder = ProcessingContextBuilder::new()
            .processing_mode(ProcessingMode::Delayed)
            .resolution_strategy(ConflictResolutionStrategy::Maximum);

        let context1 = builder.clone().validate_effects(true).build();
        let context2 = builder.validate_effects(false).build();

        // Both should have the same processing mode and resolution strategy
        assert_eq!(context1.processing_mode, ProcessingMode::Delayed);
        assert_eq!(context2.processing_mode, ProcessingMode::Delayed);
        assert_eq!(
            context1.resolution_strategy,
            ConflictResolutionStrategy::Maximum
        );
        assert_eq!(
            context2.resolution_strategy,
            ConflictResolutionStrategy::Maximum
        );

        // But different validation settings
        assert!(context1.validate_effects);
        assert!(!context2.validate_effects);
    }

    #[test]
    fn test_processing_context_builder_from_default() {
        let builder = ProcessingContextBuilder::default();
        let context = builder.build();
        let default_context = ProcessingContext::default();

        assert_eq!(context.processing_mode, default_context.processing_mode);
        assert_eq!(
            context.resolution_strategy,
            default_context.resolution_strategy
        );
        assert_eq!(context.validate_effects, default_context.validate_effects);
        assert_eq!(
            context.max_retriggered_effects,
            default_context.max_retriggered_effects
        );
    }

    #[test]
    fn test_processing_context_builder_all_processing_modes() {
        let immediate_context = ProcessingContext::builder()
            .processing_mode(ProcessingMode::Immediate)
            .build();
        assert_eq!(immediate_context.processing_mode, ProcessingMode::Immediate);

        let delayed_context = ProcessingContext::builder()
            .processing_mode(ProcessingMode::Delayed)
            .build();
        assert_eq!(delayed_context.processing_mode, ProcessingMode::Delayed);
    }

    #[test]
    fn test_processing_context_builder_all_resolution_strategies() {
        let sum_context = ProcessingContext::builder()
            .resolution_strategy(ConflictResolutionStrategy::Sum)
            .build();
        assert_eq!(
            sum_context.resolution_strategy,
            ConflictResolutionStrategy::Sum
        );

        let max_context = ProcessingContext::builder()
            .resolution_strategy(ConflictResolutionStrategy::Maximum)
            .build();
        assert_eq!(
            max_context.resolution_strategy,
            ConflictResolutionStrategy::Maximum
        );

        let min_context = ProcessingContext::builder()
            .resolution_strategy(ConflictResolutionStrategy::Minimum)
            .build();
        assert_eq!(
            min_context.resolution_strategy,
            ConflictResolutionStrategy::Minimum
        );
    }

    #[test]
    fn test_processing_context_builder_validation_flags() {
        let validate_true = ProcessingContext::builder().validate_effects(true).build();
        assert!(validate_true.validate_effects);

        let validate_false = ProcessingContext::builder().validate_effects(false).build();
        assert!(!validate_false.validate_effects);
    }

    #[test]
    fn test_processing_context_builder_max_retriggered_effects() {
        let context = ProcessingContext::builder()
            .max_retriggered_effects(25)
            .build();
        assert_eq!(context.max_retriggered_effects, 25);

        let high_limit_context = ProcessingContext::builder()
            .max_retriggered_effects(1000)
            .build();
        assert_eq!(high_limit_context.max_retriggered_effects, 1000);

        let zero_limit_context = ProcessingContext::builder()
            .max_retriggered_effects(0)
            .build();
        assert_eq!(zero_limit_context.max_retriggered_effects, 0);
    }

    #[test]
    fn test_processing_context_builder_backward_compatibility() {
        // Test that existing code still works
        let mut manual_context = ProcessingContext::default();
        manual_context.processing_mode = ProcessingMode::Delayed;
        manual_context.validate_effects = false;

        let builder_context = ProcessingContext::builder()
            .processing_mode(ProcessingMode::Delayed)
            .validate_effects(false)
            .build();

        assert_eq!(
            manual_context.processing_mode,
            builder_context.processing_mode
        );
        assert_eq!(
            manual_context.validate_effects,
            builder_context.validate_effects
        );
        assert_eq!(
            manual_context.resolution_strategy,
            builder_context.resolution_strategy
        );
        assert_eq!(
            manual_context.max_retriggered_effects,
            builder_context.max_retriggered_effects
        );
    }

    #[test]
    fn test_processing_context_builder_debug_trait() {
        let builder = ProcessingContext::builder()
            .processing_mode(ProcessingMode::Delayed)
            .validate_effects(false);

        // Should be able to debug print the builder
        let debug_string = format!("{:?}", builder);
        assert!(debug_string.contains("ProcessingContextBuilder"));
    }

    #[test]
    fn test_processing_context_builder_clone_trait() {
        let builder = ProcessingContext::builder()
            .processing_mode(ProcessingMode::Delayed)
            .validate_effects(false);

        let cloned_builder = builder.clone();
        let original_context = builder.build();
        let cloned_context = cloned_builder.build();

        assert_eq!(
            original_context.processing_mode,
            cloned_context.processing_mode
        );
        assert_eq!(
            original_context.validate_effects,
            cloned_context.validate_effects
        );
        assert_eq!(
            original_context.resolution_strategy,
            cloned_context.resolution_strategy
        );
        assert_eq!(
            original_context.max_retriggered_effects,
            cloned_context.max_retriggered_effects
        );
    }

    #[test]
    fn test_configurable_priority_strategies() {
        // Test default behavior (should use MetadataPriorityStrategy)
        let default_processor = JokerEffectProcessor::new();
        let default_priority = default_processor.get_joker_priority(JokerId::Joker);
        assert_eq!(default_priority, EffectPriority::Normal);

        // Test with DefaultPriorityStrategy
        let default_context = ProcessingContext::builder()
            .priority_strategy(Arc::new(DefaultPriorityStrategy))
            .build();
        let default_strategy_processor = JokerEffectProcessor::with_context(default_context);
        
        // Should always return Normal for any joker
        assert_eq!(default_strategy_processor.get_joker_priority(JokerId::Joker), EffectPriority::Normal);
        assert_eq!(default_strategy_processor.get_joker_priority(JokerId::GreedyJoker), EffectPriority::Normal);
        assert_eq!(default_strategy_processor.get_joker_priority(JokerId::LustyJoker), EffectPriority::Normal);

        // Test with CustomPriorityStrategy
        let mut custom_mappings = std::collections::HashMap::new();
        custom_mappings.insert(JokerId::Joker, EffectPriority::High);
        custom_mappings.insert(JokerId::GreedyJoker, EffectPriority::Critical);
        
        let custom_context = ProcessingContext::builder()
            .priority_strategy(Arc::new(CustomPriorityStrategy::new(custom_mappings)))
            .build();
        let custom_processor = JokerEffectProcessor::with_context(custom_context);
        
        // Should use custom mappings
        assert_eq!(custom_processor.get_joker_priority(JokerId::Joker), EffectPriority::High);
        assert_eq!(custom_processor.get_joker_priority(JokerId::GreedyJoker), EffectPriority::Critical);
        // Should fall back to metadata strategy for unmapped jokers
        assert_eq!(custom_processor.get_joker_priority(JokerId::LustyJoker), EffectPriority::Normal);

        // Test with ContextAwarePriorityStrategy
        let context_aware_context = ProcessingContext::builder()
            .priority_strategy(Arc::new(ContextAwarePriorityStrategy::new()))
            .build();
        let context_aware_processor = JokerEffectProcessor::with_context(context_aware_context);
        
        // Should provide context-aware priorities
        let joker_priority = context_aware_processor.get_joker_priority(JokerId::Joker);
        let lusty_priority = context_aware_processor.get_joker_priority(JokerId::LustyJoker);
        
        // LustyJoker should get boosted priority (Normal -> High in this implementation)
        assert_eq!(joker_priority, EffectPriority::Normal);
        assert_eq!(lusty_priority, EffectPriority::High);
    }

    #[test]
    fn test_priority_strategy_api_from_issue() {
        // Test the exact API proposed in the issue
        let context = ProcessingContext::builder()
            .priority_strategy(Arc::new(MetadataPriorityStrategy::new()))
            .build();
            
        let processor = JokerEffectProcessor::with_context(context);
        
        // Verify it works
        let priority = processor.get_joker_priority(JokerId::Joker);
        assert_eq!(priority, EffectPriority::Normal);
    }

    #[test]
    fn test_processing_context_builder_api() {
        // Test the builder pattern works correctly
        let context = ProcessingContext::builder()
            .processing_mode(ProcessingMode::Delayed)
            .validate_effects(false)
            .max_retriggered_effects(50)
            .priority_strategy(Arc::new(DefaultPriorityStrategy))
            .build();
            
        assert_eq!(context.processing_mode, ProcessingMode::Delayed);
        assert_eq!(context.validate_effects, false);
        assert_eq!(context.max_retriggered_effects, 50);
        
        // Test the processor can be created with this context
        let processor = JokerEffectProcessor::with_context(context);
        assert_eq!(processor.get_joker_priority(JokerId::Joker), EffectPriority::Normal);
    }

    #[test]
    fn test_priority_strategy_runtime_changes() {
        // Test that priority strategy can be changed at runtime
        let mut processor = JokerEffectProcessor::new();
        
        // Initial priority (using default MetadataPriorityStrategy)
        let initial_priority = processor.get_joker_priority(JokerId::Joker);
        assert_eq!(initial_priority, EffectPriority::Normal);
        
        // Change to DefaultPriorityStrategy
        let new_context = ProcessingContext::builder()
            .priority_strategy(Arc::new(DefaultPriorityStrategy))
            .build();
        processor.set_context(new_context);
        
        // Should still return Normal (DefaultPriorityStrategy always returns Normal)
        let new_priority = processor.get_joker_priority(JokerId::Joker);
        assert_eq!(new_priority, EffectPriority::Normal);
        
        // Change to custom strategy with different priorities
        let mut custom_mappings = std::collections::HashMap::new();
        custom_mappings.insert(JokerId::Joker, EffectPriority::Critical);
        
        let custom_context = ProcessingContext::builder()
            .priority_strategy(Arc::new(CustomPriorityStrategy::new(custom_mappings)))
            .build();
        processor.set_context(custom_context);
        
        // Should now return Critical for Joker
        let custom_priority = processor.get_joker_priority(JokerId::Joker);
        assert_eq!(custom_priority, EffectPriority::Critical);
    }
}
