use crate::core::action::{Action, Actions};
use crate::core::card::Card;
use crate::core::deck::Deck;
use crate::core::hand::{MadeHand, SelectHand};
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

    // get all legal actions that can be executed given current state
    pub fn gen_actions(&self) -> impl Iterator<Item = Box<dyn Action>> {
        let mut actions: Vec<Box<dyn Action>> = vec![];
        // For all available cards, we can both play or discard every combination
        // of 1, 2, 3, 4 or 5 cards. We generate all combos and then create both
        // Play and Discard actions from the select.
        let combos: Vec<Vec<Card>> = self
            .available
            .clone()
            .into_iter()
            .combinations(5)
            .chain(self.available.clone().into_iter().combinations(4))
            .chain(self.available.clone().into_iter().combinations(3))
            .chain(self.available.clone().into_iter().combinations(2))
            .chain(self.available.clone().into_iter().combinations(1))
            .collect();
        for combo in combos.clone() {
            actions.push(Box::new(Actions::Play(combo)))
        }
        for combo in combos {
            actions.push(Box::new(Actions::Discard(combo)))
        }
        return actions.into_iter();
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
}
