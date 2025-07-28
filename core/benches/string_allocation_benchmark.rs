use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::io::Write;

// Bot Dean Production Benchmark for String Allocation Optimization
// Measures the 500ns overhead from format! calls in hot paths

/// Benchmark the current format! based cache key generation (hot path)
fn bench_format_cache_key(c: &mut Criterion) {
    let mut group = c.benchmark_group("cache_key_generation");
    
    // Simulate the current implementation using format!
    group.bench_function("current_format", |b| {
        b.iter(|| {
            let mut hasher = DefaultHasher::new();
            42u64.hash(&mut hasher);
            123u64.hash(&mut hasher);
            black_box(format!("hand_{:x}", hasher.finish()))
        })
    });
    
    // Test optimized implementation using write! with pre-allocated buffer
    group.bench_function("optimized_write", |b| {
        b.iter(|| {
            let mut hasher = DefaultHasher::new();
            42u64.hash(&mut hasher);
            123u64.hash(&mut hasher);
            let hash_value = hasher.finish();
            
            // Use stack-allocated buffer for small strings
            let mut buffer = [0u8; 32]; // "hand_" + 16 hex digits + null terminator
            let mut cursor = std::io::Cursor::new(&mut buffer[..]);
            write!(cursor, "hand_{:x}", hash_value).unwrap();
            
            let len = cursor.position() as usize;
            let result = std::str::from_utf8(&buffer[..len]).unwrap();
            black_box(result.to_string())
        })
    });
    
    // Test with pre-allocated String for reuse
    group.bench_function("optimized_reuse_string", |b| {
        let mut reusable_string = String::with_capacity(32);
        b.iter(|| {
            let mut hasher = DefaultHasher::new();
            42u64.hash(&mut hasher);
            123u64.hash(&mut hasher);
            let hash_value = hasher.finish();
            
            reusable_string.clear();
            use std::fmt::Write as FmtWrite;
            write!(reusable_string, "hand_{:x}", hash_value).unwrap();
            black_box(reusable_string.clone())
        })
    });
    
    group.finish();
}

/// Benchmark joker message generation (moderate hot path)
fn bench_joker_messages(c: &mut Criterion) {
    let mut group = c.benchmark_group("joker_messages");
    
    // Current implementation using format!
    group.bench_function("current_format", |b| {
        b.iter(|| {
            let hands_remaining = black_box(7u32);
            black_box(format!("Seltzer: {} hands remaining", hands_remaining))
        })
    });
    
    // Optimized using write! with pre-allocated buffer
    group.bench_function("optimized_write", |b| {
        b.iter(|| {
            let hands_remaining = black_box(7u32);
            let mut buffer = [0u8; 64];
            let mut cursor = std::io::Cursor::new(&mut buffer[..]);
            write!(cursor, "Seltzer: {} hands remaining", hands_remaining).unwrap();
            
            let len = cursor.position() as usize;
            let result = std::str::from_utf8(&buffer[..len]).unwrap();
            black_box(result.to_string())
        })
    });
    
    // Test with string interning for common patterns
    group.bench_function("optimized_interned", |b| {
        // Pre-compute common message patterns
        const SELTZER_TEMPLATE: &str = "Seltzer: ";
        const REMAINING_SUFFIX: &str = " hands remaining";
        
        b.iter(|| {
            let hands_remaining = black_box(7u32);
            let mut result = String::with_capacity(32);
            result.push_str(SELTZER_TEMPLATE);
            result.push_str(&hands_remaining.to_string());
            result.push_str(REMAINING_SUFFIX);
            black_box(result)
        })
    });
    
    group.finish();
}

/// Benchmark card value formatting (frequent in retrigger paths)
fn bench_card_value_format(c: &mut Criterion) {
    let mut group = c.benchmark_group("card_value_format");
    
    // Simulate card value enum formatting
    #[derive(Debug)]
    enum TestCardValue {
        Ace, Two, Three, King, Queen, Jack
    }
    
    let card_value = TestCardValue::King;
    
    // Current format! approach
    group.bench_function("current_format", |b| {
        b.iter(|| {
            black_box(format!("Sock and Buskin: {:?} retriggered!", card_value))
        })
    });
    
    // Optimized with match-based string mapping
    group.bench_function("optimized_match", |b| {
        b.iter(|| {
            let card_str = match card_value {
                TestCardValue::Ace => "Ace",
                TestCardValue::Two => "Two", 
                TestCardValue::Three => "Three",
                TestCardValue::King => "King",
                TestCardValue::Queen => "Queen",
                TestCardValue::Jack => "Jack",
            };
            let mut result = String::with_capacity(32);
            result.push_str("Sock and Buskin: ");
            result.push_str(card_str);
            result.push_str(" retriggered!");
            black_box(result)
        })
    });
    
    group.finish();
}

/// Production-scale benchmark simulating RL training load
fn bench_production_simulation(c: &mut Criterion) {
    let mut group = c.benchmark_group("production_simulation");
    
    // Simulate processing 1000 joker effects (typical RL training burst)
    group.bench_function("format_overhead_1000x", |b| {
        b.iter(|| {
            let mut total_allocations = 0;
            for i in 0..1000 {
                let mut hasher = DefaultHasher::new();
                i.hash(&mut hasher);
                let _cache_key = format!("hand_{:x}", hasher.finish());
                let _message = format!("Joker {}: activated", i % 10);
                total_allocations += 2;
            }
            black_box(total_allocations)
        })
    });
    
    // Optimized version
    group.bench_function("optimized_1000x", |b| {
        b.iter(|| {
            let mut cache_key_buffer = String::with_capacity(32);
            let mut message_buffer = String::with_capacity(32);
            let mut total_operations = 0;
            
            for i in 0..1000 {
                // Optimized cache key generation
                let mut hasher = DefaultHasher::new();
                i.hash(&mut hasher);
                cache_key_buffer.clear();
                use std::fmt::Write as FmtWrite;
                write!(cache_key_buffer, "hand_{:x}", hasher.finish()).unwrap();
                
                // Optimized message generation
                message_buffer.clear();
                write!(message_buffer, "Joker {}: activated", i % 10).unwrap();
                
                total_operations += 2;
            }
            black_box(total_operations)
        })
    });
    
    group.finish();
}

criterion_group!(
    benches,
    bench_format_cache_key,
    bench_joker_messages, 
    bench_card_value_format,
    bench_production_simulation
);
criterion_main!(benches);