use crate::board::piece::Color;
use crate::board::Board;
use crate::engine_utils;

/// Return winner of the game, if one there is
pub fn basic_evaluation(board: &impl Board) -> Option<Color> {
    engine_utils::game_is_over(board.state())
}

/// Evaluate material of both Players, (red, blue)
pub fn material_evaluation(_board: &impl Board) -> ((Color, i32), (Color, i32)) {
    todo!()
}
