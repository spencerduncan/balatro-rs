pub mod action;
pub mod ante;
pub mod card;
pub mod deck;
pub mod effect;
pub mod error;
pub mod game;
pub mod hand;
pub mod rank;
pub mod stage;

#[cfg(test)]
mod tests {
    use crate::action::Action;
    use crate::game::Game;
    use crate::stage::Stage;

    use rand::Rng;

    #[test]
    fn test_game() {
        let mut g = Game::new();

        g.start();
        while !g.is_over() {
            // Get all available moves
            let actions: Vec<Action> = g.gen_moves().collect();
            if actions.len() == 0 {
                break;
            }

            // Pick a random move and execute it
            let i = rand::thread_rng().gen_range(0..actions.len());
            let action = actions[i].clone();
            println!("game state:\n{}", g.clone());
            println!("play action: {}", action.clone());
            let action_res = g.handle_action(action);
            debug_assert!(action_res.is_ok());
        }
        let result = g.result();
        // Ensure game is over at end
        assert!(result.is_some());
        // Check game state at end
        assert!(matches!(g.stage, Stage::End(_)));
        println!("game action history: {:?}", g.action_history);
    }
}
