use crate::joker::{Joker, JokerId, JokerRarity};
use crate::joker_impl::*;

/// Factory for creating joker instances by ID
pub struct JokerFactory;

impl JokerFactory {
    /// Create a joker instance by its ID
    pub fn create(id: JokerId) -> Option<Box<dyn Joker>> {
        match id {
            JokerId::Joker => Some(Box::new(TheJoker)),
            JokerId::GreedyJoker => Some(Box::new(GreedyJoker)),
            JokerId::LustyJoker => Some(Box::new(LustyJoker)),
            JokerId::WrathfulJoker => Some(Box::new(WrathfulJoker)),
            JokerId::GluttonousJoker => Some(Box::new(GluttonousJoker)),
            JokerId::JollyJoker => Some(Box::new(JollyJoker)),
            JokerId::ZanyJoker => Some(Box::new(ZanyJoker)),
            JokerId::MadJoker => Some(Box::new(MadJoker)),
            JokerId::CrazyJoker => Some(Box::new(CrazyJoker)),
            JokerId::DrollJoker => Some(Box::new(DrollJoker)),
            JokerId::SlyJoker => Some(Box::new(SlyJoker)),
            JokerId::WilyJoker => Some(Box::new(WilyJoker)),
            JokerId::CleverJoker => Some(Box::new(CleverJoker)),
            JokerId::DeviousJoker => Some(Box::new(DeviousJoker)),
            JokerId::CraftyJoker => Some(Box::new(CraftyJoker)),
            // Money-based conditional jokers
            JokerId::BusinessCard => Some(Box::new(BusinessCard)),
            JokerId::EggJoker => Some(Box::new(Egg)),
            JokerId::Burglar => Some(Box::new(Burglar)),
            // Hand type conditional jokers
            JokerId::Runner => Some(Box::new(RunnerJoker)),
            JokerId::Supernova => Some(Box::new(SupernovaJoker)),
            JokerId::SpaceJoker => Some(Box::new(SpaceJoker)),
            JokerId::IceCream => Some(Box::new(IceCreamJoker::new())),
            // TODO: Implement remaining jokers
            _ => None,
        }
    }

    /// Get all joker IDs by rarity
    pub fn get_by_rarity(rarity: JokerRarity) -> Vec<JokerId> {
        use JokerId::*;

        match rarity {
            JokerRarity::Common => vec![
                Joker,
                GreedyJoker,
                LustyJoker,
                WrathfulJoker,
                GluttonousJoker,
                JollyJoker,
                ZanyJoker,
                MadJoker,
                CrazyJoker,
                DrollJoker,
                SlyJoker,
                WilyJoker,
                CleverJoker,
                DeviousJoker,
                CraftyJoker,
                // Money-based conditional jokers
                BusinessCard,
                EggJoker,
                // Hand type conditional jokers
                Runner,
                Supernova,
                IceCream,
                // Add more common jokers here
            ],
            JokerRarity::Uncommon => vec![
                // Money-based conditional jokers
                Burglar,
                // Hand type conditional jokers
                SpaceJoker,
                // TODO: Add more uncommon jokers
            ],
            JokerRarity::Rare => vec![
                // TODO: Add rare jokers
            ],
            JokerRarity::Legendary => vec![
                // TODO: Add legendary jokers
            ],
        }
    }

    /// Get all implemented joker IDs
    pub fn get_all_implemented() -> Vec<JokerId> {
        use JokerId::*;
        vec![
            Joker,
            GreedyJoker,
            LustyJoker,
            WrathfulJoker,
            GluttonousJoker,
            JollyJoker,
            ZanyJoker,
            MadJoker,
            CrazyJoker,
            DrollJoker,
            SlyJoker,
            WilyJoker,
            CleverJoker,
            DeviousJoker,
            CraftyJoker,
            // Money-based conditional jokers
            BusinessCard,
            EggJoker,
            Burglar,
            // Hand type conditional jokers
            Runner,
            Supernova,
            SpaceJoker,
            IceCream,
        ]
    }
}
