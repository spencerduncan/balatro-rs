#[cfg(test)]
mod optimization_tests {
    use balatro_rs::card::{Card, Suit, Value};
    use balatro_rs::hand::SelectHand;
    use balatro_rs::joker::Joker;
    use balatro_rs::joker_effect_processor::JokerEffectProcessor;
    use balatro_rs::joker_impl::{GreedyJoker, LustyJoker, TheJoker};
    use balatro_rs::stage::Stage;
    use std::time::Instant;

    #[test]
    fn test_optimization_performance() {
        let mut processor = JokerEffectProcessor::new();

        // Create test jokers
        let jokers: Vec<Box<dyn Joker>> = vec![
            Box::new(TheJoker),
            Box::new(GreedyJoker),
            Box::new(LustyJoker),
        ];

        // Create test game context
        let test_stage = Stage::Blind(balatro_rs::stage::Blind::Small);
        let test_hand = balatro_rs::hand::Hand::new(vec![]);
        let test_joker_state_manager =
            std::sync::Arc::new(balatro_rs::joker_state::JokerStateManager::new());
        let test_hand_type_counts = std::collections::HashMap::new();
        let test_rng = balatro_rs::rng::GameRng::for_testing(12345);

        let mut game_context = balatro_rs::joker::GameContext {
            chips: 100,
            mult: 4,
            money: 100,
            ante: 1,
            round: 1,
            stage: &test_stage,
            hands_played: 0,
            discards_used: 0,
            jokers: &[],
            hand: &test_hand,
            discarded: &[],
            joker_state_manager: &test_joker_state_manager,
            hand_type_counts: &test_hand_type_counts,
            cards_in_deck: 52,
            stone_cards_in_deck: 0,
            rng: &test_rng,
        };

        let hand = SelectHand::new(vec![
            Card::new(Value::Ace, Suit::Heart),
            Card::new(Value::King, Suit::Heart),
            Card::new(Value::Queen, Suit::Heart),
            Card::new(Value::Jack, Suit::Heart),
            Card::new(Value::Ten, Suit::Heart),
        ]);

        // Warm up and test
        let iterations = 1000;

        // Test legacy path
        let start = Instant::now();
        for _ in 0..iterations {
            processor.process_hand_effects(&jokers, &mut game_context, &hand);
        }
        let legacy_time = start.elapsed();

        // Clear caches for fair comparison
        processor.clear_all_caches();

        // Test optimized path
        let start = Instant::now();
        for _ in 0..iterations {
            processor.process_hand_effects_optimized(
                &jokers,
                &mut game_context,
                &hand,
                &test_stage,
            );
        }
        let optimized_time = start.elapsed();

        // Get metrics
        let metrics = processor.trait_optimization_metrics();

        println!("\n=== Optimization Performance Test Results ===");
        println!("Legacy time: {:?}", legacy_time);
        println!("Optimized time: {:?}", optimized_time);
        println!(
            "Speedup: {:.2}x",
            legacy_time.as_nanos() as f64 / optimized_time.as_nanos() as f64
        );
        println!("\nOptimization metrics:");
        println!(
            "- Gameplay optimized count: {}",
            metrics.gameplay_optimized_count
        );
        println!(
            "- Optimization ratio: {:.2}%",
            metrics.optimization_ratio() * 100.0
        );

        // NOTE: Optimization is temporarily disabled due to JokerGameplay trait requiring &mut self
        // All jokers currently use the legacy path until the optimization layer is refactored
        // This is expected behavior for PR#580
        assert!(
            metrics.gameplay_optimized_count == 0,
            "Optimization should be disabled (all jokers use legacy path)"
        );
        assert!(
            metrics.optimization_ratio() == 0.0,
            "Should have optimization ratio of 0 (optimization disabled)"
        );

        // TODO: Re-enable these assertions once optimization layer supports &mut self
        // assert!(metrics.gameplay_optimized_count > 0, "Optimization paths should be used");
        // assert!(metrics.optimization_ratio() > 0.0, "Should have optimization ratio > 0");
    }
}
