use stratego_lib::board::classic_board::create_stratego_board;
use stratego_lib::board::piece::Color;
use stratego_lib::board::Board;
use stratego_lib::engine::{Engine, StrategoEngine};
use stratego_lib::engine_utils::game_is_over;
use stratego_lib::player::ai_player::AIPlayer;
//use stratego_lib::player::HumanPlayer;

fn main() {
    let mut engine = StrategoEngine::new(
        create_stratego_board(),
        (
            Box::new(AIPlayer::new(Color::Red, String::from("random"))),
            Box::new(AIPlayer::new(Color::Blue, String::from("random"))),
        ),
    );

    println!("{}", engine.display_by_color(&engine.get_turn()));
    loop {
        let board = engine.status();
        match game_is_over(board.state()) {
            Some(Color::Red) => {
                println!("Red wins");
                break;
            }
            Some(Color::Blue) => {
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
