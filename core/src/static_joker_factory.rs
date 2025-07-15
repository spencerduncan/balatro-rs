use crate::card::{Suit, Value};
use crate::joker::{Joker, JokerId, JokerRarity};
use crate::rank::HandRank;
use crate::static_joker::{StaticCondition, StaticJoker};

/// Factory functions for creating static jokers using the framework
pub struct StaticJokerFactory;

impl StaticJokerFactory {
    /// Create the basic Joker (+4 Mult)
    pub fn create_joker() -> Box<dyn Joker> {
        Box::new(
            StaticJoker::builder(JokerId::Joker, "Joker", "+4 Mult")
                .rarity(JokerRarity::Common)
                .cost(2)
                .mult(4)
                .per_hand()
                .build()
                .expect("Valid joker configuration"),
        )
    }

    /// Create Greedy Joker (Diamond cards give +3 Mult when scored)
    pub fn create_greedy_joker() -> Box<dyn Joker> {
        Box::new(
            StaticJoker::builder(
                JokerId::GreedyJoker,
                "Greedy Joker",
                "Played cards with Diamond suit give +3 Mult when scored",
            )
            .rarity(JokerRarity::Common)
            .cost(5)
            .mult(3)
            .condition(StaticCondition::SuitScored(Suit::Diamond))
            .per_card()
            .build()
            .expect("Valid joker configuration"),
        )
    }

    /// Create Lusty Joker (Heart cards give +3 Mult when scored)
    pub fn create_lusty_joker() -> Box<dyn Joker> {
        Box::new(
            StaticJoker::builder(
                JokerId::LustyJoker,
                "Lusty Joker",
                "Played cards with Heart suit give +3 Mult when scored",
            )
            .rarity(JokerRarity::Common)
            .cost(5)
            .mult(3)
            .condition(StaticCondition::SuitScored(Suit::Heart))
            .per_card()
            .build()
            .expect("Valid joker configuration"),
        )
    }

    /// Create Wrathful Joker (Spade cards give +3 Mult when scored)
    pub fn create_wrathful_joker() -> Box<dyn Joker> {
        Box::new(
            StaticJoker::builder(
                JokerId::WrathfulJoker,
                "Wrathful Joker",
                "Played cards with Spade suit give +3 Mult when scored",
            )
            .rarity(JokerRarity::Common)
            .cost(5)
            .mult(3)
            .condition(StaticCondition::SuitScored(Suit::Spade))
            .per_card()
            .build()
            .expect("Valid joker configuration"),
        )
    }

    /// Create Gluttonous Joker (Club cards give +3 Mult when scored)
    pub fn create_gluttonous_joker() -> Box<dyn Joker> {
        Box::new(
            StaticJoker::builder(
                JokerId::GluttonousJoker,
                "Gluttonous Joker",
                "Played cards with Club suit give +3 Mult when scored",
            )
            .rarity(JokerRarity::Common)
            .cost(5)
            .mult(3)
            .condition(StaticCondition::SuitScored(Suit::Club))
            .per_card()
            .build()
            .expect("Valid joker configuration"),
        )
    }

    /// Create Jolly Joker (+8 Mult if played hand contains Pair)
    pub fn create_jolly_joker() -> Box<dyn Joker> {
        Box::new(
            StaticJoker::builder(
                JokerId::JollyJoker,
                "Jolly Joker",
                "+8 Mult if played hand contains Pair",
            )
            .rarity(JokerRarity::Common)
            .cost(3)
            .mult(8)
            .condition(StaticCondition::HandType(HandRank::OnePair))
            .per_hand()
            .build()
            .expect("Valid joker configuration"),
        )
    }

    /// Create Zany Joker (+12 Mult if played hand contains Three of a Kind)
    pub fn create_zany_joker() -> Box<dyn Joker> {
        Box::new(
            StaticJoker::builder(
                JokerId::ZanyJoker,
                "Zany Joker",
                "+12 Mult if played hand contains Three of a Kind",
            )
            .rarity(JokerRarity::Common)
            .cost(4)
            .mult(12)
            .condition(StaticCondition::HandType(HandRank::ThreeOfAKind))
            .per_hand()
            .build()
            .expect("Valid joker configuration"),
        )
    }

    /// Create Mad Joker (+10 Mult if played hand contains Two Pair)
    pub fn create_mad_joker() -> Box<dyn Joker> {
        Box::new(
            StaticJoker::builder(
                JokerId::MadJoker,
                "Mad Joker",
                "+10 Mult if played hand contains Two Pair",
            )
            .rarity(JokerRarity::Common)
            .cost(4)
            .mult(10)
            .condition(StaticCondition::HandType(HandRank::TwoPair))
            .per_hand()
            .build()
            .expect("Valid joker configuration"),
        )
    }

    /// Create Crazy Joker (+12 Mult if played hand contains Straight)
    pub fn create_crazy_joker() -> Box<dyn Joker> {
        Box::new(
            StaticJoker::builder(
                JokerId::CrazyJoker,
                "Crazy Joker",
                "+12 Mult if played hand contains Straight",
            )
            .rarity(JokerRarity::Common)
            .cost(4)
            .mult(12)
            .condition(StaticCondition::HandType(HandRank::Straight))
            .per_hand()
            .build()
            .expect("Valid joker configuration"),
        )
    }

    /// Create Droll Joker (+10 Mult if played hand contains Flush)
    pub fn create_droll_joker() -> Box<dyn Joker> {
        Box::new(
            StaticJoker::builder(
                JokerId::DrollJoker,
                "Droll Joker",
                "+10 Mult if played hand contains Flush",
            )
            .rarity(JokerRarity::Common)
            .cost(4)
            .mult(10)
            .condition(StaticCondition::HandType(HandRank::Flush))
            .per_hand()
            .build()
            .expect("Valid joker configuration"),
        )
    }

    /// Create Sly Joker (+50 Chips if played hand contains Pair)
    pub fn create_sly_joker() -> Box<dyn Joker> {
        Box::new(
            StaticJoker::builder(
                JokerId::SlyJoker,
                "Sly Joker",
                "+50 Chips if played hand contains Pair",
            )
            .rarity(JokerRarity::Common)
            .cost(3)
            .chips(50)
            .condition(StaticCondition::HandType(HandRank::OnePair))
            .per_hand()
            .build()
            .expect("Valid joker configuration"),
        )
    }

    /// Create Wily Joker (+100 Chips if played hand contains Three of a Kind)
    pub fn create_wily_joker() -> Box<dyn Joker> {
        Box::new(
            StaticJoker::builder(
                JokerId::WilyJoker,
                "Wily Joker",
                "+100 Chips if played hand contains Three of a Kind",
            )
            .rarity(JokerRarity::Common)
            .cost(4)
            .chips(100)
            .condition(StaticCondition::HandType(HandRank::ThreeOfAKind))
            .per_hand()
            .build()
            .expect("Valid joker configuration"),
        )
    }

    /// Create Clever Joker (+80 Chips if played hand contains Two Pair)
    pub fn create_clever_joker() -> Box<dyn Joker> {
        Box::new(
            StaticJoker::builder(
                JokerId::CleverJoker,
                "Clever Joker",
                "+80 Chips if played hand contains Two Pair",
            )
            .rarity(JokerRarity::Common)
            .cost(4)
            .chips(80)
            .condition(StaticCondition::HandType(HandRank::TwoPair))
            .per_hand()
            .build()
            .expect("Valid joker configuration"),
        )
    }

    /// Create Devious Joker (+100 Chips if played hand contains Straight)
    pub fn create_devious_joker() -> Box<dyn Joker> {
        Box::new(
            StaticJoker::builder(
                JokerId::DeviousJoker,
                "Devious Joker",
                "+100 Chips if played hand contains Straight",
            )
            .rarity(JokerRarity::Common)
            .cost(4)
            .chips(100)
            .condition(StaticCondition::HandType(HandRank::Straight))
            .per_hand()
            .build()
            .expect("Valid joker configuration"),
        )
    }

    /// Create Crafty Joker (+80 Chips if played hand contains Flush)
    pub fn create_crafty_joker() -> Box<dyn Joker> {
        Box::new(
            StaticJoker::builder(
                JokerId::CraftyJoker,
                "Crafty Joker",
                "+80 Chips if played hand contains Flush",
            )
            .rarity(JokerRarity::Common)
            .cost(4)
            .chips(80)
            .condition(StaticCondition::HandType(HandRank::Flush))
            .per_hand()
            .build()
            .expect("Valid joker configuration"),
        )
    }

    /// Create Even Steven (Even cards (2, 4, 6, 8, 10) give +4 Mult when scored)
    pub fn create_even_steven() -> Box<dyn Joker> {
        Box::new(
            StaticJoker::builder(
                JokerId::EvenSteven,
                "Even Steven",
                "Played cards with even rank (2, 4, 6, 8, 10) give +4 Mult when scored",
            )
            .rarity(JokerRarity::Common)
            .cost(4)
            .mult(4)
            .condition(StaticCondition::AnyRankScored(vec![
                Value::Two,
                Value::Four,
                Value::Six,
                Value::Eight,
                Value::Ten,
            ]))
            .per_card()
            .build()
            .expect("Valid joker configuration"),
        )
    }

    /// Create Odd Todd (Odd cards (3, 5, 7, 9, A) give +31 Chips when scored)
    pub fn create_odd_todd() -> Box<dyn Joker> {
        Box::new(
            StaticJoker::builder(
                JokerId::OddTodd,
                "Odd Todd",
                "Played cards with odd rank (3, 5, 7, 9, A) give +31 Chips when scored",
            )
            .rarity(JokerRarity::Common)
            .cost(4)
            .chips(31)
            .condition(StaticCondition::AnyRankScored(vec![
                Value::Three,
                Value::Five,
                Value::Seven,
                Value::Nine,
                Value::Ace,
            ]))
            .per_card()
            .build()
            .expect("Valid joker configuration"),
        )
    }

    /// Create Scholar (Aces give +20 Chips and +4 Mult when scored)
    pub fn create_scholar() -> Box<dyn Joker> {
        Box::new(
            StaticJoker::builder(
                JokerId::Scholar,
                "Scholar",
                "Played Aces give +20 Chips and +4 Mult when scored",
            )
            .rarity(JokerRarity::Common)
            .cost(4)
            .chips(20)
            .mult(4)
            .condition(StaticCondition::RankScored(Value::Ace))
            .per_card()
            .build()
            .expect("Valid joker configuration"),
        )
    }

    /// Test-only methods that return concrete types for internal testing
    #[cfg(test)]
    pub fn create_greedy_joker_concrete() -> StaticJoker {
        StaticJoker::builder(
            JokerId::GreedyJoker,
            "Greedy Joker",
            "Played cards with Diamond suit give +3 Mult when scored",
        )
        .rarity(JokerRarity::Common)
        .cost(5)
        .mult(3)
        .condition(StaticCondition::SuitScored(Suit::Diamond))
        .per_card()
        .build()
        .expect("Valid joker configuration")
    }

    /// Test-only methods that return concrete types for internal testing
    #[cfg(test)]
    pub fn create_lusty_joker_concrete() -> StaticJoker {
        StaticJoker::builder(
            JokerId::LustyJoker,
            "Lusty Joker",
            "Played cards with Heart suit give +3 Mult when scored",
        )
        .rarity(JokerRarity::Common)
        .cost(5)
        .mult(3)
        .condition(StaticCondition::SuitScored(Suit::Heart))
        .per_card()
        .build()
        .expect("Valid joker configuration")
    }

    /// Test-only methods that return concrete types for internal testing
    #[cfg(test)]
    pub fn create_wrathful_joker_concrete() -> StaticJoker {
        StaticJoker::builder(
            JokerId::WrathfulJoker,
            "Wrathful Joker",
            "Played cards with Spade suit give +3 Mult when scored",
        )
        .rarity(JokerRarity::Common)
        .cost(5)
        .mult(3)
        .condition(StaticCondition::SuitScored(Suit::Spade))
        .per_card()
        .build()
        .expect("Valid joker configuration")
    }

    /// Test-only methods that return concrete types for internal testing
    #[cfg(test)]
    pub fn create_gluttonous_joker_concrete() -> StaticJoker {
        StaticJoker::builder(
            JokerId::GluttonousJoker,
            "Gluttonous Joker",
            "Played cards with Club suit give +3 Mult when scored",
        )
        .rarity(JokerRarity::Common)
        .cost(5)
        .mult(3)
        .condition(StaticCondition::SuitScored(Suit::Club))
        .per_card()
        .build()
        .expect("Valid joker configuration")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_joker_creation() {
        let joker = StaticJokerFactory::create_joker();
        assert_eq!(joker.id(), JokerId::Joker);
        assert_eq!(joker.name(), "Joker");
        assert_eq!(joker.cost(), 2);
        assert_eq!(joker.description(), "+4 Mult");
        assert_eq!(joker.rarity(), JokerRarity::Common);
    }

    #[test]
    fn test_all_suit_jokers() {
        // Test Greedy Joker (Diamond)
        let greedy = StaticJokerFactory::create_greedy_joker();
        assert_eq!(greedy.id(), JokerId::GreedyJoker);
        assert_eq!(greedy.name(), "Greedy Joker");
        assert_eq!(
            greedy.description(),
            "Played cards with Diamond suit give +3 Mult when scored"
        );
        assert_eq!(greedy.rarity(), JokerRarity::Common);
        assert_eq!(greedy.cost(), 5);

        // Test Lusty Joker (Heart)
        let lusty = StaticJokerFactory::create_lusty_joker();
        assert_eq!(lusty.id(), JokerId::LustyJoker);
        assert_eq!(lusty.name(), "Lusty Joker");
        assert_eq!(
            lusty.description(),
            "Played cards with Heart suit give +3 Mult when scored"
        );
        assert_eq!(lusty.rarity(), JokerRarity::Common);

        // Test Wrathful Joker (Spade)
        let wrathful = StaticJokerFactory::create_wrathful_joker();
        assert_eq!(wrathful.id(), JokerId::WrathfulJoker);
        assert_eq!(wrathful.name(), "Wrathful Joker");
        assert_eq!(
            wrathful.description(),
            "Played cards with Spade suit give +3 Mult when scored"
        );

        // Test Gluttonous Joker (Club)
        let gluttonous = StaticJokerFactory::create_gluttonous_joker();
        assert_eq!(gluttonous.id(), JokerId::GluttonousJoker);
        assert_eq!(gluttonous.name(), "Gluttonous Joker");
        assert_eq!(
            gluttonous.description(),
            "Played cards with Club suit give +3 Mult when scored"
        );
    }

    #[test]
    fn test_all_hand_type_mult_jokers() {
        // Test Jolly Joker (Pair)
        let jolly = StaticJokerFactory::create_jolly_joker();
        assert_eq!(jolly.id(), JokerId::JollyJoker);
        assert_eq!(jolly.name(), "Jolly Joker");
        assert_eq!(jolly.description(), "+8 Mult if played hand contains Pair");
        assert_eq!(jolly.cost(), 3);

        // Test Zany Joker (Three of a Kind)
        let zany = StaticJokerFactory::create_zany_joker();
        assert_eq!(zany.id(), JokerId::ZanyJoker);
        assert_eq!(zany.name(), "Zany Joker");
        assert_eq!(
            zany.description(),
            "+12 Mult if played hand contains Three of a Kind"
        );
        assert_eq!(zany.cost(), 4);

        // Test Mad Joker (Two Pair)
        let mad = StaticJokerFactory::create_mad_joker();
        assert_eq!(mad.id(), JokerId::MadJoker);
        assert_eq!(mad.name(), "Mad Joker");
        assert_eq!(
            mad.description(),
            "+10 Mult if played hand contains Two Pair"
        );

        // Test Crazy Joker (Straight)
        let crazy = StaticJokerFactory::create_crazy_joker();
        assert_eq!(crazy.id(), JokerId::CrazyJoker);
        assert_eq!(crazy.name(), "Crazy Joker");
        assert_eq!(
            crazy.description(),
            "+12 Mult if played hand contains Straight"
        );

        // Test Droll Joker (Flush)
        let droll = StaticJokerFactory::create_droll_joker();
        assert_eq!(droll.id(), JokerId::DrollJoker);
        assert_eq!(droll.name(), "Droll Joker");
        assert_eq!(
            droll.description(),
            "+10 Mult if played hand contains Flush"
        );
    }

    #[test]
    fn test_all_hand_type_chip_jokers() {
        // Test Sly Joker (Pair)
        let sly = StaticJokerFactory::create_sly_joker();
        assert_eq!(sly.id(), JokerId::SlyJoker);
        assert_eq!(sly.name(), "Sly Joker");
        assert_eq!(sly.description(), "+50 Chips if played hand contains Pair");

        // Test Wily Joker (Three of a Kind)
        let wily = StaticJokerFactory::create_wily_joker();
        assert_eq!(wily.id(), JokerId::WilyJoker);
        assert_eq!(wily.name(), "Wily Joker");
        assert_eq!(
            wily.description(),
            "+100 Chips if played hand contains Three of a Kind"
        );

        // Test Clever Joker (Two Pair)
        let clever = StaticJokerFactory::create_clever_joker();
        assert_eq!(clever.id(), JokerId::CleverJoker);
        assert_eq!(clever.name(), "Clever Joker");
        assert_eq!(
            clever.description(),
            "+80 Chips if played hand contains Two Pair"
        );

        // Test Devious Joker (Straight)
        let devious = StaticJokerFactory::create_devious_joker();
        assert_eq!(devious.id(), JokerId::DeviousJoker);
        assert_eq!(devious.name(), "Devious Joker");
        assert_eq!(
            devious.description(),
            "+100 Chips if played hand contains Straight"
        );

        // Test Crafty Joker (Flush)
        let crafty = StaticJokerFactory::create_crafty_joker();
        assert_eq!(crafty.id(), JokerId::CraftyJoker);
        assert_eq!(crafty.name(), "Crafty Joker");
        assert_eq!(
            crafty.description(),
            "+80 Chips if played hand contains Flush"
        );
    }

    #[test]
    fn test_rank_based_jokers() {
        // Test Even Steven
        let even_steven = StaticJokerFactory::create_even_steven();
        assert_eq!(even_steven.id(), JokerId::EvenSteven);
        assert_eq!(even_steven.name(), "Even Steven");
        assert_eq!(
            even_steven.description(),
            "Played cards with even rank (2, 4, 6, 8, 10) give +4 Mult when scored"
        );

        // Test Odd Todd
        let odd_todd = StaticJokerFactory::create_odd_todd();
        assert_eq!(odd_todd.id(), JokerId::OddTodd);
        assert_eq!(odd_todd.name(), "Odd Todd");
        assert_eq!(
            odd_todd.description(),
            "Played cards with odd rank (3, 5, 7, 9, A) give +31 Chips when scored"
        );

        // Test Scholar
        let scholar = StaticJokerFactory::create_scholar();
        assert_eq!(scholar.id(), JokerId::Scholar);
        assert_eq!(scholar.name(), "Scholar");
        assert_eq!(
            scholar.description(),
            "Played Aces give +20 Chips and +4 Mult when scored"
        );
    }

    #[test]
    fn test_joker_cost_distribution() {
        // Test that jokers have appropriate costs based on rarity/power
        let basic_jokers = vec![
            StaticJokerFactory::create_joker(), // 2
        ];

        let mid_tier_jokers = vec![
            StaticJokerFactory::create_jolly_joker(), // 3
            StaticJokerFactory::create_sly_joker(),   // 3
        ];

        let higher_tier_jokers = vec![
            StaticJokerFactory::create_zany_joker(), // 4
            StaticJokerFactory::create_wily_joker(), // 4
        ];

        let suit_jokers = vec![
            StaticJokerFactory::create_greedy_joker(),     // 5
            StaticJokerFactory::create_lusty_joker(),      // 5
            StaticJokerFactory::create_wrathful_joker(),   // 5
            StaticJokerFactory::create_gluttonous_joker(), // 5
        ];

        // Verify cost progression
        for joker in basic_jokers {
            assert_eq!(joker.cost(), 2);
        }

        for joker in mid_tier_jokers {
            assert_eq!(joker.cost(), 3);
        }

        for joker in higher_tier_jokers {
            assert_eq!(joker.cost(), 4);
        }

        for joker in suit_jokers {
            assert_eq!(joker.cost(), 5);
        }
    }

    #[test]
    fn test_all_factory_jokers_can_be_created() {
        // Test that all 18 jokers can be created without panicking
        let jokers = vec![
            StaticJokerFactory::create_joker(),
            StaticJokerFactory::create_greedy_joker(),
            StaticJokerFactory::create_lusty_joker(),
            StaticJokerFactory::create_wrathful_joker(),
            StaticJokerFactory::create_gluttonous_joker(),
            StaticJokerFactory::create_jolly_joker(),
            StaticJokerFactory::create_zany_joker(),
            StaticJokerFactory::create_mad_joker(),
            StaticJokerFactory::create_crazy_joker(),
            StaticJokerFactory::create_droll_joker(),
            StaticJokerFactory::create_sly_joker(),
            StaticJokerFactory::create_wily_joker(),
            StaticJokerFactory::create_clever_joker(),
            StaticJokerFactory::create_devious_joker(),
            StaticJokerFactory::create_crafty_joker(),
            StaticJokerFactory::create_even_steven(),
            StaticJokerFactory::create_odd_todd(),
            StaticJokerFactory::create_scholar(),
        ];

        assert_eq!(jokers.len(), 18);

        // Ensure all have valid IDs and names
        for joker in &jokers {
            assert!(!joker.name().is_empty());
            assert!(!joker.description().is_empty());
            assert!(joker.cost() > 0);
        }
    }

    #[test]
    fn test_joker_rarity_distribution() {
        // Test that jokers have appropriate rarities
        let common_jokers = vec![
            StaticJokerFactory::create_joker(),
            StaticJokerFactory::create_greedy_joker(),
            StaticJokerFactory::create_lusty_joker(),
            StaticJokerFactory::create_wrathful_joker(),
            StaticJokerFactory::create_gluttonous_joker(),
        ];

        for joker in common_jokers {
            assert_eq!(joker.rarity(), JokerRarity::Common);
        }

        // Verify specific jokers have expected rarities
        let jolly = StaticJokerFactory::create_jolly_joker();
        assert_eq!(jolly.rarity(), JokerRarity::Common);

        let scholar = StaticJokerFactory::create_scholar();
        assert_eq!(scholar.rarity(), JokerRarity::Common);
    }
}
