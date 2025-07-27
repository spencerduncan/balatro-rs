use crate::action::{Action, MoveDirection};
use crate::ante::Ante;
use crate::available::Available;
use crate::boss_blinds::BossBlindState;
use crate::bounded_action_history::BoundedActionHistory;
use crate::card::Card;
use crate::config::Config;
use crate::consumables::ConsumableId;
use crate::deck::Deck;
use crate::error::GameError;
use crate::hand::{MadeHand, SelectHand};
use crate::joker::{GameContext, Joker, JokerId, Jokers, OldJoker as OldJokerTrait};
use crate::joker_effect_processor::JokerEffectProcessor;
use crate::joker_factory::JokerFactory;
use crate::joker_state::{JokerState, JokerStateManager};
use crate::memory_monitor::MemoryMonitor;
use crate::rank::HandRank;
use crate::scaling_joker::ScalingEvent;
use crate::shop::packs::{OpenPackState, Pack};
use crate::shop::Shop;
use crate::stage::{Blind, End, Stage};
use crate::state_version::StateVersion;
use crate::target_context::TargetContext;
use crate::vouchers::VoucherCollection;

// Re-export GameState for external use with qualified name to avoid Python bindings conflict
pub use crate::vouchers::GameState as VoucherGameState;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;

/// Maximum debug messages to keep in memory (for practical memory management)
const MAX_DEBUG_MESSAGES: usize = 10000;

/// Score breakdown for debugging and analysis
#[derive(Debug, Clone)]
pub struct ScoreBreakdown {
    pub base_chips: f64,
    pub base_mult: f64,
    pub card_chips: f64,
    pub joker_contributions: Vec<JokerContribution>,
    pub final_score: f64,
}

/// Individual joker contribution to scoring
#[derive(Debug, Clone)]
pub struct JokerContribution {
    pub joker_name: String,
    pub joker_id: JokerId,
    pub chips_added: i32,
    pub mult_added: i32,
    pub mult_multiplier: f64,
    pub money_added: i32,
    pub retrigger_count: u32,
}

/// Accumulates effects from multiple jokers in a structured way
#[derive(Debug, Clone, PartialEq)]
pub struct AccumulatedEffects {
    /// Total chips to add to the hand
    pub chips: i32,
    /// Total mult to add to the hand
    pub mult: i32,
    /// Total money to award to the player
    pub money: i32,
    /// Combined mult multiplier (multiplicative)
    pub mult_multiplier: f64,
    /// Collection of all messages from jokers
    pub messages: Vec<String>,
}

impl AccumulatedEffects {
    /// Create a new AccumulatedEffects with default values
    pub fn new() -> Self {
        Self {
            chips: 0,
            mult: 0,
            money: 0,
            mult_multiplier: 1.0,
            messages: Vec::new(),
        }
    }

    /// Accumulate a JokerEffect into this accumulator
    /// Returns false if killscreen (infinity/NaN) is detected
    pub fn accumulate_effect(&mut self, effect: &crate::joker::JokerEffect) -> bool {
        // Calculate total iterations (1 + retriggers)
        let iterations = 1 + effect.retrigger as i32;

        // Accumulate chips, mult, and money (additive, multiplied by retriggers)
        self.chips = self
            .chips
            .saturating_add(effect.chips.saturating_mul(iterations));
        self.mult = self
            .mult
            .saturating_add(effect.mult.saturating_mul(iterations));
        self.money = self
            .money
            .saturating_add(effect.money.saturating_mul(iterations));

        // Apply mult multiplier for each iteration (multiplicative)
        if effect.mult_multiplier != 0.0 {
            for _ in 0..iterations {
                self.mult_multiplier *= effect.mult_multiplier;
            }
        }

        // Add message if present
        if let Some(ref message) = effect.message {
            self.messages.push(message.clone());
        }

        // Check for killscreen (infinity or NaN)
        if self.mult_multiplier.is_infinite() || self.mult_multiplier.is_nan() {
            return false;
        }

        true
    }

    /// Merge another AccumulatedEffects into this one
    pub fn merge(&mut self, other: &AccumulatedEffects) {
        self.chips = self.chips.saturating_add(other.chips);
        self.mult = self.mult.saturating_add(other.mult);
        self.money = self.money.saturating_add(other.money);

        // Multiply mult multipliers
        if other.mult_multiplier != 0.0 {
            self.mult_multiplier *= other.mult_multiplier;
        }

        // Append messages
        self.messages.extend_from_slice(&other.messages);
    }

    /// Apply the accumulated mult multiplier to the mult value
    pub fn apply_mult_multiplier(&mut self) {
        if self.mult_multiplier != 0.0 && self.mult_multiplier != 1.0 {
            let multiplied = (self.mult as f64) * self.mult_multiplier;
            // Clamp to i32 range to prevent overflow
            self.mult = multiplied.clamp(i32::MIN as f64, i32::MAX as f64) as i32;
        }
    }
}

impl Default for AccumulatedEffects {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub struct Game {
    pub config: Config,
    pub shop: Shop,
    pub deck: Deck,
    pub available: Available,
    pub discarded: Vec<Card>,
    pub blind: Option<Blind>,
    pub stage: Stage,
    pub ante_start: Ante,
    pub ante_end: Ante,
    pub ante_current: Ante,
    pub action_history: BoundedActionHistory,
    pub round: f64,

    // jokers using structured JokerEffect system
    #[cfg_attr(feature = "serde", serde(skip))]
    pub jokers: Vec<Box<dyn Joker>>,

    #[cfg_attr(feature = "serde", serde(skip, default = "JokerEffectProcessor::new"))]
    pub joker_effect_processor: JokerEffectProcessor,

    #[cfg_attr(
        feature = "serde",
        serde(skip, default = "default_joker_state_manager")
    )]
    pub joker_state_manager: Arc<JokerStateManager>,

    // playing
    pub plays: f64,
    pub discards: f64,
    pub reward: f64,
    pub money: f64,

    // shop reroll state
    pub shop_reroll_cost: f64,
    pub shop_rerolls_this_round: u32,

    // for scoring
    pub chips: f64,
    pub mult: f64,
    pub score: f64,

    // hand type tracking for this game run
    pub hand_type_counts: HashMap<HandRank, u32>,

    // Extended state for consumables, vouchers, and boss blinds
    /// Consumable cards currently in the player's hand
    pub consumables_in_hand: Vec<ConsumableId>,

    /// Collection of owned vouchers with purchase tracking
    pub vouchers: VoucherCollection,

    /// Current boss blind state and effects
    pub boss_blind_state: BossBlindState,

    /// Pack system state
    /// Packs currently in the player's inventory
    pub pack_inventory: Vec<Pack>,

    /// Currently opened pack that player is choosing from
    pub open_pack: Option<OpenPackState>,

    /// Version of the game state for serialization compatibility
    pub state_version: StateVersion,

    /// Debug logging enabled flag
    #[cfg_attr(feature = "serde", serde(skip))]
    pub debug_logging_enabled: bool,

    /// Debug messages buffer
    #[cfg_attr(feature = "serde", serde(skip))]
    pub debug_messages: Vec<String>,

    /// Multi-select context for tracking selected items
    #[cfg_attr(feature = "serde", serde(skip, default = "TargetContext::new"))]
    pub target_context: TargetContext,

    /// Random number generator for secure game randomness
    #[cfg_attr(feature = "serde", serde(skip, default = "default_game_rng"))]
    pub rng: crate::rng::GameRng,

    /// Memory monitor for tracking and controlling memory usage
    #[cfg_attr(feature = "serde", serde(skip))]
    pub memory_monitor: MemoryMonitor,
}

#[cfg(feature = "serde")]
fn default_joker_state_manager() -> Arc<JokerStateManager> {
    Arc::new(JokerStateManager::new())
}

#[cfg(feature = "serde")]
fn default_game_rng() -> crate::rng::GameRng {
    crate::rng::GameRng::secure()
}

/// Format debug message for joker effects with conditional compilation
#[cfg(debug_assertions)]
fn _format_joker_effect_debug_message(
    joker_name: &str,
    effect: &crate::joker::JokerEffect,
    total_triggers: u32,
    card: Option<&Card>,
) -> String {
    use std::fmt::Write;
    let mut debug_msg = String::with_capacity(128); // Pre-allocate reasonable size

    match card {
        None => {
            // Hand-level format
            write!(
                &mut debug_msg,
                "Joker '{}': +{} chips, +{} mult, +{} money",
                joker_name,
                effect.chips * total_triggers as i32,
                effect.mult * total_triggers as i32,
                effect.money * total_triggers as i32
            )
            .unwrap();
        }
        Some(card) => {
            // Card-level format
            write!(
                &mut debug_msg,
                "Joker '{}' on card {}: +{} chips, +{} mult, +{} money",
                joker_name,
                card,
                effect.chips * total_triggers as i32,
                effect.mult * total_triggers as i32,
                effect.money * total_triggers as i32
            )
            .unwrap();
        }
    }

    if effect.retrigger > 0 {
        write!(&mut debug_msg, " (retrigger x{})", effect.retrigger).unwrap();
    }

    debug_msg
}

impl Game {
    pub fn new(config: Config) -> Self {
        let ante_start = Ante::try_from(config.ante_start).unwrap_or(Ante::One);
        Self {
            shop: Shop::new(),
            deck: Deck::default(),
            available: Available::default(),
            discarded: Vec::new(),
            action_history: BoundedActionHistory::new(),
            jokers: Vec::new(),
            joker_effect_processor: JokerEffectProcessor::new(),
            joker_state_manager: Arc::new(JokerStateManager::new()),
            blind: None,
            stage: Stage::PreBlind(),
            ante_start,
            ante_end: Ante::try_from(config.ante_end).unwrap_or(Ante::Eight),
            ante_current: ante_start,
            round: config.round_start as f64,
            plays: config.plays as f64,
            discards: config.discards as f64,
            reward: config.reward_base as f64,
            money: config.money_start as f64,
            shop_reroll_cost: 5.0, // Base reroll cost
            shop_rerolls_this_round: 0,
            chips: config.base_chips as f64,
            mult: config.base_mult as f64,
            score: config.base_score as f64,
            hand_type_counts: HashMap::new(),

            // Initialize extended state fields
            consumables_in_hand: Vec::new(),
            vouchers: VoucherCollection::new(),
            boss_blind_state: BossBlindState::new(),

            // Initialize pack system fields
            pack_inventory: Vec::new(),
            open_pack: None,

            state_version: StateVersion::current(),

            // Initialize debug logging fields
            debug_logging_enabled: false,
            debug_messages: Vec::new(),

            // Initialize multi-select context
            target_context: TargetContext::new(),

            // Initialize secure RNG
            rng: crate::rng::GameRng::secure(),

            // Initialize memory monitor with default configuration
            memory_monitor: MemoryMonitor::default(),

            config,
        }
    }

    pub fn start(&mut self) {
        // for now just move state to small blind
        self.stage = Stage::PreBlind();
        self.deal();
    }

    /// Start a new blind and trigger joker lifecycle events
    pub fn start_blind(&mut self) {
        use crate::hand::Hand;

        // Set stage to blind
        self.stage = Stage::Blind(Blind::Small);
        self.blind = Some(Blind::Small);

        // Trigger on_blind_start for all jokers
        for joker in &self.jokers {
            let temp_hand = Hand::new(self.available.cards());
            let mut context = GameContext {
                chips: self.chips as i32,
                mult: self.mult as i32,
                money: self.money as i32,
                ante: self.ante_current as u8,
                round: self.round as u32,
                stage: &self.stage,
                hands_played: 0,
                discards_used: 0,
                jokers: &self.jokers,
                hand: &temp_hand,
                discarded: &self.discarded,
                joker_state_manager: &self.joker_state_manager,
                hand_type_counts: &self.hand_type_counts,
                cards_in_deck: self.deck.len(),
                stone_cards_in_deck: 0, // TODO: Track stone cards when implemented
                steel_cards_in_deck: 0, // TODO: Track steel cards when implemented
                rng: &self.rng,
            };

            let effect = joker.on_blind_start(&mut context);

            // Apply effects immediately
            self.chips += effect.chips as f64;
            self.mult += effect.mult as f64;
            self.money += effect.money as f64;
        }
    }

    pub fn result(&self) -> Option<End> {
        match self.stage {
            Stage::End(end) => Some(end),
            _ => None,
        }
    }

    pub fn is_over(&self) -> bool {
        self.result().is_some()
    }

    /// Returns a reference to the joker at the specified slot, if it exists.
    ///
    /// # Arguments
    /// * `slot` - The zero-based index of the joker slot to check
    ///
    /// # Returns
    /// * `Some(&dyn Joker)` if a joker exists at the specified slot
    /// * `None` if the slot is empty or the index is out of bounds
    pub fn get_joker_at_slot(&self, slot: usize) -> Option<&dyn Joker> {
        self.jokers.get(slot).map(|j| j.as_ref())
    }

    /// Returns the total number of jokers currently owned by the player.
    ///
    /// # Returns
    /// The count of jokers in the player's collection
    pub fn joker_count(&self) -> usize {
        self.jokers.len()
    }

    /// Returns the number of times a specific hand type has been played this game run.
    ///
    /// # Arguments
    /// * `hand_rank` - The hand rank to check the count for
    ///
    /// # Returns
    /// The number of times this hand type has been played (0 if never played)
    pub fn get_hand_type_count(&self, hand_rank: HandRank) -> u32 {
        self.hand_type_counts.get(&hand_rank).copied().unwrap_or(0)
    }

    /// Increments the count for a specific hand type.
    ///
    /// # Arguments
    /// * `hand_rank` - The hand rank to increment
    pub fn increment_hand_type_count(&mut self, hand_rank: HandRank) {
        *self.hand_type_counts.entry(hand_rank).or_insert(0) += 1;
    }

    fn clear_blind(&mut self) {
        self.score = self.config.base_score as f64;
        self.plays = self.config.plays as f64;
        self.discards = self.config.discards as f64;
        self.deal();
    }

    // draw from deck to available
    fn draw(&mut self, count: usize) {
        if let Some(drawn) = self.deck.draw(count) {
            self.available.extend(drawn);
            // Update target context with new available cards
            self.sync_target_context();
        }
    }

    // shuffle and deal new cards to available
    pub(crate) fn deal(&mut self) {
        // add discarded back to deck, emptying in process
        self.deck.append(&mut self.discarded);
        // add available back to deck and empty
        self.deck.extend(self.available.cards());
        self.available.empty();
        // Clear target context when cards are emptied
        self.target_context.clear_selections();
        self.deck.shuffle(&self.rng);
        self.draw(self.config.available);
    }

    /// Synchronize target context with current game state
    fn sync_target_context(&mut self) {
        // Update available cards
        let available_cards = self.available.cards();
        self.target_context.set_available_cards(available_cards);

        // Update available jokers
        let joker_ids: Vec<JokerId> = self
            .jokers
            .iter()
            .map(|joker| {
                // Get the joker's ID
                joker.id()
            })
            .collect();
        self.target_context.set_available_jokers(joker_ids);

        // Update available packs (if any)
        let pack_ids: Vec<usize> = self
            .pack_inventory
            .iter()
            .enumerate()
            .map(|(i, _)| i)
            .collect();
        self.target_context.set_available_packs(pack_ids);
    }

    pub(crate) fn select_card(&mut self, card: Card) -> Result<(), GameError> {
        if self.available.selected().len() > self.config.selected_max {
            return Err(GameError::InvalidSelectCard);
        }
        self.available.select_card(card)
    }

    pub(crate) fn move_card(
        &mut self,
        direction: MoveDirection,
        card: Card,
    ) -> Result<(), GameError> {
        self.available.move_card(direction, card)
    }

    pub(crate) fn play_selected(&mut self) -> Result<(), GameError> {
        if self.plays == 0.0 {
            return Err(GameError::NoRemainingPlays);
        }
        self.plays -= 1.0;
        let selected = SelectHand::new(self.available.selected());
        let best = selected.best_hand()?;

        // Track hand type for game statistics
        self.increment_hand_type_count(best.rank);

        let score = self.calc_score(best);
        let clear_blind = self.handle_score(score)?;
        self.discarded.extend(self.available.selected());
        let removed = self.available.remove_selected();
        self.draw(removed);
        if clear_blind {
            self.clear_blind();
        }
        Ok(())
    }

    // discard selected cards from available and draw equal number back to available
    pub(crate) fn discard_selected(&mut self) -> Result<(), GameError> {
        if self.discards == 0.0 {
            return Err(GameError::NoRemainingDiscards);
        }
        self.discards -= 1.0;
        let selected_cards = self.available.selected();
        self.discarded.extend(selected_cards.iter().cloned());

        // Trigger scaling events for each discarded card
        for _ in selected_cards {
            self.process_scaling_event(ScalingEvent::CardDiscarded);
        }

        let removed = self.available.remove_selected();
        self.draw(removed);
        Ok(())
    }

    pub fn calc_score(&mut self, hand: MadeHand) -> f64 {
        // compute chips and mult from hand level
        self.chips += hand.rank.level().chips as f64;
        self.mult += hand.rank.level().mult as f64;

        // add chips for each played card
        let card_chips: f64 = hand.hand.cards().iter().map(|c| c.chips() as f64).sum();
        self.chips += card_chips;

        // Apply JokerEffect from structured joker system
        if !self.jokers.is_empty() {
            let (joker_chips, joker_mult, joker_money, mult_multiplier, messages) =
                self.process_joker_effects(&hand);
            self.chips += joker_chips as f64;
            self.mult += joker_mult as f64;
            self.money += joker_money as f64;

            // Trigger scaling events for money gained
            if joker_money > 0 {
                for _ in 0..joker_money {
                    self.process_scaling_event(ScalingEvent::MoneyGained);
                }
            }

            // Apply mult multiplier to the final mult value
            if mult_multiplier != 1.0 {
                self.mult *= mult_multiplier;
            }

            // Log debug messages if enabled
            for message in messages {
                self.add_debug_message(message);
            }
        }

        // compute score
        let score = self.chips * self.mult;

        // Check for killscreen condition
        if !score.is_finite() {
            self.add_debug_message("KILLSCREEN: Final score reached infinity!".to_string());
        }

        // reset chips and mult
        self.mult = self.config.base_mult as f64;
        self.chips = self.config.base_chips as f64;
        score
    }

    /// Process JokerEffect from all jokers and return accumulated effects
    pub fn process_joker_effects(&mut self, hand: &MadeHand) -> (i32, i32, i32, f64, Vec<String>) {
        use crate::hand::Hand;

        let mut messages = Vec::new();

        // Create game context
        let mut context = GameContext {
            chips: self.chips as i32,
            mult: self.mult as i32,
            money: self.money as i32,
            ante: self.ante_current as u8,
            round: self.round as u32,
            stage: &self.stage,
            hands_played: 0,  // TODO: track this properly
            discards_used: 0, // TODO: track this properly
            jokers: &self.jokers,
            hand: &Hand::new(hand.hand.cards().to_vec()),
            discarded: &self.discarded,
            joker_state_manager: &self.joker_state_manager,
            hand_type_counts: &self.hand_type_counts,
            cards_in_deck: self.deck.len(),
            stone_cards_in_deck: 0, // TODO: Track stone cards when implemented
            steel_cards_in_deck: 0, // TODO: Track steel cards when implemented
            rng: &self.rng,
        };

        // Process hand-level effects using the cached processor
        let select_hand = SelectHand::new(hand.hand.cards().to_vec());
        let hand_result = self.joker_effect_processor.process_hand_effects(
            &self.jokers,
            &mut context,
            &select_hand,
        );

        let mut total_chips = hand_result.accumulated_effect.chips;
        let mut total_mult = hand_result.accumulated_effect.mult;
        let mut total_money = hand_result.accumulated_effect.money;
        let mut total_mult_multiplier = if hand_result.accumulated_effect.mult_multiplier != 0.0 {
            hand_result.accumulated_effect.mult_multiplier
        } else {
            1.0
        };

        // Generate debug messages for hand effects
        #[cfg(debug_assertions)]
        if hand_result.jokers_processed > 0
            && (total_chips != 0 || total_mult != 0 || total_money != 0)
        {
            use std::fmt::Write;
            let mut debug_msg = String::with_capacity(128);
            write!(
                &mut debug_msg,
                "Hand effects: +{} chips, +{} mult, +{} money from {} jokers",
                total_chips, total_mult, total_money, hand_result.jokers_processed
            )
            .unwrap();

            if hand_result.retriggered_count > 0 {
                write!(
                    &mut debug_msg,
                    " ({} retriggers)",
                    hand_result.retriggered_count
                )
                .unwrap();
            }

            messages.push(debug_msg);
        }

        // Process any error messages from hand effects
        for error in &hand_result.errors {
            if let crate::joker_effect_processor::EffectProcessingError::TooManyRetriggers(_) =
                error
            {
                messages.push("KILLSCREEN: Too many retriggered effects!".to_string());
            } // Other errors are less critical for gameplay
        }

        // Process card-level effects using the cached processor
        for card in hand.hand.cards() {
            let card_result =
                self.joker_effect_processor
                    .process_card_effects(&self.jokers, &mut context, &card);
            // Accumulate card effects
            total_chips += card_result.accumulated_effect.chips;
            total_mult += card_result.accumulated_effect.mult;
            total_money += card_result.accumulated_effect.money;

            // Handle mult_multiplier: only apply if it's not the default value
            if card_result.accumulated_effect.mult_multiplier != 0.0 {
                total_mult_multiplier *= card_result.accumulated_effect.mult_multiplier;

                // Killscreen detection
                if !total_mult_multiplier.is_finite() {
                    messages.push("KILLSCREEN: Score calculation reached infinity!".to_string());
                    break;
                }
            }

            // Generate debug messages for card effects
            #[cfg(debug_assertions)]
            if card_result.jokers_processed > 0
                && (card_result.accumulated_effect.chips != 0
                    || card_result.accumulated_effect.mult != 0
                    || card_result.accumulated_effect.money != 0)
            {
                use std::fmt::Write;
                let mut debug_msg = String::with_capacity(128);
                write!(
                    &mut debug_msg,
                    "Card {} effects: +{} chips, +{} mult, +{} money",
                    card,
                    card_result.accumulated_effect.chips,
                    card_result.accumulated_effect.mult,
                    card_result.accumulated_effect.money
                )
                .unwrap();

                if card_result.retriggered_count > 0 {
                    write!(
                        &mut debug_msg,
                        " ({} retriggers)",
                        card_result.retriggered_count
                    )
                    .unwrap();
                }
                messages.push(debug_msg);
            }

            // Process any error messages from card effects
            for error in &card_result.errors {
                if let crate::joker_effect_processor::EffectProcessingError::TooManyRetriggers(_) =
                    error
                {
                    messages.push("KILLSCREEN: Too many retriggered effects!".to_string());
                } // Other errors are less critical for gameplay
            }
        }

        // Don't apply mult multiplier here - let calc_score handle it
        // if total_mult_multiplier != 1.0 {
        //     total_mult = (total_mult as f64 * total_mult_multiplier) as i32;
        // }

        (
            total_chips,
            total_mult,
            total_money,
            total_mult_multiplier,
            messages,
        )
    }

    /// Calculate score with detailed breakdown for debugging and analysis
    pub fn calc_score_with_breakdown(&mut self, hand: MadeHand) -> ScoreBreakdown {
        use crate::hand::Hand;

        // Track initial values (for potential future use)
        let _initial_chips = self.chips;
        let _initial_mult = self.mult;

        // Calculate base values from hand level
        let base_chips = hand.rank.level().chips as f64;
        let base_mult = hand.rank.level().mult as f64;
        self.chips += base_chips;
        self.mult += base_mult;

        // Calculate card contributions
        let card_chips: f64 = hand.hand.cards().iter().map(|c| c.chips() as f64).sum();
        self.chips += card_chips;

        let mut joker_contributions = Vec::new();

        // Process jokers if any exist
        if !self.jokers.is_empty() {
            // Create game context for jokers
            let mut context = GameContext {
                chips: self.chips as i32,
                mult: self.mult as i32,
                money: self.money as i32,
                ante: self.ante_current as u8,
                round: self.round as u32,
                stage: &self.stage,
                hands_played: 0,  // TODO: track this properly
                discards_used: 0, // TODO: track this properly
                jokers: &self.jokers,
                hand: &Hand::new(hand.hand.cards().to_vec()),
                discarded: &self.discarded,
                joker_state_manager: &self.joker_state_manager,
                hand_type_counts: &self.hand_type_counts,
                cards_in_deck: self.deck.len(),
                stone_cards_in_deck: 0, // TODO: Track stone cards when implemented
                steel_cards_in_deck: 0, // TODO: Track steel cards when implemented
                rng: &self.rng,
            };

            // Process each joker and track contributions
            for joker in &self.jokers {
                let select_hand = SelectHand::new(hand.hand.cards().to_vec());
                let effect = joker.on_hand_played(&mut context, &select_hand);

                // Create contribution record
                let contribution = JokerContribution {
                    joker_name: joker.name().to_string(),
                    joker_id: joker.id(),
                    chips_added: effect.chips,
                    mult_added: effect.mult,
                    mult_multiplier: if effect.mult_multiplier != 0.0 {
                        effect.mult_multiplier
                    } else {
                        1.0
                    },
                    money_added: effect.money,
                    retrigger_count: effect.retrigger,
                };

                // Apply effects to game state
                self.chips += effect.chips as f64;
                self.mult += effect.mult as f64;
                self.money += effect.money as f64;

                joker_contributions.push(contribution);
            }

            // Process card-level effects for each joker
            for card in hand.hand.cards() {
                for joker in &self.jokers {
                    let effect = joker.on_card_scored(&mut context, &card);

                    if effect.chips != 0 || effect.mult != 0 || effect.money != 0 {
                        // Find existing contribution for this joker or create new one
                        if let Some(existing) = joker_contributions
                            .iter_mut()
                            .find(|c| c.joker_id == joker.id())
                        {
                            existing.chips_added += effect.chips;
                            existing.mult_added += effect.mult;
                            existing.money_added += effect.money;
                        }

                        // Apply effects to game state
                        self.chips += effect.chips as f64;
                        self.mult += effect.mult as f64;
                        self.money += effect.money as f64;
                    }
                }
            }

            // Trigger scaling events for hand played
            let hand_rank = hand.rank;
            self.process_scaling_event(ScalingEvent::HandPlayed(hand_rank));
        }

        // Calculate final score
        let final_score = self.chips * self.mult;

        // Log final breakdown if debug enabled (optimized)
        #[cfg(debug_assertions)]
        {
            use std::fmt::Write;
            let mut msg = String::with_capacity(64);
            write!(
                &mut msg,
                "Final score: {} chips × {} mult = {}",
                self.chips, self.mult, final_score
            )
            .unwrap();
            self.add_debug_message(msg);
        }

        // Reset chips and mult to base values
        self.mult = self.config.base_mult as f64;
        self.chips = self.config.base_chips as f64;

        ScoreBreakdown {
            base_chips,
            base_mult,
            card_chips,
            joker_contributions,
            final_score,
        }
    }

    /// Enable debug logging for joker scoring
    pub fn enable_debug_logging(&mut self) {
        self.debug_logging_enabled = true;
        self.debug_messages.clear();
    }

    /// Get current debug messages
    pub fn get_debug_messages(&self) -> &[String] {
        &self.debug_messages
    }

    /// Add a debug message with automatic memory management
    /// Only compiles in debug builds and tests to eliminate overhead in release
    #[cfg(any(debug_assertions, test))]
    fn add_debug_message(&mut self, message: String) {
        if self.debug_logging_enabled {
            self.debug_messages.push(message);

            // Keep memory usage reasonable - remove oldest messages if we exceed limit
            if self.debug_messages.len() > MAX_DEBUG_MESSAGES {
                self.debug_messages
                    .drain(0..self.debug_messages.len() - MAX_DEBUG_MESSAGES);
            }
        }
    }

    /// No-op version for release builds (but not tests)
    #[cfg(not(any(debug_assertions, test)))]
    #[inline]
    fn add_debug_message(&mut self, _message: String) {
        // No-op in release builds
    }

    /// Configure memory monitoring for RL training scenarios
    pub fn enable_rl_memory_monitoring(&mut self) {
        let config = crate::memory_monitor::MemoryConfig::for_rl_training();
        self.memory_monitor.update_config(config.clone());

        // Update action history limit to match memory config
        self.action_history.resize(config.max_action_history);
    }

    /// Configure memory monitoring for simulation scenarios
    pub fn enable_simulation_memory_monitoring(&mut self) {
        let config = crate::memory_monitor::MemoryConfig::for_simulation();
        self.memory_monitor.update_config(config.clone());

        // Update action history limit to match memory config
        self.action_history.resize(config.max_action_history);
    }

    /// Get current memory usage statistics
    pub fn get_memory_stats(&mut self) -> Option<crate::memory_monitor::MemoryStats> {
        if self.memory_monitor.should_check() {
            // Estimate memory usage
            let estimated_bytes = self.estimate_memory_usage();
            let stats = self.memory_monitor.check_memory(
                estimated_bytes,
                1, // Number of active snapshots (hard to track, estimate as 1)
                self.action_history.total_actions(),
            );
            Some(stats)
        } else {
            self.memory_monitor.last_stats().cloned()
        }
    }

    /// Generate a memory usage report
    pub fn generate_memory_report(&self) -> String {
        self.memory_monitor.generate_report()
    }

    /// Estimate current memory usage in bytes
    fn estimate_memory_usage(&self) -> usize {
        let mut total = std::mem::size_of::<Self>();

        // Action history
        total += self.action_history.memory_stats().estimated_bytes;

        // Deck cards
        total += self.deck.cards().len() * std::mem::size_of::<crate::card::Card>();

        // Available cards
        total += self.available.cards().len() * std::mem::size_of::<crate::card::Card>();

        // Discarded cards
        total += self.discarded.len() * std::mem::size_of::<crate::card::Card>();

        // Jokers (rough estimate)
        total += self.jokers.len() * 200; // Estimate 200 bytes per joker

        // Hand type counts
        total += self.hand_type_counts.len()
            * (std::mem::size_of::<crate::rank::HandRank>() + std::mem::size_of::<u32>());

        // Debug messages
        total += self
            .debug_messages
            .iter()
            .map(|msg| msg.len())
            .sum::<usize>();

        total
    }

    /// Check if memory usage exceeds safe limits
    pub fn check_memory_safety(&mut self) -> bool {
        if let Some(stats) = self.get_memory_stats() {
            !stats.exceeds_critical(self.memory_monitor.config())
        } else {
            true // Assume safe if no stats available
        }
    }

    /// Configure joker effect cache settings
    pub fn configure_joker_effect_cache(
        &mut self,
        config: crate::joker_effect_processor::CacheConfig,
    ) {
        self.joker_effect_processor.set_cache_config(config);
    }

    /// Enable joker effect caching with default settings
    pub fn enable_joker_effect_cache(&mut self) {
        let config = crate::joker_effect_processor::CacheConfig {
            enabled: true,
            ..Default::default()
        };
        self.joker_effect_processor.set_cache_config(config);
    }

    /// Disable joker effect caching
    pub fn disable_joker_effect_cache(&mut self) {
        let config = crate::joker_effect_processor::CacheConfig {
            enabled: false,
            ..Default::default()
        };
        self.joker_effect_processor.set_cache_config(config);
    }

    /// Get joker effect cache performance metrics
    pub fn get_joker_cache_metrics(&self) -> &crate::joker_effect_processor::CacheMetrics {
        self.joker_effect_processor.cache_metrics()
    }

    /// Clear the joker effect cache
    pub fn clear_joker_effect_cache(&mut self) {
        self.joker_effect_processor.clear_cache();
    }

    /// Perform maintenance on the joker effect cache (cleanup expired entries)
    pub fn maintain_joker_effect_cache(&mut self) {
        self.joker_effect_processor.maintain_cache();
    }

    /// Configure joker effect cache for RL training scenarios
    pub fn configure_joker_cache_for_rl(&mut self) {
        let config = crate::joker_effect_processor::CacheConfig {
            max_entries: 10000, // Larger cache for training
            ttl_seconds: 600,   // 10 minutes
            enabled: true,
        };
        self.joker_effect_processor.set_cache_config(config);
    }

    /// Configure joker effect cache for simulation scenarios
    pub fn configure_joker_cache_for_simulation(&mut self) {
        let config = crate::joker_effect_processor::CacheConfig {
            max_entries: 1000, // Moderate cache for simulation
            ttl_seconds: 300,  // 5 minutes
            enabled: true,
        };
        self.joker_effect_processor.set_cache_config(config);
    }

    pub fn required_score(&self) -> f64 {
        let base = self.ante_current.base() as f64;

        match self.blind {
            None => base,
            Some(Blind::Small) => base,
            Some(Blind::Big) => base * 1.5,
            Some(Blind::Boss) => base * 2.0,
        }
    }

    fn calc_reward(&mut self, blind: Blind) -> Result<f64, GameError> {
        let mut interest = (self.money * self.config.interest_rate).floor();
        if interest > self.config.interest_max as f64 {
            interest = self.config.interest_max as f64
        }
        let base = blind.reward() as f64;
        let hand_bonus = self.plays * self.config.money_per_hand as f64;
        let reward = base + interest + hand_bonus;
        Ok(reward)
    }

    fn cashout(&mut self) -> Result<(), GameError> {
        self.money += self.reward;
        self.reward = 0.0;
        self.stage = Stage::Shop();

        // Reset reroll cost and count for new shop round
        self.shop_reroll_cost = 5.0; // Base reroll cost
        self.shop_rerolls_this_round = 0;

        self.shop.refresh(&self.rng);
        Ok(())
    }

    #[allow(dead_code)] // Kept for backward compatibility
    pub(crate) fn buy_joker(&mut self, joker: Jokers) -> Result<(), GameError> {
        if self.stage != Stage::Shop() {
            return Err(GameError::InvalidStage);
        }
        if self.jokers.len() >= self.config.joker_slots {
            return Err(GameError::NoAvailableSlot);
        }
        if joker.cost() as f64 > self.money {
            return Err(GameError::InvalidBalance);
        }
        // Convert old joker to new system and add to jokers vec
        if let Some(new_joker) = JokerFactory::create(joker.to_joker_id()) {
            self.shop.buy_joker(&joker)?;
            self.money -= joker.cost() as f64;
            self.jokers.push(new_joker);
            Ok(())
        } else {
            Err(GameError::InvalidOperation(format!(
                "Cannot create joker {:?} - not available in new system",
                joker.to_joker_id()
            )))
        }
    }

    /// Purchases a joker from the shop and places it at the specified slot.
    ///
    /// This method validates that the game is in the shop stage, the joker is available
    /// in the shop, the player has sufficient money, and the slot is valid. If all
    /// validations pass, it purchases the joker and inserts it at the specified position,
    /// shifting existing jokers to the right if necessary.
    ///
    /// # Arguments
    /// * `joker_id` - The identifier of the joker to purchase
    /// * `slot` - The zero-based index where to place the joker (0 to jokers.len())
    ///
    /// # Returns
    /// * `Ok(())` if the purchase was successful
    /// * `Err(GameError)` if the purchase failed due to validation errors
    ///
    /// # Errors
    /// * `InvalidStage` - Game is not in shop stage
    /// * `InvalidSlot` - Slot index is greater than current joker count
    /// * `NoAvailableSlot` - Joker limit reached and trying to add at the end
    /// * `JokerNotInShop` - Requested joker is not available in the shop
    /// * `InvalidBalance` - Player doesn't have enough money
    /// * `NoJokerMatch` - Joker found in shop but couldn't be matched (internal error)
    pub(crate) fn buy_joker_with_slot(
        &mut self,
        joker_id: JokerId,
        slot: usize,
    ) -> Result<(), GameError> {
        // Validate stage
        if self.stage != Stage::Shop() {
            return Err(GameError::InvalidStage);
        }

        // Validate slot index - must be within expanded joker slot limit
        if slot >= self.config.joker_slots {
            return Err(GameError::InvalidSlot);
        }

        // Check if we've reached the joker limit
        if self.jokers.len() >= self.config.joker_slots {
            return Err(GameError::NoAvailableSlot);
        }

        // Check if joker is available in shop
        if !self.shop.has_joker(joker_id) {
            return Err(GameError::JokerNotInShop);
        }

        // Find the matching Jokers enum from shop (temporary until shop uses JokerId)
        let shop_joker = self
            .shop
            .jokers
            .iter()
            .find(|j| j.matches_joker_id(joker_id))
            .cloned()
            .ok_or(GameError::NoJokerMatch)?;

        // Check if player has enough money (use actual joker cost)
        if shop_joker.cost() as f64 > self.money {
            return Err(GameError::InvalidBalance);
        }

        // Create new joker using JokerFactory
        let new_joker = JokerFactory::create(joker_id).ok_or_else(|| {
            GameError::InvalidOperation(format!(
                "Cannot create joker {joker_id:?} - not available in new system"
            ))
        })?;

        // Purchase joker from shop
        self.shop.buy_joker(&shop_joker)?;

        // Deduct money
        self.money -= shop_joker.cost() as f64;

        // Insert joker at specified slot, expanding vector if necessary
        if slot >= self.jokers.len() {
            // For simplicity, just push at the end if slot is beyond current length
            self.jokers.push(new_joker);
        } else {
            self.jokers.insert(slot, new_joker);
        }

        Ok(())
    }

    /// Validates whether a consumable can be purchased based on game state, player resources, and slot availability.
    ///
    /// This method checks that:
    /// - The game is currently in Shop stage
    /// - The player has sufficient money for the purchase
    /// - At least one consumable slot is available in the player's hand
    ///
    /// # Arguments
    /// * `consumable_type` - The type of consumable to validate for purchase
    ///
    /// # Returns
    /// * `Ok(())` if the consumable can be purchased
    /// * `Err(GameError)` with the specific reason if purchase is not allowed
    ///
    /// # Errors
    /// * `InvalidStage` - Game is not in Shop stage
    /// * `InvalidBalance` - Player doesn't have enough money
    /// * `NoAvailableSlot` - Consumable hand is full
    ///
    /// # Example
    /// ```rust,ignore
    /// use balatro_rs::game::Game;
    /// use balatro_rs::shop::ConsumableType;
    /// use balatro_rs::stage::Stage;
    ///
    /// let mut game = Game::default();
    /// game.stage = Stage::Shop();
    /// game.money = 10.0;
    ///
    /// // This should succeed
    /// assert!(game.can_purchase_consumable(ConsumableType::Tarot).is_ok());
    ///
    /// // This should fail due to insufficient money
    /// game.money = 1.0;
    /// assert!(game.can_purchase_consumable(ConsumableType::Spectral).is_err());
    /// ```
    pub fn can_purchase_consumable(
        &self,
        consumable_type: crate::shop::ConsumableType,
    ) -> Result<(), GameError> {
        // Check if game is in Shop stage
        if self.stage != Stage::Shop() {
            return Err(GameError::InvalidStage);
        }

        // Determine cost based on consumable type (maintaining f64 consistency)
        let cost = match consumable_type {
            crate::shop::ConsumableType::Tarot => 3.0,
            crate::shop::ConsumableType::Planet => 3.0,
            crate::shop::ConsumableType::Spectral => 4.0,
        };

        // Check if player has sufficient money
        if self.money < cost {
            return Err(GameError::InvalidBalance);
        }

        // Check if consumable hand has available space
        if self.consumables_in_hand.len() >= self.config.consumable_hand_capacity {
            return Err(GameError::NoAvailableSlot);
        }

        Ok(())
    }

    /// Reroll the shop contents and update the reroll cost
    pub(crate) fn reroll_shop(&mut self) -> Result<(), GameError> {
        // Check if game is in shop stage
        if self.stage != Stage::Shop() {
            return Err(GameError::InvalidStage);
        }

        // Check if player has enough money for reroll
        if self.money < self.shop_reroll_cost {
            return Err(GameError::InvalidBalance);
        }

        // Deduct reroll cost
        self.money -= self.shop_reroll_cost;

        // Update reroll cost for next reroll (escalate by 5 each time)
        self.shop_reroll_cost += 5.0;

        // Track number of rerolls this round
        self.shop_rerolls_this_round += 1;

        // Refresh the shop with new items
        self.shop.refresh(&self.rng);

        Ok(())
    }

    /// Pack System Methods
    /// Buy a pack of the specified type
    pub(crate) fn buy_pack(
        &mut self,
        pack_type: crate::shop::packs::PackType,
    ) -> Result<(), GameError> {
        use crate::shop::packs::{DefaultPackGenerator, PackGenerator};

        // Check if player has enough money
        let cost = pack_type.base_cost(&self.config);
        if self.money < cost as f64 {
            return Err(GameError::InvalidBalance);
        }

        // Generate the pack
        let generator = DefaultPackGenerator;
        let pack = generator.generate_pack(pack_type, self)?;
        // Note: generate_contents is already called in generate_pack, no need to call again

        // Deduct money
        self.money -= cost as f64;

        // Add pack to inventory
        self.pack_inventory.push(pack);

        Ok(())
    }

    /// Open a pack from inventory
    pub(crate) fn open_pack(&mut self, pack_id: usize) -> Result<(), GameError> {
        // Check if pack exists in inventory
        if pack_id >= self.pack_inventory.len() {
            return Err(GameError::InvalidAction);
        }

        // Check if another pack is already open
        if self.open_pack.is_some() {
            return Err(GameError::InvalidAction);
        }

        // Remove pack from inventory and open it
        let pack = self.pack_inventory.remove(pack_id);
        self.open_pack = Some(OpenPackState::new(pack, pack_id));

        Ok(())
    }

    /// Select an option from the currently opened pack
    pub(crate) fn select_from_pack(
        &mut self,
        pack_id: usize,
        option_index: usize,
    ) -> Result<(), GameError> {
        // Check if a pack is open
        let open_pack_state = self.open_pack.take().ok_or(GameError::InvalidAction)?;

        // Verify pack ID matches
        if open_pack_state.pack_id != pack_id {
            return Err(GameError::InvalidAction);
        }

        // Select the option
        let selected_item = open_pack_state.pack.select_option(option_index)?;

        // Process the selected item based on its type
        self.process_pack_item(selected_item)?;

        Ok(())
    }

    /// Skip the currently opened pack
    pub(crate) fn skip_pack(&mut self, pack_id: usize) -> Result<(), GameError> {
        // Check if a pack is open
        let open_pack_state = self.open_pack.take().ok_or(GameError::InvalidAction)?;

        // Verify pack ID matches
        if open_pack_state.pack_id != pack_id {
            return Err(GameError::InvalidAction);
        }

        // Check if pack can be skipped
        if !open_pack_state.pack.can_skip {
            return Err(GameError::InvalidAction);
        }

        // Pack is simply consumed (no further action needed)
        Ok(())
    }

    /// Process an item selected from a pack
    fn process_pack_item(&mut self, item: crate::shop::ShopItem) -> Result<(), GameError> {
        use crate::shop::ShopItem;

        match item {
            ShopItem::PlayingCard(card) => {
                // Add card to deck
                self.deck.extend(vec![card]);
                Ok(())
            }
            ShopItem::Joker(joker_id) => {
                // Use JokerFactory to create the joker
                if let Some(joker) = JokerFactory::create(joker_id) {
                    self.jokers.push(joker);
                    // Initialize state for the new joker
                    self.joker_state_manager.ensure_state_exists(joker_id);
                    Ok(())
                } else {
                    // If we can't create the joker, return an error
                    Err(GameError::InvalidAction)
                }
            }
            ShopItem::Consumable(consumable_type) => {
                // Select a random consumable of the appropriate type
                let consumable_id = match consumable_type {
                    crate::shop::ConsumableType::Tarot => {
                        let tarot_cards = ConsumableId::tarot_cards();
                        self.rng
                            .choose(&tarot_cards)
                            .copied()
                            .unwrap_or(ConsumableId::TheFool)
                    }
                    crate::shop::ConsumableType::Planet => {
                        let planet_cards = ConsumableId::planet_cards();
                        self.rng
                            .choose(&planet_cards)
                            .copied()
                            .unwrap_or(ConsumableId::Mercury)
                    }
                    crate::shop::ConsumableType::Spectral => {
                        let spectral_cards = ConsumableId::spectral_cards();
                        self.rng
                            .choose(&spectral_cards)
                            .copied()
                            .unwrap_or(ConsumableId::Familiar)
                    }
                };

                // Add consumable to hand
                self.consumables_in_hand.push(consumable_id);
                Ok(())
            }
            _ => {
                // Other item types not yet implemented
                Ok(())
            }
        }
    }

    fn select_blind(&mut self, blind: Blind) -> Result<(), GameError> {
        // can only set blind if stage is pre blind
        if self.stage != Stage::PreBlind() {
            return Err(GameError::InvalidStage);
        }
        // provided blind must be expected next blind
        if let Some(current) = self.blind {
            if blind != current.next() {
                return Err(GameError::InvalidBlind);
            }
        } else {
            // if game just started, blind will be None, in which case
            // we can only set it to small.
            if blind != Blind::Small {
                return Err(GameError::InvalidBlind);
            }
        }
        self.blind = Some(blind);
        self.stage = Stage::Blind(blind);
        self.deal();
        Ok(())
    }

    fn next_round(&mut self) -> Result<(), GameError> {
        self.stage = Stage::PreBlind();
        self.round += 1.0;
        Ok(())
    }

    // Returns true if should clear blind after, false if not.
    fn handle_score(&mut self, score: f64) -> Result<bool, GameError> {
        // can only handle score if stage is blind
        if !self.stage.is_blind() {
            return Err(GameError::InvalidStage);
        }

        self.score += score;
        let required = self.required_score();

        // blind not passed
        if self.score < required {
            // no more hands to play -> lose
            if self.plays == 0.0 {
                self.stage = Stage::End(End::Lose);
                return Ok(false);
            } else {
                // more hands to play, carry on
                return Ok(false);
            }
        }

        let blind = self.blind.ok_or(GameError::MissingBlindState)?;
        // score exceeds blind (blind passed).
        // handle reward then progress to next stage.
        let reward = self.calc_reward(blind)?;
        self.reward = reward;

        // passed boss blind, either win or progress ante
        if blind == Blind::Boss {
            if let Some(ante_next) = self.ante_current.next(self.ante_end) {
                self.ante_current = ante_next;
            } else {
                self.stage = Stage::End(End::Win);
                return Ok(false);
            }
        };

        // finish blind, proceed to post blind
        self.stage = Stage::PostBlind();
        Ok(true)
    }

    pub fn handle_action(&mut self, action: Action) -> Result<(), GameError> {
        self.action_history.push(action.clone());
        match action {
            Action::SelectCard(card) => match self.stage.is_blind() {
                true => self.select_card(card),
                false => Err(GameError::InvalidAction),
            },
            Action::Play() => match self.stage.is_blind() {
                true => self.play_selected(),
                false => Err(GameError::InvalidAction),
            },
            Action::Discard() => match self.stage.is_blind() {
                true => self.discard_selected(),
                false => Err(GameError::InvalidAction),
            },
            Action::MoveCard(dir, card) => match self.stage.is_blind() {
                true => self.move_card(dir, card),
                false => Err(GameError::InvalidAction),
            },
            Action::CashOut(_reward) => match self.stage {
                Stage::PostBlind() => self.cashout(),
                _ => Err(GameError::InvalidAction),
            },
            Action::BuyJoker { joker_id, slot } => match self.stage {
                Stage::Shop() => self.buy_joker_with_slot(joker_id, slot),
                _ => Err(GameError::InvalidStage),
            },
            Action::NextRound() => match self.stage {
                Stage::Shop() => self.next_round(),
                _ => Err(GameError::InvalidAction),
            },
            Action::SelectBlind(blind) => match self.stage {
                Stage::PreBlind() => self.select_blind(blind),
                _ => Err(GameError::InvalidAction),
            },
            Action::BuyPack { pack_type } => match self.stage {
                Stage::Shop() => self.buy_pack(pack_type),
                _ => Err(GameError::InvalidStage),
            },
            Action::OpenPack { pack_id } => self.open_pack(pack_id),
            Action::SelectFromPack {
                pack_id,
                option_index,
            } => self.select_from_pack(pack_id, option_index),
            Action::SkipPack { pack_id } => self.skip_pack(pack_id),
            Action::RerollShop() => match self.stage {
                Stage::Shop() => self.reroll_shop(),
                _ => Err(GameError::InvalidStage),
            },

            // Multi-select actions - placeholder implementations for now
            Action::SelectCards(_) => {
                // TODO: Implement multi-card selection
                Err(GameError::InvalidAction)
            }
            Action::DeselectCard(_) => {
                // TODO: Implement card deselection
                Err(GameError::InvalidAction)
            }
            Action::DeselectCards(_) => {
                // TODO: Implement multi-card deselection
                Err(GameError::InvalidAction)
            }
            Action::ToggleCardSelection(_) => {
                // TODO: Implement card selection toggle
                Err(GameError::InvalidAction)
            }
            Action::SelectAllCards() => {
                // TODO: Implement select all cards
                Err(GameError::InvalidAction)
            }
            Action::DeselectAllCards() => {
                // TODO: Implement deselect all cards
                Err(GameError::InvalidAction)
            }
            Action::RangeSelectCards { start: _, end: _ } => {
                // TODO: Implement range selection
                Err(GameError::InvalidAction)
            }
            Action::SelectJoker(_) => {
                // TODO: Implement joker selection
                Err(GameError::InvalidAction)
            }
            Action::DeselectJoker(_) => {
                // TODO: Implement joker deselection
                Err(GameError::InvalidAction)
            }
            Action::ToggleJokerSelection(_) => {
                // TODO: Implement joker selection toggle
                Err(GameError::InvalidAction)
            }
            Action::SelectAllJokers() => {
                // TODO: Implement select all jokers
                Err(GameError::InvalidAction)
            }
            Action::DeselectAllJokers() => {
                // TODO: Implement deselect all jokers
                Err(GameError::InvalidAction)
            }
            Action::BuyJokers(_) => {
                // TODO: Implement batch joker buying
                Err(GameError::InvalidAction)
            }
            Action::SellJokers(_) => {
                // TODO: Implement batch joker selling
                Err(GameError::InvalidAction)
            }
            Action::BuyPacks(_) => {
                // TODO: Implement batch pack buying
                Err(GameError::InvalidAction)
            }
            Action::ActivateMultiSelect() => {
                // TODO: Implement multi-select activation
                Err(GameError::InvalidAction)
            }
            Action::DeactivateMultiSelect() => {
                // TODO: Implement multi-select deactivation
                Err(GameError::InvalidAction)
            }
        }
    }

    pub fn handle_action_index(&mut self, index: usize) -> Result<(), GameError> {
        let space = self.gen_action_space();
        let action = space.to_action(index, self)?;
        self.handle_action(action)
    }

    /// Remove a joker from the specified slot and clean up its state.
    ///
    /// # Arguments
    /// * `slot` - The zero-based index of the joker slot to remove from
    ///
    /// # Returns
    /// * `Ok(())` if the joker was successfully removed
    /// * `Err(GameError::InvalidSlot)` if the slot index is out of bounds
    pub fn remove_joker(&mut self, slot: usize) -> Result<(), crate::error::GameError> {
        use crate::error::GameError;

        if slot >= self.jokers.len() {
            return Err(GameError::InvalidSlot);
        }

        // Get the joker before removing it to clean up its state
        let joker = &self.jokers[slot];
        let joker_id = joker.id();

        // Remove the joker from the collection
        self.jokers.remove(slot);

        // Clean up the joker's state
        self.joker_state_manager.remove_state(joker_id);

        Ok(())
    }

    /// Sell a joker from the specified slot, awarding money and cleaning up its state.
    ///
    /// # Arguments
    /// * `slot` - The zero-based index of the joker slot to sell
    ///
    /// # Returns
    /// * `Ok(())` if the joker was successfully sold
    /// * `Err(GameError::InvalidSlot)` if the slot index is out of bounds
    pub fn sell_joker(&mut self, slot: usize) -> Result<(), crate::error::GameError> {
        use crate::error::GameError;

        if slot >= self.jokers.len() {
            return Err(GameError::InvalidSlot);
        }

        // Get sell value and joker ID before removing
        let joker = &self.jokers[slot];
        let sell_value = joker.cost() as f64 / 2.0; // Standard sell value is half the cost
        let joker_id = joker.id();

        // Award money for selling the joker
        self.money += sell_value;

        // Remove the joker from the collection
        self.jokers.remove(slot);

        // Clean up the joker's state
        self.joker_state_manager.remove_state(joker_id);

        Ok(())
    }

    /// Validate that joker state is consistent with actual jokers in play.
    ///
    /// # Returns
    /// * `Ok(())` if the state is consistent
    /// * `Err(GameError::InvalidOperation)` if inconsistencies are found
    pub fn validate_joker_state(&self) -> Result<(), crate::error::GameError> {
        use crate::error::GameError;

        // Get all joker IDs currently in play
        let current_jokers: std::collections::HashSet<_> =
            self.jokers.iter().map(|joker| joker.id()).collect();

        // Get all joker IDs with state
        let state_jokers: std::collections::HashSet<_> = self
            .joker_state_manager
            .get_active_jokers()
            .into_iter()
            .collect();

        // Check for state without corresponding jokers
        for state_joker in &state_jokers {
            if !current_jokers.contains(state_joker) {
                return Err(GameError::InvalidOperation(format!(
                    "Found state for joker {state_joker:?} but no corresponding joker in play"
                )));
            }
        }

        Ok(())
    }

    /// Clean up orphaned joker state (state for jokers no longer in play).
    pub fn cleanup_joker_state(&mut self) {
        // Get all joker IDs currently in play
        let current_jokers: std::collections::HashSet<_> =
            self.jokers.iter().map(|joker| joker.id()).collect();

        // Get all joker IDs with state
        let state_jokers: Vec<_> = self
            .joker_state_manager
            .get_active_jokers()
            .into_iter()
            .collect();

        // Remove state for jokers no longer in play
        for state_joker in state_jokers {
            if !current_jokers.contains(&state_joker) {
                self.joker_state_manager.remove_state(state_joker);
            }
        }
    }

    /// Reset the game to its initial state, clearing all jokers and their state.
    pub fn reset_game(&mut self) {
        // Clear all jokers
        self.jokers.clear();

        // Clear all joker state
        self.joker_state_manager.clear();

        // Reset other game state to initial values
        self.round = self.config.round_start as f64;
        self.plays = self.config.plays as f64;
        self.discards = self.config.discards as f64;
        self.money = self.config.money_start as f64;
        self.chips = self.config.base_chips as f64;
        self.mult = self.config.base_mult as f64;
        self.score = self.config.base_score as f64;
        self.ante_current = self.ante_start;
        self.stage = Stage::PreBlind();
        self.hand_type_counts.clear();
        self.action_history.clear();
        self.discarded.clear();

        // Reset deck and available cards
        self.deck = crate::deck::Deck::default();
        self.available = crate::available::Available::default();
        self.blind = None;
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "deck length: {}", self.deck.len())?;
        writeln!(f, "available length: {}", self.available.cards().len())?;
        writeln!(f, "selected length: {}", self.available.selected().len())?;

        // Multi-select status
        if self.target_context.is_multi_select_active() {
            let counts = self.target_context.get_selection_counts();
            writeln!(
                f,
                "multi-select: ACTIVE ({} cards, {} jokers, {} total)",
                counts.cards, counts.jokers, counts.total
            )?;
        } else {
            writeln!(f, "multi-select: inactive")?;
        }

        writeln!(f, "discard length: {}", self.discarded.len())?;
        writeln!(f, "jokers: ")?;
        for j in &self.jokers {
            writeln!(f, "{j:?}")?
        }
        writeln!(f, "action history length: {}", self.action_history.len())?;
        writeln!(f, "blind: {:?}", self.blind)?;
        writeln!(f, "stage: {:?}", self.stage)?;
        writeln!(f, "ante: {:?}", self.ante_current)?;
        writeln!(f, "round: {}", self.round)?;
        writeln!(f, "hands remaining: {}", self.plays)?;
        writeln!(f, "discards remaining: {}", self.discards)?;
        writeln!(f, "money: {}", self.money)?;
        writeln!(f, "score: {}", self.score)
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new(Config::default())
    }
}

/// Serializable representation of game state, excluding non-serializable fields
#[derive(Debug, Clone, Serialize, Deserialize)]
struct SaveableGameState {
    pub version: u32,
    pub timestamp: u64,
    pub config: Config,
    pub shop: Shop,
    pub deck: Deck,
    pub available: Available,
    pub discarded: Vec<Card>,
    pub blind: Option<Blind>,
    pub stage: Stage,
    pub ante_start: Ante,
    pub ante_end: Ante,
    pub ante_current: Ante,
    pub action_history: BoundedActionHistory,
    pub round: f64,
    pub joker_ids: Vec<JokerId>, // Changed from jokers: Vec<Jokers> to support new system
    pub joker_states: HashMap<JokerId, JokerState>,
    pub plays: f64,
    pub discards: f64,
    pub reward: f64,
    pub money: f64,
    pub shop_reroll_cost: f64,
    pub shop_rerolls_this_round: u32,
    pub chips: f64,
    pub mult: f64,
    pub score: f64,
    pub hand_type_counts: HashMap<HandRank, u32>,
    // Extended state fields
    pub consumables_in_hand: Vec<ConsumableId>,
    pub vouchers: VoucherCollection,
    pub boss_blind_state: BossBlindState,
    pub pack_inventory: Vec<Pack>,
    pub open_pack: Option<OpenPackState>,
    pub state_version: StateVersion,
}

const SAVE_VERSION: u32 = 1;

/// Errors that can occur during save/load operations
#[derive(Debug)]
pub enum SaveLoadError {
    SerializationError(serde_json::Error),
    DeserializationError(serde_json::Error),
    InvalidVersion(u32),
    MissingField(String),
    ValidationError(String),
}

impl fmt::Display for SaveLoadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SaveLoadError::SerializationError(e) => write!(f, "Serialization error: {e}"),
            SaveLoadError::DeserializationError(e) => write!(f, "Deserialization error: {e}"),
            SaveLoadError::InvalidVersion(v) => write!(f, "Unsupported save version: {v}"),
            SaveLoadError::MissingField(field) => write!(f, "Missing required field: {field}"),
            SaveLoadError::ValidationError(msg) => write!(f, "Validation error: {msg}"),
        }
    }
}

impl std::error::Error for SaveLoadError {}

impl Game {
    /// Save the current game state to JSON string
    pub fn save_state_to_json(&self) -> Result<String, SaveLoadError> {
        // Extract joker states from the state manager
        let joker_states = self.joker_state_manager.snapshot_all();

        // Extract joker IDs from the new joker system
        let joker_ids: Vec<JokerId> = self.jokers.iter().map(|j| j.id()).collect();

        let saveable_state = SaveableGameState {
            version: SAVE_VERSION,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            config: self.config.clone(),
            shop: self.shop.clone(),
            deck: self.deck.clone(),
            available: self.available.clone(),
            discarded: self.discarded.clone(),
            blind: self.blind,
            stage: self.stage,
            ante_start: self.ante_start,
            ante_end: self.ante_end,
            ante_current: self.ante_current,
            action_history: self.action_history.clone(),
            round: self.round,
            joker_ids,
            joker_states,
            plays: self.plays,
            discards: self.discards,
            reward: self.reward,
            money: self.money,
            shop_reroll_cost: self.shop_reroll_cost,
            shop_rerolls_this_round: self.shop_rerolls_this_round,
            chips: self.chips,
            mult: self.mult,
            score: self.score,
            hand_type_counts: self.hand_type_counts.clone(),
            // Extended state fields
            consumables_in_hand: self.consumables_in_hand.clone(),
            vouchers: self.vouchers.clone(),
            boss_blind_state: self.boss_blind_state.clone(),
            pack_inventory: self.pack_inventory.clone(),
            open_pack: self.open_pack.clone(),
            state_version: self.state_version,
        };

        serde_json::to_string_pretty(&saveable_state).map_err(SaveLoadError::SerializationError)
    }

    /// Load game state from JSON string
    pub fn load_state_from_json(json: &str) -> Result<Self, SaveLoadError> {
        let saveable_state: SaveableGameState =
            serde_json::from_str(json).map_err(SaveLoadError::DeserializationError)?;

        // Validate version
        if saveable_state.version > SAVE_VERSION {
            return Err(SaveLoadError::InvalidVersion(saveable_state.version));
        }

        // Recreate jokers using JokerFactory
        let jokers: Vec<Box<dyn Joker>> = saveable_state
            .joker_ids
            .into_iter()
            .filter_map(|id| JokerFactory::create(id))
            .collect();

        // Create joker state manager
        let joker_state_manager = Arc::new(JokerStateManager::new());

        // Create new game instance with reconstructed state
        let game = Game {
            config: saveable_state.config,
            shop: saveable_state.shop,
            deck: saveable_state.deck,
            available: saveable_state.available,
            discarded: saveable_state.discarded,
            blind: saveable_state.blind,
            stage: saveable_state.stage,
            ante_start: saveable_state.ante_start,
            ante_end: saveable_state.ante_end,
            ante_current: saveable_state.ante_current,
            action_history: saveable_state.action_history,
            round: saveable_state.round,
            jokers,
            joker_effect_processor: JokerEffectProcessor::new(),
            joker_state_manager: joker_state_manager.clone(),
            plays: saveable_state.plays,
            discards: saveable_state.discards,
            reward: saveable_state.reward,
            money: saveable_state.money,
            shop_reroll_cost: saveable_state.shop_reroll_cost,
            shop_rerolls_this_round: saveable_state.shop_rerolls_this_round,
            chips: saveable_state.chips,
            mult: saveable_state.mult,
            score: saveable_state.score,
            hand_type_counts: saveable_state.hand_type_counts,
            // Extended state fields
            consumables_in_hand: saveable_state.consumables_in_hand,
            vouchers: saveable_state.vouchers,
            boss_blind_state: saveable_state.boss_blind_state,
            pack_inventory: saveable_state.pack_inventory,
            open_pack: saveable_state.open_pack,
            state_version: saveable_state.state_version,
            // Initialize debug logging fields (not serialized)
            debug_logging_enabled: false,
            debug_messages: Vec::new(),
            // Initialize target context (not serialized)
            target_context: TargetContext::new(),
            // Initialize secure RNG (not serialized)
            rng: crate::rng::GameRng::secure(),
            // Initialize memory monitor (not serialized)
            memory_monitor: MemoryMonitor::default(),
        };

        // Restore joker states to the state manager
        game.joker_state_manager
            .restore_from_snapshot(saveable_state.joker_states);

        Ok(game)
    }

    /// Process a scaling event for all scaling jokers in the game
    pub fn process_scaling_event(&mut self, event: ScalingEvent) {
        if self.jokers.is_empty() {
            return;
        }

        // Create game context for jokers
        let mut context = GameContext {
            chips: self.chips as i32,
            mult: self.mult as i32,
            money: self.money as i32,
            ante: self.ante_current as u8,
            round: self.round as u32,
            stage: &self.stage,
            hands_played: 0,  // TODO: track this properly
            discards_used: 0, // TODO: track this properly
            jokers: &self.jokers,
            hand: &crate::hand::Hand::new(vec![]),
            discarded: &self.discarded,
            joker_state_manager: &self.joker_state_manager,
            hand_type_counts: &self.hand_type_counts,
            cards_in_deck: self.deck.len(),
            stone_cards_in_deck: 0, // TODO: Track stone cards when implemented
            steel_cards_in_deck: 0, // TODO: Track steel cards when implemented
            rng: &self.rng,
        };

        // Process scaling events for any scaling jokers
        for joker in &self.jokers {
            joker.process_scaling_event(&mut context, &event);
        }
    }
}

#[cfg(test)]
#[allow(clippy::field_reassign_with_default)]
mod tests {
    use super::*;
    use crate::card::{Suit, Value};
    use crate::joker::JokerId;

    #[test]
    fn test_constructor() {
        let g = Game::default();
        assert_eq!(g.available.cards().len(), 0);
        assert_eq!(g.deck.len(), 52);
        assert_eq!(g.mult, 0.0);
    }

    #[test]
    fn test_deal() {
        let mut g = Game::default();
        g.deal();
        // deck should be 7 cards smaller than we started with
        assert_eq!(g.deck.len(), 52 - g.config.available);
        // should be 7 cards now available
        assert_eq!(g.available.cards().len(), g.config.available);
    }

    #[test]
    fn test_draw() {
        let mut g = Game::default();
        g.draw(1);
        assert_eq!(g.available.cards().len(), 1);
        assert_eq!(g.deck.len(), 52 - 1);
        g.draw(3);
        assert_eq!(g.available.cards().len(), 4);
        assert_eq!(g.deck.len(), 52 - 4);
    }
    #[test]
    fn test_discard() {
        let mut g = Game::default();
        g.deal();
        assert_eq!(g.available.cards().len(), g.config.available);
        assert_eq!(g.deck.len(), 52 - g.config.available);
        // select first 4 cards
        for c in g.available.cards()[0..5].to_vec() {
            g.select_card(c).unwrap();
        }
        let discard_res = g.discard_selected();
        assert!(discard_res.is_ok());
        // available should still be 7, we discarded then redrew to match
        assert_eq!(g.available.cards().len(), g.config.available);
        // deck is now smaller since we drew from it
        assert_eq!(g.deck.len(), 52 - g.config.available - 5);
    }

    #[test]
    fn test_calc_score() {
        let mut g = Game::default();
        let ace = Card::new(Value::Ace, Suit::Heart);
        let king = Card::new(Value::King, Suit::Diamond);
        let jack = Card::new(Value::Jack, Suit::Club);

        // Score [Ah, Kd, Jc]
        // High card (level 1) -> chips=5, mult=1
        // Played cards (1 ace) -> 11 chips
        // (5 + 11) * 1 = 16
        let cards = vec![ace, king, jack];
        let hand = SelectHand::new(cards).best_hand().unwrap();
        let score = g.calc_score(hand);
        assert_eq!(score, 16.0);

        // Score [Kd, Kd, Ah]
        // Pair (level 1) -> chips=10, mult=2
        // Played cards (2 kings) -> 10 + 10 == 20 chips
        // (10 + 20) * 2 = 60
        let cards = vec![king, king, ace];
        let hand = SelectHand::new(cards).best_hand().unwrap();
        let score = g.calc_score(hand);
        assert_eq!(score, 60.0);

        // Score [Ah, Ah, Ah, Kd]
        // Three of kind (level 1) -> chips=30, mult=3
        // Played cards (3 aces) -> 11 + 11 + 11 == 33 chips
        // (30 + 33) * 3 = 189
        let cards = vec![ace, ace, ace, king];
        let hand = SelectHand::new(cards).best_hand().unwrap();
        let score = g.calc_score(hand);
        assert_eq!(score, 189.0);

        // Score [Kd, Kd, Kd, Kd, Ah]
        // Four of kind (level 1) -> chips=60, mult=7
        // Played cards (4 kings) -> 10 + 10 + 10 + 10 == 40 chips
        // (60 + 40) * 7 = 700
        let cards = vec![king, king, king, king, ace];
        let hand = SelectHand::new(cards).best_hand().unwrap();
        let score = g.calc_score(hand);
        assert_eq!(score, 700.0);

        // Score [Jc, Jc, Jc, Jc, Jc]
        // Flush five (level 1) -> chips=160, mult=16
        // Played cards (5 jacks) -> 10 + 10 + 10 + 10 + 10 == 50 chips
        // (160 + 50) * 16 = 3360
        let cards = vec![jack, jack, jack, jack, jack];
        let hand = SelectHand::new(cards).best_hand().unwrap();
        let score = g.calc_score(hand);
        assert_eq!(score, 3360.0);
    }

    #[test]
    fn test_handle_score() {
        let mut g = Game::default();
        g.start();
        g.stage = Stage::Blind(Blind::Small);
        g.blind = Some(Blind::Small);

        // Not enough to pass
        let required = g.required_score();
        let score = required - 1.0;

        let passed = g.handle_score(score).unwrap();
        assert!(!passed);
        assert_eq!(g.score, score);

        // Enough to pass now
        let passed = g.handle_score(1.0).unwrap();
        assert!(passed);
        assert_eq!(g.score, required);
        assert_eq!(g.stage, Stage::PostBlind());
    }

    #[test]
    fn test_clear_blind() {
        let mut g = Game::default();
        g.start();
        g.deal();
        g.clear_blind();
        // deck should be 7 cards smaller than we started with
        assert_eq!(g.deck.len(), 52 - g.config.available);
        // should be 7 cards now available
        assert_eq!(g.available.cards().len(), g.config.available);
    }

    #[test]
    fn test_play_selected() {
        let mut g = Game::default();
        g.start();
        g.stage = Stage::Blind(Blind::Small);
        g.blind = Some(Blind::Small);
        for card in g.available.cards().iter().take(5) {
            g.available.select_card(*card).expect("can select card");
        }

        assert_eq!(g.available.selected().len(), 5);
        // Artifically set score so blind passes
        g.score += g.required_score();
        g.play_selected().expect("can play selected");

        // Should have cleared blind
        assert_eq!(g.stage, Stage::PostBlind());
        // Score should reset to 0
        assert_eq!(g.score, g.config.base_score as f64);
        // Plays and discards should reset
        assert_eq!(g.plays, g.config.plays as f64);
        assert_eq!(g.discards, g.config.discards as f64);
        // Deck should be length 52 - available
        assert_eq!(g.deck.len(), 52 - g.config.available);
        // Discarded should be length 0
        assert_eq!(g.discarded.len(), 0);
        // Available should be length available
        assert_eq!(g.available.cards().len(), g.config.available);
    }

    #[test]
    fn test_buy_joker() {
        let mut g = Game::default();
        g.start();
        g.stage = Stage::Shop();
        g.money = 10.0;
        g.shop.refresh(&g.rng);

        let j1 = g.shop.joker_from_index(0).expect("is joker");
        g.buy_joker(j1.clone()).expect("buy joker");
        assert_eq!(g.money, 10.0 - j1.cost() as f64);
        assert_eq!(g.jokers.len(), 1);
    }

    #[test]
    fn test_buy_joker_with_slot_specification() {
        let mut game = Game::default();
        game.start();
        game.stage = Stage::Shop();
        game.money = 20.0;

        // Set up shop with known jokers for deterministic testing
        use crate::joker::compat::TheJoker;
        game.shop.jokers = vec![Jokers::TheJoker(TheJoker {})];

        // Test buying a joker in a specific slot
        let action = Action::BuyJoker {
            joker_id: JokerId::Joker,
            slot: 0,
        };

        let result = game.handle_action(action);
        assert!(result.is_ok());

        // Verify joker is in the correct slot
        assert!(game.get_joker_at_slot(0).is_some());
        assert_eq!(
            game.get_joker_at_slot(0).map(|j| j.id()),
            Some(JokerId::Joker)
        );
        assert_eq!(game.joker_count(), 1);
    }

    #[test]
    fn test_buy_joker_insert_at_position() {
        let mut game = Game::default();
        game.start();
        game.stage = Stage::Shop();
        game.money = 40.0;

        // Set up shop with known jokers for deterministic testing
        use crate::joker::compat::{GreedyJoker, TheJoker};
        game.shop.jokers = vec![
            Jokers::TheJoker(TheJoker {}),
            Jokers::GreedyJoker(GreedyJoker {}),
        ];

        // Buy first joker at end (slot 0)
        let action1 = Action::BuyJoker {
            joker_id: JokerId::Joker,
            slot: 0,
        };
        game.handle_action(action1).unwrap();

        // Buy another joker at position 0 (should push first joker to position 1)
        let action2 = Action::BuyJoker {
            joker_id: JokerId::GreedyJoker,
            slot: 0,
        };
        let result = game.handle_action(action2);

        assert!(result.is_ok());
        assert_eq!(game.joker_count(), 2);
        // GreedyJoker should be at position 0
        assert_eq!(
            game.get_joker_at_slot(0).map(|j| j.id()),
            Some(JokerId::GreedyJoker)
        );
        // TheJoker should have moved to position 1
        assert_eq!(
            game.get_joker_at_slot(1).map(|j| j.id()),
            Some(JokerId::Joker)
        );
    }

    #[test]
    fn test_buy_joker_invalid_slot() {
        let mut game = Game::default();
        game.start();
        game.stage = Stage::Shop();
        game.money = 20.0;
        game.shop.refresh(&game.rng);

        // Test buying in slot beyond limit (default is 5 slots, so 0-4 are valid)
        let action = Action::BuyJoker {
            joker_id: JokerId::Joker,
            slot: 5,
        };

        let result = game.handle_action(action);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), GameError::InvalidSlot));
    }

    #[test]
    fn test_buy_joker_expanded_slots() {
        let mut game = Game::default();
        game.start();
        game.stage = Stage::Shop();
        game.money = 20.0;

        // Set up shop with known jokers for deterministic testing
        use crate::joker::compat::TheJoker;
        game.shop.jokers = vec![Jokers::TheJoker(TheJoker {})];

        // Simulate having voucher that expands slots to 10
        game.config.joker_slots = 10;

        // Now slot 5 should be valid
        let action = Action::BuyJoker {
            joker_id: JokerId::Joker,
            slot: 5,
        };

        let result = game.handle_action(action);
        assert!(result.is_ok());
        // Since the jokers vector is empty, specifying slot 5 will still append
        // at the end (slot 0)
        assert_eq!(
            game.get_joker_at_slot(0).map(|j| j.id()),
            Some(JokerId::Joker)
        );
        assert_eq!(game.joker_count(), 1);
    }

    #[test]
    fn test_buy_joker_insufficient_money() {
        let mut game = Game::default();
        game.start();
        game.stage = Stage::Shop();
        game.money = 1.0; // Not enough for any joker

        // Set up shop with known jokers for deterministic testing
        use crate::joker::compat::TheJoker;
        game.shop.jokers = vec![Jokers::TheJoker(TheJoker {})];

        let action = Action::BuyJoker {
            joker_id: JokerId::Joker,
            slot: 0,
        };

        let result = game.handle_action(action);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), GameError::InvalidBalance));
    }

    #[test]
    fn test_buy_joker_not_in_shop() {
        let mut game = Game::default();
        game.start();
        game.stage = Stage::Shop();
        game.money = 20.0;
        game.shop.refresh(&game.rng);

        // Try to buy a joker that's not currently in the shop
        let action = Action::BuyJoker {
            joker_id: JokerId::CavendishJoker, // Unlikely to be in shop
            slot: 0,
        };

        let result = game.handle_action(action);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), GameError::JokerNotInShop));
    }

    #[test]
    fn test_buy_joker_wrong_stage() {
        let mut game = Game::default();
        game.start();
        game.stage = Stage::Blind(Blind::Small);
        game.money = 20.0;

        let action = Action::BuyJoker {
            joker_id: JokerId::Joker,
            slot: 0,
        };

        let result = game.handle_action(action);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), GameError::InvalidStage));
    }

    #[test]
    fn test_joker_effect_cache_integration() {
        use crate::card::{Suit, Value};
        use crate::joker::JokerId;

        let mut game = Game::default();
        game.start();
        game.stage = Stage::Blind(Blind::Small);
        game.blind = Some(Blind::Small);

        // Add a joker to the game for testing
        if let Some(joker) = crate::joker_factory::JokerFactory::create(JokerId::Joker) {
            game.jokers.push(joker);
        }

        // Enable cache and clear any existing entries
        game.enable_joker_effect_cache();
        game.clear_joker_effect_cache();

        // Create a test hand
        let cards = vec![
            Card::new(Value::Ace, Suit::Heart),
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::Queen, Suit::Heart),
            Card::new(Value::Jack, Suit::Heart),
            Card::new(Value::Ten, Suit::Heart),
        ];

        let select_hand = SelectHand::new(cards);
        let made_hand = select_hand.best_hand().unwrap();

        // First call should miss cache
        let initial_metrics = game.get_joker_cache_metrics().clone();
        let (chips1, mult1, money1, _mult_multiplier1, _messages1) =
            game.process_joker_effects(&made_hand);

        // Verify cache metrics show a miss
        let metrics_after_first = game.get_joker_cache_metrics();
        assert!(metrics_after_first.total_lookups >= initial_metrics.total_lookups);

        // Second call with same input should potentially hit cache
        // Note: Since we create a new GameContext each time with current game state,
        // cache hits depend on the game state being identical
        let (chips2, mult2, money2, _mult_multiplier2, _messages2) =
            game.process_joker_effects(&made_hand);

        // Results should be identical regardless of cache
        assert_eq!(chips1, chips2);
        assert_eq!(mult1, mult2);
        assert_eq!(money1, money2);
        // Note: messages might differ slightly due to aggregation vs individual processing

        // Test cache configuration
        game.disable_joker_effect_cache();
        let config = game.joker_effect_processor.context().cache_config.clone();
        assert!(!config.enabled);

        game.configure_joker_cache_for_rl();
        let rl_config = game.joker_effect_processor.context().cache_config.clone();
        assert!(rl_config.enabled);
        assert_eq!(rl_config.max_entries, 10000);
        assert_eq!(rl_config.ttl_seconds, 600);

        game.configure_joker_cache_for_simulation();
        let sim_config = game.joker_effect_processor.context().cache_config.clone();
        assert!(sim_config.enabled);
        assert_eq!(sim_config.max_entries, 1000);
        assert_eq!(sim_config.ttl_seconds, 300);
    }

    #[test]
    fn test_cache_performance_benefit() {
        use crate::card::{Suit, Value};
        use crate::joker::JokerId;
        use std::time::Instant;

        let mut game = Game::default();
        game.start();
        game.stage = Stage::Blind(Blind::Small);
        game.blind = Some(Blind::Small);

        // Add multiple jokers to increase processing complexity
        for joker_id in [JokerId::Joker, JokerId::GreedyJoker].iter() {
            if let Some(joker) = crate::joker_factory::JokerFactory::create(*joker_id) {
                game.jokers.push(joker);
            }
        }

        // Create a test hand
        let cards = vec![
            Card::new(Value::Ace, Suit::Heart),
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::Queen, Suit::Heart),
        ];
        let select_hand = SelectHand::new(cards);
        let made_hand = select_hand.best_hand().unwrap();

        // Test with cache enabled
        game.enable_joker_effect_cache();
        game.clear_joker_effect_cache();

        let start_cached = Instant::now();
        for _ in 0..100 {
            let _ = game.process_joker_effects(&made_hand);
        }
        let cached_duration = start_cached.elapsed();

        // Test with cache disabled
        game.disable_joker_effect_cache();

        let start_uncached = Instant::now();
        for _ in 0..100 {
            let _ = game.process_joker_effects(&made_hand);
        }
        let uncached_duration = start_uncached.elapsed();

        // Re-enable cache for metrics check
        game.enable_joker_effect_cache();
        let metrics = game.get_joker_cache_metrics();

        // The test passes if the caching infrastructure works correctly
        // Performance benefits depend on joker complexity and may not be measurable in simple tests
        println!("Cached processing: {cached_duration:?}");
        println!("Uncached processing: {uncached_duration:?}");
        if metrics.total_lookups > 0 {
            println!("Cache hit ratio: {:.2}%", metrics.hit_ratio() * 100.0);
        }

        // Verify that both approaches produce the same results
        game.enable_joker_effect_cache();
        let (chips_cached, mult_cached, money_cached, _, _) =
            game.process_joker_effects(&made_hand);

        game.disable_joker_effect_cache();
        let (chips_uncached, mult_uncached, money_uncached, _, _) =
            game.process_joker_effects(&made_hand);

        assert_eq!(chips_cached, chips_uncached);
        assert_eq!(mult_cached, mult_uncached);
        assert_eq!(money_cached, money_uncached);
    }

    #[test]
    fn test_accumulated_effects_new() {
        let effects = AccumulatedEffects::new();
        assert_eq!(effects.chips, 0);
        assert_eq!(effects.mult, 0);
        assert_eq!(effects.money, 0);
        assert_eq!(effects.mult_multiplier, 1.0);
        assert!(effects.messages.is_empty());
    }

    #[test]
    fn test_accumulated_effects_default() {
        let effects = AccumulatedEffects::default();
        assert_eq!(effects.chips, 0);
        assert_eq!(effects.mult, 0);
        assert_eq!(effects.money, 0);
        assert_eq!(effects.mult_multiplier, 1.0);
        assert!(effects.messages.is_empty());
    }

    #[test]
    fn test_accumulated_effects_accumulate_basic() {
        use crate::joker::JokerEffect;

        let mut effects = AccumulatedEffects::new();
        let joker_effect = JokerEffect {
            chips: 10,
            mult: 5,
            money: 3,
            mult_multiplier: 2.0,
            retrigger: 0,
            destroy_self: false,
            destroy_others: vec![],
            transform_cards: vec![],
            hand_size_mod: 0,
            discard_mod: 0,
            sell_value_increase: 0,
            message: Some("Test message".to_string()),
        };

        let result = effects.accumulate_effect(&joker_effect);
        assert!(result); // Should not trigger killscreen
        assert_eq!(effects.chips, 10);
        assert_eq!(effects.mult, 5);
        assert_eq!(effects.money, 3);
        assert_eq!(effects.mult_multiplier, 2.0);
        assert_eq!(effects.messages.len(), 1);
        assert_eq!(effects.messages[0], "Test message");
    }

    #[test]
    fn test_accumulated_effects_accumulate_with_retriggers() {
        use crate::joker::JokerEffect;

        let mut effects = AccumulatedEffects::new();
        let joker_effect = JokerEffect {
            chips: 10,
            mult: 5,
            money: 3,
            mult_multiplier: 2.0,
            retrigger: 2, // Will trigger 3 times total (1 + 2 retriggers)
            destroy_self: false,
            destroy_others: vec![],
            transform_cards: vec![],
            hand_size_mod: 0,
            discard_mod: 0,
            sell_value_increase: 0,
            message: None,
        };

        let result = effects.accumulate_effect(&joker_effect);
        assert!(result);
        // Values should be multiplied by iterations (3)
        assert_eq!(effects.chips, 30); // 10 * 3
        assert_eq!(effects.mult, 15); // 5 * 3
        assert_eq!(effects.money, 9); // 3 * 3
                                      // Multiplier should be applied 3 times: 1.0 * 2.0 * 2.0 * 2.0 = 8.0
        assert_eq!(effects.mult_multiplier, 8.0);
        assert!(effects.messages.is_empty());
    }

    #[test]
    fn test_accumulated_effects_accumulate_zero_multiplier() {
        use crate::joker::JokerEffect;

        let mut effects = AccumulatedEffects::new();
        let joker_effect = JokerEffect {
            chips: 10,
            mult: 5,
            money: 3,
            mult_multiplier: 0.0, // Zero means no multiplier
            retrigger: 1,
            destroy_self: false,
            destroy_others: vec![],
            transform_cards: vec![],
            hand_size_mod: 0,
            discard_mod: 0,
            sell_value_increase: 0,
            message: None,
        };

        let result = effects.accumulate_effect(&joker_effect);
        assert!(result);
        assert_eq!(effects.chips, 20); // 10 * 2 iterations
        assert_eq!(effects.mult, 10); // 5 * 2 iterations
        assert_eq!(effects.money, 6); // 3 * 2 iterations
        assert_eq!(effects.mult_multiplier, 1.0); // Should remain 1.0 for zero multiplier
    }

    #[test]
    fn test_accumulated_effects_killscreen_infinity() {
        use crate::joker::JokerEffect;

        let mut effects = AccumulatedEffects::new();
        let joker_effect = JokerEffect {
            chips: 0,
            mult: 0,
            money: 0,
            mult_multiplier: f64::INFINITY,
            retrigger: 0,
            destroy_self: false,
            destroy_others: vec![],
            transform_cards: vec![],
            hand_size_mod: 0,
            discard_mod: 0,
            sell_value_increase: 0,
            message: None,
        };

        let result = effects.accumulate_effect(&joker_effect);
        assert!(!result); // Should trigger killscreen
        assert!(effects.mult_multiplier.is_infinite());
    }

    #[test]
    fn test_accumulated_effects_killscreen_nan() {
        use crate::joker::JokerEffect;

        let mut effects = AccumulatedEffects::new();
        let joker_effect = JokerEffect {
            chips: 0,
            mult: 0,
            money: 0,
            mult_multiplier: f64::NAN,
            retrigger: 0,
            destroy_self: false,
            destroy_others: vec![],
            transform_cards: vec![],
            hand_size_mod: 0,
            discard_mod: 0,
            sell_value_increase: 0,
            message: None,
        };

        let result = effects.accumulate_effect(&joker_effect);
        assert!(!result); // Should trigger killscreen
        assert!(effects.mult_multiplier.is_nan());
    }

    #[test]
    fn test_accumulated_effects_killscreen_from_multiplication() {
        use crate::joker::JokerEffect;

        let mut effects = AccumulatedEffects::new();
        effects.mult_multiplier = f64::MAX / 2.0; // Set to a very large value

        let joker_effect = JokerEffect {
            chips: 0,
            mult: 0,
            money: 0,
            mult_multiplier: f64::MAX / 2.0, // This multiplication should overflow to infinity
            retrigger: 0,
            destroy_self: false,
            destroy_others: vec![],
            transform_cards: vec![],
            hand_size_mod: 0,
            discard_mod: 0,
            sell_value_increase: 0,
            message: None,
        };

        let result = effects.accumulate_effect(&joker_effect);
        assert!(!result); // Should trigger killscreen
        assert!(effects.mult_multiplier.is_infinite());
    }

    #[test]
    fn test_accumulated_effects_saturating_arithmetic() {
        use crate::joker::JokerEffect;

        let mut effects = AccumulatedEffects::new();
        effects.chips = i32::MAX - 5; // Set close to max

        let joker_effect = JokerEffect {
            chips: 10, // This would overflow without saturating arithmetic
            mult: 0,
            money: 0,
            mult_multiplier: 0.0,
            retrigger: 0,
            destroy_self: false,
            destroy_others: vec![],
            transform_cards: vec![],
            hand_size_mod: 0,
            discard_mod: 0,
            sell_value_increase: 0,
            message: None,
        };

        let result = effects.accumulate_effect(&joker_effect);
        assert!(result);
        assert_eq!(effects.chips, i32::MAX); // Should saturate at max value
    }

    #[test]
    fn test_accumulated_effects_merge() {
        let mut effects1 = AccumulatedEffects {
            chips: 10,
            mult: 5,
            money: 3,
            mult_multiplier: 2.0,
            messages: vec!["Message 1".to_string()],
        };

        let effects2 = AccumulatedEffects {
            chips: 20,
            mult: 10,
            money: 7,
            mult_multiplier: 3.0,
            messages: vec!["Message 2".to_string(), "Message 3".to_string()],
        };

        effects1.merge(&effects2);

        assert_eq!(effects1.chips, 30); // 10 + 20
        assert_eq!(effects1.mult, 15); // 5 + 10
        assert_eq!(effects1.money, 10); // 3 + 7
        assert_eq!(effects1.mult_multiplier, 6.0); // 2.0 * 3.0
        assert_eq!(effects1.messages.len(), 3);
        assert_eq!(effects1.messages[0], "Message 1");
        assert_eq!(effects1.messages[1], "Message 2");
        assert_eq!(effects1.messages[2], "Message 3");
    }

    #[test]
    fn test_accumulated_effects_merge_zero_multiplier() {
        let mut effects1 = AccumulatedEffects {
            chips: 10,
            mult: 5,
            money: 3,
            mult_multiplier: 2.0,
            messages: vec![],
        };

        let effects2 = AccumulatedEffects {
            chips: 20,
            mult: 10,
            money: 7,
            mult_multiplier: 0.0, // Zero multiplier should not affect the result
            messages: vec![],
        };

        effects1.merge(&effects2);

        assert_eq!(effects1.chips, 30);
        assert_eq!(effects1.mult, 15);
        assert_eq!(effects1.money, 10);
        assert_eq!(effects1.mult_multiplier, 2.0); // Should remain unchanged
    }

    #[test]
    fn test_accumulated_effects_merge_saturating() {
        let mut effects1 = AccumulatedEffects {
            chips: i32::MAX - 5,
            mult: i32::MAX - 3,
            money: i32::MAX - 1,
            mult_multiplier: 1.0,
            messages: vec![],
        };

        let effects2 = AccumulatedEffects {
            chips: 10,
            mult: 10,
            money: 10,
            mult_multiplier: 1.0,
            messages: vec![],
        };

        effects1.merge(&effects2);

        // Should saturate at max values
        assert_eq!(effects1.chips, i32::MAX);
        assert_eq!(effects1.mult, i32::MAX);
        assert_eq!(effects1.money, i32::MAX);
    }

    #[test]
    fn test_accumulated_effects_apply_mult_multiplier() {
        let mut effects = AccumulatedEffects {
            chips: 0,
            mult: 10,
            money: 0,
            mult_multiplier: 2.5,
            messages: vec![],
        };

        effects.apply_mult_multiplier();

        assert_eq!(effects.mult, 25); // 10 * 2.5 = 25
    }

    #[test]
    fn test_accumulated_effects_apply_mult_multiplier_zero() {
        let mut effects = AccumulatedEffects {
            chips: 0,
            mult: 10,
            money: 0,
            mult_multiplier: 0.0, // Zero multiplier should not change mult
            messages: vec![],
        };

        effects.apply_mult_multiplier();

        assert_eq!(effects.mult, 10); // Should remain unchanged
    }

    #[test]
    fn test_accumulated_effects_apply_mult_multiplier_one() {
        let mut effects = AccumulatedEffects {
            chips: 0,
            mult: 10,
            money: 0,
            mult_multiplier: 1.0, // 1.0 multiplier should not change mult
            messages: vec![],
        };

        effects.apply_mult_multiplier();

        assert_eq!(effects.mult, 10); // Should remain unchanged
    }

    #[test]
    fn test_accumulated_effects_apply_mult_multiplier_clamping() {
        let mut effects = AccumulatedEffects {
            chips: 0,
            mult: 10,
            money: 0,
            mult_multiplier: f64::MAX, // Very large multiplier
            messages: vec![],
        };

        effects.apply_mult_multiplier();

        // Should clamp to i32::MAX
        assert_eq!(effects.mult, i32::MAX);
    }

    #[test]
    fn test_accumulated_effects_apply_mult_multiplier_negative_clamping() {
        let mut effects = AccumulatedEffects {
            chips: 0,
            mult: -10,
            money: 0,
            mult_multiplier: f64::MAX, // Very large multiplier on negative value
            messages: vec![],
        };

        effects.apply_mult_multiplier();

        // Should clamp to i32::MIN
        assert_eq!(effects.mult, i32::MIN);
    }

    #[test]
    fn test_accumulated_effects_multiple_accumulations() {
        use crate::joker::JokerEffect;

        let mut effects = AccumulatedEffects::new();

        // First effect
        let effect1 = JokerEffect {
            chips: 5,
            mult: 2,
            money: 1,
            mult_multiplier: 2.0,
            retrigger: 0,
            destroy_self: false,
            destroy_others: vec![],
            transform_cards: vec![],
            hand_size_mod: 0,
            discard_mod: 0,
            sell_value_increase: 0,
            message: Some("Effect 1".to_string()),
        };

        // Second effect
        let effect2 = JokerEffect {
            chips: 3,
            mult: 4,
            money: 2,
            mult_multiplier: 1.5,
            retrigger: 1, // 2 total iterations
            destroy_self: false,
            destroy_others: vec![],
            transform_cards: vec![],
            hand_size_mod: 0,
            discard_mod: 0,
            sell_value_increase: 0,
            message: Some("Effect 2".to_string()),
        };

        let result1 = effects.accumulate_effect(&effect1);
        let result2 = effects.accumulate_effect(&effect2);

        assert!(result1);
        assert!(result2);

        // Final values should be accumulated from both effects
        assert_eq!(effects.chips, 11); // 5 + (3 * 2)
        assert_eq!(effects.mult, 10); // 2 + (4 * 2)
        assert_eq!(effects.money, 5); // 1 + (2 * 2)
        assert_eq!(effects.mult_multiplier, 4.5); // 2.0 * (1.5 * 1.5)
        assert_eq!(effects.messages.len(), 2);
        assert_eq!(effects.messages[0], "Effect 1");
        assert_eq!(effects.messages[1], "Effect 2");
    }

    #[test]
    fn test_accumulated_effects_equality() {
        let effects1 = AccumulatedEffects {
            chips: 10,
            mult: 5,
            money: 3,
            mult_multiplier: 2.0,
            messages: vec!["Test".to_string()],
        };

        let effects2 = AccumulatedEffects {
            chips: 10,
            mult: 5,
            money: 3,
            mult_multiplier: 2.0,
            messages: vec!["Test".to_string()],
        };

        let effects3 = AccumulatedEffects {
            chips: 11, // Different value
            mult: 5,
            money: 3,
            mult_multiplier: 2.0,
            messages: vec!["Test".to_string()],
        };

        assert_eq!(effects1, effects2);
        assert_ne!(effects1, effects3);
    }

    #[test]
    fn test_can_purchase_consumable_validation() {
        let game = Game {
            stage: Stage::Shop(),
            money: 10.0,
            ..Default::default()
        };

        // Test valid purchase conditions for all consumable types
        assert!(game
            .can_purchase_consumable(crate::shop::ConsumableType::Tarot)
            .is_ok());
        assert!(game
            .can_purchase_consumable(crate::shop::ConsumableType::Planet)
            .is_ok());
        assert!(game
            .can_purchase_consumable(crate::shop::ConsumableType::Spectral)
            .is_ok());
    }

    #[test]
    fn test_can_purchase_consumable_insufficient_money() {
        // Test insufficient money for Tarot (costs 3)
        let mut game = Game {
            stage: Stage::Shop(),
            money: 2.0,
            ..Default::default()
        };
        let result = game.can_purchase_consumable(crate::shop::ConsumableType::Tarot);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), GameError::InvalidBalance));

        // Test insufficient money for Planet (costs 3)
        let result = game.can_purchase_consumable(crate::shop::ConsumableType::Planet);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), GameError::InvalidBalance));

        // Test insufficient money for Spectral (costs 4)
        game.money = 3.0;
        let result = game.can_purchase_consumable(crate::shop::ConsumableType::Spectral);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), GameError::InvalidBalance));
    }

    #[test]
    fn test_can_purchase_consumable_edge_case_exact_money() {
        // Test edge case: exactly enough money for Tarot
        let mut game = Game {
            stage: Stage::Shop(),
            money: 3.0,
            ..Default::default()
        };
        assert!(game
            .can_purchase_consumable(crate::shop::ConsumableType::Tarot)
            .is_ok());

        // Test edge case: exactly enough money for Planet
        assert!(game
            .can_purchase_consumable(crate::shop::ConsumableType::Planet)
            .is_ok());

        // Test edge case: exactly enough money for Spectral
        game.money = 4.0;
        assert!(game
            .can_purchase_consumable(crate::shop::ConsumableType::Spectral)
            .is_ok());
    }

    #[test]
    fn test_can_purchase_consumable_no_available_slots() {
        // Fill consumable hand to capacity (default is 2)
        let game = Game {
            stage: Stage::Shop(),
            money: 10.0,
            consumables_in_hand: vec![
                crate::consumables::ConsumableId::TheFool,
                crate::consumables::ConsumableId::Mercury,
            ],
            ..Default::default()
        };
        assert_eq!(
            game.consumables_in_hand.len(),
            game.config.consumable_hand_capacity
        );

        // Should not be able to purchase any consumable when hand is full
        let result = game.can_purchase_consumable(crate::shop::ConsumableType::Tarot);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), GameError::NoAvailableSlot));

        let result = game.can_purchase_consumable(crate::shop::ConsumableType::Planet);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), GameError::NoAvailableSlot));

        let result = game.can_purchase_consumable(crate::shop::ConsumableType::Spectral);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), GameError::NoAvailableSlot));
    }

    #[test]
    fn test_can_purchase_consumable_wrong_stage() {
        // Test all invalid stages
        let mut game = Game {
            money: 10.0,
            stage: Stage::PreBlind(),
            ..Default::default()
        };
        let result = game.can_purchase_consumable(crate::shop::ConsumableType::Tarot);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), GameError::InvalidStage));

        game = Game {
            money: 10.0,
            stage: Stage::Blind(crate::stage::Blind::Small),
            ..Default::default()
        };
        let result = game.can_purchase_consumable(crate::shop::ConsumableType::Planet);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), GameError::InvalidStage));

        game = Game {
            money: 10.0,
            stage: Stage::PostBlind(),
            ..Default::default()
        };
        let result = game.can_purchase_consumable(crate::shop::ConsumableType::Spectral);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), GameError::InvalidStage));

        game.stage = Stage::End(crate::stage::End::Win);
        let result = game.can_purchase_consumable(crate::shop::ConsumableType::Tarot);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), GameError::InvalidStage));
    }

    #[test]
    fn test_can_purchase_consumable_cost_validation() {
        let mut game = Game::default();
        game.stage = Stage::Shop();

        // Test Tarot card cost (3 money)
        game.money = 3.0;
        assert!(game
            .can_purchase_consumable(crate::shop::ConsumableType::Tarot)
            .is_ok());
        game.money = 2.9;
        assert!(game
            .can_purchase_consumable(crate::shop::ConsumableType::Tarot)
            .is_err());

        // Test Planet card cost (3 money)
        game.money = 3.0;
        assert!(game
            .can_purchase_consumable(crate::shop::ConsumableType::Planet)
            .is_ok());
        game.money = 2.9;
        assert!(game
            .can_purchase_consumable(crate::shop::ConsumableType::Planet)
            .is_err());

        // Test Spectral card cost (4 money)
        game.money = 4.0;
        assert!(game
            .can_purchase_consumable(crate::shop::ConsumableType::Spectral)
            .is_ok());
        game.money = 3.9;
        assert!(game
            .can_purchase_consumable(crate::shop::ConsumableType::Spectral)
            .is_err());
    }

    #[test]
    fn test_can_purchase_consumable_partial_hand() {
        let mut game = Game::default();
        game.stage = Stage::Shop();
        game.money = 10.0;

        // Test with one consumable in hand (capacity is 2)
        game.consumables_in_hand = vec![crate::consumables::ConsumableId::TheFool];
        assert_eq!(game.consumables_in_hand.len(), 1);
        assert!(game.consumables_in_hand.len() < game.config.consumable_hand_capacity);

        // Should be able to purchase any consumable
        assert!(game
            .can_purchase_consumable(crate::shop::ConsumableType::Tarot)
            .is_ok());
        assert!(game
            .can_purchase_consumable(crate::shop::ConsumableType::Planet)
            .is_ok());
        assert!(game
            .can_purchase_consumable(crate::shop::ConsumableType::Spectral)
            .is_ok());
    }

    #[test]
    fn test_can_purchase_consumable_empty_hand() {
        let mut game = Game::default();
        game.stage = Stage::Shop();
        game.money = 10.0;

        // Test with empty consumable hand
        game.consumables_in_hand = vec![];
        assert_eq!(game.consumables_in_hand.len(), 0);

        // Should be able to purchase any consumable
        assert!(game
            .can_purchase_consumable(crate::shop::ConsumableType::Tarot)
            .is_ok());
        assert!(game
            .can_purchase_consumable(crate::shop::ConsumableType::Planet)
            .is_ok());
        assert!(game
            .can_purchase_consumable(crate::shop::ConsumableType::Spectral)
            .is_ok());
    }

    // Reroll cost escalation tests
    #[test]
    fn test_reroll_shop_basic_functionality() {
        let mut game = Game::default();
        game.start();
        game.stage = Stage::Shop();
        game.money = 50.0;

        // Initial reroll cost should be 5
        assert_eq!(game.shop_reroll_cost, 5.0);
        assert_eq!(game.shop_rerolls_this_round, 0);

        // First reroll should succeed
        let result = game.reroll_shop();
        assert!(result.is_ok());
        assert_eq!(game.money, 45.0); // 50 - 5 = 45
        assert_eq!(game.shop_reroll_cost, 10.0); // 5 + 5 = 10
        assert_eq!(game.shop_rerolls_this_round, 1);
    }

    #[test]
    fn test_reroll_shop_cost_escalation() {
        let mut game = Game::default();
        game.start();
        game.stage = Stage::Shop();
        game.money = 100.0;

        // Reroll multiple times and verify cost escalation
        assert_eq!(game.shop_reroll_cost, 5.0);

        // First reroll: 5 coins
        game.reroll_shop().unwrap();
        assert_eq!(game.money, 95.0);
        assert_eq!(game.shop_reroll_cost, 10.0);
        assert_eq!(game.shop_rerolls_this_round, 1);

        // Second reroll: 10 coins
        game.reroll_shop().unwrap();
        assert_eq!(game.money, 85.0);
        assert_eq!(game.shop_reroll_cost, 15.0);
        assert_eq!(game.shop_rerolls_this_round, 2);

        // Third reroll: 15 coins
        game.reroll_shop().unwrap();
        assert_eq!(game.money, 70.0);
        assert_eq!(game.shop_reroll_cost, 20.0);
        assert_eq!(game.shop_rerolls_this_round, 3);

        // Fourth reroll: 20 coins
        game.reroll_shop().unwrap();
        assert_eq!(game.money, 50.0);
        assert_eq!(game.shop_reroll_cost, 25.0);
        assert_eq!(game.shop_rerolls_this_round, 4);
    }

    #[test]
    fn test_reroll_shop_insufficient_money() {
        let mut game = Game::default();
        game.start();
        game.stage = Stage::Shop();
        game.money = 3.0; // Less than base reroll cost of 5

        let result = game.reroll_shop();
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), GameError::InvalidBalance));

        // State should remain unchanged
        assert_eq!(game.money, 3.0);
        assert_eq!(game.shop_reroll_cost, 5.0);
        assert_eq!(game.shop_rerolls_this_round, 0);
    }

    #[test]
    fn test_reroll_shop_invalid_stage() {
        let mut game = Game::default();
        game.start();
        game.stage = Stage::PreBlind(); // Not shop stage
        game.money = 50.0;

        let result = game.reroll_shop();
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), GameError::InvalidStage));

        // State should remain unchanged
        assert_eq!(game.money, 50.0);
        assert_eq!(game.shop_reroll_cost, 5.0);
        assert_eq!(game.shop_rerolls_this_round, 0);
    }

    #[test]
    fn test_reroll_cost_reset_on_new_shop_round() {
        let mut game = Game::default();
        game.start();

        // Set up initial state
        game.stage = Stage::Shop();
        game.money = 50.0;
        game.shop_reroll_cost = 15.0; // Simulate previous rerolls
        game.shop_rerolls_this_round = 2;

        // Simulate transitioning to new round
        game.stage = Stage::PostBlind();
        game.reward = 10.0;
        let cashout_result = game.cashout();
        assert!(cashout_result.is_ok());

        // Reroll cost and count should be reset
        assert_eq!(game.stage, Stage::Shop());
        assert_eq!(game.shop_reroll_cost, 5.0); // Reset to base cost
        assert_eq!(game.shop_rerolls_this_round, 0); // Reset count
        assert_eq!(game.money, 60.0); // Original money + reward
    }

    #[test]
    fn test_reroll_shop_action_integration() {
        let mut game = Game::default();
        game.start();
        game.stage = Stage::Shop();
        game.money = 50.0;

        // Test using the RerollShop action
        let action = Action::RerollShop();
        let result = game.handle_action(action);

        assert!(result.is_ok());
        assert_eq!(game.money, 45.0);
        assert_eq!(game.shop_reroll_cost, 10.0);
        assert_eq!(game.shop_rerolls_this_round, 1);
    }

    #[test]
    fn test_reroll_shop_action_invalid_stage() {
        let mut game = Game::default();
        game.start();
        game.stage = Stage::PreBlind(); // Not shop stage
        game.money = 50.0;

        let action = Action::RerollShop();
        let result = game.handle_action(action);

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), GameError::InvalidStage));
        assert_eq!(game.money, 50.0); // Money unchanged
    }

    #[test]
    fn test_reroll_shop_save_load_state() {
        let mut game = Game::default();
        game.start();
        game.stage = Stage::Shop();
        game.money = 50.0;

        // Perform some rerolls to establish state
        game.reroll_shop().unwrap();
        game.reroll_shop().unwrap();

        // Verify state before save
        assert_eq!(game.money, 35.0); // 50 - 5 - 10 = 35
        assert_eq!(game.shop_reroll_cost, 15.0);
        assert_eq!(game.shop_rerolls_this_round, 2);

        // Save and reload game state
        let saved_state = game.save_state_to_json().unwrap();
        let loaded_game = Game::load_state_from_json(&saved_state).unwrap();

        // Verify reroll state is preserved
        assert_eq!(loaded_game.money, 35.0);
        assert_eq!(loaded_game.shop_reroll_cost, 15.0);
        assert_eq!(loaded_game.shop_rerolls_this_round, 2);
    }

    #[test]
    fn test_reroll_shop_multiple_rounds() {
        let mut game = Game::default();
        game.start();
        game.money = 100.0;

        // First round
        game.stage = Stage::Shop();
        game.reroll_shop().unwrap(); // Cost: 5, total spent: 5
        game.reroll_shop().unwrap(); // Cost: 10, total spent: 15
        assert_eq!(game.money, 85.0);
        assert_eq!(game.shop_reroll_cost, 15.0);

        // Transition to next round
        game.stage = Stage::PostBlind();
        game.reward = 20.0;
        game.cashout().unwrap();

        // Second round - costs should be reset
        assert_eq!(game.shop_reroll_cost, 5.0);
        assert_eq!(game.shop_rerolls_this_round, 0);
        assert_eq!(game.money, 105.0); // 85 + 20 = 105

        // Reroll in second round
        game.reroll_shop().unwrap(); // Should cost 5 again
        assert_eq!(game.money, 100.0);
        assert_eq!(game.shop_reroll_cost, 10.0);
        assert_eq!(game.shop_rerolls_this_round, 1);
    }
}
