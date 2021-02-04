from typing import Tuple, List
import enum 
import ai_python.src.stratego_engine as se
from ai_python.src.utils import generate_uuid, StrategoAI, TestStrategoAI
from ai_python.src.monte_carlo import MonteCarloAI
from ai_python.src.random import RandomAI
from ai_python.src.request import AIRequest


class Color(enum.Enum):
    Red = 1 
    Blue = 2


class Game:
    uuid: int
    board: se.StrategoBoardWrapper

    @staticmethod
    def new(board: se.StrategoBoardWrapper):
        return Game(board, generate_uuid(20))

    def __init__(self, board:se.StrategoBoardWrapper, uuid: int):
        self.board = board
        self.uuid = uuid


class GamePool:
    pool = List[Game]

    def __init__(self, init_pool: List[Game]):
        self.pool = init_pool

    def register(self, game: Game):
        self.pool.append(game)
        return game.uuid

    def find_game(self, uuid: int) -> Game:
        for game in self.pool:
            if game.uuid == uuid:
                return game

    def display(self) -> str:
        message = ""
        for game in self.pool:
            message += game.uuid + "\n"
        return message


def play_with_ai(data: AIRequest, game: Game) -> Tuple[Tuple[int, str], Tuple[int, str]]:
    color = data.color
    switcher = {
        RandomAI.name: RandomAI(color),
        MonteCarloAI.name : MonteCarloAI(color) 
    }
    ai = switcher.get(data.ai_name)
    return ai.ask_next_move(game.board)
     





