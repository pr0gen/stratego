from typing import Tuple
from ai_python.src.ai import StrategoAI, Move, MoveBuilder, parse_moves

import ai_python.src.stratego_engine as se
from random import *

class MonteCarloAI(StrategoAI):
    def ask_next_move(self) -> Tuple[Tuple[int, str], Tuple[int, str]]:
        board = se.rust_create_stratego_board()
        print(board.display_by_color("Blue"))
        moves = se.rust_get_available_moves(board)

        movesFormated = parse_moves(moves)
        for move in movesFormated:
            move.show()
        return ((3, "A"), (4, "A"))
        # movesFormated = parse_moves(moves)

        # # for move in movesFormated:
            # # move.show()

        # move = movesFormated[randint(0, len(moves) - 1)]
        # if move.color == 'Red':
            # move.show()
            # # print(beautifull_board_display(game_id))

        # # state = get_game_state(game_id)
        # # print(state)

        # return moves[randint(0, len(moves) - 1)]

# monte_carlo = MonteCarloAI()
# move = monte_carlo.ask_next_move()


