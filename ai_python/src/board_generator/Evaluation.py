
from PIece import Piece
from utils import get_position_by_index, get_all_neighboor_index_from_position


class Eval():
    
    def get_max_score(self):
        return 1450

    def get_score(self, board):

        score = 0
        index = 0

        for piece in board:

            if piece == Piece.Bomb:
                position = get_position_by_index(index)
                if position.y == 0:
                    score -= 10000

            elif  piece == Piece.Flag:
                position = get_position_by_index(index)
                if position.y == 0:
                    score += 1000

                neighboors = get_all_neighboor_index_from_position(position)
                for neighboor in neighboors:
                    if board[neighboor] == Piece.Bomb:
                        if neighboor < 10:
                            score += 10150
                        else:
                            score += 150

            index += 1


        return score
            