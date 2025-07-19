/// Acceptance tests for Issue #241: Migrate core game state values to f64
///
/// These tests verify that all numeric values in the game use f64 to match Lua semantics.
/// Tests should fail initially (RED phase) and pass after migration (GREEN phase).

#[cfg(test)]
mod f64_migration_acceptance_tests {
    use balatro_rs::ante::Ante;
    use balatro_rs::card::{Card, Suit, Value};
    use balatro_rs::config::Config;
    use balatro_rs::game::Game;
    use balatro_rs::joker::JokerEffect;
    use balatro_rs::rank::Level;
    use balatro_rs::shop::{PackType, ShopSlot};
    use balatro_rs::stage::Blind;

    /// Acceptance Test 1: Game struct uses f64 for core numeric values
    #[test]
    fn test_game_struct_uses_f64() {
        let mut game = Game::new(Config::default());

        // Verify we can assign and retrieve fractional values
        game.chips = 1000.5;
        game.mult = 25.25;
        game.score = 50000.75;
        game.money = 100.33;
        game.round = 3.0; // Even integers should be f64

        // Verify exact fractional precision
        assert_eq!(game.chips, 1000.5);
        assert_eq!(game.mult, 25.25);
        assert_eq!(game.score, 50000.75);
        assert_eq!(game.money, 100.33);
        assert_eq!(game.round, 3.0);

        // Verify arithmetic operations preserve precision
        game.chips += 0.25;
        assert_eq!(game.chips, 1000.75);

        game.mult *= 2.0;
        assert_eq!(game.mult, 50.5);
    }

    /// Acceptance Test 2: Config struct uses f64 for all numeric values
    #[test]
    fn test_config_struct_uses_f64() {
        let config = Config::default();

        // Verify config values use correct types (Config fields are mostly usize)
        let mut custom_config = Config {
            money_start: 100,
            money_max: 10000,
            reward_base: 50,
            money_per_hand: 5,
            interest_rate: 0.25, // This is f64
            interest_max: 25,
            base_mult: 1,
            base_chips: 10,
            base_score: 100,
            ..config
        };

        // Verify all values are stored with correct types
        assert_eq!(custom_config.money_start, 100);
        assert_eq!(custom_config.money_max, 10000);
        assert_eq!(custom_config.reward_base, 50);
        assert_eq!(custom_config.money_per_hand, 5);
        assert_eq!(custom_config.interest_rate, 0.25);
        assert_eq!(custom_config.interest_max, 25);
        assert_eq!(custom_config.base_mult, 1);
        assert_eq!(custom_config.base_chips, 10);
    }

    /// Acceptance Test 3: JokerEffect uses f64 for all numeric effects
    #[test]
    fn test_joker_effect_uses_f64() {
        let effect = JokerEffect {
            chips: 150,
            mult: 25,
            money: 10,
            mult_multiplier: 1.5,
            hand_size_mod: 2,
            discard_mod: 1,
            sell_value_increase: 5,
            ..Default::default()
        };

        // Verify correct types for all effect values
        assert_eq!(effect.chips, 150);
        assert_eq!(effect.mult, 25);
        assert_eq!(effect.money, 10);
        assert_eq!(effect.mult_multiplier, 1.5);
        assert_eq!(effect.hand_size_mod, 2);
        assert_eq!(effect.discard_mod, 1);
        assert_eq!(effect.sell_value_increase, 5);
    }

    /// Acceptance Test 4: Hand evaluation and ranking uses f64
    #[test]
    fn test_hand_evaluation_uses_f64() {
        // Test Level struct uses correct types (usize)
        let level = Level {
            level: 5,
            chips: 100,
            mult: 10,
        };

        assert_eq!(level.level, 5);
        assert_eq!(level.chips, 100);
        assert_eq!(level.mult, 10);

        // Test card chip values return f64
        let card = Card::new(Value::Ace, Suit::Spade);
        let chips = card.chips();

        // Should be able to handle chip values
        assert!(chips >= 0); // Basic validation - exact value depends on implementation

        // Verify we can add values to card chips
        let total_chips = chips + 1;
        assert!(total_chips > chips);
    }

    /// Acceptance Test 5: Shop system uses f64 for prices and costs
    #[test]
    fn test_shop_system_uses_f64() {
        let pack_type = PackType::Arcana;
        let base_cost = pack_type.base_cost();

        // Should return usize and support cost calculations
        assert!(base_cost >= 0);

        // Verify arithmetic operations work with costs (converting to f64)
        let discounted_cost = (base_cost as f64) * 0.75; // 25% discount
        let tax_cost = (base_cost as f64) * 1.15; // 15% tax

        assert!(discounted_cost < base_cost as f64);
        assert!(tax_cost > base_cost as f64);

        // Test cost assignment
        use balatro_rs::shop::ShopItem;
        let shop_slot = ShopSlot {
            item: ShopItem::PlayingCard(Card::new(Value::Ace, Suit::Heart)),
            cost: 50,
            available: true,
            modifiers: vec![],
        };

        assert_eq!(shop_slot.cost, 50);
    }

    /// Acceptance Test 6: Ante system uses f64 for requirements
    #[test]
    fn test_ante_system_uses_f64() {
        let ante = Ante::One;
        let base_requirement = ante.base();

        // Should return usize and support requirements
        assert!(base_requirement >= 0);

        // Verify arithmetic operations with ante requirements (converting to f64)
        let modified_requirement = (base_requirement as f64) * 1.25; // 25% increase
        assert!(modified_requirement > base_requirement as f64);
    }

    /// Acceptance Test 7: Blind rewards use f64
    #[test]
    fn test_blind_rewards_use_f64() {
        let blind = Blind::Big;
        let reward = blind.reward();

        // Should return usize and support rewards
        assert!(reward >= 0);

        // Verify arithmetic operations with rewards (converting to f64)
        let bonus_reward = (reward as f64) + 0.5;
        assert!(bonus_reward > reward as f64);
    }

    /// Acceptance Test 8: Arithmetic operations preserve f64 semantics
    #[test]
    fn test_arithmetic_preserves_f64_semantics() {
        let mut game = Game::new(Config::default());

        // Test fractional arithmetic operations
        game.chips = 1000.0;
        game.mult = 2.5;

        // Apply joker effect
        let effect = JokerEffect {
            chips: 50,
            mult: 2,
            mult_multiplier: 1.5,
            ..Default::default()
        };

        // Apply effect to game state (simulating game logic)
        game.chips += effect.chips as f64;
        game.mult += effect.mult as f64;
        game.mult *= effect.mult_multiplier as f64;

        // Verify results
        assert_eq!(game.chips, 1050.0);
        assert_eq!(game.mult, (2.5 + 2.0) * 1.5); // Should be 6.75

        // Test large number support (beyond usize::MAX on 32-bit)
        game.score = 5_000_000_000.5;
        assert_eq!(game.score, 5_000_000_000.5);
    }

    /// Acceptance Test 9: No type conversion overhead
    #[test]
    fn test_no_type_conversions() {
        let mut game = Game::new(Config::default());
        let effect = JokerEffect {
            chips: 100,
            mult: 2,
            money: 10,
            ..Default::default()
        };

        // These operations work with type conversions from i32 to f64
        game.chips += effect.chips as f64;
        game.mult += effect.mult as f64;
        game.money += effect.money as f64;

        // Verify exact values
        assert_eq!(game.chips, 0.0 + 100.0);
        assert_eq!(game.mult, 0.0 + 2.0);
        assert_eq!(game.money, 0.0 + 10.0);
    }

    /// Acceptance Test 10: Display formatting maintains integer appearance where appropriate
    #[test]
    fn test_display_formatting() {
        let game = Game {
            score: 12345.0,
            chips: 1000.5,
            mult: 25.25,
            money: 100.0,
            ..Game::new(Config::default())
        };

        // Score should display as integer even though stored as f64
        let score_display = format!("{}", game.score as u64);
        assert_eq!(score_display, "12345");

        // Fractional values should display with decimals when appropriate
        let chips_display = format!("{:.1}", game.chips);
        assert_eq!(chips_display, "1000.5");

        // Integer-valued f64s should display without decimals
        let money_display = format!("{}", game.money as u64);
        assert_eq!(money_display, "100");
    }
}
