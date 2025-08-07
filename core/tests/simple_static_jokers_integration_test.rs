use balatro_rs::joker::JokerId;
use balatro_rs::joker_factory::JokerFactory;

/// Integration test to verify that the four new simple static jokers
/// can be created and work correctly through the main factory
#[test]
fn test_simple_static_jokers_integration() {
    // Test that all four jokers can be created through the main factory
    let smiley_face = JokerFactory::create(JokerId::Smiley).expect("Failed to create Smiley Face");
    let baron = JokerFactory::create(JokerId::BaronJoker).expect("Failed to create Baron");
    let raised_fist =
        JokerFactory::create(JokerId::RaisedFist).expect("Failed to create Raised Fist");
    let rough_gem = JokerFactory::create(JokerId::RoughGem).expect("Failed to create Rough Gem");

    // Verify basic properties
    assert_eq!(smiley_face.id(), JokerId::Smiley);
    assert_eq!(smiley_face.name(), "Smiley Face");

    assert_eq!(baron.id(), JokerId::BaronJoker);
    assert_eq!(baron.name(), "Baron");

    assert_eq!(raised_fist.id(), JokerId::RaisedFist);
    assert_eq!(raised_fist.name(), "Raised Fist");

    assert_eq!(rough_gem.id(), JokerId::RoughGem);
    assert_eq!(rough_gem.name(), "Rough Gem");
}

/// Test Baron joker properties
/// Baron: Each King held in hand gives X1.5 Mult
#[test]
fn test_baron_properties() {
    let baron = JokerFactory::create(JokerId::BaronJoker).expect("Failed to create Baron");

    // Verify properties match joker.json specifications
    assert_eq!(baron.id(), JokerId::BaronJoker);
    assert_eq!(baron.name(), "Baron");
    assert!(baron.description().contains("King"));
    assert!(baron.description().contains("held in hand"));
    assert!(baron.description().contains("X1.5"));

    // Check that it's a rare joker as expected
    assert_eq!(baron.rarity(), balatro_rs::joker::JokerRarity::Rare);
    assert_eq!(baron.cost(), 8);
}

/// Test Smiley Face joker properties
/// Smiley Face: Played face cards give +4 Mult when scored
#[test]
fn test_smiley_face_properties() {
    let smiley_face = JokerFactory::create(JokerId::Smiley).expect("Failed to create Smiley Face");

    // Verify properties match joker.json specifications
    assert_eq!(smiley_face.id(), JokerId::Smiley);
    assert_eq!(smiley_face.name(), "Smiley Face");
    assert!(smiley_face.description().contains("face"));
    assert!(smiley_face.description().contains("Mult"));
    assert!(smiley_face.description().contains("scored"));

    // Check that it's a common joker as expected
    assert_eq!(smiley_face.rarity(), balatro_rs::joker::JokerRarity::Common);
    assert_eq!(smiley_face.cost(), 3);
}

/// Test Rough Gem joker properties
/// Rough Gem: Played cards with Diamond suit earn $1 when scored
#[test]
fn test_rough_gem_properties() {
    let rough_gem = JokerFactory::create(JokerId::RoughGem).expect("Failed to create Rough Gem");

    // Verify properties match joker.json specifications
    assert_eq!(rough_gem.id(), JokerId::RoughGem);
    assert_eq!(rough_gem.name(), "Rough Gem");
    assert!(rough_gem.description().contains("Diamond"));
    assert!(rough_gem.description().contains("$1"));
    assert!(rough_gem.description().contains("scored"));

    // Check that it's a common joker as expected
    assert_eq!(rough_gem.rarity(), balatro_rs::joker::JokerRarity::Common);
    assert_eq!(rough_gem.cost(), 4);
}

/// Test Raised Fist joker properties
/// Raised Fist: Adds double the rank of lowest ranked card held in hand to Mult
#[test]
fn test_raised_fist_properties() {
    let raised_fist =
        JokerFactory::create(JokerId::RaisedFist).expect("Failed to create Raised Fist");

    // Verify properties match joker.json specifications
    assert_eq!(raised_fist.id(), JokerId::RaisedFist);
    assert_eq!(raised_fist.name(), "Raised Fist");
    assert!(raised_fist.description().contains("double"));
    assert!(raised_fist.description().contains("lowest"));
    assert!(raised_fist.description().contains("held in hand"));

    // Check that it's a common joker as expected
    assert_eq!(raised_fist.rarity(), balatro_rs::joker::JokerRarity::Common);
    assert_eq!(raised_fist.cost(), 3);
}

/// Test all four jokers can be created without conflicts
#[test]
fn test_multiple_simple_static_jokers() {
    // Create all four jokers
    let smiley_face = JokerFactory::create(JokerId::Smiley).expect("Failed to create Smiley Face");
    let baron = JokerFactory::create(JokerId::BaronJoker).expect("Failed to create Baron");
    let raised_fist =
        JokerFactory::create(JokerId::RaisedFist).expect("Failed to create Raised Fist");
    let rough_gem = JokerFactory::create(JokerId::RoughGem).expect("Failed to create Rough Gem");

    // Collect in an array to test coexistence
    let jokers = [smiley_face, baron, raised_fist, rough_gem];

    // Verify all are present and have correct IDs
    assert_eq!(jokers.len(), 4);
    assert_eq!(jokers[0].id(), JokerId::Smiley);
    assert_eq!(jokers[1].id(), JokerId::BaronJoker);
    assert_eq!(jokers[2].id(), JokerId::RaisedFist);
    assert_eq!(jokers[3].id(), JokerId::RoughGem);

    // Verify no naming conflicts
    let names: Vec<&str> = jokers.iter().map(|j| j.name()).collect();
    assert_eq!(names.len(), 4);
    assert!(names.contains(&"Smiley Face"));
    assert!(names.contains(&"Baron"));
    assert!(names.contains(&"Raised Fist"));
    assert!(names.contains(&"Rough Gem"));
}

/// Test parameter loading from joker.json (if available)
#[test]
fn test_parameter_loading() {
    // Test that jokers can be created even if joker.json parameters are not available
    // The factory methods should have fallback values

    let smiley_face = JokerFactory::create(JokerId::Smiley)
        .expect("Smiley Face should be created with fallback parameters");
    let baron = JokerFactory::create(JokerId::BaronJoker)
        .expect("Baron should be created with fallback parameters");
    let raised_fist = JokerFactory::create(JokerId::RaisedFist)
        .expect("Raised Fist should be created with fallback parameters");
    let rough_gem = JokerFactory::create(JokerId::RoughGem)
        .expect("Rough Gem should be created with fallback parameters");

    // Verify they have reasonable properties
    assert!(smiley_face.cost() > 0);
    assert!(baron.cost() > 0);
    assert!(raised_fist.cost() > 0);
    assert!(rough_gem.cost() > 0);

    // Verify descriptions are not empty
    assert!(!smiley_face.description().is_empty());
    assert!(!baron.description().is_empty());
    assert!(!raised_fist.description().is_empty());
    assert!(!rough_gem.description().is_empty());
}
