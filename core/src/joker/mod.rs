use crate::card::Card;
use crate::hand::{Hand, SelectHand};
use crate::joker_state::{JokerState, JokerStateManager};
use crate::rank::HandRank;
use crate::stage::Stage;
#[cfg(feature = "python")]
use pyo3::{pyclass, pymethods};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;

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
#[cfg_attr(feature = "python", pyclass(eq))]
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

/// Effect that a joker can apply to modify game state.
///
/// `JokerEffect` represents the comprehensive impact a joker can have on the game
/// when triggered. Effects can modify scoring, award resources, trigger special
/// mechanics, or even destroy jokers and transform cards.
///
/// # Effect Categories
///
/// ## Scoring Effects
/// - **Chips**: Bonus chips added to hand total (`chips`)
/// - **Mult**: Bonus mult added to hand total (`mult`)
/// - **Mult Multiplier**: Percentage multiplier applied to total mult (`mult_multiplier`)
///
/// ## Resource Effects  
/// - **Money**: Coins awarded to the player (`money`)
/// - **Hand Size**: Temporary hand size modification (`hand_size_mod`)
/// - **Discards**: Temporary discard count modification (`discard_mod`)
///
/// ## Special Effects
/// - **Retrigger**: Number of times to retrigger the effect (`retrigger`)
/// - **Self Destruction**: Whether this joker destroys itself (`destroy_self`)
/// - **Other Destruction**: Other jokers to destroy (`destroy_others`)
/// - **Card Transformation**: Cards to transform into other cards (`transform_cards`)
/// - **Messages**: Custom messages to display (`message`)
///
/// # Application Order
///
/// Effects are applied during scoring in this order:
/// 1. **Chips**: Base chips + all chip bonuses  
/// 2. **Mult**: Base mult + all mult bonuses
/// 3. **Mult Multipliers**: Apply all multipliers to total mult
/// 4. **Final Score**: (Total Chips) × (Total Mult)
/// 5. **Resources**: Award money and apply modifiers
/// 6. **Special Effects**: Handle retriggering, destruction, transformation
///
/// # Examples
///
/// ```rust,ignore
/// use balatro_rs::joker::JokerEffect;
///
/// // Simple scoring bonus
/// let effect = JokerEffect::new().with_mult(5);
///
/// // Combined scoring effect
/// let effect = JokerEffect::new()
///     .with_chips(50)
///     .with_mult(3)
///     .with_money(2);
///
/// // Multiplicative effect
/// let effect = JokerEffect::new().with_mult_multiplier(1.5); // +50% mult
///
/// // Complex effect with multiplicative bonus
/// let effect = JokerEffect::new()
///     .with_mult(10)
///     .with_mult_multiplier(2.0);
/// ```
///
/// # Performance Notes
///
/// `JokerEffect` instances are created frequently during scoring. The builder
/// pattern methods consume and return `self` to enable efficient method chaining
/// without additional allocations.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "python", pyclass)]
pub struct JokerEffect {
    /// Additional chips to add to the hand's base chips.
    ///
    /// Applied before mult calculations. Positive values increase the score.
    pub chips: i32,

    /// Additional mult to add to the hand's base mult.
    ///
    /// Applied before mult multipliers. Positive values increase the score.
    pub mult: i32,

    /// Money to award to the player.
    ///
    /// Applied immediately after hand scoring. Positive values give money.
    pub money: i32,

    /// Multiplier to apply to the total mult.
    ///
    /// Applied after all mult bonuses are summed. For example:
    /// - 1.0 = no change (100%)
    /// - 1.5 = +50% mult
    /// - 2.0 = double mult (200%)
    pub mult_multiplier: f32,

    /// Number of times to retrigger this effect.
    ///
    /// A value of 2 means the effect triggers 3 times total (1 + 2 retriggers).
    pub retrigger: u32,

    /// Whether this joker should destroy itself after applying the effect.
    ///
    /// Used for one-time jokers or jokers with limited uses.
    pub destroy_self: bool,

    /// Other jokers to destroy when this effect is applied.
    ///
    /// Used for jokers that consume other jokers for powerful effects.
    pub destroy_others: Vec<JokerId>,

    /// Cards to transform into other cards.
    ///
    /// Each tuple represents (source_card, target_card). Used for jokers
    /// that modify the deck or hand composition.
    pub transform_cards: Vec<(Card, Card)>,

    /// Temporary modification to hand size.
    ///
    /// Positive values increase hand size, negative values decrease it.
    /// Applied for the current hand only.
    pub hand_size_mod: i32,

    /// Temporary modification to discard count.
    ///
    /// Positive values give extra discards, negative values reduce discards.
    /// Applied for the current round only.
    pub discard_mod: i32,

    /// Sell value increase for this joker
    pub sell_value_increase: i32,

    /// Custom message to display to the player.
    ///
    /// Used for jokers with special effects, Easter eggs, or important notifications.
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

    /// Set sell value increase
    pub fn with_sell_value_increase(mut self, increase: i32) -> Self {
        self.sell_value_increase = increase;
        self
    }

    /// Set custom message
    pub fn with_message(mut self, message: String) -> Self {
        self.message = Some(message);
        self
    }
}

#[cfg_attr(feature = "python", pymethods)]
impl JokerEffect {
    /// Get additional chips bonus
    #[cfg(feature = "python")]
    #[getter]
    fn chips(&self) -> i32 {
        self.chips
    }

    /// Get additional mult bonus
    #[cfg(feature = "python")]
    #[getter]
    fn mult(&self) -> i32 {
        self.mult
    }

    /// Get money awarded
    #[cfg(feature = "python")]
    #[getter]
    fn money(&self) -> i32 {
        self.money
    }

    /// Get mult multiplier
    #[cfg(feature = "python")]
    #[getter]
    fn mult_multiplier(&self) -> f32 {
        self.mult_multiplier
    }

    /// Get retrigger count
    #[cfg(feature = "python")]
    #[getter]
    fn retrigger(&self) -> u32 {
        self.retrigger
    }

    /// Get whether this joker destroys itself
    #[cfg(feature = "python")]
    #[getter]
    fn destroy_self(&self) -> bool {
        self.destroy_self
    }

    /// Get other jokers to destroy
    #[cfg(feature = "python")]
    #[getter]
    fn destroy_others(&self) -> Vec<JokerId> {
        self.destroy_others.clone()
    }

    /// Get card transformations
    #[cfg(feature = "python")]
    #[getter]
    fn transform_cards(&self) -> Vec<(Card, Card)> {
        self.transform_cards.clone()
    }

    /// Get hand size modification
    #[cfg(feature = "python")]
    #[getter]
    fn hand_size_mod(&self) -> i32 {
        self.hand_size_mod
    }

    /// Get discard modification
    #[cfg(feature = "python")]
    #[getter]
    fn discard_mod(&self) -> i32 {
        self.discard_mod
    }

    /// Get sell value increase
    #[cfg(feature = "python")]
    #[getter]
    fn sell_value_increase(&self) -> i32 {
        self.sell_value_increase
    }

    /// Get custom message
    #[cfg(feature = "python")]
    #[getter]
    fn message(&self) -> Option<String> {
        self.message.clone()
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
    /// Joker state manager for persistent state
    pub joker_state_manager: &'a Arc<JokerStateManager>,
    /// Hand type counts for this game run
    pub hand_type_counts: &'a HashMap<HandRank, u32>,
}

impl<'a> GameContext<'a> {
    /// Get the number of times a specific hand type has been played this game run.
    ///
    /// # Arguments
    /// * `hand_rank` - The hand rank to check the count for
    ///
    /// # Returns
    /// The number of times this hand type has been played (0 if never played)
    pub fn get_hand_type_count(&self, hand_rank: HandRank) -> u32 {
        self.hand_type_counts.get(&hand_rank).copied().unwrap_or(0)
    }
}

/// Core trait that all jokers must implement.
///
/// This trait defines the interface for all joker implementations in the Balatro-RS system.
/// Jokers can modify game scoring through lifecycle hooks and provide static information
/// about their identity and behavior.
///
/// # Implementation Patterns
///
/// There are several patterns for implementing jokers:
///
/// ## 1. Direct Implementation
/// For complex jokers requiring custom logic:
/// ```rust,ignore
/// use balatro_rs::joker::{Joker, JokerId, JokerRarity, JokerEffect, GameContext};
/// use balatro_rs::card::Card;
/// use balatro_rs::hand::SelectHand;
///
/// #[derive(Debug)]
/// struct CustomJoker;
///
/// impl Joker for CustomJoker {
///     fn id(&self) -> JokerId { JokerId::CustomJoker }
///     fn name(&self) -> &str { "Custom Joker" }
///     fn description(&self) -> &str { "Complex custom logic" }
///     fn rarity(&self) -> JokerRarity { JokerRarity::Common }
///     
///     fn on_card_scored(&self, context: &mut GameContext, card: &Card) -> JokerEffect {
///         // Custom scoring logic
///         if self.complex_condition(context, card) {
///             JokerEffect::new().with_mult(5)
///         } else {
///             JokerEffect::new()
///         }
///     }
/// }
/// ```
///
/// ## 2. Static Joker Framework
/// For simple conditional jokers:
/// ```rust,ignore
/// use balatro_rs::static_joker::{StaticJoker, StaticCondition};
/// use balatro_rs::card::Suit;
///
/// let greedy_joker = StaticJoker::builder(
///     JokerId::GreedyJoker,
///     "Greedy Joker",
///     "+3 Mult per Diamond"
/// )
/// .rarity(JokerRarity::Common)
/// .mult(3)
/// .condition(StaticCondition::SuitScored(Suit::Diamond))
/// .per_card()
/// .build()?;
/// ```
///
/// # Lifecycle Events
///
/// Jokers integrate with the game through well-defined lifecycle events that mirror
/// Balatro's game flow:
///
/// 1. **Blind Start**: `on_blind_start()` - Called when a new blind begins
/// 2. **Hand Play**: `on_hand_played()` - Called when a hand is played for scoring
/// 3. **Card Scoring**: `on_card_scored()` - Called for each individual scoring card
/// 4. **Discard**: `on_discard()` - Called when cards are discarded
/// 5. **Shop**: `on_shop_open()` - Called when entering the shop
/// 6. **Round End**: `on_round_end()` - Called at the end of each round
///
/// # Effect Application
///
/// Joker effects are applied in a specific order to ensure consistent behavior:
/// 1. Chips calculation: Base chips + chip bonuses
/// 2. Mult calculation: Base mult + mult bonuses
/// 3. Mult multiplication: Apply mult multipliers
/// 4. Final score: chips × mult
///
/// # Thread Safety
///
/// All joker implementations must be `Send + Sync` to support concurrent access
/// in multi-threaded environments and RL training scenarios.
///
/// # Performance Considerations
///
/// - `on_card_scored()` is called most frequently and should be optimized
/// - Use early returns for non-matching conditions
/// - Avoid expensive operations in hot paths
/// - Consider caching expensive calculations in joker state
pub trait Joker: Send + Sync + std::fmt::Debug {
    /// Get the unique identifier for this joker.
    ///
    /// This ID must be unique across all joker implementations and is used for
    /// state management, serialization, and factory creation.
    ///
    /// # Returns
    /// The unique `JokerId` for this joker type.
    fn id(&self) -> JokerId;

    /// Get the display name of this joker.
    ///
    /// This should be a human-readable name that matches the Balatro naming convention.
    ///
    /// # Returns
    /// A string slice containing the joker's name.
    fn name(&self) -> &str;

    /// Get the description of this joker's effect.
    ///
    /// This should clearly describe what the joker does in a concise format
    /// suitable for display in the user interface.
    ///
    /// # Returns
    /// A string slice containing the joker's description.
    fn description(&self) -> &str;

    /// Get the rarity level of this joker.
    ///
    /// Rarity affects the base cost and availability in shops.
    ///
    /// # Returns
    /// The `JokerRarity` level for this joker.
    fn rarity(&self) -> JokerRarity;

    /// Get the base cost of this joker in the shop.
    ///
    /// The default implementation uses rarity-based pricing:
    /// - Common: 3 coins
    /// - Uncommon: 6 coins  
    /// - Rare: 8 coins
    /// - Legendary: 20 coins
    ///
    /// Override this method for jokers with custom pricing.
    ///
    /// # Returns
    /// The cost in coins to purchase this joker.
    fn cost(&self) -> usize {
        match self.rarity() {
            JokerRarity::Common => 3,
            JokerRarity::Uncommon => 6,
            JokerRarity::Rare => 8,
            JokerRarity::Legendary => 20,
        }
    }

    /// Get the current sell value (defaults to cost / 2)
    fn sell_value(&self, accumulated_bonus: f64) -> usize {
        (self.cost() / 2) + (accumulated_bonus as usize)
    }

    // Lifecycle hooks with default implementations

    /// Called when a hand is played and scored.
    ///
    /// This is the primary hook for jokers that provide per-hand effects,
    /// such as bonuses for specific hand types (Pair, Flush, etc.).
    ///
    /// # Arguments
    /// * `context` - Mutable reference to the current game context
    /// * `hand` - The hand being played and scored
    ///
    /// # Returns
    /// A `JokerEffect` describing any bonuses to apply to the hand.
    ///
    /// # Example
    /// ```rust,ignore
    /// fn on_hand_played(&self, context: &mut GameContext, hand: &SelectHand) -> JokerEffect {
    ///     if hand.is_pair().is_some() {
    ///         JokerEffect::new().with_mult(8)  // +8 mult for pairs
    ///     } else {
    ///         JokerEffect::new()
    ///     }
    /// }
    /// ```
    fn on_hand_played(&self, _context: &mut GameContext, _hand: &SelectHand) -> JokerEffect {
        JokerEffect::new()
    }

    /// Called for each card as it's scored.
    ///
    /// This is the primary hook for jokers that provide per-card effects,
    /// such as bonuses for specific suits, ranks, or card properties.
    ///
    /// # Arguments
    /// * `context` - Mutable reference to the current game context
    /// * `card` - The individual card being scored
    ///
    /// # Returns
    /// A `JokerEffect` describing any bonuses to apply for this card.
    ///
    /// # Performance Note
    /// This method is called frequently (once per scoring card) and should be optimized.
    /// Use early returns for non-matching conditions.
    ///
    /// # Example
    /// ```rust,ignore
    /// fn on_card_scored(&self, context: &mut GameContext, card: &Card) -> JokerEffect {
    ///     if card.suit == Suit::Diamond {
    ///         JokerEffect::new().with_mult(3)  // +3 mult per Diamond
    ///     } else {
    ///         JokerEffect::new()
    ///     }
    /// }
    /// ```
    fn on_card_scored(&self, _context: &mut GameContext, _card: &Card) -> JokerEffect {
        JokerEffect::new()
    }

    /// Called when a new blind starts.
    ///
    /// This hook is useful for jokers that need to reset state, initialize
    /// counters, or apply one-time effects at the beginning of each blind.
    ///
    /// # Arguments
    /// * `context` - Mutable reference to the current game context
    ///
    /// # Returns
    /// A `JokerEffect` describing any bonuses to apply when the blind starts.
    ///
    /// # Example
    /// ```rust,ignore
    /// fn on_blind_start(&self, context: &mut GameContext) -> JokerEffect {
    ///     // Provide bonus at start of blind
    ///     JokerEffect::new().with_chips(10)
    /// }
    /// ```
    fn on_blind_start(&self, _context: &mut GameContext) -> JokerEffect {
        JokerEffect::new()
    }

    /// Called when the shop opens.
    ///
    /// This hook is useful for jokers that interact with the shop phase,
    /// such as those that modify shop contents or provide shop-related bonuses.
    ///
    /// # Arguments
    /// * `context` - Mutable reference to the current game context
    ///
    /// # Returns
    /// A `JokerEffect` describing any bonuses to apply when entering the shop.
    fn on_shop_open(&self, _context: &mut GameContext) -> JokerEffect {
        JokerEffect::new()
    }

    /// Called when cards are discarded.
    ///
    /// This hook is useful for jokers that interact with the discard pile
    /// or provide bonuses based on discarded cards.
    ///
    /// # Arguments
    /// * `context` - Mutable reference to the current game context
    /// * `cards` - Slice of cards being discarded
    ///
    /// # Returns
    /// A `JokerEffect` describing any bonuses to apply for the discard action.
    fn on_discard(&self, _context: &mut GameContext, _cards: &[Card]) -> JokerEffect {
        JokerEffect::new()
    }

    /// Called at the end of each round.
    ///
    /// This hook is useful for jokers that accumulate bonuses over time,
    /// apply end-of-round effects, or clean up temporary state.
    ///
    /// # Arguments
    /// * `context` - Mutable reference to the current game context
    ///
    /// # Returns
    /// A `JokerEffect` describing any bonuses to apply at round end.
    ///
    /// # Example
    /// ```rust,ignore
    /// fn on_round_end(&self, context: &mut GameContext) -> JokerEffect {
    ///     // Provide bonus at end of round
    ///     JokerEffect::new().with_money(5)
    /// }
    /// ```
    fn on_round_end(&self, _context: &mut GameContext) -> JokerEffect {
        JokerEffect::new()
    }

    // State lifecycle hooks with default implementations

    /// Called when a joker is first created or purchased.
    ///
    /// This hook is useful for jokers that need to initialize state, set up
    /// counters, or perform one-time setup operations when they enter the game.
    ///
    /// # Arguments
    /// * `context` - Mutable reference to the current game context
    ///
    /// # Returns
    /// A `JokerEffect` describing any bonuses to apply when the joker is created.
    ///
    /// # Example
    /// ```rust,ignore
    /// fn on_created(&self, context: &mut GameContext) -> JokerEffect {
    ///     // Initialize joker-specific state
    ///     context.joker_state_manager.set_custom_data(
    ///         self.id(),
    ///         "creation_round",
    ///         context.round
    ///     );
    ///     JokerEffect::new()
    /// }
    /// ```
    fn on_created(&self, _context: &mut GameContext) -> JokerEffect {
        JokerEffect::new()
    }

    /// Called when a joker becomes active in the game.
    ///
    /// This hook is useful for jokers that need to register themselves for
    /// specific events, start accumulating bonuses, or activate special mechanics.
    /// Different from creation - activation can happen multiple times if a joker
    /// is temporarily deactivated and reactivated.
    ///
    /// # Arguments
    /// * `context` - Mutable reference to the current game context
    ///
    /// # Returns
    /// A `JokerEffect` describing any bonuses to apply when the joker activates.
    ///
    /// # Example
    /// ```rust,ignore
    /// fn on_activated(&self, context: &mut GameContext) -> JokerEffect {
    ///     // Reset activation-specific counters
    ///     context.joker_state_manager.set_custom_data(
    ///         self.id(),
    ///         "active_since_round",
    ///         context.round
    ///     );
    ///     JokerEffect::new().with_message("Joker activated!".to_string())
    /// }
    /// ```
    fn on_activated(&self, _context: &mut GameContext) -> JokerEffect {
        JokerEffect::new()
    }

    /// Called when a joker becomes inactive.
    ///
    /// This hook is useful for jokers that need to pause their effects,
    /// save temporary state, or clean up active resources while remaining
    /// in the player's collection. The joker may be reactivated later.
    ///
    /// # Arguments
    /// * `context` - Mutable reference to the current game context
    ///
    /// # Returns
    /// A `JokerEffect` describing any final bonuses before deactivation.
    ///
    /// # Example
    /// ```rust,ignore
    /// fn on_deactivated(&self, context: &mut GameContext) -> JokerEffect {
    ///     // Save current progress before going inactive
    ///     let current_progress = self.calculate_progress(context);
    ///     context.joker_state_manager.set_custom_data(
    ///         self.id(),
    ///         "saved_progress",
    ///         current_progress
    ///     );
    ///     JokerEffect::new()
    /// }
    /// ```
    fn on_deactivated(&self, _context: &mut GameContext) -> JokerEffect {
        JokerEffect::new()
    }

    /// Called when a joker is permanently removed from the game.
    ///
    /// This hook is useful for jokers that need to perform cleanup operations,
    /// transfer state to other jokers, or apply final effects before being
    /// destroyed. This is the final method called before the joker is removed.
    ///
    /// # Arguments
    /// * `context` - Mutable reference to the current game context
    ///
    /// # Returns
    /// A `JokerEffect` describing any final bonuses before cleanup.
    ///
    /// # Example
    /// ```rust,ignore
    /// fn on_cleanup(&self, context: &mut GameContext) -> JokerEffect {
    ///     // Transfer accumulated value to money before destruction
    ///     let accumulated = context.joker_state_manager
    ///         .get_accumulated_value(self.id())
    ///         .unwrap_or(0.0);
    ///     
    ///     // Clean up state
    ///     context.joker_state_manager.remove_state(self.id());
    ///     
    ///     JokerEffect::new().with_money(accumulated as i32)
    /// }
    /// ```
    fn on_cleanup(&self, _context: &mut GameContext) -> JokerEffect {
        JokerEffect::new()
    }

    // Modifier hooks with default implementations

    /// Modify the base chips value before scoring calculations.
    ///
    /// This hook allows jokers to modify the baseline chips before any
    /// bonuses are applied. Use this for jokers that fundamentally change
    /// the chip calculation system.
    ///
    /// # Arguments
    /// * `context` - Reference to the current game context
    /// * `base_chips` - The current base chips value
    ///
    /// # Returns
    /// The modified base chips value.
    ///
    /// # Note
    /// This is different from chip bonuses applied via `JokerEffect`.
    /// Chip bonuses are added to the final total, while this modifier
    /// changes the base value before other calculations.
    fn modify_chips(&self, _context: &GameContext, base_chips: i32) -> i32 {
        base_chips
    }

    /// Modify the base mult value before scoring calculations.
    ///
    /// This hook allows jokers to modify the baseline mult before any
    /// bonuses are applied. Use this for jokers that fundamentally change
    /// the mult calculation system.
    ///
    /// # Arguments
    /// * `context` - Reference to the current game context
    /// * `base_mult` - The current base mult value
    ///
    /// # Returns
    /// The modified base mult value.
    ///
    /// # Note
    /// This is different from mult bonuses applied via `JokerEffect`.
    /// Mult bonuses are added to the final total, while this modifier
    /// changes the base value before other calculations.
    fn modify_mult(&self, _context: &GameContext, base_mult: i32) -> i32 {
        base_mult
    }

    /// Modify the maximum hand size.
    ///
    /// This hook allows jokers to permanently change the number of cards
    /// the player can hold in their hand.
    ///
    /// # Arguments
    /// * `context` - Reference to the current game context
    /// * `base_size` - The current base hand size
    ///
    /// # Returns
    /// The modified hand size.
    ///
    /// # Example
    /// ```rust,ignore
    /// fn modify_hand_size(&self, _context: &GameContext, base_size: usize) -> usize {
    ///     base_size + 2  // +2 hand size
    /// }
    /// ```
    fn modify_hand_size(&self, _context: &GameContext, base_size: usize) -> usize {
        base_size
    }

    /// Modify the number of discards available per round.
    ///
    /// This hook allows jokers to change the number of discards the player
    /// can use each round.
    ///
    /// # Arguments
    /// * `context` - Reference to the current game context
    /// * `base_discards` - The current base number of discards
    ///
    /// # Returns
    /// The modified number of discards.
    ///
    /// # Example
    /// ```rust,ignore
    /// fn modify_discards(&self, _context: &GameContext, base_discards: usize) -> usize {
    ///     base_discards + 1  // +1 discard per round
    /// }
    /// ```
    fn modify_discards(&self, _context: &GameContext, base_discards: usize) -> usize {
        base_discards
    }

    // State serialization hooks with default implementations

    /// Serialize joker-specific state for persistence.
    ///
    /// This hook allows jokers to define custom serialization logic for their state.
    /// The default implementation uses the standard JokerState serialization.
    ///
    /// # Arguments
    /// * `context` - Reference to the current game context
    /// * `state` - The joker's current state to serialize
    ///
    /// # Returns
    /// A Result containing the serialized state as JSON Value or an error
    ///
    /// # Example
    /// ```rust,ignore
    /// fn serialize_state(&self, context: &GameContext, state: &JokerState) -> Result<Value, serde_json::Error> {
    ///     // Custom serialization for complex joker state
    ///     let mut custom_state = serde_json::to_value(state)?;
    ///     custom_state["joker_type"] = Value::String(self.name().to_string());
    ///     custom_state["creation_timestamp"] = Value::Number(
    ///         serde_json::Number::from(context.round)
    ///     );
    ///     Ok(custom_state)
    /// }
    /// ```
    fn serialize_state(
        &self,
        _context: &GameContext,
        state: &JokerState,
    ) -> Result<Value, serde_json::Error> {
        serde_json::to_value(state)
    }

    /// Deserialize joker-specific state from persistence.
    ///
    /// This hook allows jokers to define custom deserialization logic for their state.
    /// The default implementation uses the standard JokerState deserialization.
    ///
    /// # Arguments
    /// * `context` - Reference to the current game context
    /// * `data` - The serialized state data to deserialize
    ///
    /// # Returns
    /// A Result containing the deserialized JokerState or an error
    ///
    /// # Example
    /// ```rust,ignore
    /// fn deserialize_state(&self, context: &GameContext, data: &Value) -> Result<JokerState, serde_json::Error> {
    ///     // Custom deserialization with validation
    ///     let mut state: JokerState = serde_json::from_value(data.clone())?;
    ///     
    ///     // Validate and migrate old state format if needed
    ///     if let Some(version) = data.get("version") {
    ///         if version.as_u64() == Some(1) {
    ///             // Migrate from version 1 to current format
    ///             state.accumulated_value *= 2.0; // Example migration
    ///         }
    ///     }
    ///     
    ///     Ok(state)
    /// }
    /// ```
    fn deserialize_state(
        &self,
        _context: &GameContext,
        data: &Value,
    ) -> Result<JokerState, serde_json::Error> {
        serde_json::from_value(data.clone())
    }

    /// Validate the integrity of a joker's state.
    ///
    /// This hook allows jokers to define custom validation rules for their state.
    /// Called after deserialization to ensure state integrity.
    ///
    /// # Arguments
    /// * `context` - Reference to the current game context
    /// * `state` - The state to validate
    ///
    /// # Returns
    /// Ok(()) if state is valid, Err with description if invalid
    ///
    /// # Example
    /// ```rust,ignore
    /// fn validate_state(&self, context: &GameContext, state: &JokerState) -> Result<(), String> {
    ///     // Custom validation for this joker type
    ///     if state.accumulated_value < 0.0 {
    ///         return Err("Accumulated value cannot be negative for this joker".to_string());
    ///     }
    ///     
    ///     if let Some(level) = state.get_custom::<i32>("level").unwrap_or(None) {
    ///         if level > 100 {
    ///             return Err("Level cannot exceed 100".to_string());
    ///         }
    ///     }
    ///     
    ///     Ok(())
    /// }
    /// ```
    fn validate_state(&self, _context: &GameContext, _state: &JokerState) -> Result<(), String> {
        Ok(())
    }

    /// Initialize default state for a new joker instance.
    ///
    /// This hook allows jokers to define their initial state when first created.
    /// Called when a joker is purchased or otherwise added to the game.
    ///
    /// # Arguments
    /// * `context` - Reference to the current game context
    ///
    /// # Returns
    /// The initial JokerState for this joker instance
    ///
    /// # Example
    /// ```rust,ignore
    /// fn initialize_state(&self, context: &GameContext) -> JokerState {
    ///     let mut state = JokerState::new();
    ///     
    ///     // Set initial values based on game context
    ///     state.set_custom("creation_round", context.round).unwrap();
    ///     state.set_custom("level", 1).unwrap();
    ///     
    ///     // Some jokers start with triggers based on ante
    ///     if context.ante <= 3 {
    ///         state.triggers_remaining = Some(5);
    ///     } else {
    ///         state.triggers_remaining = Some(3);
    ///     }
    ///     
    ///     state
    /// }
    /// ```
    fn initialize_state(&self, _context: &GameContext) -> JokerState {
        JokerState::new()
    }

    /// Migrate state from an older version.
    ///
    /// This hook allows jokers to handle state migration when loading save games
    /// from older versions of the game. Useful for maintaining compatibility.
    ///
    /// # Arguments
    /// * `context` - Reference to the current game context
    /// * `old_state` - The state in the old format
    /// * `from_version` - The version the state is being migrated from
    ///
    /// # Returns
    /// The migrated state in the current format, or an error if migration fails
    ///
    /// # Example
    /// ```rust,ignore
    /// fn migrate_state(&self, context: &GameContext, old_state: &Value, from_version: u32) -> Result<JokerState, String> {
    ///     match from_version {
    ///         1 => {
    ///             // Migrate from version 1
    ///             let mut state: JokerState = serde_json::from_value(old_state.clone())
    ///                 .map_err(|e| format!("Failed to parse v1 state: {}", e))?;
    ///             
    ///             // In v1, accumulated_value was stored as integer
    ///             if let Some(old_value) = old_state.get("old_accumulated") {
    ///                 state.accumulated_value = old_value.as_i64().unwrap_or(0) as f64;
    ///             }
    ///             
    ///             Ok(state)
    ///         }
    ///         _ => Err(format!("Unknown version: {}", from_version))
    ///     }
    /// }
    /// ```
    fn migrate_state(
        &self,
        _context: &GameContext,
        old_state: &Value,
        _from_version: u32,
    ) -> Result<JokerState, String> {
        serde_json::from_value(old_state.clone()).map_err(|e| format!("Migration failed: {e}"))
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

// Include hand composition jokers (Ride the Bus, Blackboard, DNA)
pub mod hand_composition_jokers;

// Include tests for hand composition jokers (Ride the Bus, Blackboard, DNA)
#[cfg(test)]
mod hand_composition_tests;

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
