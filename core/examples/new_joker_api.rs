use balatro_rs::joker::{Joker, JokerEffect, JokerId, JokerRarity};
use balatro_rs::joker_factory::JokerFactory;
use balatro_rs::joker_impl::TheJoker;

fn main() {
    // Example 1: Using the new trait directly
    let joker = TheJoker;
    println!("Joker ID: {:?}", joker.id());
    println!("Joker Name: {}", joker.name());
    println!("Joker Description: {}", joker.description());
    println!("Joker Rarity: {:?}", joker.rarity());
    println!("Joker Cost: {}", joker.cost());

    // Example 2: Using the factory to create jokers by ID
    if let Some(boxed_joker) = JokerFactory::create(JokerId::Joker) {
        println!("\nCreated joker via factory:");
        println!("Name: {}", boxed_joker.name());
        println!("Description: {}", boxed_joker.description());
    }

    // Example 3: Getting jokers by rarity
    let common_jokers = JokerFactory::get_by_rarity(JokerRarity::Common);
    println!("\nCommon jokers available: {common_jokers:?}");

    // Example 4: Demonstrating JokerEffect
    let effect = JokerEffect::new().with_mult(4.0).with_chips(50.0);

    println!("\nJoker Effect:");
    println!("  Mult: {}", effect.mult);
    println!("  Chips: {}", effect.chips);
}
