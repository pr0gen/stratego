use crate::board::classic_board::StrategoBoard;
use crate::board::piece::{Color, PieceType, PyPieceType};
use crate::board::Board;
use crate::engine_utils;
use crate::py_bindings::evaluation_function::{self, Material};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

pub enum EvaluationFunction {
    Material(Vec<(PyPieceType, i16)>, Vec<i32>, Color),
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum EvaluationFunctionResponse {
    Material(Material),
}

impl EvaluationFunction {
    pub fn evaluate(&self, board: &StrategoBoard) -> (bool, EvaluationFunctionResponse) {
        match self {
            EvaluationFunction::Material(material_values, stopping_critera, color) => {
                let material_values: Vec<_> = material_values
                    .iter()
                    .map(|(key, value)| (PieceType::from(key), *value))
                    .collect();

                let (first_ai, second_ai) =
                    evaluation_function::material_evaluation(board, &material_values);

                let eval = EvaluationFunctionResponse::Material((first_ai, second_ai));
                if let Some(winner) = engine_utils::game_is_over(board.state()) {
                    (
                        winner == *color
                            && stopping_critera.iter().any(|&value| value == first_ai.1),
                        eval,
                    )
                } else {
                    (false, eval)
                }
            }
        }
    }
}

impl Ord for EvaluationFunctionResponse {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (
                EvaluationFunctionResponse::Material((red, _)),
                EvaluationFunctionResponse::Material((red_other, _)),
            ) => red.1.cmp(&red_other.1),
        }
    }
}

impl PartialOrd for EvaluationFunctionResponse {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (
                EvaluationFunctionResponse::Material((red, _)),
                EvaluationFunctionResponse::Material((red_other, _)),
            ) => Some(red.1.cmp(&red_other.1)),
        }
    }
}
