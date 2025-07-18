use crate::joker::{JokerId, JokerRarity};
use crate::joker_registry::{calculate_joker_cost, JokerDefinition, UnlockCondition};
#[cfg(feature = "python")]
use pyo3::pyclass;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Comprehensive metadata for a joker including all properties and state information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "python", pyclass)]
pub struct JokerMetadata {
    /// Core joker properties
    pub id: JokerId,
    pub name: String,
    pub description: String,
    pub rarity: JokerRarity,
    pub cost: i32,
    pub sell_value: i32,

    /// Effect information
    pub effect_type: String,
    pub effect_description: String,
    pub scaling_info: Option<HashMap<String, String>>,

    /// Conditional information
    pub triggers_on: Vec<String>,
    pub conditions: Vec<String>,

    /// State information
    pub uses_state: bool,
    pub max_triggers: Option<i32>,
    pub persistent_data: bool,

    /// Unlock information
    pub unlock_condition: Option<UnlockCondition>,
    pub is_unlocked: bool,
}

impl JokerMetadata {
    /// Create a new JokerMetadata from a JokerDefinition
    pub fn from_definition(definition: &JokerDefinition, is_unlocked: bool) -> Self {
        // Extract cost based on rarity (using centralized function)
        let cost = calculate_joker_cost(definition.rarity);

        // Calculate sell value (half of cost)
        let sell_value = cost / 2;

        // Determine effect type based on description and ID
        let effect_type = determine_effect_type(&definition.id, &definition.description);

        // Extract trigger information
        let triggers_on = extract_triggers(&definition.id, &definition.description);

        // Extract condition information
        let conditions = extract_conditions(&definition.id, &definition.description);

        // Determine if joker uses state
        let uses_state = check_uses_state(&definition.id);

        // Extract max triggers if applicable
        let max_triggers = extract_max_triggers(&definition.id);

        // Check if joker uses persistent data
        let persistent_data = check_persistent_data(&definition.id);

        // Create effect description
        let effect_description = create_effect_description(&definition.id, &definition.description);

        Self {
            id: definition.id,
            name: definition.name.clone(),
            description: definition.description.clone(),
            rarity: definition.rarity,
            cost,
            sell_value,
            effect_type,
            effect_description,
            scaling_info: None, // Will be populated in future PRs
            triggers_on,
            conditions,
            uses_state,
            max_triggers,
            persistent_data,
            unlock_condition: definition.unlock_condition.clone(),
            is_unlocked,
        }
    }
}

/// Determine the effect type of a joker based on its ID and description
fn determine_effect_type(id: &JokerId, description: &str) -> String {
    // Basic pattern matching for effect types
    if description.contains("Mult") {
        if description.contains("×") || description.contains("X") {
            "multiplicative_mult".to_string()
        } else {
            "additive_mult".to_string()
        }
    } else if description.contains("Chips") {
        "additive_chips".to_string()
    } else if description.contains("$") || description.contains("money") {
        "economy".to_string()
    } else if description.contains("hand size") {
        "hand_modification".to_string()
    } else if description.contains("discard") {
        "discard_modification".to_string()
    } else {
        // Default based on specific joker IDs
        match id {
            JokerId::IceCream => "conditional_chips".to_string(),
            _ => "special".to_string(),
        }
    }
}

/// Extract trigger conditions from joker description
fn extract_triggers(id: &JokerId, description: &str) -> Vec<String> {
    let mut triggers = Vec::new();

    // Check for common trigger patterns
    if description.contains("when scored") {
        triggers.push("card_scored".to_string());
    }
    if description.contains("played") {
        triggers.push("hand_played".to_string());
    }
    if description.contains("discarded") {
        triggers.push("card_discarded".to_string());
    }
    if description.contains("round") {
        triggers.push("round_end".to_string());
    }
    if description.contains("bought") || description.contains("purchased") {
        triggers.push("item_purchased".to_string());
    }

    // Special cases for specific jokers
    match id {
        JokerId::IceCream => triggers.push("hand_played".to_string()),
        JokerId::EggJoker => triggers.push("round_end".to_string()),
        _ => {}
    }

    if triggers.is_empty() {
        triggers.push("passive".to_string());
    }

    triggers
}

/// Extract condition information from joker
fn extract_conditions(_id: &JokerId, description: &str) -> Vec<String> {
    let mut conditions = Vec::new();

    // Check for suit conditions
    if description.contains("Heart") {
        conditions.push("suit:hearts".to_string());
    }
    if description.contains("Diamond") {
        conditions.push("suit:diamonds".to_string());
    }
    if description.contains("Club") {
        conditions.push("suit:clubs".to_string());
    }
    if description.contains("Spade") {
        conditions.push("suit:spades".to_string());
    }

    // Check for hand type conditions
    let hand_types = ["Flush", "Straight", "Full House", "Four of a Kind", "Pair"];
    for hand_type in &hand_types {
        if description.contains(hand_type) {
            conditions.push(format!(
                "hand_type:{}",
                hand_type.to_lowercase().replace(' ', "_")
            ));
        }
    }

    // Check for other conditions
    if description.contains("face card") {
        conditions.push("face_card".to_string());
    }
    if description.contains("contains no") {
        conditions.push("exclusion".to_string());
    }

    if conditions.is_empty() {
        conditions.push("always".to_string());
    }

    conditions
}

/// Check if a joker uses state tracking
fn check_uses_state(id: &JokerId) -> bool {
    matches!(
        id,
        JokerId::IceCream
            | JokerId::EggJoker
            | JokerId::BurglarJoker
            | JokerId::Cartomancer
            | JokerId::SpaceJoker
    )
}

/// Extract maximum trigger count for limited-use jokers
fn extract_max_triggers(_id: &JokerId) -> Option<i32> {
    // Most jokers have unlimited triggers
    // Future jokers with limited uses would be added here
    None
}

/// Check if a joker uses persistent data between rounds
fn check_persistent_data(id: &JokerId) -> bool {
    matches!(id, JokerId::IceCream | JokerId::EggJoker)
}

/// Create a detailed effect description
fn create_effect_description(_id: &JokerId, base_description: &str) -> String {
    // For now, use the base description
    // In future PRs, this will be enhanced with more detailed effect information
    base_description.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_joker_metadata_creation() {
        let definition = JokerDefinition {
            id: JokerId::Joker,
            name: "Joker".to_string(),
            description: "+4 Mult".to_string(),
            rarity: JokerRarity::Common,
            unlock_condition: None,
        };

        let metadata = JokerMetadata::from_definition(&definition, true);

        assert_eq!(metadata.id, JokerId::Joker);
        assert_eq!(metadata.name, "Joker");
        assert_eq!(metadata.cost, 3); // Common rarity
        assert_eq!(metadata.sell_value, 1); // Half of cost
        assert_eq!(metadata.effect_type, "additive_mult");
        assert!(metadata.is_unlocked);
        assert!(!metadata.uses_state);
    }

    #[test]
    fn test_effect_type_detection() {
        assert_eq!(
            determine_effect_type(&JokerId::Joker, "+4 Mult"),
            "additive_mult"
        );
        assert_eq!(
            determine_effect_type(&JokerId::Joker, "X2 Mult"),
            "multiplicative_mult"
        );
        assert_eq!(
            determine_effect_type(&JokerId::Joker, "+100 Chips"),
            "additive_chips"
        );
        assert_eq!(determine_effect_type(&JokerId::Joker, "Earn $3"), "economy");
    }

    #[test]
    fn test_trigger_extraction() {
        let triggers = extract_triggers(
            &JokerId::GreedyJoker,
            "Played cards with Diamond suit give +3 Mult when scored",
        );
        assert!(triggers.contains(&"card_scored".to_string()));

        // Test a different joker that triggers on hand played
        let triggers2 = extract_triggers(&JokerId::IceCream, "Ice Cream gives chips");
        assert!(triggers2.contains(&"hand_played".to_string()));
    }

    #[test]
    fn test_condition_extraction() {
        let conditions = extract_conditions(
            &JokerId::GreedyJoker,
            "Played cards with Diamond suit give +3 Mult when scored",
        );
        assert!(conditions.contains(&"suit:diamonds".to_string()));
    }
}
