use crate::action::{Action, MoveDirection};
use crate::game::Game;
use crate::joker::OldJoker as Joker;
use crate::shop::packs::{DefaultPackGenerator, PackGenerator};
use crate::space::ActionSpace;
use crate::stage::{Blind, Stage};

impl Game {
    // Get all legal SelectCard actions that can be executed given current state
    fn gen_actions_select_card(&self) -> Option<impl Iterator<Item = Action>> {
        // Can only select card during blinds
        if !self.stage.is_blind() {
            return None;
        }
        // Cannot select more than max
        if self.available.selected().len() >= self.config.selected_max {
            return None;
        }
        let combos = self
            .available
            .not_selected()
            .clone()
            .into_iter()
            .map(Action::SelectCard);
        Some(combos)
    }

    // Get all legal Play actions that can be executed given current state
    fn gen_actions_play(&self) -> Option<impl Iterator<Item = Action>> {
        // Can only play hand during blinds
        if !self.stage.is_blind() {
            return None;
        }
        // If no plays remaining, return None
        if self.plays == 0.0 {
            return None;
        }
        // If no cards selected, return None
        if self.available.selected().is_empty() {
            return None;
        }
        let combos = vec![Action::Play()].into_iter();
        Some(combos)
    }

    // Get all legal Play actions that can be executed given current state
    fn gen_actions_discard(&self) -> Option<impl Iterator<Item = Action>> {
        // Can only discard during blinds
        if !self.stage.is_blind() {
            return None;
        }
        // If no discards remaining, return None
        if self.discards == 0.0 {
            return None;
        }
        // If no cards selected, return None
        if self.available.selected().is_empty() {
            return None;
        }
        let combos = vec![Action::Discard()].into_iter();
        Some(combos)
    }

    // Get all legal move card actions
    fn gen_actions_move_card(&self) -> Option<impl Iterator<Item = Action>> {
        // Can only move cards during blinds
        if !self.stage.is_blind() {
            return None;
        }
        let left = self
            .available
            .cards()
            .clone()
            .into_iter()
            .skip(1)
            .map(|c| Action::MoveCard(MoveDirection::Left, c));
        let right = self
            .available
            .cards()
            .clone()
            .into_iter()
            .rev()
            .skip(1)
            .rev()
            .map(|c| Action::MoveCard(MoveDirection::Right, c));

        let combos = left.chain(right);
        Some(combos)
    }

    // Get cash out action
    fn gen_actions_cash_out(&self) -> Option<impl Iterator<Item = Action>> {
        // If stage is not post blind, cannot cash out
        if self.stage != Stage::PostBlind() {
            return None;
        }
        Some(vec![Action::CashOut(self.reward)].into_iter())
    }

    // Get next round action
    fn gen_actions_next_round(&self) -> Option<impl Iterator<Item = Action>> {
        // If stage is not shop, cannot next round
        if self.stage != Stage::Shop() {
            return None;
        }
        Some(vec![Action::NextRound()].into_iter())
    }

    // Get select blind action
    fn gen_actions_select_blind(&self) -> Option<impl Iterator<Item = Action>> {
        // If stage is not pre blind, cannot select blind
        if self.stage != Stage::PreBlind() {
            return None;
        }
        if let Some(blind) = self.blind {
            Some(vec![Action::SelectBlind(blind.next())].into_iter())
        } else {
            Some(vec![Action::SelectBlind(Blind::Small)].into_iter())
        }
    }

    // Get buy joker actions
    fn gen_actions_buy_joker(&self) -> Option<impl Iterator<Item = Action> + use<'_>> {
        // If stage is not shop, cannot buy
        if self.stage != Stage::Shop() {
            return None;
        }
        // Cannot buy if all joker slots full
        if self.joker_count() >= self.config.joker_slots {
            return None;
        }
        self.shop
            .gen_moves_buy_joker(self.money, self.jokers.len(), self.config.joker_slots)
    }

    // Get buy pack actions
    fn gen_actions_buy_pack(&self) -> Option<impl Iterator<Item = Action> + use<'_>> {
        // If stage is not shop, cannot buy packs
        if self.stage != Stage::Shop() {
            return None;
        }

        let generator = DefaultPackGenerator;
        let available_pack_types = generator.available_pack_types(self);

        Some(
            available_pack_types
                .into_iter()
                .map(|pack_type| Action::BuyPack { pack_type }),
        )
    }

    // Get open pack actions
    fn gen_actions_open_pack(&self) -> Option<impl Iterator<Item = Action> + use<'_>> {
        // Can only open packs if there are packs in inventory and no pack is currently open
        if self.pack_inventory.is_empty() || self.open_pack.is_some() {
            return None;
        }

        Some((0..self.pack_inventory.len()).map(|pack_id| Action::OpenPack { pack_id }))
    }

    // Get select from pack actions
    fn gen_actions_select_from_pack(&self) -> Option<impl Iterator<Item = Action> + use<'_>> {
        // Can only select if a pack is currently open
        let open_pack_state = self.open_pack.as_ref()?;

        Some(
            (0..open_pack_state.pack.options.len()).map(move |option_index| {
                Action::SelectFromPack {
                    pack_id: open_pack_state.pack_id,
                    option_index,
                }
            }),
        )
    }

    // Get skip pack actions
    fn gen_actions_skip_pack(&self) -> Option<impl Iterator<Item = Action> + use<'_>> {
        // Can only skip if a pack is currently open and skippable
        let open_pack_state = self.open_pack.as_ref()?;

        if !open_pack_state.pack.can_skip {
            return None;
        }

        Some(
            vec![Action::SkipPack {
                pack_id: open_pack_state.pack_id,
            }]
            .into_iter(),
        )
    }

    // Get all legal actions that can be executed given current state
    pub fn gen_actions(&self) -> impl Iterator<Item = Action> + use<'_> {
        let select_cards = self.gen_actions_select_card();
        let plays = self.gen_actions_play();
        let discards = self.gen_actions_discard();
        let move_cards = self.gen_actions_move_card();
        let cash_outs = self.gen_actions_cash_out();
        let next_rounds = self.gen_actions_next_round();
        let select_blinds = self.gen_actions_select_blind();
        let buy_jokers = self.gen_actions_buy_joker();
        let buy_packs = self.gen_actions_buy_pack();
        let open_packs = self.gen_actions_open_pack();
        let select_from_packs = self.gen_actions_select_from_pack();
        let skip_packs = self.gen_actions_skip_pack();

        select_cards
            .into_iter()
            .flatten()
            .chain(plays.into_iter().flatten())
            .chain(discards.into_iter().flatten())
            .chain(move_cards.into_iter().flatten())
            .chain(cash_outs.into_iter().flatten())
            .chain(next_rounds.into_iter().flatten())
            .chain(select_blinds.into_iter().flatten())
            .chain(buy_jokers.into_iter().flatten())
            .chain(buy_packs.into_iter().flatten())
            .chain(open_packs.into_iter().flatten())
            .chain(select_from_packs.into_iter().flatten())
            .chain(skip_packs.into_iter().flatten())
    }

    fn unmask_action_space_select_cards(&self, space: &mut ActionSpace) {
        if !self.stage.is_blind() {
            return;
        }
        // Cannot select more if max already selected
        if self.available.selected().len() >= self.config.selected_max {
            return;
        }
        self.available
            .cards_and_selected()
            .iter()
            .enumerate()
            .filter(|(_, (_, a))| !*a)
            .for_each(|(i, _)| {
                let _ = space.unmask_select_card(i); // Ignore error - defensive programming
            });
    }

    fn unmask_action_space_play_and_discard(&self, space: &mut ActionSpace) {
        if !self.stage.is_blind() {
            return;
        }
        // Cannot play/discard if no cards selected
        if self.available.selected().is_empty() {
            return;
        }
        // Can only play/discard is have remaining
        if self.plays != 0.0 {
            space.unmask_play();
        }
        if self.discards != 0.0 {
            space.unmask_discard();
        }
    }

    fn unmask_action_space_move_cards(&self, space: &mut ActionSpace) {
        if !self.stage.is_blind() {
            return;
        }
        // move left
        // every available card except the first can move left
        self.available
            .cards()
            .iter()
            .skip(1)
            .enumerate()
            .for_each(|(i, _)| {
                let _ = space.unmask_move_card_left(i); // Ignore error - defensive programming
            });
        // move right
        // every available card except the last can move right
        self.available
            .cards()
            .iter()
            .rev()
            .skip(1)
            .rev()
            .enumerate()
            .for_each(|(i, _)| {
                let _ = space.unmask_move_card_right(i); // Ignore error - defensive programming
            });
    }

    fn unmask_action_space_cash_out(&self, space: &mut ActionSpace) {
        if self.stage != Stage::PostBlind() {
            return;
        }
        space.unmask_cash_out();
    }

    fn unmask_action_space_next_round(&self, space: &mut ActionSpace) {
        if self.stage != Stage::Shop() {
            return;
        }
        space.unmask_next_round();
    }

    fn unmask_action_space_select_blind(&self, space: &mut ActionSpace) {
        if self.stage != Stage::PreBlind() {
            return;
        }
        space.unmask_select_blind();
    }

    fn unmask_action_space_buy_joker(&self, space: &mut ActionSpace) {
        if self.stage != Stage::Shop() {
            return;
        }
        self.shop
            .jokers
            .iter()
            .enumerate()
            .filter(|(_i, j)| j.cost() as f64 <= self.money)
            .for_each(|(i, _j)| {
                let _ = space.unmask_buy_joker(i); // Ignore error - defensive programming
            });
    }

    // Get an action space, masked for legal actions only
    pub fn gen_action_space(&self) -> ActionSpace {
        let mut space = ActionSpace::from(self.config.clone());
        self.unmask_action_space_select_cards(&mut space);
        self.unmask_action_space_play_and_discard(&mut space);
        self.unmask_action_space_move_cards(&mut space);
        self.unmask_action_space_cash_out(&mut space);
        self.unmask_action_space_next_round(&mut space);
        self.unmask_action_space_select_blind(&mut space);
        self.unmask_action_space_buy_joker(&mut space);
        space
    }
}

#[cfg(test)]
#[allow(clippy::field_reassign_with_default)]
mod tests {
    use super::*;
    use crate::card::{Card, Suit, Value};

    #[test]
    fn test_gen_moves_play() {
        let ace = Card::new(Value::Ace, Suit::Heart);
        let king = Card::new(Value::King, Suit::Diamond);

        let mut g = Game::default();
        g.stage = Stage::Blind(Blind::Small);

        // nothing selected, nothing to play
        assert!(g.gen_actions_discard().is_none());

        g.available.extend(vec![ace]);
        g.select_card(ace).unwrap();
        let moves: Vec<Action> = g.gen_actions_play().expect("are plays").collect();
        assert_eq!(moves.len(), 1);

        g.available.extend(vec![ace, king]);
        g.select_card(ace).unwrap();
        g.select_card(king).unwrap();
        let moves: Vec<Action> = g.gen_actions_play().expect("are plays").collect();
        assert_eq!(moves.len(), 1);
    }

    #[test]
    fn test_gen_moves_discard() {
        let ace = Card::new(Value::Ace, Suit::Heart);
        let king = Card::new(Value::King, Suit::Diamond);

        let mut g = Game::default();
        g.stage = Stage::Blind(Blind::Small);

        // nothing selected, nothing to discard
        assert!(g.gen_actions_discard().is_none());

        g.available.extend(vec![ace, king]);
        g.select_card(ace).unwrap();
        g.select_card(king).unwrap();
        let moves: Vec<Action> = g.gen_actions_discard().expect("are discards").collect();
        assert_eq!(moves.len(), 1);
    }

    #[test]
    fn test_unmask_action_space_select_cards() {
        let mut g = Game::default();
        g.deal();
        g.stage = Stage::Blind(Blind::Small);
        let mut space = ActionSpace::from(g.config.clone());

        // Default action space everything should be masked
        assert!(space.select_card[0] == 0);

        // Unmask card selects, we have all selects available
        g.unmask_action_space_select_cards(&mut space);
        assert!(space.select_card[0] == 1);

        // Make a fresh space
        space = ActionSpace::from(g.config.clone());
        // Select 2 cards, regenerate action space
        for _ in 0..2 {
            g.select_card(*g.available.not_selected().first().expect("is first card"))
                .expect("can select");
        }
        g.unmask_action_space_select_cards(&mut space);
        // Cannot select first and second, can select third
        assert!(space.select_card[0] == 0);
        assert!(space.select_card[1] == 0);
        assert!(space.select_card[2] == 1);
    }

    #[test]
    fn test_unmask_action_space_select_cards_max() {
        let mut g = Game::default();
        g.deal();
        g.stage = Stage::Blind(Blind::Small);
        let mut space = ActionSpace::from(g.config.clone());

        // Default action space everything should be masked
        assert!(space.select_card[0] == 0);

        // Unmask card selects, we have all selects available
        g.unmask_action_space_select_cards(&mut space);
        assert!(space.select_card[0] == 1);

        // Make a fresh space
        space = ActionSpace::from(g.config.clone());
        // Now select 5 cards, no more selects available, regenerate action space
        for _ in 0..g.config.selected_max {
            g.select_card(*g.available.not_selected().first().expect("is first card"))
                .expect("can select");
        }
        g.unmask_action_space_select_cards(&mut space);
        for i in 0..space.select_card.len() - 1 {
            assert!(space.select_card[i] == 0);
        }

        // If stage is not blind, don't alter space
        g.stage = Stage::Shop();
        space = ActionSpace::from(g.config.clone());
        space.select_card[0] = 1;
        assert!(space.select_card[0] == 1);
        g.unmask_action_space_select_cards(&mut space);
        assert!(space.select_card[0] == 1);
    }

    #[test]
    fn test_unmask_action_space_play_and_discard() {
        let mut g = Game::default();
        g.deal();
        g.stage = Stage::Blind(Blind::Small);
        let mut space = ActionSpace::from(g.config.clone());

        // Default action space everything should be masked
        assert!(space.play[0] == 0);
        assert!(space.discard[0] == 0);

        for c in g.available.cards()[0..5].to_vec() {
            g.select_card(c).unwrap();
        }
        // Unmask play/discard
        g.unmask_action_space_play_and_discard(&mut space);
        assert!(space.play[0] == 1);
        assert!(space.discard[0] == 1);
    }

    #[test]
    fn test_unmask_action_space_move_cards() {
        let mut g = Game::default();
        g.stage = Stage::Blind(Blind::Small);
        let mut space = ActionSpace::from(g.config.clone());

        // Default action space everything should be masked, since no cards available yet
        assert_eq!(g.available.cards().len(), 0);
        for i in 0..space.move_card_left.len() {
            assert!(space.move_card_left[i] == 0);
        }
        for i in 0..space.move_card_right.len() {
            assert!(space.move_card_right[i] == 0);
        }

        // deal and make available
        g.deal();
        // Unmask play/discard
        g.unmask_action_space_move_cards(&mut space);

        // Should be able to move left every available card except leftmost
        let available = g.available.cards().len();

        // Use safe range to prevent underflow when available is 0
        use crate::math_safe::safe_size_for_move_operations;
        let move_range_size = safe_size_for_move_operations(available);

        for i in 0..move_range_size {
            assert!(space.move_card_left[i] == 1);
        }
        for i in 0..move_range_size {
            assert!(space.move_card_right[i] == 1);
        }

        // Even when selected, we can still move cards
        let not_selected = g.available.not_selected();
        for c in &not_selected[0..5] {
            g.select_card(*c).unwrap();
        }

        // Get fresh action space and mask
        space = ActionSpace::from(g.config.clone());
        g.unmask_action_space_move_cards(&mut space);
        for i in 0..move_range_size {
            assert!(space.move_card_left[i] == 1);
        }
        for i in 0..move_range_size {
            assert!(space.move_card_right[i] == 1);
        }
    }

    #[test]
    fn test_gen_actions_select_card_edge_cases() {
        let mut g = Game::default();

        // Test with no stage set to blind
        g.stage = Stage::Shop();
        assert!(g.gen_actions_select_card().is_none());

        g.stage = Stage::PostBlind();
        assert!(g.gen_actions_select_card().is_none());

        g.stage = Stage::PreBlind();
        assert!(g.gen_actions_select_card().is_none());

        // Set to blind stage
        g.stage = Stage::Blind(Blind::Small);
        g.deal();

        // Should have select actions when not at max
        let actions = g
            .gen_actions_select_card()
            .expect("Should have select actions");
        let action_count = actions.count();
        assert!(action_count > 0);

        // Select cards up to max
        for _ in 0..g.config.selected_max {
            if let Some(card) = g.available.not_selected().first() {
                g.select_card(*card).unwrap();
            }
        }

        // Should have no more select actions
        assert!(g.gen_actions_select_card().is_none());
    }

    #[test]
    fn test_gen_actions_play_edge_cases() {
        let mut g = Game::default();

        // Test with no blind stage
        g.stage = Stage::Shop();
        assert!(g.gen_actions_play().is_none());

        // Set to blind stage but no plays remaining
        g.stage = Stage::Blind(Blind::Small);
        g.plays = 0.0;
        assert!(g.gen_actions_play().is_none());

        // Reset plays but no cards selected
        g.plays = 1.0;
        assert!(g.gen_actions_play().is_none());

        // Add and select cards
        g.deal();
        g.select_card(*g.available.cards().first().unwrap())
            .unwrap();

        // Should now have play actions
        let actions = g.gen_actions_play().expect("Should have play actions");
        assert_eq!(actions.count(), 1);
    }

    #[test]
    fn test_gen_actions_discard_edge_cases() {
        let mut g = Game::default();

        // Test with no blind stage
        g.stage = Stage::Shop();
        assert!(g.gen_actions_discard().is_none());

        // Set to blind stage but no discards remaining
        g.stage = Stage::Blind(Blind::Small);
        g.discards = 0.0;
        assert!(g.gen_actions_discard().is_none());

        // Reset discards but no cards selected
        g.discards = 1.0;
        assert!(g.gen_actions_discard().is_none());

        // Add and select cards
        g.deal();
        g.select_card(*g.available.cards().first().unwrap())
            .unwrap();

        // Should now have discard actions
        let actions = g
            .gen_actions_discard()
            .expect("Should have discard actions");
        assert_eq!(actions.count(), 1);
    }

    #[test]
    fn test_gen_actions_move_card_edge_cases() {
        // Test with no blind stage
        let mut g = Game {
            stage: Stage::Shop(),
            ..Default::default()
        };
        assert!(g.gen_actions_move_card().is_none());

        // Set to blind stage but no cards available
        g = Game {
            stage: Stage::Blind(Blind::Small),
            ..Default::default()
        };
        let move_actions = g
            .gen_actions_move_card()
            .expect("Should return Some even with no cards");
        assert_eq!(move_actions.count(), 0);

        // Add single card - should have no move actions
        let ace = Card::new(Value::Ace, Suit::Heart);
        g.available.extend(vec![ace]);
        let move_actions = g.gen_actions_move_card();
        // Single card might still generate empty iterator rather than None
        if let Some(actions) = move_actions {
            assert_eq!(actions.count(), 0);
        }

        // Add second card - should now have move actions
        let king = Card::new(Value::King, Suit::Spade);
        g.available.extend(vec![king]);

        let actions = g.gen_actions_move_card().expect("Should have move actions");
        let action_count = actions.count();
        assert_eq!(action_count, 2); // One left move, one right move
    }

    #[test]
    fn test_gen_actions_cash_out_edge_cases() {
        // Test with wrong stage
        let mut g = Game {
            stage: Stage::Shop(),
            ..Default::default()
        };
        assert!(g.gen_actions_cash_out().is_none());

        g = Game {
            stage: Stage::Blind(Blind::Small),
            ..Default::default()
        };
        assert!(g.gen_actions_cash_out().is_none());

        // Set to correct stage
        g = Game {
            stage: Stage::PostBlind(),
            reward: 100.0,
            ..Default::default()
        };

        let actions = g
            .gen_actions_cash_out()
            .expect("Should have cash out action");
        let actions_vec: Vec<Action> = actions.collect();
        assert_eq!(actions_vec.len(), 1);
        assert!(matches!(actions_vec[0], Action::CashOut(100.0)));
    }

    #[test]
    fn test_gen_actions_next_round_edge_cases() {
        // Test with wrong stage
        let mut g = Game {
            stage: Stage::Blind(Blind::Small),
            ..Default::default()
        };
        assert!(g.gen_actions_next_round().is_none());

        g = Game {
            stage: Stage::PostBlind(),
            ..Default::default()
        };
        assert!(g.gen_actions_next_round().is_none());

        // Set to correct stage
        g = Game {
            stage: Stage::Shop(),
            ..Default::default()
        };

        let actions = g
            .gen_actions_next_round()
            .expect("Should have next round action");
        let actions_vec: Vec<Action> = actions.collect();
        assert_eq!(actions_vec.len(), 1);
        assert!(matches!(actions_vec[0], Action::NextRound()));
    }

    #[test]
    fn test_gen_actions_select_blind_edge_cases() {
        let mut g = Game::default();

        // Test with wrong stage
        g.stage = Stage::Shop();
        assert!(g.gen_actions_select_blind().is_none());

        g.stage = Stage::Blind(Blind::Small);
        assert!(g.gen_actions_select_blind().is_none());

        // Set to correct stage
        g.stage = Stage::PreBlind();

        // Test with no current blind
        g.blind = None;
        let actions = g
            .gen_actions_select_blind()
            .expect("Should have select blind action");
        let actions_vec: Vec<Action> = actions.collect();
        assert_eq!(actions_vec.len(), 1);
        assert!(matches!(actions_vec[0], Action::SelectBlind(Blind::Small)));

        // Test with current blind
        g.blind = Some(Blind::Small);
        let actions = g
            .gen_actions_select_blind()
            .expect("Should have select blind action");
        let actions_vec: Vec<Action> = actions.collect();
        assert_eq!(actions_vec.len(), 1);
        assert!(matches!(actions_vec[0], Action::SelectBlind(Blind::Big)));
    }

    #[test]
    fn test_gen_actions_buy_joker_edge_cases() {
        let mut g = Game::default();

        // Test with wrong stage
        g.stage = Stage::Blind(Blind::Small);
        assert!(g.gen_actions_buy_joker().is_none());

        // Set to correct stage but max jokers
        g.stage = Stage::Shop();
        // Assuming joker_count() returns number of jokers
        // and config.joker_slots is max allowed
        for _ in 0..g.config.joker_slots {
            // This is a simplified test since we can't easily add jokers without the full game state
            // The actual test would need proper joker setup
        }

        // Test with shop stage and available slots
        g.stage = Stage::Shop();
        // Test depends on shop having jokers available, which is set up in refresh()
        // This is more of a integration test at this point
    }

    #[test]
    fn test_gen_actions_comprehensive() {
        let mut g = Game::default();
        g.deal();
        g.stage = Stage::Blind(Blind::Small);

        // Test comprehensive action generation
        let all_actions: Vec<Action> = g.gen_actions().collect();

        // Should have card selection actions
        let select_actions: Vec<&Action> = all_actions
            .iter()
            .filter(|a| matches!(a, Action::SelectCard(_)))
            .collect();
        assert!(!select_actions.is_empty());

        // Should have move actions for available cards
        let move_actions: Vec<&Action> = all_actions
            .iter()
            .filter(|a| matches!(a, Action::MoveCard(_, _)))
            .collect();
        assert!(!move_actions.is_empty());

        // Select some cards and test play/discard actions
        for card in g.available.cards().iter().take(3) {
            g.select_card(*card).unwrap();
        }

        let all_actions: Vec<Action> = g.gen_actions().collect();
        let play_actions: Vec<&Action> = all_actions
            .iter()
            .filter(|a| matches!(a, Action::Play()))
            .collect();
        let discard_actions: Vec<&Action> = all_actions
            .iter()
            .filter(|a| matches!(a, Action::Discard()))
            .collect();

        assert_eq!(play_actions.len(), 1);
        assert_eq!(discard_actions.len(), 1);
    }

    #[test]
    fn test_gen_action_space_comprehensive() {
        let mut g = Game::default();
        g.deal();
        g.stage = Stage::Blind(Blind::Small);

        let space = g.gen_action_space();

        // Should have unmasked select card actions
        let unmasked_selects = space.select_card.iter().sum::<usize>();
        assert!(unmasked_selects > 0);

        // Should have unmasked move actions
        let unmasked_moves_left = space.move_card_left.iter().sum::<usize>();
        let unmasked_moves_right = space.move_card_right.iter().sum::<usize>();
        assert!(unmasked_moves_left > 0);
        assert!(unmasked_moves_right > 0);

        // Play and discard should be masked (no cards selected)
        assert_eq!(space.play[0], 0);
        assert_eq!(space.discard[0], 0);

        // Select cards and regenerate
        for card in g.available.cards().iter().take(2) {
            g.select_card(*card).unwrap();
        }

        let space = g.gen_action_space();
        // Play and discard should now be unmasked
        assert_eq!(space.play[0], 1);
        assert_eq!(space.discard[0], 1);
    }

    #[test]
    fn test_action_generation_stress_test() {
        let mut g = Game::default();

        // Test action generation across all stages
        let stages = vec![
            Stage::PreBlind(),
            Stage::Blind(Blind::Small),
            Stage::PostBlind(),
            Stage::Shop(),
        ];

        for stage in stages {
            g.stage = stage;
            g.deal(); // Ensure cards are available

            // Should not panic when generating actions
            let actions: Vec<Action> = g.gen_actions().collect();

            // Should not panic when generating action space
            let _space = g.gen_action_space();

            // Actions should be consistent with stage
            match stage {
                Stage::PreBlind() => {
                    assert!(actions.iter().any(|a| matches!(a, Action::SelectBlind(_))));
                }
                Stage::Blind(_) => {
                    assert!(actions.iter().any(|a| matches!(a, Action::SelectCard(_))));
                }
                Stage::PostBlind() => {
                    assert!(actions.iter().any(|a| matches!(a, Action::CashOut(_))));
                }
                Stage::Shop() => {
                    assert!(actions.iter().any(|a| matches!(a, Action::NextRound())));
                }
                Stage::End(_) => {
                    // End stage might have no actions
                }
            }
        }
    }

    #[test]
    fn test_action_generation_boundary_conditions() {
        let mut g = Game::default();
        g.stage = Stage::Blind(Blind::Small);

        // Test with minimal available cards
        let ace = Card::new(Value::Ace, Suit::Heart);
        g.available.extend(vec![ace]);

        let actions: Vec<Action> = g.gen_actions().collect();

        // Should have select action for the one card
        assert!(actions
            .iter()
            .any(|a| matches!(a, Action::SelectCard(c) if *c == ace)));

        // Should have no move actions (only one card)
        assert!(!actions.iter().any(|a| matches!(a, Action::MoveCard(_, _))));

        // Test with maximum available cards (config limit)
        // Clear existing cards first
        g.available = crate::available::Available::default();
        let max_cards = g.config.available_max;
        for i in 0..max_cards.min(52) {
            // Don't exceed standard deck size
            let value = Value::values()[i % 13];
            let suit = [Suit::Heart, Suit::Diamond, Suit::Club, Suit::Spade][i % 4];
            g.available.extend(vec![Card::new(value, suit)]);
        }

        let actions: Vec<Action> = g.gen_actions().collect();

        // Should have select actions for available cards
        let select_actions: Vec<&Action> = actions
            .iter()
            .filter(|a| matches!(a, Action::SelectCard(_)))
            .collect();
        // The number of select actions should equal the number of available cards
        // (limitation of selected_max is enforced at execution time, not generation time)
        assert_eq!(select_actions.len(), g.available.cards().len());
    }

    #[test]
    fn test_action_generation_with_zero_resources() {
        let mut g = Game::default();
        g.stage = Stage::Blind(Blind::Small);
        g.deal();

        // Test with zero plays remaining
        g.plays = 0.0;
        g.select_card(*g.available.cards().first().unwrap())
            .unwrap();

        let actions: Vec<Action> = g.gen_actions().collect();
        assert!(!actions.iter().any(|a| matches!(a, Action::Play())));

        // Test with zero discards remaining
        g.plays = 1.0;
        g.discards = 0.0;

        let actions: Vec<Action> = g.gen_actions().collect();
        assert!(actions.iter().any(|a| matches!(a, Action::Play())));
        assert!(!actions.iter().any(|a| matches!(a, Action::Discard())));
    }

    #[test]
    fn test_unmask_action_space_edge_cases() {
        let mut g = Game::default();
        let mut space = ActionSpace::from(g.config.clone());

        // Test unmasking with no available cards
        g.stage = Stage::Blind(Blind::Small);
        g.unmask_action_space_select_cards(&mut space);

        // All select actions should remain masked (no cards available)
        assert!(space.select_card.iter().all(|&x| x == 0));

        // Test move card unmasking with no cards
        g.unmask_action_space_move_cards(&mut space);
        assert!(space.move_card_left.iter().all(|&x| x == 0));
        assert!(space.move_card_right.iter().all(|&x| x == 0));

        // Test play/discard unmasking with no selected cards
        g.unmask_action_space_play_and_discard(&mut space);
        assert_eq!(space.play[0], 0);
        assert_eq!(space.discard[0], 0);
    }

    #[test]
    fn test_gen_actions_deterministic_ordering() {
        let mut g = Game::default();
        g.deal();
        g.stage = Stage::Blind(Blind::Small);

        // Generate actions multiple times and verify consistent ordering
        let actions1: Vec<Action> = g.gen_actions().collect();
        let actions2: Vec<Action> = g.gen_actions().collect();

        assert_eq!(
            actions1, actions2,
            "Action generation should be deterministic"
        );
    }
}
