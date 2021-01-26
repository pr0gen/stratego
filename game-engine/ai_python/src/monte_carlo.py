from typing import Tuple
from ai_python.src.utils import StrategoAI, Move, MoveBuilder, parse_moves

import ai_python.src.stratego_engine as se
from ai_python.src.stratego_engine import RustStrategoBoard
from random import *

class MonteCarloAI(StrategoAI):
    def ask_next_move(self, board: RustStrategoBoard) -> Tuple[Tuple[int, str], Tuple[int, str]]:
        moves = board.get_available_moves()
        print(moves)

        # movesFormated = parse_moves(moves)
        # for move in movesFormated:
            # move.show()
        return ((3, "A"), (4, "A"))
