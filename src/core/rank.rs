pub struct Score {
    chips: usize,
    mult: usize,
}

/// All the different possible hand ranks.
/// For each hand rank the u32 corresponds to
/// the strength of the hand in comparison to others
/// of the same rank.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Copy)]
pub enum HandRank {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
    RoyalFlush,
    FiveOfAKind,
    FlushHouse,
    FlushFive,
}

impl HandRank {
    pub fn score(&self) -> Score {
        match self {
            Self::HighCard => Score { chips: 5, mult: 1 },
            Self::OnePair => Score { chips: 10, mult: 2 },
            Self::TwoPair => Score { chips: 20, mult: 2 },
            Self::ThreeOfAKind => Score { chips: 30, mult: 3 },
            Self::Straight => Score { chips: 30, mult: 4 },
            Self::Flush => Score { chips: 35, mult: 4 },
            Self::FullHouse => Score { chips: 40, mult: 4 },
            Self::FourOfAKind => Score { chips: 60, mult: 7 },
            Self::StraightFlush => Score {
                chips: 100,
                mult: 8,
            },
            Self::RoyalFlush => Score {
                chips: 100,
                mult: 8,
            },
            Self::FiveOfAKind => Score {
                chips: 120,
                mult: 12,
            },
            Self::FlushHouse => Score {
                chips: 140,
                mult: 14,
            },
            Self::FlushFive => Score {
                chips: 160,
                mult: 16,
            },
        }
    }
}
