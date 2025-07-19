use balatro_rs::consumables::{
    Consumable, ConsumableEffect, ConsumableError, ConsumableType, Target, TargetType,
};
use balatro_rs::game::Game;

#[test]
fn test_consumable_trait_object_compatibility() {
    #[derive(Debug)]
    struct MockConsumable;

    impl Consumable for MockConsumable {
        fn consumable_type(&self) -> ConsumableType {
            ConsumableType::Tarot
        }

        fn can_use(&self, _game_state: &Game, _target: &Target) -> bool {
            true
        }

        fn use_effect(
            &self,
            _game_state: &mut Game,
            _target: Target,
        ) -> Result<(), ConsumableError> {
            Ok(())
        }

        fn get_description(&self) -> String {
            "Mock consumable for testing".to_string()
        }

        fn get_target_type(&self) -> TargetType {
            TargetType::None
        }

        fn get_effect_category(&self) -> ConsumableEffect {
            ConsumableEffect::Enhancement
        }
    }

    // Test trait object compatibility with Send + Sync + Debug bounds
    let consumable: Box<dyn Consumable + Send + Sync> = Box::new(MockConsumable);

    // Test basic trait methods work through trait object
    assert_eq!(consumable.consumable_type(), ConsumableType::Tarot);
    assert_eq!(consumable.get_target_type(), TargetType::None);
    assert_eq!(
        consumable.get_effect_category(),
        ConsumableEffect::Enhancement
    );
    assert_eq!(consumable.get_description(), "Mock consumable for testing");
}

#[test]
fn test_consumable_effect_enum_categories() {
    // Test all effect categories are properly defined
    let enhancement = ConsumableEffect::Enhancement;
    let destruction = ConsumableEffect::Destruction;
    let generation = ConsumableEffect::Generation;
    let modification = ConsumableEffect::Modification;
    let utility = ConsumableEffect::Utility;

    // Test Debug implementation
    assert!(format!("{:?}", enhancement).contains("Enhancement"));
    assert!(format!("{:?}", destruction).contains("Destruction"));
    assert!(format!("{:?}", generation).contains("Generation"));
    assert!(format!("{:?}", modification).contains("Modification"));
    assert!(format!("{:?}", utility).contains("Utility"));
}

#[test]
fn test_target_type_definitions() {
    // Test all target types are properly defined
    let none = TargetType::None;
    let cards = TargetType::Cards(2);
    let hand_type = TargetType::HandType;
    let joker = TargetType::Joker;
    let deck = TargetType::Deck;

    // Test Debug implementation
    assert!(format!("{:?}", none).contains("None"));
    assert!(format!("{:?}", cards).contains("Cards"));
    assert!(format!("{:?}", hand_type).contains("HandType"));
    assert!(format!("{:?}", joker).contains("Joker"));
    assert!(format!("{:?}", deck).contains("Deck"));
}

#[test]
fn test_target_validation() {
    let game = Game::default();

    // Test different target types
    let no_target = Target::None;
    let card_targets = Target::Cards(vec![0, 1]);
    let hand_target = Target::HandType(balatro_rs::rank::HandRank::OnePair);

    assert!(matches!(no_target, Target::None));
    assert!(matches!(card_targets, Target::Cards(_)));
    assert!(matches!(hand_target, Target::HandType(_)));

    // Test validation methods
    assert!(no_target.is_valid(&game));
    // Card targets would need game state validation
    assert!(hand_target.is_valid(&game));
}

#[test]
fn test_consumable_error_types() {
    // Test error type definitions
    let invalid_target = ConsumableError::InvalidTarget("Mock error".to_string());
    let insufficient_resources = ConsumableError::InsufficientResources;
    let invalid_game_state = ConsumableError::InvalidGameState("Mock state error".to_string());
    let effect_failed = ConsumableError::EffectFailed("Mock effect error".to_string());

    // Test error display
    assert!(invalid_target.to_string().contains("Invalid target"));
    assert!(insufficient_resources
        .to_string()
        .contains("Insufficient resources"));
    assert!(invalid_game_state
        .to_string()
        .contains("Invalid game state"));
    assert!(effect_failed.to_string().contains("Effect failed"));
}

#[test]
fn test_consumable_type_existing_variants() {
    // Test existing ConsumableType variants still work
    let tarot = ConsumableType::Tarot;
    let planet = ConsumableType::Planet;
    let spectral = ConsumableType::Spectral;

    assert_eq!(tarot.to_string(), "Tarot");
    assert_eq!(planet.to_string(), "Planet");
    assert_eq!(spectral.to_string(), "Spectral");
}

// TODO: Add async test when tokio dependency is available
// #[tokio::test]
// async fn test_async_effect_application() {
//     // Async support will be added in future iterations
// }

#[test]
fn test_enhanced_consumable_trait_methods() {
    #[derive(Debug)]
    struct EnhancedMockConsumable;

    impl Consumable for EnhancedMockConsumable {
        fn consumable_type(&self) -> ConsumableType {
            ConsumableType::Planet
        }

        fn can_use(&self, game_state: &Game, target: &Target) -> bool {
            // Mock validation logic
            match target {
                Target::None => true,
                Target::Cards(cards) => {
                    !cards.is_empty() && cards.len() <= game_state.available.cards().len()
                }
                Target::HandType(_) => true,
                Target::Joker(_) => game_state.jokers.len() > 0,
                Target::Deck => true,
            }
        }

        fn use_effect(
            &self,
            _game_state: &mut Game,
            _target: Target,
        ) -> Result<(), ConsumableError> {
            Ok(())
        }

        fn get_description(&self) -> String {
            "Enhanced mock consumable with validation".to_string()
        }

        fn get_target_type(&self) -> TargetType {
            TargetType::HandType
        }

        fn get_effect_category(&self) -> ConsumableEffect {
            ConsumableEffect::Modification
        }
    }

    let consumable = EnhancedMockConsumable;
    let game = Game::default();

    // Test validation methods
    assert!(consumable.can_use(&game, &Target::None));
    assert!(consumable.can_use(
        &game,
        &Target::HandType(balatro_rs::rank::HandRank::OnePair)
    ));

    // Test metadata methods
    assert_eq!(consumable.get_target_type(), TargetType::HandType);
    assert_eq!(
        consumable.get_effect_category(),
        ConsumableEffect::Modification
    );
    assert!(consumable.get_description().contains("Enhanced mock"));
}
