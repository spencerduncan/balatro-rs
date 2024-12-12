use pyo3::pyclass;

/// Types of blinds
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "python", pyclass(eq))]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Copy)]
pub enum Blind {
    Small,
    Big,
    Boss,
}

impl Blind {
    /// reward is money earned for beating the blind
    pub fn reward(&self) -> usize {
        match self {
            Self::Small => 3,
            Self::Big => 4,
            Self::Boss => 5,
        }
    }

    pub fn next(&self) -> Self {
        match self {
            Self::Small => Self::Big,
            Self::Big => Self::Boss,
            Self::Boss => Self::Small,
        }
    }
}

/// Game ending
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "python", pyclass(eq))]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Copy)]
pub enum End {
    Win,
    Lose,
}

/// Stages of playing.
// Playing through an ante looks like:
// Pre -> Small -> Post -> Shop -> Pre -> Big -> Post -> Shop -> Boss -> Post -> Shop
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "python", pyclass)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Copy)]
pub enum Stage {
    // See blind conditions, choose blind (or skip blind)
    PreBlind(),
    // Play blind
    Blind(Blind),
    // Collect payout, optionally play consumables
    PostBlind(),
    // Buy jokers, consumables
    Shop(),
    // Game ending
    End(End),
}

impl Stage {
    pub fn is_blind(&self) -> bool {
        return match self {
            Stage::Blind(_) => true,
            _ => false,
        };
    }
}
