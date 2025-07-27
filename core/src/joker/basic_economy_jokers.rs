//! Basic Economy Jokers implementation
//!
//! This module implements jokers that provide money-earning mechanics.
//! These jokers affect the player's economy through various triggers and conditions.

use crate::{
    card::Card,
    hand::SelectHand,
    joker::{
        traits::{ProcessContext, ProcessResult, Rarity},
        GameContext, Joker, JokerEffect, JokerGameplay, JokerId, JokerIdentity, JokerLifecycle,
        JokerRarity,
    },
    stage::Stage,
};

/// Delayed Gratification - Earn $2 per discard if no discards are used by end of round
#[derive(Debug, Clone)]
pub struct DelayedGratificationJoker {
    id: JokerId,
    name: String,
    description: String,
    rarity: JokerRarity,
    cost: usize,
}

impl Default for DelayedGratificationJoker {
    fn default() -> Self {
        Self::new()
    }
}

impl DelayedGratificationJoker {
    pub fn new() -> Self {
        Self {
            id: JokerId::DelayedGratification,
            name: "Delayed Gratification".to_string(),
            description: "Earn $2 per discard if no discards are used by end of round".to_string(),
            rarity: JokerRarity::Common,
            cost: 4,
        }
    }
}

impl JokerIdentity for DelayedGratificationJoker {
    fn joker_type(&self) -> &'static str {
        "delayed_gratification"
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn rarity(&self) -> Rarity {
        match self.rarity {
            JokerRarity::Common => Rarity::Common,
            JokerRarity::Uncommon => Rarity::Uncommon,
            JokerRarity::Rare => Rarity::Rare,
            JokerRarity::Legendary => Rarity::Legendary,
        }
    }

    fn base_cost(&self) -> u64 {
        self.cost as u64
    }
}

impl Joker for DelayedGratificationJoker {
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

    fn on_round_end(&self, context: &mut GameContext) -> JokerEffect {
        // Check if player used no discards
        if context.discards_used == 0 {
            // Award $2 per discard (assuming standard 3 discards)
            // TODO: Get actual discard count from game state
            let base_discards = 3; // Standard discard count
            let money_earned = base_discards * 2;
            JokerEffect::new()
                .with_money(money_earned)
                .with_message(format!(
                    "Delayed Gratification: +${money_earned} (no discards used)"
                ))
        } else {
            JokerEffect::new()
        }
    }
}

impl JokerLifecycle for DelayedGratificationJoker {
    fn on_round_end(&mut self) {
        // Money is handled in the Joker trait method
    }
}

/// Seed Money - Earn $1 for each 5 of a kind contained in played hand
#[derive(Debug, Clone)]
pub struct SeedMoneyJoker {
    id: JokerId,
    name: String,
    description: String,
    rarity: JokerRarity,
    cost: usize,
}

impl Default for SeedMoneyJoker {
    fn default() -> Self {
        Self::new()
    }
}

impl SeedMoneyJoker {
    pub fn new() -> Self {
        Self {
            id: JokerId::SeedMoney,
            name: "Seed Money".to_string(),
            description: "Earn $1 for each 5 of a kind contained in played hand".to_string(),
            rarity: JokerRarity::Common,
            cost: 3,
        }
    }

    fn count_five_of_a_kinds(cards: &[Card]) -> usize {
        // Count occurrences of each value
        let mut value_counts = std::collections::HashMap::new();
        for card in cards {
            *value_counts.entry(card.value).or_insert(0) += 1;
        }

        // Count how many values have 5 or more cards
        value_counts.values().filter(|&&count| count >= 5).count()
    }
}

impl JokerIdentity for SeedMoneyJoker {
    fn joker_type(&self) -> &'static str {
        "seed_money"
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn rarity(&self) -> Rarity {
        match self.rarity {
            JokerRarity::Common => Rarity::Common,
            JokerRarity::Uncommon => Rarity::Uncommon,
            JokerRarity::Rare => Rarity::Rare,
            JokerRarity::Legendary => Rarity::Legendary,
        }
    }

    fn base_cost(&self) -> u64 {
        self.cost as u64
    }
}

impl Joker for SeedMoneyJoker {
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

    fn on_hand_played(&self, _context: &mut GameContext, hand: &SelectHand) -> JokerEffect {
        let cards: Vec<Card> = hand.cards().to_vec();
        let five_of_a_kind_count = Self::count_five_of_a_kinds(&cards);

        if five_of_a_kind_count > 0 {
            let money_earned = five_of_a_kind_count as i32;
            JokerEffect::new()
                .with_money(money_earned)
                .with_message(format!(
                    "Seed Money: +${} ({} five of a kind{})",
                    money_earned,
                    five_of_a_kind_count,
                    if five_of_a_kind_count > 1 { "s" } else { "" }
                ))
        } else {
            JokerEffect::new()
        }
    }
}

impl JokerGameplay for SeedMoneyJoker {
    fn process(&mut self, stage: &Stage, context: &mut ProcessContext) -> ProcessResult {
        if !matches!(stage, Stage::Blind(_)) {
            return ProcessResult::default();
        }

        let _five_of_a_kind_count = Self::count_five_of_a_kinds(context.played_cards);

        // Note: ProcessResult doesn't have money_earned field
        // Money earning is handled through JokerEffect in the old trait
        ProcessResult::default()
    }

    fn can_trigger(&self, stage: &Stage, context: &ProcessContext) -> bool {
        matches!(stage, Stage::Blind(_)) && Self::count_five_of_a_kinds(context.played_cards) > 0
    }
}

/// To The Moon - Earn $1 of interest for every $5 you have at end of round (max $5)
#[derive(Debug, Clone)]
pub struct ToTheMoonJoker {
    id: JokerId,
    name: String,
    description: String,
    rarity: JokerRarity,
    cost: usize,
}

impl Default for ToTheMoonJoker {
    fn default() -> Self {
        Self::new()
    }
}

impl ToTheMoonJoker {
    pub fn new() -> Self {
        Self {
            id: JokerId::ToTheMoon,
            name: "To The Moon".to_string(),
            description: "Earn $1 of interest for every $5 you have at end of round (max $5)"
                .to_string(),
            rarity: JokerRarity::Uncommon,
            cost: 5,
        }
    }
}

impl JokerIdentity for ToTheMoonJoker {
    fn joker_type(&self) -> &'static str {
        "to_the_moon"
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn rarity(&self) -> Rarity {
        match self.rarity {
            JokerRarity::Common => Rarity::Common,
            JokerRarity::Uncommon => Rarity::Uncommon,
            JokerRarity::Rare => Rarity::Rare,
            JokerRarity::Legendary => Rarity::Legendary,
        }
    }

    fn base_cost(&self) -> u64 {
        self.cost as u64
    }
}

impl Joker for ToTheMoonJoker {
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

    fn on_round_end(&self, context: &mut GameContext) -> JokerEffect {
        // Calculate interest: $1 per $5 owned, max $5
        let interest = (context.money / 5).clamp(0, 5);

        if interest > 0 {
            JokerEffect::new()
                .with_money(interest)
                .with_message(format!("To The Moon: +${interest} interest"))
        } else {
            JokerEffect::new()
        }
    }
}

impl JokerLifecycle for ToTheMoonJoker {
    fn on_round_end(&mut self) {
        // Money is handled in the Joker trait method
    }
}

/// Gift Card - Add $1 sell value to each Joker and consumable card in shop at end of round
#[derive(Debug, Clone)]
pub struct GiftCardJoker {
    id: JokerId,
    name: String,
    description: String,
    rarity: JokerRarity,
    cost: usize,
}

impl Default for GiftCardJoker {
    fn default() -> Self {
        Self::new()
    }
}

impl GiftCardJoker {
    pub fn new() -> Self {
        Self {
            id: JokerId::GiftCard,
            name: "Gift Card".to_string(),
            description: "Add $1 sell value to each Joker and consumable card at end of round"
                .to_string(),
            rarity: JokerRarity::Uncommon,
            cost: 6,
        }
    }
}

impl JokerIdentity for GiftCardJoker {
    fn joker_type(&self) -> &'static str {
        "gift_card"
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn rarity(&self) -> Rarity {
        match self.rarity {
            JokerRarity::Common => Rarity::Common,
            JokerRarity::Uncommon => Rarity::Uncommon,
            JokerRarity::Rare => Rarity::Rare,
            JokerRarity::Legendary => Rarity::Legendary,
        }
    }

    fn base_cost(&self) -> u64 {
        self.cost as u64
    }
}

impl Joker for GiftCardJoker {
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

    fn on_round_end(&self, context: &mut GameContext) -> JokerEffect {
        // Count all jokers (including this one)
        let joker_count = context.jokers.len();
        // TODO: Count consumable cards when implemented
        let consumable_count = 0;

        let total_items = joker_count + consumable_count;

        if total_items > 0 {
            // Each item gains $1 sell value
            JokerEffect::new()
                .with_sell_value_increase(1)
                .with_message(format!(
                    "Gift Card: {total_items} items gained $1 sell value"
                ))
        } else {
            JokerEffect::new()
        }
    }
}

impl JokerLifecycle for GiftCardJoker {
    fn on_round_end(&mut self) {
        // Sell value increase is handled in the Joker trait method
    }
}

/// Factory functions for creating basic economy jokers
pub fn create_delayed_gratification_joker() -> Box<dyn Joker> {
    Box::new(DelayedGratificationJoker::new())
}

pub fn create_seed_money_joker() -> Box<dyn Joker> {
    Box::new(SeedMoneyJoker::new())
}

pub fn create_to_the_moon_joker() -> Box<dyn Joker> {
    Box::new(ToTheMoonJoker::new())
}

pub fn create_gift_card_joker() -> Box<dyn Joker> {
    Box::new(GiftCardJoker::new())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::{Card, Suit, Value};

    #[test]
    fn test_delayed_gratification() {
        let delayed = DelayedGratificationJoker::new();

        // Test identity
        assert_eq!(delayed.joker_type(), "delayed_gratification");
        assert_eq!(JokerIdentity::name(&delayed), "Delayed Gratification");
        assert_eq!(delayed.base_cost(), 4);

        // Test with unused discards
        let mut test_context = crate::joker::test_utils::TestContextBuilder::new()
            .with_money(10)
            .with_discards_used(0) // 0 discards used
            .build();

        let effect = delayed.on_round_end(&mut test_context);
        assert_eq!(effect.money, 6); // Standard 3 discards * $2
    }

    #[test]
    fn test_seed_money() {
        let seed_money = SeedMoneyJoker::new();

        // Test identity
        assert_eq!(seed_money.joker_type(), "seed_money");
        assert_eq!(JokerIdentity::name(&seed_money), "Seed Money");
        assert_eq!(seed_money.base_cost(), 3);

        // Test with five of a kind
        let cards = vec![
            Card::new(Value::Ace, Suit::Heart),
            Card::new(Value::Ace, Suit::Diamond),
            Card::new(Value::Ace, Suit::Spade),
            Card::new(Value::Ace, Suit::Club),
            Card::new(Value::Ace, Suit::Heart), // 5th Ace
        ];

        assert_eq!(SeedMoneyJoker::count_five_of_a_kinds(&cards), 1);
    }

    #[test]
    fn test_to_the_moon() {
        let moon = ToTheMoonJoker::new();

        // Test identity
        assert_eq!(moon.joker_type(), "to_the_moon");
        assert_eq!(JokerIdentity::name(&moon), "To The Moon");
        assert_eq!(moon.base_cost(), 5);

        // Test interest calculation
        let mut test_context = crate::joker::test_utils::TestContextBuilder::new()
            .with_money(23) // Should give $4 interest ($23 / $5 = 4.6, rounded down to 4)
            .build();

        let effect = moon.on_round_end(&mut test_context);
        assert_eq!(effect.money, 4);

        // Test max interest cap
        let mut rich_context = crate::joker::test_utils::TestContextBuilder::new()
            .with_money(100) // Should cap at $5 interest
            .build();

        let effect_max = moon.on_round_end(&mut rich_context);
        assert_eq!(effect_max.money, 5);
    }

    #[test]
    fn test_gift_card() {
        let gift_card = GiftCardJoker::new();

        // Test identity
        assert_eq!(gift_card.joker_type(), "gift_card");
        assert_eq!(JokerIdentity::name(&gift_card), "Gift Card");
        assert_eq!(gift_card.base_cost(), 6);

        // Test round end effect
        let gift_card = GiftCardJoker::new();
        let mut test_context = crate::joker::test_utils::TestContextBuilder::new().build();

        // Create some test jokers
        let test_jokers: Vec<Box<dyn Joker>> = vec![
            Box::new(DelayedGratificationJoker::new()),
            Box::new(SeedMoneyJoker::new()),
        ];
        test_context.jokers = &test_jokers;

        let effect = gift_card.on_round_end(&mut test_context);
        // Should increase sell value for 2 jokers
        assert_eq!(effect.sell_value_increase, 1);
    }

    #[test]
    fn test_five_of_a_kind_detection() {
        // Test with exactly 5 of a kind
        let five_cards = vec![
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::King, Suit::Diamond),
            Card::new(Value::King, Suit::Spade),
            Card::new(Value::King, Suit::Club),
            Card::new(Value::King, Suit::Heart), // 5th King
            Card::new(Value::Ace, Suit::Heart),  // Different card
        ];
        assert_eq!(SeedMoneyJoker::count_five_of_a_kinds(&five_cards), 1);

        // Test with more than 5 of a kind
        let six_cards = vec![
            Card::new(Value::Queen, Suit::Heart),
            Card::new(Value::Queen, Suit::Diamond),
            Card::new(Value::Queen, Suit::Spade),
            Card::new(Value::Queen, Suit::Club),
            Card::new(Value::Queen, Suit::Heart),   // 5th Queen
            Card::new(Value::Queen, Suit::Diamond), // 6th Queen
        ];
        assert_eq!(SeedMoneyJoker::count_five_of_a_kinds(&six_cards), 1);

        // Test with multiple five of a kinds
        let multiple = vec![
            // 5 Aces
            Card::new(Value::Ace, Suit::Heart),
            Card::new(Value::Ace, Suit::Diamond),
            Card::new(Value::Ace, Suit::Spade),
            Card::new(Value::Ace, Suit::Club),
            Card::new(Value::Ace, Suit::Heart),
            // 5 Kings
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::King, Suit::Diamond),
            Card::new(Value::King, Suit::Spade),
            Card::new(Value::King, Suit::Club),
            Card::new(Value::King, Suit::Heart),
        ];
        assert_eq!(SeedMoneyJoker::count_five_of_a_kinds(&multiple), 2);

        // Test with no five of a kind
        let no_five = vec![
            Card::new(Value::Ace, Suit::Heart),
            Card::new(Value::King, Suit::Diamond),
            Card::new(Value::Queen, Suit::Spade),
            Card::new(Value::Jack, Suit::Club),
        ];
        assert_eq!(SeedMoneyJoker::count_five_of_a_kinds(&no_five), 0);
    }
}
