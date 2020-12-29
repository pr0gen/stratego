import abc 
from typing import Tuple

from stratego_engine import hello_world

def hello_world_2(): 
    return "hello world 2"

class StrategoAI(abc.ABC):
    @abc.abstractmethod
    def ask_next_move(self) -> Tuple[Tuple[int, int], Tuple[int, int]]:
        pass

class TestStrategoAI(StrategoAI):
    def ask_next_move(self) -> Tuple[Tuple[int, int], Tuple[int, int]]:
        return ((0,0), (0,1))


#exposed functions to Rust
#functions as to be nammed : ask_next_move_ + ai name (ask_next_move_test)
def ask_next_move_test() -> Tuple[Tuple[int, int], Tuple[int, int]]:
    p = TestStrategoAI()
    return p.ask_next_move()
