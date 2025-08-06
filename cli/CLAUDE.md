# CLAUDE.md - CLI Package

## Directory Purpose

The CLI package provides a simple command-line interface for playing Balatro interactively. It serves as a text-based frontend to the core game engine, allowing users to play complete games through terminal input/output.

## Architecture

### Minimal Design
- Single `main.rs` file with ~90 lines of focused code
- Direct integration with core engine
- No abstraction layers or complex state management

### Security-First Input Handling
Implements secure input validation with:
- Max 3 retry attempts for invalid input
- Input length limits (10 characters max) to prevent memory attacks
- Graceful error handling with typed `InputError` enum
- Process termination on repeated failures

## Key Components

### Main Functions
1. **`secure_input_loop()`**: Hardened input validation with security controls
2. **`input_loop()`**: Wrapper providing clean error handling and program exit
3. **`game_loop()`**: Core game loop that:
   - Generates available actions from the engine
   - Displays action menu with indices
   - Special index 0 for showing game state
   - Executes selected actions

## Dependencies

```toml
[dependencies]
colored = "2.2.0"  # Terminal color output
balatro-rs = {path = "../core/", features = ["colored"]}
```

- Minimal dependencies for simplicity
- Enables "colored" feature in core library for enhanced display

## Usage

### Running the CLI
```bash
# From workspace root
cargo run -p balatro-cli

# Direct execution
./target/debug/balatro-cli
```

### Interaction Flow
1. Game starts automatically with default configuration
2. Player sees numbered menu of available actions
3. Enter 0 to view current game state
4. Enter 1-N to execute specific action
5. Game continues until win/loss condition

### Security Features
- Input validation prevents buffer overflow attacks
- Retry limits prevent infinite loop DoS
- Clean error messages guide valid input

## Core Library Integration

### Direct Game API Usage
```rust
use balatro_rs::action::Action;
use balatro_rs::game::Game;

// Simple initialization
let mut game = Game::default();
game.start();

// Action generation and execution
let actions: Vec<Action> = game.gen_actions().collect();
game.handle_action(action)?;
```

### Key Integration Points
- Uses `Game::gen_actions()` for move generation
- Direct `handle_action()` calls for state mutations
- Leverages Display traits for game state visualization
- No complex state management or caching

## Project Structure

```
cli/
├── Cargo.toml      # Package configuration
├── src/
│   └── main.rs     # Single source file
└── tests/
    └── test.rs     # Basic integration tests
```

## Development

### Building
```bash
cargo build -p balatro-cli
cargo build -p balatro-cli --release
```

### Testing
```bash
cargo test -p balatro-cli
```

## Design Philosophy

The CLI follows a minimalist approach:
- **Simple**: Direct user interaction without complexity
- **Secure**: Every input path validated and bounded
- **Focused**: Only handles user I/O, delegates logic to core
- **Educational**: Clear code structure for learning the engine
