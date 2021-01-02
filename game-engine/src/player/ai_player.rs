use pyo3::prelude::*;

use crate::error::StrategoError;
use crate::player::*;
use crate::py_bindings::{get_to_coord, PyCoords};

const AI_STRATEGO_INIT_FILE: &str = "__init__";
const AI_STRATEGO_BASE_ASK_NEXT_MOVE_FUNCTION: &str = "ask_next_move";

#[derive(Hash, Clone, Eq, PartialEq)]
pub struct AIPlayer {
    color: Color,
    name: String,
}

impl AIPlayer {
    pub fn new(color: Color, name: String, ) -> Self {
        AIPlayer { color, name, }
    }
}

impl<'p> Player for AIPlayer {
    fn ask_next_move(&self) -> (Coordinate, Coordinate) {
        ask_ai_next_move(Python::acquire_gil().python(), self.name.as_str()).unwrap()
    }

    fn get_color(&self) -> &Color {
        &self.color
    }

    fn get_name(&self) -> &str {
        &self.name
    }
}

fn ask_ai_next_move(py: Python, name: &str) -> Result<(Coordinate, Coordinate), StrategoError> {
    println!("AI {} is playing", name);

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

    let next_move: PyCoords = next_move.extract().unwrap();
    Ok(get_to_coord(next_move))
}

#[test]
#[ignore]
fn should_ask_next_move_to_test_ai() {
    let player = AIPlayer::new(Color::Blue, String::from("test"), );

    assert_eq!(
        (Coordinate::new(3, 0), Coordinate::new(4, 0)),
        player.ask_next_move()
    );
}
