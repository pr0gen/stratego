from ai_python.src.utils import parse_moves, generate_uuid

def test_parsing_moves():

    moves = parse_moves([((3, '\x00B'), (4, '\x00B'), 'Blue', 'None')])

    move = moves[0] 
    assert move.from_x == 3
    assert move.from_y == 'B'

    assert move.to_x == 4
    assert move.to_y == 'B'

    assert move.color_from == 'Blue'
    assert move.color_to == 'None'


def test_generate_uuid():
    uuid = generate_uuid(30)
    assert len(uuid) == 30
