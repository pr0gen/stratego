use crate::writter;
use std::thread::{self, JoinHandle};
use stratego_lib::engine_utils;
use stratego_lib::board::piece::Color;
use stratego_lib::board::Board;
use stratego_lib::engine::{StrategoEngine, Engine};
use stratego_lib::error::StrategoError;
use stratego_lib::player::ai_player::AIPlayer;

pub fn spawn_thread_for_stratego(
    thread_number: usize,
    file_name: String,
    first_ai_name: String,
    second_ai_name: String,
) -> Result<JoinHandle<()>, StrategoError> {
    Ok(thread::spawn(move || {
        let mut engine = StrategoEngine::new(
            engine_utils::create_stratego_board(),
            (
                Box::new(AIPlayer::new(Color::Red, first_ai_name)),
                Box::new(AIPlayer::new(Color::Blue, second_ai_name)),
            ),
        );

        println!("{}", engine.display_by_color(&engine.get_turn()));
        loop {
            let board = engine.status();
            match engine_utils::game_is_over(board.state()) {
                Some(Color::Red) => {
                    println!("Red wins");
                    writter::write_into_file(file_name.as_str(), "Red");
                    break;
                }
                Some(Color::Blue) => {
                    println!("Blue wins");
                    writter::write_into_file(file_name.as_str(), "Blue");
                    break;
                }
                _ => match engine.moving() {
                    Ok(_) => println!("[Thread-{}]", thread_number),
                    Err(e) => panic!("{:#?}", e),
                },
            }
        }
    }))
}
