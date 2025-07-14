import pytest
import pylatro
import random


def test_game_creation():
    """Test basic game engine creation"""
    game = pylatro.GameEngine()
    assert game is not None
    assert not game.is_over
    

def test_game_state():
    """Test game state access"""
    game = pylatro.GameEngine()
    state = game.state
    
    # Check state has expected attributes
    assert hasattr(state, 'round')
    assert hasattr(state, 'ante')
    assert hasattr(state, 'money')
    assert hasattr(state, 'score')
    

def test_action_space():
    """Test action space generation and basic gameplay"""
    game = pylatro.GameEngine()
    
    # Play a few rounds
    moves = 0
    max_moves = 1000  # Prevent infinite loops in tests
    
    while not game.is_over and moves < max_moves:
        action_space = game.gen_action_space()
        assert isinstance(action_space, list)
        assert len(action_space) > 0
        
        # Find valid actions
        valid_actions = [i for i, valid in enumerate(action_space) if valid == 1]
        assert len(valid_actions) > 0, "Should have at least one valid action"
        
        # Execute a random valid action
        action_idx = random.choice(valid_actions)
        game.handle_action_index(action_idx)
        moves += 1
    
    assert game.is_over
    

def test_action_names():
    """Test action name generation"""
    game = pylatro.GameEngine()
    
    # Get some actions
    action_space = game.gen_action_space()
    valid_actions = [i for i, valid in enumerate(action_space) if valid == 1]
    
    if valid_actions:
        action_idx = valid_actions[0]
        action_name = game.get_action_name(action_idx)
        assert isinstance(action_name, str)
        assert len(action_name) > 0


def test_game_completion():
    """Test that games can complete with win or loss"""
    game = pylatro.GameEngine()
    
    # Play until game over
    while not game.is_over:
        action_space = game.gen_action_space()
        valid_actions = [i for i, valid in enumerate(action_space) if valid == 1]
        if not valid_actions:
            break
        action_idx = random.choice(valid_actions)
        game.handle_action_index(action_idx)
    
    assert game.is_over
    # Game should either be won or lost
    assert hasattr(game, 'is_win')


def test_multiple_games():
    """Test running multiple games to ensure stability"""
    for _ in range(5):
        game = pylatro.GameEngine()
        moves = 0
        max_moves = 500
        
        while not game.is_over and moves < max_moves:
            action_space = game.gen_action_space()
            valid_actions = [i for i, valid in enumerate(action_space) if valid == 1]
            if not valid_actions:
                break
            action_idx = random.choice(valid_actions)
            game.handle_action_index(action_idx)
            moves += 1
        
        assert game.is_over or moves >= max_moves


if __name__ == "__main__":
    pytest.main([__file__, "-v"])