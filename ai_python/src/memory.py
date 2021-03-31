from typing import Tuple


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
    uuid = None
    moved = False
    scout = False
    value = None
    position = Tuple[int, int]

    def __init__(self, x, y):
        self.position = (x, y)

    def show(self):
        print('piece : uuid ' + str(self.uuid) + ',  moved ' + str(self.moved) + ', scout ' + str(self.scout))


class Cache:
    pieces = []

    def __init__(self, color):

        for x in range(10):
            for y in range(4):
                temp = 0
                if color == 'Red':
                    temp += 5
                self.pieces.append(PieceCache(x,y + temp))

    def get_piece(self, x, y):
        for piece in self.pieces:
            original_x, original_y = piece.position
            if original_x == x and original_y == y:
                return piece
        raise NameError('uuid does not exist')

    def update_piece(self, x_from,y_from, x_to,y_to):
        piece = self.get_piece(x_from,y_from)
        piece.position = (x_to,y_to)


class Probability:
    cache = None
    pieceManager = None

    data = {}

    def __init__(self, cache, pieceManager):
        self.cache = cache
        self.pieceManager = pieceManager

    def init(self):
        return {
            'marshal': 0,
            'general': 0,
            'colonel': 0,
            'major': 0,
            'captain': 0,
            'lieutenant': 0,
            'sergeant': 0,
            'miner': 0,
            'scout': 0,
            'spy': 0,
            'flag': 0,
            'bomb': 0,
        }

    def get_probability(self, x, y):
        piece: PieceCache = self.cache.get_piece(x, y)
        pourcent = 100

        probabilities = self.init()

        if piece.scout:
            probabilities['scout'] = 100
            return probabilities

        if piece.value is not None:
            switcher = {
                10: 'marshal',
                9: 'general',
                8: 'colonel',
                7: 'major',
                6: 'captain',
                5: 'lieutenant',
                4: 'sergeant',
                3: 'miner',
                2: 'scout',
                1: 'spy',
                0: 'flag',
                -1: 'bomb',
            }
            piece_name = switcher.get(piece.value)
            probabilities[piece_name] = 100
            return probabilities

        return probabilities


# cache = Cache()
# pieceManager = PiecesManager()
# # cache.get_piece(5).show()
#
# p = Probability(cache, pieceManager)
# p.getProbability()
