// Tests for additional static jokers (Issue #90)
// Note: Runner is implemented as RunnerJoker in joker_impl.rs, not as a static joker
// This file tests 9 jokers: 5 fully implemented + 4 placeholders

use balatro_rs::joker::{JokerId, JokerRarity};
use balatro_rs::static_joker_factory::StaticJokerFactory;

#[test]
fn test_red_card_joker() {
    let joker = StaticJokerFactory::create_red_card();
    assert_eq!(joker.id(), JokerId::RedCard);
    assert_eq!(joker.name(), "Red Card");
    assert_eq!(
        joker.description(),
        "Red cards (Hearts and Diamonds) give +3 Mult when scored"
    );
    assert_eq!(joker.rarity(), JokerRarity::Uncommon);
    assert_eq!(joker.cost(), 6);
}

#[test]
fn test_blue_joker() {
    let joker = StaticJokerFactory::create_blue_joker();
    assert_eq!(joker.id(), JokerId::BlueJoker);
    assert_eq!(joker.name(), "Blue Joker");
    assert_eq!(
        joker.description(),
        "Black cards (Clubs and Spades) give +3 Mult when scored"
    );
    assert_eq!(joker.rarity(), JokerRarity::Uncommon);
    assert_eq!(joker.cost(), 6);
}

#[test]
fn test_faceless_joker() {
    let joker = StaticJokerFactory::create_faceless_joker();
    assert_eq!(joker.id(), JokerId::FacelessJoker);
    assert_eq!(joker.name(), "Faceless Joker");
    assert_eq!(
        joker.description(),
        "Face cards (Jack, Queen, King) give +5 Mult when scored"
    );
    assert_eq!(joker.rarity(), JokerRarity::Common);
    assert_eq!(joker.cost(), 3);
}

#[test]
fn test_square_joker() {
    let joker = StaticJokerFactory::create_square();
    assert_eq!(joker.id(), JokerId::Square);
    assert_eq!(joker.name(), "Square");
    assert_eq!(
        joker.description(),
        "Number cards (2, 3, 4, 5, 6, 7, 8, 9, 10) give +4 Chips when scored"
    );
    assert_eq!(joker.rarity(), JokerRarity::Common);
    assert_eq!(joker.cost(), 3);
}

#[test]
fn test_walkie_joker() {
    let joker = StaticJokerFactory::create_walkie();
    assert_eq!(joker.id(), JokerId::Walkie);
    assert_eq!(joker.name(), "Walkie");
    assert_eq!(
        joker.description(),
        "+10 Chips and +4 Mult if played hand contains a Straight"
    );
    assert_eq!(joker.rarity(), JokerRarity::Common);
    assert_eq!(joker.cost(), 3);
}

// Note: Runner is implemented as RunnerJoker in joker_impl.rs, not as a static joker

// Tests for jokers that need framework extensions
#[test]
#[ignore] // Ignore until framework supports hand size conditions
fn test_half_joker() {
    let joker = StaticJokerFactory::create_half_joker();
    assert_eq!(joker.id(), JokerId::HalfJoker);
    assert_eq!(joker.name(), "Half Joker");
    assert_eq!(
        joker.description(),
        "+20 Mult if played hand has 4 or fewer cards"
    );
    assert_eq!(joker.rarity(), JokerRarity::Common);
    assert_eq!(joker.cost(), 3);
}

#[test]
#[ignore] // Ignore until framework supports discard count
fn test_banner_joker() {
    let joker = StaticJokerFactory::create_banner();
    assert_eq!(joker.id(), JokerId::Banner);
    assert_eq!(joker.name(), "Banner");
    assert_eq!(joker.description(), "+30 Chips for each remaining discard");
    assert_eq!(joker.rarity(), JokerRarity::Common);
    assert_eq!(joker.cost(), 3);
}

#[test]
#[ignore] // Ignore until framework supports joker interactions
fn test_abstract_joker() {
    let joker = StaticJokerFactory::create_abstract_joker();
    assert_eq!(joker.id(), JokerId::AbstractJoker);
    assert_eq!(joker.name(), "Abstract Joker");
    assert_eq!(joker.description(), "All Jokers give X0.25 more Mult");
    assert_eq!(joker.rarity(), JokerRarity::Common);
    assert_eq!(joker.cost(), 3);
}

#[test]
fn test_steel_joker() {
    use crate::joker::test_utils::TestContextBuilder;
    use crate::hand::SelectHand;

    let joker = StaticJokerFactory::create_steel_joker();
    
    // Test basic properties
    assert_eq!(joker.id(), JokerId::SteelJoker);
    assert_eq!(joker.name(), "Steel Joker");
    assert_eq!(
        joker.description(),
        "Each Steel Card in your full deck multiplies this Joker by X1.25"
    );
    assert_eq!(joker.rarity(), JokerRarity::Uncommon);
    assert_eq!(joker.cost(), 6);

    // Test deck composition behavior with 0 Steel cards
    let mut context = TestContextBuilder::new()
        .with_steel_cards_in_deck(0)
        .build();
    let hand = SelectHand::new(vec![]);
    let effect = joker.on_hand_played(&mut context, &hand);
    // With 0 Steel cards: 1.25^0 = X1.0
    assert_eq!(effect.mult_multiplier, 1.0);

    // Test deck composition behavior with 1 Steel card
    let mut context = TestContextBuilder::new()
        .with_steel_cards_in_deck(1)
        .build();
    let effect = joker.on_hand_played(&mut context, &hand);
    // With 1 Steel card: 1.25^1 = X1.25
    assert_eq!(effect.mult_multiplier, 1.25);

    // Test deck composition behavior with 4 Steel cards
    let mut context = TestContextBuilder::new()
        .with_steel_cards_in_deck(4)
        .build();
    let effect = joker.on_hand_played(&mut context, &hand);
    // With 4 Steel cards: 1.25^4 = X2.44140625
    assert!((effect.mult_multiplier - 2.44140625).abs() < 0.0001);

    // Test deck composition behavior with 8 Steel cards
    let mut context = TestContextBuilder::new()
        .with_steel_cards_in_deck(8)
        .build();
    let effect = joker.on_hand_played(&mut context, &hand);
    // With 8 Steel cards: 1.25^8 = X5.9604644775390625
    assert!((effect.mult_multiplier - 5.9604644775390625).abs() < 0.0001);
}
