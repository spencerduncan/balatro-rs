/// Types of blinds
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Copy)]
pub enum Blind {
    Small,
    Big,
    Boss,
}

/// Game ending
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Copy)]
pub enum End {
    Win,
    Lose,
}

/// Stages of playing.
// Playing through an ante looks like:
// Pre -> Small -> Post -> Shop -> Pre -> Big -> Post -> Shop -> Boss -> Post -> Shop
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Copy)]
pub enum Stage {
    // See blind conditions, choose blind (or skip blind)
    PreBlind,
    // Play blind
    Blind(Blind),
    // Collect payout, optionally play consumables
    PostBlind,
    // Buy jokers, consumables
    Shop,
    // Game ending
    End(End),
}
