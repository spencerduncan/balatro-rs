//! Individual voucher implementations for Issue #18 gameplay vouchers
//!
//! This module contains the specific implementations for all 13 gameplay vouchers
//! that modify core game mechanics like hand size, joker slots, and interest caps.

use super::{GameState, Voucher, VoucherEffect, VoucherId, VoucherTier};

/// Grabber voucher - +1 hand size permanently
#[derive(Debug, Clone)]
pub struct GrabberVoucher;

impl Voucher for GrabberVoucher {
    fn id(&self) -> VoucherId {
        VoucherId::Grabber
    }

    fn tier(&self) -> VoucherTier {
        VoucherTier::Base
    }

    fn prerequisite(&self) -> Option<VoucherId> {
        None
    }

    fn can_purchase(&self, game_state: &GameState) -> bool {
        game_state.can_afford(self.cost()) && !game_state.owns_voucher(self.id())
    }

    fn apply_effect(&self, game_state: &mut GameState) {
        for effect in self.get_effects() {
            let _ = game_state.apply_voucher_effect(&effect);
        }
    }

    fn get_effects(&self) -> Vec<VoucherEffect> {
        vec![VoucherEffect::HandSizeIncrease(1)]
    }

    fn name(&self) -> &'static str {
        "Grabber"
    }

    fn description(&self) -> &'static str {
        "+1 hand size permanently"
    }
}

/// Nacho Tong voucher - +1 hand size permanently
#[derive(Debug, Clone)]
pub struct NachoTongVoucher;

impl Voucher for NachoTongVoucher {
    fn id(&self) -> VoucherId {
        VoucherId::NachoTong
    }

    fn tier(&self) -> VoucherTier {
        VoucherTier::Upgraded
    }

    fn prerequisite(&self) -> Option<VoucherId> {
        Some(VoucherId::Grabber)
    }

    fn can_purchase(&self, game_state: &GameState) -> bool {
        game_state.can_afford(self.cost())
            && !game_state.owns_voucher(self.id())
            && game_state.owns_voucher(VoucherId::Grabber)
    }

    fn apply_effect(&self, game_state: &mut GameState) {
        for effect in self.get_effects() {
            let _ = game_state.apply_voucher_effect(&effect);
        }
    }

    fn get_effects(&self) -> Vec<VoucherEffect> {
        vec![VoucherEffect::HandSizeIncrease(1)]
    }

    fn name(&self) -> &'static str {
        "Nacho Tong"
    }

    fn description(&self) -> &'static str {
        "+1 hand size permanently"
    }
}

/// Wasteful voucher - +1 hand size, +1 discard each round
#[derive(Debug, Clone)]
pub struct WastefulVoucher;

impl Voucher for WastefulVoucher {
    fn id(&self) -> VoucherId {
        VoucherId::Wasteful
    }

    fn tier(&self) -> VoucherTier {
        VoucherTier::Base
    }

    fn prerequisite(&self) -> Option<VoucherId> {
        None
    }

    fn can_purchase(&self, game_state: &GameState) -> bool {
        game_state.can_afford(self.cost()) && !game_state.owns_voucher(self.id())
    }

    fn apply_effect(&self, game_state: &mut GameState) {
        for effect in self.get_effects() {
            let _ = game_state.apply_voucher_effect(&effect);
        }
    }

    fn get_effects(&self) -> Vec<VoucherEffect> {
        vec![
            VoucherEffect::HandSizeIncrease(1),
            VoucherEffect::DiscardIncrease(1),
        ]
    }

    fn name(&self) -> &'static str {
        "Wasteful"
    }

    fn description(&self) -> &'static str {
        "+1 hand size, +1 discard each round"
    }
}

/// Seed Money voucher - +$1 interest cap
#[derive(Debug, Clone)]
pub struct SeedMoneyVoucher;

impl Voucher for SeedMoneyVoucher {
    fn id(&self) -> VoucherId {
        VoucherId::SeedMoney
    }

    fn tier(&self) -> VoucherTier {
        VoucherTier::Base
    }

    fn prerequisite(&self) -> Option<VoucherId> {
        None
    }

    fn can_purchase(&self, game_state: &GameState) -> bool {
        game_state.can_afford(self.cost()) && !game_state.owns_voucher(self.id())
    }

    fn apply_effect(&self, game_state: &mut GameState) {
        for effect in self.get_effects() {
            let _ = game_state.apply_voucher_effect(&effect);
        }
    }

    fn get_effects(&self) -> Vec<VoucherEffect> {
        vec![VoucherEffect::InterestCapIncrease(1)]
    }

    fn name(&self) -> &'static str {
        "Seed Money"
    }

    fn description(&self) -> &'static str {
        "+$1 interest cap"
    }
}

/// Money Tree voucher - +$2 interest cap (upgraded from Seed Money)
#[derive(Debug, Clone)]
pub struct MoneyTreeVoucher;

impl Voucher for MoneyTreeVoucher {
    fn id(&self) -> VoucherId {
        VoucherId::MoneyTree
    }

    fn tier(&self) -> VoucherTier {
        VoucherTier::Upgraded
    }

    fn prerequisite(&self) -> Option<VoucherId> {
        Some(VoucherId::SeedMoney)
    }

    fn can_purchase(&self, game_state: &GameState) -> bool {
        game_state.can_afford(self.cost())
            && !game_state.owns_voucher(self.id())
            && game_state.owns_voucher(VoucherId::SeedMoney)
    }

    fn apply_effect(&self, game_state: &mut GameState) {
        for effect in self.get_effects() {
            let _ = game_state.apply_voucher_effect(&effect);
        }
    }

    fn get_effects(&self) -> Vec<VoucherEffect> {
        vec![VoucherEffect::InterestCapIncrease(2)]
    }

    fn name(&self) -> &'static str {
        "Money Tree"
    }

    fn description(&self) -> &'static str {
        "+$2 interest cap"
    }
}

/// Hieroglyph voucher - -1 Ante, -1 hand each round
#[derive(Debug, Clone)]
pub struct HieroglyphVoucher;

impl Voucher for HieroglyphVoucher {
    fn id(&self) -> VoucherId {
        VoucherId::Hieroglyph
    }

    fn tier(&self) -> VoucherTier {
        VoucherTier::Base
    }

    fn prerequisite(&self) -> Option<VoucherId> {
        None
    }

    fn can_purchase(&self, game_state: &GameState) -> bool {
        game_state.can_afford(self.cost()) && !game_state.owns_voucher(self.id())
    }

    fn apply_effect(&self, game_state: &mut GameState) {
        for effect in self.get_effects() {
            let _ = game_state.apply_voucher_effect(&effect);
        }
    }

    fn get_effects(&self) -> Vec<VoucherEffect> {
        vec![
            VoucherEffect::AnteWinRequirementDecrease(1),
            VoucherEffect::HandSizeDecrease(1),
        ]
    }

    fn name(&self) -> &'static str {
        "Hieroglyph"
    }

    fn description(&self) -> &'static str {
        "-1 Ante, -1 hand each round"
    }
}

/// Petroglyph voucher - -1 Ante, -1 discard each round
#[derive(Debug, Clone)]
pub struct PetroglyphVoucher;

impl Voucher for PetroglyphVoucher {
    fn id(&self) -> VoucherId {
        VoucherId::Petroglyph
    }

    fn tier(&self) -> VoucherTier {
        VoucherTier::Upgraded
    }

    fn prerequisite(&self) -> Option<VoucherId> {
        Some(VoucherId::Hieroglyph)
    }

    fn can_purchase(&self, game_state: &GameState) -> bool {
        game_state.can_afford(self.cost())
            && !game_state.owns_voucher(self.id())
            && game_state.owns_voucher(VoucherId::Hieroglyph)
    }

    fn apply_effect(&self, game_state: &mut GameState) {
        for effect in self.get_effects() {
            let _ = game_state.apply_voucher_effect(&effect);
        }
    }

    fn get_effects(&self) -> Vec<VoucherEffect> {
        vec![
            VoucherEffect::AnteWinRequirementDecrease(1),
            VoucherEffect::DiscardDecrease(1),
        ]
    }

    fn name(&self) -> &'static str {
        "Petroglyph"
    }

    fn description(&self) -> &'static str {
        "-1 Ante, -1 discard each round"
    }
}

/// Antimatter voucher - +1 Joker slot
#[derive(Debug, Clone)]
pub struct AntimatterVoucher;

impl Voucher for AntimatterVoucher {
    fn id(&self) -> VoucherId {
        VoucherId::Antimatter
    }

    fn tier(&self) -> VoucherTier {
        VoucherTier::Upgraded
    }

    fn prerequisite(&self) -> Option<VoucherId> {
        Some(VoucherId::Blank)
    }

    fn can_purchase(&self, game_state: &GameState) -> bool {
        game_state.can_afford(self.cost())
            && !game_state.owns_voucher(self.id())
            && game_state.owns_voucher(VoucherId::Blank)
    }

    fn apply_effect(&self, game_state: &mut GameState) {
        for effect in self.get_effects() {
            let _ = game_state.apply_voucher_effect(&effect);
        }
    }

    fn get_effects(&self) -> Vec<VoucherEffect> {
        vec![VoucherEffect::JokerSlotIncrease(1)]
    }

    fn name(&self) -> &'static str {
        "Antimatter"
    }

    fn description(&self) -> &'static str {
        "+1 Joker slot"
    }
}

/// Magic Trick voucher - Playing cards can be purchased from shop
#[derive(Debug, Clone)]
pub struct MagicTrickVoucher;

impl Voucher for MagicTrickVoucher {
    fn id(&self) -> VoucherId {
        VoucherId::MagicTrick
    }

    fn tier(&self) -> VoucherTier {
        VoucherTier::Base
    }

    fn prerequisite(&self) -> Option<VoucherId> {
        None
    }

    fn can_purchase(&self, game_state: &GameState) -> bool {
        game_state.can_afford(self.cost()) && !game_state.owns_voucher(self.id())
    }

    fn apply_effect(&self, game_state: &mut GameState) {
        for effect in self.get_effects() {
            let _ = game_state.apply_voucher_effect(&effect);
        }
    }

    fn get_effects(&self) -> Vec<VoucherEffect> {
        vec![VoucherEffect::ShopPlayingCardsEnabled]
    }

    fn name(&self) -> &'static str {
        "Magic Trick"
    }

    fn description(&self) -> &'static str {
        "Playing cards can be purchased from shop"
    }
}

/// Illusion voucher - Playing cards in shop may have enhancements
#[derive(Debug, Clone)]
pub struct IllusionVoucher;

impl Voucher for IllusionVoucher {
    fn id(&self) -> VoucherId {
        VoucherId::Illusion
    }

    fn tier(&self) -> VoucherTier {
        VoucherTier::Upgraded
    }

    fn prerequisite(&self) -> Option<VoucherId> {
        Some(VoucherId::MagicTrick)
    }

    fn can_purchase(&self, game_state: &GameState) -> bool {
        game_state.can_afford(self.cost())
            && !game_state.owns_voucher(self.id())
            && game_state.owns_voucher(VoucherId::MagicTrick)
    }

    fn apply_effect(&self, game_state: &mut GameState) {
        for effect in self.get_effects() {
            let _ = game_state.apply_voucher_effect(&effect);
        }
    }

    fn get_effects(&self) -> Vec<VoucherEffect> {
        vec![VoucherEffect::ShopEnhancementsEnabled]
    }

    fn name(&self) -> &'static str {
        "Illusion"
    }

    fn description(&self) -> &'static str {
        "Playing cards in shop may have enhancements"
    }
}

/// Blank voucher - Does nothing (flavor text)
#[derive(Debug, Clone)]
pub struct BlankVoucher;

impl Voucher for BlankVoucher {
    fn id(&self) -> VoucherId {
        VoucherId::Blank
    }

    fn tier(&self) -> VoucherTier {
        VoucherTier::Base
    }

    fn prerequisite(&self) -> Option<VoucherId> {
        None
    }

    fn can_purchase(&self, game_state: &GameState) -> bool {
        game_state.can_afford(self.cost()) && !game_state.owns_voucher(self.id())
    }

    fn apply_effect(&self, game_state: &mut GameState) {
        for effect in self.get_effects() {
            let _ = game_state.apply_voucher_effect(&effect);
        }
    }

    fn get_effects(&self) -> Vec<VoucherEffect> {
        vec![VoucherEffect::NoEffect]
    }

    fn name(&self) -> &'static str {
        "Blank"
    }

    fn description(&self) -> &'static str {
        "Does nothing"
    }
}

/// Paint Brush voucher - +1 hand size, -1 joker slot
#[derive(Debug, Clone)]
pub struct PaintBrushVoucher;

impl Voucher for PaintBrushVoucher {
    fn id(&self) -> VoucherId {
        VoucherId::PaintBrush
    }

    fn tier(&self) -> VoucherTier {
        VoucherTier::Base
    }

    fn prerequisite(&self) -> Option<VoucherId> {
        None
    }

    fn can_purchase(&self, game_state: &GameState) -> bool {
        game_state.can_afford(self.cost()) && !game_state.owns_voucher(self.id())
    }

    fn apply_effect(&self, game_state: &mut GameState) {
        for effect in self.get_effects() {
            let _ = game_state.apply_voucher_effect(&effect);
        }
    }

    fn get_effects(&self) -> Vec<VoucherEffect> {
        vec![
            VoucherEffect::HandSizeIncrease(1),
            VoucherEffect::JokerSlotDecrease(1),
        ]
    }

    fn name(&self) -> &'static str {
        "Paint Brush"
    }

    fn description(&self) -> &'static str {
        "+1 hand size, -1 joker slot"
    }
}

/// Tarot Merchant voucher - Tarot cards appear 2X more
#[derive(Debug, Clone)]
pub struct TarotMerchantVoucher;

impl Voucher for TarotMerchantVoucher {
    fn id(&self) -> VoucherId {
        VoucherId::TarotMerchant
    }

    fn tier(&self) -> VoucherTier {
        VoucherTier::Base
    }

    fn prerequisite(&self) -> Option<VoucherId> {
        None
    }

    fn can_purchase(&self, game_state: &GameState) -> bool {
        game_state.can_afford(self.cost()) && !game_state.owns_voucher(self.id())
    }

    fn apply_effect(&self, game_state: &mut GameState) {
        for effect in self.get_effects() {
            let _ = game_state.apply_voucher_effect(&effect);
        }
    }

    fn get_effects(&self) -> Vec<VoucherEffect> {
        vec![VoucherEffect::TarotFrequencyMultiplier(2.0)]
    }

    fn name(&self) -> &'static str {
        "Tarot Merchant"
    }

    fn description(&self) -> &'static str {
        "Tarot cards appear 2X more"
    }
}

/// Tarot Tycoon voucher - Tarot cards appear 4X more (upgraded from Tarot Merchant)
#[derive(Debug, Clone)]
pub struct TarotTycoonVoucher;

impl Voucher for TarotTycoonVoucher {
    fn id(&self) -> VoucherId {
        VoucherId::TarotTycoon
    }

    fn tier(&self) -> VoucherTier {
        VoucherTier::Upgraded
    }

    fn prerequisite(&self) -> Option<VoucherId> {
        Some(VoucherId::TarotMerchant)
    }

    fn can_purchase(&self, game_state: &GameState) -> bool {
        game_state.can_afford(self.cost())
            && !game_state.owns_voucher(self.id())
            && game_state.owns_voucher(VoucherId::TarotMerchant)
    }

    fn apply_effect(&self, game_state: &mut GameState) {
        for effect in self.get_effects() {
            let _ = game_state.apply_voucher_effect(&effect);
        }
    }

    fn get_effects(&self) -> Vec<VoucherEffect> {
        vec![VoucherEffect::TarotFrequencyMultiplier(4.0)]
    }

    fn name(&self) -> &'static str {
        "Tarot Tycoon"
    }

    fn description(&self) -> &'static str {
        "Tarot cards appear 4X more"
    }
}

/// Overstock voucher - +1 card slot in shop
#[derive(Debug, Clone)]
pub struct OverstockVoucher;

impl Voucher for OverstockVoucher {
    fn id(&self) -> VoucherId {
        VoucherId::Overstock
    }

    fn tier(&self) -> VoucherTier {
        VoucherTier::Base
    }

    fn prerequisite(&self) -> Option<VoucherId> {
        None
    }

    fn can_purchase(&self, game_state: &GameState) -> bool {
        game_state.can_afford(self.cost()) && !game_state.owns_voucher(self.id())
    }

    fn apply_effect(&self, game_state: &mut GameState) {
        for effect in self.get_effects() {
            let _ = game_state.apply_voucher_effect(&effect);
        }
    }

    fn get_effects(&self) -> Vec<VoucherEffect> {
        vec![VoucherEffect::ShopSlotIncrease(1)]
    }

    fn name(&self) -> &'static str {
        "Overstock"
    }

    fn description(&self) -> &'static str {
        "+1 card slot in shop"
    }
}

/// Overstock Plus voucher - +2 card slots in shop (upgraded from Overstock)
#[derive(Debug, Clone)]
pub struct OverstockPlusVoucher;

impl Voucher for OverstockPlusVoucher {
    fn id(&self) -> VoucherId {
        VoucherId::OverstockPlus
    }

    fn tier(&self) -> VoucherTier {
        VoucherTier::Upgraded
    }

    fn prerequisite(&self) -> Option<VoucherId> {
        Some(VoucherId::Overstock)
    }

    fn can_purchase(&self, game_state: &GameState) -> bool {
        game_state.can_afford(self.cost())
            && !game_state.owns_voucher(self.id())
            && game_state.owns_voucher(VoucherId::Overstock)
    }

    fn apply_effect(&self, game_state: &mut GameState) {
        for effect in self.get_effects() {
            let _ = game_state.apply_voucher_effect(&effect);
        }
    }

    fn get_effects(&self) -> Vec<VoucherEffect> {
        vec![VoucherEffect::ShopSlotIncrease(2)]
    }

    fn name(&self) -> &'static str {
        "Overstock Plus"
    }

    fn description(&self) -> &'static str {
        "+2 card slots in shop"
    }
}

/// Clearance Sale voucher - All items in shop 50% off
#[derive(Debug, Clone)]
pub struct ClearanceSaleVoucher;

impl Voucher for ClearanceSaleVoucher {
    fn id(&self) -> VoucherId {
        VoucherId::ClearanceSale
    }

    fn tier(&self) -> VoucherTier {
        VoucherTier::Base
    }

    fn prerequisite(&self) -> Option<VoucherId> {
        None
    }

    fn can_purchase(&self, game_state: &GameState) -> bool {
        game_state.can_afford(self.cost()) && !game_state.owns_voucher(self.id())
    }

    fn apply_effect(&self, game_state: &mut GameState) {
        for effect in self.get_effects() {
            let _ = game_state.apply_voucher_effect(&effect);
        }
    }

    fn get_effects(&self) -> Vec<VoucherEffect> {
        vec![VoucherEffect::ShopDiscountPercent(50.0)]
    }

    fn name(&self) -> &'static str {
        "Clearance Sale"
    }

    fn description(&self) -> &'static str {
        "All items in shop 50% off"
    }
}

/// Hone voucher - Foil/Holo/Polychrome cards appear 2X more
#[derive(Debug, Clone)]
pub struct HoneVoucher;

impl Voucher for HoneVoucher {
    fn id(&self) -> VoucherId {
        VoucherId::Hone
    }

    fn tier(&self) -> VoucherTier {
        VoucherTier::Base
    }

    fn prerequisite(&self) -> Option<VoucherId> {
        None
    }

    fn can_purchase(&self, game_state: &GameState) -> bool {
        game_state.can_afford(self.cost()) && !game_state.owns_voucher(self.id())
    }

    fn apply_effect(&self, game_state: &mut GameState) {
        for effect in self.get_effects() {
            let _ = game_state.apply_voucher_effect(&effect);
        }
    }

    fn get_effects(&self) -> Vec<VoucherEffect> {
        vec![VoucherEffect::PolychromeFrequencyMultiplier(2.0)]
    }

    fn name(&self) -> &'static str {
        "Hone"
    }

    fn description(&self) -> &'static str {
        "Foil/Holo/Polychrome cards appear 2X more"
    }
}

/// Reroll Surplus voucher - Rerolls cost $1 less
#[derive(Debug, Clone)]
pub struct RerollSurplusVoucher;

impl Voucher for RerollSurplusVoucher {
    fn id(&self) -> VoucherId {
        VoucherId::RerollSurplus
    }

    fn tier(&self) -> VoucherTier {
        VoucherTier::Base
    }

    fn prerequisite(&self) -> Option<VoucherId> {
        None
    }

    fn can_purchase(&self, game_state: &GameState) -> bool {
        game_state.can_afford(self.cost()) && !game_state.owns_voucher(self.id())
    }

    fn apply_effect(&self, game_state: &mut GameState) {
        for effect in self.get_effects() {
            let _ = game_state.apply_voucher_effect(&effect);
        }
    }

    fn get_effects(&self) -> Vec<VoucherEffect> {
        vec![VoucherEffect::RerollCostReduction(1)]
    }

    fn name(&self) -> &'static str {
        "Reroll Surplus"
    }

    fn description(&self) -> &'static str {
        "Rerolls cost $1 less"
    }
}

/// Crystal Ball voucher - +1 consumable slot
#[derive(Debug, Clone)]
pub struct CrystalBallVoucher;

impl Voucher for CrystalBallVoucher {
    fn id(&self) -> VoucherId {
        VoucherId::CrystalBall
    }

    fn tier(&self) -> VoucherTier {
        VoucherTier::Base
    }

    fn prerequisite(&self) -> Option<VoucherId> {
        None
    }

    fn can_purchase(&self, game_state: &GameState) -> bool {
        game_state.can_afford(self.cost()) && !game_state.owns_voucher(self.id())
    }

    fn apply_effect(&self, game_state: &mut GameState) {
        for effect in self.get_effects() {
            let _ = game_state.apply_voucher_effect(&effect);
        }
    }

    fn get_effects(&self) -> Vec<VoucherEffect> {
        vec![VoucherEffect::ConsumableSlotIncrease(1)]
    }

    fn name(&self) -> &'static str {
        "Crystal Ball"
    }

    fn description(&self) -> &'static str {
        "+1 consumable slot"
    }
}

/// Reroll Glut voucher - Rerolls cost $2 less (upgraded from Reroll Surplus)
#[derive(Debug, Clone)]
pub struct RerollGlutVoucher;

impl Voucher for RerollGlutVoucher {
    fn id(&self) -> VoucherId {
        VoucherId::RerollGlut
    }

    fn tier(&self) -> VoucherTier {
        VoucherTier::Upgraded
    }

    fn prerequisite(&self) -> Option<VoucherId> {
        Some(VoucherId::RerollSurplus)
    }

    fn can_purchase(&self, game_state: &GameState) -> bool {
        game_state.can_afford(self.cost())
            && !game_state.owns_voucher(self.id())
            && game_state.owns_voucher(VoucherId::RerollSurplus)
    }

    fn apply_effect(&self, game_state: &mut GameState) {
        for effect in self.get_effects() {
            let _ = game_state.apply_voucher_effect(&effect);
        }
    }

    fn get_effects(&self) -> Vec<VoucherEffect> {
        vec![VoucherEffect::RerollCostReduction(2)]
    }

    fn name(&self) -> &'static str {
        "Reroll Glut"
    }

    fn description(&self) -> &'static str {
        "Rerolls cost $2 less"
    }
}

/// Glow Up voucher - Foil, Holographic, and Polychrome cards appear 4X more often (upgrade of Hone)
#[derive(Debug, Clone)]
pub struct GlowUpVoucher;

impl Voucher for GlowUpVoucher {
    fn id(&self) -> VoucherId {
        VoucherId::GlowUp
    }

    fn tier(&self) -> VoucherTier {
        VoucherTier::Upgraded
    }

    fn prerequisite(&self) -> Option<VoucherId> {
        Some(VoucherId::Hone)
    }

    fn can_purchase(&self, game_state: &GameState) -> bool {
        game_state.can_afford(self.cost())
            && !game_state.owns_voucher(self.id())
            && game_state.owns_voucher(VoucherId::Hone)
    }

    fn apply_effect(&self, game_state: &mut GameState) {
        for effect in self.get_effects() {
            let _ = game_state.apply_voucher_effect(&effect);
        }
    }

    fn get_effects(&self) -> Vec<VoucherEffect> {
        vec![VoucherEffect::PolychromeFrequencyMultiplier(4.0)]
    }

    fn name(&self) -> &'static str {
        "Glow Up"
    }

    fn description(&self) -> &'static str {
        "Foil, Holographic, and Polychrome cards appear 4X more often"
    }
}

/// Liquidation voucher - All cards and packs in shop are 50% off (upgrade of Clearance Sale)
#[derive(Debug, Clone)]
pub struct LiquidationVoucher;

impl Voucher for LiquidationVoucher {
    fn id(&self) -> VoucherId {
        VoucherId::Liquidation
    }

    fn tier(&self) -> VoucherTier {
        VoucherTier::Upgraded
    }

    fn prerequisite(&self) -> Option<VoucherId> {
        Some(VoucherId::ClearanceSale)
    }

    fn can_purchase(&self, game_state: &GameState) -> bool {
        game_state.can_afford(self.cost())
            && !game_state.owns_voucher(self.id())
            && game_state.owns_voucher(VoucherId::ClearanceSale)
    }

    fn apply_effect(&self, game_state: &mut GameState) {
        for effect in self.get_effects() {
            let _ = game_state.apply_voucher_effect(&effect);
        }
    }

    fn get_effects(&self) -> Vec<VoucherEffect> {
        vec![VoucherEffect::ShopDiscountMultiplier(0.5)]
    }

    fn name(&self) -> &'static str {
        "Liquidation"
    }

    fn description(&self) -> &'static str {
        "All cards and packs in shop are 50% off"
    }
}

/// Recyclomancy voucher - Permanently gain +1 discard each round (upgrade of Wasteful)
#[derive(Debug, Clone)]
pub struct RecyclomancyVoucher;

impl Voucher for RecyclomancyVoucher {
    fn id(&self) -> VoucherId {
        VoucherId::Recyclomancy
    }

    fn tier(&self) -> VoucherTier {
        VoucherTier::Upgraded
    }

    fn prerequisite(&self) -> Option<VoucherId> {
        Some(VoucherId::Wasteful)
    }

    fn can_purchase(&self, game_state: &GameState) -> bool {
        game_state.can_afford(self.cost())
            && !game_state.owns_voucher(self.id())
            && game_state.owns_voucher(VoucherId::Wasteful)
    }

    fn apply_effect(&self, game_state: &mut GameState) {
        for effect in self.get_effects() {
            let _ = game_state.apply_voucher_effect(&effect);
        }
    }

    fn get_effects(&self) -> Vec<VoucherEffect> {
        vec![VoucherEffect::DiscardIncrease(1)]
    }

    fn name(&self) -> &'static str {
        "Recyclomancy"
    }

    fn description(&self) -> &'static str {
        "Permanently gain +1 discard each round"
    }
}

/// Planet Merchant voucher - Planet cards appear 2X more frequently in shop
#[derive(Debug, Clone)]
pub struct PlanetMerchantVoucher;

impl Voucher for PlanetMerchantVoucher {
    fn id(&self) -> VoucherId {
        VoucherId::PlanetMerchant
    }

    fn tier(&self) -> VoucherTier {
        VoucherTier::Base
    }

    fn prerequisite(&self) -> Option<VoucherId> {
        None
    }

    fn can_purchase(&self, game_state: &GameState) -> bool {
        game_state.can_afford(self.cost()) && !game_state.owns_voucher(self.id())
    }

    fn apply_effect(&self, game_state: &mut GameState) {
        for effect in self.get_effects() {
            let _ = game_state.apply_voucher_effect(&effect);
        }
    }

    fn get_effects(&self) -> Vec<VoucherEffect> {
        vec![VoucherEffect::PlanetFrequencyMultiplier(2.0)]
    }

    fn name(&self) -> &'static str {
        "Planet Merchant"
    }

    fn description(&self) -> &'static str {
        "Planet cards appear 2X more frequently in shop"
    }
}

/// Planet Tycoon voucher - Planet cards appear 4X more frequently in shop (upgrade of Planet Merchant)
#[derive(Debug, Clone)]
pub struct PlanetTycoonVoucher;

impl Voucher for PlanetTycoonVoucher {
    fn id(&self) -> VoucherId {
        VoucherId::PlanetTycoon
    }

    fn tier(&self) -> VoucherTier {
        VoucherTier::Upgraded
    }

    fn prerequisite(&self) -> Option<VoucherId> {
        Some(VoucherId::PlanetMerchant)
    }

    fn can_purchase(&self, game_state: &GameState) -> bool {
        game_state.can_afford(self.cost())
            && !game_state.owns_voucher(self.id())
            && game_state.owns_voucher(VoucherId::PlanetMerchant)
    }

    fn apply_effect(&self, game_state: &mut GameState) {
        for effect in self.get_effects() {
            let _ = game_state.apply_voucher_effect(&effect);
        }
    }

    fn get_effects(&self) -> Vec<VoucherEffect> {
        vec![VoucherEffect::PlanetFrequencyMultiplier(4.0)]
    }

    fn name(&self) -> &'static str {
        "Planet Tycoon"
    }

    fn description(&self) -> &'static str {
        "Planet cards appear 4X more frequently in shop"
    }
}

/// Director's Cut voucher - Reroll Boss Blind 1 time per Ante, $10 per roll
#[derive(Debug, Clone)]
pub struct DirectorsCutVoucher;

impl Voucher for DirectorsCutVoucher {
    fn id(&self) -> VoucherId {
        VoucherId::DirectorsCut
    }

    fn tier(&self) -> VoucherTier {
        VoucherTier::Base
    }

    fn prerequisite(&self) -> Option<VoucherId> {
        None
    }

    fn can_purchase(&self, game_state: &GameState) -> bool {
        game_state.can_afford(self.cost()) && !game_state.owns_voucher(self.id())
    }

    fn apply_effect(&self, game_state: &mut GameState) {
        for effect in self.get_effects() {
            let _ = game_state.apply_voucher_effect(&effect);
        }
    }

    fn get_effects(&self) -> Vec<VoucherEffect> {
        vec![VoucherEffect::BossBlindRerollEnabled {
            unlimited: false,
            cost_per_roll: 10,
        }]
    }

    fn name(&self) -> &'static str {
        "Director's Cut"
    }

    fn description(&self) -> &'static str {
        "Reroll Boss Blind 1 time per Ante, $10 per roll"
    }
}

/// Retcon voucher - Reroll Boss Blinds unlimited times, $10 per roll (upgrade of Director's Cut)
#[derive(Debug, Clone)]
pub struct RetconVoucher;

impl Voucher for RetconVoucher {
    fn id(&self) -> VoucherId {
        VoucherId::Retcon
    }

    fn tier(&self) -> VoucherTier {
        VoucherTier::Upgraded
    }

    fn prerequisite(&self) -> Option<VoucherId> {
        Some(VoucherId::DirectorsCut)
    }

    fn can_purchase(&self, game_state: &GameState) -> bool {
        game_state.can_afford(self.cost())
            && !game_state.owns_voucher(self.id())
            && game_state.owns_voucher(VoucherId::DirectorsCut)
    }

    fn apply_effect(&self, game_state: &mut GameState) {
        for effect in self.get_effects() {
            let _ = game_state.apply_voucher_effect(&effect);
        }
    }

    fn get_effects(&self) -> Vec<VoucherEffect> {
        vec![VoucherEffect::BossBlindRerollEnabled {
            unlimited: true,
            cost_per_roll: 10,
        }]
    }

    fn name(&self) -> &'static str {
        "Retcon"
    }

    fn description(&self) -> &'static str {
        "Reroll Boss Blinds unlimited times, $10 per roll"
    }
}

/// Palette voucher - +1 hand size (upgrade of Paint Brush)
#[derive(Debug, Clone)]
pub struct PaletteVoucher;

impl Voucher for PaletteVoucher {
    fn id(&self) -> VoucherId {
        VoucherId::Palette
    }

    fn tier(&self) -> VoucherTier {
        VoucherTier::Upgraded
    }

    fn prerequisite(&self) -> Option<VoucherId> {
        Some(VoucherId::PaintBrush)
    }

    fn can_purchase(&self, game_state: &GameState) -> bool {
        game_state.can_afford(self.cost())
            && !game_state.owns_voucher(self.id())
            && game_state.owns_voucher(VoucherId::PaintBrush)
    }

    fn apply_effect(&self, game_state: &mut GameState) {
        for effect in self.get_effects() {
            let _ = game_state.apply_voucher_effect(&effect);
        }
    }

    fn get_effects(&self) -> Vec<VoucherEffect> {
        vec![VoucherEffect::HandSizeIncrease(1)]
    }

    fn name(&self) -> &'static str {
        "Palette"
    }

    fn description(&self) -> &'static str {
        "+1 hand size"
    }
}
