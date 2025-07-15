use crate::error::GameError;
use crate::joker::{Joker, JokerId, JokerRarity};
#[cfg(feature = "python")]
use pyo3::{pyclass, pymethods};
use std::collections::HashMap;
use std::sync::{Arc, OnceLock, RwLock};

/// Definition of a joker's metadata and properties
#[derive(Debug, Clone)]
#[cfg_attr(feature = "python", pyclass)]
pub struct JokerDefinition {
    pub id: JokerId,
    pub name: String,
    pub description: String,
    pub rarity: JokerRarity,
    pub unlock_condition: Option<UnlockCondition>,
}

/// Represents conditions that must be met to unlock a joker
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub enum UnlockCondition {
    /// Reach a certain ante level
    ReachAnte(u32),
    /// Win with a specific deck
    WinWithDeck(String),
    /// Score a certain amount in a single hand
    ScoreInHand(u64),
    /// Have a certain amount of money
    HaveMoney(u32),
    /// Play a certain number of hands
    PlayHands(u32),
    /// Custom unlock condition with description
    Custom(String),
}

#[cfg_attr(feature = "python", pymethods)]
impl JokerDefinition {
    /// Get the joker ID
    #[cfg(feature = "python")]
    #[getter]
    fn id(&self) -> JokerId {
        self.id
    }

    /// Get the joker name
    #[cfg(feature = "python")]
    #[getter]
    fn name(&self) -> String {
        self.name.clone()
    }

    /// Get the joker description
    #[cfg(feature = "python")]
    #[getter]
    fn description(&self) -> String {
        self.description.clone()
    }

    /// Get the joker rarity
    #[cfg(feature = "python")]
    #[getter]
    fn rarity(&self) -> JokerRarity {
        self.rarity
    }

    /// Get the unlock condition
    #[cfg(feature = "python")]
    #[getter]
    fn unlock_condition(&self) -> Option<UnlockCondition> {
        self.unlock_condition.clone()
    }
}

/// Factory function type for creating joker instances
type JokerFactory = fn() -> Box<dyn Joker>;

/// Central registry for all joker definitions and factories
pub struct JokerRegistry {
    definitions: HashMap<JokerId, JokerDefinition>,
    factories: HashMap<JokerId, JokerFactory>,
}

impl JokerRegistry {
    /// Creates a new empty registry
    pub fn new() -> Self {
        Self {
            definitions: HashMap::new(),
            factories: HashMap::new(),
        }
    }

    /// Registers a joker definition with its factory function
    pub fn register(
        &mut self,
        definition: JokerDefinition,
        factory: JokerFactory,
    ) -> Result<(), GameError> {
        let id = definition.id;

        if self.definitions.contains_key(&id) {
            return Err(GameError::InvalidOperation(format!(
                "Joker {id:?} is already registered"
            )));
        }

        self.definitions.insert(id, definition);
        self.factories.insert(id, factory);
        Ok(())
    }

    /// Looks up a joker definition by ID
    pub fn get_definition(&self, id: &JokerId) -> Option<&JokerDefinition> {
        self.definitions.get(id)
    }

    /// Creates a new instance of a joker by ID
    pub fn create_joker(&self, id: &JokerId) -> Result<Box<dyn Joker>, GameError> {
        self.factories
            .get(id)
            .map(|factory| factory())
            .ok_or_else(|| GameError::JokerNotFound(format!("{id:?}")))
    }

    /// Returns all registered joker definitions
    pub fn all_definitions(&self) -> Vec<&JokerDefinition> {
        self.definitions.values().collect()
    }

    /// Returns definitions filtered by rarity
    pub fn definitions_by_rarity(&self, rarity: JokerRarity) -> Vec<&JokerDefinition> {
        self.definitions
            .values()
            .filter(|def| def.rarity == rarity)
            .collect()
    }

    /// Returns unlocked joker definitions based on a predicate
    pub fn unlocked_definitions<F>(&self, is_unlocked: F) -> Vec<&JokerDefinition>
    where
        F: Fn(&Option<UnlockCondition>) -> bool,
    {
        self.definitions
            .values()
            .filter(|def| is_unlocked(&def.unlock_condition))
            .collect()
    }

    /// Checks if a joker is registered
    pub fn is_registered(&self, id: &JokerId) -> bool {
        self.definitions.contains_key(id)
    }

    /// Returns the number of registered jokers
    pub fn count(&self) -> usize {
        self.definitions.len()
    }
}

impl Default for JokerRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Global joker registry instance
static JOKER_REGISTRY: OnceLock<Arc<RwLock<JokerRegistry>>> = OnceLock::new();

/// Gets or initializes the global joker registry
fn get_registry() -> &'static Arc<RwLock<JokerRegistry>> {
    JOKER_REGISTRY.get_or_init(|| {
        let mut registry = JokerRegistry::new();

        // Initialize with existing joker implementations
        // This will be populated by the initialization module
        initialize_default_jokers(&mut registry);

        Arc::new(RwLock::new(registry))
    })
}

/// Initializes the registry with default joker implementations
fn initialize_default_jokers(registry: &mut JokerRegistry) {
    // Register basic jokers
    register_joker(registry, JokerId::Joker, "Joker", "+4 Mult", JokerRarity::Common, None, create_the_joker);
    register_joker(registry, JokerId::GreedyJoker, "Greedy Joker", "Played cards with Diamond suit give +3 Mult when scored", JokerRarity::Common, None, create_greedy_joker);
    register_joker(registry, JokerId::LustyJoker, "Lusty Joker", "Played cards with Heart suit give +3 Mult when scored", JokerRarity::Common, None, create_lusty_joker);
    register_joker(registry, JokerId::WrathfulJoker, "Wrathful Joker", "Played cards with Spade suit give +3 Mult when scored", JokerRarity::Common, None, create_wrathful_joker);
    register_joker(registry, JokerId::GluttonousJoker, "Gluttonous Joker", "Played cards with Club suit give +3 Mult when scored", JokerRarity::Common, None, create_gluttonous_joker);
}

// Factory functions for creating joker instances
fn create_the_joker() -> Box<dyn Joker> {
    Box::new(crate::joker_impl::TheJoker)
}

fn create_greedy_joker() -> Box<dyn Joker> {
    Box::new(crate::joker_impl::GreedyJoker)
}

fn create_lusty_joker() -> Box<dyn Joker> {
    Box::new(crate::joker_impl::LustyJoker)
}

fn create_wrathful_joker() -> Box<dyn Joker> {
    Box::new(crate::joker_impl::WrathfulJoker)
}

fn create_gluttonous_joker() -> Box<dyn Joker> {
    Box::new(crate::joker_impl::GluttonousJoker)
}

/// Helper function to register a joker with its definition and factory
fn register_joker(
    registry: &mut JokerRegistry,
    id: JokerId,
    name: &str,
    description: &str,
    rarity: JokerRarity,
    unlock_condition: Option<UnlockCondition>,
    factory: JokerFactory,
) {
    let definition = JokerDefinition {
        id,
        name: name.to_string(),
        description: description.to_string(),
        rarity,
        unlock_condition,
    };
    
    if let Err(e) = registry.register(definition, factory) {
        eprintln!("Failed to register joker {id:?}: {e}");
    }
}

/// Convenience functions for working with the global registry
pub mod registry {
    use super::*;

    /// Creates a new joker instance by ID
    pub fn create_joker(id: &JokerId) -> Result<Box<dyn Joker>, GameError> {
        get_registry()
            .read()
            .map_err(|_| {
                GameError::InvalidOperation("Failed to acquire registry lock".to_string())
            })?
            .create_joker(id)
    }

    /// Gets a joker definition by ID
    pub fn get_definition(id: &JokerId) -> Result<Option<JokerDefinition>, GameError> {
        get_registry()
            .read()
            .map_err(|_| GameError::InvalidOperation("Failed to acquire registry lock".to_string()))
            .map(|registry| registry.get_definition(id).cloned())
    }

    /// Gets all joker definitions
    pub fn all_definitions() -> Result<Vec<JokerDefinition>, GameError> {
        get_registry()
            .read()
            .map_err(|_| GameError::InvalidOperation("Failed to acquire registry lock".to_string()))
            .map(|registry| registry.all_definitions().into_iter().cloned().collect())
    }

    /// Gets joker definitions by rarity
    pub fn definitions_by_rarity(rarity: JokerRarity) -> Result<Vec<JokerDefinition>, GameError> {
        get_registry()
            .read()
            .map_err(|_| GameError::InvalidOperation("Failed to acquire registry lock".to_string()))
            .map(|registry| {
                registry
                    .definitions_by_rarity(rarity)
                    .into_iter()
                    .cloned()
                    .collect()
            })
    }

    /// Registers a joker definition with factory (mainly for tests and extensions)
    pub fn register(definition: JokerDefinition, factory: JokerFactory) -> Result<(), GameError> {
        get_registry()
            .write()
            .map_err(|_| {
                GameError::InvalidOperation("Failed to acquire registry write lock".to_string())
            })?
            .register(definition, factory)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::joker_impl::TheJoker;

    #[test]
    fn test_registry_creation() {
        let registry = JokerRegistry::new();
        assert_eq!(registry.count(), 0);
    }

    #[test]
    fn test_register_joker() {
        let mut registry = JokerRegistry::new();

        let definition = JokerDefinition {
            id: JokerId::Joker,
            name: "The Joker".to_string(),
            description: "+4 Mult".to_string(),
            rarity: JokerRarity::Common,
            unlock_condition: None,
        };

        let result = registry.register(definition.clone(), || Box::new(TheJoker));
        assert!(result.is_ok());
        assert_eq!(registry.count(), 1);

        // Test duplicate registration
        let duplicate_result = registry.register(definition, || Box::new(TheJoker));
        assert!(duplicate_result.is_err());
    }

    #[test]
    fn test_create_joker() {
        let mut registry = JokerRegistry::new();

        let definition = JokerDefinition {
            id: JokerId::Joker,
            name: "The Joker".to_string(),
            description: "+4 Mult".to_string(),
            rarity: JokerRarity::Common,
            unlock_condition: None,
        };

        registry
            .register(definition, || Box::new(TheJoker))
            .unwrap();

        let joker = registry.create_joker(&JokerId::Joker);
        assert!(joker.is_ok());

        let missing = registry.create_joker(&JokerId::GreedyJoker);
        assert!(missing.is_err());
    }

    #[test]
    fn test_get_definition() {
        let mut registry = JokerRegistry::new();

        let definition = JokerDefinition {
            id: JokerId::Joker,
            name: "The Joker".to_string(),
            description: "+4 Mult".to_string(),
            rarity: JokerRarity::Common,
            unlock_condition: None,
        };

        registry
            .register(definition.clone(), || Box::new(TheJoker))
            .unwrap();

        let retrieved = registry.get_definition(&JokerId::Joker);
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().name, "The Joker");
    }

    #[test]
    fn test_filter_by_rarity() {
        let mut registry = JokerRegistry::new();

        // Add common joker
        registry
            .register(
                JokerDefinition {
                    id: JokerId::Joker,
                    name: "The Joker".to_string(),
                    description: "+4 Mult".to_string(),
                    rarity: JokerRarity::Common,
                    unlock_condition: None,
                },
                || Box::new(TheJoker),
            )
            .unwrap();

        // Add rare joker
        registry
            .register(
                JokerDefinition {
                    id: JokerId::GreedyJoker,
                    name: "Greedy Joker".to_string(),
                    description: "Test".to_string(),
                    rarity: JokerRarity::Rare,
                    unlock_condition: None,
                },
                || Box::new(TheJoker), // Using TheJoker as placeholder
            )
            .unwrap();

        let common_jokers = registry.definitions_by_rarity(JokerRarity::Common);
        assert_eq!(common_jokers.len(), 1);

        let rare_jokers = registry.definitions_by_rarity(JokerRarity::Rare);
        assert_eq!(rare_jokers.len(), 1);

        let legendary_jokers = registry.definitions_by_rarity(JokerRarity::Legendary);
        assert_eq!(legendary_jokers.len(), 0);
    }

    #[test]
    fn test_unlocked_definitions() {
        let mut registry = JokerRegistry::new();

        // Add unlocked joker
        registry
            .register(
                JokerDefinition {
                    id: JokerId::Joker,
                    name: "The Joker".to_string(),
                    description: "+4 Mult".to_string(),
                    rarity: JokerRarity::Common,
                    unlock_condition: None,
                },
                || Box::new(TheJoker),
            )
            .unwrap();

        // Add locked joker
        registry
            .register(
                JokerDefinition {
                    id: JokerId::GreedyJoker,
                    name: "Greedy Joker".to_string(),
                    description: "Test".to_string(),
                    rarity: JokerRarity::Common,
                    unlock_condition: Some(UnlockCondition::ReachAnte(5)),
                },
                || Box::new(TheJoker),
            )
            .unwrap();

        let unlocked = registry.unlocked_definitions(|condition| condition.is_none());
        assert_eq!(unlocked.len(), 1);
        assert_eq!(unlocked[0].id, JokerId::Joker);
    }

    #[test]
    fn test_thread_safety() {
        use std::sync::Arc;
        use std::thread;

        let registry = Arc::new(RwLock::new(JokerRegistry::new()));
        let registry_clone = Arc::clone(&registry);

        // Write in one thread
        let write_handle = thread::spawn(move || {
            let mut reg = registry_clone.write().unwrap();
            reg.register(
                JokerDefinition {
                    id: JokerId::Joker,
                    name: "The Joker".to_string(),
                    description: "+4 Mult".to_string(),
                    rarity: JokerRarity::Common,
                    unlock_condition: None,
                },
                || Box::new(TheJoker),
            )
            .unwrap();
        });

        write_handle.join().unwrap();

        // Read in another thread
        let read_handle = thread::spawn(move || {
            let reg = registry.read().unwrap();
            assert_eq!(reg.count(), 1);
        });

        read_handle.join().unwrap();
    }
}
