//! FourFingers Joker implementation
//!
//! This joker allows Flushes and Straights to be made with 4 cards instead of 5.

use super::traits::{
    JokerGameplay, JokerIdentity, JokerLifecycle, JokerModifiers, JokerState as JokerStateTrait,
    ProcessContext, ProcessResult, Rarity,
};
use super::{Joker, JokerId, JokerRarity};
use crate::stage::Stage;
use serde::{Deserialize, Serialize};

/// FourFingers Joker: All Flushes and Straights can be made with 4 cards
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FourFingersJoker;

impl FourFingersJoker {
    pub fn new() -> Self {
        Self
    }
}

impl JokerIdentity for FourFingersJoker {
    fn joker_type(&self) -> &'static str {
        "four_fingers"
    }

    fn name(&self) -> &str {
        "Four Fingers"
    }

    fn description(&self) -> &str {
        "All Flushes and Straights can be made with 4 cards"
    }

    fn rarity(&self) -> Rarity {
        Rarity::Uncommon
    }

    fn base_cost(&self) -> u64 {
        7
    }
}

impl JokerLifecycle for FourFingersJoker {
    // FourFingers doesn't need lifecycle management - it provides config through traits
}

impl JokerGameplay for FourFingersJoker {
    fn process(&mut self, _stage: &Stage, _context: &mut ProcessContext) -> ProcessResult {
        // FourFingers doesn't need processing - it provides config through traits
        ProcessResult::default()
    }

    fn can_trigger(&self, _stage: &Stage, _context: &ProcessContext) -> bool {
        // FourFingers doesn't need triggering - it provides config through traits
        false
    }
}

impl JokerModifiers for FourFingersJoker {
    fn get_hand_size_modifier(&self) -> i32 {
        // FourFingers doesn't change hand size - you can still play 5 cards
        // It changes the requirements for flushes/straights to only need 4 cards
        0
    }

    fn get_hand_eval_config(&self) -> Option<crate::hand::HandEvalConfig> {
        // FourFingers allows flushes and straights to be made with 4 cards
        Some(crate::hand::HandEvalConfig {
            min_flush_cards: 4,
            min_straight_cards: 4,
        })
    }
}

impl JokerStateTrait for FourFingersJoker {
    fn has_state(&self) -> bool {
        false
    }

    fn serialize_state(&self) -> Option<serde_json::Value> {
        None
    }

    fn deserialize_state(&mut self, _value: serde_json::Value) -> Result<(), String> {
        Ok(())
    }

    fn debug_state(&self) -> String {
        "{}".to_string()
    }

    fn reset_state(&mut self) {
        // No state to reset
    }
}

// Legacy Joker trait implementation for backward compatibility
impl Joker for FourFingersJoker {
    fn id(&self) -> JokerId {
        JokerId::FourFingers
    }

    fn name(&self) -> &str {
        JokerIdentity::name(self)
    }

    fn description(&self) -> &str {
        JokerIdentity::description(self)
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Uncommon
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::{Card, Suit, Value};
    use crate::hand::SelectHand;
    use crate::rank::HandRank;

    #[test]
    fn test_four_fingers_basic_properties() {
        let joker = FourFingersJoker::new();
        assert_eq!(joker.joker_type(), "four_fingers");
        assert_eq!(JokerIdentity::name(&joker), "Four Fingers");
        assert_eq!(JokerIdentity::rarity(&joker), Rarity::Uncommon);
        assert_eq!(joker.get_hand_size_modifier(), 0);
    }

    #[test]
    fn test_four_fingers_flush_only() {
        // Create a hand with 4 hearts that don't form a straight
        let cards = vec![
            Card::new(Value::Ace, Suit::Heart),
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::Three, Suit::Heart), // Not consecutive
            Card::new(Value::Two, Suit::Heart),   // Not consecutive
            Card::new(Value::Jack, Suit::Diamond), // Not a heart
        ];

        let hand = SelectHand::new(cards.clone());

        // Without FourFingers: NOT a flush (only 4 hearts)
        let result_before = hand.best_hand().unwrap();
        assert_ne!(
            result_before.rank,
            HandRank::Flush,
            "Should NOT be a flush with normal rules"
        );

        // With FourFingers config: SHOULD be a flush!
        let joker = FourFingersJoker::new();
        let four_fingers_config = joker.get_hand_eval_config().unwrap();
        let hand2 = SelectHand::new(cards);
        let result_after = hand2.best_hand_with_config(&four_fingers_config).unwrap();
        assert_eq!(
            result_after.rank,
            HandRank::Flush,
            "SHOULD be a flush with FourFingers config!"
        );
    }

    #[test]
    fn test_four_fingers_allows_four_card_straight() {
        // Create a hand with 4 consecutive cards and 1 random card
        let cards = vec![
            Card::new(Value::Five, Suit::Heart),
            Card::new(Value::Six, Suit::Diamond),
            Card::new(Value::Seven, Suit::Club),
            Card::new(Value::Eight, Suit::Spade),
            Card::new(Value::King, Suit::Heart), // Not consecutive
        ];

        let hand = SelectHand::new(cards.clone());

        // Without FourFingers: NOT a straight
        let result_before = hand.best_hand().unwrap();
        assert_ne!(
            result_before.rank,
            HandRank::Straight,
            "Should NOT be a straight with normal rules"
        );

        // With FourFingers config: SHOULD be a straight!
        let joker = FourFingersJoker::new();
        let four_fingers_config = joker.get_hand_eval_config().unwrap();
        let hand2 = SelectHand::new(cards);
        let result_after = hand2.best_hand_with_config(&four_fingers_config).unwrap();
        assert_eq!(
            result_after.rank,
            HandRank::Straight,
            "SHOULD be a straight with FourFingers!"
        );
    }

    #[test]
    fn test_four_fingers_low_ace_straight() {
        // Test A-2-3-4 straight with FourFingers config
        let joker = FourFingersJoker::new();
        let four_fingers_config = joker.get_hand_eval_config().unwrap();

        let cards = vec![
            Card::new(Value::Ace, Suit::Heart),
            Card::new(Value::Two, Suit::Diamond),
            Card::new(Value::Three, Suit::Club),
            Card::new(Value::Four, Suit::Spade),
            Card::new(Value::Jack, Suit::Heart), // Random high card
        ];

        let hand = SelectHand::new(cards);
        let result = hand.best_hand_with_config(&four_fingers_config).unwrap();
        assert_eq!(
            result.rank,
            HandRank::Straight,
            "A-2-3-4 should be a straight with FourFingers!"
        );
    }

    #[test]
    fn test_four_fingers_straight_flush() {
        // Test that we can get a straight flush with 4 suited consecutive cards
        let joker = FourFingersJoker::new();
        let four_fingers_config = joker.get_hand_eval_config().unwrap();

        let cards = vec![
            Card::new(Value::Five, Suit::Heart),
            Card::new(Value::Six, Suit::Heart),
            Card::new(Value::Seven, Suit::Heart),
            Card::new(Value::Eight, Suit::Heart),
            Card::new(Value::King, Suit::Diamond), // Different suit
        ];

        let hand = SelectHand::new(cards);
        let result = hand.best_hand_with_config(&four_fingers_config).unwrap();

        // With FourFingers, this should be a straight flush!
        // (4 hearts in sequence)
        assert_eq!(
            result.rank,
            HandRank::StraightFlush,
            "4 suited consecutive cards should be a straight flush!"
        );
    }

    #[test]
    fn test_four_fingers_provides_consistent_config() {
        // Test that FourFingers provides consistent configuration
        let joker = FourFingersJoker::new();

        // Should always provide the same configuration
        let config1 = joker.get_hand_eval_config().unwrap();
        let config2 = joker.get_hand_eval_config().unwrap();

        assert_eq!(config1.min_flush_cards, 4);
        assert_eq!(config1.min_straight_cards, 4);
        assert_eq!(config2.min_flush_cards, 4);
        assert_eq!(config2.min_straight_cards, 4);

        // FourFingers joker is stateless and doesn't need state management
        assert!(!joker.has_state());
    }
}
