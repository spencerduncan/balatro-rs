use crate::joker::{Joker, JokerId, JokerRarity};
use crate::joker_impl::*;
use crate::static_joker_factory::StaticJokerFactory;

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
            
            // Jokers from main branch
            JokerId::Supernova => Some(Box::new(SupernovaJoker)),
            JokerId::SpaceJoker => Some(Box::new(SpaceJoker)),
            
            // Static jokers from StaticJokerFactory
            JokerId::RedCard => Some(StaticJokerFactory::create_red_card()),
            JokerId::BlueJoker => Some(StaticJokerFactory::create_blue_joker()),
            JokerId::FacelessJoker => Some(StaticJokerFactory::create_faceless_joker()),
            JokerId::Square => Some(StaticJokerFactory::create_square()),
            JokerId::Walkie => Some(StaticJokerFactory::create_walkie()),
            JokerId::Runner => Some(Box::new(RunnerJoker)),
            
            // Placeholder jokers with TODO comments
            JokerId::HalfJoker => Some(StaticJokerFactory::create_half_joker()),
            JokerId::Banner => Some(StaticJokerFactory::create_banner()),
            JokerId::AbstractJoker => Some(StaticJokerFactory::create_abstract_joker()),
            JokerId::SteelJoker => Some(StaticJokerFactory::create_steel_joker()),
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
                Supernova,
                // New static jokers
                FacelessJoker,
                Square,
                Walkie,
                Runner,
                HalfJoker,
                Banner,
                AbstractJoker,
            ],
            JokerRarity::Uncommon => vec![
                SpaceJoker,
                // New static jokers
                RedCard,
                BlueJoker,
                SteelJoker,
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
            Supernova,
            SpaceJoker,
            // New fully implemented static jokers
            RedCard,
            BlueJoker,
            FacelessJoker,
            Square,
            Walkie,
            Runner,
            // Note: HalfJoker, Banner, AbstractJoker, and SteelJoker are placeholders
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_new_static_jokers() {
        // Test fully implemented jokers
        let red_card = JokerFactory::create(JokerId::RedCard);
        assert!(red_card.is_some());
        assert_eq!(red_card.unwrap().id(), JokerId::RedCard);

        let blue_joker = JokerFactory::create(JokerId::BlueJoker);
        assert!(blue_joker.is_some());
        assert_eq!(blue_joker.unwrap().id(), JokerId::BlueJoker);

        let faceless = JokerFactory::create(JokerId::FacelessJoker);
        assert!(faceless.is_some());
        assert_eq!(faceless.unwrap().id(), JokerId::FacelessJoker);

        let square = JokerFactory::create(JokerId::Square);
        assert!(square.is_some());
        assert_eq!(square.unwrap().id(), JokerId::Square);

        let walkie = JokerFactory::create(JokerId::Walkie);
        assert!(walkie.is_some());
        assert_eq!(walkie.unwrap().id(), JokerId::Walkie);

        let runner = JokerFactory::create(JokerId::Runner);
        assert!(runner.is_some());
        assert_eq!(runner.unwrap().id(), JokerId::Runner);
    }

    #[test]
    fn test_create_placeholder_jokers() {
        // Test placeholder jokers are created (even though they don't work correctly yet)
        let half = JokerFactory::create(JokerId::HalfJoker);
        assert!(half.is_some());
        assert_eq!(half.unwrap().id(), JokerId::HalfJoker);

        let banner = JokerFactory::create(JokerId::Banner);
        assert!(banner.is_some());
        assert_eq!(banner.unwrap().id(), JokerId::Banner);

        let abstract_joker = JokerFactory::create(JokerId::AbstractJoker);
        assert!(abstract_joker.is_some());
        assert_eq!(abstract_joker.unwrap().id(), JokerId::AbstractJoker);

        let steel = JokerFactory::create(JokerId::SteelJoker);
        assert!(steel.is_some());
        assert_eq!(steel.unwrap().id(), JokerId::SteelJoker);
    }

    #[test]
    fn test_new_jokers_in_rarity_lists() {
        let common_jokers = JokerFactory::get_by_rarity(JokerRarity::Common);
        assert!(common_jokers.contains(&JokerId::FacelessJoker));
        assert!(common_jokers.contains(&JokerId::Square));
        assert!(common_jokers.contains(&JokerId::Walkie));
        assert!(common_jokers.contains(&JokerId::Runner));
        assert!(common_jokers.contains(&JokerId::HalfJoker));
        assert!(common_jokers.contains(&JokerId::Banner));
        assert!(common_jokers.contains(&JokerId::AbstractJoker));

        let uncommon_jokers = JokerFactory::get_by_rarity(JokerRarity::Uncommon);
        assert!(uncommon_jokers.contains(&JokerId::RedCard));
        assert!(uncommon_jokers.contains(&JokerId::BlueJoker));
        assert!(uncommon_jokers.contains(&JokerId::SteelJoker));
    }

    #[test]
    fn test_new_jokers_in_implemented_list() {
        let implemented = JokerFactory::get_all_implemented();
        
        // Fully implemented jokers should be in the list
        assert!(implemented.contains(&JokerId::RedCard));
        assert!(implemented.contains(&JokerId::BlueJoker));
        assert!(implemented.contains(&JokerId::FacelessJoker));
        assert!(implemented.contains(&JokerId::Square));
        assert!(implemented.contains(&JokerId::Walkie));
        assert!(implemented.contains(&JokerId::Runner));
        
        // Note: Placeholder jokers (HalfJoker, Banner, AbstractJoker, SteelJoker)
        // are intentionally not in get_all_implemented() as they're not complete
    }
}
