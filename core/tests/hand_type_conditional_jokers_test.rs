use balatro_rs::joker::JokerId;
use balatro_rs::joker_factory::JokerFactory;
use balatro_rs::joker_state::JokerStateManager;
use std::sync::Arc;


// Simplified tests that don't require full GameContext integration
// TODO: Add integration tests when GameContext creation is simplified

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
    assert_eq!(supernova.description(), "Mult equal to times this poker hand has been played");
    assert_eq!(supernova.cost(), 3);
    
    let runner = JokerFactory::create(JokerId::Runner).unwrap();
    assert_eq!(runner.name(), "Runner");
    assert_eq!(runner.description(), "+15 Chips if played hand contains a Straight");
    assert_eq!(runner.cost(), 3);
    
    let space_joker = JokerFactory::create(JokerId::SpaceJoker).unwrap();
    assert_eq!(space_joker.name(), "Space Joker");
    assert_eq!(space_joker.description(), "1 in 4 chance to upgrade level of played poker hand");
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
    state_manager.set_custom_data(supernova_id, "Pair", 5).unwrap();
    
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