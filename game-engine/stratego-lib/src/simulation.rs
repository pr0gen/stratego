use crate::board::board_utils;
use crate::board::case::Coordinate;
use crate::board::piece::Color;
use crate::board::Board;
use crate::board::classic_board::StrategoBoard;
use crate::error::StrategoError;
use rand::prelude::*;

pub fn simulate<Stop>(
    board: &impl Board,
    ai_red: &dyn Fn(&StrategoBoard, &Color) -> (Coordinate, Coordinate),
    ai_blue: &dyn Fn(&StrategoBoard, &Color) -> (Coordinate, Coordinate),
    evaluation_function: &dyn Fn(&StrategoBoard) -> Stop,
    iteration_max: i32,
    stopping_critera: Stop,
    color: Color,
) -> Result<(Coordinate, Coordinate), StrategoError> {
    todo!();
}

pub fn choose_randomly(board: &impl Board, color: &Color) -> (Coordinate, Coordinate) {
    let moves: Vec<_> = board_utils::get_availables_moves_by_color(board, &color);
    let mut rng = rand::thread_rng();
    let index: usize = rng.gen_range(0, moves.len());
    if let Some((from, to, _, _)) = moves.get(index) {
        (*from, *to)
    } else {
        panic!("Something went wrong when choosing ramdomly a move")
    }
}
