from ai_python.src.utils import StrategoAI, Move, MoveBuilder, parse_moves, move_ready
from ai_python.src.monte_carlo import MonteCarloAI
from ai_python.src.stratego_engine import StrategoBoardWrapper

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
    assert 1 == 0
    

