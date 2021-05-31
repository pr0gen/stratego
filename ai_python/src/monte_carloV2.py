from ai_python.src.memory import Cache, PiecesManager, CacheException
from ai_python.src.utils import *
import ai_python.src.stratego_engine as se
from ai_python.src.stratego_engine import StrategoBoardWrapper
from ai_python.src.random import RandomAI


class MonteCarloV2AI(StrategoAI):
    name = "monte_carlo_v2"
    color: str
    cache: Cache

    # piecesManager: PiecesManager

    def __init__(self, color: str, width, height):
        self.color = color
        self.cache = Cache.instance(color, width, height)
        self.piecesManager = PiecesManager()

    def __convert_rank(self, rank: str) -> int:
        switcher = {
            'Marshal': 10,
            'General': 9,
            'Colonel': 8,
            'Major': 7,
            'Captain': 6,
            'Lieutenant': 5,
            'Sergeant': 4,
            'Miner': 3,
            'Scout': 2,
            'Spy': 1,
            'Flag': 0,
            'Bomb': -1,
            'Null': None,
        }
        value = switcher.get(rank)
        if value is None:
            print(rank, ' Does not exist')
        return value

    def ask_next_move(self, board: StrategoBoardWrapper) -> Tuple[Tuple[int, str], Tuple[int, str]]:
        moves = board.get_available_moves_by_color(self.color)
        movesFormated = parse_moves(moves)

        self.__store_opponents_move(board)

        move = self.__generate_move(board)

        print('Move:', move)

        self.__store_our_move(board, move)

        self.cache.show()

        return move

    def __generate_move(self, board: StrategoBoardWrapper) -> Tuple[Tuple[int, str], Tuple[int, str]]:

        # We default on a random move

        ai = RandomAI(self.color)
        move = ai.ask_next_move(board)
        score = self.__move_to_score(board, move)

        for i in range(100):
            temp_move = ai.ask_next_move(board)
            temp_score = self.__move_to_score(board, temp_move)

            if temp_score > score:
                score = temp_score
                move = temp_move

        return move

    def __move_to_score(self, board, move):
        _from, to = move
        from_case = board.at(_from)

        _from = (_from[0], self.letter_to_int(_from[1]))
        to = (to[0], self.letter_to_int(to[1]))

        my_piece_value = self.__convert_rank(from_case['content']['rank'])

        try:
            piece = self.cache.get_piece(to)

            if piece.value is not None:
                if piece.value < my_piece_value:
                    return 10000

                elif piece.value >= my_piece_value:
                    return -10000

            # 3 == miner
            if my_piece_value == 3 and piece.moved:
                return -7500

            if my_piece_value == 3 and piece.moved is False:
                return 7500


        except CacheException:
            return 1000 - (my_piece_value**3)

        return 1000 - (my_piece_value ** 3)

    def __store_opponents_move(self, board: StrategoBoardWrapper):
        coup = board.get_last_coup()

        if coup is None:
            return  # This is the first turn

        (_from, to), won = coup
        co_from = _from['coordinate']
        co_from = (co_from['x'], co_from['y'])
        co_to = to['coordinate']
        co_to = (co_to['x'], co_to['y'])

        content_to = to['content']
        if won == 3:  # Opponent won
            rank = self.__convert_rank(content_to['rank'])
            self.cache.update_piece(co_from, co_to, rank)
        elif won == 2:  # Both loose
            self.cache.delete_piece(co_to)
            return
        elif won == 1:  # Opponent loose
            self.cache.delete_piece(co_from)
        else:  # Opponent moved on empty square
            self.cache.update_piece(co_from, co_to)

    def __check_we_won_on_opponent_move(self, board, content) -> bool:
        return content['color'] == self.color

    def __store_our_move(self,
                         board: StrategoBoardWrapper,
                         move: Tuple[Tuple[str, int], Tuple[str, int]]
                         ):

        from_case = board.at(move[0])
        to_case = board.at(move[1])

        from_rank = self.__convert_rank(from_case['content']['rank'])
        to_rank = self.__convert_rank(to_case['content']['rank'])

        if to_rank is None:  # We moved on an empty case
            return

        co_to = to_case['coordinate']
        co_to = (co_to['x'], co_to['y'])

        if from_rank < to_rank:  # we attempt to attack, and we loose
            self.cache.update_piece(co_to, co_to, self.__convert_rank(to_rank))
        elif from_rank > to_rank:  # we attempt to attack, and we won
            self.cache.delete_piece(co_to)
        elif from_rank == to_rank:  # both loose
            print('both loose')
            self.cache.delete_piece(co_to)

    def __get_material_range(self) -> List[int]:
        return list(range(0, 50))

    def letter_to_int(self, letter):
        return ord(letter) - 65
