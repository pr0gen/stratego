import logging
from typing import Optional
from fastapi import FastAPI
from typing import Tuple, List

from ai_python.src.utils import StrategoAI, Move, MoveBuilder, parse_moves, generate_uuid, parse_board
from ai_python.src.engine import GamePool, Game, Engine, Player, Color
import ai_python.src.stratego_engine as se

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
                

class MoveResponse(StrategoResponse):
    moves = []
    
    def __init__(self, status: int, error: bool, message: str, uuid: str, moves):
        self.status = status
        self.error = error
        self.message = message
        self.uuid = uuid
        self.moves = moves


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
def read_create_game(player_id_1: str, player_id_2: str, type: str, board: List[List[str]]):
    parsed_board = parse_board(board)
    engine = Engine((Player(player_id_1, Color.Red), Player(player_id_2, Color.Blue)), parsed_board)
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
        moves = se.rust_get_available_moves_by_color(engine.board, player_color)
        return MoveResponse(200, False, "", uuid, parse_moves(moves))
    except: 
        logging.error("Failed to get available moves for", uuid)
        return StrategoResponse(200, True, "Move available error", uuid)
    

@app.post("/moves")
def move_piece(uuid: str, player_id: str, coordinate_from: Tuple[int, str], coordinate_to: Tuple[int, str]):
    try:
      game = game_pool.find_game(uuid) 
      engine = game.engine
      board = engine.board
      moved = board.moving(board.at(coordinate_from), coordinate_to)
      return MoveResponse(200, False, "", uuid, moved)
    except:
        logging.error("Failed to find game for uuid", uuid)
        return StrategoResponse(200, True, "Game Not Found", uuid)

