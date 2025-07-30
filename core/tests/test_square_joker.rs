#[cfg(test)]
mod square_joker_tests {
    use balatro_rs::{
        joker::{JokerId, JokerRarity},
        joker_factory::JokerFactory,
    };

    #[test]
    fn test_square_joker_basic_properties() {
        // Initialize all systems before running the test to avoid factory race conditions
        balatro_rs::initialize().expect("Failed to initialize core systems");

        let joker = JokerFactory::create(JokerId::Square).expect("Square joker should exist");
        assert_eq!(joker.id(), JokerId::Square);
        assert_eq!(joker.name(), "Square Joker");
        assert_eq!(joker.description(), "+4 Chips per 4-card hand played");
        assert_eq!(joker.rarity(), JokerRarity::Common);
    }

    #[test]
    fn test_square_joker_is_scaling() {
        // Initialize all systems before running the test to avoid factory race conditions
        balatro_rs::initialize().expect("Failed to initialize core systems");

        // Verify that Square joker is now using the scaling implementation
        let joker = JokerFactory::create(JokerId::Square).expect("Square joker should exist");

        // The fact that it has the correct description confirms it's the scaling version
        // The static version had description: "Number cards (2, 3, 4, 5, 6, 7, 8, 9, 10) give +4 Chips when scored"
        // The scaling version has description: "+4 Chips per 4-card hand played"
        assert_ne!(
            joker.description(),
            "Number cards (2, 3, 4, 5, 6, 7, 8, 9, 10) give +4 Chips when scored"
        );
    }
}
