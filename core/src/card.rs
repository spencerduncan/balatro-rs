#[cfg(feature = "colored")]
use colored::Colorize;
#[cfg(feature = "python")]
use pyo3::pyclass;
use std::{
    fmt,
    sync::atomic::{AtomicUsize, Ordering},
};

// Useful balatro docs: https://balatrogame.fandom.com/wiki/Card_Ranks

/// Card rank or value.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "python", pyclass(eq))]
#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Clone, Copy, Hash)]
pub enum Value {
    Two = 0,
    Three = 1,
    Four = 2,
    Five = 3,
    Six = 4,
    Seven = 5,
    Eight = 6,
    Nine = 7,
    Ten = 8,
    Jack = 9,
    Queen = 10,
    King = 11,
    Ace = 12,
}

/// Constant of all the values.
/// This is what `Value::values()` returns
const VALUES: [Value; 13] = [
    Value::Two,
    Value::Three,
    Value::Four,
    Value::Five,
    Value::Six,
    Value::Seven,
    Value::Eight,
    Value::Nine,
    Value::Ten,
    Value::Jack,
    Value::Queen,
    Value::King,
    Value::Ace,
];

impl Value {
    pub const fn values() -> [Self; 13] {
        VALUES
    }

    /// Convert from u8 index to Value enum variant
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Value::Two),
            1 => Some(Value::Three),
            2 => Some(Value::Four),
            3 => Some(Value::Five),
            4 => Some(Value::Six),
            5 => Some(Value::Seven),
            6 => Some(Value::Eight),
            7 => Some(Value::Nine),
            8 => Some(Value::Ten),
            9 => Some(Value::Jack),
            10 => Some(Value::Queen),
            11 => Some(Value::King),
            12 => Some(Value::Ace),
            _ => None,
        }
    }
}

impl From<Value> for char {
    fn from(value: Value) -> Self {
        match value {
            Value::Two => '2',
            Value::Three => '3',
            Value::Four => '4',
            Value::Five => '5',
            Value::Six => '6',
            Value::Seven => '7',
            Value::Eight => '8',
            Value::Nine => '9',
            Value::Ten => 'T',
            Value::Jack => 'J',
            Value::Queen => 'Q',
            Value::King => 'K',
            Value::Ace => 'A',
        }
    }
}

/// Enum for the four different suits.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "python", pyclass(eq))]
#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Clone, Copy, Hash)]
pub enum Suit {
    Spade = 0,
    Club = 1,
    Heart = 2,
    Diamond = 3,
}

/// All of the `Suit`'s. This is what `Suit::suits()` returns.
const SUITS: [Suit; 4] = [Suit::Spade, Suit::Club, Suit::Heart, Suit::Diamond];

impl Suit {
    pub const fn suits() -> [Self; 4] {
        SUITS
    }
    pub fn unicode(&self) -> &str {
        match self {
            Self::Spade => "♤",
            Self::Club => "♧",
            Self::Heart => "♡",
            Self::Diamond => "♢",
        }
    }
}

impl From<Suit> for char {
    fn from(value: Suit) -> Self {
        match value {
            Suit::Spade => 's',
            Suit::Club => 'c',
            Suit::Heart => 'h',
            Suit::Diamond => 'd',
        }
    }
}

/// Enum for card  enhancements
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "python", pyclass(eq))]
#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Clone, Copy, Hash)]
pub enum Enhancement {
    Bonus,
    Mult,
    Wild,
    Glass,
    Steel,
    Stone,
    Gold,
    Lucky,
}

/// Enum for card  editions
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "python", pyclass(eq))]
#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Clone, Copy, Hash)]
pub enum Edition {
    Base,
    Foil,
    Holographic,
    Polychrome,
    Negative,
}

/// Enum for card seals
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "python", pyclass(eq))]
#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Clone, Copy, Hash)]
pub enum Seal {
    Gold,
    Red,
    Blue,
    Purple,
}

// Each card gets a unique id. Not sure this is strictly
// necessary but it makes identifying otherwise identical cards
// possible (i.e. for trashing, reordering, etc)
static CARD_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "python", pyclass)]
#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Hash)]
pub struct Card {
    pub value: Value,
    pub suit: Suit,
    pub id: usize,
    pub edition: Edition,
    pub enhancement: Option<Enhancement>,
    pub seal: Option<Seal>,
}

impl Card {
    pub fn new(value: Value, suit: Suit) -> Self {
        let id = CARD_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        Self {
            value,
            suit,
            id,
            edition: Edition::Base,
            enhancement: None,
            seal: None,
        }
    }

    pub fn is_face(&self) -> bool {
        matches!(self.value, Value::Jack | Value::Queen | Value::King)
    }

    pub fn is_even(&self) -> bool {
        self.value != Value::Ace && !self.is_face() && self.value as u16 % 2 == 0
    }

    pub fn is_odd(&self) -> bool {
        self.value == Value::Ace || !self.is_face() && self.value as u16 % 2 != 0
    }

    pub fn chips(&self) -> usize {
        let base_chips = match self.value {
            Value::Two => 1,
            Value::Three => 2,
            Value::Four => 3,
            Value::Five => 4,
            Value::Six => 5,
            Value::Seven => 6,
            Value::Eight => 7,
            Value::Nine => 8,
            Value::Ten => 9,
            Value::Jack => 10,
            Value::Queen => 10,
            Value::King => 10,
            Value::Ace => 11,
        };

        // Add edition bonuses - Foil edition gives +50 chips
        match self.edition {
            Edition::Foil => base_chips + 50,
            _ => base_chips,
        }
    }
}

impl fmt::Debug for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        #[cfg(feature = "colored")]
        let suit = match self.suit {
            Suit::Spade => self.suit.unicode().bold(),
            Suit::Club => self.suit.unicode().green().bold(),
            Suit::Heart => self.suit.unicode().red().bold(),
            Suit::Diamond => self.suit.unicode().blue().bold(),
        };
        #[cfg(not(feature = "colored"))]
        let suit = self.suit.unicode();
        write!(f, "Card({}{})", char::from(self.value), suit)
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        #[cfg(feature = "colored")]
        let suit = match self.suit {
            Suit::Spade => self.suit.unicode().bold(),
            Suit::Club => self.suit.unicode().green().bold(),
            Suit::Heart => self.suit.unicode().red().bold(),
            Suit::Diamond => self.suit.unicode().blue().bold(),
        };
        #[cfg(not(feature = "colored"))]
        let suit = self.suit.unicode();
        write!(f, "{}{}", char::from(self.value), suit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constructor() {
        let c = Card::new(Value::King, Suit::Heart);
        assert_eq!(Value::King, c.value);
        assert_eq!(Suit::Heart, c.suit);
    }

    #[test]
    fn test_face() {
        let king = Card::new(Value::King, Suit::Heart);
        assert!(king.is_face());
        let two = Card::new(Value::Two, Suit::Diamond);
        assert!(!two.is_face());
    }

    #[test]
    fn test_even_odd() {
        // ace is odd
        let ace = Card::new(Value::Ace, Suit::Spade);
        assert!(!ace.is_even());
        assert!(ace.is_odd());

        // two is even
        let two = Card::new(Value::Two, Suit::Diamond);
        assert!(two.is_even());
        assert!(!two.is_odd());

        // three is odd
        let three = Card::new(Value::Three, Suit::Heart);
        assert!(!three.is_even());
        assert!(three.is_odd());

        // ten is even
        let ten = Card::new(Value::Ten, Suit::Heart);
        assert!(ten.is_even());
        assert!(!ten.is_odd());

        //king is neither odd nor even
        let king = Card::new(Value::King, Suit::Club);
        assert!(!king.is_even());
        assert!(!king.is_odd());
    }

    #[test]
    fn test_edition_bonus_foil_chips() {
        // Test Foil edition gives +50 chips bonus
        let mut card = Card::new(Value::Ace, Suit::Heart);
        card.edition = Edition::Foil;

        // Ace normally gives 11 chips, Foil should add 50 for total of 61
        assert_eq!(card.chips(), 61);

        // Test with different card values
        let mut two = Card::new(Value::Two, Suit::Spade);
        two.edition = Edition::Foil;
        assert_eq!(two.chips(), 51); // 1 + 50

        let mut king = Card::new(Value::King, Suit::Diamond);
        king.edition = Edition::Foil;
        assert_eq!(king.chips(), 60); // 10 + 50
    }

    #[test]
    fn test_edition_bonus_base_no_bonus() {
        // Test Base edition gives no bonus
        let card = Card::new(Value::Ace, Suit::Heart);
        assert_eq!(card.edition, Edition::Base); // Default should be Base
        assert_eq!(card.chips(), 11); // No bonus applied

        let two = Card::new(Value::Two, Suit::Spade);
        assert_eq!(two.chips(), 1); // No bonus applied
    }

    #[test]
    fn test_edition_bonus_other_editions_no_chip_bonus() {
        // Test that Holographic, Polychrome, and Negative don't affect chips
        // (they have other bonuses applied elsewhere)
        let mut holo_card = Card::new(Value::Ace, Suit::Heart);
        holo_card.edition = Edition::Holographic;
        assert_eq!(holo_card.chips(), 11); // No chip bonus from Holographic

        let mut poly_card = Card::new(Value::Ace, Suit::Heart);
        poly_card.edition = Edition::Polychrome;
        assert_eq!(poly_card.chips(), 11); // No chip bonus from Polychrome

        let mut neg_card = Card::new(Value::Ace, Suit::Heart);
        neg_card.edition = Edition::Negative;
        assert_eq!(neg_card.chips(), 11); // No chip bonus from Negative
    }

    #[test]
    fn test_edition_bonus_production_edge_cases() {
        // Production-ready edge case testing

        // Test with all card values for Foil edition
        for value in &Value::values() {
            let mut card = Card::new(*value, Suit::Heart);
            card.edition = Edition::Foil;

            let base_chips = Card::new(*value, Suit::Heart).chips();
            assert_eq!(
                card.chips(),
                base_chips + 50,
                "Foil edition should add exactly 50 chips to {value:?}"
            );
        }

        // Test overflow safety (should not panic even with extreme values)
        let mut max_card = Card::new(Value::Ace, Suit::Heart); // Highest base chips (11)
        max_card.edition = Edition::Foil;
        let result = max_card.chips();
        assert_eq!(result, 61);
        assert!(result < usize::MAX, "Should not overflow");
    }
}
