use crate::board::classic_board::StrategoBoard;
use crate::board::piece::PieceType;
use crate::board::piece::PyPieceType;
use crate::py_bindings::evaluation_function::{self, Material};
use std::cmp::Ordering;
pub enum EvaluationFunction {
    Material(Vec<(PyPieceType, i16)>, Vec<i32>),
}

#[derive(Debug, Eq, PartialEq)]
pub enum EvaluationFunctionResponse {
    Material(Material),
}

impl EvaluationFunction {
    pub fn evaluate(&self, board: &StrategoBoard) -> (bool, EvaluationFunctionResponse) {
        match self {
            EvaluationFunction::Material(material_values, stopping_critera) => {
                let material_values: Vec<_> = material_values
                    .iter()
                    .map(|(key, value)| (PieceType::from(key), *value))
                    .collect();

                let (first_ai, second_ai) =
                    evaluation_function::material_evaluation(board, &material_values);
                (
                    stopping_critera.iter().any(|&value| value == first_ai.1),
                    EvaluationFunctionResponse::Material((first_ai, second_ai)),
                )
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
