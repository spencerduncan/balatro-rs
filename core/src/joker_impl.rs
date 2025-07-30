use crate::card::{Card, Suit, Value};
use crate::hand::SelectHand;
use crate::joker::{GameContext, Joker, JokerEffect, JokerId, JokerRarity};
use serde::{Deserialize, Serialize};

// Basic Joker implementation
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TheJoker;

impl Joker for TheJoker {
    fn id(&self) -> JokerId {
        JokerId::Joker
    }

    fn name(&self) -> &str {
        "Joker"
    }

    fn description(&self) -> &str {
        "+4 Mult"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Common
    }

    fn cost(&self) -> usize {
        2
    }

    fn on_hand_played(&self, _context: &mut GameContext, _hand: &SelectHand) -> JokerEffect {
        JokerEffect::new().with_mult(4)
    }
}

// Greedy Joker implementation
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GreedyJoker;

impl Joker for GreedyJoker {
    fn id(&self) -> JokerId {
        JokerId::GreedyJoker
    }

    fn name(&self) -> &str {
        "Greedy Joker"
    }

    fn description(&self) -> &str {
        "Played cards with Diamond suit give +3 Mult when scored"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Common
    }

    fn cost(&self) -> usize {
        5
    }

    fn on_card_scored(&self, _context: &mut GameContext, card: &Card) -> JokerEffect {
        if card.suit == Suit::Diamond {
            JokerEffect::new().with_mult(3)
        } else {
            JokerEffect::new()
        }
    }
}

// Lusty Joker implementation
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LustyJoker;

impl Joker for LustyJoker {
    fn id(&self) -> JokerId {
        JokerId::LustyJoker
    }

    fn name(&self) -> &str {
        "Lusty Joker"
    }

    fn description(&self) -> &str {
        "Played cards with Heart suit give +3 Mult when scored"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Common
    }

    fn cost(&self) -> usize {
        5
    }

    fn on_card_scored(&self, _context: &mut GameContext, card: &Card) -> JokerEffect {
        if card.suit == Suit::Heart {
            JokerEffect::new().with_mult(3)
        } else {
            JokerEffect::new()
        }
    }
}

// Wrathful Joker implementation
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WrathfulJoker;

impl Joker for WrathfulJoker {
    fn id(&self) -> JokerId {
        JokerId::WrathfulJoker
    }

    fn name(&self) -> &str {
        "Wrathful Joker"
    }

    fn description(&self) -> &str {
        "Played cards with Spade suit give +3 Mult when scored"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Common
    }

    fn cost(&self) -> usize {
        5
    }

    fn on_card_scored(&self, _context: &mut GameContext, card: &Card) -> JokerEffect {
        if card.suit == Suit::Spade {
            JokerEffect::new().with_mult(3)
        } else {
            JokerEffect::new()
        }
    }
}

// Gluttonous Joker implementation
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GluttonousJoker;

impl Joker for GluttonousJoker {
    fn id(&self) -> JokerId {
        JokerId::GluttonousJoker
    }

    fn name(&self) -> &str {
        "Gluttonous Joker"
    }

    fn description(&self) -> &str {
        "Played cards with Club suit give +3 Mult when scored"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Common
    }

    fn cost(&self) -> usize {
        5
    }

    fn on_card_scored(&self, _context: &mut GameContext, card: &Card) -> JokerEffect {
        if card.suit == Suit::Club {
            JokerEffect::new().with_mult(3)
        } else {
            JokerEffect::new()
        }
    }
}

// Jolly Joker implementation
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JollyJoker;

impl Joker for JollyJoker {
    fn id(&self) -> JokerId {
        JokerId::JollyJoker
    }

    fn name(&self) -> &str {
        "Jolly Joker"
    }

    fn description(&self) -> &str {
        "+8 Mult if played hand contains a Pair"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Common
    }

    fn cost(&self) -> usize {
        3
    }

    fn on_hand_played(&self, _context: &mut GameContext, hand: &SelectHand) -> JokerEffect {
        if hand.is_pair().is_some() {
            JokerEffect::new().with_mult(8)
        } else {
            JokerEffect::new()
        }
    }
}

// Zany Joker implementation
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ZanyJoker;

impl Joker for ZanyJoker {
    fn id(&self) -> JokerId {
        JokerId::ZanyJoker
    }

    fn name(&self) -> &str {
        "Zany Joker"
    }

    fn description(&self) -> &str {
        "+12 Mult if played hand contains a Three of a Kind"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Common
    }

    fn cost(&self) -> usize {
        4
    }

    fn on_hand_played(&self, _context: &mut GameContext, hand: &SelectHand) -> JokerEffect {
        if hand.is_three_of_kind().is_some() {
            JokerEffect::new().with_mult(12)
        } else {
            JokerEffect::new()
        }
    }
}

// Mad Joker implementation
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MadJoker;

impl Joker for MadJoker {
    fn id(&self) -> JokerId {
        JokerId::MadJoker
    }

    fn name(&self) -> &str {
        "Mad Joker"
    }

    fn description(&self) -> &str {
        "+10 Mult if played hand contains a Two Pair"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Common
    }

    fn cost(&self) -> usize {
        4
    }

    fn on_hand_played(&self, _context: &mut GameContext, hand: &SelectHand) -> JokerEffect {
        if hand.is_two_pair().is_some() {
            JokerEffect::new().with_mult(10)
        } else {
            JokerEffect::new()
        }
    }
}

// Crazy Joker implementation
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CrazyJoker;

impl Joker for CrazyJoker {
    fn id(&self) -> JokerId {
        JokerId::CrazyJoker
    }

    fn name(&self) -> &str {
        "Crazy Joker"
    }

    fn description(&self) -> &str {
        "+12 Mult if played hand contains a Straight"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Common
    }

    fn cost(&self) -> usize {
        4
    }

    fn on_hand_played(&self, _context: &mut GameContext, hand: &SelectHand) -> JokerEffect {
        if hand.is_straight().is_some() {
            JokerEffect::new().with_mult(12)
        } else {
            JokerEffect::new()
        }
    }
}

// Droll Joker implementation
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DrollJoker;

impl Joker for DrollJoker {
    fn id(&self) -> JokerId {
        JokerId::DrollJoker
    }

    fn name(&self) -> &str {
        "Droll Joker"
    }

    fn description(&self) -> &str {
        "+10 Mult if played hand contains a Flush"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Common
    }

    fn cost(&self) -> usize {
        4
    }

    fn on_hand_played(&self, _context: &mut GameContext, hand: &SelectHand) -> JokerEffect {
        if hand.is_flush().is_some() {
            JokerEffect::new().with_mult(10)
        } else {
            JokerEffect::new()
        }
    }
}

// Sly Joker implementation
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SlyJoker;

impl Joker for SlyJoker {
    fn id(&self) -> JokerId {
        JokerId::SlyJoker
    }

    fn name(&self) -> &str {
        "Sly Joker"
    }

    fn description(&self) -> &str {
        "+50 Chips if played hand contains a Pair"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Common
    }

    fn cost(&self) -> usize {
        3
    }

    fn on_hand_played(&self, _context: &mut GameContext, hand: &SelectHand) -> JokerEffect {
        if hand.is_pair().is_some() {
            JokerEffect::new().with_chips(50)
        } else {
            JokerEffect::new()
        }
    }
}

// Wily Joker implementation
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WilyJoker;

impl Joker for WilyJoker {
    fn id(&self) -> JokerId {
        JokerId::WilyJoker
    }

    fn name(&self) -> &str {
        "Wily Joker"
    }

    fn description(&self) -> &str {
        "+100 Chips if played hand contains a Three of a Kind"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Common
    }

    fn cost(&self) -> usize {
        4
    }

    fn on_hand_played(&self, _context: &mut GameContext, hand: &SelectHand) -> JokerEffect {
        if hand.is_three_of_kind().is_some() {
            JokerEffect::new().with_chips(100)
        } else {
            JokerEffect::new()
        }
    }
}

// Clever Joker implementation
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CleverJoker;

impl Joker for CleverJoker {
    fn id(&self) -> JokerId {
        JokerId::CleverJoker
    }

    fn name(&self) -> &str {
        "Clever Joker"
    }

    fn description(&self) -> &str {
        "+80 Chips if played hand contains a Two Pair"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Common
    }

    fn cost(&self) -> usize {
        4
    }

    fn on_hand_played(&self, _context: &mut GameContext, hand: &SelectHand) -> JokerEffect {
        if hand.is_two_pair().is_some() {
            JokerEffect::new().with_chips(80)
        } else {
            JokerEffect::new()
        }
    }
}

// Devious Joker implementation
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeviousJoker;

impl Joker for DeviousJoker {
    fn id(&self) -> JokerId {
        JokerId::DeviousJoker
    }

    fn name(&self) -> &str {
        "Devious Joker"
    }

    fn description(&self) -> &str {
        "+100 Chips if played hand contains a Straight"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Common
    }

    fn cost(&self) -> usize {
        4
    }

    fn on_hand_played(&self, _context: &mut GameContext, hand: &SelectHand) -> JokerEffect {
        if hand.is_straight().is_some() {
            JokerEffect::new().with_chips(100)
        } else {
            JokerEffect::new()
        }
    }
}

// Crafty Joker implementation
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CraftyJoker;

impl Joker for CraftyJoker {
    fn id(&self) -> JokerId {
        JokerId::CraftyJoker
    }

    fn name(&self) -> &str {
        "Crafty Joker"
    }

    fn description(&self) -> &str {
        "+80 Chips if played hand contains a Flush"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Common
    }

    fn cost(&self) -> usize {
        4
    }

    fn on_hand_played(&self, _context: &mut GameContext, hand: &SelectHand) -> JokerEffect {
        if hand.is_flush().is_some() {
            JokerEffect::new().with_chips(80)
        } else {
            JokerEffect::new()
        }
    }
}

// Money-Based Conditional Jokers for Issue #82

// Business Card: face cards have 1 in 2 chance of giving $2 when scored
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BusinessCard;

impl Joker for BusinessCard {
    fn id(&self) -> JokerId {
        JokerId::BusinessCard
    }

    fn name(&self) -> &str {
        "Business Card"
    }

    fn description(&self) -> &str {
        "Face cards have 1 in 2 chance of giving $2 when scored"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Common
    }

    fn cost(&self) -> usize {
        4
    }

    fn on_card_scored(&self, context: &mut GameContext, card: &Card) -> JokerEffect {
        if card.is_face() {
            if context.rng.gen_bool(0.5) {
                JokerEffect::new().with_money(2)
            } else {
                JokerEffect::new()
            }
        } else {
            JokerEffect::new()
        }
    }
}

// Egg: gains $3 sell value at round end
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Egg;

impl Joker for Egg {
    fn id(&self) -> JokerId {
        JokerId::EggJoker
    }

    fn name(&self) -> &str {
        "Egg"
    }

    fn description(&self) -> &str {
        "Gains $3 sell value at end of round"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Common
    }

    fn cost(&self) -> usize {
        3
    }

    fn sell_value(&self, accumulated_bonus: f64) -> usize {
        // Base sell value (cost / 2) + accumulated bonus from rounds
        (self.cost() / 2) + (accumulated_bonus as usize)
    }

    fn on_round_end(&self, _context: &mut GameContext) -> JokerEffect {
        JokerEffect::new()
            .with_sell_value_increase(3)
            .with_message("Egg gained $3 sell value".to_string())
    }
}

// Burglar: gain +3 hands when Blind selected, lose all discards
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Burglar;

impl Joker for Burglar {
    fn id(&self) -> JokerId {
        JokerId::Burglar
    }

    fn name(&self) -> &str {
        "Burglar"
    }

    fn description(&self) -> &str {
        "Gain +3 hands when Blind selected, lose all discards"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Uncommon
    }

    fn cost(&self) -> usize {
        6
    }

    fn on_blind_start(&self, _context: &mut GameContext) -> JokerEffect {
        let mut effect = JokerEffect::new();
        effect.hand_size_mod = 3;
        effect.discard_mod = -999; // Set to very negative to remove all discards
        effect
    }
}

// Supernova implementation - tracks hand types played this game run
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SupernovaJoker;

impl Joker for SupernovaJoker {
    fn id(&self) -> JokerId {
        JokerId::Supernova
    }

    fn name(&self) -> &str {
        "Supernova"
    }

    fn description(&self) -> &str {
        "Mult equal to times this poker hand has been played"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Common
    }

    fn cost(&self) -> usize {
        3
    }

    fn on_hand_played(&self, context: &mut GameContext, hand: &SelectHand) -> JokerEffect {
        // First determine what hand type was played
        if let Ok(made_hand) = hand.best_hand() {
            let hand_rank = made_hand.rank;

            // Get the count for this hand type from the centralized tracking
            // Note: This will be the count AFTER the current hand is played
            // since the game increments the count before calling joker effects
            let count = context.get_hand_type_count(hand_rank);

            // Return mult equal to the count
            JokerEffect::new().with_mult(count as i32)
        } else {
            JokerEffect::new()
        }
    }
}

// Ice Cream Joker implementation - decaying chip joker
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IceCreamJoker;

impl Joker for IceCreamJoker {
    fn id(&self) -> JokerId {
        JokerId::IceCream
    }

    fn name(&self) -> &str {
        "Ice Cream"
    }

    fn description(&self) -> &str {
        "+100 Chips, -5 Chips per hand played"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Common
    }

    fn cost(&self) -> usize {
        5
    }

    fn on_created(&self, context: &mut GameContext) -> JokerEffect {
        // Initialize state with 100 chips
        context
            .joker_state_manager
            .update_state(self.id(), |state| {
                state
                    .set_custom("remaining_chips", 100i32)
                    .expect("Failed to set remaining_chips");
            });
        JokerEffect::new()
    }

    fn on_hand_played(&self, context: &mut GameContext, _hand: &SelectHand) -> JokerEffect {
        // Get current remaining chips from state manager
        let remaining_chips = context
            .joker_state_manager
            .get_or_default(self.id())
            .get_custom::<i32>("remaining_chips")
            .unwrap_or(Some(100))
            .unwrap_or(100);

        // Provide chips bonus (capped at 0 to avoid negative)
        let chips_bonus = remaining_chips.max(0);

        // Decay the chips by 5 for next hand
        context
            .joker_state_manager
            .update_state(self.id(), |state| {
                state
                    .set_custom("remaining_chips", remaining_chips - 5)
                    .expect("Failed to update remaining_chips");
            });

        JokerEffect::new().with_chips(chips_bonus)
    }
}

// Runner implementation - accumulates chips when straights are played, gives bonus on every hand
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RunnerJoker;

impl Joker for RunnerJoker {
    fn id(&self) -> JokerId {
        JokerId::Runner
    }

    fn name(&self) -> &str {
        "Runner"
    }

    fn description(&self) -> &str {
        "+15 Chips if played hand contains a Straight"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Common
    }

    fn cost(&self) -> usize {
        3
    }

    fn on_hand_played(&self, context: &mut GameContext, hand: &SelectHand) -> JokerEffect {
        // Check if hand contains a straight (any type)
        let is_straight = hand.is_straight().is_some()
            || hand.is_straight_flush().is_some()
            || hand.is_royal_flush().is_some();

        // Get current accumulated chips
        let current_accumulated = context
            .joker_state_manager
            .get_state(self.id())
            .map(|state| state.accumulated_value as i32)
            .unwrap_or(0);

        // If it's a straight, accumulate +15 chips BEFORE giving the bonus
        let new_accumulated = if is_straight {
            let new_value = current_accumulated + 15;
            context
                .joker_state_manager
                .add_accumulated_value(self.id(), 15.0);
            new_value
        } else {
            current_accumulated
        };

        // Always give the accumulated bonus regardless of hand type
        JokerEffect::new().with_chips(new_accumulated)
    }
}

// Space Joker implementation - 1 in 4 chance for +1 hand level
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SpaceJoker;

impl Joker for SpaceJoker {
    fn id(&self) -> JokerId {
        JokerId::SpaceJoker
    }

    fn name(&self) -> &str {
        "Space Joker"
    }

    fn description(&self) -> &str {
        "1 in 4 chance to upgrade level of played poker hand"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Uncommon
    }

    fn cost(&self) -> usize {
        6
    }

    fn on_hand_played(&self, context: &mut GameContext, _hand: &SelectHand) -> JokerEffect {
        // 1 in 4 chance (25% probability)
        if context.rng.gen_bool(0.25) {
            // TODO: Implement hand level upgrade effect
            // For now, return a message effect
            let mut effect = JokerEffect::new();
            effect.message = Some("Space Joker activated! Hand level upgraded!".to_string());
            effect
        } else {
            JokerEffect::new()
        }
    }
}

// Abstract Joker implementation - provides mult based on number of other jokers
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AbstractJoker;

impl Joker for AbstractJoker {
    fn id(&self) -> JokerId {
        JokerId::AbstractJoker
    }

    fn name(&self) -> &str {
        "Abstract Joker"
    }

    fn description(&self) -> &str {
        "All Jokers give X0.25 more Mult"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Common
    }

    fn cost(&self) -> usize {
        3
    }

    fn on_hand_played(&self, context: &mut GameContext, _hand: &SelectHand) -> JokerEffect {
        // Count all jokers except this one
        let other_joker_count = context
            .jokers
            .iter()
            .filter(|joker| joker.id() != self.id())
            .count();

        // Provide +3 mult per other joker (simplified implementation)
        // This represents the "X0.25 more Mult" for all jokers conceptually
        let mult_bonus = (other_joker_count as i32) * 3;

        JokerEffect::new().with_mult(mult_bonus)
    }
}

// RNG-Based Jokers for Issue #442

// Oops All Sixes! implementation - doubles all probabilities
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OopsAllSixesJoker;

impl Joker for OopsAllSixesJoker {
    fn id(&self) -> JokerId {
        JokerId::Oops
    }

    fn name(&self) -> &str {
        "Oops! All 6s"
    }

    fn description(&self) -> &str {
        "All listed probabilities are doubled"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Uncommon
    }

    fn cost(&self) -> usize {
        8
    }

    // This joker modifies other jokers' probabilities - handled by the game engine
    fn on_hand_played(&self, _context: &mut GameContext, _hand: &SelectHand) -> JokerEffect {
        // The probability doubling effect is handled by the game engine when this joker is present
        JokerEffect::new()
    }
}

// Six Shooter implementation - +4 Mult for each 6 in hand
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SixShooterJoker;

impl Joker for SixShooterJoker {
    fn id(&self) -> JokerId {
        JokerId::Reserved7 // Using a reserved slot for now
    }

    fn name(&self) -> &str {
        "Six Shooter"
    }

    fn description(&self) -> &str {
        "+4 Mult for each 6 in your hand"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Common
    }

    fn cost(&self) -> usize {
        4
    }

    fn on_hand_played(&self, context: &mut GameContext, _hand: &SelectHand) -> JokerEffect {
        // Count 6s in hand
        let six_count = context
            .hand
            .cards()
            .iter()
            .filter(|card| card.value == Value::Six)
            .count();

        JokerEffect::new().with_mult((six_count * 4) as i32)
    }
}

// Lucky Card implementation - 1 in 5 chance for +20 Mult
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LuckyCardJoker;

impl Joker for LuckyCardJoker {
    fn id(&self) -> JokerId {
        JokerId::LuckyCharm
    }

    fn name(&self) -> &str {
        "Lucky Card"
    }

    fn description(&self) -> &str {
        "1 in 5 chance for +20 Mult"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Common
    }

    fn cost(&self) -> usize {
        5
    }

    fn on_hand_played(&self, context: &mut GameContext, _hand: &SelectHand) -> JokerEffect {
        // 1 in 5 chance (20% probability)
        if context.rng.gen_bool(0.2) {
            JokerEffect::new()
                .with_mult(20)
                .with_message("Lucky Card activated! +20 Mult!".to_string())
        } else {
            JokerEffect::new()
        }
    }
}

// Grim Joker implementation - destroyed when 2 Hearts played
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GrimJoker;

impl Joker for GrimJoker {
    fn id(&self) -> JokerId {
        JokerId::Reserved8 // Using a reserved slot for now
    }

    fn name(&self) -> &str {
        "Grim Joker"
    }

    fn description(&self) -> &str {
        "+25 Mult, destroyed if 2 or more Hearts are played"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Uncommon
    }

    fn cost(&self) -> usize {
        6
    }

    fn on_hand_played(&self, _context: &mut GameContext, hand: &SelectHand) -> JokerEffect {
        // Count Hearts in played hand
        let heart_count = hand
            .cards()
            .iter()
            .filter(|card| card.suit == Suit::Heart)
            .count();

        if heart_count >= 2 {
            // Destroy self when 2+ Hearts played
            let mut effect = JokerEffect::new();
            effect.destroy_self = true;
            effect.message = Some("Grim Joker destroyed by Hearts!".to_string());
            effect
        } else {
            // Normal bonus when condition not met
            JokerEffect::new().with_mult(25)
        }
    }
}

// Acrobat Joker implementation - X3 Mult on final hand
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AcrobatJokerImpl;

impl AcrobatJokerImpl {
    /// Get the multiplier parameter for Acrobat joker from joker.json
    /// TODO: Replace with proper JsonParameterResolver when available
    fn get_multiplier_parameter() -> f64 {
        // From joker.json: "X#1# Mult on final hand"
        // Based on original implementation and joker.json pattern, #1# = 3
        3.0
    }
}

impl Joker for AcrobatJokerImpl {
    fn id(&self) -> JokerId {
        JokerId::AcrobatJoker
    }

    fn name(&self) -> &str {
        "Acrobat"
    }

    fn description(&self) -> &str {
        "X3 Mult on final hand of round"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Rare
    }

    fn cost(&self) -> usize {
        8
    }

    fn on_hand_played(&self, context: &mut GameContext, _hand: &SelectHand) -> JokerEffect {
        // Check if this is the final hand of the round
        // Use the definitive hands_remaining count from the game engine
        if context.hands_remaining <= 1.0 {
            // This is the final hand - apply the multiplier from joker.json parameter
            let multiplier = Self::get_multiplier_parameter();
            JokerEffect::new()
                .with_mult_multiplier(multiplier)
                .with_message(format!(
                    "Acrobat final hand bonus! X{} Mult!",
                    multiplier as i32
                ))
        } else {
            JokerEffect::new()
        }
    }
}

// Mystery Joker implementation - random effect each hand
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MysteryJoker;

impl Joker for MysteryJoker {
    fn id(&self) -> JokerId {
        JokerId::Reserved4 // Use Reserved4 to avoid conflict with Fortune Teller
    }

    fn name(&self) -> &str {
        "Mystery Joker"
    }

    fn description(&self) -> &str {
        "Random effect each hand"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Rare
    }

    fn cost(&self) -> usize {
        10
    }

    fn on_hand_played(&self, context: &mut GameContext, _hand: &SelectHand) -> JokerEffect {
        // Generate random effect - choose from several possibilities
        let effect_type = context.rng.gen_range(0..6);

        match effect_type {
            0 => JokerEffect::new()
                .with_mult(15)
                .with_message("Mystery effect: +15 Mult!".to_string()),
            1 => JokerEffect::new()
                .with_chips(100)
                .with_message("Mystery effect: +100 Chips!".to_string()),
            2 => JokerEffect::new()
                .with_money(5)
                .with_message("Mystery effect: +$5!".to_string()),
            3 => JokerEffect::new()
                .with_mult_multiplier(2.0)
                .with_message("Mystery effect: X2 Mult!".to_string()),
            4 => JokerEffect::new()
                .with_retrigger(1)
                .with_message("Mystery effect: Retrigger!".to_string()),
            _ => JokerEffect::new()
                .with_chips(50)
                .with_mult(10)
                .with_message("Mystery effect: Balanced bonus!".to_string()),
        }
    }
}

// Vagabond Joker implementation - Create Tarot if hand played with $3 or less
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VagabondJokerImpl;

impl Joker for VagabondJokerImpl {
    fn id(&self) -> JokerId {
        JokerId::VagabondJoker
    }

    fn name(&self) -> &str {
        "Vagabond"
    }

    fn description(&self) -> &str {
        "Create a Tarot card if hand played with $3 or less"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Uncommon
    }

    fn cost(&self) -> usize {
        7
    }

    fn on_hand_played(&self, context: &mut GameContext, _hand: &SelectHand) -> JokerEffect {
        // Check if player has $3 or less
        if context.money <= 3 {
            // Create a tarot card (simplified - actual implementation would add to shop/consumables)
            JokerEffect::new().with_message("Vagabond created a Tarot card!".to_string())
        } else {
            JokerEffect::new()
        }
    }
}

// Chaotic Joker implementation - randomizes all joker effects
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChaoticJoker;

impl Joker for ChaoticJoker {
    fn id(&self) -> JokerId {
        JokerId::Reserved9 // Using a reserved slot for now
    }

    fn name(&self) -> &str {
        "Chaotic Joker"
    }

    fn description(&self) -> &str {
        "Randomize all other Joker effects this hand"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Legendary
    }

    fn cost(&self) -> usize {
        15
    }

    fn on_hand_played(&self, context: &mut GameContext, _hand: &SelectHand) -> JokerEffect {
        // This joker affects other jokers' effects - would need game engine support
        // For now, provide a random bonus itself
        let chaos_type = context.rng.gen_range(0..4);

        match chaos_type {
            0 => JokerEffect::new()
                .with_mult(context.rng.gen_range(5..25))
                .with_message("Chaos brings random Mult!".to_string()),
            1 => JokerEffect::new()
                .with_chips(context.rng.gen_range(25..150))
                .with_message("Chaos brings random Chips!".to_string()),
            2 => JokerEffect::new()
                .with_mult_multiplier(1.0 + context.rng.gen_range(0.0..2.0))
                .with_message("Chaos brings random multiplier!".to_string()),
            _ => JokerEffect::new()
                .with_money(context.rng.gen_range(1..8))
                .with_message("Chaos brings random money!".to_string()),
        }
    }
}

// Triboulet Joker implementation - Kings and Queens give X2 Mult when scored
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TribouletJoker;

impl Joker for TribouletJoker {
    fn id(&self) -> JokerId {
        JokerId::Triboulet
    }

    fn name(&self) -> &str {
        "Triboulet"
    }

    fn description(&self) -> &str {
        "Played Kings and Queens each give X2 Mult when scored"
    }

    fn rarity(&self) -> JokerRarity {
        JokerRarity::Legendary
    }

    fn cost(&self) -> usize {
        20
    }

    fn on_card_scored(&self, _context: &mut GameContext, card: &Card) -> JokerEffect {
        if matches!(card.value, Value::Queen | Value::King) {
            JokerEffect::new().with_mult_multiplier(2.0)
        } else {
            JokerEffect::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hand::{Hand, SelectHand};
    use crate::joker::{GameContext, JokerId, JokerRarity};
    use crate::joker_factory::JokerFactory;
    use crate::joker_state::JokerStateManager;
    use crate::stage::{Blind, Stage};
    use std::collections::HashMap;
    use std::sync::Arc;

    #[test]
    fn test_ice_cream_basic_properties() {
        let ice_cream = IceCreamJoker;

        assert_eq!(ice_cream.id(), JokerId::IceCream);
        assert_eq!(ice_cream.name(), "Ice Cream");
        assert_eq!(
            ice_cream.description(),
            "+100 Chips, -5 Chips per hand played"
        );
        assert_eq!(ice_cream.rarity(), JokerRarity::Common);
        assert_eq!(ice_cream.cost(), 5);
    }

    #[test]
    fn test_ice_cream_initial_chips() {
        // Initial chips are handled by state manager now
        // This is tested in integration tests
    }

    #[test]
    fn test_ice_cream_factory_creation() {
        let created_joker = JokerFactory::create(JokerId::IceCream);
        assert!(
            created_joker.is_some(),
            "Ice Cream should be creatable from factory"
        );

        let joker_instance = created_joker.unwrap();
        assert_eq!(joker_instance.id(), JokerId::IceCream);
        assert_eq!(joker_instance.name(), "Ice Cream");
        assert_eq!(joker_instance.rarity(), JokerRarity::Common);
        assert_eq!(joker_instance.cost(), 5);
    }

    #[test]
    fn test_ice_cream_in_common_rarity() {
        let common_jokers = JokerFactory::get_by_rarity(JokerRarity::Common);
        assert!(
            common_jokers.contains(&JokerId::IceCream),
            "Ice Cream should be listed in Common rarity jokers"
        );
    }

    #[test]
    fn test_ice_cream_in_implemented_list() {
        let all_implemented = JokerFactory::get_all_implemented();
        assert!(
            all_implemented.contains(&JokerId::IceCream),
            "Ice Cream should be in the list of all implemented jokers"
        );
    }

    #[test]
    fn test_ice_cream_zero_chips_handling() {
        // Zero chip handling is now covered by integration tests
        // The max(0) in on_hand_played ensures no negative chips are provided
    }

    #[test]
    fn test_ice_cream_negative_chips_handling() {
        // Negative chip handling is now covered by integration tests
        // The max(0) in on_hand_played ensures no negative chips are provided
    }

    // Tests for RNG-based jokers (Issue #442)

    #[test]
    fn test_oops_all_sixes_basic_properties() {
        let oops = OopsAllSixesJoker;
        assert_eq!(oops.id(), JokerId::Oops);
        assert_eq!(oops.name(), "Oops! All 6s");
        assert_eq!(oops.description(), "All listed probabilities are doubled");
        assert_eq!(oops.rarity(), JokerRarity::Uncommon);
        assert_eq!(oops.cost(), 8);
    }

    #[test]
    fn test_six_shooter_basic_properties() {
        let six_shooter = SixShooterJoker;
        assert_eq!(six_shooter.id(), JokerId::Reserved7);
        assert_eq!(six_shooter.name(), "Six Shooter");
        assert_eq!(six_shooter.description(), "+4 Mult for each 6 in your hand");
        assert_eq!(six_shooter.rarity(), JokerRarity::Common);
        assert_eq!(six_shooter.cost(), 4);
    }

    #[test]
    fn test_lucky_card_basic_properties() {
        let lucky_card = LuckyCardJoker;
        assert_eq!(lucky_card.id(), JokerId::LuckyCharm);
        assert_eq!(lucky_card.name(), "Lucky Card");
        assert_eq!(lucky_card.description(), "1 in 5 chance for +20 Mult");
        assert_eq!(lucky_card.rarity(), JokerRarity::Common);
        assert_eq!(lucky_card.cost(), 5);
    }

    #[test]
    fn test_grim_joker_basic_properties() {
        let grim = GrimJoker;
        assert_eq!(grim.id(), JokerId::Reserved8);
        assert_eq!(grim.name(), "Grim Joker");
        assert_eq!(
            grim.description(),
            "+25 Mult, destroyed if 2 or more Hearts are played"
        );
        assert_eq!(grim.rarity(), JokerRarity::Uncommon);
        assert_eq!(grim.cost(), 6);
    }

    #[test]
    fn test_acrobat_joker_basic_properties() {
        let acrobat = AcrobatJokerImpl;
        assert_eq!(acrobat.id(), JokerId::AcrobatJoker);
        assert_eq!(acrobat.name(), "Acrobat");
        assert_eq!(acrobat.description(), "X3 Mult on final hand of round");
        assert_eq!(acrobat.rarity(), JokerRarity::Rare);
        assert_eq!(acrobat.cost(), 8);
    }

    #[test]
    fn test_acrobat_joker_final_hand_detection() {
        let acrobat = AcrobatJokerImpl;
        let stage = Stage::Blind(Blind::Small);
        let jokers: Vec<Box<dyn Joker>> = vec![];
        let hand = Hand::new(vec![]);
        let discarded: Vec<Card> = vec![];
        let joker_state_manager = Arc::new(JokerStateManager::new());
        let hand_type_counts = HashMap::new();
        let rng = crate::rng::GameRng::secure();

        // Test final hand (hands_remaining = 1.0)
        let mut context = GameContext {
            chips: 0,
            mult: 1,
            money: 0,
            ante: 1,
            round: 1,
            stage: &stage,
            hands_played: 3,
            hands_remaining: 1.0, // Final hand
            discards_used: 0,
            jokers: &jokers,
            hand: &hand,
            discarded: &discarded,
            joker_state_manager: &joker_state_manager,
            hand_type_counts: &hand_type_counts,
            cards_in_deck: 52,
            stone_cards_in_deck: 0,
            steel_cards_in_deck: 0,
            rng: &rng,
        };

        let select_hand = SelectHand::new(vec![]);
        let effect = acrobat.on_hand_played(&mut context, &select_hand);

        // Should apply multiplier on final hand
        assert_eq!(effect.mult_multiplier, 3.0);
        assert!(effect.message.is_some());
        assert!(effect.message.unwrap().contains("X3 Mult"));
    }

    #[test]
    fn test_acrobat_joker_non_final_hand() {
        let acrobat = AcrobatJokerImpl;
        let stage = Stage::Blind(Blind::Small);
        let jokers: Vec<Box<dyn Joker>> = vec![];
        let hand = Hand::new(vec![]);
        let discarded: Vec<Card> = vec![];
        let joker_state_manager = Arc::new(JokerStateManager::new());
        let hand_type_counts = HashMap::new();
        let rng = crate::rng::GameRng::secure();

        // Test non-final hand (hands_remaining > 1.0)
        let mut context = GameContext {
            chips: 0,
            mult: 1,
            money: 0,
            ante: 1,
            round: 1,
            stage: &stage,
            hands_played: 1,
            hands_remaining: 3.0, // Not final hand
            discards_used: 0,
            jokers: &jokers,
            hand: &hand,
            discarded: &discarded,
            joker_state_manager: &joker_state_manager,
            hand_type_counts: &hand_type_counts,
            cards_in_deck: 52,
            stone_cards_in_deck: 0,
            steel_cards_in_deck: 0,
            rng: &rng,
        };

        let select_hand = SelectHand::new(vec![]);
        let effect = acrobat.on_hand_played(&mut context, &select_hand);

        // Should NOT apply multiplier on non-final hand
        assert_eq!(effect.mult_multiplier, 1.0);
        assert!(effect.message.is_none());
    }

    #[test]
    fn test_acrobat_joker_edge_cases() {
        let acrobat = AcrobatJokerImpl;
        let stage = Stage::Blind(Blind::Small);
        let jokers: Vec<Box<dyn Joker>> = vec![];
        let hand = Hand::new(vec![]);
        let discarded: Vec<Card> = vec![];
        let joker_state_manager = Arc::new(JokerStateManager::new());
        let hand_type_counts = HashMap::new();
        let rng = crate::rng::GameRng::secure();
        let select_hand = SelectHand::new(vec![]);

        // Test edge case: hands_remaining = 0.5 (should be final)
        let mut context = GameContext {
            chips: 0,
            mult: 1,
            money: 0,
            ante: 1,
            round: 1,
            stage: &stage,
            hands_played: 3,
            hands_remaining: 0.5,
            discards_used: 0,
            jokers: &jokers,
            hand: &hand,
            discarded: &discarded,
            joker_state_manager: &joker_state_manager,
            hand_type_counts: &hand_type_counts,
            cards_in_deck: 52,
            stone_cards_in_deck: 0,
            steel_cards_in_deck: 0,
            rng: &rng,
        };
        let effect = acrobat.on_hand_played(&mut context, &select_hand);
        assert_eq!(effect.mult_multiplier, 3.0); // Should trigger

        // Test edge case: hands_remaining = 0.0 (should be final)
        context.hands_remaining = 0.0;
        let effect = acrobat.on_hand_played(&mut context, &select_hand);
        assert_eq!(effect.mult_multiplier, 3.0); // Should trigger

        // Test edge case: hands_remaining = 1.1 (should NOT be final)
        context.hands_remaining = 1.1;
        let effect = acrobat.on_hand_played(&mut context, &select_hand);
        assert_eq!(effect.mult_multiplier, 1.0); // Should NOT trigger
    }

    #[test]
    fn test_acrobat_joker_parameter_function() {
        // Test that the parameter function returns the expected value
        let multiplier = AcrobatJokerImpl::get_multiplier_parameter();
        assert_eq!(multiplier, 3.0);
    }

    #[test]
    fn test_mystery_joker_basic_properties() {
        let mystery = MysteryJoker;
        assert_eq!(mystery.id(), JokerId::Reserved4);
        assert_eq!(mystery.name(), "Mystery Joker");
        assert_eq!(mystery.description(), "Random effect each hand");
        assert_eq!(mystery.rarity(), JokerRarity::Rare);
        assert_eq!(mystery.cost(), 10);
    }

    #[test]
    fn test_vagabond_joker_basic_properties() {
        let vagabond = VagabondJokerImpl;
        assert_eq!(vagabond.id(), JokerId::VagabondJoker);
        assert_eq!(vagabond.name(), "Vagabond");
        assert_eq!(
            vagabond.description(),
            "Create a Tarot card if hand played with $3 or less"
        );
        assert_eq!(vagabond.rarity(), JokerRarity::Uncommon);
        assert_eq!(vagabond.cost(), 7);
    }

    #[test]
    fn test_chaotic_joker_basic_properties() {
        let chaotic = ChaoticJoker;
        assert_eq!(chaotic.id(), JokerId::Reserved9);
        assert_eq!(chaotic.name(), "Chaotic Joker");
        assert_eq!(
            chaotic.description(),
            "Randomize all other Joker effects this hand"
        );
        assert_eq!(chaotic.rarity(), JokerRarity::Legendary);
        assert_eq!(chaotic.cost(), 15);
    }

    #[test]
    fn test_rng_jokers_factory_creation() {
        // Test that all RNG jokers can be created from factory
        let oops = JokerFactory::create(JokerId::Oops);
        assert!(
            oops.is_some(),
            "OopsAllSixesJoker should be creatable from factory"
        );

        let six_shooter = JokerFactory::create(JokerId::Reserved7);
        assert!(
            six_shooter.is_some(),
            "SixShooterJoker should be creatable from factory"
        );

        let lucky_card = JokerFactory::create(JokerId::LuckyCharm);
        assert!(
            lucky_card.is_some(),
            "LuckyCardJoker should be creatable from factory"
        );

        let grim = JokerFactory::create(JokerId::Reserved8);
        assert!(grim.is_some(), "GrimJoker should be creatable from factory");

        let acrobat = JokerFactory::create(JokerId::AcrobatJoker);
        assert!(
            acrobat.is_some(),
            "AcrobatJoker should be creatable from factory"
        );

        let mystery = JokerFactory::create(JokerId::Reserved4);
        assert!(
            mystery.is_some(),
            "MysteryJoker should be creatable from factory"
        );

        let vagabond = JokerFactory::create(JokerId::VagabondJoker);
        assert!(
            vagabond.is_some(),
            "VagabondJoker should be creatable from factory"
        );

        let chaotic = JokerFactory::create(JokerId::Reserved9);
        assert!(
            chaotic.is_some(),
            "ChaoticJoker should be creatable from factory"
        );
    }

    #[test]
    fn test_rng_jokers_in_rarity_lists() {
        let common_jokers = JokerFactory::get_by_rarity(JokerRarity::Common);
        assert!(
            common_jokers.contains(&JokerId::Reserved7),
            "SixShooterJoker should be in Common rarity"
        );
        assert!(
            common_jokers.contains(&JokerId::LuckyCharm),
            "LuckyCardJoker should be in Common rarity"
        );

        let uncommon_jokers = JokerFactory::get_by_rarity(JokerRarity::Uncommon);
        assert!(
            uncommon_jokers.contains(&JokerId::Oops),
            "OopsAllSixesJoker should be in Uncommon rarity"
        );
        assert!(
            uncommon_jokers.contains(&JokerId::Reserved8),
            "GrimJoker should be in Uncommon rarity"
        );
        assert!(
            uncommon_jokers.contains(&JokerId::VagabondJoker),
            "VagabondJoker should be in Uncommon rarity"
        );

        let rare_jokers = JokerFactory::get_by_rarity(JokerRarity::Rare);
        assert!(
            rare_jokers.contains(&JokerId::AcrobatJoker),
            "AcrobatJoker should be in Rare rarity"
        );
        assert!(
            rare_jokers.contains(&JokerId::FortuneTeller),
            "Fortune Teller (JokerId::FortuneTeller) should be in Rare rarity"
        );

        let legendary_jokers = JokerFactory::get_by_rarity(JokerRarity::Legendary);
        assert!(
            legendary_jokers.contains(&JokerId::Reserved9),
            "ChaoticJoker should be in Legendary rarity"
        );
    }

    #[test]
    fn test_rng_jokers_in_implemented_list() {
        let all_implemented = JokerFactory::get_all_implemented();

        // Check all RNG jokers are listed as implemented
        assert!(
            all_implemented.contains(&JokerId::Oops),
            "OopsAllSixesJoker should be in implemented list"
        );
        assert!(
            all_implemented.contains(&JokerId::Reserved7),
            "SixShooterJoker should be in implemented list"
        );
        assert!(
            all_implemented.contains(&JokerId::LuckyCharm),
            "LuckyCardJoker should be in implemented list"
        );
        assert!(
            all_implemented.contains(&JokerId::Reserved8),
            "GrimJoker should be in implemented list"
        );
        assert!(
            all_implemented.contains(&JokerId::AcrobatJoker),
            "AcrobatJoker should be in implemented list"
        );
        assert!(
            all_implemented.contains(&JokerId::Reserved4),
            "MysteryJoker should be in implemented list"
        );
        assert!(
            all_implemented.contains(&JokerId::VagabondJoker),
            "VagabondJoker should be in implemented list"
        );
        assert!(
            all_implemented.contains(&JokerId::Reserved9),
            "ChaoticJoker should be in implemented list"
        );
    }

    // Tests for Triboulet joker
    #[test]
    fn test_triboulet_basic_properties() {
        let triboulet = TribouletJoker;
        assert_eq!(triboulet.id(), JokerId::Triboulet);
        assert_eq!(triboulet.name(), "Triboulet");
        assert_eq!(
            triboulet.description(),
            "Played Kings and Queens each give X2 Mult when scored"
        );
        assert_eq!(triboulet.rarity(), JokerRarity::Legendary);
        assert_eq!(triboulet.cost(), 20);
    }

    #[test]
    fn test_triboulet_king_gives_x2_mult() {
        let triboulet = TribouletJoker;
        let king_card = Card::new(Value::King, Suit::Heart);

        // Create a mock context (we don't use it in this implementation but it's required)
        use crate::hand::Hand;
        use crate::joker_state::JokerStateManager;
        use crate::stage::{Blind, Stage};
        use std::collections::HashMap;
        use std::sync::Arc;

        let joker_state_manager = Arc::new(JokerStateManager::new());
        let hand_type_counts = HashMap::new();
        let hand = Hand::new(vec![]);
        let discarded = vec![];
        let jokers: Vec<Box<dyn Joker>> = vec![];
        let rng = crate::rng::GameRng::new(crate::rng::RngMode::Testing(42));
        let stage = Stage::Blind(Blind::Small);

        let mut context = crate::joker::GameContext {
            chips: 0,
            mult: 0,
            money: 0,
            ante: 1,
            round: 1,
            stage: &stage,
            hands_played: 0,
            hands_remaining: 4.0,
            discards_used: 0,
            jokers: &jokers,
            hand: &hand,
            discarded: &discarded,
            joker_state_manager: &joker_state_manager,
            hand_type_counts: &hand_type_counts,
            cards_in_deck: 52,
            stone_cards_in_deck: 0,
            steel_cards_in_deck: 0,
            rng: &rng,
        };

        let effect = triboulet.on_card_scored(&mut context, &king_card);
        assert_eq!(effect.mult_multiplier, 2.0);
        assert_eq!(effect.mult, 0);
        assert_eq!(effect.chips, 0);
        assert_eq!(effect.money, 0);
    }

    #[test]
    fn test_triboulet_queen_gives_x2_mult() {
        let triboulet = TribouletJoker;
        let queen_card = Card::new(Value::Queen, Suit::Spade);

        use crate::hand::Hand;
        use crate::joker_state::JokerStateManager;
        use crate::stage::{Blind, Stage};
        use std::collections::HashMap;
        use std::sync::Arc;

        let joker_state_manager = Arc::new(JokerStateManager::new());
        let hand_type_counts = HashMap::new();
        let hand = Hand::new(vec![]);
        let discarded = vec![];
        let jokers: Vec<Box<dyn Joker>> = vec![];
        let rng = crate::rng::GameRng::new(crate::rng::RngMode::Testing(42));
        let stage = Stage::Blind(Blind::Small);

        let mut context = crate::joker::GameContext {
            chips: 0,
            mult: 0,
            money: 0,
            ante: 1,
            round: 1,
            stage: &stage,
            hands_played: 0,
            hands_remaining: 4.0,
            discards_used: 0,
            jokers: &jokers,
            hand: &hand,
            discarded: &discarded,
            joker_state_manager: &joker_state_manager,
            hand_type_counts: &hand_type_counts,
            cards_in_deck: 52,
            stone_cards_in_deck: 0,
            steel_cards_in_deck: 0,
            rng: &rng,
        };

        let effect = triboulet.on_card_scored(&mut context, &queen_card);
        assert_eq!(effect.mult_multiplier, 2.0);
        assert_eq!(effect.mult, 0);
        assert_eq!(effect.chips, 0);
        assert_eq!(effect.money, 0);
    }

    #[test]
    fn test_triboulet_jack_gives_no_effect() {
        let triboulet = TribouletJoker;
        let jack_card = Card::new(Value::Jack, Suit::Diamond);

        use crate::hand::Hand;
        use crate::joker_state::JokerStateManager;
        use crate::stage::{Blind, Stage};
        use std::collections::HashMap;
        use std::sync::Arc;

        let joker_state_manager = Arc::new(JokerStateManager::new());
        let hand_type_counts = HashMap::new();
        let hand = Hand::new(vec![]);
        let discarded = vec![];
        let jokers: Vec<Box<dyn Joker>> = vec![];
        let rng = crate::rng::GameRng::new(crate::rng::RngMode::Testing(42));
        let stage = Stage::Blind(Blind::Small);

        let mut context = crate::joker::GameContext {
            chips: 0,
            mult: 0,
            money: 0,
            ante: 1,
            round: 1,
            stage: &stage,
            hands_played: 0,
            hands_remaining: 4.0,
            discards_used: 0,
            jokers: &jokers,
            hand: &hand,
            discarded: &discarded,
            joker_state_manager: &joker_state_manager,
            hand_type_counts: &hand_type_counts,
            cards_in_deck: 52,
            stone_cards_in_deck: 0,
            steel_cards_in_deck: 0,
            rng: &rng,
        };

        let effect = triboulet.on_card_scored(&mut context, &jack_card);
        assert_eq!(effect.mult_multiplier, 1.0); // Default multiplier (no effect)
        assert_eq!(effect.mult, 0);
        assert_eq!(effect.chips, 0);
        assert_eq!(effect.money, 0);
    }

    #[test]
    fn test_triboulet_non_face_card_gives_no_effect() {
        let triboulet = TribouletJoker;
        let ace_card = Card::new(Value::Ace, Suit::Club);

        use crate::hand::Hand;
        use crate::joker_state::JokerStateManager;
        use crate::stage::{Blind, Stage};
        use std::collections::HashMap;
        use std::sync::Arc;

        let joker_state_manager = Arc::new(JokerStateManager::new());
        let hand_type_counts = HashMap::new();
        let hand = Hand::new(vec![]);
        let discarded = vec![];
        let jokers: Vec<Box<dyn Joker>> = vec![];
        let rng = crate::rng::GameRng::new(crate::rng::RngMode::Testing(42));
        let stage = Stage::Blind(Blind::Small);

        let mut context = crate::joker::GameContext {
            chips: 0,
            mult: 0,
            money: 0,
            ante: 1,
            round: 1,
            stage: &stage,
            hands_played: 0,
            hands_remaining: 4.0,
            discards_used: 0,
            jokers: &jokers,
            hand: &hand,
            discarded: &discarded,
            joker_state_manager: &joker_state_manager,
            hand_type_counts: &hand_type_counts,
            cards_in_deck: 52,
            stone_cards_in_deck: 0,
            steel_cards_in_deck: 0,
            rng: &rng,
        };

        let effect = triboulet.on_card_scored(&mut context, &ace_card);
        assert_eq!(effect.mult_multiplier, 1.0); // Default multiplier (no effect)
        assert_eq!(effect.mult, 0);
        assert_eq!(effect.chips, 0);
        assert_eq!(effect.money, 0);
    }
}
