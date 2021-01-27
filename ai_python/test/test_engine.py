from ai_python.src.engine import Player, Color, Engine, GamePool, Game
import ai_python.src.stratego_engine as se

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

