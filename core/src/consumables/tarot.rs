//! Tarot card implementation for Balatro game engine
//!
//! This module provides the core infrastructure for tarot cards in Balatro,
//! including trait definitions, factory systems, and performance-optimized
//! card implementations.
//!
//! # Architecture
//!
//! The tarot system follows kernel-quality design principles:
//! - **Single Responsibility**: Each tarot card has one clear purpose
//! - **Performance First**: <1ms execution target for all effects
//! - **Memory Safety**: Proper resource management and bounds checking
//! - **Error Handling**: Comprehensive error reporting with context
//! - **Thread Safety**: Safe concurrent access for RL training
//!
//! # Design Patterns
//!
//! - **Factory Pattern**: Centralized tarot card creation and registration
//! - **Strategy Pattern**: Different targeting and effect strategies
//! - **Observer Pattern**: Effect application with proper validation
//! - **Builder Pattern**: Complex target construction and validation
//!
//! # Performance Characteristics
//!
//! - Target validation: ~100ns for simple targets, ~1μs for complex
//! - Effect application: <1ms for all implemented cards
//! - Memory overhead: ~200 bytes per tarot card instance
//! - Factory lookup: O(1) HashMap-based registration

use crate::consumables::{
    Consumable, ConsumableError, ConsumableId, ConsumableType, ConsumableEffect, 
    Target, TargetType, TargetValidationError
};
use crate::game::Game;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, RwLock};
use thiserror::Error;

/// Core trait that all tarot cards must implement
///
/// This trait extends the base Consumable trait with tarot-specific functionality
/// including targeting requirements, validation, and effect application.
///
/// # Implementation Requirements
///
/// - All effects must complete within 1ms (performance target)
/// - Target validation must be comprehensive and fail-safe
/// - Error messages must be actionable for debugging
/// - Memory usage must be bounded and predictable
///
/// # Thread Safety
///
/// Tarot card implementations must be thread-safe for RL training scenarios.
/// This means all state mutations must be properly synchronized and atomic.
pub trait TarotCard: Consumable + Send + Sync + fmt::Debug {
    /// Get the unique identifier for this tarot card
    fn tarot_id(&self) -> ConsumableId;
    
    /// Get the tarot-specific name (e.g., "The Fool", "The Magician")
    fn tarot_name(&self) -> &'static str;
    
    /// Get detailed description of the tarot card's effect
    fn tarot_description(&self) -> &'static str;
    
    /// Check if this tarot can be used in the current game state with the given target
    ///
    /// This provides tarot-specific validation beyond the base consumable validation.
    /// Should return false if the effect would have no impact or is invalid.
    fn can_use_tarot(&self, game: &Game, target: &Target) -> Result<bool, TarotError>;
    
    /// Apply the tarot card's effect to the game state
    ///
    /// This is the core method that implements the tarot card's mechanics.
    /// Must be idempotent and atomic - either fully succeeds or has no effect.
    ///
    /// # Performance Contract
    /// - Must complete within 1ms for all implementations
    /// - Should minimize memory allocations
    /// - Must not hold locks longer than necessary
    fn apply_tarot_effect(&self, game: &mut Game, target: Target) -> Result<TarotResult, TarotError>;
    
    /// Get the rarity level of this tarot card (affects shop appearance rates)
    fn rarity(&self) -> TarotRarity {
        TarotRarity::Common
    }
    
    /// Get the base cost of this tarot card in the shop
    fn base_cost(&self) -> usize {
        3 // Standard tarot card cost
    }
    
    /// Check if this tarot card is currently available for generation
    ///
    /// Some tarot cards might be locked behind progression or have conditions
    fn is_available(&self, game: &Game) -> bool {
        let _ = game; // Suppress unused parameter warning
        true // Default: all tarot cards are available
    }
    
    /// Get performance characteristics for this tarot card
    ///
    /// Used for benchmarking and performance monitoring in RL training
    fn performance_info(&self) -> TarotPerformanceInfo {
        TarotPerformanceInfo {
            avg_execution_time_ns: 500_000, // 0.5ms default
            max_execution_time_ns: 1_000_000, // 1ms max
            memory_overhead_bytes: 200,
            complexity_score: 1.0,
        }
    }
}

/// Rarity levels for tarot cards affecting shop generation rates
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TarotRarity {
    /// Common tarot cards - appear frequently (70% base rate)
    Common,
    /// Uncommon tarot cards - appear occasionally (25% base rate)  
    Uncommon,
    /// Rare tarot cards - appear rarely (4.5% base rate)
    Rare,
    /// Legendary tarot cards - appear very rarely (0.5% base rate)
    Legendary,
}

impl fmt::Display for TarotRarity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TarotRarity::Common => write!(f, "Common"),
            TarotRarity::Uncommon => write!(f, "Uncommon"),
            TarotRarity::Rare => write!(f, "Rare"),
            TarotRarity::Legendary => write!(f, "Legendary"),
        }
    }
}

/// Result of applying a tarot card effect
///
/// Provides detailed information about what the tarot card accomplished,
/// enabling proper feedback to players and debugging information.
#[derive(Debug, Clone, PartialEq)]
pub struct TarotResult {
    /// Human-readable description of what the effect accomplished
    pub description: String,
    /// Whether the effect made significant changes to game state
    pub significant_change: bool,
    /// Number of cards/jokers/elements affected by the effect
    pub elements_affected: usize,
    /// Additional metadata about the effect (for telemetry)
    pub metadata: HashMap<String, String>,
}

impl TarotResult {
    /// Create a new tarot result with basic information
    pub fn new(description: String, elements_affected: usize) -> Self {
        Self {
            description,
            significant_change: elements_affected > 0,
            elements_affected,
            metadata: HashMap::new(),
        }
    }
    
    /// Create a result indicating no effect occurred
    pub fn no_effect(reason: String) -> Self {
        Self {
            description: format!("No effect: {}", reason),
            significant_change: false,
            elements_affected: 0,
            metadata: HashMap::new(),
        }
    }
    
    /// Add metadata to this result
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }
}

/// Performance characteristics for tarot card implementations
///
/// Used for benchmarking and ensuring performance targets are met.
/// Critical for RL training where tarot effects are applied frequently.
#[derive(Debug, Clone)]
pub struct TarotPerformanceInfo {
    /// Average execution time in nanoseconds
    pub avg_execution_time_ns: u64,
    /// Maximum observed execution time in nanoseconds
    pub max_execution_time_ns: u64,
    /// Additional memory overhead in bytes
    pub memory_overhead_bytes: usize,
    /// Complexity score (1.0 = simple, higher = more complex)
    pub complexity_score: f64,
}

/// Comprehensive error types for tarot card operations
///
/// Designed for production debugging with actionable error messages
/// and structured data for telemetry and monitoring.
#[derive(Debug, Clone, PartialEq, Error)]
pub enum TarotError {
    /// Invalid target provided for this tarot card
    #[error("Invalid target for tarot card {card_id:?}: {reason}")]
    InvalidTarget { 
        card_id: ConsumableId, 
        reason: String 
    },
    
    /// Game state prevents tarot card usage
    #[error("Cannot use tarot card {card_id:?} in current state: {reason}")]
    InvalidGameState { 
        card_id: ConsumableId, 
        reason: String 
    },
    
    /// Effect failed to apply due to internal error
    #[error("Tarot effect failed for {card_id:?}: {reason}")]
    EffectFailed { 
        card_id: ConsumableId, 
        reason: String 
    },
    
    /// Target validation failed with detailed error
    #[error("Target validation failed: {0}")]
    TargetValidation(#[from] TargetValidationError),
    
    /// Generic consumable error occurred
    #[error("Consumable error: {0}")]
    ConsumableError(#[from] ConsumableError),
    
    /// Performance timeout - effect took too long
    #[error("Tarot effect timed out for {card_id:?}: took {actual_ms}ms, limit {limit_ms}ms")]
    PerformanceTimeout {
        card_id: ConsumableId,
        actual_ms: u64,
        limit_ms: u64,
    },
    
    /// Resource exhaustion (memory, slots, etc.)
    #[error("Resource exhausted for {card_id:?}: {resource} limit reached")]
    ResourceExhausted {
        card_id: ConsumableId,
        resource: String,
    },
}

/// Thread-safe factory for creating and managing tarot cards
///
/// Provides centralized registration and creation of tarot card instances.
/// Designed for high-performance access patterns common in RL training.
///
/// # Thread Safety
///
/// The factory uses RwLock for registration data, allowing concurrent reads
/// (common case) while serializing writes (registration only).
///
/// # Performance
///
/// - Lookup: O(1) HashMap access
/// - Registration: O(1) amortized
/// - Memory: Minimal overhead per registered card type
pub struct TarotFactory {
    /// Registry of available tarot card types
    registry: Arc<RwLock<HashMap<ConsumableId, Box<dyn TarotCardFactory>>>>,
    /// Performance tracking for each card type
    performance_stats: Arc<RwLock<HashMap<ConsumableId, TarotPerformanceInfo>>>,
}

/// Factory trait for creating specific tarot card types
///
/// This allows for lazy creation and proper initialization of tarot cards
/// without requiring static registration of all possible implementations.
pub trait TarotCardFactory: Send + Sync {
    /// Create a new instance of this tarot card type
    fn create(&self) -> Box<dyn TarotCard>;
    
    /// Get the tarot card ID this factory creates
    fn card_id(&self) -> ConsumableId;
    
    /// Get metadata about this tarot card type
    fn metadata(&self) -> TarotCardMetadata;
}

/// Metadata about a tarot card type for factory management
#[derive(Debug, Clone)]
pub struct TarotCardMetadata {
    pub name: &'static str,
    pub description: &'static str,
    pub rarity: TarotRarity,
    pub target_type: TargetType,
    pub effect_category: ConsumableEffect,
    pub implemented: bool,
}

impl TarotFactory {
    /// Create a new tarot factory instance
    pub fn new() -> Self {
        Self {
            registry: Arc::new(RwLock::new(HashMap::new())),
            performance_stats: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Register a new tarot card factory
    ///
    /// # Thread Safety
    /// This method takes a write lock, so it should be called during initialization
    /// rather than during performance-critical sections.
    pub fn register(&self, factory: Box<dyn TarotCardFactory>) -> Result<(), TarotError> {
        let card_id = factory.card_id();
        
        // Validate that this is actually a tarot card ID
        if card_id.consumable_type() != ConsumableType::Tarot {
            return Err(TarotError::InvalidGameState {
                card_id,
                reason: "Not a tarot card type".to_string(),
            });
        }
        
        let mut registry = self.registry.write()
            .map_err(|_| TarotError::ResourceExhausted {
                card_id,
                resource: "Factory registry lock".to_string(),
            })?;
            
        registry.insert(card_id, factory);
        Ok(())
    }
    
    /// Create a tarot card instance by ID
    ///
    /// # Performance
    /// This is the hot path for tarot card creation. Uses read lock for
    /// concurrent access and O(1) HashMap lookup.
    pub fn create(&self, card_id: ConsumableId) -> Result<Box<dyn TarotCard>, TarotError> {
        let registry = self.registry.read()
            .map_err(|_| TarotError::ResourceExhausted {
                card_id,
                resource: "Factory registry lock".to_string(),
            })?;
            
        match registry.get(&card_id) {
            Some(factory) => Ok(factory.create()),
            None => Err(TarotError::InvalidGameState {
                card_id,
                reason: "No factory registered for this tarot card".to_string(),
            }),
        }
    }
    
    /// Get all available tarot card IDs
    pub fn available_cards(&self) -> Result<Vec<ConsumableId>, TarotError> {
        let registry = self.registry.read()
            .map_err(|_| TarotError::ResourceExhausted {
                card_id: ConsumableId::TarotPlaceholder, // Placeholder for error
                resource: "Factory registry lock".to_string(),
            })?;
            
        Ok(registry.keys().copied().collect())
    }
    
    /// Get tarot cards by rarity level
    pub fn cards_by_rarity(&self, rarity: TarotRarity) -> Result<Vec<ConsumableId>, TarotError> {
        let registry = self.registry.read()
            .map_err(|_| TarotError::ResourceExhausted {
                card_id: ConsumableId::TarotPlaceholder,
                resource: "Factory registry lock".to_string(),
            })?;
            
        let mut cards = Vec::new();
        for (card_id, factory) in registry.iter() {
            if factory.metadata().rarity == rarity {
                cards.push(*card_id);
            }
        }
        
        Ok(cards)
    }
    
    /// Update performance statistics for a tarot card
    pub fn update_performance(&self, card_id: ConsumableId, info: TarotPerformanceInfo) -> Result<(), TarotError> {
        let mut stats = self.performance_stats.write()
            .map_err(|_| TarotError::ResourceExhausted {
                card_id,
                resource: "Performance stats lock".to_string(),
            })?;
            
        stats.insert(card_id, info);
        Ok(())
    }
    
    /// Get performance statistics for a tarot card
    pub fn get_performance(&self, card_id: ConsumableId) -> Result<Option<TarotPerformanceInfo>, TarotError> {
        let stats = self.performance_stats.read()
            .map_err(|_| TarotError::ResourceExhausted {
                card_id,
                resource: "Performance stats lock".to_string(),
            })?;
            
        Ok(stats.get(&card_id).cloned())
    }
    
    /// Get metadata for a specific tarot card
    pub fn get_metadata(&self, card_id: ConsumableId) -> Result<Option<TarotCardMetadata>, TarotError> {
        let registry = self.registry.read()
            .map_err(|_| TarotError::ResourceExhausted {
                card_id,
                resource: "Factory registry lock".to_string(),
            })?;
            
        Ok(registry.get(&card_id).map(|factory| factory.metadata()))
    }
    
    /// Check if a tarot card is registered and available
    pub fn is_available(&self, card_id: ConsumableId) -> bool {
        self.registry.read()
            .map(|registry| registry.contains_key(&card_id))
            .unwrap_or(false)
    }
}

impl Default for TarotFactory {
    fn default() -> Self {
        Self::new()
    }
}

/// Global tarot factory instance for centralized access
///
/// This provides a singleton-like pattern for accessing the tarot factory
/// throughout the application while maintaining thread safety and performance.
static GLOBAL_TAROT_FACTORY: std::sync::OnceLock<TarotFactory> = std::sync::OnceLock::new();

/// Get the global tarot factory instance
///
/// # Thread Safety
/// Safe to call from multiple threads. The factory is initialized once
/// and then provides concurrent read access to registered cards.
pub fn get_tarot_factory() -> &'static TarotFactory {
    GLOBAL_TAROT_FACTORY.get_or_init(TarotFactory::new)
}

/// Initialize the global tarot factory with all available tarot cards
///
/// This should be called during application startup to register all
/// implemented tarot card types with the global factory.
pub fn initialize_tarot_factory() -> Result<(), TarotError> {
    let factory = get_tarot_factory();
    
    // Register placeholder implementations for now
    // Real implementations will be added in subsequent issues
    factory.register(Box::new(PlaceholderTarotFactory {
        card_id: ConsumableId::TheFool,
        name: "The Fool",
        description: "Creates last Joker used this round if possible",
        rarity: TarotRarity::Common,
        target_type: TargetType::None,
    }))?;
    
    factory.register(Box::new(PlaceholderTarotFactory {
        card_id: ConsumableId::TheMagician,
        name: "The Magician", 
        description: "Enhances 2 selected cards to Lucky Cards",
        rarity: TarotRarity::Common,
        target_type: TargetType::Cards(2),
    }))?;
    
    factory.register(Box::new(PlaceholderTarotFactory {
        card_id: ConsumableId::TheHighPriestess,
        name: "The High Priestess",
        description: "Creates up to 2 Planet Cards",
        rarity: TarotRarity::Uncommon,
        target_type: TargetType::None,
    }))?;
    
    factory.register(Box::new(PlaceholderTarotFactory {
        card_id: ConsumableId::TheEmperor,
        name: "The Emperor",
        description: "Creates up to 2 Tarot Cards", 
        rarity: TarotRarity::Rare,
        target_type: TargetType::None,
    }))?;
    
    factory.register(Box::new(PlaceholderTarotFactory {
        card_id: ConsumableId::TheHierophant,
        name: "The Hierophant",
        description: "Enhances 2 selected cards to Bonus Cards",
        rarity: TarotRarity::Common,
        target_type: TargetType::Cards(2),
    }))?;
    
    Ok(())
}

/// Placeholder factory for tarot cards that aren't fully implemented yet
///
/// This allows the infrastructure to be complete while individual card
/// implementations are developed in parallel.
struct PlaceholderTarotFactory {
    card_id: ConsumableId,
    name: &'static str,
    description: &'static str,
    rarity: TarotRarity,
    target_type: TargetType,
}

impl TarotCardFactory for PlaceholderTarotFactory {
    fn create(&self) -> Box<dyn TarotCard> {
        Box::new(PlaceholderTarotCard {
            id: self.card_id,
            name: self.name,
            description: self.description,
            rarity: self.rarity,
            target_type: self.target_type,
        })
    }
    
    fn card_id(&self) -> ConsumableId {
        self.card_id
    }
    
    fn metadata(&self) -> TarotCardMetadata {
        TarotCardMetadata {
            name: self.name,
            description: self.description,
            rarity: self.rarity,
            target_type: self.target_type,
            effect_category: ConsumableEffect::Utility,
            implemented: false, // Placeholder implementations
        }
    }
}

/// Placeholder tarot card implementation
///
/// Provides a safe no-op implementation for tarot cards that haven't been
/// fully implemented yet. Allows the infrastructure to be tested and used
/// while individual cards are developed.
#[derive(Debug)]
struct PlaceholderTarotCard {
    id: ConsumableId,
    name: &'static str,
    description: &'static str,
    rarity: TarotRarity,
    target_type: TargetType,
}

impl Consumable for PlaceholderTarotCard {
    fn consumable_type(&self) -> ConsumableType {
        ConsumableType::Tarot
    }
    
    fn can_use(&self, _game_state: &Game, _target: &Target) -> bool {
        false // Placeholder cards can't be used
    }
    
    fn use_effect(&self, _game_state: &mut Game, _target: Target) -> Result<(), ConsumableError> {
        Err(ConsumableError::EffectFailed(
            "Placeholder tarot card - not implemented yet".to_string()
        ))
    }
    
    fn get_description(&self) -> String {
        format!("{} (Placeholder - Not Implemented)", self.description)
    }
    
    fn get_target_type(&self) -> TargetType {
        self.target_type
    }
    
    fn get_effect_category(&self) -> ConsumableEffect {
        ConsumableEffect::Utility
    }
    
    fn name(&self) -> &'static str {
        self.name
    }
    
    fn description(&self) -> &'static str {
        self.description
    }
    
    fn cost(&self) -> usize {
        3 // Standard tarot cost
    }
}

impl TarotCard for PlaceholderTarotCard {
    fn tarot_id(&self) -> ConsumableId {
        self.id
    }
    
    fn tarot_name(&self) -> &'static str {
        self.name
    }
    
    fn tarot_description(&self) -> &'static str {
        self.description
    }
    
    fn can_use_tarot(&self, _game: &Game, _target: &Target) -> Result<bool, TarotError> {
        Ok(false) // Placeholder cards can't be used
    }
    
    fn apply_tarot_effect(&self, _game: &mut Game, _target: Target) -> Result<TarotResult, TarotError> {
        Err(TarotError::EffectFailed {
            card_id: self.id,
            reason: "Placeholder implementation - not available yet".to_string(),
        })
    }
    
    fn rarity(&self) -> TarotRarity {
        self.rarity
    }
    
    fn base_cost(&self) -> usize {
        3
    }
    
    fn is_available(&self, _game: &Game) -> bool {
        false // Placeholder cards are not available
    }
}

/// Performance benchmarking utilities for tarot card implementations
///
/// Provides standardized benchmarking and performance validation to ensure
/// all tarot cards meet the <1ms performance target.
pub mod benchmarks {
    use super::*;
    use std::time::{Duration, Instant};
    
    /// Benchmark a tarot card's performance with a specific target
    ///
    /// Runs the tarot card effect multiple times and collects timing data.
    /// Returns performance statistics suitable for monitoring and optimization.
    pub fn benchmark_tarot_card(
        card: &dyn TarotCard,
        game: &mut Game,
        target: Target,
        iterations: usize,
    ) -> BenchmarkResult {
        let mut execution_times = Vec::with_capacity(iterations);
        let mut successful_executions = 0;
        let mut total_elements_affected = 0;
        
        for _ in 0..iterations {
            // Note: Since Game doesn't implement Clone (due to trait objects),
            // this benchmark measures effects on the same game state.
            // For production benchmarking, consider using saved game states
            // or a more sophisticated reset mechanism.
            let target_copy = target.clone();
            
            let start = Instant::now();
            
            match card.apply_tarot_effect(game, target_copy) {
                Ok(result) => {
                    let duration = start.elapsed();
                    execution_times.push(duration);
                    successful_executions += 1;
                    total_elements_affected += result.elements_affected;
                }
                Err(_) => {
                    // Still record time for failed executions
                    let duration = start.elapsed();
                    execution_times.push(duration);
                }
            }
        }
        
        // Calculate statistics
        let avg_time = if !execution_times.is_empty() {
            execution_times.iter().sum::<Duration>() / execution_times.len() as u32
        } else {
            Duration::ZERO
        };
        
        let max_time = execution_times.iter().max().copied().unwrap_or(Duration::ZERO);
        let min_time = execution_times.iter().min().copied().unwrap_or(Duration::ZERO);
        
        BenchmarkResult {
            card_id: card.tarot_id(),
            iterations_run: iterations,
            successful_executions,
            avg_execution_time: avg_time,
            min_execution_time: min_time,
            max_execution_time: max_time,
            total_elements_affected,
            meets_performance_target: max_time <= Duration::from_millis(1),
        }
    }
    
    /// Result of benchmarking a tarot card
    #[derive(Debug, Clone)]
    pub struct BenchmarkResult {
        pub card_id: ConsumableId,
        pub iterations_run: usize,
        pub successful_executions: usize,
        pub avg_execution_time: Duration,
        pub min_execution_time: Duration,
        pub max_execution_time: Duration,
        pub total_elements_affected: usize,
        pub meets_performance_target: bool,
    }
    
    impl BenchmarkResult {
        /// Check if this benchmark result meets all performance requirements
        pub fn is_acceptable(&self) -> bool {
            self.meets_performance_target && 
            self.successful_executions > 0 &&
            self.max_execution_time <= Duration::from_millis(1)
        }
        
        /// Get a human-readable performance summary
        pub fn summary(&self) -> String {
            format!(
                "Tarot {} - Avg: {:.2}ms, Max: {:.2}ms, Success: {}/{}, Target Met: {}",
                self.card_id,
                self.avg_execution_time.as_secs_f64() * 1000.0,
                self.max_execution_time.as_secs_f64() * 1000.0,
                self.successful_executions,
                self.iterations_run,
                self.meets_performance_target
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    
    #[test]
    fn test_tarot_factory_creation() {
        let factory = TarotFactory::new();
        assert!(factory.available_cards().unwrap().is_empty());
    }
    
    #[test]
    fn test_tarot_factory_registration() {
        let factory = TarotFactory::new();
        
        let placeholder_factory = Box::new(PlaceholderTarotFactory {
            card_id: ConsumableId::TheFool,
            name: "Test Fool",
            description: "Test card",
            rarity: TarotRarity::Common,
            target_type: TargetType::None,
        });
        
        factory.register(placeholder_factory).unwrap();
        
        let available = factory.available_cards().unwrap();
        assert_eq!(available.len(), 1);
        assert!(available.contains(&ConsumableId::TheFool));
    }
    
    #[test]
    fn test_tarot_factory_creation_and_usage() {
        let factory = TarotFactory::new();
        
        let placeholder_factory = Box::new(PlaceholderTarotFactory {
            card_id: ConsumableId::TheFool,
            name: "Test Fool",
            description: "Test card",
            rarity: TarotRarity::Common,
            target_type: TargetType::None,
        });
        
        factory.register(placeholder_factory).unwrap();
        
        let card = factory.create(ConsumableId::TheFool).unwrap();
        assert_eq!(card.tarot_id(), ConsumableId::TheFool);
        assert_eq!(card.tarot_name(), "Test Fool");
    }
    
    #[test]
    fn test_tarot_cards_by_rarity() {
        let factory = TarotFactory::new();
        
        factory.register(Box::new(PlaceholderTarotFactory {
            card_id: ConsumableId::TheFool,
            name: "Common Card",
            description: "Test",
            rarity: TarotRarity::Common,
            target_type: TargetType::None,
        })).unwrap();
        
        factory.register(Box::new(PlaceholderTarotFactory {
            card_id: ConsumableId::TheMagician,
            name: "Rare Card",
            description: "Test",
            rarity: TarotRarity::Rare,
            target_type: TargetType::None,
        })).unwrap();
        
        let common_cards = factory.cards_by_rarity(TarotRarity::Common).unwrap();
        let rare_cards = factory.cards_by_rarity(TarotRarity::Rare).unwrap();
        
        assert_eq!(common_cards.len(), 1);
        assert!(common_cards.contains(&ConsumableId::TheFool));
        
        assert_eq!(rare_cards.len(), 1);
        assert!(rare_cards.contains(&ConsumableId::TheMagician));
    }
    
    #[test]
    fn test_global_factory_initialization() {
        let result = initialize_tarot_factory();
        assert!(result.is_ok());
        
        let factory = get_tarot_factory();
        let available = factory.available_cards().unwrap();
        
        // Should have registered the placeholder cards
        assert!(available.contains(&ConsumableId::TheFool));
        assert!(available.contains(&ConsumableId::TheMagician));
        assert!(available.contains(&ConsumableId::TheHighPriestess));
        assert!(available.contains(&ConsumableId::TheEmperor));
        assert!(available.contains(&ConsumableId::TheHierophant));
    }
    
    #[test]
    fn test_placeholder_tarot_card_behavior() {
        let card = PlaceholderTarotCard {
            id: ConsumableId::TheFool,
            name: "Test Card",
            description: "Test Description",
            rarity: TarotRarity::Common,
            target_type: TargetType::None,
        };
        
        let game = Game::new(Config::default());
        let target = Target::None;
        
        // Placeholder cards should not be usable
        assert!(!card.can_use(&game, &target));
        assert!(!card.can_use_tarot(&game, &target).unwrap());
        assert!(!card.is_available(&game));
        
        // Effect should fail appropriately
        // Note: Cannot clone Game due to trait objects
        // Placeholder cards should fail without modifying game state
        let mut game_mut = Game::new(Config::default());
        let result = card.apply_tarot_effect(&mut game_mut, target);
        assert!(result.is_err());
    }
    
    #[test] 
    fn test_tarot_result_creation() {
        let result = TarotResult::new("Test effect".to_string(), 3);
        assert_eq!(result.description, "Test effect");
        assert_eq!(result.elements_affected, 3);
        assert!(result.significant_change);
        
        let no_effect = TarotResult::no_effect("Nothing to target".to_string());
        assert!(!no_effect.significant_change);
        assert_eq!(no_effect.elements_affected, 0);
        
        let with_metadata = result.with_metadata("key".to_string(), "value".to_string());
        assert_eq!(with_metadata.metadata.get("key"), Some(&"value".to_string()));
    }
    
    #[test]
    fn test_tarot_error_types() {
        let invalid_target = TarotError::InvalidTarget {
            card_id: ConsumableId::TheFool,
            reason: "Wrong target type".to_string(),
        };
        
        assert!(invalid_target.to_string().contains("TheFool"));
        assert!(invalid_target.to_string().contains("Wrong target type"));
        
        let timeout = TarotError::PerformanceTimeout {
            card_id: ConsumableId::TheMagician,
            actual_ms: 1500,
            limit_ms: 1000,
        };
        
        assert!(timeout.to_string().contains("1500ms"));
        assert!(timeout.to_string().contains("1000ms"));
    }
    
    #[test]
    fn test_performance_info_defaults() {
        let card = PlaceholderTarotCard {
            id: ConsumableId::TheFool,
            name: "Test",
            description: "Test",
            rarity: TarotRarity::Common,
            target_type: TargetType::None,
        };
        
        let perf_info = card.performance_info();
        assert_eq!(perf_info.avg_execution_time_ns, 500_000);
        assert_eq!(perf_info.max_execution_time_ns, 1_000_000);
        assert_eq!(perf_info.memory_overhead_bytes, 200);
        assert_eq!(perf_info.complexity_score, 1.0);
    }
}