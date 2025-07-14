use balatro_rs::action::Action;
use balatro_rs::game::Game;
use text_io::read;

fn input_loop(max: usize) -> usize {
    loop {
        let i: usize = read!();
        if i <= max {
            return i;
        } else {
            println!("Input must be between 0 and {max}")
        }
    }
}

fn game_loop(game: &mut Game) {
    loop {
        if game.is_over() {
            return;
        }
        let actions: Vec<Action> = game.gen_actions().collect();
        println!("Select action:");
        println!("[0] Show game state");
        for (i, action) in actions.clone().iter().enumerate() {
            println!("[{}] {:}", i + 1, action);
        }
        let index = input_loop(actions.len());
        if index == 0 {
            println!("\n{game}");
            continue;
        }
        let action = actions[index - 1].clone();
        game.handle_action(action).expect("handle action");
    }
}

fn main() {
    let mut game = Game::default();
    game.start();
    println!("Starting game...");
    game_loop(&mut game);
    println!("Game over!");
}
