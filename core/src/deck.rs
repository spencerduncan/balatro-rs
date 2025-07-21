use crate::card::{Card, Suit, Value};
use balatro_rs::rng::GameRng;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        Self { cards: Vec::new() }
    }
    pub(crate) fn draw(&mut self, n: usize) -> Option<Vec<Card>> {
        if self.cards.len() < n {
            return None;
        }
        Some(self.cards.drain(0..n).collect())
    }
    pub(crate) fn len(&self) -> usize {
        self.cards.len()
    }

    pub(crate) fn shuffle(&mut self, rng: &GameRng) {
        rng.shuffle(&mut self.cards);
    }

    pub(crate) fn append(&mut self, other: &mut Vec<Card>) {
        self.cards.append(other);
    }

    pub(crate) fn extend(&mut self, other: Vec<Card>) {
        self.cards.extend(other);
    }

    pub fn cards(&self) -> Vec<Card> {
        self.cards.clone()
    }

    // // Loops through cards, assigning index to each equal to index in deck
    // pub(crate) fn index_cards(&mut self) {
    //     let mut i = 0;
    //     for card in &mut self.cards {
    //         card.index = Some(i);
    //         i += 1;
    //     }
    // }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::{Suit, Value};

    #[test]
    fn test_deck_new() {
        let deck = Deck::new();
        assert_eq!(deck.len(), 0);
        assert!(deck.cards().is_empty());
    }

    #[test]
    fn test_deck_default() {
        let deck = Deck::default();

        // Standard deck should have 52 cards
        assert_eq!(deck.len(), 52);

        // Should have exactly 4 suits × 13 values = 52 cards
        let cards = deck.cards();
        assert_eq!(cards.len(), 52);

        // Verify we have all suits and values
        let mut suit_counts = std::collections::HashMap::new();
        let mut value_counts = std::collections::HashMap::new();

        for card in &cards {
            *suit_counts.entry(card.suit).or_insert(0) += 1;
            *value_counts.entry(card.value).or_insert(0) += 1;
        }

        // Should have exactly 13 cards of each suit
        assert_eq!(suit_counts.len(), 4);
        for &count in suit_counts.values() {
            assert_eq!(count, 13);
        }

        // Should have exactly 4 cards of each value
        assert_eq!(value_counts.len(), 13);
        for &count in value_counts.values() {
            assert_eq!(count, 4);
        }
    }

    #[test]
    fn test_deck_draw_normal() {
        let mut deck = Deck::default();
        let initial_size = deck.len();

        // Draw 5 cards
        let drawn = deck.draw(5).expect("Should be able to draw 5 cards");
        assert_eq!(drawn.len(), 5);
        assert_eq!(deck.len(), initial_size - 5);

        // Verify the drawn cards are distinct
        for i in 0..drawn.len() {
            for j in i + 1..drawn.len() {
                assert_ne!(drawn[i], drawn[j], "Drawn cards should be distinct");
            }
        }
    }

    #[test]
    fn test_deck_draw_exact_amount() {
        let mut deck = Deck::default();
        let total_cards = deck.len();

        // Draw exactly all cards
        let drawn = deck
            .draw(total_cards)
            .expect("Should be able to draw all cards");
        assert_eq!(drawn.len(), total_cards);
        assert_eq!(deck.len(), 0);
    }

    #[test]
    fn test_deck_draw_more_than_available() {
        let mut deck = Deck::default();
        let total_cards = deck.len();

        // Try to draw more cards than available
        let result = deck.draw(total_cards + 1);
        assert!(
            result.is_none(),
            "Should return None when trying to draw more cards than available"
        );

        // Deck should be unchanged
        assert_eq!(deck.len(), total_cards);
    }

    #[test]
    fn test_deck_draw_from_empty() {
        let mut deck = Deck::new();

        // Try to draw from empty deck
        let result = deck.draw(1);
        assert!(
            result.is_none(),
            "Should return None when drawing from empty deck"
        );

        let result = deck.draw(0);
        assert_eq!(
            result,
            Some(vec![]),
            "Should return empty vector when drawing 0 cards"
        );
    }

    #[test]
    fn test_deck_draw_zero_cards() {
        let mut deck = Deck::default();
        let initial_size = deck.len();

        // Draw 0 cards
        let drawn = deck.draw(0).expect("Should be able to draw 0 cards");
        assert_eq!(drawn.len(), 0);
        assert_eq!(deck.len(), initial_size);
    }

    #[test]
    fn test_deck_draw_order() {
        let mut deck = Deck::new();

        // Add specific cards in order
        let card1 = Card::new(Value::Ace, Suit::Heart);
        let card2 = Card::new(Value::King, Suit::Spade);
        let card3 = Card::new(Value::Queen, Suit::Diamond);
        deck.extend(vec![card1, card2, card3]);

        // Draw should return cards in FIFO order (from the beginning)
        let drawn = deck.draw(2).expect("Should draw 2 cards");
        assert_eq!(drawn[0], card1);
        assert_eq!(drawn[1], card2);

        // Remaining card should be the last one
        let remaining = deck.draw(1).expect("Should draw remaining card");
        assert_eq!(remaining[0], card3);
    }

    #[test]
    fn test_deck_len() {
        let mut deck = Deck::new();
        assert_eq!(deck.len(), 0);

        deck.extend(vec![Card::new(Value::Ace, Suit::Heart)]);
        assert_eq!(deck.len(), 1);

        deck.extend(vec![
            Card::new(Value::King, Suit::Spade),
            Card::new(Value::Queen, Suit::Diamond),
        ]);
        assert_eq!(deck.len(), 3);
    }

    #[test]
    fn test_deck_shuffle() {
        use balatro_rs::rng::GameRng;
        let mut deck = Deck::default();
        let original_cards = deck.cards();
        let rng = GameRng::for_testing(42);

        // Shuffle the deck multiple times to test randomization
        deck.shuffle(&rng);
        let shuffled_cards = deck.cards();

        // Should have same number of cards
        assert_eq!(shuffled_cards.len(), original_cards.len());

        // Should have the same cards (just in different order)
        let mut original_sorted = original_cards.clone();
        let mut shuffled_sorted = shuffled_cards.clone();
        original_sorted.sort_by_key(|c| (c.suit as u8, c.value as u8));
        shuffled_sorted.sort_by_key(|c| (c.suit as u8, c.value as u8));
        assert_eq!(original_sorted, shuffled_sorted);
    }

    #[test]
    fn test_deck_shuffle_empty() {
        use balatro_rs::rng::GameRng;
        let mut deck = Deck::new();
        let rng = GameRng::for_testing(42);

        // Shuffling empty deck should not panic
        deck.shuffle(&rng);
        assert_eq!(deck.len(), 0);
    }

    #[test]
    fn test_deck_shuffle_single_card() {
        use balatro_rs::rng::GameRng;
        let mut deck = Deck::new();
        let card = Card::new(Value::Ace, Suit::Heart);
        deck.extend(vec![card]);
        let rng = GameRng::for_testing(42);

        // Shuffling single card should not change anything
        deck.shuffle(&rng);
        assert_eq!(deck.len(), 1);
        assert_eq!(deck.cards()[0], card);
    }

    #[test]
    fn test_deck_append() {
        let mut deck = Deck::new();
        let card1 = Card::new(Value::Ace, Suit::Heart);
        let card2 = Card::new(Value::King, Suit::Spade);

        let mut other_cards = vec![card1, card2];
        deck.append(&mut other_cards);

        assert_eq!(deck.len(), 2);
        assert!(other_cards.is_empty()); // append should drain the source

        let deck_cards = deck.cards();
        assert_eq!(deck_cards[0], card1);
        assert_eq!(deck_cards[1], card2);
    }

    #[test]
    fn test_deck_append_empty() {
        let mut deck = Deck::new();
        let mut empty_cards = vec![];

        deck.append(&mut empty_cards);
        assert_eq!(deck.len(), 0);
    }

    #[test]
    fn test_deck_append_to_existing() {
        let mut deck = Deck::new();
        let existing_card = Card::new(Value::Queen, Suit::Diamond);
        deck.extend(vec![existing_card]);

        let new_card = Card::new(Value::Jack, Suit::Club);
        let mut new_cards = vec![new_card];
        deck.append(&mut new_cards);

        assert_eq!(deck.len(), 2);
        let deck_cards = deck.cards();
        assert_eq!(deck_cards[0], existing_card);
        assert_eq!(deck_cards[1], new_card);
    }

    #[test]
    fn test_deck_extend() {
        let mut deck = Deck::new();
        let card1 = Card::new(Value::Ace, Suit::Heart);
        let card2 = Card::new(Value::King, Suit::Spade);

        let cards_to_add = vec![card1, card2];
        deck.extend(cards_to_add);

        assert_eq!(deck.len(), 2);
        let deck_cards = deck.cards();
        assert_eq!(deck_cards[0], card1);
        assert_eq!(deck_cards[1], card2);
    }

    #[test]
    fn test_deck_extend_empty() {
        let mut deck = Deck::new();
        deck.extend(vec![]);
        assert_eq!(deck.len(), 0);
    }

    #[test]
    fn test_deck_extend_multiple() {
        let mut deck = Deck::new();

        // First extend
        deck.extend(vec![Card::new(Value::Ace, Suit::Heart)]);
        assert_eq!(deck.len(), 1);

        // Second extend
        deck.extend(vec![
            Card::new(Value::King, Suit::Spade),
            Card::new(Value::Queen, Suit::Diamond),
        ]);
        assert_eq!(deck.len(), 3);
    }

    #[test]
    fn test_deck_cards_clone() {
        let mut deck = Deck::default();
        let cards = deck.cards();

        // cards() should return a clone, not affect original deck
        assert_eq!(cards.len(), deck.len());

        // Modifying returned cards should not affect deck
        drop(cards);
        assert_eq!(deck.len(), 52);

        // Drawing from deck should not affect previously returned cards
        let cards_before_draw = deck.cards();
        deck.draw(10);
        assert_eq!(cards_before_draw.len(), 52); // Original clone unchanged
        assert_eq!(deck.len(), 42); // Deck is modified
    }

    #[test]
    fn test_deck_cards_empty() {
        let deck = Deck::new();
        let cards = deck.cards();
        assert!(cards.is_empty());
    }

    #[test]
    fn test_deck_multiple_operations() {
        let mut deck = Deck::new();
        let rng = balatro_rs::rng::GameRng::for_testing(42);

        // Add some cards
        deck.extend(vec![
            Card::new(Value::Ace, Suit::Heart),
            Card::new(Value::King, Suit::Spade),
            Card::new(Value::Queen, Suit::Diamond),
            Card::new(Value::Jack, Suit::Club),
        ]);
        assert_eq!(deck.len(), 4);

        // Draw some cards
        let drawn = deck.draw(2).expect("Should draw 2 cards");
        assert_eq!(drawn.len(), 2);
        assert_eq!(deck.len(), 2);

        // Shuffle remaining cards
        deck.shuffle(&rng);
        assert_eq!(deck.len(), 2);

        // Add more cards
        deck.extend(vec![Card::new(Value::Ten, Suit::Heart)]);
        assert_eq!(deck.len(), 3);

        // Draw all remaining
        let all_remaining = deck.draw(3).expect("Should draw all remaining");
        assert_eq!(all_remaining.len(), 3);
        assert_eq!(deck.len(), 0);
    }

    #[test]
    fn test_deck_edge_case_boundary_draw() {
        let mut deck = Deck::new();

        // Add exactly one card
        deck.extend(vec![Card::new(Value::Ace, Suit::Heart)]);

        // Try to draw exactly the number available
        let drawn = deck.draw(1).expect("Should draw the only card");
        assert_eq!(drawn.len(), 1);
        assert_eq!(deck.len(), 0);

        // Try to draw from now-empty deck
        let result = deck.draw(1);
        assert!(result.is_none());
    }

    #[test]
    fn test_deck_stress_operations() {
        use balatro_rs::rng::GameRng;
        let mut deck = Deck::default();
        let rng = GameRng::for_testing(42);

        // Perform many operations to test robustness
        for _ in 0..10 {
            deck.shuffle(&rng);

            if deck.len() > 5 {
                deck.draw(5).expect("Should be able to draw 5 cards");
            }

            // Add some cards back
            deck.extend(vec![
                Card::new(Value::Ace, Suit::Heart),
                Card::new(Value::King, Suit::Spade),
            ]);
        }

        // Should still be in valid state
        assert!(deck.len() > 0);
        let final_cards = deck.cards();
        assert_eq!(final_cards.len(), deck.len());
    }

    #[test]
    fn test_deck_clone() {
        let mut original_deck = Deck::default();
        original_deck.draw(5).expect("Draw from original");

        let cloned_deck = original_deck.clone();

        // Cloned deck should have same state
        assert_eq!(cloned_deck.len(), original_deck.len());
        assert_eq!(cloned_deck.cards(), original_deck.cards());

        // Modifying original should not affect clone
        original_deck.draw(5).expect("Draw more from original");
        assert_ne!(cloned_deck.len(), original_deck.len());
    }

    #[test]
    fn test_deck_debug() {
        let deck = Deck::default();
        let debug_str = format!("{:?}", deck);

        // Debug output should contain deck information
        assert!(debug_str.contains("Deck"));
        assert!(debug_str.contains("cards"));
    }

    #[test]
    fn test_deck_large_operations() {
        let mut deck = Deck::new();

        // Add many cards
        for suit in &[Suit::Heart, Suit::Diamond, Suit::Club, Suit::Spade] {
            for value in &Value::values() {
                deck.extend(vec![Card::new(*value, *suit)]);
            }
        }
        assert_eq!(deck.len(), 52);

        // Draw large number at once
        let drawn = deck.draw(26).expect("Should draw half the deck");
        assert_eq!(drawn.len(), 26);
        assert_eq!(deck.len(), 26);

        // Verify no duplicates in drawn cards
        for i in 0..drawn.len() {
            for j in i + 1..drawn.len() {
                assert_ne!(drawn[i], drawn[j]);
            }
        }
    }
}
