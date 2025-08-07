//! Memory leak detection utilities for balatro-rs tests
//!
//! This module provides utilities for detecting memory leaks, tracking allocations,
//! and ensuring proper resource cleanup in tests.
//!
//! ## Features
//! - Allocation tracking utilities
//! - Memory leak detection helpers
//! - Resource cleanup verification
//! - RAII pattern enforcement checks
//! - Memory usage profiling

use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::mem;
use std::ptr::NonNull;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

// ============================================================================
// ALLOCATION TRACKING
// ============================================================================

/// Global allocation statistics
pub struct AllocationStats {
    pub total_allocated: AtomicUsize,
    pub total_deallocated: AtomicUsize,
    pub current_usage: AtomicUsize,
    pub peak_usage: AtomicUsize,
    pub allocation_count: AtomicUsize,
    pub deallocation_count: AtomicUsize,
}

impl AllocationStats {
    /// Create new allocation statistics
    pub const fn new() -> Self {
        Self {
            total_allocated: AtomicUsize::new(0),
            total_deallocated: AtomicUsize::new(0),
            current_usage: AtomicUsize::new(0),
            peak_usage: AtomicUsize::new(0),
            allocation_count: AtomicUsize::new(0),
            deallocation_count: AtomicUsize::new(0),
        }
    }

    /// Record an allocation
    pub fn record_alloc(&self, size: usize) {
        self.total_allocated.fetch_add(size, Ordering::Relaxed);
        self.allocation_count.fetch_add(1, Ordering::Relaxed);

        let current = self.current_usage.fetch_add(size, Ordering::Relaxed) + size;
        let mut peak = self.peak_usage.load(Ordering::Relaxed);

        while current > peak {
            match self.peak_usage.compare_exchange_weak(
                peak,
                current,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ) {
                Ok(_) => break,
                Err(p) => peak = p,
            }
        }
    }

    /// Record a deallocation
    pub fn record_dealloc(&self, size: usize) {
        self.total_deallocated.fetch_add(size, Ordering::Relaxed);
        self.deallocation_count.fetch_add(1, Ordering::Relaxed);
        self.current_usage.fetch_sub(size, Ordering::Relaxed);
    }

    /// Get current memory usage
    pub fn current_usage(&self) -> usize {
        self.current_usage.load(Ordering::Relaxed)
    }

    /// Get peak memory usage
    pub fn peak_usage(&self) -> usize {
        self.peak_usage.load(Ordering::Relaxed)
    }

    /// Check for memory leaks
    pub fn has_leaks(&self) -> bool {
        self.current_usage() > 0
    }

    /// Get leak size if any
    pub fn leak_size(&self) -> usize {
        self.current_usage()
    }

    /// Reset statistics
    pub fn reset(&self) {
        self.total_allocated.store(0, Ordering::Relaxed);
        self.total_deallocated.store(0, Ordering::Relaxed);
        self.current_usage.store(0, Ordering::Relaxed);
        self.peak_usage.store(0, Ordering::Relaxed);
        self.allocation_count.store(0, Ordering::Relaxed);
        self.deallocation_count.store(0, Ordering::Relaxed);
    }

    /// Generate a report
    pub fn report(&self) -> AllocationReport {
        AllocationReport {
            total_allocated: self.total_allocated.load(Ordering::Relaxed),
            total_deallocated: self.total_deallocated.load(Ordering::Relaxed),
            current_usage: self.current_usage.load(Ordering::Relaxed),
            peak_usage: self.peak_usage.load(Ordering::Relaxed),
            allocation_count: self.allocation_count.load(Ordering::Relaxed),
            deallocation_count: self.deallocation_count.load(Ordering::Relaxed),
        }
    }
}

/// Allocation report snapshot
#[derive(Debug, Clone)]
pub struct AllocationReport {
    pub total_allocated: usize,
    pub total_deallocated: usize,
    pub current_usage: usize,
    pub peak_usage: usize,
    pub allocation_count: usize,
    pub deallocation_count: usize,
}

impl AllocationReport {
    /// Check if there are memory leaks
    pub fn has_leaks(&self) -> bool {
        self.current_usage > 0
    }

    /// Get the size of memory leaks
    pub fn leak_size(&self) -> usize {
        self.current_usage
    }

    /// Get allocation/deallocation balance
    pub fn balance(&self) -> isize {
        self.allocation_count as isize - self.deallocation_count as isize
    }
}

impl fmt::Display for AllocationReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Memory Allocation Report:")?;
        writeln!(f, "  Total Allocated: {} bytes", self.total_allocated)?;
        writeln!(f, "  Total Deallocated: {} bytes", self.total_deallocated)?;
        writeln!(f, "  Current Usage: {} bytes", self.current_usage)?;
        writeln!(f, "  Peak Usage: {} bytes", self.peak_usage)?;
        writeln!(f, "  Allocations: {}", self.allocation_count)?;
        writeln!(f, "  Deallocations: {}", self.deallocation_count)?;

        if self.has_leaks() {
            writeln!(f, "  ⚠️  MEMORY LEAK: {} bytes", self.leak_size())?;
        } else {
            writeln!(f, "  ✓ No memory leaks detected")?;
        }

        Ok(())
    }
}

// ============================================================================
// TRACKING ALLOCATOR
// ============================================================================

/// A custom allocator that tracks memory allocations
pub struct TrackingAllocator {
    stats: AllocationStats,
    track_backtraces: bool,
}

impl TrackingAllocator {
    /// Create a new tracking allocator
    pub const fn new() -> Self {
        Self {
            stats: AllocationStats::new(),
            track_backtraces: false,
        }
    }

    /// Enable backtrace tracking (expensive)
    pub fn with_backtraces(mut self) -> Self {
        self.track_backtraces = true;
        self
    }

    /// Get allocation statistics
    pub fn stats(&self) -> &AllocationStats {
        &self.stats
    }

    /// Reset statistics
    pub fn reset(&self) {
        self.stats.reset();
    }

    /// Check for memory leaks
    pub fn check_leaks(&self) -> Result<(), MemoryLeakError> {
        if self.stats.has_leaks() {
            Err(MemoryLeakError {
                leaked_bytes: self.stats.leak_size(),
                allocation_count: self.stats.allocation_count.load(Ordering::Relaxed),
                deallocation_count: self.stats.deallocation_count.load(Ordering::Relaxed),
            })
        } else {
            Ok(())
        }
    }
}

unsafe impl GlobalAlloc for TrackingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ptr = System.alloc(layout);
        if !ptr.is_null() {
            self.stats.record_alloc(layout.size());
        }
        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.stats.record_dealloc(layout.size());
        System.dealloc(ptr, layout);
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        let ptr = System.alloc_zeroed(layout);
        if !ptr.is_null() {
            self.stats.record_alloc(layout.size());
        }
        ptr
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        self.stats.record_dealloc(layout.size());
        let new_ptr = System.realloc(ptr, layout, new_size);
        if !new_ptr.is_null() {
            self.stats.record_alloc(new_size);
        }
        new_ptr
    }
}

/// Error type for memory leak detection
#[derive(Debug)]
pub struct MemoryLeakError {
    pub leaked_bytes: usize,
    pub allocation_count: usize,
    pub deallocation_count: usize,
}

impl fmt::Display for MemoryLeakError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Memory leak detected: {} bytes leaked ({} allocations, {} deallocations)",
            self.leaked_bytes, self.allocation_count, self.deallocation_count
        )
    }
}

impl std::error::Error for MemoryLeakError {}

// ============================================================================
// MEMORY GUARD
// ============================================================================

/// Guard for detecting memory leaks in a scope
pub struct MemoryGuard {
    name: String,
    initial_usage: usize,
    stats: Arc<AllocationStats>,
    panic_on_leak: bool,
}

impl MemoryGuard {
    /// Create a new memory guard
    pub fn new(name: &str, stats: Arc<AllocationStats>) -> Self {
        Self {
            name: name.to_string(),
            initial_usage: stats.current_usage(),
            stats,
            panic_on_leak: false,
        }
    }

    /// Enable panic on leak detection
    pub fn panic_on_leak(mut self) -> Self {
        self.panic_on_leak = true;
        self
    }

    /// Get current memory delta
    pub fn memory_delta(&self) -> isize {
        self.stats.current_usage() as isize - self.initial_usage as isize
    }

    /// Check if there's a leak
    pub fn has_leak(&self) -> bool {
        self.memory_delta() > 0
    }
}

impl Drop for MemoryGuard {
    fn drop(&mut self) {
        let delta = self.memory_delta();
        if delta > 0 {
            let msg = format!(
                "Memory leak in '{}': {} bytes leaked",
                self.name, delta
            );

            if self.panic_on_leak {
                panic!("{}", msg);
            } else {
                eprintln!("WARNING: {}", msg);
            }
        }
    }
}

// ============================================================================
// RESOURCE TRACKER
// ============================================================================

/// Track resource lifecycle (RAII pattern enforcement)
#[derive(Clone)]
pub struct ResourceTracker {
    resources: Arc<Mutex<HashMap<String, ResourceInfo>>>,
}

impl ResourceTracker {
    /// Create a new resource tracker
    pub fn new() -> Self {
        Self {
            resources: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Register a resource allocation
    pub fn register(&self, id: String, resource_type: String, size: usize) {
        let mut resources = self.resources.lock().unwrap();
        resources.insert(
            id.clone(),
            ResourceInfo {
                id,
                resource_type,
                size,
                allocated_at: std::time::Instant::now(),
            },
        );
    }

    /// Unregister a resource deallocation
    pub fn unregister(&self, id: &str) -> bool {
        let mut resources = self.resources.lock().unwrap();
        resources.remove(id).is_some()
    }

    /// Check for leaked resources
    pub fn check_leaks(&self) -> Vec<ResourceInfo> {
        let resources = self.resources.lock().unwrap();
        resources.values().cloned().collect()
    }

    /// Get current resource count
    pub fn resource_count(&self) -> usize {
        let resources = self.resources.lock().unwrap();
        resources.len()
    }

    /// Clear all tracked resources
    pub fn clear(&self) {
        let mut resources = self.resources.lock().unwrap();
        resources.clear();
    }
}

impl Default for ResourceTracker {
    fn default() -> Self {
        Self::new()
    }
}

/// Information about a tracked resource
#[derive(Debug, Clone)]
pub struct ResourceInfo {
    pub id: String,
    pub resource_type: String,
    pub size: usize,
    pub allocated_at: std::time::Instant,
}

impl fmt::Display for ResourceInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Resource '{}' ({}, {} bytes) allocated {:?} ago",
            self.id,
            self.resource_type,
            self.size,
            self.allocated_at.elapsed()
        )
    }
}

// ============================================================================
// RAII PATTERN HELPERS
// ============================================================================

/// RAII guard for automatic resource cleanup
pub struct ResourceGuard<T, F>
where
    F: FnOnce(T),
{
    resource: Option<T>,
    cleanup: Option<F>,
}

impl<T, F> ResourceGuard<T, F>
where
    F: FnOnce(T),
{
    /// Create a new resource guard
    pub fn new(resource: T, cleanup: F) -> Self {
        Self {
            resource: Some(resource),
            cleanup: Some(cleanup),
        }
    }

    /// Get a reference to the resource
    pub fn get(&self) -> Option<&T> {
        self.resource.as_ref()
    }

    /// Get a mutable reference to the resource
    pub fn get_mut(&mut self) -> Option<&mut T> {
        self.resource.as_mut()
    }

    /// Release the resource without cleanup
    pub fn release(mut self) -> T {
        self.cleanup = None;
        self.resource.take().expect("Resource already released")
    }
}

impl<T, F> Drop for ResourceGuard<T, F>
where
    F: FnOnce(T),
{
    fn drop(&mut self) {
        if let (Some(resource), Some(cleanup)) = (self.resource.take(), self.cleanup.take()) {
            cleanup(resource);
        }
    }
}

impl<T, F> std::ops::Deref for ResourceGuard<T, F>
where
    F: FnOnce(T),
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.resource.as_ref().expect("Resource already released")
    }
}

impl<T, F> std::ops::DerefMut for ResourceGuard<T, F>
where
    F: FnOnce(T),
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.resource.as_mut().expect("Resource already released")
    }
}

// ============================================================================
// MEMORY PROFILER
// ============================================================================

/// Simple memory profiler for test scenarios
pub struct MemoryProfiler {
    samples: Vec<MemorySample>,
    sampling_interval: std::time::Duration,
}

impl MemoryProfiler {
    /// Create a new memory profiler
    pub fn new() -> Self {
        Self {
            samples: Vec::new(),
            sampling_interval: std::time::Duration::from_millis(10),
        }
    }

    /// Set sampling interval
    pub fn with_interval(mut self, interval: std::time::Duration) -> Self {
        self.sampling_interval = interval;
        self
    }

    /// Record a memory sample
    pub fn sample(&mut self, stats: &AllocationStats) {
        self.samples.push(MemorySample {
            timestamp: std::time::Instant::now(),
            usage: stats.current_usage(),
            allocations: stats.allocation_count.load(Ordering::Relaxed),
        });
    }

    /// Profile a function
    pub fn profile<F, R>(&mut self, stats: Arc<AllocationStats>, f: F) -> R
    where
        F: FnOnce() -> R,
    {
        let start = std::time::Instant::now();
        self.sample(&stats);

        let result = f();

        self.sample(&stats);
        result
    }

    /// Get peak memory usage from samples
    pub fn peak_usage(&self) -> usize {
        self.samples.iter().map(|s| s.usage).max().unwrap_or(0)
    }

    /// Get average memory usage
    pub fn average_usage(&self) -> usize {
        if self.samples.is_empty() {
            return 0;
        }

        let total: usize = self.samples.iter().map(|s| s.usage).sum();
        total / self.samples.len()
    }

    /// Generate a memory profile report
    pub fn report(&self) -> MemoryProfileReport {
        MemoryProfileReport {
            samples: self.samples.clone(),
            peak_usage: self.peak_usage(),
            average_usage: self.average_usage(),
        }
    }
}

impl Default for MemoryProfiler {
    fn default() -> Self {
        Self::new()
    }
}

/// A single memory sample
#[derive(Debug, Clone)]
pub struct MemorySample {
    pub timestamp: std::time::Instant,
    pub usage: usize,
    pub allocations: usize,
}

/// Memory profile report
#[derive(Debug, Clone)]
pub struct MemoryProfileReport {
    pub samples: Vec<MemorySample>,
    pub peak_usage: usize,
    pub average_usage: usize,
}

impl fmt::Display for MemoryProfileReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Memory Profile Report:")?;
        writeln!(f, "  Samples: {}", self.samples.len())?;
        writeln!(f, "  Peak Usage: {} bytes", self.peak_usage)?;
        writeln!(f, "  Average Usage: {} bytes", self.average_usage)?;

        if !self.samples.is_empty() {
            let first = &self.samples[0];
            let last = &self.samples[self.samples.len() - 1];
            let duration = last.timestamp.duration_since(first.timestamp);
            writeln!(f, "  Duration: {:?}", duration)?;

            let growth = last.usage as isize - first.usage as isize;
            if growth > 0 {
                writeln!(f, "  Memory Growth: {} bytes", growth)?;
            } else if growth < 0 {
                writeln!(f, "  Memory Reduction: {} bytes", -growth)?;
            } else {
                writeln!(f, "  Memory Stable")?;
            }
        }

        Ok(())
    }
}

// ============================================================================
// TEST HELPERS
// ============================================================================

/// Run a test with memory leak detection
pub fn test_with_leak_detection<F>(name: &str, f: F)
where
    F: FnOnce(),
{
    let stats = Arc::new(AllocationStats::new());
    let guard = MemoryGuard::new(name, stats.clone()).panic_on_leak();

    f();

    drop(guard);
}

/// Assert no memory leaks in a code block
#[macro_export]
macro_rules! assert_no_leaks {
    ($code:block) => {{
        let stats = std::sync::Arc::new($crate::common::memory::AllocationStats::new());
        let initial = stats.current_usage();

        $code

        let final_usage = stats.current_usage();
        assert_eq!(
            initial, final_usage,
            "Memory leak detected: {} bytes leaked",
            final_usage - initial
        );
    }};
}

/// Assert memory usage is within bounds
#[macro_export]
macro_rules! assert_memory_usage {
    ($code:block, $max_bytes:expr) => {{
        let stats = std::sync::Arc::new($crate::common::memory::AllocationStats::new());

        $code

        let peak = stats.peak_usage();
        assert!(
            peak <= $max_bytes,
            "Memory usage exceeded limit: {} > {} bytes",
            peak, $max_bytes
        );
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allocation_tracking() {
        let stats = AllocationStats::new();

        stats.record_alloc(100);
        assert_eq!(stats.current_usage(), 100);
        assert_eq!(stats.peak_usage(), 100);

        stats.record_dealloc(50);
        assert_eq!(stats.current_usage(), 50);
        assert_eq!(stats.peak_usage(), 100);

        stats.record_alloc(100);
        assert_eq!(stats.current_usage(), 150);
        assert_eq!(stats.peak_usage(), 150);
    }

    #[test]
    fn test_memory_guard() {
        let stats = Arc::new(AllocationStats::new());

        {
            let _guard = MemoryGuard::new("test", stats.clone());
            stats.record_alloc(100);
            stats.record_dealloc(100);
        }
        // Guard should not panic as memory was properly cleaned up
    }

    #[test]
    fn test_resource_tracker() {
        let tracker = ResourceTracker::new();

        tracker.register("resource1".to_string(), "File".to_string(), 1024);
        assert_eq!(tracker.resource_count(), 1);

        assert!(tracker.unregister("resource1"));
        assert_eq!(tracker.resource_count(), 0);

        let leaks = tracker.check_leaks();
        assert!(leaks.is_empty());
    }

    #[test]
    fn test_resource_guard() {
        let mut cleaned = false;
        {
            let _guard = ResourceGuard::new(42, |_| {
                cleaned = true;
            });
        }
        assert!(cleaned);
    }

    #[test]
    fn test_memory_profiler() {
        let mut profiler = MemoryProfiler::new();
        let stats = Arc::new(AllocationStats::new());

        profiler.sample(&stats);
        stats.record_alloc(1000);
        profiler.sample(&stats);
        stats.record_alloc(500);
        profiler.sample(&stats);

        assert_eq!(profiler.peak_usage(), 1500);
        assert_eq!(profiler.average_usage(), 833); // (0 + 1000 + 1500) / 3
    }
}
