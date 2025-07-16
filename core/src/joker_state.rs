use crate::joker::JokerId;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

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

    /// Get a copy of a joker's state if it exists
    pub fn get_state(&self, joker_id: JokerId) -> Option<JokerState> {
        let states = self.states.read().unwrap();
        states.get(&joker_id).cloned()
    }

    /// Get a copy of a joker's state or create default if not exists
    pub fn get_or_default(&self, joker_id: JokerId) -> JokerState {
        self.get_state(joker_id).unwrap_or_default()
    }

    /// Get or insert a joker's state with a custom initialization function
    ///
    /// # Examples
    ///
    /// ```
    /// # use balatro_rs::joker_state::{JokerStateManager, JokerState};
    /// # use balatro_rs::joker::JokerId;
    /// let manager = JokerStateManager::new();
    ///
    /// // Get or create state with initial triggers
    /// let state = manager.get_or_insert_with(JokerId::Joker, || {
    ///     JokerState::with_triggers(5)
    /// });
    ///
    /// // Get or create state with initial accumulated value
    /// let state = manager.get_or_insert_with(JokerId::GreedyJoker, || {
    ///     JokerState::with_accumulated_value(100.0)
    /// });
    /// ```
    pub fn get_or_insert_with<F>(&self, joker_id: JokerId, init_fn: F) -> JokerState
    where
        F: FnOnce() -> JokerState,
    {
        // First check if state exists
        if let Some(state) = self.get_state(joker_id) {
            return state;
        }

        // If not, create and insert new state
        let mut states = self.states.write().unwrap();
        let state = init_fn();
        states.insert(joker_id, state.clone());
        state
    }

    /// Set a joker's state
    pub fn set_state(&self, joker_id: JokerId, state: JokerState) {
        let mut states = self.states.write().unwrap();
        states.insert(joker_id, state);
    }

    /// Update a joker's state with a closure
    ///
    /// # Examples
    ///
    /// ```
    /// # use balatro_rs::joker_state::JokerStateManager;
    /// # use balatro_rs::joker::JokerId;
    /// let manager = JokerStateManager::new();
    ///
    /// // Add to accumulated value
    /// manager.update_state(JokerId::Joker, |state| {
    ///     state.accumulated_value += 10.0;
    /// });
    ///
    /// // Set custom data
    /// manager.update_state(JokerId::Joker, |state| {
    ///     state.set_custom("combo_count", 5).unwrap();
    /// });
    /// ```
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

    /// Use a trigger for a joker (returns true if trigger was successfully used)
    ///
    /// This will create a default state if none exists. Default states have
    /// unlimited triggers (always returns true).
    ///
    /// # Examples
    ///
    /// ```
    /// # use balatro_rs::joker_state::{JokerStateManager, JokerState};
    /// # use balatro_rs::joker::JokerId;
    /// let manager = JokerStateManager::new();
    /// let joker_id = JokerId::Joker;
    ///
    /// // Using trigger on non-existent state creates default (unlimited triggers)
    /// assert!(manager.use_trigger(joker_id));
    /// assert!(manager.use_trigger(joker_id)); // Still true - unlimited
    ///
    /// // Set limited triggers
    /// manager.set_state(joker_id, JokerState::with_triggers(2));
    /// assert!(manager.use_trigger(joker_id)); // Uses first trigger
    /// assert!(manager.use_trigger(joker_id)); // Uses second trigger
    /// assert!(!manager.use_trigger(joker_id)); // No triggers left
    /// ```
    pub fn use_trigger(&self, joker_id: JokerId) -> bool {
        let mut states = self.states.write().unwrap();

        // Get or create the state
        let state = states.entry(joker_id).or_default();

        // Use the trigger and return the result directly
        state.use_trigger()
    }

    /// Check if a joker has triggers available
    ///
    /// This is a read-only operation that does not create state.
    /// Returns `false` if no state exists for the joker.
    ///
    /// # Examples
    ///
    /// ```
    /// # use balatro_rs::joker_state::{JokerStateManager, JokerState};
    /// # use balatro_rs::joker::JokerId;
    /// let manager = JokerStateManager::new();
    /// let joker_id = JokerId::Joker;
    ///
    /// // No state exists yet
    /// assert!(!manager.has_triggers(joker_id));
    ///
    /// // After creating state with triggers
    /// manager.set_state(joker_id, JokerState::with_triggers(2));
    /// assert!(manager.has_triggers(joker_id));
    /// ```
    pub fn has_triggers(&self, joker_id: JokerId) -> bool {
        match self.get_state(joker_id) {
            Some(state) => state.has_triggers(),
            None => false, // No state means no triggers available
        }
    }

    /// Set custom data for a joker
    ///
    /// # Examples
    ///
    /// ```
    /// # use balatro_rs::joker_state::JokerStateManager;
    /// # use balatro_rs::joker::JokerId;
    /// let manager = JokerStateManager::new();
    ///
    /// // Store a simple value
    /// manager.set_custom_data(JokerId::Joker, "level", 3).unwrap();
    ///
    /// // Store a complex structure
    /// #[derive(serde::Serialize)]
    /// struct ComboData {
    ///     count: u32,
    ///     multiplier: f64,
    /// }
    ///
    /// let combo = ComboData { count: 5, multiplier: 1.5 };
    /// manager.set_custom_data(JokerId::Joker, "combo", combo).unwrap();
    /// ```
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
    ///
    /// Returns None if the joker has no state or the key doesn't exist.
    ///
    /// # Examples
    ///
    /// ```
    /// # use balatro_rs::joker_state::{JokerStateManager, JokerState};
    /// # use balatro_rs::joker::JokerId;
    /// # use std::collections::HashMap;
    /// let manager = JokerStateManager::new();
    /// let joker_id = JokerId::Joker;
    ///
    /// // Set and retrieve data
    /// manager.set_custom_data(joker_id, "level", 5).unwrap();
    /// let level: Option<i32> = manager.get_custom_data(joker_id, "level").unwrap();
    /// assert_eq!(level, Some(5));
    ///
    /// // Non-existent key returns None
    /// let missing: Option<String> = manager.get_custom_data(joker_id, "missing").unwrap();
    /// assert_eq!(missing, None);
    ///
    /// // Complex types
    /// #[derive(serde::Deserialize, serde::Serialize)]
    /// struct Stats { wins: u32, losses: u32 }
    ///
    /// let stats = Stats { wins: 10, losses: 2 };
    /// manager.set_custom_data(joker_id, "stats", stats).unwrap();
    /// let retrieved: Option<Stats> = manager.get_custom_data(joker_id, "stats").unwrap();
    /// assert!(retrieved.is_some());
    /// assert_eq!(retrieved.unwrap().wins, 10);
    /// ```
    pub fn get_custom_data<T: for<'de> Deserialize<'de>>(
        &self,
        joker_id: JokerId,
        key: &str,
    ) -> Result<Option<T>, serde_json::Error> {
        match self.get_state(joker_id) {
            Some(state) => state.get_custom(key),
            None => Ok(None), // No state means no custom data
        }
    }
}

impl Default for JokerStateManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Type alias for state update function
type StateUpdateFn = Box<dyn FnOnce(&mut JokerState)>;

/// Enhanced state management capabilities for comprehensive control
impl JokerStateManager {
    /// Bulk update operations for improved performance
    ///
    /// Applies multiple state updates efficiently in a single write lock operation.
    ///
    /// # Arguments
    /// * `updates` - Vector of (JokerId, update_function) tuples
    ///
    /// # Examples
    /// ```rust,ignore
    /// let manager = JokerStateManager::new();
    /// let updates = vec![
    ///     (JokerId::Joker, Box::new(|state: &mut JokerState| state.add_value(10.0)) as StateUpdateFn),
    ///     (JokerId::GreedyJoker, Box::new(|state: &mut JokerState| state.add_value(5.0)) as StateUpdateFn),
    ///     (JokerId::LustyJoker, Box::new(|state: &mut JokerState| {
    ///         state.set_custom("level", 3).unwrap();
    ///     }) as StateUpdateFn),
    /// ];
    /// manager.bulk_update(updates);
    /// ```
    pub fn bulk_update(&self, updates: Vec<(JokerId, StateUpdateFn)>) {
        let mut states = self.states.write().unwrap();
        for (joker_id, update_fn) in updates {
            let state = states.entry(joker_id).or_default();
            update_fn(state);
        }
    }

    /// Create a snapshot of all current joker states
    ///
    /// Useful for save games, debugging, and state history tracking.
    ///
    /// # Returns
    /// A HashMap containing copies of all current joker states
    ///
    /// # Examples
    /// ```rust,ignore
    /// let manager = JokerStateManager::new();
    /// manager.add_accumulated_value(JokerId::Joker, 50.0);
    ///
    /// let snapshot = manager.snapshot_all();
    /// assert!(snapshot.contains_key(&JokerId::Joker));
    /// assert_eq!(snapshot[&JokerId::Joker].accumulated_value, 50.0);
    /// ```
    pub fn snapshot_all(&self) -> HashMap<JokerId, JokerState> {
        let states = self.states.read().unwrap();
        states.clone()
    }

    /// Restore state from a snapshot
    ///
    /// Replaces all current state with the provided snapshot.
    /// Useful for loading save games or implementing undo functionality.
    ///
    /// # Arguments
    /// * `snapshot` - HashMap of joker states to restore
    ///
    /// # Examples
    /// ```rust,ignore
    /// let manager = JokerStateManager::new();
    ///
    /// // Create and save a snapshot
    /// manager.add_accumulated_value(JokerId::Joker, 50.0);
    /// let snapshot = manager.snapshot_all();
    ///
    /// // Modify state
    /// manager.add_accumulated_value(JokerId::Joker, 25.0);
    ///
    /// // Restore from snapshot
    /// manager.restore_from_snapshot(snapshot);
    /// assert_eq!(manager.get_accumulated_value(JokerId::Joker), Some(50.0));
    /// ```
    pub fn restore_from_snapshot(&self, snapshot: HashMap<JokerId, JokerState>) {
        let mut states = self.states.write().unwrap();
        *states = snapshot;
    }

    /// Get accumulated value for a joker (convenience method)
    ///
    /// # Arguments
    /// * `joker_id` - The joker to get accumulated value for
    ///
    /// # Returns
    /// The accumulated value if the joker has state, None otherwise
    pub fn get_accumulated_value(&self, joker_id: JokerId) -> Option<f64> {
        self.get_state(joker_id)
            .map(|state| state.accumulated_value)
    }

    /// Compact states by removing inactive jokers
    ///
    /// Removes state for jokers that are no longer active to save memory.
    /// Useful for long-running games with many joker changes.
    ///
    /// # Arguments
    /// * `active_jokers` - Slice of currently active joker IDs
    ///
    /// # Returns
    /// Number of states that were removed
    ///
    /// # Examples
    /// ```rust,ignore
    /// let manager = JokerStateManager::new();
    /// manager.add_accumulated_value(JokerId::Joker, 10.0);
    /// manager.add_accumulated_value(JokerId::GreedyJoker, 20.0);
    /// manager.add_accumulated_value(JokerId::LustyJoker, 30.0);
    ///
    /// // Only Joker and GreedyJoker are still active
    /// let active = vec![JokerId::Joker, JokerId::GreedyJoker];
    /// let removed = manager.compact_states(&active);
    /// assert_eq!(removed, 1); // LustyJoker state was removed
    /// assert!(!manager.has_state(JokerId::LustyJoker));
    /// ```
    pub fn compact_states(&self, active_jokers: &[JokerId]) -> usize {
        let mut states = self.states.write().unwrap();
        let active_set: std::collections::HashSet<_> = active_jokers.iter().collect();

        let initial_count = states.len();
        states.retain(|joker_id, _| active_set.contains(joker_id));

        initial_count - states.len()
    }

    /// Get detailed memory usage report
    ///
    /// Provides insights into memory usage for performance monitoring.
    ///
    /// # Returns
    /// A StateMemoryReport with detailed memory statistics
    pub fn get_memory_usage(&self) -> StateMemoryReport {
        let states = self.states.read().unwrap();

        let total_jokers = states.len();
        let total_custom_data_entries = states.values().map(|state| state.custom_data.len()).sum();

        // Estimate memory usage (rough calculation)
        let estimated_bytes = std::mem::size_of::<HashMap<JokerId, JokerState>>()
            + total_jokers * std::mem::size_of::<(JokerId, JokerState)>()
            + total_custom_data_entries
                * (
                    std::mem::size_of::<String>() + std::mem::size_of::<Value>() + 32
                    // Estimated average string/value size
                );

        StateMemoryReport {
            total_jokers,
            total_custom_data_entries,
            estimated_bytes,
            average_custom_data_per_joker: if total_jokers > 0 {
                total_custom_data_entries as f64 / total_jokers as f64
            } else {
                0.0
            },
        }
    }

    /// Validate all joker states
    ///
    /// Checks all stored states for consistency and validity.
    /// Useful for debugging and ensuring data integrity.
    ///
    /// # Returns
    /// A vector of validation errors found
    pub fn validate_all_states(&self) -> Vec<StateValidationError> {
        let states = self.states.read().unwrap();
        let mut errors = Vec::new();

        for (joker_id, state) in states.iter() {
            // Check for negative accumulated values (might be invalid for some jokers)
            if state.accumulated_value < 0.0 {
                errors.push(StateValidationError::NegativeAccumulatedValue {
                    joker_id: *joker_id,
                    value: state.accumulated_value,
                });
            }

            // Check for extremely large accumulated values (might indicate overflow)
            if state.accumulated_value > 1_000_000.0 {
                errors.push(StateValidationError::ExcessiveAccumulatedValue {
                    joker_id: *joker_id,
                    value: state.accumulated_value,
                });
            }

            // Check for invalid trigger counts
            if let Some(triggers) = state.triggers_remaining {
                if triggers > 1000 {
                    errors.push(StateValidationError::ExcessiveTriggerCount {
                        joker_id: *joker_id,
                        count: triggers,
                    });
                }
            }

            // Check for excessive custom data
            if state.custom_data.len() > 50 {
                errors.push(StateValidationError::ExcessiveCustomData {
                    joker_id: *joker_id,
                    count: state.custom_data.len(),
                });
            }
        }

        errors
    }

    /// Batch set multiple joker states efficiently
    ///
    /// Sets multiple joker states in a single write operation for better performance.
    ///
    /// # Arguments
    /// * `batch_states` - Vector of (JokerId, JokerState) tuples to set
    ///
    /// # Examples
    /// ```rust,ignore
    /// let manager = JokerStateManager::new();
    /// let batch = vec![
    ///     (JokerId::Joker, JokerState::with_accumulated_value(10.0)),
    ///     (JokerId::GreedyJoker, JokerState::with_triggers(5)),
    ///     (JokerId::LustyJoker, JokerState::new()),
    /// ];
    /// manager.batch_set_states(batch);
    /// ```
    pub fn batch_set_states(&self, batch_states: Vec<(JokerId, JokerState)>) {
        let mut states = self.states.write().unwrap();
        for (joker_id, state) in batch_states {
            states.insert(joker_id, state);
        }
    }

    /// Get states for multiple jokers efficiently
    ///
    /// Retrieves multiple joker states in a single read operation.
    ///
    /// # Arguments
    /// * `joker_ids` - Slice of joker IDs to retrieve states for
    ///
    /// # Returns
    /// HashMap with requested joker states (only includes jokers that have state)
    pub fn batch_get_states(&self, joker_ids: &[JokerId]) -> HashMap<JokerId, JokerState> {
        let states = self.states.read().unwrap();
        let mut result = HashMap::new();

        for &joker_id in joker_ids {
            if let Some(state) = states.get(&joker_id) {
                result.insert(joker_id, state.clone());
            }
        }

        result
    }

    /// Ensure a state exists for the given joker ID
    ///
    /// Creates a default state if one doesn't already exist.
    ///
    /// # Arguments
    /// * `joker_id` - The joker ID to ensure state exists for
    pub fn ensure_state_exists(&self, joker_id: JokerId) {
        let mut states = self.states.write().unwrap();
        states.entry(joker_id).or_default();
    }
}

/// Memory usage report for joker state management
#[derive(Debug, Clone)]
pub struct StateMemoryReport {
    /// Total number of jokers with state
    pub total_jokers: usize,
    /// Total number of custom data entries across all jokers
    pub total_custom_data_entries: usize,
    /// Estimated memory usage in bytes
    pub estimated_bytes: usize,
    /// Average number of custom data entries per joker
    pub average_custom_data_per_joker: f64,
}

/// Validation errors for joker states
#[derive(Debug, Clone, PartialEq)]
pub enum StateValidationError {
    /// Accumulated value is negative when it shouldn't be
    NegativeAccumulatedValue { joker_id: JokerId, value: f64 },
    /// Accumulated value is excessively large (possible overflow)
    ExcessiveAccumulatedValue { joker_id: JokerId, value: f64 },
    /// Trigger count is suspiciously high
    ExcessiveTriggerCount { joker_id: JokerId, count: u32 },
    /// Too many custom data entries (possible memory leak)
    ExcessiveCustomData { joker_id: JokerId, count: usize },
}

/// Comprehensive joker persistence interface for save/load operations
pub struct JokerPersistenceManager {
    state_manager: Arc<JokerStateManager>,
}

impl JokerPersistenceManager {
    /// Create a new persistence manager
    pub fn new(state_manager: Arc<JokerStateManager>) -> Self {
        Self { state_manager }
    }

    /// Save all joker states to a persistent format
    ///
    /// Uses each joker's custom serialization logic via the Joker trait.
    ///
    /// # Arguments
    /// * `jokers` - Slice of active jokers to save state for
    /// * `context` - Current game context for serialization
    ///
    /// # Returns
    /// A versioned save data structure containing all joker states
    ///
    /// # Examples
    /// ```rust,ignore
    /// let persistence = JokerPersistenceManager::new(state_manager);
    /// let save_data = persistence.save_all_states(&active_jokers, &context)?;
    ///
    /// // Save to file
    /// let json = serde_json::to_string_pretty(&save_data)?;
    /// std::fs::write("joker_states.json", json)?;
    /// ```
    pub fn save_all_states(
        &self,
        jokers: &[Box<dyn crate::joker::Joker>],
        context: &crate::joker::GameContext,
    ) -> Result<VersionedSaveData, PersistenceError> {
        let mut joker_states = HashMap::new();

        for joker in jokers {
            let joker_id = joker.id();

            // Get current state from manager
            if let Some(state) = self.state_manager.get_state(joker_id) {
                // Use joker's custom serialization
                match joker.serialize_state(context, &state) {
                    Ok(serialized) => {
                        joker_states.insert(joker_id, serialized);
                    }
                    Err(e) => {
                        return Err(PersistenceError::SerializationFailed {
                            joker_id,
                            source: e,
                        });
                    }
                }
            }
        }

        Ok(VersionedSaveData {
            version: CURRENT_SAVE_VERSION,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            joker_states,
            metadata: SaveMetadata {
                total_jokers: jokers.len(),
                game_round: context.round,
                game_ante: context.ante,
            },
        })
    }

    /// Load joker states from persistent format
    ///
    /// Uses each joker's custom deserialization and validation logic.
    ///
    /// # Arguments
    /// * `save_data` - The versioned save data to load from
    /// * `jokers` - Slice of jokers to load state for
    /// * `context` - Current game context for deserialization
    ///
    /// # Returns
    /// Result indicating success or failure of the load operation
    ///
    /// # Examples
    /// ```rust,ignore
    /// let persistence = JokerPersistenceManager::new(state_manager);
    ///
    /// // Load from file
    /// let json = std::fs::read_to_string("joker_states.json")?;
    /// let save_data: VersionedSaveData = serde_json::from_str(&json)?;
    ///
    /// persistence.load_all_states(&save_data, &active_jokers, &context)?;
    /// ```
    pub fn load_all_states(
        &self,
        save_data: &VersionedSaveData,
        jokers: &[Box<dyn crate::joker::Joker>],
        context: &crate::joker::GameContext,
    ) -> Result<LoadResult, PersistenceError> {
        let mut loaded_states = HashMap::new();
        let mut errors = Vec::new();
        let mut migrated_count = 0;

        for joker in jokers {
            let joker_id = joker.id();

            if let Some(serialized_state) = save_data.joker_states.get(&joker_id) {
                // Handle version migration if needed
                let state = if save_data.version != CURRENT_SAVE_VERSION {
                    migrated_count += 1;
                    match joker.migrate_state(context, serialized_state, save_data.version) {
                        Ok(migrated_state) => migrated_state,
                        Err(e) => {
                            errors.push(LoadError::MigrationFailed {
                                joker_id,
                                from_version: save_data.version,
                                error: e,
                            });
                            continue;
                        }
                    }
                } else {
                    // Use joker's custom deserialization
                    match joker.deserialize_state(context, serialized_state) {
                        Ok(state) => state,
                        Err(e) => {
                            errors.push(LoadError::DeserializationFailed {
                                joker_id,
                                source: e,
                            });
                            continue;
                        }
                    }
                };

                // Validate the loaded/migrated state
                if let Err(validation_error) = joker.validate_state(context, &state) {
                    errors.push(LoadError::ValidationFailed {
                        joker_id,
                        error: validation_error,
                    });
                    continue;
                }

                loaded_states.insert(joker_id, state);
            }
        }

        // Apply all successfully loaded states
        self.state_manager
            .batch_set_states(loaded_states.into_iter().collect());

        Ok(LoadResult {
            loaded_count: jokers.len() - errors.len(),
            migrated_count,
            errors,
        })
    }

    /// Initialize states for new jokers
    ///
    /// Uses each joker's initialize_state method to set up default state.
    ///
    /// # Arguments
    /// * `jokers` - Slice of new jokers to initialize
    /// * `context` - Current game context for initialization
    pub fn initialize_new_jokers(
        &self,
        jokers: &[Box<dyn crate::joker::Joker>],
        context: &crate::joker::GameContext,
    ) {
        let new_states: Vec<_> = jokers
            .iter()
            .map(|joker| (joker.id(), joker.initialize_state(context)))
            .collect();

        self.state_manager.batch_set_states(new_states);
    }

    /// Validate all current joker states
    ///
    /// Uses both the state manager's built-in validation and each joker's
    /// custom validation logic.
    ///
    /// # Arguments
    /// * `jokers` - Slice of active jokers to validate
    /// * `context` - Current game context for validation
    ///
    /// # Returns
    /// Comprehensive validation report
    pub fn validate_all_states(
        &self,
        jokers: &[Box<dyn crate::joker::Joker>],
        context: &crate::joker::GameContext,
    ) -> ValidationReport {
        let general_errors = self.state_manager.validate_all_states();
        let mut joker_specific_errors = Vec::new();

        for joker in jokers {
            let joker_id = joker.id();

            if let Some(state) = self.state_manager.get_state(joker_id) {
                if let Err(error) = joker.validate_state(context, &state) {
                    joker_specific_errors.push(JokerValidationError { joker_id, error });
                }
            }
        }

        ValidationReport {
            general_errors,
            joker_specific_errors,
            total_jokers_validated: jokers.len(),
        }
    }

    /// Create a backup of current state
    ///
    /// Creates a complete snapshot that can be restored later.
    pub fn create_backup(&self) -> StateBackup {
        StateBackup {
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            states: self.state_manager.snapshot_all(),
        }
    }

    /// Restore from a backup
    ///
    /// Replaces all current state with the backup data.
    pub fn restore_from_backup(&self, backup: &StateBackup) {
        self.state_manager
            .restore_from_snapshot(backup.states.clone());
    }

    /// Validate state data without loading it
    ///
    /// Checks if the provided state data is structurally valid.
    pub fn validate_state_data(&self, states: &HashMap<JokerId, Value>) -> Result<(), String> {
        for (joker_id, value) in states {
            // Try to deserialize the value to JokerState to validate structure
            match serde_json::from_value::<JokerState>(value.clone()) {
                Ok(_) => continue,
                Err(e) => return Err(format!("Invalid state for joker {joker_id:?}: {e}")),
            }
        }
        Ok(())
    }

    /// Load states with recovery for corrupted data
    ///
    /// Attempts to load valid states and reports errors for invalid ones.
    pub fn load_states_with_recovery(
        &self,
        states: &HashMap<JokerId, Value>,
    ) -> Result<(HashMap<JokerId, JokerState>, Vec<String>), String> {
        let mut loaded_states = HashMap::new();
        let mut errors = Vec::new();

        for (joker_id, value) in states {
            match serde_json::from_value::<JokerState>(value.clone()) {
                Ok(state) => {
                    loaded_states.insert(*joker_id, state);
                }
                Err(e) => {
                    errors.push(format!("Failed to load state for joker {joker_id:?}: {e}"));
                }
            }
        }

        Ok((loaded_states, errors))
    }

    /// Load from JSON with handling for unknown jokers
    ///
    /// Loads save data and gracefully handles unknown joker IDs.
    pub fn load_from_json_with_unknown_handling(
        &self,
        json: &str,
    ) -> Result<(HashMap<JokerId, JokerState>, Vec<String>), String> {
        // Parse as a generic Value first to handle unknown joker IDs
        let raw_data: Value =
            serde_json::from_str(json).map_err(|e| format!("Failed to parse JSON: {e}"))?;

        let mut loaded_states = HashMap::new();
        let mut warnings = Vec::new();

        // Extract joker_states as a generic object
        if let Some(joker_states_obj) = raw_data.get("joker_states") {
            if let Some(states_map) = joker_states_obj.as_object() {
                for (joker_id_str, state_value) in states_map {
                    // Try to parse the joker ID string to JokerId enum
                    match serde_json::from_str::<JokerId>(&format!("\"{joker_id_str}\"")) {
                        Ok(joker_id) => {
                            // Known joker ID, try to parse the state
                            match serde_json::from_value::<JokerState>(state_value.clone()) {
                                Ok(state) => {
                                    loaded_states.insert(joker_id, state);
                                }
                                Err(_) => {
                                    warnings.push(format!(
                                        "Failed to parse state for joker: {joker_id_str}"
                                    ));
                                }
                            }
                        }
                        Err(_) => {
                            // Unknown joker ID, skip with warning
                            warnings.push(format!("Skipping unknown joker: {joker_id_str}"));
                        }
                    }
                }
            }
        }

        Ok((loaded_states, warnings))
    }
}

/// Current save file version
const CURRENT_SAVE_VERSION: u32 = 1;

/// Versioned save data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionedSaveData {
    /// Save format version for migration support
    pub version: u32,
    /// Timestamp when save was created
    pub timestamp: u64,
    /// Serialized state for each joker
    pub joker_states: HashMap<JokerId, Value>,
    /// Additional metadata about the save
    pub metadata: SaveMetadata,
}

/// Metadata about a save file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveMetadata {
    /// Number of jokers in the save
    pub total_jokers: usize,
    /// Game round when saved
    pub game_round: u32,
    /// Game ante when saved
    pub game_ante: u8,
}

/// Result of a load operation
#[derive(Debug)]
pub struct LoadResult {
    /// Number of joker states successfully loaded
    pub loaded_count: usize,
    /// Number of states that required migration
    pub migrated_count: usize,
    /// Errors encountered during loading
    pub errors: Vec<LoadError>,
}

/// Errors that can occur during loading
#[derive(Debug)]
pub enum LoadError {
    /// Failed to deserialize a joker's state
    DeserializationFailed {
        joker_id: JokerId,
        source: serde_json::Error,
    },
    /// Failed to migrate state from old version
    MigrationFailed {
        joker_id: JokerId,
        from_version: u32,
        error: String,
    },
    /// State validation failed after loading
    ValidationFailed { joker_id: JokerId, error: String },
}

/// Errors that can occur during persistence operations
#[derive(Debug)]
pub enum PersistenceError {
    /// Failed to serialize a joker's state
    SerializationFailed {
        joker_id: JokerId,
        source: serde_json::Error,
    },
    /// Invalid save data format
    InvalidSaveData { reason: String },
    /// Unsupported save version
    UnsupportedVersion { version: u32 },
}

/// Complete validation report
#[derive(Debug)]
pub struct ValidationReport {
    /// General state validation errors from StateManager
    pub general_errors: Vec<StateValidationError>,
    /// Joker-specific validation errors
    pub joker_specific_errors: Vec<JokerValidationError>,
    /// Total number of jokers validated
    pub total_jokers_validated: usize,
}

/// Joker-specific validation error
#[derive(Debug)]
pub struct JokerValidationError {
    /// The joker that failed validation
    pub joker_id: JokerId,
    /// The validation error message
    pub error: String,
}

/// State backup for undo/restore functionality
#[derive(Debug, Clone)]
pub struct StateBackup {
    /// When the backup was created
    pub timestamp: u64,
    /// Complete state snapshot
    pub states: HashMap<JokerId, JokerState>,
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

        // Get returns None when no state exists
        let state = manager.get_state(joker_id);
        assert!(state.is_none());

        // Set state
        let new_state = JokerState::with_accumulated_value(15.0);
        manager.set_state(joker_id, new_state);
        assert!(manager.has_state(joker_id));
        assert_eq!(manager.count(), 1);

        let retrieved = manager.get_state(joker_id);
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().accumulated_value, 15.0);
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
        assert!(state.is_some());
        assert_eq!(state.unwrap().accumulated_value, 10.0);

        // Update existing state
        manager.update_state(joker_id, |state| {
            state.add_value(5.0);
        });

        let state = manager.get_state(joker_id);
        assert!(state.is_some());
        assert_eq!(state.unwrap().accumulated_value, 15.0);
    }

    #[test]
    fn test_state_manager_accumulated_value() {
        let manager = JokerStateManager::new();
        let joker_id = JokerId::Joker;

        manager.add_accumulated_value(joker_id, 3.0);
        manager.add_accumulated_value(joker_id, 2.0);

        let state = manager.get_state(joker_id);
        assert!(state.is_some());
        assert_eq!(state.unwrap().accumulated_value, 5.0);
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

    // Tests for enhanced state management capabilities

    #[test]
    fn test_bulk_update() {
        let manager = JokerStateManager::new();

        let updates = vec![
            (
                JokerId::Joker,
                Box::new(|state: &mut JokerState| state.add_value(10.0)) as StateUpdateFn,
            ),
            (
                JokerId::GreedyJoker,
                Box::new(|state: &mut JokerState| state.add_value(20.0)) as StateUpdateFn,
            ),
            (
                JokerId::LustyJoker,
                Box::new(|state: &mut JokerState| {
                    state.set_custom("level", 5).unwrap();
                }) as StateUpdateFn,
            ),
        ];

        manager.bulk_update(updates);

        assert_eq!(manager.get_accumulated_value(JokerId::Joker), Some(10.0));
        assert_eq!(
            manager.get_accumulated_value(JokerId::GreedyJoker),
            Some(20.0)
        );
        assert_eq!(
            manager
                .get_custom_data::<i32>(JokerId::LustyJoker, "level")
                .unwrap(),
            Some(5)
        );
    }

    #[test]
    fn test_snapshot_and_restore() {
        let manager = JokerStateManager::new();

        // Create initial state
        manager.add_accumulated_value(JokerId::Joker, 50.0);
        manager.add_accumulated_value(JokerId::GreedyJoker, 75.0);

        // Take snapshot
        let snapshot = manager.snapshot_all();
        assert_eq!(snapshot.len(), 2);
        assert_eq!(snapshot[&JokerId::Joker].accumulated_value, 50.0);

        // Modify state
        manager.add_accumulated_value(JokerId::Joker, 25.0);
        assert_eq!(manager.get_accumulated_value(JokerId::Joker), Some(75.0));

        // Restore from snapshot
        manager.restore_from_snapshot(snapshot);
        assert_eq!(manager.get_accumulated_value(JokerId::Joker), Some(50.0));
        assert_eq!(
            manager.get_accumulated_value(JokerId::GreedyJoker),
            Some(75.0)
        );
    }

    #[test]
    fn test_compact_states() {
        let manager = JokerStateManager::new();

        // Add multiple states
        manager.add_accumulated_value(JokerId::Joker, 10.0);
        manager.add_accumulated_value(JokerId::GreedyJoker, 20.0);
        manager.add_accumulated_value(JokerId::LustyJoker, 30.0);
        manager.add_accumulated_value(JokerId::SteelJoker, 40.0);

        assert_eq!(manager.count(), 4);

        // Compact to only keep some jokers
        let active = vec![JokerId::Joker, JokerId::GreedyJoker];
        let removed = manager.compact_states(&active);

        assert_eq!(removed, 2); // LustyJoker and SteelJoker removed
        assert_eq!(manager.count(), 2);
        assert!(manager.has_state(JokerId::Joker));
        assert!(manager.has_state(JokerId::GreedyJoker));
        assert!(!manager.has_state(JokerId::LustyJoker));
        assert!(!manager.has_state(JokerId::SteelJoker));
    }

    #[test]
    fn test_memory_usage_report() {
        let manager = JokerStateManager::new();

        // Add states with custom data
        for i in 0..5 {
            let joker_id = match i {
                0 => JokerId::Joker,
                1 => JokerId::GreedyJoker,
                2 => JokerId::LustyJoker,
                3 => JokerId::SteelJoker,
                _ => JokerId::AbstractJoker,
            };

            manager.add_accumulated_value(joker_id, i as f64 * 10.0);
            manager.set_custom_data(joker_id, "level", i).unwrap();
            manager
                .set_custom_data(joker_id, "multiplier", i as f64 * 1.5)
                .unwrap();
        }

        let report = manager.get_memory_usage();
        assert_eq!(report.total_jokers, 5);
        assert_eq!(report.total_custom_data_entries, 10); // 2 entries per joker
        assert_eq!(report.average_custom_data_per_joker, 2.0);
        assert!(report.estimated_bytes > 0);
    }

    #[test]
    fn test_state_validation() {
        let manager = JokerStateManager::new();

        // Add valid states
        manager.add_accumulated_value(JokerId::Joker, 100.0);
        manager.add_accumulated_value(JokerId::GreedyJoker, 50.0);

        // Add invalid states
        manager.add_accumulated_value(JokerId::LustyJoker, -10.0); // Negative value
        manager.add_accumulated_value(JokerId::SteelJoker, 2_000_000.0); // Excessive value

        // Add excessive triggers
        manager.set_state(JokerId::AbstractJoker, JokerState::with_triggers(5000));

        let errors = manager.validate_all_states();

        // Should find the issues we created
        let negative_errors: Vec<_> = errors
            .iter()
            .filter(|e| matches!(e, StateValidationError::NegativeAccumulatedValue { .. }))
            .collect();
        assert_eq!(negative_errors.len(), 1);

        let excessive_value_errors: Vec<_> = errors
            .iter()
            .filter(|e| matches!(e, StateValidationError::ExcessiveAccumulatedValue { .. }))
            .collect();
        assert_eq!(excessive_value_errors.len(), 1);

        let excessive_trigger_errors: Vec<_> = errors
            .iter()
            .filter(|e| matches!(e, StateValidationError::ExcessiveTriggerCount { .. }))
            .collect();
        assert_eq!(excessive_trigger_errors.len(), 1);
    }

    #[test]
    fn test_batch_operations() {
        let manager = JokerStateManager::new();

        // Test batch set
        let batch_states = vec![
            (JokerId::Joker, JokerState::with_accumulated_value(10.0)),
            (JokerId::GreedyJoker, JokerState::with_triggers(5)),
            (JokerId::LustyJoker, JokerState::new()),
        ];

        manager.batch_set_states(batch_states);

        assert_eq!(manager.get_accumulated_value(JokerId::Joker), Some(10.0));
        assert!(manager.has_triggers(JokerId::GreedyJoker));
        assert!(manager.has_state(JokerId::LustyJoker));

        // Test batch get
        let joker_ids = vec![JokerId::Joker, JokerId::GreedyJoker, JokerId::SteelJoker];
        let retrieved = manager.batch_get_states(&joker_ids);

        assert_eq!(retrieved.len(), 2); // Only Joker and GreedyJoker have state
        assert!(retrieved.contains_key(&JokerId::Joker));
        assert!(retrieved.contains_key(&JokerId::GreedyJoker));
        assert!(!retrieved.contains_key(&JokerId::SteelJoker));
    }

    // Tests for persistence functionality would require creating mock jokers
    // These are integration tests that would be better placed in a separate test file
    // with access to actual joker implementations

    #[test]
    fn test_versioned_save_data_serialization() {
        let mut joker_states = HashMap::new();
        joker_states.insert(
            JokerId::Joker,
            serde_json::json!({"accumulated_value": 10.0}),
        );

        let save_data = VersionedSaveData {
            version: 1,
            timestamp: 1234567890,
            joker_states,
            metadata: SaveMetadata {
                total_jokers: 1,
                game_round: 5,
                game_ante: 2,
            },
        };

        // Test serialization/deserialization
        let serialized = serde_json::to_string(&save_data).unwrap();
        let deserialized: VersionedSaveData = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized.version, 1);
        assert_eq!(deserialized.timestamp, 1234567890);
        assert_eq!(deserialized.metadata.total_jokers, 1);
        assert_eq!(deserialized.metadata.game_round, 5);
        assert_eq!(deserialized.metadata.game_ante, 2);
    }

    #[test]
    fn test_state_backup_and_restore() {
        let manager = Arc::new(JokerStateManager::new());
        let persistence = JokerPersistenceManager::new(manager.clone());

        // Add some state
        manager.add_accumulated_value(JokerId::Joker, 100.0);
        manager.add_accumulated_value(JokerId::GreedyJoker, 200.0);

        // Create backup
        let backup = persistence.create_backup();
        assert_eq!(backup.states.len(), 2);
        assert!(backup.timestamp > 0);

        // Modify state
        manager.add_accumulated_value(JokerId::Joker, 50.0);
        assert_eq!(manager.get_accumulated_value(JokerId::Joker), Some(150.0));

        // Restore from backup
        persistence.restore_from_backup(&backup);
        assert_eq!(manager.get_accumulated_value(JokerId::Joker), Some(100.0));
        assert_eq!(
            manager.get_accumulated_value(JokerId::GreedyJoker),
            Some(200.0)
        );
    }
}
