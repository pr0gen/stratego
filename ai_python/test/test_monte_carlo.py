from ai_python.src.utils import StrategoAI, Move, MoveBuilder, parse_moves, move_ready
from ai_python.src.monte_carlo import MonteCarloAI
from ai_python.src.random import RandomAI
from ai_python.src.monte_carloV2 import MonteCarloV2AI
from ai_python.src.stratego_engine import StrategoBoardWrapper

def _play_one_turn(board, move):
    f, t = move
    board.moving(f, t)

def _play_many_turns(board, turns, ai, ai_2):
    for i in range(0, turns):
        # print('=== START ', i, '=== ')
        # print('   === NOUS === ')
        _play_one_turn(board, ai.ask_next_move(board))

        # print('   === RANDOM === ')
        _play_one_turn(board, ai_2.ask_next_move(board))

        # print('=== END === ')


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
    # cases = [
        # ["-2B", "07B", "10B", "03B", "09B", "01B", "03B", "04B", "02B", "04B"],
        # ["06B", "02B", "03B", "-2B", "-1B", "-2B", "02B", "02B", "02B", "03B"],
        # ["06B", "07B", "04B", "08B", "-2B", "06B", "02B", "02B", "04B", "08B"],
        # ["05B", "-2B", "05B", "05B", "-2B", "02B", "05B", "03B", "06B", "07B"],
        # ["NOP", "NOP", "XXX", "XXX", "NOP", "NOP", "XXX", "XXX", "NOP", "NOP"],
        # ["NOP", "NOP", "XXX", "XXX", "NOP", "NOP", "XXX", "XXX", "NOP", "NOP"],
        # ["06R", "02R", "03R", "-2R", "-1R", "-2R", "02R", "02R", "02R", "03R"],
        # ["06R", "07R", "04R", "08R", "-2R", "06R", "02R", "02R", "04R", "08R"],
        # ["05R", "-2R", "05R", "05R", "-2R", "02R", "05R", "03R", "06R", "07R"],
        # ["-2R", "07R", "10R", "03R", "09R", "01R", "03R", "04R", "02R", "04R"]
    # ]

    cases = [
        ["02B", "07B", "10B", "03B", "09B", "01B", "03B", "04B", "02B", "04B"],
        ["06B", "02B", "03B", "02B", "-1B", "02B", "02B", "02B", "02B", "03B"],
        ["06B", "07B", "04B", "08B", "02B", "06B", "02B", "02B", "04B", "08B"],
        ["05B", "02B", "05B", "05B", "02B", "02B", "05B", "03B", "06B", "07B"],
        ["NOP", "NOP", "XXX", "XXX", "NOP", "NOP", "XXX", "XXX", "NOP", "NOP"],
        ["NOP", "NOP", "XXX", "XXX", "NOP", "NOP", "XXX", "XXX", "NOP", "NOP"],
        ["06R", "02R", "03R", "02R", "-1R", "02R", "02R", "02R", "02R", "03R"],
        ["06R", "07R", "04R", "08R", "02R", "06R", "02R", "02R", "04R", "08R"],
        ["05R", "02R", "05R", "05R", "02R", "02R", "05R", "03R", "06R", "07R"],
        ["02R", "07R", "10R", "03R", "09R", "01R", "03R", "04R", "02R", "04R"]
    ]
    board = StrategoBoardWrapper(cases)
    ai = MonteCarloV2AI('Red', 4, 10)
    ai_2 = RandomAI('Blue')

    # print(board.display())
    _play_many_turns(board, 200, ai, ai_2)
    ai.cache.show()
    print(board.display())

    assert 1 == 0
    
