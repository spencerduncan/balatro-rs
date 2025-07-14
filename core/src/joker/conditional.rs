use crate::card::{Card, Suit, Value as Rank};
use crate::hand::SelectHand;
use crate::joker::{GameContext, Joker, JokerEffect, JokerId, JokerRarity};
use crate::rank::HandRank;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Debug};

/// Enum representing all possible conditions for conditional jokers
#[derive(Clone, Serialize, Deserialize)]
pub enum JokerCondition {
    /// Money is less than specified amount
    MoneyLessThan(i32),
    /// Money is greater than specified amount
    MoneyGreaterThan(i32),
    /// Hand size is exactly the specified value
    HandSizeExactly(usize),
    /// No face cards (J, Q, K) are held
    NoFaceCardsHeld,
    /// Hand contains at least one card of the specified rank
    ContainsRank(Rank),
    /// Hand contains at least one card of the specified suit
    ContainsSuit(Suit),
    /// Played hand is of the specified type
    PlayedHandType(HandRank),
    /// Composite condition - all must be true
    All(Vec<JokerCondition>),
    /// Composite condition - at least one must be true
    Any(Vec<JokerCondition>),
    /// Negates a condition
    Not(Box<JokerCondition>),
    /// Always true
    Always,
    // Note: Custom conditions are not supported for serialization/cloning
    // Use specific condition types instead
}

impl Debug for JokerCondition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MoneyLessThan(amount) => write!(f, "MoneyLessThan({amount})"),
            Self::MoneyGreaterThan(amount) => write!(f, "MoneyGreaterThan({amount})"),
            Self::HandSizeExactly(size) => write!(f, "HandSizeExactly({size})"),
            Self::NoFaceCardsHeld => write!(f, "NoFaceCardsHeld"),
            Self::ContainsRank(rank) => write!(f, "ContainsRank({rank:?})"),
            Self::ContainsSuit(suit) => write!(f, "ContainsSuit({suit:?})"),
            Self::PlayedHandType(hand_type) => write!(f, "PlayedHandType({hand_type:?})"),
            Self::All(conditions) => write!(f, "All({conditions:?})"),
            Self::Any(conditions) => write!(f, "Any({conditions:?})"),
            Self::Not(condition) => write!(f, "Not({condition:?})"),
            Self::Always => write!(f, "Always"),
        }
    }
}

impl JokerCondition {
    /// Evaluate the condition against the current game context
    pub fn evaluate(&self, context: &GameContext) -> bool {
        match self {
            Self::MoneyLessThan(amount) => context.money < *amount,
            Self::MoneyGreaterThan(amount) => context.money > *amount,
            Self::HandSizeExactly(_size) => {
                // This condition needs to be evaluated with the actual played hand
                // Return true as default, actual evaluation happens in evaluate_with_hand
                true
            }
            Self::NoFaceCardsHeld => {
                // This condition needs to be evaluated with the actual played hand
                // Return true as default, actual evaluation happens in evaluate_with_hand
                true
            }
            Self::ContainsRank(_rank) => {
                // This condition needs to be evaluated with the actual played hand
                // Return true as default, actual evaluation happens in evaluate_with_hand
                true
            }
            Self::ContainsSuit(_suit) => {
                // This condition needs to be evaluated with the actual played hand
                // Return true as default, actual evaluation happens in evaluate_with_hand
                true
            }
            Self::PlayedHandType(_hand_type) => {
                // This will be evaluated in on_hand_played with the actual played hand
                // For now, return false as a default
                false
            }
            Self::All(conditions) => conditions.iter().all(|cond| cond.evaluate(context)),
            Self::Any(conditions) => conditions.iter().any(|cond| cond.evaluate(context)),
            Self::Not(condition) => !condition.evaluate(context),
            Self::Always => true,
        }
    }

    /// Special evaluation for hand-based conditions
    pub fn evaluate_with_hand(&self, context: &GameContext, hand: &SelectHand) -> bool {
        match self {
            Self::PlayedHandType(hand_type) => match hand.best_hand() {
                Ok(made_hand) => made_hand.rank == *hand_type,
                Err(_) => false,
            },
            Self::HandSizeExactly(size) => hand.len() == *size,
            Self::NoFaceCardsHeld => !hand
                .cards()
                .iter()
                .any(|card| matches!(card.value, Rank::Jack | Rank::Queen | Rank::King)),
            Self::ContainsRank(rank) => hand.cards().iter().any(|card| card.value == *rank),
            Self::ContainsSuit(suit) => hand.cards().iter().any(|card| card.suit == *suit),
            Self::All(conditions) => conditions
                .iter()
                .all(|cond| cond.evaluate_with_hand(context, hand)),
            Self::Any(conditions) => conditions
                .iter()
                .any(|cond| cond.evaluate_with_hand(context, hand)),
            Self::Not(condition) => !condition.evaluate_with_hand(context, hand),
            _ => self.evaluate(context),
        }
    }
}

/// A conditional joker that activates based on game state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionalJoker {
    /// Unique identifier for this joker
    pub id: JokerId,
    /// Display name
    pub name: String,
    /// Description of the joker's effect
    pub description: String,
    /// Rarity level
    pub rarity: JokerRarity,
    /// Base cost in the shop
    pub cost: usize,
    /// Condition that must be met for the joker to activate
    pub condition: JokerCondition,
    /// Effect when the condition is met
    pub effect: JokerEffect,
    /// Optional effect for each card scored (when condition is met)
    pub card_effect: Option<JokerEffect>,
}

impl ConditionalJoker {
    /// Create a new conditional joker
    pub fn new(
        id: JokerId,
        name: impl Into<String>,
        description: impl Into<String>,
        rarity: JokerRarity,
        condition: JokerCondition,
        effect: JokerEffect,
    ) -> Self {
        let cost = match rarity {
            JokerRarity::Common => 3,
            JokerRarity::Uncommon => 6,
            JokerRarity::Rare => 8,
            JokerRarity::Legendary => 20,
        };

        Self {
            id,
            name: name.into(),
            description: description.into(),
            rarity,
            cost,
            condition,
            effect,
            card_effect: None,
        }
    }

    /// Set a custom cost
    pub fn with_cost(mut self, cost: usize) -> Self {
        self.cost = cost;
        self
    }

    /// Set a per-card effect (for jokers that trigger on each scored card)
    pub fn with_card_effect(mut self, effect: JokerEffect) -> Self {
        self.card_effect = Some(effect);
        self
    }
}

impl Joker for ConditionalJoker {
    fn id(&self) -> JokerId {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn rarity(&self) -> JokerRarity {
        self.rarity
    }

    fn cost(&self) -> usize {
        self.cost
    }

    fn on_hand_played(&self, context: &mut GameContext, hand: &SelectHand) -> JokerEffect {
        if self.condition.evaluate_with_hand(context, hand) {
            self.effect.clone()
        } else {
            JokerEffect::new()
        }
    }

    fn on_card_scored(&self, context: &mut GameContext, _card: &Card) -> JokerEffect {
        if let Some(ref card_effect) = self.card_effect {
            if self.condition.evaluate(context) {
                card_effect.clone()
            } else {
                JokerEffect::new()
            }
        } else {
            JokerEffect::new()
        }
    }

    fn on_blind_start(&self, context: &mut GameContext) -> JokerEffect {
        if self.condition.evaluate(context) {
            self.effect.clone()
        } else {
            JokerEffect::new()
        }
    }

    fn on_shop_open(&self, context: &mut GameContext) -> JokerEffect {
        if self.condition.evaluate(context) {
            self.effect.clone()
        } else {
            JokerEffect::new()
        }
    }

    fn on_discard(&self, context: &mut GameContext, _cards: &[Card]) -> JokerEffect {
        if self.condition.evaluate(context) {
            self.effect.clone()
        } else {
            JokerEffect::new()
        }
    }

    fn on_round_end(&self, context: &mut GameContext) -> JokerEffect {
        if self.condition.evaluate(context) {
            self.effect.clone()
        } else {
            JokerEffect::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_money_conditions() {
        // Test money conditions without needing a full GameContext
        // Since evaluate() for money conditions only uses context.money,
        // we can test them independently

        // Test MoneyLessThan
        let less_than_100 = JokerCondition::MoneyLessThan(100);
        let _less_than_50 = JokerCondition::MoneyLessThan(50);

        // Test MoneyGreaterThan
        let greater_than_25 = JokerCondition::MoneyGreaterThan(25);
        let _greater_than_100 = JokerCondition::MoneyGreaterThan(100);

        // We'll test these conditions using a mock context in integration tests
        // For now, just verify they can be created and formatted
        assert_eq!(format!("{:?}", less_than_100), "MoneyLessThan(100)");
        assert_eq!(format!("{:?}", greater_than_25), "MoneyGreaterThan(25)");
    }

    #[test]
    fn test_hand_conditions_formatting() {
        // Test that hand conditions can be created and formatted properly
        let no_face = JokerCondition::NoFaceCardsHeld;
        let contains_king = JokerCondition::ContainsRank(Rank::King);
        let contains_heart = JokerCondition::ContainsSuit(Suit::Heart);
        let size_two = JokerCondition::HandSizeExactly(2);
        let played_flush = JokerCondition::PlayedHandType(HandRank::Flush);

        assert_eq!(format!("{:?}", no_face), "NoFaceCardsHeld");
        assert_eq!(format!("{:?}", contains_king), "ContainsRank(King)");
        assert_eq!(format!("{:?}", contains_heart), "ContainsSuit(Heart)");
        assert_eq!(format!("{:?}", size_two), "HandSizeExactly(2)");
        assert_eq!(format!("{:?}", played_flush), "PlayedHandType(Flush)");
    }

    #[test]
    fn test_composite_conditions_formatting() {
        // Test composite conditions can be created and formatted
        let all_condition = JokerCondition::All(vec![
            JokerCondition::MoneyLessThan(100),
            JokerCondition::MoneyGreaterThan(25),
        ]);

        let any_condition = JokerCondition::Any(vec![
            JokerCondition::MoneyLessThan(25),
            JokerCondition::NoFaceCardsHeld,
        ]);

        let not_condition = JokerCondition::Not(Box::new(JokerCondition::Always));

        // Test formatting
        assert!(format!("{:?}", all_condition).contains("All"));
        assert!(format!("{:?}", any_condition).contains("Any"));
        assert!(format!("{:?}", not_condition).contains("Not"));

        // Test Always condition
        let always = JokerCondition::Always;
        assert_eq!(format!("{:?}", always), "Always");
    }

    #[test]
    fn test_conditional_joker() {
        let joker = ConditionalJoker::new(
            JokerId::Banner,
            "Banner",
            "+40 chips when you have remaining discards",
            JokerRarity::Common,
            JokerCondition::Always,
            JokerEffect::new().with_chips(40),
        );

        assert_eq!(joker.id(), JokerId::Banner);
        assert_eq!(joker.name(), "Banner");
        assert_eq!(joker.cost(), 3);
        assert_eq!(joker.rarity(), JokerRarity::Common);
    }
}
