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
