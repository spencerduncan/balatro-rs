//! Scaling Chips Jokers (Compatibility module)
//!
//! This module provides access to scaling chips jokers that accumulate chip bonuses over time.
//! These jokers implement the core Balatro scaling mechanics for chips-based effects.
//!
//! Jokers included:
//! - Castle: Gains chips per discarded card of a specific suit (changes each round)
//! - Wee: Gains chips when 2s are scored
//! - Stuntman: Provides flat chips but reduces hand size
//! - Hiker: Every played card permanently gains chips
//! - Odd Todd: Chips for odd rank cards (A, 9, 7, 5, 3)
//! - Arrowhead: Chips for Spade suit cards
//! - Scholar: Chips and mult for Aces

pub use crate::joker::scaling_chips_jokers::{
    ArrowheadJoker, CastleJoker, HikerJoker, OddToddJoker, ScholarJoker, StuntmanJoker, WeeJoker,
};