use balatro_rs::action::Action;
use balatro_rs::card::Card;
use balatro_rs::config::Config;
use balatro_rs::error::GameError;
use balatro_rs::game::Game;
use balatro_rs::joker::Jokers;
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
        Ok(format!("{}", action))
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
    Ok(())
}
