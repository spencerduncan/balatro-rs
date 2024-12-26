import pylatro
import random


def run_game() -> bool:
    config = pylatro.Config()
    config.ante_end = 1
    game = pylatro.GameEngine(config)

    # Run the game
    action_space_loop(game)
    # action_loop(game)

    if game.is_win:
        print(f"score/required: {game.state.score}/{game.state.required_score}")
        print(game.state)
        print(game.state.action_history)

    return game.is_win


# Use dynamic action api (dynamic sized list)
def action_loop(game: pylatro.GameState):
    while True:
        if game.is_over:
            break
        actions = game.gen_actions()
        if len(actions) > 0:
            action = random.choice(actions)
            game.handle_action(action)


# Use action space api (static/bounded list)
def action_space_loop(game: pylatro.GameState):
    while True:
        if game.is_over:
            break
        space = game.gen_action_space()
        while True:
            if 1 not in set(space):
                print("NO UNMASKED ACTIONS IN SPACE")
                print(space)
                print(game.state)
                print(game.state.action_history)
                return
            if len(space) == 0:
                print(f"ACTION SPACE LEN 0 {space}")
                break
            index = random.choice(range(len(space)))
            if space[index] == 1:
                game.handle_action_index(index)
                break


def main():
    for i in range(10_000):
        print(f"running game {i}")
        win = run_game()
        if win:
            break


if __name__ == "__main__":
    main()
