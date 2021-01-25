from typing import Tuple
from ai_python.src.utils import StrategoAI, Move, MoveBuilder, parse_moves

import ai_python.src.stratego_engine as se
from ai_python.src.stratego_engine import RustStrategoBoard
from random import *

class MonteCarloAI(StrategoAI):
    def ask_next_move(self, board: RustStrategoBoard) -> Tuple[Tuple[int, str], Tuple[int, str]]:

        newBoard = RustStrategoBoard(board.state()) 
        moves = se.rust_get_available_moves(newBoard)

        # movesFormated = parse_moves(moves)
        # for move in movesFormated:
            # move.show()
        return ((3, "A"), (4, "A"))
