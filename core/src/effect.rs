use crate::game::Game;
use crate::hand::MadeHand;
use crate::joker::{Jokers, OldJoker as Joker};
use std::sync::{Arc, Mutex};

// Type aliases to simplify complex types
type GameHandCallback = Arc<Mutex<dyn Fn(&mut Game, MadeHand) + Send + 'static>>;
type GameCallback = Arc<Mutex<dyn Fn(&mut Game) + Send + 'static>>;

#[derive(Debug, Clone)]
pub struct EffectRegistry {
    pub on_play: Vec<Effects>,
    pub on_discard: Vec<Effects>,
    pub on_score: Vec<Effects>,
    pub on_handrank: Vec<Effects>,
}

impl Default for EffectRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl EffectRegistry {
    pub fn new() -> Self {
        Self {
            on_play: Vec::new(),
            on_discard: Vec::new(),
            on_score: Vec::new(),
            on_handrank: Vec::new(),
        }
    }
    pub(crate) fn register_jokers(&mut self, jokers: Vec<Jokers>, game: &Game) {
        for j in jokers.clone() {
            for e in j.effects(game) {
                match e {
                    Effects::OnPlay(_) => self.on_play.push(e),
                    Effects::OnDiscard(_) => self.on_discard.push(e),
                    Effects::OnScore(_) => self.on_score.push(e),
                    Effects::OnHandRank(_) => self.on_handrank.push(e),
                }
            }
        }
    }
    
    /// Register a single joker with the effect registry
    pub fn register_joker(&mut self, joker: Jokers, game: &Game) {
        for e in joker.effects(game) {
            match e {
                Effects::OnPlay(_) => self.on_play.push(e),
                Effects::OnDiscard(_) => self.on_discard.push(e),
                Effects::OnScore(_) => self.on_score.push(e),
                Effects::OnHandRank(_) => self.on_handrank.push(e),
            }
        }
    }
    
    /// Count the total number of registered effects
    pub fn count_registered_effects(&self) -> usize {
        self.on_play.len() + self.on_discard.len() + self.on_score.len() + self.on_handrank.len()
    }
}

#[derive(Clone)]
// signature of these callbacks are more complicated so they
// can be used by pyo3 as part of python class.
pub enum Effects {
    OnPlay(GameHandCallback),
    OnDiscard(GameHandCallback),
    OnScore(GameHandCallback),
    OnHandRank(GameCallback),
}

impl std::fmt::Debug for Effects {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::OnPlay(_) => write!(f, "OnPlay"),
            Self::OnDiscard(_) => write!(f, "OnDiscard"),
            Self::OnScore(_) => write!(f, "OnScore"),
            Self::OnHandRank(_) => write!(f, "OnHandRank"),
        }
    }
}
