//! Skip Tag Effects - Unified Implementation
//!
//! This module handles both utility effect functions and persistent state management
//! for skip tags, combining the best of both approaches.

use super::{SkipTagContext, SkipTagId, TagEffectResult};
use crate::game::Game;
use crate::rank::HandRank;
use serde::{Deserialize, Serialize};

// Export both approaches for compatibility
pub use self::state_management::*;
pub use self::utility_effects::*;

/// State management module for persistent skip tag effects
pub mod state_management {
    use super::*;

    /// Active skip tag state that persists across game events
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct ActiveSkipTags {
        // Economic tag state (from Issue #693)
        pub investment_count: u32,
        pub blinds_skipped: u32,

        // Shop enhancement modifiers (this issue - #694)
        pub next_shop_vouchers: u32,
        pub next_shop_coupon: bool,
        pub next_shop_free_reroll: bool,
        pub next_shop_foil_joker: bool,
        pub next_shop_holographic_joker: bool,
        pub next_shop_polychrome_joker: bool,
    }

    impl Default for ActiveSkipTags {
        fn default() -> Self {
            Self::new()
        }
    }

    impl ActiveSkipTags {
        /// Create new empty active skip tags state
        pub fn new() -> Self {
            Self {
                investment_count: 0,
                blinds_skipped: 0,
                next_shop_vouchers: 0,
                next_shop_coupon: false,
                next_shop_free_reroll: false,
                next_shop_foil_joker: false,
                next_shop_holographic_joker: false,
                next_shop_polychrome_joker: false,
            }
        }

        /// Reset all next shop modifiers (called when entering shop)
        pub fn consume_next_shop_modifiers(&mut self) -> NextShopModifiers {
            let modifiers = NextShopModifiers {
                vouchers_to_add: self.next_shop_vouchers,
                coupon_active: self.next_shop_coupon,
                free_reroll: self.next_shop_free_reroll,
                foil_joker: self.next_shop_foil_joker,
                holographic_joker: self.next_shop_holographic_joker,
                polychrome_joker: self.next_shop_polychrome_joker,
            };

            // Reset modifiers after consumption
            self.next_shop_vouchers = 0;
            self.next_shop_coupon = false;
            self.next_shop_free_reroll = false;
            self.next_shop_foil_joker = false;
            self.next_shop_holographic_joker = false;
            self.next_shop_polychrome_joker = false;

            modifiers
        }

        /// Apply a shop enhancement tag effect
        pub fn apply_shop_enhancement_effect(&mut self, tag_id: SkipTagId) {
            match tag_id {
                SkipTagId::Voucher => {
                    self.next_shop_vouchers += 1;
                }
                SkipTagId::Coupon => {
                    self.next_shop_coupon = true;
                }
                SkipTagId::D6 => {
                    self.next_shop_free_reroll = true;
                }
                SkipTagId::Foil => {
                    self.next_shop_foil_joker = true;
                }
                SkipTagId::Holographic => {
                    self.next_shop_holographic_joker = true;
                }
                SkipTagId::Polychrome => {
                    self.next_shop_polychrome_joker = true;
                }
                _ => {
                    // Not a shop enhancement tag - ignore
                }
            }
        }
    }

    /// Next shop modifiers consumed when entering shop
    #[derive(Debug, Clone, PartialEq, Default)]
    pub struct NextShopModifiers {
        /// Number of vouchers to add to shop
        pub vouchers_to_add: u32,
        /// Whether coupon (free items) is active
        pub coupon_active: bool,
        /// Whether rerolls are free
        pub free_reroll: bool,
        /// Whether to add foil joker
        pub foil_joker: bool,
        /// Whether to add holographic joker
        pub holographic_joker: bool,
        /// Whether to add polychrome joker
        pub polychrome_joker: bool,
    }

    impl NextShopModifiers {
        /// Check if any modifiers are active
        pub fn has_any_modifier(&self) -> bool {
            self.vouchers_to_add > 0
                || self.coupon_active
                || self.free_reroll
                || self.foil_joker
                || self.holographic_joker
                || self.polychrome_joker
        }

        /// Get count of edition modifiers
        pub fn edition_modifier_count(&self) -> u32 {
            let mut count = 0;
            if self.foil_joker { count += 1; }
            if self.holographic_joker { count += 1; }
            if self.polychrome_joker { count += 1; }
            count
        }
    }
}

/// Utility effect functions module
pub mod utility_effects {
    use super::*;

    /// Effect that gives immediate money reward
    pub fn money_effect(game: &mut Game, amount: i64) -> TagEffectResult {
        game.money += amount as f64;
        TagEffectResult::with_money_and_message(amount as i32, format!("Gained ${amount}"))
    }

    /// Effect that duplicates another tag (Double tag)
    pub fn duplication_effect(selected_tag: SkipTagId) -> TagEffectResult {
        // Exclude Double tags from duplication
        if matches!(selected_tag, SkipTagId::Double) {
            return TagEffectResult {
                money_reward: 0,
                messages: vec!["Cannot duplicate Double tags".to_string()],
                persist_tag: false,
            };
        }

        TagEffectResult::with_money_and_message(0, format!("Duplicated {selected_tag} tag"))
    }

    /// Effect that upgrades a poker hand by levels
    pub fn hand_upgrade_effect(game: &Game, levels: u32) -> TagEffectResult {
        let available_hands = get_upgradeable_hands(game);

        if available_hands.is_empty() {
            return TagEffectResult {
                money_reward: 0,
                messages: vec!["No hands available to upgrade".to_string()],
                persist_tag: false,
            };
        }

        // Select a random hand to upgrade
        let random_index = fastrand::usize(0..available_hands.len());
        let selected_hand = available_hands[random_index];

        // TODO: Implement hand level upgrading
        TagEffectResult::with_money_and_message(
            0,
            format!("Upgraded {selected_hand} by {levels} levels"),
        )
    }

    /// Get all hand types that can be upgraded
    fn get_upgradeable_hands(_game: &Game) -> Vec<HandRank> {
        // TODO: This should check which hands have been played in the current run
        vec![
            HandRank::HighCard,
            HandRank::OnePair,
            HandRank::TwoPair,
            HandRank::ThreeOfAKind,
            HandRank::Straight,
            HandRank::Flush,
            HandRank::FullHouse,
            HandRank::FourOfAKind,
            HandRank::StraightFlush,
            HandRank::RoyalFlush,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod state_management_tests {
        use super::*;

        #[test]
        fn test_active_skip_tags_creation() {
            let tags = ActiveSkipTags::new();

            // All shop modifiers should start inactive
            assert_eq!(tags.next_shop_vouchers, 0);
            assert!(!tags.next_shop_coupon);
            assert!(!tags.next_shop_free_reroll);
            assert!(!tags.next_shop_foil_joker);
            assert!(!tags.next_shop_holographic_joker);
            assert!(!tags.next_shop_polychrome_joker);

            // Economic tag state should start at zero
            assert_eq!(tags.investment_count, 0);
            assert_eq!(tags.blinds_skipped, 0);
        }

        #[test]
        fn test_apply_shop_enhancement_effects() {
            let mut tags = ActiveSkipTags::new();

            tags.apply_shop_enhancement_effect(SkipTagId::Voucher);
            assert_eq!(tags.next_shop_vouchers, 1);

            tags.apply_shop_enhancement_effect(SkipTagId::Coupon);
            assert!(tags.next_shop_coupon);

            tags.apply_shop_enhancement_effect(SkipTagId::D6);
            assert!(tags.next_shop_free_reroll);

            tags.apply_shop_enhancement_effect(SkipTagId::Foil);
            assert!(tags.next_shop_foil_joker);
        }

        #[test]
        fn test_consume_next_shop_modifiers() {
            let mut tags = ActiveSkipTags::new();

            // Set up some shop modifiers
            tags.apply_shop_enhancement_effect(SkipTagId::Voucher);
            tags.apply_shop_enhancement_effect(SkipTagId::Coupon);

            // Consume modifiers
            let modifiers = tags.consume_next_shop_modifiers();

            // Verify consumed modifiers
            assert_eq!(modifiers.vouchers_to_add, 1);
            assert!(modifiers.coupon_active);

            // Verify tags are reset after consumption
            assert_eq!(tags.next_shop_vouchers, 0);
            assert!(!tags.next_shop_coupon);
        }
    }

    mod utility_effects_tests {
        use super::*;

        #[test]
        fn test_money_effect() {
            let mut game = Game::default();
            let initial_money = game.money;

            let result = utility_effects::money_effect(&mut game, 50);

            assert_eq!(game.money, initial_money + 50.0);
            assert_eq!(result.money_reward, 50);
            assert!(result.messages[0].contains("$50"));
        }

        #[test]
        fn test_duplication_effect_success() {
            let result = utility_effects::duplication_effect(SkipTagId::Boss);
            assert!(result.messages[0].contains("Duplicated Boss tag"));
        }

        #[test]
        fn test_duplication_effect_double_rejection() {
            let result = utility_effects::duplication_effect(SkipTagId::Double);
            assert!(result.messages[0].contains("Cannot duplicate Double"));
        }
    }
}
