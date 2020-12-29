use pyo3::prelude::*;
use pyo3::types::PyList;
use pyo3::wrap_pyfunction;
use std::env::current_dir;

use crate::board::case::Coordinate;
use crate::error::StrategoError;
use crate::parse;
//use crate::player::*;

const AI_STRATEGO_PYTHON_MODULE: &str = "ai-python";

pub type PyCoords = ((i16, String), (i16, String));

pub fn get_to_coord(co: PyCoords) -> (Coordinate, Coordinate) {
    let co_0 = co.0;
    let co_1 = co.1;

    (
        Coordinate::new(co_0.0, parse::parse_letter_to_i16(co_0.1.as_str())),
        Coordinate::new(co_1.0, parse::parse_letter_to_i16(co_1.1.as_str())),
    )
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

#[pyfunction]
fn hello_world() -> PyResult<String> {
    Ok(String::from("hello world"))
}

#[pymodule]
fn stratego_engine(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(hello_world))?;
    Ok(())
}

//struct Game {
//players: (Box<dyn Player>, Box<dyn Player>),
//}
