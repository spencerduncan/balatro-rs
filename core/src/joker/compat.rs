use crate::card::Suit;
use crate::effect::Effects;
use crate::game::Game;
use crate::hand::MadeHand;
use crate::joker::{Categories, Joker as NewJoker, JokerRarity as Rarity};
use pyo3::pyclass;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::sync::{Arc, Mutex};
use strum::{EnumIter, IntoEnumIterator};

/// Old-style Joker trait for compatibility
pub trait Joker: std::fmt::Debug + Clone {
    fn name(&self) -> String;
    fn desc(&self) -> String;
    fn cost(&self) -> usize;
    fn rarity(&self) -> Rarity;
    fn categories(&self) -> Vec<Categories>;
    fn effects(&self, game: &Game) -> Vec<Effects>;
}

// Macro to create joker wrapper structs
macro_rules! impl_joker_wrapper {
    ($name:ident, $category:expr, $effect:expr) => {
        #[derive(Debug, Clone, Default, Eq, PartialEq, Hash, Serialize, Deserialize)]
        #[cfg_attr(feature = "python", pyclass(eq))]
        pub struct $name {}

        impl Joker for $name {
            fn name(&self) -> String {
                let joker = crate::joker_impl::$name;
                NewJoker::name(&joker).to_string()
            }
            fn desc(&self) -> String {
                let joker = crate::joker_impl::$name;
                NewJoker::description(&joker).to_string()
            }
            fn cost(&self) -> usize {
                let joker = crate::joker_impl::$name;
                NewJoker::cost(&joker)
            }
            fn rarity(&self) -> Rarity {
                let joker = crate::joker_impl::$name;
                NewJoker::rarity(&joker)
            }
            fn categories(&self) -> Vec<Categories> {
                vec![$category]
            }
            fn effects(&self, _game: &Game) -> Vec<Effects> {
                vec![Effects::OnScore(Arc::new(Mutex::new($effect)))]
            }
        }
    };
}

// Implement all the joker wrappers
impl_joker_wrapper!(
    TheJoker,
    Categories::MultPlus,
    |g: &mut Game, _hand: MadeHand| {
        g.mult += 4;
    }
);

impl_joker_wrapper!(
    GreedyJoker,
    Categories::MultPlus,
    |g: &mut Game, hand: MadeHand| {
        let diamonds = hand
            .hand
            .suits()
            .iter()
            .filter(|s| **s == Suit::Diamond)
            .count();
        g.mult += diamonds * 3
    }
);

impl_joker_wrapper!(
    LustyJoker,
    Categories::MultPlus,
    |g: &mut Game, hand: MadeHand| {
        let hearts = hand
            .hand
            .suits()
            .iter()
            .filter(|s| **s == Suit::Heart)
            .count();
        g.mult += hearts * 3
    }
);

impl_joker_wrapper!(
    WrathfulJoker,
    Categories::MultPlus,
    |g: &mut Game, hand: MadeHand| {
        let spades = hand
            .hand
            .suits()
            .iter()
            .filter(|s| **s == Suit::Spade)
            .count();
        g.mult += spades * 3
    }
);

impl_joker_wrapper!(
    GluttonousJoker,
    Categories::MultPlus,
    |g: &mut Game, hand: MadeHand| {
        let clubs = hand
            .hand
            .suits()
            .iter()
            .filter(|s| **s == Suit::Club)
            .count();
        g.mult += clubs * 3
    }
);

impl_joker_wrapper!(
    JollyJoker,
    Categories::MultPlus,
    |g: &mut Game, hand: MadeHand| {
        if hand.hand.is_pair().is_some() {
            g.mult += 8
        }
    }
);

impl_joker_wrapper!(
    ZanyJoker,
    Categories::MultPlus,
    |g: &mut Game, hand: MadeHand| {
        if hand.hand.is_three_of_kind().is_some() {
            g.mult += 12
        }
    }
);

impl_joker_wrapper!(
    MadJoker,
    Categories::MultPlus,
    |g: &mut Game, hand: MadeHand| {
        if hand.hand.is_two_pair().is_some() {
            g.mult += 10
        }
    }
);

impl_joker_wrapper!(
    CrazyJoker,
    Categories::MultPlus,
    |g: &mut Game, hand: MadeHand| {
        if hand.hand.is_straight().is_some() {
            g.mult += 12
        }
    }
);

impl_joker_wrapper!(
    DrollJoker,
    Categories::MultPlus,
    |g: &mut Game, hand: MadeHand| {
        if hand.hand.is_flush().is_some() {
            g.mult += 10
        }
    }
);

impl_joker_wrapper!(
    SlyJoker,
    Categories::Chips,
    |g: &mut Game, hand: MadeHand| {
        if hand.hand.is_pair().is_some() {
            g.chips += 50
        }
    }
);

impl_joker_wrapper!(
    WilyJoker,
    Categories::Chips,
    |g: &mut Game, hand: MadeHand| {
        if hand.hand.is_three_of_kind().is_some() {
            g.chips += 100
        }
    }
);

impl_joker_wrapper!(
    CleverJoker,
    Categories::Chips,
    |g: &mut Game, hand: MadeHand| {
        if hand.hand.is_two_pair().is_some() {
            g.chips += 80
        }
    }
);

impl_joker_wrapper!(
    DeviousJoker,
    Categories::Chips,
    |g: &mut Game, hand: MadeHand| {
        if hand.hand.is_straight().is_some() {
            g.chips += 100
        }
    }
);

impl_joker_wrapper!(
    CraftyJoker,
    Categories::Chips,
    |g: &mut Game, hand: MadeHand| {
        if hand.hand.is_flush().is_some() {
            g.chips += 80
        }
    }
);

// Macro to create the enum of all jokers
macro_rules! make_jokers {
    ($($x:ident), *) => {
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        #[cfg_attr(feature = "python", pyclass(eq))]
        #[derive(Debug, Clone, EnumIter, Eq, PartialEq, Hash)]
        pub enum Jokers {
            $(
                $x($x),
            )*
        }

        impl Joker for Jokers {
            fn name(&self) -> String {
                match self {
                    $(
                        Jokers::$x(joker) => joker.name(),
                    )*
                }
            }
            fn desc(&self) -> String {
                match self {
                    $(
                        Jokers::$x(joker) => joker.desc(),
                    )*
                }
            }
            fn cost(&self) -> usize {
                match self {
                    $(
                        Jokers::$x(joker) => joker.cost(),
                    )*
                }
            }
            fn rarity(&self) -> Rarity {
                match self {
                    $(
                        Jokers::$x(joker) => joker.rarity(),
                    )*
                }
            }
            fn categories(&self) -> Vec<Categories> {
                match self {
                    $(
                        Jokers::$x(joker) => joker.categories(),
                    )*
                }
            }
            fn effects(&self, game: &Game) -> Vec<Effects> {
                match self {
                    $(
                        Jokers::$x(joker) => joker.effects(game),
                    )*
                }
            }
        }
    }
}

// Create the Jokers enum
make_jokers!(
    TheJoker,
    GreedyJoker,
    LustyJoker,
    WrathfulJoker,
    GluttonousJoker,
    JollyJoker,
    ZanyJoker,
    MadJoker,
    CrazyJoker,
    DrollJoker,
    SlyJoker,
    WilyJoker,
    CleverJoker,
    DeviousJoker,
    CraftyJoker
);

impl Jokers {
    pub(crate) fn by_rarity(rarity: Rarity) -> Vec<Self> {
        Self::iter().filter(|j| j.rarity() == rarity).collect()
    }
}

impl fmt::Display for Jokers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} [${}, {}] {}",
            self.name(),
            self.cost(),
            self.rarity(),
            self.desc()
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::card::{Card, Suit, Value};
    use crate::game::Game;
    use crate::hand::SelectHand;
    use crate::joker::{JokerId, JokerRarity};
    use crate::joker_factory::JokerFactory;
    use crate::stage::{Blind, Stage};

    use super::*;

    fn score_before_after_joker(joker: Jokers, hand: SelectHand, before: usize, after: usize) {
        let mut g = Game::default();
        g.stage = Stage::Blind(Blind::Small);

        // First score without joker
        let score = g.calc_score(hand.best_hand().unwrap());
        assert_eq!(score, before);

        // Buy (and apply) the joker
        g.money += 1000; // Give adequate money to buy
        g.stage = Stage::Shop();
        g.shop.jokers.push(joker.clone());
        g.buy_joker(joker).unwrap();
        g.stage = Stage::Blind(Blind::Small);
        // Second score with joker applied
        let score = g.calc_score(hand.best_hand().unwrap());
        assert_eq!(score, after);
    }

    #[test]
    fn test_the_joker() {
        let ace = Card::new(Value::Ace, Suit::Heart);
        let hand = SelectHand::new(vec![ace]);

        // Score Ace high without joker
        // High card (level 1) -> 5 chips, 1 mult
        // Played cards (1 ace) -> 11 chips
        // (5 + 11) * (1) = 16
        let before = 16;
        // Score Ace high with the Joker
        // High card (level 1) -> 5 chips, 1 mult
        // Played cards (1 ace) -> 11 chips
        // Joker (The Joker) -> 4 mult
        // (5 + 11) * (1 + 4) = 80
        let after = 80;

        let j = Jokers::TheJoker(TheJoker {});
        score_before_after_joker(j, hand, before, after);
    }

    #[test]
    fn test_lusty_joker() {
        let ah = Card::new(Value::Ace, Suit::Heart);
        let ac = Card::new(Value::Ace, Suit::Club);
        let ad = Card::new(Value::Ace, Suit::Diamond);
        let hand = SelectHand::new(vec![ah, ah, ac, ad]);

        // Score 4ok without joker
        // 4ok (level 1) -> 60 chips, 7 mult
        // Played cards (4 ace) -> 44 chips
        // (60 + 44) * (7) = 728
        let before = 728;
        // Score 4ok (2 hearts) with joker
        // 4ok (level 1) -> 60 chips, 7 mult
        // Played cards (4 ace) -> 44 chips
        // joker w/ 2 hearts = +6 mult
        // (60 + 44) * (7 + 6) = 1352
        let after = 1352;

        let j = Jokers::LustyJoker(LustyJoker {});
        score_before_after_joker(j, hand, before, after)
    }

    #[test]
    fn test_greedy_joker() {
        let ah = Card::new(Value::Ace, Suit::Heart);
        let ad = Card::new(Value::Ace, Suit::Diamond);
        let hand = SelectHand::new(vec![ad, ad, ad, ah]);

        // Score 4ok without joker
        // 4ok (level 1) -> 60 chips, 7 mult
        // Played cards (4 ace) -> 44 chips
        // (60 + 44) * (7) = 728
        let before = 728;
        // Score 4ok (3 diamonds) with joker
        // 4ok (level 1) -> 60 chips, 7 mult
        // Played cards (4 ace) -> 44 chips
        // joker w/ 3 diamonds = +9 mult
        // (60 + 44) * (7 + 9) = 1664
        let after = 1664;

        let j = Jokers::GreedyJoker(GreedyJoker {});
        score_before_after_joker(j, hand, before, after);
    }

    #[test]
    fn test_wrathful_joker() {
        let asp = Card::new(Value::Ace, Suit::Spade);
        let ad = Card::new(Value::Ace, Suit::Diamond);
        let hand = SelectHand::new(vec![asp, ad, ad, ad]);

        // Score 4ok without joker
        // 4ok (level 1) -> 60 chips, 7 mult
        // Played cards (4 ace) -> 44 chips
        // (60 + 44) * (7) = 728
        let before = 728;
        // Score 4ok (1 spade) with joker
        // 4ok (level 1) -> 60 chips, 7 mult
        // Played cards (4 ace) -> 44 chips
        // joker w/ 1 spade = +3 mult
        // (60 + 44) * (7 + 3) = 1040
        let after = 1040;

        let j = Jokers::WrathfulJoker(WrathfulJoker {});
        score_before_after_joker(j, hand, before, after);
    }

    #[test]
    fn test_gluttonous_joker() {
        let ac = Card::new(Value::Ace, Suit::Club);
        let hand = SelectHand::new(vec![ac, ac, ac, ac]);

        // Score 4ok without joker
        // 4ok (level 1) -> 60 chips, 7 mult
        // Played cards (4 ace) -> 44 chips
        // (60 + 44) * (7) = 728
        let before = 728;
        // Score 4ok (4 clubs) with joker
        // 4ok (level 1) -> 60 chips, 7 mult
        // Played cards (4 ace) -> 44 chips
        // joker w/ 4 clubs = +12 mult
        // (60 + 44) * (7 + 12) = 1976
        let after = 1976;

        let j = Jokers::GluttonousJoker(GluttonousJoker {});
        score_before_after_joker(j, hand, before, after)
    }

    #[test]
    fn test_jolly_joker() {
        let ac = Card::new(Value::Ace, Suit::Club);
        let hand = SelectHand::new(vec![ac, ac, ac, ac]);

        // Score 4ok without joker
        // 4ok (level 1) -> 60 chips, 7 mult
        // Played cards (4 ace) -> 44 chips
        // (60 + 44) * (7) = 728
        let before = 728;
        // Score 4ok with joker
        // 4ok (level 1) -> 60 chips, 7 mult
        // Played cards (4 ace) -> 44 chips
        // joker w/ pair = +8 mult
        // (60 + 44) * (7 + 8) = 1560
        let after = 1560;

        let j = Jokers::JollyJoker(JollyJoker {});
        score_before_after_joker(j, hand, before, after)
    }

    #[test]
    fn test_zany_joker() {
        let ac = Card::new(Value::Ace, Suit::Club);
        let hand = SelectHand::new(vec![ac, ac, ac, ac]);

        // Score 4ok without joker
        // 4ok (level 1) -> 60 chips, 7 mult
        // Played cards (4 ace) -> 44 chips
        // (60 + 44) * (7) = 728
        let before = 728;
        // Score 4ok with joker
        // 4ok (level 1) -> 60 chips, 7 mult
        // Played cards (4 ace) -> 44 chips
        // joker w/ 3ok = +12 mult
        // (60 + 44) * (7 + 12) = 1976
        let after = 1976;

        let j = Jokers::ZanyJoker(ZanyJoker {});
        score_before_after_joker(j, hand, before, after)
    }

    #[test]
    fn test_mad_joker() {
        let ac = Card::new(Value::Ace, Suit::Club);
        let kc = Card::new(Value::King, Suit::Club);
        let hand = SelectHand::new(vec![ac, ac, kc, kc]);

        // Score two pair without joker
        // two pair (level 1) -> 20 chips, 2 mult
        // Played cards (2 ace, 2 king) -> 42 chips
        // (20 + 42) * (2) = 124
        let before = 124;
        let j = Jokers::MadJoker(MadJoker {});
        // Score two pair with joker
        // two pair (level 1) -> 20 chips, 2 mult
        // Played cards (2 ace, 2 king) -> 42 chips
        // joker w/ two pair = +10 mult
        // (20 + 42) * (2 + 10) = 744
        let after = 744;

        score_before_after_joker(j, hand, before, after);
    }

    #[test]
    fn test_crazy_joker() {
        let two = Card::new(Value::Two, Suit::Club);
        let three = Card::new(Value::Three, Suit::Club);
        let four = Card::new(Value::Four, Suit::Club);
        let five = Card::new(Value::Five, Suit::Club);
        let six = Card::new(Value::Six, Suit::Heart);
        let hand = SelectHand::new(vec![two, three, four, five, six]);

        // Score straight without joker
        // straight (level 1) -> 30 chips, 4 mult
        // Played cards (2, 3, 4, 5, 6) -> 15 chips
        // (15 + 30) * (4) = 180
        let before = 180;
        // Score straight with joker
        // straight (level 1) -> 30 chips, 4 mult
        // Played cards (2, 3, 4, 5, 6) -> 15 chips
        // joker w/ straight = +12 mult
        // (15+ 30) * (4 + 12) = 720
        let after = 720;

        let j = Jokers::CrazyJoker(CrazyJoker {});
        score_before_after_joker(j, hand, before, after);
    }

    #[test]
    fn test_droll_joker() {
        let two = Card::new(Value::Two, Suit::Club);
        let three = Card::new(Value::Three, Suit::Club);
        let four = Card::new(Value::Four, Suit::Club);
        let five = Card::new(Value::Five, Suit::Club);
        let ten = Card::new(Value::Ten, Suit::Club);
        let hand = SelectHand::new(vec![two, three, four, five, ten]);

        // Score flush without joker
        // flush (level 1) -> 35 chips, 4 mult
        // Played cards (2, 3, 4, 5, 10) -> 19 chips
        // (19 + 35) * (4) = 216
        let before = 216;
        // Score flush with joker
        // flush (level 1) -> 35 chips, 4 mult
        // Played cards (2, 3, 4, 5, 10) -> 19 chips
        // joker w/ flush = +10 mult
        // (19 + 35) * (4 + 10) = 756
        let after = 756;

        let j = Jokers::DrollJoker(DrollJoker {});
        score_before_after_joker(j, hand, before, after);
    }

    #[test]
    fn test_sly_joker() {
        let ac = Card::new(Value::Ace, Suit::Club);
        let hand = SelectHand::new(vec![ac, ac, ac, ac]);

        // Score 4ok without joker
        // 4ok (level 1) -> 60 chips, 7 mult
        // Played cards (4 ace) -> 44 chips
        // (60 + 44) * (7) = 728
        let before = 728;
        // Score 4ok with joker
        // 4ok (level 1) -> 60 chips, 7 mult
        // Played cards (4 ace) -> 44 chips
        // joker w/ pair = +50 chips
        // (60 + 44 + 50) * (7) = 1078
        let after = 1078;

        let j = Jokers::SlyJoker(SlyJoker {});
        score_before_after_joker(j, hand, before, after);
    }

    #[test]
    fn test_wily_joker() {
        let ac = Card::new(Value::Ace, Suit::Club);
        let hand = SelectHand::new(vec![ac, ac, ac, ac]);

        // Score 4ok without joker
        // 4ok (level 1) -> 60 chips, 7 mult
        // Played cards (4 ace) -> 44 chips
        // (60 + 44) * (7) = 728
        let before = 728;
        // Score 4ok with joker
        // 4ok (level 1) -> 60 chips, 7 mult
        // Played cards (4 ace) -> 44 chips
        // joker w/ 3ok = +100 chips
        // (60 + 44 + 100) * (7) = 1428
        let after = 1428;

        let j = Jokers::WilyJoker(WilyJoker {});
        score_before_after_joker(j, hand, before, after);
    }

    #[test]
    fn test_clever_joker() {
        let ac = Card::new(Value::Ace, Suit::Club);
        let kc = Card::new(Value::King, Suit::Club);
        let hand = SelectHand::new(vec![ac, ac, kc, kc]);

        // Score two pair without joker
        // two pair (level 1) -> 20 chips, 2 mult
        // Played cards (2 ace, 2 king) -> 42 chips
        // (20 + 42) * (2) = 124
        let before = 124;
        // Score two pair with joker
        // two pair (level 1) -> 20 chips, 2 mult
        // Played cards (2 ace, 2 king) -> 42 chips
        // joker w/ two pair = +80 chips
        // (20 + 42 + 80) * (2) = 284
        let after = 284;

        let j = Jokers::CleverJoker(CleverJoker {});
        score_before_after_joker(j, hand, before, after);
    }

    #[test]
    fn test_devious_joker() {
        let two = Card::new(Value::Two, Suit::Club);
        let three = Card::new(Value::Three, Suit::Club);
        let four = Card::new(Value::Four, Suit::Club);
        let five = Card::new(Value::Five, Suit::Club);
        let six = Card::new(Value::Six, Suit::Heart);
        let hand = SelectHand::new(vec![two, three, four, five, six]);

        // Score straight without joker
        // straight (level 1) -> 30 chips, 4 mult
        // Played cards (2, 3, 4, 5, 6) -> 15 chips
        // (15 + 30) * (4) = 180
        let before = 180;
        // Score straight with joker
        // straight (level 1) -> 30 chips, 4 mult
        // Played cards (2, 3, 4, 5, 6) -> 15 chips
        // joker w/ straight = +100 chips
        // (15+ 30 + 100) * (4) = 580
        let after = 580;

        let j = Jokers::DeviousJoker(DeviousJoker {});
        score_before_after_joker(j, hand, before, after);
    }

    #[test]
    fn test_crafty_joker() {
        let two = Card::new(Value::Two, Suit::Club);
        let three = Card::new(Value::Three, Suit::Club);
        let four = Card::new(Value::Four, Suit::Club);
        let five = Card::new(Value::Five, Suit::Club);
        let ten = Card::new(Value::Ten, Suit::Club);
        let hand = SelectHand::new(vec![two, three, four, five, ten]);

        // Score flush without joker
        // flush (level 1) -> 35 chips, 4 mult
        // Played cards (2, 3, 4, 5, 10) -> 19 chips
        // (19 + 35) * (4) = 216
        let before = 216;
        // Score flush with joker
        // flush (level 1) -> 35 chips, 4 mult
        // Played cards (2, 3, 4, 5, 10) -> 19 chips
        // joker w/ flush = +80 chips
        // (19 + 35 + 80) * (4) = 536
        let after = 536;
        let j = Jokers::CraftyJoker(CraftyJoker {});
        score_before_after_joker(j, hand, before, after);
    }

    /// Test for Issue #87: Basic Joker Implementation
    /// Validates all acceptance criteria:
    /// - Joker provides +4 mult unconditionally
    /// - Test coverage for basic functionality  
    /// - Integration with scoring system
    /// - Proper joker registration
    #[test]
    fn test_issue_87_basic_joker_acceptance_criteria() {
        // Test 1: Verify joker provides exactly +4 mult with single card
        let king = Card::new(Value::King, Suit::Heart);
        let single_hand = SelectHand::new(vec![king]);

        // High card (level 1) -> 5 chips, 1 mult
        // King -> 10 chips
        // Base calculation: (5 + 10) * 1 = 15
        let before_single = 15;
        // With TheJoker (+4 mult): (5 + 10) * (1 + 4) = 75
        let after_single = 75;

        let joker = Jokers::TheJoker(TheJoker {});
        score_before_after_joker(joker.clone(), single_hand, before_single, after_single);

        // Test 2: Verify joker works unconditionally with different hand types
        let ace_spade = Card::new(Value::Ace, Suit::Spade);
        let ace_heart = Card::new(Value::Ace, Suit::Heart);
        let pair_hand = SelectHand::new(vec![ace_spade, ace_heart]);

        // Pair (level 1) -> 10 chips, 2 mult
        // Two Aces -> 22 chips
        // Base calculation: (10 + 22) * 2 = 64
        let before_pair = 64;
        // With TheJoker (+4 mult): (10 + 22) * (2 + 4) = 192
        let after_pair = 192;

        score_before_after_joker(joker.clone(), pair_hand, before_pair, after_pair);

        // Test 3: Verify joker is properly registered in factory
        let created_joker = JokerFactory::create(JokerId::Joker);
        assert!(
            created_joker.is_some(),
            "TheJoker should be registered in JokerFactory"
        );

        let joker_instance = created_joker.unwrap();
        assert_eq!(joker_instance.id(), JokerId::Joker);
        assert_eq!(joker_instance.name(), "Joker");
        assert_eq!(joker_instance.description(), "+4 Mult");
        assert_eq!(joker_instance.rarity(), JokerRarity::Common);
        assert_eq!(joker_instance.cost(), 2);

        // Test 4: Verify joker appears in common rarity list
        let common_jokers = JokerFactory::get_by_rarity(JokerRarity::Common);
        assert!(
            common_jokers.contains(&JokerId::Joker),
            "TheJoker should be listed in Common rarity jokers"
        );

        // Test 5: Verify joker is in implemented jokers list
        let all_implemented = JokerFactory::get_all_implemented();
        assert!(
            all_implemented.contains(&JokerId::Joker),
            "TheJoker should be in the list of all implemented jokers"
        );
    }
}
