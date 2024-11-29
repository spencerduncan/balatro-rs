use crate::core::card::{Card, Suit, Value};
use rand::{seq::SliceRandom, thread_rng};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        Self { cards: Vec::new() }
    }
    pub fn contains(&self, c: &Card) -> bool {
        self.cards.contains(c)
    }
    pub fn remove(&mut self, c: &Card) -> bool {
        if let Some(pos) = self.cards.iter().position(|x| x == c) {
            self.cards.remove(pos);
            return true;
        }
        return false;
    }
    pub fn push(&mut self, c: Card) {
        self.cards.push(c)
    }
    pub fn draw(&mut self, n: usize) -> Option<Vec<Card>> {
        if self.cards.len() < n {
            return None;
        }
        return Some(self.cards.drain(0..n).collect());
    }
    pub fn len(&self) -> usize {
        self.cards.len()
    }
    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut thread_rng());
    }
}

impl Default for Deck {
    fn default() -> Self {
        let mut cards: Vec<Card> = Vec::new();
        for v in &Value::values() {
            for s in &Suit::suits() {
                let c = Card::new(*v, *s);
                cards.push(c);
            }
        }
        Self { cards }
    }
}
