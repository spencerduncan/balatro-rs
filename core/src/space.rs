use crate::action::{Action, MoveDirection};
use crate::config::Config;
use crate::error::ActionSpaceError;
use crate::game::Game;
use pyo3::pyclass;

// Hard code a bounded action space.
// Given constraints:
// available max = 24
// store consumable slots max = 4
//
// 0-23: select card
// 24-46: move card (left)
// 47-69: move card (right)
// 70: play
// 71: discard
// 72: cashout
// 73-76: buy joker
// 77: next round
// 78: select blind
//
// We end up with a vector of length 35 where each index
// represents a potential action.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "python", pyclass(eq))]
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct ActionSpace {
    pub select_card: Vec<usize>,
    pub move_card_left: Vec<usize>,
    pub move_card_right: Vec<usize>,
    pub play: Vec<usize>,
    pub discard: Vec<usize>,
    pub cash_out: Vec<usize>,
    pub buy_joker: Vec<usize>,
    pub next_round: Vec<usize>,
    pub select_blind: Vec<usize>,
}

impl ActionSpace {
    pub fn size(&self) -> usize {
        return self.select_card.len()
            + self.move_card_left.len()
            + self.move_card_right.len()
            + self.play.len()
            + self.discard.len()
            + self.cash_out.len()
            + self.buy_joker.len()
            + self.next_round.len()
            + self.select_blind.len();
    }

    pub fn select_card_min(&self) -> usize {
        return 0;
    }

    pub fn select_card_max(&self) -> usize {
        return self.select_card_min() + self.select_card.len();
    }

    pub fn move_card_left_min(&self) -> usize {
        return self.select_card_max() + 1;
    }

    pub fn move_card_left_max(&self) -> usize {
        return self.move_card_left_min() + self.select_card.len() - 1;
    }

    pub fn move_card_right_min(&self) -> usize {
        return self.move_card_left_max() + 1;
    }

    pub fn move_card_right_max(&self) -> usize {
        return self.move_card_right_min() + self.select_card.len() - 1;
    }

    pub fn play_min(&self) -> usize {
        return self.move_card_right_max() + 1;
    }

    pub fn play_max(&self) -> usize {
        return self.play_min() + self.play.len();
    }

    pub fn discard_min(&self) -> usize {
        return self.play_max() + 1;
    }

    pub fn discard_max(&self) -> usize {
        return self.discard_min() + self.discard.len();
    }

    pub fn cash_out_min(&self) -> usize {
        return self.discard_max() + 1;
    }

    pub fn cash_out_max(&self) -> usize {
        return self.cash_out_min() + self.cash_out.len();
    }

    pub fn buy_joker_min(&self) -> usize {
        return self.cash_out_max() + 1;
    }

    pub fn buy_joker_max(&self) -> usize {
        return self.buy_joker_min() + self.buy_joker.len();
    }

    pub fn next_round_min(&self) -> usize {
        return self.buy_joker_max() + 1;
    }

    pub fn next_round_max(&self) -> usize {
        return self.next_round_min() + self.next_round.len();
    }

    pub fn select_blind_min(&self) -> usize {
        return self.next_round_max() + 1;
    }

    pub fn select_blind_max(&self) -> usize {
        return self.select_blind_min() + self.select_blind.len();
    }

    // Not all actions are always legal, by default all actions
    // are masked out, but provide methods to unmask valid.
    pub fn unmask_select_card(&mut self, i: usize) -> Result<(), ActionSpaceError> {
        if i >= self.select_card.len() {
            return Err(ActionSpaceError::InvalidIndex);
        }
        self.select_card[i] = 1;
        return Ok(());
    }

    pub fn unmask_move_card_left(&mut self, i: usize) -> Result<(), ActionSpaceError> {
        if i >= self.move_card_left.len() {
            return Err(ActionSpaceError::InvalidIndex);
        }
        self.move_card_left[i] = 1;
        return Ok(());
    }

    pub fn unmask_move_card_right(&mut self, i: usize) -> Result<(), ActionSpaceError> {
        if i >= self.move_card_right.len() {
            return Err(ActionSpaceError::InvalidIndex);
        }
        self.move_card_right[i] = 1;
        return Ok(());
    }

    pub fn unmask_play(&mut self) {
        self.play[0] = 1;
    }

    pub fn unmask_discard(&mut self) {
        self.discard[0] = 1;
    }

    pub fn unmask_cash_out(&mut self) {
        self.cash_out[0] = 1;
    }

    pub fn unmask_buy_joker(&mut self, i: usize) -> Result<(), ActionSpaceError> {
        if i >= self.buy_joker.len() {
            return Err(ActionSpaceError::InvalidIndex);
        }
        self.buy_joker[i] = 1;
        return Ok(());
    }

    pub fn unmask_next_round(&mut self) {
        self.next_round[0] = 1;
    }

    pub fn unmask_select_blind(&mut self) {
        self.select_blind[0] = 1;
    }

    pub fn to_action(&self, index: usize, game: Game) -> Result<Action, ActionSpaceError> {
        let vec = self.to_vec();
        if let Some(v) = vec.get(index) {
            if *v == 0 {
                return Err(ActionSpaceError::MaskedAction);
            }
        } else {
            return Err(ActionSpaceError::InvalidIndex);
        }
        match index {
            // Cannot reference runtime values in patterns, so this is workaround
            n if (self.select_card_min()..=self.select_card_max()).contains(&n) => {
                if let Some(card) = game.available.card_from_index(index) {
                    return Ok(Action::SelectCard(card));
                } else {
                    return Err(ActionSpaceError::InvalidActionConversion);
                }
            }
            n if (self.move_card_left_min()..=self.move_card_left_max()).contains(&n) => {
                let n_offset = n - self.move_card_left_min();
                if let Some(card) = game.available.card_from_index(n_offset) {
                    return Ok(Action::MoveCard(MoveDirection::Left, card));
                } else {
                    return Err(ActionSpaceError::InvalidActionConversion);
                }
            }
            n if (self.move_card_right_min()..=self.move_card_right_max()).contains(&n) => {
                let n_offset = n - self.move_card_right_min();
                if let Some(card) = game.available.card_from_index(n_offset) {
                    return Ok(Action::MoveCard(MoveDirection::Right, card));
                } else {
                    return Err(ActionSpaceError::InvalidActionConversion);
                }
            }
            n if (self.play_min()..=self.play_max()).contains(&n) => {
                let n_offset = n - self.play_min();

                if let Some(card) = game.available.card_from_index(n_offset) {
                    return Ok(Action::SelectCard(card));
                } else {
                    return Err(ActionSpaceError::InvalidActionConversion);
                }
            }
            _ => return Err(ActionSpaceError::InvalidActionConversion),
        }
    }

    fn to_vec(&self) -> Vec<usize> {
        return [
            self.select_card.clone(),
            self.move_card_left.clone(),
            self.move_card_right.clone(),
            self.play.clone(),
            self.discard.clone(),
            self.cash_out.clone(),
            self.buy_joker.clone(),
            self.next_round.clone(),
            self.select_blind.clone(),
        ]
        .concat();
    }
}

impl From<Config> for ActionSpace {
    fn from(c: Config) -> Self {
        return ActionSpace {
            select_card: vec![0; c.available_max],
            move_card_left: vec![0; c.available_max - 1], // every card but leftmost can move left
            move_card_right: vec![0; c.available_max - 1], // every card but rightmost can move right
            play: vec![0; 1],
            discard: vec![0; 1],
            cash_out: vec![0; 1],
            buy_joker: vec![0; c.store_consumable_slots_max],
            next_round: vec![0; 1],
            select_blind: vec![0; 1],
        };
    }
}

// Generate an action space vector, masked based on current state
impl From<ActionSpace> for Vec<usize> {
    fn from(a: ActionSpace) -> Vec<usize> {
        return [
            a.select_card,
            a.move_card_left,
            a.move_card_right,
            a.play,
            a.discard,
            a.cash_out,
            a.buy_joker,
            a.next_round,
            a.select_blind,
        ]
        .concat();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;

    #[test]
    fn test_unmask() {
        let c = Config::default();
        let mut a = ActionSpace::from(c);

        a.unmask_select_card(1).unwrap();

        let v: Vec<usize> = Vec::from(a);
        dbg!(v.clone());
        assert_eq!(v[0], 0);
        assert_eq!(v[1], 1);
    }

    #[test]
    fn test_unmask_max_index() {
        let c = Config::default();
        let mut a = ActionSpace::from(c);

        a.unmask_select_card(23).unwrap();

        let v: Vec<usize> = Vec::from(a.clone());
        dbg!(v.clone());
        assert_eq!(v[0], 0);
        assert_eq!(v[23], 1);

        let res = a.unmask_select_card(24);
        assert!(res.is_err());
    }
}
