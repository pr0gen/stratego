from typing import Tuple, List
import enum 

import ai_python.src.stratego_engine as se
from ai_python.src.utils import generate_uuid


class Color(enum.Enum):
    Blue = 1
    Red = 2


class Player:
    name = str
    color = Color

    def __init__(self, name: str, color: Color):
        self.name = name
        self.color = color


class Engine:
    players: Tuple[type(Player), type(Player)]
    board: se.StrategoBoardWrapper
    turn: Color

    def __init__(self, players: Tuple[Player, Player], board: se.StrategoBoardWrapper):
        self.players = players
        self.board = board
        self.color = Color.Red


class Game:
    engine: Engine
    uuid: int


    @staticmethod
    def new(engine: Engine):
        return Game(engine, generate_uuid(20))


    def __init__(self, engine: Engine, uuid: int):
        self.engine = engine
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










