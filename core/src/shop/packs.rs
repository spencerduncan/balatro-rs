use crate::card::Card;
use crate::consumables::ConsumableId;
use crate::error::GameError;
use crate::game::Game;
use crate::joker::JokerId;
use crate::shop::{ConsumableType, ShopItem};

/// Types of booster packs available in the shop
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "python", pyo3::pyclass(eq))]
pub enum PackType {
    /// Standard pack with 3 playing cards (choose 1)
    Standard,
    /// Jumbo pack with 5 playing cards (choose 1)
    Jumbo,
    /// Mega pack with 7 playing cards (choose 1)
    Mega,
    /// Enhanced pack with 3-4 enhanced playing cards (choose 1)
    Enhanced,
    /// Variety pack with mixed contents (choose 1)
    Variety,
    /// Buffoon pack with 2 joker cards (choose 1)
    Buffoon,
    /// Arcana pack with 2-3 Tarot cards (choose 1)
    Arcana,
    /// Celestial pack with 2-3 Planet cards (choose 1)
    Celestial,
    /// Spectral pack with 2-3 Spectral cards (choose 1)
    Spectral,
    /// Mega Buffoon pack with 4 joker cards (choose 1)
    MegaBuffoon,
    /// Mega Arcana pack with 4-6 Tarot cards (choose 1)
    MegaArcana,
    /// Mega Celestial pack with 4-6 Planet cards (choose 1)
    MegaCelestial,
    /// Mega Spectral pack with 4-6 Spectral cards (choose 1)
    MegaSpectral,
}

impl PackType {
    /// Get the base cost of this pack type
    pub fn base_cost(self) -> usize {
        match self {
            PackType::Standard => 4,
            PackType::Jumbo => 6,
            PackType::Mega => 8,
            PackType::Enhanced => 6,
            PackType::Variety => 5,
            PackType::Buffoon => 4,
            PackType::Arcana => 4,
            PackType::Celestial => 4,
            PackType::Spectral => 4,
            PackType::MegaBuffoon => 8,
            PackType::MegaArcana => 8,
            PackType::MegaCelestial => 8,
            PackType::MegaSpectral => 8,
        }
    }

    /// Get the number of options this pack type generates
    pub fn option_count(self) -> (usize, usize) {
        match self {
            PackType::Standard => (3, 3),      // Exactly 3 options
            PackType::Jumbo => (5, 5),         // Exactly 5 options
            PackType::Mega => (7, 7),          // Exactly 7 options
            PackType::Enhanced => (3, 4),      // 3-4 options
            PackType::Variety => (3, 5),       // 3-5 options (mixed)
            PackType::Buffoon => (2, 2),       // Exactly 2 options
            PackType::Arcana => (2, 3),        // 2-3 options
            PackType::Celestial => (2, 3),     // 2-3 options
            PackType::Spectral => (2, 3),      // 2-3 options
            PackType::MegaBuffoon => (4, 4),   // Exactly 4 options
            PackType::MegaArcana => (4, 6),    // 4-6 options
            PackType::MegaCelestial => (4, 6), // 4-6 options
            PackType::MegaSpectral => (4, 6),  // 4-6 options
        }
    }

    /// Get the number of items the player can choose from this pack
    pub fn choose_count(self) -> usize {
        1 // All packs allow choosing 1 item
    }

    /// Check if this pack type can be skipped
    pub fn can_skip(self) -> bool {
        true // All packs can be skipped
    }
}

impl std::fmt::Display for PackType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PackType::Standard => write!(f, "Standard Pack"),
            PackType::Jumbo => write!(f, "Jumbo Pack"),
            PackType::Mega => write!(f, "Mega Pack"),
            PackType::Enhanced => write!(f, "Enhanced Pack"),
            PackType::Variety => write!(f, "Variety Pack"),
            PackType::Buffoon => write!(f, "Buffoon Pack"),
            PackType::Arcana => write!(f, "Arcana Pack"),
            PackType::Celestial => write!(f, "Celestial Pack"),
            PackType::Spectral => write!(f, "Spectral Pack"),
            PackType::MegaBuffoon => write!(f, "Mega Buffoon Pack"),
            PackType::MegaArcana => write!(f, "Mega Arcana Pack"),
            PackType::MegaCelestial => write!(f, "Mega Celestial Pack"),
            PackType::MegaSpectral => write!(f, "Mega Spectral Pack"),
        }
    }
}

/// An individual option within a pack
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PackOption {
    /// The item offered by this option
    pub item: ShopItem,
    /// Preview information about this option
    pub preview_info: String,
}

impl PackOption {
    /// Create a new pack option
    pub fn new(item: ShopItem, preview_info: String) -> Self {
        Self { item, preview_info }
    }
}

/// A booster pack containing multiple options for the player to choose from
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Pack {
    /// Type of pack
    pub pack_type: PackType,
    /// Options available in this pack
    pub options: Vec<PackOption>,
    /// Number of items the player can choose
    pub choose_count: usize,
    /// Whether this pack can be skipped
    pub can_skip: bool,
    /// Cost to purchase this pack
    pub cost: usize,
}

impl Pack {
    /// Create a new pack of the specified type
    pub fn new(pack_type: PackType) -> Self {
        Self {
            pack_type,
            options: Vec::new(),
            choose_count: pack_type.choose_count(),
            can_skip: pack_type.can_skip(),
            cost: pack_type.base_cost(),
        }
    }

    /// Generate pack contents based on pack type and game state
    pub fn generate_contents(&mut self, game: &Game) -> Result<(), GameError> {
        use rand::Rng;

        let (min_options, max_options) = self.pack_type.option_count();
        let mut option_count = if min_options == max_options {
            min_options
        } else {
            // Use proper randomization for variable option counts
            rand::thread_rng().gen_range(min_options..=max_options)
        };

        // Check for Grab Bag voucher - adds +1 option to all packs
        if game.vouchers.owns(crate::vouchers::VoucherId::GrabBag) {
            option_count += 1;
        }

        self.options.clear();

        match self.pack_type {
            PackType::Standard | PackType::Jumbo | PackType::Mega | PackType::Enhanced => {
                self.generate_standard_options(option_count, game)?
            }
            PackType::Variety => {
                // Variety packs have mixed contents - for now just use standard
                self.generate_standard_options(option_count, game)?
            }
            PackType::Buffoon | PackType::MegaBuffoon => {
                self.generate_buffoon_options(option_count, game)?
            }
            PackType::Arcana | PackType::MegaArcana => {
                self.generate_arcana_options(option_count, game)?
            }
            PackType::Celestial | PackType::MegaCelestial => {
                self.generate_celestial_options(option_count, game)?
            }
            PackType::Spectral | PackType::MegaSpectral => {
                self.generate_spectral_options(option_count, game)?
            }
        }

        Ok(())
    }

    /// Generate standard pack options (playing cards)
    fn generate_standard_options(&mut self, count: usize, _game: &Game) -> Result<(), GameError> {
        use crate::card::{Enhancement, Suit, Value};
        use rand::seq::SliceRandom;
        use rand::Rng;

        let suits = [Suit::Heart, Suit::Diamond, Suit::Club, Suit::Spade];
        let values = [
            Value::Ace,
            Value::Two,
            Value::Three,
            Value::Four,
            Value::Five,
            Value::Six,
            Value::Seven,
            Value::Eight,
            Value::Nine,
            Value::Ten,
            Value::Jack,
            Value::Queen,
            Value::King,
        ];

        let mut rng = rand::thread_rng();

        for _ in 0..count {
            // Select random suit and value
            let suit = *suits.choose(&mut rng).unwrap();
            let value = *values.choose(&mut rng).unwrap();

            // Create card with potential enhancement (10% chance)
            let mut card = Card::new(value, suit);
            if rng.gen_bool(0.1) {
                // 10% chance for enhancement
                let enhancements = [
                    Enhancement::Bonus,
                    Enhancement::Mult,
                    Enhancement::Wild,
                    Enhancement::Glass,
                    Enhancement::Steel,
                ];
                card.enhancement = Some(*enhancements.choose(&mut rng).unwrap());
            }

            let enhancement_prefix = match card.enhancement {
                Some(enh) => format!("{enh:?} "),
                None => String::new(),
            };

            let option = PackOption::new(
                ShopItem::PlayingCard(card),
                format!("{enhancement_prefix}{value:?} of {suit:?}"),
            );
            self.options.push(option);
        }

        Ok(())
    }

    /// Generate buffoon pack options (jokers)
    fn generate_buffoon_options(&mut self, count: usize, _game: &Game) -> Result<(), GameError> {
        use crate::joker::JokerRarity;
        use rand::seq::SliceRandom;
        use rand::Rng;

        let mut rng = rand::thread_rng();

        // Define rarity weights: 70% Common, 25% Uncommon, 5% Rare
        let rarities = [
            (JokerRarity::Common, 70),
            (JokerRarity::Uncommon, 25),
            (JokerRarity::Rare, 5),
        ];

        // Available jokers by rarity (using the jokers we know exist)
        let common_jokers = [
            JokerId::Joker,
            JokerId::GreedyJoker,
            JokerId::LustyJoker,
            JokerId::WrathfulJoker,
            JokerId::GluttonousJoker,
            JokerId::JollyJoker,
        ];

        let uncommon_jokers = [
            JokerId::ZanyJoker,
            JokerId::MadJoker,
            JokerId::CrazyJoker,
            JokerId::DrollJoker,
        ];

        let rare_jokers = [
            JokerId::SlyJoker,
            JokerId::WilyJoker,
            JokerId::CleverJoker,
            JokerId::DeviousJoker,
        ];

        for _ in 0..count {
            // Select rarity based on weighted distribution
            let total_weight: u32 = rarities.iter().map(|(_, weight)| weight).sum();
            let roll = rng.gen_range(1..=total_weight);

            let mut current_weight = 0;
            let selected_rarity = rarities
                .iter()
                .find(|(_, weight)| {
                    current_weight += weight;
                    roll <= current_weight
                })
                .map(|(rarity, _)| *rarity)
                .unwrap_or(JokerRarity::Common);

            // Select joker based on rarity
            let joker_id = match selected_rarity {
                JokerRarity::Common => *common_jokers.choose(&mut rng).unwrap(),
                JokerRarity::Uncommon => *uncommon_jokers.choose(&mut rng).unwrap(),
                JokerRarity::Rare => *rare_jokers.choose(&mut rng).unwrap(),
                JokerRarity::Legendary => JokerId::Joker, // Fallback to basic for legendary
            };

            let option = PackOption::new(
                ShopItem::Joker(joker_id),
                format!("{joker_id:?} Joker ({selected_rarity:?})"),
            );
            self.options.push(option);
        }

        Ok(())
    }

    /// Generate arcana pack options (tarot cards)
    fn generate_arcana_options(&mut self, count: usize, _game: &Game) -> Result<(), GameError> {
        use rand::seq::SliceRandom;

        let tarot_cards = ConsumableId::tarot_cards();
        let mut rng = rand::thread_rng();

        for _ in 0..count {
            // Randomly select a specific tarot card for preview info
            let selected_tarot = tarot_cards
                .choose(&mut rng)
                .unwrap_or(&ConsumableId::TheFool);

            let option = PackOption::new(
                ShopItem::Consumable(ConsumableType::Tarot),
                format!("{selected_tarot}"), // Use the specific card name
            );
            self.options.push(option);
        }

        Ok(())
    }

    /// Generate celestial pack options (planet cards)
    fn generate_celestial_options(&mut self, count: usize, _game: &Game) -> Result<(), GameError> {
        use rand::seq::SliceRandom;

        let planet_cards = ConsumableId::planet_cards();
        let mut rng = rand::thread_rng();

        for _ in 0..count {
            // Randomly select a specific planet card for preview info
            let selected_planet = planet_cards
                .choose(&mut rng)
                .unwrap_or(&ConsumableId::Mercury);

            let option = PackOption::new(
                ShopItem::Consumable(ConsumableType::Planet),
                format!("{selected_planet}"), // Use the specific card name
            );
            self.options.push(option);
        }

        Ok(())
    }

    /// Generate spectral pack options (spectral cards)
    fn generate_spectral_options(&mut self, count: usize, _game: &Game) -> Result<(), GameError> {
        use rand::seq::SliceRandom;

        let spectral_cards = ConsumableId::spectral_cards();
        let mut rng = rand::thread_rng();

        for _ in 0..count {
            // Randomly select a specific spectral card for preview info
            let selected_spectral = spectral_cards
                .choose(&mut rng)
                .unwrap_or(&ConsumableId::Familiar);

            let option = PackOption::new(
                ShopItem::Consumable(ConsumableType::Spectral),
                format!("{selected_spectral}"), // Use the specific card name
            );
            self.options.push(option);
        }

        Ok(())
    }

    /// Check if a selection index is valid for this pack
    pub fn is_valid_selection(&self, option_index: usize) -> bool {
        option_index < self.options.len()
    }

    /// Select an option from this pack
    pub fn select_option(&self, option_index: usize) -> Result<ShopItem, GameError> {
        if !self.is_valid_selection(option_index) {
            return Err(GameError::InvalidAction);
        }

        Ok(self.options[option_index].item.clone())
    }
}

/// State for an opened pack that player is currently choosing from
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OpenPackState {
    /// The pack being opened
    pub pack: Pack,
    /// The pack's ID in the inventory
    pub pack_id: usize,
}

impl OpenPackState {
    /// Create a new open pack state
    pub fn new(pack: Pack, pack_id: usize) -> Self {
        Self { pack, pack_id }
    }
}

/// Trait for pack generation and management
pub trait PackGenerator {
    /// Generate a pack of the specified type
    fn generate_pack(&self, pack_type: PackType, game: &Game) -> Result<Pack, GameError>;

    /// Check if a pack type is available for purchase
    fn is_pack_available(&self, pack_type: PackType, game: &Game) -> bool;

    /// Get all available pack types for current game state
    fn available_pack_types(&self, game: &Game) -> Vec<PackType>;
}

/// Default pack generator implementation
pub struct DefaultPackGenerator;

impl PackGenerator for DefaultPackGenerator {
    fn generate_pack(&self, pack_type: PackType, game: &Game) -> Result<Pack, GameError> {
        let mut pack = Pack::new(pack_type);
        pack.generate_contents(game)?;
        Ok(pack)
    }

    fn is_pack_available(&self, pack_type: PackType, game: &Game) -> bool {
        // Basic availability logic - all packs available if player has money
        game.money >= pack_type.base_cost()
    }

    fn available_pack_types(&self, game: &Game) -> Vec<PackType> {
        let all_pack_types = [
            PackType::Standard,
            PackType::Buffoon,
            PackType::Arcana,
            PackType::Celestial,
            PackType::Spectral,
            PackType::MegaBuffoon,
            PackType::MegaArcana,
            PackType::MegaCelestial,
            PackType::MegaSpectral,
        ];

        all_pack_types
            .iter()
            .filter(|&&pack_type| self.is_pack_available(pack_type, game))
            .copied()
            .collect()
    }
}
