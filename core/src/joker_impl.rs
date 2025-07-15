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
            let hand_type = format!("{:?}", hand_rank); // Convert HandRank to string
            
            // Increment the count for this hand type
            let current_count: i32 = context.joker_state_manager
                .get_custom_data(self.id(), &hand_type)
                .unwrap_or(Some(0))
                .unwrap_or(0);
            
            let new_count = current_count + 1;
            
            // Update the count for this hand type
            context.joker_state_manager
                .set_custom_data(self.id(), &hand_type, new_count)
                .unwrap_or(());
            
            // Return mult equal to the new count
            JokerEffect::new().with_mult(new_count)
        } else {
            JokerEffect::new()
        }
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
        let is_straight = hand.is_straight().is_some() || 
                         hand.is_straight_flush().is_some() || 
                         hand.is_royal_flush().is_some();
        
        // Get current accumulated chips
        let current_accumulated = context.joker_state_manager
            .get_state(self.id())
            .map(|state| state.accumulated_value as i32)
            .unwrap_or(0);
        
        // If it's a straight, accumulate +15 chips BEFORE giving the bonus
        let new_accumulated = if is_straight {
            let new_value = current_accumulated + 15;
            context.joker_state_manager.add_accumulated_value(self.id(), 15.0);
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
