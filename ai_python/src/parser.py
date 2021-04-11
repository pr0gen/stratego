from typing import List, Dict
from ai_python.src.stratego_engine import StrategoBoardWrapper
from ai_python.src.board_generator.board import Board
import copy

def pons_dict() -> Dict:
    return {
        'bomb': '-2', 
        'Mars': '10',
        'Gene': '09',
        'colo': '08',
        'majo': '07', 
        'capt': '06', 
        'lieu': '05',
        'serg': '04', 
        'mine': '03',
        'scou': '02', 
        'spy': '01', 
        'flag': '-1',
    }

EMPTY_CASE = 'NOP'
UNREACHABLE_CASE = 'XXX'


def random_color() -> str:
    board = Board()
    board.initBoard()
    board.full_shuffle()
    return board.to_string()


def read_file(file_name) -> str:
    f = open(file_name, "r")
    return f.read()


def middle_row() -> List[str]:
    return [
        EMPTY_CASE, EMPTY_CASE, 
        UNREACHABLE_CASE, UNREACHABLE_CASE,
        EMPTY_CASE, EMPTY_CASE,
        UNREACHABLE_CASE, UNREACHABLE_CASE,
        EMPTY_CASE, EMPTY_CASE
    ]

def parse_board_to_list(board: str, color: str) -> List[List[str]]:
    board_to_list: List[List[str]] = []
    cases: List[str] = board.split('|')
    line: List[str] = []
    dict = pons_dict()
    for i, case in enumerate(cases):
        line.append(dict.get(case) + color)
        if i % 10 == 9:
            board_to_list.append(copy.deepcopy(line))
            line = []

    return board_to_list


def parse_all_board(blue: str, red: str) -> StrategoBoardWrapper:
    first_player = parse_board_to_list(blue, 'B')
    second_player = parse_board_to_list(red, 'R')
     
    cases: List[List[str]] = []
    
    [cases.append(row) for row in first_player]
    cases.append(middle_row())
    cases.append(middle_row())
    [cases.append(row) for row in second_player]

    return StrategoBoardWrapper(cases)

    
def parse_board_to_rust(file_name, dictionnary: Dict) -> StrategoBoardWrapper:
    content: str = read_file(file_name)
    return parse_all_board(content, random_color())

