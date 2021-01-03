use crate::error::StrategoError;
use crate::player::*;
use crate::py_bindings;
use crate::utils;

const AI_STRATEGO_INIT_FILE: &str = "__init__";
const AI_STRATEGO_BASE_ASK_NEXT_MOVE_FUNCTION: &str = "ask_next_move";

#[derive(Hash, Clone, Eq, PartialEq)]
pub struct AIPlayer {
    color: Color,
    name: String,
}

impl AIPlayer {
    pub fn new(color: Color, name: String) -> Self {
        AIPlayer { color, name }
    }
}

impl<'p> Player for AIPlayer {
    fn ask_next_move(&self) -> (Coordinate, Coordinate) {
        ask_ai_next_move(self.name.as_str()).unwrap_or_else(|e| panic!("{}", e.message()))
    }

    fn get_color(&self) -> &Color {
        &self.color
    }

    fn get_name(&self) -> &str {
        &self.name
    }
}

fn ask_ai_next_move(name: &str) -> Result<(Coordinate, Coordinate), StrategoError> {
    let gil_holder = utils::get_gild_holder();
    match gil_holder {
        Ok(gil_holder) => {
            let py = gil_holder.get().python();

            let import = py.import(AI_STRATEGO_INIT_FILE);
            if import.is_err() {
                return Err(StrategoError::AILoadingError(String::from(
                    "Failed to import stratego ai init file",
                )));
            }

            let function = import
                .unwrap()
                .get(format!("{}_{}", AI_STRATEGO_BASE_ASK_NEXT_MOVE_FUNCTION, name).as_str());
            if function.is_err() {
                return Err(StrategoError::AILoadingError(String::from(
                    "Failed to load AI function",
                )));
            }

            let call = function.unwrap().call0();
            if call.is_err() {
                return Err(StrategoError::AILoadingError(String::from(
                    "Failed to call AI function",
                )));
            }

            let next_move = call.unwrap().extract();
            if let Ok(next_move) = next_move {
                println!("AI {} is playing", name);
                Ok(py_bindings::get_to_coord(next_move))
            } else {
                Err(StrategoError::AILoadingError(String::from(
                    "Failed to extract result",
                )))
            }
        }
        Err(e) => Err(e),
    }
}

#[test]
#[ignore]
fn should_ask_next_move_to_test_ai() {
    let player = AIPlayer::new(Color::Blue, String::from("test"));

    assert_eq!(
        (Coordinate::new(3, 0), Coordinate::new(4, 0)),
        player.ask_next_move()
    );
}
