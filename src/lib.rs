pub mod core;

#[cfg(test)]
mod tests {
    use crate::core::action::Action;
    use crate::core::game::Game;
    use crate::core::stage::Stage;

    use rand::Rng;

    #[ignore]
    #[test]
    fn test_game() {
        let mut g = Game::new();

        dbg!("game state before start {:?}", g.clone());
        g.start();
        dbg!("game state start {:?}", g.clone());
        loop {
            if let Some(end) = g.over() {
                dbg!("game over {:?}", end);
                break;
            }
            let actions: Vec<Action> = g.gen_moves().collect();
            if actions.len() == 0 {
                break;
            }
            let i = rand::thread_rng().gen_range(0..actions.len());
            let action = actions[i].clone();
            let action_res = g.handle_action(action.clone());
            dbg!("handled action: {}", action);
            // dbg!("game state {:?}", g.clone());
            debug_assert!(action_res.is_ok());
        }
        // Ensure game is over at end
        dbg!("end game state {:?}", g.clone());
        debug_assert!(matches!(g.stage, Stage::End(_)));
    }
}
