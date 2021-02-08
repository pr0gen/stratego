import logging
from typing import Optional
from fastapi import FastAPI
from typing import Tuple, List
from pydantic import BaseModel
from ai_python.src.utils import StrategoAI, Move, MoveBuilder, parse_moves, generate_uuid, parse_board
from ai_python.src.engine import GamePool, Game, Color, play_with_ai
import ai_python.src.stratego_engine as se
from ai_python.src.stratego_engine import StrategoBoardWrapper
from ai_python.src.request import CreateGameRequest, MoveRequest, AIRequest
from ai_python.src.response import MoveResponse, StrategoResponse, AINameResponse, AIName
from ai_python.src.random import RandomAI
from ai_python.src.monte_carlo import MonteCarloAI

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
    board = parse_board(data.board)
    game = Game.new(board)
    uuid = game_pool.register(game)
    logging.info("Game has been created with uuid:", uuid)
    return StrategoResponse(200, False, "", uuid)


@app.get("/game/{uuid}/{color}")
def read_game(uuid: str, color: str):
    try:
        game = game_pool.find_game(uuid)
        board = game.board
        return BoardResponse(200, False, "", game.uuid, board.display_by_color(color))
    except:
        logging.error("Failed to find game for uuid", uuid)
        return StrategoResponse(200, True, "Game Not Found", uuid)


@app.get("/watcher/{uuid}")
def game_watcher(uuid: str):
    try:
        game = game_pool.find_game(uuid)
        board = game.board
        return BoardResponse(200, False, "", game.uuid, board.display())
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
        board = game.board
        moves = board.get_available_moves_by_color(player_color)
        return MoveResponse(200, False, "", uuid, parse_moves(moves))
    except:
        logging.error("Failed to get available moves for", uuid)
        return StrategoResponse(200, True, "Move available error", uuid)


@app.post("/moves")
def move_piece(data : MoveRequest):
    try:
      game = game_pool.find_game(data.uuid)
      board = game.board
      moved = board.moving(data.coordinate_from, data.coordinate_to)
      return MoveResponse(200, False, "", data.uuid, moved)
    except:
        logging.error("Failed to find game for uuid", data.uuid)
        return StrategoResponse(200, True, "Game Not Found", data.uuid)


@app.post("/ai")
def use_ai(data: AIRequest):
    try:
        game = game_pool.find_game(data.uuid)
        _from, to = play_with_ai(data, game)  
        moved = game.board.moving(_from, to)
        return MoveResponse(200, False, "", data.uuid, moved)
    except:
        logging.error("Failed to find game for uuid", data.uuid)
        return StrategoResponse(200, True, "Game Not Found", data.uuid)


@app.get("/ai")
def ai_name():
    ai_names = [
                AIName("Random", RandomAI.name),
                AIName("Monte Carlo", MonteCarloAI.name)
        ]
    return AINameResponse(200, False, "", ai_names)

