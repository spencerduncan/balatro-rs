#\!/usr/bin/env python3

# Script to update VoucherId enum with all shop vouchers from Issue #17

with open('core/src/vouchers/mod.rs', 'r') as f:
    content = f.read()

# Define the new enum and implementation
new_enum_section = '''/// Identifier for all voucher cards in the game
/// Extended with all shop voucher implementations for Issue #17
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumIter)]
pub enum VoucherId {
    // Existing vouchers
    /// Grab Bag voucher - +1 pack option for all booster packs
    GrabBag,
    
    // Shop vouchers from Issue #17
    /// Overstock voucher - +1 card slot in shop
    Overstock,
    /// Overstock+ voucher - +2 card slots in shop (upgraded version)
    OverstockPlus,
    /// Clearance Sale voucher - All items in shop 50% off
    ClearanceSale,
    /// Hone voucher - Foil/Holo/Polychrome cards appear 2X more
    Hone,
    /// Reroll Surplus voucher - Rerolls cost $1 less  
    RerollSurplus,
    /// Crystal Ball voucher - +1 consumable slot
    CrystalBall,
    /// Telescope voucher - Celestial packs have 1 more planet card
    Telescope,
    /// Liquidation voucher - All items 25% off, rerolls 25% off
    Liquidation,
    /// Reroll Glut voucher - Rerolls cost $2 less
    RerollGlut,
    /// Omen Globe voucher - Spectral packs may contain Planet cards
    OmenGlobe,
    /// Observatory voucher - Planet cards in shop give x1.5 mult
    Observatory,
    
    /// Placeholder for future voucher implementations
    VoucherPlaceholder,
}

impl fmt::Display for VoucherId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VoucherId::GrabBag => write\!(f, "Grab Bag"),
            VoucherId::Overstock => write\!(f, "Overstock"),
            VoucherId::OverstockPlus => write\!(f, "Overstock Plus"),
            VoucherId::ClearanceSale => write\!(f, "Clearance Sale"),
            VoucherId::Hone => write\!(f, "Hone"),
            VoucherId::RerollSurplus => write\!(f, "Reroll Surplus"),
            VoucherId::CrystalBall => write\!(f, "Crystal Ball"),
            VoucherId::Telescope => write\!(f, "Telescope"),
            VoucherId::Liquidation => write\!(f, "Liquidation"),
            VoucherId::RerollGlut => write\!(f, "Reroll Glut"),
            VoucherId::OmenGlobe => write\!(f, "Omen Globe"),
            VoucherId::Observatory => write\!(f, "Observatory"),
            VoucherId::VoucherPlaceholder => write\!(f, "Voucher Placeholder"),
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
        \!self.prerequisites().is_empty()
    }

    /// Get the prerequisite vouchers for this voucher
    pub fn prerequisites(&self) -> Vec<VoucherId> {
        match self {
            // Base vouchers have no prerequisites
            VoucherId::GrabBag => vec\![],
            VoucherId::Overstock => vec\![],
            VoucherId::ClearanceSale => vec\![],
            VoucherId::Hone => vec\![],
            VoucherId::RerollSurplus => vec\![],
            VoucherId::CrystalBall => vec\![],
            VoucherId::Telescope => vec\![],
            VoucherId::Liquidation => vec\![],
            VoucherId::RerollGlut => vec\![],
            VoucherId::OmenGlobe => vec\![],
            VoucherId::Observatory => vec\![],
            
            // Upgraded versions require base versions
            VoucherId::OverstockPlus => vec\![VoucherId::Overstock],
            
            VoucherId::VoucherPlaceholder => vec\![],
        }
    }

    /// Get the base cost of this voucher
    pub fn base_cost(&self) -> usize {
        match self {
            VoucherId::GrabBag => 10,
            VoucherId::Overstock => 10,
            VoucherId::OverstockPlus => 20, // Upgraded version costs more
            VoucherId::ClearanceSale => 10,
            VoucherId::Hone => 10,
            VoucherId::RerollSurplus => 10,
            VoucherId::CrystalBall => 10,
            VoucherId::Telescope => 10,
            VoucherId::Liquidation => 10,
            VoucherId::RerollGlut => 20, // More powerful, costs more
            VoucherId::OmenGlobe => 10, 
            VoucherId::Observatory => 10,
            VoucherId::VoucherPlaceholder => 10,
        }
    }
}'''

# Find the start and end of the enum section
start_marker = "/// Identifier for all voucher cards in the game"
end_marker = "/// Set of vouchers owned by the player"

start_pos = content.find(start_marker)
end_pos = content.find(end_marker)

if start_pos == -1 or end_pos == -1:
    print("Could not find enum section markers")
    exit(1)

# Replace the section
new_content = content[:start_pos] + new_enum_section + "\n\n" + content[end_pos:]

# Write back to file
with open('core/src/vouchers/mod.rs', 'w') as f:
    f.write(new_content)

print("Updated VoucherId enum with all shop vouchers from Issue #17")
