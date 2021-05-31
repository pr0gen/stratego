use std::sync::Arc;
use std::sync::Mutex;

use stratego_lib::board::piece::Color;
use stratego_lib::board::piece::PieceType;
use stratego_lib::board::Board;
use stratego_lib::engine::{Engine, StrategoEngine};
use stratego_lib::engine_utils;
use stratego_lib::player::ai_player::AIPlayer;
use stratego_lib::player::HumanPlayer;
use stratego_lib::py_bindings::evaluation_function;

fn main() {

    let file_name = String::from("data.csv");
    let file_name = Arc::new(Mutex::new(file_name));
    for game_number in 0..4 {
        let board =
            engine_utils::create_stratego_board_from_file("./boards.txt", String::from("R"))
                .unwrap();
        let mut engine = StrategoEngine::new(
            // engine_utils::create_stratego_board_with_same_pieces(),
            board,
            (
                Box::new(AIPlayer::new(Color::Red, String::from("monte_carlo_v2"))),
                Box::new(AIPlayer::new(Color::Blue, String::from("random"))),
                // Box::new(HumanPlayer::new(Color::Blue, String::from("Tigran"))),
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
            println!("[Game-{}]", game_number);
            let board = engine.status();
            match engine_utils::game_is_over(board.state()) {
                Some(Color::Red) => {
                    println!("==========\n{}", board.display());
                    println!("Red wins");
                    let (red, blue) =
                        evaluation_function::material_evaluation(board, &material_values);
                    let to_write = format!("Red,{},{},{}", red.1, blue.1, profondeur);
                    let file_name = file_name.lock().unwrap();
                    write_into_file(file_name.as_str(), to_write.as_str());
                    break;
                }
                Some(Color::Blue) => {
                    println!("==========\n{}", board.display());
                    println!("Blue wins");
                    let (red, blue) =
                        evaluation_function::material_evaluation(board, &material_values);
                    let to_write = format!("Blue,{},{},{}", red.1, blue.1, profondeur);
                    let file_name = file_name.lock().unwrap();
                    write_into_file(file_name.as_str(), to_write.as_str());
                    break;
                }
                _ => {
                    println!("=== NEW TURN {} ===  \n{}", profondeur, board.display());
                    if let Err(e) = engine.moving() {
                        panic!("{:#?}", e);
                    } else {
                        println!("=== END TURN {} === ", profondeur);
                        profondeur += 1;
                    }
                }
            }
        }
    }
}

use std::fs::OpenOptions;
use std::io::prelude::*;

pub fn write_into_file(file_name: &str, to_write: &str) {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_name)
        .unwrap();

    if let Err(e) = writeln!(file, "{}", to_write) {
        eprintln!("Couldn't write to file: {}", e);
    }
}
