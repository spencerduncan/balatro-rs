//! Game persistence and serialization functionality
//!
//! Handles save/load operations and state serialization for the Balatro game engine.
//! Extracted from game.rs to follow Single Responsibility Principle.

use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::ante::Ante;
use crate::available::Available;
use crate::boss_blinds::BossBlindState;
use crate::bounded_action_history::BoundedActionHistory;
use crate::card::Card;
use crate::config::Config;
use crate::consumables::ConsumableId;
use crate::deck::Deck;
use crate::joker::{Joker, JokerId};
use crate::joker_factory::JokerFactory;
use crate::joker_state::{JokerState, JokerStateManager};
use crate::rank::HandRank;
use crate::shop::packs::{OpenPackState, Pack};
use crate::shop::Shop;
use crate::stage::{Blind, Stage};
use crate::state_version::StateVersion;
use crate::vouchers::VoucherCollection;

// Additional imports needed for load functionality
use super::{DebugManager, PackManager};
use crate::joker_effect_processor::JokerEffectProcessor;
use crate::target_context::TargetContext;

/// Current save format version
const SAVE_VERSION: u32 = 1;

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
    // Card enhancement tracking
    pub stone_cards_in_deck: usize,
    pub steel_cards_in_deck: usize,
    // Extended state fields
    pub consumables_in_hand: Vec<ConsumableId>,
    pub vouchers: VoucherCollection,
    pub boss_blind_state: BossBlindState,
    pub pack_inventory: Vec<Pack>,
    pub open_pack: Option<OpenPackState>,
    pub state_version: StateVersion,
}

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

/// Manages game state persistence operations
///
/// Following Single Responsibility Principle - handles only save/load concerns.
/// Extracted from Game struct to improve code organization and maintainability.
#[derive(Debug)]
pub struct PersistenceManager;

impl PersistenceManager {
    /// Create a new persistence manager
    pub fn new() -> Self {
        Self
    }

    /// Save the current game state to JSON string
    ///
    /// Extracts serializable state from the game and converts it to JSON.
    /// Non-serializable fields like RNG and debug info are excluded.
    pub fn save_state_to_json(&self, game: &super::Game) -> Result<String, SaveLoadError> {
        // Extract joker states from the state manager
        let joker_states = game.joker_state_manager.snapshot_all();

        // Extract joker IDs from the new joker system
        let joker_ids: Vec<JokerId> = game.jokers.iter().map(|j| j.id()).collect();

        let saveable_state = SaveableGameState {
            version: SAVE_VERSION,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            config: game.config.clone(),
            shop: game.shop.clone(),
            deck: game.deck.clone(),
            available: game.available.clone(),
            discarded: game.discarded.clone(),
            blind: game.blind,
            stage: game.stage,
            ante_start: game.ante_start,
            ante_end: game.ante_end,
            ante_current: game.ante_current,
            action_history: game.action_history.clone(),
            round: game.round,
            joker_ids,
            joker_states,
            plays: game.plays,
            discards: game.discards,
            reward: game.reward,
            money: game.money,
            shop_reroll_cost: game.shop_reroll_cost,
            shop_rerolls_this_round: game.shop_rerolls_this_round,
            chips: game.chips,
            mult: game.mult,
            score: game.score,
            hand_type_counts: game.hand_type_counts.clone(),
            // Card enhancement tracking
            stone_cards_in_deck: game.stone_cards_in_deck,
            steel_cards_in_deck: game.steel_cards_in_deck,
            // Extended state fields
            consumables_in_hand: game.consumables_in_hand.clone(),
            vouchers: game.vouchers.clone(),
            boss_blind_state: game.boss_blind_state.clone(),
            pack_inventory: game.pack_manager.pack_inventory().clone(),
            open_pack: game.pack_manager.open_pack_state().clone(),
            state_version: game.state_version,
        };

        serde_json::to_string_pretty(&saveable_state).map_err(SaveLoadError::SerializationError)
    }

    /// Load game state from JSON string
    ///
    /// Deserializes JSON data and reconstructs a complete Game instance.
    /// Handles version validation and joker recreation.
    pub fn load_state_from_json(&self, json: &str) -> Result<super::Game, SaveLoadError> {
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
        let game = super::Game {
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
            hand_levels: std::collections::HashMap::new(), // Initialize with empty levels (default level 1)
            // Card enhancement tracking
            stone_cards_in_deck: saveable_state.stone_cards_in_deck,
            steel_cards_in_deck: saveable_state.steel_cards_in_deck,
            // Extended state fields
            consumables_in_hand: saveable_state.consumables_in_hand,
            vouchers: saveable_state.vouchers,
            boss_blind_state: saveable_state.boss_blind_state,
            state_version: saveable_state.state_version,
            // Initialize pack manager with loaded state
            pack_manager: {
                let mut pm = PackManager::new();
                for pack in saveable_state.pack_inventory {
                    pm.add_pack(pack);
                }
                pm
            },
            // Initialize debug manager (not serialized)
            debug_manager: DebugManager::new(),
            // Initialize target context (not serialized)
            target_context: TargetContext::new(),
            // Initialize secure RNG (not serialized)
            rng: crate::rng::GameRng::secure(),
            // Initialize skip tags system (not serialized)
            available_skip_tags: Vec::new(),
            active_skip_tags: crate::skip_tags::ActiveSkipTags::new(),
            pending_tag_selection: false,
            // Initialize persistence manager
            persistence_manager: PersistenceManager::new(),
        };

        // Restore joker states to the state manager
        game.joker_state_manager
            .restore_from_snapshot(saveable_state.joker_states);

        // Refresh enhancement counts based on loaded deck
        let mut game = game;
        game.refresh_enhancement_counts();

        Ok(game)
    }
}

impl Default for PersistenceManager {
    fn default() -> Self {
        Self::new()
    }
}
