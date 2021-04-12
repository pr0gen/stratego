# this file is readed by Rust, don't delete
import abc 
from typing import Tuple

from ai_python.src.monte_carloV2 import MonteCarloV2AI
from ai_python.src.utils import StrategoAI, TestStrategoAI
from ai_python.src.monte_carlo import MonteCarloAI
from ai_python.src.random import RandomAI
import ai_python.src.stratego_engine as se


#init Cache

#exposed functions to Rust see README
def ask_next_move_test(board: se.StrategoBoardWrapper) -> Tuple[Tuple[int, str], Tuple[int, str]]:
    p = TestStrategoAI()
    return p.ask_next_move()


def ask_next_move_test2(board: se.StrategoBoardWrapper) -> Tuple[Tuple[int, str], Tuple[int, str]]:
    return ((4, "A"), (5, "A"))


def ask_next_move_monte_carlo(board: se.StrategoBoardWrapper, color: str) -> Tuple[Tuple[int, str], Tuple[int, str]]:
    monteCarlo = MonteCarloAI(color, 20, 20)
    return monteCarlo.ask_next_move(board) 

def ask_next_move_monte_carlo_v2(board: se.StrategoBoardWrapper, color: str) -> Tuple[Tuple[int, str], Tuple[int, str]]:
    monteCarlo = MonteCarloV2AI(color)
    return monteCarlo.ask_next_move(board)


def ask_next_move_random(board: se.StrategoBoardWrapper, color: str) -> Tuple[Tuple[int, str], Tuple[int, str]]:
    random = RandomAI(color)
    return random.ask_next_move(board) 
