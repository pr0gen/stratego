from ai_python.src.ai_utils import simulate_game
from ai_python.src.utils import  basic_material_values
from ai_python.src.random import RandomAI, choose_randomly
import ai_python.src.stratego_engine as se

def test_simulate_a_game_with_random():
    board = se.rust_create_stratego_board()
    ai_1 = RandomAI('Red')
    ai_2 = RandomAI('Blue')

    res = simulate_game(
            board,
            choose_randomly,
            choose_randomly,
            board.basic_evaluation(),
            100,
            ('Red'),
            'Red'
    )

    assert res != None


def test_simulate_a_game_with_material_function():
    board = se.rust_create_stratego_board()
    ai_1 = RandomAI('Red')
    ai_2 = RandomAI('Blue')

    res = simulate_game(
            board,
            choose_randomly,
            choose_randomly,
            board.material_evaluation(basic_material_values()),
            100,
            ('Red'),
            'Red',
    )

    assert res != None
