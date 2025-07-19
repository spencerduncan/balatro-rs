#![allow(deprecated)] // Allow deprecated to_object calls temporarily - will be fixed in PyO3 migration

use balatro_rs::action::Action;
use balatro_rs::ante::Ante;
use balatro_rs::card::Card;
use balatro_rs::config::Config;
use balatro_rs::error::GameError;
use balatro_rs::game::Game;
use balatro_rs::joker::{JokerId, JokerRarity, Jokers};
use balatro_rs::joker_metadata::JokerMetadata;
use balatro_rs::joker_registry::{
    calculate_joker_cost, registry, JokerDefinition, UnlockCondition,
};
use balatro_rs::stage::{End, Stage};
use pyo3::prelude::*;
use pyo3::{PyObject, PyResult, Python};
use std::collections::HashMap;

// Security constants for input validation
const MAX_CUSTOM_DATA_KEY_LENGTH: usize = 256;
const MAX_CUSTOM_DATA_VALUE_LENGTH: usize = 8192;
const MAX_SEARCH_QUERY_LENGTH: usize = 1024;

/// A serializable snapshot of the game state for Python bindings
#[derive(Clone)]
struct GameStateSnapshot {
    stage: Stage,
    round: f64,
    action_history: Vec<Action>,
    deck_cards: Vec<Card>,
    selected_cards: Vec<Card>,
    available_cards: Vec<Card>,
    discarded_cards: Vec<Card>,
    plays: f64,
    discards: f64,
    score: f64,
    required_score: f64,
    joker_ids: Vec<JokerId>,
    joker_count: usize,
    joker_slots_max: usize,
    money: f64,
    ante: Ante,
    is_over: bool,
    #[allow(dead_code)]
    result: Option<End>,
}

impl GameStateSnapshot {
    fn from_game(game: &Game) -> Self {
        Self {
            stage: game.stage,
            round: game.round,
            action_history: game.action_history.clone(),
            deck_cards: game.deck.cards(),
            selected_cards: game.available.selected(),
            available_cards: game.available.cards(),
            discarded_cards: game.discarded.clone(),
            plays: game.plays,
            discards: game.discards,
            score: game.score,
            required_score: game.required_score(),
            joker_ids: game.jokers.iter().map(|j| j.id()).collect(),
            joker_count: game.joker_count(),
            joker_slots_max: game.config.joker_slots_max,
            money: game.money,
            ante: game.ante_current,
            is_over: game.is_over(),
            result: game.result(),
        }
    }
}

#[pyclass]
struct GameEngine {
    game: Game,
}

impl GameEngine {}

#[pymethods]
impl GameEngine {
    #[new]
    #[pyo3(signature = (config=None))]
    fn new(config: Option<Config>) -> Self {
        GameEngine {
            game: Game::new(config.unwrap_or_default()),
        }
    }

    fn gen_actions(&self) -> Vec<Action> {
        self.game.gen_actions().collect()
    }

    fn gen_action_space(&self) -> Vec<usize> {
        self.game.gen_action_space().to_vec()
    }

    fn handle_action(&mut self, action: Action) -> Result<(), GameError> {
        self.game.handle_action(action)
    }

    fn handle_action_index(&mut self, index: usize) -> Result<(), GameError> {
        self.game.handle_action_index(index)
    }

    fn get_action_name(&self, index: usize) -> Result<String, GameError> {
        let space = self.game.gen_action_space();
        let action = space.to_action(index, &self.game)?;
        Ok(format!("{action}"))
    }

    /// Get joker information by ID
    fn get_joker_info(&self, joker_id: JokerId) -> Result<Option<JokerDefinition>, GameError> {
        registry::get_definition(&joker_id)
    }

    /// Get all available joker definitions, optionally filtered by rarity
    #[pyo3(signature = (rarity=None))]
    fn get_available_jokers(
        &self,
        rarity: Option<JokerRarity>,
    ) -> Result<Vec<JokerDefinition>, GameError> {
        match rarity {
            Some(r) => registry::definitions_by_rarity(r),
            None => registry::all_definitions(),
        }
    }

    /// Check if player can afford and has space for a joker
    fn can_buy_joker(&self, joker_id: JokerId) -> bool {
        // Check if player has space
        if self.game.joker_count() >= self.game.config.joker_slots_max {
            return false;
        }

        // Check if player can afford it
        if let Ok(Some(definition)) = registry::get_definition(&joker_id) {
            let cost = calculate_joker_cost(definition.rarity) as usize;
            return self.game.money >= cost;
        }

        false
    }

    /// Get the cost of a specific joker
    fn get_joker_cost(&self, joker_id: JokerId) -> Result<Option<usize>, GameError> {
        if let Some(definition) = registry::get_definition(&joker_id)? {
            Ok(Some(calculate_joker_cost(definition.rarity) as usize))
        } else {
            Ok(None)
        }
    }

    /// Get comprehensive metadata for a specific joker
    fn get_joker_metadata(&self, joker_id: JokerId) -> Result<Option<JokerMetadata>, GameError> {
        let definition = registry::get_definition(&joker_id)?;

        match definition {
            Some(def) => {
                // Check if joker is unlocked based on game progress
                let is_unlocked = def
                    .unlock_condition
                    .as_ref()
                    .map(|condition| evaluate_unlock_condition(condition, &self.game))
                    .unwrap_or(true); // No unlock condition means always unlocked
                let metadata = JokerMetadata::from_definition(&def, is_unlocked);
                Ok(Some(metadata))
            }
            None => Ok(None),
        }
    }

    /// Get basic properties as dictionary
    fn get_joker_properties(&self, joker_id: JokerId) -> Option<pyo3::PyObject> {
        let definition = match registry::get_definition(&joker_id) {
            Ok(Some(def)) => def,
            Ok(None) => return None,
            Err(_) => return None,
        };

        let cost = calculate_joker_cost(definition.rarity);

        pyo3::Python::with_gil(|py| {
            let dict = pyo3::types::PyDict::new(py);
            let _ = dict.set_item("name", &definition.name);
            let _ = dict.set_item("description", &definition.description);
            let _ = dict.set_item("rarity", format!("{:?}", definition.rarity));
            let _ = dict.set_item("cost", cost);

            Some(dict.into())
        })
    }

    /// Get joker effect descriptions and parameters
    fn get_joker_effect_info(&self, joker_id: JokerId) -> Option<pyo3::PyObject> {
        let definition = match registry::get_definition(&joker_id) {
            Ok(Some(def)) => def,
            Ok(None) => return None,
            Err(_) => return None,
        };

        // Determine effect type and triggers based on joker ID
        let (effect_type, triggers_on) = match joker_id {
            JokerId::Joker => ("Basic Mult", vec!["hand_played"]),
            JokerId::GreedyJoker
            | JokerId::LustyJoker
            | JokerId::WrathfulJoker
            | JokerId::GluttonousJoker => ("Conditional Mult", vec!["card_scored"]),
            _ => ("Effect", vec![]),
        };

        pyo3::Python::with_gil(|py| {
            let dict = pyo3::types::PyDict::new(py);
            let _ = dict.set_item("effect_type", effect_type);
            let _ = dict.set_item("description", &definition.description);
            // Extract parameters based on joker implementation
            let parameters = extract_joker_parameters(joker_id, py);
            let _ = dict.set_item("parameters", parameters);
            let _ = dict.set_item("triggers_on", triggers_on);

            Some(dict.into())
        })
    }

    /// Get unlock condition and status
    fn get_joker_unlock_status(&self, joker_id: JokerId) -> Option<pyo3::PyObject> {
        let definition = match registry::get_definition(&joker_id) {
            Ok(Some(def)) => def,
            Ok(None) => return None,
            Err(_) => return None,
        };

        pyo3::Python::with_gil(|py| {
            let dict = pyo3::types::PyDict::new(py);
            // Check if joker is unlocked based on game progress
            let is_unlocked = definition
                .unlock_condition
                .as_ref()
                .map(|condition| evaluate_unlock_condition(condition, &self.game))
                .unwrap_or(true); // No unlock condition means always unlocked
            let _ = dict.set_item("is_unlocked", is_unlocked);

            // Add unlock condition if it exists
            match &definition.unlock_condition {
                Some(condition) => {
                    let _ = dict.set_item("unlock_condition", format!("{condition:?}"));
                }
                None => {
                    let _ = dict.set_item("unlock_condition", py.None());
                }
            }

            Some(dict.into())
        })
    }

    /// Get metadata for multiple jokers efficiently
    /// Returns a dictionary with JokerId objects as keys and JokerMetadata as values
    fn get_multiple_joker_metadata(&self, joker_ids: Vec<JokerId>) -> pyo3::PyObject {
        pyo3::Python::with_gil(|py| {
            let dict = pyo3::types::PyDict::new(py);

            // Acquire registry lock once for batch operation - fixes N+1 pattern
            if let Ok(all_definitions) = registry::all_definitions() {
                let definition_map: HashMap<JokerId, &JokerDefinition> =
                    all_definitions.iter().map(|def| (def.id, def)).collect();

                for joker_id in joker_ids {
                    if let Some(definition) = definition_map.get(&joker_id) {
                        let is_unlocked = definition
                            .unlock_condition
                            .as_ref()
                            .map(|condition| evaluate_unlock_condition(condition, &self.game))
                            .unwrap_or(true);
                        let metadata = JokerMetadata::from_definition(definition, is_unlocked);
                        let _ = dict.set_item(format!("{joker_id:?}"), metadata);
                    }
                }
            }

            dict.into()
        })
    }

    /// Get metadata for all jokers in the registry  
    /// Returns a dictionary with JokerId string names as keys and JokerMetadata as values
    fn get_all_joker_metadata(&self) -> pyo3::PyObject {
        pyo3::Python::with_gil(|py| {
            let dict = pyo3::types::PyDict::new(py);

            // Get all available jokers and their metadata - single registry access fixes N+1 pattern
            if let Ok(all_jokers) = registry::all_definitions() {
                for definition in all_jokers {
                    let is_unlocked = definition
                        .unlock_condition
                        .as_ref()
                        .map(|condition| evaluate_unlock_condition(condition, &self.game))
                        .unwrap_or(true);
                    let metadata = JokerMetadata::from_definition(&definition, is_unlocked);
                    let _ = dict.set_item(format!("{0:?}", definition.id), metadata);
                }
            }

            dict.into()
        })
    }

    /// Get jokers by rarity with optional full metadata
    #[pyo3(signature = (rarity, include_metadata=true))]
    fn get_jokers_by_rarity(
        &self,
        rarity: JokerRarity,
        include_metadata: bool,
    ) -> Vec<JokerMetadata> {
        let include_full = include_metadata;

        if let Ok(jokers) = registry::definitions_by_rarity(rarity) {
            jokers
                .into_iter()
                .filter_map(|def| {
                    if include_full {
                        self.get_joker_metadata(def.id).ok().flatten()
                    } else {
                        // Create minimal metadata with proper unlock checking
                        let is_unlocked = def
                            .unlock_condition
                            .as_ref()
                            .map(|condition| evaluate_unlock_condition(condition, &self.game))
                            .unwrap_or(true); // No unlock condition means always unlocked
                        Some(JokerMetadata::from_definition(&def, is_unlocked))
                    }
                })
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Get metadata for all currently unlocked jokers
    fn get_unlocked_jokers_metadata(&self) -> Vec<JokerMetadata> {
        if let Ok(all_jokers) = registry::all_definitions() {
            all_jokers
                .into_iter()
                .filter_map(|def| {
                    // For now, assume all jokers are unlocked
                    self.get_joker_metadata(def.id).ok().flatten()
                })
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Get joker state information for a specific joker
    fn get_joker_state_info(&self, joker_id: JokerId) -> Option<pyo3::PyObject> {
        // Check if this joker is currently active in the game
        let state = &self.game;
        let active_jokers = &state.jokers;

        // Find the joker in active jokers
        let joker_found = active_jokers.iter().any(|j| j.id() == joker_id);

        if joker_found {
            // Get actual state from the joker state manager
            if let Some(joker_state) = state.joker_state_manager.get_state(joker_id) {
                pyo3::Python::with_gil(|py| {
                    let dict = pyo3::types::PyDict::new(py);
                    let _ = dict.set_item("accumulated_value", joker_state.accumulated_value);
                    let _ = dict.set_item("triggers_remaining", joker_state.triggers_remaining);

                    // Create custom data dictionary
                    let custom_data_dict = pyo3::types::PyDict::new(py);
                    for (key, value) in &joker_state.custom_data {
                        // Convert serde_json::Value to Python object
                        let py_value = match value {
                            serde_json::Value::Null => py.None(),
                            serde_json::Value::Bool(b) => b.to_object(py),
                            serde_json::Value::Number(n) => {
                                if let Some(i) = n.as_i64() {
                                    i.to_object(py)
                                } else if let Some(f) = n.as_f64() {
                                    f.to_object(py)
                                } else {
                                    py.None()
                                }
                            }
                            serde_json::Value::String(s) => s.to_object(py),
                            serde_json::Value::Array(_) | serde_json::Value::Object(_) => {
                                // For complex types, serialize to string for now
                                serde_json::to_string(&value)
                                    .unwrap_or_default()
                                    .to_object(py)
                            }
                        };
                        let _ = custom_data_dict.set_item(key, py_value);
                    }
                    let _ = dict.set_item("custom_data", custom_data_dict);

                    let _ = dict.set_item("has_custom_data", !joker_state.custom_data.is_empty());

                    // Create list of custom data keys
                    let keys_vec: Vec<String> = joker_state.custom_data.keys().cloned().collect();
                    let _ = dict.set_item("custom_data_keys", keys_vec);

                    Some(dict.into())
                })
            } else {
                // Joker is active but has no state yet - return default state
                pyo3::Python::with_gil(|py| {
                    let dict = pyo3::types::PyDict::new(py);
                    let _ = dict.set_item("accumulated_value", 0.0f64);
                    let _ = dict.set_item("triggers_remaining", py.None());
                    let _ = dict.set_item("custom_data", pyo3::types::PyDict::new(py));
                    let _ = dict.set_item("has_custom_data", false);
                    let _ = dict.set_item("custom_data_keys", Vec::<String>::new());

                    Some(dict.into())
                })
            }
        } else {
            None
        }
    }

    /// Get state for all active jokers
    fn get_active_jokers_state(&self) -> pyo3::PyObject {
        pyo3::Python::with_gil(|py| {
            let dict = pyo3::types::PyDict::new(py);

            for joker in &self.game.jokers {
                let joker_id = joker.id();
                let state_dict = pyo3::types::PyDict::new(py);

                // Get actual state from the joker state manager
                if let Some(joker_state) = self.game.joker_state_manager.get_state(joker_id) {
                    let _ = state_dict.set_item("accumulated_value", joker_state.accumulated_value);
                    let _ =
                        state_dict.set_item("triggers_remaining", joker_state.triggers_remaining);

                    // Add custom data
                    let custom_data_dict = pyo3::types::PyDict::new(py);
                    for (key, value) in &joker_state.custom_data {
                        let py_value = match value {
                            serde_json::Value::Null => py.None(),
                            serde_json::Value::Bool(b) => b.to_object(py),
                            serde_json::Value::Number(n) => {
                                if let Some(i) = n.as_i64() {
                                    i.to_object(py)
                                } else if let Some(f) = n.as_f64() {
                                    f.to_object(py)
                                } else {
                                    py.None()
                                }
                            }
                            serde_json::Value::String(s) => s.to_object(py),
                            serde_json::Value::Array(_) | serde_json::Value::Object(_) => {
                                serde_json::to_string(&value)
                                    .unwrap_or_default()
                                    .to_object(py)
                            }
                        };
                        let _ = custom_data_dict.set_item(key, py_value);
                    }
                    let _ = state_dict.set_item("custom_data", custom_data_dict);
                } else {
                    // No state exists yet - return default values
                    let _ = state_dict.set_item("accumulated_value", 0.0f64);
                    let _ = state_dict.set_item("triggers_remaining", py.None());
                    let _ = state_dict.set_item("custom_data", pyo3::types::PyDict::new(py));
                }

                let _ = dict.set_item(format!("{joker_id:?}"), state_dict);
            }

            dict.into()
        })
    }

    /// Search jokers by name or description
    fn search_jokers(&self, query: &str) -> PyResult<Vec<JokerMetadata>> {
        // Input validation for security
        if query.len() > MAX_SEARCH_QUERY_LENGTH {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Search query too long (max {MAX_SEARCH_QUERY_LENGTH} characters)"
            )));
        }

        let query_lower = query.to_lowercase();

        if let Ok(all_jokers) = registry::all_definitions() {
            Ok(all_jokers
                .into_iter()
                .filter(|def| {
                    def.name.to_lowercase().contains(&query_lower)
                        || def.description.to_lowercase().contains(&query_lower)
                })
                .filter_map(|def| self.get_joker_metadata(def.id).ok().flatten())
                .collect())
        } else {
            Ok(Vec::new())
        }
    }

    /// Filter jokers by multiple criteria
    #[pyo3(signature = (rarity=None, unlocked_only=false, affordable_only=false))]
    fn filter_jokers(
        &self,
        rarity: Option<JokerRarity>,
        unlocked_only: bool,
        affordable_only: bool,
    ) -> Vec<JokerMetadata> {
        if let Ok(all_jokers) = registry::all_definitions() {
            all_jokers
                .into_iter()
                .filter(|def| {
                    // Filter by rarity if specified
                    if let Some(r) = rarity {
                        if def.rarity != r {
                            return false;
                        }
                    }

                    // Filter by unlock status if requested
                    if unlocked_only {
                        let is_unlocked = def
                            .unlock_condition
                            .as_ref()
                            .map(|condition| evaluate_unlock_condition(condition, &self.game))
                            .unwrap_or(true); // No unlock condition means always unlocked
                        if !is_unlocked {
                            return false;
                        }
                    }

                    // Filter by affordability if requested
                    if affordable_only {
                        let cost = calculate_joker_cost(def.rarity);
                        if self.game.money < cost as usize {
                            return false;
                        }
                    }

                    true
                })
                .filter_map(|def| self.get_joker_metadata(def.id).ok().flatten())
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Get jokers within a cost range
    fn get_jokers_by_cost_range(&self, min_cost: i32, max_cost: i32) -> Vec<JokerMetadata> {
        if let Ok(all_jokers) = registry::all_definitions() {
            all_jokers
                .into_iter()
                .filter_map(|def| {
                    let cost = calculate_joker_cost(def.rarity);

                    if cost >= min_cost && cost <= max_cost {
                        self.get_joker_metadata(def.id).ok().flatten()
                    } else {
                        None
                    }
                })
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Get joker categories
    fn get_joker_categories(&self) -> pyo3::PyObject {
        pyo3::Python::with_gil(|py| {
            let dict = pyo3::types::PyDict::new(py);

            // Basic categorization based on joker types
            let _ = dict.set_item("Basic Scoring", vec![JokerId::Joker]);
            let _ = dict.set_item(
                "Suit-based",
                vec![
                    JokerId::GreedyJoker,
                    JokerId::LustyJoker,
                    JokerId::WrathfulJoker,
                    JokerId::GluttonousJoker,
                ],
            );

            dict.into()
        })
    }

    /// Get joker registry statistics
    fn get_joker_statistics(&self) -> pyo3::PyObject {
        pyo3::Python::with_gil(|py| {
            let dict = pyo3::types::PyDict::new(py);

            if let Ok(all_jokers) = registry::all_definitions() {
                let total = all_jokers.len();
                let _ = dict.set_item("total_jokers", total);

                // Count by rarity
                let rarity_dict = pyo3::types::PyDict::new(py);
                let mut common_count = 0;
                let mut uncommon_count = 0;
                let mut rare_count = 0;
                let mut legendary_count = 0;

                for joker in &all_jokers {
                    match joker.rarity {
                        JokerRarity::Common => common_count += 1,
                        JokerRarity::Uncommon => uncommon_count += 1,
                        JokerRarity::Rare => rare_count += 1,
                        JokerRarity::Legendary => legendary_count += 1,
                    }
                }

                let _ = rarity_dict.set_item("Common", common_count);
                let _ = rarity_dict.set_item("Uncommon", uncommon_count);
                let _ = rarity_dict.set_item("Rare", rare_count);
                let _ = rarity_dict.set_item("Legendary", legendary_count);
                let _ = dict.set_item("by_rarity", rarity_dict);

                // Count actually unlocked jokers
                let unlocked_count = all_jokers
                    .iter()
                    .filter(|def| {
                        def.unlock_condition
                            .as_ref()
                            .map(|condition| evaluate_unlock_condition(condition, &self.game))
                            .unwrap_or(true) // No unlock condition means always unlocked
                    })
                    .count();
                let _ = dict.set_item("unlocked_count", unlocked_count);
            } else {
                let _ = dict.set_item("total_jokers", 0);
                let _ = dict.set_item("by_rarity", pyo3::types::PyDict::new(py));
                let _ = dict.set_item("unlocked_count", 0);
            }

            dict.into()
        })
    }

    /// Analyze joker synergies
    fn analyze_joker_synergies(&self, _joker_ids: Vec<JokerId>) -> pyo3::PyObject {
        pyo3::Python::with_gil(|py| {
            let dict = pyo3::types::PyDict::new(py);

            // Basic synergy analysis - this would be expanded with actual game logic
            let _ = dict.set_item("synergy_score", 0.0f64);
            let _ = dict.set_item("compatible", true);
            let _ = dict.set_item("conflicts", pyo3::types::PyList::empty(py));
            let _ = dict.set_item("recommendations", pyo3::types::PyList::empty(py));

            dict.into()
        })
    }

    /// Get custom data for a specific joker
    fn get_joker_custom_data(&self, joker_id: JokerId, key: &str) -> Option<pyo3::PyObject> {
        if let Some(joker_state) = self.game.joker_state_manager.get_state(joker_id) {
            if let Some(value) = joker_state.custom_data.get(key) {
                pyo3::Python::with_gil(|py| {
                    let py_value = match value {
                        serde_json::Value::Null => py.None(),
                        serde_json::Value::Bool(b) => b.to_object(py),
                        serde_json::Value::Number(n) => {
                            if let Some(i) = n.as_i64() {
                                i.to_object(py)
                            } else if let Some(f) = n.as_f64() {
                                f.to_object(py)
                            } else {
                                py.None()
                            }
                        }
                        serde_json::Value::String(s) => s.to_object(py),
                        serde_json::Value::Array(_) | serde_json::Value::Object(_) => {
                            serde_json::to_string(&value)
                                .unwrap_or_default()
                                .to_object(py)
                        }
                    };
                    Some(py_value)
                })
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Check if a joker has custom data for a specific key
    fn joker_has_custom_data(&self, joker_id: JokerId, key: &str) -> bool {
        if let Some(joker_state) = self.game.joker_state_manager.get_state(joker_id) {
            joker_state.custom_data.contains_key(key)
        } else {
            false
        }
    }

    /// Get all custom data keys for a specific joker
    fn get_joker_custom_data_keys(&self, joker_id: JokerId) -> Vec<String> {
        if let Some(joker_state) = self.game.joker_state_manager.get_state(joker_id) {
            joker_state.custom_data.keys().cloned().collect()
        } else {
            Vec::new()
        }
    }

    /// Set custom data for a specific joker (string values only for simplicity)
    fn set_joker_custom_data(
        &mut self,
        joker_id: JokerId,
        key: &str,
        value: String,
    ) -> PyResult<bool> {
        // Input validation for security
        if key.len() > MAX_CUSTOM_DATA_KEY_LENGTH {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Key too long (max {MAX_CUSTOM_DATA_KEY_LENGTH} characters)"
            )));
        }

        if value.len() > MAX_CUSTOM_DATA_VALUE_LENGTH {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Value too long (max {MAX_CUSTOM_DATA_VALUE_LENGTH} characters)"
            )));
        }

        if key.is_empty() {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Key cannot be empty",
            ));
        }

        // Check if joker is active
        let joker_exists = self.game.jokers.iter().any(|j| j.id() == joker_id);

        if joker_exists {
            // Ensure the joker has state first
            self.game.joker_state_manager.ensure_state_exists(joker_id);

            // Set the custom data
            Ok(matches!(
                self.game
                    .joker_state_manager
                    .set_custom_data(joker_id, key, value),
                Ok(())
            ))
        } else {
            Ok(false)
        }
    }

    /// Get accumulated value for a specific joker
    fn get_joker_accumulated_value(&self, joker_id: JokerId) -> Option<f64> {
        self.game
            .joker_state_manager
            .get_accumulated_value(joker_id)
    }

    /// Add to a joker's accumulated value
    fn add_joker_accumulated_value(&mut self, joker_id: JokerId, value: f64) -> bool {
        // Check if joker is active
        let joker_exists = self.game.jokers.iter().any(|j| j.id() == joker_id);

        if joker_exists {
            self.game
                .joker_state_manager
                .add_accumulated_value(joker_id, value);
            true
        } else {
            false
        }
    }

    /// Check if a joker has triggers available
    fn joker_has_triggers(&self, joker_id: JokerId) -> bool {
        self.game.joker_state_manager.has_triggers(joker_id)
    }

    /// Use a trigger for a joker (returns true if trigger was used successfully)
    fn use_joker_trigger(&mut self, joker_id: JokerId) -> bool {
        // Check if joker is active
        let joker_exists = self.game.jokers.iter().any(|j| j.id() == joker_id);

        if joker_exists {
            self.game.joker_state_manager.use_trigger(joker_id)
        } else {
            false
        }
    }

    #[getter]
    fn state(&self) -> GameState {
        GameState {
            snapshot: GameStateSnapshot::from_game(&self.game),
        }
    }
    #[getter]
    fn is_over(&self) -> bool {
        self.game.is_over()
    }
    #[getter]
    fn is_win(&self) -> bool {
        if let Some(end) = self.game.result() {
            if end == End::Win {
                return true;
            }
        }
        false
    }

    /// Get all active joker states
    fn get_joker_states(&self) -> pyo3::PyObject {
        pyo3::Python::with_gil(|py| {
            let dict = pyo3::types::PyDict::new(py);

            for joker in &self.game.jokers {
                let joker_id = joker.id();
                let state_dict = pyo3::types::PyDict::new(py);

                // Get actual state from the joker state manager
                if let Some(joker_state) = self.game.joker_state_manager.get_state(joker_id) {
                    let _ = state_dict.set_item("accumulated_value", joker_state.accumulated_value);
                    let _ =
                        state_dict.set_item("triggers_remaining", joker_state.triggers_remaining);

                    // Add custom data
                    let custom_data_dict = pyo3::types::PyDict::new(py);
                    for (key, value) in &joker_state.custom_data {
                        let py_value = match value {
                            serde_json::Value::Null => py.None(),
                            serde_json::Value::Bool(b) => b.to_object(py),
                            serde_json::Value::Number(n) => {
                                if let Some(i) = n.as_i64() {
                                    i.to_object(py)
                                } else if let Some(f) = n.as_f64() {
                                    f.to_object(py)
                                } else {
                                    py.None()
                                }
                            }
                            serde_json::Value::String(s) => s.to_object(py),
                            serde_json::Value::Array(_) | serde_json::Value::Object(_) => {
                                serde_json::to_string(&value)
                                    .unwrap_or_default()
                                    .to_object(py)
                            }
                        };
                        let _ = custom_data_dict.set_item(key, py_value);
                    }
                    let _ = state_dict.set_item("custom_data", custom_data_dict);
                } else {
                    // No state exists yet - return default values
                    let _ = state_dict.set_item("accumulated_value", 0.0f64);
                    let _ = state_dict.set_item("triggers_remaining", py.None());
                    let _ = state_dict.set_item("custom_data", pyo3::types::PyDict::new(py));
                }

                let _ = dict.set_item(format!("{joker_id:?}"), state_dict);
            }

            dict.into()
        })
    }

    /// Get accumulated values for all jokers
    fn get_joker_accumulated_values(&self) -> pyo3::PyObject {
        pyo3::Python::with_gil(|py| {
            let dict = pyo3::types::PyDict::new(py);

            for joker in &self.game.jokers {
                let joker_id = joker.id();

                // Get actual accumulated value from the joker state manager
                let accumulated_value =
                    if let Some(joker_state) = self.game.joker_state_manager.get_state(joker_id) {
                        joker_state.accumulated_value
                    } else {
                        0.0f64 // Default value if no state exists
                    };

                let _ = dict.set_item(format!("{joker_id:?}"), accumulated_value);
            }

            dict.into()
        })
    }

    /// Get triggers remaining for all jokers
    fn get_joker_triggers_remaining(&self) -> pyo3::PyObject {
        pyo3::Python::with_gil(|py| {
            let dict = pyo3::types::PyDict::new(py);

            for joker in &self.game.jokers {
                let joker_id = joker.id();

                // Get actual triggers remaining from the joker state manager
                let triggers_remaining =
                    if let Some(joker_state) = self.game.joker_state_manager.get_state(joker_id) {
                        joker_state.triggers_remaining
                    } else {
                        None // Default value if no state exists
                    };

                let _ = dict.set_item(format!("{joker_id:?}"), triggers_remaining);
            }

            dict.into()
        })
    }
}

#[pyclass]
struct GameState {
    snapshot: GameStateSnapshot,
}

#[pymethods]
impl GameState {
    #[getter]
    fn stage(&self) -> Stage {
        self.snapshot.stage
    }
    #[getter]
    fn round(&self) -> f64 {
        self.snapshot.round
    }
    #[getter]
    fn action_history(&self) -> Vec<Action> {
        self.snapshot.action_history.clone()
    }
    #[getter]
    fn deck(&self) -> Vec<Card> {
        self.snapshot.deck_cards.clone()
    }
    #[getter]
    fn selected(&self) -> Vec<Card> {
        self.snapshot.selected_cards.clone()
    }
    #[getter]
    fn available(&self) -> Vec<Card> {
        self.snapshot.available_cards.clone()
    }
    #[getter]
    fn discarded(&self) -> Vec<Card> {
        self.snapshot.discarded_cards.clone()
    }
    #[getter]
    fn plays(&self) -> f64 {
        self.snapshot.plays
    }
    #[getter]
    fn discards(&self) -> f64 {
        self.snapshot.discards
    }

    #[getter]
    fn score(&self) -> f64 {
        self.snapshot.score
    }
    #[getter]
    fn required_score(&self) -> f64 {
        self.snapshot.required_score
    }
    #[getter]
    fn jokers(&self) -> PyResult<Vec<Jokers>> {
        // Emit deprecation warning
        Python::with_gil(|py| {
            let warnings = py.import("warnings")?;
            warnings.call_method1(
                "warn",
                (
                    "GameState.jokers is deprecated. Use GameState.joker_ids with GameEngine.get_joker_info() instead. \
                     The jokers property will be removed in a future version. \
                     See migration guide for details.",
                    py.get_type::<pyo3::exceptions::PyDeprecationWarning>(),
                    2  // stacklevel - show warning at caller's location
                ),
            )?;
            Ok::<(), PyErr>(())
        })?;

        // TODO: Convert new joker system to old Jokers enum for Python compatibility
        // For now, return empty vector during migration
        Ok(Vec::new())
    }

    /// Get joker IDs using the new JokerId system
    #[getter]
    fn joker_ids(&self) -> Vec<JokerId> {
        self.snapshot.joker_ids.clone()
    }

    /// Get number of joker slots currently in use
    #[getter]
    fn joker_slots_used(&self) -> usize {
        self.snapshot.joker_count
    }

    /// Get total number of joker slots available
    #[getter]
    fn joker_slots_total(&self) -> usize {
        self.snapshot.joker_slots_max
    }

    /// Get joker names for easy migration from old API
    ///
    /// This is a convenience method to help users migrate from:
    /// `[j.name() for j in state.jokers]`
    /// to:
    /// `state.get_joker_names()`
    fn get_joker_names(&self) -> Vec<String> {
        // TODO: Convert JokerIds to names once conversion is implemented
        // For now, return empty vector during migration
        Vec::new()
    }

    /// Get joker descriptions for easy migration from old API
    ///
    /// This is a convenience method to help users migrate from:
    /// `[j.desc() for j in state.jokers]`
    /// to:
    /// `state.get_joker_descriptions()`
    fn get_joker_descriptions(&self) -> Vec<String> {
        // TODO: Convert JokerIds to descriptions once conversion is implemented
        // For now, return empty vector during migration
        Vec::new()
    }

    #[getter]
    fn money(&self) -> f64 {
        self.snapshot.money
    }
    #[getter]
    fn ante(&self) -> usize {
        match self.snapshot.ante {
            Ante::Zero => 0,
            Ante::One => 1,
            Ante::Two => 2,
            Ante::Three => 3,
            Ante::Four => 4,
            Ante::Five => 5,
            Ante::Six => 6,
            Ante::Seven => 7,
            Ante::Eight => 8,
        }
    }

    // === BACKWARDS COMPATIBILITY METHODS ===
    // These methods are deprecated and provided for backwards compatibility only.
    // New code should use GameEngine for actions and GameState only for reading state.

    /// DEPRECATED: Use GameEngine.gen_actions() instead
    /// This method is provided for backwards compatibility only.
    fn gen_actions(&self) -> PyResult<Vec<Action>> {
        // Issue deprecation warning
        Python::with_gil(|py| -> PyResult<()> {
            let warnings = py.import("warnings")?;
            warnings.call_method1(
                "warn",
                ("GameState.gen_actions() is deprecated. Use GameEngine.gen_actions() instead. GameState should only be used for reading game state, not performing actions.",),
            )?;
            warnings.call_method1(
                "warn",
                ("This method will be removed in version 2.0. Migration guide: https://github.com/spencerduncan/balatro-rs/wiki/Python-API-Migration",),
            )?;
            Ok(())
        })?;

        // This method cannot work without access to the actual Game instance
        Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
            "GameState.gen_actions() is deprecated and no longer functional. Use GameEngine.gen_actions() instead. GameState is now a read-only snapshot of the game state."
        ))
    }

    /// DEPRECATED: Use GameEngine.gen_action_space() instead
    /// This method is provided for backwards compatibility only.
    fn gen_action_space(&self) -> PyResult<Vec<usize>> {
        // Issue deprecation warning
        Python::with_gil(|py| -> PyResult<()> {
            let warnings = py.import("warnings")?;
            warnings.call_method1(
                "warn",
                ("GameState.gen_action_space() is deprecated. Use GameEngine.gen_action_space() instead. GameState should only be used for reading game state, not performing actions.",),
            )?;
            Ok(())
        })?;

        // This method cannot work without access to the actual Game instance
        Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
            "GameState.gen_action_space() is deprecated and no longer functional. Use GameEngine.gen_action_space() instead. GameState is now a read-only snapshot of the game state."
        ))
    }

    /// DEPRECATED: Use GameEngine.get_action_name() instead
    /// This method is provided for backwards compatibility only.
    fn get_action_name(&self, _index: usize) -> PyResult<String> {
        // Issue deprecation warning
        Python::with_gil(|py| -> PyResult<()> {
            let warnings = py.import("warnings")?;
            warnings.call_method1(
                "warn",
                ("GameState.get_action_name() is deprecated. Use GameEngine.get_action_name() instead. GameState should only be used for reading game state, not performing actions.",),
            )?;
            Ok(())
        })?;

        // This method cannot work without access to the actual Game instance
        Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
            "GameState.get_action_name() is deprecated and no longer functional. Use GameEngine.get_action_name() instead. GameState is now a read-only snapshot of the game state."
        ))
    }

    /// DEPRECATED: This method cannot work on read-only GameState
    /// Use GameEngine.handle_action() instead on a mutable GameEngine instance.
    fn handle_action(&self, _action: Action) -> PyResult<()> {
        // Issue deprecation warning and error
        Python::with_gil(|py| -> PyResult<()> {
            let warnings = py.import("warnings")?;
            warnings.call_method1(
                "warn",
                ("GameState.handle_action() is deprecated and cannot modify game state. Use GameEngine.handle_action() instead on a mutable GameEngine instance.",),
            )?;
            Ok(())
        })?;

        Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
            "GameState.handle_action() is deprecated and non-functional. GameState is read-only. Use GameEngine.handle_action() instead on a mutable GameEngine instance."
        ))
    }

    /// DEPRECATED: This method cannot work on read-only GameState
    /// Use GameEngine.handle_action_index() instead on a mutable GameEngine instance.
    fn handle_action_index(&self, _index: usize) -> PyResult<()> {
        // Issue deprecation warning and error
        Python::with_gil(|py| -> PyResult<()> {
            let warnings = py.import("warnings")?;
            warnings.call_method1(
                "warn",
                ("GameState.handle_action_index() is deprecated and cannot modify game state. Use GameEngine.handle_action_index() instead on a mutable GameEngine instance.",),
            )?;
            Ok(())
        })?;

        Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
            "GameState.handle_action_index() is deprecated and non-functional. GameState is read-only. Use GameEngine.handle_action_index() instead on a mutable GameEngine instance."
        ))
    }

    /// DEPRECATED: Use GameEngine.is_over instead
    /// This property is provided for backwards compatibility only.
    #[getter]
    fn is_over(&self) -> PyResult<bool> {
        // Issue deprecation warning
        Python::with_gil(|py| -> PyResult<()> {
            let warnings = py.import("warnings")?;
            warnings.call_method1(
                "warn",
                ("GameState.is_over is deprecated. Use GameEngine.is_over instead.",),
            )?;
            Ok(())
        })?;

        Ok(self.snapshot.is_over)
    }

    fn __repr__(&self) -> String {
        format!(
            "GameState: Stage={:?}, Round={}, Score={}/{}",
            self.snapshot.stage,
            self.snapshot.round,
            self.snapshot.score,
            self.snapshot.required_score
        )
    }
}

/// Evaluate if a joker unlock condition is met based on current game state
fn evaluate_unlock_condition(condition: &UnlockCondition, game: &Game) -> bool {
    match condition {
        UnlockCondition::ReachAnte(required_ante) => {
            // Convert current ante to u32 for comparison
            let current_ante_num = match game.ante_current {
                Ante::Zero => 0,
                Ante::One => 1,
                Ante::Two => 2,
                Ante::Three => 3,
                Ante::Four => 4,
                Ante::Five => 5,
                Ante::Six => 6,
                Ante::Seven => 7,
                Ante::Eight => 8,
            };
            current_ante_num >= *required_ante
        }
        UnlockCondition::HaveMoney(required_money) => game.money >= *required_money as usize,
        UnlockCondition::ScoreInHand(required_score) => game.score >= *required_score as usize,
        UnlockCondition::PlayHands(required_hands) => {
            // Sum all hand type counts to get total hands played
            let total_hands: u32 = game.hand_type_counts.values().sum();
            total_hands >= *required_hands
        }
        UnlockCondition::WinWithDeck(_deck_name) => {
            // This would require additional game state tracking for deck wins
            // For now, return false as this feature isn't implemented
            false
        }
        UnlockCondition::Custom(_description) => {
            // Custom conditions would need specific implementation
            // For now, return false as this requires case-by-case handling
            false
        }
    }
}

// calculate_joker_cost function moved to core/src/joker_registry.rs to avoid duplication

/// Extract joker parameters based on joker implementation type
fn extract_joker_parameters(joker_id: JokerId, py: pyo3::Python) -> pyo3::PyObject {
    let dict = pyo3::types::PyDict::new(py);

    match joker_id {
        JokerId::Joker => {
            // Basic joker gives +4 mult
            let _ = dict.set_item("mult_bonus", 4);
        }
        JokerId::GreedyJoker => {
            // +3 mult for each diamond card scored
            let _ = dict.set_item("mult_per_diamond", 3);
            let _ = dict.set_item("target_suit", "Diamond");
        }
        JokerId::LustyJoker => {
            // +3 mult for each heart card scored
            let _ = dict.set_item("mult_per_heart", 3);
            let _ = dict.set_item("target_suit", "Heart");
        }
        JokerId::WrathfulJoker => {
            // +3 mult for each spade card scored
            let _ = dict.set_item("mult_per_spade", 3);
            let _ = dict.set_item("target_suit", "Spade");
        }
        JokerId::GluttonousJoker => {
            // +3 mult for each club card scored
            let _ = dict.set_item("mult_per_club", 3);
            let _ = dict.set_item("target_suit", "Club");
        }
        JokerId::Runner => {
            // +15 chips accumulated when straight is played
            let _ = dict.set_item("chips_per_straight", 15);
            let _ = dict.set_item("scaling_type", "accumulated");
        }
        _ => {
            // Default empty parameters for unspecified jokers
        }
    }

    dict.into()
}

#[pymodule]
fn pylatro(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Config>()?;
    m.add_class::<GameEngine>()?;
    m.add_class::<GameState>()?;
    m.add_class::<Stage>()?;

    // Add new JokerId-based types
    m.add_class::<JokerId>()?;
    m.add_class::<JokerRarity>()?;
    m.add_class::<JokerDefinition>()?;
    m.add_class::<JokerMetadata>()?;
    m.add_class::<UnlockCondition>()?;
    m.add_class::<JokerMetadata>()?;

    Ok(())
}
