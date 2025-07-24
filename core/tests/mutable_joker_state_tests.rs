use balatro_rs::card::{Suit, Value};
use balatro_rs::joker::JokerId;
use balatro_rs::joker_state::JokerStateManager;
use balatro_rs::stage::Stage;
use std::sync::{Arc, Mutex};

/// A simple counter joker that needs mutable state
/// This test demonstrates the simplicity we could achieve with &mut self
struct SimpleCounterJoker {
    id: JokerId,
    trigger_count: u32,
    max_triggers: u32,
}

impl SimpleCounterJoker {
    fn new(max_triggers: u32) -> Self {
        Self {
            id: JokerId::Joker, // Using a placeholder ID
            trigger_count: 0,
            max_triggers,
        }
    }
}

// This is what we WANT to write - clean and simple
// Currently this won't compile because process() takes &self
// #[test]
// fn test_simple_mutable_state() {
//     impl JokerGameplay for SimpleCounterJoker {
//         fn process(&mut self, _stage: &Stage, _context: &mut ProcessContext) -> ProcessResult {
//             if self.trigger_count < self.max_triggers {
//                 self.trigger_count += 1;
//                 ProcessResult {
//                     chips_added: 0,
//                     mult_added: 2.0,
//                     retriggered: false,
//                 }
//             } else {
//                 ProcessResult::default()
//             }
//         }
//     }
// }

/// Demonstrates the current workaround needed without mutable state
#[test]
fn test_state_manager_workaround_verbosity() {
    // Create a state manager and context
    let state_manager = Arc::new(JokerStateManager::new());
    let joker_id = JokerId::Joker;

    // Initialize state - verbose!
    // Note: initialize_joker doesn't exist, we just use set_custom_data directly
    state_manager
        .set_custom_data(joker_id, "trigger_count", serde_json::json!(0))
        .unwrap();
    state_manager
        .set_custom_data(joker_id, "max_triggers", serde_json::json!(3))
        .unwrap();

    // Simulate processing - look at all this boilerplate!
    let trigger_count = state_manager
        .get_custom_data::<u32>(joker_id, "trigger_count")
        .ok()
        .flatten()
        .unwrap_or(0);

    let max_triggers = state_manager
        .get_custom_data::<u32>(joker_id, "max_triggers")
        .ok()
        .flatten()
        .unwrap_or(3);

    if trigger_count < max_triggers {
        state_manager
            .set_custom_data(
                joker_id,
                "trigger_count",
                serde_json::json!(trigger_count + 1),
            )
            .unwrap();
    }

    // Compare this to the simple version we want:
    // if self.trigger_count < self.max_triggers {
    //     self.trigger_count += 1;
    // }
}

/// Test that demonstrates type safety issues with current approach
#[test]
fn test_type_safety_issues_with_state_manager() {
    let state_manager = Arc::new(JokerStateManager::new());
    let joker_id = JokerId::Joker;

    // Note: initialize_joker doesn't exist, we just use set_custom_data directly

    // Wrong type stored - compiles but will fail at runtime
    state_manager
        .set_custom_data(joker_id, "counter", serde_json::json!("not a number"))
        .unwrap();

    // This will fail at runtime, not compile time
    let counter = state_manager
        .get_custom_data::<u32>(joker_id, "counter")
        .ok()
        .flatten();

    assert!(counter.is_none()); // Runtime type error!

    // With mutable state, this would be a compile-time error:
    // self.counter = "not a number"; // Won't compile!
}

/// Complex state joker that would benefit from mutable self
struct ComplexStateJoker {
    id: JokerId,
    phase: Phase,
    accumulated_mult: f64,
    cards_seen: Vec<(Value, Suit)>,
    last_trigger_round: Option<u32>,
}

#[derive(Debug, Clone, PartialEq)]
enum Phase {
    Charging,
    Ready,
    Cooldown(u32),
}

#[test]
fn test_complex_state_management_pain() {
    let state_manager = Arc::new(JokerStateManager::new());
    let joker_id = JokerId::Joker;

    // Initialize complex state - look at all this!
    // Note: initialize_joker doesn't exist, we just use set_custom_data directly
    state_manager
        .set_custom_data(joker_id, "phase", serde_json::json!("Charging"))
        .unwrap();
    state_manager
        .set_custom_data(joker_id, "accumulated_mult", serde_json::json!(1.0))
        .unwrap();
    state_manager
        .set_custom_data(joker_id, "cards_seen", serde_json::json!([]))
        .unwrap();
    state_manager
        .set_custom_data(joker_id, "last_trigger_round", serde_json::Value::Null)
        .unwrap();

    // Updating state requires so much code
    let phase_str = state_manager
        .get_custom_data::<String>(joker_id, "phase")
        .ok()
        .flatten()
        .unwrap_or_else(|| "Charging".to_string());

    if phase_str.as_str() == "Charging" {
        let mult = state_manager
            .get_custom_data::<f64>(joker_id, "accumulated_mult")
            .ok()
            .flatten()
            .unwrap_or(1.0);

        state_manager
            .set_custom_data(joker_id, "accumulated_mult", serde_json::json!(mult + 0.5))
            .unwrap();

        if mult >= 3.0 {
            state_manager
                .set_custom_data(joker_id, "phase", serde_json::json!("Ready"))
                .unwrap();
        }
    }

    // Compare to what we want:
    // match self.phase {
    //     Phase::Charging => {
    //         self.accumulated_mult += 0.5;
    //         if self.accumulated_mult >= 3.0 {
    //             self.phase = Phase::Ready;
    //         }
    //     }
    //     _ => {}
    // }
}

/// Test demonstrating thread safety concerns
#[test]
fn test_thread_safety_with_mutable_state() {
    // This test shows that we can still have thread safety with &mut self
    // by using interior mutability patterns when needed

    struct ThreadSafeJoker {
        id: JokerId,
        // Use interior mutability for thread-safe state
        state: Mutex<JokerState>,
    }

    struct JokerState {
        counter: u32,
        active: bool,
    }

    impl ThreadSafeJoker {
        fn new() -> Self {
            Self {
                id: JokerId::Joker,
                state: Mutex::new(JokerState {
                    counter: 0,
                    active: true,
                }),
            }
        }

        // This would work with &mut self in the trait
        fn process_with_mutex(&mut self) -> bool {
            let mut state = self.state.lock().unwrap();
            if state.active {
                state.counter += 1;
                if state.counter >= 10 {
                    state.active = false;
                }
                true
            } else {
                false
            }
        }
    }

    // The joker can still be Send + Sync
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<Arc<Mutex<ThreadSafeJoker>>>();
}

/// Performance test showing overhead of state manager
#[test]
fn test_performance_impact_of_external_state() {
    use std::time::Instant;

    let state_manager = Arc::new(JokerStateManager::new());
    let joker_id = JokerId::Joker;

    // Note: initialize_joker doesn't exist, we just use set_custom_data directly
    state_manager
        .set_custom_data(joker_id, "counter", serde_json::json!(0))
        .unwrap();

    // Measure external state access
    let start = Instant::now();
    for _ in 0..10000 {
        let counter = state_manager
            .get_custom_data::<u32>(joker_id, "counter")
            .ok()
            .flatten()
            .unwrap_or(0);

        state_manager
            .set_custom_data(joker_id, "counter", serde_json::json!(counter + 1))
            .unwrap();
    }
    let external_duration = start.elapsed();

    // Compare with direct field access (simulated)
    struct DirectJoker {
        counter: u32,
    }

    let mut direct = DirectJoker { counter: 0 };
    let start = Instant::now();
    for _ in 0..10000 {
        direct.counter += 1;
    }
    let direct_duration = start.elapsed();

    // External state is significantly slower
    println!("External state: {external_duration:?}");
    println!("Direct field: {direct_duration:?}");
    assert!(external_duration > direct_duration * 10); // At least 10x slower
}
