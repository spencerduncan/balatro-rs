//! Economic Skip Tags
//!
//! Implementation of the 5 economic category skip tags:
//! - Economy: Doubles your money (max +$40), $0 if negative balance  
//! - Investment: Gives $25 after defeating the next Boss Blind (stackable)
//! - Garbage: Gains $1 for each unused discard this run (retroactive)
//! - Speed: Gives $5 for each Blind skipped this run (minimum $5)
//! - Handy: Gains $1 for each hand played this run (retroactive)

use super::tag_effects::money_effect;
use super::{
    SkipTag, SkipTagContext, SkipTagId, SkipTagResult, TagEffectData, TagEffectResult,
    TagEffectType, TagRarity,
};
use crate::game::Game;

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

    fn apply_effect(&self, game: &mut Game) -> TagEffectResult {
        let reward = if game.money <= 0.0 {
            0
        } else {
            let current_money = game.money as i64;
            let doubled_amount = current_money * 2;
            (doubled_amount - current_money).min(40) // Max +$40
        };

        TagEffectResult {
            success: true,
            message: Some(if reward == 0 {
                "Economy Tag: No money to double (negative balance)".to_string()
            } else {
                format!("Economy Tag: +${reward} from doubling money")
            }),
            data: TagEffectData::Money(reward as f64),
            money_reward: reward,
            persist_tag: false,
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

    fn activate(&self, mut context: SkipTagContext) -> SkipTagResult {
        // Increment investment count in game state for triggering on boss blind defeat
        context.game.active_skip_tags.investment_count += 1;

        let count = context.game.active_skip_tags.investment_count;
        let total_payout = count * 25;

        SkipTagResult {
            game: context.game,
            additional_tags: vec![],
            success: true,
            message: Some(format!(
                "Investment Tag: Will gain ${total_payout} after next Boss Blind defeat ({count} investment(s))"
            )),
        }
    }

    fn apply_effect(&self, game: &mut Game) -> TagEffectResult {
        // Increment investment count for boss blind payout
        game.active_skip_tags.investment_count += 1;
        let count = game.active_skip_tags.investment_count;
        let total_payout = count * 25;

        TagEffectResult {
            success: true,
            message: Some(format!(
                "Investment Tag: Will gain ${total_payout} after next Boss Blind defeat ({count} investment(s))"
            )),
            data: TagEffectData::None,
            money_reward: 0, // No immediate reward
            persist_tag: false,
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
        // Calculate unused discards: discards are the remaining discards (unused)
        // Total discards used = starting discards - remaining discards
        let starting_discards = context.game.config.discards as i64;
        let remaining_discards = context.game.discards as i64;
        let _discards_used = starting_discards - remaining_discards;

        // For retroactive calculation, we want the total discards used across the run
        // The reward is based on unused discards, which is simply the remaining discards
        let reward = remaining_discards.max(0);

        let mut result = money_effect(context, reward);
        result.message = Some(format!(
            "Garbage Tag: +${reward} from {reward} unused discards"
        ));
        result
    }

    fn apply_effect(&self, game: &mut Game) -> TagEffectResult {
        // Calculate unused discards: remaining discards
        let remaining_discards = game.discards as i64;
        let reward = remaining_discards.max(0);

        TagEffectResult {
            success: true,
            message: Some(format!(
                "Garbage Tag: +${reward} from {reward} unused discards"
            )),
            data: TagEffectData::Money(reward as f64),
            money_reward: reward,
            persist_tag: false,
        }
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
        // Get actual skipped blinds count from active skip tags state
        let blinds_skipped = context.game.active_skip_tags.blinds_skipped as i64;
        let reward = (blinds_skipped * 5).max(5); // Minimum $5

        let mut result = money_effect(context, reward);
        result.message = Some(format!(
            "Speed Tag: +${reward} from {blinds_skipped} blind(s) skipped (min $5)"
        ));
        result
    }

    fn apply_effect(&self, game: &mut Game) -> TagEffectResult {
        // Get actual skipped blinds count from active skip tags state
        let blinds_skipped = game.active_skip_tags.blinds_skipped as i64;
        let reward = (blinds_skipped * 5).max(5); // Minimum $5

        TagEffectResult {
            success: true,
            message: Some(format!(
                "Speed Tag: +${reward} from {blinds_skipped} blind(s) skipped (min $5)"
            )),
            data: TagEffectData::Money(reward as f64),
            money_reward: reward,
            persist_tag: false,
        }
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

    fn apply_effect(&self, game: &mut Game) -> TagEffectResult {
        let hands_played = game.plays as i64;

        TagEffectResult {
            success: true,
            message: Some(format!(
                "Handy Tag: +${hands_played} from {hands_played} hands played"
            )),
            data: TagEffectData::Money(hands_played as f64),
            money_reward: hands_played,
            persist_tag: false,
        }
    }
}
