//! Money Value Object
//!
//! Money represents in-game currency with business rules enforcement.
//! It ensures money values are always non-negative and provides safe arithmetic operations.

use std::fmt;

/// In-game currency value object
///
/// Money ensures all currency values are non-negative and provides
/// safe arithmetic operations that prevent underflow.
///
/// # Examples
///
/// ```
/// use balatro_rs::domain::Money;
///
/// // Create money values
/// let wallet = Money::new(100);
/// let price = Money::new(30);
///
/// // Safe arithmetic
/// let remaining = wallet.subtract(price).unwrap();
/// assert_eq!(remaining.amount(), 70);
///
/// // Prevent negative money
/// assert!(Money::try_new(-10).is_err());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Money(i32);

impl Money {
    /// Create new Money with the given amount
    /// Defaults to 0 if negative value provided (for safety)
    pub fn new(amount: i32) -> Self {
        Self(amount.max(0))
    }

    /// Try to create Money, failing if negative
    pub fn try_new(amount: i32) -> Result<Self, String> {
        if amount < 0 {
            Err(format!("Money cannot be negative: {amount}"))
        } else {
            Ok(Self(amount))
        }
    }

    /// Create Money with zero value
    pub fn zero() -> Self {
        Self(0)
    }

    /// Get the amount as i32
    pub fn amount(&self) -> i32 {
        self.0
    }

    /// Check if money is zero
    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }

    /// Add money amounts
    pub fn add(&self, other: Money) -> Self {
        Self(self.0.saturating_add(other.0))
    }

    /// Subtract money amounts, returns None if would go negative
    pub fn subtract(&self, other: Money) -> Option<Self> {
        if self.0 >= other.0 {
            Some(Self(self.0 - other.0))
        } else {
            None
        }
    }

    /// Multiply money by a factor
    pub fn multiply(&self, factor: u32) -> Self {
        Self(self.0.saturating_mul(factor as i32))
    }

    /// Check if can afford the given amount
    pub fn can_afford(&self, cost: Money) -> bool {
        self.0 >= cost.0
    }
}

impl Default for Money {
    fn default() -> Self {
        Self::zero()
    }
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "${}", self.0)
    }
}

impl From<u32> for Money {
    fn from(value: u32) -> Self {
        Self(value as i32)
    }
}

impl std::ops::Add for Money {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0.saturating_add(rhs.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn money_new_clamps_negative_to_zero() {
        let money = Money::new(-10);
        assert_eq!(money.amount(), 0);
    }

    #[test]
    fn money_try_new_rejects_negative() {
        assert!(Money::try_new(-1).is_err());
        assert!(Money::try_new(0).is_ok());
        assert!(Money::try_new(100).is_ok());
    }

    #[test]
    fn money_arithmetic_operations() {
        let m1 = Money::new(100);
        let m2 = Money::new(50);

        // Addition
        assert_eq!(m1.add(m2).amount(), 150);
        assert_eq!((m1 + m2).amount(), 150);

        // Subtraction
        assert_eq!(m1.subtract(m2).unwrap().amount(), 50);
        assert!(m2.subtract(m1).is_none());

        // Multiplication
        assert_eq!(m1.multiply(3).amount(), 300);
    }

    #[test]
    fn money_can_afford() {
        let wallet = Money::new(100);
        let cheap = Money::new(50);
        let expensive = Money::new(150);

        assert!(wallet.can_afford(cheap));
        assert!(!wallet.can_afford(expensive));
        assert!(wallet.can_afford(wallet)); // Can afford exact amount
    }

    #[test]
    fn money_display() {
        let money = Money::new(42);
        assert_eq!(format!("{money}"), "$42");
    }

    #[test]
    fn money_ordering() {
        let m1 = Money::new(10);
        let m2 = Money::new(20);
        let m3 = Money::new(20);

        assert!(m1 < m2);
        assert!(m2 > m1);
        assert!(m2 == m3);
    }
}
