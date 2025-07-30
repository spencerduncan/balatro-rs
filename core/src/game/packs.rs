// Booster pack management module
// Extracted from Game struct to improve modularity

use crate::error::GameError;
use crate::shop::packs::{OpenPackState, Pack};
use crate::shop::ShopItem;

/// PackManager handles all booster pack-related functionality
/// Extracted from Game struct for better separation of concerns
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PackManager {
    /// Packs currently in the player's inventory
    pub pack_inventory: Vec<Pack>,

    /// Currently opened pack that player is choosing from
    pub open_pack: Option<OpenPackState>,
}

impl Default for PackManager {
    fn default() -> Self {
        Self::new()
    }
}

impl PackManager {
    /// Create a new PackManager instance
    pub fn new() -> Self {
        Self {
            pack_inventory: Vec::new(),
            open_pack: None,
        }
    }

    /// Add a pack to the inventory (used by Game after pack generation)
    pub fn add_pack(&mut self, pack: Pack) {
        self.pack_inventory.push(pack);
    }

    /// Open a pack from inventory
    pub fn open_pack(&mut self, pack_id: usize) -> Result<(), GameError> {
        // Check if pack exists in inventory
        if pack_id >= self.pack_inventory.len() {
            return Err(GameError::InvalidAction);
        }

        // Check if another pack is already open
        if self.open_pack.is_some() {
            return Err(GameError::InvalidAction);
        }

        // Remove pack from inventory and open it
        let pack = self.pack_inventory.remove(pack_id);
        self.open_pack = Some(OpenPackState::new(pack, pack_id));

        Ok(())
    }

    /// Select an option from the currently opened pack
    pub fn select_from_pack(
        &mut self,
        pack_id: usize,
        option_index: usize,
    ) -> Result<ShopItem, GameError> {
        // Check if a pack is open
        let open_pack_state = self.open_pack.take().ok_or(GameError::InvalidAction)?;

        // Verify pack ID matches
        if open_pack_state.pack_id != pack_id {
            return Err(GameError::InvalidAction);
        }

        // Select the option and return it for processing by Game
        open_pack_state.pack.select_option(option_index)
    }

    /// Skip the currently opened pack
    pub fn skip_pack(&mut self, pack_id: usize) -> Result<(), GameError> {
        // Check if a pack is open
        let open_pack_state = self.open_pack.take().ok_or(GameError::InvalidAction)?;

        // Verify pack ID matches
        if open_pack_state.pack_id != pack_id {
            return Err(GameError::InvalidAction);
        }

        // Check if pack can be skipped
        if !open_pack_state.pack.can_skip {
            return Err(GameError::InvalidAction);
        }

        // Pack is simply consumed (no further action needed)
        Ok(())
    }

    /// Get read-only access to pack inventory
    pub fn pack_inventory(&self) -> &Vec<Pack> {
        &self.pack_inventory
    }

    /// Get read-only access to open pack state
    pub fn open_pack_state(&self) -> &Option<OpenPackState> {
        &self.open_pack
    }
}
