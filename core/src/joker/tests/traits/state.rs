//! Comprehensive unit tests for the JokerState trait
//!
//! Tests all JokerState trait methods, state transitions, serialization/deserialization,
//! property-based testing, edge cases, and invalid states.

use crate::joker::traits::JokerState;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;

/// Mock implementation with simple state tracking
#[derive(Debug, Clone)]
struct SimpleMockJoker {
    internal_state: Option<Value>,
    reset_count: u32,
}

impl SimpleMockJoker {
    const fn new() -> Self {
        Self {
            internal_state: None,
            reset_count: 0,
        }
    }

    fn with_state(state: Value) -> Self {
        Self {
            internal_state: Some(state),
            reset_count: 0,
        }
    }
}

impl JokerState for SimpleMockJoker {
    fn has_state(&self) -> bool {
        self.internal_state.is_some()
    }

    fn serialize_state(&self) -> Option<Value> {
        self.internal_state.clone()
    }

    fn deserialize_state(&mut self, value: Value) -> Result<(), String> {
        self.internal_state = Some(value);
        Ok(())
    }

    fn debug_state(&self) -> String {
        match &self.internal_state {
            Some(state) => format!("State: {state}"),
            None => "No state".to_string(),
        }
    }

    fn reset_state(&mut self) {
        self.internal_state = None;
        self.reset_count += 1;
    }
}

/// Complex mock with structured state
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ComplexState {
    counter: u32,
    multiplier: f64,
    tags: Vec<String>,
    metadata: HashMap<String, Value>,
}

#[derive(Debug, Clone)]
struct ComplexMockJoker {
    state: Option<ComplexState>,
    validation_enabled: bool,
}

impl ComplexMockJoker {
    fn new() -> Self {
        Self {
            state: None,
            validation_enabled: true,
        }
    }

    fn with_initial_state() -> Self {
        Self {
            state: Some(ComplexState {
                counter: 0,
                multiplier: 1.0,
                tags: vec![],
                metadata: HashMap::new(),
            }),
            validation_enabled: true,
        }
    }
}

impl JokerState for ComplexMockJoker {
    fn has_state(&self) -> bool {
        self.state.is_some()
    }

    fn serialize_state(&self) -> Option<Value> {
        self.state
            .as_ref()
            .and_then(|s| serde_json::to_value(s).ok())
    }

    fn deserialize_state(&mut self, value: Value) -> Result<(), String> {
        if self.validation_enabled {
            // Validate structure
            if !value.is_object() {
                return Err("Expected object value".to_string());
            }

            let obj = value.as_object().unwrap();
            if !obj.contains_key("counter") || !obj.contains_key("multiplier") {
                return Err("Missing required fields".to_string());
            }

            // Validate types
            if !obj["counter"].is_u64() {
                return Err("Counter must be unsigned integer".to_string());
            }
            if !obj["multiplier"].is_f64() {
                return Err("Multiplier must be float".to_string());
            }
        }

        match serde_json::from_value::<ComplexState>(value) {
            Ok(state) => {
                self.state = Some(state);
                Ok(())
            }
            Err(e) => Err(format!("Deserialization failed: {e}")),
        }
    }

    fn debug_state(&self) -> String {
        match &self.state {
            Some(s) => format!(
                "ComplexState {{ counter: {}, multiplier: {}, tags: {:?}, metadata_keys: {} }}",
                s.counter,
                s.multiplier,
                s.tags,
                s.metadata.len()
            ),
            None => "No state".to_string(),
        }
    }

    fn reset_state(&mut self) {
        self.state = None;
    }
}

/// Mock that always fails certain operations
#[derive(Debug, Clone)]
struct FailingMockJoker {
    fail_deserialize: bool,
    fail_serialize: bool,
}

impl FailingMockJoker {
    const fn new_fail_deserialize() -> Self {
        Self {
            fail_deserialize: true,
            fail_serialize: false,
        }
    }

    const fn new_fail_serialize() -> Self {
        Self {
            fail_deserialize: false,
            fail_serialize: true,
        }
    }
}

impl JokerState for FailingMockJoker {
    fn has_state(&self) -> bool {
        true
    }

    fn serialize_state(&self) -> Option<Value> {
        if self.fail_serialize {
            None
        } else {
            Some(json!({"test": "data"}))
        }
    }

    fn deserialize_state(&mut self, _value: Value) -> Result<(), String> {
        if self.fail_deserialize {
            Err("Intentional deserialization failure".to_string())
        } else {
            Ok(())
        }
    }

    fn debug_state(&self) -> String {
        format!(
            "FailingMock {{ fail_deserialize: {}, fail_serialize: {} }}",
            self.fail_deserialize, self.fail_serialize
        )
    }

    fn reset_state(&mut self) {
        // No-op for this mock
    }
}

#[cfg(test)]
mod basic_functionality_tests {
    use super::*;

    #[test]
    fn test_has_state_default() {
        let joker = SimpleMockJoker::new();
        assert!(!joker.has_state());
    }

    #[test]
    fn test_has_state_with_data() {
        let joker = SimpleMockJoker::with_state(json!({"test": true}));
        assert!(joker.has_state());
    }

    #[test]
    fn test_serialize_state_none() {
        let joker = SimpleMockJoker::new();
        assert_eq!(joker.serialize_state(), None);
    }

    #[test]
    fn test_serialize_state_some() {
        let state = json!({"counter": 42, "active": true});
        let joker = SimpleMockJoker::with_state(state.clone());
        assert_eq!(joker.serialize_state(), Some(state));
    }

    #[test]
    fn test_deserialize_state_success() {
        let mut joker = SimpleMockJoker::new();
        let state = json!({"level": 5});

        assert!(joker.deserialize_state(state.clone()).is_ok());
        assert!(joker.has_state());
        assert_eq!(joker.serialize_state(), Some(state));
    }

    #[test]
    fn test_debug_state_formats() {
        let joker1 = SimpleMockJoker::new();
        assert_eq!(joker1.debug_state(), "No state");

        let joker2 = SimpleMockJoker::with_state(json!({"test": 123}));
        assert!(joker2.debug_state().contains("123"));
    }

    #[test]
    fn test_reset_state() {
        let mut joker = SimpleMockJoker::with_state(json!({"data": "exists"}));
        assert!(joker.has_state());

        joker.reset_state();
        assert!(!joker.has_state());
        assert_eq!(joker.reset_count, 1);

        joker.reset_state();
        assert_eq!(joker.reset_count, 2);
    }
}

#[cfg(test)]
mod state_transition_tests {
    use super::*;

    #[test]
    fn test_state_lifecycle() {
        let mut joker = SimpleMockJoker::new();

        // Initial state
        assert!(!joker.has_state());

        // Add state
        joker
            .deserialize_state(json!({"phase": "initialized"}))
            .unwrap();
        assert!(joker.has_state());

        // Update state
        joker
            .deserialize_state(json!({"phase": "active", "count": 1}))
            .unwrap();
        let state = joker.serialize_state().unwrap();
        assert_eq!(state["phase"], "active");

        // Reset state
        joker.reset_state();
        assert!(!joker.has_state());
    }

    #[test]
    fn test_multiple_state_updates() {
        let mut joker = ComplexMockJoker::with_initial_state();

        // Initial state check
        let initial = joker.serialize_state().unwrap();
        assert_eq!(initial["counter"], 0);
        assert_eq!(initial["multiplier"], 1.0);

        // First update
        joker
            .deserialize_state(json!({
                "counter": 5,
                "multiplier": 1.5,
                "tags": ["active"],
                "metadata": {"round": 1}
            }))
            .unwrap();

        let state1 = joker.serialize_state().unwrap();
        assert_eq!(state1["counter"], 5);
        assert_eq!(state1["multiplier"], 1.5);

        // Second update
        joker
            .deserialize_state(json!({
                "counter": 10,
                "multiplier": 2.0,
                "tags": ["active", "boosted"],
                "metadata": {"round": 2, "score": 100}
            }))
            .unwrap();

        let state2 = joker.serialize_state().unwrap();
        assert_eq!(state2["counter"], 10);
        assert_eq!(state2["tags"].as_array().unwrap().len(), 2);
    }

    #[test]
    fn test_state_invariants() {
        let mut joker = ComplexMockJoker::with_initial_state();

        // Deserialize preserves structure
        let original_state = json!({
            "counter": 42,
            "multiplier": std::f64::consts::PI,
            "tags": ["test", "invariant"],
            "metadata": {"key": "value"}
        });

        joker.deserialize_state(original_state.clone()).unwrap();
        let retrieved_state = joker.serialize_state().unwrap();

        assert_eq!(retrieved_state, original_state);
    }
}

#[cfg(test)]
mod serialization_tests {
    use super::*;

    #[test]
    fn test_complex_serialization_roundtrip() {
        let mut joker = ComplexMockJoker::new();

        let complex_state = json!({
            "counter": 999,
            "multiplier": 123.456,
            "tags": ["alpha", "beta", "gamma"],
            "metadata": {
                "nested": {
                    "value": 42,
                    "array": [1, 2, 3]
                },
                "boolean": true,
                "null_value": null
            }
        });

        // Deserialize
        joker.deserialize_state(complex_state.clone()).unwrap();

        // Serialize and compare
        let serialized = joker.serialize_state().unwrap();
        assert_eq!(serialized["counter"], 999);
        assert_eq!(serialized["multiplier"], 123.456);
        assert_eq!(serialized["metadata"]["nested"]["value"], 42);

        // Full equality check
        assert_eq!(serialized, complex_state);
    }

    #[test]
    fn test_unicode_in_state() {
        let mut joker = SimpleMockJoker::new();

        let unicode_state = json!({
            "name": "🃏 Joker 🎭",
            "description": "Uses 🎲 dice and ♠️♥️♣️♦️ suits",
            "symbols": ["♠️", "♥️", "♣️", "♦️"],
            "emoji": "🤡"
        });

        joker.deserialize_state(unicode_state.clone()).unwrap();
        let retrieved = joker.serialize_state().unwrap();

        assert_eq!(retrieved["name"], "🃏 Joker 🎭");
        assert_eq!(retrieved["symbols"][0], "♠️");
    }

    #[test]
    fn test_empty_state_variations() {
        let mut joker = SimpleMockJoker::new();

        // Empty object
        joker.deserialize_state(json!({})).unwrap();
        assert_eq!(joker.serialize_state(), Some(json!({})));

        // Empty array
        joker.deserialize_state(json!([])).unwrap();
        assert_eq!(joker.serialize_state(), Some(json!([])));

        // Null value
        joker.deserialize_state(json!(null)).unwrap();
        assert_eq!(joker.serialize_state(), Some(json!(null)));
    }

    #[test]
    fn test_large_state_handling() {
        let mut joker = SimpleMockJoker::new();

        // Create a large state object
        let mut large_object = serde_json::Map::new();
        for i in 0..1000 {
            large_object.insert(format!("key_{i}"), json!(i));
        }
        let large_state = Value::Object(large_object);

        joker.deserialize_state(large_state.clone()).unwrap();
        let retrieved = joker.serialize_state().unwrap();

        assert_eq!(retrieved.as_object().unwrap().len(), 1000);
    }
}

#[cfg(test)]
mod validation_tests {
    use super::*;

    #[test]
    fn test_validation_missing_fields() {
        let mut joker = ComplexMockJoker::new();

        // Missing counter field
        let invalid1 = json!({
            "multiplier": 1.0,
            "tags": [],
            "metadata": {}
        });

        let result1 = joker.deserialize_state(invalid1);
        assert!(result1.is_err());
        assert!(result1.unwrap_err().contains("Missing required fields"));
    }

    #[test]
    fn test_validation_wrong_types() {
        let mut joker = ComplexMockJoker::new();

        // Wrong type for counter (string instead of number)
        let invalid2 = json!({
            "counter": "not a number",
            "multiplier": 1.0,
            "tags": [],
            "metadata": {}
        });

        let result2 = joker.deserialize_state(invalid2);
        assert!(result2.is_err());
        assert!(result2
            .unwrap_err()
            .contains("Counter must be unsigned integer"));

        // Wrong type for multiplier (integer instead of float)
        let invalid3 = json!({
            "counter": 5,
            "multiplier": "not a float",
            "tags": [],
            "metadata": {}
        });

        let result3 = joker.deserialize_state(invalid3);
        assert!(result3.is_err());
        assert!(result3.unwrap_err().contains("Multiplier must be float"));
    }

    #[test]
    fn test_validation_can_be_disabled() {
        let mut joker = ComplexMockJoker::new();
        joker.validation_enabled = false;

        // This would normally fail validation
        let invalid = json!({
            "counter": "not a number",
            "multiplier": "not a float",
            "tags": "not an array",
            "metadata": "not an object"
        });

        // But should fail at deserialization instead
        let result = joker.deserialize_state(invalid);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Deserialization failed"));
    }
}

#[cfg(test)]
mod error_handling_tests {
    use super::*;

    #[test]
    fn test_deserialize_failure() {
        let mut joker = FailingMockJoker::new_fail_deserialize();

        let result = joker.deserialize_state(json!({"valid": "data"}));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Intentional deserialization failure");
    }

    #[test]
    fn test_serialize_failure() {
        let joker = FailingMockJoker::new_fail_serialize();

        assert!(joker.has_state()); // Claims to have state
        assert_eq!(joker.serialize_state(), None); // But fails to serialize
    }

    #[test]
    fn test_error_recovery() {
        let mut joker = ComplexMockJoker::with_initial_state();

        // Store good state
        let good_state = joker.serialize_state().unwrap();

        // Try to load bad state
        let bad_state = json!({"invalid": "structure"});
        let result = joker.deserialize_state(bad_state);
        assert!(result.is_err());

        // Verify state unchanged after error
        assert_eq!(joker.serialize_state().unwrap(), good_state);
    }
}

#[cfg(test)]
mod property_based_tests {
    use super::*;

    #[test]
    fn test_serialize_deserialize_idempotence() {
        let test_values = vec![
            json!(null),
            json!(true),
            json!(false),
            json!(42),
            json!(std::f64::consts::PI),
            json!("string"),
            json!([1, 2, 3]),
            json!({"key": "value"}),
            json!({"nested": {"deep": {"value": 42}}}),
        ];

        for value in test_values {
            let mut joker = SimpleMockJoker::new();

            // First round
            joker.deserialize_state(value.clone()).unwrap();
            let serialized1 = joker.serialize_state().unwrap();

            // Second round
            joker.deserialize_state(serialized1.clone()).unwrap();
            let serialized2 = joker.serialize_state().unwrap();

            // Should be identical
            assert_eq!(serialized1, serialized2);
            assert_eq!(serialized2, value);
        }
    }

    #[test]
    fn test_reset_state_consistency() {
        let states = vec![json!({"data": 1}), json!({"data": 2}), json!({"data": 3})];

        for state in states {
            let mut joker = SimpleMockJoker::new();

            // Set state
            joker.deserialize_state(state).unwrap();
            assert!(joker.has_state());

            // Reset should always clear state
            joker.reset_state();
            assert!(!joker.has_state());
            assert_eq!(joker.serialize_state(), None);
        }
    }

    #[test]
    fn test_state_independence() {
        let mut joker1 = SimpleMockJoker::new();
        let mut joker2 = SimpleMockJoker::new();

        joker1.deserialize_state(json!({"id": 1})).unwrap();
        joker2.deserialize_state(json!({"id": 2})).unwrap();

        assert_ne!(joker1.serialize_state(), joker2.serialize_state());

        joker1.reset_state();
        assert!(!joker1.has_state());
        assert!(joker2.has_state());
    }
}

#[cfg(test)]
mod edge_case_tests {
    use super::*;

    #[test]
    fn test_extreme_numeric_values() {
        let mut joker = SimpleMockJoker::new();

        // Test normal extreme values first
        let extreme_values = json!({
            "max_i64": i64::MAX,
            "min_i64": i64::MIN,
            "max_u64": u64::MAX,
            "tiny_float": 1e-300,
            "huge_float": 1e300,
        });

        joker.deserialize_state(extreme_values.clone()).unwrap();
        let retrieved = joker.serialize_state().unwrap();

        assert_eq!(retrieved["max_i64"], i64::MAX);
        assert_eq!(retrieved["min_i64"], i64::MIN);
        assert_eq!(retrieved["max_u64"], u64::MAX);
        assert_eq!(retrieved["tiny_float"], 1e-300);
        assert_eq!(retrieved["huge_float"], 1e300);

        // Test special float values separately (JSON doesn't support Infinity/NaN)
        // These should be handled by the application logic if needed
        let special_floats = json!({
            "positive_zero": 0.0_f64,
            "negative_zero": -0.0_f64,
            "very_large": f64::MAX,
            "very_small": f64::MIN_POSITIVE,
        });

        joker.deserialize_state(special_floats.clone()).unwrap();
        let retrieved_special = joker.serialize_state().unwrap();

        assert_eq!(retrieved_special["positive_zero"], 0.0);
        assert_eq!(retrieved_special["negative_zero"], -0.0);
        assert_eq!(retrieved_special["very_large"], f64::MAX);
        assert_eq!(retrieved_special["very_small"], f64::MIN_POSITIVE);
    }

    #[test]
    fn test_deeply_nested_structure() {
        let mut joker = SimpleMockJoker::new();

        // Create deeply nested structure
        let mut nested = json!({"value": 0});
        for i in 1..20 {
            nested = json!({"level": i, "child": nested});
        }

        joker.deserialize_state(nested.clone()).unwrap();
        let retrieved = joker.serialize_state().unwrap();

        // Verify deep nesting preserved
        let mut current = &retrieved;
        for i in (1..20).rev() {
            assert_eq!(current["level"], i);
            current = &current["child"];
        }
        assert_eq!(current["value"], 0);
    }

    #[test]
    fn test_special_json_characters() {
        let mut joker = SimpleMockJoker::new();

        let special_chars = json!({
            "quotes": "Contains \"quotes\"",
            "backslash": "Has \\ backslash",
            "newline": "Line\nbreak",
            "tab": "Tab\there",
            "unicode": "\u{1F3B0}",
            "control": "\u{0001}",
        });

        joker.deserialize_state(special_chars.clone()).unwrap();
        let retrieved = joker.serialize_state().unwrap();

        assert_eq!(retrieved["quotes"], "Contains \"quotes\"");
        assert_eq!(retrieved["backslash"], "Has \\ backslash");
        assert!(retrieved["newline"].as_str().unwrap().contains('\n'));
    }
}

#[cfg(test)]
mod concurrent_access_tests {
    use super::*;

    #[test]
    fn test_send_sync_bounds() {
        fn assert_send_sync<T: Send + Sync>() {}

        assert_send_sync::<SimpleMockJoker>();
        assert_send_sync::<ComplexMockJoker>();
        assert_send_sync::<FailingMockJoker>();
    }

    #[test]
    fn test_trait_object_state_management() {
        let jokers: Vec<Box<dyn JokerState>> = vec![
            Box::new(SimpleMockJoker::new()),
            Box::new(ComplexMockJoker::new()),
            Box::new(FailingMockJoker::new_fail_serialize()),
        ];

        for (i, joker) in jokers.iter().enumerate() {
            match i {
                0 => assert!(!joker.has_state()),
                1 => assert!(!joker.has_state()),
                2 => assert!(joker.has_state()),
                _ => unreachable!(),
            }
        }
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    /// Simulates a game round with state updates
    #[test]
    fn test_game_round_simulation() {
        let mut joker = ComplexMockJoker::new();

        // Initialize for new round
        joker
            .deserialize_state(json!({
                "counter": 0,
                "multiplier": 1.0,
                "tags": ["round_start"],
                "metadata": {"round": 1}
            }))
            .unwrap();

        // Simulate triggers during round
        for i in 1..=5 {
            let current = joker.state.as_ref().unwrap();
            let new_state = json!({
                "counter": current.counter + 1,
                "multiplier": current.multiplier * 1.1,
                "tags": current.tags.clone(),
                "metadata": {
                    "round": 1,
                    "trigger": i
                }
            });
            joker.deserialize_state(new_state).unwrap();
        }

        // Verify final state
        let final_state = joker.serialize_state().unwrap();
        assert_eq!(final_state["counter"], 5);
        assert!(final_state["multiplier"].as_f64().unwrap() > 1.5);

        // End round - reset
        joker.reset_state();
        assert!(!joker.has_state());
    }

    /// Simulates save/load cycle
    #[test]
    fn test_save_load_cycle() {
        // Create joker with game state
        let mut original = ComplexMockJoker::new();
        original
            .deserialize_state(json!({
                "counter": 42,
                "multiplier": 2.5,
                "tags": ["saved", "loaded"],
                "metadata": {
                    "save_version": 1,
                    "timestamp": 1234567890
                }
            }))
            .unwrap();

        // Simulate save
        let saved_state = original.serialize_state().unwrap();
        let saved_json = serde_json::to_string(&saved_state).unwrap();

        // Simulate load into new joker
        let mut loaded = ComplexMockJoker::new();
        let loaded_state: Value = serde_json::from_str(&saved_json).unwrap();
        loaded.deserialize_state(loaded_state).unwrap();

        // Verify states match
        assert_eq!(original.serialize_state(), loaded.serialize_state());
        assert_eq!(original.debug_state(), loaded.debug_state());
    }
}

#[cfg(test)]
mod performance_tests {
    use super::*;

    #[test]
    fn test_large_batch_operations() {
        let mut joker = SimpleMockJoker::new();

        // Perform many state updates
        for i in 0..100 {
            let state = json!({
                "iteration": i,
                "data": format!("test_{}", i)
            });

            joker.deserialize_state(state).unwrap();
            assert!(joker.has_state());

            if i % 10 == 0 {
                joker.reset_state();
                assert!(!joker.has_state());
            }
        }
    }

    #[test]
    fn test_rapid_state_transitions() {
        let mut joker = ComplexMockJoker::with_initial_state();

        let states = vec![
            json!({
                "counter": 1,
                "multiplier": 1.0,
                "tags": ["a"],
                "metadata": {}
            }),
            json!({
                "counter": 2,
                "multiplier": 2.0,
                "tags": ["b"],
                "metadata": {"key": "value"}
            }),
            json!({
                "counter": 3,
                "multiplier": 3.0,
                "tags": ["c"],
                "metadata": {"nested": {"deep": true}}
            }),
        ];

        // Rapid transitions
        for _ in 0..10 {
            for state in &states {
                joker.deserialize_state(state.clone()).unwrap();
            }
        }

        // Final state should be the last one
        let final_state = joker.serialize_state().unwrap();
        assert_eq!(final_state["counter"], 3);
    }
}

#[cfg(test)]
mod coverage_completion_tests {
    use super::*;

    #[test]
    fn test_debug_state_all_variants() {
        // SimpleMockJoker - no state
        let joker1 = SimpleMockJoker::new();
        assert_eq!(joker1.debug_state(), "No state");

        // SimpleMockJoker - with state
        let joker2 = SimpleMockJoker::with_state(json!({"debug": "test"}));
        let debug2 = joker2.debug_state();
        assert!(debug2.contains("State:"));
        assert!(debug2.contains("debug"));

        // ComplexMockJoker - no state
        let joker3 = ComplexMockJoker::new();
        assert_eq!(joker3.debug_state(), "No state");

        // ComplexMockJoker - with state
        let mut joker4 = ComplexMockJoker::new();
        joker4
            .deserialize_state(json!({
                "counter": 99,
                "multiplier": 9.9,
                "tags": ["x", "y", "z"],
                "metadata": {"a": 1, "b": 2}
            }))
            .unwrap();
        let debug4 = joker4.debug_state();
        assert!(debug4.contains("counter: 99"));
        assert!(debug4.contains("multiplier: 9.9"));
        assert!(debug4.contains("[\"x\", \"y\", \"z\"]"));
        assert!(debug4.contains("metadata_keys: 2"));

        // FailingMockJoker
        let joker5 = FailingMockJoker::new_fail_deserialize();
        let debug5 = joker5.debug_state();
        assert!(debug5.contains("fail_deserialize: true"));
        assert!(debug5.contains("fail_serialize: false"));
    }

    #[test]
    fn test_all_error_paths() {
        // Test non-object value for ComplexMockJoker
        let mut joker = ComplexMockJoker::new();
        let result = joker.deserialize_state(json!("not an object"));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Expected object value");

        // Test missing multiplier field
        let result2 = joker.deserialize_state(json!({
            "counter": 1,
            "tags": [],
            "metadata": {}
        }));
        assert!(result2.is_err());
        assert!(result2.unwrap_err().contains("Missing required fields"));

        // Test serialize failure returns None
        let failing = FailingMockJoker::new_fail_serialize();
        assert_eq!(failing.serialize_state(), None);
    }
}
