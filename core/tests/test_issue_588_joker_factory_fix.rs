use balatro_rs::{
    joker::{JokerId, JokerRarity},
    joker_factory::JokerFactory,
};

/// Test that Fortune Teller Joker is correctly created as a scaling joker
/// that gains +1 Mult per Tarot card used
#[test]
fn test_fortune_teller_joker_correctly_created() {
    // Initialize all systems before running the test to avoid factory race conditions
    balatro_rs::initialize().expect("Failed to initialize core systems");

    let fortune = JokerFactory::create(JokerId::Fortune);
    assert!(fortune.is_some());

    let joker = fortune.unwrap();
    assert_eq!(joker.id(), JokerId::Fortune);
    assert_eq!(joker.name(), "Fortune Teller");
    assert_eq!(joker.description(), "+1 Mult per Tarot card used");
    assert_eq!(joker.rarity(), JokerRarity::Rare);

    // Verify it's not the MysteryJoker (which has a different description)
    assert_ne!(joker.description(), "Random effect each hand");
}

/// Test that Red Card Joker is correctly created as a scaling joker
/// that gains +3 Mult per pack skipped
#[test]
fn test_red_card_joker_correctly_created() {
    let red_card = JokerFactory::create(JokerId::Reserved6);
    assert!(red_card.is_some());

    let joker = red_card.unwrap();
    assert_eq!(joker.id(), JokerId::Reserved6);
    assert_eq!(joker.name(), "Red Card");
    assert_eq!(joker.description(), "+3 Mult per pack skipped");
    assert_eq!(joker.rarity(), JokerRarity::Common);

    // Verify it's not the static joker that gives "+3 Mult when red cards are scored"
    assert_ne!(
        joker.description(),
        "Red cards (Hearts and Diamonds) give +3 Mult when scored"
    );
}

/// Test that Steel Joker is correctly created as a scaling joker
/// that gains +0.2x Mult per card destroyed
#[test]
fn test_steel_joker_correctly_created() {
    // Initialize all systems before running the test to avoid factory race conditions
    balatro_rs::initialize().expect("Failed to initialize core systems");

    let steel = JokerFactory::create(JokerId::SteelJoker);
    assert!(steel.is_some());

    let joker = steel.unwrap();
    assert_eq!(joker.id(), JokerId::SteelJoker);
    assert_eq!(joker.name(), "Steel Joker");
    assert_eq!(joker.description(), "+0.2x Mult per card destroyed");
    assert_eq!(joker.rarity(), JokerRarity::Uncommon);

    // Verify it's not the placeholder that gives X1.0 Mult (does nothing)
    assert_ne!(
        joker.description(),
        "This Joker gains X0.25 Mult for each Steel Card in your full deck"
    );
}

/// Integration test: Verify Fortune Teller can be created
#[test]
fn test_fortune_teller_creation() {
    // Initialize all systems before running the test to avoid factory race conditions
    balatro_rs::initialize().expect("Failed to initialize core systems");

    // Verify we can create Fortune Teller without crashes
    let fortune = JokerFactory::create(JokerId::Fortune).unwrap();
    assert_eq!(fortune.name(), "Fortune Teller");
    assert_eq!(fortune.description(), "+1 Mult per Tarot card used");
}

/// Integration test: Verify Red Card can be created
#[test]
fn test_red_card_creation() {
    // Initialize all systems before running the test to avoid factory race conditions
    balatro_rs::initialize().expect("Failed to initialize core systems");

    // Verify Red Card can be created and used without crashes
    let red_card = JokerFactory::create(JokerId::Reserved6).unwrap();
    assert_eq!(red_card.name(), "Red Card");
    assert_eq!(red_card.description(), "+3 Mult per pack skipped");
}

/// Integration test: Verify Steel Joker can be created
#[test]
fn test_steel_joker_creation() {
    // Initialize all systems before running the test to avoid factory race conditions
    balatro_rs::initialize().expect("Failed to initialize core systems");

    // Verify Steel Joker can be created and used without crashes
    let steel = JokerFactory::create(JokerId::SteelJoker).unwrap();
    assert_eq!(steel.name(), "Steel Joker");
    assert_eq!(steel.description(), "+0.2x Mult per card destroyed");
}

/// Test that all three jokers appear in the correct rarity lists
#[test]
fn test_jokers_in_rarity_lists() {
    // Initialize all systems before running the test to avoid factory race conditions
    balatro_rs::initialize().expect("Failed to initialize core systems");

    // Fortune should be in Rare (based on the MysteryJoker's original rarity)
    let rare_jokers = JokerFactory::get_by_rarity(JokerRarity::Rare);
    assert!(rare_jokers.contains(&JokerId::Fortune));

    // Red Card (Reserved6) should be in Common
    let common_jokers = JokerFactory::get_by_rarity(JokerRarity::Common);
    assert!(common_jokers.contains(&JokerId::Reserved6));

    // Steel Joker should be in Uncommon
    let uncommon_jokers = JokerFactory::get_by_rarity(JokerRarity::Uncommon);
    assert!(uncommon_jokers.contains(&JokerId::SteelJoker));
}

/// Test that all three jokers are in the implemented list
#[test]
fn test_jokers_in_implemented_list() {
    // Initialize all systems before running the test to avoid factory race conditions
    balatro_rs::initialize().expect("Failed to initialize core systems");

    let implemented = JokerFactory::get_all_implemented();

    assert!(implemented.contains(&JokerId::Fortune));
    assert!(implemented.contains(&JokerId::Reserved6)); // Red Card
    assert!(implemented.contains(&JokerId::SteelJoker));
}

/// Regression test: Ensure other jokers still work correctly
#[test]
fn test_other_jokers_not_affected() {
    // Initialize all systems before running the test to avoid factory race conditions
    balatro_rs::initialize().expect("Failed to initialize core systems");

    // Test a few other jokers to ensure we didn't break anything
    let joker = JokerFactory::create(JokerId::Joker).unwrap();
    assert_eq!(joker.name(), "Joker");

    let greedy = JokerFactory::create(JokerId::GreedyJoker).unwrap();
    assert_eq!(greedy.name(), "Greedy Joker");

    let blue = JokerFactory::create(JokerId::BlueJoker).unwrap();
    assert_eq!(blue.name(), "Blue Joker");
}
