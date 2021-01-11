from ai_python.stratego_engine import register_game, get_available_moves, get_game_state
from ai_python.ai import parse_moves


def test_parsing_moves():
    game_id = register_game("Tigran", "test2")
    moves = get_available_moves(game_id)
    movesFormated = parse_moves(moves)
    move = movesFormated[0]

    assert move.from_x == 3
    assert move.from_y == 'A'

    assert move.to_x == 4
    assert move.to_y == 'A'

    assert move.color == 'Blue'


