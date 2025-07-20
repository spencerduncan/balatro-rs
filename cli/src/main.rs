use balatro_rs::action::Action;
use balatro_rs::game::Game;
use std::io::{self, Write};

#[derive(Debug)]
enum InputError {
    IoError(io::Error),
    TooManyAttempts,
}

impl From<io::Error> for InputError {
    fn from(error: io::Error) -> Self {
        InputError::IoError(error)
    }
}

fn secure_input_loop(max: usize) -> Result<usize, InputError> {
    const MAX_ATTEMPTS: usize = 3;
    const MAX_INPUT_LENGTH: usize = 10;

    for attempt in 1..=MAX_ATTEMPTS {
        print!("Enter choice (0-{max}): ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        // Check input length to prevent memory attacks
        if input.trim().len() > MAX_INPUT_LENGTH {
            println!("Input too long. Attempt {attempt}/{MAX_ATTEMPTS}");
            continue;
        }

        // Parse and validate input
        match input.trim().parse::<usize>() {
            Ok(i) if i <= max => return Ok(i),
            Ok(_) => println!("Must be 0-{max}. Attempt {attempt}/{MAX_ATTEMPTS}"),
            Err(_) => println!("Invalid number. Attempt {attempt}/{MAX_ATTEMPTS}"),
        }
    }

    Err(InputError::TooManyAttempts)
}

fn input_loop(max: usize) -> usize {
    match secure_input_loop(max) {
        Ok(value) => value,
        Err(InputError::TooManyAttempts) => {
            println!("Too many invalid attempts. Exiting for security.");
            std::process::exit(1);
        }
        Err(InputError::IoError(e)) => {
            println!("IO error: {e}. Exiting.");
            std::process::exit(1);
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
