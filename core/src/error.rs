use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum PlayHandError {
    #[error("Played hand contains more than 5 cards")]
    TooManyCards,
    #[error("Played hand contains no cards")]
    NoCards,
    #[error("Played hand could not determine best hand")]
    UnknownHand,
}

#[derive(Error, Debug, Clone)]
pub enum GameError {
    #[error("No remaining discards")]
    NoRemainingDiscards,
    #[error("No remaining plays")]
    NoRemainingPlays,
    #[error("Invalid hand played")]
    InvalidHand(#[from] PlayHandError),
    #[error("Invalid stage")]
    InvalidStage,
    #[error("Invalid action")]
    InvalidAction,
    #[error("No card match")]
    InvalidBlind,
    #[error("Invalid blind")]
    NoCardMatch,
    #[error("Invalid move direction")]
    InvalidMoveDirection,
}

#[cfg(feature = "python")]
impl std::convert::From<GameError> for PyErr {
    fn from(err: GameError) -> PyErr {
        PyException::new_err(err.to_string())
    }
}
