use crate::core::hand::SelectHand;
use std::fmt;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum Action {
    Play(SelectHand),
    Discard(SelectHand),
}
impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Play(cards) => {
                write!(f, "(Play: {:?})", cards)
            }
            Self::Discard(cards) => {
                write!(f, "(Discard: {:?})", cards)
            }
        }
    }
}
