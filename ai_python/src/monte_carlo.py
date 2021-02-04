from typing import Tuple
from ai_python.src.utils import StrategoAI, Move, MoveBuilder, parse_moves, move_ready

import ai_python.src.stratego_engine as se
from ai_python.src.stratego_engine import RustStrategoBoard
import random
import copy

class MonteCarloAI(StrategoAI):
    color = str

    def __init__(self, color: str):
        self.color = color

    def ask_next_move(self, board: RustStrategoBoard) -> Tuple[Tuple[int, str], Tuple[int, str]]:
        moves = board.get_available_moves_by_color(self.color)

        movesFormated = parse_moves(moves)

        print(movesFormated)
        index = random.randint(0, len(movesFormated) - 1)

        isThereAWinningMove = False

        for move in movesFormated:
            copied_board = board.clone_board()
            parsed_move = parse_move(move)
            isThereAWinningMove = simulateGame(parsed_move, copied_board)
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

    def simulateGame(move, copied_board) -> bool :
        copied_board.moving((move.from_x, move.from_y), (move.to_x, move.to_y)
        while board.rust_basic_evaluation != null :
            moves = copied_board.get_available_moves_by_color(self.color)
            movesFormated = parse_moves(moves)
            index = random.randint(0, len(movesFormated) - 1)
            current_move = moving.movesFormated[index]
            copied_board((current_move.from_x, current_move.from_y) , (current_move.to_x , current_move.to_y)

        if (copied_board.rust_basic_evaluation == self.color):
            return true
        return false
