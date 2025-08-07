//! Performance monitoring utilities for balatro-rs tests
//!
//! This module provides comprehensive performance monitoring and benchmarking
//! utilities for detecting performance regressions and optimizing critical paths.
//!
//! ## Features
//! - Benchmark harness wrappers
//! - Performance regression detection
//! - Memory usage tracking
//! - Timing utilities for critical paths
//! - Statistical analysis of performance data

use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

// ============================================================================
// PERFORMANCE METRICS
// ============================================================================

/// Performance metrics for a single operation
#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    pub name: String,
    pub duration: Duration,
    pub memory_before: usize,
    pub memory_after: usize,
    pub iterations: u32,
    pub timestamp: Instant,
}

impl PerformanceMetrics {
    /// Calculate memory delta in bytes
    pub fn memory_delta(&self) -> isize {
        self.memory_after as isize - self.memory_before as isize
    }

    /// Calculate operations per second
    pub fn ops_per_second(&self) -> f64 {
        if self.duration.as_secs_f64() > 0.0 {
            self.iterations as f64 / self.duration.as_secs_f64()
        } else {
            0.0
        }
    }

    /// Calculate average duration per operation
    pub fn avg_duration(&self) -> Duration {
        if self.iterations > 0 {
            self.duration / self.iterations
        } else {
            Duration::ZERO
        }
    }
}

impl fmt::Display for PerformanceMetrics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: {:.3}ms total, {:.3}µs/op, {:.0} ops/s, {} bytes",
            self.name,
            self.duration.as_secs_f64() * 1000.0,
            self.avg_duration().as_secs_f64() * 1_000_000.0,
            self.ops_per_second(),
            self.memory_delta()
        )
    }
}

// ============================================================================
// PERFORMANCE MONITOR
// ============================================================================

/// Thread-safe performance monitor for collecting metrics
#[derive(Clone)]
pub struct PerformanceMonitor {
    metrics: Arc<Mutex<HashMap<String, Vec<PerformanceMetrics>>>>,
    thresholds: Arc<Mutex<HashMap<String, PerformanceThreshold>>>,
}

impl PerformanceMonitor {
    /// Create a new performance monitor
    pub fn new() -> Self {
        Self {
            metrics: Arc::new(Mutex::new(HashMap::new())),
            thresholds: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Record a performance measurement
    pub fn record(&self, metric: PerformanceMetrics) {
        let mut metrics = self.metrics.lock().unwrap();
        metrics.entry(metric.name.clone()).or_insert_with(Vec::new).push(metric);
    }

    /// Measure the performance of a closure
    pub fn measure<F, R>(&self, name: &str, f: F) -> R
    where
        F: FnOnce() -> R,
    {
        let memory_before = get_memory_usage();
        let start = Instant::now();

        let result = f();

        let duration = start.elapsed();
        let memory_after = get_memory_usage();

        self.record(PerformanceMetrics {
            name: name.to_string(),
            duration,
            memory_before,
            memory_after,
            iterations: 1,
            timestamp: start,
        });

        result
    }

    /// Measure the performance of multiple iterations
    pub fn measure_iterations<F>(&self, name: &str, iterations: u32, mut f: F)
    where
        F: FnMut(),
    {
        let memory_before = get_memory_usage();
        let start = Instant::now();

        for _ in 0..iterations {
            f();
        }

        let duration = start.elapsed();
        let memory_after = get_memory_usage();

        self.record(PerformanceMetrics {
            name: name.to_string(),
            duration,
            memory_before,
            memory_after,
            iterations,
            timestamp: start,
        });
    }

    /// Set a performance threshold for regression detection
    pub fn set_threshold(&self, name: &str, threshold: PerformanceThreshold) {
        let mut thresholds = self.thresholds.lock().unwrap();
        thresholds.insert(name.to_string(), threshold);
    }

    /// Check if any thresholds are exceeded
    pub fn check_thresholds(&self) -> Vec<ThresholdViolation> {
        let metrics = self.metrics.lock().unwrap();
        let thresholds = self.thresholds.lock().unwrap();
        let mut violations = Vec::new();

        for (name, threshold) in thresholds.iter() {
            if let Some(measurements) = metrics.get(name) {
                if let Some(latest) = measurements.last() {
                    if let Some(violation) = threshold.check(latest) {
                        violations.push(violation);
                    }
                }
            }
        }

        violations
    }

    /// Get statistics for a named metric
    pub fn get_statistics(&self, name: &str) -> Option<PerformanceStatistics> {
        let metrics = self.metrics.lock().unwrap();
        metrics.get(name).map(|measurements| PerformanceStatistics::from_metrics(measurements))
    }

    /// Clear all recorded metrics
    pub fn clear(&self) {
        let mut metrics = self.metrics.lock().unwrap();
        metrics.clear();
    }

    /// Generate a performance report
    pub fn report(&self) -> PerformanceReport {
        let metrics = self.metrics.lock().unwrap();
        let mut report = PerformanceReport::new();

        for (name, measurements) in metrics.iter() {
            let stats = PerformanceStatistics::from_metrics(measurements);
            report.add_section(name, stats);
        }

        report
    }
}

impl Default for PerformanceMonitor {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// PERFORMANCE THRESHOLDS
// ============================================================================

/// Performance threshold for regression detection
#[derive(Debug, Clone)]
pub struct PerformanceThreshold {
    pub max_duration: Option<Duration>,
    pub max_memory: Option<usize>,
    pub min_ops_per_second: Option<f64>,
}

impl PerformanceThreshold {
    /// Create a new threshold with duration limit
    pub fn with_duration(max_duration: Duration) -> Self {
        Self {
            max_duration: Some(max_duration),
            max_memory: None,
            min_ops_per_second: None,
        }
    }

    /// Create a new threshold with memory limit
    pub fn with_memory(max_memory: usize) -> Self {
        Self {
            max_duration: None,
            max_memory: Some(max_memory),
            min_ops_per_second: None,
        }
    }

    /// Create a new threshold with ops/s requirement
    pub fn with_ops_per_second(min_ops: f64) -> Self {
        Self {
            max_duration: None,
            max_memory: None,
            min_ops_per_second: Some(min_ops),
        }
    }

    /// Check if metrics violate this threshold
    pub fn check(&self, metrics: &PerformanceMetrics) -> Option<ThresholdViolation> {
        if let Some(max_duration) = self.max_duration {
            if metrics.avg_duration() > max_duration {
                return Some(ThresholdViolation {
                    metric_name: metrics.name.clone(),
                    violation_type: ViolationType::Duration,
                    expected: format!("{:?}", max_duration),
                    actual: format!("{:?}", metrics.avg_duration()),
                });
            }
        }

        if let Some(max_memory) = self.max_memory {
            let memory_used = metrics.memory_delta().abs() as usize;
            if memory_used > max_memory {
                return Some(ThresholdViolation {
                    metric_name: metrics.name.clone(),
                    violation_type: ViolationType::Memory,
                    expected: format!("{} bytes", max_memory),
                    actual: format!("{} bytes", memory_used),
                });
            }
        }

        if let Some(min_ops) = self.min_ops_per_second {
            if metrics.ops_per_second() < min_ops {
                return Some(ThresholdViolation {
                    metric_name: metrics.name.clone(),
                    violation_type: ViolationType::Throughput,
                    expected: format!("{:.2} ops/s", min_ops),
                    actual: format!("{:.2} ops/s", metrics.ops_per_second()),
                });
            }
        }

        None
    }
}

/// Type of threshold violation
#[derive(Debug, Clone, PartialEq)]
pub enum ViolationType {
    Duration,
    Memory,
    Throughput,
}

/// Details about a threshold violation
#[derive(Debug, Clone)]
pub struct ThresholdViolation {
    pub metric_name: String,
    pub violation_type: ViolationType,
    pub expected: String,
    pub actual: String,
}

impl fmt::Display for ThresholdViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Performance regression in '{}': {:?} - expected {}, got {}",
            self.metric_name, self.violation_type, self.expected, self.actual
        )
    }
}

// ============================================================================
// PERFORMANCE STATISTICS
// ============================================================================

/// Statistical analysis of performance measurements
#[derive(Debug, Clone)]
pub struct PerformanceStatistics {
    pub count: usize,
    pub mean_duration: Duration,
    pub median_duration: Duration,
    pub min_duration: Duration,
    pub max_duration: Duration,
    pub std_deviation: Duration,
    pub percentile_95: Duration,
    pub percentile_99: Duration,
    pub total_memory: isize,
}

impl PerformanceStatistics {
    /// Calculate statistics from a set of metrics
    pub fn from_metrics(metrics: &[PerformanceMetrics]) -> Self {
        if metrics.is_empty() {
            return Self::default();
        }

        let mut durations: Vec<Duration> = metrics.iter().map(|m| m.avg_duration()).collect();
        durations.sort();

        let count = durations.len();
        let sum: Duration = durations.iter().sum();
        let mean = sum / count as u32;

        let median = if count % 2 == 0 {
            (durations[count / 2 - 1] + durations[count / 2]) / 2
        } else {
            durations[count / 2]
        };

        let min = durations[0];
        let max = durations[count - 1];

        // Calculate standard deviation
        let variance = durations
            .iter()
            .map(|d| {
                let diff = if *d > mean {
                    d.as_nanos() - mean.as_nanos()
                } else {
                    mean.as_nanos() - d.as_nanos()
                };
                diff * diff
            })
            .sum::<u128>() / count as u128;

        let std_dev_nanos = (variance as f64).sqrt();
        let std_deviation = Duration::from_nanos(std_dev_nanos as u64);

        let percentile_95 = durations[(count as f64 * 0.95) as usize].min(max);
        let percentile_99 = durations[(count as f64 * 0.99) as usize].min(max);

        let total_memory = metrics.iter().map(|m| m.memory_delta()).sum();

        Self {
            count,
            mean_duration: mean,
            median_duration: median,
            min_duration: min,
            max_duration: max,
            std_deviation,
            percentile_95,
            percentile_99,
            total_memory,
        }
    }
}

impl Default for PerformanceStatistics {
    fn default() -> Self {
        Self {
            count: 0,
            mean_duration: Duration::ZERO,
            median_duration: Duration::ZERO,
            min_duration: Duration::ZERO,
            max_duration: Duration::ZERO,
            std_deviation: Duration::ZERO,
            percentile_95: Duration::ZERO,
            percentile_99: Duration::ZERO,
            total_memory: 0,
        }
    }
}

impl fmt::Display for PerformanceStatistics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Performance Statistics:")?;
        writeln!(f, "  Count: {}", self.count)?;
        writeln!(f, "  Mean: {:?}", self.mean_duration)?;
        writeln!(f, "  Median: {:?}", self.median_duration)?;
        writeln!(f, "  Min: {:?}", self.min_duration)?;
        writeln!(f, "  Max: {:?}", self.max_duration)?;
        writeln!(f, "  Std Dev: {:?}", self.std_deviation)?;
        writeln!(f, "  95th percentile: {:?}", self.percentile_95)?;
        writeln!(f, "  99th percentile: {:?}", self.percentile_99)?;
        writeln!(f, "  Total Memory: {} bytes", self.total_memory)?;
        Ok(())
    }
}

// ============================================================================
// PERFORMANCE REPORT
// ============================================================================

/// Comprehensive performance report
pub struct PerformanceReport {
    sections: Vec<(String, PerformanceStatistics)>,
    generated_at: Instant,
}

impl PerformanceReport {
    /// Create a new empty report
    pub fn new() -> Self {
        Self {
            sections: Vec::new(),
            generated_at: Instant::now(),
        }
    }

    /// Add a section to the report
    pub fn add_section(&mut self, name: &str, stats: PerformanceStatistics) {
        self.sections.push((name.to_string(), stats));
    }

    /// Generate markdown report
    pub fn to_markdown(&self) -> String {
        let mut output = String::new();
        output.push_str("# Performance Report\n\n");

        for (name, stats) in &self.sections {
            output.push_str(&format!("## {}\n\n", name));
            output.push_str(&format!("| Metric | Value |\n"));
            output.push_str(&format!("|--------|-------|\n"));
            output.push_str(&format!("| Count | {} |\n", stats.count));
            output.push_str(&format!("| Mean | {:?} |\n", stats.mean_duration));
            output.push_str(&format!("| Median | {:?} |\n", stats.median_duration));
            output.push_str(&format!("| Min | {:?} |\n", stats.min_duration));
            output.push_str(&format!("| Max | {:?} |\n", stats.max_duration));
            output.push_str(&format!("| Std Dev | {:?} |\n", stats.std_deviation));
            output.push_str(&format!("| P95 | {:?} |\n", stats.percentile_95));
            output.push_str(&format!("| P99 | {:?} |\n", stats.percentile_99));
            output.push_str(&format!("| Memory | {} bytes |\n\n", stats.total_memory));
        }

        output
    }
}

impl fmt::Display for PerformanceReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "=== Performance Report ===")?;
        for (name, stats) in &self.sections {
            writeln!(f, "\n[{}]", name)?;
            write!(f, "{}", stats)?;
        }
        Ok(())
    }
}

// ============================================================================
// BENCHMARK HARNESS
// ============================================================================

/// Simplified benchmark harness for critical paths
pub struct BenchmarkHarness {
    monitor: PerformanceMonitor,
    warmup_iterations: u32,
    benchmark_iterations: u32,
}

impl BenchmarkHarness {
    /// Create a new benchmark harness
    pub fn new() -> Self {
        Self {
            monitor: PerformanceMonitor::new(),
            warmup_iterations: 100,
            benchmark_iterations: 1000,
        }
    }

    /// Set warmup iterations
    pub fn with_warmup(mut self, iterations: u32) -> Self {
        self.warmup_iterations = iterations;
        self
    }

    /// Set benchmark iterations
    pub fn with_iterations(mut self, iterations: u32) -> Self {
        self.benchmark_iterations = iterations;
        self
    }

    /// Run a benchmark
    pub fn bench<F>(&self, name: &str, mut f: F) -> PerformanceStatistics
    where
        F: FnMut(),
    {
        // Warmup phase
        for _ in 0..self.warmup_iterations {
            f();
        }

        // Clear any existing metrics
        self.monitor.clear();

        // Benchmark phase
        self.monitor.measure_iterations(name, self.benchmark_iterations, f);

        // Get statistics
        self.monitor.get_statistics(name).unwrap_or_default()
    }

    /// Run a comparative benchmark
    pub fn compare<F1, F2>(&self, name1: &str, f1: F1, name2: &str, f2: F2) -> ComparisonResult
    where
        F1: FnMut(),
        F2: FnMut(),
    {
        let stats1 = self.bench(name1, f1);
        let stats2 = self.bench(name2, f2);

        ComparisonResult {
            baseline: (name1.to_string(), stats1),
            comparison: (name2.to_string(), stats2),
        }
    }

    /// Get the performance monitor
    pub fn monitor(&self) -> &PerformanceMonitor {
        &self.monitor
    }
}

impl Default for BenchmarkHarness {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of comparing two benchmarks
pub struct ComparisonResult {
    pub baseline: (String, PerformanceStatistics),
    pub comparison: (String, PerformanceStatistics),
}

impl ComparisonResult {
    /// Calculate speedup factor
    pub fn speedup(&self) -> f64 {
        let baseline_mean = self.baseline.1.mean_duration.as_nanos() as f64;
        let comparison_mean = self.comparison.1.mean_duration.as_nanos() as f64;

        if comparison_mean > 0.0 {
            baseline_mean / comparison_mean
        } else {
            0.0
        }
    }

    /// Check if comparison is faster
    pub fn is_faster(&self) -> bool {
        self.speedup() > 1.0
    }

    /// Get percentage improvement
    pub fn improvement_percent(&self) -> f64 {
        (self.speedup() - 1.0) * 100.0
    }
}

impl fmt::Display for ComparisonResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Benchmark Comparison:")?;
        writeln!(f, "  Baseline ({}): {:?}", self.baseline.0, self.baseline.1.mean_duration)?;
        writeln!(f, "  Comparison ({}): {:?}", self.comparison.0, self.comparison.1.mean_duration)?;

        if self.is_faster() {
            writeln!(f, "  {} is {:.2}x faster ({:.1}% improvement)",
                self.comparison.0, self.speedup(), self.improvement_percent())?;
        } else {
            writeln!(f, "  {} is {:.2}x slower ({:.1}% regression)",
                self.comparison.0, 1.0 / self.speedup(), -self.improvement_percent())?;
        }

        Ok(())
    }
}

// ============================================================================
// TIMING UTILITIES
// ============================================================================

/// Simple timing utility for critical paths
pub struct Timer {
    start: Instant,
    name: String,
}

impl Timer {
    /// Start a new timer
    pub fn start(name: &str) -> Self {
        Self {
            start: Instant::now(),
            name: name.to_string(),
        }
    }

    /// Get elapsed time
    pub fn elapsed(&self) -> Duration {
        self.start.elapsed()
    }

    /// Stop timer and return duration
    pub fn stop(self) -> Duration {
        self.elapsed()
    }

    /// Stop timer and print duration
    pub fn stop_and_print(self) {
        println!("{}: {:?}", self.name, self.elapsed());
    }
}

/// Macro for timing code blocks
#[macro_export]
macro_rules! time_it {
    ($name:expr, $code:block) => {{
        let _timer = $crate::common::performance::Timer::start($name);
        let result = $code;
        let duration = _timer.stop();
        println!("{}: {:?}", $name, duration);
        result
    }};
}

// ============================================================================
// MEMORY UTILITIES
// ============================================================================

/// Get current memory usage (simplified - would need proper implementation)
fn get_memory_usage() -> usize {
    // This is a placeholder - in a real implementation, you would:
    // 1. Use a memory allocator that tracks allocations
    // 2. Or use platform-specific APIs to query process memory
    // 3. Or integrate with jemalloc/mimalloc statistics

    // For now, return a dummy value
    0
}

/// Track memory allocations for a code block
pub fn track_memory<F, R>(f: F) -> (R, isize)
where
    F: FnOnce() -> R,
{
    let before = get_memory_usage();
    let result = f();
    let after = get_memory_usage();
    (result, after as isize - before as isize)
}

// ============================================================================
// REGRESSION DETECTION
// ============================================================================

/// Performance baseline for regression detection
#[derive(Debug, Clone)]
pub struct PerformanceBaseline {
    pub metrics: HashMap<String, PerformanceStatistics>,
    pub tolerance: f64, // Percentage tolerance for regression
}

impl PerformanceBaseline {
    /// Create a new baseline
    pub fn new(tolerance: f64) -> Self {
        Self {
            metrics: HashMap::new(),
            tolerance,
        }
    }

    /// Add a baseline metric
    pub fn add_baseline(&mut self, name: &str, stats: PerformanceStatistics) {
        self.metrics.insert(name.to_string(), stats);
    }

    /// Check for regressions against baseline
    pub fn check_regression(&self, name: &str, current: &PerformanceStatistics) -> Option<String> {
        if let Some(baseline) = self.metrics.get(name) {
            let baseline_mean = baseline.mean_duration.as_nanos() as f64;
            let current_mean = current.mean_duration.as_nanos() as f64;

            let regression_factor = current_mean / baseline_mean;
            let tolerance_factor = 1.0 + (self.tolerance / 100.0);

            if regression_factor > tolerance_factor {
                let regression_percent = (regression_factor - 1.0) * 100.0;
                return Some(format!(
                    "Performance regression detected: {:.1}% slower than baseline",
                    regression_percent
                ));
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_performance_monitor() {
        let monitor = PerformanceMonitor::new();

        monitor.measure("test_operation", || {
            thread::sleep(Duration::from_millis(10));
        });

        let stats = monitor.get_statistics("test_operation").unwrap();
        assert_eq!(stats.count, 1);
        assert!(stats.mean_duration >= Duration::from_millis(10));
    }

    #[test]
    fn test_benchmark_harness() {
        let harness = BenchmarkHarness::new()
            .with_warmup(10)
            .with_iterations(100);

        let stats = harness.bench("simple_operation", || {
            let _ = 1 + 1;
        });

        assert_eq!(stats.count, 100);
        assert!(stats.mean_duration < Duration::from_millis(1));
    }

    #[test]
    fn test_threshold_detection() {
        let monitor = PerformanceMonitor::new();

        monitor.set_threshold(
            "slow_operation",
            PerformanceThreshold::with_duration(Duration::from_millis(5))
        );

        monitor.measure("slow_operation", || {
            thread::sleep(Duration::from_millis(10));
        });

        let violations = monitor.check_thresholds();
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].violation_type, ViolationType::Duration);
    }

    #[test]
    fn test_comparison() {
        let harness = BenchmarkHarness::new()
            .with_warmup(10)
            .with_iterations(100);

        let result = harness.compare(
            "slow", || { thread::sleep(Duration::from_micros(10)); },
            "fast", || { thread::sleep(Duration::from_micros(5)); }
        );

        assert!(result.is_faster());
        assert!(result.speedup() > 1.0);
    }
}
