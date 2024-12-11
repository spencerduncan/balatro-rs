use crate::core::game::Game;

pub trait Effect {
    fn apply(&self, game: &mut Game);
}

pub struct Joker {
    pub name: String,
}

impl Effect for Joker {
    fn apply(&self, game: &mut Game) {
        game.mult += 4;
    }
}
