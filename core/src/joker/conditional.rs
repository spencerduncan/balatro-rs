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
    /// Evaluate the condition against the current game context.
    ///
    /// For hand-specific conditions (HandSizeExactly, NoFaceCardsHeld, ContainsRank, ContainsSuit),
    /// this method returns `false` to indicate that hand context is required.
    /// Use `evaluate_with_hand()` for proper evaluation of these conditions.
    ///
    /// For composite conditions (All, Any, Not), this method recursively evaluates all
    /// sub-conditions using their `evaluate()` method.
    ///
    /// # Arguments
    /// * `context` - The current game context containing money and other game state
    ///
    /// # Returns
    /// * `true` if the condition is satisfied given the current game context
    /// * `false` if the condition is not satisfied or requires hand context
    pub fn evaluate(&self, context: &GameContext) -> bool {
        match self {
            Self::MoneyLessThan(amount) => context.money < *amount,
            Self::MoneyGreaterThan(amount) => context.money > *amount,
            Self::HandSizeExactly(_size) => {
                // This condition requires hand context to evaluate properly
                // Return false when hand context is not available
                false
            }
            Self::NoFaceCardsHeld => {
                // This condition requires hand context to evaluate properly
                // Return false when hand context is not available
                false
            }
            Self::ContainsRank(_rank) => {
                // This condition requires hand context to evaluate properly
                // Return false when hand context is not available
                false
            }
            Self::ContainsSuit(_suit) => {
                // This condition requires hand context to evaluate properly
                // Return false when hand context is not available
                false
            }
            Self::PlayedHandType(_hand_type) => {
                // This condition requires hand context to evaluate properly
                // Return false when hand context is not available
                false
            }
            Self::All(conditions) => conditions.iter().all(|cond| cond.evaluate(context)),
            Self::Any(conditions) => conditions.iter().any(|cond| cond.evaluate(context)),
            Self::Not(condition) => !condition.evaluate(context),
            Self::Always => true,
        }
    }

    /// Evaluate the condition with both game context and hand information.
    ///
    /// This method should be used when hand context is available, as it provides
    /// proper evaluation for all condition types including hand-specific ones.
    /// For conditions that don't require hand context (like money conditions),
    /// this method delegates to `evaluate()`.
    ///
    /// # Arguments
    /// * `context` - The current game context containing money and other game state
    /// * `hand` - The hand being played/evaluated
    ///
    /// # Returns
    /// * `true` if the condition is satisfied given both the game context and hand
    /// * `false` if the condition is not satisfied
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

    /// Evaluate the condition for a specific card being scored.
    ///
    /// This method is used during card scoring events when we have access to the
    /// individual card but not the full hand context. For card-specific conditions
    /// (ContainsRank, ContainsSuit), it evaluates against the provided card.
    /// For other conditions, it delegates to `evaluate()`.
    ///
    /// # Arguments
    /// * `context` - The current game context containing money and other game state
    /// * `card` - The specific card being scored
    ///
    /// # Returns
    /// * `true` if the condition is satisfied for this card and game context
    /// * `false` if the condition is not satisfied
    pub fn evaluate_for_card(&self, context: &GameContext, card: &Card) -> bool {
        match self {
            Self::ContainsRank(rank) => card.value == *rank,
            Self::ContainsSuit(suit) => card.suit == *suit,
            Self::NoFaceCardsHeld => !matches!(card.value, Rank::Jack | Rank::Queen | Rank::King),
            Self::All(conditions) => conditions
                .iter()
                .all(|cond| cond.evaluate_for_card(context, card)),
            Self::Any(conditions) => conditions
                .iter()
                .any(|cond| cond.evaluate_for_card(context, card)),
            Self::Not(condition) => !condition.evaluate_for_card(context, card),
            // For conditions that can't be meaningfully evaluated for a single card,
            // delegate to the general evaluate method
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

    fn on_card_scored(&self, context: &mut GameContext, card: &Card) -> JokerEffect {
        if let Some(ref card_effect) = self.card_effect {
            if self.condition.evaluate_for_card(context, card) {
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

    // Note: GameContext tests are complex due to lifetime and private constructor issues
    // For now, we test the logic we can without full GameContext creation
    // Integration tests should cover full GameContext scenarios

    #[test]
    fn test_hand_condition_evaluation_without_context() {
        use crate::hand::SelectHand;

        // Create test hands
        let cards_with_ace = vec![
            Card::new(Rank::Ace, Suit::Heart),
            Card::new(Rank::King, Suit::Spade),
            Card::new(Rank::Three, Suit::Diamond),
        ];
        let hand_with_ace = SelectHand::new(cards_with_ace);

        let cards_no_face = vec![
            Card::new(Rank::Ace, Suit::Heart),
            Card::new(Rank::Two, Suit::Spade),
            Card::new(Rank::Three, Suit::Diamond),
        ];
        let hand_no_face = SelectHand::new(cards_no_face);

        let cards_with_face = vec![
            Card::new(Rank::King, Suit::Heart),
            Card::new(Rank::Queen, Suit::Spade),
            Card::new(Rank::Three, Suit::Diamond),
        ];
        let hand_with_face = SelectHand::new(cards_with_face);

        // Test hand size check (doesn't need GameContext)
        assert_eq!(hand_with_ace.len(), 3);
        assert_eq!(hand_no_face.len(), 3);
        assert_eq!(hand_with_face.len(), 3);

        // For full evaluation, we'd need GameContext, but we can test hand properties
        assert!(hand_with_ace
            .cards()
            .iter()
            .any(|card| card.value == Rank::Ace));
        assert!(hand_with_face
            .cards()
            .iter()
            .any(|card| matches!(card.value, Rank::King | Rank::Queen | Rank::Jack)));
        assert!(!hand_no_face
            .cards()
            .iter()
            .any(|card| matches!(card.value, Rank::King | Rank::Queen | Rank::Jack)));
    }

    #[test]
    fn test_card_evaluation_simple() {
        let ace_heart = Card::new(Rank::Ace, Suit::Heart);
        let king_spade = Card::new(Rank::King, Suit::Spade);

        // Test individual card properties directly
        assert_eq!(ace_heart.value, Rank::Ace);
        assert_eq!(ace_heart.suit, Suit::Heart);
        assert_eq!(king_spade.value, Rank::King);
        assert_eq!(king_spade.suit, Suit::Spade);

        // Test face card identification
        assert!(!matches!(
            ace_heart.value,
            Rank::Jack | Rank::Queen | Rank::King
        ));
        assert!(matches!(
            king_spade.value,
            Rank::Jack | Rank::Queen | Rank::King
        ));
    }

    #[test]
    fn test_conditional_joker_builder_pattern() {
        let joker = ConditionalJoker::new(
            JokerId::Banner,
            "Test Joker",
            "Test Description",
            JokerRarity::Rare,
            JokerCondition::MoneyLessThan(50),
            JokerEffect::new().with_chips(20),
        )
        .with_cost(10)
        .with_card_effect(JokerEffect::new().with_mult(2));

        assert_eq!(joker.cost(), 10);
        assert!(joker.card_effect.is_some());
        assert_eq!(joker.rarity(), JokerRarity::Rare);
    }
}
