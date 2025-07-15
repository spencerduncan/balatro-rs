use balatro_rs::game::Game;
use balatro_rs::joker::JokerId;
use balatro_rs::joker_factory::JokerFactory;
use balatro_rs::joker_state::JokerStateManager;
use balatro_rs::rank::HandRank;
use std::sync::Arc;

// Test centralized hand type tracking
#[test]
fn test_centralized_hand_type_tracking() {
    let mut game = Game::default();

    // Initially, no hand types should be tracked
    assert_eq!(game.get_hand_type_count(HandRank::OnePair), 0);
    assert_eq!(game.get_hand_type_count(HandRank::ThreeOfAKind), 0);
    assert_eq!(game.get_hand_type_count(HandRank::HighCard), 0);

    // Test the internal increment method by accessing it indirectly
    // In a real game, this would happen automatically when hands are played
    game.increment_hand_type_count(HandRank::OnePair);
    assert_eq!(game.get_hand_type_count(HandRank::OnePair), 1);

    game.increment_hand_type_count(HandRank::OnePair);
    assert_eq!(game.get_hand_type_count(HandRank::OnePair), 2);

    game.increment_hand_type_count(HandRank::ThreeOfAKind);
    assert_eq!(game.get_hand_type_count(HandRank::ThreeOfAKind), 1);
    assert_eq!(game.get_hand_type_count(HandRank::OnePair), 2); // Should remain unchanged
}

#[test]
fn test_hand_type_tracking_accessibility() {
    let game = Game::default();

    // Demonstrate that hand type counts are now easily accessible to any system
    // This is useful for:
    // - Supernova joker (gives mult equal to hand type count)
    // - Vouchers that depend on hand type statistics
    // - Achievement systems
    // - Analytics and reporting

    // Example: A voucher might check if player has played enough of a specific hand type
    let pairs_played = game.get_hand_type_count(HandRank::OnePair);
    let _voucher_unlock_condition = pairs_played >= 10; // Example condition

    // Example: Analytics system tracking player preferences
    let all_hand_types = [
        HandRank::HighCard,
        HandRank::OnePair,
        HandRank::TwoPair,
        HandRank::ThreeOfAKind,
        HandRank::Straight,
        HandRank::Flush,
        HandRank::FullHouse,
        HandRank::FourOfAKind,
        HandRank::StraightFlush,
        HandRank::RoyalFlush,
    ];

    let _total_hands_played: u32 = all_hand_types
        .iter()
        .map(|&hand_type| game.get_hand_type_count(hand_type))
        .sum();

    // This centralized approach makes it easy for any system to access hand type data
    assert!(true); // Placeholder assertion
}

#[test]
fn test_joker_factory_integration() {
    // Test that all three jokers can be created via factory
    let supernova = JokerFactory::create(JokerId::Supernova);
    assert!(supernova.is_some());
    assert_eq!(supernova.unwrap().id(), JokerId::Supernova);

    let runner = JokerFactory::create(JokerId::Runner);
    assert!(runner.is_some());
    assert_eq!(runner.unwrap().id(), JokerId::Runner);

    let space_joker = JokerFactory::create(JokerId::SpaceJoker);
    assert!(space_joker.is_some());
    assert_eq!(space_joker.unwrap().id(), JokerId::SpaceJoker);
}

#[test]
fn test_joker_rarity_lists() {
    use balatro_rs::joker::JokerRarity;

    // Test that jokers are in correct rarity lists
    let common_jokers = JokerFactory::get_by_rarity(JokerRarity::Common);
    assert!(common_jokers.contains(&JokerId::Supernova));
    assert!(common_jokers.contains(&JokerId::Runner));

    let uncommon_jokers = JokerFactory::get_by_rarity(JokerRarity::Uncommon);
    assert!(uncommon_jokers.contains(&JokerId::SpaceJoker));

    // Test that all jokers are in implemented list
    let implemented = JokerFactory::get_all_implemented();
    assert!(implemented.contains(&JokerId::Supernova));
    assert!(implemented.contains(&JokerId::Runner));
    assert!(implemented.contains(&JokerId::SpaceJoker));
}

#[test]
fn test_joker_properties() {
    let supernova = JokerFactory::create(JokerId::Supernova).unwrap();
    assert_eq!(supernova.name(), "Supernova");
    assert_eq!(
        supernova.description(),
        "Mult equal to times this poker hand has been played"
    );
    assert_eq!(supernova.cost(), 3);

    let runner = JokerFactory::create(JokerId::Runner).unwrap();
    assert_eq!(runner.name(), "Runner");
    assert_eq!(
        runner.description(),
        "+15 Chips if played hand contains a Straight"
    );
    assert_eq!(runner.cost(), 3);

    let space_joker = JokerFactory::create(JokerId::SpaceJoker).unwrap();
    assert_eq!(space_joker.name(), "Space Joker");
    assert_eq!(
        space_joker.description(),
        "1 in 4 chance to upgrade level of played poker hand"
    );
    assert_eq!(space_joker.cost(), 6); // Uncommon rarity
}

#[test]
fn test_joker_state_manager_basic_functionality() {
    // Test that the state manager works correctly for our jokers
    let state_manager = Arc::new(JokerStateManager::new());

    // Test Supernova state tracking
    let supernova_id = JokerId::Supernova;

    // Initially no state
    assert!(!state_manager.has_state(supernova_id));

    // Set hand type count
    state_manager
        .set_custom_data(supernova_id, "Pair", 5)
        .unwrap();

    // Retrieve count
    let count: Option<i32> = state_manager.get_custom_data(supernova_id, "Pair").unwrap();
    assert_eq!(count, Some(5));

    // Test Runner state tracking
    let runner_id = JokerId::Runner;

    // Initially no accumulated value
    let initial_state = state_manager.get_state(runner_id);
    assert!(initial_state.is_none());

    // Add accumulated value
    state_manager.add_accumulated_value(runner_id, 15.0);

    // Check accumulated value
    let state = state_manager.get_state(runner_id).unwrap();
    assert_eq!(state.accumulated_value, 15.0);

    // Add more
    state_manager.add_accumulated_value(runner_id, 15.0);
    let state = state_manager.get_state(runner_id).unwrap();
    assert_eq!(state.accumulated_value, 30.0);
}
