use crate::board::classic_board::StrategoBoard;
use crate::board::piece::{Color, PieceType, PyPieceType};
use crate::board::Board;
use crate::engine_utils;
use crate::py_bindings::evaluation_function::{self, Material};
use serde::{Deserialize, Serialize};

pub enum EvaluationFunction {
    Material(Vec<(PyPieceType, i16)>, Vec<i32>, Color),
}

#[derive(Debug, Serialize, Deserialize)]
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

                let delta = match color {
                    Color::Red => first_ai.1 - second_ai.1,
                    Color::Blue => second_ai.1 - first_ai.1,
                    _ => panic!("You can not evaluate, color is illegal"),
                };

                let eval = EvaluationFunctionResponse::Material((first_ai, second_ai));
                if let Some(winner) = engine_utils::game_is_over(board.state()) {
                    (
                        winner == *color && stopping_critera.iter().any(|&value| value == delta),
                        eval,
                    )
                } else {
                    (false, eval)
                }
            }
        }
    }
}
