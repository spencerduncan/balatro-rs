use crate::core::card::Card;
use crate::core::deck::Deck;
use crate::core::hand::{MadeHand, SelectHand};

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

    pub fn deal(&mut self) {
        self.deck.shuffle();
        if let Some(drawn) = self.deck.draw(7) {
            self.available.extend(drawn);
        }
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
}
