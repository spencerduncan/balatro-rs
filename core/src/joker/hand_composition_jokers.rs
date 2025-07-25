use crate::{
    card::{Card, Suit, Value},
    hand::SelectHand,
    joker::{
        ConditionalJoker, GameContext, Joker, JokerCondition, JokerEffect, JokerId, JokerRarity,
    },
    joker_state::JokerState,
};

/// Factory function for Ride the Bus joker
/// "+1 mult per hand without face card (resets when face card scored)"
pub fn create_ride_the_bus() -> Box<dyn Joker> {
    create_ride_the_bus_stateful()
}

/// Factory function for Blackboard joker
/// "X3 mult if all held cards are Spades or Clubs"
pub fn create_blackboard() -> ConditionalJoker {
    ConditionalJoker::new(
        JokerId::Blackboard,
        "Blackboard",
        "X3 mult if all held cards are Spades or Clubs",
        JokerRarity::Uncommon,
        JokerCondition::AllCardsInSuits(vec![Suit::Spade, Suit::Club]),
        JokerEffect::new().with_mult_multiplier(3.0),
    )
}

/// Custom DNA joker implementation that handles card duplication
#[derive(Debug, Clone)]
pub struct DnaJoker {
    pub id: JokerId,
    pub name: String,
    pub description: String,
    pub rarity: JokerRarity,
    pub cost: usize,
}

impl Default for DnaJoker {
    fn default() -> Self {
        Self::new()
    }
}

impl DnaJoker {
    pub fn new() -> Self {
        Self {
            id: JokerId::DNA,
            name: "DNA".to_string(),
            description: "Copy first card if only 1 in hand".to_string(),
            rarity: JokerRarity::Rare,
            cost: 8, // Rare rarity default cost
        }
    }
}

impl Joker for DnaJoker {
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
        // Check if this is the first hand of the round AND hand has exactly 1 card
        if context.hands_played == 0 && hand.len() == 1 {
            let cards = hand.cards();
            let first_card = cards[0];

            // Create a copy of the first card (new ID will be generated automatically)
            let copied_card = Card::new(first_card.value, first_card.suit);

            // Create effect with card duplication
            let mut effect = JokerEffect::new();
            effect.transform_cards = vec![(first_card, copied_card)];
            effect.message = Some("DNA: Card duplicated!".to_string());
            effect
        } else {
            JokerEffect::new()
        }
    }

    fn on_card_scored(&self, _context: &mut GameContext, _card: &Card) -> JokerEffect {
        JokerEffect::new()
    }

    fn on_blind_start(&self, _context: &mut GameContext) -> JokerEffect {
        JokerEffect::new()
    }

    fn on_shop_open(&self, _context: &mut GameContext) -> JokerEffect {
        JokerEffect::new()
    }

    fn on_discard(&self, _context: &mut GameContext, _cards: &[Card]) -> JokerEffect {
        JokerEffect::new()
    }

    fn on_round_end(&self, _context: &mut GameContext) -> JokerEffect {
        JokerEffect::new()
    }
}

/// Factory function for DNA joker
/// "Copy first card if only 1 in hand"
pub fn create_dna() -> Box<dyn Joker> {
    Box::new(DnaJoker::new())
}

/// Stateful Ride the Bus joker implementation
#[derive(Debug, Clone)]
pub struct RideTheBusStateful {
    pub id: JokerId,
    pub name: String,
    pub description: String,
    pub rarity: JokerRarity,
    pub cost: usize,
}

impl Default for RideTheBusStateful {
    fn default() -> Self {
        Self::new()
    }
}

impl RideTheBusStateful {
    pub fn new() -> Self {
        Self {
            id: JokerId::Ride,
            name: "Ride the Bus".to_string(),
            description: "+1 mult per hand without face card (resets when face card scored)"
                .to_string(),
            rarity: JokerRarity::Common,
            cost: 3,
        }
    }

    /// Check if a card is a face card (Jack, Queen, or King)
    fn is_face_card(card: &Card) -> bool {
        matches!(card.value, Value::Jack | Value::Queen | Value::King)
    }

    /// Check if a hand contains any face cards
    fn hand_has_face_cards(hand: &SelectHand) -> bool {
        hand.cards().iter().any(Self::is_face_card)
    }
}

impl Joker for RideTheBusStateful {
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
        // Increment accumulated value only if hand has no face cards
        if !Self::hand_has_face_cards(hand) {
            context
                .joker_state_manager
                .update_state(self.id(), |state| {
                    state.accumulated_value += 1.0;
                });
        }

        // Always return current accumulated value as mult bonus
        let current_mult = context
            .joker_state_manager
            .get_accumulated_value(self.id())
            .unwrap_or(0.0) as i32;

        JokerEffect::new().with_mult(current_mult)
    }

    fn on_card_scored(&self, context: &mut GameContext, card: &Card) -> JokerEffect {
        // Reset accumulated value if a face card is scored
        if Self::is_face_card(card) {
            context
                .joker_state_manager
                .update_state(self.id(), |state| {
                    state.accumulated_value = 0.0;
                });
        }
        JokerEffect::new()
    }

    fn on_blind_start(&self, _context: &mut GameContext) -> JokerEffect {
        JokerEffect::new()
    }

    fn on_shop_open(&self, _context: &mut GameContext) -> JokerEffect {
        JokerEffect::new()
    }

    fn on_discard(&self, _context: &mut GameContext, _cards: &[Card]) -> JokerEffect {
        JokerEffect::new()
    }

    fn on_round_end(&self, _context: &mut GameContext) -> JokerEffect {
        JokerEffect::new()
    }

    fn initialize_state(&self, _context: &GameContext) -> JokerState {
        JokerState::with_accumulated_value(0.0)
    }
}

/// Factory function for stateful Ride the Bus joker
pub fn create_ride_the_bus_stateful() -> Box<dyn Joker> {
    Box::new(RideTheBusStateful::new())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::joker::Joker;

    #[test]
    fn test_create_ride_the_bus() {
        let joker = create_ride_the_bus();

        assert_eq!(joker.id(), JokerId::Ride);
        assert_eq!(joker.name(), "Ride the Bus");
        assert_eq!(joker.rarity(), JokerRarity::Common);
        assert_eq!(joker.cost(), 3); // Common rarity default cost
    }

    #[test]
    fn test_create_blackboard() {
        let joker = create_blackboard();

        assert_eq!(joker.id(), JokerId::Blackboard);
        assert_eq!(joker.name(), "Blackboard");
        assert_eq!(joker.rarity(), JokerRarity::Uncommon);
        assert_eq!(joker.cost(), 6); // Uncommon rarity default cost
    }

    #[test]
    fn test_create_dna() {
        let joker = create_dna();

        assert_eq!(joker.id(), JokerId::DNA);
        assert_eq!(joker.name(), "DNA");
        assert_eq!(joker.rarity(), JokerRarity::Rare);
        assert_eq!(joker.cost(), 8); // Rare rarity default cost
    }

    #[test]
    fn test_dna_joker_direct() {
        let joker = DnaJoker::new();

        assert_eq!(joker.id(), JokerId::DNA);
        assert_eq!(joker.name(), "DNA");
        assert_eq!(joker.rarity(), JokerRarity::Rare);
        assert_eq!(joker.cost(), 8); // Rare rarity default cost
    }
}
