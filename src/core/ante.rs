/// All ante levels.
// Goes above 8 for endless mode.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Copy)]
pub enum Ante {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}

impl Ante {
    // Base chip requirement.
    // Eventually this depends on deck and stake.
    pub fn base(&self) -> usize {
        match self {
            Self::Zero => 100,
            Self::One => 300,
            Self::Two => 800,
            Self::Three => 2000,
            Self::Four => 5000,
            Self::Five => 11000,
            Self::Six => 20000,
            Self::Seven => 35000,
            Self::Eight => 50000,
        }
    }
}
