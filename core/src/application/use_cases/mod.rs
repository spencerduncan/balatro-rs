//! Use Cases - Specific Business Workflows
//!
//! This module contains specific use case implementations that represent
//! discrete business workflows. Each use case encapsulates a complete
//! business operation with proper error handling and observability.

pub mod create_game_session;
pub mod execute_game_action;

// Re-export for convenience
pub use create_game_session::CreateGameSessionUseCase;
pub use execute_game_action::ExecuteGameActionUseCase;
