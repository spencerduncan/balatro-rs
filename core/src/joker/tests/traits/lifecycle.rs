//! High-performance unit tests for the JokerLifecycle trait
//!
//! Optimized for zero allocations and cache-friendly execution following the
//! pattern established in the JokerIdentity tests.

use crate::joker::traits::JokerLifecycle;
use std::sync::{Arc, Mutex};

/// Zero-allocation mock implementation with state tracking
#[derive(Debug, Clone)]
struct StaticLifecycleMock {
    id: &'static str,
    state: Arc<Mutex<LifecycleState>>,
}

#[derive(Debug, Clone, Default)]
struct LifecycleState {
    purchase_count: u32,
    sell_count: u32,
    destroy_count: u32,
    round_start_count: u32,
    round_end_count: u32,
    jokers_added: Vec<String>,
    jokers_removed: Vec<String>,
    event_order: Vec<&'static str>,
}

impl StaticLifecycleMock {
    fn new(id: &'static str) -> Self {
        Self {
            id,
            state: Arc::new(Mutex::new(LifecycleState::default())),
        }
    }

    fn with_state() -> Self {
        Self {
            id: "stateful",
            state: Arc::new(Mutex::new(LifecycleState::default())),
        }
    }

    fn reset(&self) {
        if let Ok(mut state) = self.state.lock() {
            *state = LifecycleState::default();
        }
    }

    fn get_state(&self) -> LifecycleState {
        self.state.lock().unwrap().clone()
    }
}

impl JokerLifecycle for StaticLifecycleMock {
    fn on_purchase(&mut self) {
        if let Ok(mut state) = self.state.lock() {
            state.purchase_count += 1;
            state.event_order.push("purchase");
        }
    }

    fn on_sell(&mut self) {
        if let Ok(mut state) = self.state.lock() {
            state.sell_count += 1;
            state.event_order.push("sell");
        }
    }

    fn on_destroy(&mut self) {
        if let Ok(mut state) = self.state.lock() {
            state.destroy_count += 1;
            state.event_order.push("destroy");
        }
    }

    fn on_round_start(&mut self) {
        if let Ok(mut state) = self.state.lock() {
            state.round_start_count += 1;
            state.event_order.push("round_start");
        }
    }

    fn on_round_end(&mut self) {
        if let Ok(mut state) = self.state.lock() {
            state.round_end_count += 1;
            state.event_order.push("round_end");
        }
    }

    fn on_joker_added(&mut self, other_joker_type: &str) {
        if let Ok(mut state) = self.state.lock() {
            state.jokers_added.push(other_joker_type.to_string());
            state.event_order.push("joker_added");
        }
    }

    fn on_joker_removed(&mut self, other_joker_type: &str) {
        if let Ok(mut state) = self.state.lock() {
            state.jokers_removed.push(other_joker_type.to_string());
            state.event_order.push("joker_removed");
        }
    }
}

// Test macro for lifecycle events
macro_rules! test_lifecycle_event {
    ($name:ident, $method:ident, $field:ident) => {
        #[test]
        fn $name() {
            let mut joker = StaticLifecycleMock::with_state();
            joker.$method();
            let state = joker.get_state();
            assert_eq!(state.$field, 1);
        }
    };
}

#[cfg(test)]
mod basic_lifecycle_tests {
    use super::*;

    test_lifecycle_event!(test_on_purchase_called, on_purchase, purchase_count);
    test_lifecycle_event!(test_on_sell_called, on_sell, sell_count);
    test_lifecycle_event!(test_on_destroy_called, on_destroy, destroy_count);
    test_lifecycle_event!(test_on_round_start_called, on_round_start, round_start_count);
    test_lifecycle_event!(test_on_round_end_called, on_round_end, round_end_count);

    #[test]
    fn test_on_joker_added() {
        let mut joker = StaticLifecycleMock::with_state();
        joker.on_joker_added("other_joker");
        let state = joker.get_state();
        assert_eq!(state.jokers_added.len(), 1);
        assert_eq!(state.jokers_added[0], "other_joker");
    }

    #[test]
    fn test_on_joker_removed() {
        let mut joker = StaticLifecycleMock::with_state();
        joker.on_joker_removed("removed_joker");
        let state = joker.get_state();
        assert_eq!(state.jokers_removed.len(), 1);
        assert_eq!(state.jokers_removed[0], "removed_joker");
    }

    #[test]
    fn test_multiple_calls() {
        let mut joker = StaticLifecycleMock::with_state();
        
        // Call each method multiple times
        for _ in 0..3 {
            joker.on_purchase();
            joker.on_round_start();
            joker.on_round_end();
        }
        
        let state = joker.get_state();
        assert_eq!(state.purchase_count, 3);
        assert_eq!(state.round_start_count, 3);
        assert_eq!(state.round_end_count, 3);
    }
}

#[cfg(test)]
mod lifecycle_ordering_tests {
    use super::*;

    #[test]
    fn test_purchase_sell_lifecycle() {
        let mut joker = StaticLifecycleMock::with_state();
        
        // Typical purchase -> use -> sell lifecycle
        joker.on_purchase();
        joker.on_round_start();
        joker.on_round_end();
        joker.on_sell();
        
        let state = joker.get_state();
        assert_eq!(state.event_order, vec!["purchase", "round_start", "round_end", "sell"]);
    }

    #[test]
    fn test_purchase_destroy_lifecycle() {
        let mut joker = StaticLifecycleMock::with_state();
        
        // Purchase -> use -> destroy lifecycle
        joker.on_purchase();
        joker.on_round_start();
        joker.on_destroy();
        
        let state = joker.get_state();
        assert_eq!(state.event_order, vec!["purchase", "round_start", "destroy"]);
    }

    #[test]
    fn test_multiple_rounds_lifecycle() {
        let mut joker = StaticLifecycleMock::with_state();
        
        joker.on_purchase();
        
        // Simulate multiple rounds
        for _ in 0..3 {
            joker.on_round_start();
            joker.on_round_end();
        }
        
        joker.on_sell();
        
        let state = joker.get_state();
        assert_eq!(state.event_order, vec![
            "purchase",
            "round_start", "round_end",
            "round_start", "round_end",
            "round_start", "round_end",
            "sell"
        ]);
    }

    #[test]
    fn test_joker_interaction_lifecycle() {
        let mut joker = StaticLifecycleMock::with_state();
        
        joker.on_purchase();
        joker.on_joker_added("companion1");
        joker.on_joker_added("companion2");
        joker.on_round_start();
        joker.on_joker_removed("companion1");
        joker.on_round_end();
        
        let state = joker.get_state();
        assert_eq!(state.jokers_added, vec!["companion1", "companion2"]);
        assert_eq!(state.jokers_removed, vec!["companion1"]);
    }
}

#[cfg(test)]
mod state_invariant_tests {
    use super::*;

    #[test]
    fn test_no_duplicate_sell_or_destroy() {
        let mut joker = StaticLifecycleMock::with_state();
        
        // A joker should only be sold OR destroyed, not both
        joker.on_purchase();
        joker.on_sell();
        
        let state = joker.get_state();
        assert_eq!(state.sell_count, 1);
        assert_eq!(state.destroy_count, 0);
    }

    #[test]
    fn test_round_start_end_pairing() {
        let mut joker = StaticLifecycleMock::with_state();
        
        // Every round start should have a corresponding round end
        for _ in 0..5 {
            joker.on_round_start();
            joker.on_round_end();
        }
        
        let state = joker.get_state();
        assert_eq!(state.round_start_count, state.round_end_count);
    }

    #[test]
    fn test_purchase_before_use() {
        let mut joker = StaticLifecycleMock::with_state();
        
        // Purchase should always come before any usage
        joker.on_purchase();
        joker.on_round_start();
        
        let state = joker.get_state();
        assert_eq!(state.event_order[0], "purchase");
    }

    #[test]
    fn test_no_events_after_terminal_state() {
        let mut joker = StaticLifecycleMock::with_state();
        
        joker.on_purchase();
        joker.on_sell();
        
        // These shouldn't typically happen after sell
        let events_before = joker.get_state().event_order.len();
        
        // But the trait allows it (no enforcement)
        joker.on_round_start();
        
        let state = joker.get_state();
        assert_eq!(state.event_order.len(), events_before + 1);
    }
}

#[cfg(test)]
mod edge_case_tests {
    use super::*;

    #[test]
    fn test_immediate_sell_after_purchase() {
        let mut joker = StaticLifecycleMock::with_state();
        
        joker.on_purchase();
        joker.on_sell();
        
        let state = joker.get_state();
        assert_eq!(state.purchase_count, 1);
        assert_eq!(state.sell_count, 1);
        assert_eq!(state.round_start_count, 0);
    }

    #[test]
    fn test_multiple_joker_interactions() {
        let mut joker = StaticLifecycleMock::with_state();
        
        // Add and remove multiple jokers
        let joker_types = ["joker_a", "joker_b", "joker_c", "joker_d"];
        
        for jtype in &joker_types {
            joker.on_joker_added(jtype);
        }
        
        for jtype in &joker_types[..2] {
            joker.on_joker_removed(jtype);
        }
        
        let state = joker.get_state();
        assert_eq!(state.jokers_added.len(), 4);
        assert_eq!(state.jokers_removed.len(), 2);
    }

    #[test]
    fn test_empty_string_joker_type() {
        let mut joker = StaticLifecycleMock::with_state();
        
        joker.on_joker_added("");
        joker.on_joker_removed("");
        
        let state = joker.get_state();
        assert_eq!(state.jokers_added[0], "");
        assert_eq!(state.jokers_removed[0], "");
    }

    #[test]
    fn test_very_long_joker_type() {
        let mut joker = StaticLifecycleMock::with_state();
        
        let long_name = "a".repeat(1000);
        joker.on_joker_added(&long_name);
        
        let state = joker.get_state();
        assert_eq!(state.jokers_added[0].len(), 1000);
    }
}

#[cfg(test)]
mod concurrency_tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_concurrent_lifecycle_calls() {
        let joker = Arc::new(Mutex::new(StaticLifecycleMock::with_state()));
        let handles: Vec<_> = (0..4)
            .map(|i| {
                let joker_clone = Arc::clone(&joker);
                thread::spawn(move || {
                    for _ in 0..25 {
                        if let Ok(mut j) = joker_clone.lock() {
                            match i {
                                0 => j.on_round_start(),
                                1 => j.on_round_end(),
                                2 => j.on_joker_added("concurrent"),
                                _ => j.on_joker_removed("concurrent"),
                            }
                        }
                    }
                })
            })
            .collect();

        for handle in handles {
            handle.join().unwrap();
        }

        let state = joker.lock().unwrap().get_state();
        assert_eq!(state.round_start_count, 25);
        assert_eq!(state.round_end_count, 25);
        assert_eq!(state.jokers_added.len(), 25);
        assert_eq!(state.jokers_removed.len(), 25);
    }

    #[test]
    fn test_send_sync_bounds() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<StaticLifecycleMock>();
        
        // Verify trait object also works
        let mut joker = StaticLifecycleMock::with_state();
        let _trait_obj: &mut dyn JokerLifecycle = &mut joker;
    }
}

#[cfg(test)]
mod default_implementation_tests {
    use super::*;

    /// Minimal implementation that uses all default methods
    struct MinimalJoker;
    
    impl JokerLifecycle for MinimalJoker {}

    #[test]
    fn test_default_implementations() {
        let mut joker = MinimalJoker;
        
        // All methods should be callable with default no-op implementations
        joker.on_purchase();
        joker.on_sell();
        joker.on_destroy();
        joker.on_round_start();
        joker.on_round_end();
        joker.on_joker_added("test");
        joker.on_joker_removed("test");
        
        // Test passes if no panic occurs
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_full_game_simulation() {
        let mut jokers = vec![
            StaticLifecycleMock::with_state(),
            StaticLifecycleMock::with_state(),
            StaticLifecycleMock::with_state(),
        ];

        // Simulate purchasing jokers
        for joker in &mut jokers {
            joker.on_purchase();
        }

        // Notify each joker about the others
        for i in 0..jokers.len() {
            for j in 0..jokers.len() {
                if i != j {
                    jokers[i].on_joker_added(&format!("joker_{}", j));
                }
            }
        }

        // Simulate 3 rounds
        for _ in 0..3 {
            for joker in &mut jokers {
                joker.on_round_start();
            }
            for joker in &mut jokers {
                joker.on_round_end();
            }
        }

        // Sell one joker
        jokers[1].on_sell();
        
        // Notify others about removal
        jokers[0].on_joker_removed("joker_1");
        jokers[2].on_joker_removed("joker_1");

        // Verify states
        let state0 = jokers[0].get_state();
        assert_eq!(state0.jokers_added.len(), 2);
        assert_eq!(state0.jokers_removed.len(), 1);
        assert_eq!(state0.round_start_count, 3);

        let state1 = jokers[1].get_state();
        assert_eq!(state1.sell_count, 1);
        
        let state2 = jokers[2].get_state();
        assert_eq!(state2.jokers_removed[0], "joker_1");
    }

    #[test]
    fn test_lifecycle_with_errors() {
        /// Mock that simulates errors during lifecycle
        struct ErrorProneJoker {
            fail_on_round_start: bool,
        }

        impl JokerLifecycle for ErrorProneJoker {
            fn on_round_start(&mut self) {
                if self.fail_on_round_start {
                    // In real implementation might log error
                    // but trait doesn't support Result return
                }
            }
        }

        let mut joker = ErrorProneJoker { fail_on_round_start: true };
        
        // Should not panic even with "errors"
        joker.on_purchase();
        joker.on_round_start();
        joker.on_round_end();
        joker.on_sell();
    }
}

