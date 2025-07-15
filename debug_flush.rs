use crate::card::{Card, Suit, Value};
use crate::game::Game;
use crate::hand::SelectHand;

#[test]
fn debug_flush_scoring() {
    let mut g = Game::default();
    
    // Create the same flush as in the failing test
    let cards_flush = vec\![
        Card::new(Value::Two, Suit::Heart),
        Card::new(Value::Four, Suit::Heart),
        Card::new(Value::Six, Suit::Heart),
        Card::new(Value::Eight, Suit::Heart),
        Card::new(Value::Ten, Suit::Heart),
    ];
    let flush_hand = SelectHand::new(cards_flush);
    
    let score = g.calc_score(flush_hand.best_hand().unwrap());
    println\!("Flush score: {}", score);
    
    // Also test the pair from the failing test
    let ace_heart = Card::new(Value::Ace, Suit::Heart);
    let ace_spade = Card::new(Value::Ace, Suit::Spade);
    let pair_hand = SelectHand::new(vec\![ace_heart, ace_spade]);
    let pair_score = g.calc_score(pair_hand.best_hand().unwrap());
    println\!("Pair score: {}", pair_score);
}
