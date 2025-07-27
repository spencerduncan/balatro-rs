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
pub struct FourFingersJoker {
    // Track if we've modified hand requirements this round
    pub hand_modified_this_round: bool,
}

impl FourFingersJoker {
    pub fn new() -> Self {
        Self {
            hand_modified_this_round: false,
        }
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
    fn on_round_start(&mut self) {
        // Reset state for new round
        self.hand_modified_this_round = false;

        // Reset hand evaluation config to default
        crate::hand::set_hand_eval_config(crate::hand::HandEvalConfig::default());
    }
}

impl JokerGameplay for FourFingersJoker {
    fn process(&mut self, stage: &Stage, _context: &mut ProcessContext) -> ProcessResult {
        // FourFingers modifies hand requirements during the PreBlind stage
        // when the hand type is being determined
        if matches!(stage, Stage::PreBlind()) && !self.hand_modified_this_round {
            self.hand_modified_this_round = true;

            // Set the hand evaluation config to allow 4-card flushes and straights
            crate::hand::set_hand_eval_config(crate::hand::HandEvalConfig {
                min_flush_cards: 4,
                min_straight_cards: 4,
            });
        }

        ProcessResult::default()
    }

    fn can_trigger(&self, stage: &Stage, _context: &ProcessContext) -> bool {
        // Can trigger during PreBlind to modify hand requirements
        matches!(stage, Stage::PreBlind()) && !self.hand_modified_this_round
    }
}

impl JokerModifiers for FourFingersJoker {
    fn get_hand_size_modifier(&self) -> i32 {
        // FourFingers doesn't change hand size - you can still play 5 cards
        // It changes the requirements for flushes/straights to only need 4 cards
        0
    }
}

impl JokerStateTrait for FourFingersJoker {
    fn has_state(&self) -> bool {
        true
    }

    fn serialize_state(&self) -> Option<serde_json::Value> {
        serde_json::to_value(self.hand_modified_this_round).ok()
    }

    fn deserialize_state(&mut self, value: serde_json::Value) -> Result<(), String> {
        self.hand_modified_this_round = serde_json::from_value(value)
            .map_err(|e| format!("Failed to deserialize FourFingers state: {e}"))?;
        Ok(())
    }

    fn debug_state(&self) -> String {
        format!(
            "hand_modified_this_round: {}",
            self.hand_modified_this_round
        )
    }

    fn reset_state(&mut self) {
        self.hand_modified_this_round = false;
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
        // Reset to default config first
        crate::hand::set_hand_eval_config(crate::hand::HandEvalConfig::default());

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

        // Activate FourFingers
        let mut joker = FourFingersJoker::new();
        let state_manager = crate::joker_state::JokerStateManager::new();
        let mut hand_score = crate::joker::traits::HandScore {
            chips: 100,
            mult: 5.0,
        };
        let played_cards = vec![];
        let held_cards = vec![];
        let mut events = vec![];
        let hand = SelectHand::new(played_cards.clone());
        let mut context = ProcessContext {
            hand_score: &mut hand_score,
            played_cards: &played_cards,
            held_cards: &held_cards,
            events: &mut events,
            hand: &hand,
            joker_state_manager: &state_manager,
        };

        // Process FourFingers in PreBlind stage
        joker.process(&Stage::PreBlind(), &mut context);

        // Now check again - it SHOULD be a flush!
        let hand2 = SelectHand::new(cards);
        let result_after = hand2.best_hand().unwrap();
        assert_eq!(
            result_after.rank,
            HandRank::Flush,
            "SHOULD be a flush with FourFingers active!"
        );
    }

    #[test]
    fn test_four_fingers_allows_four_card_straight() {
        // Reset to default config first
        crate::hand::set_hand_eval_config(crate::hand::HandEvalConfig::default());

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

        // Activate FourFingers
        crate::hand::set_hand_eval_config(crate::hand::HandEvalConfig {
            min_flush_cards: 4,
            min_straight_cards: 4,
        });

        // Now it IS a straight!
        let hand2 = SelectHand::new(cards);
        let result_after = hand2.best_hand().unwrap();
        assert_eq!(
            result_after.rank,
            HandRank::Straight,
            "SHOULD be a straight with FourFingers!"
        );
    }

    #[test]
    fn test_four_fingers_low_ace_straight() {
        // Test A-2-3-4 straight with FourFingers
        crate::hand::set_hand_eval_config(crate::hand::HandEvalConfig {
            min_flush_cards: 4,
            min_straight_cards: 4,
        });

        let cards = vec![
            Card::new(Value::Ace, Suit::Heart),
            Card::new(Value::Two, Suit::Diamond),
            Card::new(Value::Three, Suit::Club),
            Card::new(Value::Four, Suit::Spade),
            Card::new(Value::Jack, Suit::Heart), // Random high card
        ];

        let hand = SelectHand::new(cards);
        let result = hand.best_hand().unwrap();
        assert_eq!(
            result.rank,
            HandRank::Straight,
            "A-2-3-4 should be a straight with FourFingers!"
        );
    }

    #[test]
    fn test_four_fingers_straight_flush() {
        // Test that we can get a straight flush with 4 suited consecutive cards
        crate::hand::set_hand_eval_config(crate::hand::HandEvalConfig {
            min_flush_cards: 4,
            min_straight_cards: 4,
        });

        let cards = vec![
            Card::new(Value::Five, Suit::Heart),
            Card::new(Value::Six, Suit::Heart),
            Card::new(Value::Seven, Suit::Heart),
            Card::new(Value::Eight, Suit::Heart),
            Card::new(Value::King, Suit::Diamond), // Different suit
        ];

        let hand = SelectHand::new(cards);
        let result = hand.best_hand().unwrap();

        // With FourFingers, this should be a straight flush!
        // (4 hearts in sequence)
        assert_eq!(
            result.rank,
            HandRank::StraightFlush,
            "4 suited consecutive cards should be a straight flush!"
        );
    }

    #[test]
    fn test_four_fingers_resets_on_round_start() {
        let mut joker = FourFingersJoker::new();

        // Set config to FourFingers mode
        crate::hand::set_hand_eval_config(crate::hand::HandEvalConfig {
            min_flush_cards: 4,
            min_straight_cards: 4,
        });

        // Simulate joker was triggered
        joker.hand_modified_this_round = true;

        // Round start should reset both the flag AND the config
        joker.on_round_start();
        assert!(!joker.hand_modified_this_round);

        // Verify config was reset to default
        let config = crate::hand::get_hand_eval_config();
        assert_eq!(config.min_flush_cards, 5);
        assert_eq!(config.min_straight_cards, 5);
    }
}
