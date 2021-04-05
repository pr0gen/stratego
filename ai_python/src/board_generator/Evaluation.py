
from PIece import Piece
from utils import get_position_by_index


class Eval():
    
    def get_max_score(self):
        return 100

    def get_score(self, board):

        score = 0
        index = 0

        for piece in board:

            if piece == Piece.Bomb:
                position = get_position_by_index(index)
                if position.y == 0:
                    score -= 20

            elif  piece == Piece.Flag:
                position = get_position_by_index(index)
                if position.y == 0:
                    score += 100


            index += 1


        return score
            