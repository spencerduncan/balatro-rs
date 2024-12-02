pub mod core;

#[cfg(test)]
mod tests {
    use crate::core::action::Action;
    use crate::core::game::Game;
    use crate::core::stage::Stage;

    use rand::Rng;

    // not working yet, wip
    #[ignore]
    #[test]
    fn test_game() {
        let mut g = Game::new();

        // game.start
        loop {
            if let Some(end) = g.over() {
                println!("Game over {:?}", end);
                break;
            }
            let actions: Vec<Action> = g.gen_moves().collect();
            let i = rand::thread_rng().gen_range(0..actions.len());
            let rand_action = actions[i].clone();
            let action_res = g.handle_action(rand_action);
            assert!(action_res.is_ok());
        }
        // Ensure game is over at end
        assert!(matches!(g.stage, Stage::End(_)));
    }
}
