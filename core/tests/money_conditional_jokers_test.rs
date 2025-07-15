use balatro_rs::card::{Card, Suit, Value};
use balatro_rs::hand::Hand;
use balatro_rs::joker::{GameContext, Joker, JokerId, JokerRarity};
use balatro_rs::joker_impl::{Burglar, BusinessCard, Egg};
use balatro_rs::stage::{Blind, Stage};
use std::sync::OnceLock;

fn create_test_context() -> GameContext<'static> {
    static STAGE: Stage = Stage::Blind(Blind::Small);
    // Use OnceLock to initialize Hand lazily in a const context
    static HAND: OnceLock<Hand> = OnceLock::new();
    let hand = HAND.get_or_init(|| {
        // Create an empty hand using the safe constructor
        Hand::new(Vec::new())
    });

    GameContext {
        chips: 0,
        mult: 1,
        money: 10,
        ante: 1,
        round: 1,
        stage: &STAGE,
        hands_played: 0,
        discards_used: 0,
        jokers: &[],
        hand,
        discarded: &[],
    }
}

#[cfg(test)]
mod business_card_tests {
    use super::*;

    #[test]
    fn test_business_card_basic_properties() {
        let joker = BusinessCard::default();

        assert_eq!(joker.id(), JokerId::BusinessCard);
        assert_eq!(joker.name(), "Business Card");
        assert_eq!(joker.description(), "Face cards give $2 when scored");
        assert_eq!(joker.rarity(), JokerRarity::Common);
        assert_eq!(joker.cost(), 4);
    }

    #[test]
    fn test_business_card_face_card_gives_money() {
        let joker = BusinessCard::default();
        let mut context = create_test_context();

        // Test each face card gives money
        let jack = Card::new(Value::Jack, Suit::Heart);
        let effect = joker.on_card_scored(&mut context, &jack);
        assert_eq!(effect.money, 2);

        let queen = Card::new(Value::Queen, Suit::Diamond);
        let effect = joker.on_card_scored(&mut context, &queen);
        assert_eq!(effect.money, 2);

        let king = Card::new(Value::King, Suit::Club);
        let effect = joker.on_card_scored(&mut context, &king);
        assert_eq!(effect.money, 2);
    }

    #[test]
    fn test_business_card_non_face_card_no_money() {
        let joker = BusinessCard::default();
        let mut context = create_test_context();

        // Test non-face cards don't give money
        let two = Card::new(Value::Two, Suit::Heart);
        let effect = joker.on_card_scored(&mut context, &two);
        assert_eq!(effect.money, 0);

        let ace = Card::new(Value::Ace, Suit::Spade);
        let effect = joker.on_card_scored(&mut context, &ace);
        assert_eq!(effect.money, 0);

        let ten = Card::new(Value::Ten, Suit::Diamond);
        let effect = joker.on_card_scored(&mut context, &ten);
        assert_eq!(effect.money, 0);
    }

    #[test]
    fn test_business_card_no_other_effects() {
        let joker = BusinessCard::default();
        let mut context = create_test_context();

        let jack = Card::new(Value::Jack, Suit::Heart);
        let effect = joker.on_card_scored(&mut context, &jack);

        // Should only give money, no other effects
        assert_eq!(effect.chips, 0);
        assert_eq!(effect.mult, 0);
        assert_eq!(effect.mult_multiplier, 0.0);
        assert_eq!(effect.retrigger, 0);
        assert!(!effect.destroy_self);
        assert!(effect.destroy_others.is_empty());
    }

    #[test]
    fn test_business_card_all_suits() {
        let joker = BusinessCard::default();
        let mut context = create_test_context();

        // Test face cards from all suits give money
        for suit in [Suit::Heart, Suit::Diamond, Suit::Club, Suit::Spade] {
            let jack = Card::new(Value::Jack, suit);
            let effect = joker.on_card_scored(&mut context, &jack);
            assert_eq!(effect.money, 2, "Jack of {:?} should give $2", suit);
        }
    }
}

#[cfg(test)]
mod egg_tests {
    use super::*;

    #[test]
    fn test_egg_basic_properties() {
        let joker = Egg::default();

        assert_eq!(joker.id(), JokerId::EggJoker);
        assert_eq!(joker.name(), "Egg");
        assert_eq!(joker.description(), "Gains $3 sell value at end of round");
        assert_eq!(joker.rarity(), JokerRarity::Common);
        assert_eq!(joker.cost(), 3);
    }

    #[test]
    fn test_egg_round_end_hook() {
        let joker = Egg::default();
        let mut context = create_test_context();

        let effect = joker.on_round_end(&mut context);

        // Should increase sell value by $3 and show message
        assert_eq!(effect.sell_value_increase, 3);
        assert_eq!(effect.message, Some("Egg gained $3 sell value".to_string()));

        // Should not affect other game values
        assert_eq!(effect.money, 0);
        assert_eq!(effect.chips, 0);
        assert_eq!(effect.mult, 0);
    }

    #[test]
    fn test_egg_sell_value() {
        let joker = Egg::default();

        // Test base sell value (cost / 2 = 3 / 2 = 1)
        assert_eq!(joker.sell_value(0.0), 1);

        // Test with accumulated bonus
        assert_eq!(joker.sell_value(3.0), 4); // Base 1 + 3 bonus
        assert_eq!(joker.sell_value(9.0), 10); // Base 1 + 9 bonus

        // Test with fractional bonus (should truncate)
        assert_eq!(joker.sell_value(2.7), 3); // Base 1 + 2 bonus (truncated)
    }

    #[test]
    fn test_egg_no_other_hooks() {
        let joker = Egg::default();
        let mut context = create_test_context();

        // Should not respond to other game events
        let card = Card::new(Value::Ace, Suit::Heart);
        let effect = joker.on_card_scored(&mut context, &card);
        assert_eq!(effect.money, 0);

        let effect = joker.on_blind_start(&mut context);
        assert_eq!(effect.money, 0);

        let effect = joker.on_shop_open(&mut context);
        assert_eq!(effect.money, 0);
    }
}

#[cfg(test)]
mod burglar_tests {
    use super::*;

    #[test]
    fn test_burglar_basic_properties() {
        let joker = Burglar::default();

        assert_eq!(joker.id(), JokerId::Burglar);
        assert_eq!(joker.name(), "Burglar");
        assert_eq!(joker.description(), "Gain $3 when Blind selected, +1 hands");
        assert_eq!(joker.rarity(), JokerRarity::Uncommon);
        assert_eq!(joker.cost(), 6);
    }

    #[test]
    fn test_burglar_blind_start_gives_money() {
        let joker = Burglar::default();
        let mut context = create_test_context();

        let effect = joker.on_blind_start(&mut context);
        assert_eq!(effect.money, 3);

        // Should only give money, no other effects
        assert_eq!(effect.chips, 0);
        assert_eq!(effect.mult, 0);
        assert_eq!(effect.mult_multiplier, 0.0);
        assert_eq!(effect.retrigger, 0);
        assert!(!effect.destroy_self);
        assert!(effect.destroy_others.is_empty());
    }

    #[test]
    fn test_burglar_increases_hand_size() {
        let joker = Burglar::default();
        let context = create_test_context();

        // Test hand size modifier
        let base_hand_size = 8;
        let modified_size = joker.modify_hand_size(&context, base_hand_size);
        assert_eq!(modified_size, 9);

        // Test with different base sizes
        assert_eq!(joker.modify_hand_size(&context, 5), 6);
        assert_eq!(joker.modify_hand_size(&context, 10), 11);
        assert_eq!(joker.modify_hand_size(&context, 0), 1);
    }

    #[test]
    fn test_burglar_no_other_hooks() {
        let joker = Burglar::default();
        let mut context = create_test_context();

        // Should not respond to other game events (except blind start)
        let card = Card::new(Value::Ace, Suit::Heart);
        let effect = joker.on_card_scored(&mut context, &card);
        assert_eq!(effect.money, 0);

        let effect = joker.on_round_end(&mut context);
        assert_eq!(effect.money, 0);

        let effect = joker.on_shop_open(&mut context);
        assert_eq!(effect.money, 0);
    }

    #[test]
    fn test_burglar_other_modifiers_unchanged() {
        let joker = Burglar::default();
        let context = create_test_context();

        // Should not modify other game aspects
        assert_eq!(joker.modify_chips(&context, 100), 100);
        assert_eq!(joker.modify_mult(&context, 5), 5);
        assert_eq!(joker.modify_discards(&context, 3), 3);
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_multiple_jokers_interactions() {
        let business_card = BusinessCard::default();
        let burglar = Burglar::default();
        let mut context = create_test_context();

        // Test burglar gives money on blind start
        let burglar_effect = burglar.on_blind_start(&mut context);
        assert_eq!(burglar_effect.money, 3);

        // Test business card gives money on face card scoring
        let jack = Card::new(Value::Jack, Suit::Heart);
        let business_effect = business_card.on_card_scored(&mut context, &jack);
        assert_eq!(business_effect.money, 2);

        // Test burglar modifies hand size
        let base_size = 8;
        let modified_size = burglar.modify_hand_size(&context, base_size);
        assert_eq!(modified_size, 9);
    }

    #[test]
    fn test_money_conditional_logic() {
        // Test that the jokers work regardless of current money state
        let business_card = BusinessCard::default();

        // Low money scenario
        let mut low_money_context = create_test_context();
        low_money_context.money = 1;

        let jack = Card::new(Value::Jack, Suit::Heart);
        let effect = business_card.on_card_scored(&mut low_money_context, &jack);
        assert_eq!(effect.money, 2);

        // High money scenario
        let mut high_money_context = create_test_context();
        high_money_context.money = 100;

        let effect = business_card.on_card_scored(&mut high_money_context, &jack);
        assert_eq!(effect.money, 2);
    }

    #[test]
    fn test_joker_traits_consistency() {
        let jokers: Vec<Box<dyn Joker>> = vec![
            Box::new(BusinessCard::default()),
            Box::new(Egg::default()),
            Box::new(Burglar::default()),
        ];

        for joker in jokers {
            // All should have valid IDs
            assert_ne!(format!("{:?}", joker.id()), "");

            // All should have non-empty names and descriptions
            assert!(!joker.name().is_empty());
            assert!(!joker.description().is_empty());

            // All should have reasonable costs
            assert!(joker.cost() > 0);
            assert!(joker.cost() <= 20);
        }
    }
}
