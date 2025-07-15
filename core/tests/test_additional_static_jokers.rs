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
#[ignore] // Ignore until framework supports deck composition
fn test_steel_joker() {
    let joker = StaticJokerFactory::create_steel_joker();
    assert_eq!(joker.id(), JokerId::SteelJoker);
    assert_eq!(joker.name(), "Steel Joker");
    assert_eq!(
        joker.description(),
        "This Joker gains X0.25 Mult for each Steel Card in your full deck"
    );
    assert_eq!(joker.rarity(), JokerRarity::Uncommon);
    assert_eq!(joker.cost(), 6);
}
