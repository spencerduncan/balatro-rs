use crate::card::{Card, Suit, Value};
use crate::hand::SelectHand;
use crate::joker::{GameContext, Joker, JokerEffect, JokerId, JokerRarity};
use crate::rank::HandRank;
use serde::{Deserialize, Serialize};

/// Condition for when a static joker effect should apply
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StaticCondition {
    /// Always apply the effect
    Always,
    /// Apply when a specific suit is scored
    SuitScored(Suit),
    /// Apply when a specific value/rank is scored
    RankScored(Value),
    /// Apply when the hand contains a specific type
    /// (e.g., OnePair triggers on Pair, Two Pair, Full House, etc.)
    HandType(HandRank),
    /// Apply when multiple suits are scored
    AnySuitScored(Vec<Suit>),
    /// Apply when multiple ranks are scored
    AnyRankScored(Vec<Value>),
}

/// A static joker that provides consistent bonuses based on conditions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaticJoker {
    /// Unique identifier for this joker
    pub id: JokerId,
    /// Display name
    pub name: &'static str,
    /// Description of what the joker does
    pub description: &'static str,
    /// Rarity level
    pub rarity: JokerRarity,
    /// Base cost override (if None, uses default for rarity)
    pub base_cost: Option<usize>,
    /// Bonus chips to add
    pub chips_bonus: Option<f64>,
    /// Bonus mult to add
    pub mult_bonus: Option<f64>,
    /// Multiplier to apply to mult
    pub mult_multiplier: Option<f64>,
    /// Condition for when to apply the effect
    pub condition: StaticCondition,
    /// Whether the effect applies per card or per hand
    pub per_card: bool,
}

impl StaticJoker {
    /// Create a new static joker builder
    pub fn builder(
        id: JokerId,
        name: &'static str,
        description: &'static str,
    ) -> StaticJokerBuilder {
        StaticJokerBuilder {
            id,
            name,
            description,
            rarity: JokerRarity::Common,
            base_cost: None,
            chips_bonus: None,
            mult_bonus: None,
            mult_multiplier: None,
            condition: StaticCondition::Always,
            per_card: false,
        }
    }
}

impl Joker for StaticJoker {
    fn id(&self) -> JokerId {
        self.id
    }

    fn name(&self) -> &str {
        self.name
    }

    fn description(&self) -> &str {
        self.description
    }

    fn rarity(&self) -> JokerRarity {
        self.rarity
    }

    fn cost(&self) -> usize {
        self.base_cost.unwrap_or(match self.rarity {
            JokerRarity::Common => 3,
            JokerRarity::Uncommon => 6,
            JokerRarity::Rare => 8,
            JokerRarity::Legendary => 20,
        })
    }

    fn on_hand_played(&self, _context: &mut GameContext, hand: &SelectHand) -> JokerEffect {
        if !self.per_card {
            // Apply effect once per hand if condition is met
            if self.check_hand_condition(hand) {
                self.create_effect()
            } else {
                JokerEffect::new()
            }
        } else {
            JokerEffect::new()
        }
    }

    fn on_card_scored(&self, _context: &mut GameContext, card: &Card) -> JokerEffect {
        if self.per_card {
            // Apply effect per card if condition is met
            if self.check_card_condition(card) {
                self.create_effect()
            } else {
                JokerEffect::new()
            }
        } else {
            JokerEffect::new()
        }
    }
}

impl StaticJoker {
    /// Check if the condition is met for a hand
    fn check_hand_condition(&self, hand: &SelectHand) -> bool {
        match &self.condition {
            StaticCondition::Always => true,
            StaticCondition::HandType(required_rank) => {
                // Check if the hand contains the required type
                match required_rank {
                    HandRank::OnePair => hand.is_pair().is_some(),
                    HandRank::TwoPair => hand.is_two_pair().is_some(),
                    HandRank::ThreeOfAKind => hand.is_three_of_kind().is_some(),
                    HandRank::Straight => hand.is_straight().is_some(),
                    HandRank::Flush => hand.is_flush().is_some(),
                    HandRank::FullHouse => hand.is_fullhouse().is_some(),
                    HandRank::FourOfAKind => hand.is_four_of_kind().is_some(),
                    HandRank::StraightFlush => hand.is_straight_flush().is_some(),
                    HandRank::RoyalFlush => hand.is_royal_flush().is_some(),
                    HandRank::FiveOfAKind => hand.is_five_of_kind().is_some(),
                    HandRank::FlushHouse => hand.is_flush_house().is_some(),
                    HandRank::FlushFive => hand.is_flush_five().is_some(),
                    HandRank::HighCard => hand.is_highcard().is_some(),
                }
            }
            _ => {
                // For suit/rank conditions on hands, check if any card matches
                hand.cards()
                    .iter()
                    .any(|card| self.check_card_condition(card))
            }
        }
    }

    /// Check if the condition is met for a card
    fn check_card_condition(&self, card: &Card) -> bool {
        match &self.condition {
            StaticCondition::Always => true,
            StaticCondition::SuitScored(suit) => card.suit == *suit,
            StaticCondition::RankScored(value) => card.value == *value,
            StaticCondition::AnySuitScored(suits) => suits.contains(&card.suit),
            StaticCondition::AnyRankScored(values) => values.contains(&card.value),
            StaticCondition::HandType(_) => {
                // Hand type conditions don't apply to individual cards
                false
            }
        }
    }

    /// Create the effect based on configured bonuses
    fn create_effect(&self) -> JokerEffect {
        let mut effect = JokerEffect::new();

        if let Some(chips) = self.chips_bonus {
            effect = effect.with_chips(chips);
        }

        if let Some(mult) = self.mult_bonus {
            effect = effect.with_mult(mult);
        }

        if let Some(multiplier) = self.mult_multiplier {
            effect = effect.with_mult_multiplier(multiplier);
        }

        effect
    }
}

/// Builder for creating static jokers
pub struct StaticJokerBuilder {
    id: JokerId,
    name: &'static str,
    description: &'static str,
    rarity: JokerRarity,
    base_cost: Option<usize>,
    chips_bonus: Option<f64>,
    mult_bonus: Option<f64>,
    mult_multiplier: Option<f64>,
    condition: StaticCondition,
    per_card: bool,
}

impl StaticJokerBuilder {
    pub fn rarity(mut self, rarity: JokerRarity) -> Self {
        self.rarity = rarity;
        self
    }

    pub fn cost(mut self, cost: usize) -> Self {
        self.base_cost = Some(cost);
        self
    }

    pub fn chips(mut self, chips: f64) -> Self {
        self.chips_bonus = Some(chips);
        self
    }

    pub fn mult(mut self, mult: f64) -> Self {
        self.mult_bonus = Some(mult);
        self
    }

    pub fn mult_multiplier(mut self, multiplier: f64) -> Self {
        self.mult_multiplier = Some(multiplier);
        self
    }

    pub fn condition(mut self, condition: StaticCondition) -> Self {
        self.condition = condition;
        self
    }

    pub fn per_card(mut self) -> Self {
        self.per_card = true;
        self
    }

    pub fn per_hand(mut self) -> Self {
        self.per_card = false;
        self
    }

    pub fn build(self) -> Result<StaticJoker, String> {
        // Validate that per_card/per_hand is compatible with condition
        match (&self.condition, self.per_card) {
            (StaticCondition::HandType(_), true) => {
                return Err("HandType conditions should be per_hand, not per_card".to_string());
            }
            (StaticCondition::SuitScored(_), false) => {
                return Err("SuitScored conditions should be per_card, not per_hand".to_string());
            }
            (StaticCondition::RankScored(_), false) => {
                return Err("RankScored conditions should be per_card, not per_hand".to_string());
            }
            (StaticCondition::AnySuitScored(_), false) => {
                return Err("AnySuitScored conditions should be per_card, not per_hand".to_string());
            }
            (StaticCondition::AnyRankScored(_), false) => {
                return Err("AnyRankScored conditions should be per_card, not per_hand".to_string());
            }
            _ => {} // Valid combinations
        }

        // Validate that at least one bonus is specified
        if self.chips_bonus.is_none() && self.mult_bonus.is_none() && self.mult_multiplier.is_none()
        {
            return Err(
                "At least one bonus (chips, mult, or mult_multiplier) must be specified"
                    .to_string(),
            );
        }

        Ok(StaticJoker {
            id: self.id,
            name: self.name,
            description: self.description,
            rarity: self.rarity,
            base_cost: self.base_cost,
            chips_bonus: self.chips_bonus,
            mult_bonus: self.mult_bonus,
            mult_multiplier: self.mult_multiplier,
            condition: self.condition,
            per_card: self.per_card,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_static_joker_builder() {
        let joker = StaticJoker::builder(JokerId::Joker, "Test Joker", "A test joker")
            .rarity(JokerRarity::Common)
            .mult(4.0)
            .per_hand()
            .build()
            .expect("Valid joker configuration");

        assert_eq!(joker.id(), JokerId::Joker);
        assert_eq!(joker.name(), "Test Joker");
        assert_eq!(joker.rarity(), JokerRarity::Common);
        assert_eq!(joker.mult_bonus, Some(4));
        assert!(!joker.per_card);
    }

    #[test]
    fn test_condition_always() {
        let joker = StaticJoker::builder(JokerId::Joker, "Always Joker", "Always gives bonus")
            .mult(5)
            .condition(StaticCondition::Always)
            .per_hand()
            .build()
            .expect("Valid joker configuration");

        let effect = joker.create_effect();
        assert_eq!(effect.mult, 5);
    }

    #[test]
    fn test_suit_condition() {
        let joker =
            StaticJoker::builder(JokerId::GreedyJoker, "Diamond Joker", "Diamonds give bonus")
                .mult(3.0)
                .condition(StaticCondition::SuitScored(Suit::Diamond))
                .per_card()
                .build()
                .expect("Valid joker configuration");

        let diamond_card = Card::new(Value::King, Suit::Diamond);
        let heart_card = Card::new(Value::King, Suit::Heart);

        assert!(joker.check_card_condition(&diamond_card));
        assert!(!joker.check_card_condition(&heart_card));
    }

    #[test]
    fn test_rank_condition() {
        let joker = StaticJoker::builder(JokerId::Scholar, "Ace Bonus", "Aces give bonus")
            .chips(20.0)
            .mult(4.0)
            .condition(StaticCondition::RankScored(Value::Ace))
            .per_card()
            .build()
            .expect("Valid joker configuration");

        let ace_card = Card::new(Value::Ace, Suit::Spade);
        let king_card = Card::new(Value::King, Suit::Spade);

        assert!(joker.check_card_condition(&ace_card));
        assert!(!joker.check_card_condition(&king_card));
    }

    #[test]
    fn test_any_suit_condition() {
        let joker = StaticJoker::builder(JokerId::RedCard, "Red Bonus", "Red cards give bonus")
            .mult(2)
            .condition(StaticCondition::AnySuitScored(vec![
                Suit::Heart,
                Suit::Diamond,
            ]))
            .per_card()
            .build()
            .expect("Valid joker configuration");

        let heart_card = Card::new(Value::Ten, Suit::Heart);
        let diamond_card = Card::new(Value::Ten, Suit::Diamond);
        let spade_card = Card::new(Value::Ten, Suit::Spade);

        assert!(joker.check_card_condition(&heart_card));
        assert!(joker.check_card_condition(&diamond_card));
        assert!(!joker.check_card_condition(&spade_card));
    }

    #[test]
    fn test_suit_jokers_greedy() {
        let joker = crate::static_joker_factory::StaticJokerFactory::create_greedy_joker_concrete();

        // Test properties
        assert_eq!(joker.id(), JokerId::GreedyJoker);
        assert_eq!(joker.name(), "Greedy Joker");
        assert_eq!(joker.mult_bonus, Some(3));
        assert!(joker.per_card);

        // Test condition checking
        let diamond_card = Card::new(Value::Ace, Suit::Diamond);
        let heart_card = Card::new(Value::King, Suit::Heart);

        assert!(joker.check_card_condition(&diamond_card));
        assert!(!joker.check_card_condition(&heart_card));

        // Test effect
        let effect = joker.create_effect();
        assert_eq!(effect.mult, 3);
    }

    #[test]
    fn test_suit_jokers_lusty() {
        let joker = crate::static_joker_factory::StaticJokerFactory::create_lusty_joker_concrete();

        // Test properties
        assert_eq!(joker.id(), JokerId::LustyJoker);
        assert_eq!(joker.name(), "Lusty Joker");
        assert_eq!(joker.mult_bonus, Some(3));
        assert!(joker.per_card);

        // Test condition checking
        let heart_card = Card::new(Value::Ace, Suit::Heart);
        let spade_card = Card::new(Value::King, Suit::Spade);

        assert!(joker.check_card_condition(&heart_card));
        assert!(!joker.check_card_condition(&spade_card));

        // Test effect
        let effect = joker.create_effect();
        assert_eq!(effect.mult, 3);
    }

    #[test]
    fn test_suit_jokers_wrathful() {
        let joker =
            crate::static_joker_factory::StaticJokerFactory::create_wrathful_joker_concrete();

        // Test properties
        assert_eq!(joker.id(), JokerId::WrathfulJoker);
        assert_eq!(joker.name(), "Wrathful Joker");
        assert_eq!(joker.mult_bonus, Some(3));
        assert!(joker.per_card);

        // Test condition checking
        let spade_card = Card::new(Value::Ace, Suit::Spade);
        let club_card = Card::new(Value::King, Suit::Club);

        assert!(joker.check_card_condition(&spade_card));
        assert!(!joker.check_card_condition(&club_card));

        // Test effect
        let effect = joker.create_effect();
        assert_eq!(effect.mult, 3);
    }

    #[test]
    fn test_suit_jokers_gluttonous() {
        let joker =
            crate::static_joker_factory::StaticJokerFactory::create_gluttonous_joker_concrete();

        // Test properties
        assert_eq!(joker.id(), JokerId::GluttonousJoker);
        assert_eq!(joker.name(), "Gluttonous Joker");
        assert_eq!(joker.mult_bonus, Some(3));
        assert!(joker.per_card);

        // Test condition checking
        let club_card = Card::new(Value::Ace, Suit::Club);
        let diamond_card = Card::new(Value::King, Suit::Diamond);

        assert!(joker.check_card_condition(&club_card));
        assert!(!joker.check_card_condition(&diamond_card));

        // Test effect
        let effect = joker.create_effect();
        assert_eq!(effect.mult, 3);
    }

    #[test]
    fn test_suit_jokers_isolation() {
        // Create all four suit jokers
        let greedy =
            crate::static_joker_factory::StaticJokerFactory::create_greedy_joker_concrete();
        let lusty = crate::static_joker_factory::StaticJokerFactory::create_lusty_joker_concrete();
        let wrathful =
            crate::static_joker_factory::StaticJokerFactory::create_wrathful_joker_concrete();
        let gluttonous =
            crate::static_joker_factory::StaticJokerFactory::create_gluttonous_joker_concrete();

        // Create one card of each suit
        let diamond_card = Card::new(Value::Ace, Suit::Diamond);
        let heart_card = Card::new(Value::King, Suit::Heart);
        let spade_card = Card::new(Value::Queen, Suit::Spade);
        let club_card = Card::new(Value::Jack, Suit::Club);

        // Each joker should only match its own suit
        assert!(greedy.check_card_condition(&diamond_card));
        assert!(!greedy.check_card_condition(&heart_card));
        assert!(!greedy.check_card_condition(&spade_card));
        assert!(!greedy.check_card_condition(&club_card));

        assert!(!lusty.check_card_condition(&diamond_card));
        assert!(lusty.check_card_condition(&heart_card));
        assert!(!lusty.check_card_condition(&spade_card));
        assert!(!lusty.check_card_condition(&club_card));

        assert!(!wrathful.check_card_condition(&diamond_card));
        assert!(!wrathful.check_card_condition(&heart_card));
        assert!(wrathful.check_card_condition(&spade_card));
        assert!(!wrathful.check_card_condition(&club_card));

        assert!(!gluttonous.check_card_condition(&diamond_card));
        assert!(!gluttonous.check_card_condition(&heart_card));
        assert!(!gluttonous.check_card_condition(&spade_card));
        assert!(gluttonous.check_card_condition(&club_card));

        // All should give the same +3 mult effect
        assert_eq!(greedy.create_effect().mult, 3);
        assert_eq!(lusty.create_effect().mult, 3);
        assert_eq!(wrathful.create_effect().mult, 3);
        assert_eq!(gluttonous.create_effect().mult, 3);
    }

    #[test]
    fn test_any_rank_condition() {
        let joker =
            StaticJoker::builder(JokerId::EvenSteven, "Even Bonus", "Even cards give bonus")
                .mult(4.0)
                .condition(StaticCondition::AnyRankScored(vec![
                    Value::Two,
                    Value::Four,
                    Value::Six,
                    Value::Eight,
                    Value::Ten,
                ]))
                .per_card()
                .build()
                .expect("Valid joker configuration");

        let even_card = Card::new(Value::Eight, Suit::Club);
        let odd_card = Card::new(Value::Seven, Suit::Club);

        assert!(joker.check_card_condition(&even_card));
        assert!(!joker.check_card_condition(&odd_card));
    }

    #[test]
    fn test_multiple_bonuses() {
        let joker = StaticJoker::builder(JokerId::Scholar, "Multi Bonus", "Multiple effects")
            .chips(50.0)
            .mult(10.0)
            .mult_multiplier(1.2)
            .per_hand()
            .build()
            .expect("Valid joker configuration");

        let effect = joker.create_effect();
        assert_eq!(effect.chips, 50);
        assert_eq!(effect.mult, 10);
        assert_eq!(effect.mult_multiplier, 1.2);
    }

    #[test]
    fn test_cost_override() {
        let joker = StaticJoker::builder(JokerId::Joker, "Expensive", "Costs more")
            .rarity(JokerRarity::Common)
            .cost(10)
            .mult(1.0) // Add minimal bonus to satisfy validation
            .build()
            .expect("Valid joker configuration");

        assert_eq!(joker.cost(), 10);
    }

    #[test]
    fn test_default_costs() {
        // Common
        let common = StaticJoker::builder(JokerId::Joker, "Common", "")
            .rarity(JokerRarity::Common)
            .mult(1.0) // Add minimal bonus to satisfy validation
            .build()
            .expect("Valid joker configuration");
        assert_eq!(common.cost(), 3);

        // Uncommon
        let uncommon = StaticJoker::builder(JokerId::Joker, "Uncommon", "")
            .rarity(JokerRarity::Uncommon)
            .mult(1.0) // Add minimal bonus to satisfy validation
            .build()
            .expect("Valid joker configuration");
        assert_eq!(uncommon.cost(), 6);

        // Rare
        let rare = StaticJoker::builder(JokerId::Joker, "Rare", "")
            .rarity(JokerRarity::Rare)
            .mult(1.0) // Add minimal bonus to satisfy validation
            .build()
            .expect("Valid joker configuration");
        assert_eq!(rare.cost(), 8);

        // Legendary
        let legendary = StaticJoker::builder(JokerId::Joker, "Legendary", "")
            .rarity(JokerRarity::Legendary)
            .mult(1.0) // Add minimal bonus to satisfy validation
            .build()
            .expect("Valid joker configuration");
        assert_eq!(legendary.cost(), 20);
    }

    #[test]
    fn test_hand_type_condition() {
        let joker = StaticJoker::builder(
            JokerId::JollyJoker,
            "Pair Bonus",
            "+8 Mult if played hand contains a Pair",
        )
        .mult(8.0)
        .condition(StaticCondition::HandType(HandRank::OnePair))
        .per_hand()
        .build()
        .expect("Valid joker configuration");

        // Test with a hand that is exactly a pair
        let pair_hand = SelectHand::new(vec![
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::King, Suit::Diamond),
        ]);

        // Test with a hand that is a two pair (contains pairs)
        let two_pair_hand = SelectHand::new(vec![
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::King, Suit::Diamond),
            Card::new(Value::Queen, Suit::Heart),
            Card::new(Value::Queen, Suit::Diamond),
        ]);

        // Test with a hand that is a full house (contains a pair)
        let full_house_hand = SelectHand::new(vec![
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::King, Suit::Diamond),
            Card::new(Value::King, Suit::Club),
            Card::new(Value::Queen, Suit::Heart),
            Card::new(Value::Queen, Suit::Diamond),
        ]);

        // Test with a hand that is high card (no pair)
        let high_card_hand = SelectHand::new(vec![
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::Queen, Suit::Diamond),
            Card::new(Value::Jack, Suit::Club),
        ]);

        assert!(joker.check_hand_condition(&pair_hand));
        assert!(joker.check_hand_condition(&two_pair_hand)); // Contains pairs
        assert!(joker.check_hand_condition(&full_house_hand)); // Contains a pair
        assert!(!joker.check_hand_condition(&high_card_hand)); // No pair
    }

    #[test]
    fn test_flush_condition_contains() {
        let flush_joker = StaticJoker::builder(
            JokerId::DrollJoker,
            "Flush Bonus",
            "+10 Mult if played hand contains a Flush",
        )
        .mult(10.0)
        .condition(StaticCondition::HandType(HandRank::Flush))
        .per_hand()
        .build()
        .expect("Valid joker configuration");

        // Regular flush
        let flush_hand = SelectHand::new(vec![
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::Queen, Suit::Heart),
            Card::new(Value::Jack, Suit::Heart),
            Card::new(Value::Ten, Suit::Heart),
            Card::new(Value::Two, Suit::Heart),
        ]);

        // Straight flush (contains a flush)
        let straight_flush_hand = SelectHand::new(vec![
            Card::new(Value::Six, Suit::Heart),
            Card::new(Value::Five, Suit::Heart),
            Card::new(Value::Four, Suit::Heart),
            Card::new(Value::Three, Suit::Heart),
            Card::new(Value::Two, Suit::Heart),
        ]);

        // Not a flush
        let mixed_hand = SelectHand::new(vec![
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::Queen, Suit::Diamond),
            Card::new(Value::Jack, Suit::Heart),
            Card::new(Value::Ten, Suit::Heart),
            Card::new(Value::Nine, Suit::Heart),
        ]);

        assert!(flush_joker.check_hand_condition(&flush_hand));
        assert!(flush_joker.check_hand_condition(&straight_flush_hand)); // Straight flush contains a flush
        assert!(!flush_joker.check_hand_condition(&mixed_hand)); // Not a flush
    }

    #[test]
    fn test_two_pair_condition() {
        let joker = StaticJoker::builder(
            JokerId::MadJoker,
            "Two Pair Bonus",
            "+10 Mult if played hand contains Two Pair",
        )
        .mult(10.0)
        .condition(StaticCondition::HandType(HandRank::TwoPair))
        .per_hand()
        .build()
        .expect("Valid joker configuration");

        // Exact two pair
        let two_pair_hand = SelectHand::new(vec![
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::King, Suit::Diamond),
            Card::new(Value::Queen, Suit::Heart),
            Card::new(Value::Queen, Suit::Diamond),
            Card::new(Value::Jack, Suit::Club),
        ]);

        // Full house (contains two pairs)
        let full_house_hand = SelectHand::new(vec![
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::King, Suit::Diamond),
            Card::new(Value::King, Suit::Club),
            Card::new(Value::Queen, Suit::Heart),
            Card::new(Value::Queen, Suit::Diamond),
        ]);

        // Single pair (doesn't contain two pair)
        let one_pair_hand = SelectHand::new(vec![
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::King, Suit::Diamond),
            Card::new(Value::Queen, Suit::Heart),
            Card::new(Value::Jack, Suit::Diamond),
            Card::new(Value::Ten, Suit::Club),
        ]);

        assert!(joker.check_hand_condition(&two_pair_hand));
        assert!(joker.check_hand_condition(&full_house_hand)); // Full house contains two pairs
        assert!(!joker.check_hand_condition(&one_pair_hand));
    }

    #[test]
    fn test_three_of_a_kind_condition() {
        let joker = StaticJoker::builder(
            JokerId::ZanyJoker,
            "Three of a Kind Bonus",
            "+12 Mult if played hand contains Three of a Kind",
        )
        .mult(12.0)
        .condition(StaticCondition::HandType(HandRank::ThreeOfAKind))
        .per_hand()
        .build()
        .expect("Valid joker configuration");

        // Exact three of a kind
        let three_kind_hand = SelectHand::new(vec![
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::King, Suit::Diamond),
            Card::new(Value::King, Suit::Club),
            Card::new(Value::Queen, Suit::Heart),
            Card::new(Value::Jack, Suit::Diamond),
        ]);

        // Full house (contains three of a kind)
        let full_house_hand = SelectHand::new(vec![
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::King, Suit::Diamond),
            Card::new(Value::King, Suit::Club),
            Card::new(Value::Queen, Suit::Heart),
            Card::new(Value::Queen, Suit::Diamond),
        ]);

        // Four of a kind (contains three of a kind)
        let four_kind_hand = SelectHand::new(vec![
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::King, Suit::Diamond),
            Card::new(Value::King, Suit::Club),
            Card::new(Value::King, Suit::Spade),
            Card::new(Value::Queen, Suit::Heart),
        ]);

        // Two pair (doesn't contain three of a kind)
        let two_pair_hand = SelectHand::new(vec![
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::King, Suit::Diamond),
            Card::new(Value::Queen, Suit::Heart),
            Card::new(Value::Queen, Suit::Diamond),
            Card::new(Value::Jack, Suit::Club),
        ]);

        assert!(joker.check_hand_condition(&three_kind_hand));
        assert!(joker.check_hand_condition(&full_house_hand));
        assert!(joker.check_hand_condition(&four_kind_hand));
        assert!(!joker.check_hand_condition(&two_pair_hand));
    }

    #[test]
    fn test_straight_condition() {
        let joker = StaticJoker::builder(
            JokerId::CrazyJoker,
            "Straight Bonus",
            "+12 Mult if played hand contains Straight",
        )
        .mult(12.0)
        .condition(StaticCondition::HandType(HandRank::Straight))
        .per_hand()
        .build()
        .expect("Valid joker configuration");

        // Regular straight
        let straight_hand = SelectHand::new(vec![
            Card::new(Value::Ten, Suit::Heart),
            Card::new(Value::Nine, Suit::Diamond),
            Card::new(Value::Eight, Suit::Club),
            Card::new(Value::Seven, Suit::Spade),
            Card::new(Value::Six, Suit::Heart),
        ]);

        // Straight flush (contains straight)
        let straight_flush_hand = SelectHand::new(vec![
            Card::new(Value::Six, Suit::Heart),
            Card::new(Value::Five, Suit::Heart),
            Card::new(Value::Four, Suit::Heart),
            Card::new(Value::Three, Suit::Heart),
            Card::new(Value::Two, Suit::Heart),
        ]);

        // Not a straight
        let non_straight_hand = SelectHand::new(vec![
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::Queen, Suit::Diamond),
            Card::new(Value::Jack, Suit::Club),
            Card::new(Value::Nine, Suit::Spade),
            Card::new(Value::Eight, Suit::Heart),
        ]);

        assert!(joker.check_hand_condition(&straight_hand));
        assert!(joker.check_hand_condition(&straight_flush_hand));
        assert!(!joker.check_hand_condition(&non_straight_hand));
    }

    #[test]
    fn test_full_house_condition() {
        let joker = StaticJoker::builder(
            JokerId::Scholar,
            "Full House Bonus",
            "+20 Chips if played hand contains Full House",
        )
        .chips(20.0)
        .condition(StaticCondition::HandType(HandRank::FullHouse))
        .per_hand()
        .build()
        .expect("Valid joker configuration");

        // Full house
        let full_house_hand = SelectHand::new(vec![
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::King, Suit::Diamond),
            Card::new(Value::King, Suit::Club),
            Card::new(Value::Queen, Suit::Heart),
            Card::new(Value::Queen, Suit::Diamond),
        ]);

        // Three of a kind (doesn't contain full house)
        let three_kind_hand = SelectHand::new(vec![
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::King, Suit::Diamond),
            Card::new(Value::King, Suit::Club),
            Card::new(Value::Queen, Suit::Heart),
            Card::new(Value::Jack, Suit::Diamond),
        ]);

        // Two pair (doesn't contain full house)
        let two_pair_hand = SelectHand::new(vec![
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::King, Suit::Diamond),
            Card::new(Value::Queen, Suit::Heart),
            Card::new(Value::Queen, Suit::Diamond),
            Card::new(Value::Jack, Suit::Club),
        ]);

        assert!(joker.check_hand_condition(&full_house_hand));
        assert!(!joker.check_hand_condition(&three_kind_hand));
        assert!(!joker.check_hand_condition(&two_pair_hand));
    }

    #[test]
    fn test_four_of_a_kind_condition() {
        let joker = StaticJoker::builder(
            JokerId::Scholar,
            "Four of a Kind Bonus",
            "+30 Chips if played hand contains Four of a Kind",
        )
        .chips(30.0)
        .condition(StaticCondition::HandType(HandRank::FourOfAKind))
        .per_hand()
        .build()
        .expect("Valid joker configuration");

        // Four of a kind
        let four_kind_hand = SelectHand::new(vec![
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::King, Suit::Diamond),
            Card::new(Value::King, Suit::Club),
            Card::new(Value::King, Suit::Spade),
            Card::new(Value::Queen, Suit::Heart),
        ]);

        // Five of a kind (contains four of a kind)
        let five_kind_hand = SelectHand::new(vec![
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::King, Suit::Diamond),
            Card::new(Value::King, Suit::Club),
            Card::new(Value::King, Suit::Spade),
            Card::new(Value::King, Suit::Heart), // Duplicate for five of a kind test
        ]);

        // Three of a kind (doesn't contain four of a kind)
        let three_kind_hand = SelectHand::new(vec![
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::King, Suit::Diamond),
            Card::new(Value::King, Suit::Club),
            Card::new(Value::Queen, Suit::Heart),
            Card::new(Value::Jack, Suit::Diamond),
        ]);

        assert!(joker.check_hand_condition(&four_kind_hand));
        assert!(joker.check_hand_condition(&five_kind_hand));
        assert!(!joker.check_hand_condition(&three_kind_hand));
    }

    #[test]
    fn test_straight_flush_condition() {
        let joker = StaticJoker::builder(
            JokerId::Scholar,
            "Straight Flush Bonus",
            "+50 Chips if played hand contains Straight Flush",
        )
        .chips(50.0)
        .condition(StaticCondition::HandType(HandRank::StraightFlush))
        .per_hand()
        .build()
        .expect("Valid joker configuration");

        // Straight flush
        let straight_flush_hand = SelectHand::new(vec![
            Card::new(Value::Six, Suit::Heart),
            Card::new(Value::Five, Suit::Heart),
            Card::new(Value::Four, Suit::Heart),
            Card::new(Value::Three, Suit::Heart),
            Card::new(Value::Two, Suit::Heart),
        ]);

        // Royal flush (contains straight flush)
        let royal_flush_hand = SelectHand::new(vec![
            Card::new(Value::Ace, Suit::Heart),
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::Queen, Suit::Heart),
            Card::new(Value::Jack, Suit::Heart),
            Card::new(Value::Ten, Suit::Heart),
        ]);

        // Regular flush (doesn't contain straight)
        let flush_hand = SelectHand::new(vec![
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::Queen, Suit::Heart),
            Card::new(Value::Jack, Suit::Heart),
            Card::new(Value::Nine, Suit::Heart),
            Card::new(Value::Seven, Suit::Heart),
        ]);

        // Regular straight (doesn't contain flush)
        let straight_hand = SelectHand::new(vec![
            Card::new(Value::Ten, Suit::Heart),
            Card::new(Value::Nine, Suit::Diamond),
            Card::new(Value::Eight, Suit::Club),
            Card::new(Value::Seven, Suit::Spade),
            Card::new(Value::Six, Suit::Heart),
        ]);

        assert!(joker.check_hand_condition(&straight_flush_hand));
        assert!(joker.check_hand_condition(&royal_flush_hand));
        assert!(!joker.check_hand_condition(&flush_hand));
        assert!(!joker.check_hand_condition(&straight_hand));
    }

    #[test]
    fn test_high_card_condition() {
        let joker = StaticJoker::builder(
            JokerId::Scholar,
            "High Card Bonus",
            "+5 Chips if played hand is High Card",
        )
        .chips(5.0)
        .condition(StaticCondition::HandType(HandRank::HighCard))
        .per_hand()
        .build()
        .expect("Valid joker configuration");

        // High card (no pairs, flushes, or straights)
        let high_card_hand = SelectHand::new(vec![
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::Queen, Suit::Diamond),
            Card::new(Value::Jack, Suit::Club),
            Card::new(Value::Nine, Suit::Spade),
            Card::new(Value::Seven, Suit::Heart),
        ]);

        // Pair (not high card)
        let pair_hand = SelectHand::new(vec![
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::King, Suit::Diamond),
            Card::new(Value::Queen, Suit::Club),
            Card::new(Value::Jack, Suit::Spade),
            Card::new(Value::Nine, Suit::Heart),
        ]);

        assert!(joker.check_hand_condition(&high_card_hand));
        assert!(!joker.check_hand_condition(&pair_hand));
    }

    #[test]
    fn test_royal_flush_condition() {
        let joker = StaticJoker::builder(
            JokerId::Scholar,
            "Royal Flush Bonus",
            "+100 Chips if played hand contains Royal Flush",
        )
        .chips(100.0)
        .condition(StaticCondition::HandType(HandRank::RoyalFlush))
        .per_hand()
        .build()
        .expect("Valid joker configuration");

        // Royal flush (A, K, Q, J, 10 all same suit)
        let royal_flush_hand = SelectHand::new(vec![
            Card::new(Value::Ace, Suit::Heart),
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::Queen, Suit::Heart),
            Card::new(Value::Jack, Suit::Heart),
            Card::new(Value::Ten, Suit::Heart),
        ]);

        // Regular straight flush (not royal)
        let straight_flush_hand = SelectHand::new(vec![
            Card::new(Value::Nine, Suit::Heart),
            Card::new(Value::Eight, Suit::Heart),
            Card::new(Value::Seven, Suit::Heart),
            Card::new(Value::Six, Suit::Heart),
            Card::new(Value::Five, Suit::Heart),
        ]);

        assert!(joker.check_hand_condition(&royal_flush_hand));
        assert!(!joker.check_hand_condition(&straight_flush_hand));
    }

    #[test]
    fn test_five_of_a_kind_condition() {
        let joker = StaticJoker::builder(
            JokerId::Scholar,
            "Five of a Kind Bonus",
            "+50 Chips if played hand contains Five of a Kind",
        )
        .chips(50.0)
        .condition(StaticCondition::HandType(HandRank::FiveOfAKind))
        .per_hand()
        .build()
        .expect("Valid joker configuration");

        // Five of a kind (5 cards of same rank)
        let five_kind_hand = SelectHand::new(vec![
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::King, Suit::Diamond),
            Card::new(Value::King, Suit::Club),
            Card::new(Value::King, Suit::Spade),
            Card::new(Value::King, Suit::Heart), // Special Balatro case with duplicate suits
        ]);

        // Four of a kind (doesn't contain five of a kind)
        let four_kind_hand = SelectHand::new(vec![
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::King, Suit::Diamond),
            Card::new(Value::King, Suit::Club),
            Card::new(Value::King, Suit::Spade),
            Card::new(Value::Queen, Suit::Heart),
        ]);

        assert!(joker.check_hand_condition(&five_kind_hand));
        assert!(!joker.check_hand_condition(&four_kind_hand));
    }

    #[test]
    fn test_flush_house_condition() {
        let joker = StaticJoker::builder(
            JokerId::Scholar,
            "Flush House Bonus",
            "+60 Chips if played hand contains Flush House",
        )
        .chips(60.0)
        .condition(StaticCondition::HandType(HandRank::FlushHouse))
        .per_hand()
        .build()
        .expect("Valid joker configuration");

        // Flush house (full house + flush - 3 of same rank + pair, all same suit)
        let flush_house_hand = SelectHand::new(vec![
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::Queen, Suit::Heart),
            Card::new(Value::Queen, Suit::Heart),
        ]);

        // Regular full house (not flush)
        let full_house_hand = SelectHand::new(vec![
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::King, Suit::Diamond),
            Card::new(Value::King, Suit::Club),
            Card::new(Value::Queen, Suit::Heart),
            Card::new(Value::Queen, Suit::Diamond),
        ]);

        // Regular flush (not full house)
        let flush_hand = SelectHand::new(vec![
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::Queen, Suit::Heart),
            Card::new(Value::Jack, Suit::Heart),
            Card::new(Value::Ten, Suit::Heart),
            Card::new(Value::Nine, Suit::Heart),
        ]);

        assert!(joker.check_hand_condition(&flush_house_hand));
        assert!(!joker.check_hand_condition(&full_house_hand));
        assert!(!joker.check_hand_condition(&flush_hand));
    }

    #[test]
    fn test_flush_five_condition() {
        let joker = StaticJoker::builder(
            JokerId::Scholar,
            "Flush Five Bonus",
            "+80 Chips if played hand contains Flush Five",
        )
        .chips(80.0)
        .condition(StaticCondition::HandType(HandRank::FlushFive))
        .per_hand()
        .build()
        .expect("Valid joker configuration");

        // Flush five (five of a kind + flush - 5 cards same rank, all same suit)
        let flush_five_hand = SelectHand::new(vec![
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::King, Suit::Heart),
        ]);

        // Five of a kind (not flush)
        let five_kind_hand = SelectHand::new(vec![
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::King, Suit::Diamond),
            Card::new(Value::King, Suit::Club),
            Card::new(Value::King, Suit::Spade),
            Card::new(Value::King, Suit::Heart),
        ]);

        // Regular flush (not five of a kind)
        let flush_hand = SelectHand::new(vec![
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::Queen, Suit::Heart),
            Card::new(Value::Jack, Suit::Heart),
            Card::new(Value::Ten, Suit::Heart),
            Card::new(Value::Nine, Suit::Heart),
        ]);

        assert!(joker.check_hand_condition(&flush_five_hand));
        assert!(!joker.check_hand_condition(&five_kind_hand));
        assert!(!joker.check_hand_condition(&flush_hand));
    }

    #[test]
    fn test_builder_validation() {
        // Test invalid configuration: HandType condition with per_card
        let result = StaticJoker::builder(
            JokerId::JollyJoker,
            "Invalid Joker",
            "This should fail validation",
        )
        .mult(8.0)
        .condition(StaticCondition::HandType(HandRank::OnePair))
        .per_card() // This should be invalid with HandType
        .build();

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("HandType conditions should be per_hand"));

        // Test invalid configuration: SuitScored condition with per_hand
        let result = StaticJoker::builder(
            JokerId::GreedyJoker,
            "Invalid Suit Joker",
            "This should fail validation",
        )
        .mult(3.0)
        .condition(StaticCondition::SuitScored(Suit::Diamond))
        .per_hand() // This should be invalid with SuitScored
        .build();

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("SuitScored conditions should be per_card"));

        // Test invalid configuration: No bonuses specified
        let result = StaticJoker::builder(
            JokerId::Joker,
            "No Bonus Joker",
            "This should fail validation",
        )
        .condition(StaticCondition::Always)
        .per_hand()
        .build();

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("At least one bonus"));

        // Test valid configuration
        let result = StaticJoker::builder(JokerId::Joker, "Valid Joker", "This should work")
            .mult(4.0)
            .condition(StaticCondition::Always)
            .per_hand()
            .build();

        assert!(result.is_ok());
    }
}
