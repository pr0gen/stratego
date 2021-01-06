use pyo3::prelude::*;
use pyo3::types::PyList;
use pyo3::wrap_pyfunction;
use std::env::current_dir;

use crate::board::case::Coordinate;
use crate::board::piece::Color;
use crate::engine::{Engine, StrategoEngine};
use crate::engine_utils;
use crate::error::StrategoError;
use crate::game_pool;
use crate::parse;
use crate::player::ai_player::AIPlayer;
use crate::player::HumanPlayer;
use crate::board::classic_board::create_stratego_board;
use crate::game_pool::Game;
use crate::GAME_POOL_ID;

const AI_STRATEGO_PYTHON_MODULE: &str = "ai-python";

pub type PyCoord = (i16, String);
pub type PyCoords = (PyCoord, PyCoord);
pub type PyColor = String;

pub fn get_to_coord(co: PyCoords) -> (Coordinate, Coordinate) {
    let co_0 = co.0;
    let co_1 = co.1;

    (
        Coordinate::new(co_0.0, parse::parse_letter_to_i16(co_0.1.as_str())),
        Coordinate::new(co_1.0, parse::parse_letter_to_i16(co_1.1.as_str())),
    )
}

pub fn into_py_coord(coordinate: Coordinate) -> PyCoord {
    (coordinate.get_x(), parse::parse_i16_to_str(coordinate.get_y() + 65))
}

pub fn into_py_color(color: Color) -> PyColor {
    match color {
        Color::Blue => String::from("Blue"),
        Color::Red => String::from("Red"),
        Color::None => String::from("None"),
    }
}

pub fn load_stratego_ai_module(py: &Python) -> Result<(), StrategoError> {
    let syspath: &PyList = py
        .import("sys")
        .unwrap_or_else(|_| {
            panic!(StrategoError::AILoadingError(String::from(
                "Failed to find sys python module"
            )))
        })
        .get("path")
        .unwrap_or_else(|_| {
            panic!(StrategoError::AILoadingError(String::from(
                "Failed to find path function in sys python module"
            )))
        })
        .try_into()
        .unwrap_or_else(|_| {
            panic!(StrategoError::AILoadingError(String::from(
                "Failed to get result from path function in sys python module"
            )))
        });

    let cur = current_dir().unwrap_or_else(|_| {
        panic!(StrategoError::AILoadingError(String::from(
            "Failed to find pwd"
        )))
    });

    let pwd = cur.as_path().as_os_str().to_str().unwrap();
    match syspath.insert(0, format!("{}/{}", pwd, AI_STRATEGO_PYTHON_MODULE)) {
        Ok(_) => Ok(()),
        Err(_) => panic!(StrategoError::AILoadingError(String::from(
            "Failed to load ai for stratego"
        ))),
    }
}

//#[pyfunction]
//fn moving() {

//}

#[pyfunction] //TODO make this works
fn get_available_moves(game_id: i128) -> PyResult<Vec<(PyCoord, PyCoord, PyColor)>> {
    if let Some(game) = game_pool::find_game_by_id(game_id) {
        let engine = game.get_engine();
        let moves = engine_utils::get_availables_moves(engine.status());

        let moves = moves.iter()
            .map(|m| {
                let from = m.0;
                let to = m.1;
                (into_py_coord(from), into_py_coord(to), into_py_color(m.2))
            })
        .collect();
       Ok(moves)
    } else {
        panic!("Failed to find game {}", game_id);
    }
}

#[pyfunction]
fn register_game(player1: String, player2: String) -> PyResult<i128> {
    let engine = StrategoEngine::new(
        create_stratego_board(),
        (
            HumanPlayer::new(Color::Red, player1),
            AIPlayer::new(Color::Blue, player2),
        ),
    );

    let mut game_id = GAME_POOL_ID.lock().unwrap();
    let gi = game_id.clone();
    let game = Game::new(*game_id, engine);
    *game_id += 1;

    if game_pool::register_to_pool(game).is_ok() {
        Ok(gi)
    } else {
        panic!("Failed to create game{}",);
    }
}

#[pyfunction]
fn hello_world() -> PyResult<String> {
    Ok(String::from("hello world"))
}

#[pymodule]
fn stratego_engine(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(hello_world))?;
    m.add_wrapped(wrap_pyfunction!(register_game))?;
    m.add_wrapped(wrap_pyfunction!(get_available_moves))?;
    Ok(())
}
