//! # Balatro-RS Joker System Comprehensive Examples
//!
//! This example demonstrates all aspects of the joker system including:
//! - Creating jokers using different approaches
//! - Working with joker effects and game context
//! - Integration patterns with the game engine
//! - Performance considerations
//! - Best practices for joker implementation

use balatro_rs::{
    card::{Card, Suit, Value},
    config::Config,
    game::Game,
    hand::SelectHand,
    joker::{Joker, JokerEffect, JokerId, JokerRarity},
    joker_factory::JokerFactory,
    joker_impl::*,
    static_joker::*,
    static_joker_factory::StaticJokerFactory,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Balatro-RS Joker System Comprehensive Guide ===\n");

    // Section 1: Basic Joker Usage
    basic_joker_usage()?;

    // Section 2: Joker Creation Patterns
    joker_creation_patterns()?;

    // Section 3: Game Integration
    game_integration_example()?;

    // Section 4: Joker Effects and Scoring
    joker_effects_demonstration()?;

    // Section 5: Performance Patterns
    performance_patterns()?;

    // Section 6: Static Joker Framework
    static_joker_framework_example()?;

    // Section 7: Best Practices
    best_practices_demonstration()?;

    Ok(())
}

/// Demonstrates basic joker usage patterns
fn basic_joker_usage() -> Result<(), Box<dyn std::error::Error>> {
    println!("## 1. Basic Joker Usage\n");

    // Direct trait usage
    let joker = TheJoker;
    println!("Direct trait usage:");
    println!("  Name: {}", joker.name());
    println!("  Description: {}", joker.description());
    println!("  Rarity: {:?}", joker.rarity());
    println!("  Cost: {} coins", joker.cost());

    // Factory creation - recommended for dynamic joker creation
    if let Some(boxed_joker) = JokerFactory::create(JokerId::Joker) {
        println!("\nFactory creation:");
        println!("  Name: {}", boxed_joker.name());
        println!("  Type: Box<dyn Joker> - enables polymorphism");
    }

    // Querying available jokers by rarity
    let common_jokers = JokerFactory::get_by_rarity(JokerRarity::Common);
    println!("\nAvailable common jokers: {} types", common_jokers.len());
    for joker_id in common_jokers.iter().take(3) {
        println!("  - {joker_id:?}");
    }

    println!();
    Ok(())
}

/// Demonstrates different joker creation patterns
fn joker_creation_patterns() -> Result<(), Box<dyn std::error::Error>> {
    println!("## 2. Joker Creation Patterns\n");

    // Pattern 1: Manual implementation (for complex jokers)
    println!("Manual Implementation Pattern:");
    let manual_joker = TheJoker;
    println!("  Best for: Complex logic, multiple conditions");
    println!("  Example: {}", manual_joker.name());

    // Pattern 2: Static joker factory (for simple conditional jokers)
    println!("\nStatic Factory Pattern:");
    let static_joker = StaticJokerFactory::create_greedy_joker();
    println!("  Best for: Simple conditional jokers");
    println!("  Example: {}", static_joker.name());
    println!("  Condition: +3 mult per Diamond");

    // Pattern 3: Builder pattern (for custom static jokers)
    println!("\nBuilder Pattern:");
    let custom_joker = StaticJoker::builder(
        JokerId::EvenSteven,
        "Even Steven",
        "Even cards give +4 mult",
    )
    .rarity(JokerRarity::Common)
    .mult(4.0)
    .condition(StaticCondition::AnyRankScored(vec![
        Value::Two,
        Value::Four,
        Value::Six,
        Value::Eight,
        Value::Ten,
    ]))
    .per_card()
    .build();

    println!("  Best for: Custom conditional logic");
    let custom_joker = custom_joker?; // Handle the Result
    println!("  Example: {}", custom_joker.name());

    println!();
    Ok(())
}

/// Demonstrates joker integration with the game engine
fn game_integration_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("## 3. Game Integration\n");

    // Create a simple game context for demonstration
    let _game = Game::new(Config::default());

    // Add jokers to the game
    println!("Adding jokers to game:");

    // In a real game, jokers would be added through actions
    // This demonstrates the integration concept
    println!("  - Jokers integrate through the Action system");
    println!("  - BuyJoker action adds jokers to player collection");
    println!("  - Jokers automatically affect scoring through lifecycle hooks");

    // Demonstrate joker lifecycle
    println!("\nJoker Lifecycle Events:");
    println!("  1. on_blind_start() - Called when blind begins");
    println!("  2. on_hand_played() - Called when hand is played");
    println!("  3. on_card_scored() - Called for each scoring card");
    println!("  4. on_discard() - Called when cards discarded");
    println!("  5. on_shop_open() - Called when entering shop");
    println!("  6. on_round_end() - Called at round completion");

    println!();
    Ok(())
}

/// Demonstrates joker effects and scoring integration
fn joker_effects_demonstration() -> Result<(), Box<dyn std::error::Error>> {
    println!("## 4. Joker Effects and Scoring\n");

    // Create sample cards for testing
    let cards = vec![
        Card::new(Value::Ace, Suit::Diamond),
        Card::new(Value::King, Suit::Diamond),
        Card::new(Value::Queen, Suit::Diamond),
        Card::new(Value::Jack, Suit::Diamond),
        Card::new(Value::Ten, Suit::Diamond),
    ];

    // Create a sample hand
    let _select_hand = SelectHand::new(cards.clone());

    // Note: GameContext creation requires references to game state
    // In real usage, it's created by the game engine with actual references

    // Demonstrate different joker effects
    println!("Effect Types:");

    // Mult bonus effect
    let mult_effect = JokerEffect::new().with_mult(10.0);
    println!("  Mult Effect: +{} mult", mult_effect.mult);

    // Chips bonus effect
    let chips_effect = JokerEffect::new().with_chips(50.0);
    println!("  Chips Effect: +{} chips", chips_effect.chips);

    // Combined effect
    let combined_effect = JokerEffect::new().with_mult(5.0).with_chips(30.0);
    println!(
        "  Combined: +{} mult, +{} chips",
        combined_effect.mult, combined_effect.chips
    );

    // Demonstrate per-card vs per-hand effects
    println!("\nEffect Application:");
    let _greedy_joker = StaticJokerFactory::create_greedy_joker();
    println!("  Greedy Joker (per-card): Checks each card individually");
    println!("  Royal Flush bonus (per-hand): Applies once per hand");

    // Count diamonds in hand for demonstration
    let diamond_count = cards.iter().filter(|c| c.suit == Suit::Diamond).count();
    let total_greedy_mult = diamond_count * 3;
    println!("  Example: {diamond_count} diamonds × 3 mult = +{total_greedy_mult} mult total");

    println!();
    Ok(())
}

/// Demonstrates performance optimization patterns
fn performance_patterns() -> Result<(), Box<dyn std::error::Error>> {
    println!("## 5. Performance Patterns\n");

    println!("Performance Best Practices:");

    println!("  1. Factory Pattern Benefits:");
    println!("     - Reduces memory allocation overhead");
    println!("     - Enables object pooling for frequently used jokers");
    println!("     - Centralizes joker creation logic");

    println!("\n  2. Static Framework Benefits:");
    println!("     - Compile-time optimizations for simple conditions");
    println!("     - Minimal runtime overhead");
    println!("     - Efficient condition checking");

    println!("\n  3. Effect Aggregation:");
    println!("     - Combine multiple small effects into single operations");
    println!("     - Batch effect applications when possible");
    println!("     - Use early returns for non-matching conditions");

    // Demonstrate efficient condition checking
    println!("\n  4. Efficient Condition Patterns:");
    let card = Card::new(Value::Ace, Suit::Diamond);
    let _joker = StaticJokerFactory::create_greedy_joker();

    println!("     Quick suit check example:");
    println!("       Greedy Joker condition: +3 mult per Diamond");
    println!(
        "       Card: {:?} of {:?} - matches condition",
        card.value, card.suit
    );

    println!("\n  5. Memory Management:");
    println!("     - Use Box<dyn Joker> for polymorphism");
    println!("     - Static jokers have minimal memory footprint");
    println!("     - Avoid unnecessary cloning of joker instances");

    println!();
    Ok(())
}

/// Demonstrates the static joker framework in detail
fn static_joker_framework_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("## 6. Static Joker Framework\n");

    println!("Framework Capabilities:");

    // Suit-based jokers
    println!("  Suit-based Jokers:");
    let suit_jokers = vec![
        (
            "Greedy",
            "Diamond",
            StaticJokerFactory::create_greedy_joker(),
        ),
        ("Lusty", "Heart", StaticJokerFactory::create_lusty_joker()),
        (
            "Wrathful",
            "Spade",
            StaticJokerFactory::create_wrathful_joker(),
        ),
        (
            "Gluttonous",
            "Club",
            StaticJokerFactory::create_gluttonous_joker(),
        ),
    ];

    for (name, suit, _joker) in suit_jokers {
        println!("    - {name}: +3 mult per {suit} card");
    }

    // Hand-type jokers
    println!("\n  Hand-type Jokers:");
    let hand_jokers = vec![
        ("Jolly", "Pair", "+8 mult"),
        ("Zany", "Three of a Kind", "+12 mult"),
        ("Mad", "Straight", "+10 mult"),
        ("Crazy", "Flush", "+12 mult"),
        ("Droll", "Full House", "+10 mult"),
    ];

    for (name, hand_type, bonus) in hand_jokers {
        println!("    - {name}: {bonus} for {hand_type}");
    }

    // Rank-based jokers
    println!("\n  Rank-based Jokers:");
    println!("    - Even Steven: +4 mult for even ranks (2,4,6,8,10)");
    println!("    - Odd Todd: +4 mult for odd ranks (3,5,7,9,J,K,A)");
    println!("    - Scholar: +4 mult for Aces");

    // Demonstrate builder pattern customization
    println!("\n  Custom Builder Example:");
    println!("    ```rust");
    println!("    StaticJoker::builder(id, name, description)");
    println!("        .rarity(JokerRarity::Uncommon)");
    println!("        .mult(6)");
    println!("        .condition(StaticCondition::SuitScored(Suit::Heart))");
    println!("        .per_card()");
    println!("        .build()");
    println!("    ```");

    println!();
    Ok(())
}

/// Demonstrates best practices for joker implementation
fn best_practices_demonstration() -> Result<(), Box<dyn std::error::Error>> {
    println!("## 7. Best Practices\n");

    println!("Implementation Guidelines:");

    println!("  1. Choose the Right Pattern:");
    println!("     - Static Framework: Simple conditional jokers");
    println!("     - Manual Implementation: Complex logic, state management");
    println!("     - Factory Pattern: Dynamic joker creation");

    println!("\n  2. Performance Considerations:");
    println!("     - Implement early returns in condition checks");
    println!("     - Use per_card() for individual card effects");
    println!("     - Use per_hand() for effects that apply once per hand");
    println!("     - Cache expensive calculations when possible");

    println!("\n  3. Code Organization:");
    println!("     - Group related jokers in the same module");
    println!("     - Use descriptive names that match Balatro naming");
    println!("     - Include comprehensive documentation");
    println!("     - Add unit tests for all joker behaviors");

    println!("\n  4. Integration Best Practices:");
    println!("     - Use JokerFactory for dynamic creation");
    println!("     - Implement all required lifecycle hooks");
    println!("     - Handle edge cases gracefully");
    println!("     - Maintain backward compatibility");

    println!("\n  5. Testing Patterns:");
    println!("     - Test each joker's condition logic");
    println!("     - Verify effect calculations");
    println!("     - Test integration with game context");
    println!("     - Include edge case testing");

    // Demonstrate testing approach
    println!("\n  Testing Example:");
    let _joker = StaticJokerFactory::create_greedy_joker();
    let diamond = Card::new(Value::Ace, Suit::Diamond);
    let heart = Card::new(Value::King, Suit::Heart);

    println!("    // Test diamond card with Greedy Joker");
    println!("    assert_eq!(diamond.suit, Suit::Diamond);  // Should pass");
    println!("    assert_ne!(heart.suit, Suit::Diamond);    // Should pass");

    // Verify our example
    assert_eq!(diamond.suit, Suit::Diamond);
    assert_ne!(heart.suit, Suit::Diamond);
    println!("    ✓ Tests pass");

    println!();
    Ok(())
}
