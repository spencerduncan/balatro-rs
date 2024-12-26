use balatro_rs::action::Action;
use balatro_rs::card::Card;
use balatro_rs::config::Config;
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
    #[pyo3(signature = (config=None))]
    fn new(config: Option<Config>) -> Self {
        GameEngine {
            game: Game::new(config.unwrap_or(Config::default())),
        }
    }

    fn gen_actions(&self) -> Vec<Action> {
        return self.game.gen_actions().collect();
    }

    fn gen_action_space(&self) -> Vec<usize> {
        return self.game.gen_action_space().to_vec();
    }

    fn handle_action(&mut self, action: Action) -> Result<(), GameError> {
        return self.game.handle_action(action);
    }

    fn handle_action_index(&mut self, index: usize) -> Result<(), GameError> {
        return self.game.handle_action_index(index);
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
    #[getter]
    fn score(&self) -> usize {
        return self.game.score;
    }
    #[getter]
    fn required_score(&self) -> usize {
        return self.game.required_score();
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
    Ok(())
}
