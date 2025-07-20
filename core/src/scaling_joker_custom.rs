use crate::scaling_joker::{ScalingJoker, ScalingEvent, ScalingEffectType};
use crate::joker::{Joker, JokerId, JokerRarity, JokerEffect, GameContext};
use crate::joker_state::JokerState;
use crate::hand::SelectHand;
use crate::card::Card;
use crate::rank::HandRank;

/// Custom implementations for scaling jokers that require special logic
/// beyond the basic ScalingJoker framework

/// Green Joker: +1 mult per hand, -1 per discard
#[derive(Debug, Clone)]
pub struct GreenJoker {
    base: ScalingJoker,
}

impl GreenJoker {
    pub fn new() -> Self {
        Self {
            base: crate::scaling_joker_impl::create_green_joker(),
        }
    }
}

impl Joker for GreenJoker {
    fn id(&self) -> JokerId {
        self.base.id()
    }

    fn name(&self) -> &str {
        self.base.name()
    }

    fn description(&self) -> &str {
        self.base.description()
    }

    fn rarity(&self) -> JokerRarity {
        self.base.rarity()
    }

    fn on_hand_played(&self, context: &mut GameContext, hand: &SelectHand) -> JokerEffect {
        // Increment value for any hand played
        context.joker_state_manager.update_state(self.id(), |state| {
            state.accumulated_value += 1.0;
        });

        // Return current effect
        let current_value = context
            .joker_state_manager
            .get_accumulated_value(self.id())
            .unwrap_or(0.0);
        JokerEffect::new().with_mult(current_value as i32)
    }

    fn on_discard(&self, context: &mut GameContext, cards: &[Card]) -> JokerEffect {
        // Decrease value for each discard action (not per card)
        context.joker_state_manager.update_state(self.id(), |state| {
            state.accumulated_value = (state.accumulated_value - 1.0).max(0.0);
        });

        JokerEffect::new()
    }

    fn initialize_state(&self, _context: &GameContext) -> JokerState {
        JokerState::with_accumulated_value(0.0)
    }
}

/// Square Joker: +4 chips per hand played with exactly 4 cards
#[derive(Debug, Clone)]
pub struct SquareJoker {
    base: ScalingJoker,
}

impl SquareJoker {
    pub fn new() -> Self {
        Self {
            base: crate::scaling_joker_impl::create_square_joker(),
        }
    }
}

impl Joker for SquareJoker {
    fn id(&self) -> JokerId {
        self.base.id()
    }

    fn name(&self) -> &str {
        self.base.name()
    }

    fn description(&self) -> &str {
        self.base.description()
    }

    fn rarity(&self) -> JokerRarity {
        self.base.rarity()
    }

    fn on_hand_played(&self, context: &mut GameContext, hand: &SelectHand) -> JokerEffect {
        // Check if hand has exactly 4 cards
        if hand.len() == 4 {
            context.joker_state_manager.update_state(self.id(), |state| {
                state.accumulated_value += 4.0;
            });
        }

        // Return current effect
        let current_value = context
            .joker_state_manager
            .get_accumulated_value(self.id())
            .unwrap_or(0.0);
        JokerEffect::new().with_chips(current_value as i32)
    }

    fn initialize_state(&self, _context: &GameContext) -> JokerState {
        JokerState::with_accumulated_value(0.0)
    }
}

/// Bull Joker: +2 chips per $1 owned
#[derive(Debug, Clone)]
pub struct BullJoker {
    base: ScalingJoker,
}

impl BullJoker {
    pub fn new() -> Self {
        Self {
            base: crate::scaling_joker_impl::create_bull_joker(),
        }
    }
}

impl Joker for BullJoker {
    fn id(&self) -> JokerId {
        self.base.id()
    }

    fn name(&self) -> &str {
        self.base.name()
    }

    fn description(&self) -> &str {
        self.base.description()
    }

    fn rarity(&self) -> JokerRarity {
        self.base.rarity()
    }

    fn on_hand_played(&self, context: &mut GameContext, _hand: &SelectHand) -> JokerEffect {
        // Effect is based on current money, not accumulated
        let chips_bonus = context.money * 2;
        JokerEffect::new().with_chips(chips_bonus)
    }

    fn initialize_state(&self, _context: &GameContext) -> JokerState {
        JokerState::new()
    }
}

/// Bootstraps: +2 mult per $5 in bank
#[derive(Debug, Clone)]
pub struct BootstrapsJoker {
    base: ScalingJoker,
}

impl BootstrapsJoker {
    pub fn new() -> Self {
        Self {
            base: crate::scaling_joker_impl::create_bootstraps(),
        }
    }
}

impl Joker for BootstrapsJoker {
    fn id(&self) -> JokerId {
        self.base.id()
    }

    fn name(&self) -> &str {
        self.base.name()
    }

    fn description(&self) -> &str {
        self.base.description()
    }

    fn rarity(&self) -> JokerRarity {
        self.base.rarity()
    }

    fn on_hand_played(&self, context: &mut GameContext, _hand: &SelectHand) -> JokerEffect {
        // Effect is based on current money divided by 5
        let mult_bonus = (context.money / 5) * 2;
        JokerEffect::new().with_mult(mult_bonus)
    }

    fn initialize_state(&self, _context: &GameContext) -> JokerState {
        JokerState::new()
    }
}

/// Banner: +30 chips per discard remaining
#[derive(Debug, Clone)]
pub struct BannerJoker {
    base: ScalingJoker,
}

impl BannerJoker {
    pub fn new() -> Self {
        Self {
            base: crate::scaling_joker_impl::create_banner(),
        }
    }
}

impl Joker for BannerJoker {
    fn id(&self) -> JokerId {
        self.base.id()
    }

    fn name(&self) -> &str {
        self.base.name()
    }

    fn description(&self) -> &str {
        self.base.description()
    }

    fn rarity(&self) -> JokerRarity {
        self.base.rarity()
    }

    fn on_hand_played(&self, context: &mut GameContext, _hand: &SelectHand) -> JokerEffect {
        // Calculate remaining discards (assuming 3 base discards per round)
        let base_discards = 3; // This should come from game config
        let remaining_discards = base_discards.saturating_sub(context.discards_used as usize);
        let chips_bonus = remaining_discards * 30;
        
        JokerEffect::new().with_chips(chips_bonus as i32)
    }

    fn initialize_state(&self, _context: &GameContext) -> JokerState {
        JokerState::new()
    }
}

/// Ceremonial Dagger: Mult doubles when Blind starts, resets when completed
#[derive(Debug, Clone)]
pub struct CeremonialDagger {
    base: ScalingJoker,
}

impl CeremonialDagger {
    pub fn new() -> Self {
        Self {
            base: crate::scaling_joker_impl::create_ceremonial_dagger(),
        }
    }
}

impl Joker for CeremonialDagger {
    fn id(&self) -> JokerId {
        self.base.id()
    }

    fn name(&self) -> &str {
        self.base.name()
    }

    fn description(&self) -> &str {
        self.base.description()
    }

    fn rarity(&self) -> JokerRarity {
        self.base.rarity()
    }

    fn on_blind_start(&self, context: &mut GameContext) -> JokerEffect {
        // Double the mult multiplier when blind starts
        context.joker_state_manager.update_state(self.id(), |state| {
            state.accumulated_value *= 2.0;
            if state.accumulated_value == 0.0 {
                state.accumulated_value = 2.0; // Start at 2x if was 0
            }
        });

        JokerEffect::new()
    }

    fn on_hand_played(&self, context: &mut GameContext, _hand: &SelectHand) -> JokerEffect {
        // Return current multiplier effect
        let current_multiplier = context
            .joker_state_manager
            .get_accumulated_value(self.id())
            .unwrap_or(1.0);
        JokerEffect::new().with_mult_multiplier(current_multiplier)
    }

    fn on_round_end(&self, context: &mut GameContext) -> JokerEffect {
        // Reset to base value at round end
        context.joker_state_manager.update_state(self.id(), |state| {
            state.accumulated_value = 1.0;
        });

        JokerEffect::new()
    }

    fn initialize_state(&self, _context: &GameContext) -> JokerState {
        JokerState::with_accumulated_value(1.0)
    }
}

/// Mystic Summit: +15 mult per unique hand type played this run
#[derive(Debug, Clone)]
pub struct MysticSummit {
    base: ScalingJoker,
}

impl MysticSummit {
    pub fn new() -> Self {
        Self {
            base: crate::scaling_joker_impl::create_mystic_summit(),
        }
    }
}

impl Joker for MysticSummit {
    fn id(&self) -> JokerId {
        self.base.id()
    }

    fn name(&self) -> &str {
        self.base.name()
    }

    fn description(&self) -> &str {
        self.base.description()
    }

    fn rarity(&self) -> JokerRarity {
        self.base.rarity()
    }

    fn on_hand_played(&self, context: &mut GameContext, hand: &SelectHand) -> JokerEffect {
        // Get hand rank from SelectHand
        let hand_rank = match hand.best_hand() {
            Ok(made_hand) => made_hand.rank,
            Err(_) => HandRank::HighCard, // Fallback to high card if evaluation fails
        };
        
        // Check if this hand type was played before
        let played_count = context.get_hand_type_count(hand_rank);
        
        // If this is the first time playing this hand type, increment
        if played_count == 1 { // Count includes current hand, so 1 means first time
            context.joker_state_manager.update_state(self.id(), |state| {
                state.accumulated_value += 15.0;
            });
        }

        // Return current effect
        let current_value = context
            .joker_state_manager
            .get_accumulated_value(self.id())
            .unwrap_or(0.0);
        JokerEffect::new().with_mult(current_value as i32)
    }

    fn initialize_state(&self, _context: &GameContext) -> JokerState {
        JokerState::with_accumulated_value(0.0)
    }
}

/// Factory function to create all custom scaling jokers
pub fn create_all_custom_scaling_jokers() -> Vec<Box<dyn Joker>> {
    vec![
        Box::new(GreenJoker::new()),
        Box::new(SquareJoker::new()),
        Box::new(BullJoker::new()),
        Box::new(BootstrapsJoker::new()),
        Box::new(BannerJoker::new()),
        Box::new(CeremonialDagger::new()),
        Box::new(MysticSummit::new()),
    ]
}

/// Get custom scaling joker by ID
pub fn get_custom_scaling_joker_by_id(id: JokerId) -> Option<Box<dyn Joker>> {
    match id {
        JokerId::GreenJoker => Some(Box::new(GreenJoker::new())),
        JokerId::Square => Some(Box::new(SquareJoker::new())),
        JokerId::BullMarket => Some(Box::new(BullJoker::new())),
        JokerId::Bootstraps => Some(Box::new(BootstrapsJoker::new())),
        JokerId::Banner => Some(Box::new(BannerJoker::new())),
        JokerId::Ceremonial => Some(Box::new(CeremonialDagger::new())),
        JokerId::Reserved2 => Some(Box::new(MysticSummit::new())),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::joker_state::JokerStateManager;
    use std::sync::Arc;
    use std::collections::HashMap;

    fn create_test_context() -> GameContext<'static> {
        // Placeholder - in real tests we'd need proper context setup
        panic!("Test context creation not implemented - requires full game setup")
    }

    #[test]
    fn test_green_joker_creation() {
        let joker = GreenJoker::new();
        assert_eq!(joker.id(), JokerId::GreenJoker);
        assert_eq!(joker.name(), "Green Joker");
    }

    #[test] 
    fn test_all_custom_jokers_created() {
        let jokers = create_all_custom_scaling_jokers();
        assert_eq!(jokers.len(), 7, "Should create exactly 7 custom scaling jokers");
        
        // Test that all jokers have unique IDs
        let mut ids = std::collections::HashSet::new();
        for joker in &jokers {
            assert!(ids.insert(joker.id()), "Duplicate joker ID found: {:?}", joker.id());
        }
    }

    #[test]
    fn test_ceremonial_dagger_creation() {
        let joker = CeremonialDagger::new();
        assert_eq!(joker.id(), JokerId::Ceremonial);
        assert_eq!(joker.rarity(), JokerRarity::Uncommon);
    }

    #[test]
    fn test_get_custom_scaling_joker_by_id() {
        assert!(get_custom_scaling_joker_by_id(JokerId::GreenJoker).is_some());
        assert!(get_custom_scaling_joker_by_id(JokerId::Square).is_some());
        assert!(get_custom_scaling_joker_by_id(JokerId::Joker).is_none()); // Not a custom scaling joker
    }
}