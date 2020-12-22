use crate::board::case::Case;

use crate::board::classic_board::create_stratego_board;
use crate::board::piece::Color;
use crate::engine::{Engine, StrategoEngine};
use crate::engine_utils::{ask_next_move, game_is_over};
use crate::error::StrategoError;
use crate::player::HumanPlayer;

pub mod board;
pub mod engine;
pub mod engine_utils;
pub mod error;
pub mod player;

fn main() {
    let mut engine: Box<dyn Engine> = Box::new(StrategoEngine::new(
        create_stratego_board(),
        (
            Box::new(HumanPlayer::new(Color::Red, String::from("Tigran"))),
            Box::new(HumanPlayer::new(Color::Blue, String::from("Cassiopee"))),
        ),
    ));

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
            _ => moving(cases.clone(), &mut engine),
        }
    }
}

fn moving(cases: Vec<Vec<Case>>, engine: &mut Box<dyn Engine>) {
    let player = engine.get_player_from_color();
    let (c, to) = ask_next_move(player, &cases);
    if c.get_content().get_color() != player.get_color() {
        println!("You should move a piece of your color !");
        moving(cases, engine);
    } else {
        match engine.execute_move(c, to) {
            Ok(_) => {
                println!("{}", engine.display())
            }
            Err(StrategoError::MoveError(message, _, _)) => {
                println!("{}", message);
                moving(cases, engine);
            }
            Err(e) => {
                panic!("Something went wrong when moving piece, {}", e.message())
            }
        }
    }
}
