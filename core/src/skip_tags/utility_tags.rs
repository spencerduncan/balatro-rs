//! Utility Skip Tags
//!
//! Implementation of the 4 utility category skip tags:
//! - Double: Duplicates the next selected tag
//! - Boss: Re-rolls the next Boss Blind
//! - Orbital: Upgrades a random poker hand by 3 levels
//! - Juggle: Adds +3 hand size for next round only

use super::tag_effects::{
    boss_reroll_effect, duplication_effect, hand_upgrade_effect, temporary_hand_size_effect,
};
use super::{SkipTag, SkipTagContext, SkipTagId, SkipTagResult, TagEffectType, TagRarity};

/// Double Tag - Gives a copy of the next Tag selected (excluding Double Tags)
/// Can be stacked, with each addition creating one additional copy of a tag
#[derive(Debug)]
pub struct DoubleTag;

impl SkipTag for DoubleTag {
    fn id(&self) -> SkipTagId {
        SkipTagId::Double
    }

    fn name(&self) -> &'static str {
        "Double"
    }

    fn description(&self) -> &'static str {
        "Gives a copy of the next Tag selected"
    }

    fn effect_type(&self) -> TagEffectType {
        TagEffectType::SpecialMechanic
    }

    fn rarity(&self) -> TagRarity {
        TagRarity::Uncommon
    }

    fn stackable(&self) -> bool {
        true
    }

    fn selectable(&self) -> bool {
        // Double tags require selection of another tag to duplicate
        true
    }

    fn activate(&self, context: SkipTagContext) -> SkipTagResult {
        // For Double tag, we need to know which tag to duplicate
        // In a real implementation, this would require UI interaction
        // For now, we'll duplicate the first available tag that isn't Double

        let available_non_double: Vec<_> = context
            .available_tags
            .iter()
            .filter(|&&tag| tag != SkipTagId::Double)
            .copied()
            .collect();

        if let Some(&first_tag) = available_non_double.first() {
            duplication_effect(context, first_tag)
        } else {
            SkipTagResult {
                game: context.game,
                additional_tags: vec![],
                success: false,
                message: Some("No valid tags to duplicate".to_string()),
            }
        }
    }

    fn can_activate(&self, context: &SkipTagContext) -> bool {
        // Can only activate if there are non-Double tags available
        context
            .available_tags
            .iter()
            .any(|&tag| tag != SkipTagId::Double)
    }
}

/// Boss Tag - Re-rolls the next Boss Blind
/// If Director's Cut has been used, this will also consume its one re-roll option
#[derive(Debug)]
pub struct BossTag;

impl SkipTag for BossTag {
    fn id(&self) -> SkipTagId {
        SkipTagId::Boss
    }

    fn name(&self) -> &'static str {
        "Boss"
    }

    fn description(&self) -> &'static str {
        "Re-rolls the next Boss Blind"
    }

    fn effect_type(&self) -> TagEffectType {
        TagEffectType::BossBlindModifier
    }

    fn rarity(&self) -> TagRarity {
        TagRarity::Rare
    }

    fn stackable(&self) -> bool {
        false
    }

    fn activate(&self, context: SkipTagContext) -> SkipTagResult {
        boss_reroll_effect(context)
    }

    fn can_activate(&self, _context: &SkipTagContext) -> bool {
        // Check if there's a boss blind coming up
        // In a full implementation, this would check the upcoming blind schedule
        // For now, always allow activation as the effect is beneficial
        true
    }
}

/// Orbital Tag - Upgrades a specified random Poker Hand by three levels
/// This can include a secret hand if it has been played in the current run
#[derive(Debug)]
pub struct OrbitalTag;

impl SkipTag for OrbitalTag {
    fn id(&self) -> SkipTagId {
        SkipTagId::Orbital
    }

    fn name(&self) -> &'static str {
        "Orbital"
    }

    fn description(&self) -> &'static str {
        "Upgrades a random Poker Hand by 3 levels"
    }

    fn effect_type(&self) -> TagEffectType {
        TagEffectType::ImmediateReward
    }

    fn rarity(&self) -> TagRarity {
        TagRarity::Uncommon
    }

    fn stackable(&self) -> bool {
        false
    }

    fn activate(&self, context: SkipTagContext) -> SkipTagResult {
        hand_upgrade_effect(context, 3)
    }

    fn can_activate(&self, _context: &SkipTagContext) -> bool {
        // Check if there are any hands that can be upgraded
        // In a full implementation, this would check discovered hands
        // For now, always allow activation as there are always upgradeable hands
        true
    }
}

/// Juggle Tag - Adds +3 Hand Size for the next round only
/// Can be stacked multiple times, with each tag adding an additional +3
#[derive(Debug)]
pub struct JuggleTag;

impl SkipTag for JuggleTag {
    fn id(&self) -> SkipTagId {
        SkipTagId::Juggle
    }

    fn name(&self) -> &'static str {
        "Juggle"
    }

    fn description(&self) -> &'static str {
        "Adds +3 Hand Size for the next round only"
    }

    fn effect_type(&self) -> TagEffectType {
        TagEffectType::GameStateModifier
    }

    fn rarity(&self) -> TagRarity {
        TagRarity::Common
    }

    fn stackable(&self) -> bool {
        true
    }

    fn activate(&self, context: SkipTagContext) -> SkipTagResult {
        temporary_hand_size_effect(context, 3)
    }

    fn can_activate(&self, _context: &SkipTagContext) -> bool {
        // Always can be activated
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::Game;
    use crate::stage::Blind;

    fn create_test_context() -> SkipTagContext {
        SkipTagContext {
            game: Game::default(),
            skipped_blind: Some(Blind::Small),
            available_tags: vec![SkipTagId::Boss, SkipTagId::Orbital, SkipTagId::Juggle],
        }
    }

    #[test]
    fn test_double_tag_properties() {
        let tag = DoubleTag;
        assert_eq!(tag.id(), SkipTagId::Double);
        assert_eq!(tag.name(), "Double");
        assert_eq!(tag.effect_type(), TagEffectType::SpecialMechanic);
        assert_eq!(tag.rarity(), TagRarity::Uncommon);
        assert!(tag.stackable());
        assert!(tag.selectable());
    }

    #[test]
    fn test_double_tag_activation() {
        let tag = DoubleTag;
        let context = create_test_context();

        let result = tag.activate(context);

        assert!(result.success);
        assert_eq!(result.additional_tags.len(), 1);
        // Should duplicate the first available non-Double tag
        assert!(result.additional_tags.contains(&SkipTagId::Boss));
    }

    #[test]
    fn test_double_tag_can_activate() {
        let tag = DoubleTag;
        let context = create_test_context();

        assert!(tag.can_activate(&context));

        // Test with only Double tags available
        let context_only_double = SkipTagContext {
            available_tags: vec![SkipTagId::Double],
            ..context
        };
        assert!(!tag.can_activate(&context_only_double));
    }

    #[test]
    fn test_boss_tag_properties() {
        let tag = BossTag;
        assert_eq!(tag.id(), SkipTagId::Boss);
        assert_eq!(tag.name(), "Boss");
        assert_eq!(tag.effect_type(), TagEffectType::BossBlindModifier);
        assert_eq!(tag.rarity(), TagRarity::Rare);
        assert!(!tag.stackable());
    }

    #[test]
    fn test_boss_tag_activation() {
        let tag = BossTag;
        let context = create_test_context();

        let result = tag.activate(context);

        assert!(result.success);
        assert!(result.message.unwrap().contains("Boss Blind"));
    }

    #[test]
    fn test_orbital_tag_properties() {
        let tag = OrbitalTag;
        assert_eq!(tag.id(), SkipTagId::Orbital);
        assert_eq!(tag.name(), "Orbital");
        assert_eq!(tag.effect_type(), TagEffectType::ImmediateReward);
        assert_eq!(tag.rarity(), TagRarity::Uncommon);
        assert!(!tag.stackable());
    }

    #[test]
    fn test_orbital_tag_activation() {
        let tag = OrbitalTag;
        let context = create_test_context();

        let result = tag.activate(context);

        assert!(result.success);
        let message = result.message.unwrap();
        assert!(message.contains("Upgraded"));
        assert!(message.contains("3 levels"));
    }

    #[test]
    fn test_juggle_tag_properties() {
        let tag = JuggleTag;
        assert_eq!(tag.id(), SkipTagId::Juggle);
        assert_eq!(tag.name(), "Juggle");
        assert_eq!(tag.effect_type(), TagEffectType::GameStateModifier);
        assert_eq!(tag.rarity(), TagRarity::Common);
        assert!(tag.stackable());
    }

    #[test]
    fn test_juggle_tag_activation() {
        let tag = JuggleTag;
        let context = create_test_context();

        let result = tag.activate(context);

        assert!(result.success);
        assert!(result.message.unwrap().contains("+3 hand size"));
    }

    #[test]
    fn test_all_tags_can_activate() {
        let context = create_test_context();

        assert!(DoubleTag.can_activate(&context));
        assert!(BossTag.can_activate(&context));
        assert!(OrbitalTag.can_activate(&context));
        assert!(JuggleTag.can_activate(&context));
    }
}
