// Tests for additional static jokers (Issue #90)
// Note: Runner is implemented as RunnerJoker in joker_impl.rs, not as a static joker
// This file tests 9 jokers: 5 fully implemented + 4 placeholders

use balatro_rs::card::{Card, Suit, Value};
use balatro_rs::hand::SelectHand;
use balatro_rs::joker::{GameContext, Joker, JokerId, JokerRarity};
use balatro_rs::static_joker_factory::StaticJokerFactory;

#[test]
fn test_red_card_joker() {
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
fn test_blue_joker() {
    let joker = StaticJokerFactory::create_blue_joker();
    assert_eq!(joker.id(), JokerId::BlueJoker);
    assert_eq!(joker.name(), "Blue Joker");
    assert_eq!(
        joker.description(),
        "Black cards (Clubs and Spades) give +3 Mult when scored"
    );
    assert_eq!(joker.rarity(), JokerRarity::Uncommon);
    assert_eq!(joker.cost(), 6);
}

#[test]
fn test_faceless_joker() {
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
fn test_square_joker() {
    let joker = StaticJokerFactory::create_square();
    assert_eq!(joker.id(), JokerId::Square);
    assert_eq!(joker.name(), "Square");
    assert_eq!(
        joker.description(),
        "Number cards (2, 3, 4, 5, 6, 7, 8, 9, 10) give +4 Chips when scored"
    );
    assert_eq!(joker.rarity(), JokerRarity::Common);
    assert_eq!(joker.cost(), 3);
}

#[test]
fn test_walkie_joker() {
    let joker = StaticJokerFactory::create_walkie();
    assert_eq!(joker.id(), JokerId::Walkie);
    assert_eq!(joker.name(), "Walkie");
    assert_eq!(
        joker.description(),
        "+10 Chips and +4 Mult if played hand contains a Straight"
    );
    assert_eq!(joker.rarity(), JokerRarity::Common);
    assert_eq!(joker.cost(), 3);
}

// Note: Runner is implemented as RunnerJoker in joker_impl.rs, not as a static joker

// Tests for jokers that need framework extensions
#[test]
fn test_half_joker() {
    let joker = StaticJokerFactory::create_half_joker();
    assert_eq!(joker.id(), JokerId::HalfJoker);
    assert_eq!(joker.name(), "Half Joker");
    assert_eq!(
        joker.description(),
        "+20 Mult if played hand has 4 or fewer cards"
    );
    assert_eq!(joker.rarity(), JokerRarity::Common);
    assert_eq!(joker.cost(), 3);
}

#[test]
fn test_half_joker_behavior_with_4_cards() {
    let joker = StaticJokerFactory::create_half_joker();
    let mut context = GameContext::default();
    
    // Test with exactly 4 cards (should trigger)
    let four_card_hand = SelectHand::new(vec![
        Card::new(Value::King, Suit::Heart),
        Card::new(Value::Queen, Suit::Diamond),
        Card::new(Value::Jack, Suit::Club),
        Card::new(Value::Ten, Suit::Spade),
    ]);
    
    let effect = joker.on_hand_played(&mut context, &four_card_hand);
    assert_eq!(effect.mult, 20, "Half Joker should provide +20 Mult with 4 cards");
    assert_eq!(effect.chips, 0, "Half Joker should not provide chips");
    assert_eq!(effect.mult_multiplier, 1.0, "Half Joker should not provide mult multiplier");
}

#[test]
fn test_half_joker_behavior_with_3_cards() {
    let joker = StaticJokerFactory::create_half_joker();
    let mut context = GameContext::default();
    
    // Test with 3 cards (should trigger)
    let three_card_hand = SelectHand::new(vec![
        Card::new(Value::King, Suit::Heart),
        Card::new(Value::Queen, Suit::Diamond),
        Card::new(Value::Jack, Suit::Club),
    ]);
    
    let effect = joker.on_hand_played(&mut context, &three_card_hand);
    assert_eq!(effect.mult, 20, "Half Joker should provide +20 Mult with 3 cards");
}

#[test]
fn test_half_joker_behavior_with_2_cards() {
    let joker = StaticJokerFactory::create_half_joker();
    let mut context = GameContext::default();
    
    // Test with 2 cards (should trigger)
    let two_card_hand = SelectHand::new(vec![
        Card::new(Value::King, Suit::Heart),
        Card::new(Value::Queen, Suit::Diamond),
    ]);
    
    let effect = joker.on_hand_played(&mut context, &two_card_hand);
    assert_eq!(effect.mult, 20, "Half Joker should provide +20 Mult with 2 cards");
}

#[test]
fn test_half_joker_behavior_with_1_card() {
    let joker = StaticJokerFactory::create_half_joker();
    let mut context = GameContext::default();
    
    // Test with 1 card (should trigger)
    let one_card_hand = SelectHand::new(vec![
        Card::new(Value::King, Suit::Heart),
    ]);
    
    let effect = joker.on_hand_played(&mut context, &one_card_hand);
    assert_eq!(effect.mult, 20, "Half Joker should provide +20 Mult with 1 card");
}

#[test]
fn test_half_joker_behavior_with_5_cards() {
    let joker = StaticJokerFactory::create_half_joker();
    let mut context = GameContext::default();
    
    // Test with 5 cards (should NOT trigger)
    let five_card_hand = SelectHand::new(vec![
        Card::new(Value::King, Suit::Heart),
        Card::new(Value::Queen, Suit::Diamond),
        Card::new(Value::Jack, Suit::Club),
        Card::new(Value::Ten, Suit::Spade),
        Card::new(Value::Nine, Suit::Heart),
    ]);
    
    let effect = joker.on_hand_played(&mut context, &five_card_hand);
    assert_eq!(effect.mult, 0, "Half Joker should provide no mult with 5 cards");
    assert_eq!(effect.chips, 0, "Half Joker should provide no chips with 5 cards");
    assert_eq!(effect.mult_multiplier, 1.0, "Half Joker should provide no mult multiplier with 5 cards");
}

#[test]
fn test_half_joker_behavior_with_6_cards() {
    let joker = StaticJokerFactory::create_half_joker();
    let mut context = GameContext::default();
    
    // Test with 6 cards (should NOT trigger)
    let six_card_hand = SelectHand::new(vec![
        Card::new(Value::King, Suit::Heart),
        Card::new(Value::Queen, Suit::Diamond),
        Card::new(Value::Jack, Suit::Club),
        Card::new(Value::Ten, Suit::Spade),
        Card::new(Value::Nine, Suit::Heart),
        Card::new(Value::Eight, Suit::Diamond),
    ]);
    
    let effect = joker.on_hand_played(&mut context, &six_card_hand);
    assert_eq!(effect.mult, 0, "Half Joker should provide no mult with 6 cards");
}

#[test]
fn test_half_joker_behavior_per_hand_not_per_card() {
    let joker = StaticJokerFactory::create_half_joker();
    let mut context = GameContext::default();
    
    // Test that Half Joker is per-hand, not per-card
    let three_card_hand = SelectHand::new(vec![
        Card::new(Value::King, Suit::Heart),
        Card::new(Value::Queen, Suit::Diamond),
        Card::new(Value::Jack, Suit::Club),
    ]);
    
    // Test on_card_scored - should return no effect since it's per-hand
    let card = Card::new(Value::King, Suit::Heart);
    let card_effect = joker.on_card_scored(&mut context, &card);
    assert_eq!(card_effect.mult, 0, "Half Joker should not trigger on individual cards");
    
    // Test on_hand_played - should return effect since it's per-hand
    let hand_effect = joker.on_hand_played(&mut context, &three_card_hand);
    assert_eq!(hand_effect.mult, 20, "Half Joker should trigger on hands with ≤4 cards");
}

#[test]
fn test_half_joker_behavior_edge_case_empty_hand() {
    let joker = StaticJokerFactory::create_half_joker();
    let mut context = GameContext::default();
    
    // Test with empty hand (should trigger as 0 ≤ 4)
    let empty_hand = SelectHand::new(vec![]);
    
    let effect = joker.on_hand_played(&mut context, &empty_hand);
    assert_eq!(effect.mult, 20, "Half Joker should provide +20 Mult with empty hand");
}

#[test]
#[ignore] // Ignore until framework supports discard count
fn test_banner_joker() {
    let joker = StaticJokerFactory::create_banner();
    assert_eq!(joker.id(), JokerId::Banner);
    assert_eq!(joker.name(), "Banner");
    assert_eq!(joker.description(), "+30 Chips for each remaining discard");
    assert_eq!(joker.rarity(), JokerRarity::Common);
    assert_eq!(joker.cost(), 3);
}

#[test]
fn test_abstract_joker() {
    use balatro_rs::joker_factory::JokerFactory;
    use balatro_rs::joker::{GameContext, Joker, JokerEffect, JokerId, JokerRarity};
    use balatro_rs::joker_state::JokerStateManager;
    use balatro_rs::hand::{Hand, SelectHand};
    use balatro_rs::card::{Card, Suit, Value};
    use balatro_rs::stage::Stage;
    use balatro_rs::rank::HandRank;
    use balatro_rs::rng::GameRng;
    use std::collections::HashMap;
    use std::sync::Arc;

    // Test basic properties first
    let joker = JokerFactory::create(JokerId::AbstractJoker).unwrap();
    assert_eq!(joker.id(), JokerId::AbstractJoker);
    assert_eq!(joker.name(), "Abstract Joker");
    assert_eq!(joker.description(), "All Jokers give X0.25 more Mult");
    assert_eq!(joker.rarity(), JokerRarity::Common);
    assert_eq!(joker.cost(), 3);

    // Test joker interaction behavior
    // Create multiple jokers for testing
    let abstract_joker = JokerFactory::create(JokerId::AbstractJoker).unwrap();
    let greedy_joker = JokerFactory::create(JokerId::GreedyJoker).unwrap();
    let jolly_joker = JokerFactory::create(JokerId::JollyJoker).unwrap();
    
    // Create a collection of jokers
    let jokers: Vec<Box<dyn Joker>> = vec![
        abstract_joker,
        greedy_joker,
        jolly_joker,
    ];
    
    // Convert to static reference for testing (unsafe but okay for tests)
    let jokers_ref: &'static [Box<dyn Joker>] = Box::leak(jokers.into_boxed_slice());
    
    // Create a test game context manually
    let joker_state_manager = Arc::new(JokerStateManager::new());
    let stage = Stage::Blind;
    let stage_ref: &'static Stage = Box::leak(Box::new(stage));
    let hand = Hand::new();
    let hand_ref: &'static Hand = Box::leak(Box::new(hand));
    let discarded: Vec<Card> = Vec::new();
    let discarded_ref: &'static [Card] = Box::leak(discarded.into_boxed_slice());
    let hand_type_counts: HashMap<HandRank, u32> = HashMap::new();
    let hand_type_counts_ref: &'static HashMap<HandRank, u32> = Box::leak(Box::new(hand_type_counts));
    let rng = GameRng::new();
    let rng_ref: &'static GameRng = Box::leak(Box::new(rng));
    
    let mut context = GameContext {
        chips: 10,
        mult: 1,
        money: 5,
        ante: 1,
        round: 1,
        stage: stage_ref,
        hands_played: 0,
        discards_used: 0,
        jokers: jokers_ref,
        hand: hand_ref,
        discarded: discarded_ref,
        joker_state_manager: &joker_state_manager,
        hand_type_counts: hand_type_counts_ref,
        cards_in_deck: 52,
        stone_cards_in_deck: 0,
        rng: rng_ref,
    };
    
    // Create a test hand to play
    let test_hand = SelectHand::new(vec![
        Card::new(Value::King, Suit::Heart),
        Card::new(Value::King, Suit::Diamond),
    ]);
    
    // Test Abstract Joker with 0 other jokers (should give 0 mult)
    // First, test with only Abstract Joker
    let single_joker: Vec<Box<dyn Joker>> = vec![
        JokerFactory::create(JokerId::AbstractJoker).unwrap(),
    ];
    let single_joker_ref: &'static [Box<dyn Joker>] = Box::leak(single_joker.into_boxed_slice());
    context.jokers = single_joker_ref;
    
    let abstract_joker_instance = &context.jokers[0];
    let effect = abstract_joker_instance.on_hand_played(&mut context, &test_hand);
    assert_eq!(effect.mult, 0, "Abstract Joker should provide 0 mult when no other jokers present");
    
    // Reset context with 3 jokers (Abstract + 2 others)
    context.jokers = jokers_ref;
    
    // Test Abstract Joker with 2 other jokers (should give 6 mult = 2 * 3)
    let abstract_joker_instance = &context.jokers[0];
    let effect = abstract_joker_instance.on_hand_played(&mut context, &test_hand);
    assert_eq!(effect.mult, 6, "Abstract Joker should provide 6 mult with 2 other jokers (2 * 3)");
    
    // Verify the effect calculation excludes itself
    assert_eq!(context.jokers.len(), 3, "Should have 3 total jokers");
    
    // Test that other jokers don't count themselves
    let greedy_joker_instance = &context.jokers[1];
    let greedy_effect = greedy_joker_instance.on_hand_played(&mut context, &test_hand);
    // Greedy joker is a per-card joker, so it should not provide mult on hand played
    assert_eq!(greedy_effect.mult, 0, "Greedy Joker should not provide mult on hand played");
    
    // Test with a different number of jokers
    let more_jokers: Vec<Box<dyn Joker>> = vec![
        JokerFactory::create(JokerId::AbstractJoker).unwrap(),
        JokerFactory::create(JokerId::GreedyJoker).unwrap(),
        JokerFactory::create(JokerId::LustyJoker).unwrap(),
        JokerFactory::create(JokerId::WrathfulJoker).unwrap(),
        JokerFactory::create(JokerId::GluttonousJoker).unwrap(),
    ];
    let more_jokers_ref: &'static [Box<dyn Joker>] = Box::leak(more_jokers.into_boxed_slice());
    context.jokers = more_jokers_ref;
    
    // Test Abstract Joker with 4 other jokers (should give 12 mult = 4 * 3)
    let abstract_joker_instance = &context.jokers[0];
    let effect = abstract_joker_instance.on_hand_played(&mut context, &test_hand);
    assert_eq!(effect.mult, 12, "Abstract Joker should provide 12 mult with 4 other jokers (4 * 3)");
}

#[test]
#[ignore] // Ignore until framework supports deck composition
fn test_steel_joker() {
    let joker = StaticJokerFactory::create_steel_joker();
    assert_eq!(joker.id(), JokerId::SteelJoker);
    assert_eq!(joker.name(), "Steel Joker");
    assert_eq!(
        joker.description(),
        "This Joker gains X0.25 Mult for each Steel Card in your full deck"
    );
    assert_eq!(joker.rarity(), JokerRarity::Uncommon);
    assert_eq!(joker.cost(), 6);
}
