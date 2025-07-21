use balatro_rs::{
    card::{Card, Suit, Value},
    hand::SelectHand,
    joker::{GameContext, JokerEffect},
    joker_effect_processor::{
        ConflictResolutionStrategy, EffectPriority, JokerEffectProcessor, ProcessingContext,
        WeightedEffect,
    },
};
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

/// Simplified benchmark focusing on core JokerEffectProcessor functionality
pub fn effect_processor_benchmarks(c: &mut Criterion) {
    basic_processing_benchmarks(c);
    conflict_resolution_benchmarks(c);
    cache_performance_benchmarks(c);
    weighted_effects_processing_benchmarks(c);
}

/// Test basic effect processing performance
fn basic_processing_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("basic_processing");

    // Test weighted effects processing with single effect
    group.bench_function("single_effect_processing", |b| {
        b.iter(|| {
            let mut processor = JokerEffectProcessor::new();
            let weighted_effects = vec![create_test_weighted_effect(5, 2, EffectPriority::Normal)];

            black_box(processor.process_weighted_effects(weighted_effects, 0))
        });
    });

    // Test with multiple effects
    group.bench_function("multiple_effects_processing", |b| {
        b.iter(|| {
            let mut processor = JokerEffectProcessor::new();
            let weighted_effects = create_multiple_weighted_effects(5);

            black_box(processor.process_weighted_effects(weighted_effects, 0))
        });
    });

    // Performance target: Single effect processing should be < 1μs
    group.sample_size(10000);
    group.finish();
}

/// Benchmark different conflict resolution strategies
fn conflict_resolution_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("conflict_resolution");

    let strategies = [
        ("sum", ConflictResolutionStrategy::Sum),
        ("maximum", ConflictResolutionStrategy::Maximum),
        ("minimum", ConflictResolutionStrategy::Minimum),
        ("first_wins", ConflictResolutionStrategy::FirstWins),
        ("last_wins", ConflictResolutionStrategy::LastWins),
    ];

    for (name, strategy) in strategies.iter() {
        group.bench_with_input(
            BenchmarkId::new("conflict_resolution", name),
            strategy,
            |b, strategy| {
                b.iter(|| {
                    let mut context = ProcessingContext::default();
                    context.conflict_resolution = strategy.clone();
                    let mut processor = JokerEffectProcessor::with_context(context);

                    // Create conflicting effects (same type, different values)
                    let weighted_effects = create_conflicting_weighted_effects();

                    black_box(processor.process_weighted_effects(weighted_effects, 0))
                });
            },
        );
    }
    
    group.finish();
}

/// Benchmark cache performance
fn cache_performance_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("cache_performance");
    
    // Cache simulation - repeated processing with same inputs
    group.bench_function("repeated_processing", |b| {
        let mut processor = JokerEffectProcessor::new();
        let weighted_effects = create_multiple_weighted_effects(3);
        
        // Prime the cache by running once
        processor.process_weighted_effects(weighted_effects.clone(), 0);
        
        b.iter(|| {
            // This should benefit from any internal caching
            black_box(processor.process_weighted_effects(weighted_effects.clone(), 0))
        });
    });
    
    group.finish();
}

/// Benchmark weighted effects processing with various scenarios
fn weighted_effects_processing_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("weighted_effects");
    
    // Test different numbers of effects
    for effect_count in [1, 5, 10, 20].iter() {
        group.throughput(Throughput::Elements(*effect_count as u64));
        
        group.bench_with_input(
            BenchmarkId::new("effect_count", effect_count),
            effect_count,
            |b, &effect_count| {
                b.iter(|| {
                    let mut processor = JokerEffectProcessor::new();
                    let weighted_effects = create_multiple_weighted_effects(effect_count);
                    
                    black_box(processor.process_weighted_effects(weighted_effects, 0))
                });
            },
        );
    }
    
    // Test priority ordering performance
    group.bench_function("priority_ordering", |b| {
        b.iter(|| {
            let mut processor = JokerEffectProcessor::new();
            let weighted_effects = create_priority_mixed_effects();
            
            black_box(processor.process_weighted_effects(weighted_effects, 0))
        });
    });
    
    // Test retriggering scenarios
    group.bench_function("retriggering_effects", |b| {
        b.iter(|| {
            let mut processor = JokerEffectProcessor::new();
            let weighted_effects = create_retriggering_effects();
            
            black_box(processor.process_weighted_effects(weighted_effects, 0))
        });
    });
    
    // Performance targets:
    // - 10 effects should be < 10μs  
    // - 20 effects should be < 50μs
    group.finish();
}

// Helper functions for creating test data

fn create_test_weighted_effect(chips: i32, mult: i32, priority: EffectPriority) -> WeightedEffect {
    WeightedEffect {
        effect: JokerEffect::new().with_chips(chips).with_mult(mult),
        priority,
        source_joker_id: balatro_rs::joker::JokerId::Joker,
        is_retriggered: false,
    }
}

fn create_multiple_weighted_effects(count: usize) -> Vec<WeightedEffect> {
    let mut effects = Vec::new();
    
    for i in 0..count {
        let chips = (i as i32 + 1) * 2;
        let mult = i as i32 + 1;
        let priority = match i % 4 {
            0 => EffectPriority::Low,
            1 => EffectPriority::Normal,
            2 => EffectPriority::High,
            _ => EffectPriority::Critical,
        };
        
        effects.push(create_test_weighted_effect(chips, mult, priority));
    }
    
    effects
}

fn create_conflicting_weighted_effects() -> Vec<WeightedEffect> {
    vec![
        create_test_weighted_effect(10, 2, EffectPriority::Normal),
        create_test_weighted_effect(15, 3, EffectPriority::Normal), 
        create_test_weighted_effect(8, 4, EffectPriority::Normal),
    ]
}

fn create_priority_mixed_effects() -> Vec<WeightedEffect> {
    vec![
        create_test_weighted_effect(5, 1, EffectPriority::Critical),
        create_test_weighted_effect(3, 2, EffectPriority::Low),
        create_test_weighted_effect(4, 1, EffectPriority::High),
        create_test_weighted_effect(2, 3, EffectPriority::Normal),
        create_test_weighted_effect(6, 1, EffectPriority::Critical),
    ]
}

fn create_retriggering_effects() -> Vec<WeightedEffect> {
    vec![
        WeightedEffect {
            effect: JokerEffect::new().with_chips(5).with_retriggers(2),
            priority: EffectPriority::Normal,
            source_joker_id: balatro_rs::joker::JokerId::Joker,
            is_retriggered: false,
        },
        WeightedEffect {
            effect: JokerEffect::new().with_mult(3).with_retriggers(1),
            priority: EffectPriority::High,
            source_joker_id: balatro_rs::joker::JokerId::GreedyJoker,
            is_retriggered: false,
        },
    ]
}

criterion_group!(
    effect_processor_benches,
    effect_processor_benchmarks
);
criterion_main!(effect_processor_benches);