from ai_python.src.board_generator.piece import Piece
from ai_python.src.board_generator.position import Position
from ai_python.src.board_generator.utils import get_position_by_index, nested_dict, \
    get_all_neighboor_index_from_position, get_index_by_position, piece_to_value


class EvalV2():
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

        need_show = True

        flag_found = False
        totalSum = 0

        for x in range(5):
            for y in range(2):
                sum = 0

                if need_show:
                    Position((x * 2), y + 1).show()
                    Position((x * 2) + 1, y + 1).show()
                    Position((x * 2), y).show()
                    Position((x * 2) + 1, y).show()
                    print('===========')

                top_left_index = get_index_by_position(Position((x * 2), y + 1))
                top_right_index = get_index_by_position(Position((x * 2) + 1, y + 1))
                bottom_left_index = get_index_by_position(Position((x * 2), y))
                bottom_right_index = get_index_by_position(Position((x * 2) + 1, y))

                sum += piece_to_value(board[top_left_index])
                sum += piece_to_value(board[top_right_index])
                sum += piece_to_value(board[bottom_left_index])
                sum += piece_to_value(board[bottom_right_index])

                totalSum += abs(sum - 15)

        print('#####################################')
        print('#####################################')

        for x in range(4):
            for y in range(2):
                sum = 0

                if False:
                    Position(((x + 1) * 2), y + 1).show()
                    Position(((x + 1) * 2) + 1, y + 1).show()
                    Position(((x + 1) * 2), y).show()
                    Position(((x + 1) * 2) + 1, y).show()
                    print('===========')

                top_left_index = get_index_by_position(Position(((x + 1) * 2), y + 1))
                top_right_index = get_index_by_position(Position(((x + 1) * 2) + 1, y + 1))
                bottom_left_index = get_index_by_position(Position(((x + 1) * 2), y))
                bottom_right_index = get_index_by_position(Position(((x + 1) * 2) + 1, y))

                sum += piece_to_value(board[top_left_index])
                sum += piece_to_value(board[top_right_index])
                sum += piece_to_value(board[bottom_left_index])
                sum += piece_to_value(board[bottom_right_index])

                totalSum += abs(sum - 15)



        print('#####################################')
        print('#####################################')

        for x in range(5):
            for y in range(1):
                sum = 0

                if False:
                    Position((x * 2), (y + 1) + 1).show()
                    Position((x * 2) + 1, (y + 1) + 1).show()
                    Position((x * 2), (y + 1)).show()
                    Position((x * 2) + 1, (y + 1)).show()
                    print('===========')

                top_left_index = get_index_by_position(Position((x * 2), (y + 1) + 1))
                top_right_index = get_index_by_position(Position((x * 2) + 1, (y + 1) + 1))
                bottom_left_index = get_index_by_position(Position((x * 2), (y + 1)))
                bottom_right_index = get_index_by_position(Position((x * 2) + 1, (y + 1)))

                sum += piece_to_value(board[top_left_index])
                sum += piece_to_value(board[top_right_index])
                sum += piece_to_value(board[bottom_left_index])
                sum += piece_to_value(board[bottom_right_index])

                totalSum += abs(sum - 15)

        return totalSum

    # print("=============", )
    #    print("next ", )

    #     print("bottom_left : ", )
    #      Position((x * 2), y*2).show()
    #       print("bottom_right : ", )
    #        Position((x * 2) + 1, y*2).show()

    #         print("top_left : ", )
    #          Position((x * 2), (y*2) + 1).show()

    #           print("top_right : ", )
    #            Position((x * 2)+1, (y*2) + 1).show()
