from typing import Tuple

from ai import StrategoAI, Move, MoveBuilder, parse_moves

from stratego_engine import register_game, get_available_moves, get_game_state, perform_move, beautifull_board_display
from random import *

class MonteCarloAI(StrategoAI):
    def ask_next_move(self) -> Tuple[Tuple[int, str], Tuple[int, str]]:
        game_id = register_game("Tigran", "test2")
        moves = get_available_moves(game_id)

        movesFormated = parse_moves(moves)

        # for move in movesFormated:
            # move.show()

        move = movesFormated[randint(0, len(moves) - 1)]
        if move.color == 'Red':
            move.show()
            print(perform_move(game_id, (move.from_x, move.from_y), (move.to_x, move.to_y)))
            # print(beautifull_board_display(game_id))

        # state = get_game_state(game_id)
        # print(state)

        return moves[randint(0, len(moves) - 1)]

monte_carlo = MonteCarloAI()
move = monte_carlo.ask_next_move()


