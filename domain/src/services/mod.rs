//! Domain Services
//!
//! This module contains domain services that coordinate business logic
//! and orchestrate interactions between entities. Services handle complex
//! business operations that don't naturally belong to a single entity.

pub mod action_validator;

pub use action_validator::{ActionValidator, BalatroActionValidator};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn services_are_available() {
        // Test that all services can be imported
        let _validator_type = std::marker::PhantomData::<dyn ActionValidator>;
    }
}
