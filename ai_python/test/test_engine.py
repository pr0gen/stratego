from ai_python.src.engine import Player, Color, Engine, GamePool, Game
from ai_python.src.utils import basic_material_values
import ai_python.src.stratego_engine as se
import copy

def test_build_player(): 
    player = Player("Tigran", Color.Blue) 
    assert player.name == "Tigran"
    assert player.color == Color.Blue

def test_build_engine():
    engine = Engine((Player("Tigran", Color.Blue), Player("Arthur", Color.Red)), se.rust_create_empty_stratego_board())
    player_0 = engine.players[0]
    assert "Tigran" == player_0.name
    assert Color.Blue == player_0.color

    player_1 = engine.players[1]
    assert "Arthur" == player_1.name
    assert Color.Red == player_1.color
    assert Color.Red == engine.color


def test_build_pool():
    engine = Engine((Player("Tigran", Color.Blue), Player("Arthur", Color.Red)), se.rust_create_empty_stratego_board())
    game_pool = GamePool([])   
    game_pool.register(Game(engine, 0))
    assert len(game_pool.pool) == 1


def test_find_game_by_uuid():
    engine = Engine((Player("Tigran", Color.Blue), Player("Arthur", Color.Red)), se.rust_create_empty_stratego_board())
    game_pool = GamePool([Game(engine, 0)])   

    game = game_pool.find_game(0)
    
    assert game.uuid == 0

def test_copy_board():

    board = se.rust_create_empty_stratego_board()
    board.place("Full", (1, "A"), 1, "Red")
    copied_board = board.clone_board();
    case = board.at((1, "A"))
    board.moving((1, "A"), (2, "A")) # move a case

    assert board.at((1, "A")).py_get_state() != copied_board.at((1, "A")).py_get_state()
    assert board.at((1, "A")).py_get_content() != copied_board.at((1, "A")).py_get_content()


def test_material_evaluation():
    board = se.rust_create_empty_stratego_board()
    res = board.material_evaluation(basic_material_values())

    assert res[0] == ("Red", 0)
    assert res[1] == ("Blue", 0)


def test_basic_evaluation():
    board = se.rust_create_stratego_board()
    res = board.basic_evaluation()

    assert res == "None"
