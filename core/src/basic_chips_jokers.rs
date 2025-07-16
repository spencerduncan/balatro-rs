use crate::card::Value;
use crate::hand::SelectHand;
use crate::joker::{GameContext, Joker, JokerEffect, JokerId, JokerRarity};

/// Banner joker: +30 chips per remaining discard
#[derive(Debug, Clone)]
pub struct BannerJoker {
    pub id: JokerId,
}

impl Default for BannerJoker {
    fn default() -> Self {
        Self::new()
    }
}

impl BannerJoker {
    pub fn new() -> Self {
        Self {
            id: JokerId::Banner,
        }
    }
}

impl Joker for BannerJoker {
    fn id(&self) -> JokerId {
        self.id
    }

    fn name(&self) -> &str {
        "Banner"
    }

    fn description(&self) -> &str {
        "+30 Chips for each remaining discard"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Common
    }

    fn cost(&self) -> usize {
        3
    }

    fn on_hand_played(&self, context: &mut GameContext, _hand: &SelectHand) -> JokerEffect {
        // Calculate discards remaining (assuming 5 total discards per round)
        const MAX_DISCARDS: u32 = 5;
        let discards_remaining = MAX_DISCARDS.saturating_sub(context.discards_used);
        let chips_bonus = 30 * discards_remaining as i32;

        JokerEffect::new().with_chips(chips_bonus)
    }
}

/// Bull joker: +2 chips per $1 owned
#[derive(Debug, Clone)]
pub struct BullJoker {
    pub id: JokerId,
}

impl Default for BullJoker {
    fn default() -> Self {
        Self::new()
    }
}

impl BullJoker {
    pub fn new() -> Self {
        Self {
            id: JokerId::BullMarket, // Using BullMarket as the ID since it exists in the enum
        }
    }
}

impl Joker for BullJoker {
    fn id(&self) -> JokerId {
        self.id
    }

    fn name(&self) -> &str {
        "Bull"
    }

    fn description(&self) -> &str {
        "+2 Chips per $1 owned"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Common
    }

    fn cost(&self) -> usize {
        3
    }

    fn on_hand_played(&self, context: &mut GameContext, _hand: &SelectHand) -> JokerEffect {
        let chips_bonus = 2 * context.money;
        JokerEffect::new().with_chips(chips_bonus)
    }
}

/// Stone joker: +25 chips per Stone card in deck
#[derive(Debug, Clone)]
pub struct StoneJoker {
    pub id: JokerId,
}

impl Default for StoneJoker {
    fn default() -> Self {
        Self::new()
    }
}

impl StoneJoker {
    pub fn new() -> Self {
        Self { id: JokerId::Stone }
    }
}

impl Joker for StoneJoker {
    fn id(&self) -> JokerId {
        self.id
    }

    fn name(&self) -> &str {
        "Stone Joker"
    }

    fn description(&self) -> &str {
        "+25 Chips per Stone card in deck"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Uncommon
    }

    fn cost(&self) -> usize {
        4
    }

    fn on_hand_played(&self, _context: &mut GameContext, _hand: &SelectHand) -> JokerEffect {
        // TODO: Access deck cards and count Stone cards
        // For now, return 0 chips until we have deck access
        // This will be improved when we have access to game deck state
        let _stone_card_count = 0; // Placeholder
        let chips_bonus = 25 * _stone_card_count;
        JokerEffect::new().with_chips(chips_bonus)
    }
}

/// Scary Face joker: +30 chips per face card scored  
#[derive(Debug, Clone)]
pub struct ScaryFaceJoker {
    pub id: JokerId,
}

impl Default for ScaryFaceJoker {
    fn default() -> Self {
        Self::new()
    }
}

impl ScaryFaceJoker {
    pub fn new() -> Self {
        Self {
            id: JokerId::ScaryFace,
        }
    }
}

impl Joker for ScaryFaceJoker {
    fn id(&self) -> JokerId {
        self.id
    }

    fn name(&self) -> &str {
        "Scary Face"
    }

    fn description(&self) -> &str {
        "+30 Chips when face cards are scored"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Common
    }

    fn cost(&self) -> usize {
        3
    }

    fn on_card_scored(&self, _context: &mut GameContext, card: &crate::card::Card) -> JokerEffect {
        match card.value {
            Value::Jack | Value::Queen | Value::King => JokerEffect::new().with_chips(30),
            _ => JokerEffect::new(),
        }
    }
}

/// Blue joker: +2 chips per remaining card in deck
#[derive(Debug, Clone)]
pub struct BlueJoker {
    pub id: JokerId,
}

impl Default for BlueJoker {
    fn default() -> Self {
        Self::new()
    }
}

impl BlueJoker {
    pub fn new() -> Self {
        Self {
            id: JokerId::BlueJoker,
        }
    }
}

impl Joker for BlueJoker {
    fn id(&self) -> JokerId {
        self.id
    }

    fn name(&self) -> &str {
        "Blue Joker"
    }

    fn description(&self) -> &str {
        "+2 Chips per remaining card in deck"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Uncommon
    }

    fn cost(&self) -> usize {
        4
    }

    fn on_hand_played(&self, _context: &mut GameContext, _hand: &SelectHand) -> JokerEffect {
        // TODO: Access deck and count remaining cards
        // For now, return 0 chips until we have deck access
        // This will be improved when we have access to game deck state
        let _cards_in_deck = 0; // Placeholder
        let chips_bonus = 2 * _cards_in_deck;
        JokerEffect::new().with_chips(chips_bonus)
    }
}
