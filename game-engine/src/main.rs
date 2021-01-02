use pyo3::{GILGuard, Python};

use board::classic_board::create_stratego_board;
use board::piece::Color;
use engine::Engine;
use engine::StrategoEngine;
use engine_utils::game_is_over;
use error::StrategoError;
use player::ai_player::AIPlayer;
use player::HumanPlayer;
use py_bindings::load_stratego_ai_module;

pub mod board;
pub mod engine;
pub mod engine_utils;
pub mod error;
pub mod parse;
pub mod player;
pub mod py_bindings;
pub mod game_pool;

fn main() {
    let gil: GILGuard = Python::acquire_gil();

    load_stratego_ai_module(&gil.python()).unwrap_or_else(|_| {
        panic!(StrategoError::AILoadingError(String::from(
            "Failed to load ai module"
        )))
    });

    let mut engine = StrategoEngine::new(
        create_stratego_board(),
        (
            HumanPlayer::new(Color::Red, String::from("Tigran")),
            HumanPlayer::new(Color::Blue, String::from("Cassiopee")),
            //AIPlayer::new(Color::Blue, String::from("test"),) ,
        ),
    );

    println!("{}", engine.display());
    loop {
        let cases = engine.status();
        match game_is_over(cases) {
            Ok(Color::Red) => {
                println!("Red wins");
                break;
            }
            Ok(Color::Blue) => {
                println!("Blue wins");
                break;
            }
            _ => {
                if let Err(e) = engine.moving() {
                    panic!("{:#?}", e)
                }
            }
        }
    }
}
