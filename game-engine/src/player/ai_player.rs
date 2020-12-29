use pyo3::prelude::*;

use crate::error::StrategoError;
use crate::player::*;
use crate::py_bindings::load_stratego_ai_module;

const AI_STRATEGO_INIT_FILE: &'static str = "__init__";
const AI_STRATEGO_BASE_ASK_NEXT_MOVE_FUNCTION: &'static str = "ask_next_move";

pub struct AIPlayer<'p> {
    color: Color,
    name: String,
    py: Python<'p>,
}

impl<'p> AIPlayer<'p> {
    pub fn new(color: Color, name: String, gil: &'p GILGuard) -> Self {
        AIPlayer {
            color,
            name,
            py: gil.python(),
        }
    }
}

impl<'p> Player for AIPlayer<'p> {
    fn ask_next_move(&self) -> (Coordinate, Coordinate) {
        ask_ai_next_move(self.py, self.name.as_str()).unwrap()
    }

    fn get_color(&self) -> &Color {
        &self.color
    }

    fn get_name(&self) -> &str {
        &self.name
    }
}

fn ask_ai_next_move(py: Python, name: &str) -> Result<(Coordinate, Coordinate), StrategoError> {
    load_stratego_ai_module(&py).unwrap();

    let next_move = py
        .import(AI_STRATEGO_INIT_FILE)
        .unwrap_or_else(|_| {
            panic!(StrategoError::AILoadingError(String::from(
                "Failed to import stratego ai init file"
            )))
        })
        .get(format!("{}_{}", AI_STRATEGO_BASE_ASK_NEXT_MOVE_FUNCTION, name).as_str())
        .unwrap_or_else(|_| {
            panic!(StrategoError::AILoadingError(String::from(
                "Failed to load AI function"
            )))
        })
        .call0()
        .unwrap_or_else(|_| {
            panic!(StrategoError::AILoadingError(String::from(
                "Failed to call AI function"
            )))
        });

    let next_move: Coords = next_move.extract().unwrap();
    Ok(next_move.get_to_coord())
}

#[pyclass]
#[derive(Clone, Copy)]
pub struct Coords {
    from: Coordinate,
    to: Coordinate,
}

impl Coords {
    fn get_to_coord(&self) -> (Coordinate, Coordinate) {
        (self.from, self.to)
    }
}

