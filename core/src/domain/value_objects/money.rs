//! Money value object - simplified version

use std::fmt;

/// Money represents in-game currency
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Money(f64);

impl Money {
    /// Create a new Money value
    pub fn new(amount: f64) -> Result<Self, String> {
        if amount < 0.0 {
            Err("Money cannot be negative".to_string())
        } else {
            Ok(Self(amount))
        }
    }

    /// Get the raw value
    pub fn value(&self) -> f64 {
        self.0
    }

    /// Add money
    pub fn add(&self, amount: f64) -> Result<Self, String> {
        Self::new(self.0 + amount)
    }

    /// Subtract money
    pub fn subtract(&self, amount: f64) -> Result<Self, String> {
        Self::new(self.0 - amount)
    }
}

impl Default for Money {
    fn default() -> Self {
        Self(0.0)
    }
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "${:.0}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_money_creation() {
        assert!(Money::new(10.0).is_ok());
        assert!(Money::new(-1.0).is_err());
    }
}
