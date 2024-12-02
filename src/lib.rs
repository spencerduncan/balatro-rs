pub mod core;

#[cfg(test)]
mod tests {
    use crate::core::game::Game;
    use crate::core::moves::Move;
    use crate::core::stage::Stage;

    use rand::seq::SliceRandom;

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
            let moves: Vec<Box<dyn Move>> = g.gen_moves().collect();
            if let Some(m) = moves.choose(&mut rand::thread_rng()) {
                m.apply(&mut g);
            }
        }
        assert!(matches!(g.stage, Stage::End(_)));
    }
}
