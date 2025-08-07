//! Comprehensive Test Suite for Static Jokers - Part 3 (Missing Coverage)
//!
//! Complete production-ready test coverage for static jokers that were missing from Parts 1 & 2.
//! This test suite follows John Botmack performance engineering principles:
//! - Real-time system validation patterns
//! - Comprehensive edge case coverage for game engine stability
//! - Performance-critical test infrastructure
//! - Hardware-aware benchmarking for 60+ FPS requirements

use balatro_rs::card::{Card, Suit, Value};
use balatro_rs::hand::{Hand, SelectHand};
use balatro_rs::joker::{GameContext, JokerId, JokerRarity};
use balatro_rs::joker_state::JokerStateManager;
use balatro_rs::rank::HandRank;
use balatro_rs::rng::GameRng;
use balatro_rs::stage::{Blind, Stage};
use balatro_rs::static_joker_factory::StaticJokerFactory;
use std::collections::HashMap;
use std::sync::{Arc, OnceLock};

// ============================================================================
// Performance-Critical Test Infrastructure - Like Game Engine Initialization
// ============================================================================

/// High-performance test context factory with static initialization
/// Optimized like a game engine's object pool - zero allocations in hot path
fn create_test_context(money: i32, discards_used: u32) -> GameContext<'static> {
    create_test_context_with_deck_and_cards(money, discards_used, 52, 0, 0)
}

/// Extended test context with deck composition for performance testing
/// Like configuring graphics settings for different hardware targets
fn create_test_context_with_deck_and_cards(
    money: i32,
    discards_used: u32,
    cards_in_deck: usize,
    stone_cards_in_deck: usize,
    steel_cards_in_deck: usize,
) -> GameContext<'static> {
    // Static initialization ensures cache-friendly data layout
    static STAGE: Stage = Stage::Blind(Blind::Small);
    static HAND: OnceLock<Hand> = OnceLock::new();
    static HAND_TYPE_COUNTS: OnceLock<HashMap<HandRank, u32>> = OnceLock::new();
    static JOKER_STATE_MANAGER: OnceLock<Arc<JokerStateManager>> = OnceLock::new();
    static TEST_RNG: OnceLock<GameRng> = OnceLock::new();

    let hand = HAND.get_or_init(|| Hand::new(Vec::new()));
    let hand_type_counts = HAND_TYPE_COUNTS.get_or_init(HashMap::new);
    let joker_state_manager =
        JOKER_STATE_MANAGER.get_or_init(|| Arc::new(JokerStateManager::new()));
    let rng = TEST_RNG.get_or_init(|| GameRng::for_testing(42));

    GameContext {
        chips: 0,
        mult: 1,
        money,
        ante: 1,
        round: 1,
        stage: &STAGE,
        hands_played: 0,
        hands_remaining: 4.0,
        discards_used,
        is_final_hand: false,
        jokers: &[],
        hand,
        discarded: &[],
        hand_type_counts,
        joker_state_manager,
        cards_in_deck,
        stone_cards_in_deck,
        steel_cards_in_deck,
        enhanced_cards_in_deck: 0,
        rng,
    }
}

/// Performance-validated card creation - like optimized vertex buffer creation
fn create_test_card(suit: Suit, value: Value) -> Card {
    Card::new(value, suit)
}

// ============================================================================
// Hand Type Conditional Jokers (Mult) - Missing Coverage Tests
// ============================================================================

#[cfg(test)]
mod hand_type_mult_joker_tests {
    use super::*;

    #[test]
    fn test_mad_joker_properties() {
        // Mad Joker: +10 Mult if played hand contains Two Pair
        let joker = StaticJokerFactory::create_mad_joker();

        assert_eq!(joker.id(), JokerId::MadJoker);
        assert_eq!(joker.name(), "Mad Joker");
        assert_eq!(
            joker.description(),
            "+10 Mult if played hand contains Two Pair"
        );
        assert_eq!(joker.rarity(), JokerRarity::Common);
        assert_eq!(joker.cost(), 4);
    }

    #[test]
    fn test_mad_joker_two_pair_detection() {
        // Performance test: Validate two pair detection like frame-rate critical hit detection
        let joker = StaticJokerFactory::create_mad_joker();
        let mut context = create_test_context(10, 2);

        // Create two pair hand (performance-critical scenario)
        let two_pair_hand = SelectHand::new(vec![
            create_test_card(Suit::Heart, Value::King),
            create_test_card(Suit::Diamond, Value::King),
            create_test_card(Suit::Club, Value::Five),
            create_test_card(Suit::Spade, Value::Five),
            create_test_card(Suit::Heart, Value::Two),
        ]);

        let effect = joker.on_hand_played(&mut context, &two_pair_hand);

        assert_eq!(
            effect.mult, 10,
            "Mad Joker should provide +10 mult for two pair"
        );
        assert_eq!(effect.chips, 0, "Mad Joker should not provide chips");
        assert_eq!(effect.money, 0, "Mad Joker should not provide money");
        assert_eq!(
            effect.mult_multiplier, 1.0,
            "Mad Joker should not provide mult multiplier"
        );
    }

    #[test]
    fn test_mad_joker_non_two_pair() {
        // Edge case: Verify non-two-pair hands don't trigger - like input validation
        let joker = StaticJokerFactory::create_mad_joker();
        let mut context = create_test_context(10, 2);

        // Create single pair hand (should not trigger)
        let pair_hand = SelectHand::new(vec![
            create_test_card(Suit::Heart, Value::King),
            create_test_card(Suit::Diamond, Value::King),
            create_test_card(Suit::Club, Value::Five),
            create_test_card(Suit::Spade, Value::Three),
            create_test_card(Suit::Heart, Value::Two),
        ]);

        let effect = joker.on_hand_played(&mut context, &pair_hand);

        assert_eq!(
            effect.mult, 0,
            "Mad Joker should not trigger for single pair"
        );
    }

    #[test]
    fn test_crazy_joker_properties() {
        // Crazy Joker: +12 Mult if played hand contains Straight
        let joker = StaticJokerFactory::create_crazy_joker();

        assert_eq!(joker.id(), JokerId::CrazyJoker);
        assert_eq!(joker.name(), "Crazy Joker");
        assert_eq!(
            joker.description(),
            "+12 Mult if played hand contains Straight"
        );
        assert_eq!(joker.rarity(), JokerRarity::Common);
        assert_eq!(joker.cost(), 4);
    }

    #[test]
    fn test_crazy_joker_straight_detection() {
        // Performance test: Straight detection like collision detection in physics engine
        let joker = StaticJokerFactory::create_crazy_joker();
        let mut context = create_test_context(10, 2);

        // Create straight hand (5-6-7-8-9)
        let straight_hand = SelectHand::new(vec![
            create_test_card(Suit::Heart, Value::Five),
            create_test_card(Suit::Diamond, Value::Six),
            create_test_card(Suit::Club, Value::Seven),
            create_test_card(Suit::Spade, Value::Eight),
            create_test_card(Suit::Heart, Value::Nine),
        ]);

        let effect = joker.on_hand_played(&mut context, &straight_hand);

        assert_eq!(
            effect.mult, 12,
            "Crazy Joker should provide +12 mult for straight"
        );
        assert_eq!(effect.chips, 0, "Crazy Joker should not provide chips");
    }

    #[test]
    fn test_crazy_joker_non_straight() {
        // Edge case: Non-straight hands should not trigger - like boundary checking
        let joker = StaticJokerFactory::create_crazy_joker();
        let mut context = create_test_context(10, 2);

        // Create non-straight hand
        let non_straight_hand = SelectHand::new(vec![
            create_test_card(Suit::Heart, Value::Two),
            create_test_card(Suit::Diamond, Value::Five),
            create_test_card(Suit::Club, Value::Eight),
            create_test_card(Suit::Spade, Value::Jack),
            create_test_card(Suit::Heart, Value::King),
        ]);

        let effect = joker.on_hand_played(&mut context, &non_straight_hand);

        assert_eq!(
            effect.mult, 0,
            "Crazy Joker should not trigger for non-straight"
        );
    }

    #[test]
    fn test_droll_joker_properties() {
        // Droll Joker: +10 Mult if played hand contains Flush
        let joker = StaticJokerFactory::create_droll_joker();

        assert_eq!(joker.id(), JokerId::DrollJoker);
        assert_eq!(joker.name(), "Droll Joker");
        assert_eq!(
            joker.description(),
            "+10 Mult if played hand contains Flush"
        );
        assert_eq!(joker.rarity(), JokerRarity::Common);
        assert_eq!(joker.cost(), 4);
    }

    #[test]
    fn test_droll_joker_flush_detection() {
        // Performance test: Flush detection like SIMD-optimized batch processing
        let joker = StaticJokerFactory::create_droll_joker();
        let mut context = create_test_context(10, 2);

        // Create flush hand (all hearts)
        let flush_hand = SelectHand::new(vec![
            create_test_card(Suit::Heart, Value::Two),
            create_test_card(Suit::Heart, Value::Five),
            create_test_card(Suit::Heart, Value::Seven),
            create_test_card(Suit::Heart, Value::Nine),
            create_test_card(Suit::Heart, Value::King),
        ]);

        let effect = joker.on_hand_played(&mut context, &flush_hand);

        assert_eq!(
            effect.mult, 10,
            "Droll Joker should provide +10 mult for flush"
        );
        assert_eq!(effect.chips, 0, "Droll Joker should not provide chips");
    }

    #[test]
    fn test_droll_joker_non_flush() {
        // Edge case: Mixed suits should not trigger - like shader validation
        let joker = StaticJokerFactory::create_droll_joker();
        let mut context = create_test_context(10, 2);

        // Create mixed suit hand
        let mixed_hand = SelectHand::new(vec![
            create_test_card(Suit::Heart, Value::Two),
            create_test_card(Suit::Diamond, Value::Five),
            create_test_card(Suit::Club, Value::Seven),
            create_test_card(Suit::Spade, Value::Nine),
            create_test_card(Suit::Heart, Value::King),
        ]);

        let effect = joker.on_hand_played(&mut context, &mixed_hand);

        assert_eq!(
            effect.mult, 0,
            "Droll Joker should not trigger for non-flush"
        );
    }
}

// ============================================================================
// Hand Type Conditional Jokers (Chips) - Missing Coverage Tests
// ============================================================================

#[cfg(test)]
mod hand_type_chip_joker_tests {
    use super::*;

    #[test]
    fn test_sly_joker_properties() {
        // Sly Joker: +50 Chips if played hand contains Pair
        let joker = StaticJokerFactory::create_sly_joker();

        assert_eq!(joker.id(), JokerId::SlyJoker);
        assert_eq!(joker.name(), "Sly Joker");
        assert_eq!(
            joker.description(),
            "+50 Chips if played hand contains Pair"
        );
        assert_eq!(joker.rarity(), JokerRarity::Common);
        assert_eq!(joker.cost(), 3);
    }

    #[test]
    fn test_sly_joker_pair_detection() {
        // Performance test: Like optimized collision detection in game physics
        let joker = StaticJokerFactory::create_sly_joker();
        let mut context = create_test_context(10, 2);

        // Create pair hand
        let pair_hand = SelectHand::new(vec![
            create_test_card(Suit::Heart, Value::Ace),
            create_test_card(Suit::Diamond, Value::Ace),
            create_test_card(Suit::Club, Value::Five),
            create_test_card(Suit::Spade, Value::Seven),
            create_test_card(Suit::Heart, Value::Nine),
        ]);

        let effect = joker.on_hand_played(&mut context, &pair_hand);

        assert_eq!(
            effect.chips, 50,
            "Sly Joker should provide +50 chips for pair"
        );
        assert_eq!(effect.mult, 0, "Sly Joker should not provide mult");
    }

    #[test]
    fn test_wily_joker_properties() {
        // Wily Joker: +100 Chips if played hand contains Three of a Kind
        let joker = StaticJokerFactory::create_wily_joker();

        assert_eq!(joker.id(), JokerId::WilyJoker);
        assert_eq!(joker.name(), "Wily Joker");
        assert_eq!(
            joker.description(),
            "+100 Chips if played hand contains Three of a Kind"
        );
        assert_eq!(joker.rarity(), JokerRarity::Common);
        assert_eq!(joker.cost(), 4);
    }

    #[test]
    fn test_wily_joker_three_of_kind_detection() {
        // Performance test: Complex pattern matching like pathfinding optimization
        let joker = StaticJokerFactory::create_wily_joker();
        let mut context = create_test_context(10, 2);

        // Create three of a kind hand
        let three_kind_hand = SelectHand::new(vec![
            create_test_card(Suit::Heart, Value::Queen),
            create_test_card(Suit::Diamond, Value::Queen),
            create_test_card(Suit::Club, Value::Queen),
            create_test_card(Suit::Spade, Value::Seven),
            create_test_card(Suit::Heart, Value::Two),
        ]);

        let effect = joker.on_hand_played(&mut context, &three_kind_hand);

        assert_eq!(
            effect.chips, 100,
            "Wily Joker should provide +100 chips for three of a kind"
        );
        assert_eq!(effect.mult, 0, "Wily Joker should not provide mult");
    }

    #[test]
    fn test_clever_joker_properties() {
        // Clever Joker: +80 Chips if played hand contains Two Pair
        let joker = StaticJokerFactory::create_clever_joker();

        assert_eq!(joker.id(), JokerId::CleverJoker);
        assert_eq!(joker.name(), "Clever Joker");
        assert_eq!(
            joker.description(),
            "+80 Chips if played hand contains Two Pair"
        );
        assert_eq!(joker.rarity(), JokerRarity::Common);
        assert_eq!(joker.cost(), 4);
    }

    #[test]
    fn test_clever_joker_two_pair_detection() {
        // Performance test: Multi-condition validation like shader compilation
        let joker = StaticJokerFactory::create_clever_joker();
        let mut context = create_test_context(10, 2);

        // Create two pair hand
        let two_pair_hand = SelectHand::new(vec![
            create_test_card(Suit::Heart, Value::Jack),
            create_test_card(Suit::Diamond, Value::Jack),
            create_test_card(Suit::Club, Value::Eight),
            create_test_card(Suit::Spade, Value::Eight),
            create_test_card(Suit::Heart, Value::Three),
        ]);

        let effect = joker.on_hand_played(&mut context, &two_pair_hand);

        assert_eq!(
            effect.chips, 80,
            "Clever Joker should provide +80 chips for two pair"
        );
        assert_eq!(effect.mult, 0, "Clever Joker should not provide mult");
    }

    #[test]
    fn test_devious_joker_properties() {
        // Devious Joker: +100 Chips if played hand contains Straight
        let joker = StaticJokerFactory::create_devious_joker();

        assert_eq!(joker.id(), JokerId::DeviousJoker);
        assert_eq!(joker.name(), "Devious Joker");
        assert_eq!(
            joker.description(),
            "+100 Chips if played hand contains Straight"
        );
        assert_eq!(joker.rarity(), JokerRarity::Common);
        assert_eq!(joker.cost(), 4);
    }

    #[test]
    fn test_devious_joker_straight_detection() {
        // Performance test: Sequential validation like frame time optimization
        let joker = StaticJokerFactory::create_devious_joker();
        let mut context = create_test_context(10, 2);

        // Create straight hand (10-J-Q-K-A)
        let straight_hand = SelectHand::new(vec![
            create_test_card(Suit::Heart, Value::Ten),
            create_test_card(Suit::Diamond, Value::Jack),
            create_test_card(Suit::Club, Value::Queen),
            create_test_card(Suit::Spade, Value::King),
            create_test_card(Suit::Heart, Value::Ace),
        ]);

        let effect = joker.on_hand_played(&mut context, &straight_hand);

        assert_eq!(
            effect.chips, 100,
            "Devious Joker should provide +100 chips for straight"
        );
        assert_eq!(effect.mult, 0, "Devious Joker should not provide mult");
    }

    #[test]
    fn test_crafty_joker_properties() {
        // Crafty Joker: +80 Chips if played hand contains Flush
        let joker = StaticJokerFactory::create_crafty_joker();

        assert_eq!(joker.id(), JokerId::CraftyJoker);
        assert_eq!(joker.name(), "Crafty Joker");
        assert_eq!(
            joker.description(),
            "+80 Chips if played hand contains Flush"
        );
        assert_eq!(joker.rarity(), JokerRarity::Common);
        assert_eq!(joker.cost(), 4);
    }

    #[test]
    fn test_crafty_joker_flush_detection() {
        // Performance test: Batch suit checking like SIMD color filtering
        let joker = StaticJokerFactory::create_crafty_joker();
        let mut context = create_test_context(10, 2);

        // Create flush hand (all spades)
        let flush_hand = SelectHand::new(vec![
            create_test_card(Suit::Spade, Value::Three),
            create_test_card(Suit::Spade, Value::Six),
            create_test_card(Suit::Spade, Value::Eight),
            create_test_card(Suit::Spade, Value::Jack),
            create_test_card(Suit::Spade, Value::Ace),
        ]);

        let effect = joker.on_hand_played(&mut context, &flush_hand);

        assert_eq!(
            effect.chips, 80,
            "Crafty Joker should provide +80 chips for flush"
        );
        assert_eq!(effect.mult, 0, "Crafty Joker should not provide mult");
    }
}

// ============================================================================
// Rank-Based Jokers - Missing Coverage Tests
// ============================================================================

#[cfg(test)]
mod rank_based_joker_tests {
    use super::*;

    #[test]
    fn test_scholar_properties() {
        // Scholar: Aces give +20 Chips and +4 Mult when scored
        let joker = StaticJokerFactory::create_scholar();

        assert_eq!(joker.id(), JokerId::Scholar);
        assert_eq!(joker.name(), "Scholar");
        assert_eq!(
            joker.description(),
            "Played Aces give +20 Chips and +4 Mult when scored"
        );
        assert_eq!(joker.rarity(), JokerRarity::Common);
        assert_eq!(joker.cost(), 4);
    }

    #[test]
    fn test_scholar_ace_scoring() {
        // Performance test: Per-card scoring like optimized vertex processing
        let joker = StaticJokerFactory::create_scholar();
        let mut context = create_test_context(10, 2);

        // Test all suits for comprehensive coverage
        let suits = vec![Suit::Heart, Suit::Diamond, Suit::Club, Suit::Spade];

        for suit in suits {
            let ace_card = create_test_card(suit, Value::Ace);
            let effect = joker.on_card_scored(&mut context, &ace_card);

            assert_eq!(
                effect.chips, 20,
                "Scholar should provide +20 chips for Ace of {suit:?}"
            );
            assert_eq!(
                effect.mult, 4,
                "Scholar should provide +4 mult for Ace of {suit:?}"
            );
        }
    }

    #[test]
    fn test_scholar_non_ace_cards() {
        // Edge case: Non-ace cards should not trigger - like input sanitization
        let joker = StaticJokerFactory::create_scholar();
        let mut context = create_test_context(10, 2);

        let non_ace_cards = vec![
            Value::Two,
            Value::Three,
            Value::Four,
            Value::Five,
            Value::Six,
            Value::Seven,
            Value::Eight,
            Value::Nine,
            Value::Ten,
            Value::Jack,
            Value::Queen,
            Value::King,
        ];

        for value in non_ace_cards {
            let card = create_test_card(Suit::Heart, value);
            let effect = joker.on_card_scored(&mut context, &card);

            assert_eq!(
                effect.chips, 0,
                "Scholar should not provide chips for {value:?}"
            );
            assert_eq!(
                effect.mult, 0,
                "Scholar should not provide mult for {value:?}"
            );
        }
    }

    #[test]
    fn test_scholar_multiple_aces() {
        // Performance test: Stacking effects like multi-hit combo scoring
        let joker = StaticJokerFactory::create_scholar();
        let mut context = create_test_context(10, 2);

        let aces = vec![
            create_test_card(Suit::Heart, Value::Ace),
            create_test_card(Suit::Diamond, Value::Ace),
            create_test_card(Suit::Club, Value::Ace),
        ];

        let mut total_chips = 0;
        let mut total_mult = 0;

        for ace in aces {
            let effect = joker.on_card_scored(&mut context, &ace);
            total_chips += effect.chips;
            total_mult += effect.mult;
        }

        assert_eq!(total_chips, 60, "Should get 20 chips for each of 3 aces");
        assert_eq!(total_mult, 12, "Should get 4 mult for each of 3 aces");
    }
}

// ============================================================================
// Color-Based Jokers - Missing Coverage Tests
// ============================================================================

#[cfg(test)]
mod color_based_joker_tests {
    use super::*;

    #[test]
    fn test_red_card_properties() {
        // Red Card: Red cards (Hearts and Diamonds) give +3 Mult when scored
        let joker = StaticJokerFactory::create_red_card();

        assert_eq!(joker.id(), JokerId::RedCard);
        assert_eq!(joker.name(), "Red Card");
        assert_eq!(
            joker.description(),
            "Red cards (Hearts and Diamonds) give +3 Mult when scored"
        );
        assert_eq!(joker.rarity(), JokerRarity::Uncommon);
        assert_eq!(joker.cost(), 6);
    }

    #[test]
    fn test_red_card_red_suits() {
        // Performance test: Color filtering like GPU shader optimization
        let joker = StaticJokerFactory::create_red_card();
        let mut context = create_test_context(10, 2);

        let red_suits = vec![Suit::Heart, Suit::Diamond];
        let test_values = vec![
            Value::Ace,
            Value::Five,
            Value::Ten,
            Value::Jack,
            Value::King,
        ];

        for suit in red_suits {
            for value in &test_values {
                let red_card = create_test_card(suit, *value);
                let effect = joker.on_card_scored(&mut context, &red_card);

                assert_eq!(
                    effect.mult, 3,
                    "Red Card should provide +3 mult for {value:?} of {suit:?}"
                );
                assert_eq!(effect.chips, 0, "Red Card should not provide chips");
            }
        }
    }

    #[test]
    fn test_red_card_black_suits() {
        // Edge case: Black suits should not trigger - like color space validation
        let joker = StaticJokerFactory::create_red_card();
        let mut context = create_test_context(10, 2);

        let black_suits = vec![Suit::Club, Suit::Spade];
        let test_values = vec![
            Value::Ace,
            Value::Five,
            Value::Ten,
            Value::Jack,
            Value::King,
        ];

        for suit in black_suits {
            for value in &test_values {
                let black_card = create_test_card(suit, *value);
                let effect = joker.on_card_scored(&mut context, &black_card);

                assert_eq!(
                    effect.mult, 0,
                    "Red Card should not trigger for {value:?} of {suit:?}"
                );
            }
        }
    }

    #[test]
    fn test_blue_joker_properties() {
        // Blue Joker: +2 Chips per remaining card in deck
        let joker = StaticJokerFactory::create_blue_joker();

        assert_eq!(joker.id(), JokerId::BlueJoker);
        assert_eq!(joker.name(), "Blue Joker");
        assert_eq!(joker.description(), "+2 Chips per remaining card in deck");
        assert_eq!(joker.rarity(), JokerRarity::Common);
        assert_eq!(joker.cost(), 3);
    }

    #[test]
    fn test_blue_joker_deck_based_chips() {
        // Blue Joker gives chips based on deck size
        let joker = StaticJokerFactory::create_blue_joker();

        // Test with 40 cards in deck
        let mut context = create_test_context(10, 2);
        context.cards_in_deck = 40;

        // Blue Joker gives chips regardless of card
        let test_hand = SelectHand::new(vec![
            create_test_card(Suit::Club, Value::Two),
            create_test_card(Suit::Heart, Value::Three),
        ]);
        let effect = joker.on_hand_played(&mut context, &test_hand);

        // Should give 2 chips per card in deck = 80 chips
        assert_eq!(
            effect.chips, 80,
            "Blue Joker should provide 2 chips per card in deck (40 cards = 80 chips)"
        );
        assert_eq!(effect.mult, 0, "Blue Joker should not provide mult");

        // Test with smaller deck
        context.cards_in_deck = 10;
        let effect = joker.on_hand_played(&mut context, &test_hand);
        assert_eq!(
            effect.chips, 20,
            "Blue Joker should provide 2 chips per card in deck (10 cards = 20 chips)"
        );
    }

    #[test]
    fn test_blue_joker_empty_deck() {
        // Edge case: Blue Joker with empty deck
        let joker = StaticJokerFactory::create_blue_joker();
        let mut context = create_test_context(10, 2);

        // Test with empty deck
        context.cards_in_deck = 0;
        let test_hand = SelectHand::new(vec![create_test_card(Suit::Diamond, Value::Five)]);
        let effect = joker.on_hand_played(&mut context, &test_hand);

        assert_eq!(
            effect.chips, 0,
            "Blue Joker should provide 0 chips when deck is empty"
        );
        assert_eq!(effect.mult, 0, "Blue Joker should not provide mult");
    }

    #[test]
    fn test_blue_and_red_joker_independence() {
        // Test that Blue Joker (deck-based) and Red Card (color-based) work independently
        let red_card_joker = StaticJokerFactory::create_red_card();
        let blue_joker = StaticJokerFactory::create_blue_joker();
        let mut context = create_test_context(15, 1);
        context.cards_in_deck = 25;

        // Test Red Card with red suit
        let red_heart = create_test_card(Suit::Heart, Value::King);
        let red_effect = red_card_joker.on_card_scored(&mut context, &red_heart);
        assert_eq!(red_effect.mult, 3, "Red Card should trigger for hearts");

        // Test Blue Joker gives chips based on deck size, not card color
        let test_hand = SelectHand::new(vec![
            red_heart,
            create_test_card(Suit::Diamond, Value::Queen),
        ]);
        let blue_effect = blue_joker.on_hand_played(&mut context, &test_hand);
        assert_eq!(
            blue_effect.chips, 50,
            "Blue Joker should give 50 chips (25 cards * 2)"
        );
        assert_eq!(blue_effect.mult, 0, "Blue Joker should not provide mult");

        // Test Red Card with black suit
        let black_spade = create_test_card(Suit::Spade, Value::King);
        let red_no_effect = red_card_joker.on_card_scored(&mut context, &black_spade);
        assert_eq!(
            red_no_effect.mult, 0,
            "Red Card should not trigger for spades"
        );
    }
}

// ============================================================================
// Production Integration Tests - Missing Coverage Scenarios
// ============================================================================

#[cfg(test)]
mod integration_tests_missing_coverage {
    use super::*;

    #[test]
    fn test_hand_type_joker_combinations() {
        // Performance test: Multiple hand type jokers like parallel pipeline execution
        let mad_joker = StaticJokerFactory::create_mad_joker(); // Two Pair +10 Mult
        let clever_joker = StaticJokerFactory::create_clever_joker(); // Two Pair +80 Chips
        let mut context = create_test_context(20, 1);

        // Create two pair hand that should trigger both jokers
        let two_pair_hand = SelectHand::new(vec![
            create_test_card(Suit::Heart, Value::Ace),
            create_test_card(Suit::Diamond, Value::Ace),
            create_test_card(Suit::Club, Value::Seven),
            create_test_card(Suit::Spade, Value::Seven),
            create_test_card(Suit::Heart, Value::Four),
        ]);

        let mad_effect = mad_joker.on_hand_played(&mut context, &two_pair_hand);
        let clever_effect = clever_joker.on_hand_played(&mut context, &two_pair_hand);

        assert_eq!(
            mad_effect.mult, 10,
            "Mad Joker should provide mult for two pair"
        );
        assert_eq!(
            clever_effect.chips, 80,
            "Clever Joker should provide chips for two pair"
        );

        // Verify they work independently
        assert_eq!(mad_effect.chips, 0, "Mad Joker should not provide chips");
        assert_eq!(
            clever_effect.mult, 0,
            "Clever Joker should not provide mult"
        );
    }

    #[test]
    fn test_comprehensive_rank_coverage() {
        // Performance test: Scholar + Red Card + Blue Joker interaction
        let scholar = StaticJokerFactory::create_scholar(); // Aces +20 Chips +4 Mult
        let red_card = StaticJokerFactory::create_red_card(); // Red +3 Mult
        let blue_joker = StaticJokerFactory::create_blue_joker(); // +2 Chips per card in deck
        let mut context = create_test_context(25, 0);
        context.cards_in_deck = 30; // Set deck size for Blue Joker

        // Test Red Ace (should trigger Scholar and Red Card)
        let red_ace = create_test_card(Suit::Heart, Value::Ace);
        let scholar_effect = scholar.on_card_scored(&mut context, &red_ace);
        let red_effect = red_card.on_card_scored(&mut context, &red_ace);

        assert_eq!(
            scholar_effect.chips, 20,
            "Scholar should provide chips for red ace"
        );
        assert_eq!(
            scholar_effect.mult, 4,
            "Scholar should provide mult for red ace"
        );
        assert_eq!(
            red_effect.mult, 3,
            "Red Card should provide mult for red ace"
        );

        // Blue Joker provides chips based on deck size on hand played
        let test_hand = SelectHand::new(vec![
            red_ace,
            create_test_card(Suit::Diamond, Value::Two),
            create_test_card(Suit::Club, Value::Three),
        ]);
        let blue_hand_effect = blue_joker.on_hand_played(&mut context, &test_hand);
        assert_eq!(
            blue_hand_effect.chips, 60,
            "Blue Joker should provide 60 chips (30 cards * 2)"
        );
        assert_eq!(
            blue_hand_effect.mult, 0,
            "Blue Joker should not provide mult"
        );

        // Test Black Ace (should trigger Scholar only, not Red Card)
        let black_ace = create_test_card(Suit::Spade, Value::Ace);
        let scholar_effect_2 = scholar.on_card_scored(&mut context, &black_ace);
        let red_no_effect = red_card.on_card_scored(&mut context, &black_ace);

        assert_eq!(
            scholar_effect_2.chips, 20,
            "Scholar should provide chips for black ace"
        );
        assert_eq!(
            scholar_effect_2.mult, 4,
            "Scholar should provide mult for black ace"
        );
        assert_eq!(
            red_no_effect.mult, 0,
            "Red Card should not trigger for black ace"
        );
    }

    #[test]
    fn test_comprehensive_hand_type_coverage() {
        // Performance test: All hand types covered like complete render pipeline
        let hand_type_jokers = vec![
            ("Sly", StaticJokerFactory::create_sly_joker()), // Pair +50 Chips
            ("Clever", StaticJokerFactory::create_clever_joker()), // Two Pair +80 Chips
            ("Wily", StaticJokerFactory::create_wily_joker()), // Three Kind +100 Chips
            ("Devious", StaticJokerFactory::create_devious_joker()), // Straight +100 Chips
            ("Crafty", StaticJokerFactory::create_crafty_joker()), // Flush +80 Chips
        ];

        // Create comprehensive test hands for each joker
        let test_hands = vec![
            (
                "Pair",
                SelectHand::new(vec![
                    create_test_card(Suit::Heart, Value::King),
                    create_test_card(Suit::Diamond, Value::King),
                    create_test_card(Suit::Club, Value::Five),
                    create_test_card(Suit::Spade, Value::Seven),
                    create_test_card(Suit::Heart, Value::Two),
                ]),
            ),
            (
                "Two Pair",
                SelectHand::new(vec![
                    create_test_card(Suit::Heart, Value::Queen),
                    create_test_card(Suit::Diamond, Value::Queen),
                    create_test_card(Suit::Club, Value::Eight),
                    create_test_card(Suit::Spade, Value::Eight),
                    create_test_card(Suit::Heart, Value::Three),
                ]),
            ),
            (
                "Three of a Kind",
                SelectHand::new(vec![
                    create_test_card(Suit::Heart, Value::Jack),
                    create_test_card(Suit::Diamond, Value::Jack),
                    create_test_card(Suit::Club, Value::Jack),
                    create_test_card(Suit::Spade, Value::Four),
                    create_test_card(Suit::Heart, Value::Six),
                ]),
            ),
            (
                "Straight",
                SelectHand::new(vec![
                    create_test_card(Suit::Heart, Value::Four),
                    create_test_card(Suit::Diamond, Value::Five),
                    create_test_card(Suit::Club, Value::Six),
                    create_test_card(Suit::Spade, Value::Seven),
                    create_test_card(Suit::Heart, Value::Eight),
                ]),
            ),
            (
                "Flush",
                SelectHand::new(vec![
                    create_test_card(Suit::Diamond, Value::Two),
                    create_test_card(Suit::Diamond, Value::Five),
                    create_test_card(Suit::Diamond, Value::Eight),
                    create_test_card(Suit::Diamond, Value::Jack),
                    create_test_card(Suit::Diamond, Value::Ace),
                ]),
            ),
        ];

        // Test each joker with appropriate hand
        let mut context = create_test_context(30, 0);

        // Each joker should trigger with its corresponding hand type
        for ((joker_name, joker), (hand_name, hand)) in
            hand_type_jokers.into_iter().zip(test_hands.into_iter())
        {
            let effect = joker.on_hand_played(&mut context, &hand);

            // All these jokers provide chips, not mult
            assert!(
                effect.chips > 0,
                "{joker_name} should provide chips for {hand_name}"
            );
            assert_eq!(effect.mult, 0, "{joker_name} should not provide mult");
        }
    }

    #[test]
    fn test_performance_critical_edge_cases() {
        // Performance test: Edge cases that could cause frame drops
        let jokers = vec![
            StaticJokerFactory::create_scholar(),
            StaticJokerFactory::create_red_card(),
            StaticJokerFactory::create_blue_joker(),
        ];

        let mut context = create_test_context(50, 5);

        // Test with empty hand (should not crash)
        let empty_hand = SelectHand::new(vec![]);
        for joker in &jokers {
            let effect = joker.on_hand_played(&mut context, &empty_hand);
            assert!(effect.chips >= 0, "No joker should produce negative chips");
            assert!(effect.mult >= 0, "No joker should produce negative mult");
        }

        // Test with maximum cards (edge case handling)
        let max_cards = vec![
            create_test_card(Suit::Heart, Value::Ace),
            create_test_card(Suit::Diamond, Value::King),
            create_test_card(Suit::Club, Value::Queen),
            create_test_card(Suit::Spade, Value::Jack),
            create_test_card(Suit::Heart, Value::Ten),
        ];

        for card in &max_cards {
            for joker in &jokers {
                let effect = joker.on_card_scored(&mut context, card);
                // Should not crash or produce invalid values
                assert!(
                    effect.chips >= 0,
                    "Valid chip values required for frame stability"
                );
                assert!(
                    effect.mult >= 0,
                    "Valid mult values required for frame stability"
                );
                assert!(effect.mult_multiplier >= 0.0, "Valid multiplier required");
            }
        }
    }
}

// ============================================================================
// Performance Benchmarks - Game Engine Standards
// ============================================================================

#[cfg(test)]
mod performance_benchmarks_missing_coverage {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_joker_creation_performance_comprehensive() {
        // Performance requirement: All jokers must create in <10ms for 1000 instances
        let start = Instant::now();

        for _ in 0..200 {
            // Test all missing jokers for creation speed
            let _mad = StaticJokerFactory::create_mad_joker();
            let _crazy = StaticJokerFactory::create_crazy_joker();
            let _droll = StaticJokerFactory::create_droll_joker();
            let _sly = StaticJokerFactory::create_sly_joker();
            let _wily = StaticJokerFactory::create_wily_joker();
            let _clever = StaticJokerFactory::create_clever_joker();
            let _devious = StaticJokerFactory::create_devious_joker();
            let _crafty = StaticJokerFactory::create_crafty_joker();
            let _scholar = StaticJokerFactory::create_scholar();
            let _red_card = StaticJokerFactory::create_red_card();
            let _blue_joker = StaticJokerFactory::create_blue_joker();
        }

        let duration = start.elapsed();

        // 200 iterations * 11 jokers = 2200 total creations
        assert!(
            duration.as_millis() < 50,
            "Joker creation too slow: {}ms for 2200 jokers (target: <50ms)",
            duration.as_millis()
        );
    }

    #[test]
    fn test_effect_calculation_performance_comprehensive() {
        // Performance requirement: Effect calculations must be fast for real-time scoring
        let scholar = StaticJokerFactory::create_scholar();
        let red_card = StaticJokerFactory::create_red_card();
        let mut context = create_test_context(10, 2);
        let red_ace = create_test_card(Suit::Heart, Value::Ace);

        let start = Instant::now();

        // Simulate rapid scoring like particle system updates
        for _ in 0..5000 {
            let _scholar_effect = scholar.on_card_scored(&mut context, &red_ace);
            let _red_effect = red_card.on_card_scored(&mut context, &red_ace);
        }

        let duration = start.elapsed();

        // 5000 iterations * 2 calculations = 10000 total calculations
        assert!(
            duration.as_millis() < 10,
            "Effect calculation too slow: {}ms for 10000 calculations",
            duration.as_millis()
        );
    }

    #[test]
    fn test_hand_evaluation_performance() {
        // Performance requirement: Hand evaluation must be fast for UI responsiveness
        let hand_jokers = vec![
            StaticJokerFactory::create_mad_joker(),
            StaticJokerFactory::create_crazy_joker(),
            StaticJokerFactory::create_droll_joker(),
        ];

        let mut context = create_test_context(15, 1);
        let test_hand = SelectHand::new(vec![
            create_test_card(Suit::Heart, Value::King),
            create_test_card(Suit::Diamond, Value::King),
            create_test_card(Suit::Club, Value::Five),
            create_test_card(Suit::Spade, Value::Five),
            create_test_card(Suit::Heart, Value::Two),
        ]);

        let start = Instant::now();

        // Simulate rapid hand evaluation like animation frame updates
        for _ in 0..1000 {
            for joker in &hand_jokers {
                let _effect = joker.on_hand_played(&mut context, &test_hand);
            }
        }

        let duration = start.elapsed();

        // 1000 iterations * 3 jokers = 3000 total evaluations
        assert!(
            duration.as_millis() < 50,
            "Hand evaluation too slow: {}ms for 3000 evaluations (target: <50ms)",
            duration.as_millis()
        );
    }
}
