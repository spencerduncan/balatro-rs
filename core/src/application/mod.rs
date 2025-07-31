//! Application Layer - Use Case Orchestration and Dependency Coordination
//!
//! This module implements the Application Layer of the Clean Architecture pattern,
//! providing use case orchestration, dependency injection, and session management
//! for scalable game engine operations.
//!
//! ## Architecture Overview
//!
//! The Application Layer sits between the Domain and Infrastructure layers:
//! - **Domain Layer**: Core business logic (game rules, actions, state)
//! - **Application Layer**: Use case orchestration, session management, DI
//! - **Infrastructure Layer**: External concerns (persistence, networking, UI)
//!
//! ## Key Components
//!
//! - **Services**: High-level application services for use case orchestration
//! - **Use Cases**: Specific business workflows (create session, execute action)
//! - **Dependency Injection**: Service container and trait-based dependencies
//! - **Error Handling**: Comprehensive error types and recovery strategies
//! - **Session Management**: Multi-session lifecycle and cleanup
//!
//! ## Production Design Principles
//!
//! - **Scalability**: Designed for 100+ concurrent sessions
//! - **Fault Tolerance**: Graceful error handling and recovery
//! - **Observability**: Comprehensive metrics and tracing
//! - **Performance**: Sub-10ms latency targets for all operations

pub mod config;
pub mod container;
pub mod errors;
pub mod services;
pub mod use_cases;

// Re-export key types for easier consumption
pub use config::ApplicationConfig;
pub use container::ServiceContainer;
pub use errors::{ApplicationError, ErrorRecoveryStrategy};
pub use services::{GameApplicationService, SessionManagementService};
pub use use_cases::{CreateGameSessionUseCase, ExecuteGameActionUseCase};
