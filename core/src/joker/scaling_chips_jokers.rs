//! Scaling Chips Jokers for Issue #644
//!
//! This module implements the three jokers with specification violations identified in Issue #644:
//! - Castle: Gains chips per discarded card of specific suit (changes each round)
//! - Wee: Gains chips when 2s are scored
//! - Stuntman: Provides flat chips and reduces hand size

use crate::card::{Card, Suit, Value};
use crate::hand::SelectHand;
use crate::joker::{GameContext, Joker, JokerEffect, JokerId, JokerRarity};
use serde::{Deserialize, Serialize};

/// Castle Joker implementation - specification compliant
/// Per joker.json: "This Joker gains {C:chips}+#1#{} Chips per discarded {V:1}#2#{} card, suit changes every round"
///
/// Implements suit cycling: Heart (round % 4 == 0) → Diamond (1) → Club (2) → Spade (3)
/// Gains +3 chips per discarded card of the active suit
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CastleJoker;

impl CastleJoker {
    pub fn new() -> Self {
        Self
    }

    /// Determines the active suit based on the current round
    /// Round % 4: 0=Heart, 1=Diamond, 2=Club, 3=Spade
    fn get_active_suit(&self, round: u32) -> Suit {
        match round % 4 {
            0 => Suit::Heart,
            1 => Suit::Diamond,
            2 => Suit::Club,
            _ => Suit::Spade,
        }
    }

    /// Returns the suit name for display messages
    fn get_suit_name(&self, suit: Suit) -> &'static str {
        match suit {
            Suit::Heart => "Heart",
            Suit::Diamond => "Diamond",
            Suit::Club => "Club",
            Suit::Spade => "Spade",
        }
    }
}

impl Joker for CastleJoker {
    fn id(&self) -> JokerId {
        JokerId::Castle
    }

    fn name(&self) -> &str {
        "Castle"
    }

    fn description(&self) -> &str {
        "This Joker gains +3 Chips per discarded card of specific suit, suit changes every round"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Rare
    }

    fn cost(&self) -> usize {
        8
    }

    /// Called when hand is played - provides accumulated chips and shows current active suit
    fn on_hand_played(&self, context: &mut GameContext, _hand: &SelectHand) -> JokerEffect {
        let active_suit = self.get_active_suit(context.round);
        let suit_name = self.get_suit_name(active_suit);

        // Get accumulated chips from state manager
        let accumulated_chips = context
            .joker_state_manager
            .get_accumulated_value(self.id())
            .unwrap_or(0.0) as i32;

        JokerEffect {
            chips: accumulated_chips,
            ..JokerEffect::new()
        }
        .with_message(format!(
            "Castle: Active suit {suit_name}, {accumulated_chips} chips"
        ))
    }

    /// Called when cards are discarded - gains chips for cards of the active suit
    fn on_discard(&self, context: &mut GameContext, cards: &[Card]) -> JokerEffect {
        let active_suit = self.get_active_suit(context.round);
        let matching_cards: Vec<&Card> = cards
            .iter()
            .filter(|card| card.suit == active_suit)
            .collect();

        if matching_cards.is_empty() {
            return JokerEffect::new();
        }

        let chips_gained = (matching_cards.len() as i32) * 3;

        // Update accumulated value through JokerStateManager
        context
            .joker_state_manager
            .add_accumulated_value(self.id(), chips_gained as f64);

        JokerEffect {
            chips: chips_gained,
            ..JokerEffect::new()
        }
        .with_message(format!(
            "Castle: +{chips_gained} Chips gained from {} {} cards",
            matching_cards.len(),
            self.get_suit_name(active_suit)
        ))
    }
}

/// Wee Joker implementation - specification compliant
/// Per joker.json: "This Joker gains {C:chips}+#2#{} Chips when each played {C:attention}2{} is scored"
/// Provides +8 chips for each 2 that is scored in the current hand
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WeeJoker;

impl WeeJoker {
    pub fn new() -> Self {
        Self
    }
}

impl Joker for WeeJoker {
    fn id(&self) -> JokerId {
        JokerId::Wee
    }

    fn name(&self) -> &str {
        "Wee Joker"
    }

    fn description(&self) -> &str {
        "This Joker gains +8 Chips when each played 2 is scored"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Rare
    }

    fn cost(&self) -> usize {
        8
    }

    /// Called for each card as it's scored - detects 2s and provides chips
    /// Per specification: gains chips when each played 2 is scored
    fn on_card_scored(&self, _context: &mut GameContext, card: &Card) -> JokerEffect {
        if card.value == Value::Two {
            // Found a 2 being scored - provide +8 chips per specification
            JokerEffect {
                chips: 8, // +8 chips per 2 scored (joker.json #2# parameter)
                ..JokerEffect::new()
            }
            .with_message("Wee: +8 Chips gained from scoring a 2".to_string())
        } else {
            JokerEffect::new()
        }
    }
}

/// Stuntman Joker implementation - specification compliant
/// Per joker.json: "{C:chips}+#1#{} Chips, {C:attention}-#2#{} hand size"
/// This is a STATIC joker with fixed bonuses: +300 chips, -2 hand size
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StuntmanJoker;

impl StuntmanJoker {
    pub fn new() -> Self {
        Self
    }
}

impl Joker for StuntmanJoker {
    fn id(&self) -> JokerId {
        JokerId::Stuntman
    }

    fn name(&self) -> &str {
        "Stuntman"
    }

    fn description(&self) -> &str {
        "+300 Chips, -2 hand size"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Rare
    }

    fn cost(&self) -> usize {
        8
    }

    /// Stuntman provides chips on every hand played
    /// This implements the joker.json specification exactly: +300 chips
    fn on_hand_played(&self, _context: &mut GameContext, _hand: &SelectHand) -> JokerEffect {
        JokerEffect {
            chips: 300, // +300 chips per specification
            ..JokerEffect::new()
        }
        .with_message("Stuntman: +300 Chips".to_string())
    }

    /// Stuntman reduces hand size by 2
    /// This implements the joker.json specification exactly: -2 hand size
    fn modify_hand_size(&self, _context: &GameContext, base_size: usize) -> usize {
        base_size.saturating_sub(2) // -2 hand size with underflow protection
    }
}

// Additional scaling chips jokers for completeness (from the re-export module)

/// Hiker Joker - every scored card permanently gains +5 chips
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HikerJoker;

impl HikerJoker {
    pub fn new() -> Self {
        Self
    }
}

impl Joker for HikerJoker {
    fn id(&self) -> JokerId {
        JokerId::Hiker
    }

    fn name(&self) -> &str {
        "Hiker"
    }

    fn description(&self) -> &str {
        "Every played card permanently gains +5 Chips when scored"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Uncommon
    }

    fn cost(&self) -> usize {
        6
    }

    fn on_card_scored(&self, _context: &mut GameContext, _card: &Card) -> JokerEffect {
        JokerEffect {
            chips: 5,
            ..JokerEffect::new()
        }
        .with_message("Hiker: +5 Chips".to_string())
    }
}

/// Odd Todd Joker - provides chips for odd rank cards (A, 9, 7, 5, 3)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OddToddJoker;

impl OddToddJoker {
    pub fn new() -> Self {
        Self
    }

    fn is_odd_rank(&self, value: Value) -> bool {
        matches!(
            value,
            Value::Ace | Value::Three | Value::Five | Value::Seven | Value::Nine
        )
    }
}

impl Joker for OddToddJoker {
    fn id(&self) -> JokerId {
        JokerId::OddTodd
    }

    fn name(&self) -> &str {
        "Odd Todd"
    }

    fn description(&self) -> &str {
        "+30 Chips for odd rank cards (A, 9, 7, 5, 3)"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Common
    }

    fn cost(&self) -> usize {
        3
    }

    fn on_card_scored(&self, _context: &mut GameContext, card: &Card) -> JokerEffect {
        if self.is_odd_rank(card.value) {
            JokerEffect {
                chips: 30,
                ..JokerEffect::new()
            }
            .with_message("Odd Todd: +30 Chips from odd rank".to_string())
        } else {
            JokerEffect::new()
        }
    }
}

/// Arrowhead Joker - provides chips for Spade suit cards
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ArrowheadJoker;

impl ArrowheadJoker {
    pub fn new() -> Self {
        Self
    }
}

impl Joker for ArrowheadJoker {
    fn id(&self) -> JokerId {
        JokerId::Arrowhead
    }

    fn name(&self) -> &str {
        "Arrowhead"
    }

    fn description(&self) -> &str {
        "+50 Chips for each Spade scored"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Common
    }

    fn cost(&self) -> usize {
        3
    }

    fn on_card_scored(&self, _context: &mut GameContext, card: &Card) -> JokerEffect {
        if card.suit == Suit::Spade {
            JokerEffect {
                chips: 50,
                ..JokerEffect::new()
            }
            .with_message("Arrowhead: +50 Chips from Spade".to_string())
        } else {
            JokerEffect::new()
        }
    }
}

/// Scholar Joker - provides chips and mult for Aces
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ScholarJoker;

impl ScholarJoker {
    pub fn new() -> Self {
        Self
    }
}

impl Joker for ScholarJoker {
    fn id(&self) -> JokerId {
        JokerId::Scholar
    }

    fn name(&self) -> &str {
        "Scholar"
    }

    fn description(&self) -> &str {
        "+20 Chips, +4 Mult for each Ace scored"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Common
    }

    fn cost(&self) -> usize {
        3
    }

    fn on_card_scored(&self, _context: &mut GameContext, card: &Card) -> JokerEffect {
        if card.value == Value::Ace {
            JokerEffect {
                chips: 20,
                mult: 4,
                ..JokerEffect::new()
            }
            .with_message("Scholar: +20 Chips, +4 Mult from Ace".to_string())
        } else {
            JokerEffect::new()
        }
    }
}
