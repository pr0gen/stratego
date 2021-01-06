use board::piece::Color;
use engine::{Engine, StrategoEngine};
use engine_utils::game_is_over;
use game_pool::{Game, GamePool, HumanAIGamePool};
use player::ai_player::AIPlayer;
use player::HumanPlayer;
use board::classic_board::create_stratego_board;

use std::sync::Mutex;

#[macro_use] extern crate lazy_static;

pub mod board;
pub mod engine;
pub mod engine_utils;
pub mod error;
pub mod game_pool;
pub mod parse;
pub mod player;
pub mod py_bindings;
pub mod utils;

lazy_static! {
    pub(crate) static ref GAME_POOL: Mutex<HumanAIGamePool> = Mutex::new(GamePool::new());
    pub(crate) static ref GAME_POOL_ID: Mutex<i128> = Mutex::new(0);
}

fn main() {
    let engine = StrategoEngine::new(
        create_stratego_board(),
        (
            HumanPlayer::new(Color::Red, String::from("Tigran")),
            //HumanPlayer::new(Color::Blue, String::from("Cassiopee")),
            AIPlayer::new(Color::Blue, String::from("test")),
        ),
    );

    let game = Game::new(*GAME_POOL_ID.lock().unwrap(), engine);
    *GAME_POOL_ID.lock().unwrap() += 1;

    game_pool::register_to_pool(game).unwrap();
    let pool = GAME_POOL.lock().unwrap();
    let game = pool.find_game_by_id(0).unwrap();

    let mut engine = game.get_engine().clone();
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

