use balatro_rs::{card::{Card, Value, Suit}, hand::SelectHand};

fn main() {
    let king = Card::new(Value::King, Suit::Diamond);
    let ace = Card::new(Value::Ace, Suit::Heart);
    
    // Test the failing case: [king, king, ace]
    let cards = vec![king, king, ace];
    let hand = SelectHand::new(cards);
    let result = hand.best_hand().unwrap();
    
    println!("Hand: [Kd, Kd, Ah]");
    println!("Detected rank: {:?}", result.rank);
    println!("Selected hand length: {}", result.hand.len());
    println!("All cards length: {}", result.all.len());
}