from ai_python.src.utils import StrategoAI, Move, MoveBuilder, parse_moves, move_ready
from ai_python.src.monte_carlo import MonteCarloAI
import ai_python.src.stratego_engine as se

def test_monte_carlo(): 
    ai = MonteCarloAI('Red') 
    move = ai.ask_next_move(se.rust_create_stratego_board())
    

