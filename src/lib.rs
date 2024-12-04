pub mod core;

#[cfg(test)]
mod tests {
    use crate::core::action::Action;
    use crate::core::game::Game;
    use crate::core::stage::Stage;

    use rand::Rng;

    #[test]
    fn test_game() {
        let mut g = Game::new();

        g.start();
        loop {
            // Check if game is over
            if let Some(_end) = g.over() {
                break;
            }

            // Get all available moves
            let actions: Vec<Action> = g.gen_moves().collect();
            if actions.len() == 0 {
                break;
            }

            // Pick a random move and execute it
            let i = rand::thread_rng().gen_range(0..actions.len());
            let action = actions[i].clone();
            let action_res = g.handle_action(action.clone());
            assert!(action_res.is_ok());
        }
        // Ensure game is over at end
        assert!(matches!(g.stage, Stage::End(_)));
    }
}
