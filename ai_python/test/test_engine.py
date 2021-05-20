from ai_python.src.engine import Color, GamePool, Game
from ai_python.src.utils import basic_material_values
import ai_python.src.stratego_engine as se
import copy

def test_build_pool():
    board = se.rust_create_empty_stratego_board()
    game_pool = GamePool([]) 
    game_pool.register(Game(board, 0))
    assert len(game_pool.pool) == 1


def test_find_game_by_uuid():
    board = se.rust_create_empty_stratego_board()
    game_pool = GamePool([Game(board, 0)])   

    game = game_pool.find_game(0)
    
    assert game.uuid == 0

def test_copy_board():
    board = se.rust_create_empty_stratego_board()
    board.place("Full", (1, "A"), 1, "Red")
    copied_board = board.clone_board();
    board.moving((1, "A"), (2, "A")) # move a case

    case = board.at((1, "A"))
    assert case['state'] != copied_board.at((1, "A"))['state']
    assert case['content'] != copied_board.at((1, "A"))['content']


def test_material_evaluation():
    board = se.rust_create_empty_stratego_board()
    res = board.material_evaluation(basic_material_values())

    assert res[0] == ("Red", 0)
    assert res[1] == ("Blue", 0)


def test_basic_evaluation():
    board = se.rust_create_stratego_board()
    res = board.basic_evaluation()

    assert res == "None"


def test_last_coup():
    board = se.rust_create_empty_stratego_board()
    board.place("Full", (1, "A"), 1, "Red")
    board.moving((1, "A"), (2, "A")) # move a case
    
    last_coup, won = board.get_last_coup()
    _from = last_coup[0]
    assert 'Empty' == _from['state']
    assert {'x': 1, 'y': 0} == _from['coordinate']
    assert {'m': (0, 0), 'rank': 'Null', 'color': 'None'} == _from['content']
    assert won == False
    
    to = last_coup[1]
    assert 'Full' == to['state']
    assert {'x': 2, 'y': 0} == to['coordinate']
    assert {'m': (0, 1), 'rank': 'Spy', 'color': 'Red'} == to['content']
    

def test_first_last_coup():
    board = se.rust_create_empty_stratego_board()
    last_coup = board.get_last_coup()
    assert None == last_coup
    
