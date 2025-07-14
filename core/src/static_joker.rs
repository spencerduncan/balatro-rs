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
    pub chips_bonus: Option<i32>,
    /// Bonus mult to add
    pub mult_bonus: Option<i32>,
    /// Multiplier to apply to mult
    pub mult_multiplier: Option<f32>,
    /// Condition for when to apply the effect
    pub condition: StaticCondition,
    /// Whether the effect applies per card or per hand
    pub per_card: bool,
}

impl StaticJoker {
    /// Create a new static joker builder
    pub fn builder(id: JokerId, name: &'static str, description: &'static str) -> StaticJokerBuilder {
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
                hand.cards().iter().any(|card| self.check_card_condition(card))
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
    chips_bonus: Option<i32>,
    mult_bonus: Option<i32>,
    mult_multiplier: Option<f32>,
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

    pub fn chips(mut self, chips: i32) -> Self {
        self.chips_bonus = Some(chips);
        self
    }

    pub fn mult(mut self, mult: i32) -> Self {
        self.mult_bonus = Some(mult);
        self
    }

    pub fn mult_multiplier(mut self, multiplier: f32) -> Self {
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

    pub fn build(self) -> StaticJoker {
        StaticJoker {
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
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_static_joker_builder() {
        let joker = StaticJoker::builder(
            JokerId::Joker,
            "Test Joker",
            "A test joker"
        )
        .rarity(JokerRarity::Common)
        .mult(4)
        .per_hand()
        .build();

        assert_eq!(joker.id(), JokerId::Joker);
        assert_eq!(joker.name(), "Test Joker");
        assert_eq!(joker.rarity(), JokerRarity::Common);
        assert_eq!(joker.mult_bonus, Some(4));
        assert!(!joker.per_card);
    }

    #[test]
    fn test_condition_always() {
        let joker = StaticJoker::builder(
            JokerId::Joker,
            "Always Joker", 
            "Always gives bonus"
        )
        .mult(5)
        .condition(StaticCondition::Always)
        .per_hand()
        .build();

        let effect = joker.create_effect();
        assert_eq!(effect.mult, 5);
    }

    #[test]
    fn test_suit_condition() {
        let joker = StaticJoker::builder(
            JokerId::GreedyJoker,
            "Diamond Joker",
            "Diamonds give bonus"
        )
        .mult(3)
        .condition(StaticCondition::SuitScored(Suit::Diamond))
        .per_card()
        .build();

        let diamond_card = Card::new(Value::King, Suit::Diamond);
        let heart_card = Card::new(Value::King, Suit::Heart);

        assert!(joker.check_card_condition(&diamond_card));
        assert!(!joker.check_card_condition(&heart_card));
    }

    #[test]
    fn test_rank_condition() {
        let joker = StaticJoker::builder(
            JokerId::Scholar,
            "Ace Bonus",
            "Aces give bonus"
        )
        .chips(20)
        .mult(4)
        .condition(StaticCondition::RankScored(Value::Ace))
        .per_card()
        .build();

        let ace_card = Card::new(Value::Ace, Suit::Spade);
        let king_card = Card::new(Value::King, Suit::Spade);

        assert!(joker.check_card_condition(&ace_card));
        assert!(!joker.check_card_condition(&king_card));
    }

    #[test]
    fn test_any_suit_condition() {
        let joker = StaticJoker::builder(
            JokerId::RedCard,
            "Red Bonus",
            "Red cards give bonus"
        )
        .mult(2)
        .condition(StaticCondition::AnySuitScored(vec![Suit::Heart, Suit::Diamond]))
        .per_card()
        .build();

        let heart_card = Card::new(Value::Ten, Suit::Heart);
        let diamond_card = Card::new(Value::Ten, Suit::Diamond);
        let spade_card = Card::new(Value::Ten, Suit::Spade);

        assert!(joker.check_card_condition(&heart_card));
        assert!(joker.check_card_condition(&diamond_card));
        assert!(!joker.check_card_condition(&spade_card));
    }

    #[test]
    fn test_any_rank_condition() {
        let joker = StaticJoker::builder(
            JokerId::EvenSteven,
            "Even Bonus",
            "Even cards give bonus"
        )
        .mult(4)
        .condition(StaticCondition::AnyRankScored(vec![
            Value::Two, Value::Four, Value::Six, Value::Eight, Value::Ten
        ]))
        .per_card()
        .build();

        let even_card = Card::new(Value::Eight, Suit::Club);
        let odd_card = Card::new(Value::Seven, Suit::Club);

        assert!(joker.check_card_condition(&even_card));
        assert!(!joker.check_card_condition(&odd_card));
    }

    #[test]
    fn test_multiple_bonuses() {
        let joker = StaticJoker::builder(
            JokerId::Scholar,
            "Multi Bonus",
            "Multiple effects"
        )
        .chips(50)
        .mult(10)
        .mult_multiplier(1.2)
        .per_hand()
        .build();

        let effect = joker.create_effect();
        assert_eq!(effect.chips, 50);
        assert_eq!(effect.mult, 10);
        assert_eq!(effect.mult_multiplier, 1.2);
    }

    #[test]
    fn test_cost_override() {
        let joker = StaticJoker::builder(
            JokerId::Joker,
            "Expensive",
            "Costs more"
        )
        .rarity(JokerRarity::Common)
        .cost(10)
        .build();

        assert_eq!(joker.cost(), 10);
    }

    #[test]
    fn test_default_costs() {
        // Common
        let common = StaticJoker::builder(JokerId::Joker, "Common", "")
            .rarity(JokerRarity::Common)
            .build();
        assert_eq!(common.cost(), 3);

        // Uncommon
        let uncommon = StaticJoker::builder(JokerId::Joker, "Uncommon", "")
            .rarity(JokerRarity::Uncommon)
            .build();
        assert_eq!(uncommon.cost(), 6);

        // Rare
        let rare = StaticJoker::builder(JokerId::Joker, "Rare", "")
            .rarity(JokerRarity::Rare)
            .build();
        assert_eq!(rare.cost(), 8);

        // Legendary
        let legendary = StaticJoker::builder(JokerId::Joker, "Legendary", "")
            .rarity(JokerRarity::Legendary)
            .build();
        assert_eq!(legendary.cost(), 20);
    }

    #[test]
    fn test_hand_type_condition() {
        let joker = StaticJoker::builder(
            JokerId::JollyJoker,
            "Pair Bonus",
            "+8 Mult if played hand contains a Pair"
        )
        .mult(8)
        .condition(StaticCondition::HandType(HandRank::OnePair))
        .per_hand()
        .build();

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
            "+10 Mult if played hand contains a Flush"
        )
        .mult(10)
        .condition(StaticCondition::HandType(HandRank::Flush))
        .per_hand()
        .build();

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
}