//! Domain Interfaces
//!
//! This module defines the interfaces (traits) that external layers must implement
//! to integrate with the domain layer. Following the Dependency Inversion Principle,
//! the domain layer depends only on these abstractions, not on concrete implementations.

pub mod game_repository;
pub mod state_notifier;

pub use game_repository::GameRepository;
pub use state_notifier::{ActionResult, StateNotifier};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interfaces_are_available() {
        // Test that all interfaces can be imported
        let _repository_type = std::marker::PhantomData::<dyn GameRepository>;
        let _notifier_type = std::marker::PhantomData::<dyn StateNotifier>;
    }
}
