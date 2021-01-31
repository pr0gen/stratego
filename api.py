import logging
from typing import Optional
from fastapi import FastAPI
from typing import Tuple, List

from pydantic import BaseModel

from ai_python.src.utils import StrategoAI, Move, MoveBuilder, parse_moves, generate_uuid, parse_board
from ai_python.src.engine import GamePool, Game, Engine, Player, Color
import ai_python.src.stratego_engine as se

from web_api.Request.CreateGameRequest import CreateGameRequest
from web_api.Request.MoveRequest import MoveRequest
from web_api.Response.MoveResponse import MoveResponse
from web_api.Response.StrategoResponse import StrategoResponse

game_pool = GamePool([])

app = FastAPI()


class BoardResponse(StrategoResponse):
    cases = List[List[se.Case]]

    def __init__(self, status: int, error: bool, message: str, uuid: str, cases: List[List[se.Case]]):
        self.status = status
        self.error = error
        self.message = message
        self.uuid = uuid
        self.cases = cases


@app.get("/")
def hello_world():
    return "Hello world"


@app.post("/create-game")
def read_create_game( data : CreateGameRequest):
    parsed_board = parse_board(data.board)
    engine = Engine((Player(data.player_id_1, Color.Red), Player(data.player_id_2, Color.Blue)), parsed_board)
    game = Game.new(engine)
    uuid = game_pool.register(game)
    logging.info("Game has been created with uuid:", uuid)
    return StrategoResponse(200, False, "", uuid)


@app.get("/game/{uuid}/{color}")
def read_game(uuid: str, color: str):
    try:
        game = game_pool.find_game(uuid)
        engine = game.engine
        board = engine.board
        # TODO parse board display
        return BoardResponse(200, False, "", game.uuid, board.display_by_color(color))
    except:
        logging.error("Failed to find game for uuid", uuid)
        return StrategoResponse(200, True, "Game Not Found", uuid)


@app.get("/moves/{player_color}/{uuid}")
def read_available_moves(player_color: str, uuid: str):
    try:
        game = game_pool.find_game(uuid)
    except:
        logging.error("Failed to find game for uuid", uuid)
        return StrategoResponse(200, True, "Game Not Found", uuid)

    try:
        engine = game.engine
        moves = engine.board.get_available_moves()
        parsedMoves = parse_moves(moves)
        for m in parsedMoves:
            m.show()
        return MoveResponse(200, False, "", uuid, parsedMoves)
    except:
        logging.error("Failed to get available moves for", uuid)
        return StrategoResponse(200, True, "Move available error", uuid)


@app.post("/moves")
def move_piece(data : MoveRequest):
    try:
      game = game_pool.find_game(data.uuid)
      engine = game.engine
      board = engine.board
      moved = board.moving(data.coordinate_from, data.coordinate_to)
      return MoveResponse(200, False, "", data.uuid, moved)
    except:
        logging.error("Failed to find game for uuid", data.uuid)
        return StrategoResponse(200, True, "Game Not Found", data.uuid)

