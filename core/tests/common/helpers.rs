//! Test helper utilities for common testing scenarios
//!
//! Provides utility functions that simplify test setup and execution.
//!

#![allow(clippy::all)]

//! ## Production Engineering Patterns
//! - Test environment configuration for reproducibility
//! - Random seed management for deterministic tests
//! - Setup/teardown functions for resource management
//! - Performance monitoring utilities for benchmarking

use balatro_rs::{action::Action, config::Config, error::GameError, game::Game, stage::Stage};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Executes a sequence of actions on a game and returns the result
pub fn execute_action_sequence(game: &mut Game, actions: Vec<Action>) -> Result<(), GameError> {
    for action in actions {
        game.handle_action(action)?;
    }
    Ok(())
}

/// Plays through a game until it ends or reaches max iterations
pub fn play_until_game_over(game: &mut Game, max_iterations: usize) -> usize {
    let mut iterations = 0;

    while !game.is_over() && iterations < max_iterations {
        let actions = game.gen_actions();
        let actions_vec: Vec<Action> = actions.collect();
        if actions_vec.is_empty() {
            break;
        }

        // Take the first available action (simple AI)
        if let Err(_) = game.handle_action(actions_vec[0].clone()) {
            break;
        }

        iterations += 1;
    }

    iterations
}

/// Measures the time it takes to execute a function
pub fn measure_execution_time<F, R>(f: F) -> (R, Duration)
where
    F: FnOnce() -> R,
{
    let start = Instant::now();
    let result = f();
    let duration = start.elapsed();
    (result, duration)
}

/// Creates a game and immediately performs an action
pub fn game_with_action(action: Action) -> Result<Game, GameError> {
    let mut game = Game::default();
    game.start();
    game.handle_action(action)?;
    Ok(game)
}

// ============================================================================
// PRODUCTION-READY TEST HELPERS FROM PR #779 SALVAGE
// ============================================================================

/// Test environment configuration
/// Production pattern: Centralized test configuration
pub struct TestEnvironment {
    pub seed: u64,
    pub timeout: Duration,
    pub max_iterations: usize,
    pub enable_logging: bool,
    pub performance_tracking: bool,
}

impl Default for TestEnvironment {
    fn default() -> Self {
        Self {
            seed: 42,
            timeout: Duration::from_secs(10),
            max_iterations: 1000,
            enable_logging: false,
            performance_tracking: false,
        }
    }
}

impl TestEnvironment {
    /// Creates a new test environment with custom configuration
    pub fn new(seed: u64) -> Self {
        Self {
            seed,
            ..Default::default()
        }
    }

    /// Creates a performance testing environment
    pub fn performance() -> Self {
        Self {
            performance_tracking: true,
            timeout: Duration::from_secs(60),
            max_iterations: 10000,
            ..Default::default()
        }
    }

    /// Creates a debugging environment with logging
    pub fn debug() -> Self {
        Self {
            enable_logging: true,
            timeout: Duration::from_secs(300),
            ..Default::default()
        }
    }

    /// Creates a game with this environment's configuration
    pub fn create_game(&self) -> Game {
        let config = Config {
            // Note: Seed configuration will be available in future API updates
            // Current limitation: Config struct does not expose seed field
            ..Default::default()
        };
        let mut game = Game::new(config);
        game.start();
        game
    }
}

/// Random seed manager for deterministic testing
/// Production pattern: Reproducible random testing
pub struct SeedManager {
    base_seed: u64,
    counter: u64,
    used_seeds: HashMap<String, u64>,
}

impl SeedManager {
    /// Creates a new seed manager with base seed
    pub fn new(base_seed: u64) -> Self {
        Self {
            base_seed,
            counter: 0,
            used_seeds: HashMap::new(),
        }
    }

    /// Gets a unique seed for a test
    pub fn get_seed(&mut self, test_name: &str) -> u64 {
        if let Some(&seed) = self.used_seeds.get(test_name) {
            seed
        } else {
            self.counter += 1;
            let seed = self.base_seed + self.counter;
            self.used_seeds.insert(test_name.to_string(), seed);
            seed
        }
    }

    /// Gets a sequence of seeds for parameterized tests
    pub fn get_seed_sequence(&mut self, test_name: &str, count: usize) -> Vec<u64> {
        let base = self.get_seed(test_name);
        (0..count).map(|i| base + i as u64 * 1000).collect()
    }
}

/// Test fixture setup and teardown manager
/// Production pattern: Resource lifecycle management
pub struct TestFixture<T> {
    setup: Box<dyn Fn() -> T>,
    teardown: Option<Box<dyn Fn(&mut T)>>,
    resource: Option<T>,
}

impl<T> TestFixture<T> {
    /// Creates a new test fixture with setup function
    pub fn new<F>(setup: F) -> Self
    where
        F: Fn() -> T + 'static,
    {
        Self {
            setup: Box::new(setup),
            teardown: None,
            resource: None,
        }
    }

    /// Adds a teardown function to the fixture
    pub fn with_teardown<F>(mut self, teardown: F) -> Self
    where
        F: Fn(&mut T) + 'static,
    {
        self.teardown = Some(Box::new(teardown));
        self
    }

    /// Sets up the fixture and returns the resource
    pub fn setup(&mut self) -> &T {
        if self.resource.is_none() {
            self.resource = Some((self.setup)());
        }
        self.resource.as_ref().unwrap()
    }

    /// Sets up the fixture and returns mutable reference
    pub fn setup_mut(&mut self) -> &mut T {
        if self.resource.is_none() {
            self.resource = Some((self.setup)());
        }
        self.resource.as_mut().unwrap()
    }

    /// Tears down the fixture
    pub fn teardown(&mut self) {
        if let Some(ref mut resource) = self.resource {
            if let Some(ref teardown) = self.teardown {
                teardown(resource);
            }
        }
        self.resource = None;
    }
}

impl<T> Drop for TestFixture<T> {
    fn drop(&mut self) {
        self.teardown();
    }
}

/// Performance monitor for test execution
/// Production pattern: Performance tracking
pub struct PerformanceMonitor {
    measurements: Arc<Mutex<Vec<PerformanceMeasurement>>>,
    start_time: Option<Instant>,
    test_name: String,
}

#[derive(Debug, Clone)]
pub struct PerformanceMeasurement {
    pub test_name: String,
    pub operation: String,
    pub duration: Duration,
    pub timestamp: Instant,
}

impl PerformanceMonitor {
    /// Creates a new performance monitor
    pub fn new(test_name: &str) -> Self {
        Self {
            measurements: Arc::new(Mutex::new(Vec::new())),
            start_time: None,
            test_name: test_name.to_string(),
        }
    }

    /// Starts measuring an operation
    pub fn start(&mut self) {
        self.start_time = Some(Instant::now());
    }

    /// Stops measuring and records the result
    pub fn stop(&mut self, operation: &str) {
        if let Some(start) = self.start_time {
            let duration = start.elapsed();
            let measurement = PerformanceMeasurement {
                test_name: self.test_name.clone(),
                operation: operation.to_string(),
                duration,
                timestamp: Instant::now(),
            };

            self.measurements.lock().unwrap().push(measurement);
            self.start_time = None;
        }
    }

    /// Measures a function execution
    pub fn measure<F, R>(&mut self, operation: &str, f: F) -> R
    where
        F: FnOnce() -> R,
    {
        self.start();
        let result = f();
        self.stop(operation);
        result
    }

    /// Gets all measurements
    pub fn get_measurements(&self) -> Vec<PerformanceMeasurement> {
        self.measurements.lock().unwrap().clone()
    }

    /// Gets average duration for an operation
    pub fn get_average_duration(&self, operation: &str) -> Option<Duration> {
        let measurements = self.measurements.lock().unwrap();
        let filtered: Vec<_> = measurements
            .iter()
            .filter(|m| m.operation == operation)
            .map(|m| m.duration)
            .collect();

        if filtered.is_empty() {
            None
        } else {
            let sum: Duration = filtered.iter().sum();
            Some(sum / filtered.len() as u32)
        }
    }
}

/// Game state recorder for test debugging
/// Production pattern: State history for debugging
pub struct GameStateRecorder {
    states: Vec<GameStateRecord>,
    max_records: usize,
}

#[derive(Debug, Clone)]
pub struct GameStateRecord {
    pub step: usize,
    pub action: Option<Action>,
    pub ante: u8,
    pub round: u8,
    pub money: i32,
    pub score: i32,
    pub stage: Stage,
    pub timestamp: Instant,
}

impl GameStateRecorder {
    /// Creates a new game state recorder
    pub fn new(max_records: usize) -> Self {
        Self {
            states: Vec::new(),
            max_records,
        }
    }

    /// Records the current game state
    pub fn record(&mut self, game: &Game, step: usize, action: Option<Action>) {
        if self.states.len() >= self.max_records {
            self.states.remove(0); // Remove oldest record
        }

        self.states.push(GameStateRecord {
            step,
            action,
            ante: match game.ante_current {
                balatro_rs::ante::Ante::Zero => 0,
                balatro_rs::ante::Ante::One => 1,
                balatro_rs::ante::Ante::Two => 2,
                balatro_rs::ante::Ante::Three => 3,
                balatro_rs::ante::Ante::Four => 4,
                balatro_rs::ante::Ante::Five => 5,
                balatro_rs::ante::Ante::Six => 6,
                balatro_rs::ante::Ante::Seven => 7,
                balatro_rs::ante::Ante::Eight => 8,
            },
            round: game.round as u8,
            money: game.money as i32,
            score: game.score as i32,
            stage: game.stage.clone(),
            timestamp: Instant::now(),
        });
    }

    /// Gets all recorded states
    pub fn get_history(&self) -> &[GameStateRecord] {
        &self.states
    }

    /// Finds the first state where a condition occurred
    pub fn find_state<F>(&self, condition: F) -> Option<&GameStateRecord>
    where
        F: Fn(&GameStateRecord) -> bool,
    {
        self.states.iter().find(|&state| condition(state))
    }

    /// Prints a debug trace of state history
    pub fn print_trace(&self) {
        println!("Game State History:");
        println!("{:-<80}", "");
        for state in &self.states {
            println!(
                "Step {}: {:?} | Ante: {}, Round: {}, Money: ${}, Score: {}, Stage: {:?}",
                state.step,
                state
                    .action
                    .as_ref()
                    .map(|a| format!("{:?}", a))
                    .unwrap_or_else(|| "None".to_string()),
                state.ante,
                state.round,
                state.money,
                state.score,
                state.stage
            );
        }
        println!("{:-<80}", "");
    }
}

/// Test data validator for checking invariants
/// Production pattern: Invariant validation
pub struct TestValidator {
    invariants: Vec<Box<dyn Fn(&Game) -> Result<(), String>>>,
}

impl TestValidator {
    /// Creates a new test validator
    pub fn new() -> Self {
        Self {
            invariants: Vec::new(),
        }
    }

    /// Adds an invariant check
    pub fn add_invariant<F>(mut self, invariant: F) -> Self
    where
        F: Fn(&Game) -> Result<(), String> + 'static,
    {
        self.invariants.push(Box::new(invariant));
        self
    }

    /// Validates all invariants for a game state
    pub fn validate(&self, game: &Game) -> Result<(), Vec<String>> {
        let errors: Vec<_> = self
            .invariants
            .iter()
            .filter_map(|inv| inv(game).err())
            .collect();

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Creates a standard game validator with common invariants
    pub fn standard() -> Self {
        Self::new()
            .add_invariant(|game| {
                if game.money < 0.0 {
                    Err(format!("Money is negative: {}", game.money))
                } else {
                    Ok(())
                }
            })
            .add_invariant(|game| {
                // Check if ante exceeds maximum
                use balatro_rs::ante::Ante;
                if matches!(game.ante_current, Ante::Eight) {
                    // Already at max ante, next would exceed
                    Err(format!("Ante at maximum: {:?}", game.ante_current))
                } else {
                    Ok(())
                }
            })
            .add_invariant(|game| {
                if game.jokers.len() > 5 {
                    Err(format!("Too many jokers: {}", game.jokers.len()))
                } else {
                    Ok(())
                }
            })
    }
}

/// Batch test runner for parameterized tests
/// Production pattern: Parameterized testing
pub fn run_parameterized_test<T, F>(
    test_name: &str,
    parameters: Vec<T>,
    test_fn: F,
) -> Vec<Result<(), String>>
where
    T: std::fmt::Debug,
    F: Fn(&T) -> Result<(), String>,
{
    println!("Running parameterized test: {}", test_name);

    parameters
        .iter()
        .enumerate()
        .map(|(i, param)| {
            println!("  Test case {}: {:?}", i + 1, param);
            test_fn(param)
        })
        .collect()
}

/// Retry helper for flaky tests
/// Production pattern: Test stability
pub fn retry_test<F>(max_attempts: usize, mut test_fn: F) -> Result<(), String>
where
    F: FnMut() -> Result<(), String>,
{
    for attempt in 1..=max_attempts {
        match test_fn() {
            Ok(()) => return Ok(()),
            Err(e) if attempt == max_attempts => return Err(e),
            Err(e) => {
                println!("Test attempt {} failed: {}. Retrying...", attempt, e);
                std::thread::sleep(Duration::from_millis(100 * attempt as u64));
            }
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_measure_execution_time() {
        let (result, duration) = measure_execution_time(|| {
            let mut sum = 0;
            for i in 0..1000 {
                sum += i;
            }
            sum
        });

        assert_eq!(result, 499500);
        assert!(duration.as_nanos() > 0);
    }

    #[test]
    fn test_test_environment() {
        let env = TestEnvironment::new(123);
        assert_eq!(env.seed, 123);

        let game = env.create_game();
        assert!(!game.is_over());

        let perf_env = TestEnvironment::performance();
        assert!(perf_env.performance_tracking);
        assert_eq!(perf_env.timeout, Duration::from_secs(60));
    }

    #[test]
    fn test_seed_manager() {
        let mut manager = SeedManager::new(1000);

        let seed1 = manager.get_seed("test1");
        let seed2 = manager.get_seed("test2");
        let seed1_again = manager.get_seed("test1");

        assert_ne!(seed1, seed2);
        assert_eq!(seed1, seed1_again);

        let sequence = manager.get_seed_sequence("test3", 3);
        assert_eq!(sequence.len(), 3);
        assert!(sequence[0] < sequence[1]);
        assert!(sequence[1] < sequence[2]);
    }

    #[test]
    fn test_test_fixture() {
        let mut fixture = TestFixture::new(|| Game::default()).with_teardown(|_game| {
            // Cleanup code here
            // Can't directly set ante, it's managed by game progression
            // game.ante_current = Ante::Zero;
        });

        let game = fixture.setup_mut();
        game.start();
        assert!(!game.is_over());

        fixture.teardown();
    }

    #[test]
    fn test_performance_monitor() {
        let mut monitor = PerformanceMonitor::new("test_perf");

        monitor.measure("operation1", || {
            std::thread::sleep(Duration::from_millis(10));
        });

        monitor.measure("operation1", || {
            std::thread::sleep(Duration::from_millis(10));
        });

        let measurements = monitor.get_measurements();
        assert_eq!(measurements.len(), 2);

        let avg = monitor.get_average_duration("operation1");
        assert!(avg.is_some());
    }

    #[test]
    fn test_game_state_recorder() {
        let mut recorder = GameStateRecorder::new(10);
        let game = Game::default();

        recorder.record(&game, 0, None);
        recorder.record(&game, 1, Some(Action::Play()));

        let history = recorder.get_history();
        assert_eq!(history.len(), 2);
        assert_eq!(history[0].step, 0);
        assert_eq!(history[1].step, 1);

        let state = recorder.find_state(|s| s.step == 1);
        assert!(state.is_some());
    }

    #[test]
    fn test_validator() {
        let validator = TestValidator::standard();
        let game = Game::default();

        let result = validator.validate(&game);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parameterized_test() {
        let params = vec![1, 2, 3, 4, 5];
        let results = run_parameterized_test("test_even_numbers", params, |&n| {
            if n % 2 == 0 {
                Ok(())
            } else {
                Err(format!("{} is not even", n))
            }
        });

        assert_eq!(results.len(), 5);
        assert!(results[1].is_ok()); // 2 is even
        assert!(results[0].is_err()); // 1 is odd
    }

    #[test]
    fn test_retry() {
        let mut attempts = 0;
        let result = retry_test(3, || {
            attempts += 1;
            if attempts < 2 {
                Err("Not ready yet".to_string())
            } else {
                Ok(())
            }
        });

        assert!(result.is_ok());
        assert_eq!(attempts, 2);
    }
}
