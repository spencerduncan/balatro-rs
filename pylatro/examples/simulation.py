import pylatro
import random


def run_game() -> bool:
    config = pylatro.Config()
    config.ante_end = 1
    game = pylatro.GameEngine(config)
    while True:
        if game.is_over:
            break
        moves = game.gen_moves()
        if len(moves) > 0:
            move = random.choice(moves)
            game.handle_action(move)

    if game.is_win:
        print(f"score/required: {game.state.score}/{game.state.required_score}")
        print(game.state)
        print(game.state.action_history)

    return game.is_win


def main():
    for i in range(10_000):
        print(f"running game {i}")
        win = run_game()
        if win:
            break


if __name__ == "__main__":
    main()
