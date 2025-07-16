use balatro_rs::action::Action;
use balatro_rs::card::Card;
use balatro_rs::concurrent_state::LockFreeStateSnapshot;
use balatro_rs::config::Config;
use balatro_rs::error::GameError;
use balatro_rs::game::Game;
use balatro_rs::joker::{JokerId, JokerRarity, Jokers};
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
            return self.game.money.load(std::sync::atomic::Ordering::Acquire) >= cost;
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

    /// Get lightweight state snapshot for high-frequency access
    #[getter]
    fn lightweight_state(&self) -> LightweightGameState {
        let snapshot = self.game.get_lock_free_state_snapshot();
        LightweightGameState { snapshot }
    }

    /// Concurrent-safe access to money value
    fn get_money_concurrent(&self) -> usize {
        self.game.get_money_concurrent()
    }

    /// Concurrent-safe access to chips value
    fn get_chips_concurrent(&self) -> usize {
        self.game.get_chips_concurrent()
    }

    /// Concurrent-safe access to score value
    fn get_score_concurrent(&self) -> usize {
        self.game.get_score_concurrent()
    }

    /// Concurrent-safe access to stage value
    fn get_stage_concurrent(&self) -> String {
        self.game.get_stage_concurrent()
    }

    /// Enable action caching for improved performance
    fn enable_action_caching(&mut self, ttl_ms: u64) {
        let ttl = std::time::Duration::from_millis(ttl_ms);
        self.game.enable_action_caching(ttl);
    }

    /// Generate actions with caching optimization
    fn get_cached_actions(&self) -> Vec<Action> {
        self.game.gen_actions_cached()
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
        self.game.plays.load(std::sync::atomic::Ordering::Acquire)
    }
    #[getter]
    fn discards(&self) -> usize {
        self.game
            .discards
            .load(std::sync::atomic::Ordering::Acquire)
    }

    #[getter]
    fn score(&self) -> usize {
        self.game.score.load(std::sync::atomic::Ordering::Acquire)
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
        self.game.money.load(std::sync::atomic::Ordering::Acquire)
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

        // Create temporary GameEngine for delegation
        let temp_engine = GameEngine {
            game: self.game.clone(),
        };
        Ok(temp_engine.gen_actions())
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

        // Create temporary GameEngine for delegation
        let temp_engine = GameEngine {
            game: self.game.clone(),
        };
        Ok(temp_engine.gen_action_space())
    }

    /// DEPRECATED: Use GameEngine.get_action_name() instead
    /// This method is provided for backwards compatibility only.
    fn get_action_name(&self, index: usize) -> PyResult<String> {
        // Issue deprecation warning
        Python::with_gil(|py| -> PyResult<()> {
            let warnings = py.import("warnings")?;
            warnings.call_method1(
                "warn",
                ("GameState.get_action_name() is deprecated. Use GameEngine.get_action_name() instead. GameState should only be used for reading game state, not performing actions.",),
            )?;
            Ok(())
        })?;

        // Create temporary GameEngine for delegation
        let temp_engine = GameEngine {
            game: self.game.clone(),
        };
        temp_engine
            .get_action_name(index)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{e}")))
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

        Ok(self.game.is_over())
    }

    fn __repr__(&self) -> String {
        format!("GameState:\n{}", self.game)
    }
}

/// Lightweight game state for high-frequency access without cloning
#[pyclass]
struct LightweightGameState {
    snapshot: LockFreeStateSnapshot,
}

#[pymethods]
impl LightweightGameState {
    #[getter]
    fn money(&self) -> usize {
        self.snapshot.money
    }

    #[getter]
    fn chips(&self) -> usize {
        self.snapshot.chips
    }

    #[getter]
    fn mult(&self) -> usize {
        self.snapshot.mult
    }

    #[getter]
    fn score(&self) -> usize {
        self.snapshot.score
    }

    #[getter]
    fn stage(&self) -> String {
        self.snapshot.stage.clone()
    }

    #[getter]
    fn round(&self) -> usize {
        self.snapshot.round
    }

    #[getter]
    fn plays_remaining(&self) -> usize {
        self.snapshot.plays_remaining
    }

    #[getter]
    fn discards_remaining(&self) -> usize {
        self.snapshot.discards_remaining
    }

    fn __repr__(&self) -> String {
        format!(
            "LightweightGameState(money={}, chips={}, mult={}, score={}, stage='{}', round={})",
            self.snapshot.money,
            self.snapshot.chips,
            self.snapshot.mult,
            self.snapshot.score,
            self.snapshot.stage,
            self.snapshot.round
        )
    }
}

#[pymodule]
fn pylatro(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Config>()?;
    m.add_class::<GameEngine>()?;
    m.add_class::<GameState>()?;
    m.add_class::<LightweightGameState>()?;
    m.add_class::<Stage>()?;

    // Add new JokerId-based types
    m.add_class::<JokerId>()?;
    m.add_class::<JokerRarity>()?;
    m.add_class::<JokerDefinition>()?;
    m.add_class::<UnlockCondition>()?;

    Ok(())
}
