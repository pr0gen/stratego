from ai_python.src.utils import StrategoAI, Move, MoveBuilder, parse_moves, move_ready
from ai_python.src.monte_carlo import MonteCarloAI
from ai_python.src.random import RandomAI
from ai_python.src.monte_carloV2 import MonteCarloV2AI
from ai_python.src.stratego_engine import StrategoBoardWrapper

def _play_one_turn(board, move):
    f, t = move
    board.moving(f, t)

# It is merly a test to debug the ai 
def test_monte_carlo(): 
    cases = [
        ["09B", "04B", "-1B", "03B"],
        ["NOP", "NOP", "NOP", "NOP"],
        ["NOP", "XXX", "NOP", "NOP"],
        ["01R", "02R", "-1R", "03R"]
    ]

    board = StrategoBoardWrapper(cases)
    print(board.display())
    ai = MonteCarloAI('Blue', 4, 2) 
    move = ai.ask_next_move(board)
    
def test_monte_carlo_2():
    cases = [
        ["09B", "04B", "-1B", "07B"],
        ["01R", "02R", "-1R", "03R"],
        ["NOP", "NOP", "NOP", "NOP"],
        ["NOP", "XXX", "NOP", "NOP"]
    ]

    board = StrategoBoardWrapper(cases)
    ai = MonteCarloV2AI('Red', 1, 4)
    ai_2 = RandomAI('Blue')

    print(board.display())

    #1st round
    #Us
    _play_one_turn(board, ai.ask_next_move(board))
    #You
    _play_one_turn(board, ai_2.ask_next_move(board))

    print(board.display())

    #2nd round
    #Us
    _play_one_turn(board, ai.ask_next_move(board))
    #You
    _play_one_turn(board, ai_2.ask_next_move(board))
    print(board.display())

    assert 1 == 0

    
