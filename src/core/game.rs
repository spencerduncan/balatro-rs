use crate::core::card::Card;
use crate::core::deck::Deck;
use crate::core::hand::{MadeHand, SelectHand};
use crate::core::moves::{Move, Moves};
use std::collections::HashSet;

use itertools::Itertools;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct Game {
    pub deck: Deck,
    pub available: Vec<Card>,
    pub mult: usize,
}

impl Game {
    pub fn new() -> Self {
        Self {
            deck: Deck::default(),
            available: Vec::new(),
            mult: 1,
        }
    }

    // shuffle and deal new cards to available
    pub fn deal(&mut self) {
        self.deck.shuffle();
        self.draw(7);
    }

    // draw from deck to available
    pub fn draw(&mut self, count: usize) {
        if let Some(drawn) = self.deck.draw(count) {
            self.available.extend(drawn);
        }
    }

    // discard specific cards from available and draw equal number back to available
    pub fn discard(&mut self, select: SelectHand) {
        // retain cards that we are not discarding
        let remove: HashSet<Card> = HashSet::from_iter(select.cards());
        self.available.retain(|c| !remove.contains(c));
        self.draw(select.cards().len())
    }

    pub fn score(&self, hand: MadeHand) -> usize {
        let base_mult = hand.rank.level().mult;
        let base_chips = hand.rank.level().chips;
        let hand_chips: usize = hand.hand.cards().iter().map(|c| c.chips()).sum();
        return (hand_chips + base_chips) * base_mult;
    }

    pub fn play(&self, select: SelectHand) {
        let best = select.best_hand().expect("is best hand (for now)");
        self.score(best);
    }

    // get all legal Play moves that can be executed given current state
    pub fn gen_moves_play(&self) -> impl Iterator<Item = Box<dyn Move>> {
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
            .map(|cards| Box::new(Moves::Play(cards)) as Box<dyn Move>);
        return combos;
    }

    // get all legal Play moves that can be executed given current state
    pub fn gen_moves_discard(&self) -> impl Iterator<Item = Box<dyn Move>> {
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
            .map(|cards| Box::new(Moves::Discard(cards)) as Box<dyn Move>);
        return combos;
    }

    // get all legal moves that can be executed given current state
    pub fn gen_moves(&self) -> impl Iterator<Item = Box<dyn Move>> {
        return self.gen_moves_play().chain(self.gen_moves_discard());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(g.available.len(), 7);
    }
}
