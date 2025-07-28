//! Basic Economy Jokers implementation
//!
//! This module implements jokers that provide money-earning mechanics.
//! These jokers affect the player's economy through various triggers and conditions.

use crate::{
    config::Config,
    joker::{
        traits::Rarity, GameContext, Joker, JokerEffect, JokerId, JokerIdentity, JokerLifecycle,
        JokerRarity,
    },
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
            // Award $2 per discard using proper config constant
            let base_discards = Config::new().discards; // Use proper config value
            let money_earned = (base_discards * 2) as i32;
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

/// Rocket - Earn $1 at end of round, payout increases by $1 when Boss Blind is defeated
#[derive(Debug, Clone)]
pub struct RocketJoker {
    id: JokerId,
    name: String,
    description: String,
    rarity: JokerRarity,
    cost: usize,
    payout: i32, // Current payout amount, increases when boss blinds are defeated
}

impl Default for RocketJoker {
    fn default() -> Self {
        Self::new()
    }
}

impl RocketJoker {
    pub fn new() -> Self {
        Self {
            id: JokerId::RocketShip,
            name: "Rocket".to_string(),
            description:
                "Earn $1 at end of round, payout increases by $1 when Boss Blind is defeated"
                    .to_string(),
            rarity: JokerRarity::Common,
            cost: 4,
            payout: 1, // Starts at $1
        }
    }

    pub fn increase_payout(&mut self) {
        self.payout += 1;
    }
}

impl JokerIdentity for RocketJoker {
    fn joker_type(&self) -> &'static str {
        "rocket"
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

impl Joker for RocketJoker {
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

    fn on_round_end(&self, _context: &mut GameContext) -> JokerEffect {
        // Always earn current payout at end of round
        JokerEffect::new()
            .with_money(self.payout)
            .with_message(format!("Rocket: +${} (current payout)", self.payout))
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

pub fn create_rocket_joker() -> Box<dyn Joker> {
    Box::new(RocketJoker::new())
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
        assert_eq!(effect.money, 8); // Config default 4 discards * $2
    }

    #[test]
    fn test_rocket() {
        let rocket = RocketJoker::new();

        // Test identity
        assert_eq!(rocket.joker_type(), "rocket");
        assert_eq!(JokerIdentity::name(&rocket), "Rocket");
        assert_eq!(rocket.base_cost(), 4);

        // Test round end effect
        let mut test_context = crate::joker::test_utils::TestContextBuilder::new()
            .with_money(10)
            .build();

        let effect = rocket.on_round_end(&mut test_context);
        assert_eq!(effect.money, 1); // Starts with $1 payout

        // Test payout increase
        let mut rocket = RocketJoker::new();
        rocket.increase_payout();
        let effect = rocket.on_round_end(&mut test_context);
        assert_eq!(effect.money, 2); // Increased to $2
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
            Box::new(RocketJoker::new()),
        ];
        test_context.jokers = &test_jokers;

        let effect = gift_card.on_round_end(&mut test_context);
        // Should increase sell value for 2 jokers
        assert_eq!(effect.sell_value_increase, 1);
    }

    #[test]
    fn test_rocket_payout_modification() {
        let mut rocket = RocketJoker::new();

        // Initial payout should be $1
        assert_eq!(rocket.payout, 1);

        // Test manual payout increase (simulating boss blind defeat)
        rocket.increase_payout();
        assert_eq!(rocket.payout, 2); // Should increase to $2

        // Test another increase
        rocket.increase_payout();
        assert_eq!(rocket.payout, 3); // Should increase to $3

        // Verify the new payout works on round end
        let mut test_context = crate::joker::test_utils::TestContextBuilder::new()
            .with_money(10)
            .build();

        let round_effect = rocket.on_round_end(&mut test_context);
        assert_eq!(round_effect.money, 3); // Should earn current payout ($3)
        assert!(round_effect.message.unwrap().contains("+$3"));
    }
}
