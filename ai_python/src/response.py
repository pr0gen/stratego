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
