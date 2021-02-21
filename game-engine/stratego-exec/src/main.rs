use stratego_lib::engine_utils;
use stratego_lib::board::piece::Color;
use stratego_lib::board::Board;
use stratego_lib::engine::{Engine, StrategoEngine};
use stratego_lib::player::ai_player::AIPlayer;
use stratego_lib::player::HumanPlayer;

fn main() {
    let mut engine = StrategoEngine::new(
        engine_utils::create_stratego_board_with_same_pieces(),
        (
            Box::new(AIPlayer::new(Color::Red, String::from("monte_carlo"))),
            //Box::new(HumanPlayer::new(Color::Blue, String::from("Tigran"))),
            Box::new(AIPlayer::new(Color::Blue, String::from("random"))),
        ),
    );

    println!("{}", engine.display_by_color(&engine.get_turn()));
    loop {
        let board = engine.status();
        match engine_utils::game_is_over(board.state()) {
            Some(Color::Red) => {
                println!("==========\n{}", board.display());
                println!("Red wins");
                break;
            }
            Some(Color::Blue) => {
                println!("==========\n{}", board.display());
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
