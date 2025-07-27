use crate::joker::four_fingers::FourFingersJoker;
use crate::joker::scaling_additive_mult_jokers::*;
use crate::joker::{Joker, JokerId, JokerRarity};
use crate::joker_impl::*;
use crate::scaling_joker_custom;
use crate::scaling_joker_impl::{create_red_card, create_steel_joker_scaling};
use crate::special_jokers::*;
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

            // Money-based conditional jokers
            JokerId::BusinessCard => Some(Box::new(BusinessCard)),
            JokerId::EggJoker => Some(Box::new(Egg)),
            JokerId::Burglar => Some(Box::new(Burglar)),

            // Hand type conditional jokers from main branch
            JokerId::Supernova => Some(Box::new(SupernovaJoker)),
            JokerId::SpaceJoker => Some(Box::new(SpaceJoker)),
            JokerId::IceCream => Some(Box::new(IceCreamJoker)),
            JokerId::Runner => Some(Box::new(RunnerJoker)),

            // Static jokers from StaticJokerFactory
            JokerId::RedCard => Some(Box::new(create_red_card())),
            JokerId::BlueJoker => Some(StaticJokerFactory::create_blue_joker()),
            JokerId::FacelessJoker => Some(StaticJokerFactory::create_faceless_joker()),
            JokerId::Square => {
                scaling_joker_custom::get_custom_scaling_joker_by_id(JokerId::Square)
            }
            JokerId::Walkie => Some(StaticJokerFactory::create_walkie()),

            // Placeholder jokers with TODO comments
            JokerId::HalfJoker => Some(StaticJokerFactory::create_half_joker()),
            JokerId::Banner => Some(StaticJokerFactory::create_banner()),
            JokerId::AbstractJoker => Some(Box::new(AbstractJoker)),
            JokerId::SteelJoker => Some(Box::new(create_steel_joker_scaling())),

            // RNG-based jokers (Issue #442)
            JokerId::Oops => Some(Box::new(OopsAllSixesJoker)),
            JokerId::Reserved7 => Some(Box::new(SixShooterJoker)),
            JokerId::LuckyCharm => Some(Box::new(LuckyCardJoker)),
            JokerId::Reserved8 => Some(Box::new(GrimJoker)),
            JokerId::AcrobatJoker => Some(Box::new(AcrobatJokerImpl)),
            JokerId::Fortune => Some(create_fortune_teller()),
            JokerId::VagabondJoker => Some(Box::new(VagabondJokerImpl)),
            JokerId::Reserved9 => Some(Box::new(ChaoticJoker)),

            // Special mechanic jokers using new trait system
            JokerId::Erosion => Some(Box::new(ErosionJoker)),
            JokerId::Blueprint => Some(Box::new(BlueprintJoker::new())),
            JokerId::Photograph => Some(Box::new(PhotographJoker::new())),
            JokerId::TheOrder => Some(Box::new(TheOrderJoker)),
            JokerId::FourFingers => Some(Box::new(FourFingersJoker::new())),
            JokerId::Triboulet => Some(Box::new(TribouletJoker)),

            // Scaling additive mult jokers
            JokerId::Trousers => Some(Box::new(SpareTrousersJoker::new())),
            JokerId::GreenJoker => Some(Box::new(GreenJoker::new())),
            JokerId::Reserved5 => Some(Box::new(RideTheBusJoker::new())), // RideTheBus
            JokerId::Reserved6 => Some(Box::new(RedCardJoker::new())),    // RedCard (pack skipping)
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
                Supernova,
                IceCream,
                Runner,
                // New static jokers
                FacelessJoker,
                Square,
                Walkie,
                HalfJoker,
                Banner,
                AbstractJoker,
                // RNG-based jokers (Issue #442)
                Reserved7,  // SixShooterJoker
                LuckyCharm, // LuckyCardJoker
                // Special mechanic jokers
                Erosion,
                Photograph,
                // Scaling additive mult jokers
                GreenJoker,
                Reserved5, // RideTheBus
                Reserved6, // RedCard (pack skipping)
            ],
            JokerRarity::Uncommon => vec![
                // Money-based conditional jokers
                Burglar,    // Hand type conditional jokers
                SpaceJoker, // New static jokers
                RedCard,
                BlueJoker,
                SteelJoker,
                // RNG-based jokers (Issue #442)
                Oops,      // OopsAllSixesJoker
                Reserved8, // GrimJoker
                VagabondJoker,
                // Special mechanic jokers
                TheOrder,
                FourFingers,
                // Scaling additive mult jokers
                Trousers, // Spare Trousers
            ],
            JokerRarity::Rare => vec![
                // RNG-based jokers (Issue #442)
                AcrobatJoker,
                Fortune, // MysteryJoker
                // Special mechanic jokers
                Blueprint,
            ],
            JokerRarity::Legendary => vec![
                // RNG-based jokers (Issue #442)
                Reserved9, // ChaoticJoker
                Triboulet, // Legendary joker that gives X2 mult for Kings and Queens
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
            Supernova,
            SpaceJoker,
            IceCream,
            Runner,
            // New fully implemented static jokers
            RedCard,
            BlueJoker,
            FacelessJoker,
            Square,
            Walkie,
            // Custom implemented jokers with interaction support
            AbstractJoker,
            // RNG-based jokers (Issue #442)
            Oops,       // OopsAllSixesJoker
            Reserved7,  // SixShooterJoker
            LuckyCharm, // LuckyCardJoker
            Reserved8,  // GrimJoker
            AcrobatJoker,
            Fortune, // MysteryJoker
            VagabondJoker,
            Reserved9, // ChaoticJoker
            // Special mechanic jokers using new trait system
            Erosion,
            Blueprint,
            Photograph,
            TheOrder,
            SteelJoker, // Now properly implemented as scaling joker
            FourFingers,
            Triboulet, // Legendary joker - Kings and Queens give X2 mult
            // Scaling additive mult jokers
            Trousers, // Spare Trousers
            GreenJoker,
            Reserved5, // RideTheBus
            Reserved6, // RedCard (pack skipping)
                       // Note: HalfJoker and Banner are still placeholders
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
        assert!(implemented.contains(&JokerId::AbstractJoker));

        // AbstractJoker is now properly implemented and included above
        // Note: Placeholder jokers (HalfJoker, Banner, SteelJoker)
        // are intentionally not in get_all_implemented() as they're not complete
    }
}
