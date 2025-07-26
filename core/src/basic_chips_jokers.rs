//! Resource-Based Chips Jokers (Legacy compatibility module)
//!
//! This module provides backward compatibility for the old monolithic Joker trait
//! implementations. New code should use the trait-based implementations in
//! `joker::resource_chips_jokers` instead.
//!
//! These jokers provide chip bonuses based on game resources:
//! - Banner: +30 chips per remaining discard
//! - Bull: +2 chips per $1 owned
//! - Stone: +25 chips per Stone card in deck
//! - Scary Face: +30 chips when face cards are scored
//! - Blue: +2 chips per card remaining in deck

#[doc(hidden)]
#[deprecated(
    since = "0.1.0",
    note = "Use joker::resource_chips_jokers module instead"
)]
pub use crate::joker::resource_chips_jokers::{
    BannerJoker, BlueJoker, BullJoker, ScaryFaceJoker, StoneJoker,
};
