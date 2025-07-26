/// Example demonstrating card targeting functionality
///
/// This example shows how to:
/// 1. Create CardTarget instances
/// 2. Validate targets against game state
/// 3. Use different card collections
use balatro_rs::{
    card::{Card, Suit, Value},
    config::Config,
    consumables::{CardCollection, CardTarget, Target},
    game::Game,
    rank::HandRank,
};

fn main() {
    println!("Card Targeting Demo");
    println!("==================");

    // Create a new game
    let mut game = Game::new(Config::new());
    println!("Created game with default config");

    // Add some cards to the discard pile for testing
    for i in 0..5 {
        let card = Card::new(
            match i % 4 {
                0 => Value::Ace,
                1 => Value::Two,
                2 => Value::King,
                _ => Value::Queen,
            },
            match i % 4 {
                0 => Suit::Heart,
                1 => Suit::Diamond,
                2 => Suit::Club,
                _ => Suit::Spade,
            },
        );
        game.discarded.push(card);
    }
    println!("Added {} cards to discard pile", game.discarded.len());

    // Test 1: Create and validate a CardTarget for discard pile
    println!("\nTest 1: Basic CardTarget creation and validation");
    let target = CardTarget::new(CardCollection::DiscardPile, vec![0, 1, 2]);
    println!("Created target for discard pile with indices [0, 1, 2]");

    match target.validate(&game) {
        Ok(()) => println!("✓ Target validation successful"),
        Err(e) => println!("✗ Target validation failed: {e}"),
    }

    // Test 2: Test out of bounds validation
    println!("\nTest 2: Out of bounds validation");
    let invalid_target = CardTarget::single_card(CardCollection::DiscardPile, 10);
    println!("Created target for discard pile with index 10 (should be invalid)");

    match invalid_target.validate(&game) {
        Ok(()) => println!("✗ Unexpected success - target should be invalid"),
        Err(e) => println!("✓ Expected validation error: {e}"),
    }

    // Test 3: Test duplicate indices
    println!("\nTest 3: Duplicate indices validation");
    let duplicate_target = CardTarget::new(CardCollection::DiscardPile, vec![0, 1, 0]);
    println!("Created target with duplicate indices [0, 1, 0]");

    match duplicate_target.validate(&game) {
        Ok(()) => println!("✗ Unexpected success - target should be invalid"),
        Err(e) => println!("✓ Expected validation error: {e}"),
    }

    // Test 4: Test Target helper methods
    println!("\nTest 4: Target helper methods");
    let hand_target = Target::cards_in_hand(vec![0]);
    println!("Created hand target using helper method");
    println!("Target type: {:?}", hand_target.target_type());
    println!("Card count: {}", hand_target.card_count());

    let deck_target = Target::cards_in_deck(vec![0, 1]);
    println!("Created deck target using helper method");
    println!("Target type: {:?}", deck_target.target_type());

    // Test 5: Test different target types
    println!("\nTest 5: Different target types");
    let targets = vec![
        Target::None,
        Target::cards_in_discard(vec![0]),
        Target::HandType(HandRank::OnePair),
        Target::Joker(0),
        Target::Deck,
    ];

    for (i, target) in targets.iter().enumerate() {
        println!("Target {}: {:?}", i + 1, target.target_type());
        let is_valid = target.is_valid(&game);
        println!("  Valid: {is_valid}");
        if !is_valid {
            if let Err(e) = target.validate(&game) {
                println!("  Error: {e}");
            }
        }
    }

    // Test 6: Test CardCollection display
    println!("\nTest 6: CardCollection display");
    let collections = vec![
        CardCollection::Hand,
        CardCollection::Deck,
        CardCollection::DiscardPile,
        CardCollection::PlayedCards,
    ];

    for collection in collections {
        println!("Collection: {collection}");
    }

    // Test 7: Get cards from discard pile (the only implemented collection)
    println!("\nTest 7: Getting cards from discard pile");
    let target = CardTarget::new(CardCollection::DiscardPile, vec![0, 2]);
    match target.get_cards(&game) {
        Ok(cards) => {
            println!(
                "✓ Successfully retrieved {} cards from discard pile",
                cards.len()
            );
            for (i, card) in cards.iter().enumerate() {
                println!("  Card {}: {:?} of {:?}", i, card.value, card.suit);
            }
        }
        Err(e) => println!("✗ Failed to get cards: {e}"),
    }

    println!("\nDemo completed successfully!");
}
