use crate::card::Card;
use crate::joker::Jokers;
use crate::stage::Blind;
use pyo3::pyclass;
use std::fmt;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "python", pyclass(eq))]
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum MoveDirection {
    Left,
    Right,
}

impl fmt::Display for MoveDirection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Left => {
                write!(f, "left")
            }
            Self::Right => {
                write!(f, "right")
            }
        }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "python", pyclass(eq))]
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum Action {
    SelectCard(Card),
    MoveCard(MoveDirection, Card),
    Play(),
    Discard(),
    CashOut(usize),
    BuyJoker(Jokers),
    NextRound(),
    SelectBlind(Blind),
    // SkipBlind(Blind),
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::SelectCard(card) => {
                write!(f, "SelectCard: {card}")
            }
            Self::Play() => {
                write!(f, "Play")
            }
            Self::Discard() => {
                write!(f, "Discard")
            }
            Self::MoveCard(dir, card) => {
                write!(f, "MoveCard: {card} - {dir}")
            }
            Self::CashOut(reward) => {
                write!(f, "CashOut: {reward}")
            }
            Self::BuyJoker(joker) => {
                write!(f, "BuyJoker: {joker}")
            }
            Self::NextRound() => {
                write!(f, "NextRound")
            }
            Self::SelectBlind(blind) => {
                write!(f, "SelectBlind: {blind}")
            }
        }
    }
}

#[cfg(feature = "python")]
impl Action {
    fn __repr__(&self) -> String {
        format!("Action: {self}")
    }
}
