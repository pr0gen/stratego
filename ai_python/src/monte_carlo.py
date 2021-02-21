from typing import Tuple, List
from ai_python.src.utils import StrategoAI, Move, MoveBuilder, parse_moves, move_ready
from ai_python.src.utils import basic_material_values
import ai_python.src.stratego_engine as se
from ai_python.src.stratego_engine import StrategoBoardWrapper
from ai_python.src.random import choose_randomly

import copy
import random

class MonteCarloAI(StrategoAI):
    name = "monte_carlo"
    color = str

    def __init__(self, color: str):
        self.color = color

    def ask_next_move(self, board: StrategoBoardWrapper) -> Tuple[Tuple[int, str], Tuple[int, str]]:
        moves = board.get_available_moves_by_color(self.color)
        movesFormated = parse_moves(moves)

        best_move = None
        scores = []
        for move in movesFormated:
            copied_board = board.clone_board()
            f, t = move_ready(move);
            copied_board.moving(f, t)
            sim = copied_board.simulate_games_material(basic_material_values(), self.__get_material_range(), 30, self.color, 4)
            # print(sim)
            scores.append((move, sim))

        scores.sort(reverse=True, key=sort_scores) 

        best_move = scores[0][0]
        if best_move == None or best_move == False:
            best_move = choose_randomly(board, self.color)
        return move_ready(best_move)

    def __get_material_range(self) -> List[int]:
       return list(range(0, 20))

def sort_scores(e):
    return e[1]

