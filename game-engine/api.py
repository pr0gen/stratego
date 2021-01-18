from typing import Optional
from fastapi import FastAPI
from typing import Tuple, List

from ai_python.src.ai import StrategoAI, Move, MoveBuilder, parse_moves
import ai_python.src.stratego_engine as se
from ai_python.src.engine import GamePool, Game, Engine, Player, Color

game_pool = GamePool([])

app = FastAPI()


class StrategoResponse:
    status = int
    error = bool
    message = str
    uuid = str

    def __init__(self, status: int, error: bool, message: str, uuid: str):
        self.status = status
        self.error = error
        self.message = message
        self.uuid = uuid



def parse_board(board: List[List[str]]):
    return se.rust_create_empty_stratego_board()

@app.get("/")
def hello_world():
    return "Hello world"



# @app.post("/create-game")
# def read_create_game(player_id_1: str, player_id_2: str, type: str, board: List[List[str]]):
    # parsed_board = parse_board(board)
    # engine = Engine((Player(player_id_1, Color.Red), Player(player_id_2, Color.Blue)), parsed_board)
    # game = Game(engine, )
    # game_pool.register(game)
    # return {"wip"}


@app.get("/game/{uuid}")
def read_game(uuid: str):
    return {"wip"}


@app.get("/moves/{player_id}/{uuid}")
def read_available_moves(player_id: str, uuid: str):
    return {"wip"}


@app.post("/moves")
def move_piece(uuid: str, player_id: str, coordinate_x: int, coordinate_y: str):
    return {"wip"}

