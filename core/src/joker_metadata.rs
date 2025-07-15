use crate::joker::{JokerId, JokerRarity};
use crate::joker_registry::{JokerDefinition, UnlockCondition};
#[cfg(feature = "python")]
use pyo3::{pyclass, pymethods};
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
        // Extract cost based on rarity (matching existing logic)
        let cost = match definition.rarity {
            JokerRarity::Common => 3,
            JokerRarity::Uncommon => 6,
            JokerRarity::Rare => 8,
            JokerRarity::Legendary => 20,
        };

        // Calculate sell value (half of cost)
        let sell_value = cost / 2;

        // Determine effect type based on description and ID
        let effect_type = determine_effect_type(&definition.id, &definition.description);

        // Determine triggers and conditions based on joker type
        let (triggers_on, conditions) = determine_triggers_and_conditions(&definition.id);

        Self {
            id: definition.id,
            name: definition.name.clone(),
            description: definition.description.clone(),
            rarity: definition.rarity,
            cost,
            sell_value,
            effect_type: effect_type.to_string(),
            effect_description: definition.description.clone(),
            scaling_info: None, // TODO: Extract from joker implementations
            triggers_on,
            conditions,
            uses_state: false,      // TODO: Determine from joker implementation
            max_triggers: None,     // TODO: Extract from joker implementation
            persistent_data: false, // TODO: Determine from joker implementation
            unlock_condition: definition.unlock_condition.clone(),
            is_unlocked,
        }
    }
}

/// Determine the effect type based on joker ID and description
fn determine_effect_type(id: &JokerId, description: &str) -> &'static str {
    // Simple categorization based on description keywords
    if description.contains("Mult") && !description.contains("Chip") {
        "Mult Bonus"
    } else if description.contains("Chip") && !description.contains("Mult") {
        "Chip Bonus"
    } else if description.contains("Mult") && description.contains("Chip") {
        "Combined Bonus"
    } else if description.contains("Money") || description.contains("$") {
        "Economy"
    } else if description.contains("retrigger") || description.contains("again") {
        "Retrigger"
    } else {
        match id {
            JokerId::Joker => "Basic Mult",
            JokerId::GreedyJoker
            | JokerId::LustyJoker
            | JokerId::WrathfulJoker
            | JokerId::GluttonousJoker => "Conditional Mult",
            _ => "Effect",
        }
    }
}

/// Determine triggers and conditions based on joker ID
fn determine_triggers_and_conditions(id: &JokerId) -> (Vec<String>, Vec<String>) {
    match id {
        JokerId::Joker => (vec!["hand_played".to_string()], vec![]),
        JokerId::GreedyJoker => (
            vec!["card_scored".to_string()],
            vec!["suit:Diamond".to_string()],
        ),
        JokerId::LustyJoker => (
            vec!["card_scored".to_string()],
            vec!["suit:Heart".to_string()],
        ),
        JokerId::WrathfulJoker => (
            vec!["card_scored".to_string()],
            vec!["suit:Spade".to_string()],
        ),
        JokerId::GluttonousJoker => (
            vec!["card_scored".to_string()],
            vec!["suit:Club".to_string()],
        ),
        _ => (vec![], vec![]),
    }
}

#[cfg_attr(feature = "python", pymethods)]
impl JokerMetadata {
    /// Get the joker ID
    #[cfg(feature = "python")]
    #[getter]
    fn id(&self) -> JokerId {
        self.id
    }

    /// Get the joker name
    #[cfg(feature = "python")]
    #[getter]
    fn name(&self) -> String {
        self.name.clone()
    }

    /// Get the joker description
    #[cfg(feature = "python")]
    #[getter]
    fn description(&self) -> String {
        self.description.clone()
    }

    /// Get the joker rarity
    #[cfg(feature = "python")]
    #[getter]
    fn rarity(&self) -> JokerRarity {
        self.rarity
    }

    /// Get the joker cost
    #[cfg(feature = "python")]
    #[getter]
    fn cost(&self) -> i32 {
        self.cost
    }

    /// Get the joker sell value
    #[cfg(feature = "python")]
    #[getter]
    fn sell_value(&self) -> i32 {
        self.sell_value
    }

    /// Get the effect type
    #[cfg(feature = "python")]
    #[getter]
    fn effect_type(&self) -> String {
        self.effect_type.clone()
    }

    /// Get the effect description
    #[cfg(feature = "python")]
    #[getter]
    fn effect_description(&self) -> String {
        self.effect_description.clone()
    }

    /// Get triggers
    #[cfg(feature = "python")]
    #[getter]
    fn triggers_on(&self) -> Vec<String> {
        self.triggers_on.clone()
    }

    /// Get conditions
    #[cfg(feature = "python")]
    #[getter]
    fn conditions(&self) -> Vec<String> {
        self.conditions.clone()
    }

    /// Get whether this joker uses state
    #[cfg(feature = "python")]
    #[getter]
    fn uses_state(&self) -> bool {
        self.uses_state
    }

    /// Get unlock condition
    #[cfg(feature = "python")]
    #[getter]
    fn unlock_condition(&self) -> Option<UnlockCondition> {
        self.unlock_condition.clone()
    }

    /// Get whether this joker is unlocked
    #[cfg(feature = "python")]
    #[getter]
    fn is_unlocked(&self) -> bool {
        self.is_unlocked
    }
}
