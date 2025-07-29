/// Tag selection system for skip tags
/// 
/// This module manages the tag selection process when a player skips a blind.
/// It handles tag availability filtering, weighted selection, and state management.

use crate::game::Game;
use crate::skip_tags::tag_error::TagError;
use crate::skip_tags::tag_registry::get_global_registry;
use crate::skip_tags::tag_trait::TagId;
use crate::skip_tags::tags::get_tag_rarity;
use crate::rng::GameRng;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// State management for skip tag selection process
/// 
/// This struct manages the tag selection UI state and available options
/// when a player chooses to skip a blind.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct TagSelectionState {
    /// Tags available for selection (typically 2-4 options)
    pub available_tags: Vec<TagId>,
    
    /// Whether the player has already made a selection
    pub selection_made: bool,
    
    /// The selected tag (if any)
    pub selected_tag: Option<TagId>,
    
    /// Timestamp when selection started (for performance monitoring)
    #[cfg_attr(feature = "serde", serde(skip))]
    selection_start_time: Option<std::time::Instant>,
}

impl TagSelectionState {
    /// Create a new tag selection state
    pub fn new() -> Self {
        Self {
            available_tags: Vec::new(),
            selection_made: false,
            selected_tag: None,
            selection_start_time: None,
        }
    }
    
    /// Initialize tag selection with available options based on game state
    /// 
    /// This method performs weighted random selection to choose 2-4 tags
    /// that the player can choose from. The selection considers:
    /// - Tag availability conditions
    /// - Tag rarity weights
    /// - Game state constraints
    /// 
    /// # Performance Requirements
    /// - Must complete in <1ms for tag selection
    pub fn initialize_selection(&mut self, game_state: &Game) -> Result<(), TagError> {
        let rng = game_state.rng.clone();
        self.initialize_selection_with_rng(rng)
    }

    /// Initialize tag selection with provided RNG to avoid borrowing issues
    pub fn initialize_selection_with_rng(&mut self, mut rng: GameRng) -> Result<(), TagError> {
        self.selection_start_time = Some(std::time::Instant::now());
        
        // Get all available tags from registry - using stub tags for now
        let registry = get_global_registry()?;
        let available_tag_ids = registry.get_all_tag_ids()?;
        
        if available_tag_ids.is_empty() {
            return Err(TagError::validation("No tags available for selection"));
        }
        
        // For now, just use the first few available tags as candidates
        // In a full implementation, this would filter based on game state conditions
        let mut candidate_tags = available_tag_ids;
        
        // Apply weighted selection based on rarity
        candidate_tags = self.apply_weighted_selection_simple(candidate_tags)?;
        
        // Limit to 2-4 options for reasonable choice complexity
        let selection_count = std::cmp::min(candidate_tags.len(), 4);
        let selection_count = std::cmp::max(selection_count, 2);
        
        // Perform weighted random sampling
        self.available_tags = self.select_weighted_random(candidate_tags, selection_count, &mut rng)?;
        
        self.selection_made = false;
        self.selected_tag = None;
        
        // Performance check
        if let Some(start_time) = self.selection_start_time {
            let duration = start_time.elapsed();
            if duration.as_millis() > 1 {
                eprintln!(
                    "WARNING: Tag selection initialization took {}ms (target: <1ms)", 
                    duration.as_millis()
                );
            }
        }
        
        Ok(())
    }
    
    /// Select a tag from the available options
    pub fn select_tag(&mut self, tag_id: TagId) -> Result<(), TagError> {
        if self.selection_made {
            return Err(TagError::validation("Selection has already been made"));
        }
        
        if !self.available_tags.contains(&tag_id) {
            return Err(TagError::validation(format!(
                "Tag {:?} is not available for selection", tag_id
            )));
        }
        
        self.selected_tag = Some(tag_id);
        self.selection_made = true;
        
        Ok(())
    }
    
    /// Check if selection process is complete
    pub fn is_complete(&self) -> bool {
        self.selection_made && self.selected_tag.is_some()
    }
    
    /// Get the selected tag, if any
    pub fn get_selected_tag(&self) -> Option<TagId> {
        self.selected_tag
    }
    
    /// Reset the selection state for a new selection
    pub fn reset(&mut self) {
        self.available_tags.clear();
        self.selection_made = false;
        self.selected_tag = None;
        self.selection_start_time = None;
    }
    
    /// Apply weighted selection based on tag rarity and game conditions
    fn apply_weighted_selection(&self, tags: Vec<TagId>, _game_state: &Game) -> Result<Vec<TagId>, TagError> {
        self.apply_weighted_selection_simple(tags)
    }

    /// Apply weighted selection based on tag rarity (simplified version)
    fn apply_weighted_selection_simple(&self, mut tags: Vec<TagId>) -> Result<Vec<TagId>, TagError> {
        // Sort by rarity weight (higher weights first)
        tags.sort_by(|a, b| {
            let weight_a = get_tag_rarity(*a).selection_weight();
            let weight_b = get_tag_rarity(*b).selection_weight();
            weight_b.partial_cmp(&weight_a).unwrap_or(std::cmp::Ordering::Equal)
        });
        
        Ok(tags)
    }
    
    /// Perform weighted random sampling from candidate tags
    fn select_weighted_random(
        &self, 
        candidates: Vec<TagId>, 
        count: usize,
        rng: &mut GameRng
    ) -> Result<Vec<TagId>, TagError> {
        if candidates.len() <= count {
            return Ok(candidates);
        }
        
        // Create weighted choices based on rarity
        let weights: Vec<f32> = candidates
            .iter()
            .map(|&tag_id| get_tag_rarity(tag_id).selection_weight())
            .collect();
        
        // Use weighted selection
        let selected_indices = weighted_sample_without_replacement(&candidates, &weights, count, rng)?;
        
        let selected_tags = selected_indices
            .into_iter()
            .map(|idx| candidates[idx])
            .collect();
        
        Ok(selected_tags)
    }
}

impl Default for TagSelectionState {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of tag selection and application
#[derive(Debug, Clone)]
pub struct TagSelectionResult {
    /// The selected tag
    pub selected_tag: TagId,
    
    /// Whether the tag was successfully applied
    pub applied_successfully: bool,
    
    /// Any error that occurred during application
    pub application_error: Option<TagError>,
    
    /// Duration of the selection process
    pub selection_duration: std::time::Duration,
    
    /// Duration of the tag application process
    pub application_duration: std::time::Duration,
}

impl TagSelectionResult {
    /// Create a successful result
    pub fn success(
        selected_tag: TagId, 
        selection_duration: std::time::Duration,
        application_duration: std::time::Duration
    ) -> Self {
        Self {
            selected_tag,
            applied_successfully: true,
            application_error: None,
            selection_duration,
            application_duration,
        }
    }
    
    /// Create a failed result
    pub fn failure(
        selected_tag: TagId,
        error: TagError,
        selection_duration: std::time::Duration,
        application_duration: std::time::Duration
    ) -> Self {
        Self {
            selected_tag,
            applied_successfully: false,
            application_error: Some(error),
            selection_duration,
            application_duration,
        }
    }
}

/// Weighted sampling without replacement
/// 
/// This is a simplified implementation for tag selection.
/// In a production system, you might want to use a more sophisticated algorithm.
fn weighted_sample_without_replacement<T: Clone>(
    items: &[T],
    weights: &[f32],
    count: usize,
    _rng: &mut GameRng
) -> Result<Vec<usize>, TagError> {
    if items.len() != weights.len() {
        return Err(TagError::validation("Items and weights length mismatch"));
    }
    
    if count > items.len() {
        return Err(TagError::validation("Cannot sample more items than available"));
    }
    
    // Simple approach: Create weighted choices and sample
    let mut choices: Vec<(usize, f32)> = weights
        .iter()
        .enumerate()
        .map(|(idx, &weight)| (idx, weight))
        .collect();
    
    // Simple shuffle based on GameRng - just take the top weighted choices
    // This is a simplified approach for now
    choices.sort_by(|(_, w1), (_, w2)| w2.partial_cmp(w1).unwrap_or(std::cmp::Ordering::Equal));
    
    // Take the top choices weighted by probability  
    let selected_indices = choices
        .into_iter()
        .take(count)
        .map(|(idx, _)| idx)
        .collect();
    
    Ok(selected_indices)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::Game;
    use crate::skip_tags::tag_registry::initialize_global_registry;
    
    fn setup_test_environment() -> Result<(), TagError> {
        // Initialize global registry with stubs if not already done
        if get_global_registry().is_err() {
            initialize_global_registry()?;
        }
        Ok(())
    }
    
    #[test]
    fn test_tag_selection_state_creation() {
        let state = TagSelectionState::new();
        assert!(state.available_tags.is_empty());
        assert!(!state.selection_made);
        assert!(state.selected_tag.is_none());
    }
    
    #[test]
    fn test_tag_selection_initialization() -> Result<(), TagError> {
        setup_test_environment()?;
        
        let mut state = TagSelectionState::new();
        let game = Game::default();
        
        state.initialize_selection(&game)?;
        
        assert!(!state.available_tags.is_empty());
        assert!(!state.selection_made);
        assert!(state.selected_tag.is_none());
        
        Ok(())
    }
    
    #[test]
    fn test_tag_selection_process() -> Result<(), TagError> {
        setup_test_environment()?;
        
        let mut state = TagSelectionState::new();
        let game = Game::default();
        
        state.initialize_selection(&game)?;
        
        let first_available_tag = state.available_tags[0];
        state.select_tag(first_available_tag)?;
        
        assert!(state.selection_made);
        assert_eq!(state.selected_tag, Some(first_available_tag));
        assert!(state.is_complete());
        
        Ok(())
    }
    
    #[test]
    fn test_invalid_tag_selection() -> Result<(), TagError> {
        setup_test_environment()?;
        
        let mut state = TagSelectionState::new();
        let game = Game::default();
        
        state.initialize_selection(&game)?;
        
        // Try to select a tag that's not available
        let invalid_tag = TagId::D6; // Assuming this is not in available_tags
        if !state.available_tags.contains(&invalid_tag) {
            let result = state.select_tag(invalid_tag);
            assert!(result.is_err());
        }
        
        Ok(())
    }
    
    #[test]
    fn test_weighted_sample_without_replacement() {
        use crate::rng::{GameRng, RngMode};
        let mut rng = GameRng::new(RngMode::Testing(42));
        
        let items = vec![1, 2, 3, 4, 5];
        let weights = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        
        let result = weighted_sample_without_replacement(&items, &weights, 3, &mut rng);
        assert!(result.is_ok());
        
        let indices = result.unwrap();
        assert_eq!(indices.len(), 3);
        
        // Ensure no duplicates
        let mut sorted_indices = indices.clone();
        sorted_indices.sort();
        sorted_indices.dedup();
        assert_eq!(sorted_indices.len(), indices.len());
    }
}