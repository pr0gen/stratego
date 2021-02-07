from typing import Tuple
from ai_python.src.utils import StrategoAI, Move, MoveBuilder, parse_moves, move_ready
from ai_python.src.ai_utils import simulate_game
import ai_python.src.stratego_engine as se
from ai_python.src.stratego_engine import StrategoBoardWrapper
from ai_python.src.random import choose_randomly

import copy
import random

class MonteCarloAI(StrategoAI):
    color = str

    def __init__(self, color: str):
        self.color = color

    def ask_next_move(self, board: StrategoBoardWrapper) -> Tuple[Tuple[int, str], Tuple[int, str]]:
        moves = board.get_available_moves_by_color(self.color)
        movesFormated = parse_moves(moves)
        index = random.randint(0, len(movesFormated) - 1)

        best_move = None
        for move in movesFormated:
            copied_board = board.clone_board()
            f, t = move_ready(move);
            copied_board.moving(f, t)
            best_move = simulate_game(
                copied_board,
                choose_randomly,
                choose_randomly,
                board.basic_evaluation(),
                100,
                (self.color),
                self.color
            )
        
        if best_move == None:
            best_move = choose_randomly(board, self.color)

        print("Monte Carlo plays:", best_move)
        return best_move



