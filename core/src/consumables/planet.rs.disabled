//! Planet card implementations for the Balatro game engine
//!
//! Planet cards permanently upgrade specific poker hands by increasing their level,
//! which improves both base chips and multiplier for that hand type.
//!
//! # Implementation
//!
//! Each planet card implements the Consumable trait and targets a specific HandRank.
//! When used, they increase the level of that hand type in the game state.

use crate::consumables::{
    Consumable, ConsumableEffect, ConsumableError, ConsumableType, Target, TargetType,
};
use crate::game::Game;
use crate::rank::HandRank;

/// Mercury planet card - levels up Pair hands
#[derive(Debug, Clone)]
pub struct Mercury;

impl Consumable for Mercury {
    fn consumable_type(&self) -> ConsumableType {
        ConsumableType::Planet
    }

    fn can_use(&self, _game_state: &Game, target: &Target) -> bool {
        matches!(target, Target::HandType(HandRank::OnePair))
    }

    fn use_effect(&self, game_state: &mut Game, target: Target) -> Result<(), ConsumableError> {
        if let Target::HandType(HandRank::OnePair) = target {
            game_state.level_up_hand(HandRank::OnePair)?;
            Ok(())
        } else {
            Err(ConsumableError::InvalidTarget(
                "Mercury can only target Pair hands".to_string(),
            ))
        }
    }

    fn get_description(&self) -> String {
        "Level up Pair hands".to_string()
    }

    fn get_target_type(&self) -> TargetType {
        TargetType::HandType
    }

    fn get_effect_category(&self) -> ConsumableEffect {
        ConsumableEffect::Enhancement
    }

    fn name(&self) -> &'static str {
        "Mercury"
    }

    fn description(&self) -> &'static str {
        "Level up Pair"
    }
}

/// Venus planet card - levels up Two Pair hands
#[derive(Debug, Clone)]
pub struct Venus;

impl Consumable for Venus {
    fn consumable_type(&self) -> ConsumableType {
        ConsumableType::Planet
    }

    fn can_use(&self, _game_state: &Game, target: &Target) -> bool {
        matches!(target, Target::HandType(HandRank::TwoPair))
    }

    fn use_effect(&self, game_state: &mut Game, target: Target) -> Result<(), ConsumableError> {
        if let Target::HandType(HandRank::TwoPair) = target {
            game_state.level_up_hand(HandRank::TwoPair)?;
            Ok(())
        } else {
            Err(ConsumableError::InvalidTarget(
                "Venus can only target Two Pair hands".to_string(),
            ))
        }
    }

    fn get_description(&self) -> String {
        "Level up Two Pair hands".to_string()
    }

    fn get_target_type(&self) -> TargetType {
        TargetType::HandType
    }

    fn get_effect_category(&self) -> ConsumableEffect {
        ConsumableEffect::Enhancement
    }

    fn name(&self) -> &'static str {
        "Venus"
    }

    fn description(&self) -> &'static str {
        "Level up Two Pair"
    }
}

/// Earth planet card - levels up Full House hands
#[derive(Debug, Clone)]
pub struct Earth;

impl Consumable for Earth {
    fn consumable_type(&self) -> ConsumableType {
        ConsumableType::Planet
    }

    fn can_use(&self, _game_state: &Game, target: &Target) -> bool {
        matches!(target, Target::HandType(HandRank::FullHouse))
    }

    fn use_effect(&self, game_state: &mut Game, target: Target) -> Result<(), ConsumableError> {
        if let Target::HandType(HandRank::FullHouse) = target {
            game_state.level_up_hand(HandRank::FullHouse)?;
            Ok(())
        } else {
            Err(ConsumableError::InvalidTarget(
                "Earth can only target Full House hands".to_string(),
            ))
        }
    }

    fn get_description(&self) -> String {
        "Level up Full House hands".to_string()
    }

    fn get_target_type(&self) -> TargetType {
        TargetType::HandType
    }

    fn get_effect_category(&self) -> ConsumableEffect {
        ConsumableEffect::Enhancement
    }

    fn name(&self) -> &'static str {
        "Earth"
    }

    fn description(&self) -> &'static str {
        "Level up Full House"
    }
}

/// Mars planet card - levels up Three of a Kind hands
#[derive(Debug, Clone)]
pub struct Mars;

impl Consumable for Mars {
    fn consumable_type(&self) -> ConsumableType {
        ConsumableType::Planet
    }

    fn can_use(&self, _game_state: &Game, target: &Target) -> bool {
        matches!(target, Target::HandType(HandRank::ThreeOfAKind))
    }

    fn use_effect(&self, game_state: &mut Game, target: Target) -> Result<(), ConsumableError> {
        if let Target::HandType(HandRank::ThreeOfAKind) = target {
            game_state.level_up_hand(HandRank::ThreeOfAKind)?;
            Ok(())
        } else {
            Err(ConsumableError::InvalidTarget(
                "Mars can only target Three of a Kind hands".to_string(),
            ))
        }
    }

    fn get_description(&self) -> String {
        "Level up Three of a Kind hands".to_string()
    }

    fn get_target_type(&self) -> TargetType {
        TargetType::HandType
    }

    fn get_effect_category(&self) -> ConsumableEffect {
        ConsumableEffect::Enhancement
    }

    fn name(&self) -> &'static str {
        "Mars"
    }

    fn description(&self) -> &'static str {
        "Level up Three of a Kind"
    }
}

/// Jupiter planet card - levels up Straight hands
#[derive(Debug, Clone)]
pub struct Jupiter;

impl Consumable for Jupiter {
    fn consumable_type(&self) -> ConsumableType {
        ConsumableType::Planet
    }

    fn can_use(&self, _game_state: &Game, target: &Target) -> bool {
        matches!(target, Target::HandType(HandRank::Straight))
    }

    fn use_effect(&self, game_state: &mut Game, target: Target) -> Result<(), ConsumableError> {
        if let Target::HandType(HandRank::Straight) = target {
            game_state.level_up_hand(HandRank::Straight)?;
            Ok(())
        } else {
            Err(ConsumableError::InvalidTarget(
                "Jupiter can only target Straight hands".to_string(),
            ))
        }
    }

    fn get_description(&self) -> String {
        "Level up Straight hands".to_string()
    }

    fn get_target_type(&self) -> TargetType {
        TargetType::HandType
    }

    fn get_effect_category(&self) -> ConsumableEffect {
        ConsumableEffect::Enhancement
    }

    fn name(&self) -> &'static str {
        "Jupiter"
    }

    fn description(&self) -> &'static str {
        "Level up Straight"
    }
}

/// Saturn planet card - levels up Straight hands
#[derive(Debug, Clone)]
pub struct Saturn;

impl Consumable for Saturn {
    fn consumable_type(&self) -> ConsumableType {
        ConsumableType::Planet
    }

    fn can_use(&self, _game_state: &Game, target: &Target) -> bool {
        matches!(target, Target::HandType(HandRank::Straight))
    }

    fn use_effect(&self, game_state: &mut Game, target: Target) -> Result<(), ConsumableError> {
        if let Target::HandType(HandRank::Straight) = target {
            game_state.level_up_hand(HandRank::Straight)?;
            Ok(())
        } else {
            Err(ConsumableError::InvalidTarget(
                "Saturn can only target Straight hands".to_string(),
            ))
        }
    }

    fn get_description(&self) -> String {
        "Level up Straight hands".to_string()
    }

    fn get_target_type(&self) -> TargetType {
        TargetType::HandType
    }

    fn get_effect_category(&self) -> ConsumableEffect {
        ConsumableEffect::Enhancement
    }

    fn name(&self) -> &'static str {
        "Saturn"
    }

    fn description(&self) -> &'static str {
        "Level up Straight"
    }
}

/// Uranus planet card - levels up Two Pair hands
#[derive(Debug, Clone)]
pub struct Uranus;

impl Consumable for Uranus {
    fn consumable_type(&self) -> ConsumableType {
        ConsumableType::Planet
    }

    fn can_use(&self, _game_state: &Game, target: &Target) -> bool {
        matches!(target, Target::HandType(HandRank::TwoPair))
    }

    fn use_effect(&self, game_state: &mut Game, target: Target) -> Result<(), ConsumableError> {
        if let Target::HandType(HandRank::TwoPair) = target {
            game_state.level_up_hand(HandRank::TwoPair)?;
            Ok(())
        } else {
            Err(ConsumableError::InvalidTarget(
                "Uranus can only target Two Pair hands".to_string(),
            ))
        }
    }

    fn get_description(&self) -> String {
        "Level up Two Pair hands".to_string()
    }

    fn get_target_type(&self) -> TargetType {
        TargetType::HandType
    }

    fn get_effect_category(&self) -> ConsumableEffect {
        ConsumableEffect::Enhancement
    }

    fn name(&self) -> &'static str {
        "Uranus"
    }

    fn description(&self) -> &'static str {
        "Level up Two Pair"
    }
}

/// Neptune planet card - levels up Straight Flush hands
#[derive(Debug, Clone)]
pub struct Neptune;

impl Consumable for Neptune {
    fn consumable_type(&self) -> ConsumableType {
        ConsumableType::Planet
    }

    fn can_use(&self, _game_state: &Game, target: &Target) -> bool {
        matches!(target, Target::HandType(HandRank::StraightFlush))
    }

    fn use_effect(&self, game_state: &mut Game, target: Target) -> Result<(), ConsumableError> {
        if let Target::HandType(HandRank::StraightFlush) = target {
            game_state.level_up_hand(HandRank::StraightFlush)?;
            Ok(())
        } else {
            Err(ConsumableError::InvalidTarget(
                "Neptune can only target Straight Flush hands".to_string(),
            ))
        }
    }

    fn get_description(&self) -> String {
        "Level up Straight Flush hands".to_string()
    }

    fn get_target_type(&self) -> TargetType {
        TargetType::HandType
    }

    fn get_effect_category(&self) -> ConsumableEffect {
        ConsumableEffect::Enhancement
    }

    fn name(&self) -> &'static str {
        "Neptune"
    }

    fn description(&self) -> &'static str {
        "Level up Straight Flush"
    }
}

/// Pluto planet card - levels up High Card hands
#[derive(Debug, Clone)]
pub struct Pluto;

impl Consumable for Pluto {
    fn consumable_type(&self) -> ConsumableType {
        ConsumableType::Planet
    }

    fn can_use(&self, _game_state: &Game, target: &Target) -> bool {
        matches!(target, Target::HandType(HandRank::HighCard))
    }

    fn use_effect(&self, game_state: &mut Game, target: Target) -> Result<(), ConsumableError> {
        if let Target::HandType(HandRank::HighCard) = target {
            game_state.level_up_hand(HandRank::HighCard)?;
            Ok(())
        } else {
            Err(ConsumableError::InvalidTarget(
                "Pluto can only target High Card hands".to_string(),
            ))
        }
    }

    fn get_description(&self) -> String {
        "Level up High Card hands".to_string()
    }

    fn get_target_type(&self) -> TargetType {
        TargetType::HandType
    }

    fn get_effect_category(&self) -> ConsumableEffect {
        ConsumableEffect::Enhancement
    }

    fn name(&self) -> &'static str {
        "Pluto"
    }

    fn description(&self) -> &'static str {
        "Level up High Card"
    }
}

/// Planet X planet card - levels up Five of a Kind hands
#[derive(Debug, Clone)]
pub struct PlanetX;

impl Consumable for PlanetX {
    fn consumable_type(&self) -> ConsumableType {
        ConsumableType::Planet
    }

    fn can_use(&self, _game_state: &Game, target: &Target) -> bool {
        matches!(target, Target::HandType(HandRank::FiveOfAKind))
    }

    fn use_effect(&self, game_state: &mut Game, target: Target) -> Result<(), ConsumableError> {
        if let Target::HandType(HandRank::FiveOfAKind) = target {
            game_state.level_up_hand(HandRank::FiveOfAKind)?;
            Ok(())
        } else {
            Err(ConsumableError::InvalidTarget(
                "Planet X can only target Five of a Kind hands".to_string(),
            ))
        }
    }

    fn get_description(&self) -> String {
        "Level up Five of a Kind hands".to_string()
    }

    fn get_target_type(&self) -> TargetType {
        TargetType::HandType
    }

    fn get_effect_category(&self) -> ConsumableEffect {
        ConsumableEffect::Enhancement
    }

    fn name(&self) -> &'static str {
        "Planet X"
    }

    fn description(&self) -> &'static str {
        "Level up Five of a Kind"
    }
}

/// Ceres planet card - levels up Flush House hands
#[derive(Debug, Clone)]
pub struct Ceres;

impl Consumable for Ceres {
    fn consumable_type(&self) -> ConsumableType {
        ConsumableType::Planet
    }

    fn can_use(&self, _game_state: &Game, target: &Target) -> bool {
        matches!(target, Target::HandType(HandRank::FlushHouse))
    }

    fn use_effect(&self, game_state: &mut Game, target: Target) -> Result<(), ConsumableError> {
        if let Target::HandType(HandRank::FlushHouse) = target {
            game_state.level_up_hand(HandRank::FlushHouse)?;
            Ok(())
        } else {
            Err(ConsumableError::InvalidTarget(
                "Ceres can only target Flush House hands".to_string(),
            ))
        }
    }

    fn get_description(&self) -> String {
        "Level up Flush House hands".to_string()
    }

    fn get_target_type(&self) -> TargetType {
        TargetType::HandType
    }

    fn get_effect_category(&self) -> ConsumableEffect {
        ConsumableEffect::Enhancement
    }

    fn name(&self) -> &'static str {
        "Ceres"
    }

    fn description(&self) -> &'static str {
        "Level up Flush House"
    }
}

/// Eris planet card - levels up Flush Five hands
#[derive(Debug, Clone)]
pub struct Eris;

impl Consumable for Eris {
    fn consumable_type(&self) -> ConsumableType {
        ConsumableType::Planet
    }

    fn can_use(&self, _game_state: &Game, target: &Target) -> bool {
        matches!(target, Target::HandType(HandRank::FlushFive))
    }

    fn use_effect(&self, game_state: &mut Game, target: Target) -> Result<(), ConsumableError> {
        if let Target::HandType(HandRank::FlushFive) = target {
            game_state.level_up_hand(HandRank::FlushFive)?;
            Ok(())
        } else {
            Err(ConsumableError::InvalidTarget(
                "Eris can only target Flush Five hands".to_string(),
            ))
        }
    }

    fn get_description(&self) -> String {
        "Level up Flush Five hands".to_string()
    }

    fn get_target_type(&self) -> TargetType {
        TargetType::HandType
    }

    fn get_effect_category(&self) -> ConsumableEffect {
        ConsumableEffect::Enhancement
    }

    fn name(&self) -> &'static str {
        "Eris"
    }

    fn description(&self) -> &'static str {
        "Level up Flush Five"
    }
}

/// Factory function to create planet cards by ID
pub fn create_planet_card(id: crate::consumables::ConsumableId) -> Option<Box<dyn Consumable>> {
    use crate::consumables::ConsumableId;

    match id {
        ConsumableId::Mercury => Some(Box::new(Mercury)),
        ConsumableId::Venus => Some(Box::new(Venus)),
        ConsumableId::Earth => Some(Box::new(Earth)),
        ConsumableId::Mars => Some(Box::new(Mars)),
        ConsumableId::Jupiter => Some(Box::new(Jupiter)),
        ConsumableId::Saturn => Some(Box::new(Saturn)),
        ConsumableId::Uranus => Some(Box::new(Uranus)),
        ConsumableId::Neptune => Some(Box::new(Neptune)),
        ConsumableId::Pluto => Some(Box::new(Pluto)),
        ConsumableId::PlanetX => Some(Box::new(PlanetX)),
        ConsumableId::Ceres => Some(Box::new(Ceres)),
        ConsumableId::Eris => Some(Box::new(Eris)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;

    fn create_test_game() -> Game {
        Game::new(Config::default())
    }

    #[test]
    fn test_mercury_targets_pair() {
        let mercury = Mercury;
        let game = create_test_game();

        // Should accept Pair target
        assert!(mercury.can_use(&game, &Target::HandType(HandRank::OnePair)));

        // Should reject other targets
        assert!(!mercury.can_use(&game, &Target::HandType(HandRank::TwoPair)));
        assert!(!mercury.can_use(&game, &Target::None));
    }

    #[test]
    fn test_venus_targets_two_pair() {
        let venus = Venus;
        let game = create_test_game();

        // Should accept Two Pair target
        assert!(venus.can_use(&game, &Target::HandType(HandRank::TwoPair)));

        // Should reject other targets
        assert!(!venus.can_use(&game, &Target::HandType(HandRank::OnePair)));
        assert!(!venus.can_use(&game, &Target::None));
    }

    #[test]
    fn test_planet_card_factory() {
        use crate::consumables::ConsumableId;

        // Should create valid planet cards
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

        // Should return None for non-planet cards
        assert!(create_planet_card(ConsumableId::TheFool).is_none());
    }

    #[test]
    fn test_planet_card_properties() {
        let mercury = Mercury;

        assert_eq!(mercury.consumable_type(), ConsumableType::Planet);
        assert_eq!(mercury.get_target_type(), TargetType::HandType);
        assert_eq!(mercury.get_effect_category(), ConsumableEffect::Enhancement);
        assert_eq!(mercury.name(), "Mercury");
    }

    #[test]
    fn test_saturn_targets_straight() {
        let saturn = Saturn;
        let game = create_test_game();

        // Should accept Straight target
        assert!(saturn.can_use(&game, &Target::HandType(HandRank::Straight)));

        // Should reject other targets
        assert!(!saturn.can_use(&game, &Target::HandType(HandRank::OnePair)));
        assert!(!saturn.can_use(&game, &Target::None));
    }

    #[test]
    fn test_uranus_targets_two_pair() {
        let uranus = Uranus;
        let game = create_test_game();

        // Should accept Two Pair target
        assert!(uranus.can_use(&game, &Target::HandType(HandRank::TwoPair)));

        // Should reject other targets
        assert!(!uranus.can_use(&game, &Target::HandType(HandRank::OnePair)));
        assert!(!uranus.can_use(&game, &Target::None));
    }

    #[test]
    fn test_neptune_targets_straight_flush() {
        let neptune = Neptune;
        let game = create_test_game();

        // Should accept Straight Flush target
        assert!(neptune.can_use(&game, &Target::HandType(HandRank::StraightFlush)));

        // Should reject other targets
        assert!(!neptune.can_use(&game, &Target::HandType(HandRank::Straight)));
        assert!(!neptune.can_use(&game, &Target::None));
    }

    #[test]
    fn test_pluto_targets_high_card() {
        let pluto = Pluto;
        let game = create_test_game();

        // Should accept High Card target
        assert!(pluto.can_use(&game, &Target::HandType(HandRank::HighCard)));

        // Should reject other targets
        assert!(!pluto.can_use(&game, &Target::HandType(HandRank::OnePair)));
        assert!(!pluto.can_use(&game, &Target::None));
    }

    #[test]
    fn test_planet_x_targets_five_of_a_kind() {
        let planet_x = PlanetX;
        let game = create_test_game();

        // Should accept Five of a Kind target
        assert!(planet_x.can_use(&game, &Target::HandType(HandRank::FiveOfAKind)));

        // Should reject other targets
        assert!(!planet_x.can_use(&game, &Target::HandType(HandRank::FourOfAKind)));
        assert!(!planet_x.can_use(&game, &Target::None));
    }

    #[test]
    fn test_ceres_targets_flush_house() {
        let ceres = Ceres;
        let game = create_test_game();

        // Should accept Flush House target
        assert!(ceres.can_use(&game, &Target::HandType(HandRank::FlushHouse)));

        // Should reject other targets
        assert!(!ceres.can_use(&game, &Target::HandType(HandRank::FullHouse)));
        assert!(!ceres.can_use(&game, &Target::None));
    }

    #[test]
    fn test_eris_targets_flush_five() {
        let eris = Eris;
        let game = create_test_game();

        // Should accept Flush Five target
        assert!(eris.can_use(&game, &Target::HandType(HandRank::FlushFive)));

        // Should reject other targets
        assert!(!eris.can_use(&game, &Target::HandType(HandRank::FiveOfAKind)));
        assert!(!eris.can_use(&game, &Target::None));
    }

    #[test]
    fn test_new_planet_card_effects() {
        let mut game = create_test_game();

        // Test Saturn effect on Straight
        let saturn = Saturn;
        let initial_level = game.get_hand_level(HandRank::Straight);
        assert_eq!(initial_level.level, 1);

        let target = Target::HandType(HandRank::Straight);
        assert!(saturn.use_effect(&mut game, target).is_ok());

        let new_level = game.get_hand_level(HandRank::Straight);
        assert_eq!(new_level.level, 2);
        assert!(new_level.chips > initial_level.chips);
        assert!(new_level.mult > initial_level.mult);

        // Test Neptune effect on Straight Flush
        let neptune = Neptune;
        let initial_level = game.get_hand_level(HandRank::StraightFlush);
        assert_eq!(initial_level.level, 1);

        let target = Target::HandType(HandRank::StraightFlush);
        assert!(neptune.use_effect(&mut game, target).is_ok());

        let new_level = game.get_hand_level(HandRank::StraightFlush);
        assert_eq!(new_level.level, 2);
        assert!(new_level.chips > initial_level.chips);
        assert!(new_level.mult > initial_level.mult);
    }

    #[test]
    fn test_all_new_planet_card_properties() {
        let saturn = Saturn;
        let uranus = Uranus;
        let neptune = Neptune;
        let pluto = Pluto;
        let planet_x = PlanetX;
        let ceres = Ceres;
        let eris = Eris;

        // Test that all new planet cards have correct type
        assert_eq!(saturn.consumable_type(), ConsumableType::Planet);
        assert_eq!(uranus.consumable_type(), ConsumableType::Planet);
        assert_eq!(neptune.consumable_type(), ConsumableType::Planet);
        assert_eq!(pluto.consumable_type(), ConsumableType::Planet);
        assert_eq!(planet_x.consumable_type(), ConsumableType::Planet);
        assert_eq!(ceres.consumable_type(), ConsumableType::Planet);
        assert_eq!(eris.consumable_type(), ConsumableType::Planet);

        // Test names
        assert_eq!(saturn.name(), "Saturn");
        assert_eq!(uranus.name(), "Uranus");
        assert_eq!(neptune.name(), "Neptune");
        assert_eq!(pluto.name(), "Pluto");
        assert_eq!(planet_x.name(), "Planet X");
        assert_eq!(ceres.name(), "Ceres");
        assert_eq!(eris.name(), "Eris");

        // Test target types
        assert_eq!(saturn.get_target_type(), TargetType::HandType);
        assert_eq!(uranus.get_target_type(), TargetType::HandType);
        assert_eq!(neptune.get_target_type(), TargetType::HandType);
        assert_eq!(pluto.get_target_type(), TargetType::HandType);
        assert_eq!(planet_x.get_target_type(), TargetType::HandType);
        assert_eq!(ceres.get_target_type(), TargetType::HandType);
        assert_eq!(eris.get_target_type(), TargetType::HandType);

        // Test effect categories
        assert_eq!(saturn.get_effect_category(), ConsumableEffect::Enhancement);
        assert_eq!(uranus.get_effect_category(), ConsumableEffect::Enhancement);
        assert_eq!(neptune.get_effect_category(), ConsumableEffect::Enhancement);
        assert_eq!(pluto.get_effect_category(), ConsumableEffect::Enhancement);
        assert_eq!(
            planet_x.get_effect_category(),
            ConsumableEffect::Enhancement
        );
        assert_eq!(ceres.get_effect_category(), ConsumableEffect::Enhancement);
        assert_eq!(eris.get_effect_category(), ConsumableEffect::Enhancement);
    }

    #[test]
    fn test_invalid_targets_for_new_planet_cards() {
        let mut game = create_test_game();

        // Test Saturn with wrong target
        let saturn = Saturn;
        let wrong_target = Target::HandType(HandRank::OnePair);
        assert!(saturn.use_effect(&mut game, wrong_target).is_err());

        // Test Planet X with wrong target
        let planet_x = PlanetX;
        let wrong_target = Target::HandType(HandRank::FourOfAKind);
        assert!(planet_x.use_effect(&mut game, wrong_target).is_err());
    }
}
