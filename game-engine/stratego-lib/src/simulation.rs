use crate::board::board_utils;
use crate::board::case::Coordinate;
use crate::board::piece::Color;
use crate::board::Board;
use crate::error::StrategoError;
use rand::prelude::*;
use std::any::Any;

pub fn simulate<RedFunction, BlueFunction, EvaluationFunction, Stop>(
    board: &impl Board,
    ai_red: RedFunction,
    ai_blue: BlueFunction,
    evaluation_function: EvaluationFunction,
    iteration_max: i32,
    stopping_critera: Stop,
    color: Color,
) -> Result<(), StrategoError>
where
    RedFunction: Fn() -> (Coordinate, Coordinate),
    BlueFunction: Fn() -> (Coordinate, Coordinate),
    EvaluationFunction: Fn() -> Stop,
{
    todo!();
}

pub fn choose_randomly(board: &impl Board, color: Color) -> (Coordinate, Coordinate) {
    let moves: Vec<_> = board_utils::get_availables_moves_by_color(board, &color);
    let mut rng = rand::thread_rng();
    let index: usize = rng.gen_range(0, moves.len());
    if let Some((from, to, _, _)) = moves.get(index) {
        (*from, *to)
    } else {
        panic!("Something went wrong when choosing ramdomly a move")
    }
}
