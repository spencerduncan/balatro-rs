//! Value Objects for the Domain Layer
//!
//! Value objects are immutable objects that represent concepts in the domain
//! purely by their values. They encapsulate validation logic and provide
//! type safety for the domain.

pub mod money;
pub mod score;
pub mod session_id;
pub mod validation_result;

pub use money::Money;
pub use score::Score;
pub use session_id::{SessionId, SessionIdError};
pub use validation_result::{ValidationError, ValidationResult};

/// Common trait for value objects that can be safely constructed
pub trait ValueObject: Clone + PartialEq + Eq + std::fmt::Debug + std::fmt::Display {
    /// The raw value type this object wraps
    type Value;

    /// Attempt to create the value object with validation
    fn try_new(value: Self::Value) -> Result<Self, ValidationError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_objects_are_exported() {
        // Test that all value objects can be imported
        let _money_type = std::marker::PhantomData::<Money>;
        let _score_type = std::marker::PhantomData::<Score>;
        let _session_type = std::marker::PhantomData::<SessionId>;
        let _validation_type = std::marker::PhantomData::<ValidationResult>;
        let _error_type = std::marker::PhantomData::<ValidationError>;
    }
}