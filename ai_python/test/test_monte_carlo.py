from ai_python.src.utils import StrategoAI, Move, MoveBuilder, parse_moves, move_ready
from ai_python.src.memory import Cache, CacheException
from ai_python.src.monte_carlo import MonteCarloAI
from ai_python.src.random import RandomAI
from ai_python.src.monte_carloV2 import MonteCarloV2AI
from ai_python.src.stratego_engine import StrategoBoardWrapper
import pytest


### IMPORTANT 
### Because of MonteCarloV2AI specifications, thoses tests can not be runned on parallel

def _play_one_turn(board, move):
    f, t = move
    board.moving(f, t)

def _play_many_turns(board, turns, ai, ai_2):
    for i in range(0, turns):
        print('=== START ', i, '=== ')
        print('   === NOUS === ')
        _play_one_turn(board, ai.ask_next_move(board))

        print('   === RANDOM === ')
        _play_one_turn(board, ai_2.ask_next_move(board))
        print('=== BOARD ===')
        print(board.display())
        ai.cache.show()

        print('=== END === ')


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
    
    

def test_monte_carlo_v2_opponent_attacks_first_opponent_won():
    cases = [
        ["02B", "-2B", "NOP"],
        ["02B", "NOP", "NOP"],
        ["06R", "-2R", "NOP"],
    ]
    board = StrategoBoardWrapper(cases)
    ai = MonteCarloV2AI('Blue', 1, 3)
    ai_2 = RandomAI('Red')


    print('=== Init === ')
    ai.cache.reset_cache('Blue', 1, 3)
    ai.cache.show()
    print(board.display())
    print('============')


    #Red plays
    _play_one_turn(board, ai_2.ask_next_move(board))
    print(board.display())

    #Blue plays
    _play_one_turn(board, ai.ask_next_move(board))
    print(board.display())

    piece = ai.cache.get_piece((1, 0))
    ai.cache.show()

    assert piece.value == 6
    assert piece.moved 


def test_monte_carlo_v2_opponent_attacks_first_opponent_lost():
    with pytest.raises(CacheException) as e_info:
        cases = [
            ["02B", "-2B", "NOP"],
            ["06B", "NOP", "NOP"],
            ["02R", "-2R", "NOP"],
        ]
        board = StrategoBoardWrapper(cases)
        ai = MonteCarloV2AI('Blue', 1, 3)
        ai_2 = RandomAI('Red')

        print('=== Init ===')
        ai.cache.reset_cache('Blue', 1, 3)
        ai.cache.show()
        print(board.display())
        print('============')

        #Red plays
        _play_one_turn(board, ai_2.ask_next_move(board))
        ai.cache.show()
        print(board.display())

        #Blue plays
        _play_one_turn(board, ai.ask_next_move(board))
        ai.cache.show()
        print(board.display())

        piece = ai.cache.get_piece((2, 0))
        ai.cache.show()


def test_monte_carlo_v2_we_attack_first_we_won():
    with pytest.raises(CacheException) as e_info:
        cases = [
            ["02B", "-2B", "NOP"],
            ["NOP", "NOP", "NOP"],
            ["06R", "-2R", "NOP"],
        ]
        board = StrategoBoardWrapper(cases)
        ai = MonteCarloV2AI('Red', 1, 3)
        ai_2 = RandomAI('Blue')

        print('=== Init ===')
        ai.cache.reset_cache('Red', 1, 3)
        ai.cache.show()
        print(board.display())
        print('============')

        #Red plays
        _play_one_turn(board, ai.ask_next_move(board))

        #Blue plays
        _play_one_turn(board, ai_2.ask_next_move(board))
        ai.cache.show()

        #Red plays
        _play_one_turn(board, ai.ask_next_move(board))

        piece = ai.cache.get_piece((0, 0))
        piece.show()


def test_monte_carlo_v2_we_attack_first_we_lost():
        cases = [
            ["06B", "-2B", "-2R"],
            ["NOP", "-2R", "NOP"],
            ["03R", "-2R", "NOP"],
        ]
        board = StrategoBoardWrapper(cases)
        ai = MonteCarloV2AI('Red', 1, 3)
        ai_2 = RandomAI('Blue')

        print('=== Init ===')
        ai.cache.reset_cache('Red', 1, 3)
        ai.cache.show()
        print(board.display())
        print('============')

        #Red plays
        _play_one_turn(board, ai.ask_next_move(board))
        print(board.display())
        
        #Blue plays
        _play_one_turn(board, ai_2.ask_next_move(board))
        print(board.display())
        ai.cache.show()

        # we trick a bit the board to check the result
        board.place("Full", (2, "C"), 2, "Red") 
        print(board.display())

        #Red plays
        _play_one_turn(board, ai.ask_next_move(board))
        ai.cache.show()
        print(board.display())

        piece = ai.cache.get_piece((1, 0))
        print(board.display())

        assert piece.value == 6
        assert piece.moved == True


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

    print('=== Init ===')
    ai.cache.reset_cache('Red', 4, 10)
    ai.cache.show()
    print(board.display())
    print('============')

    # print(board.display())
    _play_many_turns(board, 200, ai, ai_2)
    ai.cache.show()
    print(board.display())

    assert 1 == 0
