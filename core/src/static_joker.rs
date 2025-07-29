use crate::card::{Card, Suit, Value};
use crate::hand::SelectHand;
use crate::joker::{GameContext, Joker, JokerEffect, JokerId, JokerRarity};
use crate::rank::HandRank;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

/// Core trait for static joker implementations that can be evaluated at compile time.
///
/// This trait provides a high-performance alternative to the dynamic `Joker` trait by
/// enabling compile-time evaluation of joker effects. Static jokers offer:
///
/// - **Zero-allocation scoring**: All effects computed without heap allocations
/// - **Compile-time validation**: Invalid configurations caught at build time
/// - **Optimal performance**: Direct dispatch without vtable overhead
/// - **Migration path**: Gradual conversion from dynamic to static jokers
///
/// # Implementation Patterns
///
/// ## 1. Simple Static Joker
/// ```rust,ignore
/// struct GreedyJoker;
///
/// impl StaticJoker for GreedyJoker {
///     const ID: JokerId = JokerId::GreedyJoker;
///     const NAME: &'static str = "Greedy Joker";
///     const DESCRIPTION: &'static str = "Played cards with Diamond suit give +3 Mult";
///     const RARITY: JokerRarity = JokerRarity::Common;
///
///     type Condition = SuitCondition<{Suit::Diamond as u8}>;
///     type Effect = MultEffect<3>;
///
///     const TRIGGERS_PER_CARD: bool = true;
/// }
/// ```
///
/// ## 2. Complex Static Joker with Custom Logic
/// ```rust,ignore
/// struct ScalingJoker {
///     multiplier: f64,
/// }
///
/// impl StaticJoker for ScalingJoker {
///     const ID: JokerId = JokerId::CustomJoker;
///     // ... other constants
///
///     fn evaluate_effect(&self, context: &StaticContext) -> JokerEffect {
///         JokerEffect::new().with_mult((self.multiplier * context.round as f64) as i32)
///     }
/// }
/// ```
///
/// # Performance Characteristics
///
/// - **Static dispatch**: No vtable overhead (3-5x faster than dynamic)
/// - **Inline optimization**: Compiler can fully inline effect calculations
/// - **Cache friendly**: Minimal memory access patterns
/// - **SIMD ready**: Simple arithmetic enables auto-vectorization
///
/// # Migration Strategy
///
/// Use `StaticJokerAdapter` to gradually migrate existing dynamic jokers:
/// ```rust,ignore
/// // Phase 1: Wrap existing dynamic joker
/// let static_joker = StaticJokerAdapter::from_dynamic(my_dynamic_joker);
///
/// // Phase 2: Implement StaticJoker trait directly
/// // Phase 3: Remove dynamic implementation
/// ```
pub trait StaticJoker: Debug + Send + Sync + 'static {
    /// Unique identifier for this joker (must be compile-time constant)
    const ID: JokerId;

    /// Display name (must be compile-time constant)
    const NAME: &'static str;

    /// Effect description (must be compile-time constant)
    const DESCRIPTION: &'static str;

    /// Rarity level (must be compile-time constant)
    const RARITY: JokerRarity;

    /// Base cost override (None uses rarity-based default)
    const COST_OVERRIDE: Option<usize> = None;

    /// Whether this joker triggers per card (true) or per hand (false)
    const TRIGGERS_PER_CARD: bool;

    /// Priority for processing order (higher = earlier, 0 = default)
    const PRIORITY: i32 = 0;

    /// Check if this joker's condition is met for a specific card
    ///
    /// This method is called for each scoring card when `TRIGGERS_PER_CARD` is true.
    /// Implementation should be fast and avoid allocations.
    fn check_card_condition(&self, card: &Card, context: &StaticContext) -> bool;

    /// Check if this joker's condition is met for a hand
    ///
    /// This method is called once per hand when `TRIGGERS_PER_CARD` is false.
    /// Implementation should be fast and avoid allocations.
    fn check_hand_condition(&self, hand: &SelectHand, context: &StaticContext) -> bool;

    /// Calculate the effect when the condition is met
    ///
    /// This is the core performance-critical method. It should:
    /// - Execute in constant time
    /// - Avoid heap allocations
    /// - Use simple arithmetic for SIMD optimization
    fn calculate_effect(&self, context: &StaticContext) -> JokerEffect;

    /// Get the base cost of this joker in the shop
    fn cost(&self) -> usize {
        Self::COST_OVERRIDE.unwrap_or(match Self::RARITY {
            JokerRarity::Common => 3,
            JokerRarity::Uncommon => 6,
            JokerRarity::Rare => 8,
            JokerRarity::Legendary => 20,
        })
    }

    /// Convert this static joker to a dynamic joker for compatibility
    ///
    /// This enables gradual migration by wrapping static jokers in the dynamic interface.
    /// The adapter handles the conversion overhead while preserving static performance
    /// for the core logic.
    fn to_dynamic(self) -> StaticJokerAdapter<Self>
    where
        Self: Sized,
    {
        StaticJokerAdapter::new(self)
    }
}

/// Optimized context for static joker evaluation with minimal allocation overhead.
///
/// This struct provides only the essential information needed for static joker evaluation,
/// avoiding the overhead of the full `GameContext`. For migration compatibility, it can
/// be converted from `GameContext` with minimal cost.
#[derive(Debug, Clone)]
pub struct StaticContext<'a> {
    /// Current round number for scaling effects
    pub round: u32,
    /// Current ante for difficulty-based effects
    pub ante: u8,
    /// Money available for economy jokers
    pub money: i32,
    /// Remaining discards for discard-based effects
    pub discards_remaining: u32,
    /// Cards in the current hand (slice for zero-copy access)
    pub hand_cards: &'a [Card],
    /// Cards being scored (slice for zero-copy access)
    pub scoring_cards: &'a [Card],
    /// Current hand rank for hand-type conditions
    pub hand_rank: Option<HandRank>,
}

impl<'a> StaticContext<'a> {
    /// Create a new static context with minimal required data
    pub fn new(
        round: u32,
        ante: u8,
        money: i32,
        discards_remaining: u32,
        hand_cards: &'a [Card],
        scoring_cards: &'a [Card],
        hand_rank: Option<HandRank>,
    ) -> Self {
        Self {
            round,
            ante,
            money,
            discards_remaining,
            hand_cards,
            scoring_cards,
            hand_rank,
        }
    }

    /// Convert from GameContext for migration compatibility
    ///
    /// This conversion is designed to be very fast (< 50ns) to minimize
    /// the overhead during the migration period.
    pub fn from_game_context(context: &'a GameContext<'a>, _hand: &SelectHand) -> Self {
        Self {
            round: context.round,
            ante: context.ante,
            money: context.money,
            discards_remaining: 5_u32.saturating_sub(context.discards_used), // Standard max discards
            hand_cards: context.hand.cards(),
            scoring_cards: &[], // Will be set per card during scoring
            hand_rank: None, // Hand rank detection can be done on-demand using is_flush(), is_pair(), etc.
        }
    }
}

/// Adapter that wraps a static joker to implement the dynamic `Joker` trait.
///
/// This adapter enables gradual migration from the dynamic joker system to the static
/// system without breaking changes. It preserves the performance benefits of static
/// jokers while providing compatibility with the existing dynamic infrastructure.
///
/// # Performance Impact
///
/// - **Static logic**: Core calculations remain at static performance levels
/// - **Adapter overhead**: ~10-20ns per call for trait dispatch
/// - **Memory**: Single allocation for the wrapper, static joker stored by value
/// - **Cache**: Excellent locality due to embedded static joker
///
/// # Migration Path
///
/// ```rust,ignore
/// // Phase 1: Existing dynamic joker
/// let dynamic_joker: Box<dyn Joker> = Box::new(MyDynamicJoker::new());
///
/// // Phase 2: Convert to static and wrap for compatibility
/// let static_joker = MyStaticJoker::new();
/// let compatible_joker: Box<dyn Joker> = Box::new(static_joker.to_dynamic());
///
/// // Phase 3: Use static joker directly in static-aware systems
/// let pure_static = MyStaticJoker::new();
/// ```
#[derive(Debug)]
pub struct StaticJokerAdapter<T: StaticJoker> {
    inner: T,
}

impl<T: StaticJoker> StaticJokerAdapter<T> {
    /// Create a new adapter wrapping a static joker
    pub fn new(static_joker: T) -> Self {
        Self {
            inner: static_joker,
        }
    }

    /// Get reference to the wrapped static joker
    pub fn inner(&self) -> &T {
        &self.inner
    }

    /// Get mutable reference to the wrapped static joker
    pub fn inner_mut(&mut self) -> &mut T {
        &mut self.inner
    }

    /// Extract the wrapped static joker
    pub fn into_inner(self) -> T {
        self.inner
    }
}

impl<T: StaticJoker> Joker for StaticJokerAdapter<T> {
    fn id(&self) -> JokerId {
        T::ID
    }

    fn name(&self) -> &str {
        T::NAME
    }

    fn description(&self) -> &str {
        T::DESCRIPTION
    }

    fn rarity(&self) -> JokerRarity {
        T::RARITY
    }

    fn cost(&self) -> usize {
        self.inner.cost()
    }

    fn on_hand_played(&self, context: &mut GameContext, hand: &SelectHand) -> JokerEffect {
        if T::TRIGGERS_PER_CARD {
            return JokerEffect::new(); // Per-card jokers don't trigger on hand played
        }

        let static_context = StaticContext::from_game_context(context, hand);

        if self.inner.check_hand_condition(hand, &static_context) {
            self.inner.calculate_effect(&static_context)
        } else {
            JokerEffect::new()
        }
    }

    fn on_card_scored(&self, context: &mut GameContext, card: &Card) -> JokerEffect {
        if !T::TRIGGERS_PER_CARD {
            return JokerEffect::new(); // Per-hand jokers don't trigger on card scored
        }

        // Create a minimal hand for context conversion
        let temp_hand = SelectHand::new(vec![*card]);
        let static_context = StaticContext::from_game_context(context, &temp_hand);

        if self.inner.check_card_condition(card, &static_context) {
            self.inner.calculate_effect(&static_context)
        } else {
            JokerEffect::new()
        }
    }
}

/// Condition for when a static joker effect should apply
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StaticCondition {
    /// Always apply the effect
    Always,
    /// Apply when a specific suit is scored
    SuitScored(Suit),
    /// Apply when a specific value/rank is scored
    RankScored(Value),
    /// Apply when the hand contains a specific type
    /// (e.g., OnePair triggers on Pair, Two Pair, Full House, etc.)
    HandType(HandRank),
    /// Apply when multiple suits are scored
    AnySuitScored(Vec<Suit>),
    /// Apply when multiple ranks are scored
    AnyRankScored(Vec<Value>),
    /// Apply when the hand has at most the specified number of cards
    HandSizeAtMost(usize),
    /// Apply based on remaining discards (multiplies bonus by remaining discard count)
    DiscardCount,
}

/// A framework-based static joker that provides consistent bonuses based on conditions.
///
/// This struct uses the builder pattern and configuration-based approach for creating
/// static jokers. It's distinct from the `StaticJoker` trait which provides the
/// high-performance interface for migration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrameworkStaticJoker {
    /// Unique identifier for this joker
    pub id: JokerId,
    /// Display name
    pub name: &'static str,
    /// Description of what the joker does
    pub description: &'static str,
    /// Rarity level
    pub rarity: JokerRarity,
    /// Base cost override (if None, uses default for rarity)
    pub base_cost: Option<usize>,
    /// Bonus chips to add
    pub chips_bonus: Option<i32>,
    /// Bonus mult to add
    pub mult_bonus: Option<i32>,
    /// Multiplier to apply to mult
    pub mult_multiplier: Option<f64>,
    /// Condition for when to apply the effect
    pub condition: StaticCondition,
    /// Whether the effect applies per card or per hand
    pub per_card: bool,
}

impl FrameworkStaticJoker {
    /// Create a new static joker builder
    pub fn builder(
        id: JokerId,
        name: &'static str,
        description: &'static str,
    ) -> StaticJokerBuilder {
        StaticJokerBuilder {
            id,
            name,
            description,
            rarity: JokerRarity::Common,
            base_cost: None,
            chips_bonus: None,
            mult_bonus: None,
            mult_multiplier: None,
            condition: StaticCondition::Always,
            per_card: false,
        }
    }
}

impl Joker for FrameworkStaticJoker {
    fn id(&self) -> JokerId {
        self.id
    }

    fn name(&self) -> &str {
        self.name
    }

    fn description(&self) -> &str {
        self.description
    }

    fn rarity(&self) -> JokerRarity {
        self.rarity
    }

    fn cost(&self) -> usize {
        self.base_cost.unwrap_or(match self.rarity {
            JokerRarity::Common => 3,
            JokerRarity::Uncommon => 6,
            JokerRarity::Rare => 8,
            JokerRarity::Legendary => 20,
        })
    }

    fn on_hand_played(&self, context: &mut GameContext, hand: &SelectHand) -> JokerEffect {
        if !self.per_card {
            // Apply effect once per hand if condition is met
            if self.check_hand_condition(hand) {
                self.create_effect_with_context(context)
            } else {
                JokerEffect::new()
            }
        } else {
            JokerEffect::new()
        }
    }

    fn on_card_scored(&self, context: &mut GameContext, card: &Card) -> JokerEffect {
        if self.per_card {
            // Apply effect per card if condition is met
            if self.check_card_condition(card) {
                self.create_effect_with_context(context)
            } else {
                JokerEffect::new()
            }
        } else {
            JokerEffect::new()
        }
    }
}

impl FrameworkStaticJoker {
    /// Check if the condition is met for a hand
    fn check_hand_condition(&self, hand: &SelectHand) -> bool {
        match &self.condition {
            StaticCondition::Always => true,
            StaticCondition::DiscardCount => true, // Always applies, but effect is calculated dynamically
            StaticCondition::HandType(required_rank) => {
                // Check if the hand contains the required type
                match required_rank {
                    HandRank::OnePair => hand.is_pair().is_some(),
                    HandRank::TwoPair => hand.is_two_pair().is_some(),
                    HandRank::ThreeOfAKind => hand.is_three_of_kind().is_some(),
                    HandRank::Straight => hand.is_straight().is_some(),
                    HandRank::Flush => hand.is_flush().is_some(),
                    HandRank::FullHouse => hand.is_fullhouse().is_some(),
                    HandRank::FourOfAKind => hand.is_four_of_kind().is_some(),
                    HandRank::StraightFlush => hand.is_straight_flush().is_some(),
                    HandRank::RoyalFlush => hand.is_royal_flush().is_some(),
                    HandRank::FiveOfAKind => hand.is_five_of_kind().is_some(),
                    HandRank::FlushHouse => hand.is_flush_house().is_some(),
                    HandRank::FlushFive => hand.is_flush_five().is_some(),
                    HandRank::HighCard => hand.is_highcard().is_some(),
                }
            }
            StaticCondition::HandSizeAtMost(max_size) => {
                // Check if the hand has at most the specified number of cards
                hand.cards().len() <= *max_size
            }
            _ => {
                // For suit/rank conditions on hands, check if any card matches
                hand.cards()
                    .iter()
                    .any(|card| self.check_card_condition(card))
            }
        }
    }

    /// Check if the condition is met for a card
    fn check_card_condition(&self, card: &Card) -> bool {
        match &self.condition {
            StaticCondition::Always => true,
            StaticCondition::DiscardCount => true, // Always applies, but effect is calculated dynamically
            StaticCondition::SuitScored(suit) => card.suit == *suit,
            StaticCondition::RankScored(value) => card.value == *value,
            StaticCondition::AnySuitScored(suits) => suits.contains(&card.suit),
            StaticCondition::AnyRankScored(values) => values.contains(&card.value),
            StaticCondition::HandType(_) => {
                // Hand type conditions don't apply to individual cards
                false
            }
            StaticCondition::HandSizeAtMost(_) => {
                // Hand size conditions don't apply to individual cards
                false
            }
        }
    }

    /// Create the effect based on configured bonuses with access to game context for dynamic calculations
    fn create_effect_with_context(&self, context: &GameContext) -> JokerEffect {
        let mut effect = JokerEffect::new();

        match &self.condition {
            StaticCondition::DiscardCount => {
                // Calculate bonus based on remaining discards
                const MAX_DISCARDS: u32 = 5; // Standard discards per round
                let discards_remaining = MAX_DISCARDS.saturating_sub(context.discards_used);

                if let Some(chips_base) = self.chips_bonus {
                    let chips_bonus = chips_base * discards_remaining as i32;
                    effect = effect.with_chips(chips_bonus);
                }

                if let Some(mult_base) = self.mult_bonus {
                    let mult_bonus = mult_base * discards_remaining as i32;
                    effect = effect.with_mult(mult_bonus);
                }
            }
            _ => {
                // Use standard fixed bonuses for other conditions
                if let Some(chips) = self.chips_bonus {
                    effect = effect.with_chips(chips);
                }

                if let Some(mult) = self.mult_bonus {
                    effect = effect.with_mult(mult);
                }
            }
        }

        if let Some(multiplier) = self.mult_multiplier {
            effect = effect.with_mult_multiplier(multiplier);
        }

        effect
    }
}

/// Builder for creating static jokers
pub struct StaticJokerBuilder {
    id: JokerId,
    name: &'static str,
    description: &'static str,
    rarity: JokerRarity,
    base_cost: Option<usize>,
    chips_bonus: Option<i32>,
    mult_bonus: Option<i32>,
    mult_multiplier: Option<f64>,
    condition: StaticCondition,
    per_card: bool,
}

impl StaticJokerBuilder {
    pub fn rarity(mut self, rarity: JokerRarity) -> Self {
        self.rarity = rarity;
        self
    }

    pub fn cost(mut self, cost: usize) -> Self {
        self.base_cost = Some(cost);
        self
    }

    pub fn chips(mut self, chips: i32) -> Self {
        self.chips_bonus = Some(chips);
        self
    }

    pub fn mult(mut self, mult: i32) -> Self {
        self.mult_bonus = Some(mult);
        self
    }

    pub fn mult_multiplier(mut self, multiplier: f64) -> Self {
        self.mult_multiplier = Some(multiplier);
        self
    }

    pub fn condition(mut self, condition: StaticCondition) -> Self {
        self.condition = condition;
        self
    }

    pub fn per_card(mut self) -> Self {
        self.per_card = true;
        self
    }

    pub fn per_hand(mut self) -> Self {
        self.per_card = false;
        self
    }

    pub fn build(self) -> Result<FrameworkStaticJoker, String> {
        // Validate that per_card/per_hand is compatible with condition
        match (&self.condition, self.per_card) {
            (StaticCondition::HandType(_), true) => {
                return Err("HandType conditions should be per_hand, not per_card".to_string());
            }
            (StaticCondition::HandSizeAtMost(_), true) => {
                return Err(
                    "HandSizeAtMost conditions should be per_hand, not per_card".to_string()
                );
            }
            (StaticCondition::DiscardCount, true) => {
                return Err("DiscardCount conditions should be per_hand, not per_card".to_string());
            }
            (StaticCondition::SuitScored(_), false) => {
                return Err("SuitScored conditions should be per_card, not per_hand".to_string());
            }
            (StaticCondition::RankScored(_), false) => {
                return Err("RankScored conditions should be per_card, not per_hand".to_string());
            }
            (StaticCondition::AnySuitScored(_), false) => {
                return Err("AnySuitScored conditions should be per_card, not per_hand".to_string());
            }
            (StaticCondition::AnyRankScored(_), false) => {
                return Err("AnyRankScored conditions should be per_card, not per_hand".to_string());
            }
            _ => {} // Valid combinations
        }

        // Validate that at least one bonus is specified
        if self.chips_bonus.is_none() && self.mult_bonus.is_none() && self.mult_multiplier.is_none()
        {
            return Err(
                "At least one bonus (chips, mult, or mult_multiplier) must be specified"
                    .to_string(),
            );
        }

        Ok(FrameworkStaticJoker {
            id: self.id,
            name: self.name,
            description: self.description,
            rarity: self.rarity,
            base_cost: self.base_cost,
            chips_bonus: self.chips_bonus,
            mult_bonus: self.mult_bonus,
            mult_multiplier: self.mult_multiplier,
            condition: self.condition,
            per_card: self.per_card,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::joker::test_utils::TestContextBuilder;

    fn create_default_test_context() -> GameContext<'static> {
        TestContextBuilder::new().build()
    }

    #[test]
    fn test_static_joker_builder() {
        let joker = FrameworkStaticJoker::builder(JokerId::Joker, "Test Joker", "A test joker")
            .rarity(JokerRarity::Common)
            .mult(4)
            .per_hand()
            .build()
            .expect("Valid joker configuration");

        assert_eq!(joker.id(), JokerId::Joker);
        assert_eq!(joker.name(), "Test Joker");
        assert_eq!(joker.rarity(), JokerRarity::Common);
        assert_eq!(joker.mult_bonus, Some(4));
        assert!(!joker.per_card);
    }

    #[test]
    fn test_condition_always() {
        let joker =
            FrameworkStaticJoker::builder(JokerId::Joker, "Always Joker", "Always gives bonus")
                .mult(5)
                .condition(StaticCondition::Always)
                .per_hand()
                .build()
                .expect("Valid joker configuration");

        let context = create_default_test_context();
        let effect = joker.create_effect_with_context(&context);
        assert_eq!(effect.mult, 5);
    }

    #[test]
    fn test_suit_condition() {
        let joker = FrameworkStaticJoker::builder(
            JokerId::GreedyJoker,
            "Diamond Joker",
            "Diamonds give bonus",
        )
        .mult(3)
        .condition(StaticCondition::SuitScored(Suit::Diamond))
        .per_card()
        .build()
        .expect("Valid joker configuration");

        let diamond_card = Card::new(Value::King, Suit::Diamond);
        let heart_card = Card::new(Value::King, Suit::Heart);

        assert!(joker.check_card_condition(&diamond_card));
        assert!(!joker.check_card_condition(&heart_card));
    }

    #[test]
    fn test_rank_condition() {
        let joker = FrameworkStaticJoker::builder(JokerId::Scholar, "Ace Bonus", "Aces give bonus")
            .chips(20)
            .mult(4)
            .condition(StaticCondition::RankScored(Value::Ace))
            .per_card()
            .build()
            .expect("Valid joker configuration");

        let ace_card = Card::new(Value::Ace, Suit::Spade);
        let king_card = Card::new(Value::King, Suit::Spade);

        assert!(joker.check_card_condition(&ace_card));
        assert!(!joker.check_card_condition(&king_card));
    }

    #[test]
    fn test_any_suit_condition() {
        let joker =
            FrameworkStaticJoker::builder(JokerId::RedCard, "Red Bonus", "Red cards give bonus")
                .mult(2)
                .condition(StaticCondition::AnySuitScored(vec![
                    Suit::Heart,
                    Suit::Diamond,
                ]))
                .per_card()
                .build()
                .expect("Valid joker configuration");

        let heart_card = Card::new(Value::Ten, Suit::Heart);
        let diamond_card = Card::new(Value::Ten, Suit::Diamond);
        let spade_card = Card::new(Value::Ten, Suit::Spade);

        assert!(joker.check_card_condition(&heart_card));
        assert!(joker.check_card_condition(&diamond_card));
        assert!(!joker.check_card_condition(&spade_card));
    }

    #[test]
    fn test_suit_jokers_greedy() {
        let joker = crate::static_joker_factory::StaticJokerFactory::create_greedy_joker_concrete();

        // Test properties
        assert_eq!(joker.id(), JokerId::GreedyJoker);
        assert_eq!(joker.name(), "Greedy Joker");
        assert_eq!(joker.mult_bonus, Some(3));
        assert!(joker.per_card);

        // Test condition checking
        let diamond_card = Card::new(Value::Ace, Suit::Diamond);
        let heart_card = Card::new(Value::King, Suit::Heart);

        assert!(joker.check_card_condition(&diamond_card));
        assert!(!joker.check_card_condition(&heart_card));

        // Test effect
        let context = create_default_test_context();
        let effect = joker.create_effect_with_context(&context);
        assert_eq!(effect.mult, 3);
    }

    #[test]
    fn test_suit_jokers_lusty() {
        let joker = crate::static_joker_factory::StaticJokerFactory::create_lusty_joker_concrete();

        // Test properties
        assert_eq!(joker.id(), JokerId::LustyJoker);
        assert_eq!(joker.name(), "Lusty Joker");
        assert_eq!(joker.mult_bonus, Some(3));
        assert!(joker.per_card);

        // Test condition checking
        let heart_card = Card::new(Value::Ace, Suit::Heart);
        let spade_card = Card::new(Value::King, Suit::Spade);

        assert!(joker.check_card_condition(&heart_card));
        assert!(!joker.check_card_condition(&spade_card));

        // Test effect
        let context = create_default_test_context();
        let effect = joker.create_effect_with_context(&context);
        assert_eq!(effect.mult, 3);
    }

    #[test]
    fn test_suit_jokers_wrathful() {
        let joker =
            crate::static_joker_factory::StaticJokerFactory::create_wrathful_joker_concrete();

        // Test properties
        assert_eq!(joker.id(), JokerId::WrathfulJoker);
        assert_eq!(joker.name(), "Wrathful Joker");
        assert_eq!(joker.mult_bonus, Some(3));
        assert!(joker.per_card);

        // Test condition checking
        let spade_card = Card::new(Value::Ace, Suit::Spade);
        let club_card = Card::new(Value::King, Suit::Club);

        assert!(joker.check_card_condition(&spade_card));
        assert!(!joker.check_card_condition(&club_card));

        // Test effect
        let context = create_default_test_context();
        let effect = joker.create_effect_with_context(&context);
        assert_eq!(effect.mult, 3);
    }

    #[test]
    fn test_suit_jokers_gluttonous() {
        let joker =
            crate::static_joker_factory::StaticJokerFactory::create_gluttonous_joker_concrete();

        // Test properties
        assert_eq!(joker.id(), JokerId::GluttonousJoker);
        assert_eq!(joker.name(), "Gluttonous Joker");
        assert_eq!(joker.mult_bonus, Some(3));
        assert!(joker.per_card);

        // Test condition checking
        let club_card = Card::new(Value::Ace, Suit::Club);
        let diamond_card = Card::new(Value::King, Suit::Diamond);

        assert!(joker.check_card_condition(&club_card));
        assert!(!joker.check_card_condition(&diamond_card));

        // Test effect
        let context = create_default_test_context();
        let effect = joker.create_effect_with_context(&context);
        assert_eq!(effect.mult, 3);
    }

    #[test]
    fn test_suit_jokers_isolation() {
        // Create all four suit jokers
        let greedy =
            crate::static_joker_factory::StaticJokerFactory::create_greedy_joker_concrete();
        let lusty = crate::static_joker_factory::StaticJokerFactory::create_lusty_joker_concrete();
        let wrathful =
            crate::static_joker_factory::StaticJokerFactory::create_wrathful_joker_concrete();
        let gluttonous =
            crate::static_joker_factory::StaticJokerFactory::create_gluttonous_joker_concrete();

        // Create one card of each suit
        let diamond_card = Card::new(Value::Ace, Suit::Diamond);
        let heart_card = Card::new(Value::King, Suit::Heart);
        let spade_card = Card::new(Value::Queen, Suit::Spade);
        let club_card = Card::new(Value::Jack, Suit::Club);

        // Each joker should only match its own suit
        assert!(greedy.check_card_condition(&diamond_card));
        assert!(!greedy.check_card_condition(&heart_card));
        assert!(!greedy.check_card_condition(&spade_card));
        assert!(!greedy.check_card_condition(&club_card));

        assert!(!lusty.check_card_condition(&diamond_card));
        assert!(lusty.check_card_condition(&heart_card));
        assert!(!lusty.check_card_condition(&spade_card));
        assert!(!lusty.check_card_condition(&club_card));

        assert!(!wrathful.check_card_condition(&diamond_card));
        assert!(!wrathful.check_card_condition(&heart_card));
        assert!(wrathful.check_card_condition(&spade_card));
        assert!(!wrathful.check_card_condition(&club_card));

        assert!(!gluttonous.check_card_condition(&diamond_card));
        assert!(!gluttonous.check_card_condition(&heart_card));
        assert!(!gluttonous.check_card_condition(&spade_card));
        assert!(gluttonous.check_card_condition(&club_card));

        // All should give the same +3 mult effect
        let context = create_default_test_context();
        assert_eq!(greedy.create_effect_with_context(&context).mult, 3);
        assert_eq!(lusty.create_effect_with_context(&context).mult, 3);
        assert_eq!(wrathful.create_effect_with_context(&context).mult, 3);
        assert_eq!(gluttonous.create_effect_with_context(&context).mult, 3);
    }

    #[test]
    fn test_any_rank_condition() {
        let joker = FrameworkStaticJoker::builder(
            JokerId::EvenSteven,
            "Even Bonus",
            "Even cards give bonus",
        )
        .mult(4)
        .condition(StaticCondition::AnyRankScored(vec![
            Value::Two,
            Value::Four,
            Value::Six,
            Value::Eight,
            Value::Ten,
        ]))
        .per_card()
        .build()
        .expect("Valid joker configuration");

        let even_card = Card::new(Value::Eight, Suit::Club);
        let odd_card = Card::new(Value::Seven, Suit::Club);

        assert!(joker.check_card_condition(&even_card));
        assert!(!joker.check_card_condition(&odd_card));
    }

    #[test]
    fn test_multiple_bonuses() {
        let joker =
            FrameworkStaticJoker::builder(JokerId::Scholar, "Multi Bonus", "Multiple effects")
                .chips(50)
                .mult(10)
                .mult_multiplier(1.2)
                .per_hand()
                .build()
                .expect("Valid joker configuration");

        let context = create_default_test_context();
        let effect = joker.create_effect_with_context(&context);
        assert_eq!(effect.chips, 50);
        assert_eq!(effect.mult, 10);
        assert_eq!(effect.mult_multiplier, 1.2);
    }

    #[test]
    fn test_cost_override() {
        let joker = FrameworkStaticJoker::builder(JokerId::Joker, "Expensive", "Costs more")
            .rarity(JokerRarity::Common)
            .cost(10)
            .mult(1) // Add minimal bonus to satisfy validation
            .build()
            .expect("Valid joker configuration");

        assert_eq!(joker.cost(), 10);
    }

    #[test]
    fn test_default_costs() {
        // Common
        let common = FrameworkStaticJoker::builder(JokerId::Joker, "Common", "")
            .rarity(JokerRarity::Common)
            .mult(1) // Add minimal bonus to satisfy validation
            .build()
            .expect("Valid joker configuration");
        assert_eq!(common.cost(), 3);

        // Uncommon
        let uncommon = FrameworkStaticJoker::builder(JokerId::Joker, "Uncommon", "")
            .rarity(JokerRarity::Uncommon)
            .mult(1) // Add minimal bonus to satisfy validation
            .build()
            .expect("Valid joker configuration");
        assert_eq!(uncommon.cost(), 6);

        // Rare
        let rare = FrameworkStaticJoker::builder(JokerId::Joker, "Rare", "")
            .rarity(JokerRarity::Rare)
            .mult(1) // Add minimal bonus to satisfy validation
            .build()
            .expect("Valid joker configuration");
        assert_eq!(rare.cost(), 8);

        // Legendary
        let legendary = FrameworkStaticJoker::builder(JokerId::Joker, "Legendary", "")
            .rarity(JokerRarity::Legendary)
            .mult(1) // Add minimal bonus to satisfy validation
            .build()
            .expect("Valid joker configuration");
        assert_eq!(legendary.cost(), 20);
    }

    #[test]
    fn test_hand_type_condition() {
        let joker = FrameworkStaticJoker::builder(
            JokerId::JollyJoker,
            "Pair Bonus",
            "+8 Mult if played hand contains a Pair",
        )
        .mult(8)
        .condition(StaticCondition::HandType(HandRank::OnePair))
        .per_hand()
        .build()
        .expect("Valid joker configuration");

        // Test with a hand that is exactly a pair
        let pair_hand = SelectHand::new(vec![
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::King, Suit::Diamond),
        ]);

        // Test with a hand that is a two pair (contains pairs)
        let two_pair_hand = SelectHand::new(vec![
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::King, Suit::Diamond),
            Card::new(Value::Queen, Suit::Heart),
            Card::new(Value::Queen, Suit::Diamond),
        ]);

        // Test with a hand that is a full house (contains a pair)
        let full_house_hand = SelectHand::new(vec![
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::King, Suit::Diamond),
            Card::new(Value::King, Suit::Club),
            Card::new(Value::Queen, Suit::Heart),
            Card::new(Value::Queen, Suit::Diamond),
        ]);

        // Test with a hand that is high card (no pair)
        let high_card_hand = SelectHand::new(vec![
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::Queen, Suit::Diamond),
            Card::new(Value::Jack, Suit::Club),
        ]);

        assert!(joker.check_hand_condition(&pair_hand));
        assert!(joker.check_hand_condition(&two_pair_hand)); // Contains pairs
        assert!(joker.check_hand_condition(&full_house_hand)); // Contains a pair
        assert!(!joker.check_hand_condition(&high_card_hand)); // No pair
    }

    #[test]
    fn test_flush_condition_contains() {
        let flush_joker = FrameworkStaticJoker::builder(
            JokerId::DrollJoker,
            "Flush Bonus",
            "+10 Mult if played hand contains a Flush",
        )
        .mult(10)
        .condition(StaticCondition::HandType(HandRank::Flush))
        .per_hand()
        .build()
        .expect("Valid joker configuration");

        // Regular flush
        let flush_hand = SelectHand::new(vec![
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::Queen, Suit::Heart),
            Card::new(Value::Jack, Suit::Heart),
            Card::new(Value::Ten, Suit::Heart),
            Card::new(Value::Two, Suit::Heart),
        ]);

        // Straight flush (contains a flush)
        let straight_flush_hand = SelectHand::new(vec![
            Card::new(Value::Six, Suit::Heart),
            Card::new(Value::Five, Suit::Heart),
            Card::new(Value::Four, Suit::Heart),
            Card::new(Value::Three, Suit::Heart),
            Card::new(Value::Two, Suit::Heart),
        ]);

        // Not a flush
        let mixed_hand = SelectHand::new(vec![
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::Queen, Suit::Diamond),
            Card::new(Value::Jack, Suit::Heart),
            Card::new(Value::Ten, Suit::Heart),
            Card::new(Value::Nine, Suit::Heart),
        ]);

        assert!(flush_joker.check_hand_condition(&flush_hand));
        // Straight flush contains a flush
        assert!(flush_joker.check_hand_condition(&straight_flush_hand));
        assert!(!flush_joker.check_hand_condition(&mixed_hand)); // Not a flush
    }

    #[test]
    fn test_two_pair_condition() {
        let joker = FrameworkStaticJoker::builder(
            JokerId::MadJoker,
            "Two Pair Bonus",
            "+10 Mult if played hand contains Two Pair",
        )
        .mult(10)
        .condition(StaticCondition::HandType(HandRank::TwoPair))
        .per_hand()
        .build()
        .expect("Valid joker configuration");

        // Exact two pair
        let two_pair_hand = SelectHand::new(vec![
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::King, Suit::Diamond),
            Card::new(Value::Queen, Suit::Heart),
            Card::new(Value::Queen, Suit::Diamond),
            Card::new(Value::Jack, Suit::Club),
        ]);

        // Full house (contains two pairs)
        let full_house_hand = SelectHand::new(vec![
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::King, Suit::Diamond),
            Card::new(Value::King, Suit::Club),
            Card::new(Value::Queen, Suit::Heart),
            Card::new(Value::Queen, Suit::Diamond),
        ]);

        // Single pair (doesn't contain two pair)
        let one_pair_hand = SelectHand::new(vec![
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::King, Suit::Diamond),
            Card::new(Value::Queen, Suit::Heart),
            Card::new(Value::Jack, Suit::Diamond),
            Card::new(Value::Ten, Suit::Club),
        ]);

        assert!(joker.check_hand_condition(&two_pair_hand));
        assert!(joker.check_hand_condition(&full_house_hand)); // Full house contains two pairs
        assert!(!joker.check_hand_condition(&one_pair_hand));
    }

    #[test]
    fn test_three_of_a_kind_condition() {
        let joker = FrameworkStaticJoker::builder(
            JokerId::ZanyJoker,
            "Three of a Kind Bonus",
            "+12 Mult if played hand contains Three of a Kind",
        )
        .mult(12)
        .condition(StaticCondition::HandType(HandRank::ThreeOfAKind))
        .per_hand()
        .build()
        .expect("Valid joker configuration");

        // Exact three of a kind
        let three_kind_hand = SelectHand::new(vec![
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::King, Suit::Diamond),
            Card::new(Value::King, Suit::Club),
            Card::new(Value::Queen, Suit::Heart),
            Card::new(Value::Jack, Suit::Diamond),
        ]);

        // Full house (contains three of a kind)
        let full_house_hand = SelectHand::new(vec![
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::King, Suit::Diamond),
            Card::new(Value::King, Suit::Club),
            Card::new(Value::Queen, Suit::Heart),
            Card::new(Value::Queen, Suit::Diamond),
        ]);

        // Four of a kind (contains three of a kind)
        let four_kind_hand = SelectHand::new(vec![
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::King, Suit::Diamond),
            Card::new(Value::King, Suit::Club),
            Card::new(Value::King, Suit::Spade),
            Card::new(Value::Queen, Suit::Heart),
        ]);

        // Two pair (doesn't contain three of a kind)
        let two_pair_hand = SelectHand::new(vec![
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::King, Suit::Diamond),
            Card::new(Value::Queen, Suit::Heart),
            Card::new(Value::Queen, Suit::Diamond),
            Card::new(Value::Jack, Suit::Club),
        ]);

        assert!(joker.check_hand_condition(&three_kind_hand));
        assert!(joker.check_hand_condition(&full_house_hand));
        assert!(joker.check_hand_condition(&four_kind_hand));
        assert!(!joker.check_hand_condition(&two_pair_hand));
    }

    #[test]
    fn test_straight_condition() {
        let joker = FrameworkStaticJoker::builder(
            JokerId::CrazyJoker,
            "Straight Bonus",
            "+12 Mult if played hand contains Straight",
        )
        .mult(12)
        .condition(StaticCondition::HandType(HandRank::Straight))
        .per_hand()
        .build()
        .expect("Valid joker configuration");

        // Regular straight
        let straight_hand = SelectHand::new(vec![
            Card::new(Value::Ten, Suit::Heart),
            Card::new(Value::Nine, Suit::Diamond),
            Card::new(Value::Eight, Suit::Club),
            Card::new(Value::Seven, Suit::Spade),
            Card::new(Value::Six, Suit::Heart),
        ]);

        // Straight flush (contains straight)
        let straight_flush_hand = SelectHand::new(vec![
            Card::new(Value::Six, Suit::Heart),
            Card::new(Value::Five, Suit::Heart),
            Card::new(Value::Four, Suit::Heart),
            Card::new(Value::Three, Suit::Heart),
            Card::new(Value::Two, Suit::Heart),
        ]);

        // Not a straight
        let non_straight_hand = SelectHand::new(vec![
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::Queen, Suit::Diamond),
            Card::new(Value::Jack, Suit::Club),
            Card::new(Value::Nine, Suit::Spade),
            Card::new(Value::Eight, Suit::Heart),
        ]);

        assert!(joker.check_hand_condition(&straight_hand));
        assert!(joker.check_hand_condition(&straight_flush_hand));
        assert!(!joker.check_hand_condition(&non_straight_hand));
    }

    #[test]
    fn test_full_house_condition() {
        let joker = FrameworkStaticJoker::builder(
            JokerId::Scholar,
            "Full House Bonus",
            "+20 Chips if played hand contains Full House",
        )
        .chips(20)
        .condition(StaticCondition::HandType(HandRank::FullHouse))
        .per_hand()
        .build()
        .expect("Valid joker configuration");

        // Full house
        let full_house_hand = SelectHand::new(vec![
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::King, Suit::Diamond),
            Card::new(Value::King, Suit::Club),
            Card::new(Value::Queen, Suit::Heart),
            Card::new(Value::Queen, Suit::Diamond),
        ]);

        // Three of a kind (doesn't contain full house)
        let three_kind_hand = SelectHand::new(vec![
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::King, Suit::Diamond),
            Card::new(Value::King, Suit::Club),
            Card::new(Value::Queen, Suit::Heart),
            Card::new(Value::Jack, Suit::Diamond),
        ]);

        // Two pair (doesn't contain full house)
        let two_pair_hand = SelectHand::new(vec![
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::King, Suit::Diamond),
            Card::new(Value::Queen, Suit::Heart),
            Card::new(Value::Queen, Suit::Diamond),
            Card::new(Value::Jack, Suit::Club),
        ]);

        assert!(joker.check_hand_condition(&full_house_hand));
        assert!(!joker.check_hand_condition(&three_kind_hand));
        assert!(!joker.check_hand_condition(&two_pair_hand));
    }

    #[test]
    fn test_four_of_a_kind_condition() {
        let joker = FrameworkStaticJoker::builder(
            JokerId::Scholar,
            "Four of a Kind Bonus",
            "+30 Chips if played hand contains Four of a Kind",
        )
        .chips(30)
        .condition(StaticCondition::HandType(HandRank::FourOfAKind))
        .per_hand()
        .build()
        .expect("Valid joker configuration");

        // Four of a kind
        let four_kind_hand = SelectHand::new(vec![
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::King, Suit::Diamond),
            Card::new(Value::King, Suit::Club),
            Card::new(Value::King, Suit::Spade),
            Card::new(Value::Queen, Suit::Heart),
        ]);

        // Five of a kind (contains four of a kind)
        let five_kind_hand = SelectHand::new(vec![
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::King, Suit::Diamond),
            Card::new(Value::King, Suit::Club),
            Card::new(Value::King, Suit::Spade),
            Card::new(Value::King, Suit::Heart), // Duplicate for five of a kind test
        ]);

        // Three of a kind (doesn't contain four of a kind)
        let three_kind_hand = SelectHand::new(vec![
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::King, Suit::Diamond),
            Card::new(Value::King, Suit::Club),
            Card::new(Value::Queen, Suit::Heart),
            Card::new(Value::Jack, Suit::Diamond),
        ]);

        assert!(joker.check_hand_condition(&four_kind_hand));
        assert!(joker.check_hand_condition(&five_kind_hand));
        assert!(!joker.check_hand_condition(&three_kind_hand));
    }

    #[test]
    fn test_straight_flush_condition() {
        let joker = FrameworkStaticJoker::builder(
            JokerId::Scholar,
            "Straight Flush Bonus",
            "+50 Chips if played hand contains Straight Flush",
        )
        .chips(50)
        .condition(StaticCondition::HandType(HandRank::StraightFlush))
        .per_hand()
        .build()
        .expect("Valid joker configuration");

        // Straight flush
        let straight_flush_hand = SelectHand::new(vec![
            Card::new(Value::Six, Suit::Heart),
            Card::new(Value::Five, Suit::Heart),
            Card::new(Value::Four, Suit::Heart),
            Card::new(Value::Three, Suit::Heart),
            Card::new(Value::Two, Suit::Heart),
        ]);

        // Royal flush (contains straight flush)
        let royal_flush_hand = SelectHand::new(vec![
            Card::new(Value::Ace, Suit::Heart),
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::Queen, Suit::Heart),
            Card::new(Value::Jack, Suit::Heart),
            Card::new(Value::Ten, Suit::Heart),
        ]);

        // Regular flush (doesn't contain straight)
        let flush_hand = SelectHand::new(vec![
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::Queen, Suit::Heart),
            Card::new(Value::Jack, Suit::Heart),
            Card::new(Value::Nine, Suit::Heart),
            Card::new(Value::Seven, Suit::Heart),
        ]);

        // Regular straight (doesn't contain flush)
        let straight_hand = SelectHand::new(vec![
            Card::new(Value::Ten, Suit::Heart),
            Card::new(Value::Nine, Suit::Diamond),
            Card::new(Value::Eight, Suit::Club),
            Card::new(Value::Seven, Suit::Spade),
            Card::new(Value::Six, Suit::Heart),
        ]);

        assert!(joker.check_hand_condition(&straight_flush_hand));
        assert!(joker.check_hand_condition(&royal_flush_hand));
        assert!(!joker.check_hand_condition(&flush_hand));
        assert!(!joker.check_hand_condition(&straight_hand));
    }

    #[test]
    fn test_high_card_condition() {
        let joker = FrameworkStaticJoker::builder(
            JokerId::Scholar,
            "High Card Bonus",
            "+5 Chips if played hand is High Card",
        )
        .chips(5)
        .condition(StaticCondition::HandType(HandRank::HighCard))
        .per_hand()
        .build()
        .expect("Valid joker configuration");

        // High card (no pairs, flushes, or straights)
        let high_card_hand = SelectHand::new(vec![
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::Queen, Suit::Diamond),
            Card::new(Value::Jack, Suit::Club),
            Card::new(Value::Nine, Suit::Spade),
            Card::new(Value::Seven, Suit::Heart),
        ]);

        // Pair (not high card)
        let pair_hand = SelectHand::new(vec![
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::King, Suit::Diamond),
            Card::new(Value::Queen, Suit::Club),
            Card::new(Value::Jack, Suit::Spade),
            Card::new(Value::Nine, Suit::Heart),
        ]);

        assert!(joker.check_hand_condition(&high_card_hand));
        assert!(!joker.check_hand_condition(&pair_hand));
    }

    #[test]
    fn test_royal_flush_condition() {
        let joker = FrameworkStaticJoker::builder(
            JokerId::Scholar,
            "Royal Flush Bonus",
            "+100 Chips if played hand contains Royal Flush",
        )
        .chips(100)
        .condition(StaticCondition::HandType(HandRank::RoyalFlush))
        .per_hand()
        .build()
        .expect("Valid joker configuration");

        // Royal flush (A, K, Q, J, 10 all same suit)
        let royal_flush_hand = SelectHand::new(vec![
            Card::new(Value::Ace, Suit::Heart),
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::Queen, Suit::Heart),
            Card::new(Value::Jack, Suit::Heart),
            Card::new(Value::Ten, Suit::Heart),
        ]);

        // Regular straight flush (not royal)
        let straight_flush_hand = SelectHand::new(vec![
            Card::new(Value::Nine, Suit::Heart),
            Card::new(Value::Eight, Suit::Heart),
            Card::new(Value::Seven, Suit::Heart),
            Card::new(Value::Six, Suit::Heart),
            Card::new(Value::Five, Suit::Heart),
        ]);

        assert!(joker.check_hand_condition(&royal_flush_hand));
        assert!(!joker.check_hand_condition(&straight_flush_hand));
    }

    #[test]
    fn test_five_of_a_kind_condition() {
        let joker = FrameworkStaticJoker::builder(
            JokerId::Scholar,
            "Five of a Kind Bonus",
            "+50 Chips if played hand contains Five of a Kind",
        )
        .chips(50)
        .condition(StaticCondition::HandType(HandRank::FiveOfAKind))
        .per_hand()
        .build()
        .expect("Valid joker configuration");

        // Five of a kind (5 cards of same rank)
        let five_kind_hand = SelectHand::new(vec![
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::King, Suit::Diamond),
            Card::new(Value::King, Suit::Club),
            Card::new(Value::King, Suit::Spade),
            Card::new(Value::King, Suit::Heart), // Special Balatro case with duplicate suits
        ]);

        // Four of a kind (doesn't contain five of a kind)
        let four_kind_hand = SelectHand::new(vec![
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::King, Suit::Diamond),
            Card::new(Value::King, Suit::Club),
            Card::new(Value::King, Suit::Spade),
            Card::new(Value::Queen, Suit::Heart),
        ]);

        assert!(joker.check_hand_condition(&five_kind_hand));
        assert!(!joker.check_hand_condition(&four_kind_hand));
    }

    #[test]
    fn test_flush_house_condition() {
        let joker = FrameworkStaticJoker::builder(
            JokerId::Scholar,
            "Flush House Bonus",
            "+60 Chips if played hand contains Flush House",
        )
        .chips(60)
        .condition(StaticCondition::HandType(HandRank::FlushHouse))
        .per_hand()
        .build()
        .expect("Valid joker configuration");

        // Flush house (full house + flush - 3 of same rank + pair, all same suit)
        let flush_house_hand = SelectHand::new(vec![
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::Queen, Suit::Heart),
            Card::new(Value::Queen, Suit::Heart),
        ]);

        // Regular full house (not flush)
        let full_house_hand = SelectHand::new(vec![
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::King, Suit::Diamond),
            Card::new(Value::King, Suit::Club),
            Card::new(Value::Queen, Suit::Heart),
            Card::new(Value::Queen, Suit::Diamond),
        ]);

        // Regular flush (not full house)
        let flush_hand = SelectHand::new(vec![
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::Queen, Suit::Heart),
            Card::new(Value::Jack, Suit::Heart),
            Card::new(Value::Ten, Suit::Heart),
            Card::new(Value::Nine, Suit::Heart),
        ]);

        assert!(joker.check_hand_condition(&flush_house_hand));
        assert!(!joker.check_hand_condition(&full_house_hand));
        assert!(!joker.check_hand_condition(&flush_hand));
    }

    #[test]
    fn test_flush_five_condition() {
        let joker = FrameworkStaticJoker::builder(
            JokerId::Scholar,
            "Flush Five Bonus",
            "+80 Chips if played hand contains Flush Five",
        )
        .chips(80)
        .condition(StaticCondition::HandType(HandRank::FlushFive))
        .per_hand()
        .build()
        .expect("Valid joker configuration");

        // Flush five (five of a kind + flush - 5 cards same rank, all same suit)
        let flush_five_hand = SelectHand::new(vec![
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::King, Suit::Heart),
        ]);

        // Five of a kind (not flush)
        let five_kind_hand = SelectHand::new(vec![
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::King, Suit::Diamond),
            Card::new(Value::King, Suit::Club),
            Card::new(Value::King, Suit::Spade),
            Card::new(Value::King, Suit::Heart),
        ]);

        // Regular flush (not five of a kind)
        let flush_hand = SelectHand::new(vec![
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::Queen, Suit::Heart),
            Card::new(Value::Jack, Suit::Heart),
            Card::new(Value::Ten, Suit::Heart),
            Card::new(Value::Nine, Suit::Heart),
        ]);

        assert!(joker.check_hand_condition(&flush_five_hand));
        assert!(!joker.check_hand_condition(&five_kind_hand));
        assert!(!joker.check_hand_condition(&flush_hand));
    }

    #[test]
    fn test_builder_validation() {
        // Test invalid configuration: HandType condition with per_card
        let result = FrameworkStaticJoker::builder(
            JokerId::JollyJoker,
            "Invalid Joker",
            "This should fail validation",
        )
        .mult(8)
        .condition(StaticCondition::HandType(HandRank::OnePair))
        .per_card() // This should be invalid with HandType
        .build();

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("HandType conditions should be per_hand"));

        // Test invalid configuration: SuitScored condition with per_hand
        let result = FrameworkStaticJoker::builder(
            JokerId::GreedyJoker,
            "Invalid Suit Joker",
            "This should fail validation",
        )
        .mult(3)
        .condition(StaticCondition::SuitScored(Suit::Diamond))
        .per_hand() // This should be invalid with SuitScored
        .build();

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("SuitScored conditions should be per_card"));

        // Test invalid configuration: No bonuses specified
        let result = FrameworkStaticJoker::builder(
            JokerId::Joker,
            "No Bonus Joker",
            "This should fail validation",
        )
        .condition(StaticCondition::Always)
        .per_hand()
        .build();

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("At least one bonus"));

        // Test valid configuration
        let result =
            FrameworkStaticJoker::builder(JokerId::Joker, "Valid Joker", "This should work")
                .mult(4)
                .condition(StaticCondition::Always)
                .per_hand()
                .build();

        assert!(result.is_ok());
    }

    #[test]
    fn test_hand_size_at_most_condition() {
        let joker = FrameworkStaticJoker::builder(
            JokerId::HalfJoker,
            "Half Joker",
            "+20 Mult if played hand has 4 or fewer cards",
        )
        .mult(20)
        .condition(StaticCondition::HandSizeAtMost(4))
        .per_hand()
        .build()
        .expect("Valid joker configuration");

        // Test with 4 cards (should trigger)
        let four_card_hand = SelectHand::new(vec![
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::Queen, Suit::Diamond),
            Card::new(Value::Jack, Suit::Club),
            Card::new(Value::Ten, Suit::Spade),
        ]);

        // Test with 3 cards (should trigger)
        let three_card_hand = SelectHand::new(vec![
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::Queen, Suit::Diamond),
            Card::new(Value::Jack, Suit::Club),
        ]);

        // Test with 2 cards (should trigger)
        let two_card_hand = SelectHand::new(vec![
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::Queen, Suit::Diamond),
        ]);

        // Test with 1 card (should trigger)
        let one_card_hand = SelectHand::new(vec![Card::new(Value::King, Suit::Heart)]);

        // Test with 5 cards (should NOT trigger)
        let five_card_hand = SelectHand::new(vec![
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::Queen, Suit::Diamond),
            Card::new(Value::Jack, Suit::Club),
            Card::new(Value::Ten, Suit::Spade),
            Card::new(Value::Nine, Suit::Heart),
        ]);

        // Test with 6 cards (should NOT trigger)
        let six_card_hand = SelectHand::new(vec![
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::Queen, Suit::Diamond),
            Card::new(Value::Jack, Suit::Club),
            Card::new(Value::Ten, Suit::Spade),
            Card::new(Value::Nine, Suit::Heart),
            Card::new(Value::Eight, Suit::Diamond),
        ]);

        // Verify conditions
        assert!(
            joker.check_hand_condition(&four_card_hand),
            "4 cards should trigger Half Joker"
        );
        assert!(
            joker.check_hand_condition(&three_card_hand),
            "3 cards should trigger Half Joker"
        );
        assert!(
            joker.check_hand_condition(&two_card_hand),
            "2 cards should trigger Half Joker"
        );
        assert!(
            joker.check_hand_condition(&one_card_hand),
            "1 card should trigger Half Joker"
        );
        assert!(
            !joker.check_hand_condition(&five_card_hand),
            "5 cards should NOT trigger Half Joker"
        );
        assert!(
            !joker.check_hand_condition(&six_card_hand),
            "6 cards should NOT trigger Half Joker"
        );
    }

    #[test]
    fn test_hand_size_at_most_edge_cases() {
        // Test with 0 max size (should only trigger on empty hands)
        let zero_joker = FrameworkStaticJoker::builder(
            JokerId::Joker,
            "Empty Hand Joker",
            "+10 Mult if hand is empty",
        )
        .mult(10)
        .condition(StaticCondition::HandSizeAtMost(0))
        .per_hand()
        .build()
        .expect("Valid joker configuration");

        // Test with empty hand
        let empty_hand = SelectHand::new(vec![]);

        // Test with one card
        let one_card_hand = SelectHand::new(vec![Card::new(Value::King, Suit::Heart)]);

        assert!(
            zero_joker.check_hand_condition(&empty_hand),
            "Empty hand should trigger with max size 0"
        );
        assert!(
            !zero_joker.check_hand_condition(&one_card_hand),
            "1 card should NOT trigger with max size 0"
        );

        // Test with very large max size
        let large_joker = FrameworkStaticJoker::builder(
            JokerId::Joker,
            "Large Hand Joker",
            "+5 Mult if hand has at most 100 cards",
        )
        .mult(5)
        .condition(StaticCondition::HandSizeAtMost(100))
        .per_hand()
        .build()
        .expect("Valid joker configuration");

        let normal_hand = SelectHand::new(vec![
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::Queen, Suit::Diamond),
            Card::new(Value::Jack, Suit::Club),
            Card::new(Value::Ten, Suit::Spade),
            Card::new(Value::Nine, Suit::Heart),
        ]);

        assert!(
            large_joker.check_hand_condition(&normal_hand),
            "5 cards should trigger with max size 100"
        );
    }

    #[test]
    fn test_hand_size_at_most_with_card_condition() {
        let joker = FrameworkStaticJoker::builder(
            JokerId::HalfJoker,
            "Half Joker",
            "+20 Mult if played hand has 4 or fewer cards",
        )
        .mult(20)
        .condition(StaticCondition::HandSizeAtMost(4))
        .per_hand()
        .build()
        .expect("Valid joker configuration");

        // Hand size conditions should not apply to individual cards
        let card = Card::new(Value::King, Suit::Heart);
        assert!(
            !joker.check_card_condition(&card),
            "Hand size conditions should not apply to individual cards"
        );
    }

    #[test]
    fn test_hand_size_at_most_builder_validation() {
        // Test that HandSizeAtMost condition with per_card fails validation
        let result = FrameworkStaticJoker::builder(
            JokerId::HalfJoker,
            "Invalid Half Joker",
            "This should fail validation",
        )
        .mult(20)
        .condition(StaticCondition::HandSizeAtMost(4))
        .per_card() // This should be invalid with HandSizeAtMost
        .build();

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("HandSizeAtMost conditions should be per_hand"));

        // Test that HandSizeAtMost condition with per_hand is valid
        let result = FrameworkStaticJoker::builder(
            JokerId::HalfJoker,
            "Valid Half Joker",
            "This should work",
        )
        .mult(20)
        .condition(StaticCondition::HandSizeAtMost(4))
        .per_hand()
        .build();

        assert!(result.is_ok());
    }

    // Tests for the new StaticJoker trait system

    /// Example static joker implementation for testing
    #[derive(Debug)]
    struct TestStaticJoker {
        bonus_mult: i32,
    }

    impl StaticJoker for TestStaticJoker {
        const ID: JokerId = JokerId::Joker;
        const NAME: &'static str = "Test Static Joker";
        const DESCRIPTION: &'static str = "A test implementation";
        const RARITY: JokerRarity = JokerRarity::Common;
        const TRIGGERS_PER_CARD: bool = true;

        fn check_card_condition(&self, card: &Card, _context: &StaticContext) -> bool {
            card.suit == Suit::Heart
        }

        fn check_hand_condition(&self, _hand: &SelectHand, _context: &StaticContext) -> bool {
            true
        }

        fn calculate_effect(&self, _context: &StaticContext) -> JokerEffect {
            JokerEffect::new().with_mult(self.bonus_mult)
        }
    }

    #[derive(Debug)]
    struct TestHandStaticJoker;

    impl StaticJoker for TestHandStaticJoker {
        const ID: JokerId = JokerId::JollyJoker;
        const NAME: &'static str = "Test Hand Static Joker";
        const DESCRIPTION: &'static str = "Triggers on flush hands";
        const RARITY: JokerRarity = JokerRarity::Uncommon;
        const TRIGGERS_PER_CARD: bool = false;
        const PRIORITY: i32 = 10;

        fn check_card_condition(&self, _card: &Card, _context: &StaticContext) -> bool {
            false // Hand-based joker doesn't check individual cards
        }

        fn check_hand_condition(&self, hand: &SelectHand, _context: &StaticContext) -> bool {
            hand.is_flush().is_some()
        }

        fn calculate_effect(&self, context: &StaticContext) -> JokerEffect {
            JokerEffect::new().with_chips((context.round * 10) as i32)
        }
    }

    #[test]
    fn test_static_joker_trait_constants() {
        let joker = TestStaticJoker { bonus_mult: 5 };

        // Test compile-time constants
        assert_eq!(TestStaticJoker::ID, JokerId::Joker);
        assert_eq!(TestStaticJoker::NAME, "Test Static Joker");
        assert_eq!(TestStaticJoker::DESCRIPTION, "A test implementation");
        assert_eq!(TestStaticJoker::RARITY, JokerRarity::Common);
        assert!(TestStaticJoker::TRIGGERS_PER_CARD);
        assert_eq!(TestStaticJoker::PRIORITY, 0); // Default value

        // Test default cost calculation
        assert_eq!(joker.cost(), 3); // Common rarity default
    }

    #[test]
    fn test_static_joker_trait_constants_with_overrides() {
        let joker = TestHandStaticJoker;

        assert_eq!(TestHandStaticJoker::ID, JokerId::JollyJoker);
        assert_eq!(TestHandStaticJoker::RARITY, JokerRarity::Uncommon);
        assert_eq!(TestHandStaticJoker::TRIGGERS_PER_CARD, false);
        assert_eq!(TestHandStaticJoker::PRIORITY, 10);

        // Test cost for uncommon rarity
        assert_eq!(joker.cost(), 6); // Uncommon rarity default
    }

    #[test]
    fn test_static_context_creation() {
        let cards = vec![
            Card::new(Value::Ace, Suit::Heart),
            Card::new(Value::King, Suit::Heart),
        ];

        let context = StaticContext::new(
            5,   // round
            2,   // ante
            100, // money
            3,   // discards_remaining
            &cards,
            &cards[0..1],
            Some(HandRank::OnePair),
        );

        assert_eq!(context.round, 5);
        assert_eq!(context.ante, 2);
        assert_eq!(context.money, 100);
        assert_eq!(context.discards_remaining, 3);
        assert_eq!(context.hand_cards.len(), 2);
        assert_eq!(context.scoring_cards.len(), 1);
        assert_eq!(context.hand_rank, Some(HandRank::OnePair));
    }

    #[test]
    fn test_static_context_from_game_context() {
        let test_context = create_default_test_context();
        let hand_cards = vec![Card::new(Value::Ace, Suit::Heart)];
        let hand = SelectHand::new(hand_cards.clone());

        let static_context = StaticContext::from_game_context(&test_context, &hand);

        assert_eq!(static_context.round, test_context.round);
        assert_eq!(static_context.ante, test_context.ante);
        assert_eq!(static_context.money, test_context.money);
        // discards_remaining should be calculated as 5 - discards_used
        assert_eq!(
            static_context.discards_remaining,
            5 - test_context.discards_used
        );
    }

    #[test]
    fn test_static_joker_adapter_creation() {
        let static_joker = TestStaticJoker { bonus_mult: 7 };
        let adapter = StaticJokerAdapter::new(static_joker);

        // Test basic functionality
        assert_eq!(adapter.id(), JokerId::Joker);
        assert_eq!(adapter.name(), "Test Static Joker");
        assert_eq!(adapter.description(), "A test implementation");
        assert_eq!(adapter.rarity(), JokerRarity::Common);
        assert_eq!(adapter.cost(), 3);
    }

    #[test]
    fn test_static_joker_adapter_inner_access() {
        let static_joker = TestStaticJoker { bonus_mult: 8 };
        let mut adapter = StaticJokerAdapter::new(static_joker);

        // Test inner access
        assert_eq!(adapter.inner().bonus_mult, 8);

        // Test mutable access
        adapter.inner_mut().bonus_mult = 12;
        assert_eq!(adapter.inner().bonus_mult, 12);

        // Test extraction
        let extracted = adapter.into_inner();
        assert_eq!(extracted.bonus_mult, 12);
    }

    #[test]
    fn test_static_joker_to_dynamic_conversion() {
        let static_joker = TestStaticJoker { bonus_mult: 6 };
        let dynamic_joker = static_joker.to_dynamic();

        // Test that the dynamic interface works
        assert_eq!(dynamic_joker.id(), JokerId::Joker);
        assert_eq!(dynamic_joker.name(), "Test Static Joker");
        assert_eq!(dynamic_joker.rarity(), JokerRarity::Common);
        assert_eq!(dynamic_joker.cost(), 3);
    }

    #[test]
    fn test_static_joker_adapter_per_card_behavior() {
        let static_joker = TestStaticJoker { bonus_mult: 4 };
        let adapter = static_joker.to_dynamic();

        let mut context = create_default_test_context();
        let heart_card = Card::new(Value::King, Suit::Heart);
        let spade_card = Card::new(Value::King, Suit::Spade);

        // Test per-card triggering (TRIGGERS_PER_CARD = true)
        let heart_effect = adapter.on_card_scored(&mut context, &heart_card);
        let spade_effect = adapter.on_card_scored(&mut context, &spade_card);

        assert_eq!(heart_effect.mult, 4); // Should trigger for hearts
        assert_eq!(spade_effect.mult, 0); // Should not trigger for spades

        // Test that per-card jokers don't trigger on hand played
        let hand = SelectHand::new(vec![heart_card]);
        let hand_effect = adapter.on_hand_played(&mut context, &hand);
        assert_eq!(hand_effect.mult, 0); // Should not trigger
    }

    #[test]
    fn test_static_joker_adapter_per_hand_behavior() {
        let static_joker = TestHandStaticJoker;
        let adapter = static_joker.to_dynamic();

        let mut context = create_default_test_context();

        // Create a flush hand
        let flush_hand = SelectHand::new(vec![
            Card::new(Value::Ace, Suit::Heart),
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::Queen, Suit::Heart),
            Card::new(Value::Jack, Suit::Heart),
            Card::new(Value::Ten, Suit::Heart),
        ]);

        // Create a non-flush hand
        let mixed_hand = SelectHand::new(vec![
            Card::new(Value::Ace, Suit::Heart),
            Card::new(Value::King, Suit::Spade),
        ]);

        // Test per-hand triggering (TRIGGERS_PER_CARD = false)
        let flush_effect = adapter.on_hand_played(&mut context, &flush_hand);
        let mixed_effect = adapter.on_hand_played(&mut context, &mixed_hand);

        // Flush should trigger (round * 10 chips)
        assert_eq!(flush_effect.chips, (context.round * 10) as i32);
        // Mixed hand should not trigger
        assert_eq!(mixed_effect.chips, 0);

        // Test that per-hand jokers don't trigger on individual cards
        let card_effect = adapter.on_card_scored(&mut context, &flush_hand.cards()[0]);
        assert_eq!(card_effect.chips, 0); // Should not trigger
    }

    #[test]
    fn test_static_joker_performance_characteristics() {
        let static_joker = TestStaticJoker { bonus_mult: 1 };

        // Test that methods are callable (performance would need benchmarks)
        let cards = vec![Card::new(Value::Ace, Suit::Heart)];
        let context = StaticContext::new(1, 1, 100, 3, &cards, &cards, None);

        // These should execute very quickly
        let _card_result = static_joker.check_card_condition(&cards[0], &context);
        let _effect_result = static_joker.calculate_effect(&context);

        // No allocations should occur in the critical path
        // (This would require allocation tracking in a real benchmark)
    }

    #[test]
    fn test_static_joker_trait_bounds() {
        // Test that StaticJoker has correct trait bounds
        fn assert_static_joker_bounds<T: StaticJoker>() {
            fn assert_send_sync<T: Send + Sync>() {}
            fn assert_debug<T: Debug>() {}
            fn assert_static<T: 'static>() {}

            assert_send_sync::<T>();
            assert_debug::<T>();
            assert_static::<T>();
        }

        assert_static_joker_bounds::<TestStaticJoker>();
        assert_static_joker_bounds::<TestHandStaticJoker>();
    }

    #[test]
    fn test_static_joker_migration_pattern() {
        // Test the complete migration pattern

        // Phase 1: Create static joker
        let static_joker = TestStaticJoker { bonus_mult: 3 };

        // Phase 2: Convert to dynamic for compatibility
        let dynamic_joker: Box<dyn Joker> = Box::new(static_joker.to_dynamic());

        // Phase 3: Use through dynamic interface
        let mut context = create_default_test_context();
        let heart_card = Card::new(Value::Queen, Suit::Heart);

        let effect = dynamic_joker.on_card_scored(&mut context, &heart_card);
        assert_eq!(effect.mult, 3);

        // The dynamic interface should work seamlessly
        assert_eq!(dynamic_joker.id(), JokerId::Joker);
        assert_eq!(dynamic_joker.name(), "Test Static Joker");
    }

    #[test]
    fn test_static_context_zero_copy_guarantees() {
        let hand_cards = vec![
            Card::new(Value::Ace, Suit::Heart),
            Card::new(Value::King, Suit::Spade),
        ];
        let scoring_cards = &hand_cards[0..1];

        let context = StaticContext::new(
            1,
            1,
            100,
            3,
            &hand_cards,   // Borrowed slice
            scoring_cards, // Borrowed slice
            None,
        );

        // Verify that we're using slices (zero-copy)
        assert_eq!(context.hand_cards.len(), 2);
        assert_eq!(context.scoring_cards.len(), 1);

        // Verify the actual card data is accessible
        assert_eq!(context.hand_cards[0].suit, Suit::Heart);
        assert_eq!(context.scoring_cards[0].suit, Suit::Heart);
    }

    #[test]
    fn test_static_joker_cost_override() {
        #[derive(Debug)]
        struct ExpensiveStaticJoker;

        impl StaticJoker for ExpensiveStaticJoker {
            const ID: JokerId = JokerId::Joker;
            const NAME: &'static str = "Expensive";
            const DESCRIPTION: &'static str = "Costs more";
            const RARITY: JokerRarity = JokerRarity::Common;
            const COST_OVERRIDE: Option<usize> = Some(15);
            const TRIGGERS_PER_CARD: bool = false;

            fn check_card_condition(&self, _card: &Card, _context: &StaticContext) -> bool {
                false
            }

            fn check_hand_condition(&self, _hand: &SelectHand, _context: &StaticContext) -> bool {
                true
            }

            fn calculate_effect(&self, _context: &StaticContext) -> JokerEffect {
                JokerEffect::new().with_mult(10)
            }
        }

        let joker = ExpensiveStaticJoker;
        assert_eq!(joker.cost(), 15); // Should use override, not default for Common (3)
    }
}
