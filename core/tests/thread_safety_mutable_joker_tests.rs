use balatro_rs::card::{Suit, Value};
use balatro_rs::joker::{JokerGameplay, JokerId, ProcessContext, ProcessResult};
use balatro_rs::joker_state::JokerStateManager;
use balatro_rs::stage::Stage;
use std::sync::{
    atomic::{AtomicU32, Ordering},
    Arc, Mutex, RwLock,
};
use std::thread;

/// Test that jokers with &mut self can still be Send + Sync
/// by using interior mutability patterns when needed
#[test]
fn test_send_sync_with_mutable_state() {
    // Joker with atomic state - lock-free and thread-safe
    struct AtomicJoker {
        id: JokerId,
        trigger_count: AtomicU32,
        max_triggers: u32,
    }

    impl AtomicJoker {
        fn new(max_triggers: u32) -> Self {
            Self {
                id: JokerId::Joker,
                trigger_count: AtomicU32::new(0),
                max_triggers,
            }
        }

        // This would work with &mut self in the trait
        fn process_with_atomic(&mut self) -> ProcessResult {
            let current = self.trigger_count.load(Ordering::SeqCst);
            if current < self.max_triggers {
                self.trigger_count.fetch_add(1, Ordering::SeqCst);
                ProcessResult {
                    chips_added: 0,
                    mult_added: 2.0,
                    mult_multiplier: 1.0,
                    retriggered: false,
                    message: None,
                }
            } else {
                ProcessResult::default()
            }
        }
    }

    // Verify it's Send + Sync
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<AtomicJoker>();

    // Can be shared across threads with Arc<Mutex<_>>
    assert_send_sync::<Arc<Mutex<AtomicJoker>>>();
}

/// Test that complex state can be managed thread-safely
#[test]
fn test_complex_state_thread_safety() {
    // Complex joker with mixed state types
    struct ComplexThreadSafeJoker {
        id: JokerId,
        // Immutable data
        name: String,
        base_mult: f64,
        // Thread-safe mutable data
        trigger_count: AtomicU32,
        // Complex mutable state behind RwLock
        state: RwLock<ComplexState>,
    }

    #[derive(Default)]
    struct ComplexState {
        cards_seen: Vec<(Value, Suit)>,
        accumulated_mult: f64,
        phase: Phase,
    }

    #[derive(Debug, Clone, PartialEq)]
    enum Phase {
        Charging,
        Ready,
        Cooldown(u32),
    }

    impl Default for Phase {
        fn default() -> Self {
            Phase::Charging
        }
    }

    impl ComplexThreadSafeJoker {
        fn new() -> Self {
            Self {
                id: JokerId::Joker,
                name: "Complex Joker".to_string(),
                base_mult: 2.0,
                trigger_count: AtomicU32::new(0),
                state: RwLock::new(ComplexState::default()),
            }
        }

        // This pattern would work with &mut self
        fn process_thread_safe(&mut self) -> ProcessResult {
            // Increment trigger count atomically
            self.trigger_count.fetch_add(1, Ordering::SeqCst);

            // Access complex state with read lock
            let mult = {
                let state = self.state.read().unwrap();
                state.accumulated_mult + self.base_mult
            };

            // Update phase with write lock
            {
                let mut state = self.state.write().unwrap();
                match state.phase {
                    Phase::Charging => {
                        state.accumulated_mult += 0.5;
                        if state.accumulated_mult >= 3.0 {
                            state.phase = Phase::Ready;
                        }
                    }
                    Phase::Ready => {
                        state.phase = Phase::Cooldown(3);
                        state.accumulated_mult = 0.0;
                    }
                    Phase::Cooldown(ref mut turns) => {
                        *turns = turns.saturating_sub(1);
                        if *turns == 0 {
                            state.phase = Phase::Charging;
                        }
                    }
                }
            }

            ProcessResult {
                chips_added: 0,
                mult_added: mult,
                mult_multiplier: 1.0,
                retriggered: false,
                message: None,
            }
        }
    }

    // Verify it's Send + Sync
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<ComplexThreadSafeJoker>();
}

/// Test concurrent access patterns with mutable jokers
#[test]
fn test_concurrent_joker_access() {
    // Shared joker with thread-safe state
    struct ConcurrentJoker {
        id: JokerId,
        // Use atomics for simple counters
        process_count: AtomicU32,
        success_count: AtomicU32,
    }

    impl ConcurrentJoker {
        fn new() -> Self {
            Self {
                id: JokerId::Joker,
                process_count: AtomicU32::new(0),
                success_count: AtomicU32::new(0),
            }
        }

        fn process_concurrent(&mut self) -> bool {
            self.process_count.fetch_add(1, Ordering::SeqCst);

            // Simulate some condition
            let count = self.process_count.load(Ordering::SeqCst);
            if count % 2 == 0 {
                self.success_count.fetch_add(1, Ordering::SeqCst);
                true
            } else {
                false
            }
        }
    }

    // Test concurrent access
    let joker = Arc::new(Mutex::new(ConcurrentJoker::new()));
    let mut handles = vec![];

    // Spawn multiple threads
    for _ in 0..10 {
        let joker_clone = Arc::clone(&joker);
        let handle = thread::spawn(move || {
            let mut successes = 0;
            for _ in 0..100 {
                let mut joker = joker_clone.lock().unwrap();
                if joker.process_concurrent() {
                    successes += 1;
                }
            }
            successes
        });
        handles.push(handle);
    }

    // Wait for all threads
    let total_successes: u32 = handles.into_iter().map(|h| h.join().unwrap()).sum();

    // Verify results
    let joker = joker.lock().unwrap();
    assert_eq!(joker.process_count.load(Ordering::SeqCst), 1000);
    assert_eq!(joker.success_count.load(Ordering::SeqCst), total_successes);
}

/// Test that Box<dyn JokerGameplay> would work with mutable trait
#[test]
fn test_trait_object_thread_safety() {
    // Mock trait that simulates JokerGameplay with &mut self
    trait MutableJokerGameplay: Send + Sync {
        fn process_mut(&mut self) -> ProcessResult;
    }

    // Simple implementation
    struct SimpleJoker {
        counter: u32,
    }

    impl MutableJokerGameplay for SimpleJoker {
        fn process_mut(&mut self) -> ProcessResult {
            self.counter += 1;
            ProcessResult {
                chips_added: self.counter as u64,
                mult_added: 1.0,
                mult_multiplier: 1.0,
                retriggered: false,
                message: None,
            }
        }
    }

    // Complex implementation with interior mutability
    struct ComplexJoker {
        state: Mutex<u32>,
    }

    impl MutableJokerGameplay for ComplexJoker {
        fn process_mut(&mut self) -> ProcessResult {
            let mut state = self.state.lock().unwrap();
            *state += 2;
            ProcessResult {
                chips_added: *state as u64,
                mult_added: 2.0,
                mult_multiplier: 1.0,
                retriggered: false,
                message: None,
            }
        }
    }

    // Test trait objects
    let jokers: Vec<Box<dyn MutableJokerGameplay>> = vec![
        Box::new(SimpleJoker { counter: 0 }),
        Box::new(ComplexJoker {
            state: Mutex::new(0),
        }),
    ];

    // Can wrap in Arc<Mutex<_>> for thread sharing
    let shared_jokers = Arc::new(Mutex::new(jokers));

    // Verify Send + Sync
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<Arc<Mutex<Vec<Box<dyn MutableJokerGameplay>>>>>();
}

/// Test migration path from immutable to mutable
#[test]
fn test_migration_compatibility() {
    // Current pattern with external state
    struct LegacyJoker {
        id: JokerId,
    }

    impl LegacyJoker {
        fn process_legacy(&self, state_manager: &JokerStateManager) -> ProcessResult {
            // Verbose state access
            let counter = state_manager
                .get_custom_data::<u32>(self.id, "counter")
                .ok()
                .flatten()
                .unwrap_or(0);

            let _ =
                state_manager.set_custom_data(self.id, "counter", serde_json::json!(counter + 1));

            ProcessResult::default()
        }
    }

    // New pattern with internal state
    struct ModernJoker {
        id: JokerId,
        counter: u32,
    }

    impl ModernJoker {
        fn process_modern(&mut self) -> ProcessResult {
            // Simple and direct
            self.counter += 1;
            ProcessResult::default()
        }
    }

    // Both can coexist during migration
    // Legacy jokers continue using state manager
    // New jokers use internal state
}

/// Benchmark thread safety overhead
#[test]
fn test_thread_safety_performance() {
    use std::time::Instant;

    // Direct mutable access
    struct DirectJoker {
        counter: u32,
    }

    impl DirectJoker {
        fn process(&mut self) {
            self.counter += 1;
        }
    }

    // Atomic access (lock-free)
    struct AtomicJoker {
        counter: AtomicU32,
    }

    impl AtomicJoker {
        fn process(&mut self) {
            self.counter.fetch_add(1, Ordering::Relaxed);
        }
    }

    // Mutex access (locking)
    struct MutexJoker {
        counter: Mutex<u32>,
    }

    impl MutexJoker {
        fn process(&mut self) {
            let mut counter = self.counter.lock().unwrap();
            *counter += 1;
        }
    }

    const ITERATIONS: u32 = 100_000;

    // Benchmark direct access
    let mut direct = DirectJoker { counter: 0 };
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        direct.process();
    }
    let direct_time = start.elapsed();

    // Benchmark atomic access
    let mut atomic = AtomicJoker {
        counter: AtomicU32::new(0),
    };
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        atomic.process();
    }
    let atomic_time = start.elapsed();

    // Benchmark mutex access
    let mut mutex = MutexJoker {
        counter: Mutex::new(0),
    };
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        mutex.process();
    }
    let mutex_time = start.elapsed();

    println!("Performance comparison ({} iterations):", ITERATIONS);
    println!("Direct access: {:?}", direct_time);
    println!(
        "Atomic access: {:?} ({}x slower)",
        atomic_time,
        atomic_time.as_nanos() / direct_time.as_nanos()
    );
    println!(
        "Mutex access: {:?} ({}x slower)",
        mutex_time,
        mutex_time.as_nanos() / direct_time.as_nanos()
    );

    // Atomics should be within 2-3x of direct access
    assert!(atomic_time.as_nanos() < direct_time.as_nanos() * 5);

    // Mutex is slower but still reasonable for uncontended access
    assert!(mutex_time.as_nanos() < direct_time.as_nanos() * 20);
}
