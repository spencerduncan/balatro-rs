use crate::action::Action;
use crate::error::GameError;
use crate::joker::{JokerId, JokerRarity as Rarity, Jokers, OldJoker as Joker};
// use rand::distributions::WeightedIndex;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct Shop {
    pub jokers: Vec<Jokers>,
    joker_gen: JokerGenerator,
}

impl Default for Shop {
    fn default() -> Self {
        Self::new()
    }
}

impl Shop {
    pub fn new() -> Self {
        Shop {
            joker_gen: JokerGenerator {},
            jokers: Vec::new(),
        }
    }

    pub(crate) fn refresh(&mut self, rng: &crate::rng::GameRng) {
        let j1 = self.joker_gen.gen_joker(rng);
        let j2 = self.joker_gen.gen_joker(rng);
        self.jokers = vec![j1, j2]
    }

    pub(crate) fn joker_from_index(&self, i: usize) -> Option<Jokers> {
        Some(self.jokers[i].clone())
    }

    #[allow(dead_code)] // Kept for backward compatibility
    pub(crate) fn buy_joker(&mut self, joker: &Jokers) -> Result<Jokers, GameError> {
        let i = self
            .jokers
            .iter()
            .position(|j| j == joker)
            .ok_or(GameError::NoJokerMatch)?;
        let out = self.jokers.remove(i);
        Ok(out)
    }

    pub(crate) fn has_joker(&self, joker_id: JokerId) -> bool {
        // FIXME: Temporary implementation using Jokers enum matching
        // This should be replaced when shop is refactored to store JokerId directly
        // instead of the full Jokers enum. See issue tracking shop JokerId migration.
        self.jokers.iter().any(|j| j.matches_joker_id(joker_id))
    }

    pub(crate) fn gen_moves_buy_joker(
        &self,
        balance: f64,
        current_joker_count: usize,
        max_slots: usize,
    ) -> Option<impl Iterator<Item = Action> + '_> {
        if self.jokers.is_empty() {
            return None;
        }

        // Check if we can add more jokers
        if current_joker_count >= max_slots {
            return None;
        }

        // Use iterator chain without intermediate Vec allocation for better performance
        let has_affordable_jokers = self.jokers.iter().any(|j| j.cost() as f64 <= balance);

        if !has_affordable_jokers {
            return None;
        }

        Some(
            self.jokers
                .iter()
                .filter(move |j| j.cost() as f64 <= balance)
                .flat_map(move |joker| {
                    // Map old Joker enum to new JokerId
                    let joker_id = joker.to_joker_id();

                    // Generate an action for each available slot (0 to current_joker_count inclusive)
                    (0..=current_joker_count).map(move |slot| Action::BuyJoker { joker_id, slot })
                }),
        )
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct JokerGenerator {}

impl JokerGenerator {
    // Randomly generate rarity of new joker.
    // 70% chance Common, 25% chance Uncommon, 5% chance Rare.
    // Legendary can only appear from Soul Spectral Card.
    fn gen_rarity(&self) -> Rarity {
        // For now, we only have common jokers...
        Rarity::Common
        // let choices = [Rarity::Common, Rarity::Uncommon, Rarity::Rare];
        // let weights = [70, 25, 5];
        // let dist = WeightedIndex::new(&weights).unwrap();
        // let mut rng = thread_rng();
        // return choices[dist.sample(&mut rng)].clone();
    }

    // Generate a random new joker
    pub(crate) fn gen_joker(&self, rng: &crate::rng::GameRng) -> Jokers {
        let rarity = self.gen_rarity();
        let choices = Jokers::by_rarity(rarity);
        let i = rng.gen_range(0..choices.len());
        // TODO: don't regenerate already generated jokers.
        // track with hashmap.
        choices[i].clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::joker::compat::{GreedyJoker, LustyJoker, TheJoker};

    #[test]
    fn test_shop_refresh() {
        let mut shop = Shop::new();
        let rng = crate::rng::GameRng::for_testing(42);
        assert_eq!(shop.jokers.len(), 0);
        shop.refresh(&rng);
        assert_eq!(shop.jokers.len(), 2);
    }

    #[test]
    fn test_shop_buy_joker() {
        let mut shop = Shop::new();
        let rng = crate::rng::GameRng::for_testing(42);
        shop.refresh(&rng);
        assert_eq!(shop.jokers.len(), 2);
        let j1 = shop.jokers[0].clone();
        assert_eq!(shop.joker_from_index(0).expect("first joker"), j1.clone());
        shop.buy_joker(&j1).expect("buy joker");
    }

    #[test]
    fn test_shop_new() {
        let shop = Shop::new();
        assert_eq!(shop.jokers.len(), 0);
        assert!(shop.jokers.is_empty());
    }

    #[test]
    fn test_shop_default() {
        let shop = Shop::default();
        assert_eq!(shop.jokers.len(), 0);

        // Default should be identical to new()
        let new_shop = Shop::new();
        assert_eq!(shop.jokers.len(), new_shop.jokers.len());
    }

    #[test]
    fn test_shop_refresh_multiple_times() {
        let mut shop = Shop::new();
        let rng = crate::rng::GameRng::for_testing(42);

        // First refresh
        shop.refresh(&rng);
        assert_eq!(shop.jokers.len(), 2);
        let first_jokers = shop.jokers.clone();

        // Second refresh - should replace jokers
        shop.refresh(&rng);
        assert_eq!(shop.jokers.len(), 2);

        // Jokers might be the same due to RNG, but structure should be consistent
        assert_eq!(shop.jokers.len(), first_jokers.len());
    }

    #[test]
    fn test_shop_joker_from_index_valid() {
        let mut shop = Shop::new();
        let rng = crate::rng::GameRng::for_testing(42);
        shop.refresh(&rng);

        // Test valid indices
        let joker0 = shop.joker_from_index(0);
        let joker1 = shop.joker_from_index(1);

        assert!(joker0.is_some());
        assert!(joker1.is_some());

        // Should match the actual jokers in the shop
        assert_eq!(joker0.unwrap(), shop.jokers[0]);
        assert_eq!(joker1.unwrap(), shop.jokers[1]);
    }

    #[test]
    #[should_panic]
    fn test_shop_joker_from_index_invalid() {
        let mut shop = Shop::new();
        let rng = crate::rng::GameRng::for_testing(42);
        shop.refresh(&rng);

        // Test invalid index - should panic due to array access
        shop.joker_from_index(2);
    }

    #[test]
    #[should_panic]
    fn test_shop_joker_from_index_empty_shop() {
        let shop = Shop::new();

        // Test index on empty shop - should panic
        shop.joker_from_index(0);
    }

    #[test]
    fn test_shop_buy_joker_success() {
        let mut shop = Shop::new();
        let rng = crate::rng::GameRng::for_testing(42);
        shop.refresh(&rng);
        let initial_count = shop.jokers.len();
        let joker_to_buy = shop.jokers[0].clone();

        let result = shop.buy_joker(&joker_to_buy);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), joker_to_buy);
        assert_eq!(shop.jokers.len(), initial_count - 1);

        // Joker should be removed from shop
        assert!(!shop.jokers.contains(&joker_to_buy));
    }

    #[test]
    fn test_shop_buy_joker_not_found() {
        let mut shop = Shop::new();
        let rng = crate::rng::GameRng::for_testing(42);
        shop.refresh(&rng);
        let original_jokers = shop.jokers.clone();

        // Create a joker that's guaranteed not to match any joker instance in the shop
        // The key is that joker comparison is done by equality, not just type
        // So we can create a joker of the same type but different instance
        let mut attempts = 0;
        let mut non_existent_joker = None;

        // Try different joker types until we find one not in the shop
        let test_jokers = vec![
            Jokers::TheJoker(TheJoker::default()),
            Jokers::GreedyJoker(GreedyJoker::default()),
            Jokers::LustyJoker(LustyJoker::default()),
        ];

        for test_joker in test_jokers {
            if !shop.jokers.contains(&test_joker) {
                non_existent_joker = Some(test_joker);
                break;
            }
            attempts += 1;
        }

        // If somehow all test jokers are in the shop, manually clear and add specific jokers
        if non_existent_joker.is_none() {
            shop.jokers.clear();
            shop.jokers.push(Jokers::TheJoker(TheJoker::default()));
            shop.jokers
                .push(Jokers::GreedyJoker(GreedyJoker::default()));
            non_existent_joker = Some(Jokers::LustyJoker(LustyJoker::default()));
        }

        let joker_to_test = non_existent_joker.unwrap();
        let result = shop.buy_joker(&joker_to_test);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), GameError::NoJokerMatch));

        // Shop should be unchanged
        assert_eq!(shop.jokers.len(), 2);
    }

    #[test]
    fn test_shop_buy_multiple_jokers() {
        let mut shop = Shop::new();
        let rng = crate::rng::GameRng::for_testing(42);
        shop.refresh(&rng);

        // Buy first joker
        let joker1 = shop.jokers[0].clone();
        let result1 = shop.buy_joker(&joker1);
        assert!(result1.is_ok());
        assert_eq!(shop.jokers.len(), 1);

        // Buy second joker
        let joker2 = shop.jokers[0].clone(); // Now at index 0 after first removal
        let result2 = shop.buy_joker(&joker2);
        assert!(result2.is_ok());
        assert_eq!(shop.jokers.len(), 0);

        // Shop should be empty
        assert!(shop.jokers.is_empty());
    }

    #[test]
    fn test_shop_buy_same_joker_twice() {
        let mut shop = Shop::new();
        let rng = crate::rng::GameRng::for_testing(42);
        shop.refresh(&rng);
        let joker = shop.jokers[0].clone();

        // First purchase should succeed
        let result1 = shop.buy_joker(&joker);
        assert!(result1.is_ok());

        // Second purchase of same joker should fail
        let result2 = shop.buy_joker(&joker);
        assert!(result2.is_err());
        assert!(matches!(result2.unwrap_err(), GameError::NoJokerMatch));
    }

    #[test]
    fn test_shop_has_joker() {
        let mut shop = Shop::new();
        let rng = crate::rng::GameRng::for_testing(42);
        shop.refresh(&rng);

        // The has_joker method should find jokers that match the JokerId
        // This is a basic test since we can't predict which jokers will be generated
        for joker in &shop.jokers {
            let joker_id = joker.to_joker_id();
            assert!(shop.has_joker(joker_id));
        }
    }

    #[test]
    fn test_shop_has_joker_not_present() {
        let shop = Shop::new(); // Empty shop

        // Empty shop should not have any jokers
        assert!(!shop.has_joker(JokerId::Joker));
        assert!(!shop.has_joker(JokerId::GreedyJoker));
        assert!(!shop.has_joker(JokerId::LustyJoker));
    }

    #[test]
    fn test_shop_gen_moves_buy_joker_empty_shop() {
        let shop = Shop::new();

        let result = shop.gen_moves_buy_joker(100.0, 0, 5);
        assert!(result.is_none(), "Empty shop should not generate buy moves");
    }

    #[test]
    fn test_shop_gen_moves_buy_joker_insufficient_funds() {
        let mut shop = Shop::new();
        let rng = crate::rng::GameRng::for_testing(42);
        shop.refresh(&rng);

        // Test with very low balance
        let result = shop.gen_moves_buy_joker(0.0, 0, 5);
        assert!(
            result.is_none(),
            "Should not generate moves with insufficient funds"
        );
    }

    #[test]
    fn test_shop_gen_moves_buy_joker_max_slots_reached() {
        let mut shop = Shop::new();
        let rng = crate::rng::GameRng::for_testing(42);
        shop.refresh(&rng);

        // Test when current joker count equals max slots
        let result = shop.gen_moves_buy_joker(100.0, 5, 5);
        assert!(
            result.is_none(),
            "Should not generate moves when max slots reached"
        );
    }

    #[test]
    fn test_shop_gen_moves_buy_joker_valid() {
        let mut shop = Shop::new();
        let rng = crate::rng::GameRng::for_testing(42);
        shop.refresh(&rng);

        let result = shop.gen_moves_buy_joker(100.0, 0, 5);
        assert!(
            result.is_some(),
            "Should generate moves with sufficient funds and slots"
        );

        let moves: Vec<Action> = result.unwrap().collect();
        assert!(!moves.is_empty(), "Should have generated some buy actions");

        // Each affordable joker should generate moves for each available slot position
        let affordable_jokers = shop
            .jokers
            .iter()
            .filter(|j| j.cost() as f64 <= 100.0)
            .count();

        // Each joker can be placed in slots 0 to current_joker_count (inclusive)
        let expected_moves = affordable_jokers; // current_joker_count=0, so only slot 0
        assert_eq!(moves.len(), expected_moves);
    }

    #[test]
    fn test_shop_gen_moves_buy_joker_partial_affordability() {
        let mut shop = Shop::new();
        let rng = crate::rng::GameRng::for_testing(42);
        shop.refresh(&rng);

        // Assume jokers cost at least 1, test with limited funds
        let result = shop.gen_moves_buy_joker(3.0, 0, 5);

        if result.is_some() {
            let moves: Vec<Action> = result.unwrap().collect();
            // Should only include affordable jokers
            for action in moves {
                if let Action::BuyJoker { joker_id, slot: _ } = action {
                    // Find the joker with this ID and verify it's affordable
                    let affordable_joker = shop
                        .jokers
                        .iter()
                        .find(|j| j.to_joker_id() == joker_id)
                        .expect("Generated joker should exist in shop");
                    assert!(affordable_joker.cost() as f64 <= 3.0);
                }
            }
        }
    }

    #[test]
    fn test_shop_clone() {
        let mut shop = Shop::new();
        let rng = crate::rng::GameRng::for_testing(42);
        shop.refresh(&rng);

        let cloned_shop = shop.clone();

        // Cloned shop should have same jokers
        assert_eq!(cloned_shop.jokers.len(), shop.jokers.len());
        assert_eq!(cloned_shop.jokers, shop.jokers);

        // Modifying original should not affect clone
        shop.refresh(&rng);
        assert_eq!(cloned_shop.jokers.len(), 2); // Clone should still have original jokers
    }

    #[test]
    fn test_shop_debug() {
        let shop = Shop::new();
        let debug_str = format!("{shop:?}");

        // Debug output should contain shop information
        assert!(debug_str.contains("Shop"));
        assert!(debug_str.contains("jokers"));
    }

    #[test]
    fn test_joker_generator_gen_rarity() {
        let generator = JokerGenerator {};

        // Current implementation always returns Common
        let rarity = generator.gen_rarity();
        assert_eq!(rarity, Rarity::Common);
    }

    #[test]
    fn test_joker_generator_gen_joker() {
        let generator = JokerGenerator {};
        let rng = crate::rng::GameRng::for_testing(42);

        // Generate multiple jokers to test consistency
        for _ in 0..10 {
            let joker = generator.gen_joker(&rng);
            // All generated jokers should be valid Jokers enum variants
            assert!(matches!(
                joker,
                Jokers::TheJoker(_)
                    | Jokers::GreedyJoker(_)
                    | Jokers::LustyJoker(_)
                    | Jokers::WrathfulJoker(_)
                    | Jokers::GluttonousJoker(_)
                    | Jokers::JollyJoker(_)
                    | Jokers::ZanyJoker(_)
                    | Jokers::MadJoker(_)
                    | Jokers::CrazyJoker(_)
                    | Jokers::DrollJoker(_)
                    | Jokers::SlyJoker(_)
                    | Jokers::WilyJoker(_)
                    | Jokers::CleverJoker(_)
                    | Jokers::DeviousJoker(_)
                    | Jokers::CraftyJoker(_)
                    | Jokers::IceCreamJoker(_)
            ));
        }
    }

    #[test]
    fn test_shop_stress_operations() {
        let mut shop = Shop::new();
        let rng = crate::rng::GameRng::for_testing(42);

        // Perform many refresh operations
        for _ in 0..100 {
            shop.refresh(&rng);
            assert_eq!(shop.jokers.len(), 2);

            // Buy all jokers
            while !shop.jokers.is_empty() {
                let joker = shop.jokers[0].clone();
                let result = shop.buy_joker(&joker);
                assert!(result.is_ok());
            }

            assert!(shop.jokers.is_empty());
        }
    }

    #[test]
    fn test_shop_edge_case_index_boundaries() {
        let mut shop = Shop::new();
        let rng = crate::rng::GameRng::for_testing(42);
        shop.refresh(&rng);

        // Test boundary indices
        assert!(shop.joker_from_index(0).is_some());
        assert!(shop.joker_from_index(1).is_some());

        // Test just beyond boundaries should panic (handled by should_panic tests above)
        // This test verifies the valid boundary cases work correctly
    }
}
