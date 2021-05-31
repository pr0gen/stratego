from typing import Tuple

from ai_python.src.utils import StrategoAI, Move, MoveBuilder, parse_moves, move_ready
import ai_python.src.stratego_engine as se
from ai_python.src.stratego_engine import StrategoBoardWrapper
import random

def choose_randomly(
        board: se.StrategoBoardWrapper,
        color: str
) -> Tuple[Tuple[int, str], Tuple[int, str]]:
        moves = board.get_available_moves_by_color(color)
        movesFormated = parse_moves(moves)
        return move_ready(movesFormated[random.randint(0, len(movesFormated) - 1)])


class RandomAI(StrategoAI):
    name = "random"
    color = str

    def __init__(self, color: str):
        self.color = color

    def ask_next_move(self, board: StrategoBoardWrapper) -> Tuple[Tuple[int, str], Tuple[int, str]]:
        move = choose_randomly(board, self.color)
        print('Color', self.color, 'Move:', move)
        return move
    
