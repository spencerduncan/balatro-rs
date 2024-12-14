import pylatro
import random


def main():
    game = pylatro.GameEngine()

    while True:
        moves = game.gen_moves()
        if len(moves) == 0:
            break
        move = random.choice(moves)
        game.handle_action(move)

    assert game.is_over
    # state = game.get_state()
    # print(state)
    # print(state.action_history)


if __name__ == "__main__":
    main()
