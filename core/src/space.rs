use crate::config::Config;
use crate::error::ActionSpaceError;

// Hard code a bounded action space.
// Given constraints:
// available max = 24
// store consumable slots max = 4
//
// 0-23: select_card
// 24-25: move_card
// 26: play
// 27: discard
// 28: cashout
// 29-32: buy joker
// 33: next round
// 34: select blind
//
// We end up with a vector of length 35 where each index
// represents a potential action.

// Not all actions are always legal, so we can
// also provide an action mask based on game state.

pub struct ActionSpace {
    pub select_card: Vec<usize>,
    pub move_card: Vec<usize>,
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
            + self.move_card.len()
            + self.play.len()
            + self.discard.len()
            + self.cash_out.len()
            + self.buy_joker.len()
            + self.next_round.len()
            + self.select_blind.len();
    }

    pub fn unmask_select_card(&mut self, i: usize) -> Result<(), ActionSpaceError> {
        if i > self.select_card.len() {
            return Err(ActionSpaceError::InvalidIndex);
        }
        self.select_card[i] = 1;
        return Ok(());
    }
}

impl From<Config> for ActionSpace {
    fn from(c: Config) -> Self {
        return ActionSpace {
            select_card: vec![0; c.available_max],
            move_card: vec![0; 2], // left, right
            play: vec![0; 1],
            discard: vec![0; 1],
            cash_out: vec![0; 1],
            buy_joker: vec![0; c.store_consumable_slots_max],
            next_round: vec![0; 1],
            select_blind: vec![0; 1],
        };
    }
}

impl From<ActionSpace> for Vec<usize> {
    fn from(a: ActionSpace) -> Vec<usize> {
        return [
            a.select_card,
            a.move_card,
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
    fn test_mask() {
        let c = Config::default();
        let mut a = ActionSpace::from(c);

        a.unmask_select_card(1).unwrap();

        let v: Vec<usize> = Vec::from(a);
        dbg!(v.clone());
        assert_eq!(v[0], 0);
        assert_eq!(v[1], 1);
    }
}
