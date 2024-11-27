use itertools::Itertools;
use std::collections::HashMap;

use crate::core::card::Card;
use crate::core::card::Suit;
use crate::core::card::Value;
use crate::core::rank::HandRank;

/// A given card and the index it is in the hand.
// Useful to relate best hand rank back to associated cards in hand.
// For example, we can determine a hand A A K Q J is a pair, then use
// the index to determine the scoring cards are index 0 and 1 (A, A)
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct CardIndex {
    card: Card,
    index: u16,
}

/// Played/made hand
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct MadeHand {
    cards: Vec<Card>,
    rank: HandRank,
}

/// Struct to hold cards in hand
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Hand(Vec<Card>);

impl Hand {
    pub fn new(cards: Vec<Card>) -> Self {
        Self(cards)
    }
    pub fn push(&mut self, c: Card) {
        self.0.push(c);
    }
    pub fn truncate(&mut self, len: usize) {
        self.0.truncate(len)
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    // Get all values in a hand
    pub fn values(&self) -> Vec<Value> {
        self.0.iter().map(|x| x.value).collect()
    }

    // Get map of each value with corresponding cards.
    // For example, K♤, A♡, J♡, J♧, J♢ -> {J: [J♡, J♧: J♢], A: [A♡], K: [K♤]}
    pub fn values_freq(&self) -> HashMap<Value, Vec<Card>> {
        let mut counts: HashMap<Value, Vec<Card>> = HashMap::new();
        for card in self.0.clone() {
            if let Some(cards) = counts.get(&card.value) {
                let mut copy = cards.clone();
                copy.push(card);
                counts.insert(card.value, copy);
            } else {
                counts.insert(card.value, vec![card]);
            }
        }
        return counts;
    }

    // Get all suits in a hand
    pub fn suits(&self) -> Vec<Suit> {
        self.0.iter().map(|x| x.suit).collect()
    }

    // Get map of each suit with corresponding cards.
    // For example, K♤, A♡, J♡, J♧, J♢ -> {♡: [J♡, A♡], ♤: [K♤], ♧: [J♧], ♢: [J♢]}
    pub fn suits_freq(&self) -> HashMap<Suit, Vec<Card>> {
        let mut counts: HashMap<Suit, Vec<Card>> = HashMap::new();
        for card in self.0.clone() {
            if let Some(cards) = counts.get(&card.suit) {
                let mut copy = cards.clone();
                copy.push(card);
                counts.insert(card.suit, copy);
            } else {
                counts.insert(card.suit, vec![card]);
            }
        }
        return counts;
    }

    /// Can play any number of cards, it is our responsibility
    /// to determine the best hand. Higher tier hands take precedence
    /// over lower tier hands regardless of their level or scoring.
    /// For example, if hand is K K K K 2, and all are diamonds,
    /// hand will always be a Four of a Kind and never a Flush.
    pub fn best_hand(&self) -> Option<MadeHand> {
        // We start trying to evaluate best hands first, so we
        // can return best hand right when we find it.

        // 5 cards can be many hands, lets check for all
        if self.len() == 5 {}

        // no cards, no best hand
        if self.len() == 0 {
            return None;
        }
        // only 1 card, must be high card
        if self.len() == 1 {
            return Some(MadeHand {
                cards: self.0.clone(),
                rank: HandRank::HighCard,
            });
        }
        // 2 cards, either a pair or high card
        if self.len() == 1 {
            return Some(MadeHand {
                cards: self.0.clone(),
                rank: HandRank::HighCard,
            });
        }
        return None;
    }
}

fn is_five_of_kind(hand: Hand) -> Option<Hand> {
    if hand.len() != 5 {
        return None;
    }
    if hand.values_freq().len() == 1 {
        return Some(hand);
    } else {
        return None;
    }
}

fn is_fullhouse(hand: Hand) -> Option<Hand> {
    if hand.len() != 5 {
        return None;
    }
    let counts = hand.values_freq();
    if counts.len() != 2 {
        return None;
    }
    let mut cards: Vec<Card> = Vec::new();
    for card_vec in counts.values() {
        cards.extend(card_vec);
    }
    return Some(Hand::new(cards));
}

fn is_four_of_kind(cards: &Vec<Card>) -> bool {
    // frequency of each card in hand
    let mut counts: Vec<usize> = cards.iter().counts().into_values().collect();
    // four of kind must have first count 4
    if counts.len() < 1 {
        return false;
    }
    counts.sort();
    return counts[0] == 4;
}

fn is_flush(cards: &Vec<Card>) -> bool {
    if cards.len() != 5 {
        return false;
    }
    let first = cards[0];
    return cards.iter().all(|&card| card.suit == first.suit);
}

fn is_straight(cards: &Vec<Card>) -> bool {
    if cards.len() != 5 {
        return false;
    }
    let mut sorted = cards.clone();
    sorted.sort();

    // special case for low ace.
    // first value must be ace, second must be two,
    // then skip the ace and check for ascending like normal.
    if sorted[0].value == Value::Ace {
        if sorted[1].value != Value::Two {
            return false;
        }
        sorted = sorted.into_iter().skip(1).collect();
    }

    // check ascending values
    return sorted
        .windows(2)
        .all(|w| (w[1].value as u16 - w[0].value as u16) == 1);
}

fn is_three_of_kind(cards: &Vec<Card>) -> bool {
    // frequency of each card in hand
    let mut counts: Vec<usize> = cards.iter().counts().into_values().collect();
    // four of kind must have first count 3
    if counts.len() < 1 {
        return false;
    }
    counts.sort();
    return counts[0] == 3;
}

fn is_two_pair(cards: &Vec<Card>) -> bool {
    // frequency of each card in hand
    let mut counts: Vec<usize> = cards.iter().counts().into_values().collect();
    // two pair must have first and second count 2
    if counts.len() < 2 {
        return false;
    }
    counts.sort();
    return counts[0] == 2 && counts[1] == 2;
}

fn is_pair(cards: &Vec<Card>) -> bool {
    // frequency of each card in hand
    let mut counts: Vec<usize> = cards.iter().counts().into_values().collect();
    dbg!(counts.clone());
    // pair must have first count 2
    if counts.len() < 1 {
        return false;
    }
    counts.sort();
    return counts[0] == 2;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_pair() {
        let _c1 = Card::new(Value::King, Suit::Heart);
        let _c2 = Card::new(Value::King, Suit::Spade);
        let _c3 = Card::new(Value::Two, Suit::Heart);
        let _c4 = Card::new(Value::Three, Suit::Heart);
        let _c5 = Card::new(Value::Four, Suit::Heart);

        // assert_eq!(is_pair(&vec![c1, c2]), true);
    }

    #[test]
    fn test_values_freq() {
        let c1 = Card::new(Value::King, Suit::Heart);
        let c2 = Card::new(Value::King, Suit::Spade);
        let c3 = Card::new(Value::Two, Suit::Heart);
        let c4 = Card::new(Value::Three, Suit::Heart);
        let c5 = Card::new(Value::Four, Suit::Heart);

        let hand = Hand::new(vec![c1, c2, c3, c4, c5]);
        let freq = hand.values_freq();

        // Should have 4 values (K, 2, 3, 4)
        assert_eq!(freq.len(), 4);

        // Expect 2 kings and 1 each of 2, 3, 4
        assert_eq!(freq.get(&Value::King).unwrap().len(), 2);
        assert_eq!(freq.get(&Value::Two).unwrap().len(), 1);
        assert_eq!(freq.get(&Value::Three).unwrap().len(), 1);
        assert_eq!(freq.get(&Value::Four).unwrap().len(), 1);
    }
}
