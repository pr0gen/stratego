from typing import Tuple, List, Callable, Any
import ai_python.src.stratego_engine as se
from ai_python.src.utils import basic_material_values

Move = Tuple[int, str]

def simulate_game(
    board: se.StrategoBoardWrapper,
    ai_red: Callable[[se.StrategoBoardWrapper, str], Tuple[Move, Move]],
    ai_blue: Callable[[se.StrategoBoardWrapper, str], Tuple[Move, Move]],
    evaluation_function: Callable[[Any], Any],
    iteration_max: int,
    stopping_critera: Any,
    color: str
    ) -> Any:
    """
    Simulate a game on a given board with a lambda function to choose which move to pick up

    - board: se.StrategoBoardWrapper - Stratego Board to play on

    - ai_red: Callable[[se.StrategoBoardWrapper, str], Tuple[Move, Move]] - Function applied to 
    choose what move to do for reds

    - ai_blue: Callable[[se.StrategoBoardWrapper, str], Tuple[Move, Move]] - Fuction applied to 
    choose what move to do for blues
    
    - evaluation_function: Callable[[], Any] - How to evaluate game state

    - stopping_critera: Any - Which value should stops your evaluation_function
    
    - color: str - Color of the player you are playing
    """

    while iteration_max >= 0:
        red_move = ai_red(board, 'Red') 
        board.moving((red_move[0][0], red_move[0][1]), (red_move[1][0], red_move[1][1]))
        blue_move = ai_blue(board, 'Blue') 
        board.moving((blue_move[0][0], blue_move[0][1]), (blue_move[1][0], blue_move[1][1]))

        red, blue = board.material_evaluation(basic_material_values())
        if color == "Red":
            delta = red[1] - blue[1]
            if delta in stopping_critera: 
                return red[1]
        else:
            delta = blue[1] - red[1]
            if delta in stopping_critera: 
                return blue[1]

        iteration_max = iteration_max - 1
                
    return False

