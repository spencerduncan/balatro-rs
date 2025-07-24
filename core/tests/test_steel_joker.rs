use balatro_rs::joker::{Joker, JokerId, JokerRarity};
use balatro_rs::joker_factory::JokerFactory;

#[test]
fn test_steel_joker_basic() {
    let joker = JokerFactory::create(JokerId::SteelJoker).expect("Steel Joker should be implemented");

    // Verify basic properties
    assert_eq!(joker.id(), JokerId::SteelJoker);
    assert_eq!(joker.name(), "Steel Joker");
    assert_eq!(
        joker.description(),
        "Each Steel Card in your full deck multiplies this Joker by X1.25"
    );
    assert_eq!(joker.rarity(), JokerRarity::Uncommon);
    assert_eq!(joker.cost(), 6);

    // Since we can't easily test the deck composition behavior from here
    // without access to test utils, we're just verifying the joker exists
    // and has the correct basic properties.
    // The actual deck composition logic is tested internally in the crate.
}