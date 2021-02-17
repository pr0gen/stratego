use crate::board::board_utils;
use crate::board::case::Coordinate;
use crate::board::classic_board::StrategoBoard;
use crate::board::piece::Color;
use crate::board::Board;
use crate::error::StrategoError;
use crate::py_bindings::board_wrapper::StrategoBoardWrapper;
use rand::prelude::*;
use std::thread::{self, JoinHandle};

pub type Move = (Coordinate, Coordinate);

pub fn simulate<Stop: Send + std::cmp::PartialEq + Sync + Clone + std::fmt::Debug>(
    board: StrategoBoardWrapper,
    first_ai: &'static (dyn Fn(&StrategoBoard, &Color) -> Option<Move> + Sync),
    second_ai: &'static (dyn Fn(&StrategoBoard, &Color) -> Option<Move> + Sync),
    evaluation_function: &'static (dyn Fn(&StrategoBoard) -> Stop + Sync),
    iteration_max: &i32,
    stopping_critera: &'static Stop,
    number_of_threads: i32,
    number_of_parties: i32,
) -> Result<Move, StrategoError> {
    todo!();
}

pub fn choose_randomly(board: &impl Board, color: &Color) -> Option<Move> {
    let moves: Vec<_> = board_utils::get_availables_moves_by_color(board, &color);
    if moves.is_empty() {
        None
    } else {
        let mut rng = rand::thread_rng();
        let index: usize = rng.gen_range(0, moves.len());
        if let Some((from, to, _, _)) = moves.get(index) {
            Some((*from, *to))
        } else {
            None
        }
    }
}
