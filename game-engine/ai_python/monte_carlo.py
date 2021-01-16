from typing import Tuple
from ai import StrategoAI, Move, MoveBuilder, parse_moves

from stratego_engine import PyStrategoBoard, Case, py_create_full_case, py_create_empty_case, py_create_piece, get_available_moves
from random import *

class MonteCarloAI(StrategoAI):
    def ask_next_move(self) -> Tuple[Tuple[int, str], Tuple[int, str]]:
        case = py_create_full_case((0, "A"), py_create_piece(1, "Blue"))
        case2 = py_create_full_case((0, "B"), py_create_piece(1, "Red"))
        case3 = py_create_empty_case((1, "A"))
        print(case)
        cases = [[case, case2], [case3, case3]]
        board = PyStrategoBoard(cases)
        print(board.display_by_color("Blue"))
        # board.moving(case, (1, "A"))
        # print(board.display_by_color("Blue"))
        moves = get_available_moves(board)

        movesFormated = parse_moves(moves)
        print(movesFormated[0].show())
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

monte_carlo = MonteCarloAI()
move = monte_carlo.ask_next_move()


