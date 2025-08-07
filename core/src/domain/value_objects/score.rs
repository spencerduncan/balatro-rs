//! Score Value Object
//!
//! Score represents game scoring with business rules enforcement.
//! It ensures scores are non-negative and provides safe operations.

use std::fmt;

/// Game score value object
///
/// Score ensures all score values are non-negative and provides
/// safe arithmetic operations for score calculations.
///
/// # Examples
///
/// ```
/// use balatro_rs::domain::Score;
///
/// // Create score values
/// let base_score = Score::new(1000);
/// let bonus = Score::new(500);
///
/// // Combine scores
/// let total = base_score.add(bonus);
/// assert_eq!(total.value(), 1500);
///
/// // Apply multiplier
/// let doubled = total.multiply(2.0);
/// assert_eq!(doubled.value(), 3000);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Score(u64);

impl Score {
    /// Create new Score with the given value
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    /// Create Score with zero value
    pub fn zero() -> Self {
        Self(0)
    }

    /// Get the score value
    pub fn value(&self) -> u64 {
        self.0
    }

    /// Check if score is zero
    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }

    /// Add score values
    pub fn add(&self, other: Score) -> Self {
        Self(self.0.saturating_add(other.0))
    }

    /// Multiply score by a factor
    pub fn multiply(&self, factor: f64) -> Self {
        if factor < 0.0 {
            Self(0)
        } else {
            let result = (self.0 as f64 * factor) as u64;
            Self(result)
        }
    }

    /// Apply chips and mult values (typical Balatro scoring)
    pub fn from_chips_and_mult(chips: u64, mult: u64) -> Self {
        Self(chips.saturating_mul(mult))
    }

    /// Check if this score beats a target
    pub fn beats(&self, target: Score) -> bool {
        self.0 > target.0
    }

    /// Check if this score meets or beats a target
    pub fn meets(&self, target: Score) -> bool {
        self.0 >= target.0
    }
}

impl Default for Score {
    fn default() -> Self {
        Self::zero()
    }
}

impl fmt::Display for Score {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Format with thousands separators for readability
        let s = self.0.to_string();
        let mut result = String::new();
        let mut count = 0;

        for c in s.chars().rev() {
            if count == 3 {
                result.insert(0, ',');
                count = 0;
            }
            result.insert(0, c);
            count += 1;
        }

        write!(f, "{result}")
    }
}

impl From<u32> for Score {
    fn from(value: u32) -> Self {
        Self(value as u64)
    }
}

impl From<u64> for Score {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl std::ops::Add for Score {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0.saturating_add(rhs.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn score_basic_operations() {
        let s1 = Score::new(1000);
        let s2 = Score::new(500);

        assert_eq!(s1.value(), 1000);
        assert_eq!(s1.add(s2).value(), 1500);
        assert_eq!((s1 + s2).value(), 1500);
    }

    #[test]
    fn score_multiplication() {
        let score = Score::new(100);

        assert_eq!(score.multiply(2.0).value(), 200);
        assert_eq!(score.multiply(0.5).value(), 50);
        assert_eq!(score.multiply(-1.0).value(), 0); // Negative factors become 0
    }

    #[test]
    fn score_from_chips_and_mult() {
        let score = Score::from_chips_and_mult(50, 10);
        assert_eq!(score.value(), 500);
    }

    #[test]
    fn score_comparisons() {
        let target = Score::new(1000);
        let low = Score::new(500);
        let high = Score::new(1500);
        let equal = Score::new(1000);

        assert!(!low.beats(target));
        assert!(!low.meets(target));
        assert!(high.beats(target));
        assert!(high.meets(target));
        assert!(!equal.beats(target));
        assert!(equal.meets(target));
    }

    #[test]
    fn score_display_formatting() {
        assert_eq!(format!("{}", Score::new(0)), "0");
        assert_eq!(format!("{}", Score::new(100)), "100");
        assert_eq!(format!("{}", Score::new(1000)), "1,000");
        assert_eq!(format!("{}", Score::new(1000000)), "1,000,000");
    }

    #[test]
    fn score_zero_checks() {
        let zero = Score::zero();
        let nonzero = Score::new(1);

        assert!(zero.is_zero());
        assert!(!nonzero.is_zero());
    }

    #[test]
    fn score_ordering() {
        let s1 = Score::new(100);
        let s2 = Score::new(200);
        let s3 = Score::new(200);

        assert!(s1 < s2);
        assert!(s2 > s1);
        assert!(s2 == s3);
    }
}
