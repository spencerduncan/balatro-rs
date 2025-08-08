use crate::card::{Suit, Value};
use crate::joker::{Joker, JokerId, JokerRarity};
use crate::joker_json_parameters::JsonParameterResolver;
use crate::rank::HandRank;
use crate::static_joker::{FrameworkStaticJoker, StaticCondition};

/// Factory functions for creating static jokers using the framework
pub struct StaticJokerFactory;

impl StaticJokerFactory {
    /// Create the basic Joker (+4 Mult)
    pub fn create_joker() -> Box<dyn Joker> {
        Box::new(
            FrameworkStaticJoker::builder(JokerId::Joker, "Joker", "+4 Mult")
                .rarity(JokerRarity::Common)
                .cost(2)
                .mult(4)
                .per_hand()
                .build()
                .expect("Valid joker configuration"),
        )
    }

    /// Create Greedy Joker (Diamond cards give +3 Mult when scored)
    pub fn create_greedy_joker() -> Box<dyn Joker> {
        Box::new(
            FrameworkStaticJoker::builder(
                JokerId::GreedyJoker,
                "Greedy Joker",
                "Played cards with Diamond suit give +3 Mult when scored",
            )
            .rarity(JokerRarity::Common)
            .cost(5)
            .mult(3)
            .condition(StaticCondition::SuitScored(Suit::Diamond))
            .per_card()
            .build()
            .expect("Valid joker configuration"),
        )
    }

    /// Create Lusty Joker (Heart cards give +3 Mult when scored)
    pub fn create_lusty_joker() -> Box<dyn Joker> {
        Box::new(
            FrameworkStaticJoker::builder(
                JokerId::LustyJoker,
                "Lusty Joker",
                "Played cards with Heart suit give +3 Mult when scored",
            )
            .rarity(JokerRarity::Common)
            .cost(5)
            .mult(3)
            .condition(StaticCondition::SuitScored(Suit::Heart))
            .per_card()
            .build()
            .expect("Valid joker configuration"),
        )
    }

    /// Create Wrathful Joker (Spade cards give +3 Mult when scored)
    pub fn create_wrathful_joker() -> Box<dyn Joker> {
        Box::new(
            FrameworkStaticJoker::builder(
                JokerId::WrathfulJoker,
                "Wrathful Joker",
                "Played cards with Spade suit give +3 Mult when scored",
            )
            .rarity(JokerRarity::Common)
            .cost(5)
            .mult(3)
            .condition(StaticCondition::SuitScored(Suit::Spade))
            .per_card()
            .build()
            .expect("Valid joker configuration"),
        )
    }

    /// Create Gluttonous Joker (Club cards give +3 Mult when scored)
    pub fn create_gluttonous_joker() -> Box<dyn Joker> {
        Box::new(
            FrameworkStaticJoker::builder(
                JokerId::GluttonousJoker,
                "Gluttonous Joker",
                "Played cards with Club suit give +3 Mult when scored",
            )
            .rarity(JokerRarity::Common)
            .cost(5)
            .mult(3)
            .condition(StaticCondition::SuitScored(Suit::Club))
            .per_card()
            .build()
            .expect("Valid joker configuration"),
        )
    }

    /// Create Jolly Joker (+8 Mult if played hand contains Pair)
    pub fn create_jolly_joker() -> Box<dyn Joker> {
        Box::new(
            FrameworkStaticJoker::builder(
                JokerId::JollyJoker,
                "Jolly Joker",
                "+8 Mult if played hand contains Pair",
            )
            .rarity(JokerRarity::Common)
            .cost(3)
            .mult(8)
            .condition(StaticCondition::HandType(HandRank::OnePair))
            .per_hand()
            .build()
            .expect("Valid joker configuration"),
        )
    }

    /// Create Zany Joker (+12 Mult if played hand contains Three of a Kind)
    pub fn create_zany_joker() -> Box<dyn Joker> {
        Box::new(
            FrameworkStaticJoker::builder(
                JokerId::ZanyJoker,
                "Zany Joker",
                "+12 Mult if played hand contains Three of a Kind",
            )
            .rarity(JokerRarity::Common)
            .cost(4)
            .mult(12)
            .condition(StaticCondition::HandType(HandRank::ThreeOfAKind))
            .per_hand()
            .build()
            .expect("Valid joker configuration"),
        )
    }

    /// Create Mad Joker (+10 Mult if played hand contains Two Pair)
    pub fn create_mad_joker() -> Box<dyn Joker> {
        Box::new(
            FrameworkStaticJoker::builder(
                JokerId::MadJoker,
                "Mad Joker",
                "+10 Mult if played hand contains Two Pair",
            )
            .rarity(JokerRarity::Common)
            .cost(4)
            .mult(10)
            .condition(StaticCondition::HandType(HandRank::TwoPair))
            .per_hand()
            .build()
            .expect("Valid joker configuration"),
        )
    }

    /// Create Crazy Joker (+12 Mult if played hand contains Straight)
    pub fn create_crazy_joker() -> Box<dyn Joker> {
        Box::new(
            FrameworkStaticJoker::builder(
                JokerId::CrazyJoker,
                "Crazy Joker",
                "+12 Mult if played hand contains Straight",
            )
            .rarity(JokerRarity::Common)
            .cost(4)
            .mult(12)
            .condition(StaticCondition::HandType(HandRank::Straight))
            .per_hand()
            .build()
            .expect("Valid joker configuration"),
        )
    }

    /// Create Droll Joker (+10 Mult if played hand contains Flush)
    pub fn create_droll_joker() -> Box<dyn Joker> {
        Box::new(
            FrameworkStaticJoker::builder(
                JokerId::DrollJoker,
                "Droll Joker",
                "+10 Mult if played hand contains Flush",
            )
            .rarity(JokerRarity::Common)
            .cost(4)
            .mult(10)
            .condition(StaticCondition::HandType(HandRank::Flush))
            .per_hand()
            .build()
            .expect("Valid joker configuration"),
        )
    }

    /// Create Sly Joker (+50 Chips if played hand contains Pair)
    pub fn create_sly_joker() -> Box<dyn Joker> {
        Box::new(
            FrameworkStaticJoker::builder(
                JokerId::SlyJoker,
                "Sly Joker",
                "+50 Chips if played hand contains Pair",
            )
            .rarity(JokerRarity::Common)
            .cost(3)
            .chips(50)
            .condition(StaticCondition::HandType(HandRank::OnePair))
            .per_hand()
            .build()
            .expect("Valid joker configuration"),
        )
    }

    /// Create Wily Joker (+100 Chips if played hand contains Three of a Kind)
    pub fn create_wily_joker() -> Box<dyn Joker> {
        Box::new(
            FrameworkStaticJoker::builder(
                JokerId::WilyJoker,
                "Wily Joker",
                "+100 Chips if played hand contains Three of a Kind",
            )
            .rarity(JokerRarity::Common)
            .cost(4)
            .chips(100)
            .condition(StaticCondition::HandType(HandRank::ThreeOfAKind))
            .per_hand()
            .build()
            .expect("Valid joker configuration"),
        )
    }

    /// Create Clever Joker (+80 Chips if played hand contains Two Pair)
    pub fn create_clever_joker() -> Box<dyn Joker> {
        Box::new(
            FrameworkStaticJoker::builder(
                JokerId::CleverJoker,
                "Clever Joker",
                "+80 Chips if played hand contains Two Pair",
            )
            .rarity(JokerRarity::Common)
            .cost(4)
            .chips(80)
            .condition(StaticCondition::HandType(HandRank::TwoPair))
            .per_hand()
            .build()
            .expect("Valid joker configuration"),
        )
    }

    /// Create Devious Joker (+100 Chips if played hand contains Straight)
    pub fn create_devious_joker() -> Box<dyn Joker> {
        Box::new(
            FrameworkStaticJoker::builder(
                JokerId::DeviousJoker,
                "Devious Joker",
                "+100 Chips if played hand contains Straight",
            )
            .rarity(JokerRarity::Common)
            .cost(4)
            .chips(100)
            .condition(StaticCondition::HandType(HandRank::Straight))
            .per_hand()
            .build()
            .expect("Valid joker configuration"),
        )
    }

    /// Create Crafty Joker (+80 Chips if played hand contains Flush)
    pub fn create_crafty_joker() -> Box<dyn Joker> {
        Box::new(
            FrameworkStaticJoker::builder(
                JokerId::CraftyJoker,
                "Crafty Joker",
                "+80 Chips if played hand contains Flush",
            )
            .rarity(JokerRarity::Common)
            .cost(4)
            .chips(80)
            .condition(StaticCondition::HandType(HandRank::Flush))
            .per_hand()
            .build()
            .expect("Valid joker configuration"),
        )
    }

    /// Create Even Steven (Even cards (2, 4, 6, 8, 10) give +4 Mult when scored)
    pub fn create_even_steven() -> Box<dyn Joker> {
        Box::new(
            FrameworkStaticJoker::builder(
                JokerId::EvenSteven,
                "Even Steven",
                "Played cards with even rank (2, 4, 6, 8, 10) give +4 Mult when scored",
            )
            .rarity(JokerRarity::Common)
            .cost(4)
            .mult(4)
            .condition(StaticCondition::AnyRankScored(vec![
                Value::Two,
                Value::Four,
                Value::Six,
                Value::Eight,
                Value::Ten,
            ]))
            .per_card()
            .build()
            .expect("Valid joker configuration"),
        )
    }

    /// Create Odd Todd (Odd cards (3, 5, 7, 9, A) give +31 Chips when scored)
    pub fn create_odd_todd() -> Box<dyn Joker> {
        Box::new(
            FrameworkStaticJoker::builder(
                JokerId::OddTodd,
                "Odd Todd",
                "Played cards with odd rank (3, 5, 7, 9, A) give +31 Chips when scored",
            )
            .rarity(JokerRarity::Common)
            .cost(4)
            .chips(31)
            .condition(StaticCondition::AnyRankScored(vec![
                Value::Three,
                Value::Five,
                Value::Seven,
                Value::Nine,
                Value::Ace,
            ]))
            .per_card()
            .build()
            .expect("Valid joker configuration"),
        )
    }

    /// Create Scholar (Aces give +20 Chips and +4 Mult when scored)
    pub fn create_scholar() -> Box<dyn Joker> {
        Box::new(
            FrameworkStaticJoker::builder(
                JokerId::Scholar,
                "Scholar",
                "Played Aces give +20 Chips and +4 Mult when scored",
            )
            .rarity(JokerRarity::Common)
            .cost(4)
            .chips(20)
            .mult(4)
            .condition(StaticCondition::RankScored(Value::Ace))
            .per_card()
            .build()
            .expect("Valid joker configuration"),
        )
    }

    /// Create Red Card (Red cards give +3 Mult when scored)
    pub fn create_red_card() -> Box<dyn Joker> {
        Box::new(
            FrameworkStaticJoker::builder(
                JokerId::RedCard,
                "Red Card",
                "Red cards (Hearts and Diamonds) give +3 Mult when scored",
            )
            .rarity(JokerRarity::Uncommon)
            .cost(6)
            .mult(3)
            .condition(StaticCondition::AnySuitScored(vec![
                Suit::Heart,
                Suit::Diamond,
            ]))
            .per_card()
            .build()
            .expect("Valid joker configuration"),
        )
    }

    /// Create Blue Joker (+2 Chips per remaining card in deck)
    pub fn create_blue_joker() -> Box<dyn Joker> {
        Box::new(
            FrameworkStaticJoker::builder(
                JokerId::BlueJoker,
                "Blue Joker",
                "+2 Chips per remaining card in deck",
            )
            .rarity(JokerRarity::Common)
            .cost(3)
            .chips(2) // Base value, multiplied by deck size in create_effect_with_context
            .condition(StaticCondition::DeckSize)
            .per_hand()
            .build()
            .expect("Valid joker configuration"),
        )
    }

    /// Create Faceless Joker (Face cards give +5 Mult when scored)
    pub fn create_faceless_joker() -> Box<dyn Joker> {
        Box::new(
            FrameworkStaticJoker::builder(
                JokerId::FacelessJoker,
                "Faceless Joker",
                "Face cards (Jack, Queen, King) give +5 Mult when scored",
            )
            .rarity(JokerRarity::Common)
            .cost(3)
            .mult(5)
            .condition(StaticCondition::AnyRankScored(vec![
                Value::Jack,
                Value::Queen,
                Value::King,
            ]))
            .per_card()
            .build()
            .expect("Valid joker configuration"),
        )
    }

    /// Create Scary Face (Face cards give +30 Chips when scored)
    pub fn create_scary_face() -> Box<dyn Joker> {
        // Load parameter from joker.json, fallback to 30 if not found
        let chips_bonus = Self::load_scary_face_parameter();

        Box::new(
            FrameworkStaticJoker::builder(
                JokerId::ScaryFace,
                "Scary Face",
                "Played face cards give +30 Chips when scored",
            )
            .rarity(JokerRarity::Common)
            .cost(3)
            .chips(chips_bonus)
            .condition(StaticCondition::AnyRankScored(vec![
                Value::Jack,
                Value::Queen,
                Value::King,
            ]))
            .per_card()
            .build()
            .expect("Valid joker configuration"),
        )
    }

    /// Load Scary Face parameter from joker.json, with fallback to 30
    fn load_scary_face_parameter() -> i32 {
        match JsonParameterResolver::new() {
            Ok(resolver) => {
                match resolver.get_parameters_by_id(JokerId::ScaryFace) {
                    Ok(params) => params.first().unwrap_or(30), // #1# = chips value
                    Err(_) => 30,                               // Fallback to original value
                }
            }
            Err(_) => 30, // Fallback to original value
        }
    }

    /// Create Fibonacci (Fibonacci sequence cards give +8 Mult when scored)
    /// Fibonacci sequence: A, 2, 3, 5, 8
    pub fn create_fibonacci() -> Box<dyn Joker> {
        // Load parameter from joker.json, fallback to 8 if not found
        let mult_bonus = Self::load_fibonacci_parameter();

        Box::new(
            FrameworkStaticJoker::builder(
                JokerId::FibonacciJoker,
                "Fibonacci",
                "Each played Ace, 2, 3, 5, or 8 gives +8 Mult when scored",
            )
            .rarity(JokerRarity::Common)
            .cost(4)
            .mult(mult_bonus)
            .condition(StaticCondition::AnyRankScored(vec![
                Value::Ace,
                Value::Two,
                Value::Three,
                Value::Five,
                Value::Eight,
            ]))
            .per_card()
            .build()
            .expect("Valid joker configuration"),
        )
    }

    /// Load Fibonacci parameter from joker.json, with fallback to 8
    fn load_fibonacci_parameter() -> i32 {
        match JsonParameterResolver::new() {
            Ok(resolver) => {
                match resolver.get_parameters_by_id(JokerId::FibonacciJoker) {
                    Ok(params) => params.first().unwrap_or(8), // #1# = mult value
                    Err(_) => 8,                               // Fallback to reasonable value
                }
            }
            Err(_) => 8, // Fallback to reasonable value
        }
    }

    // Square Joker removed - now implemented as scaling joker in scaling_joker_impl.rs

    /// Create Walkie (+10 Chips and +4 Mult if hand contains Straight)
    pub fn create_walkie() -> Box<dyn Joker> {
        Box::new(
            FrameworkStaticJoker::builder(
                JokerId::Walkie,
                "Walkie",
                "+10 Chips and +4 Mult if played hand contains a Straight",
            )
            .rarity(JokerRarity::Common)
            .cost(3)
            .chips(10)
            .mult(4)
            .condition(StaticCondition::HandType(HandRank::Straight))
            .per_hand()
            .build()
            .expect("Valid joker configuration"),
        )
    }

    // Note: Runner is implemented as RunnerJoker in joker_impl.rs, not as a static joker

    /// Create Half Joker (configurable Mult if played hand has configurable or fewer cards)
    /// Uses parameter resolution from joker.json: #1# = mult value, #2# = card count
    pub fn create_half_joker() -> Box<dyn Joker> {
        // Load parameters from joker.json, fallback to original hardcoded values if fails
        let (mult_value, card_limit) = Self::load_half_joker_parameters();

        // Use a static description since the builder requires &'static str
        let description = if mult_value == 20 && card_limit == 4 {
            "+20 Mult if played hand has 4 or fewer cards"
        } else {
            // Use a generic description for non-default parameters
            "Configurable Mult if played hand has limited cards"
        };

        Box::new(
            FrameworkStaticJoker::builder(JokerId::HalfJoker, "Half Joker", description)
                .rarity(JokerRarity::Common)
                .cost(3)
                .mult(mult_value)
                .condition(StaticCondition::HandSizeAtMost(card_limit))
                .per_hand()
                .build()
                .expect("Valid joker configuration"),
        )
    }

    /// Load Half Joker parameters from joker.json, with fallback to original hardcoded values
    fn load_half_joker_parameters() -> (i32, usize) {
        match JsonParameterResolver::new() {
            Ok(resolver) => {
                match resolver.get_parameters_by_id(JokerId::HalfJoker) {
                    Ok(params) => {
                        let mult = params.first().unwrap_or(20); // #1# = mult value
                        let cards = params.second().unwrap_or(4) as usize; // #2# = card count
                        (mult, cards)
                    }
                    Err(_) => {
                        // Fallback to original hardcoded values
                        (20, 4)
                    }
                }
            }
            Err(_) => {
                // Fallback to original hardcoded values
                (20, 4)
            }
        }
    }

    /// Create Banner (+30 Chips for each remaining discard)
    pub fn create_banner() -> Box<dyn Joker> {
        Box::new(
            FrameworkStaticJoker::builder(
                JokerId::Banner,
                "Banner",
                "+30 Chips for each remaining discard",
            )
            .rarity(JokerRarity::Common)
            .cost(3)
            .chips(30) // Base amount per remaining discard
            .condition(StaticCondition::DiscardCount)
            .per_hand()
            .build()
            .expect("Valid joker configuration"),
        )
    }

    /// Create Bull joker (+2 Chips per $1 owned)
    pub fn create_bull_joker() -> Box<dyn Joker> {
        Box::new(
            FrameworkStaticJoker::builder(JokerId::BullMarket, "Bull", "+2 Chips per $1 owned")
                .rarity(JokerRarity::Common)
                .cost(3)
                .chips(2) // Base value, multiplied by money in create_effect_with_context
                .condition(StaticCondition::MoneyCount)
                .per_hand()
                .build()
                .expect("Valid joker configuration"),
        )
    }

    /// Create Stone Joker (+25 Chips per Stone card in deck)
    pub fn create_stone_joker() -> Box<dyn Joker> {
        Box::new(
            FrameworkStaticJoker::builder(
                JokerId::Stone,
                "Stone Joker",
                "+25 Chips per Stone card in deck",
            )
            .rarity(JokerRarity::Uncommon)
            .cost(4)
            .chips(25) // Base value, multiplied by stone cards count in create_effect_with_context
            .condition(StaticCondition::StoneCardsInDeck)
            .per_hand()
            .build()
            .expect("Valid joker configuration"),
        )
    }

    /// Create Abstract Joker (All Jokers give X0.25 more Mult)
    /// TODO: Requires joker interaction system
    /// WARNING: This is a PLACEHOLDER implementation that gives X1.25 Mult to self only
    /// The actual joker should affect ALL other jokers, not provide direct mult
    pub fn create_abstract_joker() -> Box<dyn Joker> {
        // PLACEHOLDER: Currently provides self mult multiplier - DO NOT USE IN PRODUCTION
        Box::new(
            FrameworkStaticJoker::builder(
                JokerId::AbstractJoker,
                "Abstract Joker",
                "All Jokers give X0.25 more Mult",
            )
            .rarity(JokerRarity::Common)
            .cost(3)
            .mult_multiplier(1.25) // TODO: Should affect other jokers
            .condition(StaticCondition::Always)
            .per_hand()
            .build()
            .expect("Valid joker configuration"),
        )
    }

    /// Create Steel Joker (This Joker gains X0.25 Mult for each Steel Card in your full deck)
    /// TODO: Requires deck composition access
    /// WARNING: This is a PLACEHOLDER implementation that gives X1.0 Mult ALWAYS
    /// The actual joker should scale based on Steel Card count: X(1.0 + 0.25 * steel_cards)
    pub fn create_steel_joker() -> Box<dyn Joker> {
        // PLACEHOLDER: Currently provides no mult multiplier - DO NOT USE IN PRODUCTION
        Box::new(
            FrameworkStaticJoker::builder(
                JokerId::SteelJoker,
                "Steel Joker",
                "This Joker gains X0.25 Mult for each Steel Card in your full deck",
            )
            .rarity(JokerRarity::Uncommon)
            .cost(6)
            .mult_multiplier(1.0) // TODO: Should be 1.0 + (0.25 * steel_card_count)
            .condition(StaticCondition::Always)
            .per_hand()
            .build()
            .expect("Valid joker configuration"),
        )
    }

    /// Create The Duo (X2 Mult if played hand contains a Pair)
    pub fn create_the_duo() -> Box<dyn Joker> {
        Box::new(
            FrameworkStaticJoker::builder(
                JokerId::TheDuo,
                "The Duo",
                "X2 Mult if played hand contains a Pair",
            )
            .rarity(JokerRarity::Rare)
            .cost(8)
            .mult_multiplier(2.0)
            .condition(StaticCondition::HandType(HandRank::OnePair))
            .per_hand()
            .build()
            .expect("Valid joker configuration"),
        )
    }

    /// Create The Trio (X3 Mult if played hand contains Three of a Kind)
    pub fn create_the_trio() -> Box<dyn Joker> {
        Box::new(
            FrameworkStaticJoker::builder(
                JokerId::TheTrio,
                "The Trio",
                "X3 Mult if played hand contains Three of a Kind",
            )
            .rarity(JokerRarity::Rare)
            .cost(8)
            .mult_multiplier(3.0)
            .condition(StaticCondition::HandType(HandRank::ThreeOfAKind))
            .per_hand()
            .build()
            .expect("Valid joker configuration"),
        )
    }

    /// Create The Family (X4 Mult if played hand contains Four of a Kind)
    pub fn create_the_family() -> Box<dyn Joker> {
        Box::new(
            FrameworkStaticJoker::builder(
                JokerId::TheFamily,
                "The Family",
                "X4 Mult if played hand contains Four of a Kind",
            )
            .rarity(JokerRarity::Rare)
            .cost(8)
            .mult_multiplier(4.0)
            .condition(StaticCondition::HandType(HandRank::FourOfAKind))
            .per_hand()
            .build()
            .expect("Valid joker configuration"),
        )
    }

    // =============================================================================
    // NEW SIMPLE STATIC JOKERS - Clean Code Implementation
    // =============================================================================

    /// Create Smiley Face (Played face cards give +4 Mult when scored)
    /// This is a perfect fit for the StaticJoker framework
    pub fn create_smiley_face() -> Box<dyn Joker> {
        // Load parameter from joker.json, fallback to 4 if not found
        let mult_bonus = Self::load_smiley_face_parameter();

        Box::new(
            FrameworkStaticJoker::builder(
                JokerId::Smiley,
                "Smiley Face",
                "Played face cards give +4 Mult when scored",
            )
            .rarity(JokerRarity::Common)
            .cost(3)
            .mult(mult_bonus)
            .condition(StaticCondition::AnyRankScored(vec![
                Value::Jack,
                Value::Queen,
                Value::King,
            ]))
            .per_card()
            .build()
            .expect("Valid joker configuration"),
        )
    }

    /// Load Smiley Face parameter from joker.json, with fallback to 4
    fn load_smiley_face_parameter() -> i32 {
        match JsonParameterResolver::new() {
            Ok(resolver) => {
                match resolver.get_parameters_by_id(JokerId::Smiley) {
                    Ok(params) => params.first().unwrap_or(4), // #1# = mult value
                    Err(_) => 4, // Fallback to reasonable value from joker.json analysis
                }
            }
            Err(_) => 4, // Fallback to reasonable value
        }
    }

    /// Create Baron (Each King held in hand gives X1.5 Mult)
    /// Uses the new RankHeldInHand condition for clean implementation
    pub fn create_baron() -> Box<dyn Joker> {
        // Load parameter from joker.json, fallback to 1.5 if not found
        let mult_multiplier = Self::load_baron_parameter();

        Box::new(
            FrameworkStaticJoker::builder(
                JokerId::BaronJoker,
                "Baron",
                "Each King held in hand gives X1.5 Mult",
            )
            .rarity(JokerRarity::Rare)
            .cost(8)
            .mult_multiplier(mult_multiplier)
            .condition(StaticCondition::RankHeldInHand(Value::King))
            .per_hand()
            .build()
            .expect("Valid joker configuration"),
        )
    }

    /// Load Baron parameter from joker.json, with fallback to 1.5
    fn load_baron_parameter() -> f64 {
        match JsonParameterResolver::new() {
            Ok(resolver) => {
                match resolver.get_parameters_by_id(JokerId::BaronJoker) {
                    Ok(params) => {
                        // Convert i32 to f64, expecting the parameter to be stored as scaled integer
                        let scaled_value = params.first().unwrap_or(150); // 150 = 1.5 * 100
                        scaled_value as f64 / 100.0 // Convert back to decimal
                    }
                    Err(_) => 1.5,
                }
            }
            Err(_) => 1.5, // Fallback value from joker.json analysis
        }
    }

    /// Create Raised Fist (Adds double the rank of lowest ranked card held in hand to Mult)
    /// Uses the new LowestRankInHand condition for clean implementation
    pub fn create_raised_fist() -> Box<dyn Joker> {
        Box::new(
            FrameworkStaticJoker::builder(
                JokerId::RaisedFist,
                "Raised Fist",
                "Adds double the rank of lowest ranked card held in hand to Mult",
            )
            .rarity(JokerRarity::Common)
            .cost(3)
            .mult(2) // This will be multiplied by the lowest rank value
            .condition(StaticCondition::LowestRankInHand)
            .per_hand()
            .build()
            .expect("Valid joker configuration"),
        )
    }

    /// Create Rough Gem (Played cards with Diamond suit earn $1 when scored)
    /// Uses the new money support in the StaticJoker framework
    pub fn create_rough_gem() -> Box<dyn Joker> {
        // Load parameter from joker.json, fallback to 1 if not found
        let money_bonus = Self::load_rough_gem_parameter();

        Box::new(
            FrameworkStaticJoker::builder(
                JokerId::RoughGem,
                "Rough Gem",
                "Played cards with Diamond suit earn $1 when scored",
            )
            .rarity(JokerRarity::Common)
            .cost(4)
            .money(money_bonus)
            .condition(StaticCondition::SuitScored(Suit::Diamond))
            .per_card()
            .build()
            .expect("Valid joker configuration"),
        )
    }

    /// Load Rough Gem parameter from joker.json, with fallback to 1
    fn load_rough_gem_parameter() -> i32 {
        match JsonParameterResolver::new() {
            Ok(resolver) => {
                match resolver.get_parameters_by_id(JokerId::RoughGem) {
                    Ok(params) => params.first().unwrap_or(1), // #1# = money value per diamond
                    Err(_) => 1, // Fallback value from joker.json analysis
                }
            }
            Err(_) => 1, // Fallback value
        }
    }

    /// Test-only methods that return concrete types for internal testing
    #[cfg(test)]
    pub fn create_greedy_joker_concrete() -> FrameworkStaticJoker {
        FrameworkStaticJoker::builder(
            JokerId::GreedyJoker,
            "Greedy Joker",
            "Played cards with Diamond suit give +3 Mult when scored",
        )
        .rarity(JokerRarity::Common)
        .cost(5)
        .mult(3)
        .condition(StaticCondition::SuitScored(Suit::Diamond))
        .per_card()
        .build()
        .expect("Valid joker configuration")
    }

    /// Test-only methods that return concrete types for internal testing
    #[cfg(test)]
    pub fn create_lusty_joker_concrete() -> FrameworkStaticJoker {
        FrameworkStaticJoker::builder(
            JokerId::LustyJoker,
            "Lusty Joker",
            "Played cards with Heart suit give +3 Mult when scored",
        )
        .rarity(JokerRarity::Common)
        .cost(5)
        .mult(3)
        .condition(StaticCondition::SuitScored(Suit::Heart))
        .per_card()
        .build()
        .expect("Valid joker configuration")
    }

    /// Test-only methods that return concrete types for internal testing
    #[cfg(test)]
    pub fn create_wrathful_joker_concrete() -> FrameworkStaticJoker {
        FrameworkStaticJoker::builder(
            JokerId::WrathfulJoker,
            "Wrathful Joker",
            "Played cards with Spade suit give +3 Mult when scored",
        )
        .rarity(JokerRarity::Common)
        .cost(5)
        .mult(3)
        .condition(StaticCondition::SuitScored(Suit::Spade))
        .per_card()
        .build()
        .expect("Valid joker configuration")
    }

    /// Test-only methods that return concrete types for internal testing
    #[cfg(test)]
    pub fn create_gluttonous_joker_concrete() -> FrameworkStaticJoker {
        FrameworkStaticJoker::builder(
            JokerId::GluttonousJoker,
            "Gluttonous Joker",
            "Played cards with Club suit give +3 Mult when scored",
        )
        .rarity(JokerRarity::Common)
        .cost(5)
        .mult(3)
        .condition(StaticCondition::SuitScored(Suit::Club))
        .per_card()
        .build()
        .expect("Valid joker configuration")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_joker_creation() {
        let joker = StaticJokerFactory::create_joker();
        assert_eq!(joker.id(), JokerId::Joker);
        assert_eq!(joker.name(), "Joker");
        assert_eq!(joker.cost(), 2);
        assert_eq!(joker.description(), "+4 Mult");
        assert_eq!(joker.rarity(), JokerRarity::Common);
    }

    #[test]
    fn test_all_suit_jokers() {
        // Test Greedy Joker (Diamond)
        let greedy = StaticJokerFactory::create_greedy_joker();
        assert_eq!(greedy.id(), JokerId::GreedyJoker);
        assert_eq!(greedy.name(), "Greedy Joker");
        assert_eq!(
            greedy.description(),
            "Played cards with Diamond suit give +3 Mult when scored"
        );
        assert_eq!(greedy.rarity(), JokerRarity::Common);
        assert_eq!(greedy.cost(), 5);

        // Test Lusty Joker (Heart)
        let lusty = StaticJokerFactory::create_lusty_joker();
        assert_eq!(lusty.id(), JokerId::LustyJoker);
        assert_eq!(lusty.name(), "Lusty Joker");
        assert_eq!(
            lusty.description(),
            "Played cards with Heart suit give +3 Mult when scored"
        );
        assert_eq!(lusty.rarity(), JokerRarity::Common);

        // Test Wrathful Joker (Spade)
        let wrathful = StaticJokerFactory::create_wrathful_joker();
        assert_eq!(wrathful.id(), JokerId::WrathfulJoker);
        assert_eq!(wrathful.name(), "Wrathful Joker");
        assert_eq!(
            wrathful.description(),
            "Played cards with Spade suit give +3 Mult when scored"
        );

        // Test Gluttonous Joker (Club)
        let gluttonous = StaticJokerFactory::create_gluttonous_joker();
        assert_eq!(gluttonous.id(), JokerId::GluttonousJoker);
        assert_eq!(gluttonous.name(), "Gluttonous Joker");
        assert_eq!(
            gluttonous.description(),
            "Played cards with Club suit give +3 Mult when scored"
        );
    }

    #[test]
    fn test_all_hand_type_mult_jokers() {
        // Test Jolly Joker (Pair)
        let jolly = StaticJokerFactory::create_jolly_joker();
        assert_eq!(jolly.id(), JokerId::JollyJoker);
        assert_eq!(jolly.name(), "Jolly Joker");
        assert_eq!(jolly.description(), "+8 Mult if played hand contains Pair");
        assert_eq!(jolly.cost(), 3);

        // Test Zany Joker (Three of a Kind)
        let zany = StaticJokerFactory::create_zany_joker();
        assert_eq!(zany.id(), JokerId::ZanyJoker);
        assert_eq!(zany.name(), "Zany Joker");
        assert_eq!(
            zany.description(),
            "+12 Mult if played hand contains Three of a Kind"
        );
        assert_eq!(zany.cost(), 4);

        // Test Mad Joker (Two Pair)
        let mad = StaticJokerFactory::create_mad_joker();
        assert_eq!(mad.id(), JokerId::MadJoker);
        assert_eq!(mad.name(), "Mad Joker");
        assert_eq!(
            mad.description(),
            "+10 Mult if played hand contains Two Pair"
        );

        // Test Crazy Joker (Straight)
        let crazy = StaticJokerFactory::create_crazy_joker();
        assert_eq!(crazy.id(), JokerId::CrazyJoker);
        assert_eq!(crazy.name(), "Crazy Joker");
        assert_eq!(
            crazy.description(),
            "+12 Mult if played hand contains Straight"
        );

        // Test Droll Joker (Flush)
        let droll = StaticJokerFactory::create_droll_joker();
        assert_eq!(droll.id(), JokerId::DrollJoker);
        assert_eq!(droll.name(), "Droll Joker");
        assert_eq!(
            droll.description(),
            "+10 Mult if played hand contains Flush"
        );
    }

    #[test]
    fn test_all_hand_type_chip_jokers() {
        // Test Sly Joker (Pair)
        let sly = StaticJokerFactory::create_sly_joker();
        assert_eq!(sly.id(), JokerId::SlyJoker);
        assert_eq!(sly.name(), "Sly Joker");
        assert_eq!(sly.description(), "+50 Chips if played hand contains Pair");

        // Test Wily Joker (Three of a Kind)
        let wily = StaticJokerFactory::create_wily_joker();
        assert_eq!(wily.id(), JokerId::WilyJoker);
        assert_eq!(wily.name(), "Wily Joker");
        assert_eq!(
            wily.description(),
            "+100 Chips if played hand contains Three of a Kind"
        );

        // Test Clever Joker (Two Pair)
        let clever = StaticJokerFactory::create_clever_joker();
        assert_eq!(clever.id(), JokerId::CleverJoker);
        assert_eq!(clever.name(), "Clever Joker");
        assert_eq!(
            clever.description(),
            "+80 Chips if played hand contains Two Pair"
        );

        // Test Devious Joker (Straight)
        let devious = StaticJokerFactory::create_devious_joker();
        assert_eq!(devious.id(), JokerId::DeviousJoker);
        assert_eq!(devious.name(), "Devious Joker");
        assert_eq!(
            devious.description(),
            "+100 Chips if played hand contains Straight"
        );

        // Test Crafty Joker (Flush)
        let crafty = StaticJokerFactory::create_crafty_joker();
        assert_eq!(crafty.id(), JokerId::CraftyJoker);
        assert_eq!(crafty.name(), "Crafty Joker");
        assert_eq!(
            crafty.description(),
            "+80 Chips if played hand contains Flush"
        );
    }

    #[test]
    fn test_rank_based_jokers() {
        // Test Even Steven
        let even_steven = StaticJokerFactory::create_even_steven();
        assert_eq!(even_steven.id(), JokerId::EvenSteven);
        assert_eq!(even_steven.name(), "Even Steven");
        assert_eq!(
            even_steven.description(),
            "Played cards with even rank (2, 4, 6, 8, 10) give +4 Mult when scored"
        );

        // Test Odd Todd
        let odd_todd = StaticJokerFactory::create_odd_todd();
        assert_eq!(odd_todd.id(), JokerId::OddTodd);
        assert_eq!(odd_todd.name(), "Odd Todd");
        assert_eq!(
            odd_todd.description(),
            "Played cards with odd rank (3, 5, 7, 9, A) give +31 Chips when scored"
        );

        // Test Scholar
        let scholar = StaticJokerFactory::create_scholar();
        assert_eq!(scholar.id(), JokerId::Scholar);
        assert_eq!(scholar.name(), "Scholar");
        assert_eq!(
            scholar.description(),
            "Played Aces give +20 Chips and +4 Mult when scored"
        );
    }

    #[test]
    fn test_joker_cost_distribution() {
        // Test that jokers have appropriate costs based on rarity/power
        let basic_jokers = vec![
            StaticJokerFactory::create_joker(), // 2
        ];

        let suit_jokers = vec![
            StaticJokerFactory::create_greedy_joker(),     // 5
            StaticJokerFactory::create_lusty_joker(),      // 5
            StaticJokerFactory::create_wrathful_joker(),   // 5
            StaticJokerFactory::create_gluttonous_joker(), // 5
        ];

        let mid_tier_jokers = vec![
            StaticJokerFactory::create_jolly_joker(), // 3
            StaticJokerFactory::create_sly_joker(),   // 3
        ];

        let higher_tier_jokers = vec![
            StaticJokerFactory::create_zany_joker(), // 4
            StaticJokerFactory::create_wily_joker(), // 4
        ];

        // Verify cost progression
        for joker in basic_jokers {
            assert_eq!(joker.cost(), 2);
        }

        for joker in suit_jokers {
            assert_eq!(joker.cost(), 5);
        }

        for joker in mid_tier_jokers {
            assert_eq!(joker.cost(), 3);
        }

        for joker in higher_tier_jokers {
            assert_eq!(joker.cost(), 4);
        }
    }

    #[test]
    fn test_scary_face_joker() {
        let scary_face = StaticJokerFactory::create_scary_face();

        // Test properties
        assert_eq!(scary_face.id(), JokerId::ScaryFace);
        assert_eq!(scary_face.name(), "Scary Face");
        assert_eq!(scary_face.rarity(), JokerRarity::Common);
        assert_eq!(scary_face.cost(), 3);

        // Test description contains face cards and chips
        assert!(scary_face.description().contains("Played face cards give"));
        assert!(scary_face.description().contains("Chips when scored"));
    }

    #[test]
    fn test_fibonacci_joker() {
        let fibonacci = StaticJokerFactory::create_fibonacci();

        // Test properties
        assert_eq!(fibonacci.id(), JokerId::FibonacciJoker);
        assert_eq!(fibonacci.name(), "Fibonacci");
        assert_eq!(fibonacci.rarity(), JokerRarity::Common);
        assert_eq!(fibonacci.cost(), 4);

        // Test description contains Fibonacci sequence
        assert!(fibonacci
            .description()
            .contains("Each played Ace, 2, 3, 5, or 8"));
        assert!(fibonacci.description().contains("Mult when scored"));
    }

    #[test]
    fn test_all_factory_jokers_can_be_created() {
        // Test that all 20 jokers can be created without panicking
        let jokers = vec![
            StaticJokerFactory::create_joker(),
            StaticJokerFactory::create_greedy_joker(),
            StaticJokerFactory::create_lusty_joker(),
            StaticJokerFactory::create_wrathful_joker(),
            StaticJokerFactory::create_gluttonous_joker(),
            StaticJokerFactory::create_jolly_joker(),
            StaticJokerFactory::create_zany_joker(),
            StaticJokerFactory::create_mad_joker(),
            StaticJokerFactory::create_crazy_joker(),
            StaticJokerFactory::create_droll_joker(),
            StaticJokerFactory::create_sly_joker(),
            StaticJokerFactory::create_wily_joker(),
            StaticJokerFactory::create_clever_joker(),
            StaticJokerFactory::create_devious_joker(),
            StaticJokerFactory::create_crafty_joker(),
            StaticJokerFactory::create_even_steven(),
            StaticJokerFactory::create_odd_todd(),
            StaticJokerFactory::create_scholar(),
            StaticJokerFactory::create_scary_face(),
            StaticJokerFactory::create_fibonacci(),
        ];

        assert_eq!(jokers.len(), 20);

        // Ensure all have valid IDs and names
        for joker in &jokers {
            assert!(!joker.name().is_empty());
            assert!(!joker.description().is_empty());
            assert!(joker.cost() > 0);
        }
    }

    #[test]
    fn test_joker_rarity_distribution() {
        // Test that jokers have appropriate rarities
        let common_jokers = vec![
            StaticJokerFactory::create_joker(),
            StaticJokerFactory::create_greedy_joker(),
            StaticJokerFactory::create_lusty_joker(),
            StaticJokerFactory::create_wrathful_joker(),
            StaticJokerFactory::create_gluttonous_joker(),
        ];

        for joker in common_jokers {
            assert_eq!(joker.rarity(), JokerRarity::Common);
        }

        // Verify specific jokers have expected rarities
        let jolly = StaticJokerFactory::create_jolly_joker();
        assert_eq!(jolly.rarity(), JokerRarity::Common);

        let scholar = StaticJokerFactory::create_scholar();
        assert_eq!(scholar.rarity(), JokerRarity::Common);
    }

    // =============================================================================
    // TESTS FOR NEW SIMPLE STATIC JOKERS
    // =============================================================================

    #[test]
    fn test_smiley_face_joker() {
        let smiley_face = StaticJokerFactory::create_smiley_face();

        // Test properties
        assert_eq!(smiley_face.id(), JokerId::Smiley);
        assert_eq!(smiley_face.name(), "Smiley Face");
        assert_eq!(smiley_face.rarity(), JokerRarity::Common);
        assert_eq!(smiley_face.cost(), 3);

        // Test description contains expected content
        assert!(smiley_face.description().contains("Played face cards give"));
        assert!(smiley_face.description().contains("Mult when scored"));
    }

    #[test]
    fn test_baron_joker() {
        let baron = StaticJokerFactory::create_baron();

        // Test properties
        assert_eq!(baron.id(), JokerId::BaronJoker);
        assert_eq!(baron.name(), "Baron");
        assert_eq!(baron.rarity(), JokerRarity::Rare);
        assert_eq!(baron.cost(), 8);

        // Test description contains expected content
        assert!(baron.description().contains("Each King"));
        assert!(baron.description().contains("held in hand"));
        assert!(baron.description().contains("X1.5 Mult"));
    }

    #[test]
    fn test_raised_fist_joker() {
        let raised_fist = StaticJokerFactory::create_raised_fist();

        // Test properties
        assert_eq!(raised_fist.id(), JokerId::RaisedFist);
        assert_eq!(raised_fist.name(), "Raised Fist");
        assert_eq!(raised_fist.rarity(), JokerRarity::Common);
        assert_eq!(raised_fist.cost(), 3);

        // Test description contains expected content
        assert!(raised_fist.description().contains("double the rank"));
        assert!(raised_fist.description().contains("lowest ranked card"));
        assert!(raised_fist.description().contains("held in hand"));
    }

    #[test]
    fn test_rough_gem_joker() {
        let rough_gem = StaticJokerFactory::create_rough_gem();

        // Test properties
        assert_eq!(rough_gem.id(), JokerId::RoughGem);
        assert_eq!(rough_gem.name(), "Rough Gem");
        assert_eq!(rough_gem.rarity(), JokerRarity::Common);
        assert_eq!(rough_gem.cost(), 4);

        // Test description contains expected content
        assert!(rough_gem.description().contains("Diamond suit"));
        assert!(rough_gem.description().contains("earn $1"));
        assert!(rough_gem.description().contains("when scored"));
    }

    #[test]
    fn test_all_new_simple_static_jokers_creation() {
        // Test that all four new jokers can be created without panicking
        let new_jokers = vec![
            StaticJokerFactory::create_smiley_face(),
            StaticJokerFactory::create_baron(),
            StaticJokerFactory::create_raised_fist(),
            StaticJokerFactory::create_rough_gem(),
        ];

        assert_eq!(new_jokers.len(), 4);

        // Ensure all have valid IDs and names
        for joker in &new_jokers {
            assert!(!joker.name().is_empty());
            assert!(!joker.description().is_empty());
            assert!(joker.cost() > 0);
        }

        // Verify each has the expected ID
        assert_eq!(new_jokers[0].id(), JokerId::Smiley);
        assert_eq!(new_jokers[1].id(), JokerId::BaronJoker);
        assert_eq!(new_jokers[2].id(), JokerId::RaisedFist);
        assert_eq!(new_jokers[3].id(), JokerId::RoughGem);
    }
}
