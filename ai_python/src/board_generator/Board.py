import random

from PIece import Piece
from utils import piece_to_text, piece_to_text_for_file


class Board:
    board = []

    def __init__(self, board=[]):

        if not board:
            self.board = []
            self.initBoard()
            self.full_shuffle()

        else:
            self.board = board

    def addPieces(self, piece, number):
        while number > 0:
            self.board.append(piece)
            number -= 1

    def initBoard(self):
        self.addPieces(Piece.Marshal, 1)
        self.addPieces(Piece.General, 1)
        self.addPieces(Piece.Colonel, 2)
        self.addPieces(Piece.Major, 3)
        self.addPieces(Piece.Captain, 4)
        self.addPieces(Piece.Lieutenant, 4)
        self.addPieces(Piece.Sergeant, 4)
        self.addPieces(Piece.Miner, 5)
        self.addPieces(Piece.Scout, 8)
        self.addPieces(Piece.Spy, 1)
        self.addPieces(Piece.Flag, 1)
        self.addPieces(Piece.Bomb, 6)

    def full_shuffle(self):
        random.shuffle(self.board)

    def show(self):
        
        line = ''
        size = len(self.board) - 1

        while size >= 0:

            if size % 10 == 0:
                line = piece_to_text(self.board[size]) + line
                print(line)
                line = ''
            else:
                line = piece_to_text(self.board[size]) + line

            size -= 1

        print(line)


    def copy(self):
        return Board(self.board.copy())


    def random_move(self):

        index_from = random.randint(0,39)
        index_to = random.randint(0,39)

        temp = self.board[index_from]
        self.board[index_from] = self.board[index_to]
        self.board[index_to] = temp


    def random_moves(self, number_random_move):

        for i in range(number_random_move):
            self.random_move()


    
    def to_string(self):

        text = ''

        for piece in self.board:
            text += piece_to_text_for_file(piece) + '|'

        return text[:-1]