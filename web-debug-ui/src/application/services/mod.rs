//! Application Services - Use Case Orchestration
//!
//! This module contains the high-level application services that orchestrate
//! use cases and coordinate between the domain and infrastructure layers.
//!
//! ## Services Architecture
//!
//! - **GameApplicationService**: Orchestrates game-related use cases
//! - **SessionManagementService**: Manages session lifecycle and cleanup
//!
//! All services follow production patterns:
//! - Dependency injection for testability
//! - Comprehensive error handling
//! - Performance monitoring
//! - Graceful degradation

pub mod game_application_service;
pub mod session_management_service;

// Re-export for convenience
pub use game_application_service::GameApplicationService;
pub use session_management_service::SessionManagementService;