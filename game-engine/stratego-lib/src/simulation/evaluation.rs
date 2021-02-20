use std::any::Any;
use crate::board::classic_board::StrategoBoard;
use crate::board::piece::Color;
use crate::board::piece::PieceType;
use crate::board::piece::PyPieceType;
use crate::py_bindings::evaluation_function;

pub enum EvaluationFunction {
    BasicEvaluationFunction(Color),
    MaterialEvaluationFunction(Vec<(PyPieceType, i16)>, Vec<i32>),
}

impl EvaluationFunction {
    pub fn evaluate(&self, board: &StrategoBoard) -> (bool, Box<dyn Any + Send>) {
        match self {
            EvaluationFunction::BasicEvaluationFunction(_) => {
            todo!()
            }
            EvaluationFunction::MaterialEvaluationFunction(material_values, stopping_critera) => {
                let material_values: Vec<_> = material_values
                    .iter()
                    .map(|(key, value)| (PieceType::from(key), *value))
                    .collect();

                let (first_ai, second_ai) =
                    evaluation_function::material_evaluation(board, &material_values);
                eprint!("{:?}", first_ai);
                (
                    stopping_critera.iter().any(|&value| value == first_ai.1),
                    Box::new((first_ai, second_ai)),
                )
            }
        }
    }
}
