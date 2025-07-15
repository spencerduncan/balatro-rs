//! Vouchers module for Balatro game engine
//!
//! This module provides the infrastructure for voucher cards in Balatro.
//! Vouchers are permanent upgrades that can be purchased in the shop.
//!
//! # Module Organization
//!
//! - `mod.rs` - Core types and traits for vouchers
//! - `implementations.rs` - Specific voucher implementations (future)
//!
//! # Design Principles
//!
//! - Vouchers provide permanent effects that persist across rounds
//! - Each voucher can only be purchased once per run
//! - Vouchers may have prerequisites (other vouchers that must be owned first)
//! - Effects are applied passively to game state

use serde::{Deserialize, Serialize};
use std::fmt;
use strum::{EnumIter, IntoEnumIterator};

/// Core trait that all voucher types must implement
pub trait Voucher {
    /// Get the name of this voucher
    fn name(&self) -> &'static str;

    /// Get the description of what this voucher does
    fn description(&self) -> &'static str;

    /// Get the cost of this voucher in the shop
    fn cost(&self) -> usize;

    /// Get the prerequisite vouchers that must be owned before this can be purchased
    /// Returns empty vec if no prerequisites
    fn prerequisites(&self) -> Vec<VoucherId>;

    /// Apply the passive effect of this voucher to the game configuration
    /// Called when the voucher is purchased and at game start if owned
    fn apply_effect(&self, config: &mut crate::config::Config);

    /// Check if this voucher's effect should be applied in the current context
    /// Most vouchers are always active, but some may have conditional effects
    fn is_active(&self, game: &crate::game::Game) -> bool {
        // Default implementation - most vouchers are always active
        let _ = game; // Suppress unused parameter warning
        true
    }
}

/// Identifier for all voucher cards in the game
/// This will be extended as voucher implementations are added
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumIter)]
pub enum VoucherId {
    // Placeholder variants - will be expanded in future implementations
    /// Placeholder for future voucher implementations
    VoucherPlaceholder,
}

impl fmt::Display for VoucherId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VoucherId::VoucherPlaceholder => write!(f, "Voucher Placeholder"),
        }
    }
}

impl VoucherId {
    /// Get all available voucher IDs
    pub fn all() -> Vec<VoucherId> {
        Self::iter().collect()
    }

    /// Check if this voucher has any prerequisites
    pub fn has_prerequisites(&self) -> bool {
        !self.prerequisites().is_empty()
    }

    /// Get the prerequisite vouchers for this voucher
    pub fn prerequisites(&self) -> Vec<VoucherId> {
        match self {
            VoucherId::VoucherPlaceholder => vec![],
        }
    }

    /// Get the base cost of this voucher
    pub fn base_cost(&self) -> usize {
        match self {
            VoucherId::VoucherPlaceholder => 10,
        }
    }
}

/// Set of vouchers owned by the player
/// Provides efficient lookup and management of owned vouchers
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VoucherCollection {
    owned: std::collections::HashSet<VoucherId>,
}

impl VoucherCollection {
    /// Create a new empty voucher collection
    pub fn new() -> Self {
        Self {
            owned: std::collections::HashSet::new(),
        }
    }

    /// Add a voucher to the collection
    pub fn add(&mut self, voucher: VoucherId) {
        self.owned.insert(voucher);
    }

    /// Check if a voucher is owned
    pub fn owns(&self, voucher: VoucherId) -> bool {
        self.owned.contains(&voucher)
    }

    /// Get all owned vouchers
    pub fn owned_vouchers(&self) -> Vec<VoucherId> {
        self.owned.iter().copied().collect()
    }

    /// Check if all prerequisites for a voucher are met
    pub fn can_purchase(&self, voucher: VoucherId) -> bool {
        if self.owns(voucher) {
            return false; // Already owned
        }

        voucher
            .prerequisites()
            .iter()
            .all(|&prereq| self.owns(prereq))
    }

    /// Get the number of vouchers owned
    pub fn count(&self) -> usize {
        self.owned.len()
    }
}

// Re-export commonly used types
pub use VoucherId::*;
