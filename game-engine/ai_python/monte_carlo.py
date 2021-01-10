from typing import Tuple

from ai import StrategoAI
from stratego_engine import register_game, get_available_moves, get_game_state
from random import *

class MonteCarloAI(StrategoAI):
    def ask_next_move(self) -> Tuple[Tuple[int, str], Tuple[int, str]]:
        game_id = register_game("Tigran", "test2")
        moves = get_available_moves(game_id)
        print("moves" , moves)
        return moves[randint(0, len(moves) - 1)]

monte_carlo = MonteCarloAI()
print(monte_carlo.ask_next_move())

