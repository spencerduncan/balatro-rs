use balatro_rs::{card::{Card, Value, Suit}, hand::SelectHand};

// Add a simple test function that directly tests HandAnalysis
fn test_hand_analysis() {
    use balatro_rs::hand::HandAnalysis;
    
    let ad = Card::new(Value::Ace, Suit::Diamond);
    let ah = Card::new(Value::Ace, Suit::Heart);
    let cards = vec![ad, ad, ad, ah];
    
    println!("=== HandAnalysis Debug ===");
    let analysis = HandAnalysis::new(&cards);
    
    // We can't access private fields, so let's just check the rank detection
    let rank = analysis.detect_hand_rank();
    println!("Detected rank: {:?}", rank);
}

fn main() {
    test_hand_analysis();
    
    let ad = Card::new(Value::Ace, Suit::Diamond);
    let ah = Card::new(Value::Ace, Suit::Heart);
    
    // Test the failing case: [ad, ad, ad, ah] - same card repeated 3 times
    let cards = vec![ad, ad, ad, ah];
    let hand = SelectHand::new(cards);
    let result = hand.best_hand().unwrap();
    
    println!("Hand: [Ad, Ad, Ad, Ah] (3 duplicate cards + 1 different)");
    println!("Detected rank: {:?}", result.rank);
    println!("All cards length: {}", result.all.len());
    
    // Let's also test with distinct card objects
    let ad1 = Card::new(Value::Ace, Suit::Diamond);
    let ad2 = Card::new(Value::Ace, Suit::Diamond);  
    let ad3 = Card::new(Value::Ace, Suit::Diamond);
    let ah1 = Card::new(Value::Ace, Suit::Heart);
    
    let cards2 = vec![ad1, ad2, ad3, ah1];
    let hand2 = SelectHand::new(cards2);
    let result2 = hand2.best_hand().unwrap();
    
    println!("\nHand: [Ad1, Ad2, Ad3, Ah1] (4 distinct card objects)");
    println!("Detected rank: {:?}", result2.rank);
}