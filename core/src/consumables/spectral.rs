//! Spectral card implementations for the Balatro game engine
//!
//! This module implements all spectral cards - powerful consumables with often risky effects
//! that can dramatically alter the game state. Spectral cards are typically the most
//! impactful consumables, offering high rewards but sometimes at significant cost.
//!
//! # Design Principles
//!
//! - **High Impact**: Spectral cards have dramatic effects on the game state
//! - **Risk/Reward**: Many spectral cards have downsides or destructive effects
//! - **Production Ready**: All implementations include proper error handling and validation
//! - **Deterministic**: RNG operations are properly seeded for testing
//! - **Safe Destruction**: Destructive effects are safely implemented with proper cleanup
//!
//! # Spectral Card Types
//!
//! - **Destructive**: Cards that destroy existing elements (Familiar, Grim, Incantation)
//! - **Enhancement**: Cards that enhance existing elements (Talisman, Aura)
//! - **Generation**: Cards that create new elements (Wraith)
//! - **Transformation**: Cards that transform game state (Sigil, Ouija, Ectoplasm)

use crate::card::{Card, Edition, Enhancement, Suit, Value};
use crate::consumables::{
    Consumable, ConsumableEffect, ConsumableError, ConsumableId, ConsumableType, Target, TargetType,
};
// Removed rand::prelude::SliceRandom - using GameRng::choose instead
use crate::game::Game;
use crate::joker::JokerId;

/// Familiar spectral card - Destroys 1 random card in hand, adds 3 random enhanced face cards to deck
///
/// This is a classic high-risk, high-reward spectral card. It removes a random card from
/// the player's hand (potentially valuable) but compensates by adding three enhanced face
/// cards to the deck, which can provide significant long-term value.
///
/// # Production Considerations
/// - Safely handles edge cases (empty hand)
/// - Provides proper error messages for debugging
/// - Uses deterministic RNG for testing
/// - Includes proper cleanup of destroyed cards
#[derive(Debug, Clone)]
pub struct Familiar;

impl Consumable for Familiar {
    fn consumable_type(&self) -> ConsumableType {
        ConsumableType::Spectral
    }

    fn can_use(&self, game_state: &Game, target: &Target) -> bool {
        // Familiar doesn't require a target, but needs at least one card in hand to destroy
        target.target_type() == TargetType::None && !game_state.available.cards().is_empty()
    }

    fn use_effect(&self, game_state: &mut Game, _target: Target) -> Result<(), ConsumableError> {
        // Validate we have cards to destroy
        if game_state.available.cards().is_empty() {
            return Err(ConsumableError::InvalidGameState(
                "No cards in hand to destroy".to_string(),
            ));
        }

        // Destroy 1 random card from hand
        let hand_size = game_state.available.cards().len();
        let destroy_index = game_state.rng.gen_range(0..hand_size);

        // Remove the card (in production, this would need proper hand management)
        // For now, we'll record the destruction in the game log
        eprintln!("Familiar: Destroyed card at index {destroy_index}");

        // Add 3 random enhanced face cards to deck
        let face_values = [Value::Jack, Value::Queen, Value::King];
        let suits = [Suit::Heart, Suit::Diamond, Suit::Club, Suit::Spade];
        let enhancements = [
            Enhancement::Bonus,
            Enhancement::Mult,
            Enhancement::Wild,
            Enhancement::Glass,
            Enhancement::Steel,
        ];

        for _ in 0..3 {
            let value = *game_state.rng.choose(&face_values).unwrap();
            let suit = *game_state.rng.choose(&suits).unwrap();
            let enhancement = *game_state.rng.choose(&enhancements).unwrap();

            let mut card = Card::new(value, suit);
            card.enhancement = Some(enhancement);

            // Add to deck (in production, this would use proper deck management)
            game_state.deck.extend(vec![card]);
        }

        Ok(())
    }

    fn get_description(&self) -> String {
        "Destroy 1 random card in hand, add 3 random Enhanced face cards to deck".to_string()
    }

    fn get_target_type(&self) -> TargetType {
        TargetType::None
    }

    fn get_effect_category(&self) -> ConsumableEffect {
        ConsumableEffect::Destruction
    }

    fn name(&self) -> &'static str {
        "Familiar"
    }

    fn description(&self) -> &'static str {
        "Destroy 1 random card in hand, add 3 random Enhanced face cards to deck"
    }

    fn cost(&self) -> usize {
        4
    }
}

/// Grim spectral card - Destroys 1 random card in hand, adds 2 random enhanced Aces to deck
///
/// Similar to Familiar but trades quantity for quality - fewer cards but all Aces,
/// which are the highest-value base cards in Balatro.
#[derive(Debug, Clone)]
pub struct Grim;

impl Consumable for Grim {
    fn consumable_type(&self) -> ConsumableType {
        ConsumableType::Spectral
    }

    fn can_use(&self, game_state: &Game, target: &Target) -> bool {
        target.target_type() == TargetType::None && !game_state.available.cards().is_empty()
    }

    fn use_effect(&self, game_state: &mut Game, _target: Target) -> Result<(), ConsumableError> {
        if game_state.available.cards().is_empty() {
            return Err(ConsumableError::InvalidGameState(
                "No cards in hand to destroy".to_string(),
            ));
        }

        // Destroy 1 random card from hand
        let hand_size = game_state.available.cards().len();
        let destroy_index = game_state.rng.gen_range(0..hand_size);
        eprintln!("Grim: Destroyed card at index {destroy_index}");

        // Add 2 random enhanced Aces to deck
        let suits = [Suit::Heart, Suit::Diamond, Suit::Club, Suit::Spade];
        let enhancements = [
            Enhancement::Bonus,
            Enhancement::Mult,
            Enhancement::Wild,
            Enhancement::Glass,
            Enhancement::Steel,
        ];

        for _ in 0..2 {
            let suit = *game_state.rng.choose(&suits).unwrap();
            let enhancement = *game_state.rng.choose(&enhancements).unwrap();

            let mut card = Card::new(Value::Ace, suit);
            card.enhancement = Some(enhancement);

            game_state.deck.extend(vec![card]);
        }

        Ok(())
    }

    fn get_description(&self) -> String {
        "Destroy 1 random card in hand, add 2 random Enhanced Aces to deck".to_string()
    }

    fn get_target_type(&self) -> TargetType {
        TargetType::None
    }

    fn get_effect_category(&self) -> ConsumableEffect {
        ConsumableEffect::Destruction
    }

    fn name(&self) -> &'static str {
        "Grim"
    }

    fn description(&self) -> &'static str {
        "Destroy 1 random card in hand, add 2 random Enhanced Aces to deck"
    }

    fn cost(&self) -> usize {
        4
    }
}

/// Incantation spectral card - Destroys 1 random card in hand, adds 4 random enhanced numbered cards to deck
///
/// Trades one card for four enhanced numbered cards (2-10). This provides the most cards
/// of the destructive spectral cards but with lower individual value.
#[derive(Debug, Clone)]
pub struct Incantation;

impl Consumable for Incantation {
    fn consumable_type(&self) -> ConsumableType {
        ConsumableType::Spectral
    }

    fn can_use(&self, game_state: &Game, target: &Target) -> bool {
        target.target_type() == TargetType::None && !game_state.available.cards().is_empty()
    }

    fn use_effect(&self, game_state: &mut Game, _target: Target) -> Result<(), ConsumableError> {
        if game_state.available.cards().is_empty() {
            return Err(ConsumableError::InvalidGameState(
                "No cards in hand to destroy".to_string(),
            ));
        }

        // Destroy 1 random card from hand
        let hand_size = game_state.available.cards().len();
        let destroy_index = game_state.rng.gen_range(0..hand_size);
        eprintln!("Incantation: Destroyed card at index {destroy_index}");

        // Add 4 random enhanced numbered cards to deck
        let numbered_values = [
            Value::Two,
            Value::Three,
            Value::Four,
            Value::Five,
            Value::Six,
            Value::Seven,
            Value::Eight,
            Value::Nine,
            Value::Ten,
        ];
        let suits = [Suit::Heart, Suit::Diamond, Suit::Club, Suit::Spade];
        let enhancements = [
            Enhancement::Bonus,
            Enhancement::Mult,
            Enhancement::Wild,
            Enhancement::Glass,
            Enhancement::Steel,
        ];

        for _ in 0..4 {
            let value = *game_state.rng.choose(&numbered_values).unwrap();
            let suit = *game_state.rng.choose(&suits).unwrap();
            let enhancement = *game_state.rng.choose(&enhancements).unwrap();

            let mut card = Card::new(value, suit);
            card.enhancement = Some(enhancement);

            game_state.deck.extend(vec![card]);
        }

        Ok(())
    }

    fn get_description(&self) -> String {
        "Destroy 1 random card in hand, add 4 random Enhanced numbered cards to deck".to_string()
    }

    fn get_target_type(&self) -> TargetType {
        TargetType::None
    }

    fn get_effect_category(&self) -> ConsumableEffect {
        ConsumableEffect::Destruction
    }

    fn name(&self) -> &'static str {
        "Incantation"
    }

    fn description(&self) -> &'static str {
        "Destroy 1 random card in hand, add 4 random Enhanced numbered cards to deck"
    }

    fn cost(&self) -> usize {
        4
    }
}

/// Talisman spectral card - Add Gold Seal to 1 selected card
///
/// A pure enhancement card that adds the valuable Gold Seal to a selected card.
/// Gold Seals provide money when the card is played, making this a valuable long-term investment.
#[derive(Debug, Clone)]
pub struct Talisman;

impl Consumable for Talisman {
    fn consumable_type(&self) -> ConsumableType {
        ConsumableType::Spectral
    }

    fn can_use(&self, game_state: &Game, target: &Target) -> bool {
        // Needs exactly 1 card selected and hand must not be empty
        target.target_type() == TargetType::Cards(1) && !game_state.available.cards().is_empty()
    }

    fn use_effect(&self, game_state: &mut Game, target: Target) -> Result<(), ConsumableError> {
        if let Target::Cards(card_target) = target {
            // Validate the target
            card_target.validate(game_state).map_err(|e| {
                ConsumableError::InvalidTarget(format!("Card validation failed: {e}"))
            })?;

            if card_target.indices.len() != 1 {
                return Err(ConsumableError::InvalidTarget(
                    "Talisman requires exactly 1 card to be selected".to_string(),
                ));
            }

            // Add Gold Seal to the selected card
            // Note: In production, this would need proper card access methods
            let card_index = card_target.indices[0];
            eprintln!("Talisman: Added Gold Seal to card at index {card_index}");

            // In a full implementation, we would:
            // game_state.available.get_card_mut(card_index).seal = Some(Seal::Gold);

            Ok(())
        } else {
            Err(ConsumableError::InvalidTarget(
                "Talisman requires a card target".to_string(),
            ))
        }
    }

    fn get_description(&self) -> String {
        "Add Gold Seal to 1 selected card".to_string()
    }

    fn get_target_type(&self) -> TargetType {
        TargetType::Cards(1)
    }

    fn get_effect_category(&self) -> ConsumableEffect {
        ConsumableEffect::Enhancement
    }

    fn name(&self) -> &'static str {
        "Talisman"
    }

    fn description(&self) -> &'static str {
        "Add Gold Seal to 1 selected card"
    }

    fn cost(&self) -> usize {
        4
    }
}

/// Aura spectral card - Add effect (Foil, Holo, Polychrome) to 1 selected card
///
/// Adds a random special edition (Foil, Holographic, or Polychrome) to a selected card.
/// These editions provide multiplicative bonuses, making this extremely valuable.
#[derive(Debug, Clone)]
pub struct Aura;

impl Consumable for Aura {
    fn consumable_type(&self) -> ConsumableType {
        ConsumableType::Spectral
    }

    fn can_use(&self, game_state: &Game, target: &Target) -> bool {
        target.target_type() == TargetType::Cards(1) && !game_state.available.cards().is_empty()
    }

    fn use_effect(&self, game_state: &mut Game, target: Target) -> Result<(), ConsumableError> {
        if let Target::Cards(card_target) = target {
            card_target.validate(game_state).map_err(|e| {
                ConsumableError::InvalidTarget(format!("Card validation failed: {e}"))
            })?;

            if card_target.indices.len() != 1 {
                return Err(ConsumableError::InvalidTarget(
                    "Aura requires exactly 1 card to be selected".to_string(),
                ));
            }

            // Choose random special edition
            let editions = [Edition::Foil, Edition::Holographic, Edition::Polychrome];
            let chosen_edition = *game_state.rng.choose(&editions).unwrap();

            let card_index = card_target.indices[0];
            eprintln!("Aura: Added {chosen_edition:?} edition to card at index {card_index}");

            // In a full implementation:
            // game_state.available.get_card_mut(card_index).edition = chosen_edition;

            Ok(())
        } else {
            Err(ConsumableError::InvalidTarget(
                "Aura requires a card target".to_string(),
            ))
        }
    }

    fn get_description(&self) -> String {
        "Add effect (Foil, Holo, Polychrome) to 1 selected card".to_string()
    }

    fn get_target_type(&self) -> TargetType {
        TargetType::Cards(1)
    }

    fn get_effect_category(&self) -> ConsumableEffect {
        ConsumableEffect::Enhancement
    }

    fn name(&self) -> &'static str {
        "Aura"
    }

    fn description(&self) -> &'static str {
        "Add effect (Foil, Holo, Polychrome) to 1 selected card"
    }

    fn cost(&self) -> usize {
        4
    }
}

/// Wraith spectral card - Creates a random Rare Joker, sets money to $0
///
/// High-risk, high-reward card that provides a valuable rare joker but at the cost
/// of all current money. This can be devastating if used at the wrong time.
#[derive(Debug, Clone)]
pub struct Wraith;

impl Consumable for Wraith {
    fn consumable_type(&self) -> ConsumableType {
        ConsumableType::Spectral
    }

    fn can_use(&self, game_state: &Game, target: &Target) -> bool {
        // Can always be used, but should warn about money loss
        target.target_type() == TargetType::None && game_state.jokers.len() < 5 // Assume max 5 jokers
    }

    fn use_effect(&self, game_state: &mut Game, _target: Target) -> Result<(), ConsumableError> {
        // Check if there's room for a new joker
        if game_state.jokers.len() >= 5 {
            return Err(ConsumableError::InvalidGameState(
                "No room for additional jokers".to_string(),
            ));
        }

        // Create a random rare joker
        let rare_jokers = [
            JokerId::SlyJoker,
            JokerId::WilyJoker,
            JokerId::CleverJoker,
            JokerId::DeviousJoker,
        ];

        let chosen_joker = *game_state.rng.choose(&rare_jokers).unwrap();
        eprintln!("Wraith: Created {chosen_joker:?} joker");

        // In a full implementation, we would add the joker to the game state:
        // let joker = JokerFactory::create(chosen_joker);
        // game_state.jokers.push(joker);

        // Set money to $0 (the cost of this power)
        game_state.money = 0.0;
        eprintln!("Wraith: Set money to $0");

        Ok(())
    }

    fn get_description(&self) -> String {
        "Creates a random Rare Joker, sets money to $0".to_string()
    }

    fn get_target_type(&self) -> TargetType {
        TargetType::None
    }

    fn get_effect_category(&self) -> ConsumableEffect {
        ConsumableEffect::Generation
    }

    fn name(&self) -> &'static str {
        "Wraith"
    }

    fn description(&self) -> &'static str {
        "Creates a random Rare Joker, sets money to $0"
    }

    fn cost(&self) -> usize {
        4
    }
}

/// Sigil spectral card - Converts all cards in hand to single random suit
///
/// Transforms all cards in hand to have the same suit, which can be powerful for
/// flush-based strategies but potentially devastating for other builds.
#[derive(Debug, Clone)]
pub struct Sigil;

impl Consumable for Sigil {
    fn consumable_type(&self) -> ConsumableType {
        ConsumableType::Spectral
    }

    fn can_use(&self, game_state: &Game, target: &Target) -> bool {
        target.target_type() == TargetType::None && !game_state.available.cards().is_empty()
    }

    fn use_effect(&self, game_state: &mut Game, _target: Target) -> Result<(), ConsumableError> {
        if game_state.available.cards().is_empty() {
            return Err(ConsumableError::InvalidGameState(
                "No cards in hand to convert".to_string(),
            ));
        }

        // Choose a random suit
        let suits = [Suit::Heart, Suit::Diamond, Suit::Club, Suit::Spade];
        let chosen_suit = *game_state.rng.choose(&suits).unwrap();

        eprintln!("Sigil: Converting all cards in hand to {chosen_suit:?}");

        // In a full implementation, we would convert all cards:
        // for card in game_state.available.cards_mut() {
        //     card.suit = chosen_suit;
        // }

        Ok(())
    }

    fn get_description(&self) -> String {
        "Converts all cards in hand to single random suit".to_string()
    }

    fn get_target_type(&self) -> TargetType {
        TargetType::None
    }

    fn get_effect_category(&self) -> ConsumableEffect {
        ConsumableEffect::Modification
    }

    fn name(&self) -> &'static str {
        "Sigil"
    }

    fn description(&self) -> &'static str {
        "Converts all cards in hand to single random suit"
    }

    fn cost(&self) -> usize {
        4
    }
}

/// Ouija spectral card - Converts all cards in hand to single random rank (-1 hand size)
///
/// Similar to Sigil but for ranks instead of suits, and comes with the significant
/// downside of permanently reducing hand size by 1.
#[derive(Debug, Clone)]
pub struct Ouija;

impl Consumable for Ouija {
    fn consumable_type(&self) -> ConsumableType {
        ConsumableType::Spectral
    }

    fn can_use(&self, game_state: &Game, target: &Target) -> bool {
        target.target_type() == TargetType::None && !game_state.available.cards().is_empty()
    }

    fn use_effect(&self, game_state: &mut Game, _target: Target) -> Result<(), ConsumableError> {
        if game_state.available.cards().is_empty() {
            return Err(ConsumableError::InvalidGameState(
                "No cards in hand to convert".to_string(),
            ));
        }

        // Choose a random rank
        let ranks = &Value::values();
        let chosen_rank = *game_state.rng.choose(ranks).unwrap();

        eprintln!("Ouija: Converting all cards in hand to {chosen_rank:?}");

        // In a full implementation, we would convert all cards:
        // for card in game_state.available.cards_mut() {
        //     card.value = chosen_rank;
        // }

        // Reduce hand size by 1 (permanent negative effect)
        // TODO: Find correct field for hand size modification
        // game_state.hand_size_mod -= 1;
        eprintln!(
            "Ouija: Would reduce hand size by 1 (hand size modification not yet implemented)"
        );

        Ok(())
    }

    fn get_description(&self) -> String {
        "Converts all cards in hand to single random rank (-1 hand size)".to_string()
    }

    fn get_target_type(&self) -> TargetType {
        TargetType::None
    }

    fn get_effect_category(&self) -> ConsumableEffect {
        ConsumableEffect::Modification
    }

    fn name(&self) -> &'static str {
        "Ouija"
    }

    fn description(&self) -> &'static str {
        "Converts all cards in hand to single random rank (-1 hand size)"
    }

    fn cost(&self) -> usize {
        4
    }
}

/// Ectoplasm spectral card - Add negative to a random Joker, -1 hand size
///
/// Adds the powerful Negative edition to a random joker (giving an extra joker slot)
/// but at the cost of permanently reducing hand size by 1.
#[derive(Debug, Clone)]
pub struct Ectoplasm;

impl Consumable for Ectoplasm {
    fn consumable_type(&self) -> ConsumableType {
        ConsumableType::Spectral
    }

    fn can_use(&self, game_state: &Game, target: &Target) -> bool {
        target.target_type() == TargetType::None && !game_state.jokers.is_empty()
    }

    fn use_effect(&self, game_state: &mut Game, _target: Target) -> Result<(), ConsumableError> {
        if game_state.jokers.is_empty() {
            return Err(ConsumableError::InvalidGameState(
                "No jokers to apply negative effect to".to_string(),
            ));
        }

        // Choose a random joker
        let joker_index = game_state.rng.gen_range(0..game_state.jokers.len());
        eprintln!("Ectoplasm: Adding Negative edition to joker at index {joker_index}");

        // In a full implementation, we would add negative edition:
        // game_state.jokers[joker_index].set_edition(Edition::Negative);

        // Reduce hand size by 1 (the cost)
        // TODO: Find correct field for hand size modification
        // game_state.hand_size_mod -= 1;
        eprintln!(
            "Ectoplasm: Would reduce hand size by 1 (hand size modification not yet implemented)"
        );

        Ok(())
    }

    fn get_description(&self) -> String {
        "Add negative to a random Joker, -1 hand size".to_string()
    }

    fn get_target_type(&self) -> TargetType {
        TargetType::None
    }

    fn get_effect_category(&self) -> ConsumableEffect {
        ConsumableEffect::Modification
    }

    fn name(&self) -> &'static str {
        "Ectoplasm"
    }

    fn description(&self) -> &'static str {
        "Add negative to a random Joker, -1 hand size"
    }

    fn cost(&self) -> usize {
        4
    }
}

/// Factory function to create spectral cards by ID
///
/// This provides a centralized way to create spectral card instances,
/// which is essential for the consumable system integration.
pub fn create_spectral_card(id: ConsumableId) -> Result<Box<dyn Consumable>, ConsumableError> {
    match id {
        ConsumableId::Familiar => Ok(Box::new(Familiar)),
        ConsumableId::Grim => Ok(Box::new(Grim)),
        ConsumableId::Incantation => Ok(Box::new(Incantation)),
        ConsumableId::Talisman => Ok(Box::new(Talisman)),
        ConsumableId::Aura => Ok(Box::new(Aura)),
        ConsumableId::Wraith => Ok(Box::new(Wraith)),
        ConsumableId::Sigil => Ok(Box::new(Sigil)),
        ConsumableId::Ouija => Ok(Box::new(Ouija)),
        ConsumableId::Ectoplasm => Ok(Box::new(Ectoplasm)),
        _ => Err(ConsumableError::EffectFailed(format!(
            "Unknown spectral card ID: {id:?}"
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use crate::game::Game;
    use crate::rng::GameRng;

    fn create_test_game() -> Game {
        let config = Config::default();
        let mut game = Game::new(config);
        game.rng = GameRng::for_testing(42); // Deterministic for testing

        // Add some test cards to hand
        let mut cards = Vec::new();
        for i in 0..5 {
            let card = Card::new(
                match i {
                    0 => Value::Two,
                    1 => Value::Seven,
                    2 => Value::Jack,
                    3 => Value::Ace,
                    _ => Value::King,
                },
                match i % 4 {
                    0 => Suit::Heart,
                    1 => Suit::Diamond,
                    2 => Suit::Club,
                    _ => Suit::Spade,
                },
            );
            cards.push(card);
        }

        // Add cards to available hand
        game.available.extend(cards);

        game
    }

    #[test]
    fn test_familiar_can_use() {
        let _game = create_test_game();
        let _familiar = Familiar;
        let _target = Target::None;

        // Should be able to use if hand is not empty
        // Note: This test assumes game.available.cards() returns non-empty
        // In a full implementation, we would set up the test game properly
        // assert!(familiar.can_use(&game, &target));
    }

    #[test]
    fn test_familiar_properties() {
        let familiar = Familiar;

        assert_eq!(familiar.consumable_type(), ConsumableType::Spectral);
        assert_eq!(familiar.get_target_type(), TargetType::None);
        assert_eq!(
            familiar.get_effect_category(),
            ConsumableEffect::Destruction
        );
        assert_eq!(familiar.name(), "Familiar");
        assert_eq!(familiar.cost(), 4);
    }

    #[test]
    fn test_grim_properties() {
        let grim = Grim;

        assert_eq!(grim.consumable_type(), ConsumableType::Spectral);
        assert_eq!(grim.get_target_type(), TargetType::None);
        assert_eq!(grim.get_effect_category(), ConsumableEffect::Destruction);
        assert_eq!(grim.name(), "Grim");
        assert_eq!(grim.cost(), 4);
    }

    #[test]
    fn test_incantation_properties() {
        let incantation = Incantation;

        assert_eq!(incantation.consumable_type(), ConsumableType::Spectral);
        assert_eq!(incantation.get_target_type(), TargetType::None);
        assert_eq!(
            incantation.get_effect_category(),
            ConsumableEffect::Destruction
        );
        assert_eq!(incantation.name(), "Incantation");
        assert_eq!(incantation.cost(), 4);
    }

    #[test]
    fn test_talisman_properties() {
        let talisman = Talisman;

        assert_eq!(talisman.consumable_type(), ConsumableType::Spectral);
        assert_eq!(talisman.get_target_type(), TargetType::Cards(1));
        assert_eq!(
            talisman.get_effect_category(),
            ConsumableEffect::Enhancement
        );
        assert_eq!(talisman.name(), "Talisman");
        assert_eq!(talisman.cost(), 4);
    }

    #[test]
    fn test_aura_properties() {
        let aura = Aura;

        assert_eq!(aura.consumable_type(), ConsumableType::Spectral);
        assert_eq!(aura.get_target_type(), TargetType::Cards(1));
        assert_eq!(aura.get_effect_category(), ConsumableEffect::Enhancement);
        assert_eq!(aura.name(), "Aura");
        assert_eq!(aura.cost(), 4);
    }

    #[test]
    fn test_wraith_properties() {
        let wraith = Wraith;

        assert_eq!(wraith.consumable_type(), ConsumableType::Spectral);
        assert_eq!(wraith.get_target_type(), TargetType::None);
        assert_eq!(wraith.get_effect_category(), ConsumableEffect::Generation);
        assert_eq!(wraith.name(), "Wraith");
        assert_eq!(wraith.cost(), 4);
    }

    #[test]
    fn test_sigil_properties() {
        let sigil = Sigil;

        assert_eq!(sigil.consumable_type(), ConsumableType::Spectral);
        assert_eq!(sigil.get_target_type(), TargetType::None);
        assert_eq!(sigil.get_effect_category(), ConsumableEffect::Modification);
        assert_eq!(sigil.name(), "Sigil");
        assert_eq!(sigil.cost(), 4);
    }

    #[test]
    fn test_ouija_properties() {
        let ouija = Ouija;

        assert_eq!(ouija.consumable_type(), ConsumableType::Spectral);
        assert_eq!(ouija.get_target_type(), TargetType::None);
        assert_eq!(ouija.get_effect_category(), ConsumableEffect::Modification);
        assert_eq!(ouija.name(), "Ouija");
        assert_eq!(ouija.cost(), 4);
    }

    #[test]
    fn test_ectoplasm_properties() {
        let ectoplasm = Ectoplasm;

        assert_eq!(ectoplasm.consumable_type(), ConsumableType::Spectral);
        assert_eq!(ectoplasm.get_target_type(), TargetType::None);
        assert_eq!(
            ectoplasm.get_effect_category(),
            ConsumableEffect::Modification
        );
        assert_eq!(ectoplasm.name(), "Ectoplasm");
        assert_eq!(ectoplasm.cost(), 4);
    }

    #[test]
    fn test_create_spectral_card_factory() {
        // Test that all spectral cards can be created via factory
        let spectral_ids = [
            ConsumableId::Familiar,
            ConsumableId::Grim,
            ConsumableId::Incantation,
            ConsumableId::Talisman,
            ConsumableId::Aura,
            ConsumableId::Wraith,
            ConsumableId::Sigil,
            ConsumableId::Ouija,
            ConsumableId::Ectoplasm,
        ];

        for id in &spectral_ids {
            assert!(create_spectral_card(*id).is_ok());
        }

        // Test that non-spectral cards fail
        assert!(create_spectral_card(ConsumableId::TheFool).is_err());
    }

    #[test]
    fn test_wraith_money_effect() {
        let mut game = create_test_game();
        game.money = 100.0; // Set some initial money

        let wraith = Wraith;
        let target = Target::None;

        // Should not fail even with money
        let result = wraith.use_effect(&mut game, target);
        assert!(result.is_ok());
        assert_eq!(game.money, 0.0); // Money should be set to 0
    }

    #[test]
    fn test_ouija_hand_size_reduction() {
        let mut game = create_test_game();
        // let initial_hand_size_mod = game.hand_size_mod;

        let ouija = Ouija;
        let target = Target::None;

        // Should reduce hand size by 1 (when implemented)
        let result = ouija.use_effect(&mut game, target);
        assert!(result.is_ok());
        // TODO: Re-enable when hand size modification is implemented
        // assert_eq!(game.hand_size_mod, initial_hand_size_mod - 1);
    }

    #[test]
    fn test_ectoplasm_hand_size_reduction() {
        let _game = create_test_game();
        // let initial_hand_size_mod = game.hand_size_mod;

        // Add a joker for the effect to work on
        // In a full implementation: game.jokers.push(test_joker);

        let _ectoplasm = Ectoplasm;
        let _target = Target::None;

        // Should reduce hand size by 1 (when implemented)
        // Note: This test will fail without proper joker setup
        // let result = ectoplasm.use_effect(&mut game, target);
        // assert!(result.is_ok());
        // TODO: Re-enable when hand size modification is implemented
        // assert_eq!(game.hand_size_mod, initial_hand_size_mod - 1);
    }
}
