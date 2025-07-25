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
    }
}

impl JokerGameplay for FourFingersJoker {
    fn process(&mut self, stage: &Stage, _context: &mut ProcessContext) -> ProcessResult {
        // FourFingers modifies hand requirements during the PreBlind stage
        // when the hand type is being determined
        if matches!(stage, Stage::PreBlind()) && !self.hand_modified_this_round {
            self.hand_modified_this_round = true;

            // NOTE: The actual hand evaluation modification would need to be
            // implemented in the hand evaluation system. FourFingers allows:
            // - Flushes with only 4 cards of the same suit (instead of 5)
            // - Straights with only 4 consecutive ranks (instead of 5)
            // - Straight flushes if the hand contains both a 4-card flush AND a 4-card straight
            // This requires changes to the core hand evaluation logic.
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::joker::traits::HandScore;
    use crate::joker_state::JokerStateManager;

    #[test]
    fn test_four_fingers_identity() {
        let joker = FourFingersJoker::new();
        assert_eq!(joker.joker_type(), "four_fingers");
        assert_eq!(JokerIdentity::name(&joker), "Four Fingers");
        assert_eq!(
            JokerIdentity::description(&joker),
            "All Flushes and Straights can be made with 4 cards"
        );
        assert_eq!(JokerIdentity::rarity(&joker), Rarity::Uncommon);
        assert_eq!(joker.base_cost(), 7);
        assert!(!joker.is_unique());
    }

    #[test]
    fn test_four_fingers_hand_size_modifier() {
        let joker = FourFingersJoker::new();
        // FourFingers doesn't change hand size - you can still play 5 cards
        assert_eq!(joker.get_hand_size_modifier(), 0);
    }

    #[test]
    fn test_four_fingers_state_tracking() {
        let mut joker = FourFingersJoker::new();
        assert!(!joker.hand_modified_this_round);

        // Create minimal test context
        let state_manager = JokerStateManager::new();
        let mut hand_score = HandScore {
            chips: 100,
            mult: 5.0,
        };
        let played_cards = vec![];
        let held_cards = vec![];
        let mut events = vec![];

        let mut context = ProcessContext {
            hand_score: &mut hand_score,
            played_cards: &played_cards,
            held_cards: &held_cards,
            events: &mut events,
            joker_state_manager: &state_manager,
        };

        let pre_blind = Stage::PreBlind();
        let result = joker.process(&pre_blind, &mut context);
        assert_eq!(result.chips_added, 0);
        assert_eq!(result.mult_added, 0.0);
        assert!(joker.hand_modified_this_round);
    }

    #[test]
    fn test_four_fingers_only_triggers_once_per_round() {
        let mut joker = FourFingersJoker::new();

        // Create minimal test context
        let state_manager = JokerStateManager::new();
        let mut hand_score = HandScore {
            chips: 100,
            mult: 5.0,
        };
        let played_cards = vec![];
        let held_cards = vec![];
        let mut events = vec![];

        let context = ProcessContext {
            hand_score: &mut hand_score,
            played_cards: &played_cards,
            held_cards: &held_cards,
            events: &mut events,
            joker_state_manager: &state_manager,
        };

        let pre_blind = Stage::PreBlind();

        // First trigger
        assert!(joker.can_trigger(&pre_blind, &context));

        // Simulate processing
        joker.hand_modified_this_round = true;

        // Should not trigger again
        assert!(!joker.can_trigger(&pre_blind, &context));
    }

    #[test]
    fn test_four_fingers_resets_on_round_start() {
        let mut joker = FourFingersJoker::new();

        // Simulate that joker was triggered
        joker.hand_modified_this_round = true;

        // Reset for new round
        joker.on_round_start();
        assert!(!joker.hand_modified_this_round);

        // Create context to test can_trigger
        let state_manager = JokerStateManager::new();
        let mut hand_score = HandScore {
            chips: 100,
            mult: 5.0,
        };
        let played_cards = vec![];
        let held_cards = vec![];
        let mut events = vec![];

        let context = ProcessContext {
            hand_score: &mut hand_score,
            played_cards: &played_cards,
            held_cards: &held_cards,
            events: &mut events,
            joker_state_manager: &state_manager,
        };

        let pre_blind = Stage::PreBlind();
        assert!(joker.can_trigger(&pre_blind, &context));
    }

    #[test]
    fn test_four_fingers_only_triggers_in_preblind() {
        let joker = FourFingersJoker::new();
        let state_manager = JokerStateManager::new();
        let mut hand_score = HandScore {
            chips: 100,
            mult: 5.0,
        };
        let played_cards = vec![];
        let held_cards = vec![];
        let mut events = vec![];

        let context = ProcessContext {
            hand_score: &mut hand_score,
            played_cards: &played_cards,
            held_cards: &held_cards,
            events: &mut events,
            joker_state_manager: &state_manager,
        };

        // Should trigger in PreBlind
        assert!(joker.can_trigger(&Stage::PreBlind(), &context));

        // Should not trigger in other stages
        assert!(!joker.can_trigger(&Stage::Blind(crate::stage::Blind::Small), &context));
        assert!(!joker.can_trigger(&Stage::PostBlind(), &context));
        assert!(!joker.can_trigger(&Stage::Shop(), &context));
        assert!(!joker.can_trigger(&Stage::End(crate::stage::End::Win), &context));
    }

    #[test]
    fn test_four_fingers_serialization() {
        let mut joker = FourFingersJoker::new();
        joker.hand_modified_this_round = true;

        // Serialize
        let serialized = serde_json::to_value(&joker).unwrap();

        // Deserialize
        let deserialized: FourFingersJoker = serde_json::from_value(serialized).unwrap();
        assert_eq!(deserialized.hand_modified_this_round, true);
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
