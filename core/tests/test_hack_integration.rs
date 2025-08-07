use balatro_rs::joker::{JokerId, JokerRarity};
use balatro_rs::joker_factory::JokerFactory;

#[test]
fn test_hack_joker_creation() {
    let hack_joker = JokerFactory::create(JokerId::Hack);
    assert!(
        hack_joker.is_some(),
        "Hack joker should be creatable from factory"
    );

    let joker = hack_joker.unwrap();
    assert_eq!(joker.id(), JokerId::Hack, "Hack joker should have Hack ID");
    assert_eq!(joker.name(), "Hack", "Hack joker should have correct name");
    assert_eq!(
        joker.rarity(),
        JokerRarity::Uncommon,
        "Hack joker should be Uncommon rarity"
    );
}

#[test]
fn test_hack_joker_in_rarity_lists() {
    let uncommon_jokers = JokerFactory::get_by_rarity(JokerRarity::Uncommon);
    assert!(
        uncommon_jokers.contains(&JokerId::Hack),
        "Hack joker should be in Uncommon rarity list"
    );
}

#[test]
fn test_hack_joker_in_implemented_list() {
    let implemented = JokerFactory::get_all_implemented();
    assert!(
        implemented.contains(&JokerId::Hack),
        "Hack joker should be in implemented list"
    );
}

fn main() {
    test_hack_joker_creation();
    test_hack_joker_in_rarity_lists();
    test_hack_joker_in_implemented_list();
    println!("All Hack joker integration tests passed!");
}
