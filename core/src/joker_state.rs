use crate::joker::JokerId;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::RwLock;

/// Per-joker state that persists across rounds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JokerState {
    /// Accumulated value that can grow over time (e.g., for scaling jokers)
    pub accumulated_value: f64,
    /// Number of triggers remaining (None = unlimited)
    pub triggers_remaining: Option<u32>,
    /// Custom data storage for joker-specific state
    pub custom_data: HashMap<String, Value>,
}

impl JokerState {
    /// Create a new default state
    pub fn new() -> Self {
        Self {
            accumulated_value: 0.0,
            triggers_remaining: None,
            custom_data: HashMap::new(),
        }
    }

    /// Create state with an initial accumulated value
    pub fn with_accumulated_value(accumulated_value: f64) -> Self {
        Self {
            accumulated_value,
            triggers_remaining: None,
            custom_data: HashMap::new(),
        }
    }

    /// Create state with limited triggers
    pub fn with_triggers(triggers: u32) -> Self {
        Self {
            accumulated_value: 0.0,
            triggers_remaining: Some(triggers),
            custom_data: HashMap::new(),
        }
    }

    /// Add to the accumulated value
    pub fn add_value(&mut self, value: f64) {
        self.accumulated_value += value;
    }

    /// Use one trigger (returns true if triggers remain)
    pub fn use_trigger(&mut self) -> bool {
        match &mut self.triggers_remaining {
            Some(count) => {
                if *count > 0 {
                    *count -= 1;
                    true
                } else {
                    false
                }
            }
            None => true, // Unlimited triggers
        }
    }

    /// Check if triggers are available
    pub fn has_triggers(&self) -> bool {
        match self.triggers_remaining {
            Some(count) => count > 0,
            None => true,
        }
    }

    /// Set custom data value
    pub fn set_custom<T: Serialize>(
        &mut self,
        key: &str,
        value: T,
    ) -> Result<(), serde_json::Error> {
        let json_value = serde_json::to_value(value)?;
        self.custom_data.insert(key.to_string(), json_value);
        Ok(())
    }

    /// Get custom data value
    pub fn get_custom<T: for<'de> Deserialize<'de>>(
        &self,
        key: &str,
    ) -> Result<Option<T>, serde_json::Error> {
        match self.custom_data.get(key) {
            Some(value) => Ok(Some(serde_json::from_value(value.clone())?)),
            None => Ok(None),
        }
    }
}

impl Default for JokerState {
    fn default() -> Self {
        Self::new()
    }
}

/// Centralized state manager for tracking per-joker state across rounds
#[derive(Debug)]
pub struct JokerStateManager {
    /// Thread-safe storage for joker states
    states: RwLock<HashMap<JokerId, JokerState>>,
}

impl JokerStateManager {
    /// Create a new state manager
    pub fn new() -> Self {
        Self {
            states: RwLock::new(HashMap::new()),
        }
    }

    /// Get a copy of a joker's state (creates default if not exists)
    pub fn get_state(&self, joker_id: JokerId) -> JokerState {
        let states = self.states.read().unwrap();
        states.get(&joker_id).cloned().unwrap_or_default()
    }

    /// Set a joker's state
    pub fn set_state(&self, joker_id: JokerId, state: JokerState) {
        let mut states = self.states.write().unwrap();
        states.insert(joker_id, state);
    }

    /// Update a joker's state with a closure
    pub fn update_state<F>(&self, joker_id: JokerId, update_fn: F)
    where
        F: FnOnce(&mut JokerState),
    {
        let mut states = self.states.write().unwrap();
        let state = states.entry(joker_id).or_default();
        update_fn(state);
    }

    /// Remove a joker's state
    pub fn remove_state(&self, joker_id: JokerId) -> Option<JokerState> {
        let mut states = self.states.write().unwrap();
        states.remove(&joker_id)
    }

    /// Check if a joker has state
    pub fn has_state(&self, joker_id: JokerId) -> bool {
        let states = self.states.read().unwrap();
        states.contains_key(&joker_id)
    }

    /// Clear all states
    pub fn clear(&self) {
        let mut states = self.states.write().unwrap();
        states.clear();
    }

    /// Get all joker IDs that have state
    pub fn get_active_jokers(&self) -> Vec<JokerId> {
        let states = self.states.read().unwrap();
        states.keys().copied().collect()
    }

    /// Get the total number of jokers with state
    pub fn count(&self) -> usize {
        let states = self.states.read().unwrap();
        states.len()
    }

    /// Add value to a joker's accumulated value
    pub fn add_accumulated_value(&self, joker_id: JokerId, value: f64) {
        self.update_state(joker_id, |state| {
            state.add_value(value);
        });
    }

    /// Use a trigger for a joker (returns true if triggers remain)
    pub fn use_trigger(&self, joker_id: JokerId) -> bool {
        let mut result = true;
        self.update_state(joker_id, |state| {
            result = state.use_trigger();
        });
        result
    }

    /// Check if a joker has triggers available
    pub fn has_triggers(&self, joker_id: JokerId) -> bool {
        let state = self.get_state(joker_id);
        state.has_triggers()
    }

    /// Set custom data for a joker
    pub fn set_custom_data<T: Serialize>(
        &self,
        joker_id: JokerId,
        key: &str,
        value: T,
    ) -> Result<(), serde_json::Error> {
        let json_value = serde_json::to_value(value)?;
        self.update_state(joker_id, |state| {
            state.custom_data.insert(key.to_string(), json_value);
        });
        Ok(())
    }

    /// Get custom data for a joker
    pub fn get_custom_data<T: for<'de> Deserialize<'de>>(
        &self,
        joker_id: JokerId,
        key: &str,
    ) -> Result<Option<T>, serde_json::Error> {
        let state = self.get_state(joker_id);
        state.get_custom(key)
    }
}

impl Default for JokerStateManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_joker_state_creation() {
        let state = JokerState::new();
        assert_eq!(state.accumulated_value, 0.0);
        assert_eq!(state.triggers_remaining, None);
        assert!(state.custom_data.is_empty());
    }

    #[test]
    fn test_joker_state_with_value() {
        let state = JokerState::with_accumulated_value(10.5);
        assert_eq!(state.accumulated_value, 10.5);
    }

    #[test]
    fn test_joker_state_with_triggers() {
        let mut state = JokerState::with_triggers(3);
        assert_eq!(state.triggers_remaining, Some(3));
        assert!(state.has_triggers());

        // Use triggers
        assert!(state.use_trigger());
        assert_eq!(state.triggers_remaining, Some(2));
        assert!(state.use_trigger());
        assert!(state.use_trigger());
        assert_eq!(state.triggers_remaining, Some(0));
        assert!(!state.has_triggers());
        assert!(!state.use_trigger());
    }

    #[test]
    fn test_accumulated_value() {
        let mut state = JokerState::new();
        state.add_value(5.0);
        assert_eq!(state.accumulated_value, 5.0);
        state.add_value(2.5);
        assert_eq!(state.accumulated_value, 7.5);
    }

    #[test]
    fn test_custom_data() {
        let mut state = JokerState::new();

        // Set and get string
        state.set_custom("name", "test").unwrap();
        let name: Option<String> = state.get_custom("name").unwrap();
        assert_eq!(name, Some("test".to_string()));

        // Set and get number
        state.set_custom("count", 42).unwrap();
        let count: Option<i32> = state.get_custom("count").unwrap();
        assert_eq!(count, Some(42));

        // Non-existent key
        let missing: Option<String> = state.get_custom("missing").unwrap();
        assert_eq!(missing, None);
    }

    #[test]
    fn test_state_manager_basic_ops() {
        let manager = JokerStateManager::new();
        let joker_id = JokerId::Joker;

        // Initially no state
        assert!(!manager.has_state(joker_id));
        assert_eq!(manager.count(), 0);

        // Get creates default state
        let state = manager.get_state(joker_id);
        assert_eq!(state.accumulated_value, 0.0);

        // Set state
        let new_state = JokerState::with_accumulated_value(15.0);
        manager.set_state(joker_id, new_state);
        assert!(manager.has_state(joker_id));
        assert_eq!(manager.count(), 1);

        let retrieved = manager.get_state(joker_id);
        assert_eq!(retrieved.accumulated_value, 15.0);
    }

    #[test]
    fn test_state_manager_update() {
        let manager = JokerStateManager::new();
        let joker_id = JokerId::Joker;

        // Update non-existent state (creates default)
        manager.update_state(joker_id, |state| {
            state.add_value(10.0);
        });

        let state = manager.get_state(joker_id);
        assert_eq!(state.accumulated_value, 10.0);

        // Update existing state
        manager.update_state(joker_id, |state| {
            state.add_value(5.0);
        });

        let state = manager.get_state(joker_id);
        assert_eq!(state.accumulated_value, 15.0);
    }

    #[test]
    fn test_state_manager_accumulated_value() {
        let manager = JokerStateManager::new();
        let joker_id = JokerId::Joker;

        manager.add_accumulated_value(joker_id, 3.0);
        manager.add_accumulated_value(joker_id, 2.0);

        let state = manager.get_state(joker_id);
        assert_eq!(state.accumulated_value, 5.0);
    }

    #[test]
    fn test_state_manager_triggers() {
        let manager = JokerStateManager::new();
        let joker_id = JokerId::Joker;

        // Set state with limited triggers
        let state = JokerState::with_triggers(2);
        manager.set_state(joker_id, state);

        assert!(manager.has_triggers(joker_id));
        assert!(manager.use_trigger(joker_id));
        assert!(manager.has_triggers(joker_id));
        assert!(manager.use_trigger(joker_id));
        assert!(!manager.has_triggers(joker_id));
        assert!(!manager.use_trigger(joker_id));
    }

    #[test]
    fn test_state_manager_custom_data() {
        let manager = JokerStateManager::new();
        let joker_id = JokerId::Joker;

        manager
            .set_custom_data(joker_id, "test_key", "test_value")
            .unwrap();
        let value: Option<String> = manager.get_custom_data(joker_id, "test_key").unwrap();
        assert_eq!(value, Some("test_value".to_string()));
    }

    #[test]
    fn test_state_manager_remove() {
        let manager = JokerStateManager::new();
        let joker_id = JokerId::Joker;

        manager.add_accumulated_value(joker_id, 10.0);
        assert!(manager.has_state(joker_id));

        let removed = manager.remove_state(joker_id);
        assert!(removed.is_some());
        assert_eq!(removed.unwrap().accumulated_value, 10.0);
        assert!(!manager.has_state(joker_id));
    }

    #[test]
    fn test_state_manager_clear() {
        let manager = JokerStateManager::new();

        manager.add_accumulated_value(JokerId::Joker, 10.0);
        manager.add_accumulated_value(JokerId::GreedyJoker, 20.0);
        assert_eq!(manager.count(), 2);

        manager.clear();
        assert_eq!(manager.count(), 0);
    }

    #[test]
    fn test_state_manager_active_jokers() {
        let manager = JokerStateManager::new();

        manager.add_accumulated_value(JokerId::Joker, 10.0);
        manager.add_accumulated_value(JokerId::GreedyJoker, 20.0);

        let active = manager.get_active_jokers();
        assert_eq!(active.len(), 2);
        assert!(active.contains(&JokerId::Joker));
        assert!(active.contains(&JokerId::GreedyJoker));
    }
}
