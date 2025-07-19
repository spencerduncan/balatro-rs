//! Safe mathematical operations to prevent integer overflow/underflow vulnerabilities.
//!
//! This module provides secure wrappers for arithmetic operations that could cause
//! memory safety issues or panics in array operations.

use crate::error::GameError;

/// Error types for mathematical operations
#[derive(Debug, Clone, PartialEq)]
pub enum MathError {
    Overflow,
    Underflow,
    DivisionByZero,
    IndexOutOfBounds(String),
}

impl std::fmt::Display for MathError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MathError::Overflow => write!(f, "Integer overflow detected"),
            MathError::Underflow => write!(f, "Integer underflow detected"),
            MathError::DivisionByZero => write!(f, "Division by zero"),
            MathError::IndexOutOfBounds(msg) => write!(f, "Index out of bounds: {msg}"),
        }
    }
}

impl std::error::Error for MathError {}

/// Safe subtraction that prevents underflow
pub fn safe_subtract(a: usize, b: usize) -> Result<usize, MathError> {
    a.checked_sub(b).ok_or(MathError::Underflow)
}

/// Safe addition that prevents overflow
pub fn safe_add(a: usize, b: usize) -> Result<usize, MathError> {
    a.checked_add(b).ok_or(MathError::Overflow)
}

/// Safe multiplication that prevents overflow
pub fn safe_multiply(a: usize, b: usize) -> Result<usize, MathError> {
    a.checked_mul(b).ok_or(MathError::Overflow)
}

/// Safe division that prevents division by zero
pub fn safe_divide(a: usize, b: usize) -> Result<usize, MathError> {
    if b == 0 {
        Err(MathError::DivisionByZero)
    } else {
        Ok(a / b)
    }
}

/// Saturating subtraction (returns 0 if would underflow)
pub fn saturating_subtract(a: usize, b: usize) -> usize {
    a.saturating_sub(b)
}

/// Safe vector/array size calculation for operations like "size - 1"
/// Returns 0 if the input would cause underflow
pub fn safe_size_for_move_operations(size: usize) -> usize {
    saturating_subtract(size, 1)
}

/// Safe array access wrapper
pub fn safe_get<T>(vec: &[T], index: usize) -> Result<&T, MathError> {
    vec.get(index).ok_or_else(|| {
        MathError::IndexOutOfBounds(format!(
            "Index {index} out of bounds for array of length {}",
            vec.len()
        ))
    })
}

/// Safe mutable array access wrapper
pub fn safe_get_mut<T>(vec: &mut [T], index: usize) -> Result<&mut T, MathError> {
    let len = vec.len();
    vec.get_mut(index).ok_or_else(|| {
        MathError::IndexOutOfBounds(format!(
            "Index {index} out of bounds for array of length {len}"
        ))
    })
}

/// Validates a size for array allocation, ensuring it won't cause memory issues
pub fn validate_array_size(size: usize, context: &str) -> Result<(), GameError> {
    // Reasonable upper bound to prevent excessive memory allocation
    const MAX_REASONABLE_SIZE: usize = 1_000_000;

    if size > MAX_REASONABLE_SIZE {
        return Err(GameError::InvalidInput(format!(
            "Array size {size} is too large for {context}: maximum allowed is {MAX_REASONABLE_SIZE}"
        )));
    }

    Ok(())
}

/// Safe range creation for iteration (handles edge cases where end < start)
pub fn safe_range(start: usize, len: usize) -> impl Iterator<Item = usize> {
    if len == 0 {
        0..0
    } else {
        start..(start + len)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_subtract() {
        assert_eq!(safe_subtract(5, 3), Ok(2));
        assert_eq!(safe_subtract(3, 5), Err(MathError::Underflow));
        assert_eq!(safe_subtract(0, 1), Err(MathError::Underflow));
        assert_eq!(safe_subtract(1, 1), Ok(0));
    }

    #[test]
    fn test_saturating_subtract() {
        assert_eq!(saturating_subtract(5, 3), 2);
        assert_eq!(saturating_subtract(3, 5), 0);
        assert_eq!(saturating_subtract(0, 1), 0);
        assert_eq!(saturating_subtract(1, 1), 0);
    }

    #[test]
    fn test_safe_size_for_move_operations() {
        assert_eq!(safe_size_for_move_operations(5), 4);
        assert_eq!(safe_size_for_move_operations(1), 0);
        assert_eq!(safe_size_for_move_operations(0), 0);
    }

    #[test]
    fn test_safe_get() {
        let vec = vec![1, 2, 3];
        assert_eq!(safe_get(&vec, 0), Ok(&1));
        assert_eq!(safe_get(&vec, 2), Ok(&3));
        assert!(safe_get(&vec, 3).is_err());
        assert!(safe_get(&vec, 100).is_err());
    }

    #[test]
    fn test_validate_array_size() {
        assert!(validate_array_size(100, "test").is_ok());
        assert!(validate_array_size(1000, "test").is_ok());
        assert!(validate_array_size(1_000_001, "test").is_err());
    }

    #[test]
    fn test_safe_range() {
        let range: Vec<usize> = safe_range(0, 3).collect();
        assert_eq!(range, vec![0, 1, 2]);

        let empty_range: Vec<usize> = safe_range(0, 0).collect();
        assert_eq!(empty_range, Vec::<usize>::new());

        let range_from_5: Vec<usize> = safe_range(5, 3).collect();
        assert_eq!(range_from_5, vec![5, 6, 7]);
    }
}
