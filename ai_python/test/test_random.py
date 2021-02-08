import ai_python.src.stratego_engine as se
from ai_python.src.random import choose_randomly


def test_random_ai():
    board = se.rust_create_empty_stratego_board()
    board.place("Full", (0, "A"), 4, "Red")
    board.place("Full", (0, "B"), -2, "Red")

    move = choose_randomly(board, 'Red') 

    assert move[0][0] == 0
    assert move[0][1] == 'A'
    assert move[1][0] == 1
    assert move[1][1] == 'A'
