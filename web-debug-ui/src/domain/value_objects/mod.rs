//! Value Objects for the Domain Layer
//!
//! Value objects are immutable objects that represent concepts in the domain
//! purely by their values. They encapsulate validation logic and provide
//! type safety for the domain.

pub mod session_id;
pub mod validation_result;

pub use session_id::SessionId;
pub use validation_result::{ValidationResult, ValidationError};

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn value_objects_are_available() {
        // Test that all value objects can be imported
        let _session_type = std::marker::PhantomData::<SessionId>;
        let _validation_type = std::marker::PhantomData::<ValidationResult>;
        let _error_type = std::marker::PhantomData::<ValidationError>;
    }
}