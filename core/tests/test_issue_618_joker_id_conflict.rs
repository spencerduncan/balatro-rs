//! Regression test for issue #618: Fortune and FortuneTeller ID conflict resolution

use balatro_rs::joker::JokerId;
use balatro_rs::joker_factory::JokerFactory;

#[test]
fn test_fortune_teller_has_unique_id() {
    // Test that FortuneTeller can be created and has the correct ID
    let fortune_teller = JokerFactory::create(JokerId::FortuneTeller);

    assert!(
        fortune_teller.is_some(),
        "FortuneTeller should be creatable from factory"
    );

    assert_eq!(
        fortune_teller.unwrap().id(),
        JokerId::FortuneTeller,
        "FortuneTeller should have FortuneTeller ID"
    );
}

#[test]
fn test_fortune_teller_properties() {
    // Test that FortuneTeller has correct properties
    let fortune_teller = JokerFactory::create(JokerId::FortuneTeller).unwrap();

    assert_eq!(fortune_teller.name(), "Fortune Teller");
    assert_eq!(fortune_teller.description(), "+1 Mult per Tarot card used");
}

#[test]
fn test_fortune_teller_in_rarity_lists() {
    use balatro_rs::joker::JokerRarity;

    // Get the jokers by rarity
    let common_jokers = JokerFactory::get_by_rarity(JokerRarity::Common);

    // FortuneTeller is not assigned to a specific rarity but exists in get_all_implemented
    assert!(
        !common_jokers.contains(&JokerId::FortuneTeller),
        "FortuneTeller should not be in common jokers list (unassigned rarity)"
    );

    // But it should be in the all implemented list
    let all_implemented = JokerFactory::get_all_implemented();
    assert!(
        all_implemented.contains(&JokerId::FortuneTeller),
        "FortuneTeller should be in all implemented jokers list"
    );
}

#[test]
fn test_no_duplicate_joker_ids_in_all_implemented() {
    use std::collections::HashSet;

    let all_implemented = JokerFactory::get_all_implemented();
    let unique_ids: HashSet<_> = all_implemented.iter().collect();

    assert_eq!(
        all_implemented.len(),
        unique_ids.len(),
        "All implemented jokers should have unique IDs"
    );

    // Specifically check that FortuneTeller is present
    assert!(
        all_implemented.contains(&JokerId::FortuneTeller),
        "FortuneTeller should be in all implemented jokers"
    );
}
