use pyo3::prelude::*;
use pyo3::types::PyList;
use pyo3::wrap_pyfunction;
use std::env::current_dir;

use crate::error::StrategoError;
use crate::player::*;

const AI_STRATEGO_PYTHON_MODULE: &'static str = "ai-python";

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
//fn init_game(player_name_1: String, player_name_2: String) -> PyResult<String> {
//let mut engine: Box<dyn Engine> = Box::new(StrategoEngine::new(
//create_stratego_board(),
//(
//Box::new(HumanPlayer::new(Color::Red, String::from("Tigran"))),
//Box::new(HumanPlayer::new(Color::Blue, String::from("Cassiopee"))),
//),
//));

//Ok(String::from("erer"))
//}

//#[pyfunction]
//fn moving() {

//}

#[pyfunction]
fn hello_world() -> PyResult<String> {
    Ok(String::from("hello world"))
}

#[pymodule]
fn stratego_engine(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(hello_world))?;
    //m.add_wrapped(wrap_pyfunction!(init_game))?;
    Ok(())
}

struct Game {
    players: (Box<dyn Player>, Box<dyn Player>),
}
