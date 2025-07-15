use crate::action::{Action, MoveDirection};
use crate::ante::Ante;
use crate::available::Available;
use crate::card::Card;
use crate::config::Config;
use crate::deck::Deck;
use crate::effect::{EffectRegistry, Effects};
use crate::error::GameError;
use crate::hand::{MadeHand, SelectHand};
use crate::joker::{JokerId, Jokers, OldJoker as Joker};
use crate::joker_state::JokerStateManager;
use crate::rank::HandRank;
use crate::shop::Shop;
use crate::stage::{Blind, End, Stage};

use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Game {
    pub config: Config,
    pub shop: Shop,
    pub deck: Deck,
    pub available: Available,
    pub discarded: Vec<Card>,
    pub blind: Option<Blind>,
    pub stage: Stage,
    pub ante_start: Ante,
    pub ante_end: Ante,
    pub ante_current: Ante,
    pub action_history: Vec<Action>,
    pub round: usize,

    // jokers and their effects
    pub jokers: Vec<Jokers>,
    pub effect_registry: EffectRegistry,
    pub joker_state_manager: Arc<JokerStateManager>,

    // playing
    pub plays: usize,
    pub discards: usize,
    pub reward: usize,
    pub money: usize,

    // for scoring
    pub chips: usize,
    pub mult: usize,
    pub score: usize,

    // hand type tracking for this game run
    pub hand_type_counts: HashMap<HandRank, u32>,
}

impl Game {
    pub fn new(config: Config) -> Self {
        let ante_start = Ante::try_from(config.ante_start).unwrap_or(Ante::One);
        Self {
            shop: Shop::new(),
            deck: Deck::default(),
            available: Available::default(),
            discarded: Vec::new(),
            action_history: Vec::new(),
            jokers: Vec::new(),
            effect_registry: EffectRegistry::new(),
            joker_state_manager: Arc::new(JokerStateManager::new()),
            blind: None,
            stage: Stage::PreBlind(),
            ante_start,
            ante_end: Ante::try_from(config.ante_end).unwrap_or(Ante::Eight),
            ante_current: ante_start,
            round: config.round_start,
            plays: config.plays,
            discards: config.discards,
            reward: config.reward_base,
            money: config.money_start,
            chips: config.base_chips,
            mult: config.base_mult,
            score: config.base_score,
            hand_type_counts: HashMap::new(),
            config,
        }
    }

    pub fn start(&mut self) {
        // for now just move state to small blind
        self.stage = Stage::PreBlind();
        self.deal();
    }

    pub fn result(&self) -> Option<End> {
        match self.stage {
            Stage::End(end) => Some(end),
            _ => None,
        }
    }

    pub fn is_over(&self) -> bool {
        self.result().is_some()
    }

    /// Returns a reference to the joker at the specified slot, if it exists.
    ///
    /// # Arguments
    /// * `slot` - The zero-based index of the joker slot to check
    ///
    /// # Returns
    /// * `Some(&Jokers)` if a joker exists at the specified slot
    /// * `None` if the slot is empty or the index is out of bounds
    pub fn get_joker_at_slot(&self, slot: usize) -> Option<&Jokers> {
        self.jokers.get(slot)
    }

    /// Returns the total number of jokers currently owned by the player.
    ///
    /// # Returns
    /// The count of jokers in the player's collection
    pub fn joker_count(&self) -> usize {
        self.jokers.len()
    }

    /// Returns the number of times a specific hand type has been played this game run.
    ///
    /// # Arguments
    /// * `hand_rank` - The hand rank to check the count for
    ///
    /// # Returns
    /// The number of times this hand type has been played (0 if never played)
    pub fn get_hand_type_count(&self, hand_rank: HandRank) -> u32 {
        self.hand_type_counts.get(&hand_rank).copied().unwrap_or(0)
    }

    /// Increments the count for a specific hand type.
    ///
    /// # Arguments
    /// * `hand_rank` - The hand rank to increment
    pub fn increment_hand_type_count(&mut self, hand_rank: HandRank) {
        *self.hand_type_counts.entry(hand_rank).or_insert(0) += 1;
    }

    fn clear_blind(&mut self) {
        self.score = self.config.base_score;
        self.plays = self.config.plays;
        self.discards = self.config.discards;
        self.deal();
    }

    // draw from deck to available
    fn draw(&mut self, count: usize) {
        if let Some(drawn) = self.deck.draw(count) {
            self.available.extend(drawn);
            // self.available.extend(drawn);
        }
    }

    // shuffle and deal new cards to available
    pub(crate) fn deal(&mut self) {
        // add discarded back to deck, emptying in process
        self.deck.append(&mut self.discarded);
        // add available back to deck and empty
        self.deck.extend(self.available.cards());
        self.available.empty();
        self.deck.shuffle();
        self.draw(self.config.available);
    }

    pub(crate) fn select_card(&mut self, card: Card) -> Result<(), GameError> {
        if self.available.selected().len() > self.config.selected_max {
            return Err(GameError::InvalidSelectCard);
        }
        self.available.select_card(card)
    }

    pub(crate) fn move_card(
        &mut self,
        direction: MoveDirection,
        card: Card,
    ) -> Result<(), GameError> {
        self.available.move_card(direction, card)
    }

    pub(crate) fn play_selected(&mut self) -> Result<(), GameError> {
        if self.plays == 0 {
            return Err(GameError::NoRemainingPlays);
        }
        self.plays -= 1;
        let selected = SelectHand::new(self.available.selected());
        let best = selected.best_hand()?;

        // Track hand type for game statistics
        self.increment_hand_type_count(best.rank);

        let score = self.calc_score(best);
        let clear_blind = self.handle_score(score)?;
        self.discarded.extend(self.available.selected());
        let removed = self.available.remove_selected();
        self.draw(removed);
        if clear_blind {
            self.clear_blind();
        }
        Ok(())
    }

    // discard selected cards from available and draw equal number back to available
    pub(crate) fn discard_selected(&mut self) -> Result<(), GameError> {
        if self.discards == 0 {
            return Err(GameError::NoRemainingDiscards);
        }
        self.discards -= 1;
        self.discarded.extend(self.available.selected());
        let removed = self.available.remove_selected();
        self.draw(removed);
        Ok(())
    }

    pub(crate) fn calc_score(&mut self, hand: MadeHand) -> usize {
        // compute chips and mult from hand level
        self.chips += hand.rank.level().chips;
        self.mult += hand.rank.level().mult;

        // add chips for each played card
        let card_chips: usize = hand.hand.cards().iter().map(|c| c.chips()).sum();
        self.chips += card_chips;

        // Apply effects that modify game.chips and game.mult
        for e in self.effect_registry.on_score.clone() {
            if let Effects::OnScore(f) = e {
                f.lock().unwrap()(self, hand.clone())
            }
        }

        // compute score
        let score = self.chips * self.mult;

        // reset chips and mult
        self.mult = self.config.base_mult;
        self.chips = self.config.base_chips;
        score
    }

    pub fn required_score(&self) -> usize {
        let base = self.ante_current.base();

        match self.blind {
            None => base,
            Some(Blind::Small) => base,
            Some(Blind::Big) => (base as f32 * 1.5) as usize,
            Some(Blind::Boss) => base * 2,
        }
    }

    fn calc_reward(&mut self, blind: Blind) -> Result<usize, GameError> {
        let mut interest = (self.money as f32 * self.config.interest_rate).floor() as usize;
        if interest > self.config.interest_max {
            interest = self.config.interest_max
        }
        let base = blind.reward();
        let hand_bonus = self.plays * self.config.money_per_hand;
        let reward = base + interest + hand_bonus;
        Ok(reward)
    }

    fn cashout(&mut self) -> Result<(), GameError> {
        self.money += self.reward;
        self.reward = 0;
        self.stage = Stage::Shop();
        self.shop.refresh();
        Ok(())
    }

    #[allow(dead_code)] // Kept for backward compatibility
    pub(crate) fn buy_joker(&mut self, joker: Jokers) -> Result<(), GameError> {
        if self.stage != Stage::Shop() {
            return Err(GameError::InvalidStage);
        }
        if self.jokers.len() >= self.config.joker_slots {
            return Err(GameError::NoAvailableSlot);
        }
        if joker.cost() > self.money {
            return Err(GameError::InvalidBalance);
        }
        self.shop.buy_joker(&joker)?;
        self.money -= joker.cost();
        self.jokers.push(joker);
        self.effect_registry
            .register_jokers(self.jokers.clone(), &self.clone());
        Ok(())
    }

    /// Purchases a joker from the shop and places it at the specified slot.
    ///
    /// This method validates that the game is in the shop stage, the joker is available
    /// in the shop, the player has sufficient money, and the slot is valid. If all
    /// validations pass, it purchases the joker and inserts it at the specified position,
    /// shifting existing jokers to the right if necessary.
    ///
    /// # Arguments
    /// * `joker_id` - The identifier of the joker to purchase
    /// * `slot` - The zero-based index where to place the joker (0 to jokers.len())
    ///
    /// # Returns
    /// * `Ok(())` if the purchase was successful
    /// * `Err(GameError)` if the purchase failed due to validation errors
    ///
    /// # Errors
    /// * `InvalidStage` - Game is not in shop stage
    /// * `InvalidSlot` - Slot index is greater than current joker count
    /// * `NoAvailableSlot` - Joker limit reached and trying to add at the end
    /// * `JokerNotInShop` - Requested joker is not available in the shop
    /// * `InvalidBalance` - Player doesn't have enough money
    /// * `NoJokerMatch` - Joker found in shop but couldn't be matched (internal error)
    pub(crate) fn buy_joker_with_slot(
        &mut self,
        joker_id: JokerId,
        slot: usize,
    ) -> Result<(), GameError> {
        // Validate stage
        if self.stage != Stage::Shop() {
            return Err(GameError::InvalidStage);
        }

        // Validate slot index - must be within expanded joker slot limit
        if slot >= self.config.joker_slots {
            return Err(GameError::InvalidSlot);
        }

        // Check if we've reached the joker limit
        if self.jokers.len() >= self.config.joker_slots {
            return Err(GameError::NoAvailableSlot);
        }

        // Check if joker is available in shop
        if !self.shop.has_joker(joker_id) {
            return Err(GameError::JokerNotInShop);
        }

        // Find the matching Jokers enum from shop (temporary until shop uses JokerId)
        let joker = self
            .shop
            .jokers
            .iter()
            .find(|j| j.matches_joker_id(joker_id))
            .cloned()
            .ok_or(GameError::NoJokerMatch)?;

        // Check if player has enough money (use actual joker cost)
        if joker.cost() > self.money {
            return Err(GameError::InvalidBalance);
        }

        // Purchase joker from shop
        self.shop.buy_joker(&joker)?;

        // Deduct money
        self.money -= joker.cost();

        // Insert joker at specified slot, expanding vector if necessary
        if slot >= self.jokers.len() {
            // Resize vector to accommodate the slot, filling gaps with default joker
            use crate::joker::compat::TheJoker;
            let default_joker = Jokers::TheJoker(TheJoker {});
            self.jokers.resize(slot, default_joker);
            self.jokers.push(joker.clone());
        } else {
            self.jokers.insert(slot, joker.clone());
        }

        // Update effect registry
        self.effect_registry
            .register_jokers(self.jokers.clone(), &self.clone());

        Ok(())
    }

    fn select_blind(&mut self, blind: Blind) -> Result<(), GameError> {
        // can only set blind if stage is pre blind
        if self.stage != Stage::PreBlind() {
            return Err(GameError::InvalidStage);
        }
        // provided blind must be expected next blind
        if let Some(current) = self.blind {
            if blind != current.next() {
                return Err(GameError::InvalidBlind);
            }
        } else {
            // if game just started, blind will be None, in which case
            // we can only set it to small.
            if blind != Blind::Small {
                return Err(GameError::InvalidBlind);
            }
        }
        self.blind = Some(blind);
        self.stage = Stage::Blind(blind);
        self.deal();
        Ok(())
    }

    fn next_round(&mut self) -> Result<(), GameError> {
        self.stage = Stage::PreBlind();
        self.round += 1;
        Ok(())
    }

    // Returns true if should clear blind after, false if not.
    fn handle_score(&mut self, score: usize) -> Result<bool, GameError> {
        // can only handle score if stage is blind
        if !self.stage.is_blind() {
            return Err(GameError::InvalidStage);
        }

        self.score += score;
        let required = self.required_score();

        // blind not passed
        if self.score < required {
            // no more hands to play -> lose
            if self.plays == 0 {
                self.stage = Stage::End(End::Lose);
                return Ok(false);
            } else {
                // more hands to play, carry on
                return Ok(false);
            }
        }

        let blind = self.blind.expect("stage is blind");
        // score exceeds blind (blind passed).
        // handle reward then progress to next stage.
        let reward = self.calc_reward(blind)?;
        self.reward = reward;

        // passed boss blind, either win or progress ante
        if blind == Blind::Boss {
            if let Some(ante_next) = self.ante_current.next(self.ante_end) {
                self.ante_current = ante_next;
            } else {
                self.stage = Stage::End(End::Win);
                return Ok(false);
            }
        };

        // finish blind, proceed to post blind
        self.stage = Stage::PostBlind();
        Ok(true)
    }

    pub fn handle_action(&mut self, action: Action) -> Result<(), GameError> {
        self.action_history.push(action.clone());
        match action {
            Action::SelectCard(card) => match self.stage.is_blind() {
                true => self.select_card(card),
                false => Err(GameError::InvalidAction),
            },
            Action::Play() => match self.stage.is_blind() {
                true => self.play_selected(),
                false => Err(GameError::InvalidAction),
            },
            Action::Discard() => match self.stage.is_blind() {
                true => self.discard_selected(),
                false => Err(GameError::InvalidAction),
            },
            Action::MoveCard(dir, card) => match self.stage.is_blind() {
                true => self.move_card(dir, card),
                false => Err(GameError::InvalidAction),
            },
            Action::CashOut(_reward) => match self.stage {
                Stage::PostBlind() => self.cashout(),
                _ => Err(GameError::InvalidAction),
            },
            Action::BuyJoker { joker_id, slot } => match self.stage {
                Stage::Shop() => self.buy_joker_with_slot(joker_id, slot),
                _ => Err(GameError::InvalidStage),
            },
            Action::NextRound() => match self.stage {
                Stage::Shop() => self.next_round(),
                _ => Err(GameError::InvalidAction),
            },
            Action::SelectBlind(blind) => match self.stage {
                Stage::PreBlind() => self.select_blind(blind),
                _ => Err(GameError::InvalidAction),
            },
        }
    }

    pub fn handle_action_index(&mut self, index: usize) -> Result<(), GameError> {
        let space = self.gen_action_space();
        let action = space.to_action(index, self)?;
        self.handle_action(action)
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "deck length: {}", self.deck.len())?;
        writeln!(f, "available length: {}", self.available.cards().len())?;
        writeln!(f, "selected length: {}", self.available.selected().len())?;
        writeln!(f, "discard length: {}", self.discarded.len())?;
        writeln!(f, "jokers: ")?;
        for j in self.jokers.clone() {
            writeln!(f, "{j}")?
        }
        writeln!(f, "action history length: {}", self.action_history.len())?;
        writeln!(f, "blind: {:?}", self.blind)?;
        writeln!(f, "stage: {:?}", self.stage)?;
        writeln!(f, "ante: {:?}", self.ante_current)?;
        writeln!(f, "round: {}", self.round)?;
        writeln!(f, "hands remaining: {}", self.plays)?;
        writeln!(f, "discards remaining: {}", self.discards)?;
        writeln!(f, "money: {}", self.money)?;
        writeln!(f, "score: {}", self.score)
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new(Config::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::{Suit, Value};
    use crate::joker::JokerId;

    #[test]
    fn test_constructor() {
        let g = Game::default();
        assert_eq!(g.available.cards().len(), 0);
        assert_eq!(g.deck.len(), 52);
        assert_eq!(g.mult, 0);
    }

    #[test]
    fn test_deal() {
        let mut g = Game::default();
        g.deal();
        // deck should be 7 cards smaller than we started with
        assert_eq!(g.deck.len(), 52 - g.config.available);
        // should be 7 cards now available
        assert_eq!(g.available.cards().len(), g.config.available);
    }

    #[test]
    fn test_draw() {
        let mut g = Game::default();
        g.draw(1);
        assert_eq!(g.available.cards().len(), 1);
        assert_eq!(g.deck.len(), 52 - 1);
        g.draw(3);
        assert_eq!(g.available.cards().len(), 4);
        assert_eq!(g.deck.len(), 52 - 4);
    }
    #[test]
    fn test_discard() {
        let mut g = Game::default();
        g.deal();
        assert_eq!(g.available.cards().len(), g.config.available);
        assert_eq!(g.deck.len(), 52 - g.config.available);
        // select first 4 cards
        for c in g.available.cards()[0..5].to_vec() {
            g.select_card(c).unwrap();
        }
        let discard_res = g.discard_selected();
        assert!(discard_res.is_ok());
        // available should still be 7, we discarded then redrew to match
        assert_eq!(g.available.cards().len(), g.config.available);
        // deck is now smaller since we drew from it
        assert_eq!(g.deck.len(), 52 - g.config.available - 5);
    }

    #[test]
    fn test_calc_score() {
        let mut g = Game::default();
        let ace = Card::new(Value::Ace, Suit::Heart);
        let king = Card::new(Value::King, Suit::Diamond);
        let jack = Card::new(Value::Jack, Suit::Club);

        // Score [Ah, Kd, Jc]
        // High card (level 1) -> chips=5, mult=1
        // Played cards (1 ace) -> 11 chips
        // (5 + 11) * 1 = 16
        let cards = vec![ace, king, jack];
        let hand = SelectHand::new(cards).best_hand().unwrap();
        let score = g.calc_score(hand);
        assert_eq!(score, 16);

        // Score [Kd, Kd, Ah]
        // Pair (level 1) -> chips=10, mult=2
        // Played cards (2 kings) -> 10 + 10 == 20 chips
        // (10 + 20) * 2 = 60
        let cards = vec![king, king, ace];
        let hand = SelectHand::new(cards).best_hand().unwrap();
        let score = g.calc_score(hand);
        assert_eq!(score, 60);

        // Score [Ah, Ah, Ah, Kd]
        // Three of kind (level 1) -> chips=30, mult=3
        // Played cards (3 aces) -> 11 + 11 + 11 == 33 chips
        // (30 + 33) * 3 = 189
        let cards = vec![ace, ace, ace, king];
        let hand = SelectHand::new(cards).best_hand().unwrap();
        let score = g.calc_score(hand);
        assert_eq!(score, 189);

        // Score [Kd, Kd, Kd, Kd, Ah]
        // Four of kind (level 1) -> chips=60, mult=7
        // Played cards (4 kings) -> 10 + 10 + 10 + 10 == 40 chips
        // (60 + 40) * 7 = 700
        let cards = vec![king, king, king, king, ace];
        let hand = SelectHand::new(cards).best_hand().unwrap();
        let score = g.calc_score(hand);
        assert_eq!(score, 700);

        // Score [Jc, Jc, Jc, Jc, Jc]
        // Flush five (level 1) -> chips=160, mult=16
        // Played cards (5 jacks) -> 10 + 10 + 10 + 10 + 10 == 50 chips
        // (160 + 50) * 16 = 3360
        let cards = vec![jack, jack, jack, jack, jack];
        let hand = SelectHand::new(cards).best_hand().unwrap();
        let score = g.calc_score(hand);
        assert_eq!(score, 3360);
    }

    #[test]
    fn test_handle_score() {
        let mut g = Game::default();
        g.start();
        g.stage = Stage::Blind(Blind::Small);
        g.blind = Some(Blind::Small);

        // Not enough to pass
        let required = g.required_score();
        let score = required - 1;

        let passed = g.handle_score(score).unwrap();
        assert!(!passed);
        assert_eq!(g.score, score);

        // Enough to pass now
        let passed = g.handle_score(1).unwrap();
        assert!(passed);
        assert_eq!(g.score, required);
        assert_eq!(g.stage, Stage::PostBlind());
    }

    #[test]
    fn test_clear_blind() {
        let mut g = Game::default();
        g.start();
        g.deal();
        g.clear_blind();
        // deck should be 7 cards smaller than we started with
        assert_eq!(g.deck.len(), 52 - g.config.available);
        // should be 7 cards now available
        assert_eq!(g.available.cards().len(), g.config.available);
    }

    #[test]
    fn test_play_selected() {
        let mut g = Game::default();
        g.start();
        g.stage = Stage::Blind(Blind::Small);
        g.blind = Some(Blind::Small);
        for card in g.available.cards().iter().take(5) {
            g.available.select_card(*card).expect("can select card");
        }

        assert_eq!(g.available.selected().len(), 5);
        // Artifically set score so blind passes
        g.score += g.required_score();
        g.play_selected().expect("can play selected");

        // Should have cleared blind
        assert_eq!(g.stage, Stage::PostBlind());
        // Score should reset to 0
        assert_eq!(g.score, g.config.base_score);
        // Plays and discards should reset
        assert_eq!(g.plays, g.config.plays);
        assert_eq!(g.discards, g.config.discards);
        // Deck should be length 52 - available
        assert_eq!(g.deck.len(), 52 - g.config.available);
        // Discarded should be length 0
        assert_eq!(g.discarded.len(), 0);
        // Available should be length available
        assert_eq!(g.available.cards().len(), g.config.available);
    }

    #[test]
    fn test_buy_joker() {
        let mut g = Game::default();
        g.start();
        g.stage = Stage::Shop();
        g.money = 10;
        g.shop.refresh();

        let j1 = g.shop.joker_from_index(0).expect("is joker");
        g.buy_joker(j1.clone()).expect("buy joker");
        assert_eq!(g.money, 10 - j1.cost());
        assert_eq!(g.jokers.len(), 1);
    }

    #[test]
    fn test_buy_joker_with_slot_specification() {
        let mut game = Game::default();
        game.start();
        game.stage = Stage::Shop();
        game.money = 20;

        // Set up shop with known jokers for deterministic testing
        use crate::joker::compat::TheJoker;
        game.shop.jokers = vec![Jokers::TheJoker(TheJoker {})];

        // Test buying a joker in a specific slot
        let action = Action::BuyJoker {
            joker_id: JokerId::Joker,
            slot: 0,
        };

        let result = game.handle_action(action);
        assert!(result.is_ok());

        // Verify joker is in the correct slot
        assert!(game.get_joker_at_slot(0).is_some());
        assert!(matches!(
            game.get_joker_at_slot(0),
            Some(Jokers::TheJoker(_))
        ));
        assert_eq!(game.joker_count(), 1);
    }

    #[test]
    fn test_buy_joker_insert_at_position() {
        let mut game = Game::default();
        game.start();
        game.stage = Stage::Shop();
        game.money = 40;

        // Set up shop with known jokers for deterministic testing
        use crate::joker::compat::{GreedyJoker, TheJoker};
        game.shop.jokers = vec![
            Jokers::TheJoker(TheJoker {}),
            Jokers::GreedyJoker(GreedyJoker {}),
        ];

        // Buy first joker at end (slot 0)
        let action1 = Action::BuyJoker {
            joker_id: JokerId::Joker,
            slot: 0,
        };
        game.handle_action(action1).unwrap();

        // Buy another joker at position 0 (should push first joker to position 1)
        let action2 = Action::BuyJoker {
            joker_id: JokerId::GreedyJoker,
            slot: 0,
        };
        let result = game.handle_action(action2);

        assert!(result.is_ok());
        assert_eq!(game.joker_count(), 2);
        // GreedyJoker should be at position 0
        assert!(matches!(
            game.get_joker_at_slot(0),
            Some(Jokers::GreedyJoker(_))
        ));
        // TheJoker should have moved to position 1
        assert!(matches!(
            game.get_joker_at_slot(1),
            Some(Jokers::TheJoker(_))
        ));
    }

    #[test]
    fn test_buy_joker_invalid_slot() {
        let mut game = Game::default();
        game.start();
        game.stage = Stage::Shop();
        game.money = 20;
        game.shop.refresh();

        // Test buying in slot beyond limit (default is 5 slots, so 0-4 are valid)
        let action = Action::BuyJoker {
            joker_id: JokerId::Joker,
            slot: 5,
        };

        let result = game.handle_action(action);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), GameError::InvalidSlot));
    }

    #[test]
    fn test_buy_joker_expanded_slots() {
        let mut game = Game::default();
        game.start();
        game.stage = Stage::Shop();
        game.money = 20;

        // Set up shop with known jokers for deterministic testing
        use crate::joker::compat::TheJoker;
        game.shop.jokers = vec![Jokers::TheJoker(TheJoker {})];

        // Simulate having voucher that expands slots to 10
        game.config.joker_slots = 10;

        // Now slot 5 should be valid
        let action = Action::BuyJoker {
            joker_id: JokerId::Joker,
            slot: 5,
        };

        let result = game.handle_action(action);
        assert!(result.is_ok());
        assert!(matches!(
            game.get_joker_at_slot(5),
            Some(Jokers::TheJoker(_))
        ));
    }

    #[test]
    fn test_buy_joker_insufficient_money() {
        let mut game = Game::default();
        game.start();
        game.stage = Stage::Shop();
        game.money = 1; // Not enough for any joker

        // Set up shop with known jokers for deterministic testing
        use crate::joker::compat::TheJoker;
        game.shop.jokers = vec![Jokers::TheJoker(TheJoker {})];

        let action = Action::BuyJoker {
            joker_id: JokerId::Joker,
            slot: 0,
        };

        let result = game.handle_action(action);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), GameError::InvalidBalance));
    }

    #[test]
    fn test_buy_joker_not_in_shop() {
        let mut game = Game::default();
        game.start();
        game.stage = Stage::Shop();
        game.money = 20;
        game.shop.refresh();

        // Try to buy a joker that's not currently in the shop
        let action = Action::BuyJoker {
            joker_id: JokerId::CavendishJoker, // Unlikely to be in shop
            slot: 0,
        };

        let result = game.handle_action(action);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), GameError::JokerNotInShop));
    }

    #[test]
    fn test_buy_joker_wrong_stage() {
        let mut game = Game::default();
        game.start();
        game.stage = Stage::Blind(Blind::Small);
        game.money = 20;

        let action = Action::BuyJoker {
            joker_id: JokerId::Joker,
            slot: 0,
        };

        let result = game.handle_action(action);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), GameError::InvalidStage));
    }
}
