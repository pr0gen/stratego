use crate::board::case;
use crate::board::Board;
use crate::error::StrategoError;
use crate::player::*;
use crate::py_bindings::RustStrategoBoard;
use crate::utils;

const AI_STRATEGO_INIT_FILE: &str = "rust_bind";
const AI_STRATEGO_BASE_ASK_NEXT_MOVE_FUNCTION: &str = "ask_next_move";

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
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
    fn ask_next_move(&self, board: StrategoBoard) -> (Coordinate, Coordinate) {
        ask_ai_next_move(self.name.as_str(), board).unwrap_or_else(|e| panic!("{}", e.message()))
    }

    fn get_color(&self) -> &Color {
        &self.color
    }

    fn get_name(&self) -> &str {
        &self.name
    }
}

fn ask_ai_next_move(
    name: &str,
    board: StrategoBoard,
) -> Result<(Coordinate, Coordinate), StrategoError> {
    let gil_holder = utils::get_gild_holder();
    match gil_holder {
        Ok(gil_holder) => {
            let py = gil_holder.get().python();

            let import = py.import(AI_STRATEGO_INIT_FILE);
            if let Err(e) = import {
                return Err(StrategoError::AILoadingError(format!(
                    "Failed to import stratego ai init file, {}",
                    e,
                )));
            }

            let function = import
                .unwrap()
                .get(format!("{}_{}", AI_STRATEGO_BASE_ASK_NEXT_MOVE_FUNCTION, name).as_str());
            if let Err(e) = function {
                return Err(StrategoError::AILoadingError(format!(
                    "Failed to load AI function, {}",
                    e
                )));
            }

            let board = RustStrategoBoard::new(board.state().clone());
            let args = (board,);
            let call = function.unwrap().call1(args);
            if let Err(e) = call {
                return Err(StrategoError::AILoadingError(format!(
                    "Failed to call AI function, {}",
                    e
                )));
            }

            let next_move = call.unwrap().extract();
            if let Ok(next_move) = next_move {
                println!("AI {} is playing", name);
                Ok(case::into(next_move))
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
        player.ask_next_move(StrategoBoard::new(Vec::new()))
    );
}
