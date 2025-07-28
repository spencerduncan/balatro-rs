// Temporary lint overrides - see GitHub issues for tracking
#![warn(clippy::field_reassign_with_default)]
#![warn(unused_variables)]
#![warn(dead_code)]

pub mod action;
pub mod ante;
pub mod available;
pub mod basic_chips_jokers;
pub mod boss_blinds;
pub mod bounded_action_history;
pub mod card;
pub mod card_filter;
pub mod config;
pub mod consumables;
pub mod deck;
pub mod display;
pub mod error;
pub mod game;
pub mod generator;
pub mod hand;
pub mod joker;
pub mod joker_effect_processor;
pub mod joker_factory;
pub mod joker_impl;
pub mod joker_metadata;
pub mod joker_migration_tool;
pub mod joker_registry;
pub mod joker_state;
pub mod joker_toml_parser;
pub mod joker_toml_schema;
pub mod math_safe;
pub mod memory_monitor;
pub mod multi_select;
pub mod priority_strategy;
pub mod rank;
pub mod rng;
pub mod scaling_chips_jokers;
pub mod scaling_joker;
pub mod scaling_joker_custom;
pub mod scaling_joker_impl;
pub mod shop;
pub mod space;
pub mod special_jokers;
pub mod stage;
pub mod state_version;
pub mod static_joker;
pub mod static_joker_factory;
pub mod target_context;
pub mod vouchers;

#[cfg(test)]
mod tests {
    use crate::action::Action;
    use crate::game::Game;
    use crate::stage::Stage;

    #[test]
    // Test executing a full game using the gen_actions api
    fn test_game_gen_actions() {
        let mut g = Game::default();

        g.start();
        while !g.is_over() {
            // Get all available actions
            let actions: Vec<Action> = g.gen_actions().collect();
            if actions.is_empty() {
                break;
            }

            // Pick a random move and execute it
            let i = g.rng.gen_range(0..actions.len());
            let action = actions[i].clone();
            dbg!("game state:\n{}", &g);
            dbg!("play action: {}", action.clone());
            let action_res = g.handle_action(action.clone());
            dbg!(action);
            assert!(action_res.is_ok());
        }
        let result = g.result();
        // Ensure game is over at end
        assert!(result.is_some());
        // Check game state at end
        assert!(matches!(g.stage, Stage::End(_)));
        dbg!("game action history: {:?}", g.action_history);
    }

    #[test]
    // Test executing a full game using the gen_action_space (vector) api
    fn test_game_action_space() {
        let mut g = Game::default();

        g.start();
        while !g.is_over() {
            // Get action space and vector
            let space = g.gen_action_space();
            let space_vec = space.to_vec();
            if space.is_empty() {
                break;
            }

            // Pick a random move and ensure its unmasked
            let mut i: usize;
            loop {
                i = g.rng.gen_range(0..space_vec.len());
                if space_vec[i] == 1 {
                    break;
                }
            }
            let action = space.to_action(i, &g).expect("valid index to action");
            dbg!("game state:\n{}", &g);
            dbg!("play action: {}", action.clone());
            let action_res = g.handle_action(action.clone());
            dbg!(action);
            assert!(action_res.is_ok());
        }
        let result = g.result();
        // Ensure game is over at end
        assert!(result.is_some());
        // Check game state at end
        assert!(matches!(g.stage, Stage::End(_)));
        dbg!("game action history: {:?}", g.action_history);
    }
}
