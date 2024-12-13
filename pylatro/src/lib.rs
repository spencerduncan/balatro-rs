use balatro_rs::action::Action;
use balatro_rs::card::Card;
use balatro_rs::error::GameError;
use balatro_rs::game::Game;
use balatro_rs::stage::{End, Stage};
use pyo3::prelude::*;

#[pyclass]
struct GameEngine {
    game: Game,
}

#[pymethods]
impl GameEngine {
    #[new]
    fn new() -> Self {
        GameEngine {
            game: Game::default(),
        }
    }

    fn gen_moves(&self) -> Vec<Action> {
        return self.game.gen_moves().collect();
    }

    fn handle_action(&mut self, action: Action) -> Result<(), GameError> {
        return self.game.handle_action(action);
    }

    #[getter]
    fn state(&self) -> GameState {
        return GameState {
            game: self.game.clone(),
        };
    }
    #[getter]
    fn is_over(&self) -> bool {
        return self.game.is_over();
    }
    #[getter]
    fn is_win(&self) -> bool {
        if let Some(end) = self.game.result() {
            if end == End::Win {
                return true;
            }
        }
        return false;
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
    #[getter]
    fn plays(&self) -> usize {
        return self.game.plays;
    }
    #[getter]
    fn discards(&self) -> usize {
        return self.game.discards;
    }

    fn __repr__(&self) -> String {
        format!("GameState:\n{}", self.game)
    }
}

#[pymodule]
fn pylatro(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<GameEngine>()?;
    m.add_class::<GameState>()?;
    Ok(())
}
