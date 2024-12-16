use crate::card::Suit;
use crate::effect::Effects;
use crate::game::Game;
use crate::hand::MadeHand;
use pyo3::pyclass;
use std::sync::{Arc, Mutex};
use strum::{EnumIter, IntoEnumIterator};

pub trait Joker: std::fmt::Debug + Clone {
    fn name(&self) -> String;
    fn desc(&self) -> String;
    fn cost(&self) -> usize;
    fn rarity(&self) -> Rarity;
    fn categories(&self) -> Vec<Categories>;
    fn effects(&self, game: &Game) -> Vec<Effects>;
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Categories {
    MultPlus,
    MultMult,
    Chips,
    Economy,
    Retrigger,
    Effect,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Legendary,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "python", pyclass(eq))]
#[derive(Debug, Clone, EnumIter, Eq, PartialEq, Hash)]
pub enum Jokers {
    TheJoker(TheJoker),
    GreedyJoker(GreedyJoker),
    LustyJoker(LustyJoker),
    WrathfulJoker(WrathfulJoker),
    GluttonousJoker(GluttonousJoker),
}

impl Jokers {
    pub fn by_rarity(rarirty: Rarity) -> Vec<Self> {
        return Self::iter().filter(|j| j.rarity() == rarirty).collect();
    }
}

impl Joker for Jokers {
    fn name(&self) -> String {
        match self {
            Self::TheJoker(j) => j.name(),
            Self::GreedyJoker(j) => j.name(),
            Self::LustyJoker(j) => j.name(),
            Self::WrathfulJoker(j) => j.name(),
            Self::GluttonousJoker(j) => j.name(),
        }
    }
    fn desc(&self) -> String {
        match self {
            Self::TheJoker(j) => j.desc(),
            Self::GreedyJoker(j) => j.desc(),
            Self::LustyJoker(j) => j.desc(),
            Self::WrathfulJoker(j) => j.desc(),
            Self::GluttonousJoker(j) => j.desc(),
        }
    }
    fn cost(&self) -> usize {
        match self {
            Self::TheJoker(j) => j.cost(),
            Self::GreedyJoker(j) => j.cost(),
            Self::LustyJoker(j) => j.cost(),
            Self::WrathfulJoker(j) => j.cost(),
            Self::GluttonousJoker(j) => j.cost(),
        }
    }
    fn rarity(&self) -> Rarity {
        match self {
            Self::TheJoker(j) => j.rarity(),
            Self::GreedyJoker(j) => j.rarity(),
            Self::LustyJoker(j) => j.rarity(),
            Self::WrathfulJoker(j) => j.rarity(),
            Self::GluttonousJoker(j) => j.rarity(),
        }
    }
    fn categories(&self) -> Vec<Categories> {
        match self {
            Self::TheJoker(j) => j.categories(),
            Self::GreedyJoker(j) => j.categories(),
            Self::LustyJoker(j) => j.categories(),
            Self::WrathfulJoker(j) => j.categories(),
            Self::GluttonousJoker(j) => j.categories(),
        }
    }
    fn effects(&self, game: &Game) -> Vec<Effects> {
        match self {
            Self::TheJoker(j) => j.effects(game),
            Self::GreedyJoker(j) => j.effects(game),
            Self::LustyJoker(j) => j.effects(game),
            Self::WrathfulJoker(j) => j.effects(game),
            Self::GluttonousJoker(j) => j.effects(game),
        }
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct TheJoker {}

impl Joker for TheJoker {
    fn name(&self) -> String {
        "Joker".to_string()
    }
    fn desc(&self) -> String {
        "+4 Mult".to_string()
    }
    fn cost(&self) -> usize {
        2
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultPlus]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        fn apply(g: &mut Game, _hand: MadeHand) {
            g.mult += 4;
        }
        vec![Effects::OnScore(Arc::new(Mutex::new(apply)))]
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct GreedyJoker {}

impl Joker for GreedyJoker {
    fn name(&self) -> String {
        "Greedy Joker".to_string()
    }
    fn desc(&self) -> String {
        "Played cards with diamond suit give +3 mult when scored ".to_string()
    }
    fn cost(&self) -> usize {
        5
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultPlus]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        fn apply(g: &mut Game, hand: MadeHand) {
            let diamonds = hand
                .hand
                .suits()
                .iter()
                .filter(|s| **s == Suit::Diamond)
                .count();
            g.mult += diamonds * 3
        }
        vec![Effects::OnScore(Arc::new(Mutex::new(apply)))]
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct LustyJoker {}

impl Joker for LustyJoker {
    fn name(&self) -> String {
        "Lusty Joker".to_string()
    }
    fn desc(&self) -> String {
        "Played cards with heart suit give +3 mult when scored ".to_string()
    }
    fn cost(&self) -> usize {
        5
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultPlus]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        fn apply(g: &mut Game, hand: MadeHand) {
            let hearts = hand
                .hand
                .suits()
                .iter()
                .filter(|s| **s == Suit::Heart)
                .count();
            g.mult += hearts * 3
        }
        vec![Effects::OnScore(Arc::new(Mutex::new(apply)))]
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct WrathfulJoker {}

impl Joker for WrathfulJoker {
    fn name(&self) -> String {
        "Wrathful Joker".to_string()
    }
    fn desc(&self) -> String {
        "Played cards with spade suit give +3 mult when scored ".to_string()
    }
    fn cost(&self) -> usize {
        5
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultPlus]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        fn apply(g: &mut Game, hand: MadeHand) {
            let spades = hand
                .hand
                .suits()
                .iter()
                .filter(|s| **s == Suit::Spade)
                .count();
            g.mult += spades * 3
        }
        vec![Effects::OnScore(Arc::new(Mutex::new(apply)))]
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub struct GluttonousJoker {}

impl Joker for GluttonousJoker {
    fn name(&self) -> String {
        "Gluttonous Joker".to_string()
    }
    fn desc(&self) -> String {
        "Played cards with club suit give +3 mult when scored ".to_string()
    }
    fn cost(&self) -> usize {
        5
    }
    fn rarity(&self) -> Rarity {
        Rarity::Common
    }
    fn categories(&self) -> Vec<Categories> {
        vec![Categories::MultPlus]
    }
    fn effects(&self, _in: &Game) -> Vec<Effects> {
        fn apply(g: &mut Game, hand: MadeHand) {
            let clubs = hand
                .hand
                .suits()
                .iter()
                .filter(|s| **s == Suit::Club)
                .count();
            g.mult += clubs * 3
        }
        vec![Effects::OnScore(Arc::new(Mutex::new(apply)))]
    }
}

#[cfg(test)]
mod tests {
    use crate::card::{Card, Suit, Value};
    use crate::hand::SelectHand;
    use crate::stage::{Blind, Stage};

    use super::*;

    #[test]
    fn test_the_joker() {
        let mut g = Game::default();
        g.stage = Stage::Blind(Blind::Small);
        let ace = Card::new(Value::Ace, Suit::Heart);
        let hand = SelectHand::new(vec![ace]);

        // Score Ace high without joker
        // High card (level 1) -> 5 chips, 1 mult
        // Played cards (1 ace) -> 11 chips
        // (5 + 11) * (1) = 16
        let score = g.calc_score(hand.best_hand().unwrap());
        assert_eq!(score, 16);

        // buy (and apply) the joker
        g.stage = Stage::Shop();
        let j = Jokers::TheJoker(TheJoker {});
        g.buy_joker(j).unwrap();
        g.stage = Stage::Blind(Blind::Small);
        // Score Ace high with the Joker
        // High card (level 1) -> 5 chips, 1 mult
        // Played cards (1 ace) -> 11 chips
        // Joker (The Joker) -> 4 mult
        // (5 + 11) * (1 + 4) = 80
        let score = g.calc_score(hand.best_hand().unwrap());
        assert_eq!(score, 80);
    }

    #[test]
    fn test_lusty_joker() {
        let mut g = Game::default();
        g.stage = Stage::Blind(Blind::Small);
        let ah = Card::new(Value::Ace, Suit::Heart);
        let ac = Card::new(Value::Ace, Suit::Club);
        let ad = Card::new(Value::Ace, Suit::Diamond);
        let hand = SelectHand::new(vec![ah, ah, ac, ad]);

        // Score 4ok without joker
        // 4ok (level 1) -> 60 chips, 7 mult
        // Played cards (4 ace) -> 44 chips
        // (60 + 44) * (7) = 728
        let score = g.calc_score(hand.best_hand().unwrap());
        assert_eq!(score, 728);

        // buy (and apply) the joker
        g.stage = Stage::Shop();
        let j = Jokers::LustyJoker(LustyJoker {});
        g.buy_joker(j).unwrap();
        g.stage = Stage::Blind(Blind::Small);
        // Score 4ok (2 hearts) with joker
        // 4ok (level 1) -> 60 chips, 7 mult
        // Played cards (4 ace) -> 44 chips
        // joker w/ 2 hearts = +6 mult
        // (60 + 44) * (13) = 1352
        let score = g.calc_score(hand.best_hand().unwrap());
        assert_eq!(score, 1352);
    }

    #[test]
    fn test_greedy_joker() {
        let mut g = Game::default();
        g.stage = Stage::Blind(Blind::Small);
        let ah = Card::new(Value::Ace, Suit::Heart);
        let ad = Card::new(Value::Ace, Suit::Diamond);
        let hand = SelectHand::new(vec![ad, ad, ad, ah]);

        // Score 4ok without joker
        // 4ok (level 1) -> 60 chips, 7 mult
        // Played cards (4 ace) -> 44 chips
        // (60 + 44) * (7) = 728
        let score = g.calc_score(hand.best_hand().unwrap());
        assert_eq!(score, 728);

        // buy (and apply) the joker
        g.stage = Stage::Shop();
        let j = Jokers::GreedyJoker(GreedyJoker {});
        g.buy_joker(j).unwrap();
        g.stage = Stage::Blind(Blind::Small);
        // Score 4ok (3 diamonds) with joker
        // 4ok (level 1) -> 60 chips, 7 mult
        // Played cards (4 ace) -> 44 chips
        // joker w/ 3 diamonds = +9 mult
        // (60 + 44) * (16) = 1664
        let score = g.calc_score(hand.best_hand().unwrap());
        assert_eq!(score, 1664);
    }

    #[test]
    fn test_wrathful_joker() {
        let mut g = Game::default();
        g.stage = Stage::Blind(Blind::Small);
        let asp = Card::new(Value::Ace, Suit::Spade);
        let ad = Card::new(Value::Ace, Suit::Diamond);
        let hand = SelectHand::new(vec![asp, ad, ad, ad]);

        // Score 4ok without joker
        // 4ok (level 1) -> 60 chips, 7 mult
        // Played cards (4 ace) -> 44 chips
        // (60 + 44) * (7) = 728
        let score = g.calc_score(hand.best_hand().unwrap());
        assert_eq!(score, 728);

        // buy (and apply) the joker
        g.stage = Stage::Shop();
        let j = Jokers::WrathfulJoker(WrathfulJoker {});
        g.buy_joker(j).unwrap();
        g.stage = Stage::Blind(Blind::Small);
        // Score 4ok (1 spade) with joker
        // 4ok (level 1) -> 60 chips, 7 mult
        // Played cards (4 ace) -> 44 chips
        // joker w/ 1 spade = +3 mult
        // (60 + 44) * (10) = 1040
        let score = g.calc_score(hand.best_hand().unwrap());
        assert_eq!(score, 1040);
    }

    #[test]
    fn test_gluttonous_joker() {
        let mut g = Game::default();
        g.stage = Stage::Blind(Blind::Small);
        let ac = Card::new(Value::Ace, Suit::Club);
        let hand = SelectHand::new(vec![ac, ac, ac, ac]);

        // Score 4ok without joker
        // 4ok (level 1) -> 60 chips, 7 mult
        // Played cards (4 ace) -> 44 chips
        // (60 + 44) * (7) = 728
        let score = g.calc_score(hand.best_hand().unwrap());
        assert_eq!(score, 728);

        // buy (and apply) the joker
        g.stage = Stage::Shop();
        let j = Jokers::GluttonousJoker(GluttonousJoker {});
        g.buy_joker(j).unwrap();
        g.stage = Stage::Blind(Blind::Small);
        // Score 4ok (4 clubs) with joker
        // 4ok (level 1) -> 60 chips, 7 mult
        // Played cards (4 ace) -> 44 chips
        // joker w/ 4 clubs = +12 mult
        // (60 + 44) * (19) = 1976
        let score = g.calc_score(hand.best_hand().unwrap());
        assert_eq!(score, 1976);
    }
}
