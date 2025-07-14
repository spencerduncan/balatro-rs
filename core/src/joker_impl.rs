use crate::card::{Card, Suit};
use crate::hand::SelectHand;
use crate::joker::{Joker, JokerEffect, JokerRarity, JokerId, GameContext};
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