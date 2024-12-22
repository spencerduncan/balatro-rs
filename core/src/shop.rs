use crate::action::Action;
use crate::error::GameError;
use crate::joker::{Joker, Jokers, Rarity};
use rand::distributions::WeightedIndex;
use rand::prelude::*;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct Shop {
    pub jokers: Vec<Jokers>,
    joker_gen: JokerGenerator,
}

impl Shop {
    pub fn new() -> Self {
        return Shop {
            joker_gen: JokerGenerator {},
            jokers: Vec::new(),
        };
    }

    pub(crate) fn refresh(&mut self) {
        let j1 = self.joker_gen.gen_joker();
        let j2 = self.joker_gen.gen_joker();
        self.jokers = vec![j1, j2]
    }

    pub(crate) fn joker_from_index(&self, i: usize) -> Option<Jokers> {
        return Some(self.jokers[i].clone());
    }

    pub(crate) fn buy_joker(&mut self, joker: Jokers) -> Result<Jokers, GameError> {
        let i = self
            .jokers
            .iter()
            .position(|j| *j == joker)
            .ok_or(GameError::NoJokerMatch)?;
        let out = self.jokers.remove(i);
        return Ok(out);
    }

    pub(crate) fn gen_moves_buy_joker(
        &self,
        balance: usize,
    ) -> Option<impl Iterator<Item = Action>> {
        if self.jokers.len() == 0 {
            return None;
        }
        let buys = self
            .jokers
            .clone()
            .into_iter()
            .filter(move |j| j.cost() <= balance)
            .map(|j| Action::BuyJoker(j));
        return Some(buys);
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
        let choices = [Rarity::Common, Rarity::Uncommon, Rarity::Rare];
        let weights = [70, 25, 5];
        let dist = WeightedIndex::new(&weights).unwrap();
        let mut rng = thread_rng();
        return choices[dist.sample(&mut rng)].clone();
    }

    // Generate a random new joker
    pub(crate) fn gen_joker(&self) -> Jokers {
        let rarity = self.gen_rarity();
        let choices = Jokers::by_rarity(rarity);
        let i = thread_rng().gen_range(0..choices.len());
        // TODO: don't regenerate already generated jokers.
        // track with hashmap.
        return choices[i].clone();
    }
}
