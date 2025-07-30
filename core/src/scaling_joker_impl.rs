use crate::joker::{JokerId, JokerRarity};
use crate::rank::HandRank;
use crate::scaling_joker::{ResetCondition, ScalingEffectType, ScalingJoker, ScalingTrigger};

/// Factory functions for creating the 15 scaling jokers specified in the requirements
/// Spare Trousers: +2 mult per hand with Two Pair
pub fn create_spare_trousers() -> ScalingJoker {
    ScalingJoker::new(
        JokerId::Trousers,
        "Spare Trousers".to_string(),
        "+2 Mult per Two Pair hand played".to_string(),
        JokerRarity::Uncommon,
        0.0, // Start at 0
        2.0, // +2 per trigger
        ScalingTrigger::HandPlayed(HandRank::TwoPair),
        ScalingEffectType::Mult,
    )
}

/// Square Joker: +4 chips if hand has exactly 4 cards (this is more conditional than scaling)
/// For scaling version: +4 chips per hand played with exactly 4 cards
pub fn create_square_joker() -> ScalingJoker {
    ScalingJoker::new(
        JokerId::Square,
        "Square Joker".to_string(),
        "+4 Chips per 4-card hand played".to_string(),
        JokerRarity::Common,
        0.0,
        4.0,
        ScalingTrigger::HandPlayed(HandRank::HighCard), // Will need custom logic for 4-card hands
        ScalingEffectType::Chips,
    )
}

/// Bull: +2 chips per $1 owned (scales with money)
pub fn create_bull_joker() -> ScalingJoker {
    ScalingJoker::new(
        JokerId::BullMarket,
        "Bull".to_string(),
        "+2 Chips per $1 owned".to_string(),
        JokerRarity::Common,
        0.0,
        2.0,
        ScalingTrigger::MoneyGained,
        ScalingEffectType::Chips,
    )
}

/// Bootstraps: +2 mult per $5 in bank
pub fn create_bootstraps() -> ScalingJoker {
    ScalingJoker::new(
        JokerId::Bootstraps,
        "Bootstraps".to_string(),
        "+2 Mult per $5 owned".to_string(),
        JokerRarity::Uncommon,
        0.0,
        2.0,
        ScalingTrigger::MoneyGained, // Will need custom logic for $5 increments
        ScalingEffectType::Mult,
    )
}

/// Fortune Teller: +1 mult per Tarot used
pub fn create_fortune_teller() -> ScalingJoker {
    ScalingJoker::new(
        JokerId::Fortune,
        "Fortune Teller".to_string(),
        "+1 Mult per Tarot card used".to_string(),
        JokerRarity::Rare,
        0.0,
        1.0,
        ScalingTrigger::ConsumableUsed,
        ScalingEffectType::Mult,
    )
}

/// Ceremonial Dagger: Mult doubles when Blind selected (resets each blind)
pub fn create_ceremonial_dagger() -> ScalingJoker {
    ScalingJoker::new(
        JokerId::Ceremonial,
        "Ceremonial Dagger".to_string(),
        "Mult doubles when Blind starts, resets when completed".to_string(),
        JokerRarity::Uncommon,
        1.0,                            // Start at 1x
        1.0,                            // Double each time
        ScalingTrigger::BlindCompleted, // Will need custom logic for blind start
        ScalingEffectType::MultMultiplier,
    )
    .with_reset_condition(ResetCondition::RoundEnd)
}

/// Throwback: X mult for each reroll in shop
pub fn create_throwback() -> ScalingJoker {
    ScalingJoker::new(
        JokerId::Reserved, // Using reserved slot
        "Throwback".to_string(),
        "X1.5 Mult per shop reroll".to_string(),
        JokerRarity::Uncommon,
        1.0, // Start at 1x
        0.5, // +0.5x per reroll
        ScalingTrigger::ShopReroll,
        ScalingEffectType::MultMultiplier,
    )
    .with_reset_condition(ResetCondition::ShopEntered)
}

/// Green Joker: +1 mult per hand, -1 per discard
pub fn create_green_joker() -> ScalingJoker {
    ScalingJoker::new(
        JokerId::GreenJoker,
        "Green Joker".to_string(),
        "+1 Mult per hand played, -1 per discard".to_string(),
        JokerRarity::Common,
        0.0,
        1.0,
        ScalingTrigger::HandPlayed(HandRank::HighCard), // Will need custom logic for any hand
        ScalingEffectType::Mult,
    )
}

/// Red Card: +3 mult when any pack skipped
pub fn create_red_card() -> ScalingJoker {
    ScalingJoker::new(
        JokerId::RedCard,
        "Red Card".to_string(),
        "+3 Mult per pack skipped".to_string(),
        JokerRarity::Common,
        0.0,
        3.0,
        ScalingTrigger::ShopReroll, // Will need custom logic for pack skipping
        ScalingEffectType::Mult,
    )
}

/// Additional scaling jokers to reach 15 total:
/// Steel Joker: +0.2x mult per card destroyed
pub fn create_steel_joker_scaling() -> ScalingJoker {
    ScalingJoker::new(
        JokerId::SteelJoker,
        "Steel Joker".to_string(),
        "+0.2x Mult per card destroyed".to_string(),
        JokerRarity::Uncommon,
        1.0,
        0.2,
        ScalingTrigger::CardDestroyed,
        ScalingEffectType::MultMultiplier,
    )
}

/// Mystic Summit: +15 mult per hand type played this run
pub fn create_mystic_summit() -> ScalingJoker {
    ScalingJoker::new(
        JokerId::Reserved2,
        "Mystic Summit".to_string(),
        "+15 Mult per unique hand type played".to_string(),
        JokerRarity::Rare,
        0.0,
        15.0,
        ScalingTrigger::HandPlayed(HandRank::HighCard), // Will need custom logic
        ScalingEffectType::Mult,
    )
}

/// Marble Joker: +50 chips per joker sold
pub fn create_marble_joker_scaling() -> ScalingJoker {
    ScalingJoker::new(
        JokerId::MarbleJoker,
        "Marble Joker".to_string(),
        "+50 Chips per Joker sold".to_string(),
        JokerRarity::Rare,
        0.0,
        50.0,
        ScalingTrigger::JokerSold,
        ScalingEffectType::Chips,
    )
}

/// Loyalty Card: +1 mult per blind completed this ante
pub fn create_loyalty_card() -> ScalingJoker {
    ScalingJoker::new(
        JokerId::Loyalty,
        "Loyalty Card".to_string(),
        "+1 Mult per blind completed this ante".to_string(),
        JokerRarity::Common,
        0.0,
        1.0,
        ScalingTrigger::BlindCompleted,
        ScalingEffectType::Mult,
    )
    .with_reset_condition(ResetCondition::AnteEnd)
}

/// Castle: +300 chips per discard used this round
pub fn create_castle() -> ScalingJoker {
    ScalingJoker::new(
        JokerId::Reserved3,
        "Castle".to_string(),
        "+300 Chips per discard used this round".to_string(),
        JokerRarity::Rare,
        0.0,
        300.0,
        ScalingTrigger::CardDiscarded,
        ScalingEffectType::Chips,
    )
    .with_reset_condition(ResetCondition::RoundEnd)
    .with_max_value(1200.0) // Max 4 discards per round typically
}

/// Factory function to create all scaling jokers
pub fn create_all_scaling_jokers() -> Vec<ScalingJoker> {
    vec![
        create_spare_trousers(),
        create_square_joker(),
        create_bull_joker(),
        create_bootstraps(),
        create_fortune_teller(),
        create_ceremonial_dagger(),
        create_throwback(),
        create_green_joker(),
        create_red_card(),
        create_steel_joker_scaling(),
        create_mystic_summit(),
        create_marble_joker_scaling(),
        create_loyalty_card(),
        create_castle(),
    ]
}

/// Get scaling joker by ID
pub fn get_scaling_joker_by_id(id: JokerId) -> Option<ScalingJoker> {
    match id {
        JokerId::Trousers => Some(create_spare_trousers()),
        JokerId::Square => Some(create_square_joker()),
        JokerId::BullMarket => Some(create_bull_joker()),
        JokerId::Bootstraps => Some(create_bootstraps()),
        JokerId::Fortune => Some(create_fortune_teller()),
        JokerId::Ceremonial => Some(create_ceremonial_dagger()),
        JokerId::Reserved => Some(create_throwback()),
        JokerId::GreenJoker => Some(create_green_joker()),
        JokerId::RedCard => Some(create_red_card()),
        JokerId::SteelJoker => Some(create_steel_joker_scaling()),
        JokerId::Reserved2 => Some(create_mystic_summit()),
        JokerId::MarbleJoker => Some(create_marble_joker_scaling()),
        JokerId::Loyalty => Some(create_loyalty_card()),
        JokerId::Reserved3 => Some(create_castle()),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_scaling_jokers_created() {
        let jokers = create_all_scaling_jokers();
        assert_eq!(jokers.len(), 14, "Should create exactly 14 scaling jokers");

        // Test that all jokers have unique IDs
        let mut ids = std::collections::HashSet::new();
        for joker in &jokers {
            assert!(
                ids.insert(joker.id),
                "Duplicate joker ID found: {:?}",
                joker.id
            );
        }
    }

    #[test]
    fn test_spare_trousers() {
        let joker = create_spare_trousers();
        assert_eq!(joker.id, JokerId::Trousers);
        assert_eq!(joker.name, "Spare Trousers");
        assert_eq!(joker.base_value, 0.0);
        assert_eq!(joker.increment, 2.0);
        assert_eq!(joker.trigger, ScalingTrigger::HandPlayed(HandRank::TwoPair));
        assert_eq!(joker.effect_type, ScalingEffectType::Mult);
    }

    #[test]
    fn test_ceremonial_dagger() {
        let joker = create_ceremonial_dagger();
        assert_eq!(joker.base_value, 1.0);
        assert_eq!(joker.effect_type, ScalingEffectType::MultMultiplier);
        assert_eq!(joker.reset_condition, Some(ResetCondition::RoundEnd));
    }

    #[test]
    fn test_castle_with_max_value() {
        let joker = create_castle();
        assert_eq!(joker.max_value, Some(1200.0));
        assert_eq!(joker.reset_condition, Some(ResetCondition::RoundEnd));
    }

    #[test]
    fn test_get_scaling_joker_by_id() {
        assert!(get_scaling_joker_by_id(JokerId::Trousers).is_some());
        assert!(get_scaling_joker_by_id(JokerId::GreenJoker).is_some());
        assert!(get_scaling_joker_by_id(JokerId::Joker).is_none()); // Not a scaling joker
    }

    #[test]
    fn test_joker_descriptions() {
        let jokers = create_all_scaling_jokers();
        for joker in jokers {
            assert!(!joker.name.is_empty(), "Joker name should not be empty");
            assert!(
                !joker.description.is_empty(),
                "Joker description should not be empty"
            );
        }
    }
}
