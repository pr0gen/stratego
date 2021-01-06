import abc 
from typing import Tuple

from stratego_engine import hello_world, register_game, get_available_moves, get_game_state

def hello_world_2(): 
    return "hello world 2"

class StrategoAI(abc.ABC):
    @abc.abstractmethod
    def ask_next_move(self) -> Tuple[Tuple[int, str], Tuple[int, str]]:
        pass

class TestStrategoAI(StrategoAI):
    def ask_next_move(self) -> Tuple[Tuple[int, str], Tuple[int, str]]:
        game_id = register_game("Tigran", "test2")
        # moves = get_available_moves(game_id) 
        # print(moves)
        state = get_game_state(game_id)
        case = state[0][0]
        print(case)
        print(case.coordinate)

        return ((3, "A"), (4, "A"))


#exposed functions to Rust see README
def ask_next_move_test() -> Tuple[Tuple[int, str], Tuple[int, str]]:
    p = TestStrategoAI()
    return p.ask_next_move()

def ask_next_move_test2() -> Tuple[Tuple[int, str], Tuple[int, str]]:
    return ((4, "A"), (5, "A"))

