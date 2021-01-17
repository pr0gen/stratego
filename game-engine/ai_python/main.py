import abc 
from typing import Tuple


from ai import StrategoAI, TestStrategoAI
from monte_carlo import MonteCarloAI

def hello_world_2(): 
    return "hello world 2"


#exposed functions to Rust see README
def ask_next_move_test() -> Tuple[Tuple[int, str], Tuple[int, str]]:
    p = TestStrategoAI()
    return p.ask_next_move()

def ask_next_move_test2() -> Tuple[Tuple[int, str], Tuple[int, str]]:
    return ((4, "A"), (5, "A"))

def ask_next_move_monte_carlo() -> Tuple[Tuple[int, str], Tuple[int, str]]:
    monteCarlo = MonteCarloAI()
    return monteCarlo.ask_next_move() 


