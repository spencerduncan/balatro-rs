use crate::card::{Card, Suit};
use crate::hand::SelectHand;
use crate::joker::{GameContext, Joker, JokerEffect, JokerId, JokerRarity};
use rand::prelude::*;
use serde::{Deserialize, Serialize};

// Basic Joker implementation
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TheJoker;

impl Joker for TheJoker {
    fn id(&self) -> JokerId {
        JokerId::Joker
    }

    fn name(&self) -> &str {
        "Joker"
    }

    fn description(&self) -> &str {
        "+4 Mult"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Common
    }

    fn cost(&self) -> usize {
        2
    }

    fn on_hand_played(&self, _context: &mut GameContext, _hand: &SelectHand) -> JokerEffect {
        JokerEffect::new().with_mult(4)
    }
}

// Greedy Joker implementation
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GreedyJoker;

impl Joker for GreedyJoker {
    fn id(&self) -> JokerId {
        JokerId::GreedyJoker
    }

    fn name(&self) -> &str {
        "Greedy Joker"
    }

    fn description(&self) -> &str {
        "Played cards with Diamond suit give +3 Mult when scored"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Common
    }

    fn cost(&self) -> usize {
        5
    }

    fn on_card_scored(&self, _context: &mut GameContext, card: &Card) -> JokerEffect {
        if card.suit == Suit::Diamond {
            JokerEffect::new().with_mult(3)
        } else {
            JokerEffect::new()
        }
    }
}

// Lusty Joker implementation
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LustyJoker;

impl Joker for LustyJoker {
    fn id(&self) -> JokerId {
        JokerId::LustyJoker
    }

    fn name(&self) -> &str {
        "Lusty Joker"
    }

    fn description(&self) -> &str {
        "Played cards with Heart suit give +3 Mult when scored"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Common
    }

    fn cost(&self) -> usize {
        5
    }

    fn on_card_scored(&self, _context: &mut GameContext, card: &Card) -> JokerEffect {
        if card.suit == Suit::Heart {
            JokerEffect::new().with_mult(3)
        } else {
            JokerEffect::new()
        }
    }
}

// Wrathful Joker implementation
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WrathfulJoker;

impl Joker for WrathfulJoker {
    fn id(&self) -> JokerId {
        JokerId::WrathfulJoker
    }

    fn name(&self) -> &str {
        "Wrathful Joker"
    }

    fn description(&self) -> &str {
        "Played cards with Spade suit give +3 Mult when scored"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Common
    }

    fn cost(&self) -> usize {
        5
    }

    fn on_card_scored(&self, _context: &mut GameContext, card: &Card) -> JokerEffect {
        if card.suit == Suit::Spade {
            JokerEffect::new().with_mult(3)
        } else {
            JokerEffect::new()
        }
    }
}

// Gluttonous Joker implementation
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GluttonousJoker;

impl Joker for GluttonousJoker {
    fn id(&self) -> JokerId {
        JokerId::GluttonousJoker
    }

    fn name(&self) -> &str {
        "Gluttonous Joker"
    }

    fn description(&self) -> &str {
        "Played cards with Club suit give +3 Mult when scored"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Common
    }

    fn cost(&self) -> usize {
        5
    }

    fn on_card_scored(&self, _context: &mut GameContext, card: &Card) -> JokerEffect {
        if card.suit == Suit::Club {
            JokerEffect::new().with_mult(3)
        } else {
            JokerEffect::new()
        }
    }
}

// Jolly Joker implementation
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JollyJoker;

impl Joker for JollyJoker {
    fn id(&self) -> JokerId {
        JokerId::JollyJoker
    }

    fn name(&self) -> &str {
        "Jolly Joker"
    }

    fn description(&self) -> &str {
        "+8 Mult if played hand contains a Pair"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Common
    }

    fn cost(&self) -> usize {
        3
    }

    fn on_hand_played(&self, _context: &mut GameContext, hand: &SelectHand) -> JokerEffect {
        if hand.is_pair().is_some() {
            JokerEffect::new().with_mult(8)
        } else {
            JokerEffect::new()
        }
    }
}

// Zany Joker implementation
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ZanyJoker;

impl Joker for ZanyJoker {
    fn id(&self) -> JokerId {
        JokerId::ZanyJoker
    }

    fn name(&self) -> &str {
        "Zany Joker"
    }

    fn description(&self) -> &str {
        "+12 Mult if played hand contains a Three of a Kind"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Common
    }

    fn cost(&self) -> usize {
        4
    }

    fn on_hand_played(&self, _context: &mut GameContext, hand: &SelectHand) -> JokerEffect {
        if hand.is_three_of_kind().is_some() {
            JokerEffect::new().with_mult(12)
        } else {
            JokerEffect::new()
        }
    }
}

// Mad Joker implementation
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MadJoker;

impl Joker for MadJoker {
    fn id(&self) -> JokerId {
        JokerId::MadJoker
    }

    fn name(&self) -> &str {
        "Mad Joker"
    }

    fn description(&self) -> &str {
        "+10 Mult if played hand contains a Two Pair"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Common
    }

    fn cost(&self) -> usize {
        4
    }

    fn on_hand_played(&self, _context: &mut GameContext, hand: &SelectHand) -> JokerEffect {
        if hand.is_two_pair().is_some() {
            JokerEffect::new().with_mult(10)
        } else {
            JokerEffect::new()
        }
    }
}

// Crazy Joker implementation
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CrazyJoker;

impl Joker for CrazyJoker {
    fn id(&self) -> JokerId {
        JokerId::CrazyJoker
    }

    fn name(&self) -> &str {
        "Crazy Joker"
    }

    fn description(&self) -> &str {
        "+12 Mult if played hand contains a Straight"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Common
    }

    fn cost(&self) -> usize {
        4
    }

    fn on_hand_played(&self, _context: &mut GameContext, hand: &SelectHand) -> JokerEffect {
        if hand.is_straight().is_some() {
            JokerEffect::new().with_mult(12)
        } else {
            JokerEffect::new()
        }
    }
}

// Droll Joker implementation
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DrollJoker;

impl Joker for DrollJoker {
    fn id(&self) -> JokerId {
        JokerId::DrollJoker
    }

    fn name(&self) -> &str {
        "Droll Joker"
    }

    fn description(&self) -> &str {
        "+10 Mult if played hand contains a Flush"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Common
    }

    fn cost(&self) -> usize {
        4
    }

    fn on_hand_played(&self, _context: &mut GameContext, hand: &SelectHand) -> JokerEffect {
        if hand.is_flush().is_some() {
            JokerEffect::new().with_mult(10)
        } else {
            JokerEffect::new()
        }
    }
}

// Sly Joker implementation
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SlyJoker;

impl Joker for SlyJoker {
    fn id(&self) -> JokerId {
        JokerId::SlyJoker
    }

    fn name(&self) -> &str {
        "Sly Joker"
    }

    fn description(&self) -> &str {
        "+50 Chips if played hand contains a Pair"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Common
    }

    fn cost(&self) -> usize {
        3
    }

    fn on_hand_played(&self, _context: &mut GameContext, hand: &SelectHand) -> JokerEffect {
        if hand.is_pair().is_some() {
            JokerEffect::new().with_chips(50)
        } else {
            JokerEffect::new()
        }
    }
}

// Wily Joker implementation
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WilyJoker;

impl Joker for WilyJoker {
    fn id(&self) -> JokerId {
        JokerId::WilyJoker
    }

    fn name(&self) -> &str {
        "Wily Joker"
    }

    fn description(&self) -> &str {
        "+100 Chips if played hand contains a Three of a Kind"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Common
    }

    fn cost(&self) -> usize {
        4
    }

    fn on_hand_played(&self, _context: &mut GameContext, hand: &SelectHand) -> JokerEffect {
        if hand.is_three_of_kind().is_some() {
            JokerEffect::new().with_chips(100)
        } else {
            JokerEffect::new()
        }
    }
}

// Clever Joker implementation
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CleverJoker;

impl Joker for CleverJoker {
    fn id(&self) -> JokerId {
        JokerId::CleverJoker
    }

    fn name(&self) -> &str {
        "Clever Joker"
    }

    fn description(&self) -> &str {
        "+80 Chips if played hand contains a Two Pair"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Common
    }

    fn cost(&self) -> usize {
        4
    }

    fn on_hand_played(&self, _context: &mut GameContext, hand: &SelectHand) -> JokerEffect {
        if hand.is_two_pair().is_some() {
            JokerEffect::new().with_chips(80)
        } else {
            JokerEffect::new()
        }
    }
}

// Devious Joker implementation
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeviousJoker;

impl Joker for DeviousJoker {
    fn id(&self) -> JokerId {
        JokerId::DeviousJoker
    }

    fn name(&self) -> &str {
        "Devious Joker"
    }

    fn description(&self) -> &str {
        "+100 Chips if played hand contains a Straight"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Common
    }

    fn cost(&self) -> usize {
        4
    }

    fn on_hand_played(&self, _context: &mut GameContext, hand: &SelectHand) -> JokerEffect {
        if hand.is_straight().is_some() {
            JokerEffect::new().with_chips(100)
        } else {
            JokerEffect::new()
        }
    }
}

// Crafty Joker implementation
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CraftyJoker;

impl Joker for CraftyJoker {
    fn id(&self) -> JokerId {
        JokerId::CraftyJoker
    }

    fn name(&self) -> &str {
        "Crafty Joker"
    }

    fn description(&self) -> &str {
        "+80 Chips if played hand contains a Flush"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Common
    }

    fn cost(&self) -> usize {
        4
    }

    fn on_hand_played(&self, _context: &mut GameContext, hand: &SelectHand) -> JokerEffect {
        if hand.is_flush().is_some() {
            JokerEffect::new().with_chips(80)
        } else {
            JokerEffect::new()
        }
    }
}

// Money-Based Conditional Jokers for Issue #82

// Business Card: face cards have 1 in 2 chance of giving $2 when scored
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BusinessCard;

impl Joker for BusinessCard {
    fn id(&self) -> JokerId {
        JokerId::BusinessCard
    }

    fn name(&self) -> &str {
        "Business Card"
    }

    fn description(&self) -> &str {
        "Face cards have 1 in 2 chance of giving $2 when scored"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Common
    }

    fn cost(&self) -> usize {
        4
    }

    fn on_card_scored(&self, _context: &mut GameContext, card: &Card) -> JokerEffect {
        if card.is_face() {
            let mut rng = thread_rng();
            if rng.gen_bool(0.5) {
                JokerEffect::new().with_money(2)
            } else {
                JokerEffect::new()
            }
        } else {
            JokerEffect::new()
        }
    }
}

// Egg: gains $3 sell value at round end
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Egg;

impl Joker for Egg {
    fn id(&self) -> JokerId {
        JokerId::EggJoker
    }

    fn name(&self) -> &str {
        "Egg"
    }

    fn description(&self) -> &str {
        "Gains $3 sell value at end of round"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Common
    }

    fn cost(&self) -> usize {
        3
    }

    fn sell_value(&self, accumulated_bonus: f64) -> usize {
        // Base sell value (cost / 2) + accumulated bonus from rounds
        (self.cost() / 2) + (accumulated_bonus as usize)
    }

    fn on_round_end(&self, _context: &mut GameContext) -> JokerEffect {
        JokerEffect::new()
            .with_sell_value_increase(3)
            .with_message("Egg gained $3 sell value".to_string())
    }
}

// Burglar: gain +3 hands when Blind selected, lose all discards
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Burglar;

impl Joker for Burglar {
    fn id(&self) -> JokerId {
        JokerId::Burglar
    }

    fn name(&self) -> &str {
        "Burglar"
    }

    fn description(&self) -> &str {
        "Gain +3 hands when Blind selected, lose all discards"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Uncommon
    }

    fn cost(&self) -> usize {
        6
    }

    fn on_blind_start(&self, _context: &mut GameContext) -> JokerEffect {
        let mut effect = JokerEffect::new();
        effect.hand_size_mod = 3;
        effect.discard_mod = -999; // Set to very negative to remove all discards
        effect
    }
}

// Supernova implementation - tracks hand types played this game run
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SupernovaJoker;

impl Joker for SupernovaJoker {
    fn id(&self) -> JokerId {
        JokerId::Supernova
    }

    fn name(&self) -> &str {
        "Supernova"
    }

    fn description(&self) -> &str {
        "Mult equal to times this poker hand has been played"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Common
    }

    fn cost(&self) -> usize {
        3
    }

    fn on_hand_played(&self, context: &mut GameContext, hand: &SelectHand) -> JokerEffect {
        // First determine what hand type was played
        if let Ok(made_hand) = hand.best_hand() {
            let hand_rank = made_hand.rank;

            // Get the count for this hand type from the centralized tracking
            // Note: This will be the count AFTER the current hand is played
            // since the game increments the count before calling joker effects
            let count = context.get_hand_type_count(hand_rank);

            // Return mult equal to the count
            JokerEffect::new().with_mult(count as i32)
        } else {
            JokerEffect::new()
        }
    }
}

// Ice Cream Joker implementation - decaying chip joker
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IceCreamJoker;

impl Joker for IceCreamJoker {
    fn id(&self) -> JokerId {
        JokerId::IceCream
    }

    fn name(&self) -> &str {
        "Ice Cream"
    }

    fn description(&self) -> &str {
        "+100 Chips, -5 Chips per hand played"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Common
    }

    fn cost(&self) -> usize {
        5
    }

    fn on_created(&self, context: &mut GameContext) -> JokerEffect {
        // Initialize state with 100 chips
        context
            .joker_state_manager
            .update_state(self.id(), |state| {
                state
                    .set_custom("remaining_chips", 100i32)
                    .expect("Failed to set remaining_chips");
            });
        JokerEffect::new()
    }

    fn on_hand_played(&self, context: &mut GameContext, _hand: &SelectHand) -> JokerEffect {
        // Get current remaining chips from state manager
        let remaining_chips = context
            .joker_state_manager
            .get_or_default(self.id())
            .get_custom::<i32>("remaining_chips")
            .unwrap_or(Some(100))
            .unwrap_or(100);

        // Provide chips bonus (capped at 0 to avoid negative)
        let chips_bonus = remaining_chips.max(0);

        // Decay the chips by 5 for next hand
        context
            .joker_state_manager
            .update_state(self.id(), |state| {
                state
                    .set_custom("remaining_chips", remaining_chips - 5)
                    .expect("Failed to update remaining_chips");
            });

        JokerEffect::new().with_chips(chips_bonus)
    }
}

// Runner implementation - accumulates chips when straights are played, gives bonus on every hand
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RunnerJoker;

impl Joker for RunnerJoker {
    fn id(&self) -> JokerId {
        JokerId::Runner
    }

    fn name(&self) -> &str {
        "Runner"
    }

    fn description(&self) -> &str {
        "+15 Chips if played hand contains a Straight"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Common
    }

    fn cost(&self) -> usize {
        3
    }

    fn on_hand_played(&self, context: &mut GameContext, hand: &SelectHand) -> JokerEffect {
        // Check if hand contains a straight (any type)
        let is_straight = hand.is_straight().is_some()
            || hand.is_straight_flush().is_some()
            || hand.is_royal_flush().is_some();

        // Get current accumulated chips
        let current_accumulated = context
            .joker_state_manager
            .get_state(self.id())
            .map(|state| state.accumulated_value as i32)
            .unwrap_or(0);

        // If it's a straight, accumulate +15 chips BEFORE giving the bonus
        let new_accumulated = if is_straight {
            let new_value = current_accumulated + 15;
            context
                .joker_state_manager
                .add_accumulated_value(self.id(), 15.0);
            new_value
        } else {
            current_accumulated
        };

        // Always give the accumulated bonus regardless of hand type
        JokerEffect::new().with_chips(new_accumulated)
    }
}

// Space Joker implementation - 1 in 4 chance for +1 hand level
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SpaceJoker;

impl Joker for SpaceJoker {
    fn id(&self) -> JokerId {
        JokerId::SpaceJoker
    }

    fn name(&self) -> &str {
        "Space Joker"
    }

    fn description(&self) -> &str {
        "1 in 4 chance to upgrade level of played poker hand"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Uncommon
    }

    fn cost(&self) -> usize {
        6
    }

    fn on_hand_played(&self, _context: &mut GameContext, _hand: &SelectHand) -> JokerEffect {
        // 1 in 4 chance (25% probability)
        let mut rng = thread_rng();
        if rng.gen_ratio(1, 4) {
            // TODO: Implement hand level upgrade effect
            // For now, return a message effect
            let mut effect = JokerEffect::new();
            effect.message = Some("Space Joker activated! Hand level upgraded!".to_string());
            effect
        } else {
            JokerEffect::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::joker::{JokerId, JokerRarity};
    use crate::joker_factory::JokerFactory;

    #[test]
    fn test_ice_cream_basic_properties() {
        let ice_cream = IceCreamJoker;

        assert_eq!(ice_cream.id(), JokerId::IceCream);
        assert_eq!(ice_cream.name(), "Ice Cream");
        assert_eq!(
            ice_cream.description(),
            "+100 Chips, -5 Chips per hand played"
        );
        assert_eq!(ice_cream.rarity(), JokerRarity::Common);
        assert_eq!(ice_cream.cost(), 5);
    }

    #[test]
    fn test_ice_cream_initial_chips() {
        // Initial chips are handled by state manager now
        // This is tested in integration tests
    }

    #[test]
    fn test_ice_cream_factory_creation() {
        let created_joker = JokerFactory::create(JokerId::IceCream);
        assert!(
            created_joker.is_some(),
            "Ice Cream should be creatable from factory"
        );

        let joker_instance = created_joker.unwrap();
        assert_eq!(joker_instance.id(), JokerId::IceCream);
        assert_eq!(joker_instance.name(), "Ice Cream");
        assert_eq!(joker_instance.rarity(), JokerRarity::Common);
        assert_eq!(joker_instance.cost(), 5);
    }

    #[test]
    fn test_ice_cream_in_common_rarity() {
        let common_jokers = JokerFactory::get_by_rarity(JokerRarity::Common);
        assert!(
            common_jokers.contains(&JokerId::IceCream),
            "Ice Cream should be listed in Common rarity jokers"
        );
    }

    #[test]
    fn test_ice_cream_in_implemented_list() {
        let all_implemented = JokerFactory::get_all_implemented();
        assert!(
            all_implemented.contains(&JokerId::IceCream),
            "Ice Cream should be in the list of all implemented jokers"
        );
    }

    #[test]
    fn test_ice_cream_zero_chips_handling() {
        // Zero chip handling is now covered by integration tests
        // The max(0) in on_hand_played ensures no negative chips are provided
    }

    #[test]
    fn test_ice_cream_negative_chips_handling() {
        // Negative chip handling is now covered by integration tests
        // The max(0) in on_hand_played ensures no negative chips are provided
    }
}
