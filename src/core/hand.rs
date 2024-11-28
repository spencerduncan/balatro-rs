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
    // Get all values in a hand. Sorted lowest to highest.
    pub fn values(&self) -> Vec<Value> {
        self.0.iter().map(|x| x.value).sorted().collect()
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
        // Return sorted by value
        return counts
            .into_iter()
            .sorted_by(|a, b| Ord::cmp(&b.0, &a.0))
            .collect();
    }

    // Get all suits in a hand
    pub fn suits(&self) -> Vec<Suit> {
        self.0.iter().map(|x| x.suit).sorted().collect()
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
        // Return sorted by suit
        return counts
            .into_iter()
            .sorted_by(|a, b| Ord::cmp(&b.0, &a.0))
            .collect();
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
    dbg!(hand.values_freq());
    if hand.len() < 5 {
        return None;
    }
    if let Some((_value, cards)) = hand
        .values_freq()
        .into_iter()
        .find(|(_key, val)| val.len() == 5)
    {
        return Some(Hand::new(cards));
    } else {
        return None;
    }
}

fn is_fullhouse(hand: Hand) -> Option<Hand> {
    dbg!(hand.values_freq());
    if hand.len() < 5 {
        return None;
    }

    // First find 3ok
    let three = hand
        .values_freq()
        .into_iter()
        .find(|(_key, val)| val.len() == 3);
    if three.is_none() {
        return None;
    }
    let three_val = three
        .as_ref()
        .unwrap()
        .1
        .first()
        .expect("values freq has empty Vec<card>")
        .value;

    // Next find 2ok that isn't same value as 3ok
    let two = hand
        .values_freq()
        .into_iter()
        .find(|(key, val)| *key != three_val && val.len() == 2);
    if two.is_none() {
        return None;
    }

    // Combine 3ok and 2ok
    let mut cards: Vec<Card> = Vec::new();
    cards.extend(three.unwrap().1);
    cards.extend(two.unwrap().1);
    return Some(Hand::new(cards));
}

fn is_four_of_kind(hand: Hand) -> Option<Hand> {
    dbg!(hand.values_freq());
    if hand.len() < 4 {
        return None;
    }
    if let Some((_value, cards)) = hand
        .values_freq()
        .into_iter()
        .find(|(_key, val)| val.len() == 4)
    {
        return Some(Hand::new(cards));
    } else {
        return None;
    }
}

fn is_flush(hand: Hand) -> Option<Hand> {
    dbg!(hand.values_freq());
    if hand.len() < 5 {
        return None;
    }
    if let Some((_value, cards)) = hand
        .suits_freq()
        .into_iter()
        .find(|(_key, val)| val.len() == 5)
    {
        return Some(Hand::new(cards));
    } else {
        return None;
    }
}

fn is_straight(hand: Hand) -> Option<Hand> {
    dbg!(hand.values());
    if hand.len() != 5 {
        return None;
    }
    // Iterate our sorted values. Each value must be one more than the previous.
    let values = hand.values();
    if values.windows(2).all(|v| (v[1] as u16 - v[0] as u16) == 1) {
        return Some(hand);
    }

    // Special case for low ace.
    // Values are sorted with Ace as high (2, 3, 4, 5, A)
    // Therefore, we can check that last value is ace, first value is two.
    // Then remove the last value (ace) from vec and check for incremental values
    // for everything else (2, 3, 4, 5).
    if values[4] == Value::Ace && values[0] == Value::Two {
        let skip_last: Vec<Value> = values.into_iter().rev().skip(1).rev().collect();
        if skip_last
            .windows(2)
            .all(|v| (v[1] as u16 - v[0] as u16) == 1)
        {
            return Some(hand);
        }
    }
    return None;
}

fn is_straight_flush(hand: Hand) -> Option<Hand> {
    if is_flush(hand.clone()).is_some() && is_straight(hand.clone()).is_some() {
        return Some(hand);
    }
    return None;
}

fn is_royal_flush(hand: Hand) -> Option<Hand> {
    if is_straight_flush(hand.clone()).is_some()
        && hand.values().into_iter().eq(vec![
            Value::Ten,
            Value::Jack,
            Value::Queen,
            Value::King,
            Value::Ace,
        ])
    {
        return Some(hand);
    }
    return None;
}

fn is_three_of_kind(hand: Hand) -> Option<Hand> {
    dbg!(hand.values_freq());
    if hand.len() < 3 {
        return None;
    }
    if let Some((_value, cards)) = hand
        .values_freq()
        .into_iter()
        .find(|(_key, val)| val.len() == 3)
    {
        return Some(Hand::new(cards));
    } else {
        return None;
    }
}

fn is_two_pair(hand: Hand) -> Option<Hand> {
    dbg!(hand.values_freq());
    if hand.len() < 4 {
        return None;
    }

    // First find first pair
    let first = hand
        .values_freq()
        .into_iter()
        .find(|(_key, val)| val.len() == 2);
    if first.is_none() {
        return None;
    }
    let first_val = first
        .as_ref()
        .unwrap()
        .1
        .first()
        .expect("values freq has empty Vec<card>")
        .value;

    // Next find second pair that isn't same value as first pair
    let second = hand
        .values_freq()
        .into_iter()
        .find(|(key, val)| *key != first_val && val.len() == 2);
    if second.is_none() {
        return None;
    }

    // Combine first and second pair
    let mut cards: Vec<Card> = Vec::new();
    cards.extend(first.unwrap().1);
    cards.extend(second.unwrap().1);
    return Some(Hand::new(cards));
}

fn is_pair(hand: Hand) -> Option<Hand> {
    dbg!(hand.values_freq());
    if hand.len() < 2 {
        return None;
    }
    if let Some((_value, cards)) = hand
        .values_freq()
        .into_iter()
        .find(|(_key, val)| val.len() == 2)
    {
        return Some(Hand::new(cards));
    } else {
        return None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_values() {
        let c3 = Card::new(Value::Two, Suit::Heart);
        let c4 = Card::new(Value::Three, Suit::Diamond);
        let c5 = Card::new(Value::Jack, Suit::Heart);
        let c1 = Card::new(Value::King, Suit::Heart);
        let c2 = Card::new(Value::Ace, Suit::Spade);

        let hand = Hand::new(vec![c1, c2, c3, c4, c5]);
        let values = hand.values();

        // Should have 5 values
        assert_eq!(values.len(), 5);

        // Expect sorted (2, 3, J, K, A)
        assert_eq!(values[0], Value::Two);
        assert_eq!(values[1], Value::Three);
        assert_eq!(values[2], Value::Jack);
        assert_eq!(values[3], Value::King);
        assert_eq!(values[4], Value::Ace);
    }

    #[test]
    fn test_values_freq() {
        let c1 = Card::new(Value::King, Suit::Heart);
        let c2 = Card::new(Value::King, Suit::Spade);
        let c3 = Card::new(Value::Two, Suit::Heart);
        let c4 = Card::new(Value::Three, Suit::Diamond);
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

        // No extra cards
        assert_eq!(freq.get(&Value::Five), None);
        assert_eq!(freq.get(&Value::Nine), None);

        // Can also check the cards in the vec are as expected
        assert_eq!(freq.get(&Value::King).unwrap()[0].value, Value::King);
        assert_eq!(freq.get(&Value::King).unwrap()[1].value, Value::King);
        assert_eq!(freq.get(&Value::Two).unwrap()[0].value, Value::Two);
        assert_eq!(freq.get(&Value::Three).unwrap()[0].value, Value::Three);
        assert_eq!(freq.get(&Value::Four).unwrap()[0].value, Value::Four);
    }

    #[test]
    fn test_suits_freq() {
        let c1 = Card::new(Value::King, Suit::Heart);
        let c2 = Card::new(Value::King, Suit::Spade);
        let c3 = Card::new(Value::Two, Suit::Heart);
        let c4 = Card::new(Value::Three, Suit::Diamond);
        let c5 = Card::new(Value::Four, Suit::Heart);

        let hand = Hand::new(vec![c1, c2, c3, c4, c5]);
        let freq = hand.suits_freq();

        // Should have 3 values (heart, spade, diamond)
        assert_eq!(freq.len(), 3);

        // Expect 3 hearts and 1 each of spade and diamond
        assert_eq!(freq.get(&Suit::Heart).unwrap().len(), 3);
        assert_eq!(freq.get(&Suit::Spade).unwrap().len(), 1);
        assert_eq!(freq.get(&Suit::Diamond).unwrap().len(), 1);

        // No clubs to be found
        assert_eq!(freq.get(&Suit::Club), None);

        // Can also check the cards in the vec are as expected
        assert_eq!(freq.get(&Suit::Heart).unwrap()[0].suit, Suit::Heart);
        assert_eq!(freq.get(&Suit::Heart).unwrap()[1].suit, Suit::Heart);
        assert_eq!(freq.get(&Suit::Heart).unwrap()[2].suit, Suit::Heart);
        assert_eq!(freq.get(&Suit::Spade).unwrap()[0].suit, Suit::Spade);
        assert_eq!(freq.get(&Suit::Diamond).unwrap()[0].suit, Suit::Diamond);
    }

    #[test]
    fn test_five_of_kind() {
        let c1 = Card::new(Value::King, Suit::Heart);
        let c2 = Card::new(Value::King, Suit::Spade);
        let c3 = Card::new(Value::King, Suit::Heart);
        let c4 = Card::new(Value::King, Suit::Diamond);
        let c5 = Card::new(Value::King, Suit::Heart);
        let not = Card::new(Value::Ace, Suit::Heart);

        // Valid 5 (K, K, K, K, K)
        let hand = Hand::new(vec![c1, c2, c3, c4, c5]);
        let is_5 = is_five_of_kind(hand);
        assert_eq!(is_5.unwrap().len(), 5);

        // Valid 5 from 7 cards (K, K, K, K, K, A, A)
        let hand = Hand::new(vec![c1, c2, c3, c4, c5, not, not]);
        let is_5 = is_five_of_kind(hand);
        assert_eq!(is_5.unwrap().len(), 5);

        // Invalid 5 (K, K, K, K, A)
        let hand = Hand::new(vec![c1, c2, c3, c4, not]);
        let is_5 = is_five_of_kind(hand);
        assert_eq!(is_5, None);

        // Invalid 4 (K, K, K, K)
        let hand = Hand::new(vec![c1, c2, c3, c4]);
        let is_5 = is_five_of_kind(hand);
        assert_eq!(is_5, None);
    }

    #[test]
    fn test_fullhouse() {
        let c1 = Card::new(Value::King, Suit::Heart);
        let c2 = Card::new(Value::King, Suit::Spade);
        let c3 = Card::new(Value::King, Suit::Heart);
        let c4 = Card::new(Value::Four, Suit::Diamond);
        let c5 = Card::new(Value::Four, Suit::Heart);
        let not1 = Card::new(Value::Two, Suit::Heart);
        let not2 = Card::new(Value::Three, Suit::Heart);

        // Valid 5 (K, K, K, 4, 4)
        let hand = Hand::new(vec![c1, c2, c3, c4, c5]);
        let is_fh = is_fullhouse(hand);
        assert_eq!(is_fh.unwrap().len(), 5);

        // Valid 5 from 7 cards (K, K, K, 4, 4, 2, 3)
        let hand = Hand::new(vec![c1, c2, c3, c4, c5, not1, not2]);
        let is_fh = is_fullhouse(hand);
        assert_eq!(is_fh.unwrap().len(), 5);

        // Invalid 5 (K, K, K, K, 2)
        let hand = Hand::new(vec![c1, c2, c3, c3, not1]);
        let is_fh = is_fullhouse(hand);
        assert_eq!(is_fh, None);

        // Invalid 5 (K, K, 4, 4, 2)
        let hand = Hand::new(vec![c1, c2, c4, c5, not1]);
        let is_fh = is_fullhouse(hand);
        assert_eq!(is_fh, None);

        // Invalid 4 (K, K, 4, 4)
        let hand = Hand::new(vec![c1, c2, c4, c5]);
        let is_fh = is_fullhouse(hand);
        assert_eq!(is_fh, None);
    }

    #[test]
    fn test_four_of_kind() {
        let c1 = Card::new(Value::King, Suit::Heart);
        let c2 = Card::new(Value::King, Suit::Spade);
        let c3 = Card::new(Value::King, Suit::Heart);
        let c4 = Card::new(Value::King, Suit::Diamond);
        let not = Card::new(Value::Ace, Suit::Heart);

        // Valid 4 (K, K, K, K)
        let hand = Hand::new(vec![c1, c2, c3, c4, not]);
        let is_4 = is_four_of_kind(hand);
        assert_eq!(is_4.unwrap().len(), 4);

        // Valid 4 from 7 cards (K, K, K, K, A, A, A)
        let hand = Hand::new(vec![c1, c2, c3, c4, not, not, not]);
        let is_4 = is_four_of_kind(hand);
        assert_eq!(is_4.unwrap().len(), 4);

        // Invalid 4 (K, K, K, A)
        let hand = Hand::new(vec![c1, c2, c3, not]);
        let is_4 = is_four_of_kind(hand);
        assert_eq!(is_4, None);

        // Invalid 3 (K, K, K)
        let hand = Hand::new(vec![c1, c2, c3]);
        let is_4 = is_four_of_kind(hand);
        assert_eq!(is_4, None);
    }

    #[test]
    fn test_flush() {
        let c1 = Card::new(Value::King, Suit::Heart);
        let c2 = Card::new(Value::Queen, Suit::Heart);
        let c3 = Card::new(Value::Jack, Suit::Heart);
        let c4 = Card::new(Value::Seven, Suit::Heart);
        let c5 = Card::new(Value::Eight, Suit::Heart);
        let not = Card::new(Value::Ace, Suit::Diamond);

        // Valid 5 (h, h, h, h, h)
        let hand = Hand::new(vec![c1, c2, c3, c4, c5]);
        let flush = is_flush(hand);
        assert_eq!(flush.unwrap().len(), 5);

        // Valid 5 from 7 cards (h, h, h, h, h, d, d)
        let hand = Hand::new(vec![c1, c2, c3, c4, c5, not, not]);
        let flush = is_flush(hand);
        assert_eq!(flush.unwrap().len(), 5);

        // Invalid 5 (h, h, h, h, d)
        let hand = Hand::new(vec![c1, c2, c3, c4, not]);
        let flush = is_flush(hand);
        assert_eq!(flush, None);

        // Invalid 4 (h, h, h, h)
        let hand = Hand::new(vec![c1, c2, c3, c4]);
        let flush = is_flush(hand);
        assert_eq!(flush, None);
    }

    #[test]
    fn test_straight() {
        let c1 = Card::new(Value::Ace, Suit::Heart);
        let c2 = Card::new(Value::Two, Suit::Heart);
        let c3 = Card::new(Value::Three, Suit::Heart);
        let c4 = Card::new(Value::Four, Suit::Heart);
        let c5 = Card::new(Value::Five, Suit::Heart);
        let c6 = Card::new(Value::Six, Suit::Diamond);
        let c7 = Card::new(Value::Seven, Suit::Diamond);

        // Valid 5 (2, 3, 4 ,5 ,6)
        let hand = Hand::new(vec![c2, c3, c4, c5, c6]);
        let straight = is_straight(hand);
        assert_eq!(straight.unwrap().len(), 5);

        // Valid 5 with low ace (A, 2, 3, 4 ,5)
        let hand = Hand::new(vec![c1, c2, c3, c4, c5]);
        let straight = is_straight(hand);
        assert_eq!(straight.unwrap().len(), 5);

        // Invalid 5 (2, 3, 4, 5, 7)
        let hand = Hand::new(vec![c2, c3, c4, c5, c7]);
        let straight = is_straight(hand);
        assert_eq!(straight, None);

        // Invalid 5 with low ace (A, 2, 3, 4, 7)
        let hand = Hand::new(vec![c1, c2, c3, c4, c7]);
        let straight = is_straight(hand);
        assert_eq!(straight, None);

        // Invalid 4 (2, 3, 4, 5)
        let hand = Hand::new(vec![c2, c3, c4, c5]);
        let straight = is_straight(hand);
        assert_eq!(straight, None);
    }

    #[test]
    fn test_straight_flush() {
        let c1 = Card::new(Value::Ace, Suit::Heart);
        let c2 = Card::new(Value::Two, Suit::Heart);
        let c3 = Card::new(Value::Three, Suit::Heart);
        let c4 = Card::new(Value::Four, Suit::Heart);
        let c5 = Card::new(Value::Five, Suit::Heart);
        let c6 = Card::new(Value::Six, Suit::Heart);
        let not1 = Card::new(Value::Seven, Suit::Heart);
        let not2 = Card::new(Value::Six, Suit::Diamond);

        // Valid 5 (2h, 3h, 4h, 5h ,6h)
        let hand = Hand::new(vec![c2, c3, c4, c5, c6]);
        let sf = is_straight_flush(hand);
        assert_eq!(sf.unwrap().len(), 5);

        // Valid 5 with low ace (Ah, 2h, 3h, 4h, 5h)
        let hand = Hand::new(vec![c1, c2, c3, c4, c5]);
        let sf = is_straight_flush(hand);
        assert_eq!(sf.unwrap().len(), 5);

        // Invalid 5, wrong value (2h, 3h, 4h, 5h, 7h)
        let hand = Hand::new(vec![c2, c3, c4, c5, not1]);
        let sf = is_straight_flush(hand);
        assert_eq!(sf, None);

        // Invalid 5, wrong suit (2h, 3h, 4h, 5h, 6d)
        let hand = Hand::new(vec![c2, c3, c4, c5, not2]);
        let sf = is_straight_flush(hand);
        assert_eq!(sf, None);

        // Invalid 4 (2h, 3h, 4h, 5h)
        let hand = Hand::new(vec![c2, c3, c4, c5]);
        let sf = is_straight_flush(hand);
        assert_eq!(sf, None);
    }

    #[test]
    fn test_royal_flush() {
        let c1 = Card::new(Value::Ten, Suit::Spade);
        let c2 = Card::new(Value::Jack, Suit::Spade);
        let c3 = Card::new(Value::Queen, Suit::Spade);
        let c4 = Card::new(Value::King, Suit::Spade);
        let c5 = Card::new(Value::Ace, Suit::Spade);
        let not1 = Card::new(Value::Nine, Suit::Spade);
        let not2 = Card::new(Value::Ace, Suit::Diamond);

        // Valid 5 (10s, Js, Qs, Ks, As)
        let hand = Hand::new(vec![c1, c2, c3, c4, c5]);
        let rf = is_royal_flush(hand);
        assert_eq!(rf.unwrap().len(), 5);

        // Valid 5, scrambled input order (Js, 10s, Ks, Qs, As)
        let hand = Hand::new(vec![c2, c1, c4, c3, c5]);
        let rf = is_royal_flush(hand);
        assert_eq!(rf.unwrap().len(), 5);

        // Invalid 5, wrong value (9s, Js, Qs, Ks, As)
        let hand = Hand::new(vec![not1, c2, c3, c4, c5]);
        let rf = is_royal_flush(hand);
        assert_eq!(rf, None);

        // Invalid 5, wrong suit (10s, Js, Qs, Ks, Ad)
        let hand = Hand::new(vec![c1, c2, c3, c4, not2]);
        let rf = is_royal_flush(hand);
        assert_eq!(rf, None);

        // Invalid 4 (2h, 3h, 4h, 5h)
        let hand = Hand::new(vec![c2, c3, c4, c5]);
        let rf = is_royal_flush(hand);
        assert_eq!(rf, None);
    }

    #[test]
    fn test_three_of_kind() {
        let c1 = Card::new(Value::King, Suit::Heart);
        let c2 = Card::new(Value::King, Suit::Spade);
        let c3 = Card::new(Value::King, Suit::Heart);
        let not1 = Card::new(Value::Ace, Suit::Heart);
        let not2 = Card::new(Value::Two, Suit::Heart);

        // Valid 5 (K, K, K, A, 2)
        let hand = Hand::new(vec![c1, c2, c3, not1, not2]);
        let is_3 = is_three_of_kind(hand);
        assert_eq!(is_3.unwrap().len(), 3);

        // Valid 4 (K, K, K, A)
        let hand = Hand::new(vec![c1, c2, c3, not1]);
        let is_3 = is_three_of_kind(hand);
        assert_eq!(is_3.unwrap().len(), 3);

        // Valid 3 (K, K, K)
        let hand = Hand::new(vec![c1, c2, c3]);
        let is_3 = is_three_of_kind(hand);
        assert_eq!(is_3.unwrap().len(), 3);

        // Invalid 3 (K, K, A)
        let hand = Hand::new(vec![c1, c2, not1]);
        let is_3 = is_three_of_kind(hand);
        assert_eq!(is_3, None);

        // Invalid 4 (K, K, A, A),
        let hand = Hand::new(vec![c1, c2, not1, not1]);
        let is_3 = is_three_of_kind(hand);
        assert_eq!(is_3, None);

        // Invalid 5 (K, K, A, A, 2),
        let hand = Hand::new(vec![c1, c2, not1, not1, not2]);
        let is_3 = is_three_of_kind(hand);
        assert_eq!(is_3, None);

        // Invalid 2 (K, K)
        let hand = Hand::new(vec![c1, c2]);
        let is_3 = is_three_of_kind(hand);
        assert_eq!(is_3, None);
    }

    #[test]
    fn test_two_pair() {
        let c1 = Card::new(Value::King, Suit::Heart);
        let c2 = Card::new(Value::King, Suit::Spade);
        let c3 = Card::new(Value::Four, Suit::Diamond);
        let c4 = Card::new(Value::Four, Suit::Heart);
        let not1 = Card::new(Value::Two, Suit::Heart);
        let not2 = Card::new(Value::Three, Suit::Heart);

        // Valid 5 (K, K, 4, 4, 2)
        let hand = Hand::new(vec![c1, c2, c3, c4, not1]);
        let tp = is_two_pair(hand);
        assert_eq!(tp.unwrap().len(), 4);

        // Valid 4 (K, K, 4, 4)
        let hand = Hand::new(vec![c1, c2, c3, c4]);
        let tp = is_two_pair(hand);
        assert_eq!(tp.unwrap().len(), 4);

        // Invalid 5 (K, K, K, K, 2)
        let hand = Hand::new(vec![c1, c1, c2, c2, not1]);
        let tp = is_two_pair(hand);
        assert_eq!(tp, None);

        // Invalid 5 (K, 4, 3, 2, 2)
        let hand = Hand::new(vec![c1, c4, not1, not2, not2]);
        let tp = is_two_pair(hand);
        assert_eq!(tp, None);

        // Invalid 5 (K, K, 4, 3, 2)
        let hand = Hand::new(vec![c1, c1, c4, not1, not2]);
        let tp = is_two_pair(hand);
        assert_eq!(tp, None);

        // Invalid 4 (K, K, 4, 2)
        let hand = Hand::new(vec![c1, c2, c4, not1]);
        let tp = is_two_pair(hand);
        assert_eq!(tp, None);
    }

    #[test]
    fn test_is_pair() {
        let _c1 = Card::new(Value::King, Suit::Heart);
        let _c2 = Card::new(Value::King, Suit::Spade);
        let _c3 = Card::new(Value::Two, Suit::Heart);
        let _c4 = Card::new(Value::Three, Suit::Heart);
        let _c5 = Card::new(Value::Four, Suit::Heart);

        // assert_eq!(is_pair(&vec![c1, c2]), true);
    }
}
