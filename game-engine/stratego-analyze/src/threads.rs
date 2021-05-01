use crate::writter;
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use stratego_lib::board::piece::Color;
use stratego_lib::board::piece::PieceType;
use stratego_lib::board::Board;
use stratego_lib::engine::{Engine, StrategoEngine};
use stratego_lib::engine_utils;
use stratego_lib::error::StrategoError;
use stratego_lib::player::ai_player::AIPlayer;
use stratego_lib::py_bindings::evaluation_function;

pub fn spawn_thread_for_stratego(
    thread_number: usize,
    file_name: Arc<Mutex<String>>,
    first_ai_name: String,
    second_ai_name: String,
) -> Result<JoinHandle<()>, StrategoError> {
    Ok(thread::Builder::new()
        .name(format!("ANALYZE - {}", thread_number))
        .spawn(move || {

            let board = engine_utils::create_stratego_board_from_file("./boards.txt", String::from("R")).unwrap();
            let mut engine = StrategoEngine::new(
                engine_utils::create_stratego_board_with_same_pieces(),
                (
                    Box::new(AIPlayer::new(Color::Red, first_ai_name)),
                    Box::new(AIPlayer::new(Color::Blue, second_ai_name)),
                ),
            );

            let material_values: Vec<(PieceType, i16)> = vec![
                (PieceType::Bomb, 0),
                (PieceType::Marshal, 10),
                (PieceType::General, 9),
                (PieceType::Colonel, 8),
                (PieceType::Major, 7),
                (PieceType::Captain, 6),
                (PieceType::Lieutenant, 5),
                (PieceType::Sergeant, 4),
                (PieceType::Miner, 3),
                (PieceType::Scout, 2),
                (PieceType::Spy, 1),
                (PieceType::Flag, 1),
            ];

            let mut profondeur = 0;
            loop {
                println!("[Thread-{}]", thread_number);
                let board = engine.status();
                match engine_utils::game_is_over(board.state()) {
                    Some(Color::Red) => {
                println!("==========\n{}", board.display());
                        println!("Red wins");
                        let (red, blue) =
                            evaluation_function::material_evaluation(board, &material_values);
                        let to_write = format!("Red,{},{},{}", red.1, blue.1, profondeur);
                        let file_name = file_name.lock().unwrap();
                        writter::write_into_file(file_name.as_str(), to_write.as_str());
                        break;
                    }
                    Some(Color::Blue) => {
                println!("==========\n{}", board.display());
                        println!("Blue wins");
                        let (red, blue) =
                            evaluation_function::material_evaluation(board, &material_values);
                        let to_write = format!("Blue,{},{},{}", red.1, blue.1, profondeur);
                        let file_name = file_name.lock().unwrap();
                        writter::write_into_file(file_name.as_str(), to_write.as_str());
                        break;
                    }
                    _ => {
                        println!("=== NEW TURN {} ===  \n{}",profondeur,  board.display());
                        if let Err(e) = engine.moving() {
                            panic!("{:#?}", e);
                        } else {
                        println!("=== END TURN {} === ", profondeur);
                            profondeur += 1;
                        }
                    }
                }
            }
        })
        .unwrap())
}
