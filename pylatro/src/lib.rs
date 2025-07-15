#![allow(deprecated)] // Allow deprecated to_object calls temporarily - will be fixed in PyO3 migration

use balatro_rs::action::Action;
use balatro_rs::card::Card;
use balatro_rs::config::Config;
use balatro_rs::error::GameError;
use balatro_rs::game::Game;
use balatro_rs::joker::{JokerId, JokerRarity, Jokers};
use balatro_rs::joker_metadata::JokerMetadata;
use balatro_rs::joker_registry::{registry, JokerDefinition, UnlockCondition};
use balatro_rs::stage::{End, Stage};
use pyo3::prelude::*;

#[pyclass]
struct GameEngine {
    game: Game,
}

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
            // Get base cost based on rarity
            let cost = match definition.rarity {
                JokerRarity::Common => 3,
                JokerRarity::Uncommon => 6,
                JokerRarity::Rare => 8,
                JokerRarity::Legendary => 20,
            };
            return self.game.money >= cost;
        }

        false
    }

    /// Get the cost of a specific joker
    fn get_joker_cost(&self, joker_id: JokerId) -> Result<usize, GameError> {
        let definition = registry::get_definition(&joker_id)?
            .ok_or_else(|| GameError::JokerNotFound(format!("{joker_id:?}")))?;

        let cost = match definition.rarity {
            JokerRarity::Common => 3,
            JokerRarity::Uncommon => 6,
            JokerRarity::Rare => 8,
            JokerRarity::Legendary => 20,
        };

        Ok(cost)
    }

    /// Get comprehensive metadata for a specific joker
    fn get_joker_metadata(&self, joker_id: JokerId) -> Result<Option<JokerMetadata>, GameError> {
        let definition = registry::get_definition(&joker_id)?;

        match definition {
            Some(def) => {
                // For now, assume all jokers are unlocked
                let is_unlocked = true;
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

        let cost = match definition.rarity {
            JokerRarity::Common => 3,
            JokerRarity::Uncommon => 6,
            JokerRarity::Rare => 8,
            JokerRarity::Legendary => 20,
        };

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
            let _ = dict.set_item("parameters", pyo3::types::PyDict::new(py)); // TODO: Extract from joker implementation
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
            let _ = dict.set_item("is_unlocked", true); // For now, assume all jokers are unlocked

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

            for joker_id in joker_ids {
                if let Ok(Some(metadata)) = self.get_joker_metadata(joker_id) {
                    let _ = dict.set_item(format!("{joker_id:?}"), metadata);
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

            // Get all available jokers and their metadata
            if let Ok(all_jokers) = registry::all_definitions() {
                for definition in all_jokers {
                    if let Ok(Some(metadata)) = self.get_joker_metadata(definition.id) {
                        let _ = dict.set_item(format!("{0:?}", definition.id), metadata);
                    }
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
                        // Create minimal metadata
                        Some(JokerMetadata::from_definition(&def, true))
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
        let joker_found = active_jokers.iter().any(|j| j.to_joker_id() == joker_id);

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
                                serde_json::to_string(value)
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
                let joker_id = joker.to_joker_id();
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
                                serde_json::to_string(value)
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
    fn search_jokers(&self, query: &str) -> Vec<JokerMetadata> {
        let query_lower = query.to_lowercase();

        if let Ok(all_jokers) = registry::all_definitions() {
            all_jokers
                .into_iter()
                .filter(|def| {
                    def.name.to_lowercase().contains(&query_lower)
                        || def.description.to_lowercase().contains(&query_lower)
                })
                .filter_map(|def| self.get_joker_metadata(def.id).ok().flatten())
                .collect()
        } else {
            Vec::new()
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
                        // For now, assume all are unlocked
                    }

                    // Filter by affordability if requested
                    if affordable_only {
                        let cost = match def.rarity {
                            JokerRarity::Common => 3,
                            JokerRarity::Uncommon => 6,
                            JokerRarity::Rare => 8,
                            JokerRarity::Legendary => 20,
                        };
                        if self.game.money < cost {
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
                    let cost = match def.rarity {
                        JokerRarity::Common => 3,
                        JokerRarity::Uncommon => 6,
                        JokerRarity::Rare => 8,
                        JokerRarity::Legendary => 20,
                    };

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

                // For now, assume all are unlocked
                let _ = dict.set_item("unlocked_count", total);
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
                            serde_json::to_string(value)
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
    fn set_joker_custom_data(&mut self, joker_id: JokerId, key: &str, value: String) -> bool {
        // Check if joker is active
        let joker_exists = self.game.jokers.iter().any(|j| j.to_joker_id() == joker_id);

        if joker_exists {
            // Ensure the joker has state first
            self.game.joker_state_manager.ensure_state_exists(joker_id);

            // Set the custom data
            matches!(
                self.game
                    .joker_state_manager
                    .set_custom_data(joker_id, key, value),
                Ok(())
            )
        } else {
            false
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
        let joker_exists = self.game.jokers.iter().any(|j| j.to_joker_id() == joker_id);

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
        let joker_exists = self.game.jokers.iter().any(|j| j.to_joker_id() == joker_id);

        if joker_exists {
            self.game.joker_state_manager.use_trigger(joker_id)
        } else {
            false
        }
    }

    #[getter]
    fn state(&self) -> GameState {
        GameState {
            game: self.game.clone(),
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
}

#[pyclass]
struct GameState {
    game: Game,
}

#[pymethods]
impl GameState {
    #[getter]
    fn stage(&self) -> Stage {
        self.game.stage
    }
    #[getter]
    fn round(&self) -> usize {
        self.game.round
    }
    #[getter]
    fn action_history(&self) -> Vec<Action> {
        self.game.action_history.clone()
    }
    #[getter]
    fn deck(&self) -> Vec<Card> {
        self.game.deck.cards()
    }
    #[getter]
    fn selected(&self) -> Vec<Card> {
        self.game.available.selected()
    }
    #[getter]
    fn available(&self) -> Vec<Card> {
        self.game.available.cards()
    }
    #[getter]
    fn discarded(&self) -> Vec<Card> {
        self.game.discarded.clone()
    }
    #[getter]
    fn plays(&self) -> usize {
        self.game.plays
    }
    #[getter]
    fn discards(&self) -> usize {
        self.game.discards
    }

    #[getter]
    fn score(&self) -> usize {
        self.game.score
    }
    #[getter]
    fn required_score(&self) -> usize {
        self.game.required_score()
    }
    #[getter]
    fn jokers(&self) -> Vec<Jokers> {
        self.game.jokers.clone()
    }

    /// Get joker IDs using the new JokerId system
    #[getter]
    fn joker_ids(&self) -> Vec<JokerId> {
        self.game.jokers.iter().map(|j| j.to_joker_id()).collect()
    }

    /// Get number of joker slots currently in use
    #[getter]
    fn joker_slots_used(&self) -> usize {
        self.game.joker_count()
    }

    /// Get total number of joker slots available
    #[getter]
    fn joker_slots_total(&self) -> usize {
        self.game.config.joker_slots_max
    }

    #[getter]
    fn money(&self) -> usize {
        self.game.money
    }
    #[getter]
    fn ante(&self) -> usize {
        match self.game.ante_current {
            balatro_rs::ante::Ante::Zero => 0,
            balatro_rs::ante::Ante::One => 1,
            balatro_rs::ante::Ante::Two => 2,
            balatro_rs::ante::Ante::Three => 3,
            balatro_rs::ante::Ante::Four => 4,
            balatro_rs::ante::Ante::Five => 5,
            balatro_rs::ante::Ante::Six => 6,
            balatro_rs::ante::Ante::Seven => 7,
            balatro_rs::ante::Ante::Eight => 8,
        }
    }

    /// Get all active joker states
    fn get_joker_states(&self) -> pyo3::PyObject {
        pyo3::Python::with_gil(|py| {
            let dict = pyo3::types::PyDict::new(py);

            for joker in &self.game.jokers {
                let joker_id = joker.to_joker_id();
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
                                serde_json::to_string(value)
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
                let joker_id = joker.to_joker_id();

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
                let joker_id = joker.to_joker_id();

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

    fn __repr__(&self) -> String {
        format!("GameState:\n{}", self.game)
    }
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

    Ok(())
}
