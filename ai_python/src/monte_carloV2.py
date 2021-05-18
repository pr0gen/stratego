from typing import Tuple, List

from ai_python.src.memory import Cache, PiecesManager
from ai_python.src.utils import *
import ai_python.src.stratego_engine as se
from ai_python.src.stratego_engine import StrategoBoardWrapper
from ai_python.src.random import choose_randomly


class MonteCarloV2AI(StrategoAI):
    name = "monte_carlo_v2"
    color: str
    cache: Cache
    # piecesManager: PiecesManager

    def __init__(self, color: str, width, height):
        self.color = color
        self.cache = Cache.instance(color, width, height)
        self.cache.show()
        self.piecesManager = PiecesManager()

    def __convert_rank(self, rank: str) -> int:
        switcher = {
            'Marshal': 10 ,
            'General' : 9,
            'Colonel' : 8,
            'Major' : 7,
            'Captain' : 6,
            'Lieutenant' : 5,
            'Sergeant' : 4,
            'Miner' : 3,
            'Scout' : 2,
            'Spy' : 1,
            'Flag' : 0,
            'Bomb' : -1,
        }
        value = switcher.get(rank)
        print(rank, value)
        if value is None:
            print('Does not exist')
        return value



    def ask_next_move(self, board: StrategoBoardWrapper) -> Tuple[Tuple[int, str], Tuple[int, str]]:
        moves = board.get_available_moves_by_color(self.color)
        movesFormated = parse_moves(moves)
        
        # update cache
        coup = board.get_last_coup()
        
        if coup is not None: 
            cases, won = coup
            _from, to = cases
            co_from = _from['coordinate']
            co_to = to['coordinate']
            if won: 
                rank = self.__convert_rank(to['content']['rank'])
                self.cache.update_piece(co_from['x'], co_from['y'], co_to['x'], co_to['y'], rank)
                self.cache.show()
                print(board.display())
            else:
                self.cache.update_piece(co_from['x'], co_from['y'], co_to['x'], co_to['y'])
            

        scores = []
        for move in movesFormated:

            copied_board = board.clone_board()
            f, t = move_ready(move)
            copied_board.moving(f, t)

            sim = copied_board.simulate_games_material(
                basic_material_values(),
                self.__get_material_range(),
                30,
                self.color,
                4
            )
            scores.append((move, sim))

        best_scores = []
        for score in scores:
            best_scores.append(get_best_score_by_move(score, self.color))

        best_scores.sort(reverse=True, key=sort_best_scores)

        best_move = best_scores[0][0]
        if best_move == None or best_move == False:
            best_move = choose_randomly(board, self.color)

        m = move_ready(best_move)

        print('Move:', m, '- Score:', best_scores[0][1])
        # self.cache.show()
        return m

    def __get_material_range(self) -> List[int]:
        return list(range(0, 50))
