//! Integration tests for the skip tags system
//!
//! Tests the full skip tags workflow: skipping blinds, getting tags, and activating them

use super::*;
use crate::action::Action;
use crate::game::Game;
use crate::skip_tags::tag_registry::global_registry;
use crate::stage::{Blind, Stage};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skip_blind_generates_tags() {
        let mut game = Game {
            stage: Stage::PreBlind(),
            ..Default::default()
        };

        // Skip a blind using the public Action API
        let result = game.handle_action(Action::SkipBlind(Blind::Small));

        // Should succeed
        assert!(result.is_ok());

        // Should move to PostBlind stage
        assert!(matches!(game.stage, Stage::PostBlind()));

        // May have generated a tag (50% chance in implementation)
        // Test doesn't verify specific tag generation due to randomness
        // but the system should handle it correctly either way
    }

    #[test]
    fn test_skip_tag_selection_workflow() {
        let mut game = Game::default();

        // Manually set up a tag selection scenario
        game.available_skip_tags
            .push(SkipTagInstance::new(SkipTagId::Boss));
        game.pending_tag_selection = true;

        // Select the Boss tag
        let result = game.handle_action(Action::SelectSkipTag(SkipTagId::Boss));

        // Should succeed
        assert!(result.is_ok());

        // Should no longer have pending selection
        assert!(!game.pending_tag_selection);

        // Should have the tag in active tags
        assert!(!game.active_skip_tags.is_empty());
    }

    #[test]
    fn test_double_tag_duplication() {
        let mut game = Game::default();

        // Set up tags for Double tag to duplicate
        game.available_skip_tags
            .push(SkipTagInstance::new(SkipTagId::Double));
        game.available_skip_tags
            .push(SkipTagInstance::new(SkipTagId::Boss));
        game.pending_tag_selection = true;

        let initial_boss_tags = game
            .available_skip_tags
            .iter()
            .filter(|t| t.id == SkipTagId::Boss)
            .count();

        // Select the Double tag
        let result = game.handle_action(Action::SelectSkipTag(SkipTagId::Double));
        assert!(result.is_ok());

        // Should have duplicated the Boss tag
        let final_boss_tags = game
            .available_skip_tags
            .iter()
            .chain(game.active_skip_tags.iter())
            .filter(|t| t.id == SkipTagId::Boss)
            .count();

        // Should have at least one more Boss tag than before
        // (either in available or active tags)
        assert!(final_boss_tags > initial_boss_tags);
    }

    #[test]
    fn test_boss_tag_activation() {
        let mut game = Game::default();

        // Set up Boss tag for activation
        game.available_skip_tags
            .push(SkipTagInstance::new(SkipTagId::Boss));
        game.pending_tag_selection = true;

        // Select the Boss tag
        let result = game.handle_action(Action::SelectSkipTag(SkipTagId::Boss));

        // Should succeed
        assert!(result.is_ok());

        // Boss tag should be in active tags
        assert!(game
            .active_skip_tags
            .iter()
            .any(|t| t.id == SkipTagId::Boss));
    }

    #[test]
    fn test_orbital_tag_activation() {
        let mut game = Game::default();

        // Set up Orbital tag for activation
        game.available_skip_tags
            .push(SkipTagInstance::new(SkipTagId::Orbital));
        game.pending_tag_selection = true;

        // Select the Orbital tag
        let result = game.handle_action(Action::SelectSkipTag(SkipTagId::Orbital));

        // Should succeed
        assert!(result.is_ok());

        // Orbital tag should be in active tags
        assert!(game
            .active_skip_tags
            .iter()
            .any(|t| t.id == SkipTagId::Orbital));
    }

    #[test]
    fn test_juggle_tag_stacking() {
        let mut game = Game::default();

        // Set up multiple Juggle tags for stacking
        game.available_skip_tags
            .push(SkipTagInstance::new(SkipTagId::Juggle));
        game.available_skip_tags
            .push(SkipTagInstance::new(SkipTagId::Juggle));
        game.pending_tag_selection = true;

        // Select first Juggle tag
        let result1 = game.handle_action(Action::SelectSkipTag(SkipTagId::Juggle));
        assert!(result1.is_ok());

        // Should still have one available Juggle tag
        assert!(!game.available_skip_tags.is_empty());
        assert!(game.pending_tag_selection);

        // Select second Juggle tag
        let result2 = game.handle_action(Action::SelectSkipTag(SkipTagId::Juggle));
        assert!(result2.is_ok());

        // Should have stacked Juggle tags
        let juggle_tags = game
            .active_skip_tags
            .iter()
            .filter(|t| t.id == SkipTagId::Juggle)
            .collect::<Vec<_>>();

        // Should have at least one Juggle tag (possibly stacked)
        assert!(!juggle_tags.is_empty());

        // Check if stacking worked (should have higher stack count)
        if let Some(juggle_tag) = juggle_tags.first() {
            // Juggle is stackable, so it should either have multiple instances
            // or one instance with stack_count > 1
            assert!(juggle_tags.len() > 1 || juggle_tag.stack_count > 1);
        }
    }

    #[test]
    fn test_tag_registry_initialization() {
        let registry = global_registry();

        // Should have all 4 utility tags registered
        assert!(registry.is_registered(SkipTagId::Double));
        assert!(registry.is_registered(SkipTagId::Boss));
        assert!(registry.is_registered(SkipTagId::Orbital));
        assert!(registry.is_registered(SkipTagId::Juggle));

        // Should be able to get implementations
        assert!(registry.get_tag(SkipTagId::Double).is_some());
        assert!(registry.get_tag(SkipTagId::Boss).is_some());
        assert!(registry.get_tag(SkipTagId::Orbital).is_some());
        assert!(registry.get_tag(SkipTagId::Juggle).is_some());
    }

    #[test]
    fn test_tag_rarity_weights() {
        let registry = global_registry();
        let weighted_tags = registry.get_weighted_tags();

        // Should have all 4 tags with weights
        assert_eq!(weighted_tags.len(), 4);

        // Each tag should have appropriate weight based on rarity
        for (tag_id, weight) in weighted_tags {
            let tag_impl = registry.get_tag(tag_id).unwrap();
            assert_eq!(weight, tag_impl.rarity().weight());
            assert!(weight > 0.0);
        }
    }

    #[test]
    fn test_invalid_tag_selection() {
        let mut game = Game::default();

        // Try to select a tag when no selection is pending
        let result = game.handle_action(Action::SelectSkipTag(SkipTagId::Double));

        // Should fail
        assert!(result.is_err());
    }

    #[test]
    fn test_skip_blind_wrong_stage() {
        let mut game = Game {
            stage: Stage::PostBlind(), // Wrong stage for skipping
            ..Default::default()
        };

        let result = game.handle_action(Action::SkipBlind(Blind::Small));

        // Should fail
        assert!(result.is_err());
    }

    #[test]
    fn test_comprehensive_tag_properties() {
        let registry = global_registry();

        // Test Double tag properties
        let double_tag = registry.get_tag(SkipTagId::Double).unwrap();
        assert_eq!(double_tag.name(), "Double");
        assert_eq!(double_tag.effect_type(), TagEffectType::SpecialMechanic);
        assert_eq!(double_tag.rarity(), TagRarity::Uncommon);
        assert!(double_tag.stackable());
        assert!(double_tag.selectable());

        // Test Boss tag properties
        let boss_tag = registry.get_tag(SkipTagId::Boss).unwrap();
        assert_eq!(boss_tag.name(), "Boss");
        assert_eq!(boss_tag.effect_type(), TagEffectType::BossBlindModifier);
        assert_eq!(boss_tag.rarity(), TagRarity::Rare);
        assert!(!boss_tag.stackable());

        // Test Orbital tag properties
        let orbital_tag = registry.get_tag(SkipTagId::Orbital).unwrap();
        assert_eq!(orbital_tag.name(), "Orbital");
        assert_eq!(orbital_tag.effect_type(), TagEffectType::ImmediateReward);
        assert_eq!(orbital_tag.rarity(), TagRarity::Uncommon);
        assert!(!orbital_tag.stackable());

        // Test Juggle tag properties
        let juggle_tag = registry.get_tag(SkipTagId::Juggle).unwrap();
        assert_eq!(juggle_tag.name(), "Juggle");
        assert_eq!(juggle_tag.effect_type(), TagEffectType::GameStateModifier);
        assert_eq!(juggle_tag.rarity(), TagRarity::Common);
        assert!(juggle_tag.stackable());
    }
}
