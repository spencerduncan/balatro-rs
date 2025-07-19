use crate::card::Card;
use crate::joker::JokerId;
use crate::shop::packs::PackType;
use crate::stage::Blind;
#[cfg(feature = "python")]
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
#[derive(Debug, PartialEq, Clone)]
pub enum Action {
    SelectCard(Card),
    MoveCard(MoveDirection, Card),
    Play(),
    Discard(),
    CashOut(f64),
    BuyJoker { joker_id: JokerId, slot: usize },
    BuyPack { pack_type: PackType },
    OpenPack { pack_id: usize },
    SelectFromPack { pack_id: usize, option_index: usize },
    SkipPack { pack_id: usize },
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
            Self::BuyJoker { joker_id, slot } => {
                write!(f, "BuyJoker: {joker_id:?} at slot {slot}")
            }
            Self::BuyPack { pack_type } => {
                write!(f, "BuyPack: {pack_type}")
            }
            Self::OpenPack { pack_id } => {
                write!(f, "OpenPack: {pack_id}")
            }
            Self::SelectFromPack {
                pack_id,
                option_index,
            } => {
                write!(f, "SelectFromPack: pack {pack_id}, option {option_index}")
            }
            Self::SkipPack { pack_id } => {
                write!(f, "SkipPack: {pack_id}")
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
