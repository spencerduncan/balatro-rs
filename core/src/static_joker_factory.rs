use crate::joker::{JokerId, JokerRarity};
use crate::static_joker::{StaticJoker, StaticCondition};
use crate::card::{Suit, Value};
use crate::rank::HandRank;

/// Factory functions for creating static jokers using the framework
pub struct StaticJokerFactory;

impl StaticJokerFactory {
    /// Create the basic Joker (+4 Mult)
    pub fn create_joker() -> Box<StaticJoker> {
        Box::new(
            StaticJoker::builder(JokerId::Joker, "Joker", "+4 Mult")
                .rarity(JokerRarity::Common)
                .cost(2)
                .mult(4)
                .per_hand()
                .build()
        )
    }

    /// Create Greedy Joker (Diamond cards give +3 Mult when scored)
    pub fn create_greedy_joker() -> Box<StaticJoker> {
        Box::new(
            StaticJoker::builder(
                JokerId::GreedyJoker,
                "Greedy Joker",
                "Played cards with Diamond suit give +3 Mult when scored"
            )
            .rarity(JokerRarity::Common)
            .cost(5)
            .mult(3)
            .condition(StaticCondition::SuitScored(Suit::Diamond))
            .per_card()
            .build()
        )
    }

    /// Create Lusty Joker (Heart cards give +3 Mult when scored)
    pub fn create_lusty_joker() -> Box<StaticJoker> {
        Box::new(
            StaticJoker::builder(
                JokerId::LustyJoker,
                "Lusty Joker",
                "Played cards with Heart suit give +3 Mult when scored"
            )
            .rarity(JokerRarity::Common)
            .cost(5)
            .mult(3)
            .condition(StaticCondition::SuitScored(Suit::Heart))
            .per_card()
            .build()
        )
    }

    /// Create Wrathful Joker (Spade cards give +3 Mult when scored)
    pub fn create_wrathful_joker() -> Box<StaticJoker> {
        Box::new(
            StaticJoker::builder(
                JokerId::WrathfulJoker,
                "Wrathful Joker",
                "Played cards with Spade suit give +3 Mult when scored"
            )
            .rarity(JokerRarity::Common)
            .cost(5)
            .mult(3)
            .condition(StaticCondition::SuitScored(Suit::Spade))
            .per_card()
            .build()
        )
    }

    /// Create Gluttonous Joker (Club cards give +3 Mult when scored)
    pub fn create_gluttonous_joker() -> Box<StaticJoker> {
        Box::new(
            StaticJoker::builder(
                JokerId::GluttonousJoker,
                "Gluttonous Joker",
                "Played cards with Club suit give +3 Mult when scored"
            )
            .rarity(JokerRarity::Common)
            .cost(5)
            .mult(3)
            .condition(StaticCondition::SuitScored(Suit::Club))
            .per_card()
            .build()
        )
    }

    /// Create Jolly Joker (+8 Mult if played hand contains a Pair)
    pub fn create_jolly_joker() -> Box<StaticJoker> {
        Box::new(
            StaticJoker::builder(
                JokerId::JollyJoker,
                "Jolly Joker",
                "+8 Mult if played hand contains a Pair"
            )
            .rarity(JokerRarity::Common)
            .cost(3)
            .mult(8)
            .condition(StaticCondition::HandType(HandRank::OnePair))
            .per_hand()
            .build()
        )
    }

    /// Create Zany Joker (+12 Mult if played hand contains a Three of a Kind)
    pub fn create_zany_joker() -> Box<StaticJoker> {
        Box::new(
            StaticJoker::builder(
                JokerId::ZanyJoker,
                "Zany Joker",
                "+12 Mult if played hand contains a Three of a Kind"
            )
            .rarity(JokerRarity::Common)
            .cost(4)
            .mult(12)
            .condition(StaticCondition::HandType(HandRank::ThreeOfAKind))
            .per_hand()
            .build()
        )
    }

    /// Create Mad Joker (+10 Mult if played hand contains a Two Pair)
    pub fn create_mad_joker() -> Box<StaticJoker> {
        Box::new(
            StaticJoker::builder(
                JokerId::MadJoker,
                "Mad Joker",
                "+10 Mult if played hand contains a Two Pair"
            )
            .rarity(JokerRarity::Common)
            .cost(4)
            .mult(10)
            .condition(StaticCondition::HandType(HandRank::TwoPair))
            .per_hand()
            .build()
        )
    }

    /// Create Crazy Joker (+12 Mult if played hand contains a Straight)
    pub fn create_crazy_joker() -> Box<StaticJoker> {
        Box::new(
            StaticJoker::builder(
                JokerId::CrazyJoker,
                "Crazy Joker",
                "+12 Mult if played hand contains a Straight"
            )
            .rarity(JokerRarity::Common)
            .cost(4)
            .mult(12)
            .condition(StaticCondition::HandType(HandRank::Straight))
            .per_hand()
            .build()
        )
    }

    /// Create Droll Joker (+10 Mult if played hand contains a Flush)
    pub fn create_droll_joker() -> Box<StaticJoker> {
        Box::new(
            StaticJoker::builder(
                JokerId::DrollJoker,
                "Droll Joker",
                "+10 Mult if played hand contains a Flush"
            )
            .rarity(JokerRarity::Common)
            .cost(4)
            .mult(10)
            .condition(StaticCondition::HandType(HandRank::Flush))
            .per_hand()
            .build()
        )
    }

    /// Create Sly Joker (+50 Chips if played hand contains a Pair)
    pub fn create_sly_joker() -> Box<StaticJoker> {
        Box::new(
            StaticJoker::builder(
                JokerId::SlyJoker,
                "Sly Joker",
                "+50 Chips if played hand contains a Pair"
            )
            .rarity(JokerRarity::Common)
            .cost(3)
            .chips(50)
            .condition(StaticCondition::HandType(HandRank::OnePair))
            .per_hand()
            .build()
        )
    }

    /// Create Wily Joker (+100 Chips if played hand contains a Three of a Kind)
    pub fn create_wily_joker() -> Box<StaticJoker> {
        Box::new(
            StaticJoker::builder(
                JokerId::WilyJoker,
                "Wily Joker",
                "+100 Chips if played hand contains a Three of a Kind"
            )
            .rarity(JokerRarity::Common)
            .cost(4)
            .chips(100)
            .condition(StaticCondition::HandType(HandRank::ThreeOfAKind))
            .per_hand()
            .build()
        )
    }

    /// Create Clever Joker (+80 Chips if played hand contains a Two Pair)
    pub fn create_clever_joker() -> Box<StaticJoker> {
        Box::new(
            StaticJoker::builder(
                JokerId::CleverJoker,
                "Clever Joker",
                "+80 Chips if played hand contains a Two Pair"
            )
            .rarity(JokerRarity::Common)
            .cost(4)
            .chips(80)
            .condition(StaticCondition::HandType(HandRank::TwoPair))
            .per_hand()
            .build()
        )
    }

    /// Create Devious Joker (+100 Chips if played hand contains a Straight)
    pub fn create_devious_joker() -> Box<StaticJoker> {
        Box::new(
            StaticJoker::builder(
                JokerId::DeviousJoker,
                "Devious Joker",
                "+100 Chips if played hand contains a Straight"
            )
            .rarity(JokerRarity::Common)
            .cost(4)
            .chips(100)
            .condition(StaticCondition::HandType(HandRank::Straight))
            .per_hand()
            .build()
        )
    }

    /// Create Crafty Joker (+80 Chips if played hand contains a Flush)
    pub fn create_crafty_joker() -> Box<StaticJoker> {
        Box::new(
            StaticJoker::builder(
                JokerId::CraftyJoker,
                "Crafty Joker",
                "+80 Chips if played hand contains a Flush"
            )
            .rarity(JokerRarity::Common)
            .cost(4)
            .chips(80)
            .condition(StaticCondition::HandType(HandRank::Flush))
            .per_hand()
            .build()
        )
    }

    /// Create Even Steven (Even cards give +4 Mult when scored)
    pub fn create_even_steven() -> Box<StaticJoker> {
        Box::new(
            StaticJoker::builder(
                JokerId::EvenSteven,
                "Even Steven",
                "Played cards with even rank give +4 Mult when scored"
            )
            .rarity(JokerRarity::Common)
            .cost(4)
            .mult(4)
            .condition(StaticCondition::AnyRankScored(vec![
                Value::Two,
                Value::Four,
                Value::Six,
                Value::Eight,
                Value::Ten
            ]))
            .per_card()
            .build()
        )
    }

    /// Create Odd Todd (Odd cards give +31 Chips when scored)
    pub fn create_odd_todd() -> Box<StaticJoker> {
        Box::new(
            StaticJoker::builder(
                JokerId::OddTodd,
                "Odd Todd",
                "Played cards with odd rank give +31 Chips when scored"
            )
            .rarity(JokerRarity::Common)
            .cost(4)
            .chips(31)
            .condition(StaticCondition::AnyRankScored(vec![
                Value::Three,
                Value::Five,
                Value::Seven,
                Value::Nine,
                Value::Ace
            ]))
            .per_card()
            .build()
        )
    }

    /// Create Scholar (Aces give +20 Chips and +4 Mult when scored)
    pub fn create_scholar() -> Box<StaticJoker> {
        Box::new(
            StaticJoker::builder(
                JokerId::Scholar,
                "Scholar",
                "Played Aces give +20 Chips and +4 Mult when scored"
            )
            .rarity(JokerRarity::Common)
            .cost(4)
            .chips(20)
            .mult(4)
            .condition(StaticCondition::RankScored(Value::Ace))
            .per_card()
            .build()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::joker::Joker;

    #[test]
    fn test_basic_joker_creation() {
        let joker = StaticJokerFactory::create_joker();
        assert_eq!(joker.id(), JokerId::Joker);
        assert_eq!(joker.name(), "Joker");
        assert_eq!(joker.cost(), 2);
    }

    #[test]
    fn test_suit_joker_creation() {
        let greedy = StaticJokerFactory::create_greedy_joker();
        assert_eq!(greedy.id(), JokerId::GreedyJoker);
        assert_eq!(greedy.rarity(), JokerRarity::Common);
        
        let lusty = StaticJokerFactory::create_lusty_joker();
        assert_eq!(lusty.id(), JokerId::LustyJoker);
    }

    #[test]
    fn test_hand_type_joker_creation() {
        let jolly = StaticJokerFactory::create_jolly_joker();
        assert_eq!(jolly.id(), JokerId::JollyJoker);
        assert_eq!(jolly.cost(), 3);
        
        let zany = StaticJokerFactory::create_zany_joker();
        assert_eq!(zany.id(), JokerId::ZanyJoker);
        assert_eq!(zany.cost(), 4);
    }

    #[test]
    fn test_chip_joker_creation() {
        let sly = StaticJokerFactory::create_sly_joker();
        assert_eq!(sly.id(), JokerId::SlyJoker);
        assert_eq!(sly.description(), "+50 Chips if played hand contains a Pair");
    }
}