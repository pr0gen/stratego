from typing import List
from pydantic import BaseModel


class CreateGameRequest(BaseModel):
    player_id_1: str
    player_id_2: str
    type: str
    board: List[List[str]]