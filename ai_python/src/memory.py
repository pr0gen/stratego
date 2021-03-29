
class PiecesManager():
    pieces = {}

    def __init__(self):
        self.pieces = {
            'marshal': 1,
            'general': 1,
            'colonel': 2,
            'major': 3,
            'captain': 4,
            'lieutenant': 4,
            'sergeant': 4,
            'miner': 5,
            'scout': 8,
            'spy': 1,
            'flag': 1,
            'bomb': 6,
        }

    def remove_one_piece(self, piece_name):
        self.pieces[piece_name] -= 1


class PieceCache:
    uuid = None,
    moved = False,
    scout = False
    int = None

    def __init__(self, uuid):
        self.uuid = uuid

    def show(self):
        print('piece : uuid ' + str(self.uuid) + ',  moved ' + str(self.moved) + ', scout ' + str(self.scout))


i = 1


class Cache:
    pieces = []

    def __init__(self):
        for i in range(40):
            i += 1
            uuid = i
            self.pieces.append(PieceCache(uuid))

    def get_piece(self, uuid):
        for piece in self.pieces:
            if piece.uuid == uuid:
                return piece
        raise NameError('uuid does not exist')


cache = Cache()
print(cache.get_piece(5))
