//! Repository trait definitions for domain services
//!
//! These traits define the contracts for repository implementations
//! that are used by domain services.

// Re-export repository traits from game_service
pub use crate::domain::services::game_service::{
    ActionHistoryRepository, GameRepository, SessionRepository,
};
