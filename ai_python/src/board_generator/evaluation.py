from ai_python.src.board_generator.piece import Piece
from ai_python.src.board_generator.position import Position
from ai_python.src.board_generator.utils import get_position_by_index, nested_dict, \
    get_all_neighboor_index_from_position


class Eval():
    index_to_position = {}
    position_to_neighboors = None

    def __init__(self):

        self.position_to_neighboors = nested_dict(2, list)

        for i in range(40):
            p = get_position_by_index(i)
            self.index_to_position[i] = p
            self.position_to_neighboors[p.x][p.y] = get_all_neighboor_index_from_position(p)

        # print(self.position_to_neighboor[0][0])


    def can_take_this_board(self, board):

        score = 0
        index = 0

        flag_found = False

        for piece in board:

            if index == 10 and flag_found == False:
                return False

            if piece == Piece.Bomb:
                position = self.index_to_position[index]
                if position.y == 0:
                    score -= 10000

            elif piece == Piece.Flag:
                flag_found = True
                position = self.index_to_position[index]
                if position.y == 0:
                    score += 1000
                else:
                    return False

                neighboors = self.position_to_neighboors[position.x][position.y]
                number_bomb_neigboors = 0

                for neighboor in neighboors:
                    if board[neighboor] == Piece.Bomb:
                        number_bomb_neigboors += 1
                        if neighboor < 10:
                            score += 10150
                        else:
                            score += 150

                if number_bomb_neigboors < 2:
                    return False

            index += 1

        return score >= 1450

