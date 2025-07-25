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

    #[test]
    fn test_four_fingers_basic_properties() {
        let joker = FourFingersJoker::new();
        assert_eq!(joker.joker_type(), "four_fingers");
        assert_eq!(JokerIdentity::name(&joker), "Four Fingers");
        assert_eq!(JokerIdentity::rarity(&joker), Rarity::Uncommon);
        assert_eq!(joker.get_hand_size_modifier(), 0);
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
