use crate::card::Card;
use crate::joker::JokerId;
use crate::multi_select::{MultiSelectContext, MultiSelectError, SelectionLimits};

/// Represents a collection of targets for batch operations
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct TargetCollection {
    pub cards: Vec<Card>,
    pub jokers: Vec<JokerId>,
    pub pack_ids: Vec<usize>,
}

impl TargetCollection {
    /// Create a new empty target collection
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a target collection with only cards
    pub fn cards_only(cards: Vec<Card>) -> Self {
        Self {
            cards,
            jokers: Vec::new(),
            pack_ids: Vec::new(),
        }
    }

    /// Create a target collection with only jokers
    pub fn jokers_only(jokers: Vec<JokerId>) -> Self {
        Self {
            cards: Vec::new(),
            jokers,
            pack_ids: Vec::new(),
        }
    }

    /// Check if the collection is empty
    pub fn is_empty(&self) -> bool {
        self.cards.is_empty() && self.jokers.is_empty() && self.pack_ids.is_empty()
    }

    /// Get total number of targets across all types
    pub fn total_count(&self) -> usize {
        self.cards.len() + self.jokers.len() + self.pack_ids.len()
    }

    /// Check if collection contains any cards
    pub fn has_cards(&self) -> bool {
        !self.cards.is_empty()
    }

    /// Check if collection contains any jokers
    pub fn has_jokers(&self) -> bool {
        !self.jokers.is_empty()
    }

    /// Check if collection contains any packs
    pub fn has_packs(&self) -> bool {
        !self.pack_ids.is_empty()
    }
}

/// Context for managing targets and selection state
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct TargetContext {
    /// Multi-select context for tracking selections
    multi_select: MultiSelectContext,

    /// Available cards that can be targeted
    available_cards: Vec<Card>,

    /// Available jokers that can be targeted
    available_jokers: Vec<JokerId>,

    /// Available pack IDs that can be targeted
    available_packs: Vec<usize>,

    /// Whether keyboard modifiers are currently pressed
    modifier_state: ModifierState,
}

/// Tracks keyboard modifier states for multi-select operations
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Default)]
pub struct ModifierState {
    pub ctrl_pressed: bool,  // Ctrl/Cmd for multi-select
    pub shift_pressed: bool, // Shift for range select
    pub alt_pressed: bool,   // Alt for alternative operations
}

impl TargetContext {
    /// Create a new target context
    pub fn new() -> Self {
        Self {
            multi_select: MultiSelectContext::new(),
            available_cards: Vec::new(),
            available_jokers: Vec::new(),
            available_packs: Vec::new(),
            modifier_state: ModifierState::default(),
        }
    }

    /// Create a target context with custom selection limits
    pub fn with_limits(limits: SelectionLimits) -> Self {
        Self {
            multi_select: MultiSelectContext::with_limits(limits),
            available_cards: Vec::new(),
            available_jokers: Vec::new(),
            available_packs: Vec::new(),
            modifier_state: ModifierState::default(),
        }
    }

    /// Update available cards
    pub fn set_available_cards(&mut self, cards: Vec<Card>) {
        self.available_cards = cards;
    }

    /// Update available jokers
    pub fn set_available_jokers(&mut self, jokers: Vec<JokerId>) {
        self.available_jokers = jokers;
    }

    /// Update available packs
    pub fn set_available_packs(&mut self, pack_ids: Vec<usize>) {
        self.available_packs = pack_ids;
    }

    /// Activate multi-select mode
    pub fn activate_multi_select(&mut self) {
        self.multi_select.activate();
    }

    /// Deactivate multi-select mode
    pub fn deactivate_multi_select(&mut self) {
        self.multi_select.deactivate();
    }

    /// Check if multi-select is active
    pub fn is_multi_select_active(&self) -> bool {
        self.multi_select.is_active()
    }

    /// Update modifier state
    pub fn set_modifier_state(&mut self, modifiers: ModifierState) {
        self.modifier_state = modifiers;
    }

    /// Handle card targeting based on current mode and modifiers
    pub fn target_card(&mut self, card: Card) -> Result<TargetingResult, TargetContextError> {
        // If multi-select is not active, use single-select behavior
        if !self.multi_select.is_active() {
            return Ok(TargetingResult::SingleTarget(card));
        }

        // Check if card is available for selection
        if !self.available_cards.iter().any(|c| c.id == card.id) {
            return Err(TargetContextError::CardNotAvailable(card));
        }

        // Handle based on modifier state
        if self.modifier_state.shift_pressed {
            // Range selection mode
            self.handle_range_selection(card)
        } else if self.modifier_state.ctrl_pressed {
            // Multi-select mode - toggle selection
            match self.multi_select.toggle_card(card.id) {
                Ok(selected) => {
                    if selected {
                        Ok(TargetingResult::CardSelected(card))
                    } else {
                        Ok(TargetingResult::CardDeselected(card))
                    }
                }
                Err(e) => Err(TargetContextError::MultiSelectError(e)),
            }
        } else {
            // Single click in multi-select mode - select this card and deselect others
            self.multi_select.clear_cards();
            match self.multi_select.select_card(card.id) {
                Ok(_) => Ok(TargetingResult::CardSelected(card)),
                Err(e) => Err(TargetContextError::MultiSelectError(e)),
            }
        }
    }

    /// Handle joker targeting
    pub fn target_joker(
        &mut self,
        joker_id: JokerId,
    ) -> Result<TargetingResult, TargetContextError> {
        if !self.multi_select.is_active() {
            return Ok(TargetingResult::SingleJokerTarget(joker_id));
        }

        // Check if joker is available
        if !self.available_jokers.contains(&joker_id) {
            return Err(TargetContextError::JokerNotAvailable(joker_id));
        }

        // Toggle joker selection
        match self.multi_select.toggle_joker(joker_id) {
            Ok(selected) => {
                if selected {
                    Ok(TargetingResult::JokerSelected(joker_id))
                } else {
                    Ok(TargetingResult::JokerDeselected(joker_id))
                }
            }
            Err(e) => Err(TargetContextError::MultiSelectError(e)),
        }
    }

    /// Handle range selection for cards
    fn handle_range_selection(
        &mut self,
        end_card: Card,
    ) -> Result<TargetingResult, TargetContextError> {
        // Find the last selected card as the start of the range
        let selected_cards = self.multi_select.selected_cards();

        if selected_cards.is_empty() {
            // No previous selection, just select this card
            match self.multi_select.select_card(end_card.id) {
                Ok(_) => Ok(TargetingResult::CardSelected(end_card)),
                Err(e) => Err(TargetContextError::MultiSelectError(e)),
            }
        } else {
            // Use the last selected card as the start of the range
            let start_card_id = selected_cards[selected_cards.len() - 1];
            let start_card = self
                .available_cards
                .iter()
                .find(|c| c.id == start_card_id)
                .ok_or(TargetContextError::InvalidRangeStart)?;

            // Get available card IDs in order
            let available_ids: Vec<usize> = self.available_cards.iter().map(|c| c.id).collect();

            // Perform range selection
            match self
                .multi_select
                .range_select_cards(start_card.id, end_card.id, &available_ids)
            {
                Ok(_) => {
                    let range_cards = self.get_cards_in_range(*start_card, end_card);
                    Ok(TargetingResult::RangeSelected {
                        start: *start_card,
                        end: end_card,
                        cards: range_cards,
                    })
                }
                Err(e) => Err(TargetContextError::MultiSelectError(e)),
            }
        }
    }

    /// Get all cards between start and end (inclusive)
    fn get_cards_in_range(&self, start: Card, end: Card) -> Vec<Card> {
        let start_idx = self.available_cards.iter().position(|c| c.id == start.id);
        let end_idx = self.available_cards.iter().position(|c| c.id == end.id);

        match (start_idx, end_idx) {
            (Some(start_i), Some(end_i)) => {
                let (min_i, max_i) = if start_i <= end_i {
                    (start_i, end_i)
                } else {
                    (end_i, start_i)
                };
                self.available_cards[min_i..=max_i].to_vec()
            }
            _ => vec![start, end], // Fallback if indices not found
        }
    }

    /// Get current target collection from selections
    pub fn get_target_collection(&self) -> TargetCollection {
        let selected_cards = self.multi_select.selected_cards();
        let selected_jokers = self.multi_select.selected_jokers();
        let selected_packs = self.multi_select.selected_packs();

        // Convert card IDs back to Card structs
        let cards: Vec<Card> = selected_cards
            .into_iter()
            .filter_map(|id| self.available_cards.iter().find(|c| c.id == id).copied())
            .collect();

        TargetCollection {
            cards,
            jokers: selected_jokers,
            pack_ids: selected_packs,
        }
    }

    /// Clear all selections
    pub fn clear_selections(&mut self) {
        self.multi_select.clear_all();
    }

    /// Get selection count for UI display
    pub fn get_selection_counts(&self) -> SelectionCounts {
        SelectionCounts {
            cards: self.multi_select.selected_cards().len(),
            jokers: self.multi_select.selected_jokers().len(),
            packs: self.multi_select.selected_packs().len(),
            total: self.multi_select.total_selected(),
        }
    }

    /// Check if current selections meet minimum requirements for an action
    pub fn meets_action_requirements(&self) -> bool {
        self.multi_select.meets_minimum_requirements()
    }

    /// Get the underlying multi-select context (for advanced operations)
    pub fn multi_select_context(&self) -> &MultiSelectContext {
        &self.multi_select
    }

    /// Get mutable access to multi-select context (for advanced operations)
    pub fn multi_select_context_mut(&mut self) -> &mut MultiSelectContext {
        &mut self.multi_select
    }
}

impl Default for TargetContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of a targeting operation
#[derive(Debug, Clone, PartialEq)]
pub enum TargetingResult {
    /// Single card was targeted (not in multi-select mode)
    SingleTarget(Card),
    /// Single joker was targeted (not in multi-select mode)
    SingleJokerTarget(JokerId),
    /// Card was selected in multi-select mode
    CardSelected(Card),
    /// Card was deselected in multi-select mode
    CardDeselected(Card),
    /// Joker was selected in multi-select mode
    JokerSelected(JokerId),
    /// Joker was deselected in multi-select mode
    JokerDeselected(JokerId),
    /// Range of cards was selected
    RangeSelected {
        start: Card,
        end: Card,
        cards: Vec<Card>,
    },
}

/// Selection counts for UI display
#[derive(Debug, Clone, PartialEq)]
pub struct SelectionCounts {
    pub cards: usize,
    pub jokers: usize,
    pub packs: usize,
    pub total: usize,
}

/// Errors that can occur in targeting operations
#[derive(Debug, Clone, PartialEq)]
pub enum TargetContextError {
    /// Card is not available for targeting
    CardNotAvailable(Card),
    /// Joker is not available for targeting
    JokerNotAvailable(JokerId),
    /// Multi-select operation failed
    MultiSelectError(MultiSelectError),
    /// Invalid range selection
    InvalidRangeStart,
}

impl std::fmt::Display for TargetContextError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CardNotAvailable(card) => {
                write!(f, "Card {card} is not available for targeting")
            }
            Self::JokerNotAvailable(joker_id) => {
                write!(f, "Joker {joker_id:?} is not available for targeting")
            }
            Self::MultiSelectError(e) => write!(f, "Multi-select error: {e}"),
            Self::InvalidRangeStart => write!(f, "Invalid range selection start"),
        }
    }
}

impl std::error::Error for TargetContextError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::MultiSelectError(e) => Some(e),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::{Card, Suit, Value};

    #[test]
    fn test_single_targeting() {
        let mut context = TargetContext::new();
        let card = Card::new(Value::Ace, Suit::Heart);

        // Should use single-target mode when multi-select is not active
        let result = context.target_card(card).unwrap();
        assert_eq!(result, TargetingResult::SingleTarget(card));
    }

    #[test]
    fn test_multi_select_targeting() {
        let mut context = TargetContext::new();
        let card1 = Card::new(Value::Ace, Suit::Heart);
        let card2 = Card::new(Value::King, Suit::Diamond);

        context.set_available_cards(vec![card1, card2]);
        context.activate_multi_select();

        // First click should select card
        let result = context.target_card(card1).unwrap();
        assert_eq!(result, TargetingResult::CardSelected(card1));

        // Click with Ctrl should toggle
        context.modifier_state.ctrl_pressed = true;
        let result = context.target_card(card1).unwrap();
        assert_eq!(result, TargetingResult::CardDeselected(card1));
    }

    #[test]
    fn test_target_collection() {
        let mut context = TargetContext::new();
        let card1 = Card::new(Value::Ace, Suit::Heart);
        let card2 = Card::new(Value::King, Suit::Diamond);

        context.set_available_cards(vec![card1, card2]);
        context.activate_multi_select();

        // Select both cards
        context.target_card(card1).unwrap();
        context.modifier_state.ctrl_pressed = true;
        context.target_card(card2).unwrap();

        let collection = context.get_target_collection();
        assert_eq!(collection.cards.len(), 2);
        assert!(collection.cards.contains(&card1));
        assert!(collection.cards.contains(&card2));
    }

    #[test]
    fn test_selection_counts() {
        let mut context = TargetContext::new();
        let card1 = Card::new(Value::Ace, Suit::Heart);
        let joker1 = JokerId::Joker;

        context.set_available_cards(vec![card1]);
        context.set_available_jokers(vec![joker1]);
        context.activate_multi_select();

        context.target_card(card1).unwrap();
        context.target_joker(joker1).unwrap();

        let counts = context.get_selection_counts();
        assert_eq!(counts.cards, 1);
        assert_eq!(counts.jokers, 1);
        assert_eq!(counts.total, 2);
    }
}
