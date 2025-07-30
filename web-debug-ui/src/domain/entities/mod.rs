//! Domain Entities
//!
//! This module contains the core business entities that encapsulate business rules
//! and behavior. Entities are the heart of the domain layer and represent the
//! most important concepts in the business.

pub mod game_session;

pub use game_session::GameSession;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn entities_are_available() {
        // Test that all entities can be imported
        let _session_type = std::marker::PhantomData::<GameSession>;
    }
}