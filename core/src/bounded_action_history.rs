use crate::action::Action;
use std::collections::VecDeque;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Default maximum number of actions to keep in history
pub const DEFAULT_MAX_ACTIONS: usize = 10_000;

/// Bounded circular buffer for action history to prevent memory leaks
///
/// This replaces the unbounded `Vec<Action>` with a memory-efficient circular buffer
/// that maintains only the most recent actions up to a configurable limit.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct BoundedActionHistory {
    /// Internal circular buffer storing actions
    actions: VecDeque<Action>,
    /// Maximum number of actions to store
    max_size: usize,
    /// Total number of actions that have been recorded (including evicted ones)
    total_actions: usize,
}

impl BoundedActionHistory {
    /// Create a new bounded action history with the default size limit
    pub fn new() -> Self {
        Self::with_capacity(DEFAULT_MAX_ACTIONS)
    }

    /// Create a new bounded action history with a specific size limit
    pub fn with_capacity(max_size: usize) -> Self {
        Self {
            actions: VecDeque::with_capacity(max_size.min(DEFAULT_MAX_ACTIONS)),
            max_size,
            total_actions: 0,
        }
    }

    /// Add a new action to the history
    ///
    /// If the history is at capacity, the oldest action will be evicted
    pub fn push(&mut self, action: Action) {
        if self.actions.len() >= self.max_size {
            self.actions.pop_front();
        }
        self.actions.push_back(action);
        self.total_actions += 1;
    }

    /// Get all actions in the history as a Vec
    ///
    /// This returns a clone of the current actions for compatibility with existing code
    /// Note: This method is expensive and should be used sparingly
    pub fn to_vec(&self) -> Vec<Action> {
        self.actions.iter().cloned().collect()
    }

    /// Get the number of actions currently stored
    pub fn len(&self) -> usize {
        self.actions.len()
    }

    /// Check if the history is empty
    pub fn is_empty(&self) -> bool {
        self.actions.is_empty()
    }

    /// Get the total number of actions that have been recorded (including evicted ones)
    pub fn total_actions(&self) -> usize {
        self.total_actions
    }

    /// Get the maximum capacity of the history
    pub fn capacity(&self) -> usize {
        self.max_size
    }

    /// Clear all actions from the history
    pub fn clear(&mut self) {
        self.actions.clear();
        self.total_actions = 0;
    }

    /// Get an iterator over the actions (most recent first)
    pub fn iter_recent_first(&self) -> impl Iterator<Item = &Action> {
        self.actions.iter().rev()
    }

    /// Get an iterator over the actions (oldest first)  
    pub fn iter(&self) -> impl Iterator<Item = &Action> {
        self.actions.iter()
    }

    /// Get the most recent action, if any
    pub fn last(&self) -> Option<&Action> {
        self.actions.back()
    }

    /// Get the oldest action in the current history, if any
    pub fn first(&self) -> Option<&Action> {
        self.actions.front()
    }

    /// Resize the history capacity
    ///
    /// If the new capacity is smaller than the current number of actions,
    /// the oldest actions will be evicted
    pub fn resize(&mut self, new_max_size: usize) {
        self.max_size = new_max_size;
        while self.actions.len() > self.max_size {
            self.actions.pop_front();
        }
        // Also shrink the underlying capacity if significantly larger
        if self.actions.capacity() > self.max_size * 2 {
            self.actions.shrink_to_fit();
        }
    }

    /// Get memory usage statistics
    pub fn memory_stats(&self) -> MemoryStats {
        MemoryStats {
            current_actions: self.len(),
            max_capacity: self.capacity(),
            total_recorded: self.total_actions(),
            estimated_bytes: self.len() * std::mem::size_of::<Action>()
                + std::mem::size_of::<Self>(),
        }
    }
}

/// Memory usage statistics for bounded action history
#[derive(Debug, Clone)]
pub struct MemoryStats {
    pub current_actions: usize,
    pub max_capacity: usize,
    pub total_recorded: usize,
    pub estimated_bytes: usize,
}

impl Default for BoundedActionHistory {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Vec<Action>> for BoundedActionHistory {
    fn from(actions: Vec<Action>) -> Self {
        let total_len = actions.len();
        let mut history = Self::with_capacity(DEFAULT_MAX_ACTIONS);

        // If we have more actions than capacity, only keep the most recent ones
        let start_idx = total_len.saturating_sub(DEFAULT_MAX_ACTIONS);

        for action in actions.into_iter().skip(start_idx) {
            history.actions.push_back(action);
        }

        history.total_actions = total_len;
        history
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::action::Action;

    #[test]
    fn test_bounded_action_history_basic() {
        let mut history = BoundedActionHistory::with_capacity(3);

        assert_eq!(history.len(), 0);
        assert!(history.is_empty());

        history.push(Action::Play());
        history.push(Action::Discard());
        assert_eq!(history.len(), 2);
        assert_eq!(history.total_actions(), 2);
    }

    #[test]
    fn test_bounded_action_history_overflow() {
        let mut history = BoundedActionHistory::with_capacity(2);

        history.push(Action::Play());
        history.push(Action::Discard());
        history.push(Action::NextRound());

        // Should only have 2 actions (most recent)
        assert_eq!(history.len(), 2);
        assert_eq!(history.total_actions(), 3);

        let actions = history.to_vec();
        assert_eq!(actions, vec![Action::Discard(), Action::NextRound()]);
    }

    #[test]
    fn test_clear() {
        let mut history = BoundedActionHistory::with_capacity(5);
        history.push(Action::Play());
        history.push(Action::Discard());

        history.clear();
        assert_eq!(history.len(), 0);
        assert_eq!(history.total_actions(), 0);
    }

    #[test]
    fn test_resize() {
        let mut history = BoundedActionHistory::with_capacity(5);
        for _ in 0..5 {
            history.push(Action::Play());
        }

        // Resize to smaller capacity
        history.resize(3);
        assert_eq!(history.len(), 3);
        assert_eq!(history.capacity(), 3);
    }

    #[test]
    fn test_from_vec() {
        let actions = vec![Action::Play(), Action::Discard(), Action::NextRound()];
        let history = BoundedActionHistory::from(actions.clone());

        assert_eq!(history.to_vec(), actions);
        assert_eq!(history.total_actions(), 3);
    }

    #[test]
    fn test_from_large_vec() {
        let mut actions = Vec::new();
        for _ in 0..15000 {
            actions.push(Action::Play());
        }

        let history = BoundedActionHistory::from(actions);

        // Should only keep the most recent DEFAULT_MAX_ACTIONS
        assert_eq!(history.len(), DEFAULT_MAX_ACTIONS);
        assert_eq!(history.total_actions(), 15000);
    }
}
