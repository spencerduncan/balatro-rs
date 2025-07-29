/// Skip tag system for Balatro
/// 
/// Provides strategic rewards when players skip blinds instead of playing them.
/// This module implements the complete skip tag infrastructure including traits,
/// registry, and state management.

pub mod tag_error;
pub mod tag_registry;
pub mod tag_selection;
pub mod tag_trait;
pub mod tags;

// Re-exports for convenient access
pub use tag_error::{TagError, TagErrorKind};
pub use tag_registry::{TagRegistry, SKIP_TAG_REGISTRY};
pub use tag_selection::{TagSelectionState, TagSelectionResult};
pub use tag_trait::{SkipTag, TagEffectType, TagId};
pub use tags::*;

/// Performance monitoring for tag operations
#[cfg(any(debug_assertions, test))]
pub mod performance {
    use std::time::Instant;
    
    /// Track tag selection performance (target: <1ms)
    pub fn track_tag_selection<F, R>(operation: F) -> (R, std::time::Duration)
    where
        F: FnOnce() -> R,
    {
        let start = Instant::now();
        let result = operation();
        let duration = start.elapsed();
        
        if duration.as_millis() > 1 {
            eprintln!(
                "WARNING: Tag selection operation took {}ms (target: <1ms)", 
                duration.as_millis()
            );
        }
        
        (result, duration)
    }
}