//! Application Layer - Simple Session Management
//!
//! This module provides a thin application layer over the core game engine,
//! offering session management for multiple concurrent games. Designed to be
//! simple, fast, and appropriate for a game engine context.
//!
//! ## Key Components
//!
//! - **SessionManager**: In-memory session storage and lifecycle management
//! - **GameSession**: Wrapper around Game with session metadata
//! - **SessionError**: Simple error handling for session operations
//!
//! ## Design Principles
//!
//! - **Simplicity**: Minimal abstraction over core Game
//! - **Performance**: Synchronous operations, no async overhead
//! - **Game Engine Appropriate**: No enterprise patterns, just what's needed

pub mod session;

// Re-export key types for easier consumption
pub use session::{GameSession, SessionError, SessionManager};
