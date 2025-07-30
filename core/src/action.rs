use crate::card::Card;
use crate::consumables::ConsumableId;
use crate::joker::JokerId;
use crate::shop::packs::PackType;
use crate::skip_tags::SkipTagId;
use crate::stage::Blind;
use crate::vouchers::VoucherId;
#[cfg(feature = "python")]
use pyo3::pyclass;
use std::fmt;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "python", pyclass(eq))]
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum MoveDirection {
    Left,
    Right,
}

impl fmt::Display for MoveDirection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Left => {
                write!(f, "left")
            }
            Self::Right => {
                write!(f, "right")
            }
        }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "python", pyclass(eq))]
#[derive(Debug, PartialEq, Clone)]
pub enum Action {
    // Single-target actions (existing)
    SelectCard(Card),
    MoveCard(MoveDirection, Card),
    Play(),
    Discard(),
    CashOut(f64),
    BuyJoker {
        joker_id: JokerId,
        slot: usize,
    },
    BuyConsumable {
        consumable_id: ConsumableId,
        slot: usize,
    },
    UseConsumable {
        slot: usize,
        target_description: String,
    },
    SellConsumable {
        slot: usize,
    },
    BuyVoucher {
        voucher_id: VoucherId,
    },
    BuyPack {
        pack_type: PackType,
    },
    OpenPack {
        pack_id: usize,
    },
    SelectFromPack {
        pack_id: usize,
        option_index: usize,
    },
    SkipPack {
        pack_id: usize,
    },
    RerollShop(),
    NextRound(),
    SelectBlind(Blind),

    // Multi-select actions for cards
    SelectCards(Vec<Card>),    // Select multiple cards at once
    DeselectCard(Card),        // Deselect a specific card
    DeselectCards(Vec<Card>),  // Deselect multiple cards
    ToggleCardSelection(Card), // Toggle selection state of a card
    SelectAllCards(),          // Select all available cards
    DeselectAllCards(),        // Clear all card selections
    RangeSelectCards {
        start: Card,
        end: Card,
    }, // Select range of cards

    // Multi-select actions for jokers
    SelectJoker(JokerId),          // Select a joker
    DeselectJoker(JokerId),        // Deselect a joker
    ToggleJokerSelection(JokerId), // Toggle joker selection
    SelectAllJokers(),             // Select all available jokers
    DeselectAllJokers(),           // Clear all joker selections

    // Batch operations
    BuyJokers(Vec<(JokerId, usize)>), // Buy multiple jokers
    SellJokers(Vec<JokerId>),         // Sell multiple jokers
    BuyPacks(Vec<PackType>),          // Buy multiple packs

    // Multi-select control
    ActivateMultiSelect(),   // Enter multi-select mode
    DeactivateMultiSelect(), // Exit multi-select mode and clear selections

    // Skip tags system
    SkipBlind(Blind),         // Skip a blind and potentially get tags
    SelectSkipTag(SkipTagId), // Select a skip tag for activation
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            // Single-target actions (existing)
            Self::SelectCard(card) => {
                write!(f, "SelectCard: {card}")
            }
            Self::Play() => {
                write!(f, "Play")
            }
            Self::Discard() => {
                write!(f, "Discard")
            }
            Self::MoveCard(dir, card) => {
                write!(f, "MoveCard: {card} - {dir}")
            }
            Self::CashOut(reward) => {
                write!(f, "CashOut: {reward}")
            }
            Self::BuyJoker { joker_id, slot } => {
                write!(f, "BuyJoker: {joker_id:?} at slot {slot}")
            }
            Self::BuyConsumable {
                consumable_id,
                slot,
            } => {
                write!(f, "BuyConsumable: {consumable_id:?} at slot {slot}")
            }
            Self::UseConsumable {
                slot,
                target_description,
            } => {
                write!(
                    f,
                    "UseConsumable: slot {slot} with target {target_description}"
                )
            }
            Self::SellConsumable { slot } => {
                write!(f, "SellConsumable: slot {slot}")
            }
            Self::BuyPack { pack_type } => {
                write!(f, "BuyPack: {pack_type}")
            }
            Self::BuyVoucher { voucher_id } => {
                write!(f, "BuyVoucher: {voucher_id:?}")
            }
            Self::OpenPack { pack_id } => {
                write!(f, "OpenPack: {pack_id}")
            }
            Self::SelectFromPack {
                pack_id,
                option_index,
            } => {
                write!(f, "SelectFromPack: pack {pack_id}, option {option_index}")
            }
            Self::SkipPack { pack_id } => {
                write!(f, "SkipPack: {pack_id}")
            }
            Self::RerollShop() => {
                write!(f, "RerollShop")
            }
            Self::NextRound() => {
                write!(f, "NextRound")
            }
            Self::SelectBlind(blind) => {
                write!(f, "SelectBlind: {blind}")
            }

            // Multi-select actions for cards
            Self::SelectCards(cards) => {
                write!(f, "SelectCards: {} cards", cards.len())
            }
            Self::DeselectCard(card) => {
                write!(f, "DeselectCard: {card}")
            }
            Self::DeselectCards(cards) => {
                write!(f, "DeselectCards: {} cards", cards.len())
            }
            Self::ToggleCardSelection(card) => {
                write!(f, "ToggleCardSelection: {card}")
            }
            Self::SelectAllCards() => {
                write!(f, "SelectAllCards")
            }
            Self::DeselectAllCards() => {
                write!(f, "DeselectAllCards")
            }
            Self::RangeSelectCards { start, end } => {
                write!(f, "RangeSelectCards: {start} to {end}")
            }

            // Multi-select actions for jokers
            Self::SelectJoker(joker_id) => {
                write!(f, "SelectJoker: {joker_id:?}")
            }
            Self::DeselectJoker(joker_id) => {
                write!(f, "DeselectJoker: {joker_id:?}")
            }
            Self::ToggleJokerSelection(joker_id) => {
                write!(f, "ToggleJokerSelection: {joker_id:?}")
            }
            Self::SelectAllJokers() => {
                write!(f, "SelectAllJokers")
            }
            Self::DeselectAllJokers() => {
                write!(f, "DeselectAllJokers")
            }

            // Batch operations
            Self::BuyJokers(jokers) => {
                write!(f, "BuyJokers: {} jokers", jokers.len())
            }
            Self::SellJokers(jokers) => {
                write!(f, "SellJokers: {} jokers", jokers.len())
            }
            Self::BuyPacks(packs) => {
                write!(f, "BuyPacks: {} packs", packs.len())
            }

            // Multi-select control
            Self::ActivateMultiSelect() => {
                write!(f, "ActivateMultiSelect")
            }
            Self::DeactivateMultiSelect() => {
                write!(f, "DeactivateMultiSelect")
            }

            // Skip tags system
            Self::SkipBlind(blind) => {
                write!(f, "SkipBlind: {blind}")
            }
            Self::SelectSkipTag(tag_id) => {
                write!(f, "SelectSkipTag: {tag_id}")
            }
        }
    }
}

#[cfg(feature = "python")]
impl Action {
    fn __repr__(&self) -> String {
        format!("Action: {self}")
    }
}
