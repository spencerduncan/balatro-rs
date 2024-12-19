use crate::action::{Action, MoveDirection};
use crate::game::Game;
use crate::joker::Joker;
use crate::space::ActionSpace;
use crate::stage::{Blind, Stage};

impl Game {
    // get all legal SelectCard moves that can be executed given current state
    pub fn gen_moves_select_card(&self) -> Option<impl Iterator<Item = Action>> {
        // Can only select card during blinds
        if !self.stage.is_blind() {
            return None;
        }
        // Cannot select more than max
        if self.selected.len() >= self.config.selected_max {
            return None;
        }
        let combos = self
            .available
            .clone()
            .into_iter()
            .map(|c| Action::SelectCard(c));
        return Some(combos);
    }

    // get all legal Play moves that can be executed given current state
    pub fn gen_moves_play(&self) -> Option<impl Iterator<Item = Action>> {
        // Can only play hand during blinds
        if !self.stage.is_blind() {
            return None;
        }
        // If no plays remaining, return None
        if self.plays <= 0 {
            return None;
        }
        // If no cards selected, return None
        if self.selected.len() == 0 {
            return None;
        }
        let combos = vec![Action::Play()].into_iter();
        return Some(combos);
    }

    // get all legal Play moves that can be executed given current state
    pub fn gen_moves_discard(&self) -> Option<impl Iterator<Item = Action>> {
        // Can only discard during blinds
        if !self.stage.is_blind() {
            return None;
        }
        // If no discards remaining, return None
        if self.discards <= 0 {
            return None;
        }
        // If no cards selected, return None
        if self.selected.len() == 0 {
            return None;
        }
        let combos = vec![Action::Discard()].into_iter();
        return Some(combos);
    }

    // get all legal move card moves
    pub fn gen_moves_move_card(&self) -> Option<impl Iterator<Item = Action>> {
        // Can only move cards during blinds
        if !self.stage.is_blind() {
            return None;
        }
        // Must be one card selected to move
        if self.selected.len() == 0 || self.selected.len() >= 2 {
            return None;
        }
        let combos = vec![
            Action::MoveCard(MoveDirection::Left),
            Action::MoveCard(MoveDirection::Right),
        ]
        .into_iter();
        return Some(combos);
    }

    // get cash out move
    pub fn gen_moves_cash_out(&self) -> Option<impl Iterator<Item = Action>> {
        // If stage is not post blind, cannot cash out
        if self.stage != Stage::PostBlind() {
            return None;
        }
        return Some(vec![Action::CashOut(self.reward)].into_iter());
    }

    // get next round move
    pub fn gen_moves_next_round(&self) -> Option<impl Iterator<Item = Action>> {
        // If stage is not shop, cannot next round
        if self.stage != Stage::Shop() {
            return None;
        }
        return Some(vec![Action::NextRound()].into_iter());
    }

    // get select blind move
    pub fn gen_moves_select_blind(&self) -> Option<impl Iterator<Item = Action>> {
        // If stage is not pre blind, cannot select blind
        if self.stage != Stage::PreBlind() {
            return None;
        }
        if let Some(blind) = self.blind {
            return Some(vec![Action::SelectBlind(blind.next())].into_iter());
        } else {
            return Some(vec![Action::SelectBlind(Blind::Small)].into_iter());
        }
    }

    // get buy joker moves
    pub fn gen_moves_buy_joker(&self) -> Option<impl Iterator<Item = Action>> {
        // If stage is not shop, cannot buy
        if self.stage != Stage::Shop() {
            return None;
        }
        // Cannot buy if all joker slots full
        if self.jokers.len() >= self.config.joker_slots {
            return None;
        }
        return self.shop.gen_moves_buy_joker(self.money);
    }

    // get all legal moves that can be executed given current state
    pub fn gen_moves(&self) -> impl Iterator<Item = Action> {
        let select_cards = self.gen_moves_select_card();
        let plays = self.gen_moves_play();
        let discards = self.gen_moves_discard();
        let move_cards = self.gen_moves_move_card();
        let cash_outs = self.gen_moves_cash_out();
        let next_rounds = self.gen_moves_next_round();
        let select_blinds = self.gen_moves_select_blind();
        let buy_jokers = self.gen_moves_buy_joker();

        return select_cards
            .into_iter()
            .flatten()
            .chain(plays.into_iter().flatten())
            .chain(discards.into_iter().flatten())
            .chain(move_cards.into_iter().flatten())
            .chain(cash_outs.into_iter().flatten())
            .chain(next_rounds.into_iter().flatten())
            .chain(select_blinds.into_iter().flatten())
            .chain(buy_jokers.into_iter().flatten());
    }

    pub fn gen_action_space(&self) -> ActionSpace {
        let mut space = ActionSpace::from(self.config.clone());

        if self.stage.is_blind() {
            // Select cards
            if self.selected.len() < self.config.selected_max {
                self.available.iter().enumerate().for_each(|(i, _)| {
                    space
                        .unmask_select_card(i)
                        .expect("valid index for selecting")
                });
            }

            // play/discard cards if selected
            if self.selected.len() > 0 {
                space.unmask_play();
                space.unmask_discard();
            }

            // move cards
            self.available
                .iter()
                .skip(1)
                .enumerate()
                .for_each(|(i, _)| {
                    space
                        .unmask_move_card_left(i)
                        .expect("valid index for move left")
                });
            self.available
                .iter()
                .skip(1)
                .enumerate()
                .for_each(|(i, _)| {
                    space
                        .unmask_move_card_right(i)
                        .expect("valid index for moving right")
                });
        }

        if self.stage == Stage::PostBlind() {
            space.unmask_cash_out();
        }

        if self.stage == Stage::Shop() {
            space.unmask_next_round();
        }

        if self.stage == Stage::PreBlind() {
            space.unmask_select_blind();
        }

        if self.stage == Stage::Shop() {
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
        return space;
    }
}
