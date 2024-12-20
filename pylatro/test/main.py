import pylatro
import random


def main():
    game = pylatro.GameEngine()

    while True:
        actions = game.gen_actions()
        if len(actions) == 0:
            break
        action = random.choice(actions)
        game.handle_action(action)

    assert game.is_over
    # state = game.get_state()
    # print(state)
    # print(state.action_history)


if __name__ == "__main__":
    main()
