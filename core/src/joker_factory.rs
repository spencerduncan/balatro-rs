use crate::joker::basic_economy_jokers::{
    DelayedGratificationJoker, GiftCardJoker, RocketJoker, ToTheMoonJoker,
};
use crate::joker::four_fingers::FourFingersJoker;
use crate::joker::multiplicative_jokers::AcrobatJoker;
use crate::joker::resource_chips_jokers::ScaryFaceJoker;
use crate::joker::retrigger_jokers::*;
use crate::joker::scaling_additive_mult_jokers::*;
use crate::joker::scaling_chips_jokers::*;
use crate::joker::scaling_xmult_jokers::*;
use crate::joker::steel_joker_composition::SteelJoker;
use crate::joker::{Joker, JokerId, JokerRarity};
use crate::joker_impl::*;
use crate::scaling_joker_custom;
use crate::scaling_joker_impl::get_scaling_joker_by_id;
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
            // Hand-type jokers using StaticJoker framework (Clean Code: eliminate duplication)
            JokerId::JollyJoker => Some(StaticJokerFactory::create_jolly_joker()),
            JokerId::ZanyJoker => Some(StaticJokerFactory::create_zany_joker()),
            JokerId::MadJoker => Some(StaticJokerFactory::create_mad_joker()),
            JokerId::CrazyJoker => Some(StaticJokerFactory::create_crazy_joker()),
            JokerId::DrollJoker => Some(StaticJokerFactory::create_droll_joker()),
            JokerId::SlyJoker => Some(StaticJokerFactory::create_sly_joker()),
            JokerId::WilyJoker => Some(StaticJokerFactory::create_wily_joker()),
            JokerId::CleverJoker => Some(StaticJokerFactory::create_clever_joker()),
            JokerId::DeviousJoker => Some(StaticJokerFactory::create_devious_joker()),
            JokerId::CraftyJoker => Some(StaticJokerFactory::create_crafty_joker()),

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
            // Note: RedCard scaling version is handled by Reserved6 below
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
            JokerId::SteelJoker => Some(Box::new(SteelJoker::new())),
            JokerId::ScaryFace => Some(Box::new(ScaryFaceJoker::new())),

            // RNG-based jokers (Issue #442)
            JokerId::Oops => Some(Box::new(OopsAllSixesJoker)),
            JokerId::Reserved7 => Some(Box::new(SixShooterJoker)),
            JokerId::LuckyCharm => Some(Box::new(LuckyCardJoker)),
            JokerId::Reserved8 => Some(Box::new(GrimJoker)),
            JokerId::AcrobatJoker => Some(Box::new(AcrobatJoker::new())),
            JokerId::FortuneTeller => Some(Box::new(FortuneTellerJoker::new())),
            JokerId::Reserved4 => Some(Box::new(MysteryJoker)),
            JokerId::VagabondJoker => Some(Box::new(VagabondJokerImpl)),
            JokerId::Reserved9 => Some(Box::new(ChaoticJoker)),

            // Special mechanic jokers using new trait system
            JokerId::Erosion => Some(Box::new(ErosionJoker)),
            JokerId::Blueprint => Some(Box::new(BlueprintJoker::new())),
            JokerId::Photograph => Some(Box::new(PhotographJoker::new())),
            JokerId::TheOrder => Some(Box::new(TheOrderJoker)),
            JokerId::FourFingers => Some(Box::new(FourFingersJoker::new())),
            JokerId::Triboulet => Some(Box::new(TribouletJoker)),

            // Economy jokers
            JokerId::DelayedGratification => Some(Box::new(DelayedGratificationJoker::new())),
            JokerId::RocketShip => Some(Box::new(RocketJoker::new())),
            JokerId::ToTheMoon => Some(Box::new(ToTheMoonJoker::new())),
            JokerId::GiftCard => Some(Box::new(GiftCardJoker::new())),

            // Scaling additive mult jokers
            JokerId::Trousers => Some(Box::new(SpareTrousersJoker::new())),
            JokerId::GreenJoker => Some(Box::new(GreenJoker::new())),
            JokerId::Reserved5 => Some(Box::new(RideTheBusJoker::new())), // RideTheBus
            JokerId::Reserved6 => Some(Box::new(RedCardJoker::new())),    // RedCard (pack skipping)

            // Scaling chips jokers
            JokerId::Castle => Some(Box::new(CastleJoker::new())),
            JokerId::Wee => Some(Box::new(WeeJoker::new())),
            JokerId::Stuntman => Some(Box::new(StuntmanJoker::new())),
            JokerId::Hiker => Some(Box::new(HikerJoker::new())),
            JokerId::OddTodd => Some(Box::new(OddToddJoker::new())),
            JokerId::Arrowhead => Some(Box::new(ArrowheadJoker::new())),
            JokerId::Scholar => Some(Box::new(ScholarJoker::new())),

            // Scaling xmult jokers
            JokerId::Reserved => Some(Box::new(ThrowbackJoker::new())), // Throwback
            JokerId::Ceremonial => Some(Box::new(CeremonialDaggerJoker::new())),

            // Custom scaling jokers with complex logic
            JokerId::BullMarket => {
                scaling_joker_custom::get_custom_scaling_joker_by_id(JokerId::BullMarket)
            }
            JokerId::Bootstraps => {
                scaling_joker_custom::get_custom_scaling_joker_by_id(JokerId::Bootstraps)
            }
            JokerId::Reserved2 => {
                scaling_joker_custom::get_custom_scaling_joker_by_id(JokerId::Reserved2)
            } // Mystic Summit

            // Basic scaling jokers (when no trait-based alternative exists)
            JokerId::MarbleJoker => {
                get_scaling_joker_by_id(JokerId::MarbleJoker).map(|j| Box::new(j) as Box<dyn Joker>)
            }
            JokerId::Loyalty => {
                get_scaling_joker_by_id(JokerId::Loyalty).map(|j| Box::new(j) as Box<dyn Joker>)
            }

            // Retrigger jokers
            JokerId::Dusk => Some(Box::new(DuskJoker::new())),
            JokerId::Seltzer => Some(Box::new(SeltzerJoker::new())),
            JokerId::Hanging => Some(Box::new(HangingChadJoker::new())),
            JokerId::SockAndBuskin => Some(Box::new(SockAndBuskinJoker::new())),

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
                // Scaling chips jokers
                OddTodd,
                Arrowhead,
                Scholar,
                // Resource chips jokers
                ScaryFace, // ScaryFace Joker
                // Scaling xmult jokers (none in common)
                // Retrigger jokers
                Hanging, // HangingChadJoker
            ],
            JokerRarity::Uncommon => vec![
                // Money-based conditional jokers
                Burglar,    // Hand type conditional jokers
                SpaceJoker, // New static jokers
                BlueJoker,
                SteelJoker, // Scaling Steel Joker
                // RNG-based jokers (Issue #442)
                Oops,      // OopsAllSixesJoker
                Reserved8, // GrimJoker
                VagabondJoker,
                // Special mechanic jokers
                TheOrder,
                FourFingers,
                // Scaling additive mult jokers
                Trousers, // Spare Trousers
                // Scaling chips jokers
                Hiker,
                // Scaling xmult jokers
                Reserved,   // Throwback
                Ceremonial, // Ceremonial Dagger
                // Retrigger jokers
                Dusk,
                Seltzer,
                SockAndBuskin,
            ],
            JokerRarity::Rare => vec![
                // RNG-based jokers (Issue #442)
                AcrobatJoker,
                Reserved4, // Mystery Joker
                // Special mechanic jokers
                Blueprint,
                // Scaling mult jokers
                FortuneTeller, // Fortune Teller
                // Scaling chips jokers
                Castle,
                Wee,
                Stuntman,
                // Custom scaling jokers
                Reserved2, // Mystic Summit
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
            Reserved4, // MysteryJoker
            VagabondJoker,
            Reserved9, // ChaoticJoker
            // Special mechanic jokers using new trait system
            Erosion,
            Blueprint,
            Photograph,
            TheOrder,
            FourFingers,
            Triboulet, // Legendary joker - Kings and Queens give X2 mult
            // Scaling additive mult jokers
            Trousers, // Spare Trousers
            GreenJoker,
            Reserved5,     // RideTheBus
            Reserved6,     // RedCard (pack skipping)
            FortuneTeller, // Fortune Teller
            // Scaling chips jokers
            Castle,
            Wee,
            Stuntman,
            Hiker,
            OddTodd,
            Arrowhead,
            Scholar,
            // Resource chips jokers
            ScaryFace, // ScaryFace Joker
            // Scaling xmult jokers
            SteelJoker, // Scaling Steel Joker
            Reserved,   // Throwback
            Ceremonial, // Ceremonial Dagger
            // Custom scaling jokers
            BullMarket, // Bull Joker
            Bootstraps, // Bootstraps Joker
            Reserved2,  // Mystic Summit
            // Basic scaling jokers
            MarbleJoker,
            Loyalty,
            // Retrigger jokers
            Dusk,
            Seltzer,
            Hanging,
            SockAndBuskin,
            // Note: HalfJoker and Banner are still placeholders
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_new_static_jokers() {
        // Test fully implemented static jokers
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

        let scary_face = JokerFactory::create(JokerId::ScaryFace);
        assert!(scary_face.is_some());
        assert_eq!(scary_face.unwrap().id(), JokerId::ScaryFace);
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
        assert!(common_jokers.contains(&JokerId::ScaryFace));

        let uncommon_jokers = JokerFactory::get_by_rarity(JokerRarity::Uncommon);
        assert!(uncommon_jokers.contains(&JokerId::BlueJoker));
        assert!(uncommon_jokers.contains(&JokerId::SteelJoker));
        // Note: RedCard is now a scaling joker (Reserved6) handled separately
    }

    #[test]
    fn test_new_jokers_in_implemented_list() {
        let implemented = JokerFactory::get_all_implemented();

        // Fully implemented jokers should be in the list
        assert!(implemented.contains(&JokerId::BlueJoker));
        assert!(implemented.contains(&JokerId::FacelessJoker));
        assert!(implemented.contains(&JokerId::Square));
        assert!(implemented.contains(&JokerId::Walkie));
        assert!(implemented.contains(&JokerId::Runner));
        assert!(implemented.contains(&JokerId::AbstractJoker));
        assert!(implemented.contains(&JokerId::ScaryFace));

        // AbstractJoker is now properly implemented and included above
        // Note: Placeholder jokers (HalfJoker, Banner)
        // are intentionally not in get_all_implemented() as they're not complete
    }

    #[test]
    fn test_scaling_jokers_integration() {
        // Test scaling additive mult jokers
        let trousers = JokerFactory::create(JokerId::Trousers);
        assert!(trousers.is_some());
        assert_eq!(trousers.unwrap().id(), JokerId::Trousers);

        let green_joker = JokerFactory::create(JokerId::GreenJoker);
        assert!(green_joker.is_some());
        assert_eq!(green_joker.unwrap().id(), JokerId::GreenJoker);

        let ride_the_bus = JokerFactory::create(JokerId::Reserved5);
        assert!(ride_the_bus.is_some());
        assert_eq!(ride_the_bus.unwrap().id(), JokerId::Reserved5);

        let red_card = JokerFactory::create(JokerId::Reserved6);
        assert!(red_card.is_some());
        assert_eq!(red_card.unwrap().id(), JokerId::Reserved6);

        let fortune_teller = JokerFactory::create(JokerId::FortuneTeller);
        assert!(fortune_teller.is_some());
        assert_eq!(fortune_teller.unwrap().id(), JokerId::FortuneTeller);

        // Test scaling chips jokers
        let castle = JokerFactory::create(JokerId::Castle);
        assert!(castle.is_some());
        assert_eq!(castle.unwrap().id(), JokerId::Castle);

        let wee = JokerFactory::create(JokerId::Wee);
        assert!(wee.is_some());
        assert_eq!(wee.unwrap().id(), JokerId::Wee);

        let stuntman = JokerFactory::create(JokerId::Stuntman);
        assert!(stuntman.is_some());
        assert_eq!(stuntman.unwrap().id(), JokerId::Stuntman);

        let hiker = JokerFactory::create(JokerId::Hiker);
        assert!(hiker.is_some());
        assert_eq!(hiker.unwrap().id(), JokerId::Hiker);

        let odd_todd = JokerFactory::create(JokerId::OddTodd);
        assert!(odd_todd.is_some());
        assert_eq!(odd_todd.unwrap().id(), JokerId::OddTodd);

        let arrowhead = JokerFactory::create(JokerId::Arrowhead);
        assert!(arrowhead.is_some());
        assert_eq!(arrowhead.unwrap().id(), JokerId::Arrowhead);

        let scholar = JokerFactory::create(JokerId::Scholar);
        assert!(scholar.is_some());
        assert_eq!(scholar.unwrap().id(), JokerId::Scholar);

        // Test scaling xmult jokers
        let steel_joker = JokerFactory::create(JokerId::SteelJoker);
        assert!(steel_joker.is_some());
        assert_eq!(steel_joker.unwrap().id(), JokerId::SteelJoker);

        let throwback = JokerFactory::create(JokerId::Reserved);
        assert!(throwback.is_some());
        assert_eq!(throwback.unwrap().id(), JokerId::Reserved);

        let ceremonial = JokerFactory::create(JokerId::Ceremonial);
        assert!(ceremonial.is_some());
        assert_eq!(ceremonial.unwrap().id(), JokerId::Ceremonial);

        // Test custom scaling jokers
        let bull_market = JokerFactory::create(JokerId::BullMarket);
        assert!(bull_market.is_some());
        assert_eq!(bull_market.unwrap().id(), JokerId::BullMarket);

        let bootstraps = JokerFactory::create(JokerId::Bootstraps);
        assert!(bootstraps.is_some());
        assert_eq!(bootstraps.unwrap().id(), JokerId::Bootstraps);

        let mystic_summit = JokerFactory::create(JokerId::Reserved2);
        assert!(mystic_summit.is_some());
        assert_eq!(mystic_summit.unwrap().id(), JokerId::Reserved2);

        // Test legacy scaling jokers
        let marble_joker = JokerFactory::create(JokerId::MarbleJoker);
        assert!(marble_joker.is_some());
        assert_eq!(marble_joker.unwrap().id(), JokerId::MarbleJoker);

        let loyalty = JokerFactory::create(JokerId::Loyalty);
        assert!(loyalty.is_some());
        assert_eq!(loyalty.unwrap().id(), JokerId::Loyalty);
    }

    #[test]
    fn test_scaling_jokers_in_rarity_lists() {
        let common_jokers = JokerFactory::get_by_rarity(JokerRarity::Common);
        // Common scaling jokers
        assert!(common_jokers.contains(&JokerId::GreenJoker));
        assert!(common_jokers.contains(&JokerId::Reserved5)); // RideTheBus
        assert!(common_jokers.contains(&JokerId::Reserved6)); // RedCard (pack skipping)
        assert!(common_jokers.contains(&JokerId::OddTodd));
        assert!(common_jokers.contains(&JokerId::Arrowhead));
        assert!(common_jokers.contains(&JokerId::Scholar));

        let uncommon_jokers = JokerFactory::get_by_rarity(JokerRarity::Uncommon);
        // Uncommon scaling jokers
        assert!(uncommon_jokers.contains(&JokerId::Trousers)); // Spare Trousers
        assert!(uncommon_jokers.contains(&JokerId::SteelJoker));
        assert!(uncommon_jokers.contains(&JokerId::Hiker));
        assert!(uncommon_jokers.contains(&JokerId::Reserved)); // Throwback
        assert!(uncommon_jokers.contains(&JokerId::Ceremonial)); // Ceremonial Dagger

        let rare_jokers = JokerFactory::get_by_rarity(JokerRarity::Rare);
        // Rare scaling jokers
        assert!(rare_jokers.contains(&JokerId::FortuneTeller)); // Fortune Teller
        assert!(rare_jokers.contains(&JokerId::Castle));
        assert!(rare_jokers.contains(&JokerId::Wee));
        assert!(rare_jokers.contains(&JokerId::Stuntman));
        assert!(rare_jokers.contains(&JokerId::Reserved2)); // Mystic Summit
    }

    #[test]
    fn test_scaling_jokers_in_implemented_list() {
        let implemented = JokerFactory::get_all_implemented();

        // All integrated scaling jokers should be in implemented list
        // Scaling additive mult jokers
        assert!(implemented.contains(&JokerId::Trousers));
        assert!(implemented.contains(&JokerId::GreenJoker));
        assert!(implemented.contains(&JokerId::Reserved5));
        assert!(implemented.contains(&JokerId::Reserved6));
        assert!(implemented.contains(&JokerId::FortuneTeller));

        // Scaling chips jokers
        assert!(implemented.contains(&JokerId::Castle));
        assert!(implemented.contains(&JokerId::Wee));
        assert!(implemented.contains(&JokerId::Stuntman));
        assert!(implemented.contains(&JokerId::Hiker));
        assert!(implemented.contains(&JokerId::OddTodd));
        assert!(implemented.contains(&JokerId::Arrowhead));
        assert!(implemented.contains(&JokerId::Scholar));

        // Scaling xmult jokers
        assert!(implemented.contains(&JokerId::SteelJoker));
        assert!(implemented.contains(&JokerId::Reserved));
        assert!(implemented.contains(&JokerId::Ceremonial));

        // Custom scaling jokers
        assert!(implemented.contains(&JokerId::BullMarket));
        assert!(implemented.contains(&JokerId::Bootstraps));
        assert!(implemented.contains(&JokerId::Reserved2));

        // Legacy scaling jokers
        assert!(implemented.contains(&JokerId::MarbleJoker));
        assert!(implemented.contains(&JokerId::Loyalty));
    }
}
