/// Skip tag implementations
/// 
/// This module contains all skip tag implementations. During Phase 1 infrastructure
/// development, these are stub implementations that allow the tag selection system
/// to be tested. In Phase 2, these will be replaced with full implementations.

use crate::skip_tags::tag_trait::{StubTag, TagEffectType, TagId};

/// Create a stub tag implementation for the given tag ID
/// 
/// This function provides stub implementations for all skip tags to enable
/// infrastructure development and testing. Each stub contains the correct
/// metadata but performs no actual game effects.
pub fn create_stub_tag(tag_id: TagId) -> StubTag {
    match tag_id {
        // Reward Tags (8)
        TagId::Charm => StubTag::new(
            TagId::Charm,
            "Charm",
            TagEffectType::ImmediateReward,
            "Creates 2 playing cards to add to your deck",
        ),
        TagId::Ethereal => StubTag::new(
            TagId::Ethereal,
            "Ethereal",
            TagEffectType::ImmediateReward,
            "Creates 1 Spectral pack",
        ),
        TagId::Buffoon => StubTag::new(
            TagId::Buffoon,
            "Buffoon",
            TagEffectType::ImmediateReward,
            "Creates 1 Joker pack",
        ),
        TagId::Standard => StubTag::new(
            TagId::Standard,
            "Standard",
            TagEffectType::ImmediateReward,
            "Creates 1 Standard pack",
        ),
        TagId::Meteor => StubTag::new(
            TagId::Meteor,
            "Meteor",
            TagEffectType::ImmediateReward,
            "Creates 1 Planet pack",
        ),
        TagId::Rare => StubTag::new(
            TagId::Rare,
            "Rare",
            TagEffectType::ImmediateReward,
            "Creates 1 Rare Joker pack",
        ),
        TagId::Uncommon => StubTag::new(
            TagId::Uncommon,
            "Uncommon",
            TagEffectType::ImmediateReward,
            "Creates 1 Uncommon Joker pack",
        ),
        TagId::TopUp => StubTag::new(
            TagId::TopUp,
            "Top Up",
            TagEffectType::ImmediateReward,
            "Creates 1 Mega pack (most common consumable)",
        ),
        
        // Economic Tags (5)
        TagId::Economy => StubTag::new(
            TagId::Economy,
            "Economy",
            TagEffectType::ImmediateReward,
            "Gain +$1 interest for every $4 you have (max of +$5)",
        ),
        TagId::Investment => StubTag::new(
            TagId::Investment,
            "Investment",
            TagEffectType::ImmediateReward,
            "Earn $8 immediately",
        ),
        TagId::Garbage => StubTag::new(
            TagId::Garbage,
            "Garbage",
            TagEffectType::ImmediateReward,
            "Earn $1 for each unused hand",
        ),
        TagId::Speed => StubTag::new(
            TagId::Speed,
            "Speed",
            TagEffectType::ImmediateReward,
            "Earn $1 for each unused discard",
        ),
        TagId::Handy => StubTag::new(
            TagId::Handy,
            "Handy",
            TagEffectType::ImmediateReward,
            "Earn $1 for each unused hand and discard",
        ),
        
        // Shop Enhancement Tags (6)
        TagId::Voucher => StubTag::new(
            TagId::Voucher,
            "Voucher",
            TagEffectType::NextShopModifier,
            "Adds a Voucher to the next shop",
        ),
        TagId::Coupon => StubTag::new(
            TagId::Coupon,
            "Coupon",
            TagEffectType::NextShopModifier,
            "Next shop reroll is free",
        ),
        TagId::Foil => StubTag::new(
            TagId::Foil,
            "Foil",
            TagEffectType::NextShopModifier,
            "Next shop has a free Foil Joker",
        ),
        TagId::Holographic => StubTag::new(
            TagId::Holographic,
            "Holographic",
            TagEffectType::NextShopModifier,
            "Next shop has a free Holographic Joker",
        ),
        TagId::Polychrome => StubTag::new(
            TagId::Polychrome,
            "Polychrome",
            TagEffectType::NextShopModifier,
            "Next shop has a free Polychrome Joker",
        ),
        TagId::Negative => StubTag::new(
            TagId::Negative,
            "Negative",
            TagEffectType::NextShopModifier,
            "Next shop has a free Negative Joker",
        ),
        
        // Utility Tags (4)
        TagId::Double => StubTag::new(
            TagId::Double,
            "Double",
            TagEffectType::SpecialMechanic,
            "Creates a copy of next selected tag (stacking effect)",
        ),
        TagId::Boss => StubTag::new(
            TagId::Boss,
            "Boss",
            TagEffectType::SpecialMechanic,
            "Rerolls the current Boss Blind",
        ),
        TagId::Orbital => StubTag::new(
            TagId::Orbital,
            "Orbital",
            TagEffectType::GameStateModifier,
            "Upgrade 2 random poker hands by 1 level each",
        ),
        TagId::Juggle => StubTag::new(
            TagId::Juggle,
            "Juggle",
            TagEffectType::GameStateModifier,
            "Ability to remove up to 5 selected cards from deck",
        ),
        
        // Compatibility Tags (3)
        TagId::D6 => StubTag::new(
            TagId::D6,
            "D6",
            TagEffectType::ImmediateReward,
            "Creates 1 to 6 random consumables (roll dice)",
        ),
    }
}

/// Get tag rarity for selection probability weighting
/// 
/// This function will be used by the tag selection system to weight
/// the probability of different tags appearing.
pub fn get_tag_rarity(tag_id: TagId) -> TagRarity {
    match tag_id {
        // Common tags (higher probability)
        TagId::Standard | TagId::Investment | TagId::Garbage | TagId::Speed 
        | TagId::Handy | TagId::Coupon => TagRarity::Common,
        
        // Uncommon tags (medium probability)
        TagId::Charm | TagId::Buffoon | TagId::Meteor | TagId::Uncommon 
        | TagId::Economy | TagId::Voucher | TagId::Foil => TagRarity::Uncommon,
        
        // Rare tags (lower probability)
        TagId::Ethereal | TagId::Rare | TagId::TopUp | TagId::Holographic 
        | TagId::Polychrome | TagId::Orbital | TagId::Juggle => TagRarity::Rare,
        
        // Very rare tags (very low probability)
        TagId::Negative | TagId::Double | TagId::Boss | TagId::D6 => TagRarity::VeryRare,
    }
}

/// Tag rarity levels for selection probability
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TagRarity {
    Common,     // 40% base probability
    Uncommon,   // 30% base probability  
    Rare,       // 20% base probability
    VeryRare,   // 10% base probability
}

impl TagRarity {
    /// Get the base selection probability for this rarity
    pub fn base_probability(&self) -> f32 {
        match self {
            TagRarity::Common => 0.40,
            TagRarity::Uncommon => 0.30,
            TagRarity::Rare => 0.20,
            TagRarity::VeryRare => 0.10,
        }
    }
    
    /// Get the relative weight for weighted random selection
    pub fn selection_weight(&self) -> f32 {
        match self {
            TagRarity::Common => 4.0,
            TagRarity::Uncommon => 3.0,
            TagRarity::Rare => 2.0,
            TagRarity::VeryRare => 1.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::skip_tags::tag_trait::SkipTag;
    
    #[test]
    fn test_create_stub_tag_all_ids() {
        // Ensure we can create stub tags for all tag IDs
        for &tag_id in TagId::all() {
            let stub_tag = create_stub_tag(tag_id);
            assert_eq!(stub_tag.id(), tag_id);
            assert!(!stub_tag.name().is_empty());
            assert!(!stub_tag.description().is_empty());
        }
    }
    
    #[test]
    fn test_tag_rarity_probabilities() {
        // Verify that all probabilities sum to reasonable ranges
        let common_prob = TagRarity::Common.base_probability();
        let uncommon_prob = TagRarity::Uncommon.base_probability();
        let rare_prob = TagRarity::Rare.base_probability();
        let very_rare_prob = TagRarity::VeryRare.base_probability();
        
        let total = common_prob + uncommon_prob + rare_prob + very_rare_prob;
        assert!((total - 1.0).abs() < 0.001); // Should sum to 1.0
    }
    
    #[test]
    fn test_get_tag_rarity_all_ids() {
        // Ensure all tag IDs have assigned rarities
        for &tag_id in TagId::all() {
            let _rarity = get_tag_rarity(tag_id);
            // Test passes if no panic occurs
        }
    }
}