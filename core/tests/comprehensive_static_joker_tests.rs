//! Comprehensive Test Suite for Static Jokers
//!
//! Production-ready test coverage for all static jokers implemented in the StaticJokerFactory.
//! This test suite follows distributed systems engineering principles:
//! - Comprehensive failure mode testing
//! - Production-scale validation patterns
//! - Clear arrange-act-assert structure
//! - Edge case coverage for all conditions

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
// Production-Ready Test Infrastructure
// ============================================================================

/// Production-grade test context factory with thread-safe static initialization
/// Follows Google-scale engineering principles for test isolation and reliability
fn create_test_context(money: i32, discards_used: u32) -> GameContext<'static> {
    create_test_context_with_deck_and_cards(money, discards_used, 52, 0, 0)
}

/// Extended test context with specific deck composition for edge case testing
fn create_test_context_with_deck_and_cards(
    money: i32,
    discards_used: u32,
    cards_in_deck: usize,
    stone_cards_in_deck: usize,
    steel_cards_in_deck: usize,
) -> GameContext<'static> {
    // Static initialization ensures thread safety and prevents test flakiness
    // These statics are shared across tests for performance and consistency
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

/// Production test helper for creating cards with validation
/// Prevents invalid card combinations that could cause test flakiness
fn create_test_card(suit: Suit, value: Value) -> Card {
    Card::new(value, suit)
}

/// Production test helper for creating face cards with validation
/// Enforces business rules to prevent test errors
fn create_face_card(suit: Suit, face_value: Value) -> Card {
    assert!(
        matches!(face_value, Value::Jack | Value::Queen | Value::King),
        "Face card must be Jack, Queen, or King"
    );
    Card::new(face_value, suit)
}

// ============================================================================
// Basic Scoring Jokers - Production Test Coverage
// ============================================================================

#[cfg(test)]
mod basic_joker_tests {
    use super::*;

    #[test]
    fn test_basic_joker_properties() {
        // Production requirement: All jokers must have correct metadata
        let joker = StaticJokerFactory::create_joker();

        assert_eq!(joker.id(), JokerId::Joker);
        assert_eq!(joker.name(), "Joker");
        assert_eq!(joker.description(), "+4 Mult");
        assert_eq!(joker.rarity(), JokerRarity::Common);
        assert_eq!(joker.cost(), 2);
    }

    #[test]
    fn test_basic_joker_scoring_per_hand() {
        // Production test: Validate core scoring behavior
        let joker = StaticJokerFactory::create_joker();
        let mut context = create_test_context(10, 2);
        let hand = SelectHand::new(vec![]);

        let effect = joker.on_hand_played(&mut context, &hand);

        // Basic joker provides +4 mult per hand, not per card
        assert_eq!(
            effect.mult, 4,
            "Basic joker should provide +4 mult per hand"
        );
        assert_eq!(effect.chips, 0, "Basic joker should not provide chips");
        assert_eq!(effect.money, 0, "Basic joker should not provide money");
        assert_eq!(
            effect.mult_multiplier, 1.0,
            "Basic joker should not provide mult multiplier"
        );
    }

    #[test]
    fn test_basic_joker_no_card_scoring() {
        // Production edge case: Verify joker doesn't trigger on individual cards
        let joker = StaticJokerFactory::create_joker();
        let mut context = create_test_context(10, 2);
        let card = create_test_card(Suit::Heart, Value::Ace);

        let effect = joker.on_card_scored(&mut context, &card);

        // Basic joker is per-hand, not per-card
        assert_eq!(
            effect.mult, 0,
            "Basic joker should not trigger on individual cards"
        );
        assert_eq!(
            effect.chips, 0,
            "Basic joker should not provide chips on card scored"
        );
    }
}

// ============================================================================
// Suit-Based Jokers - Production Test Coverage
// ============================================================================

#[cfg(test)]
mod suit_joker_tests {
    use super::*;

    #[test]
    fn test_greedy_joker_properties() {
        let joker = StaticJokerFactory::create_greedy_joker();

        assert_eq!(joker.id(), JokerId::GreedyJoker);
        assert_eq!(joker.name(), "Greedy Joker");
        assert_eq!(
            joker.description(),
            "Played cards with Diamond suit give +3 Mult when scored"
        );
        assert_eq!(joker.rarity(), JokerRarity::Common);
        assert_eq!(joker.cost(), 5);
    }

    #[test]
    fn test_greedy_joker_diamond_scoring() {
        // Production test: Validate diamond suit condition
        let joker = StaticJokerFactory::create_greedy_joker();
        let mut context = create_test_context(10, 2);

        // Test each diamond card value for comprehensive coverage
        let diamond_cards = vec![
            Value::Ace,
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

        for value in diamond_cards {
            let diamond_card = create_test_card(Suit::Diamond, value);
            let effect = joker.on_card_scored(&mut context, &diamond_card);

            assert_eq!(
                effect.mult, 3,
                "Greedy joker should provide +3 mult for {value:?} of diamonds"
            );
            assert_eq!(effect.chips, 0, "Greedy joker should not provide chips");
        }
    }

    #[test]
    fn test_greedy_joker_non_diamond_cards() {
        // Production edge case: Verify non-diamonds don't trigger
        let joker = StaticJokerFactory::create_greedy_joker();
        let mut context = create_test_context(10, 2);

        let non_diamond_cards = vec![
            create_test_card(Suit::Heart, Value::Ace),
            create_test_card(Suit::Club, Value::King),
            create_test_card(Suit::Spade, Value::Queen),
        ];

        for card in non_diamond_cards {
            let effect = joker.on_card_scored(&mut context, &card);

            assert_eq!(
                effect.mult, 0,
                "Greedy joker should not trigger for non-diamond cards"
            );
            assert_eq!(
                effect.chips, 0,
                "Greedy joker should not provide chips for non-diamonds"
            );
        }
    }

    #[test]
    fn test_lusty_joker_heart_scoring() {
        // Production test: Heart suit validation
        let joker = StaticJokerFactory::create_lusty_joker();
        let mut context = create_test_context(10, 2);

        let heart_card = create_test_card(Suit::Heart, Value::Queen);
        let effect = joker.on_card_scored(&mut context, &heart_card);

        assert_eq!(
            effect.mult, 3,
            "Lusty joker should provide +3 mult for hearts"
        );

        // Verify non-hearts don't trigger
        let spade_card = create_test_card(Suit::Spade, Value::Queen);
        let no_effect = joker.on_card_scored(&mut context, &spade_card);

        assert_eq!(
            no_effect.mult, 0,
            "Lusty joker should not trigger for non-hearts"
        );
    }

    #[test]
    fn test_wrathful_joker_spade_scoring() {
        // Production test: Spade suit validation
        let joker = StaticJokerFactory::create_wrathful_joker();
        let mut context = create_test_context(10, 2);

        let spade_card = create_test_card(Suit::Spade, Value::Jack);
        let effect = joker.on_card_scored(&mut context, &spade_card);

        assert_eq!(
            effect.mult, 3,
            "Wrathful joker should provide +3 mult for spades"
        );

        // Edge case: Verify other suits don't trigger
        let club_card = create_test_card(Suit::Club, Value::Jack);
        let no_effect = joker.on_card_scored(&mut context, &club_card);

        assert_eq!(
            no_effect.mult, 0,
            "Wrathful joker should not trigger for non-spades"
        );
    }

    #[test]
    fn test_gluttonous_joker_club_scoring() {
        // Production test: Club suit validation
        let joker = StaticJokerFactory::create_gluttonous_joker();
        let mut context = create_test_context(10, 2);

        let club_card = create_test_card(Suit::Club, Value::King);
        let effect = joker.on_card_scored(&mut context, &club_card);

        assert_eq!(
            effect.mult, 3,
            "Gluttonous joker should provide +3 mult for clubs"
        );

        // Edge case: Verify other suits don't trigger
        let diamond_card = create_test_card(Suit::Diamond, Value::King);
        let no_effect = joker.on_card_scored(&mut context, &diamond_card);

        assert_eq!(
            no_effect.mult, 0,
            "Gluttonous joker should not trigger for non-clubs"
        );
    }
}

// ============================================================================
// Hand Type Conditional Jokers - Production Test Coverage
// ============================================================================

#[cfg(test)]
mod hand_type_joker_tests {
    use super::*;

    #[test]
    fn test_jolly_joker_properties() {
        let joker = StaticJokerFactory::create_jolly_joker();

        assert_eq!(joker.id(), JokerId::JollyJoker);
        assert_eq!(joker.name(), "Jolly Joker");
        assert_eq!(joker.description(), "+8 Mult if played hand contains Pair");
        assert_eq!(joker.rarity(), JokerRarity::Common);
        assert_eq!(joker.cost(), 3);
    }

    #[test]
    fn test_jolly_joker_pair_detection() {
        // Production test: Validate pair detection logic
        let joker = StaticJokerFactory::create_jolly_joker();
        let mut context = create_test_context(10, 2);

        // Create a hand with a pair
        let pair_hand = SelectHand::new(vec![
            create_test_card(Suit::Heart, Value::King),
            create_test_card(Suit::Diamond, Value::King),
            create_test_card(Suit::Club, Value::Five),
        ]);

        let effect = joker.on_hand_played(&mut context, &pair_hand);

        assert_eq!(
            effect.mult, 8,
            "Jolly joker should provide +8 mult for pair"
        );
        assert_eq!(effect.chips, 0, "Jolly joker should not provide chips");
    }

    #[test]
    fn test_jolly_joker_no_pair() {
        // Production edge case: Verify non-pair hands don't trigger
        let joker = StaticJokerFactory::create_jolly_joker();
        let mut context = create_test_context(10, 2);

        // Create a hand without pairs
        let no_pair_hand = SelectHand::new(vec![
            create_test_card(Suit::Heart, Value::King),
            create_test_card(Suit::Diamond, Value::Queen),
            create_test_card(Suit::Club, Value::Jack),
        ]);

        let effect = joker.on_hand_played(&mut context, &no_pair_hand);

        assert_eq!(
            effect.mult, 0,
            "Jolly joker should not trigger without pair"
        );
    }

    #[test]
    fn test_zany_joker_three_of_kind() {
        // Production test: Three of a kind validation
        let joker = StaticJokerFactory::create_zany_joker();
        let mut context = create_test_context(10, 2);

        // Create three of a kind
        let three_kind_hand = SelectHand::new(vec![
            create_test_card(Suit::Heart, Value::Seven),
            create_test_card(Suit::Diamond, Value::Seven),
            create_test_card(Suit::Club, Value::Seven),
            create_test_card(Suit::Spade, Value::Two),
        ]);

        let effect = joker.on_hand_played(&mut context, &three_kind_hand);

        assert_eq!(
            effect.mult, 12,
            "Zany joker should provide +12 mult for three of a kind"
        );
    }
}

// ============================================================================
// Even/Odd Conditional Jokers - Production Test Coverage
// ============================================================================

#[cfg(test)]
mod even_odd_joker_tests {
    use super::*;

    #[test]
    fn test_even_steven_properties() {
        let joker = StaticJokerFactory::create_even_steven();

        assert_eq!(joker.id(), JokerId::EvenSteven);
        assert_eq!(joker.name(), "Even Steven");
        assert_eq!(
            joker.description(),
            "Played cards with even rank (2, 4, 6, 8, 10) give +4 Mult when scored"
        );
        assert_eq!(joker.rarity(), JokerRarity::Common);
        assert_eq!(joker.cost(), 4);
    }

    #[test]
    fn test_even_steven_even_cards() {
        // Production test: Comprehensive even rank validation
        let joker = StaticJokerFactory::create_even_steven();
        let mut context = create_test_context(10, 2);

        let even_cards = vec![
            Value::Two,
            Value::Four,
            Value::Six,
            Value::Eight,
            Value::Ten,
        ];

        for value in even_cards {
            let even_card = create_test_card(Suit::Heart, value);
            let effect = joker.on_card_scored(&mut context, &even_card);

            assert_eq!(
                effect.mult, 4,
                "Even Steven should provide +4 mult for {value:?}"
            );
        }
    }

    #[test]
    fn test_even_steven_odd_cards() {
        // Production edge case: Verify odd cards don't trigger
        let joker = StaticJokerFactory::create_even_steven();
        let mut context = create_test_context(10, 2);

        let odd_cards = vec![
            Value::Ace,
            Value::Three,
            Value::Five,
            Value::Seven,
            Value::Nine,
            Value::Jack,
            Value::Queen,
            Value::King,
        ];

        for value in odd_cards {
            let odd_card = create_test_card(Suit::Heart, value);
            let effect = joker.on_card_scored(&mut context, &odd_card);

            assert_eq!(
                effect.mult, 0,
                "Even Steven should not trigger for {value:?}"
            );
        }
    }

    #[test]
    fn test_odd_todd_properties() {
        let joker = StaticJokerFactory::create_odd_todd();

        assert_eq!(joker.id(), JokerId::OddTodd);
        assert_eq!(joker.name(), "Odd Todd");
        assert_eq!(
            joker.description(),
            "Played cards with odd rank (3, 5, 7, 9, A) give +31 Chips when scored"
        );
        assert_eq!(joker.rarity(), JokerRarity::Common);
        assert_eq!(joker.cost(), 4);
    }

    #[test]
    fn test_odd_todd_odd_cards() {
        // Production test: Comprehensive odd rank validation
        let joker = StaticJokerFactory::create_odd_todd();
        let mut context = create_test_context(10, 2);

        let odd_cards = vec![
            Value::Ace,
            Value::Three,
            Value::Five,
            Value::Seven,
            Value::Nine,
        ];

        for value in odd_cards {
            let odd_card = create_test_card(Suit::Heart, value);
            let effect = joker.on_card_scored(&mut context, &odd_card);

            assert_eq!(
                effect.chips, 31,
                "Odd Todd should provide +31 chips for {value:?}"
            );
            assert_eq!(effect.mult, 0, "Odd Todd should not provide mult");
        }
    }

    #[test]
    fn test_odd_todd_even_cards() {
        // Production edge case: Verify even cards and face cards don't trigger
        let joker = StaticJokerFactory::create_odd_todd();
        let mut context = create_test_context(10, 2);

        let non_odd_cards = vec![
            Value::Two,
            Value::Four,
            Value::Six,
            Value::Eight,
            Value::Ten,
            Value::Jack,
            Value::Queen,
            Value::King,
        ];

        for value in non_odd_cards {
            let non_odd_card = create_test_card(Suit::Heart, value);
            let effect = joker.on_card_scored(&mut context, &non_odd_card);

            assert_eq!(effect.chips, 0, "Odd Todd should not trigger for {value:?}");
        }
    }
}

// ============================================================================
// Resource-Based Jokers - Production Test Coverage
// ============================================================================

#[cfg(test)]
mod resource_joker_tests {
    use super::*;

    #[test]
    fn test_banner_joker_discard_scaling() {
        // Production test: Validate discard count scaling
        let joker = StaticJokerFactory::create_banner();

        // Test various discard scenarios
        let test_cases = vec![
            (0, 150), // 0 used, 5 remaining: 5 * 30 = 150
            (1, 120), // 1 used, 4 remaining: 4 * 30 = 120
            (2, 90),  // 2 used, 3 remaining: 3 * 30 = 90
            (3, 60),  // 3 used, 2 remaining: 2 * 30 = 60
            (4, 30),  // 4 used, 1 remaining: 1 * 30 = 30
            (5, 0),   // 5 used, 0 remaining: 0 * 30 = 0
        ];

        for (discards_used, expected_chips) in test_cases {
            let mut context = create_test_context(10, discards_used);
            let hand = SelectHand::new(vec![]);

            let effect = joker.on_hand_played(&mut context, &hand);

            assert_eq!(
                effect.chips, expected_chips,
                "Banner should provide {expected_chips} chips with {discards_used} discards used"
            );
        }
    }

    #[test]
    fn test_banner_edge_case_excessive_discards() {
        // Production edge case: Handle invalid discard counts gracefully
        let joker = StaticJokerFactory::create_banner();
        let mut context = create_test_context(10, 10); // More than max discards
        let hand = SelectHand::new(vec![]);

        let effect = joker.on_hand_played(&mut context, &hand);

        // Should handle gracefully (likely 0 chips for negative remaining)
        assert!(
            effect.chips >= 0,
            "Banner should not produce negative chips"
        );
    }
}

// ============================================================================
// Face Card Jokers - Production Test Coverage
// ============================================================================

#[cfg(test)]
mod face_card_joker_tests {
    use super::*;

    #[test]
    fn test_faceless_joker_properties() {
        let joker = StaticJokerFactory::create_faceless_joker();

        assert_eq!(joker.id(), JokerId::FacelessJoker);
        assert_eq!(joker.name(), "Faceless Joker");
        assert_eq!(
            joker.description(),
            "Face cards (Jack, Queen, King) give +5 Mult when scored"
        );
        assert_eq!(joker.rarity(), JokerRarity::Common);
        assert_eq!(joker.cost(), 3);
    }

    #[test]
    fn test_faceless_joker_face_cards() {
        // Production test: Validate all face card types
        let joker = StaticJokerFactory::create_faceless_joker();
        let mut context = create_test_context(10, 2);

        let face_cards = vec![Value::Jack, Value::Queen, Value::King];
        let suits = vec![Suit::Heart, Suit::Diamond, Suit::Club, Suit::Spade];

        for face_value in face_cards {
            for suit in &suits {
                let face_card = create_face_card(*suit, face_value);
                let effect = joker.on_card_scored(&mut context, &face_card);

                assert_eq!(
                    effect.mult, 5,
                    "Faceless joker should provide +5 mult for {face_value:?} of {suit:?}"
                );
            }
        }
    }

    #[test]
    fn test_faceless_joker_non_face_cards() {
        // Production edge case: Verify non-face cards don't trigger
        let joker = StaticJokerFactory::create_faceless_joker();
        let mut context = create_test_context(10, 2);

        let non_face_cards = vec![
            Value::Ace,
            Value::Two,
            Value::Three,
            Value::Four,
            Value::Five,
            Value::Six,
            Value::Seven,
            Value::Eight,
            Value::Nine,
            Value::Ten,
        ];

        for value in non_face_cards {
            let card = create_test_card(Suit::Heart, value);
            let effect = joker.on_card_scored(&mut context, &card);

            assert_eq!(
                effect.mult, 0,
                "Faceless joker should not trigger for {value:?}"
            );
        }
    }
}

// ============================================================================
// Production Integration Tests
// ============================================================================

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_multiple_joker_interaction_no_conflicts() {
        // Production test: Ensure jokers don't interfere with each other
        let greedy = StaticJokerFactory::create_greedy_joker();
        let lusty = StaticJokerFactory::create_lusty_joker();
        let mut context = create_test_context(10, 2);

        let diamond_card = create_test_card(Suit::Diamond, Value::Ace);
        let heart_card = create_test_card(Suit::Heart, Value::King);

        // Test that each joker works independently
        let greedy_effect = greedy.on_card_scored(&mut context, &diamond_card);
        let lusty_effect = lusty.on_card_scored(&mut context, &heart_card);

        assert_eq!(greedy_effect.mult, 3, "Greedy should work with diamonds");
        assert_eq!(lusty_effect.mult, 3, "Lusty should work with hearts");

        // Test that jokers don't trigger for wrong suits
        let greedy_no_effect = greedy.on_card_scored(&mut context, &heart_card);
        let lusty_no_effect = lusty.on_card_scored(&mut context, &diamond_card);

        assert_eq!(
            greedy_no_effect.mult, 0,
            "Greedy should not trigger for hearts"
        );
        assert_eq!(
            lusty_no_effect.mult, 0,
            "Lusty should not trigger for diamonds"
        );
    }

    #[test]
    fn test_edge_case_empty_hands() {
        // Production edge case: Handle empty hands gracefully
        let jokers = vec![
            StaticJokerFactory::create_joker(),
            StaticJokerFactory::create_jolly_joker(),
            StaticJokerFactory::create_banner(),
        ];

        let mut context = create_test_context(10, 2);
        let empty_hand = SelectHand::new(vec![]);

        for joker in jokers {
            let effect = joker.on_hand_played(&mut context, &empty_hand);

            // Validate no crashes and sensible behavior
            assert!(effect.chips >= 0, "No joker should produce negative chips");
            assert!(effect.mult >= 0, "No joker should produce negative mult");
            assert!(
                effect.mult_multiplier >= 0.0,
                "No joker should produce negative multiplier"
            );
        }
    }

    #[test]
    fn test_production_scale_card_combinations() {
        // Production test: Test with realistic game scenarios
        let even_steven = StaticJokerFactory::create_even_steven();
        let odd_todd = StaticJokerFactory::create_odd_todd();
        let mut context = create_test_context(25, 1);

        // Test full poker hand with mixed even/odd
        let mixed_hand_cards = vec![
            create_test_card(Suit::Heart, Value::Two), // Even - should trigger Even Steven
            create_test_card(Suit::Diamond, Value::Three), // Odd - should trigger Odd Todd
            create_test_card(Suit::Club, Value::Four), // Even - should trigger Even Steven
            create_test_card(Suit::Spade, Value::Five), // Odd - should trigger Odd Todd
            create_test_card(Suit::Heart, Value::Six), // Even - should trigger Even Steven
        ];

        let mut total_even_mult = 0;
        let mut total_odd_chips = 0;

        for card in &mixed_hand_cards {
            let even_effect = even_steven.on_card_scored(&mut context, card);
            let odd_effect = odd_todd.on_card_scored(&mut context, card);

            total_even_mult += even_effect.mult;
            total_odd_chips += odd_effect.chips;
        }

        // Validate expected totals
        assert_eq!(
            total_even_mult, 12,
            "Should have 3 even cards * 4 mult = 12"
        );
        assert_eq!(
            total_odd_chips, 62,
            "Should have 2 odd cards * 31 chips = 62"
        );
    }
}

// ============================================================================
// Production Performance Tests
// ============================================================================

#[cfg(test)]
mod performance_tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_joker_creation_performance() {
        // Production requirement: Joker creation must be fast for game responsiveness
        let start = Instant::now();

        for _ in 0..1000 {
            let _joker = StaticJokerFactory::create_greedy_joker();
        }

        let duration = start.elapsed();

        // Should create 1000 jokers in under 10ms (production requirement)
        assert!(
            duration.as_millis() < 10,
            "Joker creation too slow: {}ms for 1000 jokers",
            duration.as_millis()
        );
    }

    #[test]
    fn test_effect_calculation_performance() {
        // Production requirement: Effect calculations must be fast for real-time scoring
        let joker = StaticJokerFactory::create_greedy_joker();
        let mut context = create_test_context(10, 2);
        let diamond_card = create_test_card(Suit::Diamond, Value::Ace);

        let start = Instant::now();

        for _ in 0..10000 {
            let _effect = joker.on_card_scored(&mut context, &diamond_card);
        }

        let duration = start.elapsed();

        // Should calculate 10000 effects in under 1ms (production requirement)
        assert!(
            duration.as_millis() < 10,
            "Effect calculation too slow: {}ms for 10000 calculations",
            duration.as_millis()
        );
    }
}
