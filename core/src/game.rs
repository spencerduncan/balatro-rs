use crate::action::{Action, MoveDirection};
use crate::ante::Ante;
use crate::available::Available;
use crate::boss_blinds::BossBlindState;
use crate::card::Card;
use crate::config::Config;
use crate::consumables::ConsumableId;
use crate::deck::Deck;
use crate::error::GameError;
use crate::hand::{MadeHand, SelectHand};
use crate::joker::{GameContext, Joker, JokerId, Jokers, OldJoker as OldJokerTrait};
use crate::joker_factory::JokerFactory;
use crate::joker_state::{JokerState, JokerStateManager};
use crate::rank::HandRank;
use crate::shop::packs::{OpenPackState, Pack};
use crate::shop::Shop;
use crate::stage::{Blind, End, Stage};
use crate::state_version::StateVersion;
use crate::vouchers::VoucherCollection;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
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

    // jokers using structured JokerEffect system
    #[cfg_attr(feature = "serde", serde(skip))]
    pub jokers: Vec<Box<dyn Joker>>,

    #[cfg_attr(feature = "serde", serde(skip, default = "JokerEffectProcessor::new"))]
    pub joker_effect_processor: JokerEffectProcessor,

    #[cfg_attr(
        feature = "serde",
        serde(skip, default = "default_joker_state_manager")
    )]
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

    // Extended state for consumables, vouchers, and boss blinds
    /// Consumable cards currently in the player's hand
    pub consumables_in_hand: Vec<ConsumableId>,

    /// Collection of owned vouchers with purchase tracking
    pub vouchers: VoucherCollection,

    /// Current boss blind state and effects
    pub boss_blind_state: BossBlindState,

    /// Pack system state
    /// Packs currently in the player's inventory
    pub pack_inventory: Vec<Pack>,

    /// Currently opened pack that player is choosing from
    pub open_pack: Option<OpenPackState>,

    /// Version of the game state for serialization compatibility
    pub state_version: StateVersion,
}

#[cfg(feature = "serde")]
fn default_joker_state_manager() -> Arc<JokerStateManager> {
    Arc::new(JokerStateManager::new())
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

            // Initialize extended state fields
            consumables_in_hand: Vec::new(),
            vouchers: VoucherCollection::new(),
            boss_blind_state: BossBlindState::new(),

            // Initialize pack system fields
            pack_inventory: Vec::new(),
            open_pack: None,

            state_version: StateVersion::current(),

            config,
        }
    }

    pub fn start(&mut self) {
        // for now just move state to small blind
        self.stage = Stage::PreBlind();
        self.deal();
    }

    /// Start a new blind and trigger joker lifecycle events
    pub fn start_blind(&mut self) {
        use crate::hand::Hand;

        // Set stage to blind
        self.stage = Stage::Blind(Blind::Small);
        self.blind = Some(Blind::Small);

        // Trigger on_blind_start for all jokers
        for joker in &self.jokers {
            let temp_hand = Hand::new(self.available.cards());
            let mut context = GameContext {
                chips: self.chips as i32,
                mult: self.mult as i32,
                money: self.money as i32,
                ante: self.ante_current as u8,
                round: self.round as u32,
                stage: &self.stage,
                hands_played: 0,
                discards_used: 0,
                jokers: &self.jokers,
                hand: &temp_hand,
                discarded: &self.discarded,
                joker_state_manager: &self.joker_state_manager,
                hand_type_counts: &self.hand_type_counts,
            };

            let effect = joker.on_blind_start(&mut context);

            // Apply effects immediately
            self.chips += effect.chips as usize;
            self.mult += effect.mult as usize;
            self.money += effect.money as usize;
        }
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
    /// * `Some(&dyn Joker)` if a joker exists at the specified slot
    /// * `None` if the slot is empty or the index is out of bounds
    pub fn get_joker_at_slot(&self, slot: usize) -> Option<&dyn Joker> {
        self.jokers.get(slot).map(|j| j.as_ref())
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

    pub fn calc_score(&mut self, hand: MadeHand) -> usize {
        // compute chips and mult from hand level
        self.chips += hand.rank.level().chips;
        self.mult += hand.rank.level().mult;

        // add chips for each played card
        let card_chips: usize = hand.hand.cards().iter().map(|c| c.chips()).sum();
        self.chips += card_chips;

        // Apply JokerEffect from structured joker system
        if !self.jokers.is_empty() {
            let (joker_chips, joker_mult, joker_money, _messages) =
                self.process_joker_effects(&hand);
            self.chips += joker_chips as usize;
            self.mult += joker_mult as usize;
            self.money += joker_money as usize;
        }

        // compute score
        let score = self.chips * self.mult;

        // reset chips and mult
        self.mult = self.config.base_mult;
        self.chips = self.config.base_chips;
        score
    }

    /// Process JokerEffect from all jokers and return accumulated effects
    fn process_joker_effects(&mut self, hand: &MadeHand) -> (i32, i32, i32, Vec<String>) {
        use crate::hand::Hand;

        let mut total_chips = 0i32;
        let mut total_mult = 0i32;
        let mut total_money = 0i32;
        let mut messages = Vec::new();
        let mut total_mult_multiplier = 1.0f32;

        // Create game context
        let mut context = GameContext {
            chips: self.chips as i32,
            mult: self.mult as i32,
            money: self.money as i32,
            ante: self.ante_current as u8,
            round: self.round as u32,
            stage: &self.stage,
            hands_played: 0,  // TODO: track this properly
            discards_used: 0, // TODO: track this properly
            jokers: &self.jokers,
            hand: &Hand::new(hand.hand.cards().to_vec()),
            discarded: &self.discarded,
            joker_state_manager: &self.joker_state_manager,
            hand_type_counts: &self.hand_type_counts,
        };

        // Process hand-level effects first
        for joker in &self.jokers {
            let select_hand = SelectHand::new(hand.hand.cards().to_vec());
            let effect = joker.on_hand_played(&mut context, &select_hand);

            total_chips += effect.chips;
            total_mult += effect.mult;
            total_money += effect.money;

            // Handle mult_multiplier: 0.0 means no multiplier, so treat as 1.0
            if effect.mult_multiplier != 0.0 {
                total_mult_multiplier *= effect.mult_multiplier;
            }

            if let Some(msg) = effect.message {
                messages.push(msg);
            }
        }

        // Process card-level effects
        for card in hand.hand.cards() {
            for joker in &self.jokers {
                let effect = joker.on_card_scored(&mut context, &card);

                total_chips += effect.chips;
                total_mult += effect.mult;
                total_money += effect.money;

                // Handle mult_multiplier: 0.0 means no multiplier, so treat as 1.0
                if effect.mult_multiplier != 0.0 {
                    total_mult_multiplier *= effect.mult_multiplier;
                }

                if let Some(msg) = effect.message {
                    messages.push(msg);
                }
            }
        }

        // Apply mult multiplier to the total mult bonus (not base mult)
        if total_mult_multiplier != 1.0 {
            total_mult = (total_mult as f32 * total_mult_multiplier) as i32;
        }

        (total_chips, total_mult, total_money, messages)
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
        // Convert old joker to new system and add to jokers vec
        if let Some(new_joker) = JokerFactory::create(joker.to_joker_id()) {
            self.shop.buy_joker(&joker)?;
            self.money -= joker.cost();
            self.jokers.push(new_joker);
            Ok(())
        } else {
            Err(GameError::InvalidOperation(format!(
                "Cannot create joker {:?} - not available in new system",
                joker.to_joker_id()
            )))
        }
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
        let shop_joker = self
            .shop
            .jokers
            .iter()
            .find(|j| j.matches_joker_id(joker_id))
            .cloned()
            .ok_or(GameError::NoJokerMatch)?;

        // Check if player has enough money (use actual joker cost)
        if shop_joker.cost() > self.money {
            return Err(GameError::InvalidBalance);
        }

        // Create new joker using JokerFactory
        let new_joker = JokerFactory::create(joker_id).ok_or_else(|| {
            GameError::InvalidOperation(format!(
                "Cannot create joker {joker_id:?} - not available in new system"
            ))
        })?;

        // Purchase joker from shop
        self.shop.buy_joker(&shop_joker)?;

        // Deduct money
        self.money -= shop_joker.cost();

        // Insert joker at specified slot, expanding vector if necessary
        if slot >= self.jokers.len() {
            // For simplicity, just push at the end if slot is beyond current length
            self.jokers.push(new_joker);
        } else {
            self.jokers.insert(slot, new_joker);
        }

        Ok(())
    }

    /// Pack System Methods
    /// Buy a pack of the specified type
    pub(crate) fn buy_pack(
        &mut self,
        pack_type: crate::shop::packs::PackType,
    ) -> Result<(), GameError> {
        use crate::shop::packs::{DefaultPackGenerator, PackGenerator};

        // Check if player has enough money
        let cost = pack_type.base_cost();
        if self.money < cost {
            return Err(GameError::InvalidBalance);
        }

        // Generate the pack
        let generator = DefaultPackGenerator;
        let mut pack = generator.generate_pack(pack_type, self)?;
        pack.generate_contents(self)?;

        // Deduct money
        self.money -= cost;

        // Add pack to inventory
        self.pack_inventory.push(pack);

        Ok(())
    }

    /// Open a pack from inventory
    pub(crate) fn open_pack(&mut self, pack_id: usize) -> Result<(), GameError> {
        // Check if pack exists in inventory
        if pack_id >= self.pack_inventory.len() {
            return Err(GameError::InvalidAction);
        }

        // Check if another pack is already open
        if self.open_pack.is_some() {
            return Err(GameError::InvalidAction);
        }

        // Remove pack from inventory and open it
        let pack = self.pack_inventory.remove(pack_id);
        self.open_pack = Some(OpenPackState::new(pack, pack_id));

        Ok(())
    }

    /// Select an option from the currently opened pack
    pub(crate) fn select_from_pack(
        &mut self,
        pack_id: usize,
        option_index: usize,
    ) -> Result<(), GameError> {
        // Check if a pack is open
        let open_pack_state = self.open_pack.take().ok_or(GameError::InvalidAction)?;

        // Verify pack ID matches
        if open_pack_state.pack_id != pack_id {
            return Err(GameError::InvalidAction);
        }

        // Select the option
        let selected_item = open_pack_state.pack.select_option(option_index)?;

        // Process the selected item based on its type
        self.process_pack_item(selected_item)?;

        Ok(())
    }

    /// Skip the currently opened pack
    pub(crate) fn skip_pack(&mut self, pack_id: usize) -> Result<(), GameError> {
        // Check if a pack is open
        let open_pack_state = self.open_pack.take().ok_or(GameError::InvalidAction)?;

        // Verify pack ID matches
        if open_pack_state.pack_id != pack_id {
            return Err(GameError::InvalidAction);
        }

        // Check if pack can be skipped
        if !open_pack_state.pack.can_skip {
            return Err(GameError::InvalidAction);
        }

        // Pack is simply consumed (no further action needed)
        Ok(())
    }

    /// Process an item selected from a pack
    fn process_pack_item(&mut self, item: crate::shop::ShopItem) -> Result<(), GameError> {
        use crate::shop::ShopItem;

        match item {
            ShopItem::PlayingCard(card) => {
                // Add card to deck
                self.deck.extend(vec![card]);
                Ok(())
            }
            ShopItem::Joker(joker_id) => {
                // Use JokerFactory to create the joker
                if let Some(joker) = JokerFactory::create(joker_id) {
                    self.jokers.push(joker);
                    // Initialize state for the new joker
                    self.joker_state_manager.ensure_state_exists(joker_id);
                    Ok(())
                } else {
                    // If we can't create the joker, return an error
                    Err(GameError::InvalidAction)
                }
            }
            ShopItem::Consumable(consumable_type) => {
                use rand::seq::SliceRandom;

                // Select a random consumable of the appropriate type
                let consumable_id = match consumable_type {
                    crate::shop::ConsumableType::Tarot => {
                        let tarot_cards = ConsumableId::tarot_cards();
                        tarot_cards
                            .choose(&mut rand::thread_rng())
                            .copied()
                            .unwrap_or(ConsumableId::TheFool)
                    }
                    crate::shop::ConsumableType::Planet => {
                        let planet_cards = ConsumableId::planet_cards();
                        planet_cards
                            .choose(&mut rand::thread_rng())
                            .copied()
                            .unwrap_or(ConsumableId::Mercury)
                    }
                    crate::shop::ConsumableType::Spectral => {
                        let spectral_cards = ConsumableId::spectral_cards();
                        spectral_cards
                            .choose(&mut rand::thread_rng())
                            .copied()
                            .unwrap_or(ConsumableId::Familiar)
                    }
                };

                // Add consumable to hand
                self.consumables_in_hand.push(consumable_id);
                Ok(())
            }
            _ => {
                // Other item types not yet implemented
                Ok(())
            }
        }
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
            Action::BuyPack { pack_type } => match self.stage {
                Stage::Shop() => self.buy_pack(pack_type),
                _ => Err(GameError::InvalidStage),
            },
            Action::OpenPack { pack_id } => self.open_pack(pack_id),
            Action::SelectFromPack {
                pack_id,
                option_index,
            } => self.select_from_pack(pack_id, option_index),
            Action::SkipPack { pack_id } => self.skip_pack(pack_id),
        }
    }

    pub fn handle_action_index(&mut self, index: usize) -> Result<(), GameError> {
        let space = self.gen_action_space();
        let action = space.to_action(index, self)?;
        self.handle_action(action)
    }

    /// Remove a joker from the specified slot and clean up its state.
    ///
    /// # Arguments
    /// * `slot` - The zero-based index of the joker slot to remove from
    ///
    /// # Returns
    /// * `Ok(())` if the joker was successfully removed
    /// * `Err(GameError::InvalidSlot)` if the slot index is out of bounds
    pub fn remove_joker(&mut self, slot: usize) -> Result<(), crate::error::GameError> {
        use crate::error::GameError;

        if slot >= self.jokers.len() {
            return Err(GameError::InvalidSlot);
        }

        // Get the joker before removing it to clean up its state
        let joker = &self.jokers[slot];
        let joker_id = joker.id();

        // Remove the joker from the collection
        self.jokers.remove(slot);

        // Clean up the joker's state
        self.joker_state_manager.remove_state(joker_id);

        Ok(())
    }

    /// Sell a joker from the specified slot, awarding money and cleaning up its state.
    ///
    /// # Arguments
    /// * `slot` - The zero-based index of the joker slot to sell
    ///
    /// # Returns
    /// * `Ok(())` if the joker was successfully sold
    /// * `Err(GameError::InvalidSlot)` if the slot index is out of bounds
    pub fn sell_joker(&mut self, slot: usize) -> Result<(), crate::error::GameError> {
        use crate::error::GameError;

        if slot >= self.jokers.len() {
            return Err(GameError::InvalidSlot);
        }

        // Get sell value and joker ID before removing
        let joker = &self.jokers[slot];
        let sell_value = joker.cost() / 2; // Standard sell value is half the cost
        let joker_id = joker.id();

        // Award money for selling the joker
        self.money += sell_value;

        // Remove the joker from the collection
        self.jokers.remove(slot);

        // Clean up the joker's state
        self.joker_state_manager.remove_state(joker_id);

        Ok(())
    }

    /// Validate that joker state is consistent with actual jokers in play.
    ///
    /// # Returns
    /// * `Ok(())` if the state is consistent
    /// * `Err(GameError::InvalidOperation)` if inconsistencies are found
    pub fn validate_joker_state(&self) -> Result<(), crate::error::GameError> {
        use crate::error::GameError;

        // Get all joker IDs currently in play
        let current_jokers: std::collections::HashSet<_> =
            self.jokers.iter().map(|joker| joker.id()).collect();

        // Get all joker IDs with state
        let state_jokers: std::collections::HashSet<_> = self
            .joker_state_manager
            .get_active_jokers()
            .into_iter()
            .collect();

        // Check for state without corresponding jokers
        for state_joker in &state_jokers {
            if !current_jokers.contains(state_joker) {
                return Err(GameError::InvalidOperation(format!(
                    "Found state for joker {state_joker:?} but no corresponding joker in play"
                )));
            }
        }

        Ok(())
    }

    /// Clean up orphaned joker state (state for jokers no longer in play).
    pub fn cleanup_joker_state(&mut self) {
        // Get all joker IDs currently in play
        let current_jokers: std::collections::HashSet<_> =
            self.jokers.iter().map(|joker| joker.id()).collect();

        // Get all joker IDs with state
        let state_jokers: Vec<_> = self
            .joker_state_manager
            .get_active_jokers()
            .into_iter()
            .collect();

        // Remove state for jokers no longer in play
        for state_joker in state_jokers {
            if !current_jokers.contains(&state_joker) {
                self.joker_state_manager.remove_state(state_joker);
            }
        }
    }

    /// Reset the game to its initial state, clearing all jokers and their state.
    pub fn reset_game(&mut self) {
        // Clear all jokers
        self.jokers.clear();

        // Clear all joker state
        self.joker_state_manager.clear();

        // Reset other game state to initial values
        self.round = self.config.round_start;
        self.plays = self.config.plays;
        self.discards = self.config.discards;
        self.money = self.config.money_start;
        self.chips = self.config.base_chips;
        self.mult = self.config.base_mult;
        self.score = self.config.base_score;
        self.ante_current = self.ante_start;
        self.stage = Stage::PreBlind();
        self.hand_type_counts.clear();
        self.action_history.clear();
        self.discarded.clear();

        // Reset deck and available cards
        self.deck = crate::deck::Deck::default();
        self.available = crate::available::Available::default();
        self.blind = None;
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "deck length: {}", self.deck.len())?;
        writeln!(f, "available length: {}", self.available.cards().len())?;
        writeln!(f, "selected length: {}", self.available.selected().len())?;
        writeln!(f, "discard length: {}", self.discarded.len())?;
        writeln!(f, "jokers: ")?;
        for j in &self.jokers {
            writeln!(f, "{j:?}")?
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

/// Serializable representation of game state, excluding non-serializable fields
#[derive(Debug, Clone, Serialize, Deserialize)]
struct SaveableGameState {
    pub version: u32,
    pub timestamp: u64,
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
    pub joker_ids: Vec<JokerId>, // Changed from jokers: Vec<Jokers> to support new system
    pub joker_states: HashMap<JokerId, JokerState>,
    pub plays: usize,
    pub discards: usize,
    pub reward: usize,
    pub money: usize,
    pub chips: usize,
    pub mult: usize,
    pub score: usize,
    pub hand_type_counts: HashMap<HandRank, u32>,
    // Extended state fields
    pub consumables_in_hand: Vec<ConsumableId>,
    pub vouchers: VoucherCollection,
    pub boss_blind_state: BossBlindState,
    pub pack_inventory: Vec<Pack>,
    pub open_pack: Option<OpenPackState>,
    pub state_version: StateVersion,
}

const SAVE_VERSION: u32 = 1;

/// Errors that can occur during save/load operations
#[derive(Debug)]
pub enum SaveLoadError {
    SerializationError(serde_json::Error),
    DeserializationError(serde_json::Error),
    InvalidVersion(u32),
    MissingField(String),
    ValidationError(String),
}

impl fmt::Display for SaveLoadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SaveLoadError::SerializationError(e) => write!(f, "Serialization error: {e}"),
            SaveLoadError::DeserializationError(e) => write!(f, "Deserialization error: {e}"),
            SaveLoadError::InvalidVersion(v) => write!(f, "Unsupported save version: {v}"),
            SaveLoadError::MissingField(field) => write!(f, "Missing required field: {field}"),
            SaveLoadError::ValidationError(msg) => write!(f, "Validation error: {msg}"),
        }
    }
}

impl std::error::Error for SaveLoadError {}

impl Game {
    /// Save the current game state to JSON string
    pub fn save_state_to_json(&self) -> Result<String, SaveLoadError> {
        // Extract joker states from the state manager
        let joker_states = self.joker_state_manager.snapshot_all();

        // Extract joker IDs from the new joker system
        let joker_ids: Vec<JokerId> = self.jokers.iter().map(|j| j.id()).collect();

        let saveable_state = SaveableGameState {
            version: SAVE_VERSION,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            config: self.config.clone(),
            shop: self.shop.clone(),
            deck: self.deck.clone(),
            available: self.available.clone(),
            discarded: self.discarded.clone(),
            blind: self.blind,
            stage: self.stage,
            ante_start: self.ante_start,
            ante_end: self.ante_end,
            ante_current: self.ante_current,
            action_history: self.action_history.clone(),
            round: self.round,
            joker_ids,
            joker_states,
            plays: self.plays,
            discards: self.discards,
            reward: self.reward,
            money: self.money,
            chips: self.chips,
            mult: self.mult,
            score: self.score,
            hand_type_counts: self.hand_type_counts.clone(),
            // Extended state fields
            consumables_in_hand: self.consumables_in_hand.clone(),
            vouchers: self.vouchers.clone(),
            boss_blind_state: self.boss_blind_state.clone(),
            pack_inventory: self.pack_inventory.clone(),
            open_pack: self.open_pack.clone(),
            state_version: self.state_version,
        };

        serde_json::to_string_pretty(&saveable_state).map_err(SaveLoadError::SerializationError)
    }

    /// Load game state from JSON string
    pub fn load_state_from_json(json: &str) -> Result<Self, SaveLoadError> {
        let saveable_state: SaveableGameState =
            serde_json::from_str(json).map_err(SaveLoadError::DeserializationError)?;

        // Validate version
        if saveable_state.version > SAVE_VERSION {
            return Err(SaveLoadError::InvalidVersion(saveable_state.version));
        }

        // Recreate jokers using JokerFactory
        let jokers: Vec<Box<dyn Joker>> = saveable_state
            .joker_ids
            .into_iter()
            .filter_map(|id| JokerFactory::create(id))
            .collect();

        // Create joker state manager
        let joker_state_manager = Arc::new(JokerStateManager::new());

        // Create new game instance with reconstructed state
        let game = Game {
            config: saveable_state.config,
            shop: saveable_state.shop,
            deck: saveable_state.deck,
            available: saveable_state.available,
            discarded: saveable_state.discarded,
            blind: saveable_state.blind,
            stage: saveable_state.stage,
            ante_start: saveable_state.ante_start,
            ante_end: saveable_state.ante_end,
            ante_current: saveable_state.ante_current,
            action_history: saveable_state.action_history,
            round: saveable_state.round,
            jokers,
            joker_state_manager: joker_state_manager.clone(),
            plays: saveable_state.plays,
            discards: saveable_state.discards,
            reward: saveable_state.reward,
            money: saveable_state.money,
            chips: saveable_state.chips,
            mult: saveable_state.mult,
            score: saveable_state.score,
            hand_type_counts: saveable_state.hand_type_counts,
            // Extended state fields
            consumables_in_hand: saveable_state.consumables_in_hand,
            vouchers: saveable_state.vouchers,
            boss_blind_state: saveable_state.boss_blind_state,
            pack_inventory: saveable_state.pack_inventory,
            open_pack: saveable_state.open_pack,
            state_version: saveable_state.state_version,
            // Non-serializable fields must be reconstructed
            joker_state_manager: Arc::new(JokerStateManager::new()),
        };

        // Restore joker states to the state manager
        game.joker_state_manager
            .restore_from_snapshot(saveable_state.joker_states);

        Ok(game)
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
        assert_eq!(
            game.get_joker_at_slot(0).map(|j| j.id()),
            Some(JokerId::Joker)
        );
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
        assert_eq!(
            game.get_joker_at_slot(0).map(|j| j.id()),
            Some(JokerId::GreedyJoker)
        );
        // TheJoker should have moved to position 1
        assert_eq!(
            game.get_joker_at_slot(1).map(|j| j.id()),
            Some(JokerId::Joker)
        );
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
        // Since the jokers vector is empty, specifying slot 5 will still append at the end (slot 0)
        assert_eq!(
            game.get_joker_at_slot(0).map(|j| j.id()),
            Some(JokerId::Joker)
        );
        assert_eq!(game.joker_count(), 1);
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
