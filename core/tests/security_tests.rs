//! Comprehensive security tests for array bounds and integer overflow vulnerabilities.
//!
//! This module contains tests specifically designed to validate that the security
//! fixes prevent memory corruption, crashes, and other security issues.

#[cfg(test)]
mod tests {
    use balatro_rs::config::Config;
    use balatro_rs::math_safe::{
        safe_add, safe_divide, safe_multiply, safe_size_for_move_operations, safe_subtract,
        saturating_subtract, validate_array_size, MathError,
    };
    use balatro_rs::space::ActionSpace;

    /// Test that ActionSpace creation handles zero available_max without underflow
    #[test]
    fn test_action_space_zero_available_max() {
        // This should not panic or cause underflow
        let config = Config {
            available_max: 0,
            ..Default::default()
        };
        let action_space = ActionSpace::from(config);

        // Both move arrays should be empty when available_max is 0
        assert_eq!(action_space.move_card_left.len(), 0);
        assert_eq!(action_space.move_card_right.len(), 0);
        assert_eq!(action_space.select_card.len(), 0);
    }

    /// Test that ActionSpace creation handles small available_max values correctly
    #[test]
    fn test_action_space_small_available_max() {
        let config1 = Config {
            available_max: 1,
            ..Default::default()
        };
        let action_space = ActionSpace::from(config1);

        // With 1 card available, no move operations should be possible
        assert_eq!(action_space.move_card_left.len(), 0);
        assert_eq!(action_space.move_card_right.len(), 0);
        assert_eq!(action_space.select_card.len(), 1);

        let config = Config {
            available_max: 2,
            ..Default::default()
        };
        let action_space = ActionSpace::from(config);

        // With 2 cards available, 1 move operation should be possible
        assert_eq!(action_space.move_card_left.len(), 1);
        assert_eq!(action_space.move_card_right.len(), 1);
        assert_eq!(action_space.select_card.len(), 2);
    }

    /// Test edge cases for safe mathematical operations
    #[test]
    fn test_safe_subtract_edge_cases() {
        // Normal subtraction
        assert_eq!(safe_subtract(10, 5), Ok(5));

        // Edge case: equal values
        assert_eq!(safe_subtract(5, 5), Ok(0));

        // Edge case: would underflow
        assert_eq!(safe_subtract(5, 10), Err(MathError::Underflow));
        assert_eq!(safe_subtract(0, 1), Err(MathError::Underflow));

        // Edge case: zero minus zero
        assert_eq!(safe_subtract(0, 0), Ok(0));

        // Edge case: maximum values
        assert_eq!(safe_subtract(usize::MAX, 0), Ok(usize::MAX));
        assert_eq!(safe_subtract(usize::MAX, usize::MAX), Ok(0));
    }

    /// Test safe addition with overflow detection
    #[test]
    fn test_safe_add_overflow() {
        // Normal addition
        assert_eq!(safe_add(5, 3), Ok(8));

        // Edge case: would overflow
        assert_eq!(safe_add(usize::MAX, 1), Err(MathError::Overflow));
        assert_eq!(safe_add(usize::MAX - 1, 2), Err(MathError::Overflow));

        // Edge case: maximum safe values
        assert_eq!(safe_add(usize::MAX - 1, 1), Ok(usize::MAX));
        assert_eq!(safe_add(0, usize::MAX), Ok(usize::MAX));
    }

    /// Test safe multiplication with overflow detection
    #[test]
    fn test_safe_multiply_overflow() {
        // Normal multiplication
        assert_eq!(safe_multiply(5, 3), Ok(15));

        // Edge case: would overflow
        if usize::BITS >= 64 {
            assert_eq!(safe_multiply(usize::MAX, 2), Err(MathError::Overflow));
        }

        // Edge case: multiplication by zero
        assert_eq!(safe_multiply(usize::MAX, 0), Ok(0));
        assert_eq!(safe_multiply(0, usize::MAX), Ok(0));

        // Edge case: multiplication by one
        assert_eq!(safe_multiply(100, 1), Ok(100));
        assert_eq!(safe_multiply(1, 100), Ok(100));
    }

    /// Test safe division with division by zero detection
    #[test]
    fn test_safe_divide_zero() {
        // Normal division
        assert_eq!(safe_divide(10, 2), Ok(5));

        // Edge case: division by zero
        assert_eq!(safe_divide(10, 0), Err(MathError::DivisionByZero));
        assert_eq!(safe_divide(0, 0), Err(MathError::DivisionByZero));

        // Edge case: zero divided by non-zero
        assert_eq!(safe_divide(0, 5), Ok(0));

        // Edge case: number divided by itself
        assert_eq!(safe_divide(7, 7), Ok(1));
    }

    /// Test saturating arithmetic behavior
    #[test]
    fn test_saturating_subtract() {
        // Normal subtraction
        assert_eq!(saturating_subtract(10, 3), 7);

        // Saturating behavior (returns 0 instead of underflowing)
        assert_eq!(saturating_subtract(3, 10), 0);
        assert_eq!(saturating_subtract(0, 5), 0);
        assert_eq!(saturating_subtract(0, 0), 0);

        // Edge cases
        assert_eq!(saturating_subtract(1, 1), 0);
        assert_eq!(saturating_subtract(usize::MAX, 0), usize::MAX);
    }

    /// Test safe size calculation for move operations
    #[test]
    fn test_safe_size_for_move_operations() {
        // Normal cases
        assert_eq!(safe_size_for_move_operations(5), 4);
        assert_eq!(safe_size_for_move_operations(10), 9);

        // Edge cases that previously caused vulnerabilities
        assert_eq!(safe_size_for_move_operations(0), 0);
        assert_eq!(safe_size_for_move_operations(1), 0);

        // Large values
        assert_eq!(safe_size_for_move_operations(1000), 999);
    }

    /// Test array size validation
    #[test]
    fn test_validate_array_size() {
        // Valid sizes
        assert!(validate_array_size(0, "test").is_ok());
        assert!(validate_array_size(100, "test").is_ok());
        assert!(validate_array_size(10000, "test").is_ok());
        assert!(validate_array_size(1_000_000, "test").is_ok());

        // Invalid sizes (too large)
        assert!(validate_array_size(1_000_001, "test").is_err());
        assert!(validate_array_size(usize::MAX, "test").is_err());
    }

    /// Test Config field access for validation
    #[test]
    fn test_config_available_max_validation() {
        let mut config = Config::default();

        // Valid values should work when set directly
        config.available_max = 0;
        assert_eq!(config.available_max, 0);

        config.available_max = 100;
        assert_eq!(config.available_max, 100);

        config.available_max = 1_000_000;
        assert_eq!(config.available_max, 1_000_000);

        // Verify validation function works
        assert!(validate_array_size(0, "test").is_ok());
        assert!(validate_array_size(100, "test").is_ok());
        assert!(validate_array_size(1_000_000, "test").is_ok());
        assert!(validate_array_size(1_000_001, "test").is_err());
    }

    /// Fuzz-style test for ActionSpace creation with random values
    #[test]
    fn test_action_space_fuzz() {
        use balatro_rs::rng::GameRng;
        let rng = GameRng::for_testing(42);

        // Test 1000 random valid configurations
        for _ in 0..1000 {
            let mut config = Config::default();

            // Use valid range for available_max
            let available_max = rng.gen_range(0..=1000);
            config.available_max = available_max;

            // This should never panic
            let action_space = ActionSpace::from(config);

            // Verify the relationships hold
            assert_eq!(action_space.select_card.len(), available_max);
            assert_eq!(
                action_space.move_card_left.len(),
                available_max.saturating_sub(1)
            );
            assert_eq!(
                action_space.move_card_right.len(),
                available_max.saturating_sub(1)
            );
        }
    }

    /// Property-based test for arithmetic operations
    #[test]
    fn test_arithmetic_properties() {
        use balatro_rs::rng::GameRng;
        let rng = GameRng::for_testing(43);

        for _ in 0..100 {
            let a = rng.gen_range(0..1000);
            let b = rng.gen_range(0..1000);

            // Property: safe_add is commutative for valid operations
            if let (Ok(result1), Ok(result2)) = (safe_add(a, b), safe_add(b, a)) {
                assert_eq!(result1, result2);
            }

            // Property: safe_subtract with same values returns 0
            assert_eq!(safe_subtract(a, a), Ok(0));

            // Property: saturating_subtract never panics
            let _result = saturating_subtract(a, b); // Should never panic

            // Property: safe_size_for_move_operations is always <= input
            assert!(safe_size_for_move_operations(a) <= a);
        }
    }

    /// Boundary condition tests
    #[test]
    fn test_boundary_conditions() {
        // Test zero boundaries
        assert_eq!(safe_size_for_move_operations(0), 0);
        assert_eq!(safe_size_for_move_operations(1), 0);
        assert_eq!(safe_size_for_move_operations(2), 1);

        // Test with ActionSpace creation
        let mut config = Config::default();

        for available_max in 0..=10 {
            config.available_max = available_max;
            let action_space = ActionSpace::from(config.clone());

            // Verify no arrays are created with invalid sizes
            assert!(action_space.move_card_left.len() <= available_max);
            assert!(action_space.move_card_right.len() <= available_max);
            assert_eq!(action_space.select_card.len(), available_max);
        }
    }

    /// Memory safety test - ensure no out-of-bounds access
    #[test]
    fn test_memory_safety() {
        let mut config = Config::default();

        // Test with zero-sized arrays
        config.available_max = 0;
        let action_space = ActionSpace::from(config);

        // These should be empty vectors, safe to iterate over
        if action_space.move_card_left.first().is_some() {
            panic!("Should not iterate over empty vector");
        }
        if action_space.move_card_right.first().is_some() {
            panic!("Should not iterate over empty vector");
        }
        if action_space.select_card.first().is_some() {
            panic!("Should not iterate over empty vector");
        }
    }
}
