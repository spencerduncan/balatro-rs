use thiserror::Error;

#[derive(Error, Debug)]
pub enum PlayHandError {
    #[error("Played hand contains more than 5 cards")]
    TooManyCards,
    #[error("Played hand contains no cards")]
    NoCards,
    #[error("Played hand could not determine best hand")]
    UnknownHand,
}
