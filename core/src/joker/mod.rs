use crate::card::Card;
use crate::hand::{Hand, SelectHand};
use crate::stage::Stage;
use pyo3::pyclass;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Enum representing all 150 joker identifiers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "python", pyclass(eq))]
pub enum JokerId {
    // Basic scoring jokers (Common)
    Joker,
    GreedyJoker,
    LustyJoker,
    WrathfulJoker,
    GluttonousJoker,
    JollyJoker,
    ZanyJoker,
    MadJoker,
    CrazyJoker,
    DrollJoker,
    SlyJoker,
    WilyJoker,
    CleverJoker,
    DeviousJoker,
    CraftyJoker,

    // Multiplicative jokers (Common/Uncommon)
    HalfJoker,
    AbstractJoker,
    AcrobatJoker,
    MysticalJoker,
    Misprint,
    RaisedFist,
    SteelJoker,
    FibonacciJoker,
    ScaryFace,
    RoughGem,
    PolishedJoker,

    // Conditional jokers (Common/Uncommon)
    Banner,
    EvenSteven,
    OddTodd,
    Scholar,
    Walkie,
    Runner,
    IceCream,
    DNA,
    SplashJoker,
    Hack,
    Pareidolia,
    Supernova,
    Ride,
    SpaceJoker,
    EggJoker,
    Burglar,
    Blackboard,
    Constellation,
    Hiker,
    FacelessJoker,
    GreenJoker,
    RedCard,
    BlueJoker,
    Erosion,
    Square,
    Ceremonial,
    Smiley,
    Onyx,
    Arrowhead,
    Bloodstone,
    TheIdol,
    SeeingDouble,
    Matador,
    HitTheRoad,
    TheDuo,
    TheTrip,
    TheFamily,
    TheOrder,
    TheClub,

    // Economy jokers (Common/Uncommon)
    DelayedGratification,
    RocketShip,
    ToTheMoon,
    MailInRebate,
    ToDo,
    Cartomancer,
    Astronomer,
    SatelliteJoker,
    ShootTheMoon,
    BusinessCard,
    Luchador,
    Photograph,
    GiftCard,
    CouponJoker,
    CloudNine,
    StockBroker,
    EggHead,
    Bootstraps,
    BullMarket,
    SeedMoney,

    // Retrigger jokers (Uncommon/Rare)
    Dusk,
    Seltzer,
    Hanging,
    Sock,
    Midas,

    // Effect jokers (Uncommon/Rare)
    Superposition,
    FourFingers,
    Shortcut,
    Hologram,
    VagabondJoker,
    BaronJoker,
    Obelisk,
    Midas2,
    LuckyCharm,
    BaseballCard,
    CavendishJoker,
    GrossMichel,
    TradingCard,
    FlashCard,
    Popcorn,
    AncientJoker,
    Ramen,
    WalnutJoker,
    Trousers,
    Swashbuckler,
    TroubadourJoker,
    Certificate,
    SmilingMask,
    FaceMask,
    Fortune,
    Juggler,
    Drunkard,
    Stone,
    GoldenTicket,
    MrBones,
    Acrobat2,
    Loyalty,
    Mystic,

    // Special/Legendary jokers
    MarbleJoker,
    Vampire,
    MadnesJoker,
    Oops,
    IceCream2,
    BrainstormJoker,
    Driver,
    Blueprint,
    Wee,
    Merry,
    Gros,
    Reserved,
    Reserved2,
    Reserved3,
    Reserved4,
    Reserved5,
    Reserved6,
    Cartomancer2,
    Astronomer2,
    BurglarJoker,
    Burnt,
    BootstrapBill,
    Caino,
    Triboulet,
    Yorick,
    Chicot,
    Perkeo,
    CreditCard,
    SockAndBuskin,
    TroubadourJoker2,
    Brainstorm,
    Invisible,

    // Modded/Extras (for future expansion to 150)
    Reserved7,
    Reserved8,
    Reserved9,
    Reserved10,
}

/// Joker rarity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum JokerRarity {
    Common,
    Uncommon,
    Rare,
    Legendary,
}

impl fmt::Display for JokerRarity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Common => write!(f, "Common"),
            Self::Uncommon => write!(f, "Uncommon"),
            Self::Rare => write!(f, "Rare"),
            Self::Legendary => write!(f, "Legendary"),
        }
    }
}

/// Effect that a joker can apply
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JokerEffect {
    /// Additional chips to add
    pub chips: i32,
    /// Additional mult to add
    pub mult: i32,
    /// Additional money to add
    pub money: i32,
    /// Multiplier to apply to mult (e.g., 2.0 for 2x mult)
    pub mult_multiplier: f32,
    /// Number of times to retrigger
    pub retrigger: u32,
    /// Whether to destroy this joker
    pub destroy_self: bool,
    /// Other jokers to destroy
    pub destroy_others: Vec<JokerId>,
    /// Cards to transform
    pub transform_cards: Vec<(Card, Card)>,
    /// New hand size modifier
    pub hand_size_mod: i32,
    /// New discard modifier  
    pub discard_mod: i32,
    /// Custom message to display
    pub message: Option<String>,
}

impl JokerEffect {
    /// Create a new empty effect
    pub fn new() -> Self {
        Self::default()
    }

    /// Add chips
    pub fn with_chips(mut self, chips: i32) -> Self {
        self.chips = chips;
        self
    }

    /// Add mult
    pub fn with_mult(mut self, mult: i32) -> Self {
        self.mult = mult;
        self
    }

    /// Add money
    pub fn with_money(mut self, money: i32) -> Self {
        self.money = money;
        self
    }

    /// Set mult multiplier
    pub fn with_mult_multiplier(mut self, multiplier: f32) -> Self {
        self.mult_multiplier = multiplier;
        self
    }
}

/// Context provided to joker methods for accessing game state
#[derive(Debug)]
pub struct GameContext<'a> {
    /// Current chips
    pub chips: i32,
    /// Current mult
    pub mult: i32,
    /// Current money
    pub money: i32,
    /// Current ante
    pub ante: u8,
    /// Current round
    pub round: u32,
    /// Current stage
    pub stage: &'a Stage,
    /// Number of hands played this round
    pub hands_played: u32,
    /// Number of discards used this round
    pub discards_used: u32,
    /// All jokers in play
    pub jokers: &'a [Box<dyn Joker>],
    /// Cards in hand
    pub hand: &'a Hand,
    /// Discarded cards
    pub discarded: &'a [Card],
}

/// Core trait that all jokers must implement
pub trait Joker: Send + Sync + std::fmt::Debug {
    /// Get the unique identifier for this joker
    fn id(&self) -> JokerId;

    /// Get the display name
    fn name(&self) -> &str;

    /// Get the description
    fn description(&self) -> &str;

    /// Get the rarity
    fn rarity(&self) -> JokerRarity;

    /// Get the base cost in the shop
    fn cost(&self) -> usize {
        match self.rarity() {
            JokerRarity::Common => 3,
            JokerRarity::Uncommon => 6,
            JokerRarity::Rare => 8,
            JokerRarity::Legendary => 20,
        }
    }

    // Lifecycle hooks with default implementations

    /// Called when a hand is played and scored
    fn on_hand_played(&self, _context: &mut GameContext, _hand: &SelectHand) -> JokerEffect {
        JokerEffect::new()
    }

    /// Called for each card as it's scored
    fn on_card_scored(&self, _context: &mut GameContext, _card: &Card) -> JokerEffect {
        JokerEffect::new()
    }

    /// Called when a new blind starts
    fn on_blind_start(&self, _context: &mut GameContext) -> JokerEffect {
        JokerEffect::new()
    }

    /// Called when the shop opens
    fn on_shop_open(&self, _context: &mut GameContext) -> JokerEffect {
        JokerEffect::new()
    }

    /// Called when cards are discarded
    fn on_discard(&self, _context: &mut GameContext, _cards: &[Card]) -> JokerEffect {
        JokerEffect::new()
    }

    /// Called at the end of each round
    fn on_round_end(&self, _context: &mut GameContext) -> JokerEffect {
        JokerEffect::new()
    }

    // Modifier hooks with default implementations

    /// Modify base chips
    fn modify_chips(&self, _context: &GameContext, base_chips: i32) -> i32 {
        base_chips
    }

    /// Modify base mult
    fn modify_mult(&self, _context: &GameContext, base_mult: i32) -> i32 {
        base_mult
    }

    /// Modify hand size
    fn modify_hand_size(&self, _context: &GameContext, base_size: usize) -> usize {
        base_size
    }

    /// Modify number of discards
    fn modify_discards(&self, _context: &GameContext, base_discards: usize) -> usize {
        base_discards
    }
}

// Re-export the old Categories enum for compatibility
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Categories {
    MultPlus,
    MultMult,
    Chips,
    Economy,
    Retrigger,
    Effect,
}

// Include the compatibility module for the old API
pub mod compat;

// Include the conditional joker framework
pub mod conditional;

// Re-export important types
pub use conditional::{ConditionalJoker, JokerCondition};

// Re-export old API types for backwards compatibility
pub use compat::{Joker as OldJoker, Jokers};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_joker_trait_bounds() {
        // This won't compile if Joker doesn't have Send + Sync bounds
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<Box<dyn Joker>>();
    }
}
