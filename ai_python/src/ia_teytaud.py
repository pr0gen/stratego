from typing import Tuple, List
from ai_python.src.memory import Cache, PiecesManager
from ai_python.src.utils import *
import ai_python.src.stratego_engine as se
from ai_python.src.stratego_engine import StrategoBoardWrapper
from ai_python.src.random import choose_randomly
from operator import itemgetter


class ia_teytaud(StrategoAI):
    name = "ia_teytaud"
    color = str

    # cache: Cache
    # piecesManager: PiecesManager

    def __init__(self, color: str):
        self.color = color
        self.cache = Cache(color)
        self.piecesManager = PiecesManager()

    def ask_next_move(self, board: StrategoBoardWrapper) -> Tuple[Tuple[int, str], Tuple[int, str]]:
        moves = board.get_available_moves_by_color(self.color)
        movesFormated = parse_moves(moves)

        last_move = board.get_last_coup()
        if last_move is not None:
            ##Pas d'information sur la piece mangée ?
            first_piece = last_move[0]
            second_piece = last_move[1]


        ennemy_pieces = self.piecesManager.get_pieces_left()

        nb_pieces_left = 0
        for value in ennemy_pieces.values():
            nb_pieces_left += value

        probValueList = []

        for key in ennemy_pieces:
            probValueList.append([ennemy_pieces[key],(ennemy_pieces[key]/nb_pieces_left)])

        max_prob = max(probValueList, key = lambda i : i[1])[0]

        max_pos = [x for x, y in enumerate(probValueList) if y[1] == max_prob]

        ##valeur du pion ennemi le plus probable de rencontrer :     probValueList[max_pos][0]
        ##Bouger un pion plus fort ? ou alors bouger sans prendre de pions ?

        if last_move is None :
            best_move = choose_randomly(board, self.color)

        ##Remettre a jour le pieceManager ?

        m = move_ready(best_move)
        print('Move:', m)
        return m

