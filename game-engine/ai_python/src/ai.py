import abc
from typing import Tuple


class StrategoAI(abc.ABC):
    @abc.abstractmethod
    def ask_next_move(self) -> Tuple[Tuple[int, str], Tuple[int, str]]:
        pass


class TestStrategoAI(StrategoAI):
    def ask_next_move(self) -> Tuple[Tuple[int, str], Tuple[int, str]]:
        return ((3, "A"), (4, "A"))


class MoveBuilder:
    from_x = None
    from_y = None
    to_x = None
    to_y = None
    color = None

    @staticmethod
    def build(builder):
        return Move(builder.from_x, builder.from_y, builder.to_x, builder.to_y, builder.color)

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

    def set_color(self, color):
        self.color = color
        return self


class Move:
    from_x = None
    from_y = None
    to_x = None
    to_y = None
    color = None

    def __init__(self, from_x, from_y, to_x, to_y, color):
        self.from_x = from_x
        self.from_y = from_y
        self.to_x = to_x
        self.to_y = to_y
        self.color = color

    def show(self):
        print('from:(', self.from_x, ',', self.from_y, ')  to:(', self.to_x, ',', self.to_y, ')   color:', self.color)


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
        .set_color(move[2])

    return MoveBuilder.build(moveBuilder)
