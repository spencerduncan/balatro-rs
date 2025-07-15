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
        if self.plays == 0 {
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
        if self.discards == 0 {
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
                space
                    .unmask_select_card(i)
                    .expect("valid index for selecting");
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
        if self.plays != 0 {
            space.unmask_play();
        }
        if self.discards != 0 {
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
                space
                    .unmask_move_card_left(i)
                    .expect("valid index for move left")
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
                space
                    .unmask_move_card_right(i)
                    .expect("valid index for move right")
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
            .filter(|(_i, j)| j.cost() <= self.money)
            .for_each(|(i, _j)| {
                space
                    .unmask_buy_joker(i)
                    .expect("valid index for buy joker")
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
mod tests {
    use super::*;
    use crate::card::{Card, Suit, Value};

    #[test]
    fn test_gen_moves_play() {
        let ace = Card::new(Value::Ace, Suit::Heart);
        let king = Card::new(Value::King, Suit::Diamond);

        let mut g = Game {
            stage: Stage::Blind(Blind::Small),
            ..Default::default()
        };

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

        let mut g = Game {
            stage: Stage::Blind(Blind::Small),
            ..Default::default()
        };

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
        let mut g = Game {
            stage: Stage::Blind(Blind::Small),
            ..Default::default()
        };
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
        for i in 0..available - 1 {
            assert!(space.move_card_left[i] == 1);
        }
        for i in 0..available - 1 {
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
        for i in 0..available - 1 {
            assert!(space.move_card_left[i] == 1);
        }
        for i in 0..available - 1 {
            assert!(space.move_card_right[i] == 1);
        }
    }
}
