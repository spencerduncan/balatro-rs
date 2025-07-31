//! Economic Skip Tags
//!
//! Implementation of the 5 economic category skip tags:
//! - Economy: Doubles your money (max +$40), $0 if negative balance  
//! - Investment: Gives $25 after defeating the next Boss Blind (stackable)
//! - Garbage: Gains $1 for each unused discard this run (retroactive)
//! - Speed: Gives $5 for each Blind skipped this run (minimum $5)
//! - Handy: Gains $1 for each hand played this run (retroactive)

use super::tag_effects::money_effect;
use super::{SkipTag, SkipTagContext, SkipTagId, SkipTagResult, TagEffectType, TagRarity};

/// Economy Tag - Doubles your money (max +$40), $0 if negative balance
#[derive(Debug)]
pub struct EconomyTag;

impl SkipTag for EconomyTag {
    fn id(&self) -> SkipTagId {
        SkipTagId::Economy
    }

    fn name(&self) -> &'static str {
        "Economy"
    }

    fn description(&self) -> &'static str {
        "Doubles your money (max +$40)"
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
        if context.game.money <= 0.0 {
            SkipTagResult {
                game: context.game,
                additional_tags: vec![],
                success: true,
                message: Some("Economy Tag: No money to double (negative balance)".to_string()),
            }
        } else {
            let current_money = context.game.money as i64;
            let doubled_amount = current_money * 2;
            let reward = (doubled_amount - current_money).min(40); // Max +$40

            let mut result = money_effect(context, reward);
            result.message = Some(format!("Economy Tag: +${reward} from doubling money"));
            result
        }
    }
}

/// Investment Tag - Gives $25 after defeating the next Boss Blind (stackable)
#[derive(Debug)]
pub struct InvestmentTag;

impl SkipTag for InvestmentTag {
    fn id(&self) -> SkipTagId {
        SkipTagId::Investment
    }

    fn name(&self) -> &'static str {
        "Investment"
    }

    fn description(&self) -> &'static str {
        "Gain $25 after defeating the next Boss Blind"
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
        // TODO: Store investment count in game state for triggering on boss blind defeat
        SkipTagResult {
            game: context.game,
            additional_tags: vec![],
            success: true,
            message: Some("Investment Tag: Will gain $25 after next Boss Blind defeat".to_string()),
        }
    }
}

/// Garbage Tag - Gains $1 for each unused discard this run (retroactive)
#[derive(Debug)]
pub struct GarbageTag;

impl SkipTag for GarbageTag {
    fn id(&self) -> SkipTagId {
        SkipTagId::Garbage
    }

    fn name(&self) -> &'static str {
        "Garbage"
    }

    fn description(&self) -> &'static str {
        "Gain $1 for each unused discard this run"
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
        // Calculate unused discards: total discards available minus discards used
        let total_discards_available = context.game.plays as i64; // Rough estimate
        let discards_used = context.game.discards as i64;
        let unused_discards = (total_discards_available - discards_used).max(0);

        let mut result = money_effect(context, unused_discards);
        result.message = Some(format!(
            "Garbage Tag: +${unused_discards} from {unused_discards} unused discards"
        ));
        result
    }
}

/// Speed Tag - Gives $5 for each Blind skipped this run (minimum $5)
#[derive(Debug)]
pub struct SpeedTag;

impl SkipTag for SpeedTag {
    fn id(&self) -> SkipTagId {
        SkipTagId::Speed
    }

    fn name(&self) -> &'static str {
        "Speed"
    }

    fn description(&self) -> &'static str {
        "Gain $5 for each Blind you've skipped this run (min $5)"
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
        // TODO: Get actual skipped blinds count from game state
        let blinds_skipped = 1; // Placeholder
        let reward = (blinds_skipped * 5).max(5); // Minimum $5

        let mut result = money_effect(context, reward);
        result.message = Some(format!(
            "Speed Tag: +${reward} from {blinds_skipped} blind(s) skipped (min $5)"
        ));
        result
    }
}

/// Handy Tag - Gains $1 for each hand played this run (retroactive)
#[derive(Debug)]
pub struct HandyTag;

impl SkipTag for HandyTag {
    fn id(&self) -> SkipTagId {
        SkipTagId::Handy
    }

    fn name(&self) -> &'static str {
        "Handy"
    }

    fn description(&self) -> &'static str {
        "Gain $1 for each hand played this run"
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
        let hands_played = context.game.plays as i64;

        let mut result = money_effect(context, hands_played);
        result.message = Some(format!(
            "Handy Tag: +${hands_played} from {hands_played} hands played"
        ));
        result
    }
}

