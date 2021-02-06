from typing import List, Tuple


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


class AIName():
    button: str
    ai: str

    def __init__(self, button, ai):
        self.button = button
        self.ai = ai


class AINameResponse():
    status = int
    error = bool
    message = str
    names = List[AIName]

    def __init__(self, status: int, error: bool, message: str, names: List[AIName]):
        self.status = status
        self.error = error
        self.message = message
        self.names = names


