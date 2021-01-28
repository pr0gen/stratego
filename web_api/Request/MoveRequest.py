from typing import Tuple
from pydantic import BaseModel


class MoveRequest(BaseModel):
    uuid: str
    player_id: str
    coordinate_from: Tuple[int, str]
    coordinate_to: Tuple[int, str]
