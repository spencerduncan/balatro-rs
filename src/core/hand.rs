use itertools::Itertools;

use crate::core::card::Card;
use crate::core::card::Value;
use crate::core::rank::HandRank;

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

    /// Can play any number of cards, it is our responsibility
    /// to determine the best hand. Higher tier hands take precedence
    /// over lower tier hands regardless of their level or scoring.
    /// For example, if hand is K K K K 2, and all are diamonds,
    /// hand will always be a Four of a Kind and never a Flush.
    pub fn best_hand(&self) -> Option<MadeHand> {
        // We start trying to evaluate best hands first, so we
        // can return best hand right when we find it.

        // 5 cards can be many hands, lets check for all
        if self.len() == 5 {
            // Look for flush five first
            let flush = is_flush(&self.0);
            let fok = is_five_of_kind(&self.0);
            if flush && fok {
                return Some(MadeHand {
                    cards: self.0.clone(),
                    rank: HandRank::FlushFive,
                });
            }
        }

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

fn is_five_of_kind(cards: &Vec<Card>) -> bool {
    if cards.len() != 5 {
        return false;
    }
    let first = cards[0];
    return cards.iter().all(|&card| card.value == first.value);
}

fn is_fullhouse(cards: &Vec<Card>) -> bool {
    if cards.len() != 5 {
        return false;
    }
    // frequency of each card in hand
    let mut counts: Vec<usize> = cards.iter().counts().into_values().collect();
    // full house must have first count 3 and second count 2
    if counts.len() != 2 {
        return false;
    }
    counts.sort();
    return counts[0] == 3 && counts[1] == 2;
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
    // pair must have first count 2
    if counts.len() < 1 {
        return false;
    }
    counts.sort();
    return counts[0] == 2;
}
