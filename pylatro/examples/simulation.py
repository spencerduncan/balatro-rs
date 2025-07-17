import pylatro
import random


def run_game() -> (bool, int):
    config = pylatro.Config()
    config.ante_end = 1
    game = pylatro.GameEngine(config)

    # Run the game
    action_space_loop(game)
    # action_loop(game)

    if game.is_win:
        print("game win!")
        print(game.state)
        # print(game.state.action_history)

    return game.is_win, game.state.score


# Use dynamic action api (dynamic sized list)
# FIXED: Changed parameter type from GameState to GameEngine
def action_loop(game: pylatro.GameEngine):
    while True:
        # FIXED: Use GameEngine.is_over instead of GameState.is_over
        if game.is_over:
            break
        # FIXED: Use GameEngine.gen_actions() instead of GameState.gen_actions()
        actions = game.gen_actions()
        if len(actions) > 0:
            action = random.choice(actions)
            # FIXED: Use GameEngine.handle_action() instead of GameState.handle_action()
            game.handle_action(action)


# Use action space api (static/bounded list)
# FIXED: Changed parameter type from GameState to GameEngine
def action_space_loop(game: pylatro.GameEngine):
    while True:
        # FIXED: Use GameEngine.is_over instead of GameState.is_over
        if game.is_over:
            return
        # FIXED: Use GameEngine.gen_action_space() instead of GameState.gen_action_space()
        space = game.gen_action_space()
        while True:
            if 1 not in set(space):
                # for debugging, this shouldn't happened
                print("empty action space")
                # FIXED: Removed redundant .state access since game is now GameEngine
                print(game.state)
                return
            index = random.choice(range(len(space)))
            if space[index] == 1:
                # FIXED: Use GameEngine.handle_action_index() instead of GameState.handle_action_index()
                game.handle_action_index(index)
                break


# ================================================================
# BACKWARDS COMPATIBILITY DEMONSTRATION
# The following functions show the OLD API usage (now deprecated)
# These will work but generate deprecation warnings
# ================================================================

def deprecated_action_loop_demo():
    """
    DEPRECATED: This demonstrates the old API that still works but shows warnings.
    Use action_loop() above for the correct new API.
    """
    config = pylatro.Config()
    config.ante_end = 1
    game = pylatro.GameEngine(config)
    state = game.state  # Get GameState from GameEngine
    
    # These methods are deprecated but still work with warnings:
    # actions = state.gen_actions()  # DEPRECATED: Shows warning
    # is_over = state.is_over        # DEPRECATED: Shows warning  
    # name = state.get_action_name(0) # DEPRECATED: Shows warning
    
    # These methods are deprecated and will fail:
    # state.handle_action(action)     # DEPRECATED: Fails with error
    # state.handle_action_index(0)    # DEPRECATED: Fails with error


def main():
    high_score = 0
    for i in range(100_000):
        # print(f"running game {i}")
        win, score = run_game()
        if score > high_score:
            high_score = score
            print(f"new high score: {high_score} (game #{i})")


if __name__ == "__main__":
    main()
