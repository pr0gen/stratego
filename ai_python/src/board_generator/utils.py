from ai_python.src.board_generator.piece import Piece
from ai_python.src.board_generator.position import Position
from collections import defaultdict


def piece_to_text(piece):
    if piece == Piece.Marshal:
        return '| Mars '
    if piece == Piece.General:
        return '| Gene '

    if piece == Piece.Colonel:
        return '| Colo '

    if piece == Piece.Major:
        return '| Majo '

    if piece == Piece.Captain:
        return '| Capt '

    if piece == Piece.Lieutenant:
        return '| Lieu '

    if piece == Piece.Sergeant:
        return '| Serg '

    if piece == Piece.Miner:
        return '| Mine '

    if piece == Piece.Scout:
        return '| Scou '

    if piece == Piece.Spy:
        return '| Spy  '

    if piece == Piece.Flag:
        return '| Flag '

    if piece == Piece.Bomb:
        return '| Bomb '


def piece_to_text_for_file(piece):
    if piece == Piece.Marshal:
        return 'Mars'
    if piece == Piece.General:
        return 'Gene'

    if piece == Piece.Colonel:
        return 'colo'

    if piece == Piece.Major:
        return 'majo'

    if piece == Piece.Captain:
        return 'capt'

    if piece == Piece.Lieutenant:
        return 'lieu'

    if piece == Piece.Sergeant:
        return 'serg'

    if piece == Piece.Miner:
        return 'mine'

    if piece == Piece.Scout:
        return 'scou'

    if piece == Piece.Spy:
        return 'spy'

    if piece == Piece.Flag:
        return 'flag'

    if piece == Piece.Bomb:
        return 'bomb'


def get_position_by_index(index):
    y = (int)(index / 10)
    x = index % 10
    return Position(x, y)


def get_index_by_position(position):
    return position.y * 10 + position.x


def nested_dict(n, type):
    if n == 1:
        return defaultdict(type)
    else:
        return defaultdict(lambda: nested_dict(n - 1, type))


def get_index_by_position(position):
        return position.y * 10 + position.x

def get_all_neighboor_index_from_position( p):
        neighboor = []

        if p.x > 0:
            neighboor.append(p.y * 10 + (p.x - 1))

        if p.y > 0:
            neighboor.append((p.y - 1) * 10 + p.x)

        if p.x < 9:
            neighboor.append(p.y * 10 + (p.x + 1))

        if p.y < 3:
            neighboor.append((p.y + 1) * 10 + p.x)

        return neighboor
