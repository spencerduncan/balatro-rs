use crate::core::action::Action;
use crate::core::ante::Ante;
use crate::core::card::Card;
use crate::core::deck::Deck;
use crate::core::error::GameError;
use crate::core::hand::{MadeHand, SelectHand};
use crate::core::stage::{Blind, End, Stage};
use std::collections::HashSet;

use itertools::Itertools;

const DEFAULT_PLAYS: usize = 4;
const DEFAULT_DISCARDS: usize = 4;
const HAND_SIZE: usize = 7;
const BASE_MULT: usize = 1;
const BASE_CHIPS: usize = 0;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct Game {
    pub deck: Deck,
    pub available: Vec<Card>,
    pub stage: Stage,
    pub ante: Ante,

    // playing
    pub plays: usize,
    pub discards: usize,

    // for scoring
    pub chips: usize,
    pub mult: usize,
}

impl Game {
    pub fn new() -> Self {
        Self {
            deck: Deck::default(),
            available: Vec::new(),
            stage: Stage::PreBlind,
            ante: Ante::One,
            plays: DEFAULT_PLAYS,
            discards: DEFAULT_DISCARDS,
            chips: BASE_CHIPS,
            mult: BASE_MULT,
        }
    }

    pub fn start(&mut self) {}

    pub fn over(&self) -> Option<End> {
        match self.stage {
            Stage::End(end) => {
                return Some(end);
            }
            _ => return None,
        }
    }

    // shuffle and deal new cards to available
    pub fn deal(&mut self) {
        self.deck.shuffle();
        self.draw(HAND_SIZE);
    }

    // draw from deck to available
    pub fn draw(&mut self, count: usize) {
        if let Some(drawn) = self.deck.draw(count) {
            self.available.extend(drawn);
        }
    }

    // discard specific cards from available and draw equal number back to available
    pub fn discard(&mut self, select: SelectHand) -> Result<(), GameError> {
        if self.discards <= 0 {
            return Err(GameError::NoRemainingDiscards);
        }
        self.discards -= 1;
        // retain cards that we are not discarding
        let remove: HashSet<Card> = HashSet::from_iter(select.cards());
        self.available.retain(|c| !remove.contains(c));
        self.draw(select.cards().len());
        return Ok(());
    }

    pub fn score(&self, hand: MadeHand) -> usize {
        let base_mult = hand.rank.level().mult;
        let base_chips = hand.rank.level().chips;
        let hand_chips: usize = hand.hand.cards().iter().map(|c| c.chips()).sum();
        return (hand_chips + base_chips) * base_mult;
    }

    pub fn check_score(&self, score: usize) -> Result<(), GameError> {
        let base = self.ante.base();
        let required = match self.stage {
            Stage::Blind(Blind::Small) => base,
            Stage::Blind(Blind::Big) => (base as f32 * 1.5) as usize,
            Stage::Blind(Blind::Boss) => base * 2,
            // can only check score if in blind stage
            _ => return Err(GameError::InvalidStage),
        };
        if score >= required {
            // TODO: move game stage state machine, progress blinds
        };
        // TODO: check for no more plays -> loss
        return Ok(());
    }

    pub fn play(&mut self, select: SelectHand) -> Result<(), GameError> {
        if self.plays <= 0 {
            return Err(GameError::NoRemainingPlays);
        }
        self.plays -= 1;
        let best = select.best_hand()?;
        self.score(best);
        return Ok(());
    }

    // get all legal Play moves that can be executed given current state
    pub fn gen_moves_play(&self) -> Option<impl Iterator<Item = Action>> {
        // If no plays remaining, return None
        if self.plays <= 0 {
            return None;
        }
        // For all available cards, we can both play every combination
        // of 1, 2, 3, 4 or 5 cards.
        let combos = self
            .available
            .clone()
            .into_iter()
            .combinations(5)
            .chain(self.available.clone().into_iter().combinations(4))
            .chain(self.available.clone().into_iter().combinations(3))
            .chain(self.available.clone().into_iter().combinations(2))
            .chain(self.available.clone().into_iter().combinations(1))
            .map(|cards| Action::Play(SelectHand::new(cards)));
        return Some(combos);
    }

    // get all legal Play moves that can be executed given current state
    pub fn gen_moves_discard(&self) -> Option<impl Iterator<Item = Action>> {
        // If no discards remaining, return None
        if self.discards <= 0 {
            return None;
        }
        // For all available cards, we can both discard every combination
        // of 1, 2, 3, 4 or 5 cards.
        let combos = self
            .available
            .clone()
            .into_iter()
            .combinations(5)
            .chain(self.available.clone().into_iter().combinations(4))
            .chain(self.available.clone().into_iter().combinations(3))
            .chain(self.available.clone().into_iter().combinations(2))
            .chain(self.available.clone().into_iter().combinations(1))
            .map(|cards| Action::Discard(SelectHand::new(cards)));
        return Some(combos);
    }

    // get all legal moves that can be executed given current state
    pub fn gen_moves(&self) -> impl Iterator<Item = Action> {
        let plays = self.gen_moves_play();
        let discards = self.gen_moves_discard();

        return plays
            .into_iter()
            .flatten()
            .chain(discards.into_iter().flatten());
    }

    pub fn handle_action(&mut self, action: Action) -> Result<(), GameError> {
        match action {
            Action::Play(hand) => self.play(hand)?,
            Action::Discard(hand) => self.discard(hand)?,
        }
        return Ok(());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::card::{Suit, Value};

    #[test]
    fn test_constructor() {
        let g = Game::new();
        assert_eq!(g.available.len(), 0);
        assert_eq!(g.deck.len(), 52);
        assert_eq!(g.mult, 1);
    }

    #[test]
    fn test_deal() {
        let mut g = Game::new();
        g.deal();
        // deck should be 7 cards smaller than we started with
        assert_eq!(g.deck.len(), 52 - HAND_SIZE);
        // should be 7 cards now available
        assert_eq!(g.available.len(), HAND_SIZE);
    }

    #[test]
    fn test_draw() {
        let mut g = Game::new();
        g.draw(1);
        assert_eq!(g.available.len(), 1);
        assert_eq!(g.deck.len(), 52 - 1);
        g.draw(3);
        assert_eq!(g.available.len(), 4);
        assert_eq!(g.deck.len(), 52 - 4);
    }
    #[test]
    fn test_discard() {
        let mut g = Game::new();
        g.deal();
        assert_eq!(g.available.len(), HAND_SIZE);
        assert_eq!(g.deck.len(), 52 - HAND_SIZE);
        // select first 4 cards
        let select = SelectHand::new(g.available[0..4].to_vec());
        let discard_res = g.discard(select.clone());
        assert!(discard_res.is_ok());
        // available should still be 7, we discarded then redrew to match
        assert_eq!(g.available.len(), HAND_SIZE);
        // deck is now smaller since we drew from it
        assert_eq!(g.deck.len(), 52 - HAND_SIZE - select.len());
    }

    #[test]
    fn test_score() {
        let g = Game::new();
        let ace = Card::new(Value::Ace, Suit::Heart);
        let king = Card::new(Value::King, Suit::Diamond);
        let jack = Card::new(Value::Jack, Suit::Club);

        // Score [Ah, Kd, Jc]
        // High card (level 1) -> chips=5, mult=1
        // Played cards (1 ace) -> 11 chips
        // (5 + 11) * 1 = 16
        let cards = vec![ace, king, jack];
        let hand = SelectHand::new(cards).best_hand().unwrap();
        let score = g.score(hand);
        assert_eq!(score, 16);

        // Score [Kd, Kd, Ah]
        // Pair (level 1) -> chips=10, mult=2
        // Played cards (2 kings) -> 10 + 10 == 20 chips
        // (10 + 20) * 2 = 60
        let cards = vec![king, king, ace];
        let hand = SelectHand::new(cards).best_hand().unwrap();
        let score = g.score(hand);
        assert_eq!(score, 60);

        // Score [Ah, Ah, Ah, Kd]
        // Three of kind (level 1) -> chips=30, mult=3
        // Played cards (3 aces) -> 11 + 11 + 11 == 33 chips
        // (30 + 33) * 3 = 189
        let cards = vec![ace, ace, ace, king];
        let hand = SelectHand::new(cards).best_hand().unwrap();
        let score = g.score(hand);
        assert_eq!(score, 189);

        // Score [Kd, Kd, Kd, Kd, Ah]
        // Four of kind (level 1) -> chips=60, mult=7
        // Played cards (4 kings) -> 10 + 10 + 10 + 10 == 40 chips
        // (60 + 40) * 7 = 700
        let cards = vec![king, king, king, king, ace];
        let hand = SelectHand::new(cards).best_hand().unwrap();
        let score = g.score(hand);
        assert_eq!(score, 700);

        // Score [Jc, Jc, Jc, Jc, Jc]
        // Flush five (level 1) -> chips=160, mult=16
        // Played cards (5 jacks) -> 10 + 10 + 10 + 10 + 10 == 50 chips
        // (160 + 50) * 16 = 3360
        let cards = vec![jack, jack, jack, jack, jack];
        let hand = SelectHand::new(cards).best_hand().unwrap();
        let score = g.score(hand);
        assert_eq!(score, 3360);
    }

    #[test]
    fn test_gen_moves_play() {
        let ace = Card::new(Value::Ace, Suit::Heart);
        let king = Card::new(Value::King, Suit::Diamond);
        let jack = Card::new(Value::Jack, Suit::Club);

        let mut g = Game::new();
        // Only 1 card available [(Ah)]
        // Playable moves: [Ah]
        g.available = vec![ace];
        let moves: Vec<Action> = g.gen_moves_play().expect("are plays").collect();
        assert_eq!(moves.len(), 1);

        // 2 cards available [Ah, Kd]
        // Playable moves: [(Ah, Kd), (Ah), (Kd)]
        g.available = vec![ace, king];
        let moves: Vec<Action> = g.gen_moves_play().expect("are plays").collect();
        assert_eq!(moves.len(), 3);

        // 3 cards available [Ah, Kd, Jc]
        // Playable moves: [(Ah, Kd, Jc), (Ah, Kd), (Ah, Jc), (Kd, Jc), (Ah), (Kd), (Jc)]
        g.available = vec![ace, king, jack];
        let moves: Vec<Action> = g.gen_moves_play().expect("are plays").collect();
        assert_eq!(moves.len(), 7);
    }

    #[test]
    fn test_gen_moves_discard() {
        let ace = Card::new(Value::Ace, Suit::Heart);
        let king = Card::new(Value::King, Suit::Diamond);
        let jack = Card::new(Value::Jack, Suit::Club);

        let mut g = Game::new();
        // Only 1 card available [(Ah)]
        // Playable moves: [Ah]
        g.available = vec![ace];
        let moves: Vec<Action> = g.gen_moves_discard().expect("are discards").collect();
        assert_eq!(moves.len(), 1);
        // let m = &moves[0];
        // // Test that we can apply that discard move to the game
        // m.apply(&mut g);
        // // available should still be 1, we discarded then redrew to match
        // assert_eq!(g.available.len(), 1);
        // // deck is now smaller since we drew from it
        // assert_eq!(g.deck.len(), 52 - 1);

        // 2 cards available [Ah, Kd]
        // Playable moves: [(Ah, Kd), (Ah), (Kd)]
        g.available = vec![ace, king];
        let moves: Vec<Action> = g.gen_moves_discard().expect("are discards").collect();
        assert_eq!(moves.len(), 3);

        // 3 cards available [Ah, Kd, Jc]
        // Playable moves: [(Ah, Kd, Jc), (Ah, Kd), (Ah, Jc), (Kd, Jc), (Ah), (Kd), (Jc)]
        g.available = vec![ace, king, jack];
        let moves: Vec<Action> = g.gen_moves_discard().expect("are discards").collect();
        assert_eq!(moves.len(), 7);
    }
}
