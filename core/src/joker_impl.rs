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

// Runner Joker implementation
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Runner;

impl Joker for Runner {
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

    fn on_hand_played(&self, _context: &mut GameContext, hand: &SelectHand) -> JokerEffect {
        // Check for straight in any form: straight, straight flush, or royal flush
        if hand.is_straight().is_some()
            || hand.is_straight_flush().is_some()
            || hand.is_royal_flush().is_some()
        {
            JokerEffect::new().with_chips(15)
        } else {
            JokerEffect::new()
        }
    }
}

// Supernova Joker implementation - adds hand played to mult
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
        "Adds hand played to Mult"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Common
    }

    fn cost(&self) -> usize {
        4
    }

    fn on_hand_played(&self, _context: &mut GameContext, hand: &SelectHand) -> JokerEffect {
        // Get the hand rank and convert to mult value
        if let Ok(made_hand) = hand.best_hand() {
            let mult_bonus = match made_hand.rank {
                crate::rank::HandRank::HighCard => 1,
                crate::rank::HandRank::OnePair => 2,
                crate::rank::HandRank::TwoPair => 2,
                crate::rank::HandRank::ThreeOfAKind => 3,
                crate::rank::HandRank::Straight => 4,
                crate::rank::HandRank::Flush => 4,
                crate::rank::HandRank::FullHouse => 5,
                crate::rank::HandRank::FourOfAKind => 7,
                crate::rank::HandRank::StraightFlush => 8,
                crate::rank::HandRank::RoyalFlush => 10,
                crate::rank::HandRank::FiveOfAKind => 12,
                crate::rank::HandRank::FlushHouse => 14,
                crate::rank::HandRank::FlushFive => 16,
            };
            JokerEffect::new().with_mult(mult_bonus)
        } else {
            JokerEffect::new()
        }
    }
}

// Space Joker implementation - 1 in 4 chance for +1 hand level
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SpaceJokerImpl;

impl Joker for SpaceJokerImpl {
    fn id(&self) -> JokerId {
        JokerId::SpaceJoker
    }

    fn name(&self) -> &str {
        "Space Joker"
    }

    fn description(&self) -> &str {
        "1 in 4 chance to upgrade level of played hand"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Uncommon
    }

    fn cost(&self) -> usize {
        5
    }

    fn on_hand_played(&self, _context: &mut GameContext, _hand: &SelectHand) -> JokerEffect {
        // 1 in 4 chance (25%) to trigger
        let mut rng = thread_rng();
        if rng.gen_bool(0.25) {
            // For now, provide a placeholder effect since hand level upgrading requires game state changes
            // In a full implementation, this would need to modify the hand level in the game state
            // TODO: Implement actual hand level upgrade functionality
            let mut effect = JokerEffect::new();
            effect.message = Some("Space Joker activated! +1 hand level".to_string());
            effect
        } else {
            JokerEffect::new()
        }
    }
}
