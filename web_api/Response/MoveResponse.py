from web_api.Response.StrategoResponse import StrategoResponse


class MoveResponse(StrategoResponse):
    moves = []

    def __init__(self, status: int, error: bool, message: str, uuid: str, moves):
        self.status = status
        self.error = error
        self.message = message
        self.uuid = uuid
        self.moves = moves