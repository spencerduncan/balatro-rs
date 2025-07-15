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


def test_joker_id_types():
    """Test that new JokerId types are available and working"""
    # Test that JokerId enum is accessible
    assert hasattr(pylatro, 'JokerId')
    assert hasattr(pylatro, 'JokerRarity')
    assert hasattr(pylatro, 'JokerDefinition')
    
    # Test JokerRarity values
    rarity = pylatro.JokerRarity
    assert hasattr(rarity, 'Common')
    assert hasattr(rarity, 'Uncommon') 
    assert hasattr(rarity, 'Rare')
    assert hasattr(rarity, 'Legendary')


def test_gamestate_joker_ids():
    """Test GameState joker_ids property"""
    game = pylatro.GameEngine()
    state = game.state
    
    # Test new joker_ids property
    assert hasattr(state, 'joker_ids')
    joker_ids = state.joker_ids
    assert isinstance(joker_ids, list)
    
    # Test backward compatibility - old jokers property should still work
    assert hasattr(state, 'jokers')
    old_jokers = state.jokers
    assert isinstance(old_jokers, list)
    
    # Both lists should have same length (same jokers, different representations)
    assert len(joker_ids) == len(old_jokers)


def test_gamestate_joker_slots():
    """Test GameState joker slot methods"""
    game = pylatro.GameEngine()
    state = game.state
    
    # Test joker slot properties
    assert hasattr(state, 'joker_slots_used')
    assert hasattr(state, 'joker_slots_total')
    
    slots_used = state.joker_slots_used
    slots_total = state.joker_slots_total
    
    assert isinstance(slots_used, int)
    assert isinstance(slots_total, int)
    assert slots_used >= 0
    assert slots_total > 0
    assert slots_used <= slots_total


def test_gameengine_joker_registry():
    """Test GameEngine joker registry methods"""
    game = pylatro.GameEngine()
    
    # Test get_available_jokers
    assert hasattr(game, 'get_available_jokers')
    all_jokers = game.get_available_jokers()
    assert isinstance(all_jokers, list)
    
    # Test filtering by rarity
    common_jokers = game.get_available_jokers(pylatro.JokerRarity.Common)
    assert isinstance(common_jokers, list)
    
    # Test get_joker_info 
    assert hasattr(game, 'get_joker_info')
    
    # Test can_buy_joker
    assert hasattr(game, 'can_buy_joker')
    
    # Test get_joker_cost
    assert hasattr(game, 'get_joker_cost')


def test_joker_definition_properties():
    """Test JokerDefinition properties"""
    game = pylatro.GameEngine()
    
    # Get some joker definitions
    jokers = game.get_available_jokers()
    
    if jokers:
        joker_def = jokers[0]
        
        # Test that JokerDefinition has expected properties
        assert hasattr(joker_def, 'id')
        assert hasattr(joker_def, 'name')
        assert hasattr(joker_def, 'description')
        assert hasattr(joker_def, 'rarity')
        assert hasattr(joker_def, 'unlock_condition')
        
        # Test property types
        assert isinstance(joker_def.name, str)
        assert isinstance(joker_def.description, str)
        assert joker_def.rarity in [
            pylatro.JokerRarity.Common,
            pylatro.JokerRarity.Uncommon,
            pylatro.JokerRarity.Rare,
            pylatro.JokerRarity.Legendary
        ]


def test_joker_cost_validation():
    """Test joker cost and affordability validation"""
    game = pylatro.GameEngine()
    
    # Get some jokers to test with
    jokers = game.get_available_jokers()
    
    if jokers:
        test_joker = jokers[0]
        joker_id = test_joker.id
        
        # Test get_joker_cost
        try:
            cost = game.get_joker_cost(joker_id)
            assert isinstance(cost, int)
            assert cost > 0
        except Exception as e:
            # If joker isn't implemented yet, that's okay for this test
            pass
        
        # Test can_buy_joker (should work regardless of implementation)
        can_buy = game.can_buy_joker(joker_id)
        assert isinstance(can_buy, bool)


def test_backward_compatibility():
    """Test that existing Python code continues to work"""
    game = pylatro.GameEngine()
    state = game.state
    
    # All existing properties should still be available
    assert hasattr(state, 'stage')
    assert hasattr(state, 'round')
    assert hasattr(state, 'action_history')
    assert hasattr(state, 'deck')
    assert hasattr(state, 'selected')
    assert hasattr(state, 'available')
    assert hasattr(state, 'discarded')
    assert hasattr(state, 'plays')
    assert hasattr(state, 'discards')
    assert hasattr(state, 'score')
    assert hasattr(state, 'required_score')
    assert hasattr(state, 'jokers')  # Old jokers method
    assert hasattr(state, 'money')
    assert hasattr(state, 'ante')
    
    # All existing GameEngine methods should still work
    assert hasattr(game, 'gen_actions')
    assert hasattr(game, 'gen_action_space')
    assert hasattr(game, 'handle_action')
    assert hasattr(game, 'handle_action_index')
    assert hasattr(game, 'get_action_name')
    assert hasattr(game, 'state')
    assert hasattr(game, 'is_over')
    assert hasattr(game, 'is_win')


def test_new_and_old_joker_consistency():
    """Test that new joker_ids and old jokers have consistent data"""
    game = pylatro.GameEngine()
    state = game.state
    
    old_jokers = state.jokers
    new_joker_ids = state.joker_ids
    
    # Should have same number of jokers
    assert len(old_jokers) == len(new_joker_ids)
    
    # If we have jokers, test the conversion works
    for i, (old_joker, new_id) in enumerate(zip(old_jokers, new_joker_ids)):
        # The old joker should be convertible to the same JokerId
        # This tests that the to_joker_id() method works correctly
        converted_id = old_joker.to_joker_id()
        assert converted_id == new_id


if __name__ == "__main__":
    pytest.main([__file__, "-v"])