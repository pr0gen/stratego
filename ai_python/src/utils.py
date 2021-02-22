import abc
import random
import string
from typing import Tuple, List
import ai_python.src.stratego_engine as se

Move = Tuple[int, str]

class StrategoAI(abc.ABC):
    @abc.abstractmethod
    def ask_next_move(self, board: se.StrategoBoardWrapper) -> Tuple[Move, Move]:
        pass


class TestStrategoAI(StrategoAI):
    def ask_next_move(self, board: se.StrategoBoardWrapper) -> Tuple[Move, Move]:
        return ((3, "A"), (4, "A"))


def sort_evals(evals, color):
    res = []
    for eval in evals:
        if color == "Red":
            res.append(eval['Material'][0][1])
        else:
            res.append(eval['Material'][1][1])
    res.sort(reverse=True)
    return res[0]
          

def get_best_score_by_move(score, color):
    move, evals = score
    return move, sort_evals(evals, color)


def sort_best_scores(score):
    return score[1]


def basic_material_values() -> List[Tuple[int, int]]: 
    return [ (-2, 0),
      (10, 10),
      (9, 9),
      (8, 8),
      (7, 7),
      (6, 6),
      (5, 5),
      (4, 4),
      (3, 3),
      (2, 2),
      (1, 1),
      (-1, 1)
    ]

def parse_board(board: List[List[str]]):
    # TODO
    return se.rust_create_stratego_board()


def move_ready(move) -> Tuple[Tuple[int, str], Tuple[int, str]]:
    return (move.from_x, move.from_y), (move.to_x, move.to_y)


def generate_uuid(length) -> str:
    return ''.join(random.choices(string.ascii_uppercase + string.digits, k = length))



def parse_moves(data):
    moves = []

    for move in data:
        moves.append(parse_move(move))

    return moves


def parse_move(move):
    moveBuilder = MoveBuilder()

    moveBuilder \
        .set_from_x(move[0][0]) \
        .set_from_y(move[0][1][1]) \
        .set_to_x(move[1][0]) \
        .set_to_y(move[1][1][1]) \
        .set_color_from(move[2]) \
        .set_color_to(move[3])

    return MoveBuilder.build(moveBuilder)



class MoveBuilder:
    from_x = None
    from_y = None
    to_x = None
    to_y = None
    color_from = None
    color_to = None

    @staticmethod
    def build(builder):
        return Move(builder.from_x, builder.from_y, builder.to_x, builder.to_y, builder.color_from, builder.color_to)

    def set_from_x(self, from_x):
        self.from_x = from_x
        return self

    def set_from_y(self, from_y):
        self.from_y = from_y
        return self

    def set_to_x(self, to_x):
        self.to_x = to_x
        return self

    def set_to_y(self, to_y):
        self.to_y = to_y
        return self

    def set_color_from(self, color):
        self.color_from = color
        return self

    def set_color_to(self, color):
        self.color_to = color
        return self


class Move:
    from_x = None
    from_y = None
    to_x = None
    to_y = None
    color_from = None
    color_to = None

    def __init__(self, from_x, from_y, to_x, to_y, color_from, color_to):
        self.from_x = from_x
        self.from_y = from_y
        self.to_x = to_x
        self.to_y = to_y
        self.color_from = color_from
        self.color_to = color_to

    def show(self):
        print('from:(', self.from_x, ',', self.from_y, ')  to:(', self.to_x, ',', self.to_y, ')   color:', self.color_from, 'color_to:', self.color_to)


