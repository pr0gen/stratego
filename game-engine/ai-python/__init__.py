import abc 
from typing import Tuple

from stratego_engine import hello_world

def hello_world_2(): 
    return "hello world 2"

class StrategoAI(abc.ABC):
    @abc.abstractmethod
    def ask_next_move(self) -> Tuple[Tuple[int, str], Tuple[int, str]]:
        pass

class TestStrategoAI(StrategoAI):
    def ask_next_move(self) -> Tuple[Tuple[int, str], Tuple[int, str]]:
        return ((3, "A"), (4, "A"))


#exposed functions to Rust see README
def ask_next_move_test() -> Tuple[Tuple[int, str], Tuple[int, str]]:
    p = TestStrategoAI()
    return p.ask_next_move()

