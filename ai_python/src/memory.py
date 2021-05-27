from typing import Tuple, List

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
    moved = False
    value = None
    position = Tuple[int, int]

    def __init__(self, x, y):
        self.position = (x, y)

    def is_scout(self) -> bool:
        return self.value == 2

    def show(self):
        print('piece: moved ' + str(self.moved) + ', value ' + str(self.value) + ',position ' + str(self.position))


def create_pieces(color, width, height) -> List[PieceCache]:
    pieces = []
    for x in range(width):
        for y in range(height):
            temp = 0
            if color == 'Blue':
                temp += (int) (height / 2) + 1
            pieces.append(PieceCache(x + temp, y))
    return pieces

class CacheException(Exception):
    
    def __init__(self, message: str):
        self.message = message


### This object is a Singleton
class Cache(object):
    _instance = None
    _pieces = []

    def __init__(self):
        raise RuntimeError('Call instance() instead')


    @classmethod
    def instance(cls, color, width, height):
        if cls._instance is None:
            cls._instance = cls.__new__(cls)

            # Put any initialization here.
            cls._pieces = create_pieces(color, width, height)

        return cls._instance


    def reset_cache(self, color, width, height):
        # Mainly for Tests pourpuses
            self._pieces = create_pieces(color, width, height)
        

    def get_piece(self, coord: Tuple[int, int]):
        x, y = coord
        for piece in self._pieces:
            original_x, original_y = piece.position
            if original_x == x and original_y == y:
                return piece
        raise CacheException('piece does not exist')


    def update_piece(self, _from: Tuple[int, int], to: Tuple[int, int], value=None):
        piece = self.get_piece(_from)
        piece.moved = True
        if value is not None:
            piece.value = value
        piece.position = (to[0], to[1])


    def delete_piece(self, coord: Tuple[int, int]):
        x, y = coord
        print(x, y)
        new_pieces = []
        for piece in self._pieces:
            original_x, original_y = piece.position
            if original_x != x or original_y != y:
                new_pieces.append(piece)
            else:
                print('Deleting:', coord)
        self._pieces = new_pieces


    def show(self):
        print('=== Display cache ===')
        for piece in self._pieces:
            piece.show()



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

