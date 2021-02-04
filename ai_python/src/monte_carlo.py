from typing import Tuple
from ai_python.src.utils import StrategoAI, Move, MoveBuilder, parse_moves, move_ready

import ai_python.src.stratego_engine as se
from ai_python.src.stratego_engine import StrategoBoardWrapper
import random
import copy

class MonteCarloAI(StrategoAI):
    color = str

    def __init__(self, color: str):
        self.color = color

    def ask_next_move(self, board: StrategoBoardWrapper) -> Tuple[Tuple[int, str], Tuple[int, str]]:
        moves = board.get_available_moves_by_color(self.color)

        movesFormated = parse_moves(moves)
        print(board.basic_evaluation())
        index = random.randint(0, len(movesFormated) - 1)

        isThereAWinningMove = False

        for move in movesFormated:

            copied_board = board.clone_board()
            isThereAWinningMove = simulateGame(move, copied_board)
            if(isThereAWinningMove):
                winning_move = move
                break

        if(isThereAWinningMove):
            winning_move = movesFormated[index]
            winning_move.show()
            return move_ready(winning_move)
        else:
            move = movesFormated[index]
            move.show()
            return move_ready(move)

    def simulateGame(move, board) -> bool :
        # TODO
        return true
