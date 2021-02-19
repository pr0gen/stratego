use crate::board::board_utils;
use crate::board::case::Coordinate;
use crate::board::classic_board::StrategoBoard;
use crate::board::piece::Color;
use crate::board::piece::PieceType;
use crate::board::piece::PyPieceType;
use crate::board::Board;
use crate::error::StrategoError;
use crate::py_bindings::board_wrapper::{self, StrategoBoardWrapper};
use crate::py_bindings::evaluation_function::{self, Material};
use rand::prelude::*;
use std::thread::{self, JoinHandle};

pub type Move = (Coordinate, Coordinate);

pub trait EvaluationFunction<Solution, Return> {
    fn evaluate(self) -> Return;
}

pub struct MaterialEvaluationFunction<PyBoard> {
    board: PyBoard,
    material_values: Vec<(PyPieceType, i16)>,
}

pub fn simulate<Solution, Return>(
    first_ai: &'static (dyn Fn(&StrategoBoard, &Color) -> Option<Move> + Sync),
    second_ai: &'static (dyn Fn(&StrategoBoard, &Color) -> Option<Move> + Sync),
    evaluation_function: &dyn EvaluationFunction<Solution, Return>,
    iteration_max: &i32,
    stopping_critera: &Solution,
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

impl EvaluationFunction<Vec<Material>, Material>
    for MaterialEvaluationFunction<StrategoBoardWrapper>
{
    fn evaluate(self) -> Material {
        let material_values: Vec<_> = self
            .material_values
            .iter()
            .map(|(key, value)| (PieceType::from(key), *value))
            .collect();

        evaluation_function::material_evaluation(self.board.get_board(), &material_values)
    }
}

impl MaterialEvaluationFunction<StrategoBoardWrapper> {
    pub fn new(board: StrategoBoardWrapper, material_values: Vec<(PyPieceType, i16)>) -> Self {
        MaterialEvaluationFunction {
            board,
            material_values,
        }
    }
}
