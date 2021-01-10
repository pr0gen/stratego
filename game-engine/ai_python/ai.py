import abc 
from typing import Tuple

class StrategoAI(abc.ABC):
    @abc.abstractmethod
    def ask_next_move(self) -> Tuple[Tuple[int, str], Tuple[int, str]]:
        pass

class TestStrategoAI(StrategoAI):
    def ask_next_move(self) -> Tuple[Tuple[int, str], Tuple[int, str]]:
        return ((3, "A"), (4, "A"))


