/// Card filtering infrastructure for Balatro card selection
///
/// This module provides a foundational system for filtering cards based on various criteria.
/// All filter functionality builds upon these core traits and types.
use crate::card::{Card, Edition, Suit, Value};
use crate::game::Game;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Context information passed to card filters
/// Provides access to game state and other contextual information needed for filtering
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct FilterContext {
    /// Current game state (optional - not all filters need game context)
    pub game_state: Option<GameStateSnapshot>,
    /// Additional context data for advanced filtering
    pub metadata: FilterMetadata,
}

/// Snapshot of relevant game state for filtering
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct GameStateSnapshot {
    /// Current ante level
    pub ante: u8,
    /// Player's current money
    pub money: i32,
    /// Number of jokers owned
    pub joker_count: usize,
}

/// Additional metadata for filtering context
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct FilterMetadata {
    /// Custom properties for extensibility
    pub properties: std::collections::HashMap<String, String>,
}

impl FilterContext {
    /// Create a new FilterContext with minimal data
    pub fn new() -> Self {
        Self {
            game_state: None,
            metadata: FilterMetadata {
                properties: std::collections::HashMap::new(),
            },
        }
    }

    /// Create a FilterContext from game state
    pub fn from_game(game: &Game) -> Self {
        Self {
            game_state: Some(GameStateSnapshot {
                ante: match game.ante_current {
                    crate::ante::Ante::Zero => 0,
                    crate::ante::Ante::One => 1,
                    crate::ante::Ante::Two => 2,
                    crate::ante::Ante::Three => 3,
                    crate::ante::Ante::Four => 4,
                    crate::ante::Ante::Five => 5,
                    crate::ante::Ante::Six => 6,
                    crate::ante::Ante::Seven => 7,
                    crate::ante::Ante::Eight => 8,
                },
                money: game.money as i32,
                joker_count: game.jokers.len(),
            }),
            metadata: FilterMetadata {
                properties: std::collections::HashMap::new(),
            },
        }
    }

    /// Add a metadata property
    pub fn with_property(mut self, key: String, value: String) -> Self {
        self.metadata.properties.insert(key, value);
        self
    }
}

impl Default for FilterContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Core trait for card filtering
///
/// All card filters must implement this trait. The `matches` method determines
/// whether a given card satisfies the filter criteria.
pub trait CardFilter {
    /// Test if a card matches this filter's criteria
    ///
    /// # Arguments
    /// * `card` - The card to test
    /// * `context` - Optional context information for advanced filtering
    ///
    /// # Returns
    /// `true` if the card matches the filter, `false` otherwise
    fn matches(&self, card: &Card, context: &FilterContext) -> bool;

    /// Get a human-readable description of this filter
    fn description(&self) -> String {
        "Card filter".to_string()
    }
}

/// Enumeration of basic card filter types
///
/// These are the fundamental filter types that can be composed together
/// for more complex filtering logic.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CardFilterType {
    /// Filter cards based on enhancement status
    Enhanced,
    /// Filter cards based on rarity (mapped to edition for now)
    Rarity(Edition),
    /// Filter cards based on suit
    Suit(Suit),
    /// Filter cards based on rank/value
    Rank(Value),
    /// Filter cards based on edition
    Edition(Edition),
    /// Filter cards based on seal presence
    Sealed,
    /// Filter face cards (Jack, Queen, King)
    Face,
    /// Filter even-valued cards
    Even,
    /// Filter odd-valued cards
    Odd,
}

impl CardFilter for CardFilterType {
    fn matches(&self, card: &Card, _context: &FilterContext) -> bool {
        match self {
            CardFilterType::Enhanced => card.enhancement.is_some(),
            CardFilterType::Rarity(edition) => card.edition == *edition,
            CardFilterType::Suit(suit) => card.suit == *suit,
            CardFilterType::Rank(value) => card.value == *value,
            CardFilterType::Edition(edition) => card.edition == *edition,
            CardFilterType::Sealed => card.seal.is_some(),
            CardFilterType::Face => card.is_face(),
            CardFilterType::Even => card.is_even(),
            CardFilterType::Odd => card.is_odd(),
        }
    }

    fn description(&self) -> String {
        match self {
            CardFilterType::Enhanced => "Enhanced cards".to_string(),
            CardFilterType::Rarity(edition) => format!("Cards with {edition:?} rarity"),
            CardFilterType::Suit(suit) => format!("Cards of suit {suit:?}"),
            CardFilterType::Rank(value) => format!("Cards with rank {value:?}"),
            CardFilterType::Edition(edition) => format!("Cards with {edition:?} edition"),
            CardFilterType::Sealed => "Cards with seals".to_string(),
            CardFilterType::Face => "Face cards (J, Q, K)".to_string(),
            CardFilterType::Even => "Even-valued cards".to_string(),
            CardFilterType::Odd => "Odd-valued cards".to_string(),
        }
    }
}

/// Composite filter that combines multiple filters with logical operators
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub enum CompositeFilter {
    /// All filters must match (AND logic)
    All(Vec<CardFilterType>),
    /// Any filter must match (OR logic)
    Any(Vec<CardFilterType>),
    /// Filter must not match (NOT logic)
    Not(Box<CardFilterType>),
}

impl CardFilter for CompositeFilter {
    fn matches(&self, card: &Card, context: &FilterContext) -> bool {
        match self {
            CompositeFilter::All(filters) => {
                filters.iter().all(|filter| filter.matches(card, context))
            }
            CompositeFilter::Any(filters) => {
                filters.iter().any(|filter| filter.matches(card, context))
            }
            CompositeFilter::Not(filter) => !filter.matches(card, context),
        }
    }

    fn description(&self) -> String {
        match self {
            CompositeFilter::All(filters) => {
                let descriptions: Vec<String> = filters.iter().map(|f| f.description()).collect();
                format!("All of: [{}]", descriptions.join(", "))
            }
            CompositeFilter::Any(filters) => {
                let descriptions: Vec<String> = filters.iter().map(|f| f.description()).collect();
                format!("Any of: [{}]", descriptions.join(", "))
            }
            CompositeFilter::Not(filter) => {
                format!("Not: {}", filter.description())
            }
        }
    }
}

/// Convenience functions for creating common filters
impl CardFilterType {
    /// Create a filter for enhanced cards
    pub fn enhanced() -> Self {
        CardFilterType::Enhanced
    }

    /// Create a filter for cards of a specific suit
    pub fn of_suit(suit: Suit) -> Self {
        CardFilterType::Suit(suit)
    }

    /// Create a filter for cards of a specific rank
    pub fn of_rank(value: Value) -> Self {
        CardFilterType::Rank(value)
    }

    /// Create a filter for cards with a specific edition
    pub fn of_edition(edition: Edition) -> Self {
        CardFilterType::Edition(edition)
    }

    /// Create a filter for sealed cards
    pub fn sealed() -> Self {
        CardFilterType::Sealed
    }

    /// Create a filter for face cards
    pub fn face() -> Self {
        CardFilterType::Face
    }

    /// Create a filter for even-valued cards
    pub fn even() -> Self {
        CardFilterType::Even
    }

    /// Create a filter for odd-valued cards
    pub fn odd() -> Self {
        CardFilterType::Odd
    }
}

/// Filter a collection of cards using the provided filter
///
/// # Arguments
/// * `cards` - Iterator over cards to filter
/// * `filter` - The filter to apply
/// * `context` - Context information for filtering
///
/// # Returns
/// A vector containing only the cards that match the filter
pub fn filter_cards<'a, I, F>(cards: I, filter: &F, context: &FilterContext) -> Vec<&'a Card>
where
    I: Iterator<Item = &'a Card>,
    F: CardFilter,
{
    cards.filter(|card| filter.matches(card, context)).collect()
}

/// Registry for managing and creating card filters
///
/// Provides a centralized way to create, register, and retrieve card filters.
/// Supports both predefined filter types and custom filter definitions.
#[derive(Debug, Clone)]
pub struct CardFilterRegistry {
    /// Named filters that can be referenced by string
    named_filters: std::collections::HashMap<String, CardFilterType>,
    /// Composite filters for complex logic
    composite_filters: std::collections::HashMap<String, CompositeFilter>,
}

impl CardFilterRegistry {
    /// Create a new empty registry
    pub fn new() -> Self {
        Self {
            named_filters: std::collections::HashMap::new(),
            composite_filters: std::collections::HashMap::new(),
        }
    }

    /// Create a registry with default filters pre-registered
    pub fn with_defaults() -> Self {
        let mut registry = Self::new();

        // Register common basic filters
        registry.register("enhanced", CardFilterType::enhanced());
        registry.register("sealed", CardFilterType::sealed());
        registry.register("face", CardFilterType::face());
        registry.register("even", CardFilterType::even());
        registry.register("odd", CardFilterType::odd());

        // Register suit filters
        registry.register("hearts", CardFilterType::of_suit(Suit::Heart));
        registry.register("diamonds", CardFilterType::of_suit(Suit::Diamond));
        registry.register("clubs", CardFilterType::of_suit(Suit::Club));
        registry.register("spades", CardFilterType::of_suit(Suit::Spade));

        // Register edition filters
        registry.register("foil", CardFilterType::of_edition(Edition::Foil));
        registry.register(
            "holographic",
            CardFilterType::of_edition(Edition::Holographic),
        );
        registry.register(
            "polychrome",
            CardFilterType::of_edition(Edition::Polychrome),
        );
        registry.register("negative", CardFilterType::of_edition(Edition::Negative));

        // Register some useful composite filters
        registry.register_composite(
            "red_cards",
            CompositeFilter::Any(vec![
                CardFilterType::of_suit(Suit::Heart),
                CardFilterType::of_suit(Suit::Diamond),
            ]),
        );
        registry.register_composite(
            "black_cards",
            CompositeFilter::Any(vec![
                CardFilterType::of_suit(Suit::Club),
                CardFilterType::of_suit(Suit::Spade),
            ]),
        );
        registry.register_composite(
            "special_edition",
            CompositeFilter::Not(Box::new(CardFilterType::of_edition(Edition::Base))),
        );

        registry
    }

    /// Register a basic filter with a name
    pub fn register(&mut self, name: &str, filter: CardFilterType) {
        self.named_filters.insert(name.to_string(), filter);
    }

    /// Register a composite filter with a name
    pub fn register_composite(&mut self, name: &str, filter: CompositeFilter) {
        self.composite_filters.insert(name.to_string(), filter);
    }

    /// Get a basic filter by name
    pub fn get_filter(&self, name: &str) -> Option<&CardFilterType> {
        self.named_filters.get(name)
    }

    /// Get a composite filter by name
    pub fn get_composite_filter(&self, name: &str) -> Option<&CompositeFilter> {
        self.composite_filters.get(name)
    }

    /// Get all registered basic filter names
    pub fn list_filters(&self) -> Vec<&String> {
        self.named_filters.keys().collect()
    }

    /// Get all registered composite filter names
    pub fn list_composite_filters(&self) -> Vec<&String> {
        self.composite_filters.keys().collect()
    }

    /// Create a filter from a string definition
    ///
    /// Supports basic syntax like:
    /// - "enhanced" - basic named filter
    /// - "hearts" - suit filter
    /// - "face AND enhanced" - composite filter (future extension)
    pub fn create_from_string(&self, definition: &str) -> Result<Box<dyn CardFilter>, FilterError> {
        let trimmed = definition.trim().to_lowercase();

        // Try basic filter first
        if let Some(filter) = self.get_filter(&trimmed) {
            return Ok(Box::new(filter.clone()));
        }

        // Try composite filter
        if let Some(filter) = self.get_composite_filter(&trimmed) {
            return Ok(Box::new(filter.clone()));
        }

        // Could extend this to parse complex expressions in the future
        Err(FilterError::UnknownFilter(definition.to_string()))
    }

    /// Apply a named filter to a collection of cards
    pub fn apply_filter<'a>(
        &self,
        name: &str,
        cards: &'a [Card],
        context: &FilterContext,
    ) -> Result<Vec<&'a Card>, FilterError> {
        if let Some(filter) = self.get_filter(name) {
            Ok(filter_cards(cards.iter(), filter, context))
        } else if let Some(filter) = self.get_composite_filter(name) {
            Ok(filter_cards(cards.iter(), filter, context))
        } else {
            Err(FilterError::UnknownFilter(name.to_string()))
        }
    }
}

impl Default for CardFilterRegistry {
    fn default() -> Self {
        Self::with_defaults()
    }
}

/// Errors that can occur during filter operations
#[derive(Debug, Clone, PartialEq)]
pub enum FilterError {
    /// Filter name was not found in the registry
    UnknownFilter(String),
    /// Invalid filter definition syntax
    InvalidDefinition(String),
}

impl std::fmt::Display for FilterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FilterError::UnknownFilter(name) => write!(f, "Unknown filter: {name}"),
            FilterError::InvalidDefinition(def) => write!(f, "Invalid filter definition: {def}"),
        }
    }
}

impl std::error::Error for FilterError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::{Card, Edition, Enhancement, Seal, Suit, Value};

    fn create_test_cards() -> Vec<Card> {
        vec![
            Card {
                value: Value::Ace,
                suit: Suit::Heart,
                id: 1,
                edition: Edition::Base,
                enhancement: Some(Enhancement::Bonus),
                seal: None,
            },
            Card {
                value: Value::King,
                suit: Suit::Spade,
                id: 2,
                edition: Edition::Foil,
                enhancement: None,
                seal: Some(Seal::Gold),
            },
            Card {
                value: Value::Two,
                suit: Suit::Diamond,
                id: 3,
                edition: Edition::Base,
                enhancement: None,
                seal: None,
            },
            Card {
                value: Value::Queen,
                suit: Suit::Club,
                id: 4,
                edition: Edition::Holographic,
                enhancement: Some(Enhancement::Wild),
                seal: Some(Seal::Red),
            },
        ]
    }

    #[test]
    fn test_enhanced_filter() {
        let cards = create_test_cards();
        let filter = CardFilterType::enhanced();
        let context = FilterContext::new();

        let enhanced_cards = filter_cards(cards.iter(), &filter, &context);
        assert_eq!(enhanced_cards.len(), 2); // Ace and Queen have enhancements
    }

    #[test]
    fn test_suit_filter() {
        let cards = create_test_cards();
        let filter = CardFilterType::of_suit(Suit::Heart);
        let context = FilterContext::new();

        let heart_cards = filter_cards(cards.iter(), &filter, &context);
        assert_eq!(heart_cards.len(), 1); // Only Ace of Hearts
        assert_eq!(heart_cards[0].value, Value::Ace);
    }

    #[test]
    fn test_rank_filter() {
        let cards = create_test_cards();
        let filter = CardFilterType::of_rank(Value::King);
        let context = FilterContext::new();

        let king_cards = filter_cards(cards.iter(), &filter, &context);
        assert_eq!(king_cards.len(), 1); // Only King of Spades
        assert_eq!(king_cards[0].suit, Suit::Spade);
    }

    #[test]
    fn test_edition_filter() {
        let cards = create_test_cards();
        let filter = CardFilterType::of_edition(Edition::Foil);
        let context = FilterContext::new();

        let foil_cards = filter_cards(cards.iter(), &filter, &context);
        assert_eq!(foil_cards.len(), 1); // Only King has Foil edition
    }

    #[test]
    fn test_sealed_filter() {
        let cards = create_test_cards();
        let filter = CardFilterType::sealed();
        let context = FilterContext::new();

        let sealed_cards = filter_cards(cards.iter(), &filter, &context);
        assert_eq!(sealed_cards.len(), 2); // King and Queen have seals
    }

    #[test]
    fn test_face_filter() {
        let cards = create_test_cards();
        let filter = CardFilterType::face();
        let context = FilterContext::new();

        let face_cards = filter_cards(cards.iter(), &filter, &context);
        assert_eq!(face_cards.len(), 2); // King and Queen are face cards
    }

    #[test]
    fn test_even_filter() {
        let cards = create_test_cards();
        let filter = CardFilterType::even();
        let context = FilterContext::new();

        let even_cards = filter_cards(cards.iter(), &filter, &context);
        assert_eq!(even_cards.len(), 1); // Only Two is even
        assert_eq!(even_cards[0].value, Value::Two);
    }

    #[test]
    fn test_odd_filter() {
        let cards = create_test_cards();
        let filter = CardFilterType::odd();
        let context = FilterContext::new();

        let odd_cards = filter_cards(cards.iter(), &filter, &context);
        assert_eq!(odd_cards.len(), 1); // Only Ace is odd
        assert_eq!(odd_cards[0].value, Value::Ace);
    }

    #[test]
    fn test_composite_all_filter() {
        let cards = create_test_cards();
        let filter = CompositeFilter::All(vec![CardFilterType::enhanced(), CardFilterType::face()]);
        let context = FilterContext::new();

        let matching_cards = filter_cards(cards.iter(), &filter, &context);
        assert_eq!(matching_cards.len(), 1); // Only Queen is enhanced and face card
        assert_eq!(matching_cards[0].value, Value::Queen);
    }

    #[test]
    fn test_composite_any_filter() {
        let cards = create_test_cards();
        let filter = CompositeFilter::Any(vec![
            CardFilterType::of_suit(Suit::Heart),
            CardFilterType::of_suit(Suit::Spade),
        ]);
        let context = FilterContext::new();

        let matching_cards = filter_cards(cards.iter(), &filter, &context);
        assert_eq!(matching_cards.len(), 2); // Ace of Hearts and King of Spades
    }

    #[test]
    fn test_composite_not_filter() {
        let cards = create_test_cards();
        let filter = CompositeFilter::Not(Box::new(CardFilterType::enhanced()));
        let context = FilterContext::new();

        let matching_cards = filter_cards(cards.iter(), &filter, &context);
        assert_eq!(matching_cards.len(), 2); // King and Two are not enhanced
    }

    #[test]
    fn test_filter_context_creation() {
        let context = FilterContext::new();
        assert!(context.game_state.is_none());
        assert!(context.metadata.properties.is_empty());
    }

    #[test]
    fn test_filter_context_with_properties() {
        let context =
            FilterContext::new().with_property("test_key".to_string(), "test_value".to_string());

        assert_eq!(
            context.metadata.properties.get("test_key"),
            Some(&"test_value".to_string())
        );
    }

    #[test]
    fn test_filter_descriptions() {
        assert_eq!(CardFilterType::enhanced().description(), "Enhanced cards");
        assert_eq!(
            CardFilterType::of_suit(Suit::Heart).description(),
            "Cards of suit Heart"
        );
        assert_eq!(CardFilterType::face().description(), "Face cards (J, Q, K)");
    }

    #[test]
    fn test_registry_creation() {
        let registry = CardFilterRegistry::new();
        assert!(registry.list_filters().is_empty());
        assert!(registry.list_composite_filters().is_empty());
    }

    #[test]
    fn test_registry_with_defaults() {
        let registry = CardFilterRegistry::with_defaults();
        let filters = registry.list_filters();
        let composite_filters = registry.list_composite_filters();

        // Should have basic filters
        assert!(filters.iter().any(|name| *name == "enhanced"));
        assert!(filters.iter().any(|name| *name == "hearts"));
        assert!(filters.iter().any(|name| *name == "face"));

        // Should have composite filters
        assert!(composite_filters.iter().any(|name| *name == "red_cards"));
        assert!(composite_filters.iter().any(|name| *name == "black_cards"));
    }

    #[test]
    fn test_registry_register_and_get() {
        let mut registry = CardFilterRegistry::new();
        registry.register("test_filter", CardFilterType::enhanced());

        assert!(registry.get_filter("test_filter").is_some());
        assert!(registry.get_filter("nonexistent").is_none());
    }

    #[test]
    fn test_registry_apply_filter() {
        let cards = create_test_cards();
        let registry = CardFilterRegistry::with_defaults();
        let context = FilterContext::new();

        let result = registry.apply_filter("enhanced", &cards, &context);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 2); // Two enhanced cards

        let result = registry.apply_filter("nonexistent", &cards, &context);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            FilterError::UnknownFilter("nonexistent".to_string())
        );
    }

    #[test]
    fn test_registry_create_from_string() {
        let registry = CardFilterRegistry::with_defaults();

        let result = registry.create_from_string("enhanced");
        assert!(result.is_ok());

        let result = registry.create_from_string("red_cards");
        assert!(result.is_ok());

        let result = registry.create_from_string("invalid_filter");
        assert!(result.is_err());
    }

    #[test]
    fn test_filter_error_display() {
        let error = FilterError::UnknownFilter("test".to_string());
        assert_eq!(error.to_string(), "Unknown filter: test");

        let error = FilterError::InvalidDefinition("bad syntax".to_string());
        assert_eq!(error.to_string(), "Invalid filter definition: bad syntax");
    }

    #[test]
    fn test_red_and_black_cards_composite_filters() {
        let cards = create_test_cards();
        let registry = CardFilterRegistry::with_defaults();
        let context = FilterContext::new();

        // Test red cards filter (hearts and diamonds)
        let red_cards = registry
            .apply_filter("red_cards", &cards, &context)
            .unwrap();
        assert_eq!(red_cards.len(), 2); // Ace of Hearts and Two of Diamonds

        // Test black cards filter (clubs and spades)
        let black_cards = registry
            .apply_filter("black_cards", &cards, &context)
            .unwrap();
        assert_eq!(black_cards.len(), 2); // King of Spades and Queen of Clubs
    }

    #[test]
    fn test_special_edition_filter() {
        let cards = create_test_cards();
        let registry = CardFilterRegistry::with_defaults();
        let context = FilterContext::new();

        let special_cards = registry
            .apply_filter("special_edition", &cards, &context)
            .unwrap();
        assert_eq!(special_cards.len(), 2); // King (Foil) and Queen (Holographic) are not Base edition
    }
}
