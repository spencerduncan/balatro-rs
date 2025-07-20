use crate::joker::JokerId;
use std::collections::HashSet;

/// Represents different types of selectable items in the game
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SelectableItem {
    Card(usize),      // Card ID
    Joker(JokerId),   // Joker ID
    Pack(usize),      // Pack ID
    // Future: Voucher, Tarot, Planet, etc.
}

/// Selection limits for different types of operations
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct SelectionLimits {
    pub cards_max: usize,
    pub cards_min: usize,
    pub jokers_max: usize,
    pub jokers_min: usize,
    pub packs_max: usize,
    pub packs_min: usize,
}

impl Default for SelectionLimits {
    fn default() -> Self {
        Self {
            cards_max: 5,    // Standard hand size limit
            cards_min: 0,
            jokers_max: 1,   // Usually single joker operations
            jokers_min: 0,
            packs_max: 1,    // Usually single pack operations
            packs_min: 0,
        }
    }
}

/// Multi-select context that tracks selected items across different types
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Default)]
pub struct MultiSelectContext {
    /// Currently selected items organized by type
    selected_cards: HashSet<usize>,      // Card IDs
    selected_jokers: HashSet<JokerId>,   // Joker IDs  
    selected_packs: HashSet<usize>,      // Pack IDs
    
    /// Selection limits for current operation
    limits: SelectionLimits,
    
    /// Whether selection is currently active
    active: bool,
}

impl MultiSelectContext {
    /// Create a new multi-select context with default limits
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Create a new multi-select context with custom limits
    pub fn with_limits(limits: SelectionLimits) -> Self {
        Self {
            limits,
            ..Default::default()
        }
    }
    
    /// Activate multi-select mode
    pub fn activate(&mut self) {
        self.active = true;
    }
    
    /// Deactivate multi-select mode and clear all selections
    pub fn deactivate(&mut self) {
        self.active = false;
        self.clear_all();
    }
    
    /// Check if multi-select is currently active
    pub fn is_active(&self) -> bool {
        self.active
    }
    
    /// Select a card by ID
    pub fn select_card(&mut self, card_id: usize) -> Result<(), MultiSelectError> {
        if !self.active {
            return Err(MultiSelectError::NotActive);
        }
        
        if self.selected_cards.len() >= self.limits.cards_max {
            return Err(MultiSelectError::ExceedsLimit);
        }
        
        self.selected_cards.insert(card_id);
        Ok(())
    }
    
    /// Deselect a card by ID
    pub fn deselect_card(&mut self, card_id: usize) -> Result<(), MultiSelectError> {
        if !self.active {
            return Err(MultiSelectError::NotActive);
        }
        
        if !self.selected_cards.remove(&card_id) {
            return Err(MultiSelectError::NotSelected);
        }
        
        Ok(())
    }
    
    /// Toggle card selection
    pub fn toggle_card(&mut self, card_id: usize) -> Result<bool, MultiSelectError> {
        if self.selected_cards.contains(&card_id) {
            self.deselect_card(card_id)?;
            Ok(false) // Now deselected
        } else {
            self.select_card(card_id)?;
            Ok(true)  // Now selected
        }
    }
    
    /// Select multiple cards at once
    pub fn select_cards(&mut self, card_ids: Vec<usize>) -> Result<(), MultiSelectError> {
        if !self.active {
            return Err(MultiSelectError::NotActive);
        }
        
        // Check if adding these cards would exceed limit
        let total_after = self.selected_cards.len() + card_ids.len();
        if total_after > self.limits.cards_max {
            return Err(MultiSelectError::ExceedsLimit);
        }
        
        // Add all cards
        for card_id in card_ids {
            self.selected_cards.insert(card_id);
        }
        
        Ok(())
    }
    
    /// Select a joker by ID
    pub fn select_joker(&mut self, joker_id: JokerId) -> Result<(), MultiSelectError> {
        if !self.active {
            return Err(MultiSelectError::NotActive);
        }
        
        if self.selected_jokers.len() >= self.limits.jokers_max {
            return Err(MultiSelectError::ExceedsLimit);
        }
        
        self.selected_jokers.insert(joker_id);
        Ok(())
    }
    
    /// Deselect a joker by ID
    pub fn deselect_joker(&mut self, joker_id: JokerId) -> Result<(), MultiSelectError> {
        if !self.active {
            return Err(MultiSelectError::NotActive);
        }
        
        if !self.selected_jokers.remove(&joker_id) {
            return Err(MultiSelectError::NotSelected);
        }
        
        Ok(())
    }
    
    /// Toggle joker selection
    pub fn toggle_joker(&mut self, joker_id: JokerId) -> Result<bool, MultiSelectError> {
        if self.selected_jokers.contains(&joker_id) {
            self.deselect_joker(joker_id)?;
            Ok(false) // Now deselected
        } else {
            self.select_joker(joker_id)?;
            Ok(true)  // Now selected
        }
    }
    
    /// Select a pack by ID
    pub fn select_pack(&mut self, pack_id: usize) -> Result<(), MultiSelectError> {
        if !self.active {
            return Err(MultiSelectError::NotActive);
        }
        
        if self.selected_packs.len() >= self.limits.packs_max {
            return Err(MultiSelectError::ExceedsLimit);
        }
        
        self.selected_packs.insert(pack_id);
        Ok(())
    }
    
    /// Deselect a pack by ID
    pub fn deselect_pack(&mut self, pack_id: usize) -> Result<(), MultiSelectError> {
        if !self.active {
            return Err(MultiSelectError::NotActive);
        }
        
        if !self.selected_packs.remove(&pack_id) {
            return Err(MultiSelectError::NotSelected);
        }
        
        Ok(())
    }
    
    /// Clear all selections
    pub fn clear_all(&mut self) {
        self.selected_cards.clear();
        self.selected_jokers.clear();
        self.selected_packs.clear();
    }
    
    /// Clear card selections only
    pub fn clear_cards(&mut self) {
        self.selected_cards.clear();
    }
    
    /// Clear joker selections only
    pub fn clear_jokers(&mut self) {
        self.selected_jokers.clear();
    }
    
    /// Clear pack selections only
    pub fn clear_packs(&mut self) {
        self.selected_packs.clear();
    }
    
    /// Get selected card IDs
    pub fn selected_cards(&self) -> Vec<usize> {
        self.selected_cards.iter().copied().collect()
    }
    
    /// Get selected joker IDs
    pub fn selected_jokers(&self) -> Vec<JokerId> {
        self.selected_jokers.iter().copied().collect()
    }
    
    /// Get selected pack IDs
    pub fn selected_packs(&self) -> Vec<usize> {
        self.selected_packs.iter().copied().collect()
    }
    
    /// Check if a card is selected
    pub fn is_card_selected(&self, card_id: usize) -> bool {
        self.selected_cards.contains(&card_id)
    }
    
    /// Check if a joker is selected  
    pub fn is_joker_selected(&self, joker_id: JokerId) -> bool {
        self.selected_jokers.contains(&joker_id)
    }
    
    /// Check if a pack is selected
    pub fn is_pack_selected(&self, pack_id: usize) -> bool {
        self.selected_packs.contains(&pack_id)
    }
    
    /// Get total number of selected items across all types
    pub fn total_selected(&self) -> usize {
        self.selected_cards.len() + self.selected_jokers.len() + self.selected_packs.len()
    }
    
    /// Check if selections meet minimum requirements
    pub fn meets_minimum_requirements(&self) -> bool {
        self.selected_cards.len() >= self.limits.cards_min
            && self.selected_jokers.len() >= self.limits.jokers_min
            && self.selected_packs.len() >= self.limits.packs_min
    }
    
    /// Check if any items are selected
    pub fn has_selections(&self) -> bool {
        !self.selected_cards.is_empty() 
            || !self.selected_jokers.is_empty() 
            || !self.selected_packs.is_empty()
    }
    
    /// Update selection limits
    pub fn set_limits(&mut self, limits: SelectionLimits) {
        self.limits = limits;
    }
    
    /// Get current selection limits
    pub fn limits(&self) -> &SelectionLimits {
        &self.limits
    }
    
    /// Get all selected items as SelectableItem enum
    pub fn all_selected_items(&self) -> Vec<SelectableItem> {
        let mut items = Vec::new();
        
        for &card_id in &self.selected_cards {
            items.push(SelectableItem::Card(card_id));
        }
        
        for &joker_id in &self.selected_jokers {
            items.push(SelectableItem::Joker(joker_id));
        }
        
        for &pack_id in &self.selected_packs {
            items.push(SelectableItem::Pack(pack_id));
        }
        
        items
    }
    
    /// Range select cards (select all cards between two indices, inclusive)
    pub fn range_select_cards(&mut self, start_id: usize, end_id: usize, available_card_ids: &[usize]) -> Result<(), MultiSelectError> {
        if !self.active {
            return Err(MultiSelectError::NotActive);
        }
        
        // Find the indices of start and end cards in the available list
        let start_idx = available_card_ids.iter().position(|&id| id == start_id)
            .ok_or(MultiSelectError::InvalidRange)?;
        let end_idx = available_card_ids.iter().position(|&id| id == end_id)
            .ok_or(MultiSelectError::InvalidRange)?;
        
        // Ensure proper ordering
        let (min_idx, max_idx) = if start_idx <= end_idx {
            (start_idx, end_idx)
        } else {
            (end_idx, start_idx)
        };
        
        // Get cards in range
        let cards_in_range: Vec<usize> = available_card_ids[min_idx..=max_idx].to_vec();
        
        // Check if adding these would exceed limit
        let new_cards: Vec<usize> = cards_in_range.into_iter()
            .filter(|&id| !self.selected_cards.contains(&id))
            .collect();
            
        if self.selected_cards.len() + new_cards.len() > self.limits.cards_max {
            return Err(MultiSelectError::ExceedsLimit);
        }
        
        // Select all cards in range
        for card_id in new_cards {
            self.selected_cards.insert(card_id);
        }
        
        Ok(())
    }
}

/// Errors that can occur during multi-select operations
#[derive(Debug, Clone, PartialEq)]
pub enum MultiSelectError {
    /// Multi-select is not currently active
    NotActive,
    /// Selection would exceed configured limits
    ExceedsLimit,
    /// Item is not currently selected
    NotSelected,
    /// Invalid range for range selection
    InvalidRange,
}

impl std::fmt::Display for MultiSelectError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotActive => write!(f, "Multi-select is not currently active"),
            Self::ExceedsLimit => write!(f, "Selection would exceed configured limits"),
            Self::NotSelected => write!(f, "Item is not currently selected"),
            Self::InvalidRange => write!(f, "Invalid range for range selection"),
        }
    }
}

impl std::error::Error for MultiSelectError {}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_multi_select_cards() {
        let mut context = MultiSelectContext::new();
        context.activate();
        
        // Test single card selection
        assert!(context.select_card(1).is_ok());
        assert!(context.is_card_selected(1));
        assert_eq!(context.selected_cards(), vec![1]);
        
        // Test multiple card selection
        assert!(context.select_cards(vec![2, 3]).is_ok());
        let mut selected = context.selected_cards();
        selected.sort();
        assert_eq!(selected, vec![1, 2, 3]);
        
        // Test deselection
        assert!(context.deselect_card(2).is_ok());
        assert!(!context.is_card_selected(2));
        
        // Test toggle
        assert!(context.toggle_card(4).unwrap()); // Should select
        assert!(context.is_card_selected(4));
        assert!(!context.toggle_card(4).unwrap()); // Should deselect
        assert!(!context.is_card_selected(4));
    }
    
    #[test]
    fn test_selection_limits() {
        let limits = SelectionLimits {
            cards_max: 2,
            cards_min: 0,
            jokers_max: 1,
            jokers_min: 0,
            packs_max: 1,
            packs_min: 0,
        };
        
        let mut context = MultiSelectContext::with_limits(limits);
        context.activate();
        
        // Should allow up to 2 cards
        assert!(context.select_card(1).is_ok());
        assert!(context.select_card(2).is_ok());
        
        // Should reject third card
        assert_eq!(context.select_card(3), Err(MultiSelectError::ExceedsLimit));
    }
    
    #[test]
    fn test_range_selection() {
        let mut context = MultiSelectContext::new();
        context.activate();
        
        let available_cards = vec![1, 2, 3, 4, 5];
        
        // Select range from card 2 to card 4
        assert!(context.range_select_cards(2, 4, &available_cards).is_ok());
        
        let mut selected = context.selected_cards();
        selected.sort();
        assert_eq!(selected, vec![2, 3, 4]);
    }
    
    #[test]
    fn test_inactive_operations() {
        let mut context = MultiSelectContext::new();
        // Don't activate
        
        assert_eq!(context.select_card(1), Err(MultiSelectError::NotActive));
        assert_eq!(context.select_joker(JokerId::Joker), Err(MultiSelectError::NotActive));
    }
    
    #[test]
    fn test_clear_operations() {
        let mut context = MultiSelectContext::new();
        context.activate();
        
        // Add some selections
        context.select_card(1).unwrap();
        context.select_card(2).unwrap();
        context.select_joker(JokerId::Joker).unwrap();
        
        assert!(context.has_selections());
        
        // Clear just cards
        context.clear_cards();
        assert!(context.selected_cards().is_empty());
        assert!(!context.selected_jokers().is_empty());
        
        // Clear all
        context.clear_all();
        assert!(!context.has_selections());
    }
}