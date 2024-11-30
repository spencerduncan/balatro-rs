use crate::core::card::Card;
use crate::core::game::Game;

use super::hand::SelectHand;

pub trait Move {
    fn apply(&self, game: &mut Game);
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub enum Moves {
    Play(Vec<Card>),
    Discard(Vec<Card>),
}

impl Move for Moves {
    fn apply(&self, game: &mut Game) {
        match self {
            Self::Play(cards) => {
                let hand = SelectHand::new(cards.to_vec());
                game.play(hand);
            }
            Self::Discard(cards) => {
                let hand = SelectHand::new(cards.to_vec());
                game.discard(hand);
            }
        }
    }
}
