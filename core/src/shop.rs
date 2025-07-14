use crate::action::Action;
use crate::error::GameError;
use crate::joker::{JokerId, JokerRarity as Rarity, Jokers, OldJoker as Joker};
// use rand::distributions::WeightedIndex;
use rand::prelude::*;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct Shop {
    pub jokers: Vec<Jokers>,
    joker_gen: JokerGenerator,
}

impl Default for Shop {
    fn default() -> Self {
        Self::new()
    }
}

impl Shop {
    pub fn new() -> Self {
        Shop {
            joker_gen: JokerGenerator {},
            jokers: Vec::new(),
        }
    }

    pub(crate) fn refresh(&mut self) {
        let j1 = self.joker_gen.gen_joker();
        let j2 = self.joker_gen.gen_joker();
        self.jokers = vec![j1, j2]
    }

    pub(crate) fn joker_from_index(&self, i: usize) -> Option<Jokers> {
        Some(self.jokers[i].clone())
    }

    #[allow(dead_code)] // Kept for backward compatibility
    pub(crate) fn buy_joker(&mut self, joker: &Jokers) -> Result<Jokers, GameError> {
        let i = self
            .jokers
            .iter()
            .position(|j| j == joker)
            .ok_or(GameError::NoJokerMatch)?;
        let out = self.jokers.remove(i);
        Ok(out)
    }

    pub(crate) fn has_joker(&self, joker_id: JokerId) -> bool {
        // TODO: For now, we'll check if any of the old Jokers enum matches
        // This is a temporary implementation until shop is updated to use JokerId
        self.jokers.iter().any(|j| {
            // Map between old Jokers enum and new JokerId
            matches!(
                (j, joker_id),
                (Jokers::TheJoker(_), JokerId::Joker)
                    | (Jokers::GreedyJoker(_), JokerId::GreedyJoker)
                    | (Jokers::LustyJoker(_), JokerId::LustyJoker)
                    | (Jokers::WrathfulJoker(_), JokerId::WrathfulJoker)
                    | (Jokers::GluttonousJoker(_), JokerId::GluttonousJoker)
                    | (Jokers::JollyJoker(_), JokerId::JollyJoker)
                    | (Jokers::ZanyJoker(_), JokerId::ZanyJoker)
                    | (Jokers::MadJoker(_), JokerId::MadJoker)
                    | (Jokers::CrazyJoker(_), JokerId::CrazyJoker)
                    | (Jokers::DrollJoker(_), JokerId::DrollJoker)
                    | (Jokers::SlyJoker(_), JokerId::SlyJoker)
                    | (Jokers::WilyJoker(_), JokerId::WilyJoker)
                    | (Jokers::CleverJoker(_), JokerId::CleverJoker)
                    | (Jokers::DeviousJoker(_), JokerId::DeviousJoker)
                    | (Jokers::CraftyJoker(_), JokerId::CraftyJoker)
            )
        })
    }

    pub(crate) fn gen_moves_buy_joker(
        &self,
        balance: usize,
        current_joker_count: usize,
        max_slots: usize,
    ) -> Option<impl Iterator<Item = Action>> {
        if self.jokers.is_empty() {
            return None;
        }

        // Check if we can add more jokers
        if current_joker_count >= max_slots {
            return None;
        }

        // We can insert at any position from 0 to current_joker_count (inclusive)
        let available_slots: Vec<usize> = (0..=current_joker_count).collect();

        let buys: Vec<Action> = self
            .jokers
            .iter()
            .filter(move |j| j.cost() <= balance)
            .flat_map(|joker| {
                // Map old Joker enum to new JokerId
                let joker_id = match joker {
                    Jokers::TheJoker(_) => JokerId::Joker,
                    Jokers::GreedyJoker(_) => JokerId::GreedyJoker,
                    Jokers::LustyJoker(_) => JokerId::LustyJoker,
                    Jokers::WrathfulJoker(_) => JokerId::WrathfulJoker,
                    Jokers::GluttonousJoker(_) => JokerId::GluttonousJoker,
                    Jokers::JollyJoker(_) => JokerId::JollyJoker,
                    Jokers::ZanyJoker(_) => JokerId::ZanyJoker,
                    Jokers::MadJoker(_) => JokerId::MadJoker,
                    Jokers::CrazyJoker(_) => JokerId::CrazyJoker,
                    Jokers::DrollJoker(_) => JokerId::DrollJoker,
                    Jokers::SlyJoker(_) => JokerId::SlyJoker,
                    Jokers::WilyJoker(_) => JokerId::WilyJoker,
                    Jokers::CleverJoker(_) => JokerId::CleverJoker,
                    Jokers::DeviousJoker(_) => JokerId::DeviousJoker,
                    Jokers::CraftyJoker(_) => JokerId::CraftyJoker,
                };

                // Generate an action for each available slot
                available_slots
                    .iter()
                    .map(move |&slot| Action::BuyJoker { joker_id, slot })
                    .collect::<Vec<_>>()
            })
            .collect();

        if buys.is_empty() {
            None
        } else {
            Some(buys.into_iter())
        }
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
        // For now, we only have common jokers...
        Rarity::Common
        // let choices = [Rarity::Common, Rarity::Uncommon, Rarity::Rare];
        // let weights = [70, 25, 5];
        // let dist = WeightedIndex::new(&weights).unwrap();
        // let mut rng = thread_rng();
        // return choices[dist.sample(&mut rng)].clone();
    }

    // Generate a random new joker
    pub(crate) fn gen_joker(&self) -> Jokers {
        let rarity = self.gen_rarity();
        let choices = Jokers::by_rarity(rarity);
        let i = thread_rng().gen_range(0..choices.len());
        // TODO: don't regenerate already generated jokers.
        // track with hashmap.
        choices[i].clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shop_refresh() {
        let mut shop = Shop::new();
        assert_eq!(shop.jokers.len(), 0);
        shop.refresh();
        assert_eq!(shop.jokers.len(), 2);
    }

    #[test]
    fn test_shop_buy_joker() {
        let mut shop = Shop::new();
        shop.refresh();
        assert_eq!(shop.jokers.len(), 2);
        let j1 = shop.jokers[0].clone();
        assert_eq!(shop.joker_from_index(0).expect("first joker"), j1.clone());
        shop.buy_joker(&j1).expect("buy joker");
    }
}
