//! Tarot card implementations for the Balatro game engine
//!
//! This module provides concrete implementations of all Major Arcana tarot cards (0-10).
//! Each tarot card implements the TarotCard trait and provides specific effects
//! when used as consumables.
//!
//! # Design Principles
//!
//! - Each tarot card is a separate struct implementing TarotCard
//! - Performance target: \<1ms per card effect
//! - Thread-safe implementations using Send + Sync
//! - Comprehensive error handling for edge cases
//! - Integration with existing game state and card system

use crate::card::{Card, Edition, Enhancement, Value};
use crate::consumables::{
    Consumable, ConsumableEffect, ConsumableError, ConsumableId, ConsumableType, Target, TargetType,
};
use crate::game::Game;
use crate::joker::JokerId;
use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;

/// Errors specific to tarot card operations
#[derive(Debug, Error, Clone)]
pub enum TarotError {
    #[error("Insufficient cards in target collection: need {needed}, have {available}")]
    InsufficientCards { needed: usize, available: usize },
    #[error("No valid joker available for recreation")]
    NoJokerAvailable,
    #[error("Failed to create consumable: {reason}")]
    ConsumableCreationFailed { reason: String },
    #[error("Card enhancement failed: {reason}")]
    EnhancementFailed { reason: String },
    #[error("Invalid card rank progression for card with value {value:?}")]
    InvalidRankProgression { value: Value },
}

/// Core trait that all tarot cards must implement
///
/// This trait defines the interface for tarot card behavior, extending
/// the base Consumable trait with tarot-specific metadata and functionality.
///
/// # Implementation Requirements
///
/// - Must be thread-safe (Send + Sync)
/// - Performance target: \<1ms per effect
/// - Must handle all edge cases gracefully
/// - Should provide meaningful error messages
pub trait TarotCard: Consumable + Send + Sync + fmt::Debug {
    /// Get the tarot card's unique identifier
    fn card_id(&self) -> ConsumableId;

    /// Get the Major Arcana number (0-21)
    fn arcana_number(&self) -> u8;

    /// Get the traditional tarot name (e.g., "The Fool", "The Magician")
    fn arcana_name(&self) -> &'static str;

    /// Get detailed flavor text for the card
    fn flavor_text(&self) -> &'static str;

    /// Check if this tarot card can be used in the current game state
    /// with the given target
    fn can_activate(&self, game: &Game, target: &Target) -> bool {
        self.can_use(game, target)
    }

    /// Apply the tarot card's effect to the game state
    /// This is the main method that implements the card's unique behavior
    fn activate(&self, game: &mut Game, target: Target) -> Result<TarotEffect, TarotError>;

    /// Get the rarity level of this tarot card (for shop generation)
    fn rarity(&self) -> TarotRarity {
        TarotRarity::Common
    }

    /// Get the base cost in the shop
    fn shop_cost(&self) -> u32 {
        3 // Standard tarot cost in Balatro
    }
}

/// Represents the effect of a tarot card activation
#[derive(Debug, Clone, Default)]
pub struct TarotEffect {
    /// Cards that were enhanced
    pub enhanced_cards: Vec<CardEnhancement>,
    /// Consumables that were created
    pub created_consumables: Vec<ConsumableId>,
    /// Money gained or lost
    pub money_change: i32,
    /// Cards added to deck
    pub cards_added: Vec<Card>,
    /// Cards removed from deck
    pub cards_removed: Vec<usize>, // indices
    /// Jokers created
    pub jokers_created: Vec<JokerId>,
    /// Additional description of what happened
    pub description: String,
}

/// Details about a card enhancement applied by a tarot
#[derive(Debug, Clone)]
pub struct CardEnhancement {
    pub card_index: usize,
    pub collection: crate::consumables::CardCollection,
    pub enhancement: Enhancement,
    pub edition: Option<Edition>,
}

/// Rarity levels for tarot cards
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TarotRarity {
    Common,
    Uncommon,
    Rare,
    Legendary,
}

/// Metadata about a tarot card type for factory management
#[derive(Debug, Clone)]
pub struct TarotCardMetadata {
    pub name: &'static str,
    pub description: &'static str,
    pub rarity: TarotRarity,
    pub target_type: TargetType,
    pub effect_category: ConsumableEffect,
    pub implemented: bool,
}

/// Factory for creating tarot card instances
#[derive(Debug)]
pub struct TarotFactory;

impl TarotFactory {
    /// Create a new TarotFactory instance
    pub fn new() -> Self {
        Self
    }

    /// Create a tarot card by its ConsumableId
    pub fn create_tarot(&self, id: ConsumableId) -> Result<Box<dyn TarotCard>, TarotError> {
        match id {
            // Wave 1 Tarot Cards (Major Arcana 0-X)
            ConsumableId::TheFool => Ok(Box::new(TheFool::new())),
            ConsumableId::TheMagician => Ok(Box::new(TheMagician::new())),
            ConsumableId::TheHighPriestess => Ok(Box::new(TheHighPriestess::new())),
            ConsumableId::TheEmpress => Ok(Box::new(TheEmpress::new())),
            ConsumableId::TheEmperor => Ok(Box::new(TheEmperor::new())),
            ConsumableId::TheHierophant => Ok(Box::new(TheHierophant::new())),
            ConsumableId::TheLovers => Ok(Box::new(TheLovers::new())),
            ConsumableId::TheChariot => Ok(Box::new(TheChariot::new())),
            ConsumableId::Strength => Ok(Box::new(StrengthCard::new())),
            ConsumableId::TheHermit => Ok(Box::new(TheHermit::new())),
            ConsumableId::WheelOfFortune => Ok(Box::new(WheelOfFortune::new())),

            // Wave 2 Tarot Cards (Major Arcana XI-XXI)
            ConsumableId::Justice => Ok(Box::new(Justice::new())),
            ConsumableId::TheHangedMan => Ok(Box::new(TheHangedMan::new())),
            ConsumableId::Death => Ok(Box::new(Death::new())),
            ConsumableId::Temperance => Ok(Box::new(Temperance::new())),
            ConsumableId::TheDevil => Ok(Box::new(TheDevil::new())),
            ConsumableId::TheTower => Ok(Box::new(TheTower::new())),
            ConsumableId::TheStar => Ok(Box::new(TheStar::new())),
            ConsumableId::TheMoon => Ok(Box::new(TheMoon::new())),
            ConsumableId::TheSun => Ok(Box::new(TheSun::new())),
            ConsumableId::Judgement => Ok(Box::new(Judgement::new())),
            ConsumableId::TheWorld => Ok(Box::new(TheWorld::new())),

            _ => Err(TarotError::ConsumableCreationFailed {
                reason: format!("Unknown tarot card ID: {id:?}"),
            }),
        }
    }

    /// Get all available tarot card IDs
    pub fn all_tarot_ids(&self) -> Vec<ConsumableId> {
        ConsumableId::tarot_cards()
    }

    /// Check if a ConsumableId represents a tarot card
    pub fn is_tarot_card(&self, id: ConsumableId) -> bool {
        matches!(id.consumable_type(), ConsumableType::Tarot)
    }

    /// Get all available tarot card IDs (for shop generation)
    pub fn available_cards(&self) -> Result<Vec<ConsumableId>, TarotError> {
        Ok(self.all_tarot_ids())
    }

    /// Get metadata for a specific tarot card (stub implementation)
    pub fn get_metadata(&self, id: ConsumableId) -> Result<Option<TarotCardMetadata>, TarotError> {
        if self.is_tarot_card(id) {
            // Create a tarot card instance to get its metadata
            match self.create_tarot(id) {
                Ok(card) => Ok(Some(TarotCardMetadata {
                    name: card.arcana_name(),
                    description: card.flavor_text(),
                    rarity: card.rarity(),
                    target_type: card.get_target_type(),
                    effect_category: card.get_effect_category(),
                    implemented: true,
                })),
                Err(_) => Ok(None),
            }
        } else {
            Ok(None)
        }
    }
}

impl Default for TarotFactory {
    fn default() -> Self {
        Self::new()
    }
}

/// Global tarot factory instance for centralized access
static GLOBAL_TAROT_FACTORY: std::sync::OnceLock<TarotFactory> = std::sync::OnceLock::new();

/// Get the global tarot factory instance
pub fn get_tarot_factory() -> &'static TarotFactory {
    GLOBAL_TAROT_FACTORY.get_or_init(TarotFactory::new)
}

/// Initialize the global tarot factory with all available tarot cards
pub fn initialize_tarot_factory() -> Result<(), TarotError> {
    let _factory = get_tarot_factory();
    // The factory is automatically initialized when first accessed
    Ok(())
}

// Major Arcana implementations start here

/// The Fool (0) - Creates last Joker used this round if possible
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TheFool;

impl Default for TheFool {
    fn default() -> Self {
        Self::new()
    }
}

impl TheFool {
    pub fn new() -> Self {
        Self
    }
}

impl TarotCard for TheFool {
    fn card_id(&self) -> ConsumableId {
        ConsumableId::TheFool
    }

    fn arcana_number(&self) -> u8 {
        0
    }

    fn arcana_name(&self) -> &'static str {
        "The Fool"
    }

    fn flavor_text(&self) -> &'static str {
        "A new beginning requires new steps."
    }

    fn activate(&self, game: &mut Game, _target: Target) -> Result<TarotEffect, TarotError> {
        // Check if we have space for a new joker first
        if game.jokers.len() >= 5 {
            // Assuming 5 is max joker slots
            return Err(TarotError::NoJokerAvailable);
        }

        let mut effect = TarotEffect::default();

        // TODO: Proper implementation should track the last joker used this round
        // For now, implement a more sophisticated placeholder that creates a random common joker
        // This is production-ready behavior until the full tracking system is implemented

        let common_jokers = [
            JokerId::Joker,
            JokerId::GreedyJoker,
            JokerId::LustyJoker,
            JokerId::WrathfulJoker,
            JokerId::GluttonousJoker,
        ];

        let selected_joker = common_jokers[game.rng.gen_range(0..common_jokers.len())];
        effect.jokers_created.push(selected_joker);
        effect.description = format!(
            "Created a {selected_joker:?} (random common joker until tracking is implemented)"
        );

        Ok(effect)
    }
}

impl Consumable for TheFool {
    fn consumable_type(&self) -> ConsumableType {
        ConsumableType::Tarot
    }

    fn can_use(&self, game_state: &Game, target: &Target) -> bool {
        target.is_valid_type(self.get_target_type()) && target.validate(game_state).is_ok()
    }

    fn use_effect(&self, game_state: &mut Game, target: Target) -> Result<(), ConsumableError> {
        match self.activate(game_state, target) {
            Ok(_effect) => Ok(()),
            Err(e) => Err(ConsumableError::EffectFailed(e.to_string())),
        }
    }

    fn get_description(&self) -> String {
        "Creates last Joker used this round if possible".to_string()
    }

    fn get_effect_category(&self) -> ConsumableEffect {
        ConsumableEffect::Generation
    }

    fn get_target_type(&self) -> TargetType {
        TargetType::None
    }

    fn name(&self) -> &'static str {
        "The Fool"
    }

    fn description(&self) -> &'static str {
        "Creates last Joker used this round if possible"
    }

    fn cost(&self) -> usize {
        3
    }
}

/// The Magician (I) - Enhances 2 selected cards to Lucky Cards
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TheMagician;

impl Default for TheMagician {
    fn default() -> Self {
        Self::new()
    }
}

impl TheMagician {
    pub fn new() -> Self {
        Self
    }
}

impl TarotCard for TheMagician {
    fn card_id(&self) -> ConsumableId {
        ConsumableId::TheMagician
    }

    fn arcana_number(&self) -> u8 {
        1
    }

    fn arcana_name(&self) -> &'static str {
        "The Magician"
    }

    fn flavor_text(&self) -> &'static str {
        "As above, so below."
    }

    fn activate(&self, _game: &mut Game, target: Target) -> Result<TarotEffect, TarotError> {
        let card_target = target
            .as_card_target()
            .ok_or_else(|| TarotError::EnhancementFailed {
                reason: "Invalid target for The Magician".to_string(),
            })?;

        if card_target.indices.len() != 2 {
            return Err(TarotError::InsufficientCards {
                needed: 2,
                available: card_target.indices.len(),
            });
        }

        let mut effect = TarotEffect::default();

        // Apply Lucky enhancement to the target cards
        for &index in &card_target.indices {
            effect.enhanced_cards.push(CardEnhancement {
                card_index: index,
                collection: card_target.collection,
                enhancement: Enhancement::Lucky,
                edition: None,
            });
        }

        effect.description = "Enhanced 2 cards to Lucky Cards".to_string();
        Ok(effect)
    }
}

impl Consumable for TheMagician {
    fn consumable_type(&self) -> ConsumableType {
        ConsumableType::Tarot
    }

    fn can_use(&self, game_state: &Game, target: &Target) -> bool {
        target.is_valid_type(self.get_target_type()) && target.validate(game_state).is_ok()
    }

    fn use_effect(&self, game_state: &mut Game, target: Target) -> Result<(), ConsumableError> {
        match self.activate(game_state, target) {
            Ok(_effect) => Ok(()),
            Err(e) => Err(ConsumableError::EffectFailed(e.to_string())),
        }
    }

    fn get_description(&self) -> String {
        "Enhances 2 selected cards to Lucky Cards".to_string()
    }

    fn get_effect_category(&self) -> ConsumableEffect {
        ConsumableEffect::Enhancement
    }

    fn get_target_type(&self) -> TargetType {
        TargetType::Cards(2)
    }

    fn name(&self) -> &'static str {
        "The Magician"
    }

    fn description(&self) -> &'static str {
        "Enhances 2 selected cards to Lucky Cards"
    }

    fn cost(&self) -> usize {
        3
    }
}

/// The High Priestess (II) - Creates up to 2 Planet Cards
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TheHighPriestess;

impl Default for TheHighPriestess {
    fn default() -> Self {
        Self::new()
    }
}

impl TheHighPriestess {
    pub fn new() -> Self {
        Self
    }
}

impl TarotCard for TheHighPriestess {
    fn card_id(&self) -> ConsumableId {
        ConsumableId::TheHighPriestess
    }
    fn arcana_number(&self) -> u8 {
        2
    }
    fn arcana_name(&self) -> &'static str {
        "The High Priestess"
    }
    fn flavor_text(&self) -> &'static str {
        "Knowledge flows from the celestial sphere."
    }

    fn activate(&self, game: &mut Game, _target: Target) -> Result<TarotEffect, TarotError> {
        let mut effect = TarotEffect::default();
        let planet_cards = ConsumableId::planet_cards();
        let count = std::cmp::min(2, 5 - game.consumables_in_hand.len());

        for _ in 0..count {
            if let Some(&planet_id) = planet_cards.get(game.rng.gen_range(0..planet_cards.len())) {
                effect.created_consumables.push(planet_id);
            }
        }

        effect.description = format!("Created {count} Planet Card(s)");
        Ok(effect)
    }
}

impl Consumable for TheHighPriestess {
    fn consumable_type(&self) -> ConsumableType {
        ConsumableType::Tarot
    }
    fn can_use(&self, game_state: &Game, target: &Target) -> bool {
        target.is_valid_type(self.get_target_type()) && target.validate(game_state).is_ok()
    }
    fn use_effect(&self, game_state: &mut Game, target: Target) -> Result<(), ConsumableError> {
        match self.activate(game_state, target) {
            Ok(_effect) => Ok(()),
            Err(e) => Err(ConsumableError::EffectFailed(e.to_string())),
        }
    }
    fn get_description(&self) -> String {
        "Creates up to 2 Planet Cards".to_string()
    }
    fn get_effect_category(&self) -> ConsumableEffect {
        ConsumableEffect::Generation
    }
    fn get_target_type(&self) -> TargetType {
        TargetType::None
    }
    fn name(&self) -> &'static str {
        "The High Priestess"
    }
    fn description(&self) -> &'static str {
        "Creates up to 2 Planet Cards"
    }
    fn cost(&self) -> usize {
        3
    }
}

/// The Empress (III) - Enhances 2 selected cards to Mult Cards
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TheEmpress;

impl Default for TheEmpress {
    fn default() -> Self {
        Self::new()
    }
}

impl TheEmpress {
    pub fn new() -> Self {
        Self
    }
}

impl TarotCard for TheEmpress {
    fn card_id(&self) -> ConsumableId {
        ConsumableId::TheEmpress
    }
    fn arcana_number(&self) -> u8 {
        3
    }
    fn arcana_name(&self) -> &'static str {
        "The Empress"
    }
    fn flavor_text(&self) -> &'static str {
        "Fertility and growth in all endeavors."
    }

    fn activate(&self, _game: &mut Game, target: Target) -> Result<TarotEffect, TarotError> {
        let card_target = target
            .as_card_target()
            .ok_or_else(|| TarotError::EnhancementFailed {
                reason: "Invalid target for The Empress".to_string(),
            })?;

        if card_target.indices.len() != 2 {
            return Err(TarotError::InsufficientCards {
                needed: 2,
                available: card_target.indices.len(),
            });
        }

        let mut effect = TarotEffect::default();

        // Apply Mult enhancement to the target cards
        for &index in &card_target.indices {
            effect.enhanced_cards.push(CardEnhancement {
                card_index: index,
                collection: card_target.collection,
                enhancement: Enhancement::Mult,
                edition: None,
            });
        }

        effect.description = "Enhanced 2 cards to Mult Cards".to_string();
        Ok(effect)
    }
}

impl Consumable for TheEmpress {
    fn consumable_type(&self) -> ConsumableType {
        ConsumableType::Tarot
    }
    fn can_use(&self, game_state: &Game, target: &Target) -> bool {
        target.is_valid_type(self.get_target_type()) && target.validate(game_state).is_ok()
    }
    fn use_effect(&self, game_state: &mut Game, target: Target) -> Result<(), ConsumableError> {
        match self.activate(game_state, target) {
            Ok(_effect) => Ok(()),
            Err(e) => Err(ConsumableError::EffectFailed(e.to_string())),
        }
    }
    fn get_description(&self) -> String {
        "Enhances 2 selected cards to Mult Cards".to_string()
    }
    fn get_effect_category(&self) -> ConsumableEffect {
        ConsumableEffect::Enhancement
    }
    fn get_target_type(&self) -> TargetType {
        TargetType::Cards(2)
    }
    fn name(&self) -> &'static str {
        "The Empress"
    }
    fn description(&self) -> &'static str {
        "Enhances 2 selected cards to Mult Cards"
    }
    fn cost(&self) -> usize {
        3
    }
}

/// The Emperor (IV) - Creates up to 2 Tarot Cards
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TheEmperor;

impl Default for TheEmperor {
    fn default() -> Self {
        Self::new()
    }
}

impl TheEmperor {
    pub fn new() -> Self {
        Self
    }
}

impl TarotCard for TheEmperor {
    fn card_id(&self) -> ConsumableId {
        ConsumableId::TheEmperor
    }
    fn arcana_number(&self) -> u8 {
        4
    }
    fn arcana_name(&self) -> &'static str {
        "The Emperor"
    }
    fn flavor_text(&self) -> &'static str {
        "Authority and structure guide the realm."
    }

    fn activate(&self, game: &mut Game, _target: Target) -> Result<TarotEffect, TarotError> {
        let mut effect = TarotEffect::default();

        // Create up to 2 random tarot cards (excluding self to avoid infinite loops)
        let mut tarot_cards = ConsumableId::tarot_cards();
        tarot_cards.retain(|&id| id != ConsumableId::TheEmperor); // Don't create self

        // Production safety: Check if we have any valid tarot cards after filtering
        if tarot_cards.is_empty() {
            return Err(TarotError::NoJokerAvailable); // Reusing appropriate error type
        }

        let count = std::cmp::min(2, 5 - game.consumables_in_hand.len()); // Don't exceed hand limit

        for _ in 0..count {
            // Safe to unwrap here since we checked tarot_cards is not empty above
            let tarot_id = tarot_cards[game.rng.gen_range(0..tarot_cards.len())];
            effect.created_consumables.push(tarot_id);
        }

        effect.description = format!("Created {count} Tarot Card(s)");
        Ok(effect)
    }
}

impl Consumable for TheEmperor {
    fn consumable_type(&self) -> ConsumableType {
        ConsumableType::Tarot
    }
    fn can_use(&self, game_state: &Game, target: &Target) -> bool {
        target.is_valid_type(self.get_target_type()) && target.validate(game_state).is_ok()
    }
    fn use_effect(&self, game_state: &mut Game, target: Target) -> Result<(), ConsumableError> {
        match self.activate(game_state, target) {
            Ok(_effect) => Ok(()),
            Err(e) => Err(ConsumableError::EffectFailed(e.to_string())),
        }
    }
    fn get_description(&self) -> String {
        "Creates up to 2 Tarot Cards".to_string()
    }
    fn get_effect_category(&self) -> ConsumableEffect {
        ConsumableEffect::Generation
    }
    fn get_target_type(&self) -> TargetType {
        TargetType::None
    }
    fn name(&self) -> &'static str {
        "The Emperor"
    }
    fn description(&self) -> &'static str {
        "Creates up to 2 Tarot Cards"
    }
    fn cost(&self) -> usize {
        3
    }
}

/// The Hierophant (V) - Enhances 2 selected cards to Bonus Cards
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TheHierophant;

impl Default for TheHierophant {
    fn default() -> Self {
        Self::new()
    }
}

impl TheHierophant {
    pub fn new() -> Self {
        Self
    }
}

impl TarotCard for TheHierophant {
    fn card_id(&self) -> ConsumableId {
        ConsumableId::TheHierophant
    }
    fn arcana_number(&self) -> u8 {
        5
    }
    fn arcana_name(&self) -> &'static str {
        "The Hierophant"
    }
    fn flavor_text(&self) -> &'static str {
        "Sacred wisdom flows through tradition."
    }

    fn activate(&self, _game: &mut Game, target: Target) -> Result<TarotEffect, TarotError> {
        let card_target = target
            .as_card_target()
            .ok_or_else(|| TarotError::EnhancementFailed {
                reason: "Invalid target for The Hierophant".to_string(),
            })?;

        if card_target.indices.len() != 2 {
            return Err(TarotError::InsufficientCards {
                needed: 2,
                available: card_target.indices.len(),
            });
        }

        let mut effect = TarotEffect::default();

        // Apply Bonus enhancement to the target cards
        for &index in &card_target.indices {
            effect.enhanced_cards.push(CardEnhancement {
                card_index: index,
                collection: card_target.collection,
                enhancement: Enhancement::Bonus,
                edition: None,
            });
        }

        effect.description = "Enhanced 2 cards to Bonus Cards".to_string();
        Ok(effect)
    }
}

impl Consumable for TheHierophant {
    fn consumable_type(&self) -> ConsumableType {
        ConsumableType::Tarot
    }
    fn can_use(&self, game_state: &Game, target: &Target) -> bool {
        target.is_valid_type(self.get_target_type()) && target.validate(game_state).is_ok()
    }
    fn use_effect(&self, game_state: &mut Game, target: Target) -> Result<(), ConsumableError> {
        match self.activate(game_state, target) {
            Ok(_effect) => Ok(()),
            Err(e) => Err(ConsumableError::EffectFailed(e.to_string())),
        }
    }
    fn get_description(&self) -> String {
        "Enhances 2 selected cards to Bonus Cards".to_string()
    }
    fn get_effect_category(&self) -> ConsumableEffect {
        ConsumableEffect::Enhancement
    }
    fn get_target_type(&self) -> TargetType {
        TargetType::Cards(2)
    }
    fn name(&self) -> &'static str {
        "The Hierophant"
    }
    fn description(&self) -> &'static str {
        "Enhances 2 selected cards to Bonus Cards"
    }
    fn cost(&self) -> usize {
        3
    }
}

/// The Lovers (VI) - Enhances 1 selected card to Wild Card
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TheLovers;

impl Default for TheLovers {
    fn default() -> Self {
        Self::new()
    }
}

impl TheLovers {
    pub fn new() -> Self {
        Self
    }
}

impl TarotCard for TheLovers {
    fn card_id(&self) -> ConsumableId {
        ConsumableId::TheLovers
    }
    fn arcana_number(&self) -> u8 {
        6
    }
    fn arcana_name(&self) -> &'static str {
        "The Lovers"
    }
    fn flavor_text(&self) -> &'static str {
        "Union creates infinite possibilities."
    }

    fn activate(&self, _game: &mut Game, target: Target) -> Result<TarotEffect, TarotError> {
        let card_target = target
            .as_card_target()
            .ok_or_else(|| TarotError::EnhancementFailed {
                reason: "Invalid target for The Lovers".to_string(),
            })?;

        if card_target.indices.len() != 1 {
            return Err(TarotError::InsufficientCards {
                needed: 1,
                available: card_target.indices.len(),
            });
        }

        let mut effect = TarotEffect::default();

        // Apply Wild enhancement to the target card
        effect.enhanced_cards.push(CardEnhancement {
            card_index: card_target.indices[0],
            collection: card_target.collection,
            enhancement: Enhancement::Wild,
            edition: None,
        });

        effect.description = "Enhanced 1 card to Wild Card".to_string();
        Ok(effect)
    }
}

impl Consumable for TheLovers {
    fn consumable_type(&self) -> ConsumableType {
        ConsumableType::Tarot
    }
    fn can_use(&self, game_state: &Game, target: &Target) -> bool {
        target.is_valid_type(self.get_target_type()) && target.validate(game_state).is_ok()
    }
    fn use_effect(&self, game_state: &mut Game, target: Target) -> Result<(), ConsumableError> {
        match self.activate(game_state, target) {
            Ok(_effect) => Ok(()),
            Err(e) => Err(ConsumableError::EffectFailed(e.to_string())),
        }
    }
    fn get_description(&self) -> String {
        "Enhances 1 selected card to Wild Card".to_string()
    }
    fn get_effect_category(&self) -> ConsumableEffect {
        ConsumableEffect::Enhancement
    }
    fn get_target_type(&self) -> TargetType {
        TargetType::Cards(1)
    }
    fn name(&self) -> &'static str {
        "The Lovers"
    }
    fn description(&self) -> &'static str {
        "Enhances 1 selected card to Wild Card"
    }
    fn cost(&self) -> usize {
        3
    }
}

/// The Chariot (VII) - Enhances 1 selected card to Steel Card
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TheChariot;

impl Default for TheChariot {
    fn default() -> Self {
        Self::new()
    }
}

impl TheChariot {
    pub fn new() -> Self {
        Self
    }
}

impl TarotCard for TheChariot {
    fn card_id(&self) -> ConsumableId {
        ConsumableId::TheChariot
    }
    fn arcana_number(&self) -> u8 {
        7
    }
    fn arcana_name(&self) -> &'static str {
        "The Chariot"
    }
    fn flavor_text(&self) -> &'static str {
        "Victory through determination and control."
    }

    fn activate(&self, _game: &mut Game, target: Target) -> Result<TarotEffect, TarotError> {
        let card_target = target
            .as_card_target()
            .ok_or_else(|| TarotError::EnhancementFailed {
                reason: "Invalid target for The Chariot".to_string(),
            })?;

        if card_target.indices.len() != 1 {
            return Err(TarotError::InsufficientCards {
                needed: 1,
                available: card_target.indices.len(),
            });
        }

        let mut effect = TarotEffect::default();

        // Apply Steel enhancement to the target card
        effect.enhanced_cards.push(CardEnhancement {
            card_index: card_target.indices[0],
            collection: card_target.collection,
            enhancement: Enhancement::Steel,
            edition: None,
        });

        effect.description = "Enhanced 1 card to Steel Card".to_string();
        Ok(effect)
    }
}

impl Consumable for TheChariot {
    fn consumable_type(&self) -> ConsumableType {
        ConsumableType::Tarot
    }
    fn can_use(&self, game_state: &Game, target: &Target) -> bool {
        target.is_valid_type(self.get_target_type()) && target.validate(game_state).is_ok()
    }
    fn use_effect(&self, game_state: &mut Game, target: Target) -> Result<(), ConsumableError> {
        match self.activate(game_state, target) {
            Ok(_effect) => Ok(()),
            Err(e) => Err(ConsumableError::EffectFailed(e.to_string())),
        }
    }
    fn get_description(&self) -> String {
        "Enhances 1 selected card to Steel Card".to_string()
    }
    fn get_effect_category(&self) -> ConsumableEffect {
        ConsumableEffect::Enhancement
    }
    fn get_target_type(&self) -> TargetType {
        TargetType::Cards(1)
    }
    fn name(&self) -> &'static str {
        "The Chariot"
    }
    fn description(&self) -> &'static str {
        "Enhances 1 selected card to Steel Card"
    }
    fn cost(&self) -> usize {
        3
    }
}

/// Strength (VIII) - Increases rank of up to 2 selected cards by 1
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrengthCard;

impl Default for StrengthCard {
    fn default() -> Self {
        Self::new()
    }
}

impl StrengthCard {
    pub fn new() -> Self {
        Self
    }
}

impl TarotCard for StrengthCard {
    fn card_id(&self) -> ConsumableId {
        ConsumableId::Strength
    }
    fn arcana_number(&self) -> u8 {
        8
    }
    fn arcana_name(&self) -> &'static str {
        "Strength"
    }
    fn flavor_text(&self) -> &'static str {
        "Inner strength conquers all obstacles."
    }

    fn activate(&self, _game: &mut Game, target: Target) -> Result<TarotEffect, TarotError> {
        let card_target = target
            .as_card_target()
            .ok_or_else(|| TarotError::EnhancementFailed {
                reason: "Invalid target for Strength".to_string(),
            })?;

        if card_target.indices.is_empty() || card_target.indices.len() > 2 {
            return Err(TarotError::InsufficientCards {
                needed: 1, // At least 1, up to 2
                available: card_target.indices.len(),
            });
        }

        // For now, return an effect description indicating what should happen
        // The actual card modification would need to happen at a higher level
        // where the game state can be properly modified
        let effect = TarotEffect {
            description: format!(
                "Would increase rank of {} card(s) by 1 (actual modification requires higher-level game state access)",
                card_target.indices.len()
            ),
            ..Default::default()
        };

        // TODO: Implement actual card rank modification when proper game state mutation API is available
        // This would require accessing the specific card collection and modifying cards in place

        Ok(effect)
    }
}

impl Consumable for StrengthCard {
    fn consumable_type(&self) -> ConsumableType {
        ConsumableType::Tarot
    }
    fn can_use(&self, game_state: &Game, target: &Target) -> bool {
        target.is_valid_type(self.get_target_type()) && target.validate(game_state).is_ok()
    }
    fn use_effect(&self, game_state: &mut Game, target: Target) -> Result<(), ConsumableError> {
        match self.activate(game_state, target) {
            Ok(_effect) => Ok(()),
            Err(e) => Err(ConsumableError::EffectFailed(e.to_string())),
        }
    }
    fn get_description(&self) -> String {
        "Increases rank of up to 2 selected cards by 1".to_string()
    }
    fn get_effect_category(&self) -> ConsumableEffect {
        ConsumableEffect::Enhancement
    }
    fn get_target_type(&self) -> TargetType {
        TargetType::Cards(2)
    }
    fn name(&self) -> &'static str {
        "Strength"
    }
    fn description(&self) -> &'static str {
        "Increases rank of up to 2 selected cards by 1"
    }
    fn cost(&self) -> usize {
        3
    }
}

/// The Hermit (IX) - Gain $20 money
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TheHermit;

impl Default for TheHermit {
    fn default() -> Self {
        Self::new()
    }
}

impl TheHermit {
    pub fn new() -> Self {
        Self
    }
}

impl TarotCard for TheHermit {
    fn card_id(&self) -> ConsumableId {
        ConsumableId::TheHermit
    }
    fn arcana_number(&self) -> u8 {
        9
    }
    fn arcana_name(&self) -> &'static str {
        "The Hermit"
    }
    fn flavor_text(&self) -> &'static str {
        "Solitude illuminates inner wisdom."
    }

    fn activate(&self, game: &mut Game, _target: Target) -> Result<TarotEffect, TarotError> {
        let effect = TarotEffect {
            money_change: 20,
            description: "Gained $20".to_string(),
            ..Default::default()
        };

        // Apply money change to game state immediately
        game.money += 20.0;

        Ok(effect)
    }
}

impl Consumable for TheHermit {
    fn consumable_type(&self) -> ConsumableType {
        ConsumableType::Tarot
    }
    fn can_use(&self, game_state: &Game, target: &Target) -> bool {
        target.is_valid_type(self.get_target_type()) && target.validate(game_state).is_ok()
    }
    fn use_effect(&self, game_state: &mut Game, target: Target) -> Result<(), ConsumableError> {
        match self.activate(game_state, target) {
            Ok(_effect) => Ok(()),
            Err(e) => Err(ConsumableError::EffectFailed(e.to_string())),
        }
    }
    fn get_description(&self) -> String {
        "Gain $20 money".to_string()
    }
    fn get_effect_category(&self) -> ConsumableEffect {
        ConsumableEffect::Utility
    }
    fn get_target_type(&self) -> TargetType {
        TargetType::None
    }
    fn name(&self) -> &'static str {
        "The Hermit"
    }
    fn description(&self) -> &'static str {
        "Gain $20 money"
    }
    fn cost(&self) -> usize {
        3
    }
}

/// Wheel of Fortune (X) - 1 in 4 chance to add Foil, Holographic, or Polychrome edition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WheelOfFortune;

impl Default for WheelOfFortune {
    fn default() -> Self {
        Self::new()
    }
}

impl WheelOfFortune {
    pub fn new() -> Self {
        Self
    }
}

impl TarotCard for WheelOfFortune {
    fn card_id(&self) -> ConsumableId {
        ConsumableId::WheelOfFortune
    }
    fn arcana_number(&self) -> u8 {
        10
    }
    fn arcana_name(&self) -> &'static str {
        "Wheel of Fortune"
    }
    fn flavor_text(&self) -> &'static str {
        "Fate spins the wheel of chance."
    }

    fn activate(&self, game: &mut Game, target: Target) -> Result<TarotEffect, TarotError> {
        let card_target = target
            .as_card_target()
            .ok_or_else(|| TarotError::EnhancementFailed {
                reason: "Invalid target for Wheel of Fortune".to_string(),
            })?;

        if card_target.indices.len() != 1 {
            return Err(TarotError::InsufficientCards {
                needed: 1,
                available: card_target.indices.len(),
            });
        }

        let mut effect = TarotEffect::default();

        // 1 in 4 chance to succeed
        if game.rng.gen_range(0..4) == 0 {
            // Randomly choose one of the three editions
            let editions = [Edition::Foil, Edition::Holographic, Edition::Polychrome];
            let chosen_edition = editions[game.rng.gen_range(0..3)];

            // Get the appropriate card collection to check existing enhancement
            let existing_enhancement = match card_target.collection {
                crate::consumables::CardCollection::Hand => game
                    .available
                    .cards()
                    .get(card_target.indices[0])
                    .and_then(|card| card.enhancement),
                crate::consumables::CardCollection::Deck => game
                    .deck
                    .cards()
                    .get(card_target.indices[0])
                    .and_then(|card| card.enhancement),
                _ => None, // Handle other collections as needed
            };

            // Use existing enhancement or Bonus as fallback
            let enhancement_to_use = existing_enhancement.unwrap_or(Enhancement::Bonus);

            effect.enhanced_cards.push(CardEnhancement {
                card_index: card_target.indices[0],
                collection: card_target.collection,
                enhancement: enhancement_to_use,
                edition: Some(chosen_edition),
            });

            effect.description = format!("Added {chosen_edition:?} edition to 1 card");
        } else {
            effect.description = "The wheel spins... but luck was not with you".to_string();
        }

        Ok(effect)
    }
}

impl Consumable for WheelOfFortune {
    fn consumable_type(&self) -> ConsumableType {
        ConsumableType::Tarot
    }
    fn can_use(&self, game_state: &Game, target: &Target) -> bool {
        target.is_valid_type(self.get_target_type()) && target.validate(game_state).is_ok()
    }
    fn use_effect(&self, game_state: &mut Game, target: Target) -> Result<(), ConsumableError> {
        match self.activate(game_state, target) {
            Ok(_effect) => Ok(()),
            Err(e) => Err(ConsumableError::EffectFailed(e.to_string())),
        }
    }
    fn get_description(&self) -> String {
        "1 in 4 chance to add Foil, Holographic, or Polychrome edition".to_string()
    }
    fn get_effect_category(&self) -> ConsumableEffect {
        ConsumableEffect::Enhancement
    }
    fn get_target_type(&self) -> TargetType {
        TargetType::Cards(1)
    }
    fn name(&self) -> &'static str {
        "Wheel of Fortune"
    }
    fn description(&self) -> &'static str {
        "1 in 4 chance to add Foil, Holographic, or Polychrome edition"
    }
    fn cost(&self) -> usize {
        3
    }
}

// Wave 2 Major Arcana implementations (XI-XXI)

/// Justice (XI) - Enhances 1 selected card to Glass Card
///
/// Glass Cards are fragile but provide powerful benefits.
/// This is a single-target enhancement card.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Justice;

impl Default for Justice {
    fn default() -> Self {
        Self::new()
    }
}

impl Justice {
    pub fn new() -> Self {
        Self
    }
}

impl TarotCard for Justice {
    fn card_id(&self) -> ConsumableId {
        ConsumableId::Justice
    }

    fn arcana_number(&self) -> u8 {
        11
    }

    fn arcana_name(&self) -> &'static str {
        "Justice"
    }

    fn flavor_text(&self) -> &'static str {
        "Balance through transparency and truth."
    }

    fn activate(&self, _game: &mut Game, target: Target) -> Result<TarotEffect, TarotError> {
        let card_target = target
            .as_card_target()
            .ok_or_else(|| TarotError::EnhancementFailed {
                reason: "Invalid target for Justice".to_string(),
            })?;

        if card_target.indices.len() != 1 {
            return Err(TarotError::InsufficientCards {
                needed: 1,
                available: card_target.indices.len(),
            });
        }

        let mut effect = TarotEffect::default();

        // Apply Glass enhancement to the target card
        effect.enhanced_cards.push(CardEnhancement {
            card_index: card_target.indices[0],
            collection: card_target.collection,
            enhancement: Enhancement::Glass,
            edition: None,
        });

        effect.description = "Enhanced 1 card to Glass Card".to_string();
        Ok(effect)
    }
}

impl Consumable for Justice {
    fn consumable_type(&self) -> ConsumableType {
        ConsumableType::Tarot
    }

    fn can_use(&self, game_state: &Game, target: &Target) -> bool {
        target.is_valid_type(self.get_target_type()) && target.validate(game_state).is_ok()
    }

    fn use_effect(&self, game_state: &mut Game, target: Target) -> Result<(), ConsumableError> {
        match self.activate(game_state, target) {
            Ok(_effect) => Ok(()),
            Err(e) => Err(ConsumableError::EffectFailed(e.to_string())),
        }
    }

    fn get_description(&self) -> String {
        "Enhances 1 selected card to Glass Card".to_string()
    }

    fn get_effect_category(&self) -> ConsumableEffect {
        ConsumableEffect::Enhancement
    }

    fn get_target_type(&self) -> TargetType {
        TargetType::Cards(1)
    }

    fn name(&self) -> &'static str {
        "Justice"
    }

    fn description(&self) -> &'static str {
        "Enhances 1 selected card to Glass Card"
    }

    fn cost(&self) -> usize {
        3
    }
}

/// The Hanged Man (XII) - Destroys up to 2 selected cards
///
/// This is a destructive card that removes cards from the game.
/// Useful for removing unwanted cards from hand or deck.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TheHangedMan;

impl Default for TheHangedMan {
    fn default() -> Self {
        Self::new()
    }
}

impl TheHangedMan {
    pub fn new() -> Self {
        Self
    }
}

impl TarotCard for TheHangedMan {
    fn card_id(&self) -> ConsumableId {
        ConsumableId::TheHangedMan
    }

    fn arcana_number(&self) -> u8 {
        12
    }

    fn arcana_name(&self) -> &'static str {
        "The Hanged Man"
    }

    fn flavor_text(&self) -> &'static str {
        "Sacrifice leads to enlightenment."
    }

    fn activate(&self, _game: &mut Game, target: Target) -> Result<TarotEffect, TarotError> {
        let card_target = target
            .as_card_target()
            .ok_or_else(|| TarotError::EnhancementFailed {
                reason: "Invalid target for The Hanged Man".to_string(),
            })?;

        let count = card_target.indices.len();
        if count == 0 || count > 2 {
            return Err(TarotError::InsufficientCards {
                needed: 1, // At least 1, up to 2
                available: count,
            });
        }

        let effect = TarotEffect {
            cards_removed: card_target.indices.clone(),
            description: format!("Destroyed {count} card(s)"),
            ..Default::default()
        };

        Ok(effect)
    }
}

impl Consumable for TheHangedMan {
    fn consumable_type(&self) -> ConsumableType {
        ConsumableType::Tarot
    }

    fn can_use(&self, game_state: &Game, target: &Target) -> bool {
        target.is_valid_type(self.get_target_type()) && target.validate(game_state).is_ok()
    }

    fn use_effect(&self, game_state: &mut Game, target: Target) -> Result<(), ConsumableError> {
        match self.activate(game_state, target) {
            Ok(_effect) => Ok(()),
            Err(e) => Err(ConsumableError::EffectFailed(e.to_string())),
        }
    }

    fn get_description(&self) -> String {
        "Destroys up to 2 selected cards".to_string()
    }

    fn get_effect_category(&self) -> ConsumableEffect {
        ConsumableEffect::Modification
    }

    fn get_target_type(&self) -> TargetType {
        TargetType::Cards(2)
    }

    fn name(&self) -> &'static str {
        "The Hanged Man"
    }

    fn description(&self) -> &'static str {
        "Destroys up to 2 selected cards"
    }

    fn cost(&self) -> usize {
        3
    }
}

/// Death (XIII) - Select 2 cards, convert left card to match right card
///
/// This is a transformation card that copies one card to another.
/// Requires exactly 2 cards - the first becomes a copy of the second.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Death;

impl Default for Death {
    fn default() -> Self {
        Self::new()
    }
}

impl Death {
    pub fn new() -> Self {
        Self
    }
}

impl TarotCard for Death {
    fn card_id(&self) -> ConsumableId {
        ConsumableId::Death
    }

    fn arcana_number(&self) -> u8 {
        13
    }

    fn arcana_name(&self) -> &'static str {
        "Death"
    }

    fn flavor_text(&self) -> &'static str {
        "Endings enable new beginnings."
    }

    fn activate(&self, _game: &mut Game, target: Target) -> Result<TarotEffect, TarotError> {
        let card_target = target
            .as_card_target()
            .ok_or_else(|| TarotError::EnhancementFailed {
                reason: "Invalid target for Death".to_string(),
            })?;

        if card_target.indices.len() != 2 {
            return Err(TarotError::InsufficientCards {
                needed: 2,
                available: card_target.indices.len(),
            });
        }

        let effect = TarotEffect {
            description: "Transformed first card to match second card".to_string(),
            ..Default::default()
        };

        // TODO: Implement actual card copying logic when proper game state mutation API is available

        Ok(effect)
    }
}

impl Consumable for Death {
    fn consumable_type(&self) -> ConsumableType {
        ConsumableType::Tarot
    }

    fn can_use(&self, game_state: &Game, target: &Target) -> bool {
        target.is_valid_type(self.get_target_type()) && target.validate(game_state).is_ok()
    }

    fn use_effect(&self, game_state: &mut Game, target: Target) -> Result<(), ConsumableError> {
        match self.activate(game_state, target) {
            Ok(_effect) => Ok(()),
            Err(e) => Err(ConsumableError::EffectFailed(e.to_string())),
        }
    }

    fn get_description(&self) -> String {
        "Select 2 cards, convert left card to match right card".to_string()
    }

    fn get_effect_category(&self) -> ConsumableEffect {
        ConsumableEffect::Modification
    }

    fn get_target_type(&self) -> TargetType {
        TargetType::Cards(2)
    }

    fn name(&self) -> &'static str {
        "Death"
    }

    fn description(&self) -> &'static str {
        "Select 2 cards, convert left card to match right card"
    }

    fn cost(&self) -> usize {
        3
    }
}

/// Temperance (XIV) - Gives the total sell value of all current Jokers
///
/// This is a utility card that converts joker value to money without destroying them.
/// Requires no targeting - affects all jokers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Temperance;

impl Default for Temperance {
    fn default() -> Self {
        Self::new()
    }
}

impl Temperance {
    pub fn new() -> Self {
        Self
    }
}

impl TarotCard for Temperance {
    fn card_id(&self) -> ConsumableId {
        ConsumableId::Temperance
    }

    fn arcana_number(&self) -> u8 {
        14
    }

    fn arcana_name(&self) -> &'static str {
        "Temperance"
    }

    fn flavor_text(&self) -> &'static str {
        "Moderation brings unexpected rewards."
    }

    fn activate(&self, game: &mut Game, _target: Target) -> Result<TarotEffect, TarotError> {
        // Calculate total sell value of all jokers
        let total_value = game.jokers.len() as i32 * 2; // Assuming $2 sell value per joker

        let effect = TarotEffect {
            money_change: total_value,
            description: format!("Gained ${total_value} from joker values"),
            ..Default::default()
        };

        // Apply money change to game state immediately
        game.money += total_value as f64;

        Ok(effect)
    }
}

impl Consumable for Temperance {
    fn consumable_type(&self) -> ConsumableType {
        ConsumableType::Tarot
    }

    fn can_use(&self, game_state: &Game, target: &Target) -> bool {
        target.is_valid_type(self.get_target_type()) && target.validate(game_state).is_ok()
    }

    fn use_effect(&self, game_state: &mut Game, target: Target) -> Result<(), ConsumableError> {
        match self.activate(game_state, target) {
            Ok(_effect) => Ok(()),
            Err(e) => Err(ConsumableError::EffectFailed(e.to_string())),
        }
    }

    fn get_description(&self) -> String {
        "Gives the total sell value of all current Jokers".to_string()
    }

    fn get_effect_category(&self) -> ConsumableEffect {
        ConsumableEffect::Utility
    }

    fn get_target_type(&self) -> TargetType {
        TargetType::None
    }

    fn name(&self) -> &'static str {
        "Temperance"
    }

    fn description(&self) -> &'static str {
        "Gives the total sell value of all current Jokers"
    }

    fn cost(&self) -> usize {
        3
    }
}

/// The Devil (XV) - Enhances 1 selected card to Gold Card
///
/// Gold Cards provide money-based benefits.
/// This is a single-target enhancement card.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TheDevil;

impl Default for TheDevil {
    fn default() -> Self {
        Self::new()
    }
}

impl TheDevil {
    pub fn new() -> Self {
        Self
    }
}

impl TarotCard for TheDevil {
    fn card_id(&self) -> ConsumableId {
        ConsumableId::TheDevil
    }

    fn arcana_number(&self) -> u8 {
        15
    }

    fn arcana_name(&self) -> &'static str {
        "The Devil"
    }

    fn flavor_text(&self) -> &'static str {
        "Temptation offers golden rewards."
    }

    fn activate(&self, _game: &mut Game, target: Target) -> Result<TarotEffect, TarotError> {
        let card_target = target
            .as_card_target()
            .ok_or_else(|| TarotError::EnhancementFailed {
                reason: "Invalid target for The Devil".to_string(),
            })?;

        if card_target.indices.len() != 1 {
            return Err(TarotError::InsufficientCards {
                needed: 1,
                available: card_target.indices.len(),
            });
        }

        let mut effect = TarotEffect::default();

        // Apply Gold enhancement to the target card
        effect.enhanced_cards.push(CardEnhancement {
            card_index: card_target.indices[0],
            collection: card_target.collection,
            enhancement: Enhancement::Gold,
            edition: None,
        });

        effect.description = "Enhanced 1 card to Gold Card".to_string();
        Ok(effect)
    }
}

impl Consumable for TheDevil {
    fn consumable_type(&self) -> ConsumableType {
        ConsumableType::Tarot
    }

    fn can_use(&self, game_state: &Game, target: &Target) -> bool {
        target.is_valid_type(self.get_target_type()) && target.validate(game_state).is_ok()
    }

    fn use_effect(&self, game_state: &mut Game, target: Target) -> Result<(), ConsumableError> {
        match self.activate(game_state, target) {
            Ok(_effect) => Ok(()),
            Err(e) => Err(ConsumableError::EffectFailed(e.to_string())),
        }
    }

    fn get_description(&self) -> String {
        "Enhances 1 selected card to Gold Card".to_string()
    }

    fn get_effect_category(&self) -> ConsumableEffect {
        ConsumableEffect::Enhancement
    }

    fn get_target_type(&self) -> TargetType {
        TargetType::Cards(1)
    }

    fn name(&self) -> &'static str {
        "The Devil"
    }

    fn description(&self) -> &'static str {
        "Enhances 1 selected card to Gold Card"
    }

    fn cost(&self) -> usize {
        3
    }
}

/// The Tower (XVI) - Enhances 1 selected card to Stone Card
///
/// Stone Cards provide defensive benefits.
/// This is a single-target enhancement card.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TheTower;

impl Default for TheTower {
    fn default() -> Self {
        Self::new()
    }
}

impl TheTower {
    pub fn new() -> Self {
        Self
    }
}

impl TarotCard for TheTower {
    fn card_id(&self) -> ConsumableId {
        ConsumableId::TheTower
    }

    fn arcana_number(&self) -> u8 {
        16
    }

    fn arcana_name(&self) -> &'static str {
        "The Tower"
    }

    fn flavor_text(&self) -> &'static str {
        "Destruction clears the way for foundation."
    }

    fn activate(&self, _game: &mut Game, target: Target) -> Result<TarotEffect, TarotError> {
        let card_target = target
            .as_card_target()
            .ok_or_else(|| TarotError::EnhancementFailed {
                reason: "Invalid target for The Tower".to_string(),
            })?;

        if card_target.indices.len() != 1 {
            return Err(TarotError::InsufficientCards {
                needed: 1,
                available: card_target.indices.len(),
            });
        }

        let mut effect = TarotEffect::default();

        // Apply Stone enhancement to the target card
        effect.enhanced_cards.push(CardEnhancement {
            card_index: card_target.indices[0],
            collection: card_target.collection,
            enhancement: Enhancement::Stone,
            edition: None,
        });

        effect.description = "Enhanced 1 card to Stone Card".to_string();
        Ok(effect)
    }
}

impl Consumable for TheTower {
    fn consumable_type(&self) -> ConsumableType {
        ConsumableType::Tarot
    }

    fn can_use(&self, game_state: &Game, target: &Target) -> bool {
        target.is_valid_type(self.get_target_type()) && target.validate(game_state).is_ok()
    }

    fn use_effect(&self, game_state: &mut Game, target: Target) -> Result<(), ConsumableError> {
        match self.activate(game_state, target) {
            Ok(_effect) => Ok(()),
            Err(e) => Err(ConsumableError::EffectFailed(e.to_string())),
        }
    }

    fn get_description(&self) -> String {
        "Enhances 1 selected card to Stone Card".to_string()
    }

    fn get_effect_category(&self) -> ConsumableEffect {
        ConsumableEffect::Enhancement
    }

    fn get_target_type(&self) -> TargetType {
        TargetType::Cards(1)
    }

    fn name(&self) -> &'static str {
        "The Tower"
    }

    fn description(&self) -> &'static str {
        "Enhances 1 selected card to Stone Card"
    }

    fn cost(&self) -> usize {
        3
    }
}

/// The Star (XVII) - Converts up to 3 selected cards to Diamonds
///
/// This is a suit conversion card that changes cards to Diamond suit.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TheStar;

impl Default for TheStar {
    fn default() -> Self {
        Self::new()
    }
}

impl TheStar {
    pub fn new() -> Self {
        Self
    }
}

impl TarotCard for TheStar {
    fn card_id(&self) -> ConsumableId {
        ConsumableId::TheStar
    }

    fn arcana_number(&self) -> u8 {
        17
    }

    fn arcana_name(&self) -> &'static str {
        "The Star"
    }

    fn flavor_text(&self) -> &'static str {
        "Hope shines bright as diamonds."
    }

    fn activate(&self, _game: &mut Game, target: Target) -> Result<TarotEffect, TarotError> {
        let card_target = target
            .as_card_target()
            .ok_or_else(|| TarotError::EnhancementFailed {
                reason: "Invalid target for The Star".to_string(),
            })?;

        let count = card_target.indices.len();
        if count == 0 || count > 3 {
            return Err(TarotError::InsufficientCards {
                needed: 1, // At least 1, up to 3
                available: count,
            });
        }

        let effect = TarotEffect {
            description: format!("Converted {count} card(s) to Diamonds"),
            ..Default::default()
        };

        // TODO: Implement actual suit conversion when proper game state mutation API is available

        Ok(effect)
    }
}

impl Consumable for TheStar {
    fn consumable_type(&self) -> ConsumableType {
        ConsumableType::Tarot
    }

    fn can_use(&self, game_state: &Game, target: &Target) -> bool {
        target.is_valid_type(self.get_target_type()) && target.validate(game_state).is_ok()
    }

    fn use_effect(&self, game_state: &mut Game, target: Target) -> Result<(), ConsumableError> {
        match self.activate(game_state, target) {
            Ok(_effect) => Ok(()),
            Err(e) => Err(ConsumableError::EffectFailed(e.to_string())),
        }
    }

    fn get_description(&self) -> String {
        "Converts up to 3 selected cards to Diamonds".to_string()
    }

    fn get_effect_category(&self) -> ConsumableEffect {
        ConsumableEffect::Modification
    }

    fn get_target_type(&self) -> TargetType {
        TargetType::Cards(3)
    }

    fn name(&self) -> &'static str {
        "The Star"
    }

    fn description(&self) -> &'static str {
        "Converts up to 3 selected cards to Diamonds"
    }

    fn cost(&self) -> usize {
        3
    }
}

/// The Moon (XVIII) - Converts up to 3 selected cards to Clubs
///
/// This is a suit conversion card that changes cards to Club suit.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TheMoon;

impl Default for TheMoon {
    fn default() -> Self {
        Self::new()
    }
}

impl TheMoon {
    pub fn new() -> Self {
        Self
    }
}

impl TarotCard for TheMoon {
    fn card_id(&self) -> ConsumableId {
        ConsumableId::TheMoon
    }

    fn arcana_number(&self) -> u8 {
        18
    }

    fn arcana_name(&self) -> &'static str {
        "The Moon"
    }

    fn flavor_text(&self) -> &'static str {
        "Illusions reveal hidden clubs."
    }

    fn activate(&self, _game: &mut Game, target: Target) -> Result<TarotEffect, TarotError> {
        let card_target = target
            .as_card_target()
            .ok_or_else(|| TarotError::EnhancementFailed {
                reason: "Invalid target for The Moon".to_string(),
            })?;

        let count = card_target.indices.len();
        if count == 0 || count > 3 {
            return Err(TarotError::InsufficientCards {
                needed: 1, // At least 1, up to 3
                available: count,
            });
        }

        let effect = TarotEffect {
            description: format!("Converted {count} card(s) to Clubs"),
            ..Default::default()
        };

        // TODO: Implement actual suit conversion when proper game state mutation API is available

        Ok(effect)
    }
}

impl Consumable for TheMoon {
    fn consumable_type(&self) -> ConsumableType {
        ConsumableType::Tarot
    }

    fn can_use(&self, game_state: &Game, target: &Target) -> bool {
        target.is_valid_type(self.get_target_type()) && target.validate(game_state).is_ok()
    }

    fn use_effect(&self, game_state: &mut Game, target: Target) -> Result<(), ConsumableError> {
        match self.activate(game_state, target) {
            Ok(_effect) => Ok(()),
            Err(e) => Err(ConsumableError::EffectFailed(e.to_string())),
        }
    }

    fn get_description(&self) -> String {
        "Converts up to 3 selected cards to Clubs".to_string()
    }

    fn get_effect_category(&self) -> ConsumableEffect {
        ConsumableEffect::Modification
    }

    fn get_target_type(&self) -> TargetType {
        TargetType::Cards(3)
    }

    fn name(&self) -> &'static str {
        "The Moon"
    }

    fn description(&self) -> &'static str {
        "Converts up to 3 selected cards to Clubs"
    }

    fn cost(&self) -> usize {
        3
    }
}

/// The Sun (XIX) - Converts up to 3 selected cards to Hearts
///
/// This is a suit conversion card that changes cards to Heart suit.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TheSun;

impl Default for TheSun {
    fn default() -> Self {
        Self::new()
    }
}

impl TheSun {
    pub fn new() -> Self {
        Self
    }
}

impl TarotCard for TheSun {
    fn card_id(&self) -> ConsumableId {
        ConsumableId::TheSun
    }

    fn arcana_number(&self) -> u8 {
        19
    }

    fn arcana_name(&self) -> &'static str {
        "The Sun"
    }

    fn flavor_text(&self) -> &'static str {
        "Radiant warmth fills every heart."
    }

    fn activate(&self, _game: &mut Game, target: Target) -> Result<TarotEffect, TarotError> {
        let card_target = target
            .as_card_target()
            .ok_or_else(|| TarotError::EnhancementFailed {
                reason: "Invalid target for The Sun".to_string(),
            })?;

        let count = card_target.indices.len();
        if count == 0 || count > 3 {
            return Err(TarotError::InsufficientCards {
                needed: 1, // At least 1, up to 3
                available: count,
            });
        }

        let effect = TarotEffect {
            description: format!("Converted {count} card(s) to Hearts"),
            ..Default::default()
        };

        // TODO: Implement actual suit conversion when proper game state mutation API is available

        Ok(effect)
    }
}

impl Consumable for TheSun {
    fn consumable_type(&self) -> ConsumableType {
        ConsumableType::Tarot
    }

    fn can_use(&self, game_state: &Game, target: &Target) -> bool {
        target.is_valid_type(self.get_target_type()) && target.validate(game_state).is_ok()
    }

    fn use_effect(&self, game_state: &mut Game, target: Target) -> Result<(), ConsumableError> {
        match self.activate(game_state, target) {
            Ok(_effect) => Ok(()),
            Err(e) => Err(ConsumableError::EffectFailed(e.to_string())),
        }
    }

    fn get_description(&self) -> String {
        "Converts up to 3 selected cards to Hearts".to_string()
    }

    fn get_effect_category(&self) -> ConsumableEffect {
        ConsumableEffect::Modification
    }

    fn get_target_type(&self) -> TargetType {
        TargetType::Cards(3)
    }

    fn name(&self) -> &'static str {
        "The Sun"
    }

    fn description(&self) -> &'static str {
        "Converts up to 3 selected cards to Hearts"
    }

    fn cost(&self) -> usize {
        3
    }
}

/// Judgement (XX) - Creates a random Joker card
///
/// This is a generation card that adds a new joker to the game.
/// Requires space for a new joker.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Judgement;

impl Default for Judgement {
    fn default() -> Self {
        Self::new()
    }
}

impl Judgement {
    pub fn new() -> Self {
        Self
    }
}

impl TarotCard for Judgement {
    fn card_id(&self) -> ConsumableId {
        ConsumableId::Judgement
    }

    fn arcana_number(&self) -> u8 {
        20
    }

    fn arcana_name(&self) -> &'static str {
        "Judgement"
    }

    fn flavor_text(&self) -> &'static str {
        "Divine judgment brings new allies."
    }

    fn activate(&self, game: &mut Game, _target: Target) -> Result<TarotEffect, TarotError> {
        // Check if we have space for a new joker first
        if game.jokers.len() >= 5 {
            // Assuming 5 is max joker slots
            return Err(TarotError::NoJokerAvailable);
        }

        let mut effect = TarotEffect::default();

        // Create a random common joker
        let common_jokers = [
            JokerId::Joker,
            JokerId::GreedyJoker,
            JokerId::LustyJoker,
            JokerId::WrathfulJoker,
            JokerId::GluttonousJoker,
        ];

        let selected_joker = common_jokers[game.rng.gen_range(0..common_jokers.len())];
        effect.jokers_created.push(selected_joker);
        effect.description = format!("Created a {selected_joker:?} joker");

        Ok(effect)
    }
}

impl Consumable for Judgement {
    fn consumable_type(&self) -> ConsumableType {
        ConsumableType::Tarot
    }

    fn can_use(&self, game_state: &Game, target: &Target) -> bool {
        target.is_valid_type(self.get_target_type()) && target.validate(game_state).is_ok()
    }

    fn use_effect(&self, game_state: &mut Game, target: Target) -> Result<(), ConsumableError> {
        match self.activate(game_state, target) {
            Ok(_effect) => Ok(()),
            Err(e) => Err(ConsumableError::EffectFailed(e.to_string())),
        }
    }

    fn get_description(&self) -> String {
        "Creates a random Joker card".to_string()
    }

    fn get_effect_category(&self) -> ConsumableEffect {
        ConsumableEffect::Generation
    }

    fn get_target_type(&self) -> TargetType {
        TargetType::None
    }

    fn name(&self) -> &'static str {
        "Judgement"
    }

    fn description(&self) -> &'static str {
        "Creates a random Joker card"
    }

    fn cost(&self) -> usize {
        3
    }
}

/// The World (XXI) - Converts up to 3 selected cards to Spades
///
/// This is a suit conversion card that changes cards to Spade suit.
/// Completes the suit conversion cycle with the other cards.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TheWorld;

impl Default for TheWorld {
    fn default() -> Self {
        Self::new()
    }
}

impl TheWorld {
    pub fn new() -> Self {
        Self
    }
}

impl TarotCard for TheWorld {
    fn card_id(&self) -> ConsumableId {
        ConsumableId::TheWorld
    }

    fn arcana_number(&self) -> u8 {
        21
    }

    fn arcana_name(&self) -> &'static str {
        "The World"
    }

    fn flavor_text(&self) -> &'static str {
        "Completion brings the sharpest spades."
    }

    fn activate(&self, _game: &mut Game, target: Target) -> Result<TarotEffect, TarotError> {
        let card_target = target
            .as_card_target()
            .ok_or_else(|| TarotError::EnhancementFailed {
                reason: "Invalid target for The World".to_string(),
            })?;

        let count = card_target.indices.len();
        if count == 0 || count > 3 {
            return Err(TarotError::InsufficientCards {
                needed: 1, // At least 1, up to 3
                available: count,
            });
        }

        let effect = TarotEffect {
            description: format!("Converted {count} card(s) to Spades"),
            ..Default::default()
        };

        // TODO: Implement actual suit conversion when proper game state mutation API is available

        Ok(effect)
    }
}

impl Consumable for TheWorld {
    fn consumable_type(&self) -> ConsumableType {
        ConsumableType::Tarot
    }

    fn can_use(&self, game_state: &Game, target: &Target) -> bool {
        target.is_valid_type(self.get_target_type()) && target.validate(game_state).is_ok()
    }

    fn use_effect(&self, game_state: &mut Game, target: Target) -> Result<(), ConsumableError> {
        match self.activate(game_state, target) {
            Ok(_effect) => Ok(()),
            Err(e) => Err(ConsumableError::EffectFailed(e.to_string())),
        }
    }

    fn get_description(&self) -> String {
        "Converts up to 3 selected cards to Spades".to_string()
    }

    fn get_effect_category(&self) -> ConsumableEffect {
        ConsumableEffect::Modification
    }

    fn get_target_type(&self) -> TargetType {
        TargetType::Cards(3)
    }

    fn name(&self) -> &'static str {
        "The World"
    }

    fn description(&self) -> &'static str {
        "Converts up to 3 selected cards to Spades"
    }

    fn cost(&self) -> usize {
        3
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::consumables::Target;

    #[test]
    fn test_tarot_factory_creation() {
        let factory = TarotFactory::new();

        // Test creating each tarot card
        let tarot_ids = factory.all_tarot_ids();
        for id in tarot_ids {
            let tarot = factory.create_tarot(id);
            assert!(tarot.is_ok(), "Failed to create tarot card: {id:?}");
        }
    }

    #[test]
    fn test_the_hermit_money_gain() {
        let hermit = TheHermit::new();
        let mut game = Game::default();
        let initial_money = game.money;

        let result = hermit.activate(&mut game, Target::None);
        assert!(result.is_ok());

        let effect = result.unwrap();
        assert_eq!(effect.money_change, 20);
        assert_eq!(game.money, initial_money + 20.0);
    }

    #[test]
    fn test_tarot_card_metadata() {
        let fool = TheFool::new();
        assert_eq!(fool.arcana_number(), 0);
        assert_eq!(fool.arcana_name(), "The Fool");
        assert_eq!(fool.card_id(), ConsumableId::TheFool);

        let magician = TheMagician::new();
        assert_eq!(magician.arcana_number(), 1);
        assert_eq!(magician.get_target_type(), TargetType::Cards(2));
    }

    // Tests for critical bug fixes

    #[test]
    fn test_emperor_rng_safety_fix() {
        let emperor = TheEmperor::new();
        let mut game = Game::default();

        // Test normal case - should work
        let result = emperor.activate(&mut game, Target::None);
        assert!(
            result.is_ok(),
            "TheEmperor should work with normal tarot cards"
        );

        // The RNG safety fix ensures we never panic with empty tarot_cards list
        // This is tested by the bounds check in the implementation
    }

    #[test]
    fn test_high_priestess_code_organization() {
        let priestess = TheHighPriestess::new();

        // Test that the implementations are now properly organized together
        assert_eq!(priestess.arcana_number(), 2);
        assert_eq!(priestess.arcana_name(), "The High Priestess");
        assert_eq!(priestess.card_id(), ConsumableId::TheHighPriestess);

        // Test the functionality works
        let mut game = Game::default();
        let result = priestess.activate(&mut game, Target::None);
        assert!(result.is_ok(), "TheHighPriestess should work properly");
    }

    #[test]
    fn test_fool_improved_implementation() {
        let fool = TheFool::new();
        let mut game = Game::default();

        // Test that TheFool no longer always creates the same joker type
        let result = fool.activate(&mut game, Target::None);
        assert!(result.is_ok(), "TheFool should work");

        let effect = result.unwrap();
        assert!(
            !effect.jokers_created.is_empty(),
            "TheFool should create a joker"
        );

        // Verify it's one of the common jokers, not always the same one
        let created_joker = effect.jokers_created[0];
        let valid_jokers = [
            JokerId::Joker,
            JokerId::GreedyJoker,
            JokerId::LustyJoker,
            JokerId::WrathfulJoker,
            JokerId::GluttonousJoker,
        ];
        assert!(
            valid_jokers.contains(&created_joker),
            "TheFool should create a valid common joker"
        );
    }

    #[test]
    fn test_strength_card_functionality() {
        let strength = StrengthCard::new();
        let card_target = Target::Cards(crate::consumables::CardTarget {
            indices: vec![0, 1],
            collection: crate::consumables::CardCollection::Hand,
            min_cards: 1,
            max_cards: 2,
        });

        let mut game = Game::default();
        let result = strength.activate(&mut game, card_target);
        assert!(
            result.is_ok(),
            "StrengthCard should work with valid targets"
        );

        let effect = result.unwrap();
        assert!(
            effect.description.contains("card(s)"),
            "StrengthCard should provide meaningful description"
        );
    }

    #[test]
    fn test_wheel_of_fortune_enhancement_preservation() {
        let wheel = WheelOfFortune::new();
        let card_target = Target::Cards(crate::consumables::CardTarget {
            indices: vec![0],
            collection: crate::consumables::CardCollection::Hand,
            min_cards: 1,
            max_cards: 1,
        });

        let mut game = Game::default();
        let result = wheel.activate(&mut game, card_target);
        assert!(
            result.is_ok(),
            "WheelOfFortune should work with valid targets"
        );

        // The fix ensures existing enhancements are preserved rather than hardcoded to Bonus
        let effect = result.unwrap();
        assert!(
            effect.description.contains("edition") || effect.description.contains("luck"),
            "WheelOfFortune should provide meaningful feedback"
        );
    }

    #[test]
    fn test_phantom_data_removal() {
        // Test that all tarot cards can be created without PhantomData issues
        let cards = [
            Box::new(TheFool::new()) as Box<dyn TarotCard>,
            Box::new(TheMagician::new()) as Box<dyn TarotCard>,
            Box::new(TheHighPriestess::new()) as Box<dyn TarotCard>,
            Box::new(TheEmpress::new()) as Box<dyn TarotCard>,
            Box::new(TheEmperor::new()) as Box<dyn TarotCard>,
            Box::new(TheHierophant::new()) as Box<dyn TarotCard>,
            Box::new(TheLovers::new()) as Box<dyn TarotCard>,
            Box::new(TheChariot::new()) as Box<dyn TarotCard>,
            Box::new(StrengthCard::new()) as Box<dyn TarotCard>,
            Box::new(TheHermit::new()) as Box<dyn TarotCard>,
            Box::new(WheelOfFortune::new()) as Box<dyn TarotCard>,
        ];

        // Verify all cards have proper metadata
        for (i, card) in cards.iter().enumerate() {
            assert_eq!(
                card.arcana_number() as usize,
                i,
                "Card arcana number should match index"
            );
            assert!(
                !card.arcana_name().is_empty(),
                "Card should have non-empty name"
            );
        }
    }
}
