//! Score value object - simplified version

use std::fmt;

/// Score represents game points
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Score(u64);

impl Score {
    /// Create a new Score
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    /// Get the raw value
    pub fn value(&self) -> u64 {
        self.0
    }

    /// Add to score
    pub fn add(&self, points: u64) -> Self {
        Self(self.0.saturating_add(points))
    }
}

impl fmt::Display for Score {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score_operations() {
        let score = Score::new(100);
        assert_eq!(score.add(50).value(), 150);
    }
}
