//! Integration tests for planet card functionality
//!
//! These tests verify the complete planet card implementation including:
//! - Planet card creation and properties
//! - Hand level system integration
//! - Game action integration
//! - End-to-end planet card usage

use balatro_rs::{
    action::Action,
    config::Config,
    consumables::{
        planet::{
            create_planet_card, Ceres, Earth, Eris, Jupiter, Mars, Mercury, Neptune, PlanetX,
            Pluto, Saturn, Uranus, Venus,
        },
        Consumable, ConsumableId, ConsumableType, Target,
    },
    game::Game,
    rank::HandRank,
};

#[test]
fn test_planet_card_creation() {
    // Test that all planet cards can be created
    assert!(create_planet_card(ConsumableId::Mercury).is_some());
    assert!(create_planet_card(ConsumableId::Venus).is_some());
    assert!(create_planet_card(ConsumableId::Earth).is_some());
    assert!(create_planet_card(ConsumableId::Mars).is_some());
    assert!(create_planet_card(ConsumableId::Jupiter).is_some());
    assert!(create_planet_card(ConsumableId::Saturn).is_some());
    assert!(create_planet_card(ConsumableId::Uranus).is_some());
    assert!(create_planet_card(ConsumableId::Neptune).is_some());
    assert!(create_planet_card(ConsumableId::Pluto).is_some());
    assert!(create_planet_card(ConsumableId::PlanetX).is_some());
    assert!(create_planet_card(ConsumableId::Ceres).is_some());
    assert!(create_planet_card(ConsumableId::Eris).is_some());

    // Test that non-planet cards return None
    assert!(create_planet_card(ConsumableId::TheFool).is_none());
    assert!(create_planet_card(ConsumableId::Familiar).is_none());
}

#[test]
fn test_planet_card_properties() {
    let mercury = Mercury;
    let venus = Venus;
    let earth = Earth;
    let mars = Mars;
    let jupiter = Jupiter;
    let saturn = Saturn;
    let uranus = Uranus;
    let neptune = Neptune;
    let pluto = Pluto;
    let planet_x = PlanetX;
    let ceres = Ceres;
    let eris = Eris;

    // Test that all planet cards have correct type
    assert_eq!(mercury.consumable_type(), ConsumableType::Planet);
    assert_eq!(venus.consumable_type(), ConsumableType::Planet);
    assert_eq!(earth.consumable_type(), ConsumableType::Planet);
    assert_eq!(mars.consumable_type(), ConsumableType::Planet);
    assert_eq!(jupiter.consumable_type(), ConsumableType::Planet);
    assert_eq!(saturn.consumable_type(), ConsumableType::Planet);
    assert_eq!(uranus.consumable_type(), ConsumableType::Planet);
    assert_eq!(neptune.consumable_type(), ConsumableType::Planet);
    assert_eq!(pluto.consumable_type(), ConsumableType::Planet);
    assert_eq!(planet_x.consumable_type(), ConsumableType::Planet);
    assert_eq!(ceres.consumable_type(), ConsumableType::Planet);
    assert_eq!(eris.consumable_type(), ConsumableType::Planet);

    // Test names
    assert_eq!(mercury.name(), "Mercury");
    assert_eq!(venus.name(), "Venus");
    assert_eq!(earth.name(), "Earth");
    assert_eq!(mars.name(), "Mars");
    assert_eq!(jupiter.name(), "Jupiter");
    assert_eq!(saturn.name(), "Saturn");
    assert_eq!(uranus.name(), "Uranus");
    assert_eq!(neptune.name(), "Neptune");
    assert_eq!(pluto.name(), "Pluto");
    assert_eq!(planet_x.name(), "Planet X");
    assert_eq!(ceres.name(), "Ceres");
    assert_eq!(eris.name(), "Eris");
}

#[test]
fn test_planet_card_targeting() {
    let game = Game::new(Config::default());
    let mercury = Mercury;
    let venus = Venus;

    // Test that Mercury only targets OnePair
    assert!(mercury.can_use(&game, &Target::HandType(HandRank::OnePair)));
    assert!(!mercury.can_use(&game, &Target::HandType(HandRank::TwoPair)));
    assert!(!mercury.can_use(&game, &Target::None));

    // Test that Venus only targets TwoPair
    assert!(venus.can_use(&game, &Target::HandType(HandRank::TwoPair)));
    assert!(!venus.can_use(&game, &Target::HandType(HandRank::OnePair)));
    assert!(!venus.can_use(&game, &Target::None));
}

#[test]
fn test_hand_level_system() {
    let mut game = Game::new(Config::default());

    // Test initial hand levels (should all be 1)
    let initial_level = game.get_hand_level(HandRank::OnePair);
    assert_eq!(initial_level.level, 1);
    assert!(initial_level.chips > 0);
    assert!(initial_level.mult > 0);

    // Test leveling up a hand
    assert!(game.level_up_hand(HandRank::OnePair).is_ok());
    let leveled_up = game.get_hand_level(HandRank::OnePair);
    assert_eq!(leveled_up.level, 2);
    assert!(leveled_up.chips > initial_level.chips);
    assert!(leveled_up.mult > initial_level.mult);

    // Test that other hands are unaffected
    let other_hand = game.get_hand_level(HandRank::TwoPair);
    assert_eq!(other_hand.level, 1);
}

#[test]
fn test_planet_card_effect() {
    let mut game = Game::new(Config::default());
    let mercury = Mercury;

    // Get initial level
    let initial_level = game.get_hand_level(HandRank::OnePair);
    assert_eq!(initial_level.level, 1);

    // Use Mercury to level up OnePair
    let target = Target::HandType(HandRank::OnePair);
    assert!(mercury.use_effect(&mut game, target).is_ok());

    // Verify that OnePair was leveled up
    let new_level = game.get_hand_level(HandRank::OnePair);
    assert_eq!(new_level.level, 2);
    assert!(new_level.chips > initial_level.chips);
    assert!(new_level.mult > initial_level.mult);

    // Verify other hands were not affected
    let other_level = game.get_hand_level(HandRank::TwoPair);
    assert_eq!(other_level.level, 1);
}

#[test]
fn test_planet_card_action_integration() {
    let mut game = Game::new(Config::default());

    // Test UseConsumable action (basic validation)
    let consumable_action = Action::UseConsumable {
        slot: 0,
        target_description: "none".to_string(),
    };
    // This should return InvalidAction since no consumables are in slots
    assert!(game.handle_action(consumable_action).is_err());

    // Test UsePlanetCard action with Mercury (ID 11 in enum) and OnePair (ID 1 in enum)
    let planet_action = Action::UsePlanetCard {
        planet_card_id: 11, // Mercury
        hand_rank_id: 1,    // OnePair
    };

    // Get initial hand level
    let initial_level = game.get_hand_level(HandRank::OnePair);
    assert_eq!(initial_level.level, 1);

    // Use the planet card action
    assert!(game.handle_action(planet_action).is_ok());

    // Verify the hand was leveled up
    let new_level = game.get_hand_level(HandRank::OnePair);
    assert_eq!(new_level.level, 2);
}

#[test]
fn test_multiple_planet_card_usage() {
    let mut game = Game::new(Config::default());

    // Level up OnePair multiple times
    for expected_level in 2..=5 {
        let planet_action = Action::UsePlanetCard {
            planet_card_id: 11, // Mercury
            hand_rank_id: 1,    // OnePair
        };
        assert!(game.handle_action(planet_action).is_ok());

        let current_level = game.get_hand_level(HandRank::OnePair);
        assert_eq!(current_level.level, expected_level);
    }
}

#[test]
fn test_all_planet_cards_integration() {
    let _game = Game::new(Config::default());

    // Test each planet card with its corresponding hand type
    let test_cases = vec![
        (11, 1, HandRank::OnePair),       // Mercury -> OnePair
        (12, 2, HandRank::TwoPair),       // Venus -> TwoPair
        (13, 6, HandRank::FullHouse),     // Earth -> FullHouse
        (14, 3, HandRank::ThreeOfAKind),  // Mars -> ThreeOfAKind
        (15, 4, HandRank::Straight),      // Jupiter -> Straight
        (16, 4, HandRank::Straight),      // Saturn -> Straight
        (17, 2, HandRank::TwoPair),       // Uranus -> TwoPair
        (18, 8, HandRank::StraightFlush), // Neptune -> StraightFlush
        (19, 0, HandRank::HighCard),      // Pluto -> HighCard
        (20, 10, HandRank::FiveOfAKind),  // PlanetX -> FiveOfAKind
        (21, 11, HandRank::FlushHouse),   // Ceres -> FlushHouse
        (22, 12, HandRank::FlushFive),    // Eris -> FlushFive
    ];

    for (planet_id, hand_id, hand_rank) in test_cases {
        // Create a fresh game for each test case to avoid interference
        let mut fresh_game = Game::new(Config::default());

        // Get initial level
        let initial_level = fresh_game.get_hand_level(hand_rank);
        assert_eq!(initial_level.level, 1);

        // Use planet card
        let action = Action::UsePlanetCard {
            planet_card_id: planet_id,
            hand_rank_id: hand_id,
        };
        assert!(fresh_game.handle_action(action).is_ok());

        // Verify level increased
        let new_level = fresh_game.get_hand_level(hand_rank);
        assert_eq!(new_level.level, 2);
    }
}

#[test]
fn test_invalid_planet_card_usage() {
    let mut game = Game::new(Config::default());

    // Test invalid planet card ID
    let invalid_planet_action = Action::UsePlanetCard {
        planet_card_id: 999,
        hand_rank_id: 1,
    };
    assert!(game.handle_action(invalid_planet_action).is_err());

    // Test invalid hand rank ID
    let invalid_hand_action = Action::UsePlanetCard {
        planet_card_id: 11,
        hand_rank_id: 999,
    };
    assert!(game.handle_action(invalid_hand_action).is_err());

    // Test using non-planet card (TheFool is ID 0)
    let non_planet_action = Action::UsePlanetCard {
        planet_card_id: 0,
        hand_rank_id: 1,
    };
    assert!(game.handle_action(non_planet_action).is_err());
}

#[test]
fn test_consumable_slots_basic_functionality() {
    let game = Game::new(Config::default());

    // Verify consumable_slots field exists and is initialized
    assert_eq!(game.consumable_slots.capacity(), 2);
    assert!(game.consumable_slots.is_empty());
    assert_eq!(game.consumable_slots.len(), 0);
}

#[test]
fn test_hand_level_scaling() {
    let mut game = Game::new(Config::default());

    // Test that hand levels scale properly
    let base_level = game.get_hand_level(HandRank::OnePair);

    // Level up the hand
    game.level_up_hand(HandRank::OnePair).unwrap();
    let level_2 = game.get_hand_level(HandRank::OnePair);

    // Level 2 should have more chips and mult than level 1
    assert!(level_2.chips > base_level.chips);
    assert!(level_2.mult > base_level.mult);
    assert_eq!(level_2.level, 2);

    // Level up again
    game.level_up_hand(HandRank::OnePair).unwrap();
    let level_3 = game.get_hand_level(HandRank::OnePair);

    // Level 3 should have more than level 2
    assert!(level_3.chips > level_2.chips);
    assert!(level_3.mult > level_2.mult);
    assert_eq!(level_3.level, 3);
}
