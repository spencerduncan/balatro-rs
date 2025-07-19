use balatro_rs::action::Action;
use balatro_rs::ante::Ante;
use balatro_rs::card::Card;
use balatro_rs::config::Config;
use balatro_rs::error::GameError;
use balatro_rs::game::Game;
use balatro_rs::joker::{JokerId, JokerRarity, Jokers};
use balatro_rs::joker_metadata::JokerMetadata;
use balatro_rs::joker_registry::{registry, JokerDefinition, UnlockCondition};
use balatro_rs::stage::{End, Stage};
use pyo3::prelude::*;
use pyo3::{PyObject, PyResult, Python};

/// A serializable snapshot of the game state for Python bindings
#[derive(Clone)]
struct GameStateSnapshot {
    stage: Stage,
    round: usize,
    action_history: Vec<Action>,
    deck_cards: Vec<Card>,
    selected_cards: Vec<Card>,
    available_cards: Vec<Card>,
    discarded_cards: Vec<Card>,
    plays: usize,
    discards: usize,
    score: usize,
    required_score: usize,
    joker_ids: Vec<JokerId>,
    joker_count: usize,
    joker_slots_max: usize,
    money: usize,
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

impl GameEngine {
    /// Helper method to calculate joker cost based on rarity
    fn calculate_joker_cost(rarity: JokerRarity) -> usize {
        match rarity {
            JokerRarity::Common => 3,
            JokerRarity::Uncommon => 6,
            JokerRarity::Rare => 8,
            JokerRarity::Legendary => 20,
        }
    }
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
            let cost = Self::calculate_joker_cost(definition.rarity);
            return self.game.money >= cost;
        }

        false
    }

    /// Get the cost of a specific joker
    fn get_joker_cost(&self, joker_id: JokerId) -> Result<Option<usize>, GameError> {
        if let Some(definition) = registry::get_definition(&joker_id)? {
            Ok(Some(Self::calculate_joker_cost(definition.rarity)))
        } else {
            Ok(None)
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

    // Basic Metadata Access Methods

    /// Get comprehensive metadata for a specific joker
    fn get_joker_metadata(&self, joker_id: JokerId) -> PyResult<Option<JokerMetadata>> {
        match registry::get_definition(&joker_id) {
            Ok(Some(definition)) => {
                // Check if joker is unlocked (simplified for now)
                let is_unlocked = true; // TODO: Implement actual unlock checking
                let metadata = JokerMetadata::from_definition(&definition, is_unlocked);
                Ok(Some(metadata))
            }
            Ok(None) => Ok(None),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Failed to get joker metadata: {e}"
            ))),
        }
    }

    /// Get basic properties of a joker as a dictionary
    fn get_joker_properties(&self, joker_id: JokerId) -> PyResult<Option<PyObject>> {
        Python::with_gil(|py| match self.get_joker_metadata(joker_id)? {
            Some(metadata) => {
                let dict = pyo3::types::PyDict::new(py);
                dict.set_item("id", format!("{:?}", metadata.id))?;
                dict.set_item("name", metadata.name)?;
                dict.set_item("description", metadata.description)?;
                dict.set_item("rarity", format!("{:?}", metadata.rarity))?;
                dict.set_item("cost", metadata.cost)?;
                dict.set_item("sell_value", metadata.sell_value)?;
                Ok(Some(dict.into()))
            }
            None => Ok(None),
        })
    }

    /// Get effect information for a joker
    fn get_joker_effect_info(&self, joker_id: JokerId) -> PyResult<Option<PyObject>> {
        Python::with_gil(|py| match self.get_joker_metadata(joker_id)? {
            Some(metadata) => {
                let dict = pyo3::types::PyDict::new(py);
                dict.set_item("effect_type", metadata.effect_type)?;
                dict.set_item("effect_description", metadata.effect_description)?;
                dict.set_item("triggers_on", metadata.triggers_on)?;
                dict.set_item("conditions", metadata.conditions)?;
                dict.set_item("uses_state", metadata.uses_state)?;
                dict.set_item("max_triggers", metadata.max_triggers)?;
                dict.set_item("persistent_data", metadata.persistent_data)?;
                Ok(Some(dict.into()))
            }
            None => Ok(None),
        })
    }

    /// Get unlock status and condition for a joker
    fn get_joker_unlock_status(&self, joker_id: JokerId) -> PyResult<Option<PyObject>> {
        Python::with_gil(|py| match self.get_joker_metadata(joker_id)? {
            Some(metadata) => {
                let dict = pyo3::types::PyDict::new(py);
                dict.set_item("is_unlocked", metadata.is_unlocked)?;
                if let Some(condition) = metadata.unlock_condition {
                    dict.set_item("unlock_condition", format!("{condition:?}"))?;
                } else {
                    dict.set_item("unlock_condition", py.None())?;
                }
                Ok(Some(dict.into()))
            }
            None => Ok(None),
        })
    }

    // Batch Metadata Operations

    /// Get metadata for multiple jokers at once
    fn get_multiple_joker_metadata(&self, joker_ids: Vec<JokerId>) -> PyResult<Vec<Option<JokerMetadata>>> {
        joker_ids.into_iter()
            .map(|id| self.get_joker_metadata(id))
            .collect()
    }

    /// Get metadata for all registered jokers
    fn get_all_joker_metadata(&self) -> PyResult<Vec<JokerMetadata>> {
        match registry::all_definitions() {
            Ok(definitions) => {
                let metadata_list: Vec<JokerMetadata> = definitions.into_iter()
                    .map(|def| {
                        // Check if joker is unlocked (simplified for now)
                        let is_unlocked = true; // TODO: Implement actual unlock checking
                        JokerMetadata::from_definition(&def, is_unlocked)
                    })
                    .collect();
                Ok(metadata_list)
            }
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                format!("Failed to get all joker metadata: {e}")
            )),
        }
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
    fn round(&self) -> usize {
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
    fn plays(&self) -> usize {
        self.snapshot.plays
    }
    #[getter]
    fn discards(&self) -> usize {
        self.snapshot.discards
    }

    #[getter]
    fn score(&self) -> usize {
        self.snapshot.score
    }
    #[getter]
    fn required_score(&self) -> usize {
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
    fn money(&self) -> usize {
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
    m.add_class::<UnlockCondition>()?;
    m.add_class::<JokerMetadata>()?;

    Ok(())
}
