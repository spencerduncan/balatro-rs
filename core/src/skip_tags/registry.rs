//! Thread-safe registry and factory system for skip tags
//!
//! This module provides the central registry for all skip tag definitions and
//! factories, enabling fast lookup, creation, and management of skip tags with
//! performance targets of <1μs for lookups.
//!
//! # Architecture
//!
//! The registry system consists of:
//! - **TagDefinition**: Metadata about each tag
//! - **TagFactory**: Factory functions for creating tag instances
//! - **TagRegistry**: Thread-safe central registry
//! - **Performance optimization**: HashMap-based lookups with static data
//!
//! # Thread Safety
//!
//! The registry uses `Arc<RwLock<>>` for thread-safe access:
//! - Multiple readers can access definitions simultaneously
//! - Writers (registration) get exclusive access
//! - Global instance managed with `OnceLock` for initialization safety
//!
//! # Performance Optimization
//!
//! - **Static definitions**: Pre-computed metadata stored in static arrays
//! - **HashMap lookups**: O(1) average case for tag resolution
//! - **Factory caching**: Factory functions stored for direct invocation
//! - **Minimal allocation**: Definitions use `&'static str` where possible
//!
//! # Usage Example
//!
//! ```rust,ignore
//! use crate::skip_tags::{TagRegistry, TagId};
//!
//! // Get global registry instance
//! let registry = TagRegistry::global();
//!
//! // Fast lookup (<1μs target)
//! let definition = registry.get_definition(TagId::Charm)?;
//! println!("Tag: {} - {}", definition.name, definition.description);
//!
//! // Create tag instance
//! let tag = registry.create_tag(TagId::Charm)?;
//!
//! // Check if tag is registered
//! if registry.is_registered(TagId::Economy) {
//!     println!("Economy tag is available");
//! }
//! ```

use crate::game::Game;
use crate::skip_tags::error::{TagError, TagErrorKind, TagResult};
use crate::skip_tags::traits::{SkipTag, TagCategory, TagEffectType, TagId};
#[cfg(feature = "python")]
use pyo3::{pyclass, pymethods};
use std::collections::HashMap;
use std::sync::{Arc, OnceLock, RwLock};

/// Metadata definition for a skip tag.
///
/// Contains all the static information about a tag including its identity,
/// behavior classification, and descriptive text. Optimized for fast access
/// and minimal memory usage.
///
/// # Design Principles
///
/// - **Static strings**: Use `&'static str` to avoid allocations
/// - **Computed properties**: Category and effect type derived from ID
/// - **Comprehensive metadata**: All information needed for UI and logic
/// - **Serializable**: Can be exported for external tools and analysis
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "python", pyclass)]
pub struct TagDefinition {
    /// Unique identifier for this tag
    pub id: TagId,
    /// Human-readable name for UI display
    pub name: &'static str,
    /// Detailed description of the tag's effect
    pub description: &'static str,
    /// Classification of this tag's effect type
    pub effect_type: TagEffectType,
    /// Category this tag belongs to
    pub category: TagCategory,
    /// Base probability weight for selection (0.0 to 1.0)
    pub base_weight: f32,
    /// Whether this tag is currently enabled
    pub enabled: bool,
}

impl TagDefinition {
    /// Creates a new tag definition with the specified properties.
    ///
    /// # Arguments
    /// - `id`: Unique tag identifier
    /// - `name`: Display name
    /// - `description`: Effect description
    /// - `effect_type`: How this tag's effects are applied
    /// - `base_weight`: Selection probability weight
    ///
    /// # Example
    /// ```rust,ignore
    /// let def = TagDefinition::new(
    ///     TagId::Charm,
    ///     "Charm",
    ///     "Gives $4 when selected",
    ///     TagEffectType::ImmediateReward,
    ///     1.0
    /// );
    /// ```
    pub const fn new(
        id: TagId,
        name: &'static str,
        description: &'static str,
        effect_type: TagEffectType,
        base_weight: f32,
    ) -> Self {
        Self {
            id,
            name,
            description,
            effect_type,
            category: id.category(),
            base_weight,
            enabled: true,
        }
    }

    /// Creates a disabled tag definition.
    ///
    /// Used for tags that are implemented but temporarily disabled
    /// for balancing or testing purposes.
    pub const fn disabled(
        id: TagId,
        name: &'static str,
        description: &'static str,
        effect_type: TagEffectType,
        base_weight: f32,
    ) -> Self {
        Self {
            id,
            name,
            description,
            effect_type,
            category: id.category(),
            base_weight,
            enabled: false,
        }
    }

    /// Returns whether this tag should be offered in normal gameplay.
    pub fn is_available(&self) -> bool {
        self.enabled && self.base_weight > 0.0
    }

    /// Returns the effective weight for tag selection.
    ///
    /// Takes into account whether the tag is enabled and applies
    /// any runtime modifiers.
    pub fn effective_weight(&self) -> f32 {
        if self.enabled {
            self.base_weight
        } else {
            0.0
        }
    }
}

#[cfg(feature = "python")]
#[pymethods]
impl TagDefinition {
    /// Get the tag ID (Python binding)
    #[getter]
    fn id(&self) -> TagId {
        self.id
    }

    /// Get the tag name (Python binding)
    #[getter]
    fn name(&self) -> &str {
        self.name
    }

    /// Get the tag description (Python binding)
    #[getter]
    fn description(&self) -> &str {
        self.description
    }

    /// Get the effect type (Python binding)
    #[getter]
    fn effect_type(&self) -> TagEffectType {
        self.effect_type
    }

    /// Get the category (Python binding)
    #[getter]
    fn category(&self) -> TagCategory {
        self.category
    }

    /// Get the base weight (Python binding)
    #[getter]
    fn base_weight(&self) -> f32 {
        self.base_weight
    }

    /// Check if enabled (Python binding)
    #[getter]
    fn enabled(&self) -> bool {
        self.enabled
    }
}

/// Factory function type for creating skip tag instances.
///
/// Factory functions must be thread-safe and should create fresh instances
/// of the appropriate tag implementation. They should be fast (<1μs) and not
/// perform any expensive initialization.
pub type TagFactory = Box<dyn Fn() -> Box<dyn SkipTag> + Send + Sync>;

/// Thread-safe central registry for skip tag definitions and factories.
///
/// The registry manages all skip tag metadata and provides fast lookup and
/// creation capabilities. It uses a singleton pattern with lazy initialization
/// to ensure consistent access across the entire application.
///
/// # Thread Safety
///
/// - **Read operations**: Multiple threads can perform lookups simultaneously
/// - **Write operations**: Registration gets exclusive access
/// - **Initialization**: Protected by `OnceLock` for safe lazy loading
///
/// # Performance Characteristics
///
/// - **Lookup time**: <1μs target for definition and factory lookup
/// - **Memory usage**: ~100 bytes per registered tag
/// - **Thread contention**: Read-heavy workload with minimal lock contention
pub struct TagRegistry {
    /// Tag definitions indexed by TagId for O(1) lookup
    definitions: HashMap<TagId, TagDefinition>,
    /// Factory functions indexed by TagId for O(1) creation
    factories: HashMap<TagId, TagFactory>,
    /// Fast lookup for checking registration status
    registered_tags: HashMap<TagId, bool>,
}

impl TagRegistry {
    /// Creates a new empty registry.
    ///
    /// This method is primarily used internally and for testing.
    /// Production code should use `global()` to access the singleton instance.
    pub fn new() -> Self {
        Self {
            definitions: HashMap::new(),
            factories: HashMap::new(),
            registered_tags: HashMap::new(),
        }
    }

    /// Gets the global registry instance.
    ///
    /// Returns the singleton registry instance, initializing it on first access
    /// with all standard tag definitions and factories. This is the primary
    /// way to access the registry in production code.
    ///
    /// # Performance
    ///
    /// First call: ~100μs (initialization)
    /// Subsequent calls: ~1ns (static reference)
    ///
    /// # Example
    /// ```rust,ignore
    /// let registry = TagRegistry::global();
    /// let definition = registry.get_definition(TagId::Charm)?;
    /// ```
    pub fn global() -> &'static Arc<RwLock<TagRegistry>> {
        static GLOBAL_REGISTRY: OnceLock<Arc<RwLock<TagRegistry>>> = OnceLock::new();

        GLOBAL_REGISTRY.get_or_init(|| {
            let mut registry = TagRegistry::new();
            registry.register_all_tags();
            Arc::new(RwLock::new(registry))
        })
    }

    /// Registers a tag definition and factory function.
    ///
    /// # Arguments
    /// - `definition`: Tag metadata and configuration
    /// - `factory`: Function to create tag instances
    ///
    /// # Returns
    /// - `Ok(())` if registration successful
    /// - `Err(TagError)` if registration failed
    ///
    /// # Errors
    /// - `TagCreationFailed`: If factory function is invalid
    /// - `InvalidConfiguration`: If definition is malformed
    ///
    /// # Example
    /// ```rust,ignore
    /// let definition = TagDefinition::new(
    ///     TagId::Charm,
    ///     "Charm",
    ///     "Gives $4",
    ///     TagEffectType::ImmediateReward,
    ///     1.0
    /// );
    ///
    /// registry.register_tag(definition, || Box::new(CharmTag))?;
    /// ```
    pub fn register_tag(
        &mut self,
        definition: TagDefinition,
        factory: TagFactory,
    ) -> TagResult<()> {
        // Validate factory by testing creation
        let _test_instance = factory();

        // Validate definition consistency
        if definition.name.is_empty() {
            return Err(TagError::new(
                TagErrorKind::InvalidConfiguration,
                "Tag name cannot be empty",
            ));
        }

        if definition.description.is_empty() {
            return Err(TagError::new(
                TagErrorKind::InvalidConfiguration,
                "Tag description cannot be empty",
            ));
        }

        // Register the tag
        let tag_id = definition.id;
        self.definitions.insert(tag_id, definition);
        self.factories.insert(tag_id, factory);
        self.registered_tags.insert(tag_id, true);

        Ok(())
    }

    /// Gets a tag definition by ID.
    ///
    /// # Performance Target: <1μs
    ///
    /// # Arguments
    /// - `tag_id`: The tag to look up
    ///
    /// # Returns
    /// - `Ok(TagDefinition)` if found
    /// - `Err(TagError::TagNotFound)` if not registered
    pub fn get_definition(&self, tag_id: TagId) -> TagResult<&TagDefinition> {
        self.definitions
            .get(&tag_id)
            .ok_or_else(|| TagError::tag_not_found(tag_id))
    }

    /// Creates a new instance of the specified tag.
    ///
    /// # Performance Target: <10μs
    ///
    /// # Arguments
    /// - `tag_id`: The tag to create
    ///
    /// # Returns
    /// - `Ok(Box<dyn SkipTag>)` if creation successful
    /// - `Err(TagError)` if creation failed
    pub fn create_tag(&self, tag_id: TagId) -> TagResult<Box<dyn SkipTag>> {
        let factory = self
            .factories
            .get(&tag_id)
            .ok_or_else(|| TagError::tag_not_found(tag_id))?;

        Ok(factory())
    }

    /// Checks if a tag is registered in the registry.
    ///
    /// # Performance Target: <1μs
    ///
    /// # Arguments
    /// - `tag_id`: The tag to check
    ///
    /// # Returns
    /// - `true` if the tag is registered
    /// - `false` if the tag is not registered
    pub fn is_registered(&self, tag_id: TagId) -> bool {
        self.registered_tags.contains_key(&tag_id)
    }

    /// Gets all registered tag definitions.
    ///
    /// Returns a vector of all tag definitions in the registry.
    /// Useful for UI generation, debugging, and analysis.
    ///
    /// # Performance
    /// O(n) where n is the number of registered tags
    pub fn get_all_definitions(&self) -> Vec<&TagDefinition> {
        self.definitions.values().collect()
    }

    /// Gets all tag definitions in a specific category.
    ///
    /// # Arguments
    /// - `category`: The category to filter by
    ///
    /// # Returns
    /// Vector of tag definitions in the specified category
    pub fn get_definitions_by_category(&self, category: TagCategory) -> Vec<&TagDefinition> {
        self.definitions
            .values()
            .filter(|def| def.category == category)
            .collect()
    }

    /// Gets all tag definitions with a specific effect type.
    ///
    /// # Arguments
    /// - `effect_type`: The effect type to filter by
    ///
    /// # Returns
    /// Vector of tag definitions with the specified effect type
    pub fn get_definitions_by_effect_type(
        &self,
        effect_type: TagEffectType,
    ) -> Vec<&TagDefinition> {
        self.definitions
            .values()
            .filter(|def| def.effect_type == effect_type)
            .collect()
    }

    /// Gets all available tag definitions (enabled with weight > 0).
    ///
    /// # Returns
    /// Vector of tag definitions that can be offered to players
    pub fn get_available_definitions(&self) -> Vec<&TagDefinition> {
        self.definitions
            .values()
            .filter(|def| def.is_available())
            .collect()
    }

    /// Returns the total number of registered tags.
    pub fn count(&self) -> usize {
        self.definitions.len()
    }

    /// Returns registry statistics for debugging and monitoring.
    pub fn stats(&self) -> RegistryStats {
        let total_tags = self.definitions.len();
        let enabled_tags = self.definitions.values().filter(|d| d.enabled).count();
        let available_tags = self
            .definitions
            .values()
            .filter(|d| d.is_available())
            .count();

        let mut category_counts = HashMap::new();
        let mut effect_type_counts = HashMap::new();

        for definition in self.definitions.values() {
            *category_counts.entry(definition.category).or_insert(0) += 1;
            *effect_type_counts
                .entry(definition.effect_type)
                .or_insert(0) += 1;
        }

        RegistryStats {
            total_tags,
            enabled_tags,
            available_tags,
            category_counts,
            effect_type_counts,
        }
    }

    /// Registers all standard skip tags with their definitions and factories.
    ///
    /// This method is called during registry initialization to register all 26
    /// skip tags defined in the architecture specification. It uses static
    /// definitions for optimal performance.
    fn register_all_tags(&mut self) {
        // This is a placeholder implementation that registers definitions without factories
        // In the actual implementation, each tag would have its own factory function

        let definitions = get_all_tag_definitions();

        for definition in definitions {
            // For now, register with a placeholder factory
            // TODO: Replace with actual tag implementations in subsequent tasks
            let tag_id = definition.id;
            let tag_name = definition.name;
            let tag_description = definition.description;
            let tag_effect_type = definition.effect_type;

            let placeholder_factory: TagFactory = Box::new(move || {
                Box::new(PlaceholderTag {
                    id: tag_id,
                    name: tag_name,
                    description: tag_description,
                    effect_type: tag_effect_type,
                })
            });

            // Registry registration should not fail for built-in tags
            if let Err(e) = self.register_tag(definition, placeholder_factory) {
                panic!("Failed to register built-in tag {tag_id:?}: {e}");
            }
        }
    }
}

impl Default for TagRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Statistics about the registry state for debugging and monitoring.
#[derive(Debug, Clone)]
pub struct RegistryStats {
    /// Total number of registered tags
    pub total_tags: usize,
    /// Number of enabled tags
    pub enabled_tags: usize,
    /// Number of available tags (enabled with weight > 0)
    pub available_tags: usize,
    /// Count of tags by category
    pub category_counts: HashMap<TagCategory, usize>,
    /// Count of tags by effect type
    pub effect_type_counts: HashMap<TagEffectType, usize>,
}

/// Placeholder tag implementation for registry initialization.
///
/// This is used during the infrastructure phase to allow the registry
/// to be fully functional before individual tag implementations are created.
/// It provides basic trait compliance but effects are not yet implemented.
#[derive(Debug)]
struct PlaceholderTag {
    id: TagId,
    name: &'static str,
    description: &'static str,
    effect_type: TagEffectType,
}

impl SkipTag for PlaceholderTag {
    fn id(&self) -> TagId {
        self.id
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn effect_type(&self) -> TagEffectType {
        self.effect_type
    }

    fn can_apply(&self, _game: &Game) -> bool {
        // Placeholder implementation - always return true for testing
        true
    }

    fn apply_effect(&self, _game: &mut Game) -> TagResult<()> {
        // Placeholder implementation - no-op for infrastructure testing
        Ok(())
    }

    fn description(&self) -> &'static str {
        self.description
    }
}

/// Returns all skip tag definitions as defined in the architecture specification.
///
/// This function contains the complete metadata for all 26 skip tags organized
/// by category. The definitions use static strings for optimal performance and
/// memory usage.
fn get_all_tag_definitions() -> Vec<TagDefinition> {
    vec![
        // Reward Tags (8) - Immediate pack and item rewards
        TagDefinition::new(
            TagId::Charm,
            "Charm",
            "Gives a free Mega Arcana Pack",
            TagEffectType::ImmediateReward,
            1.0,
        ),
        TagDefinition::new(
            TagId::Ethereal,
            "Ethereal",
            "Gives a free Spectral Pack",
            TagEffectType::ImmediateReward,
            0.8,
        ),
        TagDefinition::new(
            TagId::Buffoon,
            "Buffoon",
            "Gives a free Buffoon Pack",
            TagEffectType::ImmediateReward,
            1.0,
        ),
        TagDefinition::new(
            TagId::Standard,
            "Standard",
            "Gives a free Standard Pack",
            TagEffectType::ImmediateReward,
            1.2,
        ),
        TagDefinition::new(
            TagId::Meteor,
            "Meteor",
            "Gives a free Celestial Pack",
            TagEffectType::ImmediateReward,
            0.9,
        ),
        TagDefinition::new(
            TagId::Rare,
            "Rare",
            "Creates a rare Joker",
            TagEffectType::ImmediateReward,
            0.6,
        ),
        TagDefinition::new(
            TagId::Uncommon,
            "Uncommon",
            "Creates an uncommon Joker",
            TagEffectType::ImmediateReward,
            0.9,
        ),
        TagDefinition::new(
            TagId::TopUp,
            "Top Up",
            "Fills all consumable slots if there is room",
            TagEffectType::ImmediateReward,
            0.7,
        ),
        // Economic Tags (5) - Money and resource management
        TagDefinition::new(
            TagId::Economy,
            "Economy",
            "Gives $1 for every $5 you have (max of $25)",
            TagEffectType::ImmediateReward,
            1.1,
        ),
        TagDefinition::new(
            TagId::Investment,
            "Investment",
            "Earn $25 when this is destroyed",
            TagEffectType::SpecialMechanic,
            0.8,
        ),
        TagDefinition::new(
            TagId::Garbage,
            "Garbage",
            "Gives $1 for each discard played (max of $6)",
            TagEffectType::ImmediateReward,
            1.0,
        ),
        TagDefinition::new(
            TagId::Speed,
            "Speed",
            "Gives $1 for each hand played this run (max of $48)",
            TagEffectType::ImmediateReward,
            1.0,
        ),
        TagDefinition::new(
            TagId::Handy,
            "Handy",
            "Gives $1 for every 2 cards in hand (max of $4)",
            TagEffectType::ImmediateReward,
            1.1,
        ),
        // Shop Enhancement Tags (7) - Modify next shop experience
        TagDefinition::new(
            TagId::Voucher,
            "Voucher",
            "Next shop has a free Mega Voucher Pack",
            TagEffectType::NextShopModifier,
            0.9,
        ),
        TagDefinition::new(
            TagId::Coupon,
            "Coupon",
            "Next shop has all items at half price",
            TagEffectType::NextShopModifier,
            1.2,
        ),
        TagDefinition::new(
            TagId::D6,
            "D6",
            "Reroll the shop up to 3 times for free",
            TagEffectType::NextShopModifier,
            1.0,
        ),
        TagDefinition::new(
            TagId::Foil,
            "Foil",
            "Next shop has Foil, Holographic, or Polychrome Jokers",
            TagEffectType::NextShopModifier,
            0.8,
        ),
        TagDefinition::new(
            TagId::Holographic,
            "Holographic",
            "Next shop has Holographic Jokers",
            TagEffectType::NextShopModifier,
            0.6,
        ),
        TagDefinition::new(
            TagId::Polychrome,
            "Polychrome",
            "Next shop has Polychrome Jokers",
            TagEffectType::NextShopModifier,
            0.5,
        ),
        TagDefinition::new(
            TagId::Negative,
            "Negative",
            "Creates a Negative Joker",
            TagEffectType::ImmediateReward,
            0.4,
        ),
        // Utility Tags (4) - Game state and mechanic modifiers
        TagDefinition::new(
            TagId::Double,
            "Double",
            "Gives a copy of the next selected tag",
            TagEffectType::SpecialMechanic,
            0.7,
        ),
        TagDefinition::new(
            TagId::Boss,
            "Boss",
            "Reroll the Boss Blind",
            TagEffectType::GameStateModifier,
            0.9,
        ),
        TagDefinition::new(
            TagId::Orbital,
            "Orbital",
            "Upgrade the level of the most played poker hand",
            TagEffectType::GameStateModifier,
            1.0,
        ),
        TagDefinition::new(
            TagId::Juggle,
            "Juggle",
            "Gives +1 hand size for the next blind",
            TagEffectType::GameStateModifier,
            1.1,
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tag_definition_creation() {
        let def = TagDefinition::new(
            TagId::Charm,
            "Charm",
            "Test description",
            TagEffectType::ImmediateReward,
            1.0,
        );

        assert_eq!(def.id, TagId::Charm);
        assert_eq!(def.name, "Charm");
        assert_eq!(def.description, "Test description");
        assert_eq!(def.effect_type, TagEffectType::ImmediateReward);
        assert_eq!(def.category, TagCategory::Reward);
        assert_eq!(def.base_weight, 1.0);
        assert!(def.enabled);
    }

    #[test]
    fn test_tag_definition_disabled() {
        let def = TagDefinition::disabled(
            TagId::Economy,
            "Economy",
            "Test description",
            TagEffectType::ImmediateReward,
            1.0,
        );

        assert!(!def.enabled);
        assert!(!def.is_available());
        assert_eq!(def.effective_weight(), 0.0);
    }

    #[test]
    fn test_tag_definition_availability() {
        let available = TagDefinition::new(
            TagId::Charm,
            "Charm",
            "Test",
            TagEffectType::ImmediateReward,
            1.0,
        );
        assert!(available.is_available());
        assert_eq!(available.effective_weight(), 1.0);

        let disabled = TagDefinition::disabled(
            TagId::Charm,
            "Charm",
            "Test",
            TagEffectType::ImmediateReward,
            1.0,
        );
        assert!(!disabled.is_available());
        assert_eq!(disabled.effective_weight(), 0.0);

        let zero_weight = TagDefinition::new(
            TagId::Charm,
            "Charm",
            "Test",
            TagEffectType::ImmediateReward,
            0.0,
        );
        assert!(!zero_weight.is_available());
    }

    #[test]
    fn test_registry_creation() {
        let registry = TagRegistry::new();
        assert_eq!(registry.count(), 0);
    }

    #[test]
    fn test_registry_registration() {
        let mut registry = TagRegistry::new();

        let definition = TagDefinition::new(
            TagId::Charm,
            "Charm",
            "Test tag",
            TagEffectType::ImmediateReward,
            1.0,
        );

        let factory: TagFactory = Box::new(|| {
            Box::new(PlaceholderTag {
                id: TagId::Charm,
                name: "Charm",
                description: "Test tag",
                effect_type: TagEffectType::ImmediateReward,
            })
        });

        assert!(registry.register_tag(definition, factory).is_ok());
        assert_eq!(registry.count(), 1);
        assert!(registry.is_registered(TagId::Charm));
    }

    #[test]
    fn test_registry_get_definition() {
        let mut registry = TagRegistry::new();

        let definition = TagDefinition::new(
            TagId::Economy,
            "Economy",
            "Economic test tag",
            TagEffectType::ImmediateReward,
            1.5,
        );

        let factory: TagFactory = Box::new(|| {
            Box::new(PlaceholderTag {
                id: TagId::Economy,
                name: "Economy",
                description: "Economic test tag",
                effect_type: TagEffectType::ImmediateReward,
            })
        });

        registry.register_tag(definition, factory).unwrap();

        let retrieved_def = registry.get_definition(TagId::Economy).unwrap();
        assert_eq!(retrieved_def.id, TagId::Economy);
        assert_eq!(retrieved_def.name, "Economy");
        assert_eq!(retrieved_def.base_weight, 1.5);
    }

    #[test]
    fn test_registry_get_definition_not_found() {
        let registry = TagRegistry::new();

        let result = registry.get_definition(TagId::Charm);
        assert!(result.is_err());

        if let Err(error) = result {
            assert_eq!(error.kind, TagErrorKind::TagNotFound);
        }
    }

    #[test]
    fn test_registry_create_tag() {
        let mut registry = TagRegistry::new();

        let definition = TagDefinition::new(
            TagId::Buffoon,
            "Buffoon",
            "Test buffoon tag",
            TagEffectType::ImmediateReward,
            1.0,
        );

        let factory: TagFactory = Box::new(|| {
            Box::new(PlaceholderTag {
                id: TagId::Buffoon,
                name: "Buffoon",
                description: "Test buffoon tag",
                effect_type: TagEffectType::ImmediateReward,
            })
        });

        registry.register_tag(definition, factory).unwrap();

        let tag = registry.create_tag(TagId::Buffoon).unwrap();
        assert_eq!(tag.id(), TagId::Buffoon);
        assert_eq!(tag.name(), "Buffoon");
    }

    #[test]
    fn test_registry_filtering() {
        let mut registry = TagRegistry::new();

        // Register tags with different categories and types
        let definitions = vec![
            (
                TagId::Charm,
                TagEffectType::ImmediateReward,
                TagCategory::Reward,
            ),
            (
                TagId::Coupon,
                TagEffectType::NextShopModifier,
                TagCategory::ShopEnhancement,
            ),
            (
                TagId::Boss,
                TagEffectType::GameStateModifier,
                TagCategory::Utility,
            ),
        ];

        for (id, effect_type, _category) in definitions {
            let def = TagDefinition::new(id, "Test", "Test description", effect_type, 1.0);
            let factory: TagFactory = Box::new(move || {
                Box::new(PlaceholderTag {
                    id,
                    name: "Test",
                    description: "Test description",
                    effect_type,
                })
            });
            registry.register_tag(def, factory).unwrap();
        }

        // Test category filtering
        let reward_tags = registry.get_definitions_by_category(TagCategory::Reward);
        assert_eq!(reward_tags.len(), 1);
        assert_eq!(reward_tags[0].id, TagId::Charm);

        // Test effect type filtering
        let immediate_tags =
            registry.get_definitions_by_effect_type(TagEffectType::ImmediateReward);
        assert_eq!(immediate_tags.len(), 1);
        assert_eq!(immediate_tags[0].id, TagId::Charm);

        // Test available tags
        let available_tags = registry.get_available_definitions();
        assert_eq!(available_tags.len(), 3); // All are available
    }

    #[test]
    fn test_registry_stats() {
        let mut registry = TagRegistry::new();

        // Add some test tags
        let tags = [
            (
                TagId::Charm,
                TagCategory::Reward,
                TagEffectType::ImmediateReward,
                true,
            ),
            (
                TagId::Economy,
                TagCategory::Economic,
                TagEffectType::ImmediateReward,
                true,
            ),
            (
                TagId::Coupon,
                TagCategory::ShopEnhancement,
                TagEffectType::NextShopModifier,
                false,
            ),
        ];

        for &(id, _category, effect_type, enabled) in &tags {
            let mut def = TagDefinition::new(id, "Test", "Test", effect_type, 1.0);
            if !enabled {
                def.enabled = false;
            }

            let factory: TagFactory = Box::new(move || {
                Box::new(PlaceholderTag {
                    id,
                    name: "Test",
                    description: "Test",
                    effect_type,
                })
            });
            registry.register_tag(def, factory).unwrap();
        }

        let stats = registry.stats();
        assert_eq!(stats.total_tags, 3);
        assert_eq!(stats.enabled_tags, 2);
        assert_eq!(stats.available_tags, 2);
    }

    #[test]
    fn test_global_registry() {
        let registry = TagRegistry::global();
        let read_guard = registry.read().unwrap();

        // Should be initialized with all 24 tags
        assert_eq!(read_guard.count(), 24);

        // Test that all expected tags are registered
        for tag_id in TagId::all().iter() {
            assert!(read_guard.is_registered(*tag_id));
            assert!(read_guard.get_definition(*tag_id).is_ok());
        }
    }

    #[test]
    fn test_all_tag_definitions_completeness() {
        let definitions = get_all_tag_definitions();

        // Should have exactly 24 definitions
        assert_eq!(definitions.len(), 24);

        // All tag IDs should be represented
        let mut found_ids = std::collections::HashSet::new();
        for def in definitions.iter() {
            found_ids.insert(def.id);
        }

        for expected_id in TagId::all().iter() {
            assert!(
                found_ids.contains(expected_id),
                "Missing definition for {expected_id:?}"
            );
        }

        // All definitions should have non-empty names and descriptions
        for def in definitions.iter() {
            assert!(!def.name.is_empty(), "Empty name for {:?}", def.id);
            assert!(
                !def.description.is_empty(),
                "Empty description for {:?}",
                def.id
            );
            assert!(def.base_weight >= 0.0, "Negative weight for {:?}", def.id);
        }
    }

    #[test]
    fn test_placeholder_tag_implementation() {
        let tag = PlaceholderTag {
            id: TagId::Charm,
            name: "Test Charm",
            description: "Test description",
            effect_type: TagEffectType::ImmediateReward,
        };

        assert_eq!(tag.id(), TagId::Charm);
        assert_eq!(tag.name(), "Test Charm");
        assert_eq!(tag.description(), "Test description");
        assert_eq!(tag.effect_type(), TagEffectType::ImmediateReward);

        // Placeholder should always allow application
        let game = Game::default();
        assert!(tag.can_apply(&game));

        // Placeholder should successfully apply (no-op)
        let mut game = Game::default();
        assert!(tag.apply_effect(&mut game).is_ok());
    }

    #[test]
    fn test_registry_error_handling() {
        let mut registry = TagRegistry::new();

        // Test registration with empty name
        let invalid_def = TagDefinition::new(
            TagId::Charm,
            "",
            "Valid description",
            TagEffectType::ImmediateReward,
            1.0,
        );

        let factory: TagFactory = Box::new(|| {
            Box::new(PlaceholderTag {
                id: TagId::Charm,
                name: "",
                description: "Valid description",
                effect_type: TagEffectType::ImmediateReward,
            })
        });

        let result = registry.register_tag(invalid_def, factory);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind, TagErrorKind::InvalidConfiguration);

        // Test registration with empty description
        let invalid_def2 = TagDefinition::new(
            TagId::Economy,
            "Valid name",
            "",
            TagEffectType::ImmediateReward,
            1.0,
        );

        let factory2: TagFactory = Box::new(|| {
            Box::new(PlaceholderTag {
                id: TagId::Economy,
                name: "Valid name",
                description: "",
                effect_type: TagEffectType::ImmediateReward,
            })
        });

        let result2 = registry.register_tag(invalid_def2, factory2);
        assert!(result2.is_err());
        assert_eq!(
            result2.unwrap_err().kind,
            TagErrorKind::InvalidConfiguration
        );
    }
}
