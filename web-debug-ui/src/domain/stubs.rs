//! Stub implementations for testing when balatro-rs core is not available
//!
//! This module provides minimal stub implementations of balatro-rs types
//! to enable testing of the domain layer in isolation. These stubs should
//! only be used for testing and not in production code.

// Stub implementations for domain layer when balatro-rs core is not available

/// Stub implementation of balatro_rs::Action for testing
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Action {
    Play(),
    Discard(),
    SelectCard(Card),
    DeselectCard(Card),
    NextRound(),
    SelectBlind(Blind),
    RerollShop(),
    BuyJoker {
        joker_id: JokerId,
        slot: usize,
    },
    BuyConsumable {
        consumable_id: ConsumableId,
        slot: usize,
    },
    BuyVoucher {
        voucher_id: VoucherId,
    },
    BuyPack {
        pack_id: PackId,
    },
}

/// Stub implementation of balatro_rs::Game for testing
#[derive(Debug, Clone)]
pub struct Game {
    pub stage: Stage,
    pub available: Available,
    pub plays: f64,
    pub discards: f64,
    pub money: f64,
    pub score: f64,
    pub shop: Shop,
    pub shop_reroll_cost: f64,
    pub ante_current: AnteWrapper,
    pub round: f64,
}

/// Wrapper for ante to simulate balatro_rs::Ante structure
#[derive(Debug, Clone)]
pub struct AnteWrapper(pub u32);

impl AnteWrapper {
    pub fn new(ante: u32) -> Self {
        Self(ante)
    }
}

impl Game {
    pub fn new(_config: Config) -> Self {
        Self {
            stage: Stage::PreBlind,
            available: Available {
                cards: create_test_cards(),
                selected: Vec::new(),
            },
            plays: 3.0,
            discards: 3.0,
            money: 100.0,
            score: 0.0,
            shop: Shop::default(),
            shop_reroll_cost: 5.0,
            ante_current: AnteWrapper::new(1),
            round: 1.0,
        }
    }

    pub fn handle_action(&mut self, _action: Action) -> Result<(), GameError> {
        Ok(())
    }

    /// Generate available actions (stub implementation)
    pub fn gen_actions(&self) -> impl Iterator<Item = Action> {
        vec![Action::Play(), Action::Discard(), Action::NextRound()].into_iter()
    }

    /// Start the game (stub implementation)
    pub fn start(&mut self) {
        // Stub implementation - just set stage to PreBlind
        self.stage = Stage::PreBlind;
    }
}

/// Implement Default for Game
impl Default for Game {
    fn default() -> Self {
        Self::new(Config)
    }
}

/// Stub implementation of balatro_rs::stage::Stage for testing
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Stage {
    PreBlind,
    Blind,
    PostBlind,
    Shop,
    End,
}

/// Stub implementation of balatro_rs::Available for testing
#[derive(Debug, Clone, Default)]
pub struct Available {
    pub cards: Vec<Card>,
    pub selected: Vec<Card>,
}

/// Stub implementation of balatro_rs::Card for testing
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

/// Stub implementation of balatro_rs::Rank for testing
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rank {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

/// Stub implementation of balatro_rs::Suit for testing
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

/// Stub implementation of balatro_rs::stage::Blind for testing
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Blind {
    Small,
    Big,
    Boss,
}

/// Stub implementation of balatro_rs::Shop for testing
#[derive(Debug, Clone, Default)]
pub struct Shop {
    pub jokers: ShopSlots<JokerId>,
}

/// Stub implementation of shop slots
#[derive(Debug, Clone)]
pub struct ShopSlots<T> {
    slots: Vec<Option<T>>,
}

impl<T: Clone> Default for ShopSlots<T> {
    fn default() -> Self {
        Self {
            slots: vec![None; 5], // Default 5 slots
        }
    }
}

impl<T> ShopSlots<T> {
    pub fn capacity(&self) -> usize {
        self.slots.len()
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.slots.get(index)?.as_ref()
    }
}

/// Stub implementation of balatro_rs::JokerId for testing
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum JokerId {
    Joker,
}

/// Stub implementation of balatro_rs::ConsumableId for testing
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConsumableId {
    Tarot,
}

/// Stub implementation of balatro_rs::VoucherId for testing
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VoucherId {
    TestVoucher,
}

/// Stub implementation of balatro_rs::PackId for testing
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PackId {
    TestPack,
}

/// Stub implementation of balatro_rs::Config for testing
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Config;

/// Stub implementation of balatro_rs::GameError for testing
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GameError {
    InvalidAction,
}

impl std::fmt::Display for GameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidAction => write!(f, "Invalid action"),
        }
    }
}

impl std::error::Error for GameError {}

// Helper function to create test cards
pub fn create_test_cards() -> Vec<Card> {
    vec![
        Card {
            rank: Rank::Ace,
            suit: Suit::Spades,
        },
        Card {
            rank: Rank::King,
            suit: Suit::Hearts,
        },
        Card {
            rank: Rank::Queen,
            suit: Suit::Diamonds,
        },
        Card {
            rank: Rank::Jack,
            suit: Suit::Clubs,
        },
        Card {
            rank: Rank::Ten,
            suit: Suit::Spades,
        },
    ]
}
