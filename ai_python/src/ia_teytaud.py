from typing import Tuple, List
from ai_python.src.memory import Cache, PiecesManager
from ai_python.src.utils import *
import ai_python.src.stratego_engine as se
from ai_python.src.stratego_engine import StrategoBoardWrapper
from ai_python.src.random import choose_randomly
from operator import itemgetter


class ia_teytaud(StrategoAI):
    name = "ia_teytaud"
    color: str
    cache: Cache
    # piecesManager: PiecesManager

    def __init__(self, color: str, width, height):
        self.color = color
        self.cache = Cache.instance(color, width, height)
        self.piecesManager = PiecesManager()

    def ask_next_move(self, board: StrategoBoardWrapper) -> Tuple[Tuple[int, str], Tuple[int, str]]:
        moves = board.get_available_moves_by_color(self.color)
        movesFormated = parse_moves(moves)

        last_move = board.get_last_coup()

        #if coup is not None:
        #    cases, won = coup
        #    _from, to = cases
        #    co_from = _from['coordinate']
        #    co_to = to['coordinate']
        #    if won:
        #        rank = self.__convert_rank(to['content']['rank'])
        #        self.cache.update_piece(co_from['x'], co_from['y'], co_to['x'], co_to['y'], rank)
        #        self.cache.show()
        #        print(board.display())
        #    else:
        #        self.cache.update_piece(co_from['x'], co_from['y'], co_to['x'], co_to['y'])


        ennemy_pieces = self.piecesManager.get_pieces_left()

        nb_pieces_left = 0
        for value in ennemy_pieces.values():
            nb_pieces_left += value

        probValueList = []

        for key in ennemy_pieces:
            probValueList.append([ennemy_pieces[key],(ennemy_pieces[key]/nb_pieces_left)])

        max_prob = max(probValueList, key = lambda i : i[1])[0]

        max_pos = [x for x, y in enumerate(probValueList) if y[1] == max_prob]

        #valeur du pion ennemi le plus probable de rencontrer :     probValueList[max_pos][0]
        #Bouger un pion plus fort ? ou alors bouger sans prendre de pions ?

        for move in movesFormated:
            print(move)
            #TODO prendre une piece qui peut battre probValueList[max_pos][0] et la jouer

        #TODO à changer
        best_move = choose_randomly(board, self.color)

        if last_move is None :
            best_move = choose_randomly(board, self.color)

        best_move_ready = move_ready(best_move)

        #from_case = board.at(best_move_ready[0])
        #to_case = board.at(best_move_ready[1])
        #from_rank = self.__convert_rank(from_case['content']['rank'])
        #to_rank = self.__convert_rank(to_case['content']['rank'])
        #co_to = to_case['coordinate']
        #co_from = from_case['coordinate']
        #if to_rank is not None:
        #    print(self.cache.show())
        #    if from_rank < to_rank:  # we attempt to attack, but we loosed
        #        self.cache.update_piece(co_to['x'], co_to['y'], co_to['x'], co_to['y'],
        #                                self.__convert_rank(to_rank)
        #                                )
        #    else:
        #        self.cache.delete_piece(co_to['x'], co_to['y'])
        #    print(self.cache.show())

        print('Move:', best_move_ready)
        return best_move_ready

