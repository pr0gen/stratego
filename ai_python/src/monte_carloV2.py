from typing import Tuple, List

from ai_python.src.memory import Cache, PiecesManager
from ai_python.src.utils import *
import ai_python.src.stratego_engine as se
from ai_python.src.stratego_engine import StrategoBoardWrapper
from ai_python.src.random import choose_randomly
from ai_python.src.random import RandomAI


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
            'Null' : None,
        }
        value = switcher.get(rank)
        if value is None:
            print('Does not exist')
        return value



    def ask_next_move(self, board: StrategoBoardWrapper) -> Tuple[Tuple[int, str], Tuple[int, str]]:
        moves = board.get_available_moves_by_color(self.color)
        movesFormated = parse_moves(moves)

        # update cache
        coup = board.get_last_coup()

        #Store opponent move
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
                self.cache.delete_piece(co_from['x'], co_from['y'])
                self.cache.update_piece(co_from['x'], co_from['y'], co_to['x'], co_to['y'])


        random = RandomAI(self.color)
        m = random.ask_next_move(board)

        #Store our move

        from_case = board.at(m[0])
        to_case = board.at(m[1])

        from_rank = self.__convert_rank(from_case['content']['rank'])
        to_rank = self.__convert_rank(to_case['content']['rank'])

        co_to = to_case['coordinate']
        co_from = from_case['coordinate']
        if to_rank is not None:
            print('haaaaaaaaaaaaaaaaaaaaaaaaaaaa')
            print('haaaaaaaaaaaaaaaaaaaaaaaaaaaa')
            print('haaaaaaaaaaaaaaaaaaaaaaaaaaaa')
            print('haaaaaaaaaaaaaaaaaaaaaaaaaaaa')
            print('from', from_case, 'to', to_case)
            print('from', from_rank, 'to', to_rank)
            print(co_from['x'], co_from['y'], co_to['x'], co_to['y'])
            print(board.display())
            if from_rank < to_rank: # we attempt to attack, but we loosed
                self.cache.show()
                print('i take the piece')
                self.cache.update_piece(co_to['x'], co_to['y'], co_to['x'], co_to['y'], to_rank )
                self.cache.show()

            else:
                print('rip')
                self.cache.delete_piece(co_to['x'], co_to['y'])

        return m

    def __get_material_range(self) -> List[int]:
        return list(range(0, 50))
