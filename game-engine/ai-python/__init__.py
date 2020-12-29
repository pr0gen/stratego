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

p = TestStrategoAI()
print(p.ask_next_move())
