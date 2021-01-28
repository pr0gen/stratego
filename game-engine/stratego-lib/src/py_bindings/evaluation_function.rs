use crate::board::piece::Color;
use crate::board::Board;
use crate::engine_utils;

pub fn basic_evaluation(board: &impl Board) -> Option<Color> {
    engine_utils::game_is_over(board.state())
}
