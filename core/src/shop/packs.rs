use crate::card::Card;
use crate::config::Config;
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
    /// Get the base cost of this pack type using configuration values
    pub fn base_cost(self, config: &Config) -> usize {
        match self {
            PackType::Standard => config.pack_standard_cost,
            PackType::Jumbo => config.pack_jumbo_cost,
            PackType::Mega => config.pack_mega_cost,
            PackType::Enhanced => config.pack_enhanced_cost,
            PackType::Variety => config.pack_variety_cost,
            PackType::Buffoon => config.pack_buffoon_cost,
            PackType::Arcana => config.pack_consumable_cost,
            PackType::Celestial => config.pack_consumable_cost,
            PackType::Spectral => config.pack_consumable_cost,
            PackType::MegaBuffoon => config.pack_mega_consumable_cost,
            PackType::MegaArcana => config.pack_mega_consumable_cost,
            PackType::MegaCelestial => config.pack_mega_consumable_cost,
            PackType::MegaSpectral => config.pack_mega_consumable_cost,
        }
    }

    /// Get the number of options this pack type generates using configuration values
    pub fn option_count(self, config: &Config) -> (usize, usize) {
        match self {
            PackType::Standard => config.pack_standard_options,
            PackType::Jumbo => config.pack_jumbo_options,
            PackType::Mega => config.pack_mega_options,
            PackType::Enhanced => config.pack_enhanced_options,
            PackType::Variety => config.pack_variety_options,
            PackType::Buffoon => config.pack_buffoon_options,
            PackType::Arcana => config.pack_consumable_options,
            PackType::Celestial => config.pack_consumable_options,
            PackType::Spectral => config.pack_consumable_options,
            PackType::MegaBuffoon => config.pack_mega_buffoon_options,
            PackType::MegaArcana => config.pack_mega_consumable_options,
            PackType::MegaCelestial => config.pack_mega_consumable_options,
            PackType::MegaSpectral => config.pack_mega_consumable_options,
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
    /// Create a new pack of the specified type using configuration values
    pub fn new(pack_type: PackType, config: &Config) -> Self {
        Self {
            pack_type,
            options: Vec::new(),
            choose_count: pack_type.choose_count(),
            can_skip: pack_type.can_skip(),
            cost: pack_type.base_cost(config),
        }
    }

    /// Generate pack contents based on pack type, game state, and configuration
    pub fn generate_contents(&mut self, game: &Game, config: &Config) -> Result<(), GameError> {
        let (min_options, max_options) = self.pack_type.option_count(config);
        let mut option_count = if min_options == max_options {
            min_options
        } else {
            // Use proper randomization for variable option counts
            game.rng.gen_range(min_options..=max_options)
        };

        // Check for Grab Bag voucher - adds +1 option to all packs
        if game.vouchers.owns(crate::vouchers::VoucherId::GrabBag) {
            option_count += 1;
        }

        self.options.clear();

        match self.pack_type {
            PackType::Standard | PackType::Jumbo | PackType::Mega | PackType::Enhanced => {
                self.generate_standard_options(option_count, game, config)?
            }
            PackType::Variety => {
                // Variety packs have mixed contents - for now just use standard
                self.generate_standard_options(option_count, game, config)?
            }
            PackType::Buffoon | PackType::MegaBuffoon => {
                self.generate_buffoon_options(option_count, game, config)?
            }
            PackType::Arcana | PackType::MegaArcana => {
                self.generate_arcana_options(option_count, game, config)?
            }
            PackType::Celestial | PackType::MegaCelestial => {
                self.generate_celestial_options(option_count, game, config)?
            }
            PackType::Spectral | PackType::MegaSpectral => {
                self.generate_spectral_options(option_count, game, config)?
            }
        }

        Ok(())
    }

    /// Generate standard pack options (playing cards)
    fn generate_standard_options(
        &mut self,
        count: usize,
        game: &Game,
        config: &Config,
    ) -> Result<(), GameError> {
        use crate::card::{Enhancement, Suit, Value};

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

        for _ in 0..count {
            // Select random suit and value
            let suit = *game.rng.choose(&suits).unwrap();
            let value = *game.rng.choose(&values).unwrap();

            // Create card with potential enhancement (configurable chance)
            let mut card = Card::new(value, suit);
            if game.rng.gen_bool(config.enhancement_rate) {
                let enhancements = [
                    Enhancement::Bonus,
                    Enhancement::Mult,
                    Enhancement::Wild,
                    Enhancement::Glass,
                    Enhancement::Steel,
                ];
                card.enhancement = Some(*game.rng.choose(&enhancements).unwrap());
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
    fn generate_buffoon_options(
        &mut self,
        count: usize,
        game: &Game,
        config: &Config,
    ) -> Result<(), GameError> {
        use crate::joker::JokerRarity;

        // Define rarity weights using configuration values
        let rarities = [
            (JokerRarity::Common, config.joker_rarity_weight_common),
            (JokerRarity::Uncommon, config.joker_rarity_weight_uncommon),
            (JokerRarity::Rare, config.joker_rarity_weight_rare),
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
            let roll = game.rng.gen_range(1..=total_weight);

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
                JokerRarity::Common => *game.rng.choose(&common_jokers).unwrap(),
                JokerRarity::Uncommon => *game.rng.choose(&uncommon_jokers).unwrap(),
                JokerRarity::Rare => *game.rng.choose(&rare_jokers).unwrap(),
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
    fn generate_arcana_options(
        &mut self,
        count: usize,
        game: &Game,
        _config: &Config,
    ) -> Result<(), GameError> {
        // Arcana packs can contain both Tarot cards and Soul (special spectral card)
        let mut available_cards = ConsumableId::tarot_cards();
        available_cards.push(ConsumableId::TheSoul); // Soul can appear in Arcana packs

        for _ in 0..count {
            // Randomly select from both tarot cards and Soul
            let selected_card = game
                .rng
                .choose(&available_cards)
                .unwrap_or(&ConsumableId::TheFool);

            // Determine the correct consumable type based on the selected card
            let consumable_type = if *selected_card == ConsumableId::TheSoul {
                ConsumableType::Spectral
            } else {
                ConsumableType::Tarot
            };

            let option = PackOption::new(
                ShopItem::Consumable(consumable_type),
                format!("{selected_card}"), // Use the specific card name
            );
            self.options.push(option);
        }

        Ok(())
    }

    /// Generate celestial pack options (planet cards)
    fn generate_celestial_options(
        &mut self,
        count: usize,
        game: &Game,
        _config: &Config,
    ) -> Result<(), GameError> {
        // Celestial packs can contain both Planet cards and Black Hole (special spectral card)
        let mut available_cards = ConsumableId::planet_cards();
        available_cards.push(ConsumableId::BlackHole); // Black Hole can appear in Celestial packs

        for _ in 0..count {
            // Randomly select from both planet cards and Black Hole
            let selected_card = game
                .rng
                .choose(&available_cards)
                .unwrap_or(&ConsumableId::Mercury);

            // Determine the correct consumable type based on the selected card
            let consumable_type = if *selected_card == ConsumableId::BlackHole {
                ConsumableType::Spectral
            } else {
                ConsumableType::Planet
            };

            let option = PackOption::new(
                ShopItem::Consumable(consumable_type),
                format!("{selected_card}"), // Use the specific card name
            );
            self.options.push(option);
        }

        Ok(())
    }

    /// Generate spectral pack options (spectral cards)
    fn generate_spectral_options(
        &mut self,
        count: usize,
        game: &Game,
        _config: &Config,
    ) -> Result<(), GameError> {
        // Use All pool for regular spectral packs (includes Soul and Black Hole)
        let spectral_cards = crate::consumables::SpectralPool::All.get_cards();

        for _ in 0..count {
            // Randomly select a specific spectral card for preview info
            let selected_spectral = game
                .rng
                .choose(&spectral_cards)
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
        let config = &game.config;
        let mut pack = Pack::new(pack_type, config);
        pack.generate_contents(game, config)?;
        Ok(pack)
    }

    fn is_pack_available(&self, pack_type: PackType, game: &Game) -> bool {
        // Basic availability logic - all packs available if player has money
        game.money >= pack_type.base_cost(&game.config) as f64
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
