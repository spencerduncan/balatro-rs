use balatro_rs::action::Action;
use balatro_rs::card::Card;
use balatro_rs::error::GameError;
use balatro_rs::game::Game;
use balatro_rs::stage::Stage;
use pyo3::prelude::*;

#[pyclass]
struct GameEngine {
    game: Game,
}

#[pymethods]
impl GameEngine {
    #[new]
    fn new() -> Self {
        GameEngine { game: Game::new() }
    }

    fn gen_moves(&self) -> Vec<Action> {
        return self.game.gen_moves().collect();
    }

    fn handle_action(&mut self, action: Action) -> Result<(), GameError> {
        return self.game.handle_action(action);
    }

    fn get_state(&self) -> GameState {
        return GameState {
            game: self.game.clone(),
        };
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
        return self.game.stage;
    }
    #[getter]
    fn action_history(&self) -> Vec<Action> {
        return self.game.action_history.clone();
    }
    #[getter]
    fn deck(&self) -> Vec<Card> {
        return self.game.deck.cards();
    }
    #[getter]
    fn available(&self) -> Vec<Card> {
        return self.game.deck.cards();
    }
    #[getter]
    fn discarded(&self) -> Vec<Card> {
        return self.game.discarded.clone();
    }
}

#[pymodule]
fn pylatro(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<GameEngine>()?;
    m.add_class::<GameState>()?;
    Ok(())
}
