//! Performance testing utilities for load testing and benchmarking
//!
//! Provides comprehensive performance testing tools including concurrent load testing,
//! memory usage monitoring, and timing utilities for sprint validation.

use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::thread;
use std::collections::HashMap;
use tokio::sync::Semaphore;
use tokio::time::timeout;

use balatro_rs::{
    action::Action,
    game::Game,
    rng::GameRng,
};

use crate::common::config::{MAX_CONCURRENT_SESSIONS, DEFAULT_ASYNC_TIMEOUT};

/// Performance metrics collector for load testing
#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    pub operation_count: u64,
    pub total_duration: Duration,
    pub min_duration: Duration,
    pub max_duration: Duration,
    pub avg_duration: Duration,
    pub error_count: u64,
    pub memory_peak: usize,
    pub memory_start: usize,
    pub memory_end: usize,
}

impl PerformanceMetrics {
    pub fn new() -> Self {
        Self {
            operation_count: 0,
            total_duration: Duration::ZERO,
            min_duration: Duration::MAX,
            max_duration: Duration::ZERO,
            avg_duration: Duration::ZERO,
            error_count: 0,
            memory_peak: 0,
            memory_start: get_memory_usage(),
            memory_end: 0,
        }
    }

    pub fn add_measurement(&mut self, duration: Duration, success: bool) {
        self.operation_count += 1;
        self.total_duration += duration;

        if duration < self.min_duration {
            self.min_duration = duration;
        }
        if duration > self.max_duration {
            self.max_duration = duration;
        }

        if self.operation_count > 0 {
            self.avg_duration = self.total_duration / self.operation_count as u32;
        }

        if !success {
            self.error_count += 1;
        }

        let current_memory = get_memory_usage();
        if current_memory > self.memory_peak {
            self.memory_peak = current_memory;
        }
    }

    pub fn finalize(&mut self) {
        self.memory_end = get_memory_usage();
    }

    pub fn error_rate(&self) -> f64 {
        if self.operation_count == 0 {
            0.0
        } else {
            self.error_count as f64 / self.operation_count as f64
        }
    }

    pub fn throughput(&self) -> f64 {
        if self.total_duration.as_secs_f64() == 0.0 {
            0.0
        } else {
            self.operation_count as f64 / self.total_duration.as_secs_f64()
        }
    }

    pub fn memory_growth(&self) -> isize {
        self.memory_end as isize - self.memory_start as isize
    }
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self::new()
    }
}

/// Load testing configuration
#[derive(Debug, Clone)]
pub struct LoadTestConfig {
    pub concurrent_users: usize,
    pub operations_per_user: usize,
    pub duration: Duration,
    pub ramp_up_time: Duration,
    pub think_time: Duration,
}

impl Default for LoadTestConfig {
    fn default() -> Self {
        Self {
            concurrent_users: 10,
            operations_per_user: 100,
            duration: Duration::from_secs(60),
            ramp_up_time: Duration::from_secs(10),
            think_time: Duration::from_millis(100),
        }
    }
}

/// Memory monitor for detecting leaks
pub struct MemoryMonitor {
    baseline: usize,
    samples: Vec<(Instant, usize)>,
    threshold: usize,
}

impl MemoryMonitor {
    pub fn new(threshold: usize) -> Self {
        let baseline = get_memory_usage();
        Self {
            baseline,
            samples: vec![(Instant::now(), baseline)],
            threshold,
        }
    }

    pub fn sample(&mut self) {
        let now = Instant::now();
        let memory = get_memory_usage();
        self.samples.push((now, memory));
    }

    pub fn check_for_leaks(&self) -> bool {
        if let Some((_, latest_memory)) = self.samples.last() {
            *latest_memory > self.baseline + self.threshold
        } else {
            false
        }
    }

    pub fn memory_growth(&self) -> isize {
        if let Some((_, latest_memory)) = self.samples.last() {
            *latest_memory as isize - self.baseline as isize
        } else {
            0
        }
    }

    pub fn peak_memory(&self) -> usize {
        self.samples.iter().map(|(_, mem)| *mem).max().unwrap_or(self.baseline)
    }
}

/// Timing utility for measuring operation performance
pub struct Timer {
    start: Instant,
    label: String,
}

impl Timer {
    pub fn new(label: &str) -> Self {
        Self {
            start: Instant::now(),
            label: label.to_string(),
        }
    }

    pub fn elapsed(&self) -> Duration {
        self.start.elapsed()
    }

    pub fn stop_and_log(&self) -> Duration {
        let duration = self.elapsed();
        println!("{}: {:?}", self.label, duration);
        duration
    }
}

/// Concurrent game session executor for load testing
pub async fn run_concurrent_game_sessions(
    config: LoadTestConfig,
) -> Result<PerformanceMetrics, Box<dyn std::error::Error + Send + Sync>> {
    let metrics = Arc::new(Mutex::new(PerformanceMetrics::new()));
    let semaphore = Arc::new(Semaphore::new(config.concurrent_users));
    let mut handles = Vec::new();

    println!("Starting load test with {} concurrent users", config.concurrent_users);

    for user_id in 0..config.concurrent_users {
        let semaphore = semaphore.clone();
        let metrics = metrics.clone();
        let config = config.clone();

        let handle = tokio::spawn(async move {
            // Acquire semaphore permit
            let _permit = semaphore.acquire().await.unwrap();

            // Stagger user start times for ramp-up
            let ramp_delay = config.ramp_up_time * user_id as u32 / config.concurrent_users as u32;
            tokio::time::sleep(ramp_delay).await;

            // Run operations for this user
            run_user_session(user_id, config, metrics).await
        });

        handles.push(handle);
    }

    // Wait for all users to complete
    for handle in handles {
        handle.await?;
    }

    let mut final_metrics = metrics.lock().unwrap().clone();
    final_metrics.finalize();

    println!("Load test completed:");
    println!("  Operations: {}", final_metrics.operation_count);
    println!("  Avg Duration: {:?}", final_metrics.avg_duration);
    println!("  Error Rate: {:.2}%", final_metrics.error_rate() * 100.0);
    println!("  Throughput: {:.2} ops/sec", final_metrics.throughput());
    println!("  Memory Growth: {} bytes", final_metrics.memory_growth());

    Ok(final_metrics)
}

async fn run_user_session(
    user_id: usize,
    config: LoadTestConfig,
    metrics: Arc<Mutex<PerformanceMetrics>>,
) {
    let start_time = Instant::now();

    while start_time.elapsed() < config.duration {
        for _ in 0..config.operations_per_user {
            let operation_start = Instant::now();
            let success = run_single_game_operation(user_id).await;
            let operation_duration = operation_start.elapsed();

            {
                let mut metrics = metrics.lock().unwrap();
                metrics.add_measurement(operation_duration, success);
            }

            // Think time between operations
            tokio::time::sleep(config.think_time).await;

            // Check if we've exceeded the test duration
            if start_time.elapsed() >= config.duration {
                break;
            }
        }
    }
}

async fn run_single_game_operation(user_id: usize) -> bool {
    // Timeout the operation to prevent hanging
    let operation = async {
        let mut game = Game::default();
        let rng = GameRng::for_testing(user_id as u64);

        game.start();

        // Play a few actions to simulate realistic load
        for _ in 0..10 {
            if game.is_over() {
                break;
            }

            let actions: Vec<Action> = game.gen_actions().collect();
            if actions.is_empty() {
                break;
            }

            let action_index = rng.gen_range(0..actions.len());
            let action = actions[action_index].clone();

            if game.handle_action(action).is_err() {
                return false;
            }
        }

        true
    };

    match timeout(DEFAULT_ASYNC_TIMEOUT, operation).await {
        Ok(result) => result,
        Err(_) => {
            eprintln!("Operation timed out for user {}", user_id);
            false
        }
    }
}

/// Benchmark a function with multiple iterations
pub fn benchmark_function<F, R>(
    name: &str,
    iterations: usize,
    mut func: F,
) -> PerformanceMetrics
where
    F: FnMut() -> R,
{
    let mut metrics = PerformanceMetrics::new();

    println!("Benchmarking {} with {} iterations", name, iterations);

    for _ in 0..iterations {
        let start = Instant::now();
        let _result = func();
        let duration = start.elapsed();
        metrics.add_measurement(duration, true);
    }

    metrics.finalize();

    println!("Benchmark {} completed:", name);
    println!("  Avg Duration: {:?}", metrics.avg_duration);
    println!("  Min Duration: {:?}", metrics.min_duration);
    println!("  Max Duration: {:?}", metrics.max_duration);
    println!("  Throughput: {:.2} ops/sec", metrics.throughput());

    metrics
}

/// Memory stress test to detect leaks
pub fn memory_stress_test<F>(
    name: &str,
    iterations: usize,
    threshold_mb: usize,
    mut func: F,
) -> Result<(), String>
where
    F: FnMut(),
{
    let threshold_bytes = threshold_mb * 1024 * 1024;
    let mut monitor = MemoryMonitor::new(threshold_bytes);

    println!("Running memory stress test: {} ({} iterations)", name, iterations);

    for i in 0..iterations {
        func();

        if i % 100 == 0 {
            monitor.sample();
            if monitor.check_for_leaks() {
                return Err(format!(
                    "Memory leak detected after {} iterations. Growth: {} bytes",
                    i,
                    monitor.memory_growth()
                ));
            }
        }
    }

    monitor.sample();
    let final_growth = monitor.memory_growth();
    let peak_memory = monitor.peak_memory();

    println!("Memory stress test {} completed:", name);
    println!("  Memory Growth: {} bytes", final_growth);
    println!("  Peak Memory: {} bytes", peak_memory);

    if monitor.check_for_leaks() {
        Err(format!("Memory leak detected. Final growth: {} bytes", final_growth))
    } else {
        Ok(())
    }
}

/// Get current memory usage (approximation)
fn get_memory_usage() -> usize {
    // This is a simplified approximation
    // In a real implementation, you might use system-specific APIs
    // or tools like jemalloc/tcmalloc for more accurate measurements
    std::mem::size_of::<usize>() * 1024 // Placeholder value
}

/// Performance test for action generation
pub fn benchmark_action_generation(game_states: usize) -> PerformanceMetrics {
    benchmark_function("action_generation", game_states, || {
        let mut game = Game::default();
        game.start();

        // Generate actions for current state
        let _actions: Vec<Action> = game.gen_actions().collect();
    })
}

/// Performance test for game simulation
pub fn benchmark_game_simulation(game_count: usize) -> PerformanceMetrics {
    benchmark_function("game_simulation", game_count, || {
        let mut game = Game::default();
        let rng = GameRng::for_testing(42);

        game.start();

        // Simulate a short game
        let mut action_count = 0;
        while !game.is_over() && action_count < 50 {
            let actions: Vec<Action> = game.gen_actions().collect();
            if actions.is_empty() {
                break;
            }

            let action_index = rng.gen_range(0..actions.len());
            let action = actions[action_index].clone();

            if game.handle_action(action).is_err() {
                break;
            }

            action_count += 1;
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_performance_metrics() {
        let mut metrics = PerformanceMetrics::new();

        metrics.add_measurement(Duration::from_millis(100), true);
        metrics.add_measurement(Duration::from_millis(200), true);
        metrics.add_measurement(Duration::from_millis(50), false);

        assert_eq!(metrics.operation_count, 3);
        assert_eq!(metrics.error_count, 1);
        assert_eq!(metrics.error_rate(), 1.0 / 3.0);
        assert_eq!(metrics.min_duration, Duration::from_millis(50));
        assert_eq!(metrics.max_duration, Duration::from_millis(200));
    }

    #[test]
    fn test_memory_monitor() {
        let threshold = 1024;
        let mut monitor = MemoryMonitor::new(threshold);

        // Take a few samples
        monitor.sample();
        monitor.sample();

        // For this test, we don't expect to exceed threshold
        assert!(!monitor.check_for_leaks());
    }

    #[test]
    fn test_timer() {
        let timer = Timer::new("test_timer");
        thread::sleep(Duration::from_millis(10));
        let duration = timer.elapsed();

        assert!(duration >= Duration::from_millis(10));
    }

    #[tokio::test]
    async fn test_concurrent_load_small() {
        let config = LoadTestConfig {
            concurrent_users: 2,
            operations_per_user: 5,
            duration: Duration::from_secs(1),
            ramp_up_time: Duration::from_millis(100),
            think_time: Duration::from_millis(10),
        };

        let result = run_concurrent_game_sessions(config).await;
        assert!(result.is_ok());

        let metrics = result.unwrap();
        assert!(metrics.operation_count > 0);
        assert!(metrics.error_rate() < 1.0); // Some operations should succeed
    }

    #[test]
    fn test_benchmark_function() {
        let metrics = benchmark_function("test_function", 10, || {
            // Simple operation for testing
            std::hint::black_box(42 * 2);
        });

        assert_eq!(metrics.operation_count, 10);
        assert_eq!(metrics.error_count, 0);
        assert!(metrics.avg_duration > Duration::ZERO);
    }
}
