pub mod board;
pub mod engine;
pub mod engine_utils;
pub mod game_pool;
pub mod error;
pub mod parse;
pub mod player;
pub mod py_bindings;
pub mod utils;

use std::sync::Mutex;
use game_pool::{GamePool, HumanAIGamePool};

#[macro_use] extern crate lazy_static;

lazy_static! {
    pub static ref GAME_POOL: Mutex<HumanAIGamePool> = Mutex::new(GamePool::new());
    pub static ref GAME_POOL_ID: Mutex<i128> = Mutex::new(0);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub fn add_one(x: i32) -> i32 {
    x + 1
}

