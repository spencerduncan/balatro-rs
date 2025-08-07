//! Reward Skip Tags
//!
//! Implementation of skip tags that provide immediate rewards:
//! - Charm: Gives a free Mega Arcana Pack
//! - Ethereal: Gives a free Spectral Pack
//! - Buffoon: Gives a free Buffoon Pack
//! - Standard: Gives a free Standard Pack
//! - Meteor: Gives a free Celestial Pack
//! - Rare: Creates a rare Joker
//! - Uncommon: Creates an uncommon Joker
//! - TopUp: Gives up to 2 Common Jokers if you have space

use super::tag_effects::pack_effect;
use super::{SkipTag, SkipTagContext, SkipTagId, SkipTagResult, TagEffectType, TagRarity};
use crate::joker::JokerRarity;
use crate::joker_factory::JokerFactory;
use crate::shop::packs::PackType;

/// Charm Tag: Gives a free Mega Arcana Pack
#[derive(Debug)]
pub struct CharmTag;

impl SkipTag for CharmTag {
    fn id(&self) -> SkipTagId {
        SkipTagId::Charm
    }

    fn name(&self) -> &'static str {
        "Charm"
    }

    fn description(&self) -> &'static str {
        "Gives a free Mega Arcana Pack"
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
        pack_effect(context, PackType::Arcana)
    }
}

/// Ethereal Tag: Gives a free Spectral Pack
#[derive(Debug)]
pub struct EtherealTag;

impl SkipTag for EtherealTag {
    fn id(&self) -> SkipTagId {
        SkipTagId::Ethereal
    }

    fn name(&self) -> &'static str {
        "Ethereal"
    }

    fn description(&self) -> &'static str {
        "Gives a free Spectral Pack"
    }

    fn effect_type(&self) -> TagEffectType {
        TagEffectType::ImmediateReward
    }

    fn rarity(&self) -> TagRarity {
        TagRarity::Rare
    }

    fn stackable(&self) -> bool {
        false
    }

    fn activate(&self, context: SkipTagContext) -> SkipTagResult {
        pack_effect(context, PackType::Spectral)
    }
}

/// Buffoon Tag: Gives a free Buffoon Pack
#[derive(Debug)]
pub struct BuffoonTag;

impl SkipTag for BuffoonTag {
    fn id(&self) -> SkipTagId {
        SkipTagId::Buffoon
    }

    fn name(&self) -> &'static str {
        "Buffoon"
    }

    fn description(&self) -> &'static str {
        "Gives a free Buffoon Pack"
    }

    fn effect_type(&self) -> TagEffectType {
        TagEffectType::ImmediateReward
    }

    fn rarity(&self) -> TagRarity {
        TagRarity::Rare
    }

    fn stackable(&self) -> bool {
        false
    }

    fn activate(&self, context: SkipTagContext) -> SkipTagResult {
        pack_effect(context, PackType::Buffoon)
    }
}

/// Standard Tag: Gives a free Standard Pack
#[derive(Debug)]
pub struct StandardTag;

impl SkipTag for StandardTag {
    fn id(&self) -> SkipTagId {
        SkipTagId::Standard
    }

    fn name(&self) -> &'static str {
        "Standard"
    }

    fn description(&self) -> &'static str {
        "Gives a free Standard Pack"
    }

    fn effect_type(&self) -> TagEffectType {
        TagEffectType::ImmediateReward
    }

    fn rarity(&self) -> TagRarity {
        TagRarity::Common
    }

    fn stackable(&self) -> bool {
        false
    }

    fn activate(&self, context: SkipTagContext) -> SkipTagResult {
        pack_effect(context, PackType::Standard)
    }
}

/// Meteor Tag: Gives a free Celestial Pack
#[derive(Debug)]
pub struct MeteorTag;

impl SkipTag for MeteorTag {
    fn id(&self) -> SkipTagId {
        SkipTagId::Meteor
    }

    fn name(&self) -> &'static str {
        "Meteor"
    }

    fn description(&self) -> &'static str {
        "Gives a free Celestial Pack"
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
        pack_effect(context, PackType::Celestial)
    }
}

/// Rare Tag: Creates a rare Joker
#[derive(Debug)]
pub struct RareTag;

impl SkipTag for RareTag {
    fn id(&self) -> SkipTagId {
        SkipTagId::Rare
    }

    fn name(&self) -> &'static str {
        "Rare"
    }

    fn description(&self) -> &'static str {
        "Creates a rare Joker"
    }

    fn effect_type(&self) -> TagEffectType {
        TagEffectType::ImmediateReward
    }

    fn rarity(&self) -> TagRarity {
        TagRarity::Legendary
    }

    fn stackable(&self) -> bool {
        false
    }

    fn activate(&self, context: SkipTagContext) -> SkipTagResult {
        let mut game = context.game;

        // Get a random rare joker and add it to the game
        let rare_jokers = JokerFactory::get_by_rarity(JokerRarity::Rare);
        if !rare_jokers.is_empty() {
            let joker_id = *game.rng.choose(&rare_jokers).unwrap();
            if let Some(joker) = JokerFactory::create(joker_id) {
                game.jokers.push(joker);
            }
        }

        SkipTagResult {
            game,
            additional_tags: vec![],
            success: true,
            message: Some("Created a rare Joker".to_string()),
        }
    }
}

/// Uncommon Tag: Creates an uncommon Joker
#[derive(Debug)]
pub struct UncommonTag;

impl SkipTag for UncommonTag {
    fn id(&self) -> SkipTagId {
        SkipTagId::Uncommon
    }

    fn name(&self) -> &'static str {
        "Uncommon"
    }

    fn description(&self) -> &'static str {
        "Creates an uncommon Joker"
    }

    fn effect_type(&self) -> TagEffectType {
        TagEffectType::ImmediateReward
    }

    fn rarity(&self) -> TagRarity {
        TagRarity::Rare
    }

    fn stackable(&self) -> bool {
        false
    }

    fn activate(&self, context: SkipTagContext) -> SkipTagResult {
        let mut game = context.game;

        // Get a random uncommon joker and add it to the game
        let uncommon_jokers = JokerFactory::get_by_rarity(JokerRarity::Uncommon);
        if !uncommon_jokers.is_empty() {
            let joker_id = *game.rng.choose(&uncommon_jokers).unwrap();
            if let Some(joker) = JokerFactory::create(joker_id) {
                game.jokers.push(joker);
            }
        }

        SkipTagResult {
            game,
            additional_tags: vec![],
            success: true,
            message: Some("Created an uncommon Joker".to_string()),
        }
    }
}

/// TopUp Tag: Gives up to 2 Common Jokers if you have space
#[derive(Debug)]
pub struct TopUpTag;

impl SkipTag for TopUpTag {
    fn id(&self) -> SkipTagId {
        SkipTagId::TopUp
    }

    fn name(&self) -> &'static str {
        "TopUp"
    }

    fn description(&self) -> &'static str {
        "Gives up to 2 Common Jokers if you have space"
    }

    fn effect_type(&self) -> TagEffectType {
        TagEffectType::ImmediateReward
    }

    fn rarity(&self) -> TagRarity {
        TagRarity::Common
    }

    fn stackable(&self) -> bool {
        false
    }

    fn activate(&self, context: SkipTagContext) -> SkipTagResult {
        let mut game = context.game;

        // Check available joker slots (max 5 jokers)
        let max_jokers: usize = 5;
        let current_joker_count = game.jokers.len();
        let available_slots = max_jokers.saturating_sub(current_joker_count);

        if available_slots == 0 {
            return SkipTagResult {
                game,
                additional_tags: vec![],
                success: false,
                message: Some("No joker slots available".to_string()),
            };
        }

        // TopUp gives up to 2 common jokers, limited by available slots
        let jokers_to_create = std::cmp::min(2, available_slots);
        let common_jokers = JokerFactory::get_by_rarity(JokerRarity::Common);

        let mut created_count = 0;
        for _ in 0..jokers_to_create {
            if !common_jokers.is_empty() && game.jokers.len() < max_jokers {
                let joker_id = *game.rng.choose(&common_jokers).unwrap();
                if let Some(joker) = JokerFactory::create(joker_id) {
                    game.jokers.push(joker);
                    created_count += 1;
                }
            }
        }

        SkipTagResult {
            game,
            additional_tags: vec![],
            success: created_count > 0,
            message: Some(format!("Created {created_count} Common Joker(s)")),
        }
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
            available_tags: vec![],
        }
    }

    #[test]
    fn test_charm_tag_properties() {
        let tag = CharmTag;
        assert_eq!(tag.id(), SkipTagId::Charm);
        assert_eq!(tag.name(), "Charm");
        assert_eq!(tag.effect_type(), TagEffectType::ImmediateReward);
        assert_eq!(tag.rarity(), TagRarity::Uncommon);
        assert!(!tag.stackable());
    }

    #[test]
    fn test_charm_tag_activation() {
        let tag = CharmTag;
        let context = create_test_context();

        let result = tag.activate(context);

        assert!(result.success);
        assert!(result.message.unwrap().contains("Arcana Pack"));
    }

    #[test]
    fn test_ethereal_tag_properties() {
        let tag = EtherealTag;
        assert_eq!(tag.id(), SkipTagId::Ethereal);
        assert_eq!(tag.name(), "Ethereal");
        assert_eq!(tag.effect_type(), TagEffectType::ImmediateReward);
        assert_eq!(tag.rarity(), TagRarity::Rare);
        assert!(!tag.stackable());
    }

    #[test]
    fn test_buffoon_tag_properties() {
        let tag = BuffoonTag;
        assert_eq!(tag.id(), SkipTagId::Buffoon);
        assert_eq!(tag.name(), "Buffoon");
        assert_eq!(tag.effect_type(), TagEffectType::ImmediateReward);
        assert_eq!(tag.rarity(), TagRarity::Rare);
        assert!(!tag.stackable());
    }

    #[test]
    fn test_standard_tag_properties() {
        let tag = StandardTag;
        assert_eq!(tag.id(), SkipTagId::Standard);
        assert_eq!(tag.name(), "Standard");
        assert_eq!(tag.effect_type(), TagEffectType::ImmediateReward);
        assert_eq!(tag.rarity(), TagRarity::Common);
        assert!(!tag.stackable());
    }

    #[test]
    fn test_meteor_tag_properties() {
        let tag = MeteorTag;
        assert_eq!(tag.id(), SkipTagId::Meteor);
        assert_eq!(tag.name(), "Meteor");
        assert_eq!(tag.effect_type(), TagEffectType::ImmediateReward);
        assert_eq!(tag.rarity(), TagRarity::Uncommon);
        assert!(!tag.stackable());
    }

    #[test]
    fn test_rare_tag_properties() {
        let tag = RareTag;
        assert_eq!(tag.id(), SkipTagId::Rare);
        assert_eq!(tag.name(), "Rare");
        assert_eq!(tag.effect_type(), TagEffectType::ImmediateReward);
        assert_eq!(tag.rarity(), TagRarity::Legendary);
        assert!(!tag.stackable());
    }

    #[test]
    fn test_uncommon_tag_properties() {
        let tag = UncommonTag;
        assert_eq!(tag.id(), SkipTagId::Uncommon);
        assert_eq!(tag.name(), "Uncommon");
        assert_eq!(tag.effect_type(), TagEffectType::ImmediateReward);
        assert_eq!(tag.rarity(), TagRarity::Rare);
        assert!(!tag.stackable());
    }

    #[test]
    fn test_topup_tag_properties() {
        let tag = TopUpTag;
        assert_eq!(tag.id(), SkipTagId::TopUp);
        assert_eq!(tag.name(), "TopUp");
        assert_eq!(tag.effect_type(), TagEffectType::ImmediateReward);
        assert_eq!(tag.rarity(), TagRarity::Common);
        assert!(!tag.stackable());
    }

    #[test]
    fn test_all_reward_tags_activation() {
        let charm_result = CharmTag.activate(create_test_context());
        assert!(charm_result.success);

        let ethereal_result = EtherealTag.activate(create_test_context());
        assert!(ethereal_result.success);

        let buffoon_result = BuffoonTag.activate(create_test_context());
        assert!(buffoon_result.success);

        let standard_result = StandardTag.activate(create_test_context());
        assert!(standard_result.success);

        let meteor_result = MeteorTag.activate(create_test_context());
        assert!(meteor_result.success);

        let rare_result = RareTag.activate(create_test_context());
        assert!(rare_result.success);

        let uncommon_result = UncommonTag.activate(create_test_context());
        assert!(uncommon_result.success);

        let topup_result = TopUpTag.activate(create_test_context());
        assert!(topup_result.success);
    }

    #[test]
    fn test_pack_based_rewards() {
        // Test pack-based rewards individually
        let charm_result = CharmTag.activate(create_test_context());
        assert!(charm_result.success);
        assert!(charm_result.message.unwrap().contains("Arcana Pack"));

        let ethereal_result = EtherealTag.activate(create_test_context());
        assert!(ethereal_result.success);
        assert!(ethereal_result.message.unwrap().contains("Spectral Pack"));

        let buffoon_result = BuffoonTag.activate(create_test_context());
        assert!(buffoon_result.success);
        assert!(buffoon_result.message.unwrap().contains("Buffoon Pack"));

        let standard_result = StandardTag.activate(create_test_context());
        assert!(standard_result.success);
        assert!(standard_result.message.unwrap().contains("Standard Pack"));

        let meteor_result = MeteorTag.activate(create_test_context());
        assert!(meteor_result.success);
        assert!(meteor_result.message.unwrap().contains("Celestial Pack"));
    }

    #[test]
    fn test_joker_creation_rewards() {
        let rare_result = RareTag.activate(create_test_context());
        assert!(rare_result.success);
        assert!(rare_result.message.unwrap().contains("rare Joker"));

        let uncommon_result = UncommonTag.activate(create_test_context());
        assert!(uncommon_result.success);
        assert!(uncommon_result.message.unwrap().contains("uncommon Joker"));
    }
}
