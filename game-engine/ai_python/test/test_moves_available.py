from ai_python.src.ai import parse_move

def test_parsing_moves():

    move = parse_move(
        ((3, '\x00B'), (4, '\x00B'), 'Blue')
    )

    assert move.from_x == 3
    assert move.from_y == 'B'

    assert move.to_x == 4
    assert move.to_y == 'B'

    assert move.color == 'Blue'


