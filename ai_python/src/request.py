from typing import Tuple, List
from pydantic import BaseModel


class CreateGameRequest(BaseModel):
    player_id_1: str
    player_id_2: str
    type: str
    board: List[List[str]]


class MoveRequest(BaseModel):
    uuid: str
    player_id: str
    coordinate_from: Tuple[int, str]
    coordinate_to: Tuple[int, str]


class AIRequest(BaseModel):
    uuid: str
    color: str
    ai_name: str

    
