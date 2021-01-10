use pyo3::prelude::*;
use pyo3::types::PyList;
use pyo3::wrap_pyfunction;
use pythonize::pythonize;
use std::env::current_dir;

use crate::board::classic_board::create_stratego_board;
use crate::board::piece::Color;
use crate::engine::{Engine, StrategoEngine};
use crate::engine_utils;
use crate::error::StrategoError;
use crate::game_pool::{self, Game};
use crate::player::ai_player::AIPlayer;
use crate::player::HumanPlayer;
use crate::utils;
use crate::GAME_POOL_ID;

const AI_STRATEGO_PYTHON_MODULE: &str = "ai_python";

pub fn load_stratego_ai_module(py: &Python) -> Result<(), StrategoError> {
    let syspath: &PyList = py
        .import("sys")
        .unwrap_or_else(|e| {
            panic!(StrategoError::AILoadingError(format!(
                "Failed to find sys python module, {}",
                e
            )))
        })
        .get("path")
        .unwrap_or_else(|e| {
            panic!(StrategoError::AILoadingError(format!(
                "Failed to find path function in sys python module, {}",
                e
            )))
        })
        .try_into()
        .unwrap_or_else(|e| {
            panic!(StrategoError::AILoadingError(format!(
                "Failed to get result from path function in sys python module, {}",
                e
            )))
        });

    let cur = current_dir().unwrap_or_else(|e| {
        panic!(StrategoError::AILoadingError(format!(
            "Failed to find pwd, {}",
            e
        )))
    });

    let pwd = cur.as_path().as_os_str().to_str().unwrap();
    match syspath.insert(0, format!("{}/{}", pwd, AI_STRATEGO_PYTHON_MODULE)) {
        Ok(_) => Ok(()),
        Err(e) => panic!(StrategoError::AILoadingError(format!(
            "Failed to load ai for stratego, {}",
            e
        ))),
    }
}

#[pyfunction]
fn get_available_moves(game_id: i128) -> PyResult<Py<PyAny>> {
    if let Some(game) = game_pool::find_game_by_id(game_id) {
        let moves = engine_utils::get_availables_moves(game.get_engine().status());
        let gil_holder = utils::get_gild_holder().unwrap();
        let gil = gil_holder.get();
        Ok(pythonize(gil.python(), &moves).unwrap())
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
    let gi = *game_id;
    let game = Game::new(*game_id, engine);
    *game_id += 1;

    if game_pool::register_to_pool(game).is_ok() {
        Ok(gi)
    } else {
        panic!("Failed to create game {}", game_id);
    }
}

#[pyfunction]
fn get_game_state(game_id: i128) -> PyResult<Py<PyAny>> {
    if let Some(game) = game_pool::find_game_by_id(game_id) {
        let engine = game.get_engine();
        let status = engine.status().clone();
        let gil_holder = utils::get_gild_holder().unwrap();
        let gil = gil_holder.get();
        Ok(pythonize(gil.python(), &status).unwrap())
    } else {
        panic!("Failed to find game {}", game_id);
    }
}

#[pymodule]
fn stratego_engine(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(register_game))?;
    m.add_wrapped(wrap_pyfunction!(get_available_moves))?;
    m.add_wrapped(wrap_pyfunction!(get_game_state))?;
    Ok(())
}
